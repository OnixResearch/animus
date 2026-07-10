//! Standard-library runtime shell for `animus`.
//!
//! This crate owns process-facing concerns such as environment parsing and
//! runtime orchestration. It parses untyped inputs once into typed structs,
//! builds core state through `animus-core`, and crosses swappable component
//! boundaries through `animus-adapters` contracts.

#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![cfg_attr(animus_octet, allow(tigerstyle::excessive_file_length))]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod adapters;
mod config;
mod context;
#[cfg(feature = "syndicate-host")]
mod dataspace;
mod execution;
#[cfg(feature = "iroh-host")]
mod iroh;
mod observability;
mod providers;

pub use adapters::{
    AdapterDescriptor, AdapterFamily, AdapterIssue, AdapterPlan, AdapterVerdict, CredentialState,
    EffectExposure, EffectPrerequisites, FixtureKind, FixtureModelAdapter, FixtureOutput,
    FixtureToolAdapter, Fixtures, GateState, OutputState, TerminationState, plan_adapter_effect,
};
/// Backwards-compatible name for deterministic fixture bundles.
pub type FixtureAdapters = Fixtures;
pub use config::{INTENT_ENV_KEY, load_runtime};
pub use context::{STD_CONTEXT_DEFAULT_MAX_BYTES, STD_CONTEXT_DEFAULT_MAX_DIRECTORY_ENTRIES};
/// Root public name for the std local diff context resolver.
pub type LocalDiffContextResolver = context::LocalDiffResolver;
/// Root public name for the std local directory context resolver.
pub type LocalDirectoryContextResolver = context::LocalDirectoryResolver;
/// Root public name for the std local file context resolver.
pub type LocalFileContextResolver = context::LocalFileResolver;
/// Root public name for the std no-op context resolver.
pub type NullContextResolver = context::NullResolver;
/// Root public name for std context resolver policy.
pub type StdContextPolicy = context::Policy;
#[cfg(feature = "syndicate-host")]
pub use dataspace::{
    SyndicateAdapterError, SyndicateHostDataspaceAdapter, SyndicateObservationDraft,
    SyndicateObservationKind, SyndicateTransport, admit_scoped_orchestration,
    build_safe_subscription, project_observation,
};
pub use execution::{RunError, RunReport, SummaryModelAdapter, run, run_with_model};
#[cfg(feature = "iroh-host")]
pub use iroh::{
    IROH_AGENT_COMMS_PROTOCOL, IrohAdapterError, IrohAgentCommsAdapter, IrohAgentFrame,
    IrohAgentFrameDraft, IrohAgentReceipt, IrohAgentTransport, IrohFrameKind, IrohRemoteAdmission,
    IrohRouteDescriptor, IrohRouteDescriptorDraft, admit_iroh_scope, iroh_frame_receipt,
    iroh_route_receipt, project_assertion_frame, project_iroh_frame,
    project_orchestration_event_frame,
};
/// Root public name for the in-memory observability sink.
pub type InMemoryObservabilitySink = observability::InMemorySink;
/// Root public name for the no-op observability sink.
pub type NoopObservabilitySink = observability::NoopSink;
pub use providers::{
    ApiKind, CredentialHandle, CredentialRequirement, CredentialSource, EndpointIdentity,
    InputModality, MissingCredentialDiagnostic, ModelCapabilities, ModelCapabilitiesDraft,
    ModelEntry, ModelId, ModelSelection, ModelStreamingCapabilities, ModelStreamingRequest,
    OpenAiResponsesContentPart, OpenAiResponsesDispatchRequest, OpenAiResponsesMessage,
    OpenAiResponsesMetadata, OpenAiResponsesTool, ProviderDispatchCancellationState,
    ProviderDispatchConstructorInventoryEntry, ProviderDispatchCredentialState,
    ProviderDispatchDeadlineState, ProviderDispatchEntitlementState, ProviderDispatchError,
    ProviderDispatchFallbackState, ProviderDispatchInput, ProviderDispatchOutcome,
    ProviderDispatchOutcomeStatus, ProviderDispatchPlan, ProviderDispatchReceipt,
    ProviderDispatchReceiptPolicy, ProviderDispatchServiceState, ProviderDispatchSharedFieldMatrix,
    ProviderEntry, ProviderId, ProviderRegistry, ProviderStreamParserInput,
    ProviderStreamingCredentialState, ProviderStreamingDispatchInput,
    ProviderStreamingDispatchPlan, ProviderStreamingError, ProviderStreamingServiceState,
    ReasoningSupport, RegistryError, ResolvedStreamingModes, SelectedModel, StreamFeature,
    StreamRequestMode, StreamSupport, ThinkingLevel, ThinkingPolicy, TokenCost,
    UnsupportedStreamPolicy, build_openai_responses_dispatch_request,
    expected_openai_responses_constructor_count, normalize_openai_responses_fixture_response,
    parse_provider_stream_chunks, plan_provider_dispatch, plan_provider_streaming_dispatch,
    provider_dispatch_constructor_inventory, provider_dispatch_shared_field_matrix,
    render_openai_responses_request_fixture, resolve_credential_from_env_pairs,
    validate_provider_dispatch_constructor_inventory, validate_provider_dispatch_receipt,
    validate_provider_dispatch_shared_field_matrix,
};

/// Public runtime configuration type.
pub type RuntimeConfig = config::Runtime;

/// Public runtime configuration error type.
pub type ConfigError = config::Error;
