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

/// Re-export of the non-default concrete child-session host facade.
#[cfg(feature = "child-session-host")]
pub use animus_std::child_session_host;

/// Re-export of the non-default durable Steel kernel restore host.
#[cfg(feature = "durable-steel-kernel-restore")]
pub use animus_std::{
    DurableSteelKernelHostError, DurableSteelKernelHostIssue, DurableSteelKernelPersistence,
    DurableSteelKernelRestoreRequest,
};

/// Re-export of the non-default durable coding-session host facade.
#[cfg(feature = "coding-session-host")]
pub use animus_std::coding_session_host;

/// Non-default abort-only `BranchFS` speculative workspace host.
#[cfg(feature = "branchfs-host")]
pub use animus_std::branchfs_host;

/// Non-default read-only codebase query tool executor and packaged smoke.
#[cfg(feature = "codebase-tool-host")]
pub use animus_std::codebase_tool;

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

/// Non-default authority-free change-request adapter contracts.
#[cfg(feature = "change-request-adapters")]
pub use animus_adapter_change_request as change_request_adapters;

/// Non-default authority-free dependency-maintenance adapter contracts.
#[cfg(feature = "maintenance-adapters")]
pub use animus_adapter_maintenance as maintenance_adapters;

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

/// Non-default live OpenAI Responses host with explicit network and credential authority.
#[cfg(feature = "openai-provider-host")]
pub use animus_std_openai as openai_provider_host;
