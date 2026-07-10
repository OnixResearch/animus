//! Host-owned concrete provider dispatch planning and fixture rails.
//!
//! This module is intentionally std-shell scoped. It plans one concrete
//! `OpenAI` Responses fixture dispatch after the host has selected a registry
//! entry, negotiated canonical request capabilities, resolved credential and
//! service readiness, and supplied deadline/cancellation metadata. It does not
//! discover providers, read credentials, create network clients, perform retry
//! policy, or make live-provider claims.

// Provider dispatch DTOs mirror host readiness state and canonical fixture field names.
#![allow(clippy::struct_excessive_bools, clippy::struct_field_names)]

const HASH_DOMAIN: &str = "animus.provider-dispatch.v1";
const HASH_SEPARATOR: &str = "|";
const HASH_PREFIX: &str = "BLAKE3";
const OPENAI_RESPONSES_FAMILY: &str = "openai_responses";
const CREDENTIAL_NOT_REQUIRED: &str = "not_required";
const RECEIPT_STATUS_SUCCESS: &str = "success";
const RECEIPT_STATUS_REJECTED: &str = "rejected";
const FEATURE_IMAGE_CONTENT: &str = "image_content";
const FEATURE_REASONING: &str = "reasoning";
const FEATURE_STREAMING: &str = "streaming";
const FEATURE_OUTPUT_BUDGET: &str = "output_budget";
const FIELD_MODEL: &str = "model";
const FIELD_ROLE: &str = "role";
const FIELD_TEXT: &str = "text";
const FIELD_IMAGE: &str = "image";
const FIELD_TOOL: &str = "tool";
const FIELD_RESPONSE_FORMAT: &str = "response_format";
const FIELD_REASONING: &str = "reasoning";
const FIELD_STREAM: &str = "stream";
const FIELD_MAX_OUTPUT_TOKENS: &str = "max_output_tokens";
const FIELD_TEMPERATURE_MILLIS: &str = "temperature_millis";
const FIELD_METADATA: &str = "metadata";
const FIELD_FAMILY: &str = "family";
const FIELD_SEPARATOR: &str = ":";
const METADATA_SEPARATOR: &str = "=";
const IMAGE_SEPARATOR: &str = ":";
const TOOL_SEPARATOR: &str = ":";
const RESPONSE_FORMAT_TEXT: &str = "text";
const RESPONSE_FORMAT_JSON_OBJECT: &str = "json_object";
const RESPONSE_FORMAT_JSON_SCHEMA: &str = "json_schema";
const REASONING_NONE: &str = "none";
const REASONING_LOW: &str = "low";
const REASONING_MEDIUM: &str = "medium";
const REASONING_HIGH: &str = "high";
const BOOL_TRUE: &str = "true";
const BOOL_FALSE: &str = "false";
const CONSTRUCTOR_OPENAI_RESPONSES: &str = "build_openai_responses_dispatch_request";
const EXPECTED_OPENAI_RESPONSES_CONSTRUCTOR_COUNT: u64 = 1;
const MAX_PROVIDER_DISPATCH_CONSTRUCTORS: u64 = 16;
const MATRIX_REASON_EMPTY: &str = "constructor inventory is empty";
const MATRIX_REASON_COUNT: &str = "constructor inventory count drifted";
const MATRIX_REASON_FIELD: &str = "constructor omitted required shared request field";
const LABEL_EMPTY_REASON: &str = "provider dispatch label must be non-empty";
const LABEL_TOO_LONG_REASON: &str = "provider dispatch label exceeds safe length";
const MAX_SAFE_LABEL_CHARS: usize = animus_adapters::MAX_ADAPTER_FIELD_CHARS;
const REDACTION_PROVIDER_PAYLOAD: &str = "provider_payload:redacted";
const REDACTION_HIDDEN_REASONING: &str = "hidden_reasoning:redacted";
const REDACTION_CREDENTIAL: &str = "credential:redacted";
const REDACTION_TRANSPORT: &str = "transport:redacted";
const LEAK_RAW_PROVIDER_PAYLOAD: &str = "raw_provider_payload";
const LEAK_PROVIDER_PAYLOAD: &str = "provider_payload:";
const LEAK_HIDDEN_REASONING: &str = "hidden_reasoning";
const LEAK_CREDENTIAL: &str = "credential:";
const LEAK_SECRET: &str = "secret:";
const LEAK_AUTHORIZATION: &str = "authorization:";
const LEAK_BEARER: &str = "bearer ";
const LEAK_PATH_HOME: &str = "/home/";
const LEAK_PATH_TMP: &str = "/tmp/";
const UNSAFE_RECEIPT_POLICY: &str = "receipt policy requested raw provider evidence";
const UNSAFE_RECEIPT_FIELD: &str = "receipt contains unsafe material";
const UNSAFE_REDACTION_MARKER: &str = "receipt redaction marker is not whitelisted";

/// Host-side credential readiness for concrete provider dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderDispatchCredentialState {
    /// The selected provider/model does not require a credential.
    NotRequired,
    /// A shell-owned credential handle is ready; only its safe source label is retained.
    Ready {
        /// Safe credential source label, such as `env:OPENAI_API_KEY`.
        source_label: String,
    },
    /// A required credential was absent.
    MissingRequired,
}

impl ProviderDispatchCredentialState {
    /// Builds a ready state from an opaque shell credential handle.
    #[must_use]
    pub fn from_handle(handle: &super::CredentialHandle) -> Self {
        Self::Ready {
            source_label: String::from(handle.source_label()),
        }
    }
}

/// Host-injected concrete dispatch service readiness.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderDispatchServiceState {
    /// Provider backend is explicitly disabled by host policy.
    DisabledBackend,
    /// No service implementation was injected.
    Missing,
    /// A service exists but is unavailable.
    Unavailable,
    /// The service is ready for fixture dispatch.
    Available,
}

/// Host entitlement readiness for the selected provider/model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderDispatchEntitlementState {
    /// Entitlement was granted by the host.
    Granted,
    /// Entitlement failed or was denied.
    Denied,
}

/// Deadline readiness for concrete dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderDispatchDeadlineState {
    /// A safe deadline label was supplied by the host.
    Ready {
        /// Safe host deadline label.
        label: String,
    },
    /// Deadline metadata was omitted.
    Missing,
    /// Deadline expired before provider contact.
    Expired,
    /// Timeout fired before provider contact.
    TimedOut,
}

/// Cancellation readiness for concrete dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderDispatchCancellationState {
    /// A safe cancellation-token label was supplied by the host.
    Ready {
        /// Safe host cancellation token label.
        label: String,
    },
    /// Cancellation metadata was omitted.
    Missing,
    /// The request was already cancelled before provider contact.
    Cancelled,
}

