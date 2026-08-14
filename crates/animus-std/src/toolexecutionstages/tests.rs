use super::*;
use std::cell::RefCell;
use std::rc::Rc;

const TOOL_NAME: &str = "lookup";
const TOOL_SCHEMA: &str = "schema.lookup.v1";
const TOOL_CAPABILITY: &str = "capability.lookup";
const EXECUTOR_BINDING: &str = "executor.lookup";
const ARGUMENT_NAME: &str = "key";
const OUTPUT_VISIBLE_CHARS: u32 = 64;
const OUTPUT_BYTES: u64 = 256;
const RAW_SENSITIVE_OUTPUT: &str = "CREDENTIAL:x";
const SAFE_REPLACEMENT: &str = "[hidden]";

#[test]
fn ordered_handlers_narrow_output_and_observe_only_after_commit() {
    // r[verify agent_harness_runtime.tool_execution_stages.binding]
    // r[verify agent_harness_runtime.tool_execution_stages.output]
    // r[verify agent_harness_runtime.tool_execution_stages.commit]
    let fixture = Fixture::new();
    let handlers = handlers();
    let log = Rc::new(RefCell::new(Vec::new()));
    let mut executor = FakeExecutor::new(RAW_SENSITIVE_OUTPUT, Rc::clone(&log));
    let mut services = ToolExecutionStageServices {
        pre_admission: ExactPreAdmission::new(Rc::clone(&log)),
        pre_execution: ExactPreExecution::new(Rc::clone(&log)),
        output: RedactingOutput::new(Rc::clone(&log)),
        post_commit: ExactPostCommit::new(Rc::clone(&log)),
    };
    let pending = execute_staged_tool(
        &mut executor,
        &mut services,
        &fixture.stage_input(handlers.as_slice()),
    )
    .expect("staged execution");
    assert_eq!(
        log.borrow().as_slice(),
        [
            "pre_admission",
            "declared_wrapper",
            "executor",
            "output_narrowing"
        ]
    );
    assert_eq!(
        pending.outcome().response().model_visible_text(),
        SAFE_REPLACEMENT
    );
    assert_eq!(executor.call_count, 1);

    let commit = commit_observation_from_pending(&pending, hash("durable-head"));
    let completed = complete_staged_tool_commit(&mut services.post_commit, pending, &commit)
        .expect("commit and observe");
    assert_eq!(
        log.borrow().as_slice(),
        [
            "pre_admission",
            "declared_wrapper",
            "executor",
            "output_narrowing",
            "post_commit"
        ]
    );
    assert_eq!(
        completed.receipt.status(),
        stages::ToolExecutionStageStatus::Observed
    );
    assert_eq!(completed.receipt.handler_count(), 4);
}

#[test]
fn zero_handlers_keep_existing_executor_kit_authoritative() {
    // r[verify agent_harness_runtime.tool_execution_stages.plan]
    let fixture = Fixture::new();
    let mut executor = FakeExecutor::new("safe output", Rc::new(RefCell::new(Vec::new())));
    let mut services = ToolExecutionStageServices {
        pre_admission: NoToolExecutionStageHandlers,
        pre_execution: NoToolExecutionStageHandlers,
        output: NoToolExecutionStageHandlers,
        post_commit: NoToolExecutionStageHandlers,
    };
    let pending = execute_staged_tool(&mut executor, &mut services, &fixture.stage_input(&[]))
        .expect("zero-handler execution");
    assert_eq!(executor.call_count, 1);
    assert_eq!(
        pending.state().status(),
        stages::ToolExecutionStageStatus::AwaitingCommit
    );
}

