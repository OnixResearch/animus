use std::cell::RefCell;
use std::rc::Rc;

use super::*;

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const HASH_D: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const SESSION_ID: &str = "session.host";
const BRANCH_ID: &str = "branch.main";
const ACTIVE_LEAF_ID: &str = "entry.user";
const PROVIDER_ID: &str = "openai";
const MODEL_ID: &str = "gpt-host";
const TURN_GENERATION: u64 = 5;
const REQUEST_SEQUENCE: u64 = 1;
const DURABLE_CURSOR: u64 = 9;
const RETRY_ATTEMPT: u32 = 2;
const PROVIDER_OUTPUT: &str = "provider-output";
const DURABLE_POLICY_JSON: &str =
    include_str!("../../../../config/generated/durable-session-store-policy.json");
const SNAPSHOT_STORE_ID: &str = "store.model-request-snapshots";
const SNAPSHOT_LOCATION_CAPABILITY: &str = "capability.model-request-snapshot-root";
const SNAPSHOT_RECEIPT_POLICY: &str = "metadata-only";
const SNAPSHOT_JOURNAL_NAME: &str = "model-request-snapshots.journal";
const SNAPSHOT_LOCK_NAME: &str = "model-request-snapshots.lock";
const SNAPSHOT_FIRST_LEASE: &str = "lease.model-request-snapshots.first";
const SNAPSHOT_SECOND_LEASE: &str = "lease.model-request-snapshots.second";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum StoreMode {
    Exact,
    Partial,
    FailTerminal,
}

struct FakeStore {
    mode: StoreMode,
    log: Rc<RefCell<Vec<&'static str>>>,
    committed: Option<snapshot::CommittedModelRequestSnapshot>,
    terminal: Option<snapshot::ModelRequestProviderTerminalObservation>,
}

impl FakeStore {
    fn new(mode: StoreMode, log: Rc<RefCell<Vec<&'static str>>>) -> Self {
        Self {
            mode,
            log,
            committed: None,
            terminal: None,
        }
    }
}

impl ModelRequestSnapshotStorePort for FakeStore {
    fn commit_snapshot(
        &mut self,
        plan: &snapshot::ModelRequestSnapshotCommitPlan,
    ) -> Result<snapshot::ModelRequestSnapshotCommitObservation, ModelRequestSnapshotStoreIssue>
    {
        self.log.borrow_mut().push("snapshot_commit");
        let observation = snapshot::ModelRequestSnapshotCommitObservation {
            plan_identity: String::from(plan.plan_identity()),
            snapshot_identity: String::from(plan.snapshot().snapshot_identity()),
            session_id: String::from(plan.snapshot().session_id()),
            branch_id: String::from(plan.snapshot().branch_id()),
            active_leaf_id: String::from(plan.snapshot().active_leaf_id()),
            turn_generation: plan.snapshot().turn_generation(),
            request_sequence: plan.snapshot().request_sequence(),
            new_head_hash: String::from(HASH_B),
            durable_cursor: DURABLE_CURSOR,
            complete: self.mode != StoreMode::Partial,
            synchronized: true,
        };
        if observation.complete {
            self.committed =
                snapshot::observe_model_request_snapshot_commit(plan.clone(), &observation).ok();
        }
        Ok(observation)
    }

    fn commit_terminal(
        &mut self,
        observation: &snapshot::ModelRequestProviderTerminalObservation,
    ) -> Result<(), ModelRequestSnapshotStoreIssue> {
        self.log.borrow_mut().push("terminal_commit");
        if self.mode == StoreMode::FailTerminal {
            Err(ModelRequestSnapshotStoreIssue::Failed)
        } else {
            self.terminal = Some(observation.clone());
            Ok(())
        }
    }

