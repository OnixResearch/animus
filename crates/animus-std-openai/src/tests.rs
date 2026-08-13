use std::cell::{Cell, RefCell};
use std::time::Duration;

use super::*;
use crate::shell::{
    ExecutionRequest, Transport, WireRequest, WireResponse, execute_with_transport,
};

const PROVIDER_ID: &str = "openai";
const MODEL_ID: &str = "gpt-5-mini";
const USER_TEXT: &str = "Reply with one safe word.";
const RESPONSE_TEXT: &str = "safe";
const REFUSAL_REASON: &str = "request refused";
const METADATA_KEY: &str = "trace_label";
const METADATA_VALUE: &str = "live-openai-fixture";
const CREDENTIAL_LABEL: &str = "env:ANIMUS_OPENAI_LIVE_API_KEY";
const API_KEY_VALUE: &str = "test-secret-value";
const TOOL_NAME: &str = "lookup";
const TOOL_DESCRIPTION: &str = "Look up one value.";
const TOOL_SCHEMA: &str = "schema.lookup";
const IMAGE_DESCRIPTOR: &str = "image-one";
const IMAGE_MEDIA_TYPE: &str = "image/png";
const IMAGE_HASH: &str = "BLAKE3:image";
const RESPONSE_SCHEMA: &str = "schema.answer";
const STOP_SEQUENCE: &str = "STOP";
const TIMEOUT_SECONDS: u64 = 30;
const MAX_RESPONSE_BYTES: u64 = 65_536;
const OVERSIZED_RESPONSE_BYTES: usize = 65_537;
const OUTPUT_LIMIT: u32 = 64;
const TEMPERATURE_MILLIS: u16 = 250;
const CONTEXT_BUDGET: u32 = 8_192;
const MODEL_OUTPUT_BUDGET: u32 = 4_096;
const RESPONSE_INPUT_TOKENS: u32 = 12;
const RESPONSE_OUTPUT_TOKENS: u32 = 4;
const RESPONSE_CACHED_TOKENS: u32 = 2;
const HTTP_OK: u16 = 200;
const HTTP_UNAUTHORIZED: u16 = 401;
const EXPECTED_CALLS_ONE: usize = 1;
const EXPECTED_CALLS_ZERO: usize = 0;
const FIRST_ITEM_INDEX: usize = 0;

#[cfg(feature = "tool-streaming")]
mod tool_streaming;

const BASE_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "response:text",
    "reasoning:low",
    "metadata:trace_label=live-openai-fixture",
];
const STREAM_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "response:text",
    "reasoning:low",
    "stream:true",
    "metadata:trace_label=live-openai-fixture",
];
const TOOL_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "tool:lookup",
    "response:text",
    "reasoning:low",
    "metadata:trace_label=live-openai-fixture",
];
const IMAGE_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "image:descriptor",
    "response:text",
    "reasoning:low",
    "metadata:trace_label=live-openai-fixture",
];
const STRUCTURED_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "response:schema.answer",
    "reasoning:low",
    "metadata:trace_label=live-openai-fixture",
];

#[derive(Clone, Copy)]
enum RequestCase {
    Base,
    Stream,
    Tool,
    Image,
    Structured,
    Stop,
}

struct FakeTransport {
    calls: Cell<usize>,
    response: RefCell<Option<Result<WireResponse, TransportErrorClass>>>,
    request_body: RefCell<Vec<u8>>,
}

impl FakeTransport {
    fn response(status: u16, body: Vec<u8>) -> Self {
        Self {
            calls: Cell::new(EXPECTED_CALLS_ZERO),
            response: RefCell::new(Some(Ok(WireResponse { status, body }))),
            request_body: RefCell::new(Vec::new()),
        }
    }

    fn error(class: TransportErrorClass) -> Self {
        Self {
            calls: Cell::new(EXPECTED_CALLS_ZERO),
            response: RefCell::new(Some(Err(class))),
            request_body: RefCell::new(Vec::new()),
        }
    }
}

