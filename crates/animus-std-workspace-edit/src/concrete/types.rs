/// Host checkpoint after lock acquisition and before irreversible work.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostCheckpoint {
    /// Check cancellation and deadline before the exact second read.
    BeforeSecondRead,
    /// Check cancellation and deadline immediately before stage creation.
    BeforePublication,
}

/// Stable injected service failure without raw host material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostServiceIssue {
    /// Required service is absent or unavailable.
    Unavailable,
    /// Host authority or lock acquisition denied the request.
    Denied,
    /// Current generation or revision does not match.
    Stale,
    /// Cancellation became current.
    Cancelled,
    /// Deadline expired.
    TimedOut,
    /// Writer release or generation commit failed.
    ReleaseFailed,
}

/// Current revision, policy, and writer facts supplied by the host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostObservation {
    /// Exact current revision identity.
    pub revision_identity: String,
    /// Exact current metadata-policy identity.
    pub metadata_policy_identity: String,
    /// Current Animus-owned writer generation.
    pub writer_generation: u64,
}

/// Resource-scoped writer lease returned by the injected lock service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WriterLease {
    resource_identity: String,
    revision_identity: String,
    writer_generation: u64,
}

impl WriterLease {
    /// Construct one supplied lease for an exact resource and generation.
    #[must_use]
    pub fn new(
        resource_identity: impl Into<String>,
        revision_identity: impl Into<String>,
        writer_generation: u64,
    ) -> Self {
        Self {
            resource_identity: resource_identity.into(),
            revision_identity: revision_identity.into(),
            writer_generation,
        }
    }

    /// Return the leased resource identity.
    #[must_use]
    pub fn resource_identity(&self) -> &str {
        &self.resource_identity
    }

    /// Return the leased revision identity.
    #[must_use]
    pub fn revision_identity(&self) -> &str {
        &self.revision_identity
    }

    /// Return the leased writer generation.
    #[must_use]
    pub const fn writer_generation(&self) -> u64 {
        self.writer_generation
    }
}

/// Writer-release classification supplied after one terminal attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WriterRelease {
    /// Target publication did not occur; retain the current generation.
    NotPublished,
    /// Target publication occurred; commit the next generation.
    Published,
    /// Publication may have occurred; fence later writers with the next generation.
    PublicationUncertain,
}

/// Injected revision, lock, cancellation, deadline, and generation services.
pub trait HostServices {
    /// Observe current revision, policy, and writer facts.
    ///
    /// # Errors
    ///
    /// Returns a stable service issue when current facts are unavailable.
    fn observe(&mut self, resource_identity: &str) -> Result<HostObservation, HostServiceIssue>;

    /// Acquire the exact resource-scoped writer generation.
    ///
    /// # Errors
    ///
    /// Returns a stable issue when authority, revision, or generation denies the lease.
    fn acquire_writer(
        &mut self,
        resource_identity: &str,
        revision_identity: &str,
        expected_generation: u64,
    ) -> Result<WriterLease, HostServiceIssue>;

    /// Check cancellation and deadline at a named host boundary.
    ///
    /// # Errors
    ///
    /// Returns cancellation, timeout, or service-unavailable state.
    fn checkpoint(&mut self, checkpoint: HostCheckpoint) -> Result<(), HostServiceIssue>;

    /// Release the lease and return the committed current generation.
    ///
    /// # Errors
    ///
    /// Returns a stable issue when release or generation commit fails.
    fn release_writer(
        &mut self,
        lease: WriterLease,
        release: WriterRelease,
    ) -> Result<u64, HostServiceIssue>;
}

/// Caller-injected finite private-stage name source.
pub trait StageNameService {
    /// Return the next candidate leaf.
    fn next_stage_leaf(&mut self) -> String;
}

/// Exact concrete host request over one caller-opened capability root.
#[derive(Clone, Copy, Debug)]
pub struct HostRequest<'a> {
    /// Existing complete-content tagged snapshot.
    pub base_snapshot: &'a animus_adapters::tool::hashed::Snapshot,
    /// Logical capability-root-relative resource.
    pub logical_resource: &'a std::path::Path,
    /// Exact capability-root identity.
    pub root_identity: &'a str,
    /// Exact logical resource identity derived from the root and normalized path.
    pub resource_identity: &'a str,
    /// Exact expected revision identity.
    pub revision_identity: &'a str,
    /// Exact metadata and file policy identity.
    pub metadata_policy_identity: &'a str,
    /// Expected Animus writer generation.
    pub writer_generation: u64,
    /// Current tool, authority, approval, mutation, lifecycle, and output gates.
    pub gates: crate::model::GateObservations,
    /// Declared line-ending mode.
    pub line_ending_mode: crate::model::LineEndingMode,
    /// Declared final-newline state.
    pub final_newline: crate::model::FinalNewlineState,
    /// Explicit new-boundary ending.
    pub replacement_ending: crate::model::ReplacementEnding,
    /// Whether mixed source endings are explicitly admitted.
    pub mixed_endings_admitted: bool,
    /// Visible tagged line spans.
    pub visible_spans: &'a [crate::model::VisibleSpanDraft<'a>],
    /// Visible tagged insertion boundaries.
    pub visible_boundaries: &'a [crate::model::VisibleBoundaryDraft<'a>],
    /// Whether the end insertion boundary is visible.
    pub end_boundary_visible: bool,
    /// Ordered edit operations.
    pub operations: &'a [crate::model::EditOperationDraft<'a>],
    /// Expected current Unix owner on Unix hosts.
    pub expected_owner: Option<u32>,
    /// Expected current Unix permission bits on Unix hosts.
    pub expected_mode: Option<u32>,
    /// Exact final Unix permission bits for the replacement file.
    pub final_mode: u32,
    /// Finite private-stage collision attempt bound.
    pub collision_attempt_limit: usize,
    /// Whether parent-directory synchronization is required.
    pub require_durability: bool,
}

/// Stable concrete host error without paths, source, replacement, or raw I/O text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HostError {
    /// Pure admission, apply, freshness, or publication issue.
    Core(crate::model::HostIssue),
    /// Lexical path admission or resource-identity binding failed.
    PathDenied,
    /// Capability-relative metadata or file read failed.
    ReadFailed,
    /// Target is a symlink or not a regular file.
    UnsupportedFileKind,
    /// Current file permission facts violate explicit host policy.
    PermissionDenied,
    /// Current file ownership facts violate explicit host policy.
    OwnershipDenied,
    /// Injected revision, lock, lifecycle, or release service failed.
    Service(HostServiceIssue),
    /// Publication mechanism rejected its request or observation stream.
    PublicationFailed,
    /// Publication may have occurred; rollback and unchanged-target claims are blocked.
    UncertainAfterPublication,
    /// Receipt projection rejected private material or overclaim state.
    Receipt(crate::evidence::ReceiptIssue),
    /// Exact reviewed publication shell is unavailable on this platform.
    UnsupportedPlatform,
}

impl From<crate::model::HostIssue> for HostError {
    fn from(issue: crate::model::HostIssue) -> Self {
        Self::Core(issue)
    }
}

impl From<crate::evidence::ReceiptIssue> for HostError {
    fn from(issue: crate::evidence::ReceiptIssue) -> Self {
        Self::Receipt(issue)
    }
}
