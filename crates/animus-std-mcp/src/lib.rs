//! Non-default, bounded MCP standard-input client host.
//!
//! The pure core validates configuration, lifecycle, discovery, authority,
//! request-local context, and receipts. The process shell owns only an exact
//! caller-authorized child, bounded newline framing, deadlines, cancellation,
//! and owned teardown.

mod authority;
mod config;
mod context;
mod discovery;
mod evidence;
mod exposure;
mod host;
mod lifecycle;
mod process;
mod protocol;

pub use authority::{
    CallAuthorityObservation, CallBinding, CallPlan, CallPlanError, CallRequestView, GateDecision,
    argument_identity, plan_call,
};
pub use config::{
    ConfigError, McpHostConfig, McpHostConfigDraft, McpLimits, ServerPosture, ToolPolicy,
    load_generated_config, parse_config, validate_config,
};
pub use context::{InstructionContribution, InstructionContributionError, admit_instructions};
pub use discovery::{
    DiscoveredCatalog, DiscoveredTool, DiscoveryError, DiscoveryRequest, validate_discovery,
};
pub use evidence::{
    CancellationReceiptState, McpHostReceipt, McpHostReceiptDraft, ReceiptError, ReceiptLeakClass,
    project_receipt,
};
pub use exposure::{
    BuiltInToolSource, ChildToolAuthority, ExactMutationAuthority, ExposedMcpTool,
    ExposureCollision, ExposureDenial, McpExposureCatalog, McpExposureDescriptor,
    McpExposureReceipt, McpExposureRequest, ReadOnlyHint, RejectedExposure,
    project_child_mcp_exposure, reduce_mcp_exposure,
};
pub use host::{
    CallOutput, CredentialService, CredentialServiceError, HostError, HostErrorKind,
    LaunchAuthority, McpClientHost, ResolvedCredentials, ServerUnavailable,
};
pub use lifecycle::{
    FailureClass, HostReadiness, LifecycleBinding, LifecycleError, LifecycleEvent, LifecyclePhase,
    LifecycleState, ServerReadinessEntry, TeardownState, initial_lifecycle, reduce_host_readiness,
    reduce_lifecycle,
};
pub use process::{ProcessScope, TeardownObservation};
pub use protocol::MCP_PROTOCOL_VERSION;