    fn load_committed_snapshot(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<Option<snapshot::CommittedModelRequestSnapshot>, ModelRequestSnapshotStoreIssue>
    {
        Ok(self
            .committed
            .as_ref()
            .filter(|committed| committed.snapshot().snapshot_identity() == snapshot_identity)
            .cloned())
    }

    fn load_terminal_observation(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<
        Option<snapshot::ModelRequestProviderTerminalObservation>,
        ModelRequestSnapshotStoreIssue,
    > {
        Ok(self
            .terminal
            .as_ref()
            .filter(|terminal| terminal.snapshot_identity == snapshot_identity)
            .cloned())
    }
}

struct FakeProvider {
    log: Rc<RefCell<Vec<&'static str>>>,
    status: Result<&'static str, snapshot::ModelRequestProviderTerminalStatus>,
}

impl ModelRequestSnapshotProviderPort<&'static str> for FakeProvider {
    fn dispatch(
        &mut self,
        _plan: &snapshot::ModelRequestDispatchPlan,
        _request: &animus_adapters::CanonicalModelRequest,
    ) -> Result<&'static str, snapshot::ModelRequestProviderTerminalStatus> {
        self.log.borrow_mut().push("provider_dispatch");
        self.status
    }
}

#[test]
fn shell_commits_snapshot_before_provider_and_terminal_after_provider() {
    // r[verify agent_harness_runtime.model_request_snapshot.durability]
    // r[verify agent_harness_runtime.model_request_snapshot.dispatch]
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut store = FakeStore::new(StoreMode::Exact, Rc::clone(&log));
    let mut provider = FakeProvider {
        log: Rc::clone(&log),
        status: Ok(PROVIDER_OUTPUT),
    };
    let output = dispatch_initial_model_request(&mut store, &mut provider, prepared())
        .expect("ordered dispatch");
    assert_eq!(output.output, PROVIDER_OUTPUT);
    assert_eq!(
        log.borrow().as_slice(),
        ["snapshot_commit", "provider_dispatch", "terminal_commit"]
    );
    assert_eq!(
        output.terminal.status,
        snapshot::ModelRequestProviderTerminalStatus::Succeeded
    );
}

#[test]
fn partial_snapshot_commit_stops_before_provider_contact() {
    // r[verify agent_harness_runtime.model_request_snapshot.durability]
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut store = FakeStore::new(StoreMode::Partial, Rc::clone(&log));
    let mut provider = FakeProvider {
        log: Rc::clone(&log),
        status: Ok(PROVIDER_OUTPUT),
    };
    let error = dispatch_initial_model_request(&mut store, &mut provider, prepared())
        .expect_err("partial commit");
    assert_eq!(error.issue(), ModelRequestSnapshotHostIssue::CoreRejected);
    assert_eq!(
        error.core_issue(),
        Some(snapshot::ModelRequestSnapshotIssue::CommitMismatch)
    );
    assert_eq!(log.borrow().as_slice(), ["snapshot_commit"]);
}

#[test]
fn terminal_commit_failure_preserves_uncertainty_after_provider_contact() {
    // r[verify agent_harness_runtime.model_request_snapshot.recovery]
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut store = FakeStore::new(StoreMode::FailTerminal, Rc::clone(&log));
    let mut provider = FakeProvider {
        log: Rc::clone(&log),
        status: Ok(PROVIDER_OUTPUT),
    };
    let error = dispatch_initial_model_request(&mut store, &mut provider, prepared())
        .expect_err("terminal commit");
    assert_eq!(
        error.issue(),
        ModelRequestSnapshotHostIssue::TerminalCommitFailed
    );
    assert_eq!(
        log.borrow().as_slice(),
        ["snapshot_commit", "provider_dispatch", "terminal_commit"]
    );
}

