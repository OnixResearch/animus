#[test]
pub(super) fn marker_only_host_request_is_executed_before_tool_completion() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.tool_loop.marker_only]
    let config = animus_live_steel_agent::parse_live_steel_agent_config(super::CONFIG)
        .expect("fixture configuration");
    let mut kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("kernel");
    let request = tool_request("(animus-context-read \"fixture-key\")");
    let mut executor = super::ContextExecutor::default();
    let mut control = FixedControl;
    let cell_identity = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    let completed = animus_runtime::rlm_kernel::execute_bridged_cell(
        &mut kernel,
        animus_runtime::rlm_kernel::RlmBridgeExecutionRequest {
            source: request.arguments()[0].value(),
            session_identity: config.session_identity(),
            cell_identity,
            nesting_depth: 1,
        },
        &mut executor,
        &mut control,
    )
    .expect("host request completes");
    assert_eq!(executor.calls, 1);
    assert_eq!(completed.observations().len(), 1);
    assert!(matches!(
        completed.evaluation(),
        animus_steel::SteelRlmEvaluation::Value { value, .. } if value == "fixture-value"
    ));
}

#[test]
pub(super) fn failed_kernel_safe_point_stays_pending_and_blocks_close() {
    let config = animus_live_steel_agent::parse_live_steel_agent_config(super::CONFIG)
        .expect("fixture configuration");
    let kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("fixture kernel");
    let mut service = animus_live_steel_agent::SteelToolService::new(
        kernel,
        super::ContextExecutor::default(),
        FailingKernelPersistence,
        animus_live_steel_agent::SteelToolBinding {
            session_identity: String::from(config.session_identity()),
            generation: config.authority_generation(),
        },
    );
    let request = tool_request("(+ 1 2)");
    assert!(matches!(
        service.execute(&request),
        Err(animus_live_steel_agent::SteelToolError::Persistence)
    ));
    assert!(matches!(
        service.execute(&request),
        Err(animus_live_steel_agent::SteelToolError::Persistence)
    ));
    assert!(matches!(
        service.close(),
        Err(animus_live_steel_agent::SteelToolError::Persistence)
    ));
}

pub(super) struct FailingKernelPersistence;

impl animus_live_steel_agent::KernelSafePointPersistence for FailingKernelPersistence {
    fn persist(
        &mut self,
        _journal: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<(), animus_live_steel_agent::SteelToolError> {
        Err(animus_live_steel_agent::SteelToolError::Persistence)
    }

    fn close(self) -> Result<(), animus_live_steel_agent::SteelToolError> {
        Ok(())
    }
}

pub(super) struct FixedControl;

impl animus_runtime::rlm_kernel::RlmBridgeControl for FixedControl {
    fn observe(
        &mut self,
        _checkpoint: animus_runtime::rlm_kernel::RlmBridgeCheckpoint,
    ) -> animus_runtime::rlm_kernel::RlmBridgeControlObservation {
        animus_runtime::rlm_kernel::RlmBridgeControlObservation {
            cancelled: false,
            elapsed_ms: 0,
        }
    }
}

const MAX_VISIBLE_CHARS: u32 = 4_096;

pub(super) fn tool_request(code: &str) -> animus_adapters::ToolRequest {
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
    assert!(!code.is_empty());
    assert_eq!(
        arguments.schema_identity(),
        animus_std_openai::STEEL_REPL_TOOL_SCHEMA
    );
    animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(animus_std_openai::STEEL_REPL_TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some("call.host-fixture"),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: MAX_VISIBLE_CHARS,
        },
        mutation_targets: &[],
        required_capability_labels: &[animus_adapter_rlm_kernel::STEEL_REPL_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .expect("tool request")
}
