use std::collections::BTreeSet;

use super::{
    BLAKE3_HEX_LENGTH, MCP_CONFIG_DOMAIN, McpHostConfig, McpHostConfigDraft, REQUIRED_NON_CLAIMS,
    ToolPolicy,
};

const CONFIG_SCHEMA: &str = "animus.mcp-client-host.v1";
const MCP_PROTOCOL: &str = "2025-06-18";
const MCP_COHORT_REVISION: &str = "b25c0874bf0ba699a58e21ef06f659d839659de3";
const MCP_SCHEMA_BLAKE3: &str = "528797f6970a8f24175daf1f994c742f2fe24f0b8230b9456885452a08132a90";
const MCP_LICENSE_BLAKE3: &str = "fd12707e19e104da035c74958a676b0567b7dc2df01875ae07f058b79e59422f";
const TRANSPORT_STDIO: &str = "stdio";
const MAX_SAFE_LABEL_BYTES: usize = 128;
const MAX_CONFIG_ARGUMENTS: usize = 32;
const MAX_TOOL_POLICIES: usize = 64;
const MAX_BYTE_LIMIT: usize = 16_777_216;
const MAX_COUNT_LIMIT: usize = 65_536;
const MAX_TIMEOUT_MS: u64 = 300_000;

/// Bounded configuration validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfigError {
    /// Generated JSON could not be decoded.
    MalformedProjection,
    /// Schema or reviewed cohort identity drifted.
    CohortDrift,
    /// A required label or hash was malformed.
    InvalidIdentity,
    /// An unsupported or ambient behavior was enabled.
    UnsafeBehavior,
    /// A finite collection was empty, duplicated, or over budget.
    InvalidCollection,
    /// A numeric bound was zero or internally inconsistent.
    InvalidLimit,
    /// A mandatory non-claim was absent.
    MissingNonClaim,
}

/// Loads and revalidates the deterministic generated Nickel projection.
///
/// # Errors
///
/// Returns a bounded configuration error when JSON or policy validation fails.
pub fn load_generated_config() -> Result<McpHostConfig, ConfigError> {
    parse_config(include_str!("../../../../config/generated/mcp-client-host.json").as_bytes())
}

/// Parses and revalidates one deterministic JSON projection.
///
/// # Errors
///
/// Returns a bounded configuration error without process or credential effects.
// r[impl agent_harness_runtime.mcp_client_host.configuration]
pub fn parse_config(bytes: &[u8]) -> Result<McpHostConfig, ConfigError> {
    let draft = serde_json::from_slice::<McpHostConfigDraft>(bytes)
        .map_err(|_| ConfigError::MalformedProjection)?;
    validate_config(draft)
}

/// Revalidates a generated MCP host configuration and binds its BLAKE3 identity.
///
/// # Errors
///
/// Returns a bounded configuration error before any process or credential effect.
pub fn validate_config(draft: McpHostConfigDraft) -> Result<McpHostConfig, ConfigError> {
    validate_cohort(&draft)?;
    validate_server(&draft.server)?;
    validate_non_claims(draft.non_claims.as_slice())?;
    let canonical = serde_json::to_vec(&draft).map_err(|_| ConfigError::MalformedProjection)?;
    let identity = framed_hash(MCP_CONFIG_DOMAIN, canonical.as_slice());
    Ok(McpHostConfig { draft, identity })
}

fn validate_cohort(draft: &McpHostConfigDraft) -> Result<(), ConfigError> {
    if draft.schema != CONFIG_SCHEMA
        || draft.cohort_revision != MCP_COHORT_REVISION
        || draft.cohort_schema_blake3 != MCP_SCHEMA_BLAKE3
        || draft.cohort_license_blake3 != MCP_LICENSE_BLAKE3
    {
        return Err(ConfigError::CohortDrift);
    }
    validate_hash(draft.cohort_schema_blake3.as_str())?;
    validate_hash(draft.cohort_license_blake3.as_str())
}

fn validate_server(server: &super::ServerConfigDraft) -> Result<(), ConfigError> {
    validate_safe_label(server.id.as_str())?;
    validate_safe_label(server.executable_label.as_str())?;
    validate_hash(server.executable_identity.as_str())?;
    validate_safe_label(server.working_root_capability.as_str())?;
    validate_hash(server.working_root_identity.as_str())?;
    validate_hash(server.policy_identity.as_str())?;
    if server.transport != TRANSPORT_STDIO || server.protocol_version != MCP_PROTOCOL {
        return Err(ConfigError::UnsafeBehavior);
    }
    if server.auto_discovery || server.shell_fallback || server.inherit_environment {
        return Err(ConfigError::UnsafeBehavior);
    }
    validate_labels(server.arguments.as_slice(), MAX_CONFIG_ARGUMENTS, false)?;
    validate_labels(
        server.environment_keys.as_slice(),
        server.limits.max_environment_keys,
        false,
    )?;
    validate_labels(
        server.credential_refs.as_slice(),
        server.limits.max_environment_keys,
        false,
    )?;
    validate_limits(server.limits)?;
    validate_tool_policies(server.tool_policies.as_slice())
}