impl Transport for FakeTransport {
    fn send(
        &self,
        request: &WireRequest<'_>,
        api_key: &OpenAiApiKey,
    ) -> Result<WireResponse, TransportErrorClass> {
        assert_eq!(request.endpoint_url, OFFICIAL_OPENAI_RESPONSES_URL);
        assert_eq!(request.max_response_bytes, MAX_RESPONSE_BYTES);
        assert_eq!(api_key.expose_value(), API_KEY_VALUE);
        self.calls.set(self.calls.get().saturating_add(1));
        self.request_body.replace(request.body.clone());
        self.response
            .borrow_mut()
            .take()
            .expect("fake response must be configured")
    }
}

#[test]
fn pure_projection_preserves_supported_openai_fields() {
    // r[verify agent_harness_runtime.live_openai_provider_host.request_projection.text]
    let body = build_openai_request_body(&config(), &plan(RequestCase::Base))
        .expect("supported request must project");
    let value: serde_json::Value =
        serde_json::from_slice(body.as_slice()).expect("request JSON must parse");

    assert_eq!(value["model"], MODEL_ID);
    assert_eq!(value["input"][FIRST_ITEM_INDEX]["role"], "user");
    assert_eq!(
        value["input"][FIRST_ITEM_INDEX]["content"][FIRST_ITEM_INDEX]["type"],
        "input_text"
    );
    assert_eq!(value["reasoning"]["effort"], "low");
    assert_eq!(value["stream"], false);
    assert_eq!(value["max_output_tokens"], OUTPUT_LIMIT);
    assert_eq!(value["temperature"], 0.25);
    assert_eq!(value["metadata"][METADATA_KEY], METADATA_VALUE);
}

#[test]
fn unsupported_plan_fields_fail_before_transport() {
    // r[verify agent_harness_runtime.live_openai_provider_host.request_projection.unsupported]
    let cases = [
        (RequestCase::Stream, "streaming"),
        (RequestCase::Tool, "tools"),
        (RequestCase::Image, "image"),
        (RequestCase::Structured, "structured_output"),
        (RequestCase::Stop, "stop_sequences"),
    ];
    for (request_case, expected_feature) in cases {
        assert_eq!(
            build_openai_request_body(&config(), &plan(request_case)),
            Err(HostError::UnsupportedPlan {
                feature: expected_feature,
            })
        );
    }
}

#[test]
fn bounded_response_normalizes_to_canonical_outcome() {
    // r[verify agent_harness_runtime.live_openai_provider_host.response_normalization.text]
    let dispatch_plan = plan(RequestCase::Base);
    let outcome = normalize_openai_response(
        &dispatch_plan,
        successful_response().as_bytes(),
        MAX_RESPONSE_BYTES,
    )
    .expect("supported response must normalize");

    assert_eq!(
        outcome.response().items()[FIRST_ITEM_INDEX].assistant_text(),
        Some(RESPONSE_TEXT)
    );
    assert_eq!(
        outcome
            .response()
            .usage()
            .expect("usage must be present")
            .input_tokens(),
        RESPONSE_INPUT_TOKENS
    );
    assert_eq!(
        outcome.receipt().status(),
        animus_std::ProviderDispatchOutcomeStatus::Success
    );
    assert!(!format!("{:?}", outcome.receipt()).contains(API_KEY_VALUE));

    let refusal = normalize_openai_response(
        &dispatch_plan,
        refusal_response().as_bytes(),
        MAX_RESPONSE_BYTES,
    )
    .expect("supported refusal must normalize");
    assert!(matches!(
        &refusal.response().items()[FIRST_ITEM_INDEX],
        animus_adapters::CanonicalModelResponseItem::Refusal { .. }
    ));
}

#[test]
fn shell_performs_one_explicit_attempt() {
    // r[verify agent_harness_runtime.live_openai_provider_host.transport.single_attempt]
    let transport = FakeTransport::response(HTTP_OK, successful_response().into_bytes());
    let dispatch_plan = plan(RequestCase::Base);
    let key = api_key(CREDENTIAL_LABEL);
    let outcome = execute_fake(&transport, &dispatch_plan, &key, CancellationState::Active)
        .expect("admitted fake transport must succeed");

    assert_eq!(transport.calls.get(), EXPECTED_CALLS_ONE);
    assert!(!transport.request_body.borrow().is_empty());
    assert_eq!(
        outcome.response().items()[FIRST_ITEM_INDEX].assistant_text(),
        Some(RESPONSE_TEXT)
    );
}

