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

mod artifact;
mod dataspace;
mod effect;
mod ledger;
mod manifest;
mod mcp_app;
mod model;
mod policy_profile;
mod router;
mod safe;
mod status;
mod tool;
mod validation;
mod workspace;

pub use artifact::BLAKE3_HASH_HEX_CHARS;
pub use dataspace::{
    ActorKind, AdapterService, Assertion, AssertionDraft, AssertionKind, AssertionRedaction,
    AssertionStatus, OrchestrationEvent, OrchestrationEventDraft, OrchestrationEventKind, Scope,
    ScopeDraft, Subscription, SubscriptionDraft, SubscriptionIssue,
};

/// Active envelope version used for Animus-owned BLAKE3 identities.
pub const ARTIFACT_ENVELOPE_VERSION: u32 = artifact::VERSION;
/// Largest normalized payload accepted by the no-std DTO layer.
pub const MAX_ARTIFACT_PAYLOAD_CHARS: usize = artifact::MAX_PAYLOAD_CHARS;
/// Canonical Animus envelope with deterministic BLAKE3 identity.
pub type ArtifactEnvelope = artifact::Envelope;
/// Borrowed inputs used to build a canonical envelope.
pub type ArtifactEnvelopeDraft<'a> = artifact::Draft<'a>;
/// Validated lowercase BLAKE3 identity.
pub type ArtifactHash = artifact::Hash;
/// Artifact payload class used in canonical envelopes.
pub type ArtifactKind = artifact::Kind;
/// Redaction class controlling payload display.
pub type ArtifactRedactionClass = artifact::RedactionClass;
/// Errors raised while parsing or building safe artifact envelopes.
pub type ArtifactValidationError = artifact::Error;
pub use effect::{
    AuthorizationContext, AuthorizationIssue, AuthorizationLeak, AuthorizationReceipt,
    AuthorizationStatus, CancellationCheck, CancellationDecision, CancellationService,
    CapabilityClass, CapabilityDecision, CapabilityDecisionRequest, CapabilityDecisionService,
    CapabilityPolicyContract, CapabilityRequest, CaveatSummary, ConfirmationBroker,
    ConfirmationDecision, ConfirmationRequest, GuardedAction, HostRequest, HostService,
    LocalCaveat, LocalCaveatDecision, LocalPolicyLayer, ModelProviderService, OutcomeKind, Reason,
    Receipt, ReceiptDraft, RedactionMarker, RuntimePolicyDecision, RuntimePolicyRequest,
    RuntimePolicyService, SAFE_HASH_HEX_CHARS, SafeHash, SandboxConstraint, SandboxState,
    ServiceKind, ServiceOutcome, ToolExecutionService, UcanGrant, authorize_capability,
    default_deny_confirmation, validate_authorization_receipt_text,
};
pub use ledger::SessionLedgerService;

/// Neutral ledger entry owned by Animus adapter boundaries.
pub type LedgerEntry = ledger::Entry;
/// Validated neutral ledger entry identifier.
pub type LedgerEntryId = ledger::EntryId;
/// Kind of neutral ledger entry.
pub type LedgerEntryKind = ledger::EntryKind;
/// Safe replay metadata counts.
pub type LedgerReplayCounts = ledger::ReplayCounts;
/// Replay errors that fail closed without exposing raw adapter-owned payloads.
pub type LedgerReplayError = ledger::Error;
/// Successful replay result for neutral entries.
pub type LedgerReplayReport = ledger::Report;
/// Replay request sent to an injected session-ledger service.
pub type LedgerReplayRequest = ledger::Request;
/// Model-visible context item produced by replay.
pub type ModelContextItem = ledger::ContextItem;
/// Model-visible replay role.
pub type ModelContextRole = ledger::ContextRole;

/// Builds the fail-closed outcome for an absent session-ledger service.
#[must_use]
pub fn missing_session_ledger_service(
    request: &LedgerReplayRequest,
) -> ServiceOutcome<LedgerReplayReport> {
    ledger::missing_service(request)
}

