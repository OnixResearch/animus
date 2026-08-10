use std::fmt;
#[cfg(feature = "reqwest-client")]
use std::time::Duration;

#[cfg(feature = "reqwest-client")]
use crate::{
    CancellationState, OFFICIAL_OPENAI_RESPONSES_URL, OPENROUTER_RESPONSES_URL, OpenAiHostConfig,
};
use crate::{
    HostError, OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY, OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
};

const OPENAI_CREDENTIAL_SOURCE: &str = "animus-live-openai-smoke-stdin";
const OPENAI_CREDENTIAL_LABEL: &str = "brokered:animus-live-openai-smoke-stdin";
const OPENAI_PROVIDER_ID: &str = "openai";
const OPENAI_STATUS: &str = "LIVE_OPENAI_SMOKE status=passed";
const OPENROUTER_CREDENTIAL_SOURCE: &str = "animus-live-openrouter-smoke-stdin";
const OPENROUTER_CREDENTIAL_LABEL: &str = "brokered:animus-live-openrouter-smoke-stdin";
const OPENROUTER_PROVIDER_ID: &str = "openrouter";
const OPENROUTER_STATUS: &str = "LIVE_OPENROUTER_SMOKE status=passed";
/// Exact reviewed model for every packaged live `OpenRouter` smoke.
pub const OPENROUTER_SMOKE_MODEL_ID: &str = "deepseek/deepseek-v4-flash-0731";
const PROMPT: &str = "Reply with the single word READY.";
#[cfg(feature = "reqwest-client")]
const TIMEOUT_SECONDS: u64 = 60;
#[cfg(any(feature = "reqwest-client", test))]
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
const NON_CLAIMS: &str =
    "reliability,quality,billing,routing,production,retry,fallback,streaming,tool-use";

#[derive(Clone, Copy)]
struct SmokeCohort {
    provider_id: &'static str,
    endpoint_identity: &'static str,
    #[cfg(feature = "reqwest-client")]
    endpoint_url: &'static str,
    credential_source: &'static str,
    credential_label: &'static str,
    deadline_label: &'static str,
    cancellation_label: &'static str,
    status: &'static str,
}

const OPENAI_COHORT: SmokeCohort = SmokeCohort {
    provider_id: OPENAI_PROVIDER_ID,
    endpoint_identity: OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
    #[cfg(feature = "reqwest-client")]
    endpoint_url: OFFICIAL_OPENAI_RESPONSES_URL,
    credential_source: OPENAI_CREDENTIAL_SOURCE,
    credential_label: OPENAI_CREDENTIAL_LABEL,
    deadline_label: "deadline-packaged-live-openai-smoke",
    cancellation_label: "cancel-packaged-live-openai-smoke",
    status: OPENAI_STATUS,
};

const OPENROUTER_COHORT: SmokeCohort = SmokeCohort {
    provider_id: OPENROUTER_PROVIDER_ID,
    endpoint_identity: OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
    #[cfg(feature = "reqwest-client")]
    endpoint_url: OPENROUTER_RESPONSES_URL,
    credential_source: OPENROUTER_CREDENTIAL_SOURCE,
    credential_label: OPENROUTER_CREDENTIAL_LABEL,
    deadline_label: "deadline-packaged-live-openrouter-smoke",
    cancellation_label: "cancel-packaged-live-openrouter-smoke",
    status: OPENROUTER_STATUS,
};

/// Safe failure from packaged live Responses smoke planning or execution.
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
            Self::InvalidModel => formatter.write_str("live Responses smoke model is invalid"),
            Self::InvalidPlan => formatter.write_str("live Responses smoke plan is invalid"),
            Self::Dispatch(source) => {
                write!(formatter, "live Responses smoke dispatch failed: {source}")
            }
            Self::Host(source) => write!(formatter, "live Responses smoke host failed: {source}"),
            Self::EmptyOutcome => {
                formatter.write_str("live Responses smoke response has no output")
            }
        }
    }
}

impl std::error::Error for LiveOpenAiSmokeError {}

impl From<HostError> for LiveOpenAiSmokeError {
    fn from(source: HostError) -> Self {
        Self::Host(source)
    }
}

