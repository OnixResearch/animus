// Cairn adaptation compatibility modules intentionally keep public DTO names and table-driven validation compact while drained changes are archived.
#![allow(
    unknown_lints,
    ambiguous_params,
    assertion_density,
    bool_naming,
    borrowed_argument_types
)]
#![allow(
    compound_condition,
    excessive_file_length,
    function_length,
    non_trait_imports
)]
#![allow(
    numeric_units,
    path_segment_repetition,
    raw_arithmetic_overflow,
    unbounded_collection_growth
)]

use super::*;

const ENDPOINT: &str = "https://gateway.internal/v1";
const MODEL_ID: &str = "agent-model";
const PROVIDER_ID: &str = "openai-compatible";
const CAPABILITY_SCHEMA: &str = "animus.model-capability-matrix.v1";
const USER_PROMPT: &str = "Plan safely.";
const SYSTEM_PROMPT: &str = "Stay bounded.";
const IMAGE_DESCRIPTOR_ID: &str = "image-one";
const IMAGE_MEDIA_TYPE: &str = "image/png";
const IMAGE_HASH: &str = "b3-image";
const TOOL_NAME: &str = "lookup";
const TOOL_DESCRIPTION: &str = "Bounded lookup.";
const TOOL_SCHEMA: &str = "schema.lookup";
const RESPONSE_SCHEMA: &str = "schema.answer";
const CONTEXT_BUDGET: u32 = 256;
const OUTPUT_BUDGET: u32 = 64;
const SMALL_CONTEXT_BUDGET: u32 = 1;
const SMALL_OUTPUT_BUDGET: u32 = 1;

#[test]
fn api_style_resolution_handles_first_party_gateway_and_ambiguous_cases() {
    // r[agent_harness_runtime.zerostack_provider_hardening.api_style.gateway]
    assert_eq!(
        resolve_provider_api_style(
            Some(ProviderApiStyle::Responses),
            false,
            ProviderApiStylePolicy::RequireExplicit
        )
        .expect("explicit"),
        ProviderApiStyle::Responses
    );
    assert_eq!(
        resolve_provider_api_style(
            None,
            true,
            ProviderApiStylePolicy::CompatibleGatewayChatDefault
        )
        .expect("gateway default"),
        ProviderApiStyle::ChatCompletions
    );
    let diagnostics =
        resolve_provider_api_style(None, true, ProviderApiStylePolicy::RequireExplicit)
            .expect_err("ambiguous");
    assert!(
        diagnostics
            .iter()
            .any(|diagnostic| { diagnostic.kind == ProviderDiagnosticKind::AmbiguousApiStyle })
    );
}

#[test]
fn header_resolution_redacts_missing_environment_sources() {
    // r[agent_harness_runtime.zerostack_provider_hardening.headers.missing]
    let header = ProviderHeaderDescriptor {
        name: "x-api-key".to_string(),
        value_source: ProviderHeaderValueSource::SecretSource(ProviderSecretSourceReference {
            label: "GATEWAY_KEY".to_string(),
            available: false,
        }),
    };
    let diagnostics = validate_provider_header(&header).expect_err("missing env");
    assert!(diagnostics.iter().any(|diagnostic| {
        diagnostic.kind == ProviderDiagnosticKind::MissingHeaderSecret
            && !diagnostic.detail.contains("secret-value")
    }));

    let invalid = ProviderHeaderDescriptor {
        name: "Authorization".to_string(),
        value_source: ProviderHeaderValueSource::Literal("redacted".to_string()),
    };
    assert!(validate_provider_header(&invalid).is_err());
}

#[test]
fn extra_body_descriptor_cannot_cross_provider_or_model_scope() {
    // r[agent_harness_runtime.zerostack_provider_hardening.extra_body.scope]
    let descriptor = ProviderExtraBodyDescriptor {
        descriptor_id: "extra.reasoning".to_string(),
        provider_id: PROVIDER_ID.to_string(),
        model_id: Some(MODEL_ID.to_string()),
        key: "reasoning_effort".to_string(),
        value_class: "low".to_string(),
    };
    assert!(resolve_extra_body_descriptor(&descriptor, PROVIDER_ID, MODEL_ID).is_ok());
    assert!(resolve_extra_body_descriptor(&descriptor, "other", MODEL_ID).is_err());
    assert!(resolve_extra_body_descriptor(&descriptor, PROVIDER_ID, "other-model").is_err());
}

