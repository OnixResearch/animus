mod audit;
mod authority;
mod input;
mod result;
mod service;
mod taxonomy;

#[cfg(test)]
mod tests;

pub use audit::{Receipt, ReceiptDraft, SAFE_HASH_HEX_CHARS, SafeHash};
pub use authority::{
    AuthorizationContext, AuthorizationIssue, AuthorizationLeak, AuthorizationReceipt,
    AuthorizationStatus, CapabilityPolicyContract, CapabilityRequest, CaveatSummary, GuardedAction,
    LocalCaveat, LocalCaveatDecision, LocalPolicyLayer, SandboxConstraint, SandboxState, UcanGrant,
    authorize_capability, validate_authorization_receipt_text,
};
pub use input::{
    CancellationCheck, CapabilityDecisionRequest, ConfirmationRequest, HostRequest,
    RuntimePolicyRequest,
};
pub use result::{OutcomeKind, Reason, ServiceOutcome};
pub use service::{
    CancellationService, CapabilityDecisionService, ConfirmationBroker, HostService,
    ModelProviderService, RuntimePolicyService, ToolExecutionService, default_deny_confirmation,
};
pub use taxonomy::{CapabilityClass, RedactionMarker, ServiceKind};

/// Capability decision returned by an injected policy service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityDecision {
    /// Capability is allowed.
    Allow,
    /// Capability is denied.
    Deny {
        /// Reason class for denial.
        reason: Reason,
    },
}

/// Cancellation decision returned by an injected cancellation service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationDecision {
    /// Continue evaluating the turn.
    Continue,
    /// Cancel before running later effects.
    Cancel {
        /// Reason class for cancellation.
        reason: Reason,
    },
}

/// Confirmation decision returned by an injected confirmation broker.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConfirmationDecision {
    /// Protected action is approved.
    Approved,
    /// Protected action is denied.
    Denied {
        /// Reason class for denial.
        reason: Reason,
    },
}

/// Runtime-policy decision returned by an injected runtime-policy service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimePolicyDecision {
    /// Runtime policy allows the request.
    Allow,
    /// Runtime policy denies the request.
    Deny {
        /// Reason class for denial.
        reason: Reason,
    },
}