/// Builds the fixed packaged `OpenAI` smoke dispatch plan for an explicit model.
///
/// # Errors
///
/// Returns [`LiveOpenAiSmokeError`] when the model or fixed plan is not admitted.
pub fn plan_live_openai_smoke(
    model: &str,
) -> Result<animus_std::ProviderDispatchPlan, LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.packaged_live_openai_smoke.contract.plan]
    plan_live_responses_smoke(model, OPENAI_COHORT)
}

/// Builds the fixed packaged `OpenRouter` smoke dispatch plan for an explicit model.
///
/// # Errors
///
/// Returns [`LiveOpenAiSmokeError`] when the model or fixed plan is not admitted.
pub fn plan_live_openrouter_smoke(
    model: &str,
) -> Result<animus_std::ProviderDispatchPlan, LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.contract]
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.contract.plan]
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model]
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model.pass]
    validate_openrouter_smoke_model(model)?;
    plan_live_responses_smoke(model, OPENROUTER_COHORT)
}

/// Renders metadata-only evidence for one supported live `OpenAI` smoke outcome.
#[must_use]
pub fn render_live_openai_smoke_evidence(outcome: &animus_std::ProviderDispatchOutcome) -> String {
    // r[impl agent_harness_runtime.packaged_live_openai_smoke.contract.evidence]
    render_live_responses_smoke_evidence(outcome, OPENAI_COHORT)
}

/// Renders metadata-only evidence for one supported live `OpenRouter` smoke outcome.
#[must_use]
pub fn render_live_openrouter_smoke_evidence(
    outcome: &animus_std::ProviderDispatchOutcome,
) -> String {
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.contract.evidence]
    render_live_responses_smoke_evidence(outcome, OPENROUTER_COHORT)
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
    run_packaged_live_responses_smoke(model, api_key, OPENAI_COHORT)
}

/// Executes one packaged live `OpenRouter` smoke from explicit model and credential values.
///
/// # Errors
///
/// Returns [`LiveOpenAiSmokeError`] when planning, credential admission, transport, or response normalization fails.
#[cfg(feature = "reqwest-client")]
pub fn run_packaged_live_openrouter_smoke(
    model: &str,
    api_key: String,
) -> Result<String, LiveOpenAiSmokeError> {
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.binary]
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.binary.pass]
    validate_openrouter_smoke_model(model)?;
    run_packaged_live_responses_smoke(model, api_key, OPENROUTER_COHORT)
}

fn validate_openrouter_smoke_model(model: &str) -> Result<(), LiveOpenAiSmokeError> {
    if model == OPENROUTER_SMOKE_MODEL_ID {
        Ok(())
    } else {
        Err(LiveOpenAiSmokeError::InvalidModel)
    }
}

fn plan_live_responses_smoke(
    model: &str,
    cohort: SmokeCohort,
) -> Result<animus_std::ProviderDispatchPlan, LiveOpenAiSmokeError> {
    let selected = selected_model(model, cohort)?;
    let request = canonical_request()?;
    animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: &selected,
        canonical_request: &request,
        reviewed_projection_fixture: REVIEWED_PROJECTION,
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
    .map_err(LiveOpenAiSmokeError::Dispatch)
}

fn render_live_responses_smoke_evidence(
    outcome: &animus_std::ProviderDispatchOutcome,
    cohort: SmokeCohort,
) -> String {
    let receipt = outcome.receipt();
    format!(
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
        NON_CLAIMS,
    )
}