#[test]
fn model_filter_rejects_non_agent_model_classes() {
    // r[agent_harness_runtime.zerostack_provider_hardening.model_filter.reject]
    assert!(filter_agent_model(&model(ProviderModelClass::TextAgent), false).is_ok());
    for class in [
        ProviderModelClass::Embedding,
        ProviderModelClass::Image,
        ProviderModelClass::Audio,
        ProviderModelClass::Moderation,
        ProviderModelClass::Rerank,
        ProviderModelClass::OtherNonAgent,
    ] {
        let diagnostics = filter_agent_model(&model(class), false).expect_err("non agent");
        assert!(diagnostics.iter().any(|diagnostic| {
            diagnostic.kind == ProviderDiagnosticKind::UnsupportedModelClass
        }));
    }
}

#[test]
fn capability_negotiation_admits_supported_requests_and_optional_downgrades() {
    // r[agent_harness_runtime.model_capability_matrix.negotiation.positive]
    let supported_request = structured_request(true, true, true);
    let supported = full_capabilities();
    let admitted = negotiate_model_capability(ModelCapabilityNegotiationInput {
        request: &supported_request,
        capabilities: &supported,
        downgrade_policy: deny_downgrades(),
        required_privacy: ProviderPrivacyPosture::PrivateGateway,
        provider_fallback_requested: false,
    });
    assert_eq!(admitted.status, ModelCapabilityDecisionStatus::Admitted);
    assert!(admitted.issues.is_empty());

    let optional_request = structured_request(false, true, true);
    let text_only = ProviderModelCapabilityMatrix {
        content_parts: vec![ProviderModelContentPartClass::Text],
        structured_outputs: Vec::new(),
        stream_channels: Vec::new(),
        reasoning_controls: Vec::new(),
        ..full_capabilities()
    };
    let downgraded = negotiate_model_capability(ModelCapabilityNegotiationInput {
        request: &optional_request,
        capabilities: &text_only,
        downgrade_policy: allow_optional_downgrades(),
        required_privacy: ProviderPrivacyPosture::RemoteProvider,
        provider_fallback_requested: false,
    });
    assert_eq!(downgraded.status, ModelCapabilityDecisionStatus::Downgraded);
    assert!(downgraded.issues.is_empty());
    assert!(downgraded.downgrades.iter().any(|item| {
        item.feature == "content_parts" && item.policy == "host_optional_downgrade"
    }));
}

#[test]
fn capability_negotiation_denies_unsupported_required_features() {
    // r[agent_harness_runtime.model_capability_matrix.fail_closed]
    let request = structured_request(true, true, true);
    let role_limited = ProviderModelCapabilityMatrix {
        roles: vec![animus_adapters::CanonicalModelRole::User],
        ..full_capabilities()
    };
    assert_issue(
        &request,
        &role_limited,
        ModelCapabilityIssueKind::UnsupportedRole,
    );

    let text_only = ProviderModelCapabilityMatrix {
        content_parts: vec![ProviderModelContentPartClass::Text],
        tool_calls: false,
        structured_outputs: Vec::new(),
        stream_channels: Vec::new(),
        reasoning_controls: Vec::new(),
        ..full_capabilities()
    };
    let denied = negotiate_model_capability(ModelCapabilityNegotiationInput {
        request: &request,
        capabilities: &text_only,
        downgrade_policy: deny_downgrades(),
        required_privacy: ProviderPrivacyPosture::RemoteProvider,
        provider_fallback_requested: false,
    });
    assert_eq!(denied.status, ModelCapabilityDecisionStatus::Denied);
    for kind in [
        ModelCapabilityIssueKind::UnsupportedContentPart,
        ModelCapabilityIssueKind::UnsupportedToolCalls,
        ModelCapabilityIssueKind::UnsupportedStructuredOutput,
        ModelCapabilityIssueKind::UnsupportedStreaming,
        ModelCapabilityIssueKind::UnsupportedReasoning,
    ] {
        assert!(denied.issues.iter().any(|issue| issue.kind == kind));
    }
}