/// Provider fallback posture for known provider-qualified targets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderDispatchFallbackState {
    /// The host did not request fallback.
    NotRequested,
    /// Fallback was requested but policy denies it for a known provider prefix.
    RequestedButDenied,
    /// Fallback was explicitly allowed by host policy.
    RequestedAndAllowed,
}

/// Receipt evidence policy for concrete provider dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderDispatchReceiptPolicy {
    /// Durable evidence may contain only safe summaries, hashes, and redaction markers.
    SafeSummaryOnly,
    /// Raw provider evidence was requested and must fail closed.
    RawProviderPayloadRequested,
}

/// Safe status class for concrete provider dispatch receipts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderDispatchOutcomeStatus {
    /// Fixture dispatch normalized a successful provider response.
    Success,
    /// Dispatch or normalization was rejected before durable evidence.
    Rejected,
}

impl ProviderDispatchOutcomeStatus {
    /// Returns the stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Success => RECEIPT_STATUS_SUCCESS,
            Self::Rejected => RECEIPT_STATUS_REJECTED,
        }
    }
}

/// Error raised by concrete provider dispatch planning, projection, or receipt validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderDispatchError {
    /// The selected provider family is not implemented by this concrete adapter.
    UnsupportedProviderFamily {
        /// Provider `API` family.
        api_kind: super::ApiKind,
    },
    /// A selected model capability cannot satisfy the canonical request.
    UnsupportedFeature {
        /// Stable unsupported feature label.
        feature: &'static str,
    },
    /// Required credential readiness failed before provider contact.
    MissingCredential,
    /// Selected backend is disabled by host policy.
    DisabledBackend,
    /// No concrete provider service was injected.
    MissingService,
    /// Injected provider service is unavailable.
    ServiceUnavailable,
    /// Provider/model entitlement was denied.
    EntitlementDenied,
    /// Fallback was denied for a known provider prefix.
    FallbackDenied,
    /// Deadline metadata was missing.
    MissingDeadline,
    /// Deadline expired or timeout fired before provider contact.
    Timeout,
    /// Cancellation metadata was missing.
    MissingCancellation,
    /// Request was already cancelled before provider contact.
    Cancelled,
    /// Receipt policy requested unsafe raw evidence.
    UnsafeReceiptPolicy {
        /// Safe diagnostic reason.
        reason: &'static str,
    },
    /// A provider request projection fixture failed.
    RequestProjection {
        /// Underlying fixture error.
        source: animus_adapters::ProviderProjectionError,
    },
    /// A provider response fixture failed normalization.
    MalformedResponse {
        /// Underlying fixture error.
        source: animus_adapters::ProviderProjectionError,
    },
    /// Shared-field or constructor inventory parity failed.
    ConstructorInventory {
        /// Safe diagnostic reason.
        reason: &'static str,
    },
    /// Safe label validation failed.
    UnsafeLabel {
        /// Safe diagnostic reason.
        reason: &'static str,
    },
    /// Dispatch receipt validation rejected unsafe material.
    ReceiptLeak {
        /// Safe diagnostic reason.
        reason: &'static str,
    },
}

impl std::fmt::Display for ProviderDispatchError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedProviderFamily { api_kind } => {
                write!(
                    formatter,
                    "unsupported provider family {}",
                    api_kind.as_str()
                )
            }
            Self::UnsupportedFeature { feature } => {
                write!(formatter, "unsupported provider dispatch feature {feature}")
            }
            Self::MissingCredential => formatter.write_str("provider credential missing"),
            Self::DisabledBackend => formatter.write_str("provider backend disabled"),
            Self::MissingService => formatter.write_str("provider dispatch service missing"),
            Self::ServiceUnavailable => {
                formatter.write_str("provider dispatch service unavailable")
            }
            Self::EntitlementDenied => formatter.write_str("provider entitlement denied"),
            Self::FallbackDenied => formatter.write_str("provider fallback denied"),
            Self::MissingDeadline => formatter.write_str("provider deadline metadata missing"),
            Self::Timeout => formatter.write_str("provider dispatch timed out before contact"),
            Self::MissingCancellation => {
                formatter.write_str("provider cancellation metadata missing")
            }
            Self::Cancelled => formatter.write_str("provider dispatch cancelled before contact"),
            Self::UnsafeReceiptPolicy { reason }
            | Self::ConstructorInventory { reason }
            | Self::UnsafeLabel { reason }
            | Self::ReceiptLeak { reason } => formatter.write_str(reason),
            Self::RequestProjection { source } => {
                write!(formatter, "provider request projection failed: {source:?}")
            }
            Self::MalformedResponse { source } => {
                write!(
                    formatter,
                    "provider response normalization failed: {source:?}"
                )
            }
        }
    }
}

impl std::error::Error for ProviderDispatchError {}

/// Borrowed input for host-owned concrete provider dispatch planning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDispatchInput<'a> {
    /// Selected provider/model metadata from the shell-owned registry.
    pub selected_model: &'a super::SelectedModel,
    /// Already-admitted canonical model request.
    pub canonical_request: &'a animus_adapters::CanonicalModelRequest,
    /// Reviewed provider projection fixture literals for the canonical request.
    pub reviewed_projection_fixture: &'a [&'a str],
    /// Credential readiness supplied by the host shell.
    pub credential: ProviderDispatchCredentialState,
    /// Concrete provider service readiness supplied by the host shell.
    pub service: ProviderDispatchServiceState,
    /// Provider entitlement readiness supplied by the host shell.
    pub entitlement: ProviderDispatchEntitlementState,
    /// Deadline metadata supplied by the host shell.
    pub deadline: ProviderDispatchDeadlineState,
    /// Cancellation metadata supplied by the host shell.
    pub cancellation: ProviderDispatchCancellationState,
    /// Known-provider fallback posture supplied by host policy.
    pub fallback: ProviderDispatchFallbackState,
    /// Receipt evidence policy.
    pub receipt_policy: ProviderDispatchReceiptPolicy,
}

/// Provider-native `OpenAI` Responses content part fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OpenAiResponsesContentPart {
    /// Text content part.
    Text {
        /// Safe text body.
        text: String,
    },
    /// Descriptor-backed image content part.
    ImageDescriptor {
        /// Safe image descriptor id.
        descriptor_id: String,
        /// Safe media type label.
        media_type: String,
        /// Safe content hash.
        content_hash: String,
    },
}

