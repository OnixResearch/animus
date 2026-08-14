use super::*;
use crate::{
    DurableSessionLocalStore, DurableSessionStoreLocation, DurableSessionStoreOpenRequest,
    FileSynchronizer, open_durable_session_store, parse_durable_session_store_policy_json,
};

const TEST_PROMPT: &str = "bounded host prompt";
const TEST_RESPONSE: &str = "bounded host response";
const TEST_RUN: &str = "run.host";
const TEST_USER_ENTRY: &str = "entry.host-user";
const TEST_QUEUE_ITEM: &str = "queue.host";
const TEST_QUEUE_ENTRY: &str = "entry.host-queue";
const TEST_QUEUE_BODY: &str = "steer host toward evidence";
const TEST_QUEUE_REPLACEMENT: &str = "steer host toward exact queue evidence";
const TEST_POLICY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const TEST_MAX_QUEUE_ITEMS: u32 = 8;
const EXPECTED_INITIAL_COMMIT_MESSAGES: usize = 2;
const EXPECTED_SINGLE_TURN_MESSAGES: usize = 3;
const EXPECTED_QUEUED_MESSAGES: usize = 5;
const EXPECTED_SINGLE_PROVIDER_DISPATCH: u32 = 1;
const EXPECTED_SINGLE_TURN_EVENTS: usize = 10;
const FIRST_LEASE: &str = "lease.restart.first";
const SECOND_LEASE: &str = "lease.restart.second";
const MISMATCHED_LEASE: &str = "lease.mismatched-binding";
const MISMATCHED_SESSION_ID: &str = "session.mismatched";
const TEST_TOOL_NAME: &str = "fixture_tool";
const TEST_TOOL_SCHEMA: &str = "schema.fixture-tool.v1";
const TEST_TOOL_ARGUMENT: &str = "query";
const TEST_TOOL_ARGUMENT_VALUE: &str = "bounded";
const TEST_TOOL_CALL_ID: &str = "call.fixture";
const TEST_TOOL_TARGET: &str = "resource.fixture";
const TEST_TOOL_CAPABILITY: &str = "tool.read";
const TEST_TOOL_ASSISTANT: &str = "I will inspect the fixture";
const TEST_TOOL_OUTPUT: &str = "bounded fixture output";
const TEST_TOOL_VISIBLE_CHARS: u32 = 128;
const BLAKE3_HEX_CHARS: usize = 64;
const EXPECTED_TOOL_FLOW_MESSAGES: usize = 5;
const EXPECTED_COMMITTED_TOOL_MESSAGES: usize = 4;
const EXPECTED_TOOL_FLOW_COMMITS: u32 = 4;
const EXPECTED_TOOL_RESULT_COMMITS: u32 = 3;
const EXPECTED_TOOL_FLOW_PROVIDER_DISPATCHES: u32 = 2;
const EXPECTED_TOOL_FLOW_TOOL_DISPATCHES: u32 = 1;
const EXPECTED_TOOL_COMMIT_OBSERVATIONS: u32 = 1;
const EXPECTED_TOOL_FLOW_AUTHORITY_OBSERVATIONS: u32 = 3;
const FIRST_PROVIDER_DISPATCH: u32 = 1;
const SECOND_COMMIT: u32 = 2;

#[derive(Clone, Debug)]
struct MemoryPersistence {
    messages: Vec<CodingSessionMessage>,
    head_hash: String,
    fail_on_commit: Option<u32>,
    commit_count: u32,
}

impl MemoryPersistence {
    fn new(fail_commit: bool) -> Self {
        Self {
            messages: Vec::new(),
            head_hash: String::from(
                animus_runtime::coding_session_host::CODING_SESSION_EMPTY_HEAD_HASH,
            ),
            fail_on_commit: fail_commit.then_some(1),
            commit_count: 0,
        }
    }

    fn fail_on_commit(commit_count: u32) -> Self {
        Self {
            messages: Vec::new(),
            head_hash: String::from(
                animus_runtime::coding_session_host::CODING_SESSION_EMPTY_HEAD_HASH,
            ),
            fail_on_commit: Some(commit_count),
            commit_count: 0,
        }
    }
}

impl CodingSessionPersistenceService for MemoryPersistence {
    fn commit(
        &mut self,
        request: &CodingSessionCommitRequest,
    ) -> Result<CodingSessionCommitObservation, CodingSessionServiceIssue> {
        let next_commit_count = self.commit_count.saturating_add(1);
        if self.fail_on_commit == Some(next_commit_count) {
            return Err(CodingSessionServiceIssue::Failed);
        }
        self.messages.extend(request.messages().iter().cloned());
        self.commit_count = next_commit_count;
        let cursor = u64::try_from(self.messages.len())
            .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
        self.head_hash = format!("{cursor:064x}");
        Ok(CodingSessionCommitObservation {
            new_head_hash: self.head_hash.clone(),
            active_leaf_id: String::from(request.proposed_active_leaf_id()),
            durable_cursor: cursor,
            synchronized: true,
        })
    }

