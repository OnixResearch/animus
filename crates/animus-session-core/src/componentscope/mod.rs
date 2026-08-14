//! Pure generation-fenced component registration, resolution, and teardown.
//!
//! The core consumes explicit activation metadata and host observations. It
//! performs no cancellation, waiting, listener, task, process, filesystem,
//! network, clock, persistence, terminal, provider, tool, or credential effect.

mod core;
mod data;
mod receipt;

pub use core::{
    admit_child_component_scope, admit_component_scope, fail_component_scope_cleanup,
    reduce_component_scope_observation, request_component_scope_close, resolve_component,
};
pub use data::{
    ComponentLease, ComponentRegistration, ComponentRegistrationDraft, ComponentScope,
    ComponentScopeDraft, ComponentScopeError, ComponentScopeReceipt, ComponentScopeState,
    ScopeCleanupFailure, ScopeCleanupIssue, ScopeCleanupIssueKind, ScopeClosePlan,
    ScopeCloseRequest, ScopeEffectRequest, ScopeGeneration, ScopeMemberKind, ScopeObservation,
    ScopeParent, ScopeResolutionRequest, ScopeResolutionTarget, ScopeTerminalOutcome,
    ScopeTransition, ScopeTransitionClass, ScopeVisibility,
};
pub use receipt::{project_component_scope_receipt, validate_component_scope_evidence_text};

#[cfg(test)]
mod tests;
