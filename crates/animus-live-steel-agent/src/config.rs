use std::collections::BTreeSet;
use std::time::Duration;

use serde::Deserialize;

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
    raw: LiveSteelAgentConfig,
    config_identity: String,
    rlm_policy: animus_adapter_rlm_kernel::RlmKernelPolicy,
    selected_model: animus_std::SelectedModel,
    skill_modules: Vec<animus_adapter_rlm_kernel::SteelSkillModuleDescriptor>,
}

impl ValidatedLiveSteelAgentConfig {
    /// Returns the application label.
    #[must_use]
    pub fn application_id(&self) -> &str {
        self.raw.application_id.as_str()
    }

    /// Returns the exact session BLAKE3 identity.
    #[must_use]
    pub fn session_identity(&self) -> &str {
        self.raw.session_identity.as_str()
    }

    /// Returns the branch label.
    #[must_use]
    pub fn branch_id(&self) -> &str {
        self.raw.branch_id.as_str()
    }

    /// Returns the deterministic BLAKE3 identity of the validated JSON input.
    #[must_use]
    pub fn config_identity(&self) -> &str {
        self.config_identity.as_str()
    }

    /// Returns a fresh copy with explicit coding-session recovery enabled.
    #[must_use]
    pub fn with_session_resume_required(&self, resume_required: bool) -> Self {
        let mut config = self.clone();
        config.raw.session_resume_required = resume_required;
        config
    }

    /// Returns whether coding-session recovery must run before new work.
    #[must_use]
    pub const fn session_resume_required(&self) -> bool {
        self.raw.session_resume_required
    }

