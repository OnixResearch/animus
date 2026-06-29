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
mod shell;
mod status;
mod telemetry;
mod transition;

pub use contract::{
    DEFAULT_MAX_TURNS, MIN_MAX_TURNS, MaxTurns, RunReport, RuntimeConfig, RuntimeError,
    RuntimeFailure, RuntimeInput, RuntimeStatus,
};
pub use fabric::LoopFabric;
pub use shell::{RuntimeAdapters, run_agent_loop, run_with_steel_program};
pub use status::{
    AgentStatusSelection, RejectedStatusCandidate, RejectedStatusReason, StatusAuthoritySummary,
    StatusDecisionExplanation, StatusEvidenceError, StatusEvidenceSummary, StatusFallbackReason,
    StatusFreshness, select_agent_status, validate_status_evidence_summary,
};
pub use telemetry::{AdapterComponent, RuntimeEvent};
pub use transition::{
    AdapterOutcome, AdapterRequest, AdapterStep, DecisionStep, TurnPlan, TurnState,
    apply_adapter_outcome, apply_decision, complete_with_fabric_error, plan_next_turn,
};
