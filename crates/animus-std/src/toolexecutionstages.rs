//! Ordered injected shell for fixed tool-execution stages.
//!
//! This shell composes host-selected stage services around the existing
//! `tool_executor_kit`. It does not discover handlers, grant authority, persist
//! results, or continue a provider by itself.

use animus_runtime::tool_execution_stages as stages;
use animus_runtime::tool_executor_kit as kit;

const TRANSIENT_OUTPUT_IDENTITY_DOMAIN: &str = "animus.tool-execution-stages.transient-output.v1";

/// Safe host issue for staged tool execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToolExecutionStageHostIssue {
    /// The pure stage core rejected stale, widened, or reordered facts.
    CoreRejected,
    /// One injected phase handler failed without raw diagnostics.
    HandlerFailed,
    /// Existing executor-kit admission, execution, or final validation failed.
    ExecutorRejected,
    /// Exact durable result commit or post-commit observation failed.
    CommitRejected,
}

/// Safe staged tool host error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolExecutionStageHostError {
    issue: ToolExecutionStageHostIssue,
}

impl ToolExecutionStageHostError {
    /// Returns the safe issue class.
    #[must_use]
    pub const fn issue(self) -> ToolExecutionStageHostIssue {
        self.issue
    }

    const fn new(issue: ToolExecutionStageHostIssue) -> Self {
        Self { issue }
    }
}

impl core::fmt::Display for ToolExecutionStageHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "staged tool execution rejected: {:?}",
            self.issue
        )
    }
}

impl std::error::Error for ToolExecutionStageHostError {}

/// Phase-specific private input for pre-admission handlers.
pub struct ToolPreAdmissionStageInput<'input> {
    /// Immutable stage plan.
    pub plan: &'input stages::ToolExecutionStagePlan,
    /// Current pure stage state.
    pub state: &'input stages::ToolExecutionStageState,
    /// Exact private tool request.
    pub request: &'input animus_adapters::ToolRequest,
}

/// Phase-specific private input for pre-execution handlers.
pub struct ToolPreExecutionStageInput<'input> {
    /// Immutable stage plan.
    pub plan: &'input stages::ToolExecutionStagePlan,
    /// Current pure stage state.
    pub state: &'input stages::ToolExecutionStageState,
    /// Exact private tool request.
    pub request: &'input animus_adapters::ToolRequest,
}

/// Phase-specific private input for output-narrowing handlers.
pub struct ToolOutputNarrowingStageInput<'input> {
    /// Immutable stage plan.
    pub plan: &'input stages::ToolExecutionStagePlan,
    /// Current pure stage state.
    pub state: &'input stages::ToolExecutionStageState,
    /// Exact transient executor output. Public evidence must never contain it.
    pub output: &'input kit::ToolExecutorOutput,
}

/// Output and exact candidate returned by one output-narrowing handler cohort.
pub struct ToolOutputNarrowingStageResult {
    /// Exact phase candidate.
    pub candidate: stages::ToolExecutionStageCandidate,
    /// Narrowed transient output for final executor-kit validation.
    pub output: kit::ToolExecutorOutput,
    /// Whether the handler intentionally produced a bounded truncation.
    pub truncated: bool,
}

/// Metadata-only post-commit handler input.
pub struct ToolPostCommitStageInput<'input> {
    /// Immutable stage plan.
    pub plan: &'input stages::ToolExecutionStagePlan,
    /// State after exact durable commit.
    pub state: &'input stages::ToolExecutionStageState,
    /// Safe exact durable commit observation.
    pub commit: &'input stages::ToolExecutionStageCommitObservation,
}

