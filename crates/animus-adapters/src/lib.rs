//! Adapter contracts for swappable `animus` components.
//!
//! This crate is `#![no_std]` and uses `alloc` for owned boundary DTOs. It is
//! intentionally implementation-free: hosts embed `animus-core`, implement these
//! ports for their model/tool/runtime components, and choose `animus-std` only
//! when they want the default standard-library shell.

#![no_std]
#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![cfg_attr(animus_octet, allow(tigerstyle::excessive_file_length))]
#![allow(unknown_lints)]
#![allow(module_file_count)]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

mod actor;
mod aliases;
mod artifact;
mod bridge;
mod child;
mod codex;
mod dataspace;
mod effect;
mod flow;
mod front;
/// Optional OpenClaw-inspired gateway control-plane DTOs.
pub mod gateway;
mod hostcontrol;
mod invocation;
mod ledger;
mod manifest;
mod mcpapp;
mod model;
mod multimodal;
/// Optional OpenClaw-inspired node capability-host DTOs.
pub mod node;
mod observability;
/// OpenClaw-inspired security audit and plugin guardrail DTOs.
pub mod openclaw_guardrails;
/// OpenClaw-inspired session isolation and session-tool planning DTOs.
pub mod openclaw_session;
/// Public orchestration adapter-family DTOs and service contracts.
pub mod orchestration;
mod port;
mod profile;
mod prompt;
mod queue;
mod recover;
mod resource;
mod review;
/// Transition shim for the dedicated `animus-router` crate.
pub mod router;
mod safe;
mod security;
#[cfg(test)]
mod security_tests;
mod skill;
mod status;
mod timed;
pub mod tool;
/// Adapter field validation helpers shared by `animus-adapters` and
/// downstream crates such as `animus-router`.
pub mod validation;
mod versioning;
mod workspace;

