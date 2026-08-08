use super::*;

use animus_adapter_eval::{
    AgentExecutionPort, AgentExecutionRequest, ArtifactCollectPort, ArtifactCollectRequest,
    ArtifactRestorePort, ArtifactRestoreRequest, CoalescingEvidence, EndpointClass,
    ModelCallCapturePort, ModelCallCaptureRequest, ModelCallProjectionDraft, ModelDialect,
    ModelErrorClass, ModelFinishClass, ModelUsage, ObservationClass, ResultAppendPort,
    ResultAppendRequest, ResultReopenPort, ResultReopenRequest, ResultReplacePort,
    ResultReplaceRequest, ServiceAvailability, ServiceCapability, ServiceDescriptor, ServiceKind,
    ServiceObservation, ServiceRequest, ServiceResourceBounds, TaskSourcePort, TaskSourceRequest,
    TerminalStreamEvidence, TurnCommitEvidence, VerifierExecutionPort, VerifierExecutionRequest,
    binding_identity,
};
use animus_eval_core::{
    EvaluationSeat, EvaluationSeatRole, RecipeAction, RecipeActionKind, RecipeKind, RecipePlan,
    hash_identity,
};

const SERVICE_ITEMS: u32 = 16;
const SERVICE_BYTES: u64 = 4_096;
const ARTIFACT_BYTES: u64 = 128;
const BODY_BYTES: u64 = 1_024;
const STREAM_EVENTS: u32 = 8;
const EPISODE_PARALLELISM: u32 = 2;
const AGENT_PARALLELISM: u32 = 2;
const FIRST_ATTEMPT: u32 = 1;
const SECOND_ATTEMPT: u32 = 2;
const OBSERVED_ITEMS: u32 = 1;
const OBSERVED_TOKENS: u64 = 1;
const OBSERVED_BYTES: u64 = 64;
const EXPECTED_REOPEN_CALLS: usize = 2;
const EXPECTED_COMPLETE_OBSERVATIONS: usize = 10;
const ZERO_UNRESOLVED: u32 = 0;
const ONE_UNRESOLVED: u32 = 1;

const EVALUATION_ID: &str = "evaluation-v1";
const TASK_ID: &str = "task-v1";
const EPISODE_ID: &str = "episode-v1";
const ROLE_ID: &str = "solver-role-v1";
const MODEL_ID: &str = "model-v1";
const HARNESS_ID: &str = "harness-v1";
const SOLVER_ENV_ID: &str = "solver-environment-v1";
const VERIFIER_ENV_ID: &str = "verifier-environment-v1";
const TOOL_ID: &str = "tool-catalog-v1";
const VERIFIER_ID: &str = "verifier-v1";
const RUBRIC_ID: &str = "rubric-v1";
const ARTIFACT_ID: &str = "artifact-manifest-v1";
const SERVICE_POLICY_ID: &str = "evaluation-service-policy-v1";
const CANCELLATION_ID: &str = "cancellation-v1";
const DEADLINE_ID: &str = "deadline-v1";
const LINEAGE_ID: &str = "attempt-lineage-v1";
const CORRELATION_ID: &str = "correlation-v1";
const CATALOG_ID: &str = "service-catalog-v1";
const AUXILIARY_CATALOG_ID: &str = "auxiliary-catalog-v1";
const RUN_SOLVER_ID: &str = "run-solver-v1";
const RUN_SOLVER_TWO_ID: &str = "run-solver-v2";
const RUN_USER_ID: &str = "run-simulated-user-v1";
const RUN_VERIFIER_ID: &str = "run-verifier-v1";
const REQUEST_ID: &str = "model-request-v1";
const TURN_ID: &str = "active-turn-v1";
const ACCEPTANCE_ID: &str = "accepted-turn-commit-v1";
const TURN_COMMIT_ID: &str = "turn-commit-v1";
const RESULT_ID: &str = "model-result-v1";
const SAMPLING_ID: &str = "sampling-v1";
const BIND_ID: &str = "loopback-bind-v1";
const MODEL_CAPABILITY_ID: &str = "model-capability-v1";
const STATE_CAPABILITY_ID: &str = "state-capability-v1";
const STORAGE_ROOT_ID: &str = "storage-root-capability-v1";
const OUTPUT_ID: &str = "safe-output-format-v1";
const ARTIFACT_ENTRY_ID: &str = "artifact-entry-v1";
const LOGICAL_ARTIFACT_PATH: &str = "solver/output.patch";
const LOGICAL_STORAGE_PATH: &str = "evaluation/results.json";
const CLEANUP_ID: &str = "cleanup-evidence-v1";
const TOOL_OBSERVATION_ID: &str = "tool-observation-v1";
const AUXILIARY_IMPLEMENTATION_ID: &str = "auxiliary-implementation-v1";

