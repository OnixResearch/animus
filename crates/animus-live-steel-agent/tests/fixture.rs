//! Deterministic production-entrypoint and resumable-host tests.

#[path = "fixture/providerport.rs"]
mod providerport;

use animus_live_steel_agent::{
    FixtureStreamProvider, LiveSteelAgentComposition, LiveSteelAgentCompositionInput,
    parse_live_steel_agent_config,
};
use animus_runtime::rlm_kernel::RlmKernelHostExecutor;

const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
const TOOL_STREAM: &[u8] =
    include_bytes!("../../animus-std-openai/tests/fixtures/openai-responses-tool-call.sse");
const FINAL_STREAM: &[u8] =
    include_bytes!("../../animus-std-openai/tests/fixtures/openai-responses-final.sse");
const EXPECTED_MESSAGES: u32 = 5;
const EXPECTED_PROVIDER_DISPATCHES: u32 = 2;
const EXPECTED_TOOL_DISPATCHES: u32 = 1;
const EXPECTED_ZERO_HOST_EFFECTS: u64 = 0;
const CORRELATION_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const CORRELATION_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const INVALID_CORRELATION: &str = "not-a-blake3-identity";

#[derive(Default)]
struct ContextExecutor {
    calls: u32,
}

impl RlmKernelHostExecutor for ContextExecutor {
    type Error = &'static str;

    fn execute(
        &mut self,
        request: &animus_adapter_rlm_kernel::HostRequest,
    ) -> Result<String, Self::Error> {
        match request {
            animus_adapter_rlm_kernel::HostRequest::ContextRead { key } if key == "fixture-key" => {
                self.calls = self.calls.saturating_add(1);
                Ok(String::from("fixture-value"))
            }
            _other => Err("unexpected fixture request"),
        }
    }
}

impl animus_live_steel_agent::LiveSessionExecutorControl for ContextExecutor {
    type Error = &'static str;

    fn answer_approval(
        &mut self,
        correlation_id: &str,
        _decision: animus_live_steel_agent::LiveApprovalDecision,
    ) -> Result<(), Self::Error> {
        if correlation_id == CORRELATION_A {
            Ok(())
        } else {
            Err("unknown approval")
        }
    }

    fn child_status(
        &mut self,
        child_id: &str,
    ) -> Result<animus_live_steel_agent::LiveChildStatus, Self::Error> {
        if child_id == "child-active" {
            Ok(animus_live_steel_agent::LiveChildStatus::Active)
        } else {
            Err("unknown child")
        }
    }

    fn cancel_child(
        &mut self,
        child_id: &str,
    ) -> Result<animus_live_steel_agent::LiveChildStatus, Self::Error> {
        if child_id == "child-active" {
            Ok(animus_live_steel_agent::LiveChildStatus::Cancelled)
        } else {
            Err("unknown child")
        }
    }
}

#[test]
fn production_entrypoint_runs_prompt_tool_result_continuation_and_final_commit() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.verification]
    // r[verify agent_harness_runtime.live_steel_agent_shell.tool_loop.complete]
    let config = parse_live_steel_agent_config(CONFIG).expect("fixture configuration");
    let provider = FixtureStreamProvider::new(
        [TOOL_STREAM.to_vec(), FINAL_STREAM.to_vec()],
        usize::try_from(config.max_provider_response_bytes()).expect("stream bound"),
    );
    let kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("fixture kernel");
    let composition = LiveSteelAgentComposition::new(LiveSteelAgentCompositionInput {
        config,
        provider,
        session_persistence: MemoryPersistence::default(),
        snapshot_persistence: MemorySnapshotPersistence::default(),
        kernel,
        host_executor: ContextExecutor::default(),
        kernel_persistence: (),
    })
    .expect("composition");
    let output = composition
        .run_prompt("Use Steel to calculate.")
        .expect("complete tool loop");

    assert_eq!(output.final_text(), "3");
    // r[verify agent_harness_runtime.live_terminal_agent_host.private_events.source]
    assert_eq!(
        u32::try_from(output.runtime_events().len()).expect("bounded event count"),
        output.receipt().event_count()
    );
    assert!(
        output
            .runtime_events()
            .iter()
            .any(|event| matches!(event, animus_runtime::RuntimeEvent::ToolRequested { .. }))
    );
    assert!(
        output
            .runtime_events()
            .iter()
            .any(|event| matches!(event, animus_runtime::RuntimeEvent::ToolResponded { .. }))
    );
    assert!(
        output
            .runtime_events()
            .iter()
            .any(|event| matches!(event, animus_runtime::RuntimeEvent::ModelResponded { .. }))
    );
    assert!(
        output
            .runtime_events()
            .iter()
            .any(|event| matches!(event, animus_runtime::RuntimeEvent::FinalResponse { .. }))
    );
    assert_eq!(
        output.receipt().committed_message_count(),
        EXPECTED_MESSAGES
    );
    assert_eq!(
        output.receipt().provider_dispatch_count(),
        EXPECTED_PROVIDER_DISPATCHES
    );
    assert_eq!(
        output.receipt().tool_dispatch_count(),
        EXPECTED_TOOL_DISPATCHES
    );
    assert_eq!(
        output.receipt().host_observation_count(),
        EXPECTED_ZERO_HOST_EFFECTS
    );
    let rendered = output.receipt().render();
    for secret in [
        "Use Steel",
        "(+ 1 2)",
        "call_fixture",
        "resp_tool_fixture",
        "sk-test",
    ] {
        assert!(!rendered.contains(secret));
    }
}

