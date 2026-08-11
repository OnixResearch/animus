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

/// Non-default transactional daemon-handoff contracts and injected host shell.
#[cfg(feature = "daemon-handoff")]
pub use animus_std::daemon_handoff_host as daemon_handoff;

/// Non-default capability-relative workspace edit host.
#[cfg(feature = "workspace-edit-host")]
pub use animus_std_workspace_edit as workspace_edit;

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
