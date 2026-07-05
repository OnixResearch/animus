const CONTRACT_IDENTITY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_contract_identity");
const ROLE_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("model_role");
const TOOL_CALL_ID_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_tool_call_id");
const CONTENT_PART_KIND_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_content_part_kind");
const CONTENT_TEXT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_content_text");
const IMAGE_DESCRIPTOR_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_image_descriptor_id");
const IMAGE_MEDIA_TYPE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_image_media_type");
const IMAGE_HASH_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_image_content_hash");
const TOOL_NAME_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_tool_name");
const TOOL_DESCRIPTION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_tool_description");
const TOOL_SCHEMA_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_tool_schema_label");
const STOP_SEQUENCE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_stop_sequence");
const RESPONSE_SCHEMA_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_response_schema_label");
const REASONING_SUMMARY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_reasoning_summary");
const METADATA_KEY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_metadata_key");
const METADATA_VALUE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_metadata_value");
const RESPONSE_TEXT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_response_text");
const STRUCTURED_PAYLOAD_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_structured_payload_label");
const TOOL_ARGUMENTS_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_tool_arguments_label");
const REFUSAL_REASON_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_refusal_reason");
const SAFETY_CATEGORY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_safety_category");
const MALFORMED_DIAGNOSTIC_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_malformed_diagnostic");
const FINISH_REASON_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_finish_reason");
const RAW_PROVIDER_PAYLOAD_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_raw_provider_payload");
const HIDDEN_REASONING_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("model_hidden_reasoning");

const MAX_MODEL_MESSAGES: usize = 128;
const MAX_CONTENT_PARTS_PER_MESSAGE: usize = 64;
const MAX_MODEL_TOOLS: usize = 32;
const MAX_STOP_SEQUENCES: usize = 16;
const MAX_SAFE_METADATA_ITEMS: usize = 32;
const MAX_RESPONSE_ITEMS: usize = 64;
const MAX_RESPONSE_DIAGNOSTICS: usize = 16;
const MIN_TOKEN_BUDGET: u32 = 1;
const MAX_TOKEN_BUDGET: u32 = 1_000_000;
const MAX_TEMPERATURE_MILLIS: u16 = 2_000;

const ROLE_SYSTEM: &str = "system";
const ROLE_DEVELOPER: &str = "developer";
const ROLE_USER: &str = "user";
const ROLE_ASSISTANT: &str = "assistant";
const ROLE_TOOL: &str = "tool";
const FINISH_STOP: &str = "stop";
const FINISH_LENGTH: &str = "length";
const FINISH_TOOL_CALL: &str = "tool_call";
const FINISH_REFUSAL: &str = "refusal";
const FINISH_SAFETY: &str = "safety";
const FINISH_MALFORMED: &str = "malformed";
const LEAK_MARKER_RAW_PROVIDER_PAYLOAD: &str = "raw_provider_payload";
const LEAK_MARKER_PROVIDER_PAYLOAD_PREFIX: &str = "provider_payload:";
const LEAK_MARKER_CREDENTIAL_PREFIX: &str = "credential:";
const LEAK_MARKER_SECRET_PREFIX: &str = "secret:";
const LEAK_MARKER_AUTHORIZATION_PREFIX: &str = "authorization:";
const LEAK_MARKER_BEARER_PREFIX: &str = "bearer ";
const LEAK_MARKER_HIDDEN_REASONING: &str = "hidden_reasoning";
const INVALID_ZERO_USAGE: &str = "usage token counts must not both be zero";
const INVALID_CACHED_USAGE: &str = "cached input tokens must not exceed input tokens";
const INVALID_REASONING_DISABLED_EFFORT: &str = "disabled reasoning must use none effort";
const INVALID_REASONING_ENABLED_EFFORT: &str = "enabled reasoning must request non-none effort";
const INVALID_TOOL_ROLE_WITHOUT_CALL: &str = "tool role requires a tool-call id";
const INVALID_NON_TOOL_WITH_CALL: &str = "only tool role may carry a tool-call id";
const INVALID_TOKEN_BUDGET: &str = "token budget is outside accepted bounds";
const INVALID_TEMPERATURE: &str = "temperature millis is outside accepted bounds";
const INVALID_REFUSAL_COMBINATION: &str =
    "refusal or safety output cannot combine with success output";
const INVALID_MALFORMED_COMBINATION: &str = "malformed output cannot combine with success output";
const LOSSY_LEGACY_REQUEST: &str = "canonical request cannot collapse into one legacy prompt";
const LOSSY_LEGACY_RESPONSE: &str = "canonical response carries non-text semantics";

// r[impl agent_harness_runtime.model_compat.message_request]
/// Stable contract identity for canonical provider-neutral model request DTOs.
pub const CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY: &str = "animus.model-message-request.v1";
/// Stable contract identity for canonical provider-neutral model response envelopes.
pub const CANONICAL_MODEL_RESPONSE_CONTRACT_IDENTITY: &str = "animus.model-response-envelope.v1";

/// Unsafe material class rejected from canonical model contracts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalUnsafeTextReason {
    /// Raw provider-native payload material was supplied at a provider-neutral boundary.
    RawProviderPayload,
    /// Credential or authorization material was supplied at a provider-neutral boundary.
    Credential,
    /// Hidden reasoning or thinking text would become assistant-visible.
    HiddenReasoning,
}

