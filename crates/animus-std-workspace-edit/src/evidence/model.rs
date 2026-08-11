pub(crate) const RECEIPT_SCHEMA: &str = "animus.workspace-edit-host.receipt.v1";
pub(crate) const RECEIPT_DOMAIN: &[u8] = b"animus.workspace-edit-host.receipt.v1";
pub(crate) const REDACTION: &str =
    "file-body-replacement-path-vcs-process-credential-and-error-material-omitted";
const NON_CLAIM_EXTERNAL_WRITER: &str = "not-universal-writer-exclusion";
const NON_CLAIM_SEMANTIC: &str = "not-semantic-correctness";
const NON_CLAIM_AUTHORITY: &str = "not-root-authorization";
const NON_CLAIM_SANDBOX: &str = "not-sandboxing";
const NON_CLAIM_DURABILITY: &str = "not-storage-device-correctness";
const NON_CLAIM_PRODUCTION: &str = "not-production-readiness";
pub(crate) const NON_CLAIM_COUNT: usize = 6;

pub(crate) const NON_CLAIMS: [&str; NON_CLAIM_COUNT] = [
    NON_CLAIM_EXTERNAL_WRITER,
    NON_CLAIM_SEMANTIC,
    NON_CLAIM_AUTHORITY,
    NON_CLAIM_SANDBOX,
    NON_CLAIM_DURABILITY,
    NON_CLAIM_PRODUCTION,
];

/// Private material class rejected from one receipt candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrivateMaterialClass {
    /// Original file bytes or text.
    FileBody,
    /// Replacement or insertion text.
    ReplacementText,
    /// Absolute or host-private path.
    AbsolutePath,
    /// VCS branch, status, diff, or revision material beyond the safe identity.
    VcsData,
    /// Process or raw argument material.
    ProcessData,
    /// Credential material.
    CredentialData,
    /// Unrestricted host error material.
    RawError,
}

/// Unsupported claim class rejected from one receipt candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedClaimClass {
    /// Universal cross-process exclusion.
    UniversalExclusion,
    /// Semantic edit correctness.
    SemanticCorrectness,
    /// Atomic crash durability.
    AtomicDurability,
    /// Sandboxing or root authorization.
    AuthorityOrSandbox,
    /// Production or release readiness.
    Readiness,
}

/// Sensitive-material or overclaim scan result for one receipt candidate.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ReceiptLeakFacts {
    /// Candidate is free of known private material and overclaims.
    #[default]
    None,
    /// Candidate contains one private-material class.
    Private(PrivateMaterialClass),
    /// Candidate contains one unsupported claim class.
    UnsupportedClaim(UnsupportedClaimClass),
}

impl ReceiptLeakFacts {
    pub(crate) const fn has_private_material(self) -> bool {
        matches!(self, Self::Private(_))
    }

    pub(crate) const fn has_overclaim(self) -> bool {
        matches!(self, Self::UnsupportedClaim(_))
    }
}

/// Borrowed safe receipt input after one terminal publication outcome.
#[derive(Clone, Copy, Debug)]
pub struct ReceiptDraft<'a> {
    /// Exact pure edit plan.
    pub plan: &'a crate::model::EditPlan,
    /// Exact terminal publication outcome.
    pub publication: &'a crate::model::PublicationOutcome,
    /// Writer generation observed when the operation released its lock.
    pub released_writer_generation: u64,
    /// Explicit private-material and overclaim scan results.
    pub leak_facts: ReceiptLeakFacts,
}

/// Stable receipt-validation issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptIssue {
    /// Candidate contains private file, path, VCS, process, credential, or error material.
    PrivateMaterial,
    /// Candidate contains an unsupported authority, correctness, durability, or readiness claim.
    UnsupportedClaim,
    /// Writer generation is zero or older than the admitted generation.
    InvalidWriterGeneration,
}

/// Metadata-only concrete workspace-edit receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditReceipt {
    pub(crate) schema: &'static str,
    pub(crate) root_identity: String,
    pub(crate) resource_identity: String,
    pub(crate) revision_identity: String,
    pub(crate) before_identity: String,
    pub(crate) after_identity: String,
    pub(crate) snapshot_identity: String,
    pub(crate) plan_identity: String,
    pub(crate) visibility_identity: String,
    pub(crate) writer_generation: u64,
    pub(crate) released_writer_generation: u64,
    pub(crate) publication_state: &'static str,
    pub(crate) durability_state: &'static str,
    pub(crate) operation_count: u32,
    pub(crate) observation_count: u32,
    pub(crate) redaction: &'static str,
    pub(crate) non_claims: [&'static str; NON_CLAIM_COUNT],
    pub(crate) identity: String,
}

impl EditReceipt {
    /// Return the receipt schema.
    #[must_use]
    pub const fn schema(&self) -> &'static str {
        self.schema
    }

    /// Return the capability-root identity.
    #[must_use]
    pub fn root_identity(&self) -> &str {
        &self.root_identity
    }

    /// Return the logical resource identity.
    #[must_use]
    pub fn resource_identity(&self) -> &str {
        &self.resource_identity
    }

    /// Return the supplied revision identity.
    #[must_use]
    pub fn revision_identity(&self) -> &str {
        &self.revision_identity
    }

    /// Return the before-content identity.
    #[must_use]
    pub fn before_identity(&self) -> &str {
        &self.before_identity
    }

    /// Return the planned after-content identity.
    #[must_use]
    pub fn after_identity(&self) -> &str {
        &self.after_identity
    }

    /// Return the canonical admitted snapshot identity.
    #[must_use]
    pub fn snapshot_identity(&self) -> &str {
        &self.snapshot_identity
    }

    /// Return the pure plan identity.
    #[must_use]
    pub fn plan_identity(&self) -> &str {
        &self.plan_identity
    }

    /// Return the visible-span and boundary identity.
    #[must_use]
    pub fn visibility_identity(&self) -> &str {
        &self.visibility_identity
    }

    /// Return the admitted writer generation.
    #[must_use]
    pub const fn writer_generation(&self) -> u64 {
        self.writer_generation
    }

    /// Return the generation observed at lock release.
    #[must_use]
    pub const fn released_writer_generation(&self) -> u64 {
        self.released_writer_generation
    }

    /// Return the terminal publication-state label.
    #[must_use]
    pub const fn publication_state(&self) -> &'static str {
        self.publication_state
    }

    /// Return the separate durability-state label.
    #[must_use]
    pub const fn durability_state(&self) -> &'static str {
        self.durability_state
    }

    /// Return the deterministic operation count.
    #[must_use]
    pub const fn operation_count(&self) -> u32 {
        self.operation_count
    }

    /// Return the bounded mechanism observation count.
    #[must_use]
    pub const fn observation_count(&self) -> u32 {
        self.observation_count
    }

    /// Return the mandatory redaction label.
    #[must_use]
    pub const fn redaction(&self) -> &'static str {
        self.redaction
    }

    /// Return mandatory claim-boundary labels.
    #[must_use]
    pub const fn non_claims(&self) -> &[&'static str; NON_CLAIM_COUNT] {
        &self.non_claims
    }

    /// Return the canonical BLAKE3 receipt identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}