/// Host-injected pre-admission handler cohort.
pub trait ToolPreAdmissionStageService {
    /// Evaluates ordered selected pre-admission handlers.
    ///
    /// # Errors
    ///
    /// Returns a safe issue without raw arguments or diagnostics.
    fn evaluate(
        &mut self,
        input: ToolPreAdmissionStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue>;
}

/// Host-injected pre-execution handler cohort.
pub trait ToolPreExecutionStageService {
    /// Evaluates ordered selected wrappers or denials.
    ///
    /// # Errors
    ///
    /// Returns a safe issue without raw arguments or diagnostics.
    fn evaluate(
        &mut self,
        input: ToolPreExecutionStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue>;
}

/// Host-injected output-narrowing handler cohort.
pub trait ToolOutputNarrowingStageService {
    /// Redacts, truncates, classifies, or safely replaces transient output.
    ///
    /// # Errors
    ///
    /// Returns a safe issue without raw output or diagnostics.
    fn narrow(
        &mut self,
        input: ToolOutputNarrowingStageInput<'_>,
    ) -> Result<ToolOutputNarrowingStageResult, ToolExecutionStageHostIssue>;
}

/// Host-injected metadata-only post-commit observer cohort.
pub trait ToolPostCommitStageService {
    /// Observes safe metadata only after the exact durable commit.
    ///
    /// # Errors
    ///
    /// Returns a safe issue without committed result content.
    fn observe(
        &mut self,
        input: ToolPostCommitStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue>;
}

/// Explicit no-handler implementation for every extensible stage.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoToolExecutionStageHandlers;

impl ToolPreAdmissionStageService for NoToolExecutionStageHandlers {
    fn evaluate(
        &mut self,
        input: ToolPreAdmissionStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue> {
        Ok(stages::exact_tool_execution_stage_candidate(
            input.plan,
            input.state,
            stages::ToolExecutionStagePhase::PreAdmission,
            stages::ToolExecutionStageDisposition::Admitted,
        ))
    }
}

impl ToolPreExecutionStageService for NoToolExecutionStageHandlers {
    fn evaluate(
        &mut self,
        input: ToolPreExecutionStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue> {
        Ok(stages::exact_tool_execution_stage_candidate(
            input.plan,
            input.state,
            stages::ToolExecutionStagePhase::PreExecution,
            stages::ToolExecutionStageDisposition::Admitted,
        ))
    }
}

impl ToolOutputNarrowingStageService for NoToolExecutionStageHandlers {
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
            output: input.output.clone(),
            truncated: false,
        })
    }
}

impl ToolPostCommitStageService for NoToolExecutionStageHandlers {
    fn observe(
        &mut self,
        input: ToolPostCommitStageInput<'_>,
    ) -> Result<stages::ToolExecutionStageCandidate, ToolExecutionStageHostIssue> {
        Ok(stages::exact_tool_execution_stage_candidate(
            input.plan,
            input.state,
            stages::ToolExecutionStagePhase::PostCommitObservation,
            stages::ToolExecutionStageDisposition::Admitted,
        ))
    }
}

/// Exact host-selected phase services.
pub struct ToolExecutionStageServices<PreAdmission, PreExecution, Output, PostCommit> {
    /// Ordered pre-admission handler cohort.
    pub pre_admission: PreAdmission,
    /// Ordered pre-execution handler cohort.
    pub pre_execution: PreExecution,
    /// Ordered output-narrowing handler cohort.
    pub output: Output,
    /// Metadata-only post-commit observer cohort.
    pub post_commit: PostCommit,
}

/// Borrowed complete inputs for one staged execution.
pub struct StagedToolExecutionInput<'input> {
    /// Existing authoritative executor-kit plan inputs.
    pub tool: kit::ToolExecutionPlanRequest<'input>,
    /// Exact BLAKE3 runtime-authority seal identity.
    pub authority_seal_identity: &'input str,
    /// Exact BLAKE3 coding-session commit-owner identity.
    pub commit_owner_identity: &'input str,
    /// Ordered selected handler descriptors.
    pub handlers: &'input [stages::ToolExecutionStageHandler],
}

/// Final-validated output waiting for exact durable commit.
pub struct PendingStagedToolExecution {
    plan: stages::ToolExecutionStagePlan,
    state: stages::ToolExecutionStageState,
    outcome: kit::ToolExecutionOutcome,
}

impl core::fmt::Debug for PendingStagedToolExecution {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("PendingStagedToolExecution")
            .field("stage_plan_identity", &self.plan.plan_identity())
            .field("status", &self.state.status())
            .finish_non_exhaustive()
    }
}

impl PendingStagedToolExecution {
    /// Returns the immutable stage plan.
    #[must_use]
    pub const fn plan(&self) -> &stages::ToolExecutionStagePlan {
        &self.plan
    }

    /// Returns the final-validated state waiting for commit.
    #[must_use]
    pub const fn state(&self) -> &stages::ToolExecutionStageState {
        &self.state
    }

    /// Returns the exact bounded tool outcome.
    #[must_use]
    pub const fn outcome(&self) -> &kit::ToolExecutionOutcome {
        &self.outcome
    }
}

/// Fully committed and observed staged execution.
pub struct CompletedStagedToolExecution {
    /// Exact bounded tool outcome.
    pub outcome: kit::ToolExecutionOutcome,
    /// Metadata-only terminal stage receipt.
    pub receipt: stages::ToolExecutionStageReceipt,
}

impl core::fmt::Debug for CompletedStagedToolExecution {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("CompletedStagedToolExecution")
            .field("stage_plan_identity", &self.receipt.stage_plan_identity())
            .field("status", &self.receipt.status())
            .finish_non_exhaustive()
    }
}