/// Validation error for canonical model request, response, and compatibility bridge contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalModelContractError {
    /// A shared adapter-field validator rejected a scalar field.
    AdapterContract {
        /// Underlying adapter contract error.
        source: crate::AdapterContractError,
    },
    /// The supplied stable contract identity was missing or not the expected value.
    InvalidContractIdentity {
        /// Expected stable contract identity.
        expected: &'static str,
        /// Supplied stable contract identity after validation.
        actual: alloc::string::String,
    },
    /// A required collection was empty.
    EmptyCollection {
        /// Collection field name.
        field: &'static str,
    },
    /// A bounded collection exceeded its maximum item count.
    CollectionTooLarge {
        /// Collection field name.
        field: &'static str,
        /// Maximum accepted item count.
        max_items: usize,
        /// Actual item count.
        actual_items: usize,
    },
    /// A message role label is not part of the canonical role set.
    UnsupportedRole {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported role label.
        value: alloc::string::String,
    },
    /// A content part cannot be represented at the canonical boundary.
    UnsupportedContentPart {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported content-part kind.
        value: alloc::string::String,
    },
    /// A field contained unsafe provider, credential, or hidden-reasoning material.
    UnsafeText {
        /// Boundary field name.
        field: &'static str,
        /// Unsafe material class.
        reason: CanonicalUnsafeTextReason,
    },
    /// A role/correlation, output-control, or other scalar invariant failed.
    InvalidField {
        /// Boundary field name.
        field: &'static str,
        /// Human-readable invariant failure.
        reason: &'static str,
    },
    /// A response envelope omitted the required terminal finish reason.
    MissingFinishReason {
        /// Boundary field name.
        field: &'static str,
    },
    /// A response finish reason is unsupported.
    UnsupportedFinishReason {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported finish reason label.
        value: alloc::string::String,
    },
    /// Response usage metadata was malformed.
    MalformedUsage {
        /// Boundary field name.
        field: &'static str,
        /// Human-readable invariant failure.
        reason: &'static str,
    },
    /// Response items combined mutually exclusive semantic classes.
    ImpossibleResponseCombination {
        /// Human-readable invariant failure.
        reason: &'static str,
    },
    /// A compatibility projection would lose canonical semantics.
    LossyLegacyProjection {
        /// Human-readable projection failure.
        reason: &'static str,
    },
}

impl From<crate::AdapterContractError> for CanonicalModelContractError {
    fn from(source: crate::AdapterContractError) -> Self {
        Self::AdapterContract { source }
    }
}

/// Borrowed safe provenance metadata entry before canonical validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalSafeMetadataDraft<'a> {
    /// Safe metadata key.
    pub key: &'a str,
    /// Safe metadata value.
    pub value: &'a str,
}

/// Safe provenance metadata entry carried by canonical contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalSafeMetadata {
    key: alloc::string::String,
    value: alloc::string::String,
}

impl CanonicalSafeMetadata {
    /// Returns the metadata key.
    #[must_use]
    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    /// Returns the metadata value.
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

/// Canonical stable model message role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalModelRole {
    /// System-level instruction role.
    System,
    /// Developer-level instruction role.
    Developer,
    /// User-visible input role.
    User,
    /// Assistant output role.
    Assistant,
    /// Tool-result role correlated to a tool call.
    Tool,
}

impl CanonicalModelRole {
    /// Parses a stable role label.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when the label is empty, unsafe, or unsupported.
    pub fn parse(raw: &str) -> Result<Self, CanonicalModelContractError> {
        let normalized = safe_text(ROLE_FIELD, raw)?;
        match normalized.as_str() {
            ROLE_SYSTEM => Ok(Self::System),
            ROLE_DEVELOPER => Ok(Self::Developer),
            ROLE_USER => Ok(Self::User),
            ROLE_ASSISTANT => Ok(Self::Assistant),
            ROLE_TOOL => Ok(Self::Tool),
            _unsupported => Err(CanonicalModelContractError::UnsupportedRole {
                field: ROLE_FIELD.as_str(),
                value: normalized,
            }),
        }
    }

    /// Returns the stable role label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::System => ROLE_SYSTEM,
            Self::Developer => ROLE_DEVELOPER,
            Self::User => ROLE_USER,
            Self::Assistant => ROLE_ASSISTANT,
            Self::Tool => ROLE_TOOL,
        }
    }
}

/// Borrowed canonical content part before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalContentPartDraft<'a> {
    /// Assistant-visible or model-visible text content.
    Text {
        /// Text body.
        text: &'a str,
    },
    /// Descriptor-backed image content without raw bytes or URLs.
    ImageDescriptor {
        /// Safe descriptor identifier.
        descriptor_id: &'a str,
        /// Safe media type label.
        media_type: &'a str,
        /// Safe content hash.
        content_hash: &'a str,
    },
    /// Unsupported content part used to prove fail-closed behavior.
    Unsupported {
        /// Unsupported content-part kind label.
        kind: &'a str,
    },
    /// Raw provider-native payload used to prove leak rejection.
    RawProviderPayload {
        /// Unsafe payload label.
        label: &'a str,
    },
}

/// Validated canonical provider-neutral content part.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalContentPart {
    /// Text content part.
    Text {
        /// Validated text body.
        text: alloc::string::String,
    },
    /// Descriptor-backed image content part.
    ImageDescriptor {
        /// Safe descriptor identifier.
        descriptor_id: alloc::string::String,
        /// Safe media type label.
        media_type: alloc::string::String,
        /// Safe content hash.
        content_hash: alloc::string::String,
    },
}

impl CanonicalContentPart {
    /// Validates a borrowed content-part draft.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when content is empty, unsupported, or unsafe.
    pub fn from_draft(
        draft: CanonicalContentPartDraft<'_>,
    ) -> Result<Self, CanonicalModelContractError> {
        match draft {
            CanonicalContentPartDraft::Text { text } => Ok(Self::Text {
                text: safe_text(CONTENT_TEXT_FIELD, text)?,
            }),
            CanonicalContentPartDraft::ImageDescriptor {
                descriptor_id,
                media_type,
                content_hash,
            } => Ok(Self::ImageDescriptor {
                descriptor_id: safe_text(IMAGE_DESCRIPTOR_FIELD, descriptor_id)?,
                media_type: safe_text(IMAGE_MEDIA_TYPE_FIELD, media_type)?,
                content_hash: safe_text(IMAGE_HASH_FIELD, content_hash)?,
            }),
            CanonicalContentPartDraft::Unsupported { kind } => {
                let value = safe_text(CONTENT_PART_KIND_FIELD, kind)?;
                Err(CanonicalModelContractError::UnsupportedContentPart {
                    field: CONTENT_PART_KIND_FIELD.as_str(),
                    value,
                })
            }
            CanonicalContentPartDraft::RawProviderPayload { label } => {
                let _label =
                    crate::validation::normalized_adapter_field(RAW_PROVIDER_PAYLOAD_FIELD, label)?;
                Err(CanonicalModelContractError::UnsafeText {
                    field: RAW_PROVIDER_PAYLOAD_FIELD.as_str(),
                    reason: CanonicalUnsafeTextReason::RawProviderPayload,
                })
            }
        }
    }

    /// Returns true when the part is text.
    #[must_use]
    pub const fn is_text(&self) -> bool {
        matches!(self, Self::Text { .. })
    }

