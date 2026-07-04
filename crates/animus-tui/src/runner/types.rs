/// Host-owned sink for UI-neutral tool-host events.
pub trait ToolHostEventSink {
    /// Error returned by the concrete sink.
    type Error;

    /// Emits one tool-host event.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the host cannot consume the event.
    fn emit(&mut self, event: crate::ToolHostEvent) -> Result<(), Self::Error>;
}

/// Host-owned source for correlated approval commands.
pub trait ToolHostCommandSource {
    /// Error returned by the concrete command source.
    type Error;

    /// Returns one host command for an approval request.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the command source is unavailable or closed.
    fn decide(
        &mut self,
        request: &crate::ApprovalRequest,
    ) -> Result<crate::ToolHostCommand, Self::Error>;
}

/// Result of a hosted tool execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HostedToolOutcome {
    /// The wrapped tool adapter returned a validated response.
    Completed {
        /// Final tool response.
        response: animus_adapters::ToolResponse,
    },
    /// The host denied a guarded action before adapter execution.
    Denied {
        /// Typed denial failure.
        failure: crate::ToolHostFailure,
    },
    /// The host cancelled a guarded action before adapter execution.
    Cancelled {
        /// Typed cancellation failure.
        failure: crate::ToolHostFailure,
    },
    /// The host or adapter failed closed.
    Failed {
        /// Typed host failure.
        failure: crate::ToolHostFailure,
    },
}

impl HostedToolOutcome {
    /// Returns `true` when the tool adapter executed and completed successfully.
    #[must_use]
    pub const fn is_completed(&self) -> bool {
        matches!(self, Self::Completed { .. })
    }

    /// Returns the failure when the outcome did not complete.
    #[must_use]
    pub const fn failure(&self) -> Option<&crate::ToolHostFailure> {
        match self {
            Self::Completed { .. } => None,
            Self::Denied { failure } | Self::Cancelled { failure } | Self::Failed { failure } => {
                Some(failure)
            }
        }
    }
}

/// Borrowed inputs for one guarded hosted tool execution.
pub struct GuardedToolRun<'a, Tool, Sink, Commands> {
    /// Approval request that must be answered before the adapter can execute.
    pub approval: &'a crate::ApprovalRequest,
    /// Tool adapter that may execute only after an approving command.
    pub tool: &'a mut Tool,
    /// Validated tool request passed to the adapter after approval.
    pub request: &'a animus_adapters::ToolRequest,
    /// Host event sink receiving approval, command, lifecycle, and failure events.
    pub sink: &'a mut Sink,
    /// Host command source returning the correlated approval command.
    pub commands: &'a mut Commands,
}
