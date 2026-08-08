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
use animus_std_eval::{
    AcceptedTurnEvidence, ArtifactEntry, ArtifactManifest, AuxiliaryServiceCatalog,
    AuxiliaryServiceDescriptor, AuxiliaryServiceKind, CancellationPort, CleanupObligation,
    CleanupObservation, CleanupPort, DeadlinePort, EvaluationHostPolicy, GatewayBindMode,
    GatewayInvocation, GatewayTimingClass, HostBindings, HostError, HostReceiptClaim,
    HostReceiptDraft, HostReceiptError, HostReceiptLeakFlags, HostReceiptStatus, HostRunRequest,
    HostStatus, StorageTarget, ToolObservation, ToolPort, execute_evaluation_host,
    parse_evaluation_host_policy, preflight_host, project_host_receipt,
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
const ZERO_UNRESOLVED: u32 = 0;
const ONE_UNRESOLVED: u32 = 1;
const GATEWAY_NEGATIVE_COUNT: usize = 5;

const EVALUATION_ID: &str = "evaluation-conformance-v1";
const TASK_ID: &str = "task-conformance-v1";
const EPISODE_ID: &str = "episode-conformance-v1";
const ROLE_ID: &str = "solver-role-conformance-v1";
const MODEL_ID: &str = "model-conformance-v1";
const HARNESS_ID: &str = "harness-conformance-v1";
const SOLVER_ENV_ID: &str = "solver-environment-conformance-v1";
const VERIFIER_ENV_ID: &str = "verifier-environment-conformance-v1";
const TOOL_ID: &str = "tool-catalog-conformance-v1";
const VERIFIER_ID: &str = "verifier-conformance-v1";
const RUBRIC_ID: &str = "rubric-conformance-v1";
const ARTIFACT_ID: &str = "artifact-manifest-conformance-v1";
const SERVICE_POLICY_ID: &str = "evaluation-service-policy-conformance-v1";
const CANCELLATION_ID: &str = "cancellation-conformance-v1";
const DEADLINE_ID: &str = "deadline-conformance-v1";
const LINEAGE_ID: &str = "attempt-lineage-conformance-v1";
const CORRELATION_ID: &str = "correlation-conformance-v1";
const CATALOG_ID: &str = "service-catalog-conformance-v1";
const AUXILIARY_CATALOG_ID: &str = "auxiliary-catalog-conformance-v1";
const RUN_SOLVER_ID: &str = "run-solver-conformance-v1";
const RUN_SOLVER_TWO_ID: &str = "run-solver-conformance-v2";
const RUN_USER_ID: &str = "run-simulated-user-conformance-v1";
const RUN_VERIFIER_ID: &str = "run-verifier-conformance-v1";
const REQUEST_ID: &str = "model-request-conformance-v1";
const TURN_ID: &str = "active-turn-conformance-v1";
const ACCEPTANCE_ID: &str = "accepted-turn-commit-conformance-v1";
const TURN_COMMIT_ID: &str = "turn-commit-conformance-v1";
const RESULT_ID: &str = "model-result-conformance-v1";
const SAMPLING_ID: &str = "sampling-conformance-v1";
const BIND_ID: &str = "loopback-bind-conformance-v1";
const MODEL_CAPABILITY_ID: &str = "model-capability-conformance-v1";
const STATE_CAPABILITY_ID: &str = "state-capability-conformance-v1";
const STORAGE_ROOT_ID: &str = "storage-root-capability-conformance-v1";
const OUTPUT_ID: &str = "safe-output-format-conformance-v1";
const ARTIFACT_ENTRY_ID: &str = "artifact-entry-conformance-v1";
const LOGICAL_ARTIFACT_PATH: &str = "solver/output.patch";
const LOGICAL_STORAGE_PATH: &str = "evaluation/results.json";
const CLEANUP_ID: &str = "cleanup-evidence-conformance-v1";
const TOOL_OBSERVATION_ID: &str = "tool-observation-conformance-v1";
const AUXILIARY_IMPLEMENTATION_ID: &str = "auxiliary-implementation-conformance-v1";

const TASK_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::TaskSource];
const AGENT_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::AgentExecution];
const VERIFIER_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::VerifierExecution];
const COLLECT_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ArtifactCollect];
const RESTORE_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ArtifactRestore];
const MODEL_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ModelCallCapture];
const APPEND_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ResultAppend];
const REPLACE_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ResultReplace];
const REOPEN_CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::ResultReopen];

