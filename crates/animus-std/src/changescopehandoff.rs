//! Review-ownership shell for explicit agent change-scope handoffs.
//!
//! This shell presents one validated change scope to a host-injected review
//! service and records exactly the decision that service returns. It does not
//! observe files, mutate VCS state, invent decisions, enforce policy, or grant
//! product authority.

use animus_runtime::change_scope_handoff as handoff;

/// Safe host issue for change-scope handoff review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeHandoffReviewIssue {
    /// The pure handoff core rejected the presented or returned facts.
    CoreRejected,
    /// The injected review service produced no decision.
    ReviewUnavailable,
    /// Completion ran before any review decision was recorded.
    DecisionMissing,
    /// Completion ran on a recorded decision that is not an acceptance.
    DecisionNotAcceptance,
}

/// Safe change-scope handoff review error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChangeHandoffReviewError {
    issue: ChangeHandoffReviewIssue,
}

impl ChangeHandoffReviewError {
    /// Returns the safe issue class.
    #[must_use]
    pub const fn issue(self) -> ChangeHandoffReviewIssue {
        self.issue
    }

    const fn new(issue: ChangeHandoffReviewIssue) -> Self {
        Self { issue }
    }
}

impl core::fmt::Display for ChangeHandoffReviewError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "change-scope handoff review rejected: {:?}",
            self.issue
        )
    }
}

impl std::error::Error for ChangeHandoffReviewError {}

/// One presented pending scope awaiting exactly one developer decision.
pub struct ChangeHandoffScopePresentation<'input> {
    /// Pending handoff state that owns the presented scope.
    pub state: &'input handoff::ChangeHandoffState,
}

/// One developer decision returned by the injected review service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeHandoffReviewDecision {
    /// Recorded decision kind.
    pub kind: handoff::ChangeHandoffDecisionKind,
    /// Approved hunk indexes in strict ascending order; empty unless approved.
    pub approved_hunks: Vec<usize>,
}

/// Host-injected developer review service.
pub trait ChangeHandoffReviewService {
    /// Returns exactly one ownership decision for the presented scope.
    ///
    /// # Errors
    ///
    /// Returns a safe issue without raw scope content or diagnostics.
    fn decide(
        &mut self,
        presentation: ChangeHandoffScopePresentation<'_>,
    ) -> Result<ChangeHandoffReviewDecision, ChangeHandoffReviewIssue>;
}

/// Explicit no-review implementation that never invents a decision.
#[derive(Clone, Copy, Debug, Default)]
pub struct NoChangeHandoffReview;

impl ChangeHandoffReviewService for NoChangeHandoffReview {
    fn decide(
        &mut self,
        _presentation: ChangeHandoffScopePresentation<'_>,
    ) -> Result<ChangeHandoffReviewDecision, ChangeHandoffReviewIssue> {
        Err(ChangeHandoffReviewIssue::ReviewUnavailable)
    }
}

// r[impl animus.change_scope_handoff.handoff]
/// Presents one pending scope to the injected reviewer and records exactly the
/// returned decision against the presented scope identity.
///
/// The presented state is immutable, so a rejected or malformed review can
/// never mutate the caller's pending handoff.
///
/// # Errors
///
/// Returns a safe issue when the reviewer produces no decision or the pure
/// core rejects the returned decision facts.
pub fn record_presented_change_handoff_review<Review>(
    reviewer: &mut Review,
    pending: &handoff::ChangeHandoffState,
) -> Result<handoff::ChangeHandoffState, ChangeHandoffReviewError>
where
    Review: ChangeHandoffReviewService,
{
    let decision = reviewer
        .decide(ChangeHandoffScopePresentation { state: pending })
        .map_err(ChangeHandoffReviewError::new)?;
    let recorded = handoff::record_change_handoff_decision(
        pending,
        handoff::ChangeHandoffDecisionDraft {
            scope_identity: pending.scope().scope_identity(),
            kind: decision.kind,
            approved_hunks: decision.approved_hunks.as_slice(),
        },
    )
    .map_err(|_error| core_error())?;
    Ok(recorded)
}

/// One completed ownership handoff with its terminal receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CompletedChangeHandoffReview {
    /// Completion evaluation citing the recorded acceptance.
    pub completion: handoff::ChangeHandoffCompletion,
    /// Metadata-only terminal handoff receipt.
    pub receipt: handoff::ChangeHandoffReceipt,
}

// r[impl animus.change_scope_handoff.handoff]
// r[impl animus.change_scope_handoff.artifacts]
// r[impl animus.change_scope_handoff.nonclaims]
/// Completes one reviewed handoff over bounded artifacts.
///
/// Completion succeeds only for a recorded acceptance and cites its decision
/// identity; missing or non-acceptance decisions block completion.
///
/// # Errors
///
/// Returns a safe blocker issue when no acceptance is recorded and a core
/// issue when artifact or receipt facts are malformed.
pub fn complete_change_handoff_review(
    recorded: &handoff::ChangeHandoffState,
    artifacts: &[handoff::ChangeHandoffArtifact],
) -> Result<CompletedChangeHandoffReview, ChangeHandoffReviewError> {
    let completion = handoff::evaluate_change_handoff_completion(recorded).map_err(|block| {
        ChangeHandoffReviewError::new(match block {
            handoff::ChangeHandoffCompletionBlock::DecisionMissing => {
                ChangeHandoffReviewIssue::DecisionMissing
            }
            handoff::ChangeHandoffCompletionBlock::DecisionNotAcceptance => {
                ChangeHandoffReviewIssue::DecisionNotAcceptance
            }
        })
    })?;
    let receipt = handoff::project_change_handoff_receipt(recorded, artifacts)
        .map_err(|_error| core_error())?;
    Ok(CompletedChangeHandoffReview {
        completion,
        receipt,
    })
}

const fn core_error() -> ChangeHandoffReviewError {
    ChangeHandoffReviewError::new(ChangeHandoffReviewIssue::CoreRejected)
}

#[cfg(test)]
#[path = "changescopehandoff/tests.rs"]
mod tests;
