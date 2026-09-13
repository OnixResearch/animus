/// Renders metadata-only evidence for one supported live `OpenAI` smoke outcome.
#[must_use]
pub fn render_live_openai_smoke_evidence(outcome: &animus_std::ProviderDispatchOutcome) -> String {
    // r[impl agent_harness_runtime.packaged_live_openai_smoke.contract.evidence]
    render_live_responses_smoke_evidence(outcome, super::OPENAI_COHORT)
}

/// Renders metadata-only evidence for one supported live `OpenRouter` smoke outcome.
#[must_use]
pub fn render_live_openrouter_smoke_evidence(
    outcome: &animus_std::ProviderDispatchOutcome,
) -> String {
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.contract.evidence]
    render_live_responses_smoke_evidence(outcome, super::OPENROUTER_COHORT)
}

/// Renders metadata-only evidence for one supported live `local-dspark` smoke outcome.
#[must_use]
pub fn render_live_local_dspark_smoke_evidence(
    outcome: &animus_std::ProviderDispatchOutcome,
) -> String {
    render_live_responses_smoke_evidence(outcome, super::LOCAL_DSPARK_COHORT)
}

pub(super) fn render_live_responses_smoke_evidence(
    outcome: &animus_std::ProviderDispatchOutcome,
    cohort: super::SmokeCohort,
) -> String {
    let receipt = outcome.receipt();
    let evidence = format!(
        concat!(
            "{}\n",
            "provider={}\n",
            "endpoint={}\n",
            "provider_status={}\n",
            "finish_reason={}\n",
            "request_identity={}\n",
            "response_identity={}\n",
            "response_items={}\n",
            "raw_response=omitted\n",
            "model_output=omitted\n",
            "credential=omitted\n",
            "non_claims={}\n"
        ),
        cohort.status,
        receipt.provider_id(),
        receipt.endpoint_identity(),
        receipt.status().as_str(),
        receipt.finish_reason(),
        receipt.request_identity(),
        receipt.response_identity(),
        outcome.response().items().len(),
        super::NON_CLAIMS,
    );
    assert!(evidence.starts_with(cohort.status));
    assert!(evidence.ends_with('\n'));
    evidence
}