    /// Returns the text body when this is a text part.
    #[must_use]
    pub fn text(&self) -> Option<&str> {
        match self {
            Self::Text { text } => Some(text.as_str()),
            Self::ImageDescriptor { .. } => None,
        }
    }
}

/// Borrowed canonical message before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalModelMessageDraft<'a> {
    /// Stable role label.
    pub role: &'a str,
    /// Ordered content parts.
    pub content: &'a [CanonicalContentPartDraft<'a>],
    /// Tool-call identifier required only for tool-result messages.
    pub tool_call_id: Option<&'a str>,
}

/// Validated canonical message.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalModelMessage {
    role: CanonicalModelRole,
    content: alloc::vec::Vec<CanonicalContentPart>,
    tool_call_id: Option<alloc::string::String>,
}

impl CanonicalModelMessage {
    /// Validates a borrowed message draft.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when the role, content, or tool correlation is invalid.
    pub fn from_draft(
        draft: CanonicalModelMessageDraft<'_>,
    ) -> Result<Self, CanonicalModelContractError> {
        let role = CanonicalModelRole::parse(draft.role)?;
        require_collection(
            CONTENT_PART_KIND_FIELD,
            draft.content.len(),
            MAX_CONTENT_PARTS_PER_MESSAGE,
        )?;
        let content = validate_content_parts(draft.content)?;
        let tool_call_id = validate_tool_call_id(role, draft.tool_call_id)?;
        Ok(Self {
            role,
            content,
            tool_call_id,
        })
    }

    /// Returns the stable role.
    #[must_use]
    pub const fn role(&self) -> CanonicalModelRole {
        self.role
    }

    /// Returns validated content parts.
    #[must_use]
    pub fn content(&self) -> &[CanonicalContentPart] {
        self.content.as_slice()
    }

    /// Returns the optional tool-call correlation identifier.
    #[must_use]
    pub fn tool_call_id(&self) -> Option<&str> {
        self.tool_call_id.as_deref()
    }
}

/// Borrowed tool declaration before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalToolDeclarationDraft<'a> {
    /// Safe tool name.
    pub name: &'a str,
    /// Safe human-readable description.
    pub description: &'a str,
    /// Safe input schema label or digest.
    pub input_schema_label: &'a str,
}

/// Validated canonical tool declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalToolDeclaration {
    name: alloc::string::String,
    description: alloc::string::String,
    input_schema_label: alloc::string::String,
}

impl CanonicalToolDeclaration {
    /// Validates a borrowed tool declaration.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when a tool field is empty or unsafe.
    pub fn from_draft(
        draft: CanonicalToolDeclarationDraft<'_>,
    ) -> Result<Self, CanonicalModelContractError> {
        Ok(Self {
            name: safe_text(TOOL_NAME_FIELD, draft.name)?,
            description: safe_text(TOOL_DESCRIPTION_FIELD, draft.description)?,
            input_schema_label: safe_text(TOOL_SCHEMA_FIELD, draft.input_schema_label)?,
        })
    }

    /// Returns the tool name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the safe description.
    #[must_use]
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the input schema label.
    #[must_use]
    pub fn input_schema_label(&self) -> &str {
        self.input_schema_label.as_str()
    }
}

/// Borrowed output controls before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalOutputControlsDraft<'a> {
    /// Optional maximum output token budget.
    pub max_output_tokens: Option<u32>,
    /// Optional deterministic temperature encoded as thousandths.
    pub temperature_millis: Option<u16>,
    /// Stop sequences requested by the host.
    pub stop_sequences: &'a [&'a str],
    /// Whether the caller requests streaming frames.
    pub stream: bool,
}

/// Validated canonical output controls.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalOutputControls {
    max_output_tokens: Option<u32>,
    temperature_millis: Option<u16>,
    stop_sequences: alloc::vec::Vec<alloc::string::String>,
    stream: bool,
}

impl CanonicalOutputControls {
    /// Validates borrowed output controls.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when token, sampling, or stop-sequence bounds fail.
    pub fn from_draft(
        draft: CanonicalOutputControlsDraft<'_>,
    ) -> Result<Self, CanonicalModelContractError> {
        validate_token_budget(draft.max_output_tokens)?;
        validate_temperature(draft.temperature_millis)?;
        validate_collection_len(
            STOP_SEQUENCE_FIELD,
            draft.stop_sequences.len(),
            MAX_STOP_SEQUENCES,
        )?;
        Ok(Self {
            max_output_tokens: draft.max_output_tokens,
            temperature_millis: draft.temperature_millis,
            stop_sequences: validate_text_slice(STOP_SEQUENCE_FIELD, draft.stop_sequences)?,
            stream: draft.stream,
        })
    }

    /// Returns compatibility defaults for the legacy prompt/message path.
    #[must_use]
    pub fn compatibility_defaults() -> Self {
        Self {
            max_output_tokens: None,
            temperature_millis: None,
            stop_sequences: alloc::vec::Vec::new(),
            stream: false,
        }
    }

    /// Returns the optional output-token budget.
    #[must_use]
    pub const fn max_output_tokens(&self) -> Option<u32> {
        self.max_output_tokens
    }

    /// Returns the optional temperature in thousandths.
    #[must_use]
    pub const fn temperature_millis(&self) -> Option<u16> {
        self.temperature_millis
    }

    /// Returns stop sequences.
    #[must_use]
    pub fn stop_sequences(&self) -> &[alloc::string::String] {
        self.stop_sequences.as_slice()
    }

    /// Returns whether streaming is requested.
    #[must_use]
    pub const fn stream(&self) -> bool {
        self.stream
    }

    /// Returns true when controls match legacy compatibility defaults.
    #[must_use]
    pub fn is_legacy_compatible(&self) -> bool {
        self.max_output_tokens.is_none()
            && self.temperature_millis.is_none()
            && self.stop_sequences.is_empty()
            && !self.stream
    }
}

/// Borrowed response-format intent before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalResponseFormatIntentDraft<'a> {
    /// Plain assistant-visible text.
    Text,
    /// Provider-neutral JSON object intent.
    JsonObject,
    /// Provider-neutral JSON schema label or digest.
    JsonSchema {
        /// Safe schema label or digest.
        schema_label: &'a str,
    },
}

