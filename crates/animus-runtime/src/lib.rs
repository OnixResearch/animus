//! Reusable agent-loop runtime for `animus`.
//!
//! This crate owns typed orchestration over loop-fabric decisions and injected
//! model/tool adapters. It does not construct concrete providers, tools,
//! filesystem handles, processes, network clients, terminal UI, persistence, or
//! Syndicate dataspaces. The deterministic transition core is separated from the
//! thin imperative shell that calls fabrics and adapters.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod contract;
mod fabric;
mod hooks;
mod shell;
mod status;
mod telemetry;
mod transition;
mod worker_team;

pub use contract::{
    DEFAULT_MAX_TURNS, MIN_MAX_TURNS, MaxTurns, RunReport, RuntimeConfig, RuntimeError,
    RuntimeFailure, RuntimeInput, RuntimeStatus,
};
pub use fabric::LoopFabric;
pub use hooks::{
    HookAction, HookBoundary, HookDecision, LifecycleHookDefinition, LifecycleHookDefinitionDraft,
    LifecycleHookDefinitionSource, LifecycleHookEffect, LifecycleHookEffectDraft,
    LifecycleHookError, LifecycleHookErrorKind, LifecycleHookEventKind, LifecycleHookFailureClass,
    LifecycleHookInput, LifecycleHookInputDraft, LifecycleHookOutcome, LifecycleHookOutcomeClass,
    LifecycleHookOutput, LifecycleHookOutputDraft, LifecycleHookPolicyStatus,
    LifecycleHookRedaction, LifecycleHookRequest, LifecycleHookRoutePlan,
    LifecycleHookTrustDecision, LifecycleHookTrustRecord, LifecycleHookTrustRecordDraft,
    LifecycleHookTrustScope, LifecycleHookTrustStatus, MiddlewareError, RuntimeMiddleware,
    RuntimeMiddlewareStack, bind_lifecycle_hook_output_to_policy, evaluate_lifecycle_hook_trust,
    route_lifecycle_hook_requests,
};
pub use shell::{
    RuntimeAdapters, SteelPolicyRuntime, run_agent_loop, run_with_steel_policy,
    run_with_steel_program,
};
/// Pure status-selection result.
pub type AgentStatusSelection = status::choose::Selection;
/// Safe summary of a rejected status candidate.
pub type RejectedStatusCandidate = status::types::RejectedCandidate;
/// Reason an observation was not selected.
pub type RejectedStatusReason = status::types::RejectedReason;
/// Safe summary of a selected status authority.
pub type StatusAuthoritySummary = status::types::AuthoritySummary;
/// Bounded explanation for a status selection decision.
pub type StatusDecisionExplanation = status::explain::DecisionExplanation;
/// Evidence-summary validation error.
pub type StatusEvidenceError = status::safe::EvidenceError;
/// Bounded redacted evidence summary for durable explanations.
pub type StatusEvidenceSummary = status::safe::EvidenceSummary;
/// Reason a decision selected fallback or unknown state.
pub type StatusFallbackReason = status::types::FallbackReason;
/// Freshness class assigned to a status decision.
pub type StatusFreshness = status::types::Freshness;

/// Selects one semantic status authority for a target without ambient I/O.
#[must_use]
pub fn select_agent_status(
    request: &animus_adapters::HostStatusRequest,
    current_sequence: animus_adapters::StatusSequence,
    observations: &[animus_adapters::AgentStatusObservation],
) -> AgentStatusSelection {
    status::choose::select(request, current_sequence, observations)
}

/// Validates a durable status evidence summary.
///
/// # Errors
///
/// Returns [`StatusEvidenceError`] when the summary is empty, overlong, or
/// contains raw or unsafe evidence markers.
pub fn validate_status_evidence_summary(text: &str) -> Result<(), StatusEvidenceError> {
    status::safe::validate_summary(text)
}
pub use telemetry::{
    AdapterComponent, LoopDecisionKind, RuntimeEvent, RuntimeEventKind, RuntimeTerminalKind,
};
pub use transition::{
    AdapterOutcome, AdapterRequest, AdapterStep, DecisionStep, TurnPlan, TurnState,
    apply_adapter_outcome, apply_decision, complete_with_fabric_error, complete_with_guard_block,
    complete_with_guard_failure, complete_with_steel_policy_denial, plan_next_turn,
};
pub use worker_team::{
    Claim, Conflict, ConflictReport, ContextAdmissionPolicy, DelegationFailure, EvidenceReference,
    FailureKind, MutationIntent, ParentTrace, Report, ReportMergeOutcome, WorkerAuthority,
    WorkerBudgets, WorkerCancelOutcome, WorkerCapability, WorkerContextItem, WorkerContextItemKind,
    WorkerContextPacket, WorkerContextRedaction, WorkerHandle, WorkerMergedReport,
    WorkerOutputSchema, WorkerScope, WorkerService, WorkerStatus, WorkerTask, WorkerTaskDraft,
    collect_report, enqueue_task, merge_reports, request_cancel, request_status,
    validate_context_packet, validate_report, validate_task,
};
