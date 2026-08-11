mod data;
mod validate;

pub use data::{
    McpHostConfig, McpHostConfigDraft, McpLimits, ServerConfigDraft, ServerPosture, ToolPolicy,
};
pub use validate::{ConfigError, load_generated_config, parse_config, validate_config};
pub(crate) use validate::{framed_hash, validate_hash, validate_safe_label};

pub(crate) const BLAKE3_HEX_LENGTH: usize = 64;
pub(crate) const LIMIT_FIELD_COUNT: usize = 19;
pub(crate) const MCP_CONFIG_DOMAIN: &str = "animus.mcp-client-host.config.v1";
pub(crate) const REQUIRED_NON_CLAIMS: &[&str] = &[
    "server_trust",
    "server_correctness",
    "tool_safety",
    "sandboxing",
    "network_isolation",
    "exactly_once_effects",
    "mcp_parity",
    "production_readiness",
    "release_eligibility",
];
