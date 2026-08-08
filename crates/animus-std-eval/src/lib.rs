//! Explicit standard-library host orchestration for native evaluation.
//!
//! The crate consumes effect-free native plans and the separate evaluation
//! adapter ports. It never discovers services. Callers select policy, service
//! implementations, gateway capabilities, storage destinations, clocks,
//! cancellation, and cleanup behavior explicitly.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::too_many_arguments
)]
#![forbid(unsafe_code)]

mod contract;
mod gateway;
mod host;
mod policy;
mod preflight;
mod receipt;

pub use contract::*;
pub use gateway::*;
pub use host::*;
pub use policy::*;
pub use preflight::*;
pub use receipt::*;

/// Evaluation host contract schema.
pub const EVALUATION_HOST_SCHEMA: &str = "animus.native-evaluation-host.v1";
/// Reviewed Verifiers revision.
pub const VERIFIERS_HOST_REVISION: &str = "1a997814e85a2e4f409add960aa852737388dbd8";
/// Reviewed Verifiers license.
pub const VERIFIERS_HOST_LICENSE: &str = "MIT";
/// Ordered reviewed Verifiers host source cohort.
pub const VERIFIERS_HOST_SOURCE_COHORT: &[&str] = &[
    "verifiers/v1/cli/eval/runner.py",
    "verifiers/v1/cli/eval/resume.py",
    "verifiers/v1/episode.py",
    "verifiers/v1/interception/base.py",
    "verifiers/v1/interception/pool.py",
    "verifiers/v1/interception/server.py",
    "verifiers/v1/rollout.py",
    "verifiers/v1/runtimes/limiters.py",
    "verifiers/v1/state.py",
    "verifiers/v1/utils/artifacts.py",
];

#[cfg(test)]
mod tests;
