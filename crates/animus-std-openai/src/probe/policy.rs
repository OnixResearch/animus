/// Builds the fixed packaged `OpenAI` smoke dispatch plan for an explicit model.
///
/// # Errors
///
/// Returns [`crate::LiveOpenAiSmokeError`] when the model or fixed plan is not admitted.
pub fn plan_live_openai_smoke(
    model: &str,
) -> Result<animus_std::ProviderDispatchPlan, super::LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.packaged_live_openai_smoke.contract.plan]
    plan_live_responses_smoke(model, super::OPENAI_COHORT)
}

/// Builds the fixed packaged `OpenRouter` smoke dispatch plan for an explicit model.
///
/// # Errors
///
/// Returns [`crate::LiveOpenAiSmokeError`] when the model or fixed plan is not admitted.
pub fn plan_live_openrouter_smoke(
    model: &str,
) -> Result<animus_std::ProviderDispatchPlan, super::LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.contract]
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.contract.plan]
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model]
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model.pass]
    validate_openrouter_smoke_model(model)?;
    plan_live_responses_smoke(model, super::OPENROUTER_COHORT)
}

/// Builds the fixed packaged `local-dspark` smoke dispatch plan.
///
/// # Errors
///
/// Returns [`crate::LiveOpenAiSmokeError`] when the exact model or fixed plan is not admitted.
pub fn plan_live_local_dspark_smoke(
    model: &str,
) -> Result<animus_std::ProviderDispatchPlan, super::LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.local_dspark_responses.verification]
    validate_local_dspark_smoke_model(model)?;
    plan_live_responses_smoke(model, super::LOCAL_DSPARK_COHORT)
}

pub(super) fn validate_local_dspark_smoke_model(
    model: &str,
) -> Result<(), super::LiveOpenAiSmokeError> {
    if model == super::LOCAL_DSPARK_SMOKE_MODEL_ID {
        Ok(())
    } else {
        Err(super::LiveOpenAiSmokeError::InvalidModel)
    }
}

pub(super) fn validate_openrouter_smoke_model(
    model: &str,
) -> Result<(), super::LiveOpenAiSmokeError> {
    if model == super::OPENROUTER_SMOKE_MODEL_ID {
        Ok(())
    } else {
        Err(super::LiveOpenAiSmokeError::InvalidModel)
    }
}

pub(super) fn plan_live_responses_smoke(
    model: &str,
    cohort: super::SmokeCohort,
) -> Result<animus_std::ProviderDispatchPlan, super::LiveOpenAiSmokeError> {
    let selected = selected_model(model, cohort)?;
    let request = canonical_request()?;
    let plan = animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: &selected,
        canonical_request: &request,
        reviewed_projection_fixture: super::REVIEWED_PROJECTION,
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(cohort.credential_label),
        },
        service: animus_std::ProviderDispatchServiceState::Available,
        entitlement: animus_std::ProviderDispatchEntitlementState::Granted,
        deadline: animus_std::ProviderDispatchDeadlineState::Ready {
            label: String::from(cohort.deadline_label),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from(cohort.cancellation_label),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    })
    .map_err(super::LiveOpenAiSmokeError::Dispatch)?;
    assert_eq!(plan.endpoint(), cohort.endpoint_identity);
    assert_eq!(plan.credential_source_label(), cohort.credential_label);
    Ok(plan)
}

fn canonical_request() -> Result<animus_adapters::CanonicalModelRequest, super::LiveOpenAiSmokeError>
{
    let content = [animus_adapters::CanonicalMessageContentPartDraft::Text {
        text: super::PROMPT,
    }];
    let messages = [animus_adapters::CanonicalModelMessageDraft {
        role: "user",
        content: &content,
        tool_call_id: None,
    }];
    let request = animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: &[],
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(super::OUTPUT_LIMIT),
                temperature_millis: None,
                stop_sequences: &[],
                stream: false,
            },
            response_format: animus_adapters::CanonicalModelResponseFormatIntentDraft::Text,
            reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                enabled: false,
                effort: animus_adapters::CanonicalModelReasoningEffort::None,
                summary_label: None,
            },
            provenance: &[],
        },
    )
    .map_err(|_error| super::LiveOpenAiSmokeError::InvalidPlan)?;
    assert_eq!(request.messages().len(), messages.len());
    assert!(request.tools().is_empty());
    Ok(request)
}

fn selected_model(
    model: &str,
    cohort: super::SmokeCohort,
) -> Result<animus_std::SelectedModel, super::LiveOpenAiSmokeError> {
    let provider_id = animus_std::ProviderId::new(cohort.provider_id)
        .map_err(|_error| super::LiveOpenAiSmokeError::InvalidPlan)?;
    let model_id = animus_std::ModelId::new(model)
        .map_err(|_error| super::LiveOpenAiSmokeError::InvalidModel)?;
    let endpoint = animus_std::EndpointIdentity::new(cohort.endpoint_identity)
        .map_err(|_error| super::LiveOpenAiSmokeError::InvalidPlan)?;
    let capabilities =
        animus_std::ModelCapabilities::from_draft(animus_std::ModelCapabilitiesDraft {
            input_modalities: vec![animus_std::InputModality::Text],
            context_budget: super::CONTEXT_BUDGET,
            output_budget: super::MODEL_OUTPUT_BUDGET,
            reasoning: animus_std::ReasoningSupport::Optional,
            thinking_levels: vec![animus_std::ThinkingLevel::None],
            streaming: animus_std::ModelStreamingCapabilities::new(
                animus_std::StreamSupport::Supported,
                animus_std::StreamSupport::Supported,
                animus_std::StreamSupport::Supported,
                animus_std::StreamSupport::Supported,
            ),
            cost: None,
        })
        .map_err(|_error| super::LiveOpenAiSmokeError::InvalidPlan)?;
    let entry = animus_std::ModelEntry::new(
        model_id,
        animus_std::ApiKind::OpenAiResponses,
        endpoint,
        capabilities,
    );
    let requirement = animus_std::CredentialRequirement::new(
        animus_std::CredentialSource::brokered(cohort.credential_source)
            .map_err(|_error| super::LiveOpenAiSmokeError::InvalidPlan)?,
        true,
    );
    let provider = animus_std::ProviderEntry::new(provider_id, requirement, vec![entry])
        .map_err(|_error| super::LiveOpenAiSmokeError::InvalidPlan)?;
    let selected = animus_std::ProviderRegistry::new(vec![provider])
        .map_err(|_error| super::LiveOpenAiSmokeError::InvalidPlan)?
        .select_model(&animus_std::ModelSelection::new(
            animus_std::ProviderId::new(cohort.provider_id)
                .map_err(|_error| super::LiveOpenAiSmokeError::InvalidPlan)?,
            animus_std::ModelId::new(model)
                .map_err(|_error| super::LiveOpenAiSmokeError::InvalidModel)?,
            animus_std::ThinkingLevel::None,
            animus_std::ThinkingPolicy::Reject,
        ))
        .map_err(|_error| super::LiveOpenAiSmokeError::InvalidModel)?;
    assert_eq!(selected.provider_id().as_str(), cohort.provider_id);
    assert_eq!(selected.model_id().as_str(), model);
    Ok(selected)
}
