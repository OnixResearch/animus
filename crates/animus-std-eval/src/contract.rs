//! Host request, artifact, storage, lifecycle, and outcome contracts.

use animus_adapter_eval::{
    AdmissionError, BindingError, GatewayError, ObservationClass, ServiceCatalog,
    ValidatedObservation,
};
use animus_eval_core::{RecipePlan, hash_identity, validate_identity};

use crate::{
    EvaluationHostPolicy, GatewayInvocation, HostReceipt, MAX_HOST_ARTIFACT_BYTES,
    MAX_HOST_ARTIFACTS, MAX_HOST_OUTPUT_BYTES,
};

/// Maximum bytes in one logical capability-relative path.
pub const MAX_LOGICAL_PATH_BYTES: usize = 512;
/// Maximum logical path segments.
pub const MAX_LOGICAL_PATH_SEGMENTS: usize = 64;

/// One allowlisted artifact entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArtifactEntry<'a> {
    /// Capability-relative logical path.
    pub logical_path: &'a str,
    /// Exact bounded content or source identity supplied by the host.
    pub identity: &'a str,
    /// Bounded byte length.
    pub byte_count: u64,
    /// Whether policy explicitly allows the entry.
    pub allowlisted: bool,
    /// Whether the observed entry is a symbolic or other link.
    pub link: bool,
    /// Whether the entry contains hidden verifier material.
    pub hidden_test: bool,
}

/// Ordered artifact manifest supplied by the caller.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ArtifactManifest<'a> {
    /// Exact manifest identity.
    pub identity: &'a str,
    /// Ordered entries.
    pub entries: &'a [ArtifactEntry<'a>],
}

/// Explicit capability-rooted persistence destination.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StorageTarget<'a> {
    /// Root capability identity. This is not a physical path.
    pub root_capability_identity: &'a str,
    /// Capability-relative logical output path.
    pub logical_path: &'a str,
    /// Exact output-format identity.
    pub output_identity: &'a str,
    /// Maximum output bytes admitted by this destination.
    pub max_bytes: u64,
}

/// Exact host-level identity bindings shared by all service requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostBindings<'a> {
    /// Evaluation identity.
    pub evaluation_identity: &'a str,
    /// Task identity.
    pub task_identity: &'a str,
    /// Episode identity.
    pub episode_identity: &'a str,
    /// Semantic role identity.
    pub role_identity: &'a str,
    /// Effective model identity.
    pub model_identity: &'a str,
    /// Harness identity.
    pub harness_identity: &'a str,
    /// Solver environment identity.
    pub solver_environment_identity: &'a str,
    /// Distinct verifier environment identity.
    pub verifier_environment_identity: &'a str,
    /// Tool-catalog identity.
    pub tool_identity: &'a str,
    /// Verifier identity.
    pub verifier_identity: &'a str,
    /// Rubric identity.
    pub rubric_identity: &'a str,
    /// Artifact-manifest identity.
    pub artifact_identity: &'a str,
    /// Adapter service-policy identity.
    pub service_policy_identity: &'a str,
    /// Cancellation-scope identity.
    pub cancellation_identity: &'a str,
    /// Deadline-policy identity.
    pub deadline_identity: &'a str,
    /// Positive attempt number.
    pub attempt: u32,
    /// Exact attempt-lineage identity.
    pub attempt_lineage_identity: &'a str,
    /// Per-run correlation identity.
    pub correlation_identity: &'a str,
}

/// Host-only auxiliary service kind.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AuxiliaryServiceKind {
    /// Explicit tool execution or observation service.
    Tool,
    /// Explicit deadline observation service.
    Timer,
    /// Explicit cancellation observation service.
    Cancellation,
    /// Explicit cleanup service.
    Cleanup,
}

impl AuxiliaryServiceKind {
    /// Return the stable service label.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Tool => "tool",
            Self::Timer => "timer",
            Self::Cancellation => "cancellation",
            Self::Cleanup => "cleanup",
        }
    }
}

