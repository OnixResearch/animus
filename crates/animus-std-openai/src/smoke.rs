use std::fmt;
use std::time::Duration;

use crate::{
    CancellationState, HostError, OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
    OFFICIAL_OPENAI_RESPONSES_URL, OpenAiHostConfig,
};

const CREDENTIAL_SOURCE: &str = "animus-live-openai-smoke-stdin";
const CREDENTIAL_LABEL: &str = "brokered:animus-live-openai-smoke-stdin";
const PROVIDER_ID: &str = "openai";
const PROMPT: &str = "Reply with the single word READY.";
const TIMEOUT_SECONDS: u64 = 60;
const MAX_RESPONSE_BYTES: u64 = 262_144;
const OUTPUT_LIMIT: u32 = 32;
const CONTEXT_BUDGET: u32 = 8_192;
const MODEL_OUTPUT_BUDGET: u32 = 4_096;
const REVIEWED_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with the single word READY.",
    "response:text",
];
const NON_CLAIMS: &str = "reliability,quality,billing,production,retry,fallback,streaming,tool-use";

/// Safe failure from packaged live `OpenAI` smoke planning or execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveOpenAiSmokeError {
    /// The explicit model identity was not admitted.
    InvalidModel,
    /// The fixed smoke request or registry metadata could not be constructed.
    InvalidPlan,
    /// The concrete provider dispatch planner rejected the fixed smoke request.
    Dispatch(animus_std::ProviderDispatchError),
    /// The accepted live host rejected configuration, credentials, transport, or response data.
    Host(HostError),
    /// The canonical response contained no admitted output item.
    EmptyOutcome,
}

impl fmt::Display for LiveOpenAiSmokeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidModel => formatter.write_str("live OpenAI smoke model is invalid"),
            Self::InvalidPlan => formatter.write_str("live OpenAI smoke plan is invalid"),
            Self::Dispatch(source) => {
                write!(formatter, "live OpenAI smoke dispatch failed: {source}")
            }
            Self::Host(source) => write!(formatter, "live OpenAI smoke host failed: {source}"),
            Self::EmptyOutcome => formatter.write_str("live OpenAI smoke response has no output"),
        }
    }
}

impl std::error::Error for LiveOpenAiSmokeError {}

impl From<HostError> for LiveOpenAiSmokeError {
    fn from(source: HostError) -> Self {
        Self::Host(source)
    }
}

/// Builds the fixed packaged smoke dispatch plan for an explicit model.
///
/// # Errors
///
/// Returns [`LiveOpenAiSmokeError`] when the model or fixed plan is not admitted.
pub fn plan_live_openai_smoke(
    model: &str,
) -> Result<animus_std::ProviderDispatchPlan, LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.packaged_live_openai_smoke.contract.plan]
    let selected = selected_model(model)?;
    let request = canonical_request()?;
    animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: &selected,
        canonical_request: &request,
        reviewed_projection_fixture: REVIEWED_PROJECTION,
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(CREDENTIAL_LABEL),
        },
        service: animus_std::ProviderDispatchServiceState::Available,
        entitlement: animus_std::ProviderDispatchEntitlementState::Granted,
        deadline: animus_std::ProviderDispatchDeadlineState::Ready {
            label: String::from("deadline-packaged-live-openai-smoke"),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from("cancel-packaged-live-openai-smoke"),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    })
    .map_err(LiveOpenAiSmokeError::Dispatch)
}

/// Renders metadata-only evidence for one supported live smoke outcome.
#[must_use]
pub fn render_live_openai_smoke_evidence(outcome: &animus_std::ProviderDispatchOutcome) -> String {
    // r[impl agent_harness_runtime.packaged_live_openai_smoke.contract.evidence]
    let receipt = outcome.receipt();
    format!(
        concat!(
            "LIVE_OPENAI_SMOKE status=passed\n",
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
        receipt.provider_id(),
        receipt.endpoint_identity(),
        receipt.status().as_str(),
        receipt.finish_reason(),
        receipt.request_identity(),
        receipt.response_identity(),
        outcome.response().items().len(),
        NON_CLAIMS,
    )
}

/// Executes one packaged live `OpenAI` smoke from explicit model and credential values.
///
/// # Errors
///
/// Returns [`LiveOpenAiSmokeError`] when planning, credential admission, transport, or response normalization fails.
#[cfg(feature = "reqwest-client")]
pub fn run_packaged_live_openai_smoke(
    model: &str,
    api_key: String,
) -> Result<String, LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.packaged_live_openai_smoke.binary.pass]
    let key = crate::OpenAiApiKey::new(CREDENTIAL_LABEL, api_key)?;
    let plan = plan_live_openai_smoke(model)?;
    let config = OpenAiHostConfig::new(
        OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
        OFFICIAL_OPENAI_RESPONSES_URL,
        Duration::from_secs(TIMEOUT_SECONDS),
        MAX_RESPONSE_BYTES,
    )?;
    let host = crate::ReqwestOpenAiHost::new(config)?;
    let outcome = host.execute(&plan, &key, CancellationState::Active)?;
    if outcome.response().items().is_empty() {
        return Err(LiveOpenAiSmokeError::EmptyOutcome);
    }
    Ok(render_live_openai_smoke_evidence(&outcome))
}

