/// State of a required prerequisite gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateState {
    /// The prerequisite is present and satisfied.
    Satisfied,
    /// The prerequisite contract or service is absent.
    Missing,
    /// Policy or user approval explicitly denied the action.
    Denied,
    /// The prerequisite was present but incompatible with the request.
    Mismatch,
}

/// Credential prerequisite state for a concrete adapter request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialState {
    /// The adapter does not require credentials.
    NotRequired,
    /// Required credentials are available to the host adapter.
    Present,
    /// Required credentials are missing.
    Missing,
}

/// Output-validation state for a concrete adapter request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputState {
    /// Adapter output is valid at its boundary.
    Valid,
    /// Adapter output is malformed and must fail closed.
    Malformed,
}

/// Termination state observed before or during adapter dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminationState {
    /// Dispatch can continue.
    Continue,
    /// Dispatch timed out.
    TimedOut,
    /// Dispatch was cancelled.
    Cancelled,
}

/// Pure prerequisite snapshot evaluated before a std-shell adapter performs effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectPrerequisites {
    provider_registry: GateState,
    tool_contract: GateState,
    policy: GateState,
    sandbox: GateState,
    receipt_contract: GateState,
    event_contract: GateState,
    host_approval: GateState,
    credentials: CredentialState,
    output: OutputState,
    termination: TerminationState,
}

impl EffectPrerequisites {
    /// Builds a prerequisite snapshot with every required gate satisfied.
    #[must_use]
    pub const fn satisfied() -> Self {
        Self {
            provider_registry: GateState::Satisfied,
            tool_contract: GateState::Satisfied,
            policy: GateState::Satisfied,
            sandbox: GateState::Satisfied,
            receipt_contract: GateState::Satisfied,
            event_contract: GateState::Satisfied,
            host_approval: GateState::Satisfied,
            credentials: CredentialState::NotRequired,
            output: OutputState::Valid,
            termination: TerminationState::Continue,
        }
    }

    /// Marks provider-registry metadata absent.
    #[must_use]
    pub const fn without_provider_registry(mut self) -> Self {
        self.provider_registry = GateState::Missing;
        self
    }

    /// Marks accepted tool contracts absent.
    #[must_use]
    pub const fn without_tool_contract(mut self) -> Self {
        self.tool_contract = GateState::Missing;
        self
    }

    /// Marks runtime policy absent.
    #[must_use]
    pub const fn without_policy(mut self) -> Self {
        self.policy = GateState::Missing;
        self
    }

    /// Marks runtime policy denied.
    #[must_use]
    pub const fn with_policy_denied(mut self) -> Self {
        self.policy = GateState::Denied;
        self
    }

    /// Marks sandbox compatibility missing.
    #[must_use]
    pub const fn without_sandbox(mut self) -> Self {
        self.sandbox = GateState::Missing;
        self
    }

    /// Marks sandbox compatibility mismatched.
    #[must_use]
    pub const fn with_sandbox_mismatch(mut self) -> Self {
        self.sandbox = GateState::Mismatch;
        self
    }

    /// Marks safe receipt contracts absent.
    #[must_use]
    pub const fn without_receipt_contract(mut self) -> Self {
        self.receipt_contract = GateState::Missing;
        self
    }

    /// Marks runtime event contracts absent.
    #[must_use]
    pub const fn without_event_contract(mut self) -> Self {
        self.event_contract = GateState::Missing;
        self
    }

    /// Marks host approval absent.
    #[must_use]
    pub const fn without_host_approval(mut self) -> Self {
        self.host_approval = GateState::Missing;
        self
    }

    /// Marks host approval denied.
    #[must_use]
    pub const fn with_host_approval_denied(mut self) -> Self {
        self.host_approval = GateState::Denied;
        self
    }

    /// Marks credentials present for adapters that require them.
    #[must_use]
    pub const fn with_credentials_present(mut self) -> Self {
        self.credentials = CredentialState::Present;
        self
    }

    /// Marks required credentials missing.
    #[must_use]
    pub const fn with_missing_credentials(mut self) -> Self {
        self.credentials = CredentialState::Missing;
        self
    }

    /// Marks adapter output malformed.
    #[must_use]
    pub const fn with_malformed_output(mut self) -> Self {
        self.output = OutputState::Malformed;
        self
    }

    /// Marks dispatch timed out.
    #[must_use]
    pub const fn with_timeout(mut self) -> Self {
        self.termination = TerminationState::TimedOut;
        self
    }

    /// Marks dispatch cancelled.
    #[must_use]
    pub const fn with_cancellation(mut self) -> Self {
        self.termination = TerminationState::Cancelled;
        self
    }

    /// Returns the provider registry gate state.
    #[must_use]
    pub const fn provider_registry(self) -> GateState {
        self.provider_registry
    }

    /// Returns the tool contract gate state.
    #[must_use]
    pub const fn tool_contract(self) -> GateState {
        self.tool_contract
    }

    /// Returns the policy gate state.
    #[must_use]
    pub const fn policy(self) -> GateState {
        self.policy
    }

    /// Returns the sandbox gate state.
    #[must_use]
    pub const fn sandbox(self) -> GateState {
        self.sandbox
    }

    /// Returns the safe receipt contract gate state.
    #[must_use]
    pub const fn receipt_contract(self) -> GateState {
        self.receipt_contract
    }

    /// Returns the runtime event contract gate state.
    #[must_use]
    pub const fn event_contract(self) -> GateState {
        self.event_contract
    }

    /// Returns the host approval gate state.
    #[must_use]
    pub const fn host_approval(self) -> GateState {
        self.host_approval
    }

    /// Returns the credential state.
    #[must_use]
    pub const fn credentials(self) -> CredentialState {
        self.credentials
    }

    /// Returns the output validation state.
    #[must_use]
    pub const fn output(self) -> OutputState {
        self.output
    }

    /// Returns the termination state.
    #[must_use]
    pub const fn termination(self) -> TerminationState {
        self.termination
    }
}
