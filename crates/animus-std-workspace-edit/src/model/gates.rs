/// Exact host observations required before the first staging effect.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FreshnessObservation<'a> {
    /// Current capability-root identity.
    pub root_identity: &'a str,
    /// Current logical resource identity.
    pub resource_identity: &'a str,
    /// Current revision identity.
    pub revision_identity: &'a str,
    /// Current complete-content identity from the second read.
    pub content_identity: &'a str,
    /// Current metadata-policy identity.
    pub metadata_policy_identity: &'a str,
    /// Current file-metadata identity from the exact second read.
    pub file_metadata_identity: &'a str,
    /// Current writer generation under the acquired lock.
    pub writer_generation: u64,
}

/// One binary gate decision supplied by an existing host policy owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateDecision {
    /// The exact gate admits the request.
    Allowed,
    /// The exact gate denies the request.
    Denied,
}

/// Combined cancellation and deadline state before file I/O.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleGate {
    /// Cancellation and deadline remain current.
    Current,
    /// Cancellation is current.
    Cancelled,
    /// The deadline expired.
    TimedOut,
}

/// Exact tool, authority, approval, and lifecycle gates before file I/O.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GateObservations {
    /// Injected tool-executor admission.
    pub executor: GateDecision,
    /// Capability-policy admission.
    pub capability: GateDecision,
    /// Host-authority admission.
    pub authority: GateDecision,
    /// Operator-approval admission.
    pub approval: GateDecision,
    /// Mutation-scheduler admission.
    pub mutation: GateDecision,
    /// Combined cancellation and deadline state.
    pub lifecycle: LifecycleGate,
    /// Bounded metadata-only output admission.
    pub output: GateDecision,
}

/// Field that drifted before publication.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StaleField {
    /// Capability-root identity drifted.
    Root,
    /// Logical resource identity drifted.
    Resource,
    /// Revision identity drifted.
    Revision,
    /// Complete content drifted.
    Content,
    /// Metadata or file policy drifted.
    MetadataPolicy,
    /// File kind, mode, owner, or size drifted.
    FileMetadata,
    /// Animus-owned writer generation drifted.
    WriterGeneration,
}

/// Pure admission or planning issue.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HostIssue {
    /// A required BLAKE3 identity is malformed.
    InvalidIdentity(&'static str),
    /// Source is empty or not UTF-8 text.
    InvalidTextSource,
    /// Source exceeds the named byte bound.
    SourceTooLarge,
    /// Source contains a bare carriage return.
    UnsupportedLineEnding,
    /// Declared and observed line-ending modes differ.
    LineEndingMismatch,
    /// Mixed endings lack explicit policy admission.
    MixedLineEndingsDenied,
    /// Declared and observed final-newline states differ.
    FinalNewlineMismatch,
    /// The existing snapshot hash does not match exact source bytes.
    SnapshotHashMismatch,
    /// The existing snapshot shape does not match exact source lines.
    SnapshotShapeMismatch,
    /// Visibility facts are empty, malformed, duplicate, or over budget.
    InvalidVisibility,
    /// A selected tag does not exist in the exact snapshot.
    UnknownTag,
    /// A replacement or deletion includes an unobserved line.
    HiddenSpan,
    /// An insertion names an unobserved boundary.
    HiddenBoundary,
    /// The edit request is empty or over budget.
    InvalidOperationCount,
    /// A replacement is empty, malformed, or over budget.
    InvalidReplacement,
    /// Operations overlap or conflict at one boundary.
    OverlappingOperation,
    /// A required host gate denied before file I/O.
    GateDenied(&'static str),
    /// One exact pre-publication fact drifted.
    Stale(StaleField),
    /// Publication observations cannot support the requested outcome.
    InvalidPublication,
}
