/// Stable contract identity for agent change-scope handoffs.
pub const CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY: &str = "animus.change-scope-handoff.v1";
/// Stable public receipt schema for agent change-scope handoffs.
pub const CHANGE_SCOPE_HANDOFF_RECEIPT_SCHEMA: &str = "animus.change-scope-handoff-receipt.v1";
/// Marker proving that raw change content is absent from public handoff evidence.
pub const CHANGE_SCOPE_HANDOFF_REDACTION_MARKER: &str = "raw-change-values-omitted";
const SCOPE_FILE_BOUND: usize = 64;
const SCOPE_HUNK_BOUND: usize = 256;
const SCOPE_OBLIGATION_BOUND: usize = 32;
const SCOPE_LABEL_CHAR_BOUND: usize = 128;
const SCOPE_ARTIFACT_BOUND: usize = 32;
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
pub const fn scope_file_bound() -> usize {
    SCOPE_FILE_BOUND
}
/// Returns the named declared-hunk bound for one change scope.
#[must_use]
pub const fn scope_hunk_bound() -> usize {
    SCOPE_HUNK_BOUND
}
/// Returns the named validation-obligation bound for one change scope.
#[must_use]
pub const fn scope_obligation_bound() -> usize {
    SCOPE_OBLIGATION_BOUND
}
/// Returns the named label character bound for scope facts.
#[must_use]
pub const fn scope_label_char_bound() -> usize {
    SCOPE_LABEL_CHAR_BOUND
}
/// Returns the named review-artifact count bound for one handoff.
#[must_use]
pub const fn scope_artifact_bound() -> usize {
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
    /// One required BLAKE3 identity was malformed.
    InvalidIdentity,
    /// One scope fact label exceeded the named character bound.
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

/// One borrowed declared hunk inside a proposed change scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ChangeScopeHunkDraft<'input> {
    /// Declared file path label that owns this hunk.
    pub file_path: &'input str,
    /// Immutable BLAKE3 hunk-content identity.
    pub hunk_identity: &'input str,
    /// Declared old-file start line.
    pub old_start: u32,
    /// Declared old-file line count.
    pub old_lines: u32,
    /// Declared new-file start line.
    pub new_start: u32,
    /// Declared new-file line count.
    pub new_lines: u32,
}

/// Borrowed complete facts for one proposed change scope.
#[derive(Clone, Copy, Debug)]
pub struct ChangeScopeDraft<'input> {
    /// Bounded driving-task label that owns the proposed change.
    pub driving_task: &'input str,
    /// Declared bounded file-path labels.
    pub files: &'input [&'input str],
    /// Declared bounded hunks over `files`.
    pub hunks: &'input [ChangeScopeHunkDraft<'input>],
    /// Declared bounded validation-obligation labels.
    pub obligations: &'input [&'input str],
}

/// One validated declared hunk.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeScopeHunk {
    pub(super) file_path: String,
    pub(super) hunk_identity: String,
    pub(super) old_start: u32,
    pub(super) old_lines: u32,
    pub(super) new_start: u32,
    pub(super) new_lines: u32,
}

impl ChangeScopeHunk {
    /// Returns the declared owning file-path label.
    #[must_use]
    pub fn file_path(&self) -> &str {
        self.file_path.as_str()
    }

    /// Returns the immutable BLAKE3 hunk-content identity.
    #[must_use]
    pub fn hunk_identity(&self) -> &str {
        self.hunk_identity.as_str()
    }

    /// Returns the declared old-file start line.
    #[must_use]
    pub const fn old_start(&self) -> u32 {
        self.old_start
    }

    /// Returns the declared old-file line count.
    #[must_use]
    pub const fn old_lines(&self) -> u32 {
        self.old_lines
    }

    /// Returns the declared new-file start line.
    #[must_use]
    pub const fn new_start(&self) -> u32 {
        self.new_start
    }

    /// Returns the declared new-file line count.
    #[must_use]
    pub const fn new_lines(&self) -> u32 {
        self.new_lines
    }
}

/// One typed immutable change scope binding files, hunks, task, and obligations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeScope {
    pub(super) scope_identity: String,
    pub(super) driving_task: String,
    pub(super) files: Vec<String>,
    pub(super) hunks: Vec<ChangeScopeHunk>,
    pub(super) obligations: Vec<String>,
}

impl ChangeScope {
    /// Returns the immutable BLAKE3 scope identity.
    #[must_use]
    pub fn scope_identity(&self) -> &str {
        self.scope_identity.as_str()
    }

