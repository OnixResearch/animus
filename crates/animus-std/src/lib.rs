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

#[cfg(feature = "active-turn-recovery")]
mod activeturnrecovery;
mod adapters;
#[cfg(feature = "agent-role-profiles")]
mod agentroles;
#[cfg(feature = "authoritative-completion")]
#[path = "authoritativecompletion.rs"]
mod authoritativecompletion;
#[cfg(feature = "branchfs-host")]
#[path = "branchfshost.rs"]
pub mod branchfs_host;
#[cfg(feature = "codebase-graph-host")]
mod codegraph;
#[cfg(feature = "coding-session-host")]
#[path = "codingsession.rs"]
mod coding_session;
/// Non-default coding-session lifecycle host and explicit injected services.
#[cfg(feature = "coding-session-host")]
pub mod coding_session_host {
    pub use super::coding_session::*;
}
mod config;
mod context;
#[cfg(feature = "conversation-view-projection")]
mod conversationprojection;
#[cfg(feature = "syndicate-host")]
mod dataspace;
mod document;
#[cfg(feature = "dogfood-smoke")]
mod dogfood_smoke;
#[cfg(feature = "durable-session-store")]
#[path = "sessionstore.rs"]
mod durablefile;
mod execution;
#[cfg(feature = "harbor-eval")]
pub mod harbor;
mod hostadapters;
#[cfg(feature = "iroh-host")]
mod iroh;
#[cfg(feature = "session-branch-explorer")]
mod sessionbranchexplorer;
// r[impl agent_harness_runtime.syndicate_iroh_bridge.feature]
#[cfg(feature = "iroh-syndicate-host")]
mod irohsyndicate;
#[cfg(feature = "mesh-provider")]
pub mod mesh;
mod modelevidence;
#[cfg(feature = "model-runtime-profiles")]
mod modelprofiles;
#[cfg(feature = "observability-host")]
mod observability;
#[cfg(feature = "provider-retry")]
mod providerretry;
mod providers;
#[cfg(feature = "session-tree-export")]
#[path = "sessiontreeexport.rs"]
mod redactedwriter;
#[cfg(feature = "runtime-authority-seal")]
#[path = "runtimeauthorityseal.rs"]
mod runtime_authority_seal;
#[cfg(feature = "sandlock-host")]
#[path = "sandlockhost/mod.rs"]
pub mod sandlock_host;
#[cfg(feature = "spacewasm-diagnostic")]
pub mod spacewasm;
#[cfg(feature = "syndicate-host")]
mod syndicate;