pub(crate) fn run_all_host_recipes() -> Result<(), &'static str> {
    for kind in [
        RecipeKind::SingleAgent,
        RecipeKind::BestOfN,
        RecipeKind::UserSimulation,
        RecipeKind::IsolatedAgenticJudge,
        RecipeKind::SharedAdvisoryJudge,
    ] {
        let fixture = Fixture::new(kind)?;
        let mut services = FakeServices::default();
        let result = execute_evaluation_host(&fixture.request(), &mut services)
            .map_err(|_| "host-positive")?;
        if result.status != HostStatus::Completed
            || result.receipt.status != HostReceiptStatus::Completed
            || services.calls.is_empty()
            || services.tool_calls == 0
            || services.cleanup_calls != 1
        {
            return Err("host-positive");
        }
    }
    Ok(())
}

pub(crate) fn run_missing_service_negative() -> Result<(), &'static str> {
    let mut fixture = Fixture::new(RecipeKind::SingleAgent)?;
    fixture.services.pop();
    let mut services = FakeServices::default();
    let before = services.effect_count();
    let result = execute_evaluation_host(&fixture.request(), &mut services);
    if result
        != Err(HostError::Admission(
            animus_adapter_eval::AdmissionError::Absent,
        ))
        || services.effect_count() != before
    {
        return Err("service-absence");
    }
    Ok(())
}

pub(crate) fn run_gateway_positive_and_negative() -> Result<(), &'static str> {
    let fixture = Fixture::new(RecipeKind::SingleAgent)?;
    preflight_host(&fixture.request()).map_err(|_| "gateway-positive")?;

    let mut early = Fixture::new(RecipeKind::SingleAgent)?;
    early.gateway.commit.complete = false;
    if preflight_host(&early.request())
        != Err(HostError::Gateway(
            animus_adapter_eval::GatewayError::CommitOrder,
        ))
    {
        return Err("gateway-early-terminal");
    }
    let mut changed = Fixture::new(RecipeKind::SingleAgent)?;
    changed
        .gateway
        .coalescing
        .as_mut()
        .ok_or("gateway-coalescing-fixture")?
        .later_model_identity = "changed-model-conformance-v1";
    if preflight_host(&changed.request())
        != Err(HostError::Gateway(
            animus_adapter_eval::GatewayError::Coalescing,
        ))
    {
        return Err("gateway-changed-retry");
    }
    let mutations: [fn(&mut GatewayInvocation<'static>); GATEWAY_NEGATIVE_COUNT] = [
        |gateway| gateway.public_bind = true,
        |gateway| gateway.wildcard_network = true,
        |gateway| gateway.rewrites_material = true,
        |gateway| gateway.sealed = true,
        |gateway| gateway.body_bytes = u64::MAX,
    ];
    for mutate in mutations {
        let mut unsafe_fixture = Fixture::new(RecipeKind::SingleAgent)?;
        mutate(&mut unsafe_fixture.gateway);
        if preflight_host(&unsafe_fixture.request()).is_ok() {
            return Err("gateway-unsafe-posture");
        }
    }
    Ok(())
}

pub(crate) fn run_verifier_positive_and_negative() -> Result<(), &'static str> {
    let fixture = Fixture::new(RecipeKind::IsolatedAgenticJudge)?;
    preflight_host(&fixture.request()).map_err(|_| "verifier-positive")?;

    let mut hidden = Fixture::new(RecipeKind::IsolatedAgenticJudge)?;
    hidden.artifacts[0].hidden_test = true;
    if preflight_host(&hidden.request()) != Err(HostError::Artifact) {
        return Err("verifier-hostile-artifact");
    }
    let mut linked = Fixture::new(RecipeKind::IsolatedAgenticJudge)?;
    linked.artifacts[0].link = true;
    if preflight_host(&linked.request()) != Err(HostError::Artifact) {
        return Err("verifier-linked-artifact");
    }
    let mut collision = Fixture::new(RecipeKind::IsolatedAgenticJudge)?;
    collision.artifacts.push(collision.artifacts[0]);
    if preflight_host(&collision.request()) != Err(HostError::Artifact) {
        return Err("verifier-artifact-collision");
    }
    Ok(())
}

