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

/// Non-default blocking host for external pause, resume, cancellation, and subscriptions.
#[cfg(feature = "agent-control")]
pub use animus_std_control as agent_control;

/// Re-export of the host-owned generated semantic role configuration loader.
#[cfg(feature = "agent-role-profiles")]
pub use animus_std::{
    AgentRoleConfigError, AgentRoleJsonErrorKind, load_agent_role_profiles,
    parse_agent_role_profiles_json,
};

/// Non-default Pijul CLI contracts, pure validator, and injected host shell.
#[cfg(feature = "pijul-vcs-adapter")]
pub mod pijul_vcs_adapter {
    pub use animus_adapters::pijul::*;
    pub use animus_runtime::pijul::*;
    pub use animus_std::pijul::*;
}

/// Non-default transcript normalizer contracts, pure core, and injected host shell.
#[cfg(feature = "trajectory-transcript-adapter")]
pub mod trajectory_transcript_adapter {
    pub use animus_adapters::trajectory::*;
    pub use animus_runtime::trajectory::*;
    pub use animus_std::trajectory::*;
}

/// Re-export of the host-owned generated model-profile configuration loader.
#[cfg(feature = "model-runtime-profiles")]
pub use animus_std::{
    ModelRuntimeProfileConfigError, ModelRuntimeProfileJsonErrorKind, load_model_runtime_profiles,
    parse_model_runtime_profiles_json,
};

/// Re-export of pure rejected-attempt contracts and injected host services.
#[cfg(feature = "rejected-attempt-context")]
pub mod rejected_attempt_context {
    pub use animus_runtime::rejected_attempt::*;
    pub use animus_std::{
        JujutsuAttemptObservationFixture, RejectedAttemptHostError, RejectedAttemptHostIssue,
        RejectedAttemptHostRequest, RejectedAttemptHostServices, RejectionContextService,
        ReviewEvidenceService, VcsAttemptObservationService, prepare_rejected_attempt_context,
    };
}

/// Re-export of canonical conversation projection policy and cache fixtures.
#[cfg(feature = "conversation-view-projection")]
pub use animus_std::{
    ConversationMemoryCache, ConversationProjectionConfigError,
    load_conversation_projection_policy, parse_conversation_projection_policy_json,
};

/// Re-export of session branch explorer policy and deterministic fixture host.
#[cfg(feature = "session-branch-explorer")]
pub use animus_std::{
    SessionBranchExplorerConfigError, SessionBranchFixtureFailure, SessionBranchMemoryHost,
    load_session_branch_explorer_policy, parse_session_branch_explorer_policy_json,
};

/// Re-export of the non-default durable coding-session host facade.
#[cfg(feature = "coding-session-host")]
pub use animus_std::coding_session_host;

/// Non-default abort-only `BranchFS` speculative workspace host.
#[cfg(feature = "branchfs-host")]
pub use animus_std::branchfs_host;

/// Non-default codebase-graph contracts and capability-scoped Rust host.
#[cfg(feature = "codebase-graph-host")]
pub mod codebase_graph {
    pub use animus_adapters::codegraph::*;
    pub use animus_std::{
        CodeGraphHost, CodeGraphHostError, CodeGraphHostEvent, parse_code_graph_profile_json,
    };
}

/// Non-default authority-free native evaluation contracts.
#[cfg(feature = "native-evaluation")]
pub use animus_eval_core as native_evaluation;

/// Non-default authority-free native evaluation adapter contracts.
#[cfg(feature = "evaluation-adapters")]
pub use animus_adapter_eval as evaluation_adapters;

/// Non-default explicit standard-library native evaluation host.
#[cfg(feature = "evaluation-host")]
pub use animus_std_eval as evaluation_host;

/// Non-default live Basalt UCAN authority gate for guarded host effects.
#[cfg(feature = "basalt-authority-adapter")]
pub use animus_std_basalt_authority as basalt_authority;

/// Non-default bounded SecretSpec credential-resolution host.
#[cfg(feature = "secretspec-host-resolution")]
pub use animus_std_secretspec as secretspec_host_resolution;

/// Non-default deterministic native evaluation conformance suite.
#[cfg(feature = "evaluation-conformance")]
pub use animus_eval_conformance as evaluation_conformance;

/// Strict host-side parser for checked generated native evaluation policy.
#[cfg(feature = "native-evaluation-profile")]
pub use animus_std::{NativeEvaluationProfileError, parse_native_evaluation_policy_json};

/// Non-default Harbor-compatible evaluation planning, ports, and local host shell.
#[cfg(feature = "harbor-eval")]
pub mod harbor_evaluation {
    pub use animus_adapters::harbor::*;
    pub use animus_runtime::harbor_eval::*;
    pub use animus_std::harbor::{
        HostError, ProfileError, execute_separate_verifier, validate_profile_json,
    };
}

/// Non-default authority-free ATIF-v1.7 adapter contracts.
#[cfg(feature = "harbor-atif")]
pub use animus_adapter_atif as harbor_atif;

/// Non-default SpaceWasm diagnostic contracts and injected host service.
#[cfg(feature = "spacewasm-diagnostic")]
pub mod spacewasm_diagnostic {
    pub use animus_adapters::spacewasm::*;
    pub use animus_std::spacewasm::{
        DiagnosticService, ExecutionRequest, HostError, ServiceError, execute,
    };
}

