use crate::ServerPosture;

/// Exact identities that fence one server generation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleBinding {
    /// Exact configured server identity.
    pub server_id: String,
    /// Exact validated configuration identity.
    pub configuration_id: String,
    /// Exact protocol cohort identity.
    pub protocol_id: String,
    /// Exact policy identity.
    pub policy_id: String,
    /// Safe operation-scoped credential snapshot identity.
    pub credential_snapshot_id: String,
    /// Monotonic server generation.
    pub generation: u64,
    /// Exact process-attempt identity.
    pub process_attempt_id: String,
}

/// Explicit lifecycle phase for one server generation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecyclePhase {
    /// Configuration is admitted without process effects.
    Configured,
    /// Launch is planned but not observed.
    Starting,
    /// The exact child process was observed.
    Initializing,
    /// Initialization succeeded and bounded discovery is active.
    Discovering,
    /// The current catalog is ready for authority-gated calls.
    Ready,
    /// One bounded call is active.
    Calling,
    /// Cancellation is active and new calls are blocked.
    Cancelling,
    /// Owned teardown is active.
    Stopping,
    /// Terminal owned-process evidence was observed.
    Stopped,
    /// The generation failed.
    Failed,
}

/// Bounded lifecycle failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FailureClass {
    /// Process launch failed.
    Launch,
    /// Initialization failed.
    Initialization,
    /// Discovery failed.
    Discovery,
    /// Call failed.
    Call,
    /// Credential identity drifted.
    CredentialDrift,
    /// Cancellation failed or timed out.
    Cancellation,
    /// Teardown did not produce terminal evidence.
    Teardown,
    /// A transition or identity was stale.
    StaleObservation,
}

/// Owned process-tree teardown classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TeardownState {
    /// No process was started.
    NotStarted,
    /// The owned process tree is terminal.
    Terminal,
    /// Terminal process-tree evidence is absent.
    Uncertain,
}

/// Pure lifecycle event with no ambient observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LifecycleEvent {
    /// Launch effects may begin.
    BeginStart(LifecycleBinding),
    /// The exact configured child was started.
    ProcessStarted(LifecycleBinding),
    /// Initialization accepted the exact protocol cohort.
    InitializationAccepted(LifecycleBinding),
    /// Discovery accepted a complete catalog identity.
    DiscoveryAccepted {
        /// Exact generation binding.
        binding: LifecycleBinding,
        /// Complete catalog BLAKE3 identity.
        catalog_identity: String,
        /// Complete discovered tool count.
        tool_count: usize,
    },
    /// One exact authority-gated call began.
    CallStarted {
        /// Exact generation binding.
        binding: LifecycleBinding,
        /// Exact call identity.
        call_identity: String,
    },
    /// The active call reached an admitted terminal outcome.
    CallCompleted {
        /// Exact generation binding.
        binding: LifecycleBinding,
        /// Exact active call identity.
        call_identity: String,
        /// BLAKE3 identity of bounded output metadata.
        output_identity: String,
    },
    /// Credential snapshot drift blocks new calls.
    CredentialDrifted {
        /// Exact old-generation binding.
        binding: LifecycleBinding,
        /// Newly observed safe credential identity.
        observed_credential_identity: String,
    },
    /// Cancellation was requested for the active call.
    CancellationRequested {
        /// Exact generation binding.
        binding: LifecycleBinding,
        /// Exact active call identity.
        call_identity: String,
    },
    /// Owned teardown began.
    StopRequested(LifecycleBinding),
    /// Owned teardown completed or remained uncertain.
    StopObserved {
        /// Exact generation binding.
        binding: LifecycleBinding,
        /// Terminal teardown classification.
        teardown: TeardownState,
    },
    /// A bounded failure occurred.
    Failed {
        /// Exact generation binding.
        binding: LifecycleBinding,
        /// Failure class.
        class: FailureClass,
        /// Current process teardown classification.
        teardown: TeardownState,
    },
}

/// Pure lifecycle state for one exact server generation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LifecycleState {
    pub(crate) binding: LifecycleBinding,
    pub(crate) phase: LifecyclePhase,
    pub(crate) catalog_identity: Option<String>,
    pub(crate) active_call_identity: Option<String>,
    pub(crate) completed_call_count: u32,
    pub(crate) failure: Option<FailureClass>,
    pub(crate) restart_blocked: bool,
}

impl LifecycleState {
    /// Returns the exact generation binding.
    #[must_use]
    pub const fn binding(&self) -> &LifecycleBinding {
        &self.binding
    }

    /// Returns the current lifecycle phase.
    #[must_use]
    pub const fn phase(&self) -> LifecyclePhase {
        self.phase
    }

    /// Returns the current complete catalog identity, when ready.
    #[must_use]
    pub fn catalog_identity(&self) -> Option<&str> {
        self.catalog_identity.as_deref()
    }

    /// Returns the completed call count.
    #[must_use]
    pub const fn completed_call_count(&self) -> u32 {
        self.completed_call_count
    }

    /// Returns whether same-server restart is blocked.
    #[must_use]
    pub const fn restart_blocked(&self) -> bool {
        self.restart_blocked
    }
}

/// Invalid pure lifecycle transition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecycleError {
    /// An event did not bind the exact current generation.
    BindingDrift,
    /// The event is illegal in the current phase.
    InvalidTransition,
    /// A required identity was malformed.
    InvalidIdentity,
    /// The configured call budget is exhausted.
    CallBudgetExhausted,
}

/// One server entry used by host-readiness reduction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServerReadinessEntry<'a> {
    /// Required or optional posture.
    pub posture: ServerPosture,
    /// Current pure state.
    pub state: &'a LifecycleState,
}

/// Composite readiness over explicitly configured servers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostReadiness {
    /// Whether all required servers are ready.
    pub required_ready: bool,
    /// Visible required failure count.
    pub required_failure_count: usize,
    /// Visible unavailable optional-server count.
    pub optional_unavailable_count: usize,
}
