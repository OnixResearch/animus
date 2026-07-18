use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

/// Reviewed line protocol identity.
pub const PROTOCOL_IDENTITY: &str = "animus.lean-loop.protocol.v1";
/// Reviewed JSON envelope schema identity.
pub const ENVELOPE_SCHEMA_IDENTITY: &str = "animus.lean-loop.envelope.v1";
/// Exact Lean toolchain cohort used by the repository package.
pub const TOOLCHAIN_COHORT: &str = "lean4-4.30.0";
/// Generated Lean-loop profile schema identity.
pub const PROFILE_SCHEMA: &str = "animus.lean-loop.profile.v1";
pub(crate) const PROFILE_DOMAIN: &str = "animus.lean-loop.profile.v1";
/// Domain separator for executable artifact identities.
pub const ARTIFACT_DOMAIN: &str = "animus.lean-loop.artifact.v1";
/// Domain separator for the reviewed Lean proof-source identity.
pub const PROOF_SOURCE_DOMAIN: &str = "animus.lean-loop.proof-source.v1";
/// Exact domain-separated BLAKE3 identity of the reviewed proof sources.
pub const EXPECTED_PROOF_SOURCE_IDENTITY: &str =
    "d8e0ffd0ae32507bb7051c5b77e162f76484debde12a32d09f40616fed7c0c54";
pub(crate) const RECEIPT_DOMAIN: &str = "animus.lean-loop.receipt.v1";
/// Redaction statement attached to safe Lean-loop receipts.
pub const REDACTION_MARKER: &str = "raw-turn-model-tool-provider-source-path-environment-credential-stderr-and-hidden-material-omitted";
pub(crate) const EXPECTED_DECISIONS: [&str; 4] = ["model", "tool", "respond", "stop"];
/// Explicit boundaries that every Lean-loop receipt preserves.
pub const NON_CLAIMS: [&str; 9] = [
    "not-arbitrary-policy-correctness",
    "not-lean-compiler-or-runtime-soundness",
    "not-child-process-confinement",
    "not-atomic-executable-path-pinning",
    "not-provider-or-tool-correctness",
    "not-theorem-to-rust-equivalence",
    "not-authoritative-completion",
    "not-whole-agent-correctness",
    "not-production-readiness",
];

/// Safe failure classes emitted by the Lean-loop host boundary.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeanLoopFailureClass {
    /// Generated profile validation failed.
    ProfileRejected,
    /// Executable content did not match the admitted BLAKE3 identity.
    ArtifactIdentityMismatch,
    /// Executable or working-directory selection was invalid.
    ExecutableRejected,
    /// Required sandbox evidence was absent.
    SandboxEvidenceMissing,
    /// Child process launch failed.
    SpawnFailure,
    /// Protocol activation did not return the exact handshake acknowledgement.
    HandshakeFailure,
    /// Writing a bounded frame failed.
    WriteFailure,
    /// Flushing the child stdin stream failed.
    FlushFailure,
    /// An inbound or outbound frame exceeded its configured bound.
    FrameLimitExceeded,
    /// A bounded channel or pipe could not accept more data.
    Backpressure,
    /// The configured response poll budget was exhausted.
    ResponseDeadline,
    /// Cancellation was observed after a turn could have become externally visible.
    CancellationAmbiguous,
    /// The child closed stdout before a required frame arrived.
    InputEof,
    /// A child pipe was broken.
    BrokenPipe,
    /// JSON or envelope shape validation failed.
    MalformedResponse,
    /// The child returned an unreviewed decision kind.
    UnknownDecision,
    /// Sequence, profile, artifact, or tool correlation did not match.
    CorrelationMismatch,
    /// Current Rust DTO reconstruction rejected the candidate.
    DecisionRejected,
    /// The child exited unsuccessfully.
    NonZeroExit,
    /// Child diagnostics exceeded the admitted byte bound.
    StderrLimitExceeded,
    /// Bounded child cleanup failed.
    CleanupFailure,
    /// A caller attempted reuse after terminal ambiguity.
    TerminalTransport,
    /// Receipt input contained unsafe or promotive material.
    ReceiptRejected,
}

impl LeanLoopFailureClass {
    /// Returns the stable safe label for this failure class.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProfileRejected => "profile_rejected",
            Self::ArtifactIdentityMismatch => "artifact_identity_mismatch",
            Self::ExecutableRejected => "executable_rejected",
            Self::SandboxEvidenceMissing => "sandbox_evidence_missing",
            Self::SpawnFailure => "spawn_failure",
            Self::HandshakeFailure => "handshake_failure",
            Self::WriteFailure => "write_failure",
            Self::FlushFailure => "flush_failure",
            Self::FrameLimitExceeded => "frame_limit_exceeded",
            Self::Backpressure => "backpressure",
            Self::ResponseDeadline => "response_deadline",
            Self::CancellationAmbiguous => "cancellation_ambiguous",
            Self::InputEof => "input_eof",
            Self::BrokenPipe => "broken_pipe",
            Self::MalformedResponse => "malformed_response",
            Self::UnknownDecision => "unknown_decision",
            Self::CorrelationMismatch => "correlation_mismatch",
            Self::DecisionRejected => "decision_rejected",
            Self::NonZeroExit => "non_zero_exit",
            Self::StderrLimitExceeded => "stderr_limit_exceeded",
            Self::CleanupFailure => "cleanup_failure",
            Self::TerminalTransport => "terminal_transport",
            Self::ReceiptRejected => "receipt_rejected",
        }
    }
}

