use super::*;

const DISPATCH_DEVELOPER_TEXT: &str = "Preserve dispatch semantics.";
const DISPATCH_USER_TEXT: &str = "Answer safely.";
const IMAGE_DESCRIPTOR: &str = "image-one";
const IMAGE_MEDIA_TYPE: &str = "image/png";
const IMAGE_HASH: &str = "b3-image";
const TOOL_NAME: &str = "lookup";
const TOOL_DESCRIPTION: &str = "Bounded lookup.";
const TOOL_SCHEMA: &str = "schema.lookup";
const RESPONSE_SCHEMA: &str = "schema.answer";
const TRACE_LABEL: &str = "trace_label";
const TRACE_VALUE: &str = "dispatch-fixture";
const DEADLINE_LABEL: &str = "deadline-turn-main";
const CANCELLATION_LABEL: &str = "cancel-turn-main";
const RESPONSE_TEXT: &str = "safe answer";
const RAW_PAYLOAD: &str = "raw_provider_payload:{secret}";
const OUTPUT_LIMIT: u32 = 64;
const TEMPERATURE_MILLIS: u16 = 500;
const RESPONSE_INPUT_TOKENS: u32 = 12;
const RESPONSE_OUTPUT_TOKENS: u32 = 8;
const RESPONSE_CACHED_INPUT_TOKENS: u32 = 4;
const BLAKE3_PREFIX: &str = "BLAKE3:";

const REVIEWED_PROJECTION_FIXTURE: &[&str] = &[
    "family:openai_responses",
    "role:developer",
    "text:Preserve dispatch semantics.",
    "role:user",
    "text:Answer safely.",
    "image:descriptor",
    "tool:lookup",
    "response:schema.answer",
    "reasoning:high",
    "stream:true",
    "metadata:trace_label=dispatch-fixture",
];

const BAD_REVIEWED_PROJECTION_FIXTURE: &[&str] = &[
    "family:openai_responses",
    "role:developer",
    "text:Preserve dispatch semantics.",
    "role:user",
    "text:Answer safely.",
    "image:descriptor",
    "response:schema.answer",
    "reasoning:high",
    "stream:true",
    "metadata:trace_label=dispatch-fixture",
];

const REVIEWED_OPENAI_REQUEST_FIXTURE: &[&str] = &[
    "family:openai_responses",
    "model:gpt-5-mini",
    "role:developer",
    "text:Preserve dispatch semantics.",
    "role:user",
    "text:Answer safely.",
    "image:image-one:image/png:b3-image",
    "tool:lookup:schema.lookup",
    "response_format:json_schema:schema.answer",
    "reasoning:high",
    "stream:true",
    "max_output_tokens:64",
    "temperature_millis:500",
    "metadata:trace_label=dispatch-fixture",
];

#[test]
fn provider_dispatch_plans_with_reviewed_projection_and_concrete_literal() {
    // r[verify agent_harness_runtime.provider_dispatch_adapter.plan]
    // r[verify agent_harness_runtime.provider_dispatch_adapter.request_projection]
    // r[verify agent_harness_runtime.provider_dispatch_adapter.parity_rails]
    let selected = dispatch_selected_model();
    let request = canonical_dispatch_request();
    let plan = plan_provider_dispatch(&valid_dispatch_input(&selected, &request))
        .expect("provider dispatch should plan");
    let concrete_fixture = render_openai_responses_request_fixture(plan.request());
    let concrete_refs: Vec<&str> = concrete_fixture.iter().map(String::as_str).collect();
    let matrix = provider_dispatch_shared_field_matrix(&selected, &request, plan.request());

    assert_eq!(plan.provider_id(), PROVIDER_ID);
    assert_eq!(plan.model_id(), MODEL_ID);
    assert_eq!(plan.api_kind(), ApiKind::OpenAiResponses);
    assert_eq!(concrete_refs.as_slice(), REVIEWED_OPENAI_REQUEST_FIXTURE);
    assert!(matrix.is_complete());
    validate_provider_dispatch_shared_field_matrix(&matrix).expect("matrix should validate");
    validate_provider_dispatch_constructor_inventory(
        provider_dispatch_constructor_inventory().as_slice(),
        expected_openai_responses_constructor_count(),
    )
    .expect("constructor inventory should validate");
    assert!(
        plan.request_projection_identity()
            .starts_with(BLAKE3_PREFIX)
    );
    assert!(plan.request_payload_identity().starts_with(BLAKE3_PREFIX));
    assert_eq!(plan.deadline_label(), DEADLINE_LABEL);
    assert_eq!(plan.cancellation_label(), CANCELLATION_LABEL);
}

