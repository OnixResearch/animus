//! Ignored opt-in live `OpenAI` smoke test.

use std::time::Duration;

use animus_std_openai::{
    CancellationState, OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY, OFFICIAL_OPENAI_RESPONSES_URL,
    OpenAiApiKey, OpenAiHostConfig, ReqwestOpenAiHost,
};

const API_KEY_ENV: &str = "ANIMUS_OPENAI_LIVE_API_KEY";
const MODEL_ENV: &str = "ANIMUS_OPENAI_LIVE_MODEL";
const CREDENTIAL_LABEL: &str = "env:ANIMUS_OPENAI_LIVE_API_KEY";
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
    "reasoning:none",
];

#[test]
#[ignore = "requires explicit ANIMUS_OPENAI_LIVE_API_KEY and ANIMUS_OPENAI_LIVE_MODEL"]
fn explicit_live_openai_smoke() {
    let api_key = std::env::var(API_KEY_ENV).expect("explicit live API key is required");
    let model = std::env::var(MODEL_ENV).expect("explicit live model is required");
    let plan = dispatch_plan(model.as_str());
    let config = OpenAiHostConfig::new(
        OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
        OFFICIAL_OPENAI_RESPONSES_URL,
        Duration::from_secs(TIMEOUT_SECONDS),
        MAX_RESPONSE_BYTES,
    )
    .expect("live host configuration must validate");
    let host = ReqwestOpenAiHost::new(config).expect("HTTPS client must initialize");
    let key = OpenAiApiKey::new(CREDENTIAL_LABEL, api_key).expect("API key must validate");
    let outcome = host
        .execute(&plan, &key, CancellationState::Active)
        .expect("explicit live OpenAI request must succeed");

    assert!(!outcome.response().items().is_empty());
    assert_eq!(outcome.receipt().provider_id(), PROVIDER_ID);
}

fn dispatch_plan(model: &str) -> animus_std::ProviderDispatchPlan {
    let selected = selected_model(model);
    let request = canonical_request();
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
            label: String::from("deadline-live-openai-smoke"),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from("cancel-live-openai-smoke"),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    })
    .expect("live dispatch plan must validate")
}

fn canonical_request() -> animus_adapters::CanonicalModelRequest {
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
    .expect("live canonical request must validate")
}

fn selected_model(model: &str) -> animus_std::SelectedModel {
    let provider_id = animus_std::ProviderId::new(PROVIDER_ID).expect("provider ID must validate");
    let model_id = animus_std::ModelId::new(model).expect("model ID must validate");
    let endpoint = animus_std::EndpointIdentity::new(OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY)
        .expect("endpoint identity must validate");
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
        .expect("live capabilities must validate");
    let entry = animus_std::ModelEntry::new(
        model_id,
        animus_std::ApiKind::OpenAiResponses,
        endpoint,
        capabilities,
    );
    let requirement = animus_std::CredentialRequirement::new(
        animus_std::CredentialSource::environment(API_KEY_ENV)
            .expect("credential source must validate"),
        true,
    );
    let provider = animus_std::ProviderEntry::new(provider_id, requirement, vec![entry])
        .expect("provider must validate");
    animus_std::ProviderRegistry::new(vec![provider])
        .expect("registry must validate")
        .select_model(&animus_std::ModelSelection::new(
            animus_std::ProviderId::new(PROVIDER_ID).expect("provider ID must validate"),
            animus_std::ModelId::new(model).expect("model ID must validate"),
            animus_std::ThinkingLevel::None,
            animus_std::ThinkingPolicy::Reject,
        ))
        .expect("live model must select")
}