/// Provider-native `OpenAI` Responses message fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiResponsesMessage {
    role: String,
    content: Vec<OpenAiResponsesContentPart>,
    tool_call_id: Option<String>,
}

impl OpenAiResponsesMessage {
    /// Returns the provider-native role label.
    #[must_use]
    pub fn role(&self) -> &str {
        self.role.as_str()
    }

    /// Returns provider-native content parts.
    #[must_use]
    pub fn content(&self) -> &[OpenAiResponsesContentPart] {
        self.content.as_slice()
    }

    /// Returns tool-call correlation for tool messages.
    #[must_use]
    pub fn tool_call_id(&self) -> Option<&str> {
        self.tool_call_id.as_deref()
    }
}

/// Provider-native `OpenAI` Responses tool declaration fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiResponsesTool {
    name: String,
    description: String,
    input_schema_label: String,
}

impl OpenAiResponsesTool {
    /// Returns provider-native tool name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns safe tool description.
    #[must_use]
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns safe input-schema label.
    #[must_use]
    pub fn input_schema_label(&self) -> &str {
        self.input_schema_label.as_str()
    }
}

/// Provider-native `OpenAI` Responses safe metadata fixture.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiResponsesMetadata {
    key: String,
    value: String,
}

impl OpenAiResponsesMetadata {
    /// Returns safe metadata key.
    #[must_use]
    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    /// Returns safe metadata value.
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.as_str()
    }
}

/// Concrete `OpenAI` Responses request `DTO` used by fixture dispatch.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiResponsesDispatchRequest {
    model: String,
    messages: Vec<OpenAiResponsesMessage>,
    tools: Vec<OpenAiResponsesTool>,
    response_format: String,
    reasoning_effort: String,
    stream: bool,
    max_output_tokens: Option<u32>,
    temperature_millis: Option<u16>,
    metadata: Vec<OpenAiResponsesMetadata>,
}

impl OpenAiResponsesDispatchRequest {
    /// Returns selected model id.
    #[must_use]
    pub fn model(&self) -> &str {
        self.model.as_str()
    }

    /// Returns projected provider-native messages.
    #[must_use]
    pub fn messages(&self) -> &[OpenAiResponsesMessage] {
        self.messages.as_slice()
    }

    /// Returns projected provider-native tools.
    #[must_use]
    pub fn tools(&self) -> &[OpenAiResponsesTool] {
        self.tools.as_slice()
    }

    /// Returns provider-native response-format fixture label.
    #[must_use]
    pub fn response_format(&self) -> &str {
        self.response_format.as_str()
    }

    /// Returns provider-native reasoning effort fixture label.
    #[must_use]
    pub fn reasoning_effort(&self) -> &str {
        self.reasoning_effort.as_str()
    }

    /// Returns whether streaming is requested.
    #[must_use]
    pub const fn stream(&self) -> bool {
        self.stream
    }

    /// Returns optional output-token budget.
    #[must_use]
    pub const fn max_output_tokens(&self) -> Option<u32> {
        self.max_output_tokens
    }

    /// Returns optional deterministic temperature in thousandths.
    #[must_use]
    pub const fn temperature_millis(&self) -> Option<u16> {
        self.temperature_millis
    }

    /// Returns projected provider-native safe metadata.
    #[must_use]
    pub fn metadata(&self) -> &[OpenAiResponsesMetadata] {
        self.metadata.as_slice()
    }
}

/// Shared request-field preservation matrix for concrete provider construction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderDispatchSharedFieldMatrix {
    /// Model identity is preserved.
    pub model_identity: bool,
    /// Message count and roles are preserved.
    pub messages: bool,
    /// Content parts are preserved.
    pub content_parts: bool,
    /// Tool declarations are preserved.
    pub tools: bool,
    /// Response-format intent is preserved.
    pub response_format: bool,
    /// Reasoning controls are preserved.
    pub reasoning_controls: bool,
    /// Stream intent is preserved.
    pub stream_intent: bool,
    /// Output budget is preserved.
    pub output_budget: bool,
    /// Sampling controls are preserved.
    pub sampling_controls: bool,
    /// Safe metadata is preserved.
    pub safe_metadata: bool,
}

impl ProviderDispatchSharedFieldMatrix {
    /// Returns a matrix where every required shared field is preserved.
    #[must_use]
    pub const fn all_preserved() -> Self {
        Self {
            model_identity: true,
            messages: true,
            content_parts: true,
            tools: true,
            response_format: true,
            reasoning_controls: true,
            stream_intent: true,
            output_budget: true,
            sampling_controls: true,
            safe_metadata: true,
        }
    }

    /// Returns true when every required shared field is preserved.
    #[must_use]
    pub const fn is_complete(self) -> bool {
        self.model_identity
            && self.messages
            && self.content_parts
            && self.tools
            && self.response_format
            && self.reasoning_controls
            && self.stream_intent
            && self.output_budget
            && self.sampling_controls
            && self.safe_metadata
    }
}

/// Inventory entry for one reviewed concrete provider request constructor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDispatchConstructorInventoryEntry {
    /// Constructor name.
    pub constructor: String,
    /// Fields preserved by this constructor.
    pub shared_fields: ProviderDispatchSharedFieldMatrix,
}

/// Safe provider dispatch plan handed to a host-owned service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDispatchPlan {
    provider_id: String,
    model_id: String,
    api_kind: super::ApiKind,
    endpoint: String,
    request: OpenAiResponsesDispatchRequest,
    request_projection_identity: String,
    request_payload_identity: String,
    credential_source_label: String,
    deadline_label: String,
    cancellation_label: String,
    receipt_policy: ProviderDispatchReceiptPolicy,
}

impl ProviderDispatchPlan {
    /// Returns selected provider id.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    /// Returns selected model id.
    #[must_use]
    pub fn model_id(&self) -> &str {
        self.model_id.as_str()
    }

    /// Returns provider `API` family.
    #[must_use]
    pub const fn api_kind(&self) -> super::ApiKind {
        self.api_kind
    }

    /// Returns safe endpoint identity.
    #[must_use]
    pub fn endpoint(&self) -> &str {
        self.endpoint.as_str()
    }

    /// Returns provider-native fixture request DTO.
    #[must_use]
    pub const fn request(&self) -> &OpenAiResponsesDispatchRequest {
        &self.request
    }

    /// Returns `BLAKE3` identity for provider projection fixture literals.
    #[must_use]
    pub fn request_projection_identity(&self) -> &str {
        self.request_projection_identity.as_str()
    }

    /// Returns `BLAKE3` identity for concrete provider request fixture literals.
    #[must_use]
    pub fn request_payload_identity(&self) -> &str {
        self.request_payload_identity.as_str()
    }

