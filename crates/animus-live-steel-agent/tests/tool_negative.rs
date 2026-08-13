//! Steel tool cancellation, host failure, and request-denial checks.

use animus_live_steel_agent::{
    SteelToolBinding, SteelToolError, SteelToolService, parse_live_steel_agent_config,
};

const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
const HOST_SOURCE: &str = "(animus-context-read \"fixture-key\")";
const VALUE_SOURCE: &str = "(+ 1 2)";

#[derive(Default)]
struct DenyHost;

impl animus_runtime::rlm_kernel::RlmKernelHostExecutor for DenyHost {
    type Error = &'static str;

    fn execute(
        &mut self,
        _request: &animus_adapter_rlm_kernel::HostRequest,
    ) -> Result<String, Self::Error> {
        Err("host denied")
    }
}

#[test]
fn explicit_cancellation_stops_before_kernel_safe_point() {
    let config = parse_live_steel_agent_config(CONFIG).expect("config");
    let kernel = animus_steel::SteelRlmKernel::new(config.rlm_policy().clone())
        .expect("kernel");
    let mut tool = SteelToolService::new(kernel, DenyHost, (), binding(&config));
    tool.set_cancelled(true);
    let prior_head = kernel_head(&tool, &config);

    assert_eq!(tool.execute(&tool_request(HOST_SOURCE)), Err(SteelToolError::Cancelled));
    assert_eq!(kernel_head(&tool, &config), prior_head);
}

#[test]
fn host_failure_does_not_commit_the_active_cell() {
    let config = parse_live_steel_agent_config(CONFIG).expect("config");
    let kernel = animus_steel::SteelRlmKernel::new(config.rlm_policy().clone())
        .expect("kernel");
    let mut tool = SteelToolService::new(kernel, DenyHost, (), binding(&config));
    let prior_head = kernel_head(&tool, &config);

    assert_eq!(tool.execute(&tool_request(HOST_SOURCE)), Err(SteelToolError::Host));
    assert_eq!(kernel_head(&tool, &config), prior_head);
}

#[test]
fn denied_tool_request_fails_before_steel_execution() {
    let config = parse_live_steel_agent_config(CONFIG).expect("config");
    let kernel = animus_steel::SteelRlmKernel::new(config.rlm_policy().clone())
        .expect("kernel");
    let mut tool = SteelToolService::new(kernel, DenyHost, (), binding(&config));
    let prior_head = kernel_head(&tool, &config);

    assert_eq!(tool.execute(&denied_tool_request()), Err(SteelToolError::Request));
    assert_eq!(kernel_head(&tool, &config), prior_head);
}

fn binding(config: &animus_live_steel_agent::ValidatedLiveSteelAgentConfig) -> SteelToolBinding {
    SteelToolBinding {
        session_identity: String::from(config.session_identity()),
        generation: config.authority_generation(),
    }
}

fn kernel_head(
    tool: &SteelToolService<DenyHost, ()>,
    config: &animus_live_steel_agent::ValidatedLiveSteelAgentConfig,
) -> String {
    tool.kernel()
        .private_journal_snapshot(config.session_identity(), config.authority_generation())
        .expect("kernel snapshot")
        .committed_head_identity()
        .to_owned()
}

fn tool_request(code: &str) -> animus_adapters::ToolRequest {
    request(code, animus_adapters::tool::ApprovalPosture::PreApproved)
}

fn denied_tool_request() -> animus_adapters::ToolRequest {
    request(VALUE_SOURCE, animus_adapters::tool::ApprovalPosture::Denied)
}

fn request(
    code: &str,
    approval: animus_adapters::tool::ApprovalPosture,
) -> animus_adapters::ToolRequest {
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: animus_std_openai::STEEL_REPL_TOOL_SCHEMA,
            allowed_field_names: &["code"],
            compatibility_rules: &[],
            fields: &[animus_adapters::tool::ArgumentFieldDraft {
                name: Some("code"),
                value: Some(code),
            }],
        },
    )
    .expect("arguments");
    animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(animus_std_openai::STEEL_REPL_TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some("call.negative"),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: 4_096,
        },
        mutation_targets: &[],
        required_capability_labels: &[animus_adapter_rlm_kernel::STEEL_REPL_CAPABILITY],
        approval_posture: approval.as_str(),
    })
    .expect("request")
}