    /// Returns the exact admitted provider identifier.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        self.raw.provider.provider_id.as_str()
    }

    /// Returns the exact credential reference. The value is never in config.
    #[must_use]
    pub fn credential_reference(&self) -> &str {
        self.raw.provider.credential_reference.as_str()
    }

    /// Returns the validated provider reasoning effort.
    #[must_use]
    pub fn reasoning_effort(&self) -> &str {
        self.raw.provider.reasoning_effort.as_str()
    }

    /// Returns the exact selected model.
    #[must_use]
    pub const fn selected_model(&self) -> &animus_std::SelectedModel {
        &self.selected_model
    }

    /// Returns the selected model identifier.
    #[must_use]
    pub fn model_id(&self) -> &str {
        self.raw.provider.model_id.as_str()
    }

    /// Returns the endpoint identity.
    #[must_use]
    pub fn endpoint_identity(&self) -> &str {
        self.raw.provider.endpoint_identity.as_str()
    }

    /// Returns the endpoint URL.
    #[must_use]
    pub fn endpoint_url(&self) -> &str {
        self.raw.provider.endpoint_url.as_str()
    }

    /// Builds the exact admitted OpenAI-compatible host configuration.
    ///
    /// # Errors
    ///
    /// Returns [`LiveSteelAgentConfigError::ProviderMismatch`] if the lower-level
    /// host rejects the endpoint pair or transport bounds.
    pub fn provider_host_config(
        &self,
    ) -> Result<animus_std_openai::OpenAiHostConfig, LiveSteelAgentConfigError> {
        animus_std_openai::OpenAiHostConfig::new(
            self.endpoint_identity(),
            self.endpoint_url(),
            self.provider_timeout(),
            self.max_provider_response_bytes(),
        )
        .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)
    }

    /// Returns the provider deadline.
    #[must_use]
    pub const fn provider_timeout(&self) -> Duration {
        Duration::from_millis(self.raw.bounds.provider_timeout_ms)
    }

    /// Returns the aggregate provider response byte bound.
    #[must_use]
    pub const fn max_provider_response_bytes(&self) -> u64 {
        self.raw.bounds.max_provider_response_bytes
    }

    /// Returns the canonical output token bound.
    #[must_use]
    pub const fn max_output_tokens(&self) -> u64 {
        self.raw.bounds.max_output_tokens
    }

    /// Returns the model-visible tool result bound.
    #[must_use]
    pub const fn max_tool_result_chars(&self) -> u32 {
        self.raw.bounds.max_tool_result_chars
    }

    /// Returns the queue item bound.
    #[must_use]
    pub const fn max_queue_items(&self) -> u32 {
        self.raw.bounds.max_queue_items
    }

    /// Returns the validated RLM kernel policy.
    #[must_use]
    pub const fn rlm_policy(&self) -> &animus_adapter_rlm_kernel::RlmKernelPolicy {
        &self.rlm_policy
    }

    /// Returns the explicitly selected skill descriptors in deterministic order.
    #[must_use]
    pub fn skill_modules(&self) -> &[animus_adapter_rlm_kernel::SteelSkillModuleDescriptor] {
        self.skill_modules.as_slice()
    }

    /// Returns exact operation labels.
    #[must_use]
    pub fn admitted_operations(&self) -> &[String] {
        self.raw.admitted_operations.as_slice()
    }

    /// Returns exact child profile labels.
    #[must_use]
    pub fn admitted_child_profiles(&self) -> &[String] {
        self.raw.admitted_child_profiles.as_slice()
    }

    /// Returns the durable store capability label.
    #[must_use]
    pub fn store_capability_label(&self) -> &str {
        self.raw.store.capability_label.as_str()
    }

    /// Returns the session journal leaf name.
    #[must_use]
    pub fn session_journal(&self) -> &str {
        self.raw.store.session_journal.as_str()
    }

    /// Returns the session lock leaf name.
    #[must_use]
    pub fn session_lock(&self) -> &str {
        self.raw.store.session_lock.as_str()
    }

    /// Returns the kernel journal leaf name.
    #[must_use]
    pub fn kernel_journal(&self) -> &str {
        self.raw.store.kernel_journal.as_str()
    }

    /// Returns the kernel lock leaf name.
    #[must_use]
    pub fn kernel_lock(&self) -> &str {
        self.raw.store.kernel_lock.as_str()
    }

    /// Returns the configured authority generation.
    #[must_use]
    pub const fn authority_generation(&self) -> u64 {
        self.raw.authority.generation
    }

    /// Returns the finite host-executor bounds.
    #[must_use]
    pub fn executor_limits(&self) -> animus_std_rlm_host::ExecutorLimits {
        animus_std_rlm_host::ExecutorLimits {
            max_file_read_bytes: self.raw.bounds.max_file_read_bytes,
            max_context_bytes: self.raw.bounds.max_context_bytes,
            max_context_lines: self.raw.bounds.max_context_lines,
            max_result_chars: self.raw.bounds.max_tool_result_chars,
            max_child_status_checks: self.raw.bounds.max_child_status_checks,
        }
    }

    /// Returns the finite child-session bounds.
    #[must_use]
    pub const fn child_session_limits(
        &self,
    ) -> animus_runtime::child_session_service::ChildSessionLimits {
        animus_runtime::child_session_service::ChildSessionLimits {
            depth: self.raw.bounds.max_child_depth,
            fan_out: self.raw.bounds.max_child_fan_out,
            wall_time_ms: self.raw.bounds.max_child_wall_time_ms,
            tokens: self.raw.bounds.max_child_tokens,
            turns: self.raw.bounds.max_bridge_nesting,
            tool_calls: self.raw.bounds.max_host_calls_per_cell,
            result_chars: self.raw.bounds.max_child_result_chars,
        }
    }

    /// Returns the exact child service label.
    #[must_use]
    pub fn child_service_label(&self) -> &str {
        self.raw.services.child.as_str()
    }

    /// Returns the exact context service label.
    #[must_use]
    pub fn context_service_label(&self) -> &str {
        self.raw.services.context.as_str()
    }

    /// Returns one exact authority identity.
    #[must_use]
    pub fn authority_component_identity(&self) -> &str {
        self.raw.authority.component_identity.as_str()
    }

    /// Returns one exact authorization identity.
    #[must_use]
    pub fn authority_authorization_identity(&self) -> &str {
        self.raw.authority.authorization_identity.as_str()
    }

    /// Returns one exact route binding identity.
    #[must_use]
    pub fn route_binding_identity(&self) -> &str {
        self.raw.authority.route_binding_identity.as_str()
    }

    /// Returns one exact route scope identity.
    #[must_use]
    pub fn route_scope_identity(&self) -> &str {
        self.raw.authority.route_scope_identity.as_str()
    }

    /// Returns one exact operation identity.
    #[must_use]
    pub fn operation_identity(&self) -> &str {
        self.raw.authority.operation_identity.as_str()
    }

    /// Returns one exact resource identity.
    #[must_use]
    pub fn resource_identity(&self) -> &str {
        self.raw.authority.resource_identity.as_str()
    }

    /// Returns required evidence non-claims.
    #[must_use]
    pub fn non_claims(&self) -> &[String] {
        self.raw.evidence.non_claims.as_slice()
    }
}

