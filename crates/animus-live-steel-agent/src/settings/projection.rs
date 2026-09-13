pub(super) fn build_rlm_policy(
    raw: &super::shape::LiveSteelAgentConfig,
) -> Result<animus_adapter_rlm_kernel::RlmKernelPolicy, super::LiveSteelAgentConfigError> {
    let operations = raw
        .admitted_operations
        .iter()
        .map(|operation| match operation.as_str() {
            "context_read" => Ok(animus_adapter_rlm_kernel::HostOperationClass::ContextRead),
            "subagent" => Ok(animus_adapter_rlm_kernel::HostOperationClass::Subagent),
            "tool" => Ok(animus_adapter_rlm_kernel::HostOperationClass::Tool),
            _other => Err(super::LiveSteelAgentConfigError::InvalidCollection),
        })
        .collect::<Result<Vec<_>, _>>()?;
    let child_profiles = raw
        .admitted_child_profiles
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    debug_assert_eq!(operations.len(), raw.admitted_operations.len());
    debug_assert_eq!(child_profiles.len(), raw.admitted_child_profiles.len());
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
    .map_err(|_error| super::LiveSteelAgentConfigError::InvalidBound)
}

pub(super) fn build_selected_model(
    raw: &super::shape::LiveSteelAgentConfig,
) -> Result<animus_std::SelectedModel, super::LiveSteelAgentConfigError> {
    const CONTEXT_BUDGET: u32 = 128_000;
    let cohort = super::cohort::admit(&raw.provider)?;
    let provider_id = animus_std::ProviderId::new(raw.provider.provider_id.as_str())
        .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)?;
    let model_id = animus_std::ModelId::new(raw.provider.model_id.as_str())
        .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)?;
    let endpoint = animus_std::EndpointIdentity::new(raw.provider.endpoint_identity.as_str())
        .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)?;
    let capabilities =
        animus_std::ModelCapabilities::from_draft(animus_std::ModelCapabilitiesDraft {
            input_modalities: vec![animus_std::InputModality::Text],
            context_budget: CONTEXT_BUDGET,
            output_budget: u32::try_from(raw.bounds.max_output_tokens)
                .map_err(|_error| super::LiveSteelAgentConfigError::InvalidBound)?,
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
        .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)?;
    debug_assert_eq!(provider_id.as_str(), raw.provider.provider_id);
    debug_assert_eq!(model_id.as_str(), raw.provider.model_id);
    let model = animus_std::ModelEntry::new(
        model_id,
        animus_std::ApiKind::OpenAiResponses,
        endpoint,
        capabilities,
    );
    let source = animus_std::CredentialSource::brokered(cohort.credential_source)
        .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)?;
    let provider = animus_std::ProviderEntry::new(
        provider_id,
        animus_std::CredentialRequirement::new(source, true),
        vec![model],
    )
    .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)?;
    animus_std::ProviderRegistry::new(vec![provider])
        .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)?
        .select_model(&animus_std::ModelSelection::new(
            animus_std::ProviderId::new(raw.provider.provider_id.as_str())
                .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)?,
            animus_std::ModelId::new(raw.provider.model_id.as_str())
                .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)?,
            animus_std::ThinkingLevel::Low,
            animus_std::ThinkingPolicy::Reject,
        ))
        .map_err(|_error| super::LiveSteelAgentConfigError::ProviderMismatch)
}

pub(super) fn build_skill_modules(
    raw: &super::shape::LiveSteelAgentConfig,
) -> Result<
    Vec<animus_adapter_rlm_kernel::SteelSkillModuleDescriptor>,
    super::LiveSteelAgentConfigError,
> {
    if raw.selected_skills.is_empty() || raw.selected_skills.len() > super::MAX_SKILLS {
        return Err(super::LiveSteelAgentConfigError::SkillMismatch);
    }
    let mut modules = Vec::with_capacity(raw.selected_skills.len());
    for skill in &raw.selected_skills {
        super::validation::validate_labels(&skill.exports, super::MAX_ITEMS, false)?;
        super::validation::validate_labels(&skill.capabilities, super::MAX_ITEMS, true)?;
        let names = skill.exports.iter().map(String::as_str).collect::<Vec<_>>();
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
                source: super::SKILL_SOURCE,
                exports: names.as_slice(),
                dependencies: &[],
                capability_requests: capabilities.as_slice(),
                trust: animus_adapter_rlm_kernel::SteelSkillModuleTrust::Trusted,
                bounds: animus_adapter_rlm_kernel::SteelSkillModuleBounds {
                    source_chars: skill.max_source_chars,
                    evaluation_fuel: skill.max_evaluation_fuel,
                },
            },
        )
        .map_err(|_error| super::LiveSteelAgentConfigError::SkillMismatch)?;
        modules.push(descriptor);
    }
    debug_assert_eq!(modules.len(), raw.selected_skills.len());
    debug_assert!(!modules.is_empty());
    Ok(modules)
}