const TASK_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::TaskSource];
const AGENT_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::AgentExecution];
const VERIFIER_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::VerifierExecution];
const COLLECT_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ArtifactCollect];
const RESTORE_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ArtifactRestore];
const MODEL_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ModelCallCapture];
const APPEND_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ResultAppend];
const REPLACE_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ResultReplace];
const REOPEN_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ResultReopen];

#[test]
fn public_entrypoint_runs_every_recipe_with_bounded_explicit_services() {
    for kind in [
        RecipeKind::SingleAgent,
        RecipeKind::BestOfN,
        RecipeKind::UserSimulation,
        RecipeKind::IsolatedAgenticJudge,
        RecipeKind::SharedAdvisoryJudge,
    ] {
        let fixture = Fixture::new(kind);
        let mut services = FakeServices::default();
        let result = execute_evaluation_host(&fixture.request(), &mut services)
            .expect("complete explicit fake host must pass");
        assert_eq!(result.status, HostStatus::Completed);
        assert!(result.peak_episode_permits <= fixture.policy.concurrency.episodes);
        assert!(result.peak_agent_permits <= fixture.policy.concurrency.agents_per_episode);
        assert_eq!(result.receipt.status, HostReceiptStatus::Completed);
        assert_eq!(services.reopen_calls, EXPECTED_REOPEN_CALLS);
        assert!(!services.calls.is_empty());
        assert!(services.tool_calls > 0);
        assert_eq!(services.cleanup_calls, 1);
        if kind == RecipeKind::IsolatedAgenticJudge {
            assert_eq!(result.observations.len(), EXPECTED_COMPLETE_OBSERVATIONS);
            assert!(services.calls.contains(&ServiceKind::VerifierExecution));
            assert!(services.calls.contains(&ServiceKind::ArtifactCollect));
            assert!(services.calls.contains(&ServiceKind::ArtifactRestore));
        }
    }
}

#[test]
fn preflight_denies_missing_ambient_and_unsafe_gateway_paths_before_effects() {
    let mut missing = Fixture::new(RecipeKind::SingleAgent);
    missing.services.pop();
    let mut fake = FakeServices::default();
    assert_eq!(
        execute_evaluation_host(&missing.request(), &mut fake),
        Err(HostError::Admission(
            animus_adapter_eval::AdmissionError::Absent
        ))
    );
    assert_eq!(fake.effect_count(), 0);

    let mut unsafe_gateway = Fixture::new(RecipeKind::SingleAgent);
    unsafe_gateway.gateway.public_bind = true;
    assert_eq!(
        execute_evaluation_host(&unsafe_gateway.request(), &mut fake),
        Err(HostError::Policy)
    );
    assert_eq!(fake.effect_count(), 0);

    let mut absent_tool = Fixture::new(RecipeKind::SingleAgent);
    absent_tool.auxiliary[0].available = false;
    assert_eq!(
        execute_evaluation_host(&absent_tool.request(), &mut fake),
        Err(HostError::Admission(
            animus_adapter_eval::AdmissionError::Unavailable
        ))
    );
    assert_eq!(fake.effect_count(), 0);
}