    /// Returns the bounded driving-task label.
    #[must_use]
    pub fn driving_task(&self) -> &str {
        self.driving_task.as_str()
    }

    /// Returns the declared bounded file-path labels.
    #[must_use]
    pub fn files(&self) -> &[String] {
        self.files.as_slice()
    }

    /// Returns the declared bounded hunks.
    #[must_use]
    pub fn hunks(&self) -> &[ChangeScopeHunk] {
        self.hunks.as_slice()
    }

    /// Returns the declared bounded validation-obligation labels.
    #[must_use]
    pub fn obligations(&self) -> &[String] {
        self.obligations.as_slice()
    }
}

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
    /// Approved hunk indexes in strict ascending order; empty unless approved.
    pub approved_hunks: &'input [usize],
}

/// One recorded immutable ownership decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecordedChangeHandoffDecision {
    pub(super) decision_identity: String,
    pub(super) scope_identity: String,
    pub(super) kind: ChangeHandoffDecisionKind,
    pub(super) approved_hunks: Vec<usize>,
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

    /// Returns the approved hunk indexes.
    #[must_use]
    pub fn approved_hunks(&self) -> &[usize] {
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
    pub(super) scope_identity: String,
    pub(super) decision_identity: String,
    pub(super) approved_hunks: Vec<usize>,
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

    /// Returns the approved hunk indexes the completion cites.
    #[must_use]
    pub fn approved_hunks(&self) -> &[usize] {
        self.approved_hunks.as_slice()
    }
}

/// One typed change-scope handoff state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeHandoffState {
    pub(super) scope: ChangeScope,
    pub(super) decision: Option<RecordedChangeHandoffDecision>,
    pub(super) status: ChangeHandoffStatus,
}

impl ChangeHandoffState {
    /// Returns the presented immutable scope.
    #[must_use]
    pub const fn scope(&self) -> &ChangeScope {
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

/// Bounded review-artifact classes admitted by the handoff rail.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ChangeHandoffArtifactKind {
    /// Bounded diff-content identity.
    DiffIdentity,
    /// Bounded decision-record identity.
    DecisionIdentity,
    /// Raw transcript payload; always rejected before persistence.
    RawTranscript,
    /// Raw tool-output payload; always rejected before persistence.
    RawToolOutput,
    /// Raw prompt or response payload; always rejected before persistence.
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
    pub(super) kind: ChangeHandoffArtifactKind,
    pub(super) artifact_identity: String,
    pub(super) declared_byte_count: u64,
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

/// One subset-bounded continuation plan after a recorded decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeContinuationPlan {
    pub(super) scope_identity: String,
    pub(super) decision_identity: String,
    pub(super) approved_hunks: Vec<usize>,
    pub(super) fresh_scope: Option<ChangeScope>,
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

    /// Returns the only hunk indexes continuation may operate within.
    #[must_use]
    pub fn approved_hunks(&self) -> &[usize] {
        self.approved_hunks.as_slice()
    }

    /// Returns the fresh scope that owns rejected items when one exists.
    #[must_use]
    pub const fn fresh_scope(&self) -> Option<&ChangeScope> {
        self.fresh_scope.as_ref()
    }
}

/// One terminal metadata-only change-scope handoff receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChangeHandoffReceipt {
    pub(super) schema: &'static str,
    pub(super) contract_identity: &'static str,
    pub(super) scope_identity: String,
    pub(super) driving_task: String,
    pub(super) status: ChangeHandoffStatus,
    pub(super) decision_identity: Option<String>,
    pub(super) decision_kind: Option<ChangeHandoffDecisionKind>,
    pub(super) approved_hunk_count: u32,
    pub(super) hunk_count: u32,
    pub(super) artifact_identities: Vec<String>,
    pub(super) completion_decision_identity: Option<String>,
    pub(super) redaction_marker: &'static str,
    pub(super) non_claims: Vec<&'static str>,
    pub(super) receipt_identity: String,
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
    pub const fn status(&self) -> ChangeHandoffStatus {
        self.status
    }

    /// Returns the recorded decision identity when one exists.
    #[must_use]
    pub fn decision_identity(&self) -> Option<&str> {
        self.decision_identity.as_deref()
    }

    /// Returns the recorded decision kind when one exists.
    #[must_use]
    pub const fn decision_kind(&self) -> Option<ChangeHandoffDecisionKind> {
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
