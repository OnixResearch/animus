//! Typed change-scope handoff around developer review ownership.
//!
//! The module is a deterministic functional core. It does not observe files,
//! present UIs, mutate VCS state, enforce policy, or grant product authority.

mod binding;
mod core;
mod data;
mod digest;
mod evidence;

pub use binding::validate_change_scope;
pub use core::{
    evaluate_change_handoff_completion, open_change_handoff, plan_change_continuation,
    record_change_handoff_decision, validate_change_handoff_artifacts,
    validate_change_handoff_claims,
};
pub use data::{
    CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY, CHANGE_SCOPE_HANDOFF_NON_CLAIMS,
    CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA, CHANGE_SCOPE_HANDOFF_REDACTION_MARKER,
    ChangeContinuationPlan, ChangeHandoffArtifact, ChangeHandoffArtifactDraft,
    ChangeHandoffArtifactKind, ChangeHandoffCompletion, ChangeHandoffCompletionBlock,
    ChangeHandoffDecisionDraft, ChangeHandoffDecisionKind, ChangeHandoffError, ChangeHandoffIssue,
    ChangeHandoffReceipt, ChangeHandoffState, ChangeHandoffStatus, ChangeScope, ChangeScopeDraft,
    ChangeScopeHunk, ChangeScopeHunkDraft, FORBIDDEN_HANDOFF_CLAIM_LABELS,
    RecordedChangeHandoffDecision, artifact_declared_byte_bound, scope_artifact_bound,
    scope_file_bound, scope_hunk_bound, scope_label_char_bound, scope_obligation_bound,
};
pub use evidence::{project_change_handoff_receipt, validate_change_handoff_receipt};

#[cfg(test)]
mod tests;
