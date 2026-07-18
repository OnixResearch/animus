#![forbid(unsafe_code)]
//! Optional Linux Bubblewrap guardrail for explicitly authorized Animus agent processes.
//!
//! The pure core consumes checked profile data and caller-supplied path/authorization facts. The
//! shell owns explicit file reads, executable probing, sealed memfd preparation, and process launch.
//! No API discovers project policy, Bubblewrap, environment values, host roots, or fallback paths.

mod core;
mod model;
mod shell;
mod transport;

pub use core::{build_plan, build_receipt, load_generated_profile, preflight};
pub use model::{
    ActiveContext, AuthorizationBinding, BUBBLEWRAP_COHORT, EnvironmentBinding, ExecutableFacts,
    ExecutionObservation, ExitClass, GuardrailError, GuardrailErrorKind, GuardrailPlan,
    GuardrailProfile, GuardrailReceipt, NON_CLAIMS, NetworkPosture, PLAN_SCHEMA, PlanOperation,
    PreflightRequest, PreflightResult, PreflightStatus, ProfileRoot, RECEIPT_SCHEMA,
    REDACTION_MARKER, RootBinding, RootMode, SourceKind,
};
pub use shell::{load_profile_file, load_request_file, observe_and_match_executable};
pub use transport::{
    LaunchObservation, encode_options, finalize_options, launch, observe_executable,
    prepare_sealed_options,
};