// r[impl agent_harness_runtime.tool_execution_stages.binding]
// r[impl agent_harness_runtime.tool_execution_stages.output]
/// Executes through all phases up to the coding-session durable commit barrier.
///
/// `plan_tool_execution` remains final admission. `invoke_admitted_tool` remains
/// the only executor call. `validate_tool_executor_output` remains final output
/// and executor-receipt validation.
///
/// # Errors
///
/// Returns a safe issue for handler failure, stage drift, denied final
/// admission, executor failure, unsafe output, or final-validation failure.
pub fn execute_staged_tool<Executor, PreAdmission, PreExecution, Output, PostCommit>(
    executor: &mut Executor,
    services: &mut ToolExecutionStageServices<PreAdmission, PreExecution, Output, PostCommit>,
    input: &StagedToolExecutionInput<'_>,
) -> Result<PendingStagedToolExecution, ToolExecutionStageHostError>
where
    Executor: kit::ToolExecutorService,
    PreAdmission: ToolPreAdmissionStageService,
    PreExecution: ToolPreExecutionStageService,
    Output: ToolOutputNarrowingStageService,
{
    let PreparedStageExecution {
        tool_plan,
        plan,
        mut state,
    } = prepare_staged_execution(services, input)?;
    let raw_output = kit::invoke_admitted_tool(executor, &tool_plan, input.tool.request)
        .map_err(|_error| executor_error())?;
    let raw_identity = transient_output_identity(&raw_output);
    state = stages::observe_tool_stage_execution(
        &plan,
        &state,
        &stages::ToolExecutionStageRawObservation {
            stage_plan_identity: String::from(plan.plan_identity()),
            request_identity: String::from(plan.request_identity()),
            output_identity: raw_identity.clone(),
            outcome_class: stages::ToolExecutionStageOutcomeClass::Succeeded,
            output_byte_count: usize_to_u64(raw_output.body().len())?,
            output_char_count: usize_to_u64(raw_output.body().chars().count())?,
            required_redaction: raw_output.redaction_class()
                != animus_adapters::tool::OutputRedactionClass::Safe,
        },
    )
    .map_err(|_error| core_error())?;
    let narrowed = services
        .output
        .narrow(ToolOutputNarrowingStageInput {
            plan: &plan,
            state: &state,
            output: &raw_output,
        })
        .map_err(ToolExecutionStageHostError::new)?;
    let narrowed_identity = transient_output_identity(&narrowed.output);
    state = stages::apply_tool_output_narrowing(
        &plan,
        &state,
        &stages::ToolExecutionStageOutputObservation {
            candidate: narrowed.candidate,
            input_output_identity: raw_identity,
            narrowed_output_identity: narrowed_identity.clone(),
            output_byte_count: usize_to_u64(narrowed.output.body().len())?,
            output_char_count: usize_to_u64(narrowed.output.body().chars().count())?,
            truncated: narrowed.truncated,
        },
    )
    .map_err(|_error| core_error())?;
    ensure_active(&state)?;
    let outcome = kit::validate_tool_executor_output(&tool_plan, &narrowed.output)
        .map_err(|_error| executor_error())?;
    let result_identity = outcome
        .response()
        .content_hash()
        .ok_or_else(executor_error)?;
    let is_truncated =
        outcome.response().status() == animus_adapters::tool::OutputStatus::Truncated;
    state = stages::observe_tool_final_validation(
        &plan,
        &state,
        &stages::ToolExecutionStageFinalObservation {
            stage_plan_identity: String::from(plan.plan_identity()),
            narrowed_output_identity: narrowed_identity,
            result_identity: String::from(result_identity),
            executor_receipt_identity: String::from(outcome.receipt().receipt_hash()),
            outcome_class: stages::ToolExecutionStageOutcomeClass::Succeeded,
            truncated: is_truncated,
        },
    )
    .map_err(|_error| core_error())?;
    Ok(PendingStagedToolExecution {
        plan,
        state,
        outcome,
    })
}

struct PreparedStageExecution {
    tool_plan: kit::ToolExecutionPlan,
    plan: stages::ToolExecutionStagePlan,
    state: stages::ToolExecutionStageState,
}