pub(crate) fn run_cleanup_negative() -> Result<(), &'static str> {
    let fixture = Fixture::new(RecipeKind::SingleAgent)?;
    let mut services = FakeServices {
        fail_kind: Some(ServiceKind::AgentExecution),
        cleanup_complete: false,
        ..FakeServices::default()
    };
    if execute_evaluation_host(&fixture.request(), &mut services) != Err(HostError::CleanupRequired)
    {
        return Err("cleanup-required");
    }
    Ok(())
}

pub(crate) fn run_host_receipt_positive_and_negative() -> Result<(), &'static str> {
    let bounded = [HostReceiptClaim::BoundedObservation];
    let draft = receipt_draft(&bounded, HostReceiptLeakFlags::default());
    project_host_receipt(&draft).map_err(|_| "host-receipt-positive")?;
    let overclaims = [HostReceiptClaim::AuthoritativeCompletion];
    if project_host_receipt(&receipt_draft(&overclaims, HostReceiptLeakFlags::default()))
        != Err(HostReceiptError::Overclaim)
    {
        return Err("host-receipt-overclaim");
    }
    let leaks = HostReceiptLeakFlags {
        provider_body: true,
        ..HostReceiptLeakFlags::default()
    };
    if project_host_receipt(&receipt_draft(&bounded, leaks)) != Err(HostReceiptError::Leak) {
        return Err("host-receipt-leak");
    }
    Ok(())
}

fn receipt_draft(claims: &[HostReceiptClaim], leaks: HostReceiptLeakFlags) -> HostReceiptDraft<'_> {
    HostReceiptDraft {
        profile_identity: "host-profile-conformance-v1",
        evaluation_identity: EVALUATION_ID,
        plan_identity: "plan-conformance-v1",
        preflight_identity: "preflight-conformance-v1",
        service_catalog_identity: CATALOG_ID,
        gateway_identity: "gateway-conformance-v1",
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

pub(crate) struct Fixture {
    recipe: RecipePlan,
    policy: EvaluationHostPolicy,
    services: Vec<ServiceDescriptor<'static>>,
    auxiliary: Vec<AuxiliaryServiceDescriptor<'static>>,
    artifacts: Vec<ArtifactEntry<'static>>,
    gateway: GatewayInvocation<'static>,
    storage_path: &'static str,
}

impl Fixture {
    fn new(kind: RecipeKind) -> Result<Self, &'static str> {
        Ok(Self {
            recipe: recipe(kind),
            policy: parse_evaluation_host_policy(include_bytes!(
                "../../../config/generated/evaluation-host-policy.json"
            ))
            .map_err(|_| "host-policy")?,
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
        })
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
        seat_identity: "seat-solver-conformance-v1".into(),
        run_identity: RUN_SOLVER_ID.into(),
        role: EvaluationSeatRole::Solver,
        order: 0,
        trainable: true,
        isolated: false,
    }];
    match kind {
        RecipeKind::SingleAgent => {}
        RecipeKind::BestOfN => seats.push(EvaluationSeat {
            seat_identity: "seat-solver-conformance-v2".into(),
            run_identity: RUN_SOLVER_TWO_ID.into(),
            role: EvaluationSeatRole::Solver,
            order: 1,
            trainable: true,
            isolated: false,
        }),
        RecipeKind::UserSimulation => seats.push(EvaluationSeat {
            seat_identity: "seat-user-conformance-v1".into(),
            run_identity: RUN_USER_ID.into(),
            role: EvaluationSeatRole::SimulatedUser,
            order: 1,
            trainable: false,
            isolated: true,
        }),
        RecipeKind::IsolatedAgenticJudge | RecipeKind::SharedAdvisoryJudge => {
            seats.push(EvaluationSeat {
                seat_identity: "seat-verifier-conformance-v1".into(),
                run_identity: RUN_VERIFIER_ID.into(),
                role: EvaluationSeatRole::Verifier,
                order: 1,
                trainable: false,
                isolated: kind == RecipeKind::IsolatedAgenticJudge,
            });
        }
    }
    RecipePlan {
        identity: hash_identity(
            "animus.native-evaluation-conformance.host-recipe.v1",
            &[kind.label()],
        ),
        kind,
        seats,
        actions: vec![RecipeAction {
            action_identity: "recipe-action-conformance-v1".into(),
            kind: RecipeActionKind::ExecuteSolver,
            order: 0,
        }],
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
        ServiceKind::TaskSource => "task-source-implementation-conformance-v1",
        ServiceKind::AgentExecution => "agent-implementation-conformance-v1",
        ServiceKind::VerifierExecution => "verifier-implementation-conformance-v1",
        ServiceKind::ArtifactCollect => "artifact-collect-implementation-conformance-v1",
        ServiceKind::ArtifactRestore => "artifact-restore-implementation-conformance-v1",
        ServiceKind::ModelCallCapture => "model-capture-implementation-conformance-v1",
        ServiceKind::ResultAppend => "result-append-implementation-conformance-v1",
        ServiceKind::ResultReplace => "result-replace-implementation-conformance-v1",
        ServiceKind::ResultReopen => "result-reopen-implementation-conformance-v1",
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
        projection: model_projection(),
        timing: GatewayTimingClass::WithinBudget,
        terminal: terminal_evidence(),
        commit: commit_evidence(),
        coalescing: Some(coalescing_evidence()),
        body_bytes: BODY_BYTES,
        stream_events: STREAM_EVENTS,
        public_bind: false,
        wildcard_network: false,
        rewrites_material: false,
        sealed: false,
    }
}

