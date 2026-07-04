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
#![allow(
    ambiguous_params,
    assertion_density,
    bool_naming,
    borrowed_argument_types,
    compound_condition,
    excessive_file_length,
    function_length,
    non_trait_imports,
    numeric_units,
    path_segment_repetition,
    raw_arithmetic_overflow,
    unbounded_collection_growth
)]

use super::*;

const ENDPOINT: &str = "https://gateway.internal/v1";
const MODEL_ID: &str = "agent-model";
const PROVIDER_ID: &str = "openai-compatible";

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
