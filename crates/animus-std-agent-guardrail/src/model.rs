use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// Checked guardrail profile schema.
pub const PROFILE_SCHEMA: &str = "animus.agent-guardrail.profile.v1";
/// Guardrail launch-plan schema.
pub const PLAN_SCHEMA: &str = "animus.agent-guardrail.plan.v1";
/// Guardrail receipt schema.
pub const RECEIPT_SCHEMA: &str = "animus.agent-guardrail.receipt.v1";
/// Exact first-tranche Bubblewrap compatibility cohort.
pub const BUBBLEWRAP_COHORT: &str = "bubblewrap-0.11.2-args-v1";
pub(crate) const PROFILE_DOMAIN: &str = "animus.agent-guardrail.profile.v1";
pub(crate) const PLAN_DOMAIN: &str = "animus.agent-guardrail.plan.v1";
pub(crate) const ARTIFACT_DOMAIN: &str = "animus.agent-guardrail.executable.v1";
pub(crate) const RECEIPT_DOMAIN: &str = "animus.agent-guardrail.receipt.v1";
/// Safe receipt redaction statement.
pub const REDACTION_MARKER: &str = "raw-prompt-credential-environment-host-path-mount-command-output-debug-and-unbounded-events-omitted";
/// Backend-specific boundaries preserved by every receipt.
pub const NON_CLAIMS: [&str; 8] = [
    "not-hostile-code-containment",
    "not-kernel-correctness",
    "not-same-uid-secrecy",
    "not-filtered-network-egress",
    "not-authorization",
    "not-build-hermeticity",
    "not-toctou-safe-path-resolution",
    "not-production-or-release-readiness",
];

/// Filesystem exposure mode selected by the central profile.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum RootMode {
    /// Expose the root read-only.
    ReadOnly,
    /// Expose the root writable after explicit authorization.
    Writable,
}

/// Source kind observed by the host shell.
#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum SourceKind {
    /// Source is a directory.
    Directory,
    /// Source is a regular file.
    File,
}

/// Network namespace posture.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum NetworkPosture {
    /// Use Bubblewrap's private network namespace.
    Deny,
    /// Explicitly share the host network namespace without an egress-filtering claim.
    Inherit,
}

impl NetworkPosture {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Deny => "deny",
            Self::Inherit => "inherit",
        }
    }
}

/// One centrally reviewed root declaration.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ProfileRoot {
    /// Safe logical root label.
    pub label: String,
    /// Absolute path inside the child namespace.
    pub target: PathBuf,
    /// Requested exposure mode.
    pub mode: RootMode,
    /// Expected source kind.
    pub source_kind: SourceKind,
    /// Whether the root must exist for every invocation.
    pub required: bool,
}

/// Normalized checked profile consumed by Rust at runtime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardrailProfile {
    pub(crate) identity: String,
    pub(crate) label: String,
    pub(crate) cohort: String,
    pub(crate) trust_source: String,
    pub(crate) roots: Vec<ProfileRoot>,
    pub(crate) network: NetworkPosture,
    pub(crate) environment_keys: Vec<String>,
    pub(crate) max_roots: usize,
    pub(crate) max_environment: usize,
    pub(crate) max_options: usize,
}

impl GuardrailProfile {
    /// Returns the domain-separated identity of the exact checked JSON projection.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the safe profile label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    /// Returns the exact Bubblewrap compatibility cohort.
    #[must_use]
    pub fn cohort(&self) -> &str {
        self.cohort.as_str()
    }

    /// Returns the central trust posture.
    #[must_use]
    pub fn trust_source(&self) -> &str {
        self.trust_source.as_str()
    }

    /// Returns reviewed root declarations without permitting identity-breaking mutation.
    #[must_use]
    pub fn roots(&self) -> &[ProfileRoot] {
        self.roots.as_slice()
    }

    /// Returns the explicit network posture.
    #[must_use]
    pub const fn network(&self) -> NetworkPosture {
        self.network
    }

    /// Returns the exact environment-key allowlist.
    #[must_use]
    pub fn environment_keys(&self) -> &[String] {
        self.environment_keys.as_slice()
    }
}