#[test]
fn capability_negotiation_denies_bad_metadata_budgets_privacy_and_fallback() {
    // r[agent_harness_runtime.model_capability_matrix.fail_closed]
    let request = structured_request(false, false, false);
    let bad_metadata = ProviderModelCapabilityMatrix {
        schema_identity: "stale".to_string(),
        context_budget: SMALL_CONTEXT_BUDGET,
        output_budget: SMALL_OUTPUT_BUDGET,
        privacy: ProviderPrivacyPosture::RemoteProvider,
        ..full_capabilities()
    };
    let denied = negotiate_model_capability(ModelCapabilityNegotiationInput {
        request: &request,
        capabilities: &bad_metadata,
        downgrade_policy: deny_downgrades(),
        required_privacy: ProviderPrivacyPosture::LocalOnly,
        provider_fallback_requested: true,
    });
    assert_eq!(denied.status, ModelCapabilityDecisionStatus::Denied);
    for kind in [
        ModelCapabilityIssueKind::UnknownCapabilityMetadata,
        ModelCapabilityIssueKind::BudgetOverflow,
        ModelCapabilityIssueKind::UnsupportedPrivacy,
        ModelCapabilityIssueKind::ProviderFallbackDenied,
    ] {
        assert!(denied.issues.iter().any(|issue| issue.kind == kind));
    }
    assert!(validate_model_capability_matrix(&bad_metadata).is_err());
}

#[test]
fn receipts_hash_sensitive_endpoints_and_keep_reviewable_metadata() {
    // r[agent_harness_runtime.zerostack_provider_hardening.receipts.endpoint]
    let receipt = provider_selection_receipt(&ProviderSelectionReceiptInput {
        provider_id: PROVIDER_ID.to_string(),
        model: model(ProviderModelClass::TextAgent),
        endpoint: ENDPOINT.to_string(),
        sensitivity: ProviderEndpointSensitivity::Sensitive,
        api_style: ProviderApiStyle::Responses,
        extra_body_descriptor_ids: vec!["extra.reasoning".to_string()],
        diagnostics: Vec::new(),
    });
    assert!(receipt.endpoint_identity.starts_with(BLAKE3_ALGORITHM));
    assert_eq!(receipt.api_style, ProviderApiStyle::Responses);
    assert_eq!(receipt.capability_class, ProviderModelClass::TextAgent);
    assert_eq!(receipt.credential_source_class, "redacted");
}

fn model(model_class: ProviderModelClass) -> ProviderModelMetadata {
    ProviderModelMetadata {
        provider_id: PROVIDER_ID.to_string(),
        model_id: MODEL_ID.to_string(),
        display_name: "Agent Model".to_string(),
        model_class,
        context_budget_label: "large".to_string(),
        reasoning: true,
    }
}

