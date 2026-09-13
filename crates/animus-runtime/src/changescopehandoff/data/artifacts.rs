//! Review artifact facts and metadata-only receipt fields.

/// Bounded review-artifact classes admitted by the handoff rail.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeHandoffArtifactKind {
    /// Bounded diff-content identity.
    DiffIdentity,
    /// Bounded decision-record identity.
    DecisionIdentity,
    /// Raw transcript payload, always rejected before persistence.
    RawTranscript,
    /// Raw tool-output payload, always rejected before persistence.
    RawToolOutput,
    /// Raw prompt or response payload, always rejected before persistence.
    RawPromptResponse,
}

impl ChangeHandoffArtifactKind {
    /// Returns the stable artifact-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::DiffIdentity => "diff_identity",
            Self::DecisionIdentity => "decision_identity",
            Self::RawTranscript => "raw_transcript",
            Self::RawToolOutput => "raw_tool_output",
            Self::RawPromptResponse => "raw_prompt_response",
        }
    }

    /// Returns whether the kind carries bounded identities only.
    #[must_use]
    pub const fn is_bounded(self) -> bool {
        matches!(self, Self::DiffIdentity | Self::DecisionIdentity)
    }
}

/// One borrowed review-artifact candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChangeHandoffArtifactDraft<'input> {
    /// Declared artifact kind.
    pub kind: ChangeHandoffArtifactKind,
    /// Immutable BLAKE3 artifact identity for bounded kinds.
    pub artifact_identity: &'input str,
    /// Declared payload byte count within the named bound.
    pub declared_byte_count: u64,
}

/// One validated bounded review artifact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeHandoffArtifact {
    pub(in crate::changescopehandoff) kind: ChangeHandoffArtifactKind,
    pub(in crate::changescopehandoff) artifact_identity: String,
    pub(in crate::changescopehandoff) declared_byte_count: u64,
}

impl ChangeHandoffArtifact {
    /// Returns the validated artifact kind.
    #[must_use]
    pub const fn kind(&self) -> ChangeHandoffArtifactKind {
        self.kind
    }

    /// Returns the immutable BLAKE3 artifact identity.
    #[must_use]
    pub fn artifact_identity(&self) -> &str {
        self.artifact_identity.as_str()
    }

    /// Returns the declared payload byte count.
    #[must_use]
    pub const fn declared_byte_count(&self) -> u64 {
        self.declared_byte_count
    }
}

/// One terminal metadata-only change-scope handoff receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeHandoffReceipt {
    pub(in crate::changescopehandoff) schema: &'static str,
    pub(in crate::changescopehandoff) contract_identity: &'static str,
    pub(in crate::changescopehandoff) scope_identity: String,
    pub(in crate::changescopehandoff) driving_task: String,
    pub(in crate::changescopehandoff) status: super::judgment::ChangeHandoffStatus,
    pub(in crate::changescopehandoff) decision_identity: Option<String>,
    pub(in crate::changescopehandoff) decision_kind:
        Option<super::judgment::ChangeHandoffDecisionKind>,
    pub(in crate::changescopehandoff) approved_hunk_count: u32,
    pub(in crate::changescopehandoff) hunk_count: u32,
    pub(in crate::changescopehandoff) artifact_identities: Vec<String>,
    pub(in crate::changescopehandoff) completion_decision_identity: Option<String>,
    pub(in crate::changescopehandoff) redaction_marker: &'static str,
    pub(in crate::changescopehandoff) non_claims: Vec<&'static str>,
    pub(in crate::changescopehandoff) receipt_identity: String,
}

impl ChangeHandoffReceipt {
    /// Returns the receipt schema.
    #[must_use]
    pub const fn schema(&self) -> &'static str {
        self.schema
    }

    /// Returns the stable contract identity.
    #[must_use]
    pub const fn contract_identity(&self) -> &'static str {
        self.contract_identity
    }

    /// Returns the exact presented scope identity.
    #[must_use]
    pub fn scope_identity(&self) -> &str {
        self.scope_identity.as_str()
    }

    /// Returns the safe driving-task label.
    #[must_use]
    pub fn driving_task(&self) -> &str {
        self.driving_task.as_str()
    }

    /// Returns the recorded handoff status.
    #[must_use]
    pub const fn status(&self) -> super::judgment::ChangeHandoffStatus {
        self.status
    }

    /// Returns the recorded decision identity when one exists.
    #[must_use]
    pub fn decision_identity(&self) -> Option<&str> {
        self.decision_identity.as_deref()
    }

    /// Returns the recorded decision kind when one exists.
    #[must_use]
    pub const fn decision_kind(&self) -> Option<super::judgment::ChangeHandoffDecisionKind> {
        self.decision_kind
    }

    /// Returns the approved hunk count.
    #[must_use]
    pub const fn approved_hunk_count(&self) -> u32 {
        self.approved_hunk_count
    }

    /// Returns the declared hunk count.
    #[must_use]
    pub const fn hunk_count(&self) -> u32 {
        self.hunk_count
    }

    /// Returns the bounded review-artifact identities.
    #[must_use]
    pub fn artifact_identities(&self) -> &[String] {
        self.artifact_identities.as_slice()
    }

    /// Returns the cited acceptance identity when completion is recorded.
    #[must_use]
    pub fn completion_decision_identity(&self) -> Option<&str> {
        self.completion_decision_identity.as_deref()
    }

    /// Returns the redaction marker proving raw change values are absent.
    #[must_use]
    pub const fn redaction_marker(&self) -> &'static str {
        self.redaction_marker
    }

    /// Returns the fixed non-claim labels.
    #[must_use]
    pub fn non_claims(&self) -> &[&'static str] {
        self.non_claims.as_slice()
    }

    /// Returns the immutable BLAKE3 receipt identity.
    #[must_use]
    pub fn receipt_identity(&self) -> &str {
        self.receipt_identity.as_str()
    }
}