    fn recover(
        &mut self,
        _request: &CodingSessionRecoveryRequest,
    ) -> Result<CodingSessionRecoverySnapshot, CodingSessionServiceIssue> {
        let status = if self.messages.is_empty() {
            CodingSessionRecoveryStatus::Empty
        } else {
            CodingSessionRecoveryStatus::Clean
        };
        Ok(CodingSessionRecoverySnapshot {
            status,
            committed_head_hash: self.head_hash.clone(),
            active_leaf_id: self
                .messages
                .last()
                .map(|message| String::from(message.entry_id())),
            durable_cursor: u64::try_from(self.messages.len())
                .map_err(|_error| CodingSessionServiceIssue::Malformed)?,
            messages: self.messages.clone(),
        })
    }
}

#[derive(Clone, Debug)]
struct TestProvider {
    dispatch_count: u32,
}

impl CodingSessionProviderService for TestProvider {
    fn dispatch(
        &mut self,
        _request: &CodingSessionProviderRequest,
    ) -> Result<CodingSessionProviderReply, CodingSessionServiceIssue> {
        self.dispatch_count = self.dispatch_count.saturating_add(1);
        let response = animus_adapters::ModelResponse::new(TEST_RESPONSE)
            .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
        Ok(CodingSessionProviderReply::Final(response))
    }
}

#[derive(Clone, Debug)]
struct ToolSequenceProvider {
    dispatch_count: u32,
}

impl CodingSessionProviderService for ToolSequenceProvider {
    fn dispatch(
        &mut self,
        _request: &CodingSessionProviderRequest,
    ) -> Result<CodingSessionProviderReply, CodingSessionServiceIssue> {
        self.dispatch_count = self.dispatch_count.saturating_add(1);
        if self.dispatch_count == FIRST_PROVIDER_DISPATCH {
            return Ok(CodingSessionProviderReply::ToolCall {
                assistant_response: animus_adapters::ModelResponse::new(TEST_TOOL_ASSISTANT)
                    .map_err(|_error| CodingSessionServiceIssue::Malformed)?,
                tool_request: test_tool_request()?,
            });
        }
        Ok(CodingSessionProviderReply::Final(
            animus_adapters::ModelResponse::new(TEST_RESPONSE)
                .map_err(|_error| CodingSessionServiceIssue::Malformed)?,
        ))
    }
}

#[derive(Clone, Debug, Default)]
struct TestTool {
    dispatch_count: u32,
    commit_observation_count: u32,
    fail_commit_observation: bool,
}

impl CodingSessionToolService for TestTool {
    fn dispatch(
        &mut self,
        request: &CodingSessionToolRequest,
    ) -> Result<animus_adapters::ToolResponse, CodingSessionServiceIssue> {
        if request.request().correlation_id() != TEST_TOOL_CALL_ID {
            return Err(CodingSessionServiceIssue::Stale);
        }
        self.dispatch_count = self.dispatch_count.saturating_add(1);
        let bounded = animus_adapters::tool::BoundedOutput::new(
            &animus_adapters::tool::OutputDraft {
                body: TEST_TOOL_OUTPUT,
                full_output_reference: None,
                hash_requirement: animus_adapters::tool::HashRequirement::Required,
            },
            animus_adapters::tool::OutputPolicy {
                visible_char_budget: TEST_TOOL_VISIBLE_CHARS,
            },
        )
        .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
        Ok(animus_adapters::ToolResponse::from_bounded(
            bounded,
            animus_adapters::tool::OutputRedactionClass::Safe,
        ))
    }

    fn observe_committed(
        &mut self,
        observation: &CodingSessionToolCommitObservation,
    ) -> Result<(), CodingSessionServiceIssue> {
        if self.fail_commit_observation {
            return Err(CodingSessionServiceIssue::Failed);
        }
        let expected_owner =
            animus_runtime::tool_execution_stages::coding_session_tool_commit_owner_identity();
        if observation.commit_owner_identity != expected_owner
            || !observation.synchronized
            || observation.result_identity.len() != BLAKE3_HEX_CHARS
        {
            return Err(CodingSessionServiceIssue::Stale);
        }
        self.commit_observation_count = self.commit_observation_count.saturating_add(1);
        Ok(())
    }
}

#[derive(Clone, Debug, Default)]
struct RecordingEvents {
    events: Vec<RuntimeEvent>,
}