#[test]
fn persistent_session_drives_incrementally_and_finishes_with_private_events() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.persistent_session]
    let config = parse_live_steel_agent_config(CONFIG).expect("fixture configuration");
    let provider = FixtureStreamProvider::new(
        [TOOL_STREAM.to_vec(), FINAL_STREAM.to_vec()],
        usize::try_from(config.max_provider_response_bytes()).expect("stream bound"),
    );
    let kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("fixture kernel");
    let composition = LiveSteelAgentComposition::new(LiveSteelAgentCompositionInput {
        config,
        provider,
        session_persistence: MemoryPersistence::default(),
        snapshot_persistence: MemorySnapshotPersistence::default(),
        kernel,
        host_executor: ContextExecutor::default(),
        kernel_persistence: (),
    })
    .expect("composition");
    let mut session = composition.open_session().expect("persistent session");
    session
        .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
            correlation_id: CORRELATION_A,
            text: "Use Steel to calculate.",
        })
        .expect("begin prompt");
    assert_eq!(
        session.status(),
        animus_live_steel_agent::PersistentStatus::Running
    );
    for _step in 0..animus_runtime::coding_session_host::coding_session_effect_step_bound() {
        if session.status() == animus_live_steel_agent::PersistentStatus::Completed {
            break;
        }
        assert!(session.drive_one().expect("one bounded effect"));
    }
    assert_eq!(
        session.status(),
        animus_live_steel_agent::PersistentStatus::Completed
    );
    let first_batch = session
        .events_after(animus_live_steel_agent::LiveSteelEventRead {
            cursor: 0,
            maximum: animus_live_steel_agent::MAX_LIVE_SESSION_EVENT_BATCH,
        })
        .expect("bounded private events");
    assert!(!first_batch.runtime_events().is_empty());
    let output = session.finish().expect("completed output");
    assert_eq!(output.final_text(), "3");
    assert_eq!(
        first_batch.next_cursor(),
        u32::try_from(output.runtime_events().len()).expect("bounded event cursor")
    );
}