/// Replays neutral entries into model-visible context plus safe counts.
///
/// # Errors
///
/// Returns [`LedgerReplayError`] when the ledger is empty or contains an
/// unsupported or adapter-owned entry marker.
pub fn replay_ledger(entries: &[LedgerEntry]) -> Result<LedgerReplayReport, LedgerReplayError> {
    ledger::replay(entries)
}
pub use manifest::{
    LaunchPolicy, LaunchPolicyDraft, NetworkPolicy, PROHIBITED_PLUGIN_MANIFEST_EFFECTS,
    PluginDescriptor, PluginDescriptorDraft, PluginDescriptorError, RedactionPolicy,
    RedactionPolicyDraft, RuntimeKind, SandboxMode, SandboxRequirements, SandboxRequirementsDraft,
    ToolDescriptor, ToolDescriptorDraft, WorkingDirectoryMode,
};
pub use mcp_app::{MCP_APP_EVALUATION_AXES, PROHIBITED_MCP_APP_VALIDATION_EFFECTS};
/// Adoption decision for MCP Apps-style UI metadata.
pub type McpAppAdoptionDecision = mcp_app::AdoptionDecision;
/// Validated data-only MCP Apps-style UI metadata descriptor.
pub type McpAppDescriptor = mcp_app::Descriptor;
/// Borrowed MCP Apps-style UI metadata before validation.
pub type McpAppDescriptorDraft<'a> = mcp_app::DescriptorDraft<'a>;
/// Data-only MCP Apps evaluation report.
pub type McpAppEvaluation = mcp_app::Evaluation;
/// Matrix axis used to evaluate MCP Apps-style UI metadata.
pub type McpAppEvaluationAxis = mcp_app::EvaluationAxis;
/// Error raised while validating or classifying MCP Apps-style descriptors.
pub type McpAppEvaluationError = mcp_app::EvaluationError;
/// Lifecycle policy for callbacks and UI lifetime.
pub type McpAppLifecyclePolicy = mcp_app::LifecyclePolicy;
/// Admission state for a future MCP Apps host-policy gate.
pub type McpAppPolicyAdmission = mcp_app::PolicyAdmission;
/// Borrowed policy-admission flags for evaluating UI metadata.
pub type McpAppPolicyAdmissionDraft = mcp_app::PolicyAdmissionDraft;
/// Risk reason recorded with an MCP Apps adoption decision.
pub type McpAppRiskReason = mcp_app::RiskReason;
/// Preferred sizing policy for a future UI host.
pub type McpAppSizePolicy = mcp_app::SizePolicy;
/// Result of deterministic policy profile composition.
pub type PolicyCompositionReport = policy_profile::CompositionReport;
/// Diagnostic emitted while composing policy profiles.
pub type PolicyCompositionDiagnostic = policy_profile::CompositionDiagnostic;
/// Diagnostic class emitted while composing policy profiles.
pub type PolicyCompositionDiagnosticKind = policy_profile::DiagnosticKind;
/// Policy descriptor boundary relative to runtime enforcement.
pub type PolicyEnforcementBoundary = policy_profile::EnforcementBoundary;
/// Errors raised while validating policy profiles.
pub type PolicyProfileError = policy_profile::ProfileError;
/// Host-supplied policy/profile descriptor.
pub type PolicyProfile = policy_profile::Profile;
/// Borrowed host-supplied policy/profile descriptor.
pub type PolicyProfileDraft<'a> = policy_profile::ProfileDraft<'a>;
/// Policy descriptor kind.
pub type PolicyProfileKind = policy_profile::PolicyKind;
/// Prompt hygiene issue class.
pub type PolicyPromptHygieneIssue = policy_profile::PromptHygieneIssue;
/// Prompt segment attributed to a policy profile.
pub type PolicyPromptSegment = policy_profile::PromptSegment;
/// Host trust decision for project-local profiles.
pub type ProjectPolicyTrustMode = policy_profile::ProjectTrustMode;
/// Runtime guard decision required for capability enforcement.
pub type RuntimeGuardRequirement = policy_profile::RuntimeGuardRequirement;
/// Source scope for a policy descriptor.
pub type PolicySourceScope = policy_profile::SourceScope;
/// Trust status for a policy descriptor.
pub type PolicyTrustStatus = policy_profile::TrustStatus;
pub use policy_profile::{compose_policy_context, mutation_requires_runtime_guard};
pub use safe::receipt::{BoundedCount, BoundedCountDraft};