#[test]
fn cancellation_deadline_service_failure_and_cleanup_remain_explicit() {
    let fixture = Fixture::new(RecipeKind::SingleAgent);
    let mut cancelled = FakeServices {
        cancelled: true,
        ..FakeServices::default()
    };
    assert_eq!(
        execute_evaluation_host(&fixture.request(), &mut cancelled),
        Err(HostError::Cancelled)
    );
    assert!(cancelled.calls.is_empty());

    let mut timed_out = FakeServices {
        timed_out: true,
        ..FakeServices::default()
    };
    assert_eq!(
        execute_evaluation_host(&fixture.request(), &mut timed_out),
        Err(HostError::TimedOut)
    );
    assert!(timed_out.calls.is_empty());

    let mut denied = FakeServices {
        fail_kind: Some(ServiceKind::AgentExecution),
        ..FakeServices::default()
    };
    assert_eq!(
        execute_evaluation_host(&fixture.request(), &mut denied),
        Err(HostError::Service(ObservationClass::Denied))
    );
    assert_eq!(denied.cleanup_calls, 1);

    let mut failed_cleanup = FakeServices {
        fail_kind: Some(ServiceKind::AgentExecution),
        cleanup_complete: false,
        ..FakeServices::default()
    };
    assert_eq!(
        execute_evaluation_host(&fixture.request(), &mut failed_cleanup),
        Err(HostError::CleanupRequired)
    );

    let mut cleanup_required = FakeServices {
        cleanup_complete: false,
        ..FakeServices::default()
    };
    let result = execute_evaluation_host(&fixture.request(), &mut cleanup_required)
        .expect("post-terminal cleanup failure must remain a typed result");
    assert_eq!(result.status, HostStatus::CleanupRequired);
    assert_eq!(result.receipt.status, HostReceiptStatus::CleanupRequired);
    assert_eq!(result.receipt.cleanup_obligation_count, ONE_UNRESOLVED);
}

#[test]
fn artifact_storage_gateway_and_receipt_boundaries_fail_closed() {
    let mut hostile_artifact = Fixture::new(RecipeKind::IsolatedAgenticJudge);
    hostile_artifact.artifacts[0].hidden_test = true;
    assert_eq!(
        preflight_host(&hostile_artifact.request()),
        Err(HostError::Artifact)
    );

    let mut hostile_storage = Fixture::new(RecipeKind::SingleAgent);
    hostile_storage.storage_path = "../private/results.json";
    assert_eq!(
        preflight_host(&hostile_storage.request()),
        Err(HostError::Storage)
    );

    let mut retry_mismatch = Fixture::new(RecipeKind::SingleAgent);
    retry_mismatch
        .gateway
        .coalescing
        .as_mut()
        .expect("fixture has retry evidence")
        .later_model_identity = "different-model-v1";
    assert_eq!(
        preflight_host(&retry_mismatch.request()),
        Err(HostError::Gateway(
            animus_adapter_eval::GatewayError::Coalescing
        ))
    );

    let mut early_terminal = Fixture::new(RecipeKind::SingleAgent);
    early_terminal.gateway.commit.complete = false;
    assert_eq!(
        preflight_host(&early_terminal.request()),
        Err(HostError::Gateway(
            animus_adapter_eval::GatewayError::CommitOrder
        ))
    );

    let claims = [HostReceiptClaim::AuthoritativeCompletion];
    let draft = receipt_draft(&claims, HostReceiptLeakFlags::default());
    assert_eq!(
        project_host_receipt(&draft),
        Err(HostReceiptError::Overclaim)
    );
    let leaks = HostReceiptLeakFlags {
        provider_body: true,
        ..HostReceiptLeakFlags::default()
    };
    let bounded = [HostReceiptClaim::BoundedObservation];
    assert_eq!(
        project_host_receipt(&receipt_draft(&bounded, leaks)),
        Err(HostReceiptError::Leak)
    );
}

#[test]
fn checked_policy_rejects_unknown_fields_and_authority_widening() {
    let policy = parse_evaluation_host_policy(include_bytes!(
        "../../../config/generated/evaluation-host-policy.json"
    ))
    .expect("checked policy fixture must parse");
    assert_eq!(policy.schema, EVALUATION_HOST_SCHEMA);

    let unknown = br#"{"schema":"animus.native-evaluation-host.v1","unknown":true}"#;
    assert_eq!(
        parse_evaluation_host_policy(unknown),
        Err(HostPolicyError::Decode)
    );

    let mut widened = policy;
    widened.gateway.response_rewriting_allowed = true;
    assert_eq!(
        validate_evaluation_host_policy(&widened),
        Err(HostPolicyError::Invalid)
    );
}

