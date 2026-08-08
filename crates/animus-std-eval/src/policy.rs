//! Checked host policy parsing and explicit-path loading.

use std::{fs, path::Path};

use animus_eval_core::{validate_blake3_identity, validate_identity};
use serde::Deserialize;

use crate::{EVALUATION_HOST_SCHEMA, VERIFIERS_HOST_LICENSE, VERIFIERS_HOST_REVISION};

/// Maximum checked policy bytes accepted by the host shell.
pub const MAX_HOST_POLICY_BYTES: usize = 65_536;
/// Maximum concurrent episodes accepted by this host contract.
pub const MAX_EPISODE_CONCURRENCY: u32 = 1_024;
/// Maximum concurrent agent calls in one episode.
pub const MAX_AGENT_CONCURRENCY: u32 = 1_024;
/// Maximum allowed gateway bind classes.
pub const MAX_GATEWAY_BIND_MODES: usize = 2;
/// Maximum bounded gateway body bytes.
pub const MAX_GATEWAY_BODY_BYTES: u64 = 16_777_216;
/// Maximum bounded gateway stream events.
pub const MAX_GATEWAY_STREAM_EVENTS: u32 = 65_536;
/// Maximum artifact entries in one handoff.
pub const MAX_HOST_ARTIFACTS: u32 = 4_096;
/// Maximum aggregate artifact bytes in one handoff.
pub const MAX_HOST_ARTIFACT_BYTES: u64 = 67_108_864;
/// Maximum safe output bytes in one persistence operation.
pub const MAX_HOST_OUTPUT_BYTES: u64 = 67_108_864;
/// Maximum cleanup obligations retained by one run.
pub const MAX_CLEANUP_OBLIGATIONS: u32 = 4_096;
/// Canonical reviewed host-source cohort identity.
pub const VERIFIERS_HOST_SOURCE_COHORT_IDENTITY: &str =
    "b3:c6336b188596db9b18cb2f0a721c9f5bc551e619342e89807e20ec45f3bb676f";

/// Required host receipt non-claims.
pub const REQUIRED_HOST_NON_CLAIMS: &[&str] = &[
    "not-host-confinement-or-network-enforcement",
    "not-provider-or-tool-correctness",
    "not-persistence-durability-or-exactly-once-effects",
    "not-verifier-soundness-or-artifact-authenticity",
    "not-benchmark-or-training-fitness",
    "not-authoritative-completion",
    "not-production-or-release-readiness",
];

/// Checked host policy.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EvaluationHostPolicy {
    /// Policy schema.
    pub schema: String,
    /// Stable host profile identity.
    pub profile_id: String,
    /// Reviewed source cohort.
    pub source_cohort: SourceCohortPolicy,
    /// Separate concurrency limits.
    pub concurrency: ConcurrencyPolicy,
    /// Gateway posture.
    pub gateway: GatewayPolicy,
    /// Verifier posture.
    pub verifier: VerifierPolicy,
    /// Artifact transfer posture.
    pub artifacts: ArtifactPolicy,
    /// Persistence posture.
    pub persistence: PersistencePolicy,
    /// Cleanup posture.
    pub cleanup: CleanupPolicy,
    /// Receipt and claim posture.
    pub evidence: HostEvidencePolicy,
}

/// Reviewed source cohort metadata.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SourceCohortPolicy {
    /// Upstream repository.
    pub repository: String,
    /// Exact revision.
    pub revision: String,
    /// SPDX-compatible license label.
    pub license: String,
    /// Canonical BLAKE3 cohort identity.
    pub identity: String,
}

/// Separate outer and inner concurrency limits.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConcurrencyPolicy {
    /// Maximum concurrently active episodes.
    pub episodes: u32,
    /// Maximum active agent calls inside one episode.
    pub agents_per_episode: u32,
}

/// Allowed local gateway bind class.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum GatewayBindMode {
    /// Loopback-only bind selected by the host.
    Loopback,
    /// Caller-admitted bind with a separate exact capability.
    CallerAdmitted,
}