/// Largest accepted bounded count value in a safe receipt.
pub const MAX_SAFE_RECEIPT_COUNT: u32 = safe::receipt::MAX_COUNT;
/// Largest accepted safe metadata label length.
pub const MAX_SAFE_RECEIPT_LABEL_CHARS: usize = safe::receipt::MAX_LABEL_CHARS;
/// Schema identifier attached to safe receipt DTOs.
pub const SAFE_RECEIPT_SCHEMA: &str = safe::receipt::SCHEMA;
/// Issue code recorded by a safe receipt.
pub type ReceiptIssueCode = safe::receipt::IssueCode;
/// Unsafe material class rejected by safe receipt validation.
pub type ReceiptLeakClass = safe::receipt::LeakClass;
/// Effect or mutation class observed by a safe receipt.
pub type ReceiptOperation = safe::receipt::Operation;
/// Redaction marker recorded by a safe receipt.
pub type ReceiptRedaction = safe::receipt::Redaction;
/// Status class recorded by a safe receipt.
pub type ReceiptStatus = safe::receipt::Status;
/// Safe runtime/provenance receipt containing metadata, counts, hashes, and redactions only.
pub type SafeReceipt = safe::receipt::Record;
/// Borrowed inputs used to construct a safe receipt.
pub type SafeReceiptDraft<'a> = safe::receipt::Draft<'a>;
/// Errors raised by safe receipt validation.
pub type SafeReceiptError = safe::receipt::Error;
pub use status::evidence::SAFE_EVIDENCE_HASH_HEX_CHARS;

/// Optional native host session reference.
pub type AgentSessionReference = status::identity::Reference;
/// Borrowed session-reference input.
pub type AgentSessionReferenceDraft<'a> = status::identity::ReferenceDraft<'a>;
/// Supported session reference kind.
pub type AgentSessionReferenceKind = status::identity::Kind;
/// Validated semantic status observation supplied by a host adapter.
pub type AgentStatusObservation = status::report::Observation;
/// Borrowed status observation input.
pub type AgentStatusObservationDraft<'a> = status::report::ObservationDraft<'a>;
/// Observation authority class supplied by a host integration.
pub type AgentStatusObservationKind = status::lifecycle::ObservationKind;
/// Presentation-only status metadata.
pub type AgentStatusPresentation = status::view::Presentation;
/// Borrowed presentation metadata input.
pub type AgentStatusPresentationDraft<'a> = status::view::PresentationDraft<'a>;
/// Batch returned by a host status-observation adapter.
pub type HostStatusObservationBatch = status::report::HostObservationBatch;
/// Request sent to a host status-observation adapter.
pub type HostStatusRequest = status::report::HostRequest;
/// Semantic lifecycle state shared by runtime hosts.
pub type SemanticAgentState = status::lifecycle::SemanticAgent;
/// Logical monotonic sequence used for deterministic freshness checks.
pub type StatusSequence = status::lifecycle::Sequence;
/// Logical TTL for an observation.
pub type StatusTtl = status::lifecycle::Ttl;
pub use validation::{AdapterContractError, MAX_ADAPTER_FIELD_CHARS, MIN_ADAPTER_FIELD_CHARS};
/// Validated speculative workspace descriptor.
pub type SpeculativeWorkspaceDescriptor = workspace::intent::Descriptor;
/// Borrowed speculative workspace descriptor input.
pub type SpeculativeWorkspaceDescriptorDraft<'a> = workspace::intent::DescriptorDraft<'a>;
/// Abort behavior requested for a workspace.
pub type WorkspaceAbortPolicy = workspace::rules::AbortPolicy;
/// Validated workspace adapter capability advertisement.
pub type WorkspaceAdapterCapabilities = workspace::adapter::Capabilities;
/// Borrowed workspace adapter capability input.
pub type WorkspaceAdapterCapabilitiesDraft<'a> = workspace::adapter::CapabilitiesDraft<'a>;
/// Backend kind advertised by a workspace adapter.
pub type WorkspaceAdapterKind = workspace::adapter::Kind;
/// Typed workspace adapter outcome.
pub type WorkspaceAdapterOutcome = workspace::result::Outcome;
/// Borrowed workspace adapter outcome input.
pub type WorkspaceAdapterOutcomeDraft<'a> = workspace::result::OutcomeDraft<'a>;
/// Workspace adapter outcome kind.
pub type WorkspaceAdapterOutcomeKind = workspace::result::Kind;
/// Availability state advertised by a workspace adapter.
pub type WorkspaceAdapterState = workspace::adapter::State;
/// Capability class for speculative workspace adapters.
pub type WorkspaceCapability = workspace::adapter::Capability;
/// Cleanup behavior required after workspace operations.
pub type WorkspaceCleanupPolicy = workspace::rules::CleanupPolicy;
/// Target for a successful workspace commit.
pub type WorkspaceCommitTarget = workspace::rules::CommitTarget;
/// Conflict behavior for workspace commit planning.
pub type WorkspaceConflictPolicy = workspace::rules::ConflictPolicy;
/// Policy for ignored or local files in speculative workspaces.
pub type WorkspaceFileInclusionPolicy = workspace::rules::FileInclusionPolicy;
/// Requested workspace operation.
pub type WorkspaceOperation = workspace::rules::Operation;