fn receipt_draft(claims: &[HostReceiptClaim], leaks: HostReceiptLeakFlags) -> HostReceiptDraft<'_> {
    HostReceiptDraft {
        profile_identity: "host-profile-v1",
        evaluation_identity: EVALUATION_ID,
        plan_identity: "plan-v1",
        preflight_identity: "preflight-v1",
        service_catalog_identity: CATALOG_ID,
        gateway_identity: "gateway-v1",
        verifier_identity: VERIFIER_ID,
        artifact_identity: ARTIFACT_ID,
        storage_identity: OUTPUT_ID,
        cleanup_identity: CLEANUP_ID,
        status: HostReceiptStatus::Blocked,
        observation_count: OBSERVED_ITEMS,
        redaction_count: 0,
        cleanup_obligation_count: 0,
        leaks,
        claims,
    }
}

struct Fixture {
    recipe: RecipePlan,
    policy: EvaluationHostPolicy,
    services: Vec<ServiceDescriptor<'static>>,
    auxiliary: Vec<AuxiliaryServiceDescriptor<'static>>,
    artifacts: Vec<ArtifactEntry<'static>>,
    gateway: GatewayInvocation<'static>,
    storage_path: &'static str,
}

impl Fixture {
    fn new(kind: RecipeKind) -> Self {
        Self {
            recipe: recipe(kind),
            policy: parse_evaluation_host_policy(include_bytes!(
                "../../../config/generated/evaluation-host-policy.json"
            ))
            .expect("checked policy fixture must parse"),
            services: service_descriptors(),
            auxiliary: auxiliary_descriptors(),
            artifacts: vec![ArtifactEntry {
                logical_path: LOGICAL_ARTIFACT_PATH,
                identity: ARTIFACT_ENTRY_ID,
                byte_count: ARTIFACT_BYTES,
                allowlisted: true,
                link: false,
                hidden_test: false,
            }],
            gateway: gateway_invocation(),
            storage_path: LOGICAL_STORAGE_PATH,
        }
    }

    fn request(&self) -> HostRunRequest<'_> {
        HostRunRequest {
            recipe_plan: &self.recipe,
            bindings: HostBindings {
                evaluation_identity: EVALUATION_ID,
                task_identity: TASK_ID,
                episode_identity: EPISODE_ID,
                role_identity: ROLE_ID,
                model_identity: MODEL_ID,
                harness_identity: HARNESS_ID,
                solver_environment_identity: SOLVER_ENV_ID,
                verifier_environment_identity: VERIFIER_ENV_ID,
                tool_identity: TOOL_ID,
                verifier_identity: VERIFIER_ID,
                rubric_identity: RUBRIC_ID,
                artifact_identity: ARTIFACT_ID,
                service_policy_identity: SERVICE_POLICY_ID,
                cancellation_identity: CANCELLATION_ID,
                deadline_identity: DEADLINE_ID,
                attempt: FIRST_ATTEMPT,
                attempt_lineage_identity: LINEAGE_ID,
                correlation_identity: CORRELATION_ID,
            },
            service_catalog: animus_adapter_eval::ServiceCatalog {
                catalog_identity: CATALOG_ID,
                services: &self.services,
            },
            auxiliary_services: AuxiliaryServiceCatalog {
                identity: AUXILIARY_CATALOG_ID,
                services: &self.auxiliary,
            },
            policy: &self.policy,
            gateway: self.gateway,
            artifacts: ArtifactManifest {
                identity: ARTIFACT_ID,
                entries: &self.artifacts,
            },
            storage: StorageTarget {
                root_capability_identity: STORAGE_ROOT_ID,
                logical_path: self.storage_path,
                output_identity: OUTPUT_ID,
                max_bytes: SERVICE_BYTES,
            },
            requested_episode_parallelism: EPISODE_PARALLELISM,
            requested_agent_parallelism: AGENT_PARALLELISM,
            service_items: SERVICE_ITEMS,
            service_bytes: SERVICE_BYTES,
        }
    }
}

