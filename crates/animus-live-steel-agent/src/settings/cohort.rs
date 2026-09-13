#[derive(Clone, Copy)]
pub(super) struct Admitted {
    provider_id: &'static str,
    endpoint_identity: &'static str,
    endpoint_url: &'static str,
    credential_reference: &'static str,
    pub(super) credential_source: &'static str,
    reasoning_effort: &'static str,
}

const LOCAL_DSPARK_PROVIDER_COHORT: Admitted = Admitted {
    provider_id: super::LOCAL_DSPARK_PROVIDER_ID,
    endpoint_identity: animus_std_openai::LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY,
    endpoint_url: animus_std_openai::LOCAL_DSPARK_RESPONSES_URL,
    credential_reference: super::LOCAL_DSPARK_CREDENTIAL_REFERENCE,
    credential_source: super::LOCAL_DSPARK_CREDENTIAL_SOURCE,
    reasoning_effort: super::DISABLED_REASONING_EFFORT,
};
const OPENAI_PROVIDER_COHORT: Admitted = Admitted {
    provider_id: super::OPENAI_PROVIDER_ID,
    endpoint_identity: animus_std_openai::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
    endpoint_url: animus_std_openai::OFFICIAL_OPENAI_RESPONSES_URL,
    credential_reference: super::OPENAI_CREDENTIAL_REFERENCE,
    credential_source: super::OPENAI_CREDENTIAL_SOURCE,
    reasoning_effort: super::LOW_REASONING_EFFORT,
};
const OPENROUTER_PROVIDER_COHORT: Admitted = Admitted {
    provider_id: super::OPENROUTER_PROVIDER_ID,
    endpoint_identity: animus_std_openai::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
    endpoint_url: animus_std_openai::OPENROUTER_RESPONSES_URL,
    credential_reference: super::OPENROUTER_CREDENTIAL_REFERENCE,
    credential_source: super::OPENROUTER_CREDENTIAL_SOURCE,
    reasoning_effort: super::LOW_REASONING_EFFORT,
};

pub(super) fn validate_provider(
    provider: &super::shape::ProviderConfig,
) -> Result<(), super::LiveSteelAgentConfigError> {
    for label in [
        provider.provider_id.as_str(),
        provider.model_id.as_str(),
        provider.endpoint_identity.as_str(),
        provider.credential_reference.as_str(),
        provider.reasoning_effort.as_str(),
        provider.tool_name.as_str(),
        provider.tool_schema.as_str(),
    ] {
        super::validation::validate_label(label)?;
    }
    let cohort = admit(provider)?;
    let is_provider_admitted = provider.reasoning_effort == cohort.reasoning_effort
        && provider.tool_name == animus_std_openai::STEEL_REPL_TOOL_NAME
        && provider.tool_schema == animus_std_openai::STEEL_REPL_TOOL_SCHEMA;
    if !is_provider_admitted {
        return Err(super::LiveSteelAgentConfigError::ProviderMismatch);
    }
    debug_assert_eq!(provider.tool_name, animus_std_openai::STEEL_REPL_TOOL_NAME);
    debug_assert_eq!(provider.reasoning_effort, cohort.reasoning_effort);
    Ok(())
}

pub(super) fn admit(
    provider: &super::shape::ProviderConfig,
) -> Result<&'static Admitted, super::LiveSteelAgentConfigError> {
    // r[impl agent_harness_runtime.live_steel_agent_provider_cohort]
    let cohort = match provider.provider_id.as_str() {
        super::LOCAL_DSPARK_PROVIDER_ID => &LOCAL_DSPARK_PROVIDER_COHORT,
        super::OPENAI_PROVIDER_ID => &OPENAI_PROVIDER_COHORT,
        super::OPENROUTER_PROVIDER_ID => &OPENROUTER_PROVIDER_COHORT,
        _unknown => return Err(super::LiveSteelAgentConfigError::ProviderMismatch),
    };
    let is_tuple_exact = provider.provider_id == cohort.provider_id
        && provider.endpoint_identity == cohort.endpoint_identity
        && provider.endpoint_url == cohort.endpoint_url
        && provider.credential_reference == cohort.credential_reference;
    if !is_tuple_exact {
        return Err(super::LiveSteelAgentConfigError::ProviderMismatch);
    }
    Ok(cohort)
}
