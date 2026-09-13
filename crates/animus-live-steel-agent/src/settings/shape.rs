#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct LiveSteelAgentConfig {
    pub(super) schema: String,
    pub(super) application_id: String,
    pub(super) session_identity: String,
    pub(super) branch_id: String,
    #[serde(skip)]
    pub(super) session_resume_required: bool,
    pub(super) components: ComponentConfig,
    pub(super) provider: ProviderConfig,
    pub(super) store: StoreConfig,
    pub(super) services: ServiceConfig,
    pub(super) bounds: BoundConfig,
    pub(super) authority: AuthorityConfig,
    pub(super) admitted_operations: Vec<String>,
    pub(super) admitted_child_profiles: Vec<String>,
    pub(super) command_profiles: Vec<String>,
    pub(super) tool_bindings: Vec<String>,
    pub(super) selected_skills: Vec<SkillConfig>,
    pub(super) evidence: EvidenceConfig,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ComponentConfig {
    pub(super) provider: String,
    pub(super) durable_store: String,
    pub(super) coding_session: String,
    pub(super) steel_kernel: String,
    pub(super) resumable_bridge: String,
    pub(super) rlm_executor: String,
    pub(super) child_service: String,
    pub(super) skill_loader: String,
    pub(super) control: String,
    pub(super) event_sink: String,
    pub(super) receipt_sink: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ProviderConfig {
    pub(super) provider_id: String,
    pub(super) model_id: String,
    pub(super) endpoint_identity: String,
    pub(super) endpoint_url: String,
    pub(super) credential_reference: String,
    pub(super) reasoning_effort: String,
    pub(super) tool_name: String,
    pub(super) tool_schema: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct StoreConfig {
    pub(super) capability_label: String,
    pub(super) session_journal: String,
    pub(super) session_lock: String,
    pub(super) kernel_journal: String,
    pub(super) kernel_lock: String,
    pub(super) durability: String,
    pub(super) locking: String,
    pub(super) privacy: String,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ServiceConfig {
    pub(super) child: String,
    pub(super) context: String,
    pub(super) control: String,
    pub(super) events: String,
    pub(super) receipts: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct BoundConfig {
    pub(super) provider_timeout_ms: u64,
    pub(super) max_provider_response_bytes: u64,
    pub(super) max_output_tokens: u64,
    pub(super) max_tool_result_chars: u32,
    pub(super) max_queue_items: u32,
    pub(super) max_file_read_bytes: u64,
    pub(super) max_context_bytes: u32,
    pub(super) max_context_lines: u32,
    pub(super) max_child_status_checks: u32,
    pub(super) max_source_chars: u32,
    pub(super) max_environment_chars: u32,
    pub(super) max_bindings: u32,
    pub(super) max_definitions: u32,
    pub(super) max_evaluation_fuel: u64,
    pub(super) max_recursion_depth: u32,
    pub(super) max_allocation_bytes: u64,
    pub(super) max_host_request_chars: u32,
    pub(super) max_child_depth: u32,
    pub(super) max_child_fan_out: u32,
    pub(super) max_child_tokens: u64,
    pub(super) max_child_wall_time_ms: u64,
    pub(super) max_child_result_chars: u32,
    pub(super) max_host_calls_per_cell: u32,
    pub(super) max_host_result_chars: u32,
    pub(super) max_cumulative_host_result_chars: u32,
    pub(super) max_bridge_nesting: u32,
    pub(super) max_bridge_evaluation_fuel: u64,
    pub(super) max_bridge_wall_time_ms: u64,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct AuthorityConfig {
    pub(super) component_identity: String,
    pub(super) authorization_identity: String,
    pub(super) route_binding_identity: String,
    pub(super) route_scope_identity: String,
    pub(super) operation_identity: String,
    pub(super) resource_identity: String,
    pub(super) generation: u64,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct SkillConfig {
    pub(super) name: String,
    pub(super) version: String,
    pub(super) manifest_identity: String,
    pub(super) frozen_source_identity: String,
    pub(super) source_identity: String,
    pub(super) exports: Vec<String>,
    pub(super) capabilities: Vec<String>,
    pub(super) max_source_chars: u32,
    pub(super) max_evaluation_fuel: u64,
}

#[derive(Clone, Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct EvidenceConfig {
    pub(super) policy: String,
    pub(super) redaction_marker: String,
    pub(super) non_claims: Vec<String>,
}