/// Prior authorization decision bound to every launch identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorizationBinding {
    /// Whether the prior authority decision allowed the action.
    pub allowed: bool,
    /// Safe authorization receipt identity.
    pub identity: String,
    /// Action identity.
    pub action_identity: String,
    /// Subject identity.
    pub subject_identity: String,
    /// Target identity.
    pub target_identity: String,
    /// Portable sandbox descriptor identity.
    pub descriptor_identity: String,
    /// Policy identity.
    pub policy_identity: String,
    /// Checked guardrail profile identity.
    pub profile_identity: String,
}

/// Active invocation facts supplied explicitly by the host.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ActiveContext {
    /// Action identity.
    pub action_identity: String,
    /// Subject identity.
    pub subject_identity: String,
    /// Target identity.
    pub target_identity: String,
    /// Portable sandbox descriptor identity.
    pub descriptor_identity: String,
    /// Policy identity.
    pub policy_identity: String,
    /// Safe context identity; never a host path.
    pub context_identity: String,
    /// Requested capability labels.
    pub requested_capabilities: Vec<String>,
    /// Prior authorization decision.
    pub authorization: AuthorizationBinding,
}

/// Host-resolved root and symlink facts for one profile label.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct RootBinding {
    /// Profile root label.
    pub label: String,
    /// Host capability root used to bound resolution.
    pub source_root: PathBuf,
    /// Resolved source path.
    pub source: PathBuf,
    /// Destination path in the child namespace.
    pub target: PathBuf,
    /// Observed source kind.
    pub source_kind: SourceKind,
    /// Whether the source exists.
    pub exists: bool,
    /// Whether host resolution proved the symlink chain remained in `source_root`.
    pub symlink_within_root: bool,
}

/// Explicit host environment value admitted under a profile key.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct EnvironmentBinding {
    /// Environment key.
    pub key: String,
    /// Ephemeral value passed only to Bubblewrap option encoding.
    pub value: String,
}

/// Bubblewrap executable facts observed before preflight.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutableFacts {
    /// Absolute executable path.
    pub path: PathBuf,
    /// Domain-separated BLAKE3 identity of observed executable bytes.
    pub identity: String,
    /// Observed compatibility cohort.
    pub cohort: String,
    /// Whether `--args` support was observed.
    pub args_supported: bool,
}

/// Complete side-effect-free preflight request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PreflightRequest {
    /// Active invocation and authorization facts.
    pub context: ActiveContext,
    /// Host-resolved roots.
    pub roots: Vec<RootBinding>,
    /// Explicit environment values for admitted keys.
    pub environment: Vec<EnvironmentBinding>,
    /// Bubblewrap executable facts.
    pub executable: ExecutableFacts,
    /// Optional caller-expected plan identity.
    pub expected_plan_identity: Option<String>,
}

/// One typed finalized Bubblewrap operation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PlanOperation {
    /// Replace the inherited root filesystem with an empty private tmpfs before explicit binds.
    PrivateRoot,
    /// Read-only bind.
    ReadOnlyBind {
        /// Host source path retained only in the ephemeral plan.
        source: PathBuf,
        /// Child namespace target path.
        target: PathBuf,
    },
    /// Writable bind.
    WritableBind {
        /// Host source path retained only in the ephemeral plan.
        source: PathBuf,
        /// Child namespace target path.
        target: PathBuf,
    },
    /// Mount procfs at the fixed child path.
    Proc,
    /// Provide a private temporary directory.
    PrivateTmp,
    /// Provide a private device directory without broad host device exposure.
    PrivateDev,
    /// Clear the child environment.
    ClearEnvironment,
    /// Set one explicitly admitted environment value.
    SetEnvironment {
        /// Profile-admitted environment key.
        key: String,
        /// Ephemeral value omitted from receipts.
        value: String,
    },
    /// Share the host network namespace explicitly.
    ShareNetwork,
    /// Remount the private root read-only after explicit writable child mounts are installed.
    LockRoot,
}

/// Deterministic typed launch plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardrailPlan {
    /// Plan schema.
    pub schema: &'static str,
    /// Checked profile identity.
    pub profile_identity: String,
    /// Authorization identity.
    pub authorization_identity: String,
    /// Descriptor identity.
    pub descriptor_identity: String,
    /// Policy identity.
    pub policy_identity: String,
    /// Executable identity.
    pub executable_identity: String,
    /// Explicit network posture.
    pub network: NetworkPosture,
    /// Ordered typed operations.
    pub operations: Vec<PlanOperation>,
    /// Safe enforced capability labels.
    pub enforced_capabilities: Vec<String>,
    /// Safe root labels in operation order.
    pub root_labels: Vec<String>,
    /// Domain-separated BLAKE3 plan identity.
    pub identity: String,
}

