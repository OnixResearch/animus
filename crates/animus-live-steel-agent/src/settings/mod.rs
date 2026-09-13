mod access;
mod cohort;
mod limits;
mod projection;
mod shape;
mod validation;

const CONFIG_SCHEMA: &str = "animus.live-steel-agent-shell.v1";
const COMPONENT_PROVIDER: &str = "animus-std-openai:reqwest-tool-stream";
const COMPONENT_STORE: &str = "animus-std:durable-session-store";
const COMPONENT_CODING_SESSION: &str = "animus-std:CodingSessionHost";
const COMPONENT_KERNEL: &str = "animus-steel:SteelRlmKernel";
const COMPONENT_BRIDGE: &str = "animus-runtime:execute_bridged_cell";
const COMPONENT_EXECUTOR: &str = "animus-std-rlm-host:RlmHostExecutor";
const COMPONENT_CHILD: &str = "animus-std:ConcreteChildSessionService";
const COMPONENT_SKILLS: &str = "animus-steel:load_skill_modules";
const COMPONENT_CONTROL: &str = "animus-live-steel-agent:ExplicitRunControl";
const COMPONENT_EVENTS: &str = "animus-live-steel-agent:MetadataEventSink";
const COMPONENT_RECEIPTS: &str = "animus-live-steel-agent:RunReceipt";
const LOCAL_DSPARK_PROVIDER_ID: &str = "local-dspark";
const OPENAI_PROVIDER_ID: &str = "openai";
const OPENROUTER_PROVIDER_ID: &str = "openrouter";
const LOCAL_DSPARK_CREDENTIAL_REFERENCE: &str = "brokered:injected-local-dspark-api-key";
const OPENAI_CREDENTIAL_REFERENCE: &str = "brokered:injected-openai-api-key";
const OPENROUTER_CREDENTIAL_REFERENCE: &str = "brokered:injected-openrouter-api-key";
const LOCAL_DSPARK_CREDENTIAL_SOURCE: &str = "injected-local-dspark-api-key";
const OPENAI_CREDENTIAL_SOURCE: &str = "injected-openai-api-key";
const OPENROUTER_CREDENTIAL_SOURCE: &str = "injected-openrouter-api-key";
const DISABLED_REASONING_EFFORT: &str = "none";
const LOW_REASONING_EFFORT: &str = "low";
const DURABILITY: &str = "sync_all_each_commit";
const LOCKING: &str = "exclusive_create_new";
const PRIVACY: &str = "replay_private";
const EVIDENCE_POLICY: &str = "metadata-only";
const EVIDENCE_REDACTION: &str =
    "prompt-provider-tool-steel-host-child-credential-and-path-content-omitted";
const SKILL_SOURCE: &str = "(define (live-shell-add-two value) (+ value 2))";
const BLAKE3_HEX_CHARS: usize = 64;
const MAX_SAFE_LABEL_CHARS: usize = 256;
const MAX_SKILLS: usize = 64;
const MAX_ITEMS: usize = 64;
const MAX_NON_CLAIMS: usize = 32;
const REQUIRED_OPERATIONS: &[&str] = &["context_read", "subagent"];
const ALLOWED_OPERATIONS: &[&str] = &["context_read", "subagent", "tool"];
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "provider_correctness",
    "tool_correctness",
    "task_success",
    "sandboxing",
    "daemon_readiness",
    "terminal_readiness",
    "production_readiness",
    "release_eligibility",
];

/// Stable configuration failure without private values.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveSteelAgentConfigError {
    /// JSON did not match the closed data shape.
    Malformed,
    /// A fixed component or schema identity drifted.
    ComponentMismatch,
    /// An endpoint, provider, tool, or credential cohort drifted.
    ProviderMismatch,
    /// A durable-store posture drifted.
    StoreMismatch,
    /// A service label, route, or selected binding was invalid.
    ServiceMismatch,
    /// A required finite bound was absent or zero.
    InvalidBound,
    /// A BLAKE3 identity was malformed.
    InvalidIdentity,
    /// A list was empty, duplicated, over its bound, or outside the closed set.
    InvalidCollection,
    /// A reviewed skill descriptor or source identity drifted.
    SkillMismatch,
    /// Evidence policy omitted a required redaction or non-claim.
    EvidenceMismatch,
}

impl core::fmt::Display for LiveSteelAgentConfigError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "live Steel agent configuration rejected: {self:?}"
        )
    }
}

impl std::error::Error for LiveSteelAgentConfigError {}

/// Closed validated configuration used by the composition root.
#[derive(Clone, Debug)]
pub struct ValidatedLiveSteelAgentConfig {
    raw: shape::LiveSteelAgentConfig,
    config_identity: String,
    rlm_policy: animus_adapter_rlm_kernel::RlmKernelPolicy,
    selected_model: animus_std::SelectedModel,
    skill_modules: Vec<animus_adapter_rlm_kernel::SteelSkillModuleDescriptor>,
}

/// Parses and revalidates the deterministic Nickel export.
///
/// # Errors
///
/// Returns [`crate::LiveSteelAgentConfigError`] before effects for malformed, missing,
/// widened, duplicated, unsupported, or mismatched values.
pub fn parse_live_steel_agent_config(
    json: &[u8],
) -> Result<ValidatedLiveSteelAgentConfig, LiveSteelAgentConfigError> {
    // r[impl agent_harness_runtime.live_steel_agent_shell.configuration]
    let raw: shape::LiveSteelAgentConfig =
        serde_json::from_slice(json).map_err(|_error| LiveSteelAgentConfigError::Malformed)?;
    validation::validate_fixed_components(&raw)?;
    cohort::validate_provider(&raw.provider)?;
    validation::validate_store(&raw.store)?;
    validation::validate_services(&raw.services)?;
    limits::validate_bounds(raw.bounds)?;
    validation::validate_authority(&raw.authority)?;
    validation::validate_operations(&raw.admitted_operations)?;
    validation::validate_labels(&raw.admitted_child_profiles, MAX_ITEMS, false)?;
    validation::validate_labels(&raw.command_profiles, MAX_ITEMS, true)?;
    validation::exact_sorted_set(&raw.tool_bindings, &["steel_repl:animus.steel-repl.v1"])?;
    validation::validate_evidence(&raw.evidence)?;
    let rlm_policy = projection::build_rlm_policy(&raw)?;
    let selected_model = projection::build_selected_model(&raw)?;
    let skill_modules = projection::build_skill_modules(&raw)?;
    let config_identity = blake3::hash(json).to_hex().to_string();
    debug_assert_eq!(raw.schema, CONFIG_SCHEMA);
    debug_assert!(!skill_modules.is_empty());
    Ok(ValidatedLiveSteelAgentConfig {
        raw,
        config_identity,
        rlm_policy,
        selected_model,
        skill_modules,
    })
}