#[test]
fn persistent_session_interleaves_queue_mutation_and_cancellation() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.persistent_session.controls]
    let config = parse_live_steel_agent_config(CONFIG).expect("fixture configuration");
    let provider = FixtureStreamProvider::new(
        [FINAL_STREAM.to_vec()],
        usize::try_from(config.max_provider_response_bytes()).expect("stream bound"),
    );
    let kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("fixture kernel");
    let composition = LiveSteelAgentComposition::new(LiveSteelAgentCompositionInput {
        config,
        provider,
        session_persistence: MemoryPersistence::default(),
        snapshot_persistence: MemorySnapshotPersistence::default(),
        kernel,
        host_executor: ContextExecutor::default(),
        kernel_persistence: (),
    })
    .expect("composition");
    let mut session = composition.open_session().expect("persistent session");
    session
        .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
            correlation_id: CORRELATION_A,
            text: "first private prompt",
        })
        .expect("begin prompt");
    assert!(session.drive_one().expect("commit prompt"));
    session
        .begin_follow_up(animus_live_steel_agent::LiveSteelQueuedInput {
            correlation_id: CORRELATION_B,
            text: "queued private follow-up",
        })
        .expect("queue follow-up");
    let before = session.queue_snapshot().expect("safe queue snapshot");
    let item_id = before.entries.first().expect("queued item").item_id.clone();
    session
        .begin_queue_edit(animus_live_steel_agent::LiveSteelQueueEdit {
            item_id: item_id.as_str(),
            expected_generation: before.generation.value(),
            expected_queue_identity: before.queue_identity.as_str(),
            replacement: "edited private follow-up",
        })
        .expect("exact queue edit");
    let after = session.queue_snapshot().expect("edited queue snapshot");
    assert_ne!(before.queue_identity, after.queue_identity);
    let stale = session.begin_queue_removal(animus_live_steel_agent::LiveSteelQueueRemoval {
        item_id: item_id.as_str(),
        expected_generation: before.generation.value(),
        expected_queue_identity: before.queue_identity.as_str(),
    });
    assert!(stale.is_err());
    session.begin_cancel().expect("begin cancellation");
    session.drive_pending().expect("observe cancellation");
    assert_eq!(
        session.status(),
        animus_live_steel_agent::PersistentStatus::Cancelled
    );
    let events = session
        .events_after(animus_live_steel_agent::LiveSteelEventRead {
            cursor: 0,
            maximum: animus_live_steel_agent::MAX_LIVE_SESSION_EVENT_BATCH,
        })
        .expect("private event batch");
    assert!(events.runtime_events().iter().any(|event| matches!(
        event,
        animus_runtime::RuntimeEvent::SessionQueueMutationObserved { .. }
    )));
    session.close().expect("release cancelled session");
}

#[test]
fn persistent_session_rejects_malformed_identity_and_event_bounds() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.persistent_session.failure]
    let config = parse_live_steel_agent_config(CONFIG).expect("fixture configuration");
    let provider = FixtureStreamProvider::new(
        [FINAL_STREAM.to_vec()],
        usize::try_from(config.max_provider_response_bytes()).expect("stream bound"),
    );
    let kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("fixture kernel");
    let composition = LiveSteelAgentComposition::new(LiveSteelAgentCompositionInput {
        config,
        provider,
        session_persistence: MemoryPersistence::default(),
        snapshot_persistence: MemorySnapshotPersistence::default(),
        kernel,
        host_executor: ContextExecutor::default(),
        kernel_persistence: (),
    })
    .expect("composition");
    let mut session = composition.open_session().expect("persistent session");
    session
        .answer_approval(
            CORRELATION_A,
            animus_live_steel_agent::LiveApprovalDecision::Approve,
        )
        .expect("approval delegation");
    assert_eq!(
        session.child_status("child-active").expect("child status"),
        animus_live_steel_agent::LiveChildStatus::Active
    );
    assert_eq!(
        session.cancel_child("child-active").expect("child cancel"),
        animus_live_steel_agent::LiveChildStatus::Cancelled
    );
    assert!(
        session
            .answer_approval(
                INVALID_CORRELATION,
                animus_live_steel_agent::LiveApprovalDecision::Deny
            )
            .is_err()
    );
    assert!(session.child_status("missing child").is_err());
    assert!(
        session
            .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
                correlation_id: INVALID_CORRELATION,
                text: "private prompt",
            })
            .is_err()
    );
    assert!(
        session
            .events_after(animus_live_steel_agent::LiveSteelEventRead {
                cursor: 0,
                maximum: 0,
            })
            .is_err()
    );
    assert!(
        session
            .events_after(animus_live_steel_agent::LiveSteelEventRead {
                cursor: 0,
                maximum: animus_live_steel_agent::MAX_LIVE_SESSION_EVENT_BATCH + 1,
            })
            .is_err()
    );
    assert!(
        session
            .events_after(animus_live_steel_agent::LiveSteelEventRead {
                cursor: 1,
                maximum: 1,
            })
            .is_err()
    );
    assert_eq!(
        session.status(),
        animus_live_steel_agent::PersistentStatus::Idle
    );
    session.close().expect("release idle session");
}