/// Side-effect-free preflight outcome class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreflightStatus {
    /// Exact prospective plan is valid.
    Valid,
    /// Authorization denied or drifted.
    Denied,
    /// Host enforcement capability or cohort is unavailable.
    Unsupported,
    /// A required source is absent.
    MissingSource,
    /// Checked facts or expected plan identity drifted.
    Drifted,
    /// Profile, root, environment, or operation input is invalid.
    Invalid,
}

/// Safe preflight result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreflightResult {
    /// Outcome class.
    pub status: PreflightStatus,
    /// Safe issue label, if denied.
    pub issue: Option<&'static str>,
    /// Prospective plan when valid.
    pub plan: Option<GuardrailPlan>,
}

/// Observed guarded child exit class.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExitClass {
    /// Child exited successfully.
    Success,
    /// Child exited with a non-zero status.
    NonZero,
    /// Child terminated by a signal.
    Signal,
    /// Child was not launched.
    NotLaunched,
}

impl ExitClass {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Success => "success",
            Self::NonZero => "non-zero",
            Self::Signal => "signal",
            Self::NotLaunched => "not-launched",
        }
    }
}

/// Bounded shell observation used for receipt projection.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct ExecutionObservation {
    /// Safe status class.
    pub status: String,
    /// Safe issue class.
    pub issue: Option<String>,
    /// Child exit classification.
    pub exit: ExitClass,
    /// Cleanup status class.
    pub cleanup: String,
    /// Number of finalized options.
    pub option_count: usize,
}

/// Metadata-only backend-specific guardrail receipt.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GuardrailReceipt {
    /// Receipt schema.
    pub schema: &'static str,
    /// Backend class.
    pub backend: &'static str,
    /// Exact compatibility cohort.
    pub cohort: &'static str,
    /// Profile identity.
    pub profile_identity: String,
    /// Descriptor identity.
    pub descriptor_identity: String,
    /// Policy identity.
    pub policy_identity: String,
    /// Authorization identity.
    pub authorization_identity: String,
    /// Executable identity.
    pub executable_identity: String,
    /// Plan identity.
    pub plan_identity: String,
    /// Requested safe capability labels.
    pub requested_capabilities: Vec<String>,
    /// Enforced safe capability labels.
    pub enforced_capabilities: Vec<String>,
    /// Explicit network posture.
    pub network: NetworkPosture,
    /// Safe root labels only.
    pub root_labels: Vec<String>,
    /// Safe execution status.
    pub status: String,
    /// Safe issue class.
    pub issue: Option<String>,
    /// Exit classification.
    pub exit: ExitClass,
    /// Cleanup status.
    pub cleanup: String,
    /// Bounded option count.
    pub option_count: usize,
    /// Receipt redaction marker.
    pub redaction: &'static str,
    /// Backend-specific non-claims.
    pub non_claims: [&'static str; NON_CLAIMS.len()],
    /// Domain-separated BLAKE3 receipt identity.
    pub identity: String,
}

/// Guardrail error class returned by pure and shell boundaries.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuardrailErrorKind {
    /// Checked profile is malformed or unsafe.
    Profile,
    /// Authorization is denied or identity-drifted.
    Authorization,
    /// Root facts are missing or invalid.
    Root,
    /// Environment admission failed.
    Environment,
    /// Executable or cohort admission failed.
    Executable,
    /// Plan conflicts or identity drift occurred.
    Plan,
    /// Sealed option transport failed.
    Transport,
    /// Guarded child launch failed.
    Launch,
    /// Receipt input was unsafe.
    Receipt,
    /// Command-line input was malformed.
    Cli,
}

/// Redaction-safe guardrail error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GuardrailError {
    kind: GuardrailErrorKind,
    field: &'static str,
}

impl GuardrailError {
    /// Builds a redaction-safe error from a stable class and field label.
    #[must_use]
    pub const fn new(kind: GuardrailErrorKind, field: &'static str) -> Self {
        Self { kind, field }
    }

    /// Returns the stable error class.
    #[must_use]
    pub const fn kind(&self) -> GuardrailErrorKind {
        self.kind
    }

    /// Returns the safe field class.
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }
}

impl std::fmt::Display for GuardrailError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "guardrail {:?} failure at {}",
            self.kind, self.field
        )
    }
}

impl std::error::Error for GuardrailError {}