/// Checked local gateway policy.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct GatewayPolicy {
    /// Allowed bind classes.
    pub bind_modes: Vec<GatewayBindMode>,
    /// Maximum request or response body bytes.
    pub max_body_bytes: u64,
    /// Maximum stream events.
    pub max_stream_events: u32,
    /// Whether model and state capabilities must differ.
    pub separate_capabilities_required: bool,
    /// Whether accepted-turn evidence is required before dispatch.
    pub accepted_turn_required: bool,
    /// Whether terminal delivery requires matching commit evidence.
    pub terminal_commit_required: bool,
    /// Whether only exact non-streaming retries can coalesce.
    pub exact_non_streaming_coalescing_only: bool,
    /// Whether response rewriting is allowed.
    pub response_rewriting_allowed: bool,
    /// Whether wildcard network access is allowed.
    pub wildcard_network_allowed: bool,
    /// Whether raw provider bodies can enter safe records.
    pub raw_provider_bodies_allowed: bool,
}

/// Verifier isolation policy.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct VerifierPolicy {
    /// Whether authoritative-capable profiles require an isolated verifier.
    pub isolated_required: bool,
    /// Whether shared verifier results remain advisory.
    pub shared_is_advisory: bool,
    /// Whether hidden verifier material can enter the solver environment.
    pub hidden_material_in_solver_allowed: bool,
}

/// Artifact transfer policy.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ArtifactPolicy {
    /// Maximum artifact entries.
    pub max_entries: u32,
    /// Maximum aggregate bytes.
    pub max_bytes: u64,
    /// Whether an explicit allowlist is required.
    pub allowlist_required: bool,
    /// Whether no-follow handling is required.
    pub no_follow_required: bool,
    /// Whether hidden tests can cross the handoff boundary.
    pub hidden_tests_allowed: bool,
}

/// Persistence synchronization class.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum DurabilityClass {
    /// Caller accepts process-observed replacement only.
    ProcessObserved,
    /// Caller requires file synchronization evidence.
    FileSynced,
    /// Caller requires file and parent-directory synchronization evidence.
    FileAndDirectorySynced,
}

/// Explicit persistence policy.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PersistencePolicy {
    /// Maximum safe record bytes.
    pub max_record_bytes: u64,
    /// Selected synchronization class.
    pub durability: DurabilityClass,
    /// Whether complete bytes must exist before replacement.
    pub complete_bytes_required: bool,
    /// Whether replacement must be atomic under the selected host mechanism.
    pub atomic_replace_required: bool,
    /// Whether reopened records require strict validation.
    pub strict_reopen_required: bool,
    /// Whether malformed records can suppress rerun.
    pub malformed_record_can_skip: bool,
}

/// Cleanup policy.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct CleanupPolicy {
    /// Maximum retained cleanup obligations.
    pub max_obligations: u32,
    /// Whether terminal persistence can hide cleanup failure.
    pub hide_post_terminal_failure: bool,
    /// Whether late writes are accepted after sealing.
    pub late_writes_allowed: bool,
}

/// Metadata-only evidence policy.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HostEvidencePolicy {
    /// Whether safe receipts use metadata only.
    pub metadata_only: bool,
    /// Whether raw task or transcript content is allowed.
    pub raw_content_allowed: bool,
    /// Whether physical paths are allowed.
    pub physical_paths_allowed: bool,
    /// Whether authoritative completion claims are allowed.
    pub completion_claim_allowed: bool,
    /// Mandatory non-claims.
    pub non_claims: Vec<String>,
}

/// Checked policy error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostPolicyError {
    /// Explicit policy path could not be read.
    Io,
    /// Policy bytes exceed the configured bound.
    TooLarge,
    /// JSON syntax or shape is invalid.
    Decode,
    /// Schema, source, identity, or posture is invalid.
    Invalid,
}

/// Load policy bytes from one caller-selected path, then parse them through the pure core.
///
/// # Errors
/// Returns a bounded I/O, size, decode, or validation class.
pub fn load_evaluation_host_policy(path: &Path) -> Result<EvaluationHostPolicy, HostPolicyError> {
    let bytes = fs::read(path).map_err(|_| HostPolicyError::Io)?;
    if bytes.len() > MAX_HOST_POLICY_BYTES {
        return Err(HostPolicyError::TooLarge);
    }
    parse_evaluation_host_policy(&bytes)
}

/// Parse and validate caller-supplied checked policy bytes without I/O.
///
/// # Errors
/// Returns decode or validation when the checked projection is malformed or unsafe.
pub fn parse_evaluation_host_policy(bytes: &[u8]) -> Result<EvaluationHostPolicy, HostPolicyError> {
    if bytes.len() > MAX_HOST_POLICY_BYTES {
        return Err(HostPolicyError::TooLarge);
    }
    let policy: EvaluationHostPolicy =
        serde_json::from_slice(bytes).map_err(|_| HostPolicyError::Decode)?;
    validate_evaluation_host_policy(&policy)?;
    Ok(policy)
}

