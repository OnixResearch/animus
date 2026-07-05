const FAMILY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("provider_projection_family");
const FEATURE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("provider_projection_feature");
const EVIDENCE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("provider_projection_evidence");
const REASON_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("provider_projection_reason");
const OPENAI_RESPONSES_LABEL: &str = "openai_responses";
const OPENAI_CHAT_LABEL: &str = "openai_chat";
const ANTHROPIC_MESSAGES_LABEL: &str = "anthropic_messages";
const MOCK_LABEL: &str = "mock";
const FEATURE_ROLE: &str = "role";
const FEATURE_CONTENT_PART: &str = "content_part";
const FEATURE_TOOL: &str = "tool";
const FEATURE_STRUCTURED: &str = "structured_output";
const FEATURE_REASONING: &str = "reasoning";
const FEATURE_STREAMING: &str = "streaming";
const FEATURE_METADATA: &str = "metadata";
const LITERAL_FAMILY_PREFIX: &str = "family";
const LITERAL_ROLE_PREFIX: &str = "role";
const LITERAL_TEXT_PREFIX: &str = "text";
const LITERAL_IMAGE_PREFIX: &str = "image";
const LITERAL_TOOL_PREFIX: &str = "tool";
const LITERAL_RESPONSE_PREFIX: &str = "response";
const LITERAL_REASONING_PREFIX: &str = "reasoning";
const LITERAL_STREAM_PREFIX: &str = "stream";
const LITERAL_METADATA_PREFIX: &str = "metadata";
const LITERAL_SEPARATOR: &str = ":";
const LITERAL_KEY_VALUE_SEPARATOR: &str = "=";

/// Provider family covered by projection conformance fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderProjectionFamily {
    /// `OpenAI` Responses-style projection.
    OpenAiResponses,
    /// `OpenAI`-compatible Chat Completions or local `HTTP` projection.
    OpenAiChat,
    /// Anthropic Messages-style projection.
    AnthropicMessages,
    /// Deterministic mock/local fixture projection.
    Mock,
}

impl ProviderProjectionFamily {
    /// Parses a provider projection family label.
    ///
    /// # Errors
    ///
    /// Returns [`ProviderProjectionError`] when the label is empty or unsupported.
    pub fn parse(raw: &str) -> Result<Self, ProviderProjectionError> {
        let normalized = crate::validation::normalized_adapter_field(FAMILY_FIELD, raw)?;
        match normalized.as_str() {
            OPENAI_RESPONSES_LABEL => Ok(Self::OpenAiResponses),
            OPENAI_CHAT_LABEL => Ok(Self::OpenAiChat),
            ANTHROPIC_MESSAGES_LABEL => Ok(Self::AnthropicMessages),
            MOCK_LABEL => Ok(Self::Mock),
            _unsupported => Err(ProviderProjectionError::UnsupportedFamily { value: normalized }),
        }
    }

    /// Returns the stable family label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OpenAiResponses => OPENAI_RESPONSES_LABEL,
            Self::OpenAiChat => OPENAI_CHAT_LABEL,
            Self::AnthropicMessages => ANTHROPIC_MESSAGES_LABEL,
            Self::Mock => MOCK_LABEL,
        }
    }
}

/// Feature support claim for provider projection fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderProjectionFeatureSupport {
    /// The feature is preserved by the provider projection.
    Preserved,
    /// The feature is unsupported by the provider projection.
    Unsupported,
}

impl ProviderProjectionFeatureSupport {
    /// Returns true when the feature is preserved.
    #[must_use]
    pub const fn is_preserved(self) -> bool {
        matches!(self, Self::Preserved)
    }
}