#[test]
fn durable_recovery_keeps_unobserved_attempt_uncertain_and_never_replays() {
    // r[verify agent_harness_runtime.model_request_snapshot.recovery]
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut store = FakeStore::new(StoreMode::Exact, log);
    let committed = commit_model_request_snapshot(&mut store, prepared()).expect("commit");
    let identity = String::from(committed.snapshot().snapshot_identity());
    let decision =
        classify_committed_recovery(&mut store, identity.as_str()).expect("committed uncertainty");
    assert_eq!(
        decision.status,
        snapshot::ModelRequestRecoveryStatus::CommittedUnobserved
    );
    assert!(decision.retry_owner_decision_required);
    assert!(!decision.automatic_replay);

    let missing =
        classify_committed_recovery(&mut store, HASH_D).expect_err("missing committed snapshot");
    assert_eq!(
        missing.issue(),
        ModelRequestSnapshotHostIssue::RecoveryLoadFailed
    );
}

#[test]
fn durable_recovery_accepts_only_correlated_terminal_observation() {
    // r[verify agent_harness_runtime.model_request_snapshot.recovery]
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut store = FakeStore::new(StoreMode::Exact, Rc::clone(&log));
    let mut provider = FakeProvider {
        log,
        status: Ok(PROVIDER_OUTPUT),
    };
    let output = dispatch_initial_model_request(&mut store, &mut provider, prepared())
        .expect("terminal dispatch");
    let decision =
        classify_committed_recovery(&mut store, output.committed.snapshot().snapshot_identity())
            .expect("terminal recovery");
    assert_eq!(
        decision.status,
        snapshot::ModelRequestRecoveryStatus::TerminalObserved
    );
    assert!(!decision.retry_owner_decision_required);
    assert!(!decision.automatic_replay);
}

#[test]
fn durable_store_adapter_recovers_terminal_state_after_reopen() {
    // r[verify agent_harness_runtime.model_request_snapshot.durability]
    // r[verify agent_harness_runtime.model_request_snapshot.recovery]
    let temporary = tempfile::tempdir().expect("temporary snapshot store");
    let root = cap_std::fs::Dir::open_ambient_dir(temporary.path(), cap_std::ambient_authority())
        .expect("snapshot capability root");
    let policy = crate::parse_durable_session_store_policy_json(DURABLE_POLICY_JSON)
        .expect("durable policy");
    let descriptor = snapshot_store_descriptor(&policy);
    let local = open_snapshot_store(&root, &policy, &descriptor, SNAPSHOT_FIRST_LEASE);
    let mut store = DurableModelRequestSnapshotStore::new(local, &policy, &descriptor);
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut provider = FakeProvider {
        log,
        status: Ok(PROVIDER_OUTPUT),
    };
    let output = dispatch_initial_model_request(&mut store, &mut provider, prepared())
        .expect("durable adapter dispatch");
    let identity = String::from(output.committed.snapshot().snapshot_identity());
    store.close().expect("close first snapshot store");

    let reopened = open_snapshot_store(&root, &policy, &descriptor, SNAPSHOT_SECOND_LEASE);
    let mut store = DurableModelRequestSnapshotStore::new(reopened, &policy, &descriptor);
    let decision = classify_committed_recovery(&mut store, identity.as_str())
        .expect("reopened snapshot recovery");
    assert_eq!(
        decision.status,
        snapshot::ModelRequestRecoveryStatus::TerminalObserved
    );
    assert!(!decision.automatic_replay);
    store.close().expect("close reopened snapshot store");
}