fn canonical_request() -> Result<animus_adapters::CanonicalModelRequest, LiveOpenAiSmokeError> {
    let content = [animus_adapters::CanonicalMessageContentPartDraft::Text { text: PROMPT }];
    let messages = [animus_adapters::CanonicalModelMessageDraft {
        role: "user",
        content: &content,
        tool_call_id: None,
    }];
    animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: &[],
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(OUTPUT_LIMIT),
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
    .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)
}

fn selected_model(model: &str) -> Result<animus_std::SelectedModel, LiveOpenAiSmokeError> {
    let provider_id = animus_std::ProviderId::new(PROVIDER_ID)
        .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?;
    let model_id =
        animus_std::ModelId::new(model).map_err(|_error| LiveOpenAiSmokeError::InvalidModel)?;
    let endpoint = animus_std::EndpointIdentity::new(OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY)
        .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?;
    let capabilities =
        animus_std::ModelCapabilities::from_draft(animus_std::ModelCapabilitiesDraft {
            input_modalities: vec![animus_std::InputModality::Text],
            context_budget: CONTEXT_BUDGET,
            output_budget: MODEL_OUTPUT_BUDGET,
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
        .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?;
    let entry = animus_std::ModelEntry::new(
        model_id,
        animus_std::ApiKind::OpenAiResponses,
        endpoint,
        capabilities,
    );
    let requirement = animus_std::CredentialRequirement::new(
        animus_std::CredentialSource::brokered(CREDENTIAL_SOURCE)
            .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?,
        true,
    );
    let provider = animus_std::ProviderEntry::new(provider_id, requirement, vec![entry])
        .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?;
    animus_std::ProviderRegistry::new(vec![provider])
        .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?
        .select_model(&animus_std::ModelSelection::new(
            animus_std::ProviderId::new(PROVIDER_ID)
                .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?,
            animus_std::ModelId::new(model).map_err(|_error| LiveOpenAiSmokeError::InvalidModel)?,
            animus_std::ThinkingLevel::None,
            animus_std::ThinkingPolicy::Reject,
        ))
        .map_err(|_error| LiveOpenAiSmokeError::InvalidModel)
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODEL_ID: &str = "gpt-5-mini";
    const RESPONSE_TEXT: &str = "READY";
    const API_KEY_SENTINEL: &str = "secret-live-key";
    const FIRST_ITEM_INDEX: usize = 0;

    #[test]
    fn positive_plan_and_evidence_are_bounded() {
        // r[verify agent_harness_runtime.packaged_live_openai_smoke.contract.plan]
        // r[verify agent_harness_runtime.packaged_live_openai_smoke.contract.evidence]
        let plan = plan_live_openai_smoke(MODEL_ID).expect("model must produce smoke plan");
        let response = serde_json::json!({
            "status": "completed",
            "output": [{
                "type": "message",
                "role": "assistant",
                "content": [{"type": "output_text", "text": RESPONSE_TEXT}]
            }]
        })
        .to_string();
        let outcome =
            crate::normalize_openai_response(&plan, response.as_bytes(), MAX_RESPONSE_BYTES)
                .expect("supported response must normalize");
        let evidence = render_live_openai_smoke_evidence(&outcome);

        assert_eq!(
            outcome.response().items()[FIRST_ITEM_INDEX].assistant_text(),
            Some(RESPONSE_TEXT)
        );
        assert!(evidence.contains("LIVE_OPENAI_SMOKE status=passed"));
        assert!(evidence.contains("raw_response=omitted"));
        assert!(evidence.contains("credential=omitted"));
        assert!(!evidence.contains(PROMPT));
        assert!(!evidence.contains(RESPONSE_TEXT));
        assert!(!evidence.contains(API_KEY_SENTINEL));
        assert!(!evidence.contains(MODEL_ID));
    }

    #[test]
    fn negative_invalid_model_fails_before_host_execution() {
        // r[verify agent_harness_runtime.packaged_live_openai_smoke.binary.failure]
        assert_eq!(
            plan_live_openai_smoke("invalid model with spaces"),
            Err(LiveOpenAiSmokeError::InvalidModel)
        );
    }
}
