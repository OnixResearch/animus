//! Shell-owned provider and model registry boundary types.

mod capability;
mod catalog;
mod choice;
mod concrete;
mod identity;
#[cfg(feature = "models-dev-catalog")]
mod models_dev;
mod reason;
mod secret;
mod stream;

#[cfg(test)]
mod tests;

pub use capability::{
    ApiKind, EndpointIdentity, InputModality, ModelCapabilities, ModelCapabilitiesDraft,
    ModelStreamingCapabilities, ModelStreamingRequest, ResolvedStreamingModes, StreamFeature,
    StreamRequestMode, StreamSupport, TokenCost, UnsupportedStreamPolicy,
};
pub use catalog::{ModelEntry, ProviderEntry, ProviderRegistry};
pub use choice::{ModelSelection, SelectedModel};
pub use concrete::{
    OpenAiResponsesContentPart, OpenAiResponsesDispatchRequest, OpenAiResponsesMessage,
    OpenAiResponsesMetadata, OpenAiResponsesTool, ProviderDispatchCancellationState,
    ProviderDispatchConstructorInventoryEntry, ProviderDispatchCredentialState,
    ProviderDispatchDeadlineState, ProviderDispatchEntitlementState, ProviderDispatchError,
    ProviderDispatchFallbackState, ProviderDispatchInput, ProviderDispatchOutcome,
    ProviderDispatchOutcomeStatus, ProviderDispatchPlan, ProviderDispatchReceipt,
    ProviderDispatchReceiptPolicy, ProviderDispatchServiceState, ProviderDispatchSharedFieldMatrix,
    build_openai_responses_dispatch_request, expected_openai_responses_constructor_count,
    normalize_openai_responses_fixture_response, plan_provider_dispatch,
    provider_dispatch_constructor_inventory, provider_dispatch_shared_field_matrix,
    render_openai_responses_request_fixture, validate_provider_dispatch_constructor_inventory,
    validate_provider_dispatch_receipt, validate_provider_dispatch_shared_field_matrix,
};
pub use identity::{ModelId, ProviderId};
#[cfg(feature = "models-dev-catalog")]
pub use models_dev::{
    MAX_MODELS_DEV_SNAPSHOT_BYTES, MODELS_DEV_API_URL, MODELS_DEV_SOURCE_LABEL,
    ModelsDevIngestError, ModelsDevIngestReport, ModelsDevJsonErrorKind, ModelsDevProviderBinding,
    ModelsDevProviderBindingDraft, ingest_models_dev_api_json,
};
pub use reason::{ReasoningSupport, ThinkingLevel, ThinkingPolicy};
pub use secret::{
    CredentialHandle, CredentialRequirement, CredentialSource, MissingCredentialDiagnostic,
    resolve_credential_from_env_pairs,
};
pub use stream::{
    ProviderStreamParserInput, ProviderStreamingCredentialState, ProviderStreamingDispatchInput,
    ProviderStreamingDispatchPlan, ProviderStreamingError, ProviderStreamingServiceState,
    parse_provider_stream_chunks, plan_provider_streaming_dispatch,
};

const INVALID_ID_EMPTY_REASON: &str = "identifier must be non-empty";
const INVALID_ID_LENGTH_REASON: &str = "identifier exceeds the safe length limit";
const INVALID_ID_CHAR_REASON: &str = "identifier contains an unsupported character";
const INVALID_ENDPOINT_EMPTY_REASON: &str = "endpoint identity must be non-empty";
const INVALID_ENDPOINT_LENGTH_REASON: &str = "endpoint identity exceeds the safe length limit";
const INVALID_BUDGET_REASON: &str = "budget must be non-zero";
const EMPTY_COLLECTION_REASON: &str = "collection must contain at least one entry";
const COLLECTION_LIMIT_REASON: &str = "collection exceeds the safe entry limit";
const DUPLICATE_PROVIDER_REASON: &str = "provider id appears more than once";
const DUPLICATE_MODEL_REASON: &str = "model id appears more than once for provider";
const PROVIDER_ID_FIELD: &str = "provider_id";
const MODEL_ID_FIELD: &str = "model_id";
const ENDPOINT_FIELD: &str = "endpoint";
const CONTEXT_BUDGET_FIELD: &str = "context_budget";
const OUTPUT_BUDGET_FIELD: &str = "output_budget";
const MODALITIES_FIELD: &str = "input_modalities";
const THINKING_LEVELS_FIELD: &str = "thinking_levels";
const PROVIDERS_FIELD: &str = "providers";
const MODELS_FIELD: &str = "models";
const CREDENTIAL_SOURCE_FIELD: &str = "credential_source";