#[test]
fn request_projection_fixture_mismatch_fails_closed() {
    // r[verify agent_harness_runtime.provider_dispatch_adapter.request_projection]
    let selected = dispatch_selected_model();
    let request = canonical_dispatch_request();
    let mut input = valid_dispatch_input(&selected, &request);
    input.reviewed_projection_fixture = BAD_REVIEWED_PROJECTION_FIXTURE;
    let error = plan_provider_dispatch(&input).expect_err("literal mismatch should fail");

    assert!(matches!(
        error,
        ProviderDispatchError::RequestProjection { .. }
    ));
}

#[test]
fn fixture_response_normalizes_to_safe_receipt() {
    // r[verify agent_harness_runtime.provider_dispatch_adapter.receipts]
    // r[verify agent_harness_runtime.provider_dispatch_adapter.verification]
    let selected = dispatch_selected_model();
    let request = canonical_dispatch_request();
    let plan = plan_provider_dispatch(&valid_dispatch_input(&selected, &request))
        .expect("provider dispatch should plan");
    let outcome = normalize_openai_responses_fixture_response(&plan, success_response_fixture())
        .expect("fixture response should normalize");

    assert_eq!(
        outcome.response().items()[FIRST_PROVIDER_INDEX].assistant_text(),
        Some(RESPONSE_TEXT)
    );
    assert_eq!(
        outcome.receipt().status(),
        ProviderDispatchOutcomeStatus::Success
    );
    assert_eq!(outcome.receipt().provider_family(), "openai_responses");
    assert!(
        outcome
            .receipt()
            .request_identity()
            .starts_with(BLAKE3_PREFIX)
    );
    assert!(
        outcome
            .receipt()
            .response_identity()
            .starts_with(BLAKE3_PREFIX)
    );
    assert!(
        outcome
            .receipt()
            .redactions()
            .iter()
            .any(|marker| marker == "provider_payload:redacted")
    );
    assert!(!format!("{:?}", outcome.receipt()).contains(SECRET_VALUE));
}

#[test]
fn dispatch_fails_closed_before_provider_contact() {
    // r[verify agent_harness_runtime.provider_dispatch_adapter.fail_closed]
    // r[verify agent_harness_runtime.provider_dispatch_adapter.verification]
    let selected = dispatch_selected_model();
    let request = canonical_dispatch_request();

    let mut missing_credential = valid_dispatch_input(&selected, &request);
    missing_credential.credential = ProviderDispatchCredentialState::MissingRequired;
    assert_eq!(
        plan_provider_dispatch(&missing_credential).expect_err("missing credential should fail"),
        ProviderDispatchError::MissingCredential
    );

    let mut disabled_backend = valid_dispatch_input(&selected, &request);
    disabled_backend.service = ProviderDispatchServiceState::DisabledBackend;
    assert_eq!(
        plan_provider_dispatch(&disabled_backend).expect_err("disabled backend should fail"),
        ProviderDispatchError::DisabledBackend
    );

    let mut missing_service = valid_dispatch_input(&selected, &request);
    missing_service.service = ProviderDispatchServiceState::Missing;
    assert_eq!(
        plan_provider_dispatch(&missing_service).expect_err("missing service should fail"),
        ProviderDispatchError::MissingService
    );

    let mut unavailable_service = valid_dispatch_input(&selected, &request);
    unavailable_service.service = ProviderDispatchServiceState::Unavailable;
    assert_eq!(
        plan_provider_dispatch(&unavailable_service).expect_err("unavailable service should fail"),
        ProviderDispatchError::ServiceUnavailable
    );

    let mut denied_entitlement = valid_dispatch_input(&selected, &request);
    denied_entitlement.entitlement = ProviderDispatchEntitlementState::Denied;
    assert_eq!(
        plan_provider_dispatch(&denied_entitlement).expect_err("denied entitlement should fail"),
        ProviderDispatchError::EntitlementDenied
    );

    let mut denied_fallback = valid_dispatch_input(&selected, &request);
    denied_fallback.fallback = ProviderDispatchFallbackState::RequestedButDenied;
    assert_eq!(
        plan_provider_dispatch(&denied_fallback).expect_err("known-prefix fallback should fail"),
        ProviderDispatchError::FallbackDenied
    );

    let mut timed_out = valid_dispatch_input(&selected, &request);
    timed_out.deadline = ProviderDispatchDeadlineState::TimedOut;
    assert_eq!(
        plan_provider_dispatch(&timed_out).expect_err("timeout should fail"),
        ProviderDispatchError::Timeout
    );

    let mut cancelled = valid_dispatch_input(&selected, &request);
    cancelled.cancellation = ProviderDispatchCancellationState::Cancelled;
    assert_eq!(
        plan_provider_dispatch(&cancelled).expect_err("cancelled dispatch should fail"),
        ProviderDispatchError::Cancelled
    );
}

