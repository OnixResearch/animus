//! The selected model of a live route: one reviewed endpoint, one model, text-only capabilities.

/// Context tokens the selected model is declared to accept.
const CONTEXT_TOKENS: u32 = 32_768;
/// Output tokens the selected model is declared to produce.
const MODEL_OUTPUT_TOKENS: u32 = 4_096;
/// Credential source label of the route.
const CREDENTIAL_SOURCE: &str = "animus-campaign-manager-eval-stdin";

/// The selected `model` on `endpoint` with text-only capabilities.
pub(super) fn selected_model(
    endpoint: super::Endpoint,
    model: &str,
) -> Result<animus_std::SelectedModel, super::LiveFailure> {
    let selection = animus_std::ModelSelection::new(
        animus_std::ProviderId::new(endpoint.provider()).map_err(|_| super::LiveFailure::Host)?,
        animus_std::ModelId::new(model).map_err(|_| super::LiveFailure::Model)?,
        animus_std::ThinkingLevel::None,
        animus_std::ThinkingPolicy::Reject,
    );
    animus_std::ProviderRegistry::new(vec![provider(endpoint, model)?])
        .map_err(|_| super::LiveFailure::Host)?
        .select_model(&selection)
        .map_err(|_| super::LiveFailure::Model)
}

/// The provider entry offering only `model` on `endpoint`, with a brokered credential.
fn provider(
    endpoint: super::Endpoint,
    model: &str,
) -> Result<animus_std::ProviderEntry, super::LiveFailure> {
    let entry = animus_std::ModelEntry::new(
        animus_std::ModelId::new(model).map_err(|_| super::LiveFailure::Model)?,
        animus_std::ApiKind::OpenAiResponses,
        animus_std::EndpointIdentity::new(endpoint.identity())
            .map_err(|_| super::LiveFailure::Host)?,
        capabilities()?,
    );
    let requirement = animus_std::CredentialRequirement::new(
        animus_std::CredentialSource::brokered(CREDENTIAL_SOURCE)
            .map_err(|_| super::LiveFailure::Credential)?,
        true,
    );
    let provider_id =
        animus_std::ProviderId::new(endpoint.provider()).map_err(|_| super::LiveFailure::Host)?;
    animus_std::ProviderEntry::new(provider_id, requirement, vec![entry])
        .map_err(|_| super::LiveFailure::Host)
}

/// Text-only model capabilities with the route's token budgets.
fn capabilities() -> Result<animus_std::ModelCapabilities, super::LiveFailure> {
    animus_std::ModelCapabilities::from_draft(animus_std::ModelCapabilitiesDraft {
        input_modalities: vec![animus_std::InputModality::Text],
        context_budget: CONTEXT_TOKENS,
        output_budget: MODEL_OUTPUT_TOKENS,
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
    .map_err(|_| super::LiveFailure::Model)
}