/// One explicit host-only auxiliary service descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuxiliaryServiceDescriptor<'a> {
    /// Exact auxiliary service kind.
    pub kind: AuxiliaryServiceKind,
    /// Exact implementation identity.
    pub implementation_identity: &'a str,
    /// Exact policy identity.
    pub policy_identity: &'a str,
    /// Whether the service is currently available.
    pub available: bool,
}

/// Explicit host-only auxiliary service catalog.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AuxiliaryServiceCatalog<'a> {
    /// Exact catalog identity.
    pub identity: &'a str,
    /// Separate auxiliary services.
    pub services: &'a [AuxiliaryServiceDescriptor<'a>],
}

/// Complete public host entrypoint request.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct HostRunRequest<'a> {
    /// Validated native recipe plan.
    pub recipe_plan: &'a RecipePlan,
    /// Exact host bindings.
    pub bindings: HostBindings<'a>,
    /// Explicit evaluation adapter service catalog.
    pub service_catalog: ServiceCatalog<'a>,
    /// Explicit tool, timer, cancellation, and cleanup service catalog.
    pub auxiliary_services: AuxiliaryServiceCatalog<'a>,
    /// Checked host policy.
    pub policy: &'a EvaluationHostPolicy,
    /// Gateway invocation and durable ordering evidence.
    pub gateway: GatewayInvocation<'a>,
    /// Allowlisted artifact manifest.
    pub artifacts: ArtifactManifest<'a>,
    /// Explicit capability-rooted output destination.
    pub storage: StorageTarget<'a>,
    /// Requested concurrent episode permits.
    pub requested_episode_parallelism: u32,
    /// Requested concurrent agent permits inside the episode.
    pub requested_agent_parallelism: u32,
    /// Bounded service items requested per call.
    pub service_items: u32,
    /// Bounded service bytes requested per call.
    pub service_bytes: u64,
}

/// Cleanup obligation supplied to the host cleanup port.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CleanupObligation<'a> {
    /// Evaluation identity.
    pub evaluation_identity: &'a str,
    /// Episode identity.
    pub episode_identity: &'a str,
    /// Cancellation identity.
    pub cancellation_identity: &'a str,
    /// Exact run-seal identity.
    pub seal_identity: &'a str,
}

/// Host-supplied cleanup observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CleanupObservation {
    /// Exact cleanup evidence identity.
    pub identity: String,
    /// Whether all selected cleanup work completed.
    pub complete: bool,
    /// Bounded unresolved obligation count.
    pub unresolved_count: u32,
}

/// Host terminal status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostStatus {
    /// All admitted service work, persistence, reopen, and cleanup completed.
    Completed,
    /// Terminal output exists, but cleanup remains owed.
    CleanupRequired,
}

/// Completed public host-entrypoint result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostRunResult {
    /// Terminal status.
    pub status: HostStatus,
    /// Canonical host-run identity.
    pub identity: String,
    /// Validated observations in deterministic dispatch order.
    pub observations: Vec<ValidatedObservation>,
    /// Peak active episode permits.
    pub peak_episode_permits: u32,
    /// Peak active agent permits.
    pub peak_agent_permits: u32,
    /// Metadata-only host receipt.
    pub receipt: HostReceipt,
}

/// Typed host execution failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostError {
    /// Host request identity or shape is invalid.
    Identity,
    /// A selected bound is zero or exceeded.
    Bound,
    /// Host policy is invalid or does not match the request.
    Policy,
    /// Service catalog admission failed before effects.
    Admission(AdmissionError),
    /// Service request or observation binding failed.
    Binding(BindingError),
    /// Gateway posture or durable ordering failed.
    Gateway(GatewayError),
    /// Artifact handoff is unsafe or over budget.
    Artifact,
    /// Storage destination is unsafe or over budget.
    Storage,
    /// Cancellation became current.
    Cancelled,
    /// Deadline became current.
    TimedOut,
    /// A service returned a bounded non-success class.
    Service(ObservationClass),
    /// A late write targeted a sealed run.
    Sealed,
    /// Cleanup remains owed after a non-terminal service failure.
    CleanupRequired,
    /// Metadata-only receipt projection failed.
    Receipt,
}