    /// Returns safe credential source label.
    #[must_use]
    pub fn credential_source_label(&self) -> &str {
        self.credential_source_label.as_str()
    }

    /// Returns safe deadline label.
    #[must_use]
    pub fn deadline_label(&self) -> &str {
        self.deadline_label.as_str()
    }

    /// Returns safe cancellation label.
    #[must_use]
    pub fn cancellation_label(&self) -> &str {
        self.cancellation_label.as_str()
    }

    /// Returns receipt evidence policy.
    #[must_use]
    pub const fn receipt_policy(&self) -> ProviderDispatchReceiptPolicy {
        self.receipt_policy
    }
}

/// Safe provider dispatch receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDispatchReceipt {
    /// Safe provider id.
    pub provider_id: String,
    /// Safe model id.
    pub model_id: String,
    /// Provider family label.
    pub provider_family: String,
    /// Safe endpoint identity.
    pub endpoint_identity: String,
    /// `BLAKE3` request identity.
    pub request_identity: String,
    /// `BLAKE3` response identity.
    pub response_identity: String,
    /// Safe outcome status.
    pub status: ProviderDispatchOutcomeStatus,
    /// Safe finish reason label.
    pub finish_reason: String,
    /// Safe credential source label.
    pub credential_source_label: String,
    /// Allowed redaction markers.
    pub redactions: Vec<String>,
}

impl ProviderDispatchReceipt {
    /// Returns safe provider id.
    #[must_use]
    pub fn provider_id(&self) -> &str {
        self.provider_id.as_str()
    }

    /// Returns safe model id.
    #[must_use]
    pub fn model_id(&self) -> &str {
        self.model_id.as_str()
    }

    /// Returns provider family label.
    #[must_use]
    pub fn provider_family(&self) -> &str {
        self.provider_family.as_str()
    }

    /// Returns safe endpoint identity.
    #[must_use]
    pub fn endpoint_identity(&self) -> &str {
        self.endpoint_identity.as_str()
    }

    /// Returns `BLAKE3` request identity.
    #[must_use]
    pub fn request_identity(&self) -> &str {
        self.request_identity.as_str()
    }

    /// Returns `BLAKE3` response identity.
    #[must_use]
    pub fn response_identity(&self) -> &str {
        self.response_identity.as_str()
    }

    /// Returns outcome status.
    #[must_use]
    pub const fn status(&self) -> ProviderDispatchOutcomeStatus {
        self.status
    }

    /// Returns finish reason label.
    #[must_use]
    pub fn finish_reason(&self) -> &str {
        self.finish_reason.as_str()
    }

    /// Returns safe credential source label.
    #[must_use]
    pub fn credential_source_label(&self) -> &str {
        self.credential_source_label.as_str()
    }

    /// Returns allowed redaction markers.
    #[must_use]
    pub fn redactions(&self) -> &[String] {
        self.redactions.as_slice()
    }
}

/// Safe concrete provider dispatch outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDispatchOutcome {
    response: animus_adapters::CanonicalModelResponseEnvelope,
    receipt: ProviderDispatchReceipt,
}

impl ProviderDispatchOutcome {
    /// Returns normalized canonical response envelope.
    #[must_use]
    pub const fn response(&self) -> &animus_adapters::CanonicalModelResponseEnvelope {
        &self.response
    }

    /// Returns safe provider dispatch receipt.
    #[must_use]
    pub const fn receipt(&self) -> &ProviderDispatchReceipt {
        &self.receipt
    }
}

/// Builds the reviewed constructor inventory for the concrete `OpenAI` Responses request path.
#[must_use]
pub fn provider_dispatch_constructor_inventory() -> Vec<ProviderDispatchConstructorInventoryEntry> {
    vec![ProviderDispatchConstructorInventoryEntry {
        constructor: String::from(CONSTRUCTOR_OPENAI_RESPONSES),
        shared_fields: ProviderDispatchSharedFieldMatrix::all_preserved(),
    }]
}

/// Validates concrete provider constructor inventory drift.
///
/// # Errors
///
/// Returns [`ProviderDispatchError`] when constructor count or shared-field preservation drifts.
///
/// # Panics
///
/// Panics if constructor inventory invariants drift after validation has accepted them.
pub fn validate_provider_dispatch_constructor_inventory(
    inventory: &[ProviderDispatchConstructorInventoryEntry],
    expected_count: u64,
) -> Result<(), ProviderDispatchError> {
    let actual_count = u64::try_from(inventory.len()).map_err(|_error| {
        ProviderDispatchError::ConstructorInventory {
            reason: MATRIX_REASON_COUNT,
        }
    })?;
    if inventory.is_empty() {
        return Err(ProviderDispatchError::ConstructorInventory {
            reason: MATRIX_REASON_EMPTY,
        });
    }
    if actual_count != expected_count || actual_count > MAX_PROVIDER_DISPATCH_CONSTRUCTORS {
        return Err(ProviderDispatchError::ConstructorInventory {
            reason: MATRIX_REASON_COUNT,
        });
    }
    if inventory
        .iter()
        .any(|entry| entry.constructor.is_empty() || !entry.shared_fields.is_complete())
    {
        return Err(ProviderDispatchError::ConstructorInventory {
            reason: MATRIX_REASON_FIELD,
        });
    }
    assert!(
        !inventory.is_empty(),
        "validated inventory must be non-empty"
    );
    assert_eq!(
        actual_count, expected_count,
        "validated constructor count must match expectation"
    );
    assert!(
        actual_count <= MAX_PROVIDER_DISPATCH_CONSTRUCTORS,
        "validated constructor count must stay bounded"
    );
    assert!(
        inventory
            .iter()
            .all(|entry| !entry.constructor.is_empty() && entry.shared_fields.is_complete()),
        "validated constructor shared fields must stay complete"
    );
    Ok(())
}

/// Returns the expected constructor count for the `OpenAI` Responses fixture path.
#[must_use]
pub const fn expected_openai_responses_constructor_count() -> u64 {
    EXPECTED_OPENAI_RESPONSES_CONSTRUCTOR_COUNT
}