/// Request sent from the harness to a model component.
pub type ModelRequest = model::Request;

/// Response returned by a model component.
pub type ModelResponse = model::Response;
pub use router::{
    DurationBucket, FallbackEligibility, FallbackPolicy, OutputContract, PrivacyBand, QuorumClass,
    RouteFamily, RouteResult, RouteResultStatus, SelectionDenial, SignalBand, TaskShape,
    select_recipe,
};
/// Hard model-router execution bounds.
pub type ExecutionBounds = router::Bounds;
/// Borrowed fields used to validate model-router execution bounds.
pub type ExecutionBoundsDraft<'a> = router::BoundsDraft<'a>;
/// Safe model catalog projection for router selection.
pub type ModelCatalogEntry = router::CatalogEntry;
/// Borrowed safe model catalog projection fields.
pub type ModelCatalogEntryDraft<'a> = router::CatalogEntryDraft<'a>;
/// Model role used inside a router recipe.
pub type ModelRole = router::Role;
/// Borrowed request-signal projection fields.
pub type RequestSignalDraft<'a> = router::SignalDraft<'a>;
/// Bounded request signals for router selection.
pub type RequestSignals = router::Signals;
/// Router budget state.
pub type RouterBudget = router::Budget;
/// Safe router-recipe receipt.
pub type RouterReceipt = router::Receipt;
/// Borrowed fields used to build a safe router receipt.
pub type RouterReceiptDraft<'a> = router::ReceiptDraft<'a>;
/// Router receipt leak class.
pub type RouterReceiptLeak = router::ReceiptLeak;
/// Router recipe contract checked before provider dispatch.
pub type RouterRecipe = router::Recipe;
/// Borrowed fields used to validate a router recipe.
pub type RouterRecipeDraft<'a> = router::RecipeDraft<'a>;
/// Router receipt redaction marker.
pub type RouterRedaction = router::Redaction;
/// Router validation error.
pub type RouterValidationError = router::ValidationError;
/// Selected router plan.
pub type SelectedRoutePlan = router::SelectedPlan;
/// Timeout/deadline class for bounded host execution.
pub type TimeoutClass = router::DeadlineClass;
/// Validates safe router receipt text for known leak sentinels.
///
/// # Errors
///
/// Returns [`RouterReceiptLeak`] when unsafe receipt text is detected.
pub fn validate_router_receipt_text(text: &str) -> Result<(), RouterReceiptLeak> {
    router::validate_receipt_text(text)
}
/// Borrowed final-output validation facts.
pub type RouteResultDraft<'a> = router::outcome::ResultDraft<'a>;

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