/// Validate an allowlisted collision-free artifact manifest.
///
/// # Errors
/// Returns artifact when identity, path, allowlist, link, hidden material, count,
/// collision, or byte bounds are invalid.
pub fn validate_artifact_manifest(
    manifest: &ArtifactManifest<'_>,
    policy: &EvaluationHostPolicy,
) -> Result<String, HostError> {
    validate_identity(manifest.identity).map_err(|_| HostError::Artifact)?;
    let entry_count = u32::try_from(manifest.entries.len()).map_err(|_| HostError::Artifact)?;
    if manifest.entries.is_empty()
        || entry_count > policy.artifacts.max_entries
        || entry_count > MAX_HOST_ARTIFACTS
    {
        return Err(HostError::Artifact);
    }
    let mut aggregate_bytes = 0_u64;
    let mut material = Vec::with_capacity(manifest.entries.len());
    for (index, entry) in manifest.entries.iter().enumerate() {
        validate_logical_path(entry.logical_path).map_err(|_| HostError::Artifact)?;
        validate_identity(entry.identity).map_err(|_| HostError::Artifact)?;
        if !entry.allowlisted
            || entry.link
            || entry.hidden_test
            || entry.byte_count == 0
            || manifest.entries[..index]
                .iter()
                .any(|prior| prior.logical_path == entry.logical_path)
        {
            return Err(HostError::Artifact);
        }
        aggregate_bytes = aggregate_bytes
            .checked_add(entry.byte_count)
            .ok_or(HostError::Artifact)?;
        material.push(hash_identity(
            "animus.native-evaluation-host.artifact-entry.v1",
            &[
                entry.logical_path,
                entry.identity,
                &entry.byte_count.to_string(),
            ],
        ));
    }
    if aggregate_bytes > policy.artifacts.max_bytes || aggregate_bytes > MAX_HOST_ARTIFACT_BYTES {
        return Err(HostError::Artifact);
    }
    Ok(hash_identity(
        "animus.native-evaluation-host.artifact-manifest.v1",
        &[manifest.identity, &material.join(",")],
    ))
}

/// Validate one explicit capability-rooted storage target.
///
/// # Errors
/// Returns storage when identities, logical path, or byte bounds are invalid.
pub fn validate_storage_target(
    target: &StorageTarget<'_>,
    policy: &EvaluationHostPolicy,
) -> Result<String, HostError> {
    for identity in [target.root_capability_identity, target.output_identity] {
        validate_identity(identity).map_err(|_| HostError::Storage)?;
    }
    validate_logical_path(target.logical_path).map_err(|_| HostError::Storage)?;
    if target.max_bytes == 0
        || target.max_bytes > policy.persistence.max_record_bytes
        || target.max_bytes > MAX_HOST_OUTPUT_BYTES
    {
        return Err(HostError::Storage);
    }
    Ok(hash_identity(
        "animus.native-evaluation-host.storage-target.v1",
        &[
            target.root_capability_identity,
            target.logical_path,
            target.output_identity,
            &target.max_bytes.to_string(),
        ],
    ))
}

/// Validate a capability-relative logical path without filesystem access.
///
/// # Errors
/// Returns identity for absolute, empty, dot, parent, backslash, oversized, or
/// unsafe paths.
pub fn validate_logical_path(path: &str) -> Result<(), HostError> {
    if path.is_empty()
        || path.len() > MAX_LOGICAL_PATH_BYTES
        || path.starts_with('/')
        || path.contains('\\')
    {
        return Err(HostError::Identity);
    }
    let segments: Vec<&str> = path.split('/').collect();
    if segments.is_empty()
        || segments.len() > MAX_LOGICAL_PATH_SEGMENTS
        || segments.iter().any(|segment| {
            segment.is_empty()
                || matches!(*segment, "." | "..")
                || !segment
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
        })
    {
        return Err(HostError::Identity);
    }
    Ok(())
}