/// Provider-family projection support claims for conformance fixtures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderProjectionSupport {
    /// Provider family.
    pub family: ProviderProjectionFamily,
    /// Developer message support.
    pub developer_role: ProviderProjectionFeatureSupport,
    /// Descriptor-backed image part support.
    pub images: ProviderProjectionFeatureSupport,
    /// Tool declaration support.
    pub tools: ProviderProjectionFeatureSupport,
    /// `JSON` object response-format support.
    pub json_object: ProviderProjectionFeatureSupport,
    /// `JSON` schema response-format support.
    pub json_schema: ProviderProjectionFeatureSupport,
    /// Reasoning control support.
    pub reasoning: ProviderProjectionFeatureSupport,
    /// Streaming support.
    pub streaming: ProviderProjectionFeatureSupport,
    /// Safe provenance metadata support.
    pub metadata: ProviderProjectionFeatureSupport,
}

/// Provider-native request projection fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderRequestProjection {
    family: ProviderProjectionFamily,
    literal_parts: alloc::vec::Vec<alloc::string::String>,
}

impl ProviderRequestProjection {
    /// Returns the provider family.
    #[must_use]
    pub const fn family(&self) -> ProviderProjectionFamily {
        self.family
    }

    /// Returns reviewed literal projection parts.
    #[must_use]
    pub fn literal_parts(&self) -> &[alloc::string::String] {
        self.literal_parts.as_slice()
    }
}

/// Provider-native response fixture before canonical normalization.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderResponseFixtureDraft<'a> {
    /// Provider family label.
    pub family: &'a str,
    /// Optional assistant text.
    pub text: Option<&'a str>,
    /// Optional structured payload label.
    pub structured_payload: Option<&'a str>,
    /// Optional tool call identifier.
    pub tool_call_id: Option<&'a str>,
    /// Optional tool name.
    pub tool_name: Option<&'a str>,
    /// Optional tool arguments label.
    pub tool_arguments: Option<&'a str>,
    /// Optional refusal reason.
    pub refusal: Option<&'a str>,
    /// Optional safety category.
    pub safety: Option<&'a str>,
    /// Optional malformed-output diagnostic label.
    pub malformed_output: Option<&'a str>,
    /// Optional usage metadata.
    pub usage: Option<super::CanonicalUsageDraft>,
    /// Optional finish reason label.
    pub finish_reason: Option<&'a str>,
    /// Optional raw provider payload leak sentinel.
    pub raw_provider_payload: Option<&'a str>,
    /// Optional hidden reasoning leak sentinel.
    pub hidden_reasoning: Option<&'a str>,
}

/// Conformance index case status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderProjectionIndexStatus {
    /// Covered by fixture evidence.
    Covered,
    /// Unsupported with an explicit reason.
    Unsupported,
    /// Not applicable with an explicit reason.
    NotApplicable,
}

/// Borrowed conformance index entry before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderProjectionIndexEntryDraft<'a> {
    /// Provider family label.
    pub family: &'a str,
    /// Feature/provider case label.
    pub feature: &'a str,
    /// Case status.
    pub status: ProviderProjectionIndexStatus,
    /// Fixture evidence label for covered cases.
    pub evidence: Option<&'a str>,
    /// Reason label for unsupported or not-applicable cases.
    pub reason: Option<&'a str>,
}

/// Validated conformance index entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderProjectionIndexEntry {
    family: ProviderProjectionFamily,
    feature: alloc::string::String,
    status: ProviderProjectionIndexStatus,
    evidence: Option<alloc::string::String>,
    reason: Option<alloc::string::String>,
}

impl ProviderProjectionIndexEntry {
    /// Returns the provider family.
    #[must_use]
    pub const fn family(&self) -> ProviderProjectionFamily {
        self.family
    }

    /// Returns the feature label.
    #[must_use]
    pub fn feature(&self) -> &str {
        self.feature.as_str()
    }

    /// Returns the case status.
    #[must_use]
    pub const fn status(&self) -> ProviderProjectionIndexStatus {
        self.status
    }
}