/// Builds a concrete `OpenAI` Responses request `DTO` from a canonical request.
///
/// # Errors
///
/// Returns [`ProviderDispatchError`] when selected-model capability checks fail.
///
/// # Panics
///
/// Panics if the supplied selected model or canonical request bypassed prior validation and lacks
/// required model/message metadata.
pub fn build_openai_responses_dispatch_request(
    selected_model: &super::SelectedModel,
    canonical_request: &animus_adapters::CanonicalModelRequest,
) -> Result<OpenAiResponsesDispatchRequest, ProviderDispatchError> {
    assert!(
        !selected_model.model_id().as_str().is_empty(),
        "selected model ID must be validated"
    );
    assert!(
        !canonical_request.messages().is_empty(),
        "canonical request must contain messages"
    );
    validate_openai_responses_capabilities(selected_model, canonical_request)?;
    Ok(OpenAiResponsesDispatchRequest {
        model: String::from(selected_model.model_id().as_str()),
        messages: canonical_request
            .messages()
            .iter()
            .map(project_message)
            .collect(),
        tools: canonical_request.tools().iter().map(project_tool).collect(),
        response_format: response_format_label(canonical_request.response_format()),
        reasoning_effort: reasoning_effort_label(canonical_request.reasoning().effort()),
        stream: canonical_request.output().stream(),
        max_output_tokens: canonical_request.output().max_output_tokens(),
        temperature_millis: canonical_request.output().temperature_millis(),
        metadata: canonical_request
            .provenance()
            .iter()
            .map(project_metadata)
            .collect(),
    })
}

/// Renders a stable provider-native request fixture literal for review.
///
/// # Panics
///
/// Panics if the provider request bypassed construction and lacks required model/message metadata.
#[must_use]
pub fn render_openai_responses_request_fixture(
    request: &OpenAiResponsesDispatchRequest,
) -> Vec<String> {
    assert!(!request.model().is_empty(), "fixture model must be present");
    assert!(
        !request.messages().is_empty(),
        "fixture request must contain messages"
    );
    let mut parts = vec![literal(Literal {
        field: FIELD_FAMILY,
        value: OPENAI_RESPONSES_FAMILY,
    })];
    parts.push(literal(Literal {
        field: FIELD_MODEL,
        value: request.model(),
    }));
    for message in request.messages() {
        parts.push(literal(Literal {
            field: FIELD_ROLE,
            value: message.role(),
        }));
        for part in message.content() {
            parts.push(content_literal(part));
        }
    }
    for tool in request.tools() {
        parts.push(tool_literal(tool));
    }
    parts.push(literal(Literal {
        field: FIELD_RESPONSE_FORMAT,
        value: request.response_format(),
    }));
    parts.push(literal(Literal {
        field: FIELD_REASONING,
        value: request.reasoning_effort(),
    }));
    parts.push(literal(Literal {
        field: FIELD_STREAM,
        value: bool_label(request.stream()),
    }));
    if let Some(tokens) = request.max_output_tokens() {
        let rendered = tokens.to_string();
        parts.push(literal(Literal {
            field: FIELD_MAX_OUTPUT_TOKENS,
            value: rendered.as_str(),
        }));
    }
    if let Some(temperature) = request.temperature_millis() {
        let rendered = temperature.to_string();
        parts.push(literal(Literal {
            field: FIELD_TEMPERATURE_MILLIS,
            value: rendered.as_str(),
        }));
    }
    for metadata in request.metadata() {
        parts.push(metadata_literal(metadata));
    }
    parts
}

/// Builds a shared-field parity matrix for a concrete provider request.
#[must_use]
pub fn provider_dispatch_shared_field_matrix(
    selected_model: &super::SelectedModel,
    canonical_request: &animus_adapters::CanonicalModelRequest,
    provider_request: &OpenAiResponsesDispatchRequest,
) -> ProviderDispatchSharedFieldMatrix {
    ProviderDispatchSharedFieldMatrix {
        model_identity: provider_request.model() == selected_model.model_id().as_str(),
        messages: messages_match(canonical_request, provider_request),
        content_parts: content_parts_match(canonical_request, provider_request),
        tools: tools_match(canonical_request, provider_request),
        response_format: provider_request.response_format()
            == response_format_label(canonical_request.response_format()),
        reasoning_controls: provider_request.reasoning_effort()
            == reasoning_effort_label(canonical_request.reasoning().effort()),
        stream_intent: provider_request.stream() == canonical_request.output().stream(),
        output_budget: provider_request.max_output_tokens()
            == canonical_request.output().max_output_tokens(),
        sampling_controls: provider_request.temperature_millis()
            == canonical_request.output().temperature_millis(),
        safe_metadata: metadata_match(canonical_request, provider_request),
    }
}

/// Validates a shared-field preservation matrix.
///
/// # Errors
///
/// Returns [`ProviderDispatchError`] when any required shared field is missing.
pub fn validate_provider_dispatch_shared_field_matrix(
    matrix: &ProviderDispatchSharedFieldMatrix,
) -> Result<(), ProviderDispatchError> {
    if matrix.is_complete() {
        return Ok(());
    }
    Err(ProviderDispatchError::ConstructorInventory {
        reason: MATRIX_REASON_FIELD,
    })
}

/// Plans concrete provider fixture dispatch without contacting a provider.
///
/// # Errors
///
/// Returns [`ProviderDispatchError`] when any prerequisite, projection fixture,
/// capability check, or parity rail fails closed before provider contact.
///
/// # Panics
///
/// Panics if the dispatch input bypassed prerequisite setup and lacks reviewed fixture or selected
/// provider metadata.
pub fn plan_provider_dispatch(
    input: &ProviderDispatchInput<'_>,
) -> Result<ProviderDispatchPlan, ProviderDispatchError> {
    assert!(
        !input.reviewed_projection_fixture.is_empty(),
        "reviewed projection fixture must be present"
    );
    assert!(
        !input.selected_model.provider_id().as_str().is_empty(),
        "selected provider ID must be validated"
    );
    validate_dispatch_prerequisites(input)?;
    validate_provider_dispatch_constructor_inventory(
        provider_dispatch_constructor_inventory().as_slice(),
        expected_openai_responses_constructor_count(),
    )?;
    let projection = animus_adapters::assert_model_provider_request_fixture(
        input.canonical_request,
        openai_responses_projection_support(),
        input.reviewed_projection_fixture,
    )
    .map_err(|source| ProviderDispatchError::RequestProjection { source })?;
    let request =
        build_openai_responses_dispatch_request(input.selected_model, input.canonical_request)?;
    let matrix = provider_dispatch_shared_field_matrix(
        input.selected_model,
        input.canonical_request,
        &request,
    );
    validate_provider_dispatch_shared_field_matrix(&matrix)?;
    let request_payload_fixture = render_openai_responses_request_fixture(&request);

    Ok(ProviderDispatchPlan {
        provider_id: String::from(input.selected_model.provider_id().as_str()),
        model_id: String::from(input.selected_model.model_id().as_str()),
        api_kind: input.selected_model.api_kind(),
        endpoint: String::from(input.selected_model.endpoint().as_str()),
        request,
        request_projection_identity: fingerprint_projection(projection.literal_parts()),
        request_payload_identity: fingerprint_owned(request_payload_fixture.as_slice()),
        credential_source_label: credential_label(&input.credential)?,
        deadline_label: deadline_label(&input.deadline)?,
        cancellation_label: cancellation_label(&input.cancellation)?,
        receipt_policy: input.receipt_policy,
    })
}