#[test]
fn same_request_retry_routes_through_owner_and_reuses_snapshot() {
    // r[verify agent_harness_runtime.model_request_snapshot.dispatch]
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut store = FakeStore::new(StoreMode::Exact, Rc::clone(&log));
    let committed = commit_model_request_snapshot(&mut store, prepared()).expect("commit");
    let current = CurrentModelRequestSnapshotBindings::exact(committed.snapshot());
    let mut provider = FakeProvider {
        log: Rc::clone(&log),
        status: Ok(PROVIDER_OUTPUT),
    };
    let denied = dispatch_same_model_request_retry(
        &mut store,
        &mut provider,
        SameModelRequestRetryInput {
            committed: committed.clone(),
            current: &current,
            owner_decision: snapshot::ModelRequestRetryOwnerDecision::Denied,
            attempt_number: RETRY_ATTEMPT,
        },
    )
    .expect_err("retry owner denied");
    assert_eq!(denied.issue(), ModelRequestSnapshotHostIssue::CoreRejected);
    assert_eq!(log.borrow().as_slice(), ["snapshot_commit"]);

    let retried = dispatch_same_model_request_retry(
        &mut store,
        &mut provider,
        SameModelRequestRetryInput {
            committed: committed.clone(),
            current: &current,
            owner_decision: snapshot::ModelRequestRetryOwnerDecision::Admitted,
            attempt_number: RETRY_ATTEMPT,
        },
    )
    .expect("retry");
    assert_eq!(
        retried.dispatch.snapshot_identity(),
        committed.snapshot().snapshot_identity()
    );
    assert_eq!(
        retried.dispatch.attempt_kind(),
        snapshot::ModelRequestAttemptKind::SameRequestRetry
    );
}

#[test]
fn coding_session_provider_wrapper_binds_reducer_lineage_before_effect() {
    // r[verify agent_harness_runtime.model_request_snapshot.durability]
    // r[verify agent_harness_runtime.model_request_snapshot.verification]
    use crate::coding_session_host as coding;

    const MAX_QUEUE_ITEMS: u32 = 4;
    const QUEUE_SOURCE: &str = "coding-session-host";
    const QUEUE_POLICY_LABEL: &str = "snapshot-wrapper-test";
    const RUN_ID: &str = "run.snapshot-wrapper";
    const ENTRY_ID: &str = "entry.snapshot-wrapper";

    let log = Rc::new(RefCell::new(Vec::new()));
    let provider = SnapshotBoundCodingSessionProvider::new(
        CodingComposer,
        FakeStore::new(StoreMode::Exact, Rc::clone(&log)),
        coding::FakeCodingSessionProvider {
            response: String::from(PROVIDER_OUTPUT),
            dispatch_count: 0,
        },
    );
    let services = coding::CodingSessionHostServices {
        provider,
        tool: coding::DenyUnusedCodingSessionTool,
        persistence: coding::MemoryCodingSessionPersistence::default(),
        policy: coding::AllowCodingSessionPolicy,
        authority: coding::AdmittedCodingSessionAuthority::default(),
        cancellation: coding::ObservedCodingSessionCancellation,
        timer: coding::AvailableCodingSessionTimer,
        summarizer: coding::FakeCodingSessionSummarizer {
            summary: String::from("summary omitted"),
        },
        events: coding::NoopCodingSessionEventSink,
    };
    let profile = authority_profile();
    let queue_policy =
        animus_runtime::MessageQueuePolicy::new(animus_runtime::MessageQueuePolicyDraft {
            max_items: MAX_QUEUE_ITEMS,
            allowed_sources: &[QUEUE_SOURCE],
            policy_label: QUEUE_POLICY_LABEL,
            policy_hash: HASH_A,
            allow_queue_mutations: true,
            allow_interrupts: true,
        })
        .expect("queue policy");
    let state = animus_runtime::coding_session_host::new_coding_session_state(
        animus_runtime::coding_session_host::CodingSessionConfig {
            session_id: String::from(SESSION_ID),
            branch_id: String::from(BRANCH_ID),
            queue_profile:
                animus_runtime::coding_session_host::CodingSessionQueueProfile::Sequential,
            services: animus_runtime::coding_session_host::CodingSessionServiceSet::all_available(),
            authority_profile: profile,
            authority_authorization_identity: String::from(HASH_B),
            resume_required: false,
        },
        queue_policy,
    )
    .expect("state");
    let mut host = coding::CodingSessionHost::new(state, services).expect("host");
    host.prompt(
        RUN_ID,
        ENTRY_ID,
        animus_runtime::coding_session_host::CodingSessionInput {
            body: String::from("private coding prompt"),
            posture:
                animus_runtime::coding_session_host::CodingSessionInputPosture::ExpandedAndAdmitted,
        },
    )
    .expect("prompt");
    let services = host.into_services();
    let (_composer, _store, provider) = services.provider.into_parts();
    assert_eq!(
        provider.dispatch_count,
        u32::try_from(REQUEST_SEQUENCE).expect("request sequence fits provider count")
    );
    assert_eq!(
        log.borrow().as_slice(),
        ["snapshot_commit", "terminal_commit"]
    );
}

