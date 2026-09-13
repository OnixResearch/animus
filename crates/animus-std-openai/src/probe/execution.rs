/// Executes one packaged live `OpenAI` smoke from explicit model and credential values.
///
/// # Errors
///
/// Returns [`crate::LiveOpenAiSmokeError`] when planning, credential admission, transport, or response normalization fails.
#[cfg(feature = "reqwest-client")]
pub fn run_packaged_live_openai_smoke(
    model: &str,
    api_key: String,
) -> Result<String, super::LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.packaged_live_openai_smoke.binary.pass]
    run_packaged_live_responses_smoke(model, api_key, super::OPENAI_COHORT)
}

/// Executes one packaged live `OpenRouter` smoke from explicit model and credential values.
///
/// # Errors
///
/// Returns [`crate::LiveOpenAiSmokeError`] when planning, credential admission, transport, or response normalization fails.
#[cfg(feature = "reqwest-client")]
pub fn run_packaged_live_openrouter_smoke(
    model: &str,
    api_key: String,
) -> Result<String, super::LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.binary]
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.binary.pass]
    super::policy::validate_openrouter_smoke_model(model)?;
    run_packaged_live_responses_smoke(model, api_key, super::OPENROUTER_COHORT)
}

/// Executes one packaged live `local-dspark` smoke from explicit values.
///
/// # Errors
///
/// Returns [`crate::LiveOpenAiSmokeError`] when planning, credential admission, transport, or response normalization fails.
#[cfg(feature = "reqwest-client")]
pub fn run_packaged_live_local_dspark_smoke(
    model: &str,
    api_key: String,
) -> Result<String, super::LiveOpenAiSmokeError> {
    super::policy::validate_local_dspark_smoke_model(model)?;
    run_packaged_live_responses_smoke(model, api_key, super::LOCAL_DSPARK_COHORT)
}

#[cfg(feature = "reqwest-client")]
fn run_packaged_live_responses_smoke(
    model: &str,
    api_key: String,
    cohort: super::SmokeCohort,
) -> Result<String, super::LiveOpenAiSmokeError> {
    let key = crate::OpenAiApiKey::new(cohort.credential_label, api_key)?;
    let plan = super::policy::plan_live_responses_smoke(model, cohort)?;
    let config = crate::OpenAiHostConfig::new(
        cohort.endpoint_identity,
        cohort.endpoint_url,
        std::time::Duration::from_secs(super::TIMEOUT_SECONDS),
        super::MAX_RESPONSE_BYTES,
    )?;
    let host = crate::ReqwestOpenAiHost::new(config)?;
    let outcome = host.execute(&plan, &key, crate::CancellationState::Active)?;
    if outcome.response().items().is_empty() {
        return Err(super::LiveOpenAiSmokeError::EmptyOutcome);
    }
    Ok(super::report::render_live_responses_smoke_evidence(
        &outcome, cohort,
    ))
}
