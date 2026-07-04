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
use super::{
    BLAKE3_ALGORITHM, FIELD_API_STYLE, FIELD_EXTRA_BODY, FIELD_HEADER, FIELD_MODEL,
    HASH_PART_SEPARATOR, PROVIDER_HARDENING_HASH_DOMAIN,
    data::{
        ProviderApiStyle, ProviderApiStylePolicy, ProviderDiagnostic, ProviderDiagnosticKind,
        ProviderEndpointSensitivity, ProviderExtraBodyDescriptor, ProviderHeaderDescriptor,
        ProviderHeaderResolution, ProviderHeaderValueSource, ProviderModelClass,
        ProviderModelMetadata, ProviderSelectionReceipt, ProviderSelectionReceiptInput,
        ProviderValidationResult,
    },
};

const HEADER_PREFIX: &str = "x-";
const REDACTED_ENV_PREFIX: &str = "env:";

/// Computes a provider-hardening BLAKE3 fingerprint.
#[must_use]
pub fn blake3_provider_hardening_fingerprint(label: &str, bytes: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(PROVIDER_HARDENING_HASH_DOMAIN.as_bytes());
    hasher.update(HASH_PART_SEPARATOR.as_bytes());
    hasher.update(label.as_bytes());
    hasher.update(HASH_PART_SEPARATOR.as_bytes());
    hasher.update(bytes);
    format!("{BLAKE3_ALGORITHM}:{}", hasher.finalize().to_hex())
}

/// Resolves OpenAI-family API style before client construction.
///
/// # Errors
///
/// Returns an ambiguous-style diagnostic when policy requires explicit style and
/// no style is supplied.
pub fn resolve_provider_api_style(
    explicit: Option<ProviderApiStyle>,
    custom_gateway: bool,
    policy: ProviderApiStylePolicy,
) -> ProviderValidationResult<ProviderApiStyle> {
    if let Some(style) = explicit.filter(|style| *style != ProviderApiStyle::Unknown) {
        return Ok(style);
    }
    match (custom_gateway, policy) {
        (true, ProviderApiStylePolicy::CompatibleGatewayChatDefault) => {
            Ok(ProviderApiStyle::ChatCompletions)
        }
        (true, ProviderApiStylePolicy::CompatibleGatewayResponsesDefault) => {
            Ok(ProviderApiStyle::Responses)
        }
        _ => Err(vec![ProviderDiagnostic::new(
            ProviderDiagnosticKind::AmbiguousApiStyle,
            FIELD_API_STYLE,
            "provider API style is ambiguous before client construction",
        )]),
    }
}

/// Validates a custom header descriptor with redacted environment diagnostics.
///
/// # Errors
///
/// Returns diagnostics for invalid header names or unavailable environment-backed
/// sources without exposing resolved values.
pub fn validate_provider_header(
    header: &ProviderHeaderDescriptor,
) -> ProviderValidationResult<ProviderHeaderResolution> {
    if !valid_header_name(&header.name) {
        return Err(vec![ProviderDiagnostic::new(
            ProviderDiagnosticKind::InvalidHeader,
            FIELD_HEADER,
            "provider header name is invalid",
        )]);
    }
    match &header.value_source {
        ProviderHeaderValueSource::Literal(value) if value.trim().is_empty() => {
            Err(vec![ProviderDiagnostic::new(
                ProviderDiagnosticKind::InvalidHeader,
                FIELD_HEADER,
                "provider header literal is empty",
            )])
        }
        ProviderHeaderValueSource::Literal(_) => Ok(ProviderHeaderResolution {
            name: header.name.clone(),
            redacted_source: "literal:redacted".to_string(),
        }),
        ProviderHeaderValueSource::SecretSource(reference) => {
            if reference.available {
                Ok(ProviderHeaderResolution {
                    name: header.name.clone(),
                    redacted_source: format!("{REDACTED_ENV_PREFIX}{}", reference.label),
                })
            } else {
                Err(vec![ProviderDiagnostic::new(
                    ProviderDiagnosticKind::MissingHeaderSecret,
                    FIELD_HEADER,
                    format!(
                        "missing provider header source {REDACTED_ENV_PREFIX}{}",
                        reference.label
                    ),
                )])
            }
        }
    }
}

/// Resolves provider/model-scoped extra body descriptors.
///
/// # Errors
///
/// Returns an out-of-scope diagnostic when selected provider/model does not match.
pub fn resolve_extra_body_descriptor(
    descriptor: &ProviderExtraBodyDescriptor,
    selected_provider: &str,
    selected_model: &str,
) -> ProviderValidationResult<ProviderExtraBodyDescriptor> {
    let provider_matches = descriptor.provider_id == selected_provider;
    let model_matches = descriptor
        .model_id
        .as_ref()
        .is_none_or(|model| model == selected_model);
    if provider_matches && model_matches {
        Ok(descriptor.clone())
    } else {
        Err(vec![ProviderDiagnostic::new(
            ProviderDiagnosticKind::ExtraBodyOutOfScope,
            FIELD_EXTRA_BODY,
            "provider extra body descriptor is outside selected provider/model scope",
        )])
    }
}

/// Filters non-agent model classes unless host override is explicit.
///
/// # Errors
///
/// Returns unsupported-model-class diagnostics for embedding, image, audio,
/// moderation, rerank, or other non-agent classes.
pub fn filter_agent_model(
    model: &ProviderModelMetadata,
    allow_non_agent_override: bool,
) -> ProviderValidationResult<ProviderModelMetadata> {
    if model.model_class == ProviderModelClass::TextAgent || allow_non_agent_override {
        Ok(model.clone())
    } else {
        Err(vec![ProviderDiagnostic::new(
            ProviderDiagnosticKind::UnsupportedModelClass,
            FIELD_MODEL,
            "provider model class is not suitable for text agent turns",
        )])
    }
}

/// Builds a safe provider selection receipt.
#[must_use]
pub fn provider_selection_receipt(
    input: &ProviderSelectionReceiptInput,
) -> ProviderSelectionReceipt {
    let endpoint_identity = match input.sensitivity {
        ProviderEndpointSensitivity::Public => input.endpoint.clone(),
        ProviderEndpointSensitivity::Sensitive => {
            blake3_provider_hardening_fingerprint("endpoint", input.endpoint.as_bytes())
        }
    };
    ProviderSelectionReceipt {
        provider_id: input.provider_id.clone(),
        model_id: input.model.model_id.clone(),
        endpoint_identity,
        api_style: input.api_style,
        capability_class: input.model.model_class,
        credential_source_class: "redacted".to_string(),
        extra_body_descriptor_ids: input.extra_body_descriptor_ids.clone(),
        diagnostics: input.diagnostics.clone(),
    }
}

fn valid_header_name(name: &str) -> bool {
    !name.trim().is_empty()
        && name.starts_with(HEADER_PREFIX)
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
}
