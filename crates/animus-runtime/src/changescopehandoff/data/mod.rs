//! Public bounds, issue classes, and immutable handoff facts.

mod artifacts;
mod facts;
mod judgment;

pub use artifacts::{
    ChangeHandoffArtifact, ChangeHandoffArtifactDraft, ChangeHandoffArtifactKind,
    ChangeHandoffReceipt,
};
pub use facts::{ChangeScope, ChangeScopeDraft, ChangeScopeHunk, ChangeScopeHunkDraft};
pub use judgment::{
    ChangeContinuationPlan, ChangeHandoffCompletion, ChangeHandoffCompletionBlock,
    ChangeHandoffDecisionDraft, ChangeHandoffDecisionKind, ChangeHandoffState, ChangeHandoffStatus,
    RecordedChangeHandoffDecision,
};

/// Stable contract identity for agent change-scope handoffs.
pub const CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY: &str = "animus.change-scope-handoff.v1";
/// Stable public receipt schema for agent change-scope handoffs.
pub const CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA: &str = "animus.change-scope-handoff-receipt.v1";
/// Marker proving that raw change content is absent from public handoff evidence.
pub const CHANGE_SCOPE_HANDOFF_REDACTION_MARKER: &str = "raw-change-values-omitted";
const SCOPE_FILE_BOUND: u32 = 64;
const SCOPE_HUNK_BOUND: u32 = 256;
const SCOPE_OBLIGATION_BOUND: u32 = 32;
const SCOPE_LABEL_CHAR_BOUND: u32 = 128;
const SCOPE_ARTIFACT_BOUND: u32 = 32;
const ARTIFACT_DECLARED_BYTE_BOUND: u64 = 4096;
const SCOPE_NON_CLAIM_COUNT: usize = 6;
/// Explicit bounded non-claims for public handoff evidence.
pub const CHANGE_SCOPE_HANDOFF_NON_CLAIMS: [&str; SCOPE_NON_CLAIM_COUNT] = [
    "not-change-correctness",
    "not-vcs-completion",
    "not-policy-enforcement",
    "not-tool-correctness",
    "not-production-readiness",
    "not-release-eligibility",
];
/// Fixed claim labels that handoff evidence must never carry.
pub const FORBIDDEN_HANDOFF_CLAIM_LABELS: [&str; SCOPE_NON_CLAIM_COUNT] = [
    "change-correctness",
    "vcs-completion",
    "policy-enforcement",
    "tool-correctness",
    "production-readiness",
    "release-eligibility",
];
/// Returns the named declared-file bound for one change scope.
#[must_use]
pub const fn scope_file_bound() -> u32 {
    SCOPE_FILE_BOUND
}
/// Returns the named declared-hunk bound for one change scope.
#[must_use]
pub const fn scope_hunk_bound() -> u32 {
    SCOPE_HUNK_BOUND
}
/// Returns the named validation-obligation bound for one change scope.
#[must_use]
pub const fn scope_obligation_bound() -> u32 {
    SCOPE_OBLIGATION_BOUND
}
/// Returns the label bound. Scope admission measures bytes, receipt labels use characters.
#[must_use]
pub const fn scope_label_char_bound() -> u32 {
    SCOPE_LABEL_CHAR_BOUND
}
/// Returns the named review-artifact count bound for one handoff.
#[must_use]
pub const fn scope_artifact_bound() -> u32 {
    SCOPE_ARTIFACT_BOUND
}
/// Returns the named declared-byte bound for one review artifact.
#[must_use]
pub const fn artifact_declared_byte_bound() -> u64 {
    ARTIFACT_DECLARED_BYTE_BOUND
}

/// Fail-closed issue classes for the change-scope handoff rail.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeHandoffIssue {
    /// The driving task label was empty.
    MissingDrivingTask,
    /// A file or obligation label was empty.
    EmptyScopeLabel,
    /// The declared file set was empty or over bound.
    MissingScopeFiles,
    /// The declared hunk set was empty or over bound.
    MissingScopeHunks,
    /// The validation-obligation set was empty or over bound.
    MissingScopeObligations,
    /// A hunk referenced a file that the scope does not declare.
    UndeclaredScopeFile,
    /// One required BLAKE3 identity or its encoding was malformed.
    InvalidIdentity,
    /// One scope fact label exceeded its named bound.
    LabelBoundExceeded,
    /// A proposed decision referenced a different change scope.
    ScopeIdentityMismatch,
    /// A second decision tried to replace a recorded decision.
    DecisionAlreadyRecorded,
    /// The approved hunk subset was malformed for the decision kind.
    InvalidApprovedSubset,
    /// Continuation or completion ran before a recorded decision.
    DecisionMissing,
    /// An artifact declared raw transcript, tool-output, or prompt payload.
    RawArtifactKind,
    /// The artifact set exceeded the named count or byte bounds.
    ArtifactBoundExceeded,
    /// Proposed evidence labels carried a forbidden ownership claim.
    NonClaimViolation,
    /// A projected receipt field or identity did not match its inputs.
    ReceiptFieldMismatch,
}

/// Safe change-scope handoff error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChangeHandoffError {
    issue: ChangeHandoffIssue,
}

impl ChangeHandoffError {
    /// Returns the fail-closed issue class.
    #[must_use]
    pub const fn issue(self) -> ChangeHandoffIssue {
        self.issue
    }

    pub(super) const fn new(issue: ChangeHandoffIssue) -> Self {
        Self { issue }
    }
}