#[test]
fn cancellation_and_credential_mismatch_block_contact() {
    // r[verify agent_harness_runtime.live_openai_provider_host.transport.cancelled]
    let dispatch_plan = plan(RequestCase::Base);
    let key = api_key(CREDENTIAL_LABEL);
    let cancelled = FakeTransport::response(HTTP_OK, successful_response().into_bytes());
    assert_eq!(
        execute_fake(
            &cancelled,
            &dispatch_plan,
            &key,
            CancellationState::Cancelled,
        ),
        Err(HostError::Cancelled)
    );
    assert_eq!(cancelled.calls.get(), EXPECTED_CALLS_ZERO);

    let mismatch_key = api_key("env:OTHER_KEY");
    let mismatch = FakeTransport::response(HTTP_OK, successful_response().into_bytes());
    assert_eq!(
        execute_fake(
            &mismatch,
            &dispatch_plan,
            &mismatch_key,
            CancellationState::Active,
        ),
        Err(HostError::CredentialBindingMismatch)
    );
    assert_eq!(mismatch.calls.get(), EXPECTED_CALLS_ZERO);
}

#[test]
fn transport_status_bounds_and_malformed_responses_fail_safely() {
    // r[verify agent_harness_runtime.live_openai_provider_host.response_normalization.failure]
    let dispatch_plan = plan(RequestCase::Base);
    let key = api_key(CREDENTIAL_LABEL);

    let unauthorized = FakeTransport::response(HTTP_UNAUTHORIZED, b"private error body".to_vec());
    assert_eq!(
        execute_fake(
            &unauthorized,
            &dispatch_plan,
            &key,
            CancellationState::Active,
        ),
        Err(HostError::HttpStatus {
            status: HTTP_UNAUTHORIZED,
        })
    );

    let oversized = FakeTransport::response(HTTP_OK, vec![b'x'; OVERSIZED_RESPONSE_BYTES]);
    assert_eq!(
        execute_fake(&oversized, &dispatch_plan, &key, CancellationState::Active,),
        Err(HostError::ResponseTooLarge {
            limit: MAX_RESPONSE_BYTES,
        })
    );

    let malformed = FakeTransport::response(HTTP_OK, b"not-json".to_vec());
    assert!(matches!(
        execute_fake(&malformed, &dispatch_plan, &key, CancellationState::Active,),
        Err(HostError::MalformedResponse { .. })
    ));

    let timeout = FakeTransport::error(TransportErrorClass::Timeout);
    assert_eq!(
        execute_fake(&timeout, &dispatch_plan, &key, CancellationState::Active,),
        Err(HostError::Transport {
            class: TransportErrorClass::Timeout,
        })
    );
}

#[test]
fn endpoint_pairs_credentials_and_debug_output_fail_closed() {
    // r[verify agent_harness_runtime.live_openai_provider_host.boundary]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.endpoint.pass]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.endpoint.failure]
    OpenAiHostConfig::new(
        OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
        OPENROUTER_RESPONSES_URL,
        Duration::from_secs(TIMEOUT_SECONDS),
        MAX_RESPONSE_BYTES,
    )
    .expect("exact OpenRouter endpoint pair must be admitted");
    for (endpoint_identity, endpoint_url) in [
        ("other-endpoint", OFFICIAL_OPENAI_RESPONSES_URL),
        (
            OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
            OPENROUTER_RESPONSES_URL,
        ),
        (
            OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
            OFFICIAL_OPENAI_RESPONSES_URL,
        ),
        (
            OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
            "https://example.invalid/v1/responses",
        ),
    ] {
        assert!(matches!(
            OpenAiHostConfig::new(
                endpoint_identity,
                endpoint_url,
                Duration::from_secs(TIMEOUT_SECONDS),
                MAX_RESPONSE_BYTES,
            ),
            Err(HostError::InvalidConfiguration { .. })
        ));
    }
    assert!(matches!(
        OpenAiApiKey::new(CREDENTIAL_LABEL, String::new()),
        Err(HostError::InvalidCredential { .. })
    ));
    assert!(matches!(
        OpenAiApiKey::new(CREDENTIAL_LABEL, String::from("unsafe key")),
        Err(HostError::InvalidCredential { .. })
    ));
    let key = api_key(CREDENTIAL_LABEL);
    assert!(!format!("{key:?}").contains(API_KEY_VALUE));
}

