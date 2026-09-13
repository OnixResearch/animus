pub(super) fn live_steel_tool_catalog(
    request: &animus_adapters::ToolRequest,
    config: &crate::ValidatedLiveSteelAgentConfig,
) -> Result<
    animus_runtime::tool_executor_kit::ToolCatalog,
    animus_runtime::coding_session_host::CodingSessionServiceIssue,
> {
    let metadata =
        animus_adapters::tool::Definition::validate(&animus_adapters::tool::DefinitionDraft {
            name: Some(animus_std_openai::STEEL_REPL_TOOL_NAME),
            label: Some(super::LIVE_STEEL_TOOL_LABEL),
            description: Some(super::LIVE_STEEL_TOOL_DESCRIPTION),
            parameter_schema_identity: Some(request.schema_identity()),
            prompt_snippet: Some(super::LIVE_STEEL_TOOL_PROMPT),
            prompt_guidelines: &[super::LIVE_STEEL_TOOL_GUIDELINE],
            source_identity: Some(config.config_identity()),
            provenance: Some(super::LIVE_STEEL_TOOL_PROVENANCE),
            capability_labels: &[animus_adapter_rlm_kernel::STEEL_REPL_CAPABILITY],
            result_metadata_shape: Some(super::LIVE_STEEL_TOOL_RESULT_SHAPE),
            status: animus_adapters::tool::DefinitionStatus::Active,
        })
        .map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?;
    let max_output_bytes = u64::from(config.max_tool_result_chars())
        .checked_mul(super::MAX_UTF8_BYTES_PER_CHAR)
        .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    debug_assert!(config.max_tool_result_chars() > 0);
    debug_assert!(max_output_bytes >= u64::from(config.max_tool_result_chars()));
    let output_policy = animus_runtime::tool_executor_kit::ToolExecutorOutputPolicy::new(
        config.max_tool_result_chars(),
        max_output_bytes,
        false,
    )
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    let definition = animus_runtime::tool_executor_kit::ToolDefinition::from_draft(
        animus_runtime::tool_executor_kit::ToolDefinitionDraft {
            metadata,
            executor_binding: super::LIVE_STEEL_TOOL_EXECUTOR_BINDING,
            argument_names: &["code"],
            mutation_posture: animus_runtime::tool_executor_kit::ToolMutationPosture::ReadOnly,
            output_policy,
            approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved,
        },
    )
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    animus_runtime::tool_executor_kit::ToolCatalog::new(vec![definition])
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)
}

pub(super) fn live_steel_tool_capabilities() -> Result<
    animus_runtime::RuntimeCapabilityRegistry,
    animus_runtime::coding_session_host::CodingSessionServiceIssue,
> {
    animus_runtime::RuntimeCapabilityRegistry::from_drafts(&[
        animus_runtime::RuntimeCapabilityDescriptorDraft {
            label: animus_adapter_rlm_kernel::STEEL_REPL_CAPABILITY,
            service_kind: "tool",
            scope: None,
        },
    ])
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)
}