impl CodingSessionEventSink for RecordingEvents {
    fn observe(&mut self, event: &RuntimeEvent) {
        self.events.push(event.clone());
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct DenyPolicy;

impl CodingSessionPolicyService for DenyPolicy {
    fn admit(&mut self, _command: &CodingSessionCommand) -> bool {
        false
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct UnavailableAuthority {
    observation_count: u32,
}

impl CodingSessionAuthorityService for UnavailableAuthority {
    fn observe_current_enforcement(
        &mut self,
        _profile: &SessionRuntimeAuthorityProfile,
    ) -> Result<SessionAuthorityEnforcementObservation, CodingSessionServiceIssue> {
        self.observation_count = self.observation_count.saturating_add(1);
        Err(CodingSessionServiceIssue::Unavailable)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct MissingTimer;

impl CodingSessionTimerService for MissingTimer {
    fn available(&self) -> bool {
        false
    }
}

struct ScriptedControlWaiter {
    outcomes: std::collections::VecDeque<
        Result<animus_adapters::AgentControlPermit, animus_adapters::AgentControlWaitError>,
    >,
    checkpoints: Vec<animus_adapters::AgentControlCheckpointKind>,
}

impl ScriptedControlWaiter {
    fn new(
        outcome: Result<
            animus_adapters::AgentControlPermit,
            animus_adapters::AgentControlWaitError,
        >,
    ) -> Self {
        Self {
            outcomes: [outcome].into_iter().collect(),
            checkpoints: Vec::new(),
        }
    }
}

impl animus_adapters::AgentControlWait for ScriptedControlWaiter {
    fn wait_until_runnable(
        &mut self,
        checkpoint: &animus_adapters::AgentControlCheckpoint,
    ) -> Result<animus_adapters::AgentControlPermit, animus_adapters::AgentControlWaitError> {
        self.checkpoints.push(checkpoint.kind());
        self.outcomes
            .pop_front()
            .unwrap_or(Ok(animus_adapters::AgentControlPermit::Continue))
    }
}

#[test]
fn host_runs_fake_provider_over_commit_before_effect_boundaries() {
    // r[verify agent_harness_runtime.coding_session_host.facade]
    // r[verify agent_harness_runtime.coding_session_host.durable_boundaries]
    let mut host = memory_host(false).expect("host");
    host.prompt(TEST_RUN, TEST_USER_ENTRY, input(TEST_PROMPT))
        .expect("prompt");

    assert_eq!(host.message_snapshot().len(), EXPECTED_SINGLE_TURN_MESSAGES);
    let first_response = host
        .event_snapshot()
        .iter()
        .position(|event| matches!(event, RuntimeEvent::FinalResponse { .. }))
        .expect("final response event");
    let assistant_commit = host
        .event_snapshot()
        .iter()
        .rposition(|event| matches!(event, RuntimeEvent::SessionCommitObserved { .. }))
        .expect("assistant commit event");
    assert!(assistant_commit < first_response);
    let receipt = host
        .receipt(CodingSessionSensitiveDraft::default())
        .expect("receipt");
    assert_eq!(
        receipt.provider_dispatch_count,
        EXPECTED_SINGLE_PROVIDER_DISPATCH
    );
    assert_eq!(receipt.tool_dispatch_count, 0);

    let services = host.into_services();
    assert_eq!(
        services.provider.dispatch_count,
        EXPECTED_SINGLE_PROVIDER_DISPATCH
    );
    assert_eq!(services.persistence.commit_count, 2);
    assert_eq!(services.events.events.len(), EXPECTED_SINGLE_TURN_EVENTS);
    assert_eq!(
        services.authority.observation_count,
        EXPECTED_SINGLE_PROVIDER_DISPATCH
    );
}

#[test]
fn host_runs_fake_tool_call_and_result_through_durable_boundaries() {
    // r[verify agent_harness_runtime.coding_session_host.conformance]
    let services = CodingSessionHostServices {
        provider: ToolSequenceProvider { dispatch_count: 0 },
        tool: TestTool::default(),
        persistence: MemoryPersistence::new(false),
        policy: AllowCodingSessionPolicy,
        authority: AdmittedCodingSessionAuthority::default(),
        cancellation: ObservedCodingSessionCancellation,
        timer: AvailableCodingSessionTimer,
        summarizer: FakeCodingSessionSummarizer {
            summary: String::from(SMOKE_SUMMARY),
        },
        events: RecordingEvents::default(),
    };
    let mut host = CodingSessionHost::new(new_state(false), services).expect("tool host");
    host.prompt(TEST_RUN, TEST_USER_ENTRY, input(TEST_PROMPT))
        .expect("tool prompt");

    assert_eq!(host.message_snapshot().len(), EXPECTED_TOOL_FLOW_MESSAGES);
    let assistant_commit = host
        .event_snapshot()
        .iter()
        .position(|event| {
            matches!(
                event,
                RuntimeEvent::SessionCommitObserved { message_kind, .. }
                    if message_kind == "assistant_tool_call"
            )
        })
        .expect("assistant tool-call commit");
    let tool_request = host
        .event_snapshot()
        .iter()
        .position(|event| matches!(event, RuntimeEvent::ToolRequested { .. }))
        .expect("tool request");
    let tool_result_commit = host
        .event_snapshot()
        .iter()
        .position(|event| {
            matches!(
                event,
                RuntimeEvent::SessionCommitObserved { message_kind, .. }
                    if message_kind == "tool_result"
            )
        })
        .expect("tool-result commit");
    let tool_response = host
        .event_snapshot()
        .iter()
        .position(|event| matches!(event, RuntimeEvent::ToolResponded { .. }))
        .expect("tool response");
    assert!(assistant_commit < tool_request);
    assert!(tool_result_commit < tool_response);

    let services = host.into_services();
    assert_eq!(
        services.provider.dispatch_count,
        EXPECTED_TOOL_FLOW_PROVIDER_DISPATCHES
    );
    assert_eq!(
        services.tool.dispatch_count,
        EXPECTED_TOOL_FLOW_TOOL_DISPATCHES
    );
    assert_eq!(
        services.tool.commit_observation_count,
        EXPECTED_TOOL_COMMIT_OBSERVATIONS
    );
    assert_eq!(
        services.persistence.commit_count,
        EXPECTED_TOOL_FLOW_COMMITS
    );
    assert_eq!(
        services.authority.observation_count,
        EXPECTED_TOOL_FLOW_AUTHORITY_OBSERVATIONS
    );
}

#[test]
fn post_commit_observer_failure_blocks_provider_continuation() {
    // r[verify agent_harness_runtime.tool_execution_stages.commit]
    let services = CodingSessionHostServices {
        provider: ToolSequenceProvider { dispatch_count: 0 },
        tool: TestTool {
            fail_commit_observation: true,
            ..TestTool::default()
        },
        persistence: MemoryPersistence::new(false),
        policy: AllowCodingSessionPolicy,
        authority: AdmittedCodingSessionAuthority::default(),
        cancellation: ObservedCodingSessionCancellation,
        timer: AvailableCodingSessionTimer,
        summarizer: FakeCodingSessionSummarizer {
            summary: String::from(SMOKE_SUMMARY),
        },
        events: RecordingEvents::default(),
    };
    let mut host = CodingSessionHost::new(new_state(false), services).expect("tool host");
    let error = host
        .prompt(TEST_RUN, TEST_USER_ENTRY, input(TEST_PROMPT))
        .expect_err("post-commit observer failure");
    assert_eq!(error.issue(), CodingSessionHostIssue::SessionBlocked);
    assert_eq!(error.reducer_issue(), Some(CodingSessionIssue::ToolFailure));
    assert_eq!(
        host.message_snapshot().len(),
        EXPECTED_COMMITTED_TOOL_MESSAGES
    );
    assert_eq!(
        host.message_snapshot()
            .last()
            .map(CodingSessionMessage::kind),
        Some(CodingSessionMessageKind::ToolResult)
    );
    let services = host.into_services();
    assert_eq!(services.provider.dispatch_count, FIRST_PROVIDER_DISPATCH);
    assert_eq!(
        services.tool.dispatch_count,
        EXPECTED_TOOL_FLOW_TOOL_DISPATCHES
    );
    assert_eq!(services.tool.commit_observation_count, 0);
    assert_eq!(
        services.persistence.commit_count,
        EXPECTED_TOOL_RESULT_COMMITS
    );
}

#[test]
fn host_blocks_tool_dispatch_when_assistant_commit_fails() {
    // r[verify agent_harness_runtime.coding_session_host.durable_boundaries]
    // r[verify agent_harness_runtime.coding_session_host.conformance]
    let services = CodingSessionHostServices {
        provider: ToolSequenceProvider { dispatch_count: 0 },
        tool: TestTool::default(),
        persistence: MemoryPersistence::fail_on_commit(SECOND_COMMIT),
        policy: AllowCodingSessionPolicy,
        authority: AdmittedCodingSessionAuthority::default(),
        cancellation: ObservedCodingSessionCancellation,
        timer: AvailableCodingSessionTimer,
        summarizer: FakeCodingSessionSummarizer {
            summary: String::from(SMOKE_SUMMARY),
        },
        events: RecordingEvents::default(),
    };
    let mut host = CodingSessionHost::new(new_state(false), services).expect("tool host");
    let error = host
        .prompt(TEST_RUN, TEST_USER_ENTRY, input(TEST_PROMPT))
        .expect_err("assistant commit failure must block");
    assert_eq!(error.issue(), CodingSessionHostIssue::SessionBlocked);
    assert_eq!(
        error.reducer_issue(),
        Some(CodingSessionIssue::PersistenceFailure)
    );
    let services = host.into_services();
    assert_eq!(services.provider.dispatch_count, FIRST_PROVIDER_DISPATCH);
    assert_eq!(services.tool.dispatch_count, 0);
    assert_eq!(services.persistence.commit_count, 1);
    assert_eq!(
        services.persistence.messages.len(),
        EXPECTED_INITIAL_COMMIT_MESSAGES
    );
}

#[test]
fn host_accepts_explicit_steering_while_provider_effect_is_pending() {
    let mut host = memory_host(false).expect("host");
    host.begin_without_driving(&CodingSessionCommand::Prompt {
        run_id: String::from(TEST_RUN),
        entry_id: String::from(TEST_USER_ENTRY),
        input: input(TEST_PROMPT),
    })
    .expect("begin prompt");
    assert!(host.drive_one_pending().expect("commit user"));
    assert!(matches!(
        host.state_snapshot().pending_effect(),
        Some(CodingSessionEffectRequest::Provider(_))
    ));
    host.begin_without_driving(&CodingSessionCommand::Steer {
        item_id: String::from(TEST_QUEUE_ITEM),
        entry_id: String::from(TEST_QUEUE_ENTRY),
        input: input(TEST_QUEUE_BODY),
    })
    .expect("queue steering");
    assert_eq!(
        host.message_snapshot().len(),
        EXPECTED_INITIAL_COMMIT_MESSAGES
    );
    assert_eq!(host.state_snapshot().queued_count(), Some(1));
    host.drive_pending().expect("finish queued flow");
    assert_eq!(host.message_snapshot().len(), EXPECTED_QUEUED_MESSAGES);
    assert_eq!(host.state_snapshot().queued_count(), Some(0));
}

#[test]
fn ownership_crossing_queue_mutation_requires_explicit_current_preconditions() {
    // r[verify agent_harness_runtime.coding_session_queue_cas.compatibility.remote]
    let mut host = memory_host(false).expect("host");
    host.begin_without_driving(&CodingSessionCommand::Prompt {
        run_id: String::from(TEST_RUN),
        entry_id: String::from(TEST_USER_ENTRY),
        input: input(TEST_PROMPT),
    })
    .expect("begin prompt");
    assert!(host.drive_one_pending().expect("commit user"));
    host.begin_without_driving(&CodingSessionCommand::Steer {
        item_id: String::from(TEST_QUEUE_ITEM),
        entry_id: String::from(TEST_QUEUE_ENTRY),
        input: input(TEST_QUEUE_BODY),
    })
    .expect("queue steering");
    let precondition = coding_session_queue_precondition(host.state_snapshot(), TEST_QUEUE_ITEM)
        .expect("explicit precondition");
    host.begin_without_driving(&CodingSessionCommand::EditQueued {
        item_id: String::from(TEST_QUEUE_ITEM),
        precondition: precondition.clone(),
        input: input(TEST_QUEUE_REPLACEMENT),
    })
    .expect("exact edit");
    let stale = host
        .begin_without_driving(&CodingSessionCommand::RemoveQueued {
            item_id: String::from(TEST_QUEUE_ITEM),
            precondition,
        })
        .expect_err("stale explicit command");
    assert_eq!(
        stale.reducer_issue(),
        Some(CodingSessionIssue::QueueConflict)
    );
    assert!(stale.queue_conflict().is_some());
    assert!(matches!(
        host.event_snapshot().last(),
        Some(RuntimeEvent::SessionQueueMutationObserved { mutation })
            if mutation.status == "conflict"
    ));

    let missing_preconditions =
        animus_runtime::coding_session_host::CodingSessionQueueMutationPrecondition::new(
            String::from(TEST_QUEUE_ENTRY),
            animus_runtime::coding_session_host::CodingSessionQueueGeneration::default(),
            0,
            String::new(),
        );
    let rejected = host
        .begin_without_driving(&CodingSessionCommand::RemoveQueued {
            item_id: String::from(TEST_QUEUE_ITEM),
            precondition: missing_preconditions,
        })
        .expect_err("position-only fallback must be rejected");
    assert_eq!(
        rejected.reducer_issue(),
        Some(CodingSessionIssue::InvalidInput)
    );
}

#[test]
fn exclusive_local_owner_can_derive_queue_preconditions_under_one_host_borrow() {
    // r[verify agent_harness_runtime.coding_session_queue_cas.compatibility]
    let mut host = memory_host(false).expect("host");
    host.begin_without_driving(&CodingSessionCommand::Prompt {
        run_id: String::from(TEST_RUN),
        entry_id: String::from(TEST_USER_ENTRY),
        input: input(TEST_PROMPT),
    })
    .expect("begin prompt");
    assert!(host.drive_one_pending().expect("commit user"));
    host.begin_without_driving(&CodingSessionCommand::Steer {
        item_id: String::from(TEST_QUEUE_ITEM),
        entry_id: String::from(TEST_QUEUE_ENTRY),
        input: input(TEST_QUEUE_BODY),
    })
    .expect("queue steering");
    host.edit_queued_exclusive_local(TEST_QUEUE_ITEM, input(TEST_QUEUE_REPLACEMENT))
        .expect("exclusive local edit");
    assert!(
        host.message_snapshot()
            .iter()
            .any(|message| message.body() == TEST_QUEUE_REPLACEMENT)
    );
}

#[test]
fn persistence_failure_blocks_provider_and_policy_denial_preserves_state() {
    let mut failing = memory_host(true).expect("host");
    let failure = failing
        .prompt(TEST_RUN, TEST_USER_ENTRY, input(TEST_PROMPT))
        .expect_err("store failure");
    assert_eq!(failure.issue(), CodingSessionHostIssue::SessionBlocked);
    assert_eq!(
        failure.reducer_issue(),
        Some(CodingSessionIssue::PersistenceFailure)
    );
    let failed_services = failing.into_services();
    assert_eq!(failed_services.provider.dispatch_count, 0);
    assert!(failed_services.persistence.messages.is_empty());

    let state = new_state(false);
    let services = CodingSessionHostServices {
        provider: TestProvider { dispatch_count: 0 },
        tool: DenyUnusedCodingSessionTool,
        persistence: MemoryPersistence::new(false),
        policy: DenyPolicy,
        authority: AdmittedCodingSessionAuthority::default(),
        cancellation: ObservedCodingSessionCancellation,
        timer: AvailableCodingSessionTimer,
        summarizer: FakeCodingSessionSummarizer {
            summary: String::from(SMOKE_SUMMARY),
        },
        events: RecordingEvents::default(),
    };
    let mut denied = CodingSessionHost::new(state, services).expect("host");
    let error = denied
        .prompt(TEST_RUN, TEST_USER_ENTRY, input(TEST_PROMPT))
        .expect_err("policy denied");
    assert_eq!(error.issue(), CodingSessionHostIssue::PolicyDenied);
    assert!(denied.message_snapshot().is_empty());
    assert!(denied.event_snapshot().is_empty());
}

#[test]
fn current_authority_unavailability_blocks_provider_after_durable_binding() {
    // r[verify agent_harness_runtime.session_authority_profile.realized_enforcement]
    let services = CodingSessionHostServices {
        provider: TestProvider { dispatch_count: 0 },
        tool: DenyUnusedCodingSessionTool,
        persistence: MemoryPersistence::new(false),
        policy: AllowCodingSessionPolicy,
        authority: UnavailableAuthority::default(),
        cancellation: ObservedCodingSessionCancellation,
        timer: AvailableCodingSessionTimer,
        summarizer: FakeCodingSessionSummarizer {
            summary: String::from(SMOKE_SUMMARY),
        },
        events: RecordingEvents::default(),
    };
    let mut host = CodingSessionHost::new(new_state(false), services).expect("host");
    let error = host
        .prompt(TEST_RUN, TEST_USER_ENTRY, input(TEST_PROMPT))
        .expect_err("authority unavailable");
    assert_eq!(error.issue(), CodingSessionHostIssue::SessionBlocked);
    assert_eq!(
        error.reducer_issue(),
        Some(CodingSessionIssue::AuthorityEnforcementUnavailable)
    );
    let services = host.into_services();
    assert_eq!(services.authority.observation_count, 1);
    assert_eq!(services.provider.dispatch_count, 0);
    assert_eq!(
        services.persistence.messages.len(),
        EXPECTED_INITIAL_COMMIT_MESSAGES
    );
}

#[test]
fn cancellation_replaces_only_pending_external_effect_and_does_not_claim_rollback() {
    let mut host = memory_host(false).expect("host");
    host.begin_without_driving(&CodingSessionCommand::Prompt {
        run_id: String::from(TEST_RUN),
        entry_id: String::from(TEST_USER_ENTRY),
        input: input(TEST_PROMPT),
    })
    .expect("begin prompt");
    host.drive_one_pending().expect("commit user");
    host.begin_without_driving(&CodingSessionCommand::Cancel {
        run_id: String::from(TEST_RUN),
    })
    .expect("cancel");
    host.drive_pending().expect("observe cancellation");
    assert_eq!(
        host.state_snapshot().terminal(),
        Some(CodingSessionTerminalKind::Cancelled)
    );
    let services = host.into_services();
    assert_eq!(services.provider.dispatch_count, 0);
}

#[test]
fn missing_timer_is_rejected_at_host_construction() {
    let services = CodingSessionHostServices {
        provider: TestProvider { dispatch_count: 0 },
        tool: DenyUnusedCodingSessionTool,
        persistence: MemoryPersistence::new(false),
        policy: AllowCodingSessionPolicy,
        authority: AdmittedCodingSessionAuthority::default(),
        cancellation: ObservedCodingSessionCancellation,
        timer: MissingTimer,
        summarizer: FakeCodingSessionSummarizer {
            summary: String::from(SMOKE_SUMMARY),
        },
        events: RecordingEvents::default(),
    };
    let result = CodingSessionHost::new(new_state(false), services);
    match result {
        Ok(_host) => panic!("missing timer must fail"),
        Err(error) => assert_eq!(error.issue(), CodingSessionHostIssue::ServiceFailed),
    }
}

#[test]
fn packaged_smoke_uses_explicit_store_and_renders_only_safe_metadata() {
    // r[verify agent_harness_runtime.coding_session_host.conformance]
    let root = TestRoot::new("packaged-smoke");
    let receipt = run_packaged_durable_coding_session_smoke(root.path.as_path(), TEST_PROMPT)
        .expect("packaged smoke");
    assert!(receipt.contains("status=completed"));
    assert!(receipt.contains("committed_messages=3"));
    assert!(receipt.contains("NON_CLAIM surface=live-provider"));
    assert!(!receipt.contains(TEST_PROMPT));
    assert!(!receipt.contains(root.path.to_string_lossy().as_ref()));
}

#[test]
fn durable_binding_mismatch_blocks_before_provider_contact() {
    // r[verify agent_harness_runtime.coding_session_host.durable_boundaries]
    // r[verify agent_harness_runtime.coding_session_host.conformance]
    let root = TestRoot::new("mismatched-binding");
    let policy = parse_durable_session_store_policy_json(GENERATED_DURABLE_POLICY).expect("policy");
    let mut descriptor = smoke_descriptor(&policy);
    descriptor.session_id = String::from(MISMATCHED_SESSION_ID);
    let cap_root =
        cap_std::fs::Dir::open_ambient_dir(root.path.as_path(), cap_std::ambient_authority())
            .expect("cap root");
    let store = open_store(&cap_root, &policy, &descriptor, MISMATCHED_LEASE);
    let persistence = DurableCodingSessionPersistence::new(store, &policy, &descriptor);
    let mut host = CodingSessionHost::new(new_state(false), services_with_persistence(persistence))
        .expect("host");

    let error = host
        .prompt(TEST_RUN, TEST_USER_ENTRY, input(TEST_PROMPT))
        .expect_err("mismatched durable identity must block");
    assert_eq!(error.issue(), CodingSessionHostIssue::SessionBlocked);
    assert_eq!(
        error.reducer_issue(),
        Some(CodingSessionIssue::PersistenceFailure)
    );
    let services = host.into_services();
    assert_eq!(services.provider.dispatch_count, 0);
    services.persistence.close().expect("close store");
}

#[test]
fn real_file_host_reopens_and_resumes_committed_messages_without_provider_contact() {
    // r[verify agent_harness_runtime.coding_session_host.recovery]
    let root = TestRoot::new("restart");
    let policy = parse_durable_session_store_policy_json(GENERATED_DURABLE_POLICY).expect("policy");
    let descriptor = smoke_descriptor(&policy);
    let cap_root =
        cap_std::fs::Dir::open_ambient_dir(root.path.as_path(), cap_std::ambient_authority())
            .expect("cap root");
    let store = open_store(&cap_root, &policy, &descriptor, FIRST_LEASE);
    let persistence = DurableCodingSessionPersistence::new(store, &policy, &descriptor);
    let mut first =
        CodingSessionHost::new(new_state(false), services_with_persistence(persistence))
            .expect("first host");
    first
        .prompt(TEST_RUN, TEST_USER_ENTRY, input(TEST_PROMPT))
        .expect("first prompt");
    first
        .into_services()
        .persistence
        .close()
        .expect("close first store");

    let reopened_root =
        cap_std::fs::Dir::open_ambient_dir(root.path.as_path(), cap_std::ambient_authority())
            .expect("reopen cap root");
    let reopened_store = open_store(&reopened_root, &policy, &descriptor, SECOND_LEASE);
    let persistence = DurableCodingSessionPersistence::new(reopened_store, &policy, &descriptor);
    let mut resumed =
        CodingSessionHost::new(new_state(true), services_with_persistence(persistence))
            .expect("resumed host");
    resumed.resume().expect("resume");
    assert_eq!(
        resumed.message_snapshot().len(),
        EXPECTED_SINGLE_TURN_MESSAGES
    );
    assert_eq!(
        resumed.state_snapshot().recovery_posture(),
        animus_runtime::coding_session_host::CodingSessionRecoveryPosture::Ready
    );
    let services = resumed.into_services();
    assert_eq!(services.provider.dispatch_count, 0);
    services.persistence.close().expect("close reopened store");
}

#[test]
fn agent_control_gate_allows_commit_then_gates_provider_dispatch() {
    // r[verify agent_harness_runtime.agent_control.runtime.gate]
    let mut host = memory_host(false).expect("host");
    host.begin_without_driving(&CodingSessionCommand::Prompt {
        run_id: String::from(TEST_RUN),
        entry_id: String::from(TEST_USER_ENTRY),
        input: input(TEST_PROMPT),
    })
    .expect("prompt must reduce");
    let mut waiter = ScriptedControlWaiter::new(Ok(animus_adapters::AgentControlPermit::Continue));
    let mut gate = CodingSessionAgentControlGate::new(&mut waiter);

    assert!(
        host.drive_one_pending_with_gate(&mut gate)
            .expect("commit must run")
    );
    assert!(
        host.drive_one_pending_with_gate(&mut gate)
            .expect("provider must run")
    );
    assert_eq!(
        waiter.checkpoints,
        [animus_adapters::AgentControlCheckpointKind::BeforeModel]
    );
    assert_eq!(
        host.into_services().provider.dispatch_count,
        EXPECTED_SINGLE_PROVIDER_DISPATCH
    );
}

#[test]
fn agent_control_gate_cancellation_blocks_provider_after_commit() {
    // r[verify agent_harness_runtime.agent_control.concurrency.cancel]
    // r[verify agent_harness_runtime.agent_control.verification.negative]
    let mut host = memory_host(false).expect("host");
    host.begin_without_driving(&CodingSessionCommand::Prompt {
        run_id: String::from(TEST_RUN),
        entry_id: String::from(TEST_USER_ENTRY),
        input: input(TEST_PROMPT),
    })
    .expect("prompt must reduce");
    let mut waiter = ScriptedControlWaiter::new(Ok(animus_adapters::AgentControlPermit::Cancel));
    let mut gate = CodingSessionAgentControlGate::new(&mut waiter);

    assert!(
        host.drive_one_pending_with_gate(&mut gate)
            .expect("commit must run")
    );
    assert!(
        host.drive_one_pending_with_gate(&mut gate)
            .expect("provider denial must reduce")
    );
    assert_eq!(
        waiter.checkpoints,
        [animus_adapters::AgentControlCheckpointKind::BeforeModel]
    );
    assert_eq!(host.into_services().provider.dispatch_count, 0);
}

type MemoryCodingSessionHost = CodingSessionHost<
    TestProvider,
    DenyUnusedCodingSessionTool,
    MemoryPersistence,
    AllowCodingSessionPolicy,
    AdmittedCodingSessionAuthority,
    ObservedCodingSessionCancellation,
    AvailableCodingSessionTimer,
    FakeCodingSessionSummarizer,
    RecordingEvents,
>;

fn memory_host(fail_commit: bool) -> Result<MemoryCodingSessionHost, CodingSessionHostError> {
    CodingSessionHost::new(
        new_state(false),
        CodingSessionHostServices {
            provider: TestProvider { dispatch_count: 0 },
            tool: DenyUnusedCodingSessionTool,
            persistence: MemoryPersistence::new(fail_commit),
            policy: AllowCodingSessionPolicy,
            authority: AdmittedCodingSessionAuthority::default(),
            cancellation: ObservedCodingSessionCancellation,
            timer: AvailableCodingSessionTimer,
            summarizer: FakeCodingSessionSummarizer {
                summary: String::from(SMOKE_SUMMARY),
            },
            events: RecordingEvents::default(),
        },
    )
}

fn new_state(resume_required: bool) -> CodingSessionState {
    new_coding_session_state(
        CodingSessionConfig {
            session_id: String::from(SMOKE_SESSION_ID),
            branch_id: String::from(SMOKE_BRANCH_ID),
            queue_profile: CodingSessionQueueProfile::Sequential,
            services: CodingSessionServiceSet::all_available(),
            authority_profile: smoke_authority_profile().expect("authority profile"),
            authority_authorization_identity: String::from(SMOKE_AUTHORIZATION_IDENTITY),
            resume_required,
        },
        animus_runtime::MessageQueuePolicy::new(animus_runtime::MessageQueuePolicyDraft {
            max_items: TEST_MAX_QUEUE_ITEMS,
            allowed_sources: &["coding-session-host"],
            policy_label: "test-coding-session-queue",
            policy_hash: TEST_POLICY_HASH,
            allow_queue_mutations: true,
            allow_interrupts: true,
        })
        .expect("queue policy"),
    )
    .expect("state")
}

fn test_tool_request() -> Result<animus_adapters::ToolRequest, CodingSessionServiceIssue> {
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: TEST_TOOL_SCHEMA,
            allowed_field_names: &[TEST_TOOL_ARGUMENT],
            compatibility_rules: &[],
            fields: &[animus_adapters::tool::ArgumentFieldDraft {
                name: Some(TEST_TOOL_ARGUMENT),
                value: Some(TEST_TOOL_ARGUMENT_VALUE),
            }],
        },
    )
    .map_err(|_error| CodingSessionServiceIssue::Malformed)?;
    animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(TEST_TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some(TEST_TOOL_CALL_ID),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: TEST_TOOL_VISIBLE_CHARS,
        },
        mutation_targets: &[TEST_TOOL_TARGET],
        required_capability_labels: &[TEST_TOOL_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .map_err(|_error| CodingSessionServiceIssue::Malformed)
}

fn input(body: &str) -> CodingSessionInput {
    CodingSessionInput {
        body: String::from(body),
        posture: CodingSessionInputPosture::ExpandedAndAdmitted,
    }
}

fn smoke_descriptor(
    policy: &animus_runtime::durable_session_store::DurableSessionStorePolicy,
) -> animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
    animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
        store_id: String::from(SMOKE_STORE_ID),
        location_capability: String::from(SMOKE_LOCATION_CAPABILITY),
        session_id: String::from(SMOKE_SESSION_ID),
        branch_id: String::from(SMOKE_BRANCH_ID),
        policy_hash: String::from(policy.policy_hash()),
        durability:
            animus_runtime::durable_session_store::DurableSessionDurability::SyncAllEachCommit,
        locking:
            animus_runtime::durable_session_store::DurableSessionLockPolicy::ExclusiveCreateNew,
        private_data:
            animus_runtime::durable_session_store::DurableSessionPrivacyPosture::ReplayPrivate,
        encryption:
            animus_runtime::durable_session_store::DurableSessionEncryptionPosture::AbsentExplicit,
        encryption_service_available: false,
        receipt_policy: String::from(SMOKE_RECEIPT_POLICY),
    }
}

fn open_store<'a>(
    root: &'a cap_std::fs::Dir,
    policy: &'a animus_runtime::durable_session_store::DurableSessionStorePolicy,
    descriptor: &'a animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
    lease_id: &'a str,
) -> DurableSessionLocalStore<'a, FileSynchronizer> {
    open_durable_session_store(
        &DurableSessionStoreOpenRequest {
            policy,
            descriptor,
            location: DurableSessionStoreLocation {
                root,
                journal_name: SMOKE_JOURNAL_NAME,
                lock_name: SMOKE_LOCK_NAME,
            },
            lease_id,
        },
        FileSynchronizer,
    )
    .expect("open store")
}

fn services_with_persistence<Persistence>(
    persistence: Persistence,
) -> CodingSessionHostServices<
    TestProvider,
    DenyUnusedCodingSessionTool,
    Persistence,
    AllowCodingSessionPolicy,
    AdmittedCodingSessionAuthority,
    ObservedCodingSessionCancellation,
    AvailableCodingSessionTimer,
    FakeCodingSessionSummarizer,
    RecordingEvents,
> {
    CodingSessionHostServices {
        provider: TestProvider { dispatch_count: 0 },
        tool: DenyUnusedCodingSessionTool,
        persistence,
        policy: AllowCodingSessionPolicy,
        authority: AdmittedCodingSessionAuthority::default(),
        cancellation: ObservedCodingSessionCancellation,
        timer: AvailableCodingSessionTimer,
        summarizer: FakeCodingSessionSummarizer {
            summary: String::from(SMOKE_SUMMARY),
        },
        events: RecordingEvents::default(),
    }
}

struct TestRoot {
    path: std::path::PathBuf,
}

impl TestRoot {
    fn new(label: &str) -> Self {
        let path = std::env::temp_dir().join(format!(
            "animus-coding-session-{}-{label}",
            std::process::id()
        ));
        let _ignored = std::fs::remove_dir_all(path.as_path());
        std::fs::create_dir_all(path.as_path()).expect("create test root");
        Self { path }
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ignored = std::fs::remove_dir_all(self.path.as_path());
    }
}