/// Validated response-format intent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalResponseFormatIntent {
    /// Plain assistant-visible text.
    Text,
    /// Provider-neutral JSON object intent.
    JsonObject,
    /// Provider-neutral JSON schema label or digest.
    JsonSchema {
        /// Safe schema label or digest.
        schema_label: alloc::string::String,
    },
}

impl CanonicalResponseFormatIntent {
    /// Validates a borrowed response-format intent.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when a schema label is empty or unsafe.
    pub fn from_draft(
        draft: CanonicalResponseFormatIntentDraft<'_>,
    ) -> Result<Self, CanonicalModelContractError> {
        match draft {
            CanonicalResponseFormatIntentDraft::Text => Ok(Self::Text),
            CanonicalResponseFormatIntentDraft::JsonObject => Ok(Self::JsonObject),
            CanonicalResponseFormatIntentDraft::JsonSchema { schema_label } => {
                Ok(Self::JsonSchema {
                    schema_label: safe_text(RESPONSE_SCHEMA_FIELD, schema_label)?,
                })
            }
        }
    }
}

/// Canonical reasoning or thinking effort request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalReasoningEffort {
    /// No reasoning/thinking request.
    None,
    /// Low effort.
    Low,
    /// Medium effort.
    Medium,
    /// High effort.
    High,
}

/// Borrowed reasoning or thinking request before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalReasoningRequestDraft<'a> {
    /// Whether reasoning or thinking is requested.
    pub enabled: bool,
    /// Requested reasoning effort.
    pub effort: CanonicalReasoningEffort,
    /// Optional safe summary label; hidden reasoning text is rejected.
    pub summary_label: Option<&'a str>,
}

/// Validated reasoning or thinking request metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalReasoningRequest {
    enabled: bool,
    effort: CanonicalReasoningEffort,
    summary_label: Option<alloc::string::String>,
}

impl CanonicalReasoningRequest {
    /// Validates a borrowed reasoning request.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when enabled/effort invariants fail or summary metadata is unsafe.
    pub fn from_draft(
        draft: CanonicalReasoningRequestDraft<'_>,
    ) -> Result<Self, CanonicalModelContractError> {
        validate_reasoning_effort(draft.enabled, draft.effort)?;
        Ok(Self {
            enabled: draft.enabled,
            effort: draft.effort,
            summary_label: draft
                .summary_label
                .map(|raw| safe_text(REASONING_SUMMARY_FIELD, raw))
                .transpose()?,
        })
    }

    /// Returns compatibility defaults for the legacy prompt/message path.
    #[must_use]
    pub const fn compatibility_defaults() -> Self {
        Self {
            enabled: false,
            effort: CanonicalReasoningEffort::None,
            summary_label: None,
        }
    }

    /// Returns whether reasoning is requested.
    #[must_use]
    pub const fn enabled(&self) -> bool {
        self.enabled
    }

    /// Returns the requested reasoning effort.
    #[must_use]
    pub const fn effort(&self) -> CanonicalReasoningEffort {
        self.effort
    }
}

/// Borrowed canonical model request before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalModelRequestDraft<'a> {
    /// Stable contract identity.
    pub contract_identity: &'a str,
    /// Ordered messages.
    pub messages: &'a [CanonicalModelMessageDraft<'a>],
    /// Tool declarations available for this request.
    pub tools: &'a [CanonicalToolDeclarationDraft<'a>],
    /// Output controls.
    pub output: CanonicalOutputControlsDraft<'a>,
    /// Response-format intent.
    pub response_format: CanonicalResponseFormatIntentDraft<'a>,
    /// Reasoning or thinking request.
    pub reasoning: CanonicalReasoningRequestDraft<'a>,
    /// Safe provenance metadata.
    pub provenance: &'a [CanonicalSafeMetadataDraft<'a>],
}

/// Validated provider-neutral canonical model request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalModelRequest {
    messages: alloc::vec::Vec<CanonicalModelMessage>,
    tools: alloc::vec::Vec<CanonicalToolDeclaration>,
    output: CanonicalOutputControls,
    response_format: CanonicalResponseFormatIntent,
    reasoning: CanonicalReasoningRequest,
    provenance: alloc::vec::Vec<CanonicalSafeMetadata>,
}

impl CanonicalModelRequest {
    /// Validates a borrowed canonical request draft.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when any request field is malformed, unsafe, or unsupported.
    pub fn from_draft(
        draft: CanonicalModelRequestDraft<'_>,
    ) -> Result<Self, CanonicalModelContractError> {
        validate_contract_identity(
            CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            draft.contract_identity,
        )?;
        require_collection(ROLE_FIELD, draft.messages.len(), MAX_MODEL_MESSAGES)?;
        validate_collection_len(TOOL_NAME_FIELD, draft.tools.len(), MAX_MODEL_TOOLS)?;
        Ok(Self {
            messages: validate_messages(draft.messages)?,
            tools: validate_tools(draft.tools)?,
            output: CanonicalOutputControls::from_draft(draft.output)?,
            response_format: CanonicalResponseFormatIntent::from_draft(draft.response_format)?,
            reasoning: CanonicalReasoningRequest::from_draft(draft.reasoning)?,
            provenance: validate_metadata(draft.provenance)?,
        })
    }

    /// Builds a canonical compatibility request from the legacy prompt DTO.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when the legacy prompt somehow fails canonical validation.
    pub fn from_legacy_request(
        request: &super::Request,
    ) -> Result<Self, CanonicalModelContractError> {
        let content = [CanonicalContentPartDraft::Text {
            text: request.prompt(),
        }];
        let messages = [CanonicalModelMessageDraft {
            role: ROLE_USER,
            content: &content,
            tool_call_id: None,
        }];
        Self::from_draft(CanonicalModelRequestDraft {
            contract_identity: CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY,
            messages: &messages,
            tools: &[],
            output: CanonicalOutputControlsDraft {
                max_output_tokens: None,
                temperature_millis: None,
                stop_sequences: &[],
                stream: false,
            },
            response_format: CanonicalResponseFormatIntentDraft::Text,
            reasoning: CanonicalReasoningRequestDraft {
                enabled: false,
                effort: CanonicalReasoningEffort::None,
                summary_label: None,
            },
            provenance: &[],
        })
    }

