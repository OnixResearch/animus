//! Strict live-terminal configuration admission.

use serde::{Deserialize, Serialize};

const SCHEMA: &str = "animus.live-terminal-agent-host.v1";
const PROTOCOL_VERSION: u16 = 1;
const DAEMON_MODE: &str = "daemon";
const DIRECT_MODE: &str = "direct";
const ENDPOINT_AUTHORITY: &str = "explicit-capability-root";
const TERMINAL_BACKEND: &str = "crossterm";
const NON_CLAIM_COUNT: usize = 7;
const MAX_EVENT_BATCH: u32 = 1024;
const MAX_PROMPT_BYTES: u32 = 65_536;
const MAX_POLL_INTERVAL_MS: u64 = 1_000;
const MAX_DIMENSION: u16 = 4_096;
const MAX_LABEL_BYTES: usize = 256;
const EXPECTED_NON_CLAIMS: &[&str] = &[
    "not-arbitrary-terminal-compatibility",
    "not-remote-security",
    "not-crash-proof-restoration",
    "not-pty-migration",
    "not-security-sandbox",
    "not-production-readiness",
    "not-release-eligibility",
];

/// Explicit connection selection. Direct mode needs an injected service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConnectionMode {
    /// Connect to one explicitly constructed local daemon endpoint.
    Daemon,
    /// Use one explicitly injected in-process client service.
    Direct,
}

/// Strict immutable live-terminal configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedTerminalConfig {
    connection_mode: ConnectionMode,
    endpoint_name: String,
    root_id: String,
    client_id: String,
    max_event_batch: u32,
    max_prompt_bytes: u32,
    poll_interval_ms: u64,
    max_width: u16,
    max_height: u16,
    identity: String,
}

impl ValidatedTerminalConfig {
    /// Returns the explicit connection mode.
    #[must_use]
    pub const fn connection_mode(&self) -> ConnectionMode {
        self.connection_mode
    }

    /// Returns the explicit endpoint leaf name.
    #[must_use]
    pub fn endpoint_name(&self) -> &str {
        self.endpoint_name.as_str()
    }

    /// Returns the selected root identity.
    #[must_use]
    pub fn root_id(&self) -> &str {
        self.root_id.as_str()
    }

    /// Returns the terminal client identity.
    #[must_use]
    pub fn client_id(&self) -> &str {
        self.client_id.as_str()
    }

    /// Returns the maximum session events handled in one iteration.
    #[must_use]
    pub const fn max_event_batch(&self) -> u32 {
        self.max_event_batch
    }

    /// Returns the maximum prompt bytes admitted from the terminal.
    #[must_use]
    pub const fn max_prompt_bytes(&self) -> u32 {
        self.max_prompt_bytes
    }

    /// Returns the terminal polling interval in milliseconds.
    #[must_use]
    pub const fn poll_interval_ms(&self) -> u64 {
        self.poll_interval_ms
    }

    /// Returns the maximum viewport width.
    #[must_use]
    pub const fn max_width(&self) -> u16 {
        self.max_width
    }

    /// Returns the maximum viewport height.
    #[must_use]
    pub const fn max_height(&self) -> u16 {
        self.max_height
    }

    /// Returns the canonical BLAKE3 configuration identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }
}

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct RawTerminalConfig {
    schema: String,
    protocol_version: u16,
    connection_mode: String,
    endpoint_authority: String,
    endpoint_name: String,
    root_id: String,
    client_id: String,
    terminal_backend: String,
    raw_mode: bool,
    alternate_screen: bool,
    hide_cursor: bool,
    max_event_batch: u32,
    max_prompt_bytes: u32,
    poll_interval_ms: u64,
    max_width: u16,
    max_height: u16,
    non_claims: Vec<String>,
}

/// Closed configuration failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalConfigError {
    /// JSON was malformed or included an unknown field.
    Json,
    /// A fixed schema, protocol, authority, or backend value drifted.
    Contract,
    /// A label, identity, endpoint, or dimension was invalid.
    InvalidValue,
    /// A finite configured bound was zero or too large.
    Bound,
    /// Required non-claims were missing, reordered, or changed.
    NonClaims,
}

impl core::fmt::Display for TerminalConfigError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "live terminal configuration rejected: {self:?}")
    }
}

impl std::error::Error for TerminalConfigError {}

/// Parses and strictly validates generated terminal configuration JSON.
///
/// # Errors
///
/// Returns [`TerminalConfigError`] before terminal, endpoint, or signal effects.
pub fn parse_terminal_config(bytes: &[u8]) -> Result<ValidatedTerminalConfig, TerminalConfigError> {
    let raw: RawTerminalConfig =
        serde_json::from_slice(bytes).map_err(|_error| TerminalConfigError::Json)?;
    let connection_mode = match raw.connection_mode.as_str() {
        DAEMON_MODE => ConnectionMode::Daemon,
        DIRECT_MODE => ConnectionMode::Direct,
        _other => return Err(TerminalConfigError::Contract),
    };
    if raw.schema != SCHEMA
        || raw.protocol_version != PROTOCOL_VERSION
        || raw.endpoint_authority != ENDPOINT_AUTHORITY
        || raw.terminal_backend != TERMINAL_BACKEND
        || !raw.raw_mode
        || !raw.alternate_screen
        || !raw.hide_cursor
    {
        return Err(TerminalConfigError::Contract);
    }
    validate_label(raw.endpoint_name.as_str())?;
    validate_label(raw.root_id.as_str())?;
    validate_label(raw.client_id.as_str())?;
    validate_bounds(&raw)?;
    if raw.non_claims.len() != NON_CLAIM_COUNT
        || raw
            .non_claims
            .iter()
            .map(String::as_str)
            .ne(EXPECTED_NON_CLAIMS.iter().copied())
    {
        return Err(TerminalConfigError::NonClaims);
    }
    let canonical = serde_json::to_vec(&raw).map_err(|_error| TerminalConfigError::Json)?;
    let identity = blake3::hash(canonical.as_slice()).to_hex().to_string();
    Ok(ValidatedTerminalConfig {
        connection_mode,
        endpoint_name: raw.endpoint_name,
        root_id: raw.root_id,
        client_id: raw.client_id,
        max_event_batch: raw.max_event_batch,
        max_prompt_bytes: raw.max_prompt_bytes,
        poll_interval_ms: raw.poll_interval_ms,
        max_width: raw.max_width,
        max_height: raw.max_height,
        identity,
    })
}

fn validate_label(value: &str) -> Result<(), TerminalConfigError> {
    if value.is_empty()
        || value.len() > MAX_LABEL_BYTES
        || value == "."
        || value == ".."
        || value.contains('/')
        || value.contains('\\')
        || value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return Err(TerminalConfigError::InvalidValue);
    }
    Ok(())
}

fn validate_bounds(raw: &RawTerminalConfig) -> Result<(), TerminalConfigError> {
    if raw.max_event_batch == 0
        || raw.max_event_batch > MAX_EVENT_BATCH
        || raw.max_prompt_bytes == 0
        || raw.max_prompt_bytes > MAX_PROMPT_BYTES
        || raw.poll_interval_ms == 0
        || raw.poll_interval_ms > MAX_POLL_INTERVAL_MS
        || raw.max_width == 0
        || raw.max_width > MAX_DIMENSION
        || raw.max_height == 0
        || raw.max_height > MAX_DIMENSION
    {
        return Err(TerminalConfigError::Bound);
    }
    Ok(())
}