#[test]
fn handler_substitution_and_output_growth_fail_before_claims() {
    // r[verify agent_harness_runtime.tool_execution_stages.verification]
    let fixture = Fixture::new();
    let mut executor = FakeExecutor::new("safe output", Rc::new(RefCell::new(Vec::new())));
    let mut substituted = ToolExecutionStageServices {
        pre_admission: SubstitutedPreAdmission,
        pre_execution: NoToolExecutionStageHandlers,
        output: NoToolExecutionStageHandlers,
        post_commit: NoToolExecutionStageHandlers,
    };
    assert_eq!(
        execute_staged_tool(&mut executor, &mut substituted, &fixture.stage_input(&[]))
            .expect_err("substituted handler")
            .issue(),
        ToolExecutionStageHostIssue::CoreRejected
    );
    assert_eq!(executor.call_count, 0);

    let mut growing = ToolExecutionStageServices {
        pre_admission: NoToolExecutionStageHandlers,
        pre_execution: NoToolExecutionStageHandlers,
        output: GrowingOutput,
        post_commit: NoToolExecutionStageHandlers,
    };
    assert_eq!(
        execute_staged_tool(&mut executor, &mut growing, &fixture.stage_input(&[]))
            .expect_err("output growth")
            .issue(),
        ToolExecutionStageHostIssue::CoreRejected
    );
    assert_eq!(executor.call_count, 1);
}

#[test]
fn unsafe_passthrough_and_commit_mismatch_block_observation() {
    // r[verify agent_harness_runtime.tool_execution_stages.output]
    // r[verify agent_harness_runtime.tool_execution_stages.commit]
    let fixture = Fixture::new();
    let mut executor = FakeExecutor::new(RAW_SENSITIVE_OUTPUT, Rc::new(RefCell::new(Vec::new())));
    let mut passthrough = ToolExecutionStageServices {
        pre_admission: NoToolExecutionStageHandlers,
        pre_execution: NoToolExecutionStageHandlers,
        output: NoToolExecutionStageHandlers,
        post_commit: NoToolExecutionStageHandlers,
    };
    assert_eq!(
        execute_staged_tool(&mut executor, &mut passthrough, &fixture.stage_input(&[]))
            .expect_err("unsafe final output")
            .issue(),
        ToolExecutionStageHostIssue::ExecutorRejected
    );

    let log = Rc::new(RefCell::new(Vec::new()));
    let mut executor = FakeExecutor::new("safe output", Rc::clone(&log));
    let mut services = ToolExecutionStageServices {
        pre_admission: NoToolExecutionStageHandlers,
        pre_execution: NoToolExecutionStageHandlers,
        output: NoToolExecutionStageHandlers,
        post_commit: ExactPostCommit::new(Rc::clone(&log)),
    };
    let pending = execute_staged_tool(&mut executor, &mut services, &fixture.stage_input(&[]))
        .expect("pending");
    let mut commit = commit_observation_from_pending(&pending, hash("durable-head"));
    commit.result_identity = hash("wrong-result");
    assert_eq!(
        complete_staged_tool_commit(&mut services.post_commit, pending, &commit)
            .expect_err("commit mismatch")
            .issue(),
        ToolExecutionStageHostIssue::CommitRejected
    );
    assert!(!log.borrow().contains(&"post_commit"));
}

struct Fixture {
    catalog: kit::ToolCatalog,
    capabilities: animus_runtime::RuntimeCapabilityRegistry,
    executors: Vec<kit::ToolExecutorBinding>,
    request: animus_adapters::ToolRequest,
    authority_seal_identity: String,
    commit_owner_identity: String,
}

impl Fixture {
    fn new() -> Self {
        Self {
            catalog: catalog(),
            capabilities: capabilities(),
            executors: vec![
                kit::ToolExecutorBinding::new(EXECUTOR_BINDING, kit::ToolExecutorReadiness::Ready)
                    .expect("executor"),
            ],
            request: request(),
            authority_seal_identity: hash("authority"),
            commit_owner_identity: stages::coding_session_tool_commit_owner_identity(),
        }
    }

