/// Typed issue selected before a concrete adapter may perform host effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterIssue {
    /// No issue was observed.
    None,
    /// Provider registry metadata is missing.
    MissingProviderRegistry,
    /// Accepted tool contract metadata is missing.
    MissingToolContract,
    /// Runtime policy metadata is missing.
    MissingPolicy,
    /// Capability policy or host approval denied the request.
    DeniedCapability,
    /// Sandbox metadata is missing.
    MissingSandbox,
    /// Sandbox metadata does not match the request.
    SandboxMismatch,
    /// Safe receipt contract metadata is missing.
    MissingReceiptContract,
    /// Runtime event contract metadata is missing.
    MissingEventContract,
    /// Host approval is required but absent.
    MissingHostApproval,
    /// Credentials required by the adapter are absent.
    MissingCredentials,
    /// Adapter output failed boundary validation.
    MalformedOutput,
    /// Adapter dispatch timed out.
    TimedOut,
    /// Adapter dispatch was cancelled.
    Cancelled,
}

impl AdapterIssue {
    /// Returns the stable issue label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::MissingProviderRegistry => "missing_provider_registry",
            Self::MissingToolContract => "missing_tool_contract",
            Self::MissingPolicy => "missing_policy",
            Self::DeniedCapability => "denied_capability",
            Self::MissingSandbox => "missing_sandbox",
            Self::SandboxMismatch => "sandbox_mismatch",
            Self::MissingReceiptContract => "missing_receipt_contract",
            Self::MissingEventContract => "missing_event_contract",
            Self::MissingHostApproval => "missing_host_approval",
            Self::MissingCredentials => "missing_credentials",
            Self::MalformedOutput => "malformed_output",
            Self::TimedOut => "timed_out",
            Self::Cancelled => "cancelled",
        }
    }

    pub(super) const fn receipt_status(self) -> animus_adapters::ReceiptStatus {
        match self {
            Self::None => animus_adapters::ReceiptStatus::Completed,
            Self::DeniedCapability => animus_adapters::ReceiptStatus::Denied,
            Self::MalformedOutput | Self::TimedOut | Self::Cancelled => {
                animus_adapters::ReceiptStatus::Failed
            }
            Self::MissingProviderRegistry
            | Self::MissingToolContract
            | Self::MissingPolicy
            | Self::MissingSandbox
            | Self::SandboxMismatch
            | Self::MissingReceiptContract
            | Self::MissingEventContract
            | Self::MissingHostApproval
            | Self::MissingCredentials => animus_adapters::ReceiptStatus::Blocked,
        }
    }

    pub(super) const fn receipt_issue_code(self) -> animus_adapters::ReceiptIssueCode {
        match self {
            Self::None => animus_adapters::ReceiptIssueCode::None,
            Self::DeniedCapability => animus_adapters::ReceiptIssueCode::AuthorizationDenied,
            Self::MalformedOutput | Self::TimedOut | Self::Cancelled => {
                animus_adapters::ReceiptIssueCode::AdapterError
            }
            Self::MissingProviderRegistry
            | Self::MissingToolContract
            | Self::MissingPolicy
            | Self::MissingSandbox
            | Self::SandboxMismatch
            | Self::MissingReceiptContract
            | Self::MissingEventContract
            | Self::MissingHostApproval
            | Self::MissingCredentials => animus_adapters::ReceiptIssueCode::RuntimeBlocked,
        }
    }
}

/// Result verdict for a std-shell adapter plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterVerdict {
    /// All prerequisites are satisfied and dispatch may proceed.
    Allowed,
    /// Dispatch must not proceed because a typed issue was selected.
    Blocked {
        /// Typed issue that blocked dispatch.
        issue: AdapterIssue,
    },
}

impl AdapterVerdict {
    /// Returns true when the adapter may dispatch.
    #[must_use]
    pub const fn is_allowed(self) -> bool {
        match self {
            Self::Allowed => true,
            Self::Blocked { .. } => false,
        }
    }

    /// Returns the typed issue for this verdict.
    #[must_use]
    pub const fn issue(self) -> AdapterIssue {
        match self {
            Self::Allowed => AdapterIssue::None,
            Self::Blocked { issue } => issue,
        }
    }
}

/// Pure effect plan produced before a std-shell adapter performs work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterPlan {
    family: super::taxonomy::AdapterFamily,
    verdict: AdapterVerdict,
    receipt: animus_adapters::SafeReceipt,
}

impl AdapterPlan {
    pub(super) fn new(
        family: super::taxonomy::AdapterFamily,
        verdict: AdapterVerdict,
        receipt: animus_adapters::SafeReceipt,
    ) -> Self {
        Self {
            family,
            verdict,
            receipt,
        }
    }

    /// Returns the planned adapter family.
    #[must_use]
    pub const fn family(&self) -> super::taxonomy::AdapterFamily {
        self.family
    }

    /// Returns the plan verdict.
    #[must_use]
    pub const fn verdict(&self) -> AdapterVerdict {
        self.verdict
    }

    /// Returns the safe receipt for the plan.
    #[must_use]
    pub const fn receipt(&self) -> &animus_adapters::SafeReceipt {
        &self.receipt
    }
}
