use serde::{Deserialize, Serialize};

/// Required or optional server startup posture.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerPosture {
    /// Failure blocks host readiness.
    Required,
    /// Failure remains visible but does not block unrelated required servers.
    Optional,
}

/// Finite limits for one MCP server generation.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct McpLimits {
    /// Maximum newline-delimited JSON-RPC frame bytes.
    pub max_frame_bytes: usize,
    /// Maximum discovered tool count.
    pub max_tools: usize,
    /// Maximum UTF-8 tool-name bytes.
    pub max_tool_name_bytes: usize,
    /// Maximum UTF-8 description bytes.
    pub max_description_bytes: usize,
    /// Maximum bytes in one canonical input schema.
    pub max_schema_bytes_each: usize,
    /// Maximum aggregate canonical input-schema bytes.
    pub max_schema_bytes_total: usize,
    /// Maximum resource metadata bytes.
    pub max_resource_metadata_bytes: usize,
    /// Maximum bounded diagnostic bytes.
    pub max_diagnostic_bytes: usize,
    /// Maximum tool-result bytes.
    pub max_result_bytes: usize,
    /// Maximum server-instruction bytes.
    pub max_instruction_bytes: usize,
    /// Maximum object argument count.
    pub max_arguments: usize,
    /// Maximum explicit environment key count.
    pub max_environment_keys: usize,
    /// Maximum calls on one server generation.
    pub max_calls_per_generation: u32,
    /// Initialization deadline in milliseconds.
    pub initialize_timeout_ms: u64,
    /// Discovery deadline in milliseconds.
    pub discovery_timeout_ms: u64,
    /// Tool-call deadline in milliseconds.
    pub call_timeout_ms: u64,
    /// Cancellation polling interval in milliseconds.
    pub cancellation_poll_ms: u64,
    /// Graceful shutdown deadline in milliseconds.
    pub shutdown_timeout_ms: u64,
    /// Forced teardown deadline in milliseconds.
    pub teardown_timeout_ms: u64,
}

impl McpLimits {
    pub(crate) fn numeric_values(self) -> [u64; super::LIMIT_FIELD_COUNT] {
        [
            self.max_frame_bytes as u64,
            self.max_tools as u64,
            self.max_tool_name_bytes as u64,
            self.max_description_bytes as u64,
            self.max_schema_bytes_each as u64,
            self.max_schema_bytes_total as u64,
            self.max_resource_metadata_bytes as u64,
            self.max_diagnostic_bytes as u64,
            self.max_result_bytes as u64,
            self.max_instruction_bytes as u64,
            self.max_arguments as u64,
            self.max_environment_keys as u64,
            u64::from(self.max_calls_per_generation),
            self.initialize_timeout_ms,
            self.discovery_timeout_ms,
            self.call_timeout_ms,
            self.cancellation_poll_ms,
            self.shutdown_timeout_ms,
            self.teardown_timeout_ms,
        ]
    }
}

/// Exact policy for one configured MCP tool.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ToolPolicy {
    /// Exact server tool name.
    pub name: String,
    /// Animus capability labels required for dispatch.
    pub capability_labels: Vec<String>,
    /// Resource policy identities checked by existing owners.
    pub resource_policy_refs: Vec<String>,
    /// Ability policy identities checked by existing owners.
    pub ability_policy_refs: Vec<String>,
    /// Bounded result-shape identity.
    pub result_bounds: String,
}

/// Untrusted generated server configuration before Rust validation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServerConfigDraft {
    /// Exact server identity.
    pub id: String,
    /// Required or optional startup posture.
    pub posture: ServerPosture,
    /// Exact transport label.
    pub transport: String,
    /// Caller-owned executable label.
    pub executable_label: String,
    /// Caller-supplied executable identity.
    pub executable_identity: String,
    /// Exact direct-process arguments without a shell.
    pub arguments: Vec<String>,
    /// Explicit environment key allowlist.
    pub environment_keys: Vec<String>,
    /// Caller-owned credential references.
    pub credential_refs: Vec<String>,
    /// Safe working-root capability label.
    pub working_root_capability: String,
    /// Exact working-root identity.
    pub working_root_identity: String,
    /// Exact protocol cohort.
    pub protocol_version: String,
    /// Exact host policy identity.
    pub policy_identity: String,
    /// Finite generation limits.
    pub limits: McpLimits,
    /// Exact per-tool policies.
    pub tool_policies: Vec<ToolPolicy>,
    /// Ambient server discovery posture.
    pub auto_discovery: bool,
    /// Shell-command fallback posture.
    pub shell_fallback: bool,
    /// Ambient environment inheritance posture.
    pub inherit_environment: bool,
}

/// Generated host configuration before Rust validation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct McpHostConfigDraft {
    /// Configuration schema identity.
    pub schema: String,
    /// Reviewed MCP specification revision.
    pub cohort_revision: String,
    /// BLAKE3 identity of the reviewed protocol schema.
    pub cohort_schema_blake3: String,
    /// BLAKE3 identity of the reviewed license file.
    pub cohort_license_blake3: String,
    /// Exact server configuration.
    pub server: ServerConfigDraft,
    /// Mandatory bounded non-claims.
    pub non_claims: Vec<String>,
}

/// Validated MCP host configuration with a domain-separated identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpHostConfig {
    pub(crate) draft: McpHostConfigDraft,
    pub(crate) identity: String,
}

impl McpHostConfig {
    /// Returns the deterministic configuration identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the exact server configuration.
    #[must_use]
    pub const fn server(&self) -> &ServerConfigDraft {
        &self.draft.server
    }

    /// Returns the mandatory non-claims.
    #[must_use]
    pub fn non_claims(&self) -> &[String] {
        self.draft.non_claims.as_slice()
    }

    /// Returns the reviewed cohort revision.
    #[must_use]
    pub fn cohort_revision(&self) -> &str {
        self.draft.cohort_revision.as_str()
    }
}
