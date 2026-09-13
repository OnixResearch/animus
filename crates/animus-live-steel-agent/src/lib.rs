//! Non-default headless composition root for the live Animus Steel agent.
//!
//! The package owns product I/O and adapter selection. Reusable crates retain
//! deterministic contracts and injected ports. Startup validates the complete
//! application configuration before store, credential, provider, or host effects.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![forbid(unsafe_code)]

mod adapters;
mod assembly;
mod durable;
mod entrypoint;
mod evaluation;
mod evidence;
mod owner;
#[cfg(feature = "live-provider")]
mod readiness;
mod settings;

pub use adapters::delegation::admission::admit_child_request;
pub use adapters::delegation::services::{
    ExplicitChildAuthority, FixedChildTimer, MetadataChildEvents, MetadataChildStore,
};
pub use adapters::delegation::{
    ExplicitChildFactory, LiveConcreteChildService, build_child_service,
};
pub use adapters::effects::{
    ExplicitContextMap, LiveAuthorityObserver, LiveHostError, build_bindings, build_executor,
    build_expected_authority, context_route,
};
pub use adapters::responses::{
    FixtureStreamProvider, LiveOpenAiProvider, ProviderStreamError, ProviderStreamService,
};
pub use assembly::{
    LiveSteelAgentComposition, LiveSteelAgentCompositionInput, LiveSteelAgentError,
    LiveSteelAgentRunOutput, SessionPersistence, SnapshotPersistence,
};
pub use durable::recovery::{
    restore_interrupted_kernel, restore_or_create_kernel, session_resume_required,
};
pub use durable::{
    LiveStoreDefinitions, LiveStoreError, open_kernel_persistence, open_session_persistence,
    open_snapshot_persistence, store_definitions,
};
pub use entrypoint::{LiveApplicationError, LiveApplicationInput, run_live_application};
pub use evaluation::ports::KernelSafePointPersistence;
pub use evaluation::{SteelToolBinding, SteelToolError, SteelToolService};
pub use evidence::{LIVE_SHELL_NON_CLAIMS, LiveSteelAgentRunReceipt};
pub use owner::{
    LiveApprovalDecision, LiveChildStatus, LiveSessionExecutorControl, LiveSteelEventRead,
    LiveSteelPrompt, LiveSteelQueueEdit, LiveSteelQueueRemoval, LiveSteelQueuedInput,
    MAX_LIVE_SESSION_EVENT_BATCH, PersistentLiveHost, PersistentStatus, PrivateEventBatch,
};
#[cfg(feature = "live-provider")]
pub use readiness::preflight_live_application_activation;
pub use settings::{
    LiveSteelAgentConfigError, ValidatedLiveSteelAgentConfig, parse_live_steel_agent_config,
};