struct CodingComposer;

impl CodingSessionModelRequestSnapshotComposer for CodingComposer {
    fn compose(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    ) -> Result<
        PreparedModelRequestSnapshot,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        let part = [animus_adapters::CanonicalMessageContentPartDraft::Text {
            text: request.request().prompt(),
        }];
        let messages = [animus_adapters::CanonicalModelMessageDraft {
            role: "user",
            content: &part,
            tool_call_id: None,
        }];
        let canonical = animus_adapters::CanonicalModelRequest::from_draft(
            animus_adapters::CanonicalModelRequestDraft {
                contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
                messages: &messages,
                tools: &[],
                output: animus_adapters::CanonicalModelOutputControlsDraft {
                    max_output_tokens: None,
                    temperature_millis: None,
                    stop_sequences: &[],
                    stream: false,
                },
                response_format: animus_adapters::CanonicalModelResponseFormatIntentDraft::Text,
                reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                    enabled: false,
                    effort: animus_adapters::CanonicalModelReasoningEffort::None,
                    summary_label: None,
                },
                provenance: &[],
            },
        )
        .map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?;
        let route = snapshot::model_request_route_identity(PROVIDER_ID, MODEL_ID);
        let tools = snapshot::model_request_tool_schema_set_identity(&canonical);
        let output = snapshot::model_request_output_control_identity(&canonical);
        let budget = snapshot::model_request_budget_identity(&canonical);
        let validated =
            snapshot::validate_model_request_snapshot(&snapshot::ModelRequestSnapshotDraft {
                schema: snapshot::MODEL_REQUEST_SNAPSHOT_SCHEMA,
                session_id: request.session_id(),
                branch_id: request.branch_id(),
                active_leaf_id: request.active_leaf_id(),
                turn_generation: TURN_GENERATION,
                request_sequence: request.request_sequence(),
                canonical_request: &canonical,
                route: snapshot::ModelRequestRouteDraft {
                    provider_id: PROVIDER_ID,
                    model_id: MODEL_ID,
                    route_identity: route.as_str(),
                },
                model_invocation_identity: HASH_A,
                request_context_plan_identity: HASH_B,
                selected_context_contribution_identities: &[],
                tool_schema_set_identity: tools.as_str(),
                output_control_identity: output.as_str(),
                budget_identity: budget.as_str(),
                authority_seal_identity: HASH_C,
                adapter_cohort_identity: HASH_D,
                unsupported: snapshot::ModelRequestSnapshotUnsupportedDraft::default(),
            })
            .map_err(|_error| {
                animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
            })?;
        Ok(PreparedModelRequestSnapshot {
            prior_head_hash: String::from(request.durable_head_hash()),
            current: CurrentModelRequestSnapshotBindings::exact(&validated),
            snapshot: validated,
        })
    }
}

fn authority_profile() -> animus_runtime::session_authority_profile::SessionRuntimeAuthorityProfile
{
    let components = animus_runtime::session_authority_profile::SessionAuthorityComponentClass::ALL
        .map(
            |class| animus_runtime::session_authority_profile::SessionAuthorityComponentDraft {
                class,
                identity: HASH_C,
            },
        );
    animus_runtime::session_authority_profile::build_session_authority_profile(
        animus_runtime::session_authority_profile::SessionAuthorityProfileDraft {
            components: &components,
        },
    )
    .expect("authority profile")
}