#[test]
fn marker_only_host_request_is_executed_before_tool_completion() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.tool_loop.marker_only]
    let config = parse_live_steel_agent_config(CONFIG).expect("fixture configuration");
    let mut kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("kernel");
    let request = tool_request("(animus-context-read \"fixture-key\")");
    let mut executor = ContextExecutor::default();
    let mut control = FixedControl;
    let cell_identity = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let completed = animus_runtime::rlm_kernel::execute_bridged_cell(
        &mut kernel,
        animus_runtime::rlm_kernel::RlmBridgeExecutionRequest {
            source: request.arguments()[0].value(),
            session_identity: config.session_identity(),
            cell_identity,
            nesting_depth: 1,
        },
        &mut executor,
        &mut control,
    )
    .expect("host request completes");
    assert_eq!(executor.calls, 1);
    assert_eq!(completed.observations().len(), 1);
    assert!(matches!(
        completed.evaluation(),
        animus_steel::SteelRlmEvaluation::Value { value, .. } if value == "fixture-value"
    ));
}

#[test]
fn failed_kernel_safe_point_stays_pending_and_blocks_close() {
    let config = parse_live_steel_agent_config(CONFIG).expect("fixture configuration");
    let kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("fixture kernel");
    let mut service = animus_live_steel_agent::SteelToolService::new(
        kernel,
        ContextExecutor::default(),
        FailingKernelPersistence,
        animus_live_steel_agent::SteelToolBinding {
            session_identity: String::from(config.session_identity()),
            generation: config.authority_generation(),
        },
    );
    let request = tool_request("(+ 1 2)");
    assert!(matches!(
        service.execute(&request),
        Err(animus_live_steel_agent::SteelToolError::Persistence)
    ));
    assert!(matches!(
        service.execute(&request),
        Err(animus_live_steel_agent::SteelToolError::Persistence)
    ));
    assert!(matches!(
        service.close(),
        Err(animus_live_steel_agent::SteelToolError::Persistence)
    ));
}

struct FailingKernelPersistence;

impl animus_live_steel_agent::KernelSafePointPersistence for FailingKernelPersistence {
    fn persist(
        &mut self,
        _journal: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<(), animus_live_steel_agent::SteelToolError> {
        Err(animus_live_steel_agent::SteelToolError::Persistence)
    }

    fn close(self) -> Result<(), animus_live_steel_agent::SteelToolError> {
        Ok(())
    }
}

#[derive(Default)]
struct MemoryPersistence {
    messages: Vec<animus_runtime::coding_session_host::CodingSessionMessage>,
    head_hash: String,
}

impl animus_live_steel_agent::SessionPersistence for MemoryPersistence {
    fn close(self) -> Result<(), animus_live_steel_agent::LiveSteelAgentError> {
        Ok(())
    }
}

impl animus_std::coding_session_host::CodingSessionPersistenceService for MemoryPersistence {
    fn commit(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionCommitRequest,
    ) -> Result<
        animus_std::coding_session_host::CodingSessionCommitObservation,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        self.messages.extend(request.messages().iter().cloned());
        let cursor = u64::try_from(self.messages.len()).map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?;
        self.head_hash = format!("{cursor:064x}");
        Ok(
            animus_std::coding_session_host::CodingSessionCommitObservation {
                new_head_hash: self.head_hash.clone(),
                active_leaf_id: String::from(request.proposed_active_leaf_id()),
                durable_cursor: cursor,
                synchronized: true,
            },
        )
    }

    fn recover(
        &mut self,
        _request: &animus_runtime::coding_session_host::CodingSessionRecoveryRequest,
    ) -> Result<
        animus_runtime::coding_session_host::CodingSessionRecoverySnapshot,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        Ok(
            animus_runtime::coding_session_host::CodingSessionRecoverySnapshot {
                status: if self.messages.is_empty() {
                    animus_runtime::coding_session_host::CodingSessionRecoveryStatus::Empty
                } else {
                    animus_runtime::coding_session_host::CodingSessionRecoveryStatus::Clean
                },
                committed_head_hash: if self.head_hash.is_empty() {
                    String::from(
                        animus_runtime::coding_session_host::CODING_SESSION_EMPTY_HEAD_HASH,
                    )
                } else {
                    self.head_hash.clone()
                },
                active_leaf_id: self
                    .messages
                    .last()
                    .map(|message| String::from(message.entry_id())),
                durable_cursor: u64::try_from(self.messages.len()).map_err(|_error| {
                    animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
                })?,
                messages: self.messages.clone(),
            },
        )
    }
}

