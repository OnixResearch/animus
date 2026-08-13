//! Explicit non-default RLM host-effect composition.
//!
//! This crate binds every RLM operation to one injected capability pack. It
//! performs current authority checks before effects and provides no ambient
//! filesystem, command, environment, current-directory, or registry fallback.

#![deny(missing_docs)]
#![forbid(unsafe_code)]

mod authority;
mod contract;
mod executor;
mod routes;

pub use authority::{CurrentAuthorityObservation, CurrentAuthorityObserver, ExpectedAuthority};
pub use contract::{
    BindingIdentity, BindingSet, ConstructionError, DispatchFailure, DispatchFailureClass,
    DispatchObservation, ExecutorLimits, RlmHostResult, RlmHostResultDraft, RouteBinding,
    RouteClass, RouteScope,
};
pub use executor::{CapabilityPacks, RlmHostExecutor, RlmHostExecutorConfig};
pub use routes::{
    ChildRoute, ConcreteChildRoute, ContextRoute, EditRoute, FileReadPack, KernelClearRoute,
    ProcessPack, ProcessProfileSet, ResolverContextRoute, RevisionGatedEditRoute,
    SteelKernelClearRoute, ToolKitRoute, ToolRequestBinding, ToolRoute, WorkspaceEditBinding,
    WorkspaceEditRequestBinder,
};