/// Provider registry validation or selection error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RegistryError {
    /// A provider or model identifier was malformed.
    InvalidIdentifier {
        /// Identifier field name.
        field: &'static str,
        /// Validation reason.
        reason: &'static str,
    },
    /// An endpoint identity was malformed.
    InvalidEndpoint {
        /// Endpoint field name.
        field: &'static str,
        /// Validation reason.
        reason: &'static str,
    },
    /// A model budget was zero.
    InvalidBudget {
        /// Budget field name.
        field: &'static str,
        /// Validation reason.
        reason: &'static str,
    },
    /// A bounded collection was empty.
    EmptyCollection {
        /// Collection field name.
        field: &'static str,
        /// Validation reason.
        reason: &'static str,
    },
    /// A bounded collection exceeded its maximum size.
    CollectionLimit {
        /// Collection field name.
        field: &'static str,
        /// Validation reason.
        reason: &'static str,
    },
    /// A provider id was duplicated.
    DuplicateProvider {
        /// Safe provider id.
        provider: String,
        /// Validation reason.
        reason: &'static str,
    },
    /// A model id was duplicated within a provider entry.
    DuplicateModel {
        /// Safe provider id.
        provider: String,
        /// Safe model id.
        model: String,
        /// Validation reason.
        reason: &'static str,
    },
    /// A selected provider is not present.
    ProviderUnavailable {
        /// Safe provider id.
        provider: String,
    },
    /// A selected model is not present for the provider.
    ModelUnavailable {
        /// Safe provider id.
        provider: String,
        /// Safe model id.
        model: String,
    },
    /// A requested thinking level is not supported under reject policy.
    UnsupportedThinkingLevel {
        /// Requested stable thinking-level label.
        requested: &'static str,
    },
    /// A requested stream capability is not supported under reject policy.
    UnsupportedStreamCapability {
        /// Stable stream capability label.
        capability: &'static str,
    },
    /// A required credential was absent.
    MissingCredential {
        /// Redacted diagnostic.
        diagnostic: MissingCredentialDiagnostic,
    },
}

impl std::fmt::Display for RegistryError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidIdentifier { field, reason }
            | Self::InvalidEndpoint { field, reason }
            | Self::InvalidBudget { field, reason }
            | Self::EmptyCollection { field, reason }
            | Self::CollectionLimit { field, reason } => {
                write!(
                    formatter,
                    "invalid provider registry field {field}: {reason}"
                )
            }
            Self::DuplicateProvider { provider, reason } => {
                write!(formatter, "duplicate provider {provider}: {reason}")
            }
            Self::DuplicateModel {
                provider,
                model,
                reason,
            } => write!(
                formatter,
                "duplicate model {model} for provider {provider}: {reason}"
            ),
            Self::ProviderUnavailable { provider } => {
                write!(formatter, "provider {provider} is unavailable")
            }
            Self::ModelUnavailable { provider, model } => {
                write!(
                    formatter,
                    "model {model} is unavailable for provider {provider}"
                )
            }
            Self::UnsupportedThinkingLevel { requested } => {
                write!(formatter, "thinking level {requested} is not supported")
            }
            Self::UnsupportedStreamCapability { capability } => {
                write!(formatter, "stream capability {capability} is not supported")
            }
            Self::MissingCredential { diagnostic } => {
                write!(formatter, "missing provider credential: {diagnostic}")
            }
        }
    }
}

impl std::error::Error for RegistryError {}
