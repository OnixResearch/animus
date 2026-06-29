//! Adapter contracts for swappable `animus` components.
//!
//! This crate is `#![no_std]` and uses `alloc` for owned boundary DTOs. It is
//! intentionally implementation-free: hosts embed `animus-core`, implement these
//! ports for their model/tool/runtime components, and choose `animus-std` only
//! when they want the default standard-library shell.

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

mod manifest;
mod model;
mod status;
mod tool;
mod validation;

pub use manifest::{
    LaunchPolicy, LaunchPolicyDraft, NetworkPolicy, PROHIBITED_PLUGIN_MANIFEST_EFFECTS,
    PluginDescriptor, PluginDescriptorDraft, PluginDescriptorError, RedactionPolicy,
    RedactionPolicyDraft, RuntimeKind, SandboxMode, SandboxRequirements, SandboxRequirementsDraft,
    ToolDescriptor, ToolDescriptorDraft, WorkingDirectoryMode,
};
pub use status::{
    AgentSessionReference, AgentSessionReferenceDraft, AgentSessionReferenceKind,
    AgentStatusObservation, AgentStatusObservationDraft, AgentStatusObservationKind,
    AgentStatusPresentation, AgentStatusPresentationDraft, HostStatusObservationBatch,
    HostStatusRequest, SAFE_EVIDENCE_HASH_HEX_CHARS, SemanticAgentState, StatusSequence, StatusTtl,
};
pub use validation::{AdapterContractError, MAX_ADAPTER_FIELD_CHARS, MIN_ADAPTER_FIELD_CHARS};

/// Request sent from the harness to a model component.
pub type ModelRequest = model::Request;

/// Response returned by a model component.
pub type ModelResponse = model::Response;

/// Request sent from the harness to a tool component.
pub type ToolRequest = tool::Request;

/// Response returned by a tool component.
pub type ToolResponse = tool::Response;

/// Model-generation adapter contract.
pub trait ModelAdapter: ComponentAdapter<ModelRequest, ModelResponse> {}

impl<T> ModelAdapter for T where T: ComponentAdapter<ModelRequest, ModelResponse> {}

/// Tool-invocation adapter contract.
pub trait ToolAdapter: ComponentAdapter<ToolRequest, ToolResponse> {}

impl<T> ToolAdapter for T where T: ComponentAdapter<ToolRequest, ToolResponse> {}

/// Host status-observation adapter contract.
pub trait HostStatusAdapter:
    ComponentAdapter<HostStatusRequest, HostStatusObservationBatch>
{
}

impl<T> HostStatusAdapter for T where
    T: ComponentAdapter<HostStatusRequest, HostStatusObservationBatch>
{
}

/// Generic port between harness orchestration and a swappable component.
pub trait ComponentAdapter<Request, Response> {
    /// Error returned by the concrete adapter implementation.
    type Error;

    /// Calls the component with one typed request and returns one typed response.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the concrete adapter cannot satisfy the
    /// request or rejects it at its own boundary.
    fn call(&mut self, request: &Request) -> Result<Response, Self::Error>;
}

#[cfg(test)]
extern crate std;
