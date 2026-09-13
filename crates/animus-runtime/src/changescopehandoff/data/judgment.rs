//! Review decisions, handoff state, completion, and bounded continuation facts.

/// One recorded ownership decision kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeHandoffDecisionKind {
    /// The developer approved the declared subset for continuation.
    Approved,
    /// The developer rejected the presented scope.
    Rejected,
    /// The developer requested refinement without approving items.
    RefineRequested,
}

impl ChangeHandoffDecisionKind {
    /// Returns the stable decision-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Rejected => "rejected",
            Self::RefineRequested => "refine_requested",
        }
    }
}

/// Borrowed facts for one developer ownership decision.
#[derive(Clone, Copy, Debug)]
pub struct ChangeHandoffDecisionDraft<'input> {
    /// Exact BLAKE3 scope identity the decision answers.
    pub scope_identity: &'input str,
    /// Recorded decision kind.
    pub kind: ChangeHandoffDecisionKind,
    /// Approved portable hunk indexes in strict ascending order, empty unless approved.
    pub approved_hunks: &'input [u32],
}

/// One recorded immutable ownership decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordedChangeHandoffDecision {
    pub(in crate::changescopehandoff) decision_identity: String,
    pub(in crate::changescopehandoff) scope_identity: String,
    pub(in crate::changescopehandoff) kind: ChangeHandoffDecisionKind,
    pub(in crate::changescopehandoff) approved_hunks: Vec<u32>,
}

impl RecordedChangeHandoffDecision {
    /// Returns the immutable BLAKE3 decision identity.
    #[must_use]
    pub fn decision_identity(&self) -> &str {
        self.decision_identity.as_str()
    }

    /// Returns the exact answered scope identity.
    #[must_use]
    pub fn scope_identity(&self) -> &str {
        self.scope_identity.as_str()
    }

    /// Returns the recorded decision kind.
    #[must_use]
    pub const fn kind(&self) -> ChangeHandoffDecisionKind {
        self.kind
    }

    /// Returns the approved portable hunk indexes.
    #[must_use]
    pub fn approved_hunks(&self) -> &[u32] {
        self.approved_hunks.as_slice()
    }
}

/// Handoff lifecycle status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeHandoffStatus {
    /// The scope is presented but has no recorded review decision.
    PendingDecision,
    /// One immutable review decision is recorded.
    DecisionRecorded,
}

impl ChangeHandoffStatus {
    /// Returns the stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::PendingDecision => "pending_decision",
            Self::DecisionRecorded => "decision_recorded",
        }
    }
}

/// Explicit completion blocker classes for a presented scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeHandoffCompletionBlock {
    /// No review decision is recorded.
    DecisionMissing,
    /// A decision is recorded but it is not an acceptance.
    DecisionNotAcceptance,
}

/// One completion evaluation that cites the recorded acceptance.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeHandoffCompletion {
    pub(in crate::changescopehandoff) scope_identity: String,
    pub(in crate::changescopehandoff) decision_identity: String,
    pub(in crate::changescopehandoff) approved_hunks: Vec<u32>,
}

impl ChangeHandoffCompletion {
    /// Returns the completed exact scope identity.
    #[must_use]
    pub fn scope_identity(&self) -> &str {
        self.scope_identity.as_str()
    }

    /// Returns the cited recorded acceptance identity.
    #[must_use]
    pub fn decision_identity(&self) -> &str {
        self.decision_identity.as_str()
    }

    /// Returns the approved portable hunk indexes the completion cites.
    #[must_use]
    pub fn approved_hunks(&self) -> &[u32] {
        self.approved_hunks.as_slice()
    }
}

/// One typed change-scope handoff state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeHandoffState {
    pub(in crate::changescopehandoff) scope: super::facts::ChangeScope,
    pub(in crate::changescopehandoff) decision: Option<RecordedChangeHandoffDecision>,
    pub(in crate::changescopehandoff) status: ChangeHandoffStatus,
}

impl ChangeHandoffState {
    /// Returns the presented immutable scope.
    #[must_use]
    pub const fn scope(&self) -> &super::facts::ChangeScope {
        &self.scope
    }

    /// Returns the recorded decision when one exists.
    #[must_use]
    pub const fn decision(&self) -> Option<&RecordedChangeHandoffDecision> {
        self.decision.as_ref()
    }

    /// Returns the current lifecycle status.
    #[must_use]
    pub const fn status(&self) -> ChangeHandoffStatus {
        self.status
    }
}

/// One subset-bounded continuation plan after a recorded decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeContinuationPlan {
    pub(in crate::changescopehandoff) scope_identity: String,
    pub(in crate::changescopehandoff) decision_identity: String,
    pub(in crate::changescopehandoff) approved_hunks: Vec<u32>,
    pub(in crate::changescopehandoff) fresh_scope: Option<super::facts::ChangeScope>,
}

impl ChangeContinuationPlan {
    /// Returns the exact scope identity the plan continues.
    #[must_use]
    pub fn scope_identity(&self) -> &str {
        self.scope_identity.as_str()
    }

    /// Returns the cited recorded decision identity.
    #[must_use]
    pub fn decision_identity(&self) -> &str {
        self.decision_identity.as_str()
    }

    /// Returns the only portable hunk indexes that continuation permits.
    #[must_use]
    pub fn approved_hunks(&self) -> &[u32] {
        self.approved_hunks.as_slice()
    }

    /// Returns the fresh scope that owns rejected items when one exists.
    #[must_use]
    pub const fn fresh_scope(&self) -> Option<&super::facts::ChangeScope> {
        self.fresh_scope.as_ref()
    }
}