/// Non-default exact-model Mesh provider contracts and injected host adapter.
#[cfg(feature = "mesh-provider")]
pub mod mesh_provider {
    pub use animus_adapters::mesh::*;
    pub use animus_std::mesh::{
        InferenceRequest, MeshHostError, MeshProfile, MeshTransport, ProviderObservation,
        TransportOutcome, TransportRequest, TransportStatus, execute, parse_profile,
    };
}

/// Re-export of the non-default ACP stdio host adapter.
#[cfg(feature = "acp-stdio-host")]
pub use animus_acp_stdio_host as acp_stdio_host;

/// Re-export of the non-default Linux Bubblewrap agent-guardrail host adapter.
#[cfg(feature = "agent-guardrail")]
pub use animus_std_agent_guardrail as agent_guardrail;

/// Re-export of the non-default bounded Lean 4 loop-fabric host adapter.
#[cfg(feature = "lean-loop")]
pub use animus_std_lean_loop as lean_loop;

/// Re-export of active-turn recovery contracts and the non-default fixture host.
#[cfg(feature = "active-turn-recovery")]
pub use animus_std::{
    ActiveTurnFixtureFailure, ActiveTurnMemoryHost, ActiveTurnRecoveryConfigError,
    ActiveTurnRecoveryJsonErrorKind, load_active_turn_recovery_policy,
    parse_active_turn_recovery_policy_json,
};

/// Re-export of host-owned provider retry policy and injected effect seams.
#[cfg(feature = "provider-retry")]
pub use animus_std::{
    ProviderRetryConfigError, ProviderRetryDispatchError, ProviderRetryDispatchInput,
    ProviderRetryJsonErrorKind, ProviderRetryTimerService, dispatch_provider_retry_exact,
    execute_provider_retry_wait, load_provider_retry_policy, parse_provider_retry_policy_json,
    retry_model_request_hash,
};

/// Re-export of the optional Linux-only Sandlock host adapter.
#[cfg(feature = "sandlock-host")]
pub use animus_std::sandlock_host;

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

/// Re-export of the optional Iroh transport composition for Syndicate turn packets.
#[cfg(feature = "iroh-syndicate-host")]
pub use animus_std::{IrohSyndicatePacketSession, IrohSyndicatePacketTransport};

/// Re-export of the optional host-owned Syndicate turn protocol adapter.
#[cfg(feature = "syndicate-host")]
pub use animus_std::{
    DEFAULT_SYNDICATE_MAX_PACKET_BYTES, MAX_SYNDICATE_TURN_EVENTS, SYNDICATE_PROTOCOL_PROFILE,
    SyndicateEvent, SyndicateEventKind, SyndicateHandleState, SyndicatePacketTransport,
    SyndicateProtocolAdapter, SyndicateProtocolError, SyndicateTurn, SyndicateTurnEvent,
    from_syndicate_turn_bytes, from_syndicate_turn_text, plan_syndicate_handle_transition,
    to_syndicate_turn_bytes, to_syndicate_turn_text,
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

/// Re-export of the optional universal-weave branching adapter.
///
/// The adapter wraps the pinned `universal-weave` DAG weave behind
/// Animus-owned DTOs, named bounds, and metadata-only receipts. The neutral
/// session ledger remains the owner of replay and persistence semantics.
#[cfg(feature = "weave-adapter")]
pub use animus_adapter_weave as weave_adapter;

/// Re-export of the optional continual-harness refinement contract family.
#[cfg(feature = "continual-harness")]
pub use animus_adapter_continual as continual_harness;

/// Re-export of the optional daemon client, supervisor, and worker contracts.
#[cfg(feature = "daemon-workers")]
pub use animus_adapter_daemon as daemon_workers;

/// Re-export of the optional family-scoped messaging contracts.
#[cfg(feature = "messaging-receipts")]
pub use animus_adapters::messaging as messaging_receipts;

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

/// Re-export of the non-default autonomous-gate contract family.
#[cfg(feature = "autonomous-gates")]
pub use animus_runtime::autonomous_gates;

/// Re-export of the non-default schedule reliability contract family.
#[cfg(feature = "schedule-reliability")]
pub use animus_runtime::schedule_reliability;

/// Re-export of the non-default session event replay contract family.
#[cfg(feature = "session-event-replay")]
pub use animus_runtime::session_event_replay;

/// Re-export of the non-default execution backend planner.
#[cfg(feature = "execution-backend-planner")]
pub use animus_runtime::execution_backend_planner;
/// Re-export of the current-enforcement host seam.
#[cfg(feature = "execution-backend-planner")]
pub use animus_std::execution_backend_planner_host;

/// Re-export of the no-std model-facing RLM kernel contract family.
#[cfg(feature = "rlm-kernel")]
pub use animus_adapter_rlm_kernel as rlm_kernel_contracts;

/// Re-export of the optional Steel Scheme loop fabric.
#[cfg(feature = "steel")]
pub use animus_steel;

/// Re-export of the optional terminal UI host shell.
#[cfg(feature = "tui")]
pub use animus_tui;