fn structured_request(
    include_tools: bool,
    include_image: bool,
    include_controls: bool,
) -> animus_adapters::CanonicalModelRequest {
    let system_content = [animus_adapters::CanonicalMessageContentPartDraft::Text {
        text: SYSTEM_PROMPT,
    }];
    let text_part = animus_adapters::CanonicalMessageContentPartDraft::Text { text: USER_PROMPT };
    let image_part = animus_adapters::CanonicalMessageContentPartDraft::ImageDescriptor {
        descriptor_id: IMAGE_DESCRIPTOR_ID,
        media_type: IMAGE_MEDIA_TYPE,
        content_hash: IMAGE_HASH,
    };
    let user_content_with_image = [text_part, image_part];
    let user_content_text_only = [text_part];
    let user_content = if include_image {
        user_content_with_image.as_slice()
    } else {
        user_content_text_only.as_slice()
    };
    let messages = [
        animus_adapters::CanonicalModelMessageDraft {
            role: animus_adapters::CanonicalModelRole::System.as_str(),
            content: &system_content,
            tool_call_id: None,
        },
        animus_adapters::CanonicalModelMessageDraft {
            role: animus_adapters::CanonicalModelRole::User.as_str(),
            content: user_content,
            tool_call_id: None,
        },
    ];
    let tools = [animus_adapters::CanonicalModelToolDeclarationDraft {
        name: TOOL_NAME,
        description: TOOL_DESCRIPTION,
        input_schema_label: TOOL_SCHEMA,
    }];
    let selected_tools = if include_tools { tools.as_slice() } else { &[] };
    animus_adapters::CanonicalModelRequest::from_draft(
        animus_adapters::CanonicalModelRequestDraft {
            contract_identity: animus_adapters::CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: selected_tools,
            output: animus_adapters::CanonicalModelOutputControlsDraft {
                max_output_tokens: Some(OUTPUT_BUDGET),
                temperature_millis: None,
                stop_sequences: &[],
                stream: include_controls,
            },
            response_format: if include_controls {
                animus_adapters::CanonicalModelResponseFormatIntentDraft::JsonSchema {
                    schema_label: RESPONSE_SCHEMA,
                }
            } else {
                animus_adapters::CanonicalModelResponseFormatIntentDraft::Text
            },
            reasoning: animus_adapters::CanonicalModelReasoningRequestDraft {
                enabled: include_controls,
                effort: if include_controls {
                    animus_adapters::CanonicalModelReasoningEffort::Medium
                } else {
                    animus_adapters::CanonicalModelReasoningEffort::None
                },
                summary_label: None,
            },
            provenance: &[],
        },
    )
    .expect("canonical request fixture should validate")
}

fn full_capabilities() -> ProviderModelCapabilityMatrix {
    ProviderModelCapabilityMatrix {
        schema_identity: CAPABILITY_SCHEMA.to_string(),
        provider_id: PROVIDER_ID.to_string(),
        model_id: MODEL_ID.to_string(),
        roles: vec![
            animus_adapters::CanonicalModelRole::System,
            animus_adapters::CanonicalModelRole::User,
            animus_adapters::CanonicalModelRole::Assistant,
            animus_adapters::CanonicalModelRole::Tool,
        ],
        content_parts: vec![
            ProviderModelContentPartClass::Text,
            ProviderModelContentPartClass::Image,
        ],
        tool_calls: true,
        structured_outputs: vec![
            ProviderStructuredOutputMode::JsonObject,
            ProviderStructuredOutputMode::JsonSchema,
        ],
        stream_channels: vec![
            ProviderStreamChannel::Content,
            ProviderStreamChannel::ToolCall,
            ProviderStreamChannel::Usage,
        ],
        reasoning_controls: vec![
            animus_adapters::CanonicalModelReasoningEffort::Low,
            animus_adapters::CanonicalModelReasoningEffort::Medium,
            animus_adapters::CanonicalModelReasoningEffort::High,
        ],
        context_budget: CONTEXT_BUDGET,
        output_budget: OUTPUT_BUDGET,
        privacy: ProviderPrivacyPosture::PrivateGateway,
    }
}

const fn deny_downgrades() -> ModelCapabilityDowngradePolicy {
    ModelCapabilityDowngradePolicy {
        allow_image_omission: false,
        allow_structured_output_omission: false,
        allow_streaming_omission: false,
        allow_reasoning_omission: false,
    }
}

const fn allow_optional_downgrades() -> ModelCapabilityDowngradePolicy {
    ModelCapabilityDowngradePolicy {
        allow_image_omission: true,
        allow_structured_output_omission: true,
        allow_streaming_omission: true,
        allow_reasoning_omission: true,
    }
}

fn assert_issue(
    request: &animus_adapters::CanonicalModelRequest,
    capabilities: &ProviderModelCapabilityMatrix,
    expected: ModelCapabilityIssueKind,
) {
    let denied = negotiate_model_capability(ModelCapabilityNegotiationInput {
        request,
        capabilities,
        downgrade_policy: deny_downgrades(),
        required_privacy: ProviderPrivacyPosture::RemoteProvider,
        provider_fallback_requested: false,
    });
    assert_eq!(denied.status, ModelCapabilityDecisionStatus::Denied);
    assert!(denied.issues.iter().any(|issue| issue.kind == expected));
}
