/// Capability decision returned by an injected policy service.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityDecision {
    /// Capability is allowed.
    Allow,
    /// Capability is denied.
    Deny {
        /// Reason class for denial.
        reason: super::Reason,
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
        reason: super::Reason,
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
        reason: super::Reason,
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
        reason: super::Reason,
    },
}