pub use aliases::*;
pub use aliases::{
    DEFAULT_MODEL_PROMPT_IMAGE_BYTE_BUDGET, MAX_MODEL_IMAGE_BYTE_COUNT, ModelContentPart,
    ModelContentPartDraft, ModelImageDetailPreference, ModelImageMediaType, ModelImageMetadata,
    ModelImageMetadataDraft, ModelImageOutcome, ModelImageOutcomeStatus, ModelImagePart,
    ModelImagePartDraft, ModelImagePayloadError, ModelImagePayloadHandle,
    ModelImagePayloadHandleDraft, ModelImagePayloadHandleKind, ModelImagePayloadService,
    ModelImageReceipt, ModelImageReceiptDraft, ModelImageSourceTrust, ModelInputModality,
    ModelMultimodalDispatchPlan, ModelMultimodalError, ModelPromptAssembly,
    ModelPromptAssemblyPolicy, ModelProviderNativePart, ModelProviderNativeRequest,
    ModelProviderSerializationMode, ModelProviderSerializationPolicy,
    ModelProviderSerializationPolicyDraft, ModelTextPart, ModelUnsafeContentReason,
    SelectedModelInputCapabilities, SelectedModelInputCapabilitiesDraft,
    assemble_model_content_parts, prepare_model_multimodal_dispatch,
    serialize_model_provider_native_fixture,
};
pub use bridge::{
    AcpDescriptor, AcpDescriptorDraft, AcpPermissionMode, AcpSessionCorrelation,
    AcpToolPassThroughPolicy, InteropError, InteropOperationKind, InteropReceipt,
    InteropReceiptDraft, McpAdvertisementDecision, McpAdvertisementInput, McpArgumentStatus,
    McpDescriptor, McpDescriptorDraft, McpDispatchDecision, McpDispatchRequest, McpMutationStatus,
    McpReceipt, McpReceiptDraft, McpReceiptTruncationState, McpServerTrust, McpToolMapping,
    McpToolMappingDraft, McpTransportDecision, McpTransportFailureClass, McpTransportKind,
    McpTransportState, PROHIBITED_INTEROP_EFFECTS, ToolPassThroughInput, ToolPassThroughResolution,
    evaluate_mcp_dispatch, evaluate_mcp_transport, plan_mcp_advertisement,
    resolve_tool_pass_through, validate_interop_receipt_text, validate_mcp_receipt_text,
};
pub use codex::{
    DiffReviewActionKind, DiffReviewDenialReason, DiffReviewError, DiffReviewEvidence,
    DiffReviewEvidenceDraft, DiffReviewEvidenceKind, DiffReviewFinding, DiffReviewFindingCategory,
    DiffReviewFindingDraft, DiffReviewFindingSeverity, DiffReviewFindingState,
    DiffReviewFindingStateRecord, DiffReviewPolicyDecision, DiffReviewPolicyProfile,
    DiffReviewReport, DiffReviewReportDraft, DiffReviewRequest, DiffReviewRequestDraft,
    DiffReviewSelector, DiffReviewSelectorDraft, DiffReviewSelectorKind, DiffReviewSpan,
    DiffReviewSpanDraft, PROHIBITED_DIFF_REVIEW_EFFECTS, compare_diff_review_rerun,
};
pub use dataspace::{
    ActorKind, AdapterService, Assertion, AssertionDraft, AssertionKind, AssertionRedaction,
    AssertionStatus, OrchestrationEvent, OrchestrationEventDraft, OrchestrationEventKind, Scope,
    ScopeDraft, Subscription, SubscriptionDraft, SubscriptionIssue,
};
pub use front::*;
pub use hostcontrol::{
    HostControlEnvelope, HostControlError, HostEvent, HostEventKind, HostEventSubscription,
    HostOperation, HostOutputReadDraft, HostOutputSource, HostOutputView, HostRedactionStatus,
    HostResourceKind, HostSelector, HostSelectorDraft, HostSemanticStatus, HostTopologyResource,
    HostTopologyResourceDraft, HostTopologySnapshot, HostTopologySnapshotDraft,
    HostTruncationStatus, HostTypedError, HostUnknownFields, HostWaitOutcome, HostWaitOutcomeKind,
    HostWaitRequest, LayoutApplyRequest, LayoutApplyResponse, LayoutAxis, LayoutCommandDescriptor,
    LayoutCommandDescriptorDraft, LayoutNode, LayoutPaneDraft, SelectorLifetime, TopologyCommand,
    TopologyCommandDraft, filter_host_events, resolve_host_wait,
};
pub use invocation::{
    ADAPTER_CALL_OUTCOME_CONTRACT_IDENTITY, AdapterCallFailureClass, AdapterCallFailureMetadata,
    AdapterCallFailureMetadataDraft, AdapterCallOutcome, ComponentOutcomeAdapter,
    ModelOutcomeAdapter, ToolOutcomeAdapter,
};
pub use recover::{
    CheckpointBackendKind, CheckpointDraft, CheckpointError, CheckpointFailureClass,
    CheckpointHash, CheckpointOperation, CheckpointPolicy, CheckpointReceipt,
    CheckpointReceiptDraft, CheckpointReceiptLeak, CheckpointRecord, CheckpointService,
    CheckpointServiceRequest, CheckpointServiceResponse, CheckpointServiceState, CheckpointStatus,
    MutationProtection, ProtectedMutationDecision, ProtectedMutationInput,
    RollbackConfirmationStatus, RollbackValidation, RollbackValidationInput,
    evaluate_protected_mutation, validate_checkpoint_receipt_text, validate_rollback,
};
pub use versioning::{BoundaryContractIdentity, parse_boundary_contract_identity};

