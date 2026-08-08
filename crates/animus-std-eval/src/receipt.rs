//! Metadata-only host receipt projection and claim boundaries.

use animus_eval_core::{hash_identity, validate_identity};

use crate::REQUIRED_HOST_NON_CLAIMS;

/// Host receipt terminal status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostReceiptStatus {
    /// Host run completed and cleanup finished.
    Completed,
    /// Host run reached terminal persistence but cleanup remains owed.
    CleanupRequired,
    /// Host run was blocked before terminal persistence.
    Blocked,
    /// Host run failed under bounded service evidence.
    Failed,
    /// Host run was cancelled.
    Cancelled,
    /// Host run reached its deadline.
    TimedOut,
}

impl HostReceiptStatus {
    const fn label(self) -> &'static str {
        match self {
            Self::Completed => "completed",
            Self::CleanupRequired => "cleanup_required",
            Self::Blocked => "blocked",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
            Self::TimedOut => "timed_out",
        }
    }
}

/// Host receipt claim class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostReceiptClaim {
    /// Bounded host observations only.
    BoundedObservation,
    /// Host confinement.
    HostConfinement,
    /// Network enforcement.
    NetworkEnforcement,
    /// Provider correctness.
    ProviderCorrectness,
    /// Persistence durability beyond selected evidence.
    PersistenceDurability,
    /// Exactly-once remote effects.
    ExactlyOnceRemoteEffects,
    /// Verifier soundness.
    VerifierSoundness,
    /// Artifact authenticity.
    ArtifactAuthenticity,
    /// Benchmark validity.
    BenchmarkValidity,
    /// Authoritative completion.
    AuthoritativeCompletion,
    /// Production readiness.
    ProductionReadiness,
    /// Release eligibility.
    ReleaseEligibility,
}

/// Explicit prohibited-content facts supplied to receipt projection.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct HostReceiptLeakFlags {
    /// Raw task content is present.
    pub task_content: bool,
    /// Raw transcript content is present.
    pub transcript_content: bool,
    /// Hidden tests are present.
    pub hidden_tests: bool,
    /// Reasoning is present.
    pub reasoning: bool,
    /// Tool body is present.
    pub tool_body: bool,
    /// Provider body is present.
    pub provider_body: bool,
    /// Credential is present.
    pub credential: bool,
    /// Environment value is present.
    pub environment_value: bool,
    /// Physical path is present.
    pub physical_path: bool,
    /// Unbounded log is present.
    pub unbounded_log: bool,
}

impl HostReceiptLeakFlags {
    const fn any(self) -> bool {
        self.task_content
            || self.transcript_content
            || self.hidden_tests
            || self.reasoning
            || self.tool_body
            || self.provider_body
            || self.credential
            || self.environment_value
            || self.physical_path
            || self.unbounded_log
    }
}

/// Borrowed host receipt draft.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostReceiptDraft<'a> {
    /// Host profile identity.
    pub profile_identity: &'a str,
    /// Evaluation identity.
    pub evaluation_identity: &'a str,
    /// Native plan identity.
    pub plan_identity: &'a str,
    /// Host preflight identity.
    pub preflight_identity: &'a str,
    /// Service catalog identity.
    pub service_catalog_identity: &'a str,
    /// Gateway admission identity.
    pub gateway_identity: &'a str,
    /// Verifier identity.
    pub verifier_identity: &'a str,
    /// Artifact-manifest identity.
    pub artifact_identity: &'a str,
    /// Storage-target identity.
    pub storage_identity: &'a str,
    /// Cleanup evidence identity.
    pub cleanup_identity: &'a str,
    /// Terminal status.
    pub status: HostReceiptStatus,
    /// Bounded service observation count.
    pub observation_count: u32,
    /// Redacted item count.
    pub redaction_count: u32,
    /// Unresolved cleanup obligation count.
    pub cleanup_obligation_count: u32,
    /// Explicit leak flags.
    pub leaks: HostReceiptLeakFlags,
    /// Requested claim classes.
    pub claims: &'a [HostReceiptClaim],
}

/// Validated metadata-only host receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostReceipt {
    /// Canonical receipt identity.
    pub identity: String,
    /// Host profile identity.
    pub profile_identity: String,
    /// Evaluation identity.
    pub evaluation_identity: String,
    /// Plan identity.
    pub plan_identity: String,
    /// Terminal status.
    pub status: HostReceiptStatus,
    /// Bounded observation count.
    pub observation_count: u32,
    /// Redacted item count.
    pub redaction_count: u32,
    /// Unresolved cleanup obligation count.
    pub cleanup_obligation_count: u32,
    /// Explicit bounded non-claims.
    pub non_claims: Vec<&'static str>,
}

/// Host receipt validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostReceiptError {
    /// Identity is malformed.
    Identity,
    /// Count is inconsistent.
    Bound,
    /// Prohibited raw content is present.
    Leak,
    /// Unsupported claim was requested.
    Overclaim,
}

/// Project one deterministic metadata-only host receipt.
///
/// # Errors
/// Returns identity, bound, leak, or overclaim before emitting an identity.
pub fn project_host_receipt(draft: &HostReceiptDraft<'_>) -> Result<HostReceipt, HostReceiptError> {
    for identity in [
        draft.profile_identity,
        draft.evaluation_identity,
        draft.plan_identity,
        draft.preflight_identity,
        draft.service_catalog_identity,
        draft.gateway_identity,
        draft.verifier_identity,
        draft.artifact_identity,
        draft.storage_identity,
        draft.cleanup_identity,
    ] {
        validate_identity(identity).map_err(|_| HostReceiptError::Identity)?;
    }
    if draft.redaction_count > draft.observation_count {
        return Err(HostReceiptError::Bound);
    }
    if draft.leaks.any() {
        return Err(HostReceiptError::Leak);
    }
    if draft
        .claims
        .iter()
        .any(|claim| *claim != HostReceiptClaim::BoundedObservation)
    {
        return Err(HostReceiptError::Overclaim);
    }
    let counts = format!(
        "{}:{}:{}",
        draft.observation_count, draft.redaction_count, draft.cleanup_obligation_count
    );
    let identity = hash_identity(
        "animus.native-evaluation-host.receipt.v1",
        &[
            draft.profile_identity,
            draft.evaluation_identity,
            draft.plan_identity,
            draft.preflight_identity,
            draft.service_catalog_identity,
            draft.gateway_identity,
            draft.verifier_identity,
            draft.artifact_identity,
            draft.storage_identity,
            draft.cleanup_identity,
            draft.status.label(),
            &counts,
        ],
    );
    Ok(HostReceipt {
        identity,
        profile_identity: draft.profile_identity.into(),
        evaluation_identity: draft.evaluation_identity.into(),
        plan_identity: draft.plan_identity.into(),
        status: draft.status,
        observation_count: draft.observation_count,
        redaction_count: draft.redaction_count,
        cleanup_obligation_count: draft.cleanup_obligation_count,
        non_claims: REQUIRED_HOST_NON_CLAIMS.to_vec(),
    })
}
