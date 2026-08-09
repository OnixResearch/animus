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

mod facades;

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
#[allow(unused_imports)]
pub use facades::*;

/// Re-export of the default standard-library runtime shell.
#[cfg(feature = "std")]
pub use animus_std::{
    ConfigError, INTENT_ENV_KEY, RunError, RunReport, RuntimeConfig, SummaryModelAdapter,
    load_runtime, run, run_with_model,
};