/// Validate the exact checked host policy.
///
/// # Errors
/// Returns invalid for wrong identities, zero or excessive bounds, or widened authority.
pub fn validate_evaluation_host_policy(
    policy: &EvaluationHostPolicy,
) -> Result<(), HostPolicyError> {
    if policy.schema != EVALUATION_HOST_SCHEMA
        || policy.source_cohort.repository != "PrimeIntellect-ai/verifiers"
        || policy.source_cohort.revision != VERIFIERS_HOST_REVISION
        || policy.source_cohort.license != VERIFIERS_HOST_LICENSE
        || policy.source_cohort.identity != VERIFIERS_HOST_SOURCE_COHORT_IDENTITY
    {
        return Err(HostPolicyError::Invalid);
    }
    validate_identity(&policy.profile_id).map_err(|_| HostPolicyError::Invalid)?;
    validate_blake3_identity(&policy.source_cohort.identity)
        .map_err(|_| HostPolicyError::Invalid)?;
    validate_bounds(policy)?;
    validate_posture(policy)?;
    Ok(())
}

fn validate_bounds(policy: &EvaluationHostPolicy) -> Result<(), HostPolicyError> {
    if policy.concurrency.episodes == 0
        || policy.concurrency.episodes > MAX_EPISODE_CONCURRENCY
        || policy.concurrency.agents_per_episode == 0
        || policy.concurrency.agents_per_episode > MAX_AGENT_CONCURRENCY
        || policy.gateway.max_body_bytes == 0
        || policy.gateway.max_body_bytes > MAX_GATEWAY_BODY_BYTES
        || policy.gateway.max_stream_events == 0
        || policy.gateway.max_stream_events > MAX_GATEWAY_STREAM_EVENTS
        || policy.artifacts.max_entries == 0
        || policy.artifacts.max_entries > MAX_HOST_ARTIFACTS
        || policy.artifacts.max_bytes == 0
        || policy.artifacts.max_bytes > MAX_HOST_ARTIFACT_BYTES
        || policy.persistence.max_record_bytes == 0
        || policy.persistence.max_record_bytes > MAX_HOST_OUTPUT_BYTES
        || policy.cleanup.max_obligations == 0
        || policy.cleanup.max_obligations > MAX_CLEANUP_OBLIGATIONS
    {
        return Err(HostPolicyError::Invalid);
    }
    Ok(())
}

fn validate_posture(policy: &EvaluationHostPolicy) -> Result<(), HostPolicyError> {
    let bind_modes_valid = !policy.gateway.bind_modes.is_empty()
        && policy.gateway.bind_modes.len() <= MAX_GATEWAY_BIND_MODES
        && !policy
            .gateway
            .bind_modes
            .iter()
            .enumerate()
            .any(|(index, mode)| policy.gateway.bind_modes[..index].contains(mode));
    let required_non_claims = REQUIRED_HOST_NON_CLAIMS.iter().all(|required| {
        policy
            .evidence
            .non_claims
            .iter()
            .any(|actual| actual == required)
    });
    if !bind_modes_valid
        || !policy.gateway.separate_capabilities_required
        || !policy.gateway.accepted_turn_required
        || !policy.gateway.terminal_commit_required
        || !policy.gateway.exact_non_streaming_coalescing_only
        || policy.gateway.response_rewriting_allowed
        || policy.gateway.wildcard_network_allowed
        || policy.gateway.raw_provider_bodies_allowed
        || !policy.verifier.isolated_required
        || !policy.verifier.shared_is_advisory
        || policy.verifier.hidden_material_in_solver_allowed
        || !policy.artifacts.allowlist_required
        || !policy.artifacts.no_follow_required
        || policy.artifacts.hidden_tests_allowed
        || !policy.persistence.complete_bytes_required
        || !policy.persistence.atomic_replace_required
        || !policy.persistence.strict_reopen_required
        || policy.persistence.malformed_record_can_skip
        || policy.cleanup.hide_post_terminal_failure
        || policy.cleanup.late_writes_allowed
        || !policy.evidence.metadata_only
        || policy.evidence.raw_content_allowed
        || policy.evidence.physical_paths_allowed
        || policy.evidence.completion_claim_allowed
        || !required_non_claims
    {
        return Err(HostPolicyError::Invalid);
    }
    Ok(())
}
