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
#[cfg(feature = "syndicate-host")]
mod dataspace;
mod execution;
mod providers;

pub use adapters::{
    AdapterDescriptor, AdapterFamily, AdapterIssue, AdapterPlan, AdapterVerdict, CredentialState,
    EffectExposure, EffectPrerequisites, FixtureKind, FixtureModelAdapter, FixtureOutput,
    FixtureToolAdapter, Fixtures, GateState, OutputState, TerminationState, plan_adapter_effect,
};
/// Backwards-compatible name for deterministic fixture bundles.
pub type FixtureAdapters = Fixtures;
pub use config::{INTENT_ENV_KEY, load_runtime};
#[cfg(feature = "syndicate-host")]
pub use dataspace::{
    SyndicateAdapterError, SyndicateHostDataspaceAdapter, SyndicateObservationDraft,
    SyndicateObservationKind, SyndicateTransport, admit_scoped_orchestration,
    build_safe_subscription, project_observation,
};
pub use execution::{RunError, RunReport, SummaryModelAdapter, run, run_with_model};
pub use providers::{
    ApiKind, CredentialHandle, CredentialRequirement, CredentialSource, EndpointIdentity,
    InputModality, MissingCredentialDiagnostic, ModelCapabilities, ModelCapabilitiesDraft,
    ModelEntry, ModelId, ModelSelection, ModelStreamingCapabilities, ModelStreamingRequest,
    ProviderEntry, ProviderId, ProviderRegistry, ReasoningSupport, RegistryError,
    ResolvedStreamingModes, SelectedModel, StreamFeature, StreamRequestMode, StreamSupport,
    ThinkingLevel, ThinkingPolicy, TokenCost, UnsupportedStreamPolicy,
    resolve_credential_from_env_pairs,
};

/// Public runtime configuration type.
pub type RuntimeConfig = config::Runtime;

/// Public runtime configuration error type.
pub type ConfigError = config::Error;