/// Normalizes a fixture response and projects a safe provider dispatch receipt.
///
/// # Errors
///
/// Returns [`ProviderDispatchError`] when response normalization or receipt validation fails.
pub fn normalize_openai_responses_fixture_response(
    plan: &ProviderDispatchPlan,
    draft: animus_adapters::ProviderResponseFixtureDraft<'_>,
) -> Result<ProviderDispatchOutcome, ProviderDispatchError> {
    let response = animus_adapters::normalize_model_provider_response_fixture(draft)
        .map_err(|source| ProviderDispatchError::MalformedResponse { source })?;
    let receipt = provider_dispatch_receipt(plan, &response)?;
    Ok(ProviderDispatchOutcome { response, receipt })
}

/// Validates a safe provider dispatch receipt.
///
/// # Errors
///
/// Returns [`ProviderDispatchError`] when the receipt contains unsafe material or unapproved markers.
///
/// # Panics
///
/// Panics if compile-time provider redaction markers are empty, indicating an internal receipt
/// contract configuration error.
pub fn validate_provider_dispatch_receipt(
    receipt: &ProviderDispatchReceipt,
) -> Result<(), ProviderDispatchError> {
    assert!(
        !REDACTION_PROVIDER_PAYLOAD.is_empty(),
        "provider payload redaction marker must be present"
    );
    assert!(
        !UNSAFE_REDACTION_MARKER.is_empty(),
        "unsafe marker diagnostic must be present"
    );
    for value in [
        receipt.provider_id(),
        receipt.model_id(),
        receipt.provider_family(),
        receipt.endpoint_identity(),
        receipt.request_identity(),
        receipt.response_identity(),
        receipt.status().as_str(),
        receipt.finish_reason(),
        receipt.credential_source_label(),
    ] {
        validate_safe_receipt_text(value)?;
    }
    for marker in receipt.redactions() {
        if !is_allowed_redaction(marker.as_str()) {
            return Err(ProviderDispatchError::ReceiptLeak {
                reason: UNSAFE_REDACTION_MARKER,
            });
        }
    }
    Ok(())
}

fn validate_dispatch_prerequisites(
    input: &ProviderDispatchInput<'_>,
) -> Result<(), ProviderDispatchError> {
    if input.selected_model.api_kind() != super::ApiKind::OpenAiResponses {
        return Err(ProviderDispatchError::UnsupportedProviderFamily {
            api_kind: input.selected_model.api_kind(),
        });
    }
    validate_openai_responses_capabilities(input.selected_model, input.canonical_request)?;
    credential_label(&input.credential)?;
    validate_service_state(input.service)?;
    validate_entitlement_state(input.entitlement)?;
    deadline_label(&input.deadline)?;
    cancellation_label(&input.cancellation)?;
    validate_fallback_state(input.fallback)?;
    validate_receipt_policy(input.receipt_policy)?;
    Ok(())
}

fn validate_openai_responses_capabilities(
    selected_model: &super::SelectedModel,
    request: &animus_adapters::CanonicalModelRequest,
) -> Result<(), ProviderDispatchError> {
    assert!(
        !selected_model.model_id().as_str().is_empty(),
        "selected model ID must be validated"
    );
    assert!(
        !request.messages().is_empty(),
        "capability validation requires messages"
    );
    if request_has_images(request)
        && !selected_model
            .capabilities()
            .input_modalities()
            .contains(&super::InputModality::Image)
    {
        return Err(ProviderDispatchError::UnsupportedFeature {
            feature: FEATURE_IMAGE_CONTENT,
        });
    }
    if request.reasoning().enabled()
        && selected_model.capabilities().reasoning() == super::ReasoningSupport::None
    {
        return Err(ProviderDispatchError::UnsupportedFeature {
            feature: FEATURE_REASONING,
        });
    }
    if request.output().stream()
        && !selected_model
            .capabilities()
            .streaming()
            .content()
            .is_supported()
    {
        return Err(ProviderDispatchError::UnsupportedFeature {
            feature: FEATURE_STREAMING,
        });
    }
    if request
        .output()
        .max_output_tokens()
        .is_some_and(|tokens| tokens > selected_model.capabilities().output_budget().get())
    {
        return Err(ProviderDispatchError::UnsupportedFeature {
            feature: FEATURE_OUTPUT_BUDGET,
        });
    }
    Ok(())
}

fn validate_service_state(
    service: ProviderDispatchServiceState,
) -> Result<(), ProviderDispatchError> {
    match service {
        ProviderDispatchServiceState::Available => Ok(()),
        ProviderDispatchServiceState::DisabledBackend => {
            Err(ProviderDispatchError::DisabledBackend)
        }
        ProviderDispatchServiceState::Missing => Err(ProviderDispatchError::MissingService),
        ProviderDispatchServiceState::Unavailable => Err(ProviderDispatchError::ServiceUnavailable),
    }
}

fn validate_entitlement_state(
    entitlement: ProviderDispatchEntitlementState,
) -> Result<(), ProviderDispatchError> {
    match entitlement {
        ProviderDispatchEntitlementState::Granted => Ok(()),
        ProviderDispatchEntitlementState::Denied => Err(ProviderDispatchError::EntitlementDenied),
    }
}

fn validate_fallback_state(
    fallback: ProviderDispatchFallbackState,
) -> Result<(), ProviderDispatchError> {
    match fallback {
        ProviderDispatchFallbackState::NotRequested
        | ProviderDispatchFallbackState::RequestedAndAllowed => Ok(()),
        ProviderDispatchFallbackState::RequestedButDenied => {
            Err(ProviderDispatchError::FallbackDenied)
        }
    }
}

fn validate_receipt_policy(
    policy: ProviderDispatchReceiptPolicy,
) -> Result<(), ProviderDispatchError> {
    match policy {
        ProviderDispatchReceiptPolicy::SafeSummaryOnly => Ok(()),
        ProviderDispatchReceiptPolicy::RawProviderPayloadRequested => {
            Err(ProviderDispatchError::UnsafeReceiptPolicy {
                reason: UNSAFE_RECEIPT_POLICY,
            })
        }
    }
}