#[cfg(feature = "reqwest-client")]
fn run_packaged_live_responses_smoke(
    model: &str,
    api_key: String,
    cohort: SmokeCohort,
) -> Result<String, LiveOpenAiSmokeError> {
    let key = crate::OpenAiApiKey::new(cohort.credential_label, api_key)?;
    let plan = plan_live_responses_smoke(model, cohort)?;
    let config = OpenAiHostConfig::new(
        cohort.endpoint_identity,
        cohort.endpoint_url,
        Duration::from_secs(TIMEOUT_SECONDS),
        MAX_RESPONSE_BYTES,
    )?;
    let host = crate::ReqwestOpenAiHost::new(config)?;
    let outcome = host.execute(&plan, &key, CancellationState::Active)?;
    if outcome.response().items().is_empty() {
        return Err(LiveOpenAiSmokeError::EmptyOutcome);
    }
    Ok(render_live_responses_smoke_evidence(&outcome, cohort))
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

fn selected_model(
    model: &str,
    cohort: SmokeCohort,
) -> Result<animus_std::SelectedModel, LiveOpenAiSmokeError> {
    let provider_id = animus_std::ProviderId::new(cohort.provider_id)
        .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?;
    let model_id =
        animus_std::ModelId::new(model).map_err(|_error| LiveOpenAiSmokeError::InvalidModel)?;
    let endpoint = animus_std::EndpointIdentity::new(cohort.endpoint_identity)
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
        animus_std::CredentialSource::brokered(cohort.credential_source)
            .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?,
        true,
    );
    let provider = animus_std::ProviderEntry::new(provider_id, requirement, vec![entry])
        .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?;
    animus_std::ProviderRegistry::new(vec![provider])
        .map_err(|_error| LiveOpenAiSmokeError::InvalidPlan)?
        .select_model(&animus_std::ModelSelection::new(
            animus_std::ProviderId::new(cohort.provider_id)
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

    const OPENAI_MODEL_ID: &str = "gpt-5-mini";
    const SUBSTITUTED_OPENROUTER_MODEL_ID: &str = "openai/gpt-5.5";
    const RESPONSE_TEXT: &str = "READY";
    const API_KEY_SENTINEL: &str = "secret-live-key";
    const FIRST_ITEM_INDEX: usize = 0;

    #[test]
    fn positive_plans_and_evidence_are_bounded() {
        // r[verify agent_harness_runtime.packaged_live_openai_smoke.contract.plan]
        // r[verify agent_harness_runtime.packaged_live_openai_smoke.contract.evidence]
        // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.contract.plan]
        // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.contract.evidence]
        let cases = [
            (
                plan_live_openai_smoke(OPENAI_MODEL_ID).expect("OpenAI model must produce plan"),
                OPENAI_STATUS,
                OPENAI_PROVIDER_ID,
                OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
                OPENAI_MODEL_ID,
                render_live_openai_smoke_evidence
                    as fn(&animus_std::ProviderDispatchOutcome) -> String,
            ),
            (
                plan_live_openrouter_smoke(OPENROUTER_SMOKE_MODEL_ID)
                    .expect("reviewed OpenRouter model must produce plan"),
                OPENROUTER_STATUS,
                OPENROUTER_PROVIDER_ID,
                OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
                OPENROUTER_SMOKE_MODEL_ID,
                render_live_openrouter_smoke_evidence
                    as fn(&animus_std::ProviderDispatchOutcome) -> String,
            ),
        ];
        for (plan, status, provider, endpoint, model, render) in cases {
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
            let evidence = render(&outcome);

            assert_eq!(plan.provider_id(), provider);
            assert_eq!(plan.endpoint(), endpoint);
            assert_eq!(
                outcome.response().items()[FIRST_ITEM_INDEX].assistant_text(),
                Some(RESPONSE_TEXT)
            );
            assert!(evidence.contains(status));
            assert!(evidence.contains("raw_response=omitted"));
            assert!(evidence.contains("credential=omitted"));
            assert!(!evidence.contains(PROMPT));
            assert!(!evidence.contains(RESPONSE_TEXT));
            assert!(!evidence.contains(API_KEY_SENTINEL));
            assert!(!evidence.contains(model));
        }
    }

    #[test]
    fn negative_invalid_model_fails_before_host_execution() {
        // r[verify agent_harness_runtime.packaged_live_openai_smoke.binary.failure]
        // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.binary.failure]
        let invalid_model = "invalid model with spaces";
        assert_eq!(
            plan_live_openai_smoke(invalid_model),
            Err(LiveOpenAiSmokeError::InvalidModel)
        );
        assert_eq!(
            plan_live_openrouter_smoke(invalid_model),
            Err(LiveOpenAiSmokeError::InvalidModel)
        );
        // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.reviewed_model.failure]
        assert_eq!(
            plan_live_openrouter_smoke(SUBSTITUTED_OPENROUTER_MODEL_ID),
            Err(LiveOpenAiSmokeError::InvalidModel)
        );
    }
}