pub use effect::{
    AuthorizationContext, AuthorizationIssue, AuthorizationLeak, AuthorizationReceipt,
    AuthorizationStatus, CancellationCheck, CancellationDecision, CancellationService,
    CapabilityClass, CapabilityDecision, CapabilityDecisionRequest, CapabilityDecisionService,
    CapabilityPolicyContract, CapabilityRequest, CaveatSummary, ConfirmationActionKind,
    ConfirmationBinding, ConfirmationBindingDraft, ConfirmationBroker, ConfirmationBrokerClass,
    ConfirmationDecision, ConfirmationEvaluation, ConfirmationExpiry, ConfirmationExpiryClass,
    ConfirmationIssue, ConfirmationLevel, ConfirmationReceipt, ConfirmationReceiptDraft,
    ConfirmationReceiptLeak, ConfirmationRequest, ConfirmationRequestDraft, ConfirmationStatus,
    ConfirmationTick, ConfirmationTimingClass, ExecPolicyCommandIntent,
    ExecPolicyCommandIntentDraft, ExecPolicyDecision, ExecPolicyError, ExecPolicyEvaluation,
    ExecPolicyRequest, ExecPolicyRule, ExecPolicyRuleDraft, ExecPolicyRuleEvidence,
    ExecPolicySandboxContext, ExecPolicySandboxContextDraft, ExecPolicyShellAnalysis,
    ExecPolicyShellAnalysisKind, GuardedAction, HostRequest, HostService, LocalCaveat,
    LocalCaveatDecision, LocalPolicyLayer, ModelProviderService, OutcomeKind,
    PermissionDecisionReason, PermissionDecisionStatus, PermissionKernelReceipt,
    PermissionKernelReceiptDraft, PermissionMode, PermissionModeDefaultReason,
    PermissionModeDefaultRequest, PermissionModeDefaultResolution, PermissionOutcome,
    PermissionPatternMatchKind, PermissionPatternRule, PermissionPatternRuleDraft,
    PermissionPolicy, PermissionPolicyDraft, PermissionPolicyError, PermissionPromptDecision,
    PermissionPromptDecisionDraft, PermissionPromptRequest, PermissionPromptRequestDraft,
    PermissionPromptStatus, PermissionReceipt, PermissionReceiptDraft, PermissionReceiptLeak,
    PermissionRepeatedActionInput, PermissionRepeatedActionPolicy,
    PermissionRepeatedActionResolution, PermissionRepeatedActionStatus, PermissionResolution,
    PermissionResolutionRequest, PermissionSessionAllowlistEntry,
    PermissionSessionAllowlistEntryDraft, PermissionSessionAllowlistStatus,
    PermissionSessionAllowlistValidation, PermissionTrustScope, PermissionTrustStatus,
    PromptDecisionOutcome, PromptSummaryPolicy, Reason, Receipt, ReceiptDraft, RedactionMarker,
    RuntimePolicyDecision, RuntimePolicyRequest, RuntimePolicyService, SAFE_HASH_HEX_CHARS,
    SAFE_RECEIPT_CONTRACT_IDENTITY, SafeHash, SandboxConstraint, SandboxConstraintClass,
    SandboxCowDisposition, SandboxDescriptor, SandboxDescriptorDraft, SandboxDeterminismPolicy,
    SandboxDispatchDecision, SandboxDispatchIssue, SandboxDispatchReceipt,
    SandboxDispatchReceiptDraft, SandboxDispatchStatus, SandboxEnforcementRequirement,
    SandboxEnvironmentPolicy, SandboxFilesystemMode, SandboxHttpPolicy, SandboxNetworkPolicy,
    SandboxPlatformCapability, SandboxPlatformSupport, SandboxPlatformSupportDraft, SandboxPolicy,
    SandboxPolicyDraft, SandboxProcessProfile, SandboxReceiptLeak, SandboxState,
    SandboxWorkdirMode, ServiceKind, ServiceOutcome, ToolExecutionService, UcanGrant,
    analyze_exec_policy_shell_wrapper, authorize_capability, default_deny_confirmation,
    detect_permission_repeated_action, evaluate_confirmation_decision,
    evaluate_confirmation_outcome, evaluate_exec_policy, evaluate_sandbox_dispatch,
    resolve_permission, resolve_permission_mode_default, validate_authorization_receipt_text,
    validate_confirmation_receipt_text, validate_permission_receipt_text,
    validate_permission_session_allowlist, validate_sandbox_receipt_text,
};
pub use manifest::{
    LaunchPolicy, LaunchPolicyDraft, NetworkPolicy, PROHIBITED_PLUGIN_MANIFEST_EFFECTS,
    PluginCompatibility, PluginCompatibilityDraft, PluginDescriptor, PluginDescriptorDraft,
    PluginDescriptorError, PluginDiagnostic, PluginDiagnosticKind, PluginDiagnosticSeverity,
    PluginHostCompatibilityDraft, PluginInvocationContext, PluginInvocationContextDraft,
    PluginInvocationContextLeak, PluginRegistryOperationDraft, PluginRegistryOperationKind,
    PluginRegistryPlan, PluginRegistryPlanStatus, PluginRole, PluginRoleDraft, PluginRoleKind,
    RedactionPolicy, RedactionPolicyDraft, RuntimeKind, SandboxMode, SandboxRequirements,
    SandboxRequirementsDraft, ToolDescriptor, ToolDescriptorDraft, WorkingDirectoryMode,
};
pub use observability::{
    LogLevel, LogRecord, LogRecordDraft, LoggingAdapter, MetricKind, MetricLabel, MetricLabelDraft,
    MetricSample, MetricSampleDraft, MetricUnit, MetricsAdapter,
};
/// Observability delivery policy for root-facade consumers.
pub type ObservabilityDeliveryPolicy = observability::DeliveryPolicy;
/// Observability validation error for root-facade consumers.
pub type ObservabilityError = observability::Error;
/// Observability guarded-dispatch decision for root-facade consumers.
pub type ObservabilityGuardDecision = observability::GuardDecision;
/// Observability sink failure class for root-facade consumers.
pub type ObservabilitySinkFailureClass = observability::SinkFailureClass;
/// Observability sink outcome for root-facade consumers.
pub type ObservabilitySinkOutcome = observability::SinkOutcome;
/// Evaluates observability delivery for root-facade consumers.
#[must_use]
pub fn evaluate_observability_delivery(
    policy: ObservabilityDeliveryPolicy,
    outcome: &ObservabilitySinkOutcome,
) -> ObservabilityGuardDecision {
    observability::evaluate_delivery(policy, outcome)
}
pub use port::{ComponentAdapter, HostStatusAdapter, ModelAdapter, ToolAdapter};
pub use review::{
    ReviewDiagnostic, ReviewDiagnosticCode, ReviewFinding, ReviewMetricFinding, ReviewMetricRecord,
    ReviewMetricRecordDraft, ReviewMetricsSummary, ReviewPromotionCandidate, ReviewReport,
    ReviewReportDraft, ReviewReportError, ReviewSection, ReviewVerdict,
    append_review_metric_record, parse_review_report, review_promotion_candidates,
    summarize_review_metrics,
};
/// Plugin manifest toolset-kind descriptor label.
pub type PluginToolsetKind = manifest::ToolsetKind;
/// Plugin manifest host trust status.
pub type PluginTrustStatus = manifest::TrustStatus;
pub use child::{
    ExternalSubagentDescriptor, ExternalSubagentDescriptorDraft, ExternalSubagentProtocol,
    SubagentAggregateReport, SubagentCapabilityDenial, SubagentCapabilityInput,
    SubagentCapabilityResolution, SubagentDelegation, SubagentDelegationDraft,
    SubagentDelegationSource, SubagentError, SubagentEvent, SubagentEventDraft, SubagentEventKind,
    SubagentHandoff, SubagentHandoffDraft, SubagentOutcomeDraft, SubagentOutcomeKind,
    SubagentReturnMode, aggregate_subagent_outcomes, build_subagent_handoff,
    resolve_subagent_capabilities,
};
pub use flow::{
    RecipeAutomationOutcome, RecipeAutomationStatus, RecipeBinding, RecipeBindingDraft,
    RecipeHostEffectKind, RecipeInfluence, RecipeInfluenceStatus, RecipeOutputDraft,
    RecipeOutputFailure, RecipeOutputFieldDraft, RecipeParameter, RecipeParameterDraft,
    RecipeParameterInput, RecipeParameterInputDraft, RecipeParameterInputKind, RecipeParameterKind,
    RecipeProvenanceDraft, RecipeResponseField, RecipeResponseFieldDraft, RecipeResponseFieldKind,
    RecipeResponseSchema, RecipeResponseSchemaDraft, RecipeRetryCheck, RecipeRetryCheckDraft,
    RecipeRetryDenialReason, RecipeRetryPlan, RecipeRetryPlanDraft, RecipeRetryPolicy,
    RecipeRetryPolicyDraft, RecipeRetryRequest, RecipeRetryRequestStatus, RecipeSourceKind,
    RecipeTrustReceipt, RecipeTrustStatus, RecipeWorkflow, RecipeWorkflowDraft,
    RecipeWorkflowError, SAFE_TEMPLATE_BUILTINS, bind_recipe_parameters, plan_recipe_retry,
    validate_recipe_output,
};
pub use prompt::{
    ContextAdapterAvailability, ContextAdapterFamilyManifest, ContextAdapterOutcomeClass,
    ContextAdapterPolicyDecision, ContextAdapterRole, ContextAdapterRoleDraft,
    ContextAdapterRoleRecord, ContextLineRange, ContextOversizedPolicy, ContextReference,
    ContextReferenceBudget, ContextReferenceCounts, ContextReferenceError,
    ContextReferenceFragment, ContextReferenceKind, ContextReferencePolicy,
    ContextReferencePolicyDecision, ContextReferenceProvenance, ContextReferenceProvenanceDraft,
    ContextReferenceStatus, ContextRenderMode, ContextResolutionPlan, ContextResolverKind,
    ContextResolverSet, ContextSourceTrust, ContextTruncationState, DiffContextResolver,
    DirectoryContextResolver, FileContextResolver, ImageContextResolver, ProfileAssembly,
    ProfileAssemblyDiagnostic, ProfileAssemblyPolicy, ProfileCapability, ProfileCapabilityBinding,
    ProfileCapabilityBindingStatus, ProfileChainDecision, ProfileChainPlan, ProfileChainRequest,
    ProfileDescriptor, ProfileDescriptorDraft, ProfileDirectiveDiagnostic, ProfileDirectiveIssue,
    ProfileDirectiveReport, ProfileError, ProfileInclusionPolicy, ProfileMode, ProfileModeBinding,
    ProfileModeBindingRequest, ProfileOmittedReason, ProfileReceipt, ProfileReceiptRedaction,
    ProfileSegment, ProfileSourceAdmission, ProfileSourceTrust, ProfileTrustScope,
    ResolvedContextCandidate, UrlContextResolver, apply_context_budget, assemble_profiles,
    bind_profile_mode, expand_context_reference, parse_context_reference, parse_profile_directives,
    plan_context_resolution, validate_context_resolver_role,
};
pub use queue::{
    BackgroundQueueService, QueueCapabilities, QueueCapabilityKind, QueueContractError,
    QueueControlDraft, QueueControlRequest, QueueEnqueueDraft, QueueEnqueueRequest,
    QueueGroupStatus, QueueGroupStatusDraft, QueueInteractionDraft, QueueInteractionKind,
    QueueInteractionRequest, QueueLogBounds, QueueLogRequest, QueueLogRequestDraft,
    QueueLogViewport, QueueLogViewportDraft, QueueOperationKind, QueueServiceRequest,
    QueueServiceResponse, QueueStatusCountDraft, QueueStatusCounts, QueueTarget, QueueTargetDraft,
    QueueTargetKind, QueueTaskObservation, QueueTaskObservationDraft, QueueTaskStatus,
};
pub use timed::{
    NextRunInput, NextRunPlan, PROHIBITED_SCHEDULE_EFFECTS, ScheduleCadence,
    ScheduleConcurrencyPolicy, ScheduleControlAction, ScheduleControlDraft, ScheduleControlRequest,
    ScheduleError, ScheduleExecutionMode, ScheduledConcurrencyDecision, ScheduledConcurrencyInput,
    ScheduledRunAdmissionDraft, ScheduledRunAdmissionStatus, ScheduledRunDenial,
    ScheduledRunPolicyAdmission, ScheduledRunRecord, ScheduledRunRecordDraft,
    ScheduledRunRuntimeAdmission, ScheduledWorkflow, ScheduledWorkflowDraft, admit_scheduled_run,
    evaluate_scheduled_concurrency, plan_next_run, validate_scheduled_record_text,
};
pub use validation::{
    AdapterContractError, FieldName, MAX_ADAPTER_FIELD_CHARS, MIN_ADAPTER_FIELD_CHARS,
    normalized_adapter_field,
};

#[cfg(test)]
extern crate std;