    /// Projects a canonical compatibility request back to the legacy prompt DTO.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when projection would lose structured, tool, stream, or reasoning semantics.
    pub fn to_legacy_request(&self) -> Result<super::Request, CanonicalModelContractError> {
        if !self.tools.is_empty()
            || !self.output.is_legacy_compatible()
            || self.response_format != CanonicalResponseFormatIntent::Text
            || self.reasoning.enabled()
            || !self.provenance.is_empty()
        {
            return Err(lossy_legacy_request());
        }
        let [message] = self.messages.as_slice() else {
            return Err(lossy_legacy_request());
        };
        if message.role() != CanonicalModelRole::User || message.tool_call_id().is_some() {
            return Err(lossy_legacy_request());
        }
        let [part] = message.content() else {
            return Err(lossy_legacy_request());
        };
        let Some(text) = part.text() else {
            return Err(lossy_legacy_request());
        };
        Ok(super::Request::new(text)?)
    }

    /// Returns validated messages.
    #[must_use]
    pub fn messages(&self) -> &[CanonicalModelMessage] {
        self.messages.as_slice()
    }

    /// Returns validated tool declarations.
    #[must_use]
    pub fn tools(&self) -> &[CanonicalToolDeclaration] {
        self.tools.as_slice()
    }

    /// Returns output controls.
    #[must_use]
    pub const fn output(&self) -> &CanonicalOutputControls {
        &self.output
    }

    /// Returns response-format intent.
    #[must_use]
    pub const fn response_format(&self) -> &CanonicalResponseFormatIntent {
        &self.response_format
    }

    /// Returns reasoning request metadata.
    #[must_use]
    pub const fn reasoning(&self) -> &CanonicalReasoningRequest {
        &self.reasoning
    }

    /// Returns safe provenance metadata.
    #[must_use]
    pub fn provenance(&self) -> &[CanonicalSafeMetadata] {
        self.provenance.as_slice()
    }

    /// Returns the stable canonical request contract identity.
    #[must_use]
    pub const fn contract_identity() -> &'static str {
        CANONICAL_MODEL_REQUEST_CONTRACT_IDENTITY
    }
}

/// Borrowed usage metadata before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalUsageDraft {
    /// Provider-reported input token count.
    pub input: u32,
    /// Provider-reported output token count.
    pub output: u32,
    /// Optional cached input token count.
    pub cached_input: Option<u32>,
}

/// Validated usage metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalUsage {
    input: u32,
    output: u32,
    cached_input: Option<u32>,
}

impl CanonicalUsage {
    /// Validates usage metadata.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when usage counts are malformed.
    pub fn from_draft(draft: CanonicalUsageDraft) -> Result<Self, CanonicalModelContractError> {
        if draft.input == 0 && draft.output == 0 {
            return Err(CanonicalModelContractError::MalformedUsage {
                field: RESPONSE_TEXT_FIELD.as_str(),
                reason: INVALID_ZERO_USAGE,
            });
        }
        if let Some(cached) = draft.cached_input
            && cached > draft.input
        {
            return Err(CanonicalModelContractError::MalformedUsage {
                field: RESPONSE_TEXT_FIELD.as_str(),
                reason: INVALID_CACHED_USAGE,
            });
        }
        Ok(Self {
            input: draft.input,
            output: draft.output,
            cached_input: draft.cached_input,
        })
    }

    /// Returns input-token count.
    #[must_use]
    pub const fn input_tokens(&self) -> u32 {
        self.input
    }

    /// Returns output-token count.
    #[must_use]
    pub const fn output_tokens(&self) -> u32 {
        self.output
    }

    /// Returns cached input-token count when provided.
    #[must_use]
    pub const fn cached_input_tokens(&self) -> Option<u32> {
        self.cached_input
    }
}

/// Canonical terminal finish reason.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalFinishReason {
    /// Provider reached normal stop.
    Stop,
    /// Provider stopped because output length was exhausted.
    Length,
    /// Provider requested one or more tool calls.
    ToolCall,
    /// Provider refused the request.
    Refusal,
    /// Provider returned a safety block.
    Safety,
    /// Provider output was malformed and normalized as a diagnostic.
    Malformed,
}

impl CanonicalFinishReason {
    /// Parses a stable finish-reason label.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when the label is empty, unsafe, or unsupported.
    pub fn parse(raw: &str) -> Result<Self, CanonicalModelContractError> {
        let normalized = safe_text(FINISH_REASON_FIELD, raw)?;
        match normalized.as_str() {
            FINISH_STOP => Ok(Self::Stop),
            FINISH_LENGTH => Ok(Self::Length),
            FINISH_TOOL_CALL => Ok(Self::ToolCall),
            FINISH_REFUSAL => Ok(Self::Refusal),
            FINISH_SAFETY => Ok(Self::Safety),
            FINISH_MALFORMED => Ok(Self::Malformed),
            _unsupported => Err(CanonicalModelContractError::UnsupportedFinishReason {
                field: FINISH_REASON_FIELD.as_str(),
                value: normalized,
            }),
        }
    }

    /// Returns the stable finish-reason label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stop => FINISH_STOP,
            Self::Length => FINISH_LENGTH,
            Self::ToolCall => FINISH_TOOL_CALL,
            Self::Refusal => FINISH_REFUSAL,
            Self::Safety => FINISH_SAFETY,
            Self::Malformed => FINISH_MALFORMED,
        }
    }
}

/// Borrowed response item before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CanonicalModelResponseItemDraft<'a> {
    /// Assistant-visible text.
    AssistantText {
        /// Text body.
        text: &'a str,
    },
    /// Structured output represented by safe labels rather than raw provider payloads.
    StructuredPayload {
        /// Safe schema label.
        schema_label: &'a str,
        /// Safe payload label or digest.
        payload_label: &'a str,
    },
    /// Tool-call request.
    ToolCall {
        /// Safe call identifier.
        call_id: &'a str,
        /// Safe tool name.
        tool_name: &'a str,
        /// Safe arguments label or digest.
        arguments_label: &'a str,
    },
    /// Refusal block.
    Refusal {
        /// Safe refusal reason.
        reason: &'a str,
    },
    /// Safety block.
    Safety {
        /// Safe safety category.
        category: &'a str,
    },
    /// Malformed-output diagnostic.
    MalformedOutput {
        /// Safe diagnostic label.
        diagnostic: &'a str,
    },
    /// Raw provider-native payload used to prove leak rejection.
    RawProviderPayload {
        /// Unsafe payload label.
        label: &'a str,
    },
    /// Hidden reasoning text used to prove leak rejection.
    HiddenReasoning {
        /// Unsafe hidden reasoning label.
        label: &'a str,
    },
}

