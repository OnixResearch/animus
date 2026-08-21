//! Non-default headless composition root for the live Animus Steel agent.
//!
//! The package owns product I/O and adapter selection. Reusable crates retain
//! deterministic contracts and injected ports. Startup validates the complete
//! application configuration before store, credential, provider, or host effects.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

#[cfg(feature = "live-provider")]
mod activation;
mod application;
mod config;
mod receipt;
mod runtime;
mod session;
mod shell;
mod store;
mod tool;

#[cfg(feature = "live-provider")]
pub use activation::preflight_live_application_activation;
pub use application::{LiveApplicationError, LiveApplicationInput, run_live_application};
pub use config::{
    LiveSteelAgentConfigError, ValidatedLiveSteelAgentConfig, parse_live_steel_agent_config,
};
pub use receipt::{LIVE_SHELL_NON_CLAIMS, LiveSteelAgentRunReceipt};
pub use runtime::child::{
    ExplicitChildAuthority, ExplicitChildFactory, FixedChildTimer, LiveConcreteChildService,
    MetadataChildEvents, MetadataChildStore, admit_child_request, build_child_service,
};
pub use runtime::host::{
    ExplicitContextMap, LiveAuthorityObserver, LiveHostError, build_bindings, build_executor,
    build_expected_authority, context_route,
};
pub use runtime::provider::{FixtureStreamProvider, LiveOpenAiProvider, ProviderStreamService};
pub use session::{
    LiveApprovalDecision, LiveChildStatus, LiveSessionExecutorControl, LiveSteelEventRead,
    LiveSteelPrompt, LiveSteelQueueEdit, LiveSteelQueueRemoval, LiveSteelQueuedInput,
    MAX_LIVE_SESSION_EVENT_BATCH, PersistentLiveHost, PersistentStatus, PrivateEventBatch,
};
pub use shell::{
    LiveSteelAgentComposition, LiveSteelAgentCompositionInput, LiveSteelAgentError,
    LiveSteelAgentRunOutput, SessionPersistence, SnapshotPersistence,
};
pub use store::{
    LiveStoreDefinitions, LiveStoreError, open_kernel_persistence, open_session_persistence,
    open_snapshot_persistence, restore_interrupted_kernel, restore_or_create_kernel,
    session_resume_required, store_definitions,
};
pub use tool::{KernelSafePointPersistence, SteelToolBinding, SteelToolError, SteelToolService};