/// Provider projection conformance error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderProjectionError {
    /// Shared adapter scalar validation failed.
    AdapterContract {
        /// Underlying adapter error.
        source: crate::AdapterContractError,
    },
    /// Canonical model contract validation failed.
    CanonicalContract {
        /// Underlying canonical contract error.
        source: super::CanonicalModelContractError,
    },
    /// Model-stream fixture parsing or validation failed.
    StreamFixture {
        /// Underlying stream error.
        source: super::wire::failure::Error,
    },
    /// Provider family label is unsupported.
    UnsupportedFamily {
        /// Unsupported value.
        value: alloc::string::String,
    },
    /// Request projection would drop or misrepresent a canonical feature.
    LossyProjection {
        /// Safe feature label.
        feature: &'static str,
    },
    /// Literal fixture comparison failed.
    LiteralMismatch {
        /// Expected reviewed literal fixture.
        expected: alloc::vec::Vec<alloc::string::String>,
        /// Actual generated literal fixture.
        actual: alloc::vec::Vec<alloc::string::String>,
    },
    /// Conformance index entry is missing required evidence or reason.
    MissingIndexField {
        /// Missing field label.
        field: &'static str,
    },
}

impl From<crate::AdapterContractError> for ProviderProjectionError {
    fn from(source: crate::AdapterContractError) -> Self {
        Self::AdapterContract { source }
    }
}

impl From<super::CanonicalModelContractError> for ProviderProjectionError {
    fn from(source: super::CanonicalModelContractError) -> Self {
        Self::CanonicalContract { source }
    }
}

impl From<super::wire::failure::Error> for ProviderProjectionError {
    fn from(source: super::wire::failure::Error) -> Self {
        Self::StreamFixture { source }
    }
}

/// Projects a canonical request into reviewed provider-family literal fixture parts.
///
/// # Errors
///
/// Returns [`ProviderProjectionError`] when the selected provider support would lose canonical semantics.
pub fn project_provider_request_fixture(
    request: &super::CanonicalModelRequest,
    support: ProviderProjectionSupport,
) -> Result<ProviderRequestProjection, ProviderProjectionError> {
    reject_unsupported_request_features(request, support)?;
    let mut literal_parts = alloc::vec![literal(LITERAL_FAMILY_PREFIX, support.family.as_str())];
    for message in request.messages() {
        literal_parts.push(literal(LITERAL_ROLE_PREFIX, message.role().as_str()));
        for part in message.content() {
            if let Some(text) = part.text() {
                literal_parts.push(literal(LITERAL_TEXT_PREFIX, text));
            } else {
                literal_parts.push(literal(LITERAL_IMAGE_PREFIX, "descriptor"));
            }
        }
    }
    for tool in request.tools() {
        literal_parts.push(literal(LITERAL_TOOL_PREFIX, tool.name()));
    }
    push_response_format(request.response_format(), &mut literal_parts);
    if request.reasoning().enabled() {
        literal_parts.push(literal(
            LITERAL_REASONING_PREFIX,
            reasoning_label(request.reasoning().effort()),
        ));
    }
    if request.output().stream() {
        literal_parts.push(literal(LITERAL_STREAM_PREFIX, "true"));
    }
    for metadata in request.provenance() {
        literal_parts.push(metadata_literal(metadata));
    }
    Ok(ProviderRequestProjection {
        family: support.family,
        literal_parts,
    })
}

/// Compares a provider request projection against reviewed literal fixture data.
///
/// # Errors
///
/// Returns [`ProviderProjectionError`] when projection is lossy or differs from expected literal data.
pub fn assert_provider_request_fixture(
    request: &super::CanonicalModelRequest,
    support: ProviderProjectionSupport,
    expected: &[&str],
) -> Result<ProviderRequestProjection, ProviderProjectionError> {
    let projection = project_provider_request_fixture(request, support)?;
    let expected: alloc::vec::Vec<alloc::string::String> = expected
        .iter()
        .map(|part| alloc::string::String::from(*part))
        .collect();
    if projection.literal_parts() == expected.as_slice() {
        return Ok(projection);
    }
    Err(ProviderProjectionError::LiteralMismatch {
        expected,
        actual: projection.literal_parts().to_vec(),
    })
}