/// Validated canonical response item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CanonicalModelResponseItem {
    /// Assistant-visible text.
    AssistantText {
        /// Validated text body.
        text: alloc::string::String,
    },
    /// Structured output represented by safe labels rather than raw provider payloads.
    StructuredPayload {
        /// Safe schema label.
        schema_label: alloc::string::String,
        /// Safe payload label or digest.
        payload_label: alloc::string::String,
    },
    /// Tool-call request.
    ToolCall {
        /// Safe call identifier.
        call_id: alloc::string::String,
        /// Safe tool name.
        tool_name: alloc::string::String,
        /// Safe arguments label or digest.
        arguments_label: alloc::string::String,
    },
    /// Refusal block.
    Refusal {
        /// Safe refusal reason.
        reason: alloc::string::String,
    },
    /// Safety block.
    Safety {
        /// Safe safety category.
        category: alloc::string::String,
    },
    /// Malformed-output diagnostic.
    MalformedOutput {
        /// Safe diagnostic label.
        diagnostic: alloc::string::String,
    },
}

impl CanonicalModelResponseItem {
    /// Validates a borrowed response item.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when the item is empty, unsafe, or leaky.
    pub fn from_draft(
        draft: CanonicalModelResponseItemDraft<'_>,
    ) -> Result<Self, CanonicalModelContractError> {
        match draft {
            CanonicalModelResponseItemDraft::AssistantText { text } => Ok(Self::AssistantText {
                text: safe_text(RESPONSE_TEXT_FIELD, text)?,
            }),
            CanonicalModelResponseItemDraft::StructuredPayload {
                schema_label,
                payload_label,
            } => Ok(Self::StructuredPayload {
                schema_label: safe_text(RESPONSE_SCHEMA_FIELD, schema_label)?,
                payload_label: safe_text(STRUCTURED_PAYLOAD_FIELD, payload_label)?,
            }),
            CanonicalModelResponseItemDraft::ToolCall {
                call_id,
                tool_name,
                arguments_label,
            } => Ok(Self::ToolCall {
                call_id: safe_text(TOOL_CALL_ID_FIELD, call_id)?,
                tool_name: safe_text(TOOL_NAME_FIELD, tool_name)?,
                arguments_label: safe_text(TOOL_ARGUMENTS_FIELD, arguments_label)?,
            }),
            CanonicalModelResponseItemDraft::Refusal { reason } => Ok(Self::Refusal {
                reason: safe_text(REFUSAL_REASON_FIELD, reason)?,
            }),
            CanonicalModelResponseItemDraft::Safety { category } => Ok(Self::Safety {
                category: safe_text(SAFETY_CATEGORY_FIELD, category)?,
            }),
            CanonicalModelResponseItemDraft::MalformedOutput { diagnostic } => {
                Ok(Self::MalformedOutput {
                    diagnostic: safe_text(MALFORMED_DIAGNOSTIC_FIELD, diagnostic)?,
                })
            }
            CanonicalModelResponseItemDraft::RawProviderPayload { label } => {
                let _label =
                    crate::validation::normalized_adapter_field(RAW_PROVIDER_PAYLOAD_FIELD, label)?;
                Err(CanonicalModelContractError::UnsafeText {
                    field: RAW_PROVIDER_PAYLOAD_FIELD.as_str(),
                    reason: CanonicalUnsafeTextReason::RawProviderPayload,
                })
            }
            CanonicalModelResponseItemDraft::HiddenReasoning { label } => {
                let _label =
                    crate::validation::normalized_adapter_field(HIDDEN_REASONING_FIELD, label)?;
                Err(CanonicalModelContractError::UnsafeText {
                    field: HIDDEN_REASONING_FIELD.as_str(),
                    reason: CanonicalUnsafeTextReason::HiddenReasoning,
                })
            }
        }
    }

    /// Returns true when the item is assistant text.
    #[must_use]
    pub const fn is_assistant_text(&self) -> bool {
        matches!(self, Self::AssistantText { .. })
    }

    /// Returns the assistant text when present.
    #[must_use]
    pub fn assistant_text(&self) -> Option<&str> {
        match self {
            Self::AssistantText { text } => Some(text.as_str()),
            Self::StructuredPayload { .. }
            | Self::ToolCall { .. }
            | Self::Refusal { .. }
            | Self::Safety { .. }
            | Self::MalformedOutput { .. } => None,
        }
    }
}

/// Borrowed canonical response envelope before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CanonicalModelResponseEnvelopeDraft<'a> {
    /// Stable contract identity.
    pub contract_identity: &'a str,
    /// Ordered response items.
    pub items: &'a [CanonicalModelResponseItemDraft<'a>],
    /// Optional usage metadata.
    pub usage: Option<CanonicalUsageDraft>,
    /// Required finish reason label.
    pub finish_reason: Option<&'a str>,
    /// Safe malformed-output or normalization diagnostics.
    pub diagnostics: &'a [&'a str],
    /// Safe provenance metadata.
    pub provenance: &'a [CanonicalSafeMetadataDraft<'a>],
}

/// Validated canonical model response envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CanonicalModelResponseEnvelope {
    items: alloc::vec::Vec<CanonicalModelResponseItem>,
    usage: Option<CanonicalUsage>,
    finish_reason: CanonicalFinishReason,
    diagnostics: alloc::vec::Vec<alloc::string::String>,
    provenance: alloc::vec::Vec<CanonicalSafeMetadata>,
}

impl CanonicalModelResponseEnvelope {
    /// Validates a borrowed response-envelope draft.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when any response field is malformed, unsafe, or semantically impossible.
    pub fn from_draft(
        draft: CanonicalModelResponseEnvelopeDraft<'_>,
    ) -> Result<Self, CanonicalModelContractError> {
        validate_contract_identity(
            CANONICAL_MODEL_RESPONSE_CONTRACT_IDENTITY,
            draft.contract_identity,
        )?;
        require_collection(RESPONSE_TEXT_FIELD, draft.items.len(), MAX_RESPONSE_ITEMS)?;
        validate_collection_len(
            MALFORMED_DIAGNOSTIC_FIELD,
            draft.diagnostics.len(),
            MAX_RESPONSE_DIAGNOSTICS,
        )?;
        let Some(raw_finish_reason) = draft.finish_reason else {
            return Err(CanonicalModelContractError::MissingFinishReason {
                field: FINISH_REASON_FIELD.as_str(),
            });
        };
        let items = validate_response_items(draft.items)?;
        validate_response_combination(items.as_slice())?;
        Ok(Self {
            items,
            usage: draft.usage.map(CanonicalUsage::from_draft).transpose()?,
            finish_reason: CanonicalFinishReason::parse(raw_finish_reason)?,
            diagnostics: validate_text_slice(MALFORMED_DIAGNOSTIC_FIELD, draft.diagnostics)?,
            provenance: validate_metadata(draft.provenance)?,
        })
    }