fn recipe(kind: RecipeKind) -> RecipePlan {
    let mut seats = vec![EvaluationSeat {
        seat_identity: "seat-solver-v1".into(),
        run_identity: RUN_SOLVER_ID.into(),
        role: EvaluationSeatRole::Solver,
        order: 0,
        trainable: true,
        isolated: false,
    }];
    match kind {
        RecipeKind::SingleAgent => {}
        RecipeKind::BestOfN => seats.push(EvaluationSeat {
            seat_identity: "seat-solver-v2".into(),
            run_identity: RUN_SOLVER_TWO_ID.into(),
            role: EvaluationSeatRole::Solver,
            order: 1,
            trainable: true,
            isolated: false,
        }),
        RecipeKind::UserSimulation => seats.push(EvaluationSeat {
            seat_identity: "seat-user-v1".into(),
            run_identity: RUN_USER_ID.into(),
            role: EvaluationSeatRole::SimulatedUser,
            order: 1,
            trainable: false,
            isolated: true,
        }),
        RecipeKind::IsolatedAgenticJudge | RecipeKind::SharedAdvisoryJudge => {
            seats.push(EvaluationSeat {
                seat_identity: "seat-verifier-v1".into(),
                run_identity: RUN_VERIFIER_ID.into(),
                role: EvaluationSeatRole::Verifier,
                order: 1,
                trainable: false,
                isolated: kind == RecipeKind::IsolatedAgenticJudge,
            });
        }
    }
    let actions = vec![RecipeAction {
        action_identity: "recipe-action-v1".into(),
        kind: RecipeActionKind::ExecuteSolver,
        order: 0,
    }];
    let identity = hash_identity(
        "animus.native-evaluation-host.test-recipe.v1",
        &[kind.label()],
    );
    RecipePlan {
        identity,
        kind,
        seats,
        actions,
        advisory: kind == RecipeKind::SharedAdvisoryJudge,
        verifier_isolated: kind == RecipeKind::IsolatedAgenticJudge,
    }
}

fn service_descriptors() -> Vec<ServiceDescriptor<'static>> {
    [
        (ServiceKind::TaskSource, TASK_CAPABILITIES),
        (ServiceKind::AgentExecution, AGENT_CAPABILITIES),
        (ServiceKind::VerifierExecution, VERIFIER_CAPABILITIES),
        (ServiceKind::ArtifactCollect, COLLECT_CAPABILITIES),
        (ServiceKind::ArtifactRestore, RESTORE_CAPABILITIES),
        (ServiceKind::ModelCallCapture, MODEL_CAPABILITIES),
        (ServiceKind::ResultAppend, APPEND_CAPABILITIES),
        (ServiceKind::ResultReplace, REPLACE_CAPABILITIES),
        (ServiceKind::ResultReopen, REOPEN_CAPABILITIES),
    ]
    .into_iter()
    .map(|(kind, capabilities)| ServiceDescriptor {
        kind,
        implementation_identity: implementation_identity(kind),
        capabilities,
        availability: ServiceAvailability::Available,
        bounds: ServiceResourceBounds {
            max_items: SERVICE_ITEMS,
            max_bytes: SERVICE_BYTES,
        },
        policy_identity: SERVICE_POLICY_ID,
    })
    .collect()
}

fn auxiliary_descriptors() -> Vec<AuxiliaryServiceDescriptor<'static>> {
    [
        AuxiliaryServiceKind::Tool,
        AuxiliaryServiceKind::Timer,
        AuxiliaryServiceKind::Cancellation,
        AuxiliaryServiceKind::Cleanup,
    ]
    .into_iter()
    .map(|kind| AuxiliaryServiceDescriptor {
        kind,
        implementation_identity: AUXILIARY_IMPLEMENTATION_ID,
        policy_identity: SERVICE_POLICY_ID,
        available: true,
    })
    .collect()
}