/// Parses and revalidates the deterministic Nickel export.
///
/// # Errors
///
/// Returns [`LiveSteelAgentConfigError`] before effects for malformed, missing,
/// widened, duplicated, unsupported, or mismatched values.
pub fn parse_live_steel_agent_config(
    json: &[u8],
) -> Result<ValidatedLiveSteelAgentConfig, LiveSteelAgentConfigError> {
    // r[impl agent_harness_runtime.live_steel_agent_shell.configuration]
    let raw: LiveSteelAgentConfig =
        serde_json::from_slice(json).map_err(|_error| LiveSteelAgentConfigError::Malformed)?;
    validate_fixed_components(&raw)?;
    validate_provider(&raw.provider)?;
    validate_store(&raw.store)?;
    validate_services(&raw.services)?;
    validate_bounds(raw.bounds)?;
    validate_authority(&raw.authority)?;
    validate_operations(&raw.admitted_operations)?;
    validate_labels(&raw.admitted_child_profiles, MAX_ITEMS, false)?;
    validate_labels(&raw.command_profiles, MAX_ITEMS, true)?;
    exact_sorted_set(&raw.tool_bindings, &["steel_repl:animus.steel-repl.v1"])?;
    validate_evidence(&raw.evidence)?;
    let rlm_policy = build_rlm_policy(&raw)?;
    let selected_model = build_selected_model(&raw)?;
    let skill_modules = build_skill_modules(&raw)?;
    let config_identity = blake3::hash(json).to_hex().to_string();
    Ok(ValidatedLiveSteelAgentConfig {
        raw,
        config_identity,
        rlm_policy,
        selected_model,
        skill_modules,
    })
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct LiveSteelAgentConfig {
    schema: String,
    application_id: String,
    session_identity: String,
    branch_id: String,
    #[serde(skip)]
    session_resume_required: bool,
    components: ComponentConfig,
    provider: ProviderConfig,
    store: StoreConfig,
    services: ServiceConfig,
    bounds: BoundConfig,
    authority: AuthorityConfig,
    admitted_operations: Vec<String>,
    admitted_child_profiles: Vec<String>,
    command_profiles: Vec<String>,
    tool_bindings: Vec<String>,
    selected_skills: Vec<SkillConfig>,
    evidence: EvidenceConfig,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ComponentConfig {
    provider: String,
    durable_store: String,
    coding_session: String,
    steel_kernel: String,
    resumable_bridge: String,
    rlm_executor: String,
    child_service: String,
    skill_loader: String,
    control: String,
    event_sink: String,
    receipt_sink: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ProviderConfig {
    provider_id: String,
    model_id: String,
    endpoint_identity: String,
    endpoint_url: String,
    credential_reference: String,
    reasoning_effort: String,
    tool_name: String,
    tool_schema: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct StoreConfig {
    capability_label: String,
    session_journal: String,
    session_lock: String,
    kernel_journal: String,
    kernel_lock: String,
    durability: String,
    locking: String,
    privacy: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ServiceConfig {
    child: String,
    context: String,
    control: String,
    events: String,
    receipts: String,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct BoundConfig {
    provider_timeout_ms: u64,
    max_provider_response_bytes: u64,
    max_output_tokens: u64,
    max_tool_result_chars: u32,
    max_queue_items: u32,
    max_file_read_bytes: u64,
    max_context_bytes: u32,
    max_context_lines: u32,
    max_child_status_checks: u32,
    max_source_chars: u32,
    max_environment_chars: u32,
    max_bindings: u32,
    max_definitions: u32,
    max_evaluation_fuel: u64,
    max_recursion_depth: u32,
    max_allocation_bytes: u64,
    max_host_request_chars: u32,
    max_child_depth: u32,
    max_child_fan_out: u32,
    max_child_tokens: u64,
    max_child_wall_time_ms: u64,
    max_child_result_chars: u32,
    max_host_calls_per_cell: u32,
    max_host_result_chars: u32,
    max_cumulative_host_result_chars: u32,
    max_bridge_nesting: u32,
    max_bridge_evaluation_fuel: u64,
    max_bridge_wall_time_ms: u64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuthorityConfig {
    component_identity: String,
    authorization_identity: String,
    route_binding_identity: String,
    route_scope_identity: String,
    operation_identity: String,
    resource_identity: String,
    generation: u64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SkillConfig {
    name: String,
    version: String,
    manifest_identity: String,
    frozen_source_identity: String,
    source_identity: String,
    exports: Vec<String>,
    capabilities: Vec<String>,
    max_source_chars: u32,
    max_evaluation_fuel: u64,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct EvidenceConfig {
    policy: String,
    redaction_marker: String,
    non_claims: Vec<String>,
}

fn validate_fixed_components(raw: &LiveSteelAgentConfig) -> Result<(), LiveSteelAgentConfigError> {
    validate_label(raw.application_id.as_str())?;
    validate_label(raw.branch_id.as_str())?;
    if raw.schema != CONFIG_SCHEMA {
        return Err(LiveSteelAgentConfigError::ComponentMismatch);
    }
    validate_blake3(raw.session_identity.as_str())?;
    let actual = [
        raw.components.provider.as_str(),
        raw.components.durable_store.as_str(),
        raw.components.coding_session.as_str(),
        raw.components.steel_kernel.as_str(),
        raw.components.resumable_bridge.as_str(),
        raw.components.rlm_executor.as_str(),
        raw.components.child_service.as_str(),
        raw.components.skill_loader.as_str(),
        raw.components.control.as_str(),
        raw.components.event_sink.as_str(),
        raw.components.receipt_sink.as_str(),
    ];
    let expected = [
        COMPONENT_PROVIDER,
        COMPONENT_STORE,
        COMPONENT_CODING_SESSION,
        COMPONENT_KERNEL,
        COMPONENT_BRIDGE,
        COMPONENT_EXECUTOR,
        COMPONENT_CHILD,
        COMPONENT_SKILLS,
        COMPONENT_CONTROL,
        COMPONENT_EVENTS,
        COMPONENT_RECEIPTS,
    ];
    if actual != expected {
        return Err(LiveSteelAgentConfigError::ComponentMismatch);
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct AdmittedProviderCohort {
    provider_id: &'static str,
    endpoint_identity: &'static str,
    endpoint_url: &'static str,
    credential_reference: &'static str,
    credential_source: &'static str,
    reasoning_effort: &'static str,
}

const LOCAL_DSPARK_PROVIDER_COHORT: AdmittedProviderCohort = AdmittedProviderCohort {
    provider_id: LOCAL_DSPARK_PROVIDER_ID,
    endpoint_identity: animus_std_openai::LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY,
    endpoint_url: animus_std_openai::LOCAL_DSPARK_RESPONSES_URL,
    credential_reference: LOCAL_DSPARK_CREDENTIAL_REFERENCE,
    credential_source: LOCAL_DSPARK_CREDENTIAL_SOURCE,
    reasoning_effort: DISABLED_REASONING_EFFORT,
};
const OPENAI_PROVIDER_COHORT: AdmittedProviderCohort = AdmittedProviderCohort {
    provider_id: OPENAI_PROVIDER_ID,
    endpoint_identity: animus_std_openai::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
    endpoint_url: animus_std_openai::OFFICIAL_OPENAI_RESPONSES_URL,
    credential_reference: OPENAI_CREDENTIAL_REFERENCE,
    credential_source: OPENAI_CREDENTIAL_SOURCE,
    reasoning_effort: LOW_REASONING_EFFORT,
};
const OPENROUTER_PROVIDER_COHORT: AdmittedProviderCohort = AdmittedProviderCohort {
    provider_id: OPENROUTER_PROVIDER_ID,
    endpoint_identity: animus_std_openai::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
    endpoint_url: animus_std_openai::OPENROUTER_RESPONSES_URL,
    credential_reference: OPENROUTER_CREDENTIAL_REFERENCE,
    credential_source: OPENROUTER_CREDENTIAL_SOURCE,
    reasoning_effort: LOW_REASONING_EFFORT,
};

fn validate_provider(provider: &ProviderConfig) -> Result<(), LiveSteelAgentConfigError> {
    for label in [
        provider.provider_id.as_str(),
        provider.model_id.as_str(),
        provider.endpoint_identity.as_str(),
        provider.credential_reference.as_str(),
        provider.reasoning_effort.as_str(),
        provider.tool_name.as_str(),
        provider.tool_schema.as_str(),
    ] {
        validate_label(label)?;
    }
    let cohort = admit_provider_cohort(provider)?;
    let is_provider_admitted = provider.reasoning_effort == cohort.reasoning_effort
        && provider.tool_name == animus_std_openai::STEEL_REPL_TOOL_NAME
        && provider.tool_schema == animus_std_openai::STEEL_REPL_TOOL_SCHEMA;
    if !is_provider_admitted {
        return Err(LiveSteelAgentConfigError::ProviderMismatch);
    }
    Ok(())
}

fn admit_provider_cohort(
    provider: &ProviderConfig,
) -> Result<&'static AdmittedProviderCohort, LiveSteelAgentConfigError> {
    // r[impl agent_harness_runtime.live_steel_agent_provider_cohort]
    let cohort = match provider.provider_id.as_str() {
        LOCAL_DSPARK_PROVIDER_ID => &LOCAL_DSPARK_PROVIDER_COHORT,
        OPENAI_PROVIDER_ID => &OPENAI_PROVIDER_COHORT,
        OPENROUTER_PROVIDER_ID => &OPENROUTER_PROVIDER_COHORT,
        _unknown => return Err(LiveSteelAgentConfigError::ProviderMismatch),
    };
    let is_tuple_exact = provider.provider_id == cohort.provider_id
        && provider.endpoint_identity == cohort.endpoint_identity
        && provider.endpoint_url == cohort.endpoint_url
        && provider.credential_reference == cohort.credential_reference;
    if !is_tuple_exact {
        return Err(LiveSteelAgentConfigError::ProviderMismatch);
    }
    Ok(cohort)
}

fn validate_store(store: &StoreConfig) -> Result<(), LiveSteelAgentConfigError> {
    for label in [
        store.capability_label.as_str(),
        store.session_journal.as_str(),
        store.session_lock.as_str(),
        store.kernel_journal.as_str(),
        store.kernel_lock.as_str(),
    ] {
        validate_label(label)?;
    }
    let distinct_leaf_names = store.session_journal != store.session_lock
        && store.session_journal != store.kernel_journal
        && store.session_journal != store.kernel_lock
        && store.session_lock != store.kernel_journal
        && store.session_lock != store.kernel_lock
        && store.kernel_journal != store.kernel_lock;
    if store.durability != DURABILITY
        || store.locking != LOCKING
        || store.privacy != PRIVACY
        || !distinct_leaf_names
    {
        return Err(LiveSteelAgentConfigError::StoreMismatch);
    }
    Ok(())
}

fn validate_services(services: &ServiceConfig) -> Result<(), LiveSteelAgentConfigError> {
    let labels = [
        services.child.as_str(),
        services.context.as_str(),
        services.control.as_str(),
        services.events.as_str(),
        services.receipts.as_str(),
    ];
    for label in labels {
        validate_label(label)?;
    }
    Ok(())
}

fn validate_bounds(bounds: BoundConfig) -> Result<(), LiveSteelAgentConfigError> {
    let values = [
        bounds.provider_timeout_ms,
        bounds.max_provider_response_bytes,
        bounds.max_output_tokens,
        u64::from(bounds.max_tool_result_chars),
        u64::from(bounds.max_queue_items),
        bounds.max_file_read_bytes,
        u64::from(bounds.max_context_bytes),
        u64::from(bounds.max_context_lines),
        u64::from(bounds.max_child_status_checks),
        u64::from(bounds.max_source_chars),
        u64::from(bounds.max_environment_chars),
        u64::from(bounds.max_bindings),
        u64::from(bounds.max_definitions),
        bounds.max_evaluation_fuel,
        u64::from(bounds.max_recursion_depth),
        bounds.max_allocation_bytes,
        u64::from(bounds.max_host_request_chars),
        u64::from(bounds.max_child_depth),
        u64::from(bounds.max_child_fan_out),
        bounds.max_child_tokens,
        bounds.max_child_wall_time_ms,
        u64::from(bounds.max_child_result_chars),
        u64::from(bounds.max_host_calls_per_cell),
        u64::from(bounds.max_host_result_chars),
        u64::from(bounds.max_cumulative_host_result_chars),
        u64::from(bounds.max_bridge_nesting),
        bounds.max_bridge_evaluation_fuel,
        bounds.max_bridge_wall_time_ms,
    ];
    if values.contains(&0) {
        return Err(LiveSteelAgentConfigError::InvalidBound);
    }
    Ok(())
}

fn validate_authority(authority: &AuthorityConfig) -> Result<(), LiveSteelAgentConfigError> {
    for identity in [
        authority.component_identity.as_str(),
        authority.authorization_identity.as_str(),
        authority.route_binding_identity.as_str(),
        authority.route_scope_identity.as_str(),
        authority.operation_identity.as_str(),
        authority.resource_identity.as_str(),
    ] {
        validate_blake3(identity)?;
    }
    if authority.generation == 0 {
        return Err(LiveSteelAgentConfigError::InvalidBound);
    }
    Ok(())
}

fn validate_evidence(evidence: &EvidenceConfig) -> Result<(), LiveSteelAgentConfigError> {
    if evidence.policy != EVIDENCE_POLICY || evidence.redaction_marker != EVIDENCE_REDACTION {
        return Err(LiveSteelAgentConfigError::EvidenceMismatch);
    }
    exact_sorted_set(&evidence.non_claims, REQUIRED_NON_CLAIMS)
        .map_err(|_error| LiveSteelAgentConfigError::EvidenceMismatch)
}

fn build_rlm_policy(
    raw: &LiveSteelAgentConfig,
) -> Result<animus_adapter_rlm_kernel::RlmKernelPolicy, LiveSteelAgentConfigError> {
    let operations = raw
        .admitted_operations
        .iter()
        .map(|operation| match operation.as_str() {
            "context_read" => Ok(animus_adapter_rlm_kernel::HostOperationClass::ContextRead),
            "subagent" => Ok(animus_adapter_rlm_kernel::HostOperationClass::Subagent),
            "tool" => Ok(animus_adapter_rlm_kernel::HostOperationClass::Tool),
            _other => Err(LiveSteelAgentConfigError::InvalidCollection),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let child_profiles = raw
        .admitted_child_profiles
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let non_claims = animus_adapter_rlm_kernel::REQUIRED_NON_CLAIMS;
    animus_adapter_rlm_kernel::RlmKernelPolicy::from_draft(
        &animus_adapter_rlm_kernel::RlmKernelPolicyDraft {
            schema: animus_adapter_rlm_kernel::RLM_KERNEL_POLICY_SCHEMA,
            cohort: "prime-agent-steel-repl-reviewed",
            cohort_revision: "c5991bc853d27754aed345c13ff4d2e05c40f1f4",
            profile: "live-steel-agent",
            bounds: animus_adapter_rlm_kernel::ResourceBounds {
                max_source_chars: raw.bounds.max_source_chars,
                max_environment_chars: raw.bounds.max_environment_chars,
                max_bindings: raw.bounds.max_bindings,
                max_definitions: raw.bounds.max_definitions,
                max_evaluation_fuel: raw.bounds.max_evaluation_fuel,
                max_recursion_depth: raw.bounds.max_recursion_depth,
                max_allocation_bytes: raw.bounds.max_allocation_bytes,
                max_host_request_chars: raw.bounds.max_host_request_chars,
                max_child_depth: raw.bounds.max_child_depth,
                max_child_fan_out: raw.bounds.max_child_fan_out,
                max_child_tokens: raw.bounds.max_child_tokens,
                max_child_wall_time_ms: raw.bounds.max_child_wall_time_ms,
                max_child_result_chars: raw.bounds.max_child_result_chars,
                max_host_calls_per_cell: raw.bounds.max_host_calls_per_cell,
                max_host_result_chars: raw.bounds.max_host_result_chars,
                max_cumulative_host_result_chars: raw.bounds.max_cumulative_host_result_chars,
                max_bridge_nesting: raw.bounds.max_bridge_nesting,
                max_bridge_evaluation_fuel: raw.bounds.max_bridge_evaluation_fuel,
                max_bridge_wall_time_ms: raw.bounds.max_bridge_wall_time_ms,
            },
            admitted_operations: operations.as_slice(),
            admitted_child_profiles: child_profiles.as_slice(),
            non_claims,
        },
    )
    .map_err(|_error| LiveSteelAgentConfigError::InvalidBound)
}

fn build_selected_model(
    raw: &LiveSteelAgentConfig,
) -> Result<animus_std::SelectedModel, LiveSteelAgentConfigError> {
    const CONTEXT_BUDGET: u32 = 128_000;
    let cohort = admit_provider_cohort(&raw.provider)?;
    let provider_id = animus_std::ProviderId::new(raw.provider.provider_id.as_str())
        .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)?;
    let model_id = animus_std::ModelId::new(raw.provider.model_id.as_str())
        .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)?;
    let endpoint = animus_std::EndpointIdentity::new(raw.provider.endpoint_identity.as_str())
        .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)?;
    let capabilities =
        animus_std::ModelCapabilities::from_draft(animus_std::ModelCapabilitiesDraft {
            input_modalities: vec![animus_std::InputModality::Text],
            context_budget: CONTEXT_BUDGET,
            output_budget: u32::try_from(raw.bounds.max_output_tokens)
                .map_err(|_error| LiveSteelAgentConfigError::InvalidBound)?,
            reasoning: animus_std::ReasoningSupport::Optional,
            thinking_levels: vec![
                animus_std::ThinkingLevel::None,
                animus_std::ThinkingLevel::Low,
            ],
            streaming: animus_std::ModelStreamingCapabilities::new(
                animus_std::StreamSupport::Supported,
                animus_std::StreamSupport::Supported,
                animus_std::StreamSupport::Supported,
                animus_std::StreamSupport::Supported,
            ),
            cost: None,
        })
        .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)?;
    let model = animus_std::ModelEntry::new(
        model_id,
        animus_std::ApiKind::OpenAiResponses,
        endpoint,
        capabilities,
    );
    let source = animus_std::CredentialSource::brokered(cohort.credential_source)
        .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)?;
    let provider = animus_std::ProviderEntry::new(
        provider_id,
        animus_std::CredentialRequirement::new(source, true),
        vec![model],
    )
    .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)?;
    animus_std::ProviderRegistry::new(vec![provider])
        .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)?
        .select_model(&animus_std::ModelSelection::new(
            animus_std::ProviderId::new(raw.provider.provider_id.as_str())
                .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)?,
            animus_std::ModelId::new(raw.provider.model_id.as_str())
                .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)?,
            animus_std::ThinkingLevel::Low,
            animus_std::ThinkingPolicy::Reject,
        ))
        .map_err(|_error| LiveSteelAgentConfigError::ProviderMismatch)
}

fn build_skill_modules(
    raw: &LiveSteelAgentConfig,
) -> Result<Vec<animus_adapter_rlm_kernel::SteelSkillModuleDescriptor>, LiveSteelAgentConfigError> {
    if raw.selected_skills.is_empty() || raw.selected_skills.len() > MAX_SKILLS {
        return Err(LiveSteelAgentConfigError::SkillMismatch);
    }
    let mut modules = Vec::with_capacity(raw.selected_skills.len());
    for skill in &raw.selected_skills {
        validate_labels(&skill.exports, MAX_ITEMS, false)?;
        validate_labels(&skill.capabilities, MAX_ITEMS, true)?;
        let exports = skill.exports.iter().map(String::as_str).collect::<Vec<_>>();
        let capabilities = skill
            .capabilities
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let descriptor = animus_adapter_rlm_kernel::SteelSkillModuleDescriptor::validate(
            &animus_adapter_rlm_kernel::SteelSkillModuleDescriptorDraft {
                schema: animus_adapter_rlm_kernel::STEEL_SKILL_MODULE_SCHEMA,
                name: skill.name.as_str(),
                version: skill.version.as_str(),
                manifest_identity: skill.manifest_identity.as_str(),
                frozen_source_identity: skill.frozen_source_identity.as_str(),
                source_identity: skill.source_identity.as_str(),
                source: SKILL_SOURCE,
                exports: exports.as_slice(),
                dependencies: &[],
                capability_requests: capabilities.as_slice(),
                trust: animus_adapter_rlm_kernel::SteelSkillModuleTrust::Trusted,
                bounds: animus_adapter_rlm_kernel::SteelSkillModuleBounds {
                    source_chars: skill.max_source_chars,
                    evaluation_fuel: skill.max_evaluation_fuel,
                },
            },
        )
        .map_err(|_error| LiveSteelAgentConfigError::SkillMismatch)?;
        modules.push(descriptor);
    }
    Ok(modules)
}

fn validate_operations(actual: &[String]) -> Result<(), LiveSteelAgentConfigError> {
    // r[impl agent_harness_runtime.pueue_host.live_shell_tool_route]
    validate_labels(actual, MAX_ITEMS, false)?;
    let actual = actual.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let required = REQUIRED_OPERATIONS.iter().copied().collect::<BTreeSet<_>>();
    let allowed = ALLOWED_OPERATIONS.iter().copied().collect::<BTreeSet<_>>();
    if !required.is_subset(&actual) || !actual.is_subset(&allowed) {
        return Err(LiveSteelAgentConfigError::InvalidCollection);
    }
    Ok(())
}

fn exact_sorted_set(actual: &[String], expected: &[&str]) -> Result<(), LiveSteelAgentConfigError> {
    validate_labels(actual, MAX_NON_CLAIMS, false)?;
    let actual = actual.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let expected = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual != expected {
        return Err(LiveSteelAgentConfigError::InvalidCollection);
    }
    Ok(())
}

fn validate_labels(
    labels: &[String],
    max_items: usize,
    allow_empty: bool,
) -> Result<(), LiveSteelAgentConfigError> {
    if labels.len() > max_items || (!allow_empty && labels.is_empty()) {
        return Err(LiveSteelAgentConfigError::InvalidCollection);
    }
    let mut unique = BTreeSet::new();
    for label in labels {
        validate_label(label.as_str())?;
        if !unique.insert(label.as_str()) {
            return Err(LiveSteelAgentConfigError::InvalidCollection);
        }
    }
    Ok(())
}

fn validate_label(label: &str) -> Result<(), LiveSteelAgentConfigError> {
    let valid = !label.is_empty()
        && label.chars().count() <= MAX_SAFE_LABEL_CHARS
        && !label.chars().any(char::is_control);
    if !valid {
        return Err(LiveSteelAgentConfigError::ServiceMismatch);
    }
    Ok(())
}

fn validate_blake3(identity: &str) -> Result<(), LiveSteelAgentConfigError> {
    if identity.len() != BLAKE3_HEX_CHARS
        || !identity
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(LiveSteelAgentConfigError::InvalidIdentity);
    }
    Ok(())
}
