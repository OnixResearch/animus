//! Strict generated configuration admission.

use serde::{Deserialize, Serialize};

const SCHEMA: &str = "animus.local-daemon.v1";
const PROTOCOL_VERSION: u16 = 1;
const TRANSPORT: &str = "unix-stream";
const ENDPOINT_AUTHORITY: &str = "explicit-capability-root";
const WORKER_FEATURE: &str = "live-worker";
const REQUIRED_NON_CLAIMS: [&str; 7] = [
    "not-remote-security",
    "not-security-sandbox",
    "not-multi-machine-supervision",
    "not-high-availability",
    "not-service-manager-integration",
    "not-production-readiness",
    "not-release-eligibility",
];
const MAX_LOCAL_NAME_BYTES: usize = 128;

/// Closed configuration failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalDaemonConfigError {
    /// Input is not valid JSON or has an unknown field.
    Json,
    /// An exact component selection differs from the reviewed cohort.
    Component,
    /// A name or numeric bound is invalid.
    Bound,
    /// Required non-claims differ from the reviewed set.
    NonClaims,
}

impl core::fmt::Display for LocalDaemonConfigError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "local daemon configuration rejected: {self:?}")
    }
}

impl std::error::Error for LocalDaemonConfigError {}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct RawLocalDaemonConfig {
    schema: String,
    protocol_version: u16,
    transport: String,
    endpoint_authority: String,
    endpoint_name: String,
    registry_name: String,
    worker_binary: String,
    worker_feature: String,
    max_frame_bytes: u32,
    max_events: u32,
    max_attachments: u16,
    max_workers: u16,
    operation_deadline_ms: u64,
    cancel_poll_ms: u64,
    non_claims: Vec<String>,
}

/// Strict immutable local daemon configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedLocalDaemonConfig {
    raw: RawLocalDaemonConfig,
    identity: String,
}

impl ValidatedLocalDaemonConfig {
    /// Returns the deterministic BLAKE3 configuration identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the explicit endpoint name below the supplied capability root.
    #[must_use]
    pub fn endpoint_name(&self) -> &str {
        self.raw.endpoint_name.as_str()
    }

    /// Returns the explicit registry file name below the capability root.
    #[must_use]
    pub fn registry_name(&self) -> &str {
        self.raw.registry_name.as_str()
    }

    /// Returns the exact worker binary name.
    #[must_use]
    pub fn worker_binary(&self) -> &str {
        self.raw.worker_binary.as_str()
    }

    /// Returns the maximum frame bytes.
    #[must_use]
    pub const fn max_frame_bytes(&self) -> u32 {
        self.raw.max_frame_bytes
    }

    /// Returns the maximum retained events.
    #[must_use]
    pub const fn max_events(&self) -> u32 {
        self.raw.max_events
    }

    /// Returns the maximum simultaneous attachments per worker.
    #[must_use]
    pub const fn max_attachments(&self) -> u16 {
        self.raw.max_attachments
    }

    /// Returns the maximum registry worker count.
    #[must_use]
    pub const fn max_workers(&self) -> u16 {
        self.raw.max_workers
    }

    /// Returns the operation deadline in milliseconds.
    #[must_use]
    pub const fn operation_deadline_ms(&self) -> u64 {
        self.raw.operation_deadline_ms
    }

    /// Returns the cancellation polling interval in milliseconds.
    #[must_use]
    pub const fn cancel_poll_ms(&self) -> u64 {
        self.raw.cancel_poll_ms
    }

    /// Returns required claim exclusions.
    #[must_use]
    pub fn non_claims(&self) -> &[String] {
        self.raw.non_claims.as_slice()
    }
}

/// Parses generated Nickel JSON and revalidates every field.
///
/// # Errors
///
/// Returns [`LocalDaemonConfigError`] for malformed, widened, unknown, or
/// incompatible input.
pub fn parse_local_daemon_config(
    input: &[u8],
) -> Result<ValidatedLocalDaemonConfig, LocalDaemonConfigError> {
    let raw: RawLocalDaemonConfig =
        serde_json::from_slice(input).map_err(|_error| LocalDaemonConfigError::Json)?;
    if raw.schema != SCHEMA
        || raw.protocol_version != PROTOCOL_VERSION
        || raw.transport != TRANSPORT
        || raw.endpoint_authority != ENDPOINT_AUTHORITY
        || raw.worker_feature != WORKER_FEATURE
    {
        return Err(LocalDaemonConfigError::Component);
    }
    validate_local_name(raw.endpoint_name.as_str())?;
    validate_local_name(raw.registry_name.as_str())?;
    validate_local_name(raw.worker_binary.as_str())?;
    if raw.max_frame_bytes == 0
        || raw.max_events == 0
        || raw.max_attachments == 0
        || raw.max_workers == 0
        || raw.operation_deadline_ms == 0
        || raw.cancel_poll_ms == 0
        || raw.cancel_poll_ms >= raw.operation_deadline_ms
    {
        return Err(LocalDaemonConfigError::Bound);
    }
    let required = REQUIRED_NON_CLAIMS.map(String::from);
    if raw.non_claims != required {
        return Err(LocalDaemonConfigError::NonClaims);
    }
    let canonical = serde_json::to_vec(&raw).map_err(|_error| LocalDaemonConfigError::Json)?;
    let identity = blake3::hash(canonical.as_slice()).to_hex().to_string();
    Ok(ValidatedLocalDaemonConfig { raw, identity })
}

fn validate_local_name(value: &str) -> Result<(), LocalDaemonConfigError> {
    if value.is_empty()
        || value.len() > MAX_LOCAL_NAME_BYTES
        || value == "."
        || value == ".."
        || value.contains('/')
        || value.contains('\\')
        || value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control())
    {
        return Err(LocalDaemonConfigError::Bound);
    }
    Ok(())
}