    /// Builds a canonical compatibility response from the legacy message DTO.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when the legacy message somehow fails canonical validation.
    pub fn from_legacy_response(
        response: &super::Response,
    ) -> Result<Self, CanonicalModelContractError> {
        let items = [CanonicalModelResponseItemDraft::AssistantText {
            text: response.message(),
        }];
        Self::from_draft(CanonicalModelResponseEnvelopeDraft {
            contract_identity: CANONICAL_MODEL_RESPONSE_CONTRACT_IDENTITY,
            items: &items,
            usage: None,
            finish_reason: Some(FINISH_STOP),
            diagnostics: &[],
            provenance: &[],
        })
    }

    /// Projects a canonical compatibility response back to the legacy message DTO.
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalModelContractError`] when projection would lose structured, tool, refusal, safety, or malformed semantics.
    pub fn to_legacy_response(&self) -> Result<super::Response, CanonicalModelContractError> {
        if !matches!(
            self.finish_reason,
            CanonicalFinishReason::Stop | CanonicalFinishReason::Length
        ) || !self.diagnostics.is_empty()
            || self.usage.is_some()
            || !self.provenance.is_empty()
        {
            return Err(lossy_legacy_response());
        }
        let [item] = self.items.as_slice() else {
            return Err(lossy_legacy_response());
        };
        let Some(text) = item.assistant_text() else {
            return Err(lossy_legacy_response());
        };
        Ok(super::Response::new(text)?)
    }

    /// Returns response items.
    #[must_use]
    pub fn items(&self) -> &[CanonicalModelResponseItem] {
        self.items.as_slice()
    }

    /// Returns optional usage metadata.
    #[must_use]
    pub const fn usage(&self) -> Option<CanonicalUsage> {
        self.usage
    }

    /// Returns the terminal finish reason.
    #[must_use]
    pub const fn finish_reason(&self) -> CanonicalFinishReason {
        self.finish_reason
    }

    /// Returns safe diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[alloc::string::String] {
        self.diagnostics.as_slice()
    }

    /// Returns safe provenance metadata.
    #[must_use]
    pub fn provenance(&self) -> &[CanonicalSafeMetadata] {
        self.provenance.as_slice()
    }

    /// Returns the stable canonical response contract identity.
    #[must_use]
    pub const fn contract_identity() -> &'static str {
        CANONICAL_MODEL_RESPONSE_CONTRACT_IDENTITY
    }
}

fn validate_contract_identity(
    expected: &'static str,
    raw: &str,
) -> Result<(), CanonicalModelContractError> {
    let actual = safe_text(CONTRACT_IDENTITY_FIELD, raw)?;
    if actual == expected {
        return Ok(());
    }
    Err(CanonicalModelContractError::InvalidContractIdentity { expected, actual })
}

fn validate_messages(
    drafts: &[CanonicalModelMessageDraft<'_>],
) -> Result<alloc::vec::Vec<CanonicalModelMessage>, CanonicalModelContractError> {
    drafts
        .iter()
        .copied()
        .map(CanonicalModelMessage::from_draft)
        .collect()
}

fn validate_content_parts(
    drafts: &[CanonicalContentPartDraft<'_>],
) -> Result<alloc::vec::Vec<CanonicalContentPart>, CanonicalModelContractError> {
    drafts
        .iter()
        .copied()
        .map(CanonicalContentPart::from_draft)
        .collect()
}

fn validate_tools(
    drafts: &[CanonicalToolDeclarationDraft<'_>],
) -> Result<alloc::vec::Vec<CanonicalToolDeclaration>, CanonicalModelContractError> {
    drafts
        .iter()
        .copied()
        .map(CanonicalToolDeclaration::from_draft)
        .collect()
}

fn validate_response_items(
    drafts: &[CanonicalModelResponseItemDraft<'_>],
) -> Result<alloc::vec::Vec<CanonicalModelResponseItem>, CanonicalModelContractError> {
    drafts
        .iter()
        .copied()
        .map(CanonicalModelResponseItem::from_draft)
        .collect()
}

fn validate_text_slice(
    field: crate::validation::FieldName,
    values: &[&str],
) -> Result<alloc::vec::Vec<alloc::string::String>, CanonicalModelContractError> {
    values.iter().map(|value| safe_text(field, value)).collect()
}

fn validate_metadata(
    drafts: &[CanonicalSafeMetadataDraft<'_>],
) -> Result<alloc::vec::Vec<CanonicalSafeMetadata>, CanonicalModelContractError> {
    validate_collection_len(METADATA_KEY_FIELD, drafts.len(), MAX_SAFE_METADATA_ITEMS)?;
    drafts
        .iter()
        .map(|draft| {
            Ok(CanonicalSafeMetadata {
                key: safe_text(METADATA_KEY_FIELD, draft.key)?,
                value: safe_text(METADATA_VALUE_FIELD, draft.value)?,
            })
        })
        .collect()
}

fn validate_tool_call_id(
    role: CanonicalModelRole,
    value: Option<&str>,
) -> Result<Option<alloc::string::String>, CanonicalModelContractError> {
    match (role, value) {
        (CanonicalModelRole::Tool, Some(raw)) => Ok(Some(safe_text(TOOL_CALL_ID_FIELD, raw)?)),
        (CanonicalModelRole::Tool, None) => Err(CanonicalModelContractError::InvalidField {
            field: TOOL_CALL_ID_FIELD.as_str(),
            reason: INVALID_TOOL_ROLE_WITHOUT_CALL,
        }),
        (_, Some(_raw)) => Err(CanonicalModelContractError::InvalidField {
            field: TOOL_CALL_ID_FIELD.as_str(),
            reason: INVALID_NON_TOOL_WITH_CALL,
        }),
        (_, None) => Ok(None),
    }
}

fn validate_token_budget(value: Option<u32>) -> Result<(), CanonicalModelContractError> {
    if value.is_some_and(|tokens| !(MIN_TOKEN_BUDGET..=MAX_TOKEN_BUDGET).contains(&tokens)) {
        return Err(CanonicalModelContractError::InvalidField {
            field: RESPONSE_TEXT_FIELD.as_str(),
            reason: INVALID_TOKEN_BUDGET,
        });
    }
    Ok(())
}

fn validate_temperature(value: Option<u16>) -> Result<(), CanonicalModelContractError> {
    if value.is_some_and(|temperature| temperature > MAX_TEMPERATURE_MILLIS) {
        return Err(CanonicalModelContractError::InvalidField {
            field: RESPONSE_TEXT_FIELD.as_str(),
            reason: INVALID_TEMPERATURE,
        });
    }
    Ok(())
}

fn validate_reasoning_effort(
    enabled: bool,
    effort: CanonicalReasoningEffort,
) -> Result<(), CanonicalModelContractError> {
    match (enabled, effort) {
        (false, CanonicalReasoningEffort::None)
        | (
            true,
            CanonicalReasoningEffort::Low
            | CanonicalReasoningEffort::Medium
            | CanonicalReasoningEffort::High,
        ) => Ok(()),
        (false, _) => Err(CanonicalModelContractError::InvalidField {
            field: REASONING_SUMMARY_FIELD.as_str(),
            reason: INVALID_REASONING_DISABLED_EFFORT,
        }),
        (true, CanonicalReasoningEffort::None) => Err(CanonicalModelContractError::InvalidField {
            field: REASONING_SUMMARY_FIELD.as_str(),
            reason: INVALID_REASONING_ENABLED_EFFORT,
        }),
    }
}

fn validate_response_combination(
    items: &[CanonicalModelResponseItem],
) -> Result<(), CanonicalModelContractError> {
    let mut success_count = 0usize;
    let mut refusal_count = 0usize;
    let mut malformed_count = 0usize;
    for item in items {
        match item {
            CanonicalModelResponseItem::AssistantText { .. }
            | CanonicalModelResponseItem::StructuredPayload { .. }
            | CanonicalModelResponseItem::ToolCall { .. } => {
                success_count = success_count.saturating_add(1);
            }
            CanonicalModelResponseItem::Refusal { .. }
            | CanonicalModelResponseItem::Safety { .. } => {
                refusal_count = refusal_count.saturating_add(1);
            }
            CanonicalModelResponseItem::MalformedOutput { .. } => {
                malformed_count = malformed_count.saturating_add(1);
            }
        }
    }
    if refusal_count > 0 && success_count > 0 {
        return Err(CanonicalModelContractError::ImpossibleResponseCombination {
            reason: INVALID_REFUSAL_COMBINATION,
        });
    }
    if malformed_count > 0 && success_count > 0 {
        return Err(CanonicalModelContractError::ImpossibleResponseCombination {
            reason: INVALID_MALFORMED_COMBINATION,
        });
    }
    Ok(())
}

fn require_collection(
    field: crate::validation::FieldName,
    actual: usize,
    max_items: usize,
) -> Result<(), CanonicalModelContractError> {
    if actual == 0 {
        return Err(CanonicalModelContractError::EmptyCollection {
            field: field.as_str(),
        });
    }
    validate_collection_len(field, actual, max_items)
}

fn validate_collection_len(
    field: crate::validation::FieldName,
    actual: usize,
    max_items: usize,
) -> Result<(), CanonicalModelContractError> {
    if actual > max_items {
        return Err(CanonicalModelContractError::CollectionTooLarge {
            field: field.as_str(),
            max_items,
            actual_items: actual,
        });
    }
    Ok(())
}

fn safe_text(
    field: crate::validation::FieldName,
    raw: &str,
) -> Result<alloc::string::String, CanonicalModelContractError> {
    let normalized = crate::validation::normalized_adapter_field(field, raw)?;
    if contains_leak_marker(normalized.as_str(), LEAK_MARKER_RAW_PROVIDER_PAYLOAD)
        || contains_leak_marker(normalized.as_str(), LEAK_MARKER_PROVIDER_PAYLOAD_PREFIX)
    {
        return Err(CanonicalModelContractError::UnsafeText {
            field: field.as_str(),
            reason: CanonicalUnsafeTextReason::RawProviderPayload,
        });
    }
    if contains_leak_marker(normalized.as_str(), LEAK_MARKER_CREDENTIAL_PREFIX)
        || contains_leak_marker(normalized.as_str(), LEAK_MARKER_SECRET_PREFIX)
        || contains_leak_marker(normalized.as_str(), LEAK_MARKER_AUTHORIZATION_PREFIX)
        || contains_leak_marker(normalized.as_str(), LEAK_MARKER_BEARER_PREFIX)
    {
        return Err(CanonicalModelContractError::UnsafeText {
            field: field.as_str(),
            reason: CanonicalUnsafeTextReason::Credential,
        });
    }
    if contains_leak_marker(normalized.as_str(), LEAK_MARKER_HIDDEN_REASONING) {
        return Err(CanonicalModelContractError::UnsafeText {
            field: field.as_str(),
            reason: CanonicalUnsafeTextReason::HiddenReasoning,
        });
    }
    Ok(normalized)
}

fn contains_leak_marker(value: &str, marker: &str) -> bool {
    assert!(!marker.is_empty(), "leak marker must not be empty");
    value
        .as_bytes()
        .windows(marker.len())
        .any(|window| ascii_case_eq(window, marker.as_bytes()))
}

fn ascii_case_eq(left: &[u8], right: &[u8]) -> bool {
    assert!(left.len() == right.len(), "slices must be the same length");
    left.iter()
        .zip(right.iter())
        .all(|(left_byte, right_byte)| left_byte.eq_ignore_ascii_case(right_byte))
}

fn lossy_legacy_request() -> CanonicalModelContractError {
    CanonicalModelContractError::LossyLegacyProjection {
        reason: LOSSY_LEGACY_REQUEST,
    }
}

fn lossy_legacy_response() -> CanonicalModelContractError {
    CanonicalModelContractError::LossyLegacyProjection {
        reason: LOSSY_LEGACY_RESPONSE,
    }
}