#[test]
fn unsupported_feature_and_family_fail_closed() {
    // r[verify agent_harness_runtime.provider_dispatch_adapter.fail_closed]
    let text_only = text_only_selected_model();
    let request = canonical_dispatch_request();
    let image_error = plan_provider_dispatch(&valid_dispatch_input(&text_only, &request))
        .expect_err("image request should fail against text-only model");
    assert_eq!(
        image_error,
        ProviderDispatchError::UnsupportedFeature {
            feature: "image_content",
        }
    );

    let mock = mock_selected_model();
    let family_error = plan_provider_dispatch(&valid_dispatch_input(&mock, &request))
        .expect_err("mock family should not use OpenAI adapter");
    assert_eq!(
        family_error,
        ProviderDispatchError::UnsupportedProviderFamily {
            api_kind: ApiKind::Mock,
        }
    );
}

#[test]
fn malformed_responses_and_receipt_leaks_fail_closed() {
    // r[verify agent_harness_runtime.provider_dispatch_adapter.receipts]
    // r[verify agent_harness_runtime.provider_dispatch_adapter.verification]
    let selected = dispatch_selected_model();
    let request = canonical_dispatch_request();
    let plan = plan_provider_dispatch(&valid_dispatch_input(&selected, &request))
        .expect("provider dispatch should plan");

    let missing_finish = normalize_openai_responses_fixture_response(
        &plan,
        animus_adapters::ProviderResponseFixtureDraft {
            finish_reason: None,
            ..success_response_fixture()
        },
    )
    .expect_err("missing finish should fail");
    assert!(matches!(
        missing_finish,
        ProviderDispatchError::MalformedResponse { .. }
    ));

    let raw_payload = normalize_openai_responses_fixture_response(
        &plan,
        animus_adapters::ProviderResponseFixtureDraft {
            text: None,
            raw_provider_payload: Some(RAW_PAYLOAD),
            ..success_response_fixture()
        },
    )
    .expect_err("raw payload should fail");
    assert!(matches!(
        raw_payload,
        ProviderDispatchError::MalformedResponse { .. }
    ));

    let outcome = normalize_openai_responses_fixture_response(&plan, success_response_fixture())
        .expect("fixture response should normalize");
    let mut forged = outcome.receipt().clone();
    forged.redactions.push(String::from(RAW_PAYLOAD));
    assert_eq!(
        validate_provider_dispatch_receipt(&forged).expect_err("forged redaction should fail"),
        ProviderDispatchError::ReceiptLeak {
            reason: "receipt redaction marker is not whitelisted",
        }
    );
}