    fn stage_input<'fixture>(
        &'fixture self,
        handlers: &'fixture [stages::ToolExecutionStageHandler],
    ) -> StagedToolExecutionInput<'fixture> {
        StagedToolExecutionInput {
            tool: kit::ToolExecutionPlanRequest {
                request: &self.request,
                catalog: &self.catalog,
                capabilities: &self.capabilities,
                executors: self.executors.as_slice(),
                admission: kit::ToolExecutorAdmission::admitted(),
            },
            authority_seal_identity: self.authority_seal_identity.as_str(),
            commit_owner_identity: self.commit_owner_identity.as_str(),
            handlers,
        }
    }
}

struct FakeExecutor {
    output: String,
    log: Rc<RefCell<Vec<&'static str>>>,
    call_count: u32,
}

impl FakeExecutor {
    fn new(output: &str, log: Rc<RefCell<Vec<&'static str>>>) -> Self {
        Self {
            output: String::from(output),
            log,
            call_count: 0,
        }
    }
}

impl kit::ToolExecutorService for FakeExecutor {
    fn binding_label(&self) -> &str {
        EXECUTOR_BINDING
    }

    fn execute(
        &mut self,
        _plan: &kit::ToolExecutionPlan,
        _request: &animus_adapters::ToolRequest,
    ) -> Result<kit::ToolExecutorOutput, kit::ToolExecutorServiceFailure> {
        self.call_count = self.call_count.saturating_add(1);
        self.log.borrow_mut().push("executor");
        Ok(kit::ToolExecutorOutput::new(
            self.output.as_str(),
            None,
            animus_adapters::tool::OutputRedactionClass::Safe,
        ))
    }
}