/// Redacted error returned by a Lean-loop profile, protocol, or process boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeanLoopError {
    failure: LeanLoopFailureClass,
    field: &'static str,
}

impl LeanLoopError {
    /// Builds a redacted error from a safe class and field label.
    #[must_use]
    pub const fn new(failure: LeanLoopFailureClass, field: &'static str) -> Self {
        Self { failure, field }
    }

    /// Returns the safe failure class.
    #[must_use]
    pub const fn failure(&self) -> LeanLoopFailureClass {
        self.failure
    }

    /// Returns the safe field label.
    #[must_use]
    pub const fn field(&self) -> &'static str {
        self.field
    }
}

impl core::fmt::Display for LeanLoopError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "lean loop {} at {}",
            self.failure.as_str(),
            self.field
        )
    }
}

impl std::error::Error for LeanLoopError {}

/// Sandbox evidence required by a checked profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeanSandboxPosture {
    /// Host enforcement evidence is required.
    Required,
    /// Only explicitly labeled local fixture execution is admitted.
    FixtureOnly,
}

/// Host observation supplied for one child launch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeanSandboxObservation {
    /// External sandbox enforcement was observed.
    Enforced,
    /// Execution is deliberately classified as an unconstrained fixture.
    FixtureOnly,
    /// No sandbox or fixture classification was supplied.
    Missing,
}

/// Finite resource and polling bounds loaded from checked Nickel output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeanLoopBounds {
    /// Maximum bytes in one line-delimited protocol frame.
    pub frame_bytes: usize,
    /// Maximum bytes in one model, tool, or response text field.
    pub text_bytes: usize,
    /// Maximum run-local protocol sequence.
    pub sequence_limit: u64,
    /// Maximum captured child stderr bytes.
    pub diagnostic_bytes: usize,
    /// Maximum handshake poll count.
    pub startup_poll_count: u32,
    /// Maximum turn-response poll count.
    pub response_poll_count: u32,
    /// Maximum shutdown-response poll count.
    pub shutdown_poll_count: u32,
    /// Duration between bounded response polls.
    pub poll_interval: Duration,
    /// Maximum cleanup poll count before forced termination.
    pub cleanup_poll_count: u32,
    /// Maximum tool argument count.
    pub argument_count: usize,
    /// Maximum mutation-target count.
    pub target_count: usize,
    /// Maximum required-capability count.
    pub capability_count: usize,
}

/// Validated runtime projection of the Nickel Lean-loop profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeanLoopProfile {
    /// Domain-separated BLAKE3 identity of the checked projection.
    pub profile_identity: String,
    /// Domain-separated BLAKE3 identity of the reviewed Lean proof sources.
    pub proof_source_identity: String,
    /// Finite transport and candidate bounds.
    pub bounds: LeanLoopBounds,
    /// Required sandbox evidence posture.
    pub sandbox_posture: LeanSandboxPosture,
}

/// Explicit host-owned child launch inputs.
#[derive(Clone, Debug)]
pub struct LeanLaunchDescriptor {
    /// Absolute executable path selected by the host.
    pub executable: PathBuf,
    /// Expected domain-separated BLAKE3 executable identity.
    pub expected_artifact_identity: String,
    /// Absolute child working directory selected by the host.
    pub working_directory: PathBuf,
    /// Current sandbox evidence classification.
    pub sandbox_observation: LeanSandboxObservation,
    /// Explicit cancellation signal for this child lifecycle.
    pub cancellation: Arc<AtomicBool>,
}

/// Bounded cleanup result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LeanCleanupStatus {
    /// Child exited and all reader threads joined normally.
    Completed,
    /// Child exceeded cleanup polls and was killed.
    Forced,
    /// Cleanup or diagnostic validation failed.
    Failed,
}

impl LeanCleanupStatus {
    /// Returns the stable receipt label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::Forced => "forced",
            Self::Failed => "failed",
        }
    }
}

/// Safe bounded transport observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeanTransportObservation {
    /// Total inbound and outbound frame count.
    pub frame_count: u32,
    /// Total inbound and outbound byte count.
    pub byte_count: u64,
    /// Primary safe transport failure, if any.
    pub failure: Option<LeanLoopFailureClass>,
    /// Latest cleanup classification.
    pub cleanup: LeanCleanupStatus,
}

/// Safe caller-supplied context for receipt projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LeanReceiptContext<'a> {
    /// Verified executable artifact identity.
    pub artifact_identity: &'a str,
    /// Last candidate decision class.
    pub decision_class: &'a str,
    /// Sandbox observation bound to this execution.
    pub sandbox: LeanSandboxObservation,
}

/// Metadata-only Lean-loop evidence receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LeanLoopReceipt {
    /// Receipt schema identity.
    pub schema: &'static str,
    /// Checked profile identity.
    pub profile_identity: String,
    /// Verified executable artifact identity.
    pub artifact_identity: String,
    /// Reviewed Lean proof-source identity.
    pub proof_source_identity: String,
    /// Last candidate decision class.
    pub decision_class: String,
    /// Safe failure class, if any.
    pub failure_class: Option<String>,
    /// Sandbox observation class.
    pub sandbox_class: String,
    /// Cleanup class.
    pub cleanup_class: String,
    /// Bounded protocol frame count.
    pub frame_count: u32,
    /// Bounded protocol byte count.
    pub byte_count: u64,
    /// Required omission statement.
    pub redaction_marker: &'static str,
    /// Explicit receipt non-claims.
    pub non_claims: Vec<&'static str>,
    /// Domain-separated BLAKE3 receipt identity.
    pub receipt_identity: String,
}