/// Normalizes a provider response fixture into the canonical response envelope.
///
/// # Errors
///
/// Returns [`ProviderProjectionError`] when response semantics are malformed, leaky, or impossible.
pub fn normalize_provider_response_fixture(
    draft: ProviderResponseFixtureDraft<'_>,
) -> Result<super::CanonicalModelResponseEnvelope, ProviderProjectionError> {
    let _family = ProviderProjectionFamily::parse(draft.family)?;
    let mut items = alloc::vec::Vec::new();
    if let Some(label) = draft.raw_provider_payload {
        items.push(super::CanonicalModelResponseItemDraft::RawProviderPayload { label });
    }
    if let Some(label) = draft.hidden_reasoning {
        items.push(super::CanonicalModelResponseItemDraft::HiddenReasoning { label });
    }
    if let Some(text) = draft.text {
        items.push(super::CanonicalModelResponseItemDraft::AssistantText { text });
    }
    if let Some(payload_label) = draft.structured_payload {
        items.push(super::CanonicalModelResponseItemDraft::StructuredPayload {
            schema_label: "provider.schema",
            payload_label,
        });
    }
    if let (Some(call_id), Some(tool_name), Some(arguments_label)) =
        (draft.tool_call_id, draft.tool_name, draft.tool_arguments)
    {
        items.push(super::CanonicalModelResponseItemDraft::ToolCall {
            call_id,
            tool_name,
            arguments_label,
        });
    }
    if let Some(reason) = draft.refusal {
        items.push(super::CanonicalModelResponseItemDraft::Refusal { reason });
    }
    if let Some(category) = draft.safety {
        items.push(super::CanonicalModelResponseItemDraft::Safety { category });
    }
    if let Some(diagnostic) = draft.malformed_output {
        items.push(super::CanonicalModelResponseItemDraft::MalformedOutput { diagnostic });
    }
    super::CanonicalModelResponseEnvelope::from_draft(super::CanonicalModelResponseEnvelopeDraft {
        contract_identity: super::CANONICAL_MODEL_RESPONSE_CONTRACT_IDENTITY,
        items: items.as_slice(),
        usage: draft.usage,
        finish_reason: draft.finish_reason,
        diagnostics: &[],
        provenance: &[],
    })
    .map_err(ProviderProjectionError::from)
}

/// Validates provider streaming fixture chunks through the production model-stream parser.
///
/// # Errors
///
/// Returns [`ProviderProjectionError`] when the stream fixture is malformed or frame kinds differ.
pub fn assert_provider_stream_fixture(
    chunks: &[&str],
    expected_kinds: &[super::wire::parts::Kind],
) -> Result<alloc::vec::Vec<super::wire::types::Frame>, ProviderProjectionError> {
    let frames = super::wire::fixture::parse(chunks)?;
    let actual_kinds: alloc::vec::Vec<_> =
        frames.iter().map(super::wire::types::Frame::kind).collect();
    if actual_kinds.as_slice() == expected_kinds {
        Ok(frames)
    } else {
        Err(ProviderProjectionError::LossyProjection {
            feature: FEATURE_STREAMING,
        })
    }
}