fn credential_label(
    credential: &ProviderDispatchCredentialState,
) -> Result<String, ProviderDispatchError> {
    match credential {
        ProviderDispatchCredentialState::NotRequired => Ok(String::from(CREDENTIAL_NOT_REQUIRED)),
        ProviderDispatchCredentialState::Ready { source_label } => {
            safe_dispatch_label(source_label)
        }
        ProviderDispatchCredentialState::MissingRequired => {
            Err(ProviderDispatchError::MissingCredential)
        }
    }
}

fn deadline_label(
    deadline: &ProviderDispatchDeadlineState,
) -> Result<String, ProviderDispatchError> {
    match deadline {
        ProviderDispatchDeadlineState::Ready { label } => safe_dispatch_label(label),
        ProviderDispatchDeadlineState::Missing => Err(ProviderDispatchError::MissingDeadline),
        ProviderDispatchDeadlineState::Expired | ProviderDispatchDeadlineState::TimedOut => {
            Err(ProviderDispatchError::Timeout)
        }
    }
}

fn cancellation_label(
    cancellation: &ProviderDispatchCancellationState,
) -> Result<String, ProviderDispatchError> {
    match cancellation {
        ProviderDispatchCancellationState::Ready { label } => safe_dispatch_label(label),
        ProviderDispatchCancellationState::Missing => {
            Err(ProviderDispatchError::MissingCancellation)
        }
        ProviderDispatchCancellationState::Cancelled => Err(ProviderDispatchError::Cancelled),
    }
}

fn safe_dispatch_label(raw: &str) -> Result<String, ProviderDispatchError> {
    if raw.is_empty() {
        return Err(ProviderDispatchError::UnsafeLabel {
            reason: LABEL_EMPTY_REASON,
        });
    }
    if raw.chars().count() > MAX_SAFE_LABEL_CHARS {
        return Err(ProviderDispatchError::UnsafeLabel {
            reason: LABEL_TOO_LONG_REASON,
        });
    }
    validate_safe_receipt_text(raw)?;
    Ok(String::from(raw))
}

fn project_message(message: &animus_adapters::CanonicalModelMessage) -> OpenAiResponsesMessage {
    OpenAiResponsesMessage {
        role: String::from(message.role().as_str()),
        content: message.content().iter().map(project_content_part).collect(),
        tool_call_id: message.tool_call_id().map(String::from),
    }
}

fn project_content_part(
    part: &animus_adapters::CanonicalMessageContentPart,
) -> OpenAiResponsesContentPart {
    match part {
        animus_adapters::CanonicalMessageContentPart::Text { text } => {
            OpenAiResponsesContentPart::Text { text: text.clone() }
        }
        animus_adapters::CanonicalMessageContentPart::ImageDescriptor {
            descriptor_id,
            media_type,
            content_hash,
        } => OpenAiResponsesContentPart::ImageDescriptor {
            descriptor_id: descriptor_id.clone(),
            media_type: media_type.clone(),
            content_hash: content_hash.clone(),
        },
    }
}

fn project_tool(tool: &animus_adapters::CanonicalModelToolDeclaration) -> OpenAiResponsesTool {
    OpenAiResponsesTool {
        name: String::from(tool.name()),
        description: String::from(tool.description()),
        input_schema_label: String::from(tool.input_schema_label()),
    }
}

fn project_metadata(
    metadata: &animus_adapters::CanonicalModelSafeMetadata,
) -> OpenAiResponsesMetadata {
    OpenAiResponsesMetadata {
        key: String::from(metadata.key()),
        value: String::from(metadata.value()),
    }
}

fn response_format_label(format: &animus_adapters::CanonicalModelResponseFormatIntent) -> String {
    match format {
        animus_adapters::CanonicalModelResponseFormatIntent::Text => {
            String::from(RESPONSE_FORMAT_TEXT)
        }
        animus_adapters::CanonicalModelResponseFormatIntent::JsonObject => {
            String::from(RESPONSE_FORMAT_JSON_OBJECT)
        }
        animus_adapters::CanonicalModelResponseFormatIntent::JsonSchema { schema_label } => {
            let mut output = String::from(RESPONSE_FORMAT_JSON_SCHEMA);
            output.push_str(FIELD_SEPARATOR);
            output.push_str(schema_label);
            output
        }
    }
}

fn reasoning_effort_label(effort: animus_adapters::CanonicalModelReasoningEffort) -> String {
    String::from(match effort {
        animus_adapters::CanonicalModelReasoningEffort::None => REASONING_NONE,
        animus_adapters::CanonicalModelReasoningEffort::Low => REASONING_LOW,
        animus_adapters::CanonicalModelReasoningEffort::Medium => REASONING_MEDIUM,
        animus_adapters::CanonicalModelReasoningEffort::High => REASONING_HIGH,
    })
}

fn request_has_images(request: &animus_adapters::CanonicalModelRequest) -> bool {
    request.messages().iter().any(|message| {
        message.content().iter().any(|part| {
            !matches!(
                part,
                animus_adapters::CanonicalMessageContentPart::Text { .. }
            )
        })
    })
}

fn openai_responses_projection_support() -> animus_adapters::ProviderProjectionSupport {
    let preserved = animus_adapters::ProviderProjectionFeatureSupport::Preserved;
    animus_adapters::ProviderProjectionSupport {
        family: animus_adapters::ProviderProjectionFamily::OpenAiResponses,
        developer_role: preserved,
        images: preserved,
        tools: preserved,
        json_object: preserved,
        json_schema: preserved,
        reasoning: preserved,
        streaming: preserved,
        metadata: preserved,
    }
}

fn messages_match(
    canonical_request: &animus_adapters::CanonicalModelRequest,
    provider_request: &OpenAiResponsesDispatchRequest,
) -> bool {
    canonical_request.messages().len() == provider_request.messages().len()
        && canonical_request
            .messages()
            .iter()
            .zip(provider_request.messages())
            .all(|(canonical, provider)| {
                canonical.role().as_str() == provider.role()
                    && canonical.tool_call_id() == provider.tool_call_id()
            })
}

fn content_parts_match(
    canonical_request: &animus_adapters::CanonicalModelRequest,
    provider_request: &OpenAiResponsesDispatchRequest,
) -> bool {
    canonical_request
        .messages()
        .iter()
        .zip(provider_request.messages())
        .all(|(canonical, provider)| canonical.content().len() == provider.content().len())
}