fn prepared() -> PreparedModelRequestSnapshot {
    let request = canonical_request();
    let route_identity = snapshot::model_request_route_identity(PROVIDER_ID, MODEL_ID);
    let tool_identity = snapshot::model_request_tool_schema_set_identity(&request);
    let output_identity = snapshot::model_request_output_control_identity(&request);
    let budget_identity = snapshot::model_request_budget_identity(&request);
    let validated =
        snapshot::validate_model_request_snapshot(&snapshot::ModelRequestSnapshotDraft {
            schema: snapshot::MODEL_REQUEST_SNAPSHOT_SCHEMA,
            session_id: SESSION_ID,
            branch_id: BRANCH_ID,
            active_leaf_id: ACTIVE_LEAF_ID,
            turn_generation: TURN_GENERATION,
            request_sequence: REQUEST_SEQUENCE,
            canonical_request: &request,
            route: snapshot::ModelRequestRouteDraft {
                provider_id: PROVIDER_ID,
                model_id: MODEL_ID,
                route_identity: route_identity.as_str(),
            },
            model_invocation_identity: HASH_A,
            request_context_plan_identity: HASH_B,
            selected_context_contribution_identities: &[HASH_C],
            tool_schema_set_identity: tool_identity.as_str(),
            output_control_identity: output_identity.as_str(),
            budget_identity: budget_identity.as_str(),
            authority_seal_identity: HASH_C,
            adapter_cohort_identity: HASH_D,
            unsupported: snapshot::ModelRequestSnapshotUnsupportedDraft::default(),
        })
        .expect("snapshot");
    let current = CurrentModelRequestSnapshotBindings::exact(&validated);
    PreparedModelRequestSnapshot {
        snapshot: validated,
        prior_head_hash: String::from(HASH_A),
        current,
    }
}

fn snapshot_store_descriptor(
    policy: &animus_runtime::durable_session_store::DurableSessionStorePolicy,
) -> animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
    animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
        store_id: String::from(SNAPSHOT_STORE_ID),
        location_capability: String::from(SNAPSHOT_LOCATION_CAPABILITY),
        session_id: String::from(SESSION_ID),
        branch_id: String::from(BRANCH_ID),
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
        receipt_policy: String::from(SNAPSHOT_RECEIPT_POLICY),
    }
}

fn open_snapshot_store<'store>(
    root: &'store cap_std::fs::Dir,
    policy: &'store animus_runtime::durable_session_store::DurableSessionStorePolicy,
    descriptor: &'store animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
    lease_id: &'store str,
) -> crate::DurableSessionLocalStore<'store, crate::FileSynchronizer> {
    crate::open_durable_session_store(
        &crate::DurableSessionStoreOpenRequest {
            policy,
            descriptor,
            location: crate::DurableSessionStoreLocation {
                root,
                journal_name: SNAPSHOT_JOURNAL_NAME,
                lock_name: SNAPSHOT_LOCK_NAME,
            },
            lease_id,
        },
        crate::FileSynchronizer,
    )
    .expect("open snapshot store")
}

fn canonical_request() -> animus_adapters::CanonicalModelRequest {
    let parts = [animus_adapters::CanonicalMessageContentPartDraft::Text {
        text: "private prompt",
    }];
    let messages = [animus_adapters::CanonicalModelMessageDraft {
        role: "user",
        content: &parts,
        tool_call_id: None,
    }];
    let tools = [animus_adapters::CanonicalModelToolDeclarationDraft {
        name: "steel_repl",
        description: "Evaluate an admitted Steel cell.",
        input_schema_label: "schema.steel.v1",
    }];
    animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: &tools,
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: None,
                temperature_millis: None,
                stop_sequences: &[],
                stream: true,
            },
            response_format: animus_adapters::CanonicalModelResponseFormatIntentDraft::Text,
            reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                enabled: false,
                effort: animus_adapters::CanonicalModelReasoningEffort::None,
                summary_label: None,
            },
            provenance: &[],
        },
    )
    .expect("canonical request")
}