#[test]
fn constructor_inventory_detects_drift() {
    // r[verify agent_harness_runtime.provider_dispatch_adapter.parity_rails]
    // r[verify agent_harness_runtime.provider_dispatch_adapter.verification]
    let count_error = validate_provider_dispatch_constructor_inventory(
        provider_dispatch_constructor_inventory().as_slice(),
        expected_openai_responses_constructor_count().saturating_add(SINGLE_PROVIDER_COUNT),
    )
    .expect_err("constructor count drift should fail");
    assert!(matches!(
        count_error,
        ProviderDispatchError::ConstructorInventory { .. }
    ));

    let drifted = [ProviderDispatchConstructorInventoryEntry {
        constructor: String::from("drifted_openai_constructor"),
        shared_fields: ProviderDispatchSharedFieldMatrix {
            stream_intent: false,
            ..ProviderDispatchSharedFieldMatrix::all_preserved()
        },
    }];
    let field_error = validate_provider_dispatch_constructor_inventory(
        &drifted,
        expected_openai_responses_constructor_count(),
    )
    .expect_err("missing stream intent should fail");
    assert!(matches!(
        field_error,
        ProviderDispatchError::ConstructorInventory { .. }
    ));
}

fn valid_dispatch_input<'a>(
    selected_model: &'a SelectedModel,
    canonical_request: &'a animus_adapters::CanonicalModelRequest,
) -> ProviderDispatchInput<'a> {
    ProviderDispatchInput {
        selected_model,
        canonical_request,
        reviewed_projection_fixture: REVIEWED_PROJECTION_FIXTURE,
        credential: ready_credential(),
        service: ProviderDispatchServiceState::Available,
        entitlement: ProviderDispatchEntitlementState::Granted,
        deadline: ProviderDispatchDeadlineState::Ready {
            label: String::from(DEADLINE_LABEL),
        },
        cancellation: ProviderDispatchCancellationState::Ready {
            label: String::from(CANCELLATION_LABEL),
        },
        fallback: ProviderDispatchFallbackState::NotRequested,
        receipt_policy: ProviderDispatchReceiptPolicy::SafeSummaryOnly,
    }
}

fn ready_credential() -> ProviderDispatchCredentialState {
    let requirement = CredentialRequirement::new(
        CredentialSource::environment(API_KEY_ENV).expect("credential source should validate"),
        true,
    );
    let handle = resolve_credential_from_env_pairs(&requirement, [(API_KEY_ENV, SECRET_VALUE)])
        .expect("credential lookup should not fail")
        .expect("required credential should be present");
    ProviderDispatchCredentialState::from_handle(&handle)
}

fn dispatch_selected_model() -> SelectedModel {
    dispatch_registry(ApiKind::OpenAiResponses, dispatch_capabilities())
        .select_model(&selection(ThinkingLevel::High, ThinkingPolicy::Reject))
        .expect("dispatch model should select")
}

fn text_only_selected_model() -> SelectedModel {
    dispatch_registry(ApiKind::OpenAiResponses, text_only_dispatch_capabilities())
        .select_model(&selection(ThinkingLevel::High, ThinkingPolicy::Reject))
        .expect("text-only dispatch model should select")
}

fn mock_selected_model() -> SelectedModel {
    dispatch_registry(ApiKind::Mock, dispatch_capabilities())
        .select_model(&selection(ThinkingLevel::High, ThinkingPolicy::Reject))
        .expect("mock model should select")
}

fn dispatch_registry(api_kind: ApiKind, capabilities: ModelCapabilities) -> ProviderRegistry {
    let entry = ModelEntry::new(model_id(), api_kind, endpoint(), capabilities);
    let credential = CredentialRequirement::new(
        CredentialSource::environment(API_KEY_ENV).expect("credential source should validate"),
        true,
    );
    let provider = ProviderEntry::new(provider_id(), credential, vec![entry])
        .expect("provider entry should validate");
    ProviderRegistry::new(vec![provider]).expect("registry should validate")
}