fn validate_limits(limits: super::McpLimits) -> Result<(), ConfigError> {
    if limits.numeric_values().contains(&0)
        || byte_limits(limits)
            .iter()
            .any(|value| *value > MAX_BYTE_LIMIT)
        || count_limits(limits)
            .iter()
            .any(|value| *value > MAX_COUNT_LIMIT)
        || timeout_limits(limits)
            .iter()
            .any(|value| *value > MAX_TIMEOUT_MS)
    {
        return Err(ConfigError::InvalidLimit);
    }
    if limits.max_schema_bytes_each > limits.max_schema_bytes_total
        || limits.cancellation_poll_ms > limits.initialize_timeout_ms
        || limits.cancellation_poll_ms > limits.discovery_timeout_ms
        || limits.cancellation_poll_ms > limits.call_timeout_ms
        || limits.shutdown_timeout_ms > limits.teardown_timeout_ms
    {
        return Err(ConfigError::InvalidLimit);
    }
    Ok(())
}

const fn byte_limits(limits: super::McpLimits) -> [usize; 9] {
    [
        limits.max_frame_bytes,
        limits.max_tool_name_bytes,
        limits.max_description_bytes,
        limits.max_schema_bytes_each,
        limits.max_schema_bytes_total,
        limits.max_resource_metadata_bytes,
        limits.max_diagnostic_bytes,
        limits.max_result_bytes,
        limits.max_instruction_bytes,
    ]
}

fn count_limits(limits: super::McpLimits) -> [usize; 4] {
    [
        limits.max_tools,
        limits.max_arguments,
        limits.max_environment_keys,
        limits.max_calls_per_generation as usize,
    ]
}

const fn timeout_limits(limits: super::McpLimits) -> [u64; 6] {
    [
        limits.initialize_timeout_ms,
        limits.discovery_timeout_ms,
        limits.call_timeout_ms,
        limits.cancellation_poll_ms,
        limits.shutdown_timeout_ms,
        limits.teardown_timeout_ms,
    ]
}

fn validate_tool_policies(policies: &[ToolPolicy]) -> Result<(), ConfigError> {
    if policies.is_empty() || policies.len() > MAX_TOOL_POLICIES {
        return Err(ConfigError::InvalidCollection);
    }
    let mut names = BTreeSet::new();
    for policy in policies {
        validate_safe_label(policy.name.as_str())?;
        if !names.insert(policy.name.as_str()) {
            return Err(ConfigError::InvalidCollection);
        }
        validate_labels(policy.capability_labels.as_slice(), MAX_TOOL_POLICIES, true)?;
        validate_labels(
            policy.resource_policy_refs.as_slice(),
            MAX_TOOL_POLICIES,
            true,
        )?;
        validate_labels(
            policy.ability_policy_refs.as_slice(),
            MAX_TOOL_POLICIES,
            true,
        )?;
        validate_safe_label(policy.result_bounds.as_str())?;
    }
    Ok(())
}

fn validate_non_claims(values: &[String]) -> Result<(), ConfigError> {
    let mut unique = BTreeSet::new();
    for value in values {
        validate_safe_label(value.as_str())?;
        if !unique.insert(value.as_str()) {
            return Err(ConfigError::InvalidCollection);
        }
    }
    if REQUIRED_NON_CLAIMS
        .iter()
        .any(|required| !unique.contains(required))
    {
        return Err(ConfigError::MissingNonClaim);
    }
    Ok(())
}

fn validate_labels(
    values: &[String],
    maximum: usize,
    require_non_empty: bool,
) -> Result<(), ConfigError> {
    if values.len() > maximum || (require_non_empty && values.is_empty()) {
        return Err(ConfigError::InvalidCollection);
    }
    let mut unique = BTreeSet::new();
    for value in values {
        validate_safe_label(value.as_str())?;
        if !unique.insert(value.as_str()) {
            return Err(ConfigError::InvalidCollection);
        }
    }
    Ok(())
}

pub(crate) fn validate_safe_label(value: &str) -> Result<(), ConfigError> {
    if value.is_empty() || value.len() > MAX_SAFE_LABEL_BYTES {
        return Err(ConfigError::InvalidIdentity);
    }
    if value
        .bytes()
        .any(|byte| !(byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')))
    {
        return Err(ConfigError::InvalidIdentity);
    }
    Ok(())
}

pub(crate) fn validate_hash(value: &str) -> Result<(), ConfigError> {
    if value.len() != BLAKE3_HEX_LENGTH
        || value
            .bytes()
            .any(|byte| !byte.is_ascii_hexdigit() || byte.is_ascii_uppercase())
    {
        return Err(ConfigError::InvalidIdentity);
    }
    Ok(())
}

pub(crate) fn framed_hash(domain: &str, bytes: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&[0]);
    hasher.update(&(bytes.len() as u64).to_le_bytes());
    hasher.update(bytes);
    hasher.finalize().to_hex().to_string()
}
