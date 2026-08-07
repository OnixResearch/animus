//! Pure, VCS-neutral rejected-attempt context planning.
//!
//! Hosts supply validated review, current VCS, redaction, and classification
//! facts. This module does not inspect repositories, read files, build prompts,
//! create runs, mutate tasks, or grant retry or completion authority.

mod core;
mod data;

pub use core::{
    plan_rejected_attempt_context, project_rejected_attempt_receipt,
    validate_rejected_attempt_receipt,
};
pub use data::{
    AttemptIdentity, ContentIdentity, RawDiffDisposition, ReceiptIdentity,
    RejectedAttemptContextPlan, RejectedAttemptIssue, RejectedAttemptPolicy,
    RejectedAttemptReceipt, RejectedAttemptRequest, RejectionContextFacts, ReviewDisposition,
    ReviewEvidenceProfile, ReviewHandoffFacts, ReviewHandoffIdentity, RevisionIdentity,
    RunIdentity, SummaryIdentity, TaskIdentity, VcsAttemptFacts, VcsProvider,
    WorkspaceSnapshotIdentity,
};

#[cfg(test)]
mod tests;
