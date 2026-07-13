//! Thin compatibility crate for the `animus` workspace.
//!
//! New business logic belongs in `animus-core`; swappable component contracts
//! belong in `animus-adapters`; reusable agent-loop orchestration belongs in
//! `animus-runtime`; standard-library host shell concerns belong in `animus-std`.
//! This crate re-exports the workspace crates for downstream users that want one
//! dependency while keeping crate barriers explicit in-tree. Disable default
//! features to use the no-std facade over `animus-core` and `animus-adapters`.

#![cfg_attr(not(feature = "std"), no_std)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

pub use animus_adapters::{
    AdapterContractError, AgentSessionReference, AgentSessionReferenceDraft,
    AgentSessionReferenceKind, AgentStatusObservation, AgentStatusObservationDraft,
    AgentStatusObservationKind, AgentStatusPresentation, AgentStatusPresentationDraft,
    ComponentAdapter, HostStatusAdapter, HostStatusObservationBatch, HostStatusRequest,
    MAX_ADAPTER_FIELD_CHARS, MIN_ADAPTER_FIELD_CHARS, ModelAdapter, ModelRequest, ModelResponse,
    SAFE_EVIDENCE_HASH_HEX_CHARS, SemanticAgentState, SpeculativeWorkspaceDescriptor,
    SpeculativeWorkspaceDescriptorDraft, StatusSequence, StatusTtl, ToolAdapter, ToolRequest,
    ToolResponse, WorkspaceAbortPolicy, WorkspaceAdapterCapabilities,
    WorkspaceAdapterCapabilitiesDraft, WorkspaceAdapterKind, WorkspaceAdapterOutcome,
    WorkspaceAdapterOutcomeDraft, WorkspaceAdapterOutcomeKind, WorkspaceAdapterState,
    WorkspaceCapability, WorkspaceCleanupPolicy, WorkspaceCommitTarget, WorkspaceConflictPolicy,
    WorkspaceFileInclusionPolicy, WorkspaceOperation,
};
pub use animus_core::{AnimusState, CoreError, Intent, MAX_INTENT_CHARS, MIN_INTENT_CHARS};

/// Re-export of the default standard-library runtime shell.
#[cfg(feature = "std")]
pub use animus_std::{
    ConfigError, INTENT_ENV_KEY, RunError, RunReport, RuntimeConfig, SummaryModelAdapter,
    load_runtime, run, run_with_model,
};

/// Re-export of the host-owned generated semantic role configuration loader.
#[cfg(feature = "agent-role-profiles")]
pub use animus_std::{
    AgentRoleConfigError, AgentRoleJsonErrorKind, load_agent_role_profiles,
    parse_agent_role_profiles_json,
};

/// Re-export of the host-owned generated model-profile configuration loader.
#[cfg(feature = "model-runtime-profiles")]
pub use animus_std::{
    ModelRuntimeProfileConfigError, ModelRuntimeProfileJsonErrorKind, load_model_runtime_profiles,
    parse_model_runtime_profiles_json,
};

/// Re-export of host-owned provider retry policy and injected effect seams.
#[cfg(feature = "provider-retry")]
pub use animus_std::{
    ProviderRetryConfigError, ProviderRetryDispatchError, ProviderRetryDispatchInput,
    ProviderRetryJsonErrorKind, ProviderRetryTimerService, dispatch_provider_retry_exact,
    execute_provider_retry_wait, load_provider_retry_policy, parse_provider_retry_policy_json,
    retry_model_request_hash,
};

/// Re-export of the capability-scoped durable local session-store host.
#[cfg(feature = "durable-session-store")]
pub use animus_std::{
    DurableSessionCommitOutcome, DurableSessionHostError, DurableSessionHostIssue,
    DurableSessionLocalStore, DurableSessionStoreConfigError, DurableSessionStoreJsonErrorKind,
    DurableSessionStoreLocation, DurableSessionStoreOpenRequest, DurableSessionSynchronizer,
    FileSynchronizer, load_durable_session_store_policy, open_durable_session_store,
    parse_durable_session_store_policy_json, read_durable_session_snapshot,
};

/// Re-export of host-owned redacted session-tree export loading and writing seams.
#[cfg(feature = "session-tree-export")]
pub use animus_std::{
    SessionTreeExportConfigError, SessionTreeExportJsonErrorKind, SessionTreeExportWriteCall,
    SessionTreeExportWriteError, SessionTreeExportWriteOutcome, SessionTreeExportWriteRequest,
    SessionTreeExportWriteService, execute_session_tree_export_write,
    load_session_tree_export_policy, parse_session_tree_export_policy_json,
    project_session_tree_export_write_evidence,
};

/// Re-export of the optional host-owned Iroh agent communication adapter seam.
#[cfg(feature = "iroh-host")]
pub use animus_std::{
    IROH_AGENT_COMMS_PROTOCOL, IrohAdapterError, IrohAgentCommsAdapter, IrohAgentFrame,
    IrohAgentFrameDraft, IrohAgentReceipt, IrohAgentTransport, IrohFrameKind, IrohRemoteAdmission,
    IrohRouteDescriptor, IrohRouteDescriptorDraft, admit_iroh_scope, iroh_frame_receipt,
    iroh_route_receipt, project_assertion_frame, project_iroh_frame,
    project_orchestration_event_frame,
};

/// Re-export of the optional Basalt UCAN contract-boundary dependency.
#[cfg(feature = "basalt-reference")]
pub use basalt;

/// Re-export of the optional Cairn lifecycle-engine core dependency.
#[cfg(feature = "cairn-reference")]
pub use cairn_core;

/// Re-export of the optional Trellis verified-logic dependency.
#[cfg(feature = "trellis-reference")]
pub use trellis;

/// Re-export of the optional broad Preserves compatibility facade.
///
/// Adapter-only consumers should depend on `animus-preserves-adapter` directly;
/// schema-only consumers should use `animus-preserves-schema`.
#[cfg(feature = "preserves")]
pub use animus_preserves;

/// Re-export of the optional model-router decision core.
#[cfg(feature = "router")]
pub use animus_router;

/// Re-export of the optional reusable agent-loop runtime.
#[cfg(feature = "runtime")]
pub use animus_runtime;

/// Re-export of the optional Steel Scheme loop fabric.
#[cfg(feature = "steel")]
pub use animus_steel;

/// Re-export of the optional terminal UI host shell.
#[cfg(feature = "tui")]
pub use animus_tui;