fn execute_fake(
    transport: &FakeTransport,
    plan: &animus_std::ProviderDispatchPlan,
    api_key: &OpenAiApiKey,
    cancellation: CancellationState,
) -> Result<animus_std::ProviderDispatchOutcome, HostError> {
    let host_config = config();
    execute_with_transport(
        transport,
        ExecutionRequest {
            config: &host_config,
            plan,
            api_key,
            cancellation,
        },
    )
}

fn config() -> OpenAiHostConfig {
    OpenAiHostConfig::new(
        OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
        OFFICIAL_OPENAI_RESPONSES_URL,
        Duration::from_secs(TIMEOUT_SECONDS),
        MAX_RESPONSE_BYTES,
    )
    .expect("test configuration must be admitted")
}

fn api_key(source_label: &str) -> OpenAiApiKey {
    OpenAiApiKey::new(source_label, String::from(API_KEY_VALUE)).expect("test key must be admitted")
}

fn plan(request_case: RequestCase) -> animus_std::ProviderDispatchPlan {
    let selected = selected_model();
    let canonical = canonical_request(request_case);
    animus_std::plan_provider_dispatch(&animus_std::ProviderDispatchInput {
        selected_model: &selected,
        canonical_request: &canonical,
        reviewed_projection_fixture: projection(request_case),
        credential: animus_std::ProviderDispatchCredentialState::Ready {
            source_label: String::from(CREDENTIAL_LABEL),
        },
        service: animus_std::ProviderDispatchServiceState::Available,
        entitlement: animus_std::ProviderDispatchEntitlementState::Granted,
        deadline: animus_std::ProviderDispatchDeadlineState::Ready {
            label: String::from("deadline-live-openai"),
        },
        cancellation: animus_std::ProviderDispatchCancellationState::Ready {
            label: String::from("cancel-live-openai"),
        },
        fallback: animus_std::ProviderDispatchFallbackState::NotRequested,
        receipt_policy: animus_std::ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    })
    .expect("test dispatch plan must be admitted")
}

fn projection(request_case: RequestCase) -> &'static [&'static str] {
    match request_case {
        RequestCase::Base | RequestCase::Stop => BASE_PROJECTION,
        RequestCase::Stream => STREAM_PROJECTION,
        RequestCase::Tool => TOOL_PROJECTION,
        RequestCase::Image => IMAGE_PROJECTION,
        RequestCase::Structured => STRUCTURED_PROJECTION,
    }
}