fn dispatch_capabilities() -> ModelCapabilities {
    ModelCapabilities::from_draft(ModelCapabilitiesDraft {
        input_modalities: vec![InputModality::Text, InputModality::Image],
        context_budget: CONTEXT_BUDGET,
        output_budget: OUTPUT_BUDGET,
        reasoning: ReasoningSupport::Optional,
        thinking_levels: vec![
            ThinkingLevel::None,
            ThinkingLevel::Low,
            ThinkingLevel::Medium,
            ThinkingLevel::High,
        ],
        streaming: ModelStreamingCapabilities::new(
            StreamSupport::Supported,
            StreamSupport::Supported,
            StreamSupport::Supported,
            StreamSupport::Supported,
        ),
        cost: Some(TokenCost::new(INPUT_COST_MICRO_USD, OUTPUT_COST_MICRO_USD)),
    })
    .expect("dispatch capabilities should validate")
}

fn text_only_dispatch_capabilities() -> ModelCapabilities {
    ModelCapabilities::from_draft(ModelCapabilitiesDraft {
        input_modalities: vec![InputModality::Text],
        context_budget: CONTEXT_BUDGET,
        output_budget: OUTPUT_BUDGET,
        reasoning: ReasoningSupport::Optional,
        thinking_levels: vec![
            ThinkingLevel::None,
            ThinkingLevel::Low,
            ThinkingLevel::Medium,
            ThinkingLevel::High,
        ],
        streaming: ModelStreamingCapabilities::new(
            StreamSupport::Supported,
            StreamSupport::Supported,
            StreamSupport::Supported,
            StreamSupport::Supported,
        ),
        cost: None,
    })
    .expect("text-only capabilities should validate")
}

fn canonical_dispatch_request() -> animus_adapters::CanonicalModelRequest {
    let developer_content = [animus_adapters::CanonicalMessageContentPartDraft::Text {
        text: DISPATCH_DEVELOPER_TEXT,
    }];
    let user_content = [
        animus_adapters::CanonicalMessageContentPartDraft::Text {
            text: DISPATCH_USER_TEXT,
        },
        animus_adapters::CanonicalMessageContentPartDraft::ImageDescriptor {
            descriptor_id: IMAGE_DESCRIPTOR,
            media_type: IMAGE_MEDIA_TYPE,
            content_hash: IMAGE_HASH,
        },
    ];
    let messages = [
        animus_adapters::CanonicalModelMessageDraft {
            role: "developer",
            content: &developer_content,
            tool_call_id: None,
        },
        animus_adapters::CanonicalModelMessageDraft {
            role: "user",
            content: &user_content,
            tool_call_id: None,
        },
    ];
    let tools = [animus_adapters::CanonicalModelToolDeclarationDraft {
        name: TOOL_NAME,
        description: TOOL_DESCRIPTION,
        input_schema_label: TOOL_SCHEMA,
    }];
    let provenance = [animus_adapters::CanonicalModelSafeMetadataDraft {
        key: TRACE_LABEL,
        value: TRACE_VALUE,
    }];
    animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: &tools,
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(OUTPUT_LIMIT),
                temperature_millis: Some(TEMPERATURE_MILLIS),
                stop_sequences: &[],
                stream: true,
            },
            response_format: animus_adapters::CanonicalModelResponseFormatIntentDraft::JsonSchema {
                schema_label: RESPONSE_SCHEMA,
            },
            reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                enabled: true,
                effort: animus_adapters::CanonicalModelReasoningEffort::High,
                summary_label: Some("summary-label"),
            },
            provenance: &provenance,
        },
    )
    .expect("canonical dispatch request should validate")
}

fn success_response_fixture() -> animus_adapters::ProviderResponseFixtureDraft<'static> {
    animus_adapters::ProviderResponseFixtureDraft {
        family: animus_adapters::ProviderProjectionFamily::OpenAiResponses.as_str(),
        text: Some(RESPONSE_TEXT),
        structured_payload: None,
        tool_call_id: None,
        tool_name: None,
        tool_arguments: None,
        refusal: None,
        safety: None,
        malformed_output: None,
        usage: Some(animus_adapters::CanonicalModelUsageDraft {
            input: RESPONSE_INPUT_TOKENS,
            output: RESPONSE_OUTPUT_TOKENS,
            cached_input: Some(RESPONSE_CACHED_INPUT_TOKENS),
        }),
        finish_reason: Some(animus_adapters::CanonicalModelFinishReason::Stop.as_str()),
        raw_provider_payload: None,
        hidden_reasoning: None,
    }
}
