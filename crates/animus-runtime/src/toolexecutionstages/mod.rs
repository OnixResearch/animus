//! Fixed, monotonic tool-execution stages around existing authority owners.
//!
//! The module is a deterministic functional core. It does not discover
//! handlers, execute tools, persist results, call providers, or grant authority.

mod core;
mod data;
mod receipt;

pub use core::{
    apply_tool_final_admission, apply_tool_output_narrowing, apply_tool_post_commit_candidate,
    apply_tool_pre_admission_candidate, apply_tool_pre_execution_candidate,
    build_tool_execution_stage_candidate, coding_session_tool_commit_owner_identity,
    exact_tool_execution_stage_candidate, observe_tool_final_validation,
    observe_tool_stage_durable_commit, observe_tool_stage_execution, plan_tool_execution_stages,
    start_tool_execution_stages, tool_execution_stage_handler_set_identity,
    validate_tool_execution_stage_handler,
};
pub use data::{
    TOOL_EXECUTION_STAGE_CONTRACT_IDENTITY, TOOL_EXECUTION_STAGE_NON_CLAIMS,
    TOOL_EXECUTION_STAGE_PHASES, TOOL_EXECUTION_STAGE_RECEIPT_SCHEMA,
    TOOL_EXECUTION_STAGE_REDACTION_MARKER, ToolExecutionStageCandidate,
    ToolExecutionStageCandidateDraft, ToolExecutionStageCommitObservation,
    ToolExecutionStageDisposition, ToolExecutionStageError, ToolExecutionStageFinalObservation,
    ToolExecutionStageHandler, ToolExecutionStageHandlerDraft, ToolExecutionStageIssue,
    ToolExecutionStageOutcomeClass, ToolExecutionStageOutputObservation, ToolExecutionStagePhase,
    ToolExecutionStagePlan, ToolExecutionStagePlanDraft, ToolExecutionStageRawObservation,
    ToolExecutionStageReceipt, ToolExecutionStageSensitiveDraft, ToolExecutionStageState,
    ToolExecutionStageStatus,
};
pub use receipt::{project_tool_execution_stage_receipt, validate_tool_execution_stage_receipt};

#[cfg(test)]
mod tests;
