//! Standard-library runtime shell for `animus`.
//!
//! This crate owns process-facing concerns such as environment parsing and
//! runtime orchestration. It parses untyped inputs once into typed structs,
//! builds core state through `animus-core`, and crosses swappable component
//! boundaries through `animus-adapters` contracts.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod adapters;
mod config;
mod execution;

pub use adapters::{
    AdapterDescriptor, AdapterFamily, AdapterIssue, AdapterPlan, AdapterVerdict, CredentialState,
    EffectExposure, EffectPrerequisites, FixtureAdapters, FixtureKind, FixtureModelAdapter,
    FixtureOutput, FixtureToolAdapter, GateState, OutputState, TerminationState,
    plan_adapter_effect,
};
pub use config::{INTENT_ENV_KEY, load_runtime};
pub use execution::{RunError, RunReport, SummaryModelAdapter, run, run_with_model};

/// Public runtime configuration type.
pub type RuntimeConfig = config::Runtime;

/// Public runtime configuration error type.
pub type ConfigError = config::Error;