pub(crate) const fn model_projection() -> ModelCallProjectionDraft<'static> {
    ModelCallProjectionDraft {
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
    }
}

pub(crate) const fn terminal_evidence() -> TerminalStreamEvidence<'static> {
    TerminalStreamEvidence {
        run_identity: RUN_SOLVER_ID,
        request_identity: REQUEST_ID,
        turn_identity: TURN_ID,
        policy_identity: SERVICE_POLICY_ID,
        attempt: FIRST_ATTEMPT,
        result_identity: RESULT_ID,
        commit_identity: TURN_COMMIT_ID,
    }
}

pub(crate) const fn commit_evidence() -> TurnCommitEvidence<'static> {
    TurnCommitEvidence {
        run_identity: RUN_SOLVER_ID,
        request_identity: REQUEST_ID,
        turn_identity: TURN_ID,
        policy_identity: SERVICE_POLICY_ID,
        attempt: FIRST_ATTEMPT,
        commit_identity: TURN_COMMIT_ID,
        result_identity: RESULT_ID,
        complete: true,
    }
}

pub(crate) const fn coalescing_evidence() -> CoalescingEvidence<'static> {
    CoalescingEvidence {
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
    }
}

struct FakeServices {
    calls: Vec<ServiceKind>,
    fail_kind: Option<ServiceKind>,
    cleanup_complete: bool,
    tool_calls: usize,
    cleanup_calls: usize,
}

impl Default for FakeServices {
    fn default() -> Self {
        Self {
            calls: Vec::new(),
            fail_kind: None,
            cleanup_complete: true,
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
                "animus.native-evaluation-conformance.fake-result.v1",
                &[request.service().label(), request.binding().run_identity],
            )
        });
        ServiceObservation {
            service: request.service(),
            request_identity: request.identity().expect("admitted request identity"),
            binding_identity: binding_identity(request.binding()).expect("valid binding"),
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
fake_port!(ResultReopenPort, ResultReopenRequest, reopen_result);

impl CancellationPort for FakeServices {
    fn cancellation_current(&mut self, _cancellation_identity: &str) -> bool {
        false
    }
}

impl DeadlinePort for FakeServices {
    fn deadline_expired(&mut self, _deadline_identity: &str) -> bool {
        false
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