fn prepare_staged_execution<PreAdmission, PreExecution, Output, PostCommit>(
    services: &mut ToolExecutionStageServices<PreAdmission, PreExecution, Output, PostCommit>,
    input: &StagedToolExecutionInput<'_>,
) -> Result<PreparedStageExecution, ToolExecutionStageHostError>
where
    PreAdmission: ToolPreAdmissionStageService,
    PreExecution: ToolPreExecutionStageService,
{
    let tool_plan = kit::plan_tool_execution(input.tool);
    let plan = stages::plan_tool_execution_stages(stages::ToolExecutionStagePlanDraft {
        phase_order: &stages::TOOL_EXECUTION_STAGE_PHASES,
        handlers: input.handlers,
        tool_plan: &tool_plan,
        request: input.tool.request,
        authority_seal_identity: input.authority_seal_identity,
        commit_owner_identity: input.commit_owner_identity,
    })
    .map_err(|_error| core_error())?;
    let mut state = stages::start_tool_execution_stages(&plan);
    let candidate = services
        .pre_admission
        .evaluate(ToolPreAdmissionStageInput {
            plan: &plan,
            state: &state,
            request: input.tool.request,
        })
        .map_err(ToolExecutionStageHostError::new)?;
    state = stages::apply_tool_pre_admission_candidate(&plan, &state, &candidate)
        .map_err(|_error| core_error())?;
    ensure_active(&state)?;
    state = stages::apply_tool_final_admission(&plan, &state, &tool_plan)
        .map_err(|_error| core_error())?;
    ensure_active(&state)?;
    let candidate = services
        .pre_execution
        .evaluate(ToolPreExecutionStageInput {
            plan: &plan,
            state: &state,
            request: input.tool.request,
        })
        .map_err(ToolExecutionStageHostError::new)?;
    state = stages::apply_tool_pre_execution_candidate(&plan, &state, &candidate)
        .map_err(|_error| core_error())?;
    ensure_active(&state)?;
    Ok(PreparedStageExecution {
        tool_plan,
        plan,
        state,
    })
}

// r[impl agent_harness_runtime.tool_execution_stages.commit]
/// Completes exact durable commit and metadata-only post-commit observation.
///
/// # Errors
///
/// Returns a safe issue when commit correlation, synchronization, post-commit
/// handler execution, or terminal receipt projection fails.
pub fn complete_staged_tool_commit<PostCommit>(
    post_commit: &mut PostCommit,
    pending: PendingStagedToolExecution,
    observation: &stages::ToolExecutionStageCommitObservation,
) -> Result<CompletedStagedToolExecution, ToolExecutionStageHostError>
where
    PostCommit: ToolPostCommitStageService,
{
    let committed =
        stages::observe_tool_stage_durable_commit(&pending.plan, &pending.state, observation)
            .map_err(|_error| commit_error())?;
    let candidate = post_commit
        .observe(ToolPostCommitStageInput {
            plan: &pending.plan,
            state: &committed,
            commit: observation,
        })
        .map_err(ToolExecutionStageHostError::new)?;
    let observed = stages::apply_tool_post_commit_candidate(&pending.plan, &committed, &candidate)
        .map_err(|_error| commit_error())?;
    let receipt = stages::project_tool_execution_stage_receipt(
        &pending.plan,
        &observed,
        stages::ToolExecutionStageSensitiveDraft::default(),
    )
    .map_err(|_error| commit_error())?;
    Ok(CompletedStagedToolExecution {
        outcome: pending.outcome,
        receipt,
    })
}

fn ensure_active(
    state: &stages::ToolExecutionStageState,
) -> Result<(), ToolExecutionStageHostError> {
    if state.status() == stages::ToolExecutionStageStatus::Denied {
        Err(core_error())
    } else {
        Ok(())
    }
}

fn transient_output_identity(output: &kit::ToolExecutorOutput) -> String {
    let mut hasher = blake3::Hasher::new();
    hash_field(&mut hasher, TRANSIENT_OUTPUT_IDENTITY_DOMAIN.as_bytes());
    hash_field(&mut hasher, output.body().as_bytes());
    hash_field(
        &mut hasher,
        output.full_output_reference().unwrap_or("none").as_bytes(),
    );
    hash_field(&mut hasher, output.redaction_class().as_str().as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn hash_field(hasher: &mut blake3::Hasher, field: &[u8]) {
    let length = u64::try_from(field.len()).unwrap_or(u64::MAX);
    hasher.update(&length.to_le_bytes());
    hasher.update(field);
}

fn usize_to_u64(value: usize) -> Result<u64, ToolExecutionStageHostError> {
    u64::try_from(value).map_err(|_overflow| executor_error())
}

const fn core_error() -> ToolExecutionStageHostError {
    ToolExecutionStageHostError::new(ToolExecutionStageHostIssue::CoreRejected)
}

const fn executor_error() -> ToolExecutionStageHostError {
    ToolExecutionStageHostError::new(ToolExecutionStageHostIssue::ExecutorRejected)
}

const fn commit_error() -> ToolExecutionStageHostError {
    ToolExecutionStageHostError::new(ToolExecutionStageHostIssue::CommitRejected)
}

#[cfg(test)]
#[path = "toolexecutionstages/tests.rs"]
mod tests;