fn tools_match(
    canonical_request: &animus_adapters::CanonicalModelRequest,
    provider_request: &OpenAiResponsesDispatchRequest,
) -> bool {
    canonical_request.tools().len() == provider_request.tools().len()
        && canonical_request
            .tools()
            .iter()
            .zip(provider_request.tools())
            .all(|(canonical, provider)| {
                canonical.name() == provider.name()
                    && canonical.description() == provider.description()
                    && canonical.input_schema_label() == provider.input_schema_label()
            })
}

fn metadata_match(
    canonical_request: &animus_adapters::CanonicalModelRequest,
    provider_request: &OpenAiResponsesDispatchRequest,
) -> bool {
    canonical_request.provenance().len() == provider_request.metadata().len()
        && canonical_request
            .provenance()
            .iter()
            .zip(provider_request.metadata())
            .all(|(canonical, provider)| {
                canonical.key() == provider.key() && canonical.value() == provider.value()
            })
}

fn provider_dispatch_receipt(
    plan: &ProviderDispatchPlan,
    response: &animus_adapters::CanonicalModelResponseEnvelope,
) -> Result<ProviderDispatchReceipt, ProviderDispatchError> {
    assert!(
        !plan.provider_id().is_empty(),
        "receipt provider ID must be present"
    );
    assert!(
        !response.finish_reason().as_str().is_empty(),
        "receipt finish reason must be present"
    );
    let receipt = ProviderDispatchReceipt {
        provider_id: String::from(plan.provider_id()),
        model_id: String::from(plan.model_id()),
        provider_family: String::from(OPENAI_RESPONSES_FAMILY),
        endpoint_identity: String::from(plan.endpoint()),
        request_identity: String::from(plan.request_payload_identity()),
        response_identity: response_identity(response),
        status: ProviderDispatchOutcomeStatus::Success,
        finish_reason: String::from(response.finish_reason().as_str()),
        credential_source_label: String::from(plan.credential_source_label()),
        redactions: vec![
            String::from(REDACTION_PROVIDER_PAYLOAD),
            String::from(REDACTION_HIDDEN_REASONING),
            String::from(REDACTION_CREDENTIAL),
            String::from(REDACTION_TRANSPORT),
        ],
    };
    validate_provider_dispatch_receipt(&receipt)?;
    Ok(receipt)
}

fn response_identity(response: &animus_adapters::CanonicalModelResponseEnvelope) -> String {
    let mut parts = vec![String::from(response.finish_reason().as_str())];
    if let Some(usage) = response.usage() {
        parts.push(usage.input_tokens().to_string());
        parts.push(usage.output_tokens().to_string());
    }
    for item in response.items() {
        parts.push(response_item_class(item));
    }
    fingerprint_owned(parts.as_slice())
}

fn response_item_class(item: &animus_adapters::CanonicalModelResponseItem) -> String {
    String::from(match item {
        animus_adapters::CanonicalModelResponseItem::AssistantText { .. } => "assistant_text",
        animus_adapters::CanonicalModelResponseItem::StructuredPayload { .. } => {
            "structured_payload"
        }
        animus_adapters::CanonicalModelResponseItem::ToolCall { .. } => "tool_call",
        animus_adapters::CanonicalModelResponseItem::Refusal { .. } => "refusal",
        animus_adapters::CanonicalModelResponseItem::Safety { .. } => "safety",
        animus_adapters::CanonicalModelResponseItem::MalformedOutput { .. } => "malformed_output",
    })
}

#[derive(Clone, Copy)]
struct Literal<'a> {
    field: &'a str,
    value: &'a str,
}

fn literal(input: Literal<'_>) -> String {
    let mut output = String::from(input.field);
    output.push_str(FIELD_SEPARATOR);
    output.push_str(input.value);
    output
}

fn content_literal(part: &OpenAiResponsesContentPart) -> String {
    match part {
        OpenAiResponsesContentPart::Text { text } => literal(Literal {
            field: FIELD_TEXT,
            value: text,
        }),
        OpenAiResponsesContentPart::ImageDescriptor {
            descriptor_id,
            media_type,
            content_hash,
        } => {
            let mut value = String::from(descriptor_id);
            value.push_str(IMAGE_SEPARATOR);
            value.push_str(media_type);
            value.push_str(IMAGE_SEPARATOR);
            value.push_str(content_hash);
            literal(Literal {
                field: FIELD_IMAGE,
                value: value.as_str(),
            })
        }
    }
}

fn tool_literal(tool: &OpenAiResponsesTool) -> String {
    let mut value = String::from(tool.name());
    value.push_str(TOOL_SEPARATOR);
    value.push_str(tool.input_schema_label());
    literal(Literal {
        field: FIELD_TOOL,
        value: value.as_str(),
    })
}

fn metadata_literal(metadata: &OpenAiResponsesMetadata) -> String {
    let mut value = String::from(metadata.key());
    value.push_str(METADATA_SEPARATOR);
    value.push_str(metadata.value());
    literal(Literal {
        field: FIELD_METADATA,
        value: value.as_str(),
    })
}

const fn bool_label(value: bool) -> &'static str {
    if value { BOOL_TRUE } else { BOOL_FALSE }
}

fn fingerprint_projection(parts: &[String]) -> String {
    fingerprint_owned(parts)
}

fn fingerprint_owned(parts: &[String]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(HASH_DOMAIN.as_bytes());
    for part in parts {
        hasher.update(HASH_SEPARATOR.as_bytes());
        hasher.update(part.as_bytes());
    }
    let hash = hasher.finalize();
    format!("{HASH_PREFIX}:{}", hash.to_hex())
}

fn validate_safe_receipt_text(value: &str) -> Result<(), ProviderDispatchError> {
    if contains_leak_marker(value) {
        return Err(ProviderDispatchError::ReceiptLeak {
            reason: UNSAFE_RECEIPT_FIELD,
        });
    }
    Ok(())
}

fn contains_leak_marker(value: &str) -> bool {
    [
        LEAK_RAW_PROVIDER_PAYLOAD,
        LEAK_PROVIDER_PAYLOAD,
        LEAK_HIDDEN_REASONING,
        LEAK_CREDENTIAL,
        LEAK_SECRET,
        LEAK_AUTHORIZATION,
        LEAK_BEARER,
        LEAK_PATH_HOME,
        LEAK_PATH_TMP,
    ]
    .iter()
    .any(|marker| value.to_ascii_lowercase().contains(marker))
}

fn is_allowed_redaction(marker: &str) -> bool {
    matches!(
        marker,
        REDACTION_PROVIDER_PAYLOAD
            | REDACTION_HIDDEN_REASONING
            | REDACTION_CREDENTIAL
            | REDACTION_TRANSPORT
    )
}
