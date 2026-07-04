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
/// OpenAI-family API style.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderApiStyle {
    /// `OpenAI` Responses API.
    Responses,
    /// `OpenAI` Chat Completions API.
    ChatCompletions,
    /// Ambiguous/unknown style.
    Unknown,
}

/// API style resolution policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderApiStylePolicy {
    /// Require explicit style.
    RequireExplicit,
    /// Default custom gateways to Chat Completions.
    CompatibleGatewayChatDefault,
    /// Default custom gateways to Responses.
    CompatibleGatewayResponsesDefault,
}

/// Shell-resolved secret source reference for a header value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSecretSourceReference {
    /// Redacted source label.
    pub label: String,
    /// Whether the shell found the source.
    pub available: bool,
}

/// Header value source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderHeaderValueSource {
    /// Literal safe value.
    Literal(String),
    /// Shell-resolved secret source label.
    SecretSource(ProviderSecretSourceReference),
}

/// Custom header descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderHeaderDescriptor {
    /// Header name.
    pub name: String,
    /// Value source.
    pub value_source: ProviderHeaderValueSource,
}

/// Header resolution report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderHeaderResolution {
    /// Header name.
    pub name: String,
    /// Redacted source label.
    pub redacted_source: String,
}

/// Extra body descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderExtraBodyDescriptor {
    /// Descriptor ID.
    pub descriptor_id: String,
    /// Provider scope.
    pub provider_id: String,
    /// Optional model scope.
    pub model_id: Option<String>,
    /// Field key.
    pub key: String,
    /// Redacted value class.
    pub value_class: String,
}

/// Normalized model class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderModelClass {
    /// Text agent model.
    TextAgent,
    /// Embedding model.
    Embedding,
    /// Image model.
    Image,
    /// Audio model.
    Audio,
    /// Moderation model.
    Moderation,
    /// Rerank model.
    Rerank,
    /// Other non-agent class.
    OtherNonAgent,
}

/// Normalized model metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderModelMetadata {
    /// Provider ID.
    pub provider_id: String,
    /// Model ID.
    pub model_id: String,
    /// Display name.
    pub display_name: String,
    /// Model class.
    pub model_class: ProviderModelClass,
    /// Context budget label.
    pub context_budget_label: String,
    /// Reasoning support.
    pub reasoning: bool,
}

/// Endpoint sensitivity.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderEndpointSensitivity {
    /// Safe label can be shown.
    Public,
    /// Endpoint must be hashed/redacted.
    Sensitive,
}

/// Provider selection receipt input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSelectionReceiptInput {
    /// Provider ID.
    pub provider_id: String,
    /// Model metadata.
    pub model: ProviderModelMetadata,
    /// Endpoint label or URL supplied by the shell.
    pub endpoint: String,
    /// Endpoint sensitivity.
    pub sensitivity: ProviderEndpointSensitivity,
    /// API style.
    pub api_style: ProviderApiStyle,
    /// Extra body descriptor IDs.
    pub extra_body_descriptor_ids: Vec<String>,
    /// Diagnostics.
    pub diagnostics: Vec<ProviderDiagnostic>,
}

/// Provider selection receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderSelectionReceipt {
    /// Provider ID.
    pub provider_id: String,
    /// Model ID.
    pub model_id: String,
    /// Endpoint identity or hash.
    pub endpoint_identity: String,
    /// API style.
    pub api_style: ProviderApiStyle,
    /// Capability class.
    pub capability_class: ProviderModelClass,
    /// Credential source class.
    pub credential_source_class: String,
    /// Extra body descriptor IDs.
    pub extra_body_descriptor_ids: Vec<String>,
    /// Diagnostics.
    pub diagnostics: Vec<ProviderDiagnostic>,
}

/// Provider diagnostic class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderDiagnosticKind {
    /// Ambiguous API style.
    AmbiguousApiStyle,
    /// Invalid header.
    InvalidHeader,
    /// Missing header secret.
    MissingHeaderSecret,
    /// Extra body out of scope.
    ExtraBodyOutOfScope,
    /// Unsupported model class.
    UnsupportedModelClass,
}

/// Provider diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDiagnostic {
    /// Kind.
    pub kind: ProviderDiagnosticKind,
    /// Field.
    pub field: String,
    /// Detail.
    pub detail: String,
}

impl ProviderDiagnostic {
    /// Builds a diagnostic.
    #[must_use]
    pub fn new(
        kind: ProviderDiagnosticKind,
        field: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            field: field.into(),
            detail: detail.into(),
        }
    }
}

/// Provider validation result.
pub type ProviderValidationResult<T> = Result<T, Vec<ProviderDiagnostic>>;