/// Validates a provider projection conformance index.
///
/// # Errors
///
/// Returns [`ProviderProjectionError`] when covered cases lack evidence or non-covered cases lack reasons.
pub fn validate_provider_projection_index(
    drafts: &[ProviderProjectionIndexEntryDraft<'_>],
) -> Result<alloc::vec::Vec<ProviderProjectionIndexEntry>, ProviderProjectionError> {
    drafts.iter().map(validate_index_entry).collect()
}

fn validate_index_entry(
    draft: &ProviderProjectionIndexEntryDraft<'_>,
) -> Result<ProviderProjectionIndexEntry, ProviderProjectionError> {
    let family = ProviderProjectionFamily::parse(draft.family)?;
    let feature = crate::validation::normalized_adapter_field(FEATURE_FIELD, draft.feature)?;
    let evidence = draft
        .evidence
        .map(|value| crate::validation::normalized_adapter_field(EVIDENCE_FIELD, value))
        .transpose()?;
    let reason = draft
        .reason
        .map(|value| crate::validation::normalized_adapter_field(REASON_FIELD, value))
        .transpose()?;
    match draft.status {
        ProviderProjectionIndexStatus::Covered if evidence.is_none() => {
            Err(ProviderProjectionError::MissingIndexField {
                field: EVIDENCE_FIELD.as_str(),
            })
        }
        ProviderProjectionIndexStatus::Unsupported
        | ProviderProjectionIndexStatus::NotApplicable
            if reason.is_none() =>
        {
            Err(ProviderProjectionError::MissingIndexField {
                field: REASON_FIELD.as_str(),
            })
        }
        _valid => Ok(ProviderProjectionIndexEntry {
            family,
            feature,
            status: draft.status,
            evidence,
            reason,
        }),
    }
}

fn reject_unsupported_request_features(
    request: &super::CanonicalModelRequest,
    support: ProviderProjectionSupport,
) -> Result<(), ProviderProjectionError> {
    for message in request.messages() {
        if message.role() == super::CanonicalModelRole::Developer
            && !support.developer_role.is_preserved()
        {
            return Err(lossy(FEATURE_ROLE));
        }
        for part in message.content() {
            if !part.is_text() && !support.images.is_preserved() {
                return Err(lossy(FEATURE_CONTENT_PART));
            }
        }
    }
    if !request.tools().is_empty() && !support.tools.is_preserved() {
        return Err(lossy(FEATURE_TOOL));
    }
    match request.response_format() {
        super::CanonicalResponseFormatIntent::Text => {}
        super::CanonicalResponseFormatIntent::JsonObject => {
            if !support.json_object.is_preserved() {
                return Err(lossy(FEATURE_STRUCTURED));
            }
        }
        super::CanonicalResponseFormatIntent::JsonSchema { .. } => {
            if !support.json_schema.is_preserved() {
                return Err(lossy(FEATURE_STRUCTURED));
            }
        }
    }
    if request.reasoning().enabled() && !support.reasoning.is_preserved() {
        return Err(lossy(FEATURE_REASONING));
    }
    if request.output().stream() && !support.streaming.is_preserved() {
        return Err(lossy(FEATURE_STREAMING));
    }
    if !request.provenance().is_empty() && !support.metadata.is_preserved() {
        return Err(lossy(FEATURE_METADATA));
    }
    Ok(())
}

fn push_response_format(
    format: &super::CanonicalResponseFormatIntent,
    literal_parts: &mut alloc::vec::Vec<alloc::string::String>,
) {
    match format {
        super::CanonicalResponseFormatIntent::Text => {
            literal_parts.push(literal(LITERAL_RESPONSE_PREFIX, "text"));
        }
        super::CanonicalResponseFormatIntent::JsonObject => {
            literal_parts.push(literal(LITERAL_RESPONSE_PREFIX, "json_object"));
        }
        super::CanonicalResponseFormatIntent::JsonSchema { schema_label } => {
            literal_parts.push(literal(LITERAL_RESPONSE_PREFIX, schema_label));
        }
    }
}

fn reasoning_label(effort: super::CanonicalReasoningEffort) -> &'static str {
    match effort {
        super::CanonicalReasoningEffort::None => "none",
        super::CanonicalReasoningEffort::Low => "low",
        super::CanonicalReasoningEffort::Medium => "medium",
        super::CanonicalReasoningEffort::High => "high",
    }
}

fn literal(prefix: &str, value: &str) -> alloc::string::String {
    let mut output = alloc::string::String::from(prefix);
    output.push_str(LITERAL_SEPARATOR);
    output.push_str(value);
    output
}

fn metadata_literal(metadata: &super::CanonicalSafeMetadata) -> alloc::string::String {
    let mut value = alloc::string::String::from(metadata.key());
    value.push_str(LITERAL_KEY_VALUE_SEPARATOR);
    value.push_str(metadata.value());
    literal(LITERAL_METADATA_PREFIX, value.as_str())
}

const fn lossy(feature: &'static str) -> ProviderProjectionError {
    ProviderProjectionError::LossyProjection { feature }
}
