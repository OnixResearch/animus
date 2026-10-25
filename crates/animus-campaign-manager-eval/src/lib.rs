#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![cfg_attr(
    animus_octet,
    allow(
        tigerstyle::module_file_count,
        reason = "the lint counts animus-eval-core's source directory, which this crate reaches \
                  through the animus_runtime::harbor_eval re-exports; this crate's own module \
                  directories hold at most two Rust files"
    )
)]
#![forbid(unsafe_code)]
//! A Harbor-compatible evaluation lane for the Animus campaign manager.
//!
//! The lane runs the manager through Animus's Harbor slice (`animus_runtime::harbor_eval`,
//! `animus_adapters::harbor`, and `animus_std::harbor`), not through Harbor's Python application.
//!
//! - [`TaskSet`] is the typed Nickel task set (`config/campaign-manager-eval-tasks.ncl`),
//!   revalidated from its checked generated projection. Each task is one exploration campaign
//!   whose candidates hide a signal and one target.
//! - [`CampaignEnvironment`] is the Harbor environment service. One trial opens a real Campaign
//!   campaign on in-memory Choregraph history and runs manager turns with the trial's model route.
//!   Between turns it lets Campaign's planner select, simulates the selected operation against the
//!   hidden outcomes, and publishes the result. The model sees an outcome only as the signal the
//!   host writes into that candidate's label.
//! - [`CampaignVerifier`] is the separate verifier. It reloads the published control projection
//!   from the trial's Choregraph graph and grades the replayed state, never the environment's
//!   claims.
//! - [`run_job`] plans the task × route matrix with `plan_trials`, runs every trial through
//!   `execute_separate_verifier`, reduces rewards per route, projects ATIF trajectories, and
//!   builds the metadata-only receipt.
//!
//! Model routes are [`ScriptedPolicy`] values, which are deterministic and offline, or, with the
//! `live` feature, an OpenAI-compatible Responses route over `animus-std-openai`.
//!
//! The crate is `AGPL-3.0-or-later` because it links Campaign, and it stays outside the
//! workspace's default members.
// r[impl agent_harness_runtime.campaign_manager_eval.boundary]

mod artifacts;
mod catalog;
mod grading;
mod history;
mod matrix;
#[cfg(feature = "live")]
mod provider;
mod rollout;
mod routes;
mod trajectory;

pub use artifacts::{ArtifactStore, TrialArtifact};
pub use catalog::{
    EvalTask, HarborPins, MAX_CANDIDATES, MAX_SIGNAL, MAX_TURNS, MandateLimits, TASK_SET_SCHEMA,
    TaskCandidate, TaskError, TaskSet, embedded_task_set, parse_task_set,
};
pub use grading::{CampaignVerifier, METRIC_EFFICIENCY, METRIC_FOUND, METRIC_REFUSALS, grade};
pub use history::{CandidateEntry, HostFailure, TrialCampaign};
pub use matrix::{ARTIFACT_NAMES, JobError, JobReport, RouteSummary, TrialFailure, run_job};
#[cfg(feature = "live")]
pub use provider::{Endpoint, LiveFailure, LiveRoute};
pub use rollout::{CampaignEnvironment, HostAction, TrialRun, TurnOutcome, TurnRecord, run_turns};
pub use routes::{Route, RouteAdapter, RouteFailure, RouteModel, ScriptedPolicy};
pub use trajectory::project_turns;