#[derive(Default)]
struct MemorySnapshotPersistence {
    committed: Vec<animus_runtime::model_request_snapshot::CommittedModelRequestSnapshot>,
    terminal: Vec<animus_runtime::model_request_snapshot::ModelRequestProviderTerminalObservation>,
}

impl animus_live_steel_agent::SnapshotPersistence for MemorySnapshotPersistence {
    fn close(self) -> Result<(), animus_live_steel_agent::LiveSteelAgentError> {
        Ok(())
    }
}

impl animus_std::model_request_snapshot_host::ModelRequestSnapshotStorePort
    for MemorySnapshotPersistence
{
    fn commit_snapshot(
        &mut self,
        plan: &animus_runtime::model_request_snapshot::ModelRequestSnapshotCommitPlan,
    ) -> Result<
        animus_runtime::model_request_snapshot::ModelRequestSnapshotCommitObservation,
        animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue,
    > {
        let cursor = u64::try_from(self.committed.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or(
                animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed,
            )?;
        let observation =
            animus_runtime::model_request_snapshot::ModelRequestSnapshotCommitObservation {
                plan_identity: String::from(plan.plan_identity()),
                snapshot_identity: String::from(plan.snapshot().snapshot_identity()),
                session_id: String::from(plan.snapshot().session_id()),
                branch_id: String::from(plan.snapshot().branch_id()),
                active_leaf_id: String::from(plan.snapshot().active_leaf_id()),
                turn_generation: plan.snapshot().turn_generation(),
                request_sequence: plan.snapshot().request_sequence(),
                new_head_hash: String::from(plan.plan_identity()),
                durable_cursor: cursor,
                complete: true,
                synchronized: true,
            };
        let committed =
            animus_runtime::model_request_snapshot::observe_model_request_snapshot_commit(
                plan.clone(),
                &observation,
            )
            .map_err(|_error| {
                animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed
            })?;
        self.committed.push(committed);
        Ok(observation)
    }

    fn commit_terminal(
        &mut self,
        observation: &animus_runtime::model_request_snapshot::ModelRequestProviderTerminalObservation,
    ) -> Result<(), animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue> {
        self.terminal.push(observation.clone());
        Ok(())
    }

    fn load_committed_snapshot(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<
        Option<animus_runtime::model_request_snapshot::CommittedModelRequestSnapshot>,
        animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue,
    > {
        Ok(self
            .committed
            .iter()
            .rev()
            .find(|value| value.snapshot().snapshot_identity() == snapshot_identity)
            .cloned())
    }

    fn load_terminal_observation(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<
        Option<animus_runtime::model_request_snapshot::ModelRequestProviderTerminalObservation>,
        animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue,
    > {
        Ok(self
            .terminal
            .iter()
            .rev()
            .find(|value| value.snapshot_identity == snapshot_identity)
            .cloned())
    }
}

struct FixedControl;

impl animus_runtime::rlm_kernel::RlmBridgeControl for FixedControl {
    fn observe(
        &mut self,
        _checkpoint: animus_runtime::rlm_kernel::RlmBridgeCheckpoint,
    ) -> animus_runtime::rlm_kernel::RlmBridgeControlObservation {
        animus_runtime::rlm_kernel::RlmBridgeControlObservation {
            cancelled: false,
            elapsed_ms: 0,
        }
    }
}

fn tool_request(code: &str) -> animus_adapters::ToolRequest {
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: animus_std_openai::STEEL_REPL_TOOL_SCHEMA,
            allowed_field_names: &["code"],
            compatibility_rules: &[],
            fields: &[animus_adapters::tool::ArgumentFieldDraft {
                name: Some("code"),
                value: Some(code),
            }],
        },
    )
    .expect("arguments");
    animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(animus_std_openai::STEEL_REPL_TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some("call.host-fixture"),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: 4_096,
        },
        mutation_targets: &[],
        required_capability_labels: &[animus_adapter_rlm_kernel::STEEL_REPL_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .expect("tool request")
}