struct ExactPreAdmission {
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl ExactPreAdmission {
    fn new(log: Rc<RefCell<Vec<&'static str>>>) -> Self {
        Self { log }
    }
}

impl ToolPreAdmissionStageService for ExactPreAdmission {
    fn evaluate(
        &mut self,
        input: ToolPreAdmissionStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue> {
        self.log.borrow_mut().push("pre_admission");
        Ok(stages::exact_tool_execution_stage_candidate(
            input.plan,
            input.state,
            stages::ToolExecutionStagePhase::PreAdmission,
            stages::ToolExecutionStageDisposition::Admitted,
        ))
    }
}

struct ExactPreExecution {
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl ExactPreExecution {
    fn new(log: Rc<RefCell<Vec<&'static str>>>) -> Self {
        Self { log }
    }
}

impl ToolPreExecutionStageService for ExactPreExecution {
    fn evaluate(
        &mut self,
        input: ToolPreExecutionStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue> {
        self.log.borrow_mut().push("declared_wrapper");
        Ok(stages::exact_tool_execution_stage_candidate(
            input.plan,
            input.state,
            stages::ToolExecutionStagePhase::PreExecution,
            stages::ToolExecutionStageDisposition::Admitted,
        ))
    }
}

struct RedactingOutput {
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl RedactingOutput {
    fn new(log: Rc<RefCell<Vec<&'static str>>>) -> Self {
        Self { log }
    }
}

impl ToolOutputNarrowingStageService for RedactingOutput {
    fn narrow(
        &mut self,
        input: ToolOutputNarrowingStageInput<'_>,
    ) -> Result<ToolOutputNarrowingStageResult, ToolExecutionStageHostIssue> {
        self.log.borrow_mut().push("output_narrowing");
        let candidate = candidate_with_redaction(input.plan, input.state);
        Ok(ToolOutputNarrowingStageResult {
            candidate,
            output: kit::ToolExecutorOutput::new(
                SAFE_REPLACEMENT,
                None,
                animus_adapters::tool::OutputRedactionClass::Redacted,
            ),
            truncated: false,
        })
    }
}

struct ExactPostCommit {
    log: Rc<RefCell<Vec<&'static str>>>,
}

impl ExactPostCommit {
    fn new(log: Rc<RefCell<Vec<&'static str>>>) -> Self {
        Self { log }
    }
}

impl ToolPostCommitStageService for ExactPostCommit {
    fn observe(
        &mut self,
        input: ToolPostCommitStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue> {
        self.log.borrow_mut().push("post_commit");
        Ok(stages::exact_tool_execution_stage_candidate(
            input.plan,
            input.state,
            stages::ToolExecutionStagePhase::PostCommitObservation,
            stages::ToolExecutionStageDisposition::Admitted,
        ))
    }
}

struct SubstitutedPreAdmission;

impl ToolPreAdmissionStageService for SubstitutedPreAdmission {
    fn evaluate(
        &mut self,
        input: ToolPreAdmissionStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue> {
        let capabilities = input.plan.capability_labels().to_vec();
        stages::build_tool_execution_stage_candidate(stages::ToolExecutionStageCandidateDraft {
            phase: stages::ToolExecutionStagePhase::PreAdmission,
            stage_plan_identity: input.plan.plan_identity(),
            handler_set_identity: hash("substituted-handler-set").as_str(),
            request_identity: input.plan.request_identity(),
            schema_identity: input.plan.schema_identity(),
            executor_binding: input.plan.executor_binding(),
            authority_seal_identity: input.plan.authority_seal_identity(),
            capability_labels: capabilities.as_slice(),
            mutation_targets: input.plan.mutation_targets(),
            mutation_posture: input.plan.mutation_posture(),
            max_visible_chars: input
                .plan
                .output_policy()
                .map(kit::ToolExecutorOutputPolicy::max_visible_chars),
            max_output_bytes: input
                .plan
                .output_policy()
                .map(kit::ToolExecutorOutputPolicy::max_output_bytes),
            required_redaction: false,
            disposition: stages::ToolExecutionStageDisposition::Admitted,
            outcome_class: stages::ToolExecutionStageOutcomeClass::Pending,
        })
        .map_err(|_error| ToolExecutionStageHostIssue::HandlerFailed)
    }
}

struct GrowingOutput;

impl ToolOutputNarrowingStageService for GrowingOutput {
    fn narrow(
        &mut self,
        input: ToolOutputNarrowingStageInput<'_>,
    ) -> Result<ToolOutputNarrowingStageResult, ToolExecutionStageHostIssue> {
        Ok(ToolOutputNarrowingStageResult {
            candidate: stages::exact_tool_execution_stage_candidate(
                input.plan,
                input.state,
                stages::ToolExecutionStagePhase::OutputNarrowing,
                stages::ToolExecutionStageDisposition::Admitted,
            ),
            output: kit::ToolExecutorOutput::new(
                "safe output that is larger than the original",
                None,
                animus_adapters::tool::OutputRedactionClass::Safe,
            ),
            truncated: false,
        })
    }
}

fn candidate_with_redaction(
    plan: &stages::ToolExecutionStagePlan,
    state: &stages::ToolExecutionStageState,
) -> stages::ToolExecutionStageCandidate {
    let handler_set_identity = stages::tool_execution_stage_handler_set_identity(
        plan,
        stages::ToolExecutionStagePhase::OutputNarrowing,
    );
    stages::build_tool_execution_stage_candidate(stages::ToolExecutionStageCandidateDraft {
        phase: stages::ToolExecutionStagePhase::OutputNarrowing,
        stage_plan_identity: plan.plan_identity(),
        handler_set_identity: handler_set_identity.as_str(),
        request_identity: plan.request_identity(),
        schema_identity: plan.schema_identity(),
        executor_binding: plan.executor_binding(),
        authority_seal_identity: plan.authority_seal_identity(),
        capability_labels: plan.capability_labels(),
        mutation_targets: plan.mutation_targets(),
        mutation_posture: plan.mutation_posture(),
        max_visible_chars: plan
            .output_policy()
            .map(kit::ToolExecutorOutputPolicy::max_visible_chars),
        max_output_bytes: plan
            .output_policy()
            .map(kit::ToolExecutorOutputPolicy::max_output_bytes),
        required_redaction: true,
        disposition: stages::ToolExecutionStageDisposition::Admitted,
        outcome_class: state.outcome_class(),
    })
    .expect("redaction candidate")
}

fn commit_observation_from_pending(
    pending: &PendingStagedToolExecution,
    durable_head_identity: String,
) -> stages::ToolExecutionStageCommitObservation {
    stages::ToolExecutionStageCommitObservation {
        stage_plan_identity: String::from(pending.plan().plan_identity()),
        result_identity: pending
            .state()
            .result_identity()
            .map(String::from)
            .expect("final result identity"),
        commit_owner_identity: String::from(pending.plan().commit_owner_identity()),
        durable_head_identity,
        complete: true,
        synchronized: true,
    }
}

fn handlers() -> Vec<stages::ToolExecutionStageHandler> {
    [
        (stages::ToolExecutionStagePhase::PreAdmission, "pre"),
        (stages::ToolExecutionStagePhase::PreExecution, "wrapper"),
        (stages::ToolExecutionStagePhase::OutputNarrowing, "output"),
        (
            stages::ToolExecutionStagePhase::PostCommitObservation,
            "post",
        ),
    ]
    .into_iter()
    .map(|(phase, label)| {
        stages::validate_tool_execution_stage_handler(stages::ToolExecutionStageHandlerDraft {
            phase,
            handler_identity: hash(label).as_str(),
            cohort_identity: hash("cohort").as_str(),
        })
        .expect("handler")
    })
    .collect()
}

fn catalog() -> kit::ToolCatalog {
    let metadata =
        animus_adapters::tool::Definition::validate(&animus_adapters::tool::DefinitionDraft {
            name: Some(TOOL_NAME),
            label: Some("Lookup"),
            description: Some("Reads one deterministic fixture."),
            parameter_schema_identity: Some(TOOL_SCHEMA),
            prompt_snippet: Some("Use lookup for fixture reads."),
            prompt_guidelines: &["Supply one key."],
            source_identity: Some("tool-stage-host-tests"),
            provenance: Some("maintained-test"),
            capability_labels: &[TOOL_CAPABILITY],
            result_metadata_shape: Some("bounded-text.v1"),
            status: animus_adapters::tool::DefinitionStatus::Active,
        })
        .expect("metadata");
    let definition = kit::ToolDefinition::from_draft(kit::ToolDefinitionDraft {
        metadata,
        executor_binding: EXECUTOR_BINDING,
        argument_names: &[ARGUMENT_NAME],
        mutation_posture: kit::ToolMutationPosture::ReadOnly,
        output_policy: kit::ToolExecutorOutputPolicy::new(
            OUTPUT_VISIBLE_CHARS,
            OUTPUT_BYTES,
            false,
        )
        .expect("output policy"),
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved,
    })
    .expect("definition");
    kit::ToolCatalog::new(vec![definition]).expect("catalog")
}

fn capabilities() -> animus_runtime::RuntimeCapabilityRegistry {
    animus_runtime::RuntimeCapabilityRegistry::from_drafts(&[
        animus_runtime::RuntimeCapabilityDescriptorDraft {
            label: TOOL_CAPABILITY,
            service_kind: "tool",
            scope: None,
        },
    ])
    .expect("capabilities")
}

fn request() -> animus_adapters::ToolRequest {
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: TOOL_SCHEMA,
            allowed_field_names: &[ARGUMENT_NAME],
            compatibility_rules: &[],
            fields: &[animus_adapters::tool::ArgumentFieldDraft {
                name: Some(ARGUMENT_NAME),
                value: Some("alpha"),
            }],
        },
    )
    .expect("arguments");
    animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some("tool-call-1"),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: OUTPUT_VISIBLE_CHARS,
        },
        mutation_targets: &[],
        required_capability_labels: &[TOOL_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .expect("request")
}

fn hash(label: &str) -> String {
    blake3::hash(label.as_bytes()).to_hex().to_string()
}