#[cfg(feature = "active-turn-recovery")]
pub use activeturnrecovery::{
    ActiveTurnFixtureFailure, ActiveTurnMemoryHost, ActiveTurnRecoveryConfigError,
    ActiveTurnRecoveryJsonErrorKind, load_active_turn_recovery_policy,
    parse_active_turn_recovery_policy_json,
};
pub use adapters::{
    AdapterDescriptor, AdapterFamily, AdapterIssue, AdapterPlan, AdapterVerdict, CredentialState,
    EffectExposure, EffectPrerequisites, FixtureKind, FixtureModelAdapter, FixtureOutput,
    FixtureToolAdapter, Fixtures, GateState, OutputState, TerminationState, plan_adapter_effect,
};
#[cfg(feature = "agent-role-profiles")]
pub use agentroles::{
    AgentRoleConfigError, AgentRoleJsonErrorKind, load_agent_role_profiles,
    parse_agent_role_profiles_json,
};
#[cfg(feature = "authoritative-completion")]
pub use authoritativecompletion::{
    AgentCheckExecutionRequest, AgentCheckExecutionService, AgentCheckRegistryConfigError,
    CairnCompletionSummary, CairnCompletionValidationRequest, CairnCompletionValidationService,
    ChangedArtifactObservationRequest, ChangedArtifactObservationService,
    CompletionArtifactPersistenceRequest, CompletionArtifactPersistenceService,
    FinalSubjectHashRequest, FinalSubjectHashService, HostArtifactObservation,
    HostCheckExecutionObservation, ObservedChangedArtifact, load_agent_check_registry,
    parse_agent_check_registry_json,
};
#[cfg(feature = "codebase-graph-host")]
pub use codegraph::{
    CodeGraphHost, CodeGraphHostError, CodeGraphHostEvent, parse_code_graph_profile_json,
};
#[cfg(feature = "conversation-view-projection")]
pub use conversationprojection::{
    ConversationMemoryCache, ConversationProjectionConfigError,
    load_conversation_projection_policy, parse_conversation_projection_policy_json,
};
#[cfg(feature = "session-branch-explorer")]
pub use sessionbranchexplorer::{
    SessionBranchExplorerConfigError, SessionBranchFixtureFailure, SessionBranchMemoryHost,
    load_session_branch_explorer_policy, parse_session_branch_explorer_policy_json,
};
/// Backwards-compatible name for deterministic fixture bundles.
pub type FixtureAdapters = Fixtures;
pub use config::{INTENT_ENV_KEY, load_runtime};
pub use context::{
    CapabilityRoot, CapabilityRootKind, STD_CONTEXT_DEFAULT_MAX_BYTES,
    STD_CONTEXT_DEFAULT_MAX_DIRECTORY_ENTRIES,
};
pub use document::{
    DocumentExtractionError, DocumentExtractionHost, DocumentExtractionIsolationRequirement,
    DocumentExtractionPlan, DocumentExtractionRequest, DocumentExtractionServiceState,
    DocumentExtractor, plan_document_extraction, validate_document_report_at_shell,
};
#[cfg(feature = "dogfood-smoke")]
pub use dogfood_smoke::run_packaged_headless_dogfood_smoke;
pub use hostadapters::{
    HostAdapterAvailability, HostAdapterDescriptor, HostAdapterFallback, HostAdapterFamily,
    HostAdapterPlan, HostAdapterPlanIssue, HostAdapterPlanRequest, HostAdapterPlanStatus,
    HostAuthority, host_adapter_catalog, plan_host_adapter,
};
/// Root public name for the std local diff context resolver.
pub type LocalDiffContextResolver = context::LocalDiffResolver;
/// Root public name for the std local directory context resolver.
pub type LocalDirectoryContextResolver = context::LocalDirectoryResolver;
/// Root public name for the std local file context resolver.
pub type LocalFileContextResolver = context::LocalFileResolver;
/// Root public name for the std no-op context resolver.
pub type NullContextResolver = context::NullResolver;
pub use modelevidence::{
    ModelEvidenceIngestError, ModelEvidenceIngestHost, ModelEvidenceIngestPlan,
    ModelEvidenceIngestRequest, ModelEvidenceIngestServiceState, ModelEvidenceSource,
    plan_model_evidence_ingest, validate_model_evidence_at_shell,
};
#[cfg(feature = "model-runtime-profiles")]
pub use modelprofiles::{
    ModelRuntimeProfileConfigError, ModelRuntimeProfileJsonErrorKind, load_model_runtime_profiles,
    parse_model_runtime_profiles_json,
};
/// Root public name for std context resolver policy.
pub type StdContextPolicy = context::Policy;
#[cfg(feature = "syndicate-host")]
pub use animus_preserves::{
    MAX_SYNDICATE_TURN_EVENTS, SYNDICATE_PROTOCOL_PROFILE, SyndicateEvent, SyndicateEventKind,
    SyndicateTurn, SyndicateTurnEvent, from_syndicate_turn_bytes, from_syndicate_turn_text,
    to_syndicate_turn_bytes, to_syndicate_turn_text,
};
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
#[cfg(feature = "iroh-syndicate-host")]
pub use irohsyndicate::{IrohSyndicatePacketSession, IrohSyndicatePacketTransport};
#[cfg(feature = "syndicate-host")]
pub use syndicate::{
    DEFAULT_SYNDICATE_MAX_PACKET_BYTES, SyndicateHandleState, SyndicatePacketTransport,
    SyndicateProtocolAdapter, SyndicateProtocolError, plan_syndicate_handle_transition,
};
/// Root public name for the in-memory observability sink.
#[cfg(feature = "observability-host")]
pub type InMemoryObservabilitySink = observability::InMemorySink;
/// Root public name for the no-op observability sink.
#[cfg(feature = "observability-host")]
pub type NoopObservabilitySink = observability::NoopSink;
#[cfg(feature = "durable-session-store")]
pub use durablefile::{
    DurableSessionCommitOutcome, DurableSessionHostError, DurableSessionHostIssue,
    DurableSessionLocalStore, DurableSessionStoreConfigError, DurableSessionStoreJsonErrorKind,
    DurableSessionStoreLocation, DurableSessionStoreOpenRequest, DurableSessionSynchronizer,
    FileSynchronizer, load_durable_session_store_policy, open_durable_session_store,
    parse_durable_session_store_policy_json, read_durable_session_snapshot,
};
#[cfg(feature = "provider-retry")]
pub use providerretry::{
    ProviderRetryConfigError, ProviderRetryDispatchError, ProviderRetryDispatchInput,
    ProviderRetryJsonErrorKind, ProviderRetryTimerService, dispatch_provider_retry_exact,
    execute_provider_retry_wait, load_provider_retry_policy, parse_provider_retry_policy_json,
    retry_model_request_hash,
};
/// Non-default host shell for one sealed runtime-authority dispatch.
#[cfg(feature = "runtime-authority-seal")]
pub mod runtime_authority_seal_host {
    pub use super::runtime_authority_seal::*;
}
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
#[cfg(feature = "session-tree-export")]
pub use redactedwriter::{
    SessionTreeExportConfigError, SessionTreeExportJsonErrorKind, SessionTreeExportWriteCall,
    SessionTreeExportWriteError, SessionTreeExportWriteOutcome, SessionTreeExportWriteRequest,
    SessionTreeExportWriteService, execute_session_tree_export_write,
    load_session_tree_export_policy, parse_session_tree_export_policy_json,
    project_session_tree_export_write_evidence,
};

/// Public runtime configuration type.
pub type RuntimeConfig = config::Runtime;

/// Public runtime configuration error type.
pub type ConfigError = config::Error;