const fn implementation_identity(kind: ServiceKind) -> &'static str {
    match kind {
        ServiceKind::TaskSource => "task-source-implementation-v1",
        ServiceKind::AgentExecution => "agent-implementation-v1",
        ServiceKind::VerifierExecution => "verifier-implementation-v1",
        ServiceKind::ArtifactCollect => "artifact-collect-implementation-v1",
        ServiceKind::ArtifactRestore => "artifact-restore-implementation-v1",
        ServiceKind::ModelCallCapture => "model-capture-implementation-v1",
        ServiceKind::ResultAppend => "result-append-implementation-v1",
        ServiceKind::ResultReplace => "result-replace-implementation-v1",
        ServiceKind::ResultReopen => "result-reopen-implementation-v1",
    }
}

const fn gateway_invocation() -> GatewayInvocation<'static> {
    GatewayInvocation {
        bind_mode: GatewayBindMode::Loopback,
        bind_identity: BIND_ID,
        model_capability_identity: MODEL_CAPABILITY_ID,
        state_capability_identity: STATE_CAPABILITY_ID,
        accepted_turn: AcceptedTurnEvidence {
            run_identity: RUN_SOLVER_ID,
            request_identity: REQUEST_ID,
            turn_identity: TURN_ID,
            policy_identity: SERVICE_POLICY_ID,
            acceptance_identity: ACCEPTANCE_ID,
            complete: true,
        },
        projection: ModelCallProjectionDraft {
            dialect: ModelDialect::OpenAiResponses,
            endpoint: EndpointClass::Responses,
            effective_model_identity: MODEL_ID,
            sampling_identity: SAMPLING_ID,
            tool_catalog_identity: TOOL_ID,
            request_identity: REQUEST_ID,
            run_identity: RUN_SOLVER_ID,
            policy_identity: SERVICE_POLICY_ID,
            attempt: FIRST_ATTEMPT,
            streaming: false,
            usage: ModelUsage {
                input_tokens: OBSERVED_TOKENS,
                output_tokens: OBSERVED_TOKENS,
                cached_input_tokens: 0,
                reasoning_tokens: 0,
            },
            finish: ModelFinishClass::Stop,
            error: ModelErrorClass::None,
            retains_raw_provider_body: false,
            rewrites_response: false,
            shares_model_and_state_capability: false,
            unbounded_material: false,
        },
        timing: GatewayTimingClass::WithinBudget,
        terminal: TerminalStreamEvidence {
            run_identity: RUN_SOLVER_ID,
            request_identity: REQUEST_ID,
            turn_identity: TURN_ID,
            policy_identity: SERVICE_POLICY_ID,
            attempt: FIRST_ATTEMPT,
            result_identity: RESULT_ID,
            commit_identity: TURN_COMMIT_ID,
        },
        commit: TurnCommitEvidence {
            run_identity: RUN_SOLVER_ID,
            request_identity: REQUEST_ID,
            turn_identity: TURN_ID,
            policy_identity: SERVICE_POLICY_ID,
            attempt: FIRST_ATTEMPT,
            commit_identity: TURN_COMMIT_ID,
            result_identity: RESULT_ID,
            complete: true,
        },
        coalescing: Some(CoalescingEvidence {
            first_request_identity: REQUEST_ID,
            later_request_identity: REQUEST_ID,
            first_run_identity: RUN_SOLVER_ID,
            later_run_identity: RUN_SOLVER_ID,
            first_model_identity: MODEL_ID,
            later_model_identity: MODEL_ID,
            first_sampling_identity: SAMPLING_ID,
            later_sampling_identity: SAMPLING_ID,
            first_policy_identity: SERVICE_POLICY_ID,
            later_policy_identity: SERVICE_POLICY_ID,
            first_turn_identity: TURN_ID,
            later_turn_identity: TURN_ID,
            attempt_lineage_identity: LINEAGE_ID,
            first_result_identity: RESULT_ID,
            first_attempt: FIRST_ATTEMPT,
            later_attempt: SECOND_ATTEMPT,
            identical_bounded_request: true,
            non_streaming: true,
        }),
        body_bytes: BODY_BYTES,
        stream_events: STREAM_EVENTS,
        public_bind: false,
        wildcard_network: false,
        rewrites_material: false,
        sealed: false,
    }
}