fn canonical_request(request_case: RequestCase) -> animus_adapters::CanonicalModelRequest {
    let text = [animus_adapters::CanonicalMessageContentPartDraft::Text { text: USER_TEXT }];
    let image = [
        animus_adapters::CanonicalMessageContentPartDraft::Text { text: USER_TEXT },
        animus_adapters::CanonicalMessageContentPartDraft::ImageDescriptor {
            descriptor_id: IMAGE_DESCRIPTOR,
            media_type: IMAGE_MEDIA_TYPE,
            content_hash: IMAGE_HASH,
        },
    ];
    let content = if matches!(request_case, RequestCase::Image) {
        image.as_slice()
    } else {
        text.as_slice()
    };
    let messages = [animus_adapters::CanonicalModelMessageDraft {
        role: "user",
        content,
        tool_call_id: None,
    }];
    let tools = [animus_adapters::CanonicalModelToolDeclarationDraft {
        name: TOOL_NAME,
        description: TOOL_DESCRIPTION,
        input_schema_label: TOOL_SCHEMA,
    }];
    let metadata = [animus_adapters::CanonicalModelSafeMetadataDraft {
        key: METADATA_KEY,
        value: METADATA_VALUE,
    }];
    let stop_sequences = [STOP_SEQUENCE];
    animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: if matches!(request_case, RequestCase::Tool) {
                &tools
            } else {
                &[]
            },
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(OUTPUT_LIMIT),
                temperature_millis: Some(TEMPERATURE_MILLIS),
                stop_sequences: if matches!(request_case, RequestCase::Stop) {
                    &stop_sequences
                } else {
                    &[]
                },
                stream: matches!(request_case, RequestCase::Stream),
            },
            response_format: if matches!(request_case, RequestCase::Structured) {
                animus_adapters::CanonicalModelResponseFormatIntentDraft::JsonSchema {
                    schema_label: RESPONSE_SCHEMA,
                }
            } else {
                animus_adapters::CanonicalModelResponseFormatIntentDraft::Text
            },
            reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                enabled: true,
                effort: animus_adapters::CanonicalModelReasoningEffort::Low,
                summary_label: None,
            },
            provenance: &metadata,
        },
    )
    .expect("test canonical request must validate")
}

fn selected_model() -> animus_std::SelectedModel {
    let provider_id = animus_std::ProviderId::new(PROVIDER_ID).expect("provider ID must validate");
    let model_id = animus_std::ModelId::new(MODEL_ID).expect("model ID must validate");
    let endpoint = animus_std::EndpointIdentity::new(OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY)
        .expect("endpoint identity must validate");
    let capabilities =
        animus_std::ModelCapabilities::from_draft(animus_std::ModelCapabilitiesDraft {
            input_modalities: vec![
                animus_std::InputModality::Text,
                animus_std::InputModality::Image,
            ],
            context_budget: CONTEXT_BUDGET,
            output_budget: MODEL_OUTPUT_BUDGET,
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
        .expect("capabilities must validate");
    let model = animus_std::ModelEntry::new(
        model_id,
        animus_std::ApiKind::OpenAiResponses,
        endpoint,
        capabilities,
    );
    let credential = animus_std::CredentialRequirement::new(
        animus_std::CredentialSource::environment("ANIMUS_OPENAI_LIVE_API_KEY")
            .expect("credential source must validate"),
        true,
    );
    let provider = animus_std::ProviderEntry::new(provider_id, credential, vec![model])
        .expect("provider entry must validate");
    animus_std::ProviderRegistry::new(vec![provider])
        .expect("registry must validate")
        .select_model(&animus_std::ModelSelection::new(
            animus_std::ProviderId::new(PROVIDER_ID).expect("provider ID must validate"),
            animus_std::ModelId::new(MODEL_ID).expect("model ID must validate"),
            animus_std::ThinkingLevel::Low,
            animus_std::ThinkingPolicy::Reject,
        ))
        .expect("model must select")
}

fn refusal_response() -> String {
    serde_json::json!({
        "id": "resp_refusal_fixture",
        "object": "response",
        "status": "completed",
        "output": [
            {
                "type": "message",
                "role": "assistant",
                "content": [
                    {"type": "refusal", "refusal": REFUSAL_REASON}
                ]
            }
        ]
    })
    .to_string()
}

fn successful_response() -> String {
    serde_json::json!({
        "id": "resp_fixture",
        "object": "response",
        "status": "completed",
        "output": [
            {"type": "reasoning", "summary": []},
            {
                "type": "message",
                "role": "assistant",
                "content": [
                    {"type": "output_text", "text": RESPONSE_TEXT, "annotations": []}
                ]
            }
        ],
        "usage": {
            "input_tokens": RESPONSE_INPUT_TOKENS,
            "output_tokens": RESPONSE_OUTPUT_TOKENS,
            "input_tokens_details": {"cached_tokens": RESPONSE_CACHED_TOKENS}
        }
    })
    .to_string()
}