struct FakeServices {
    calls: Vec<ServiceKind>,
    fail_kind: Option<ServiceKind>,
    cancelled: bool,
    timed_out: bool,
    cleanup_complete: bool,
    reopen_calls: usize,
    tool_calls: usize,
    cleanup_calls: usize,
}

impl Default for FakeServices {
    fn default() -> Self {
        Self {
            calls: Vec::new(),
            fail_kind: None,
            cancelled: false,
            timed_out: false,
            cleanup_complete: true,
            reopen_calls: 0,
            tool_calls: 0,
            cleanup_calls: 0,
        }
    }
}

impl FakeServices {
    fn observe(&mut self, request: &ServiceRequest<'_>) -> ServiceObservation {
        self.calls.push(request.service());
        let outcome = if self.fail_kind == Some(request.service()) {
            ObservationClass::Denied
        } else {
            ObservationClass::Success
        };
        let result_identity = (outcome == ObservationClass::Success).then(|| {
            hash_identity(
                "animus.native-evaluation-host.fake-result.v1",
                &[request.service().label(), request.binding().run_identity],
            )
        });
        ServiceObservation {
            service: request.service(),
            request_identity: request.identity().expect("request must be valid"),
            binding_identity: binding_identity(request.binding()).expect("binding must be valid"),
            correlation_identity: request.binding().correlation_identity.into(),
            implementation_identity: request.implementation_identity().into(),
            attempt: request.binding().attempt,
            outcome,
            result_identity,
            item_count: OBSERVED_ITEMS,
            byte_count: OBSERVED_BYTES,
        }
    }

    fn effect_count(&self) -> usize {
        self.calls.len() + self.tool_calls + self.cleanup_calls
    }
}

macro_rules! fake_port {
    ($trait_name:ident, $request_name:ident, $method:ident) => {
        impl $trait_name for FakeServices {
            fn $method(&mut self, request: &$request_name<'_>) -> ServiceObservation {
                self.observe(request.service())
            }
        }
    };
}

fake_port!(TaskSourcePort, TaskSourceRequest, materialize);
fake_port!(AgentExecutionPort, AgentExecutionRequest, execute_agent);
fake_port!(
    VerifierExecutionPort,
    VerifierExecutionRequest,
    execute_verifier
);
fake_port!(
    ArtifactCollectPort,
    ArtifactCollectRequest,
    collect_artifacts
);
fake_port!(
    ArtifactRestorePort,
    ArtifactRestoreRequest,
    restore_artifacts
);
fake_port!(
    ModelCallCapturePort,
    ModelCallCaptureRequest,
    capture_model_call
);
fake_port!(ResultAppendPort, ResultAppendRequest, append_result);
fake_port!(ResultReplacePort, ResultReplaceRequest, replace_result);

impl ResultReopenPort for FakeServices {
    fn reopen_result(&mut self, request: &ResultReopenRequest<'_>) -> ServiceObservation {
        self.reopen_calls += 1;
        self.observe(request.service())
    }
}

impl CancellationPort for FakeServices {
    fn cancellation_current(&mut self, _cancellation_identity: &str) -> bool {
        self.cancelled
    }
}

impl DeadlinePort for FakeServices {
    fn deadline_expired(&mut self, _deadline_identity: &str) -> bool {
        self.timed_out
    }
}

impl ToolPort for FakeServices {
    fn execute_tool(&mut self, _tool_identity: &str, _run_identity: &str) -> ToolObservation {
        self.tool_calls += 1;
        ToolObservation {
            identity: TOOL_OBSERVATION_ID.into(),
            complete: true,
        }
    }
}

impl CleanupPort for FakeServices {
    fn cleanup(&mut self, _obligation: &CleanupObligation<'_>) -> CleanupObservation {
        self.cleanup_calls += 1;
        let complete = self.cleanup_complete;
        CleanupObservation {
            identity: CLEANUP_ID.into(),
            complete,
            unresolved_count: if complete {
                ZERO_UNRESOLVED
            } else {
                ONE_UNRESOLVED
            },
        }
    }
}
