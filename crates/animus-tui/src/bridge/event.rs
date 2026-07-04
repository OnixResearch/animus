/// UI-neutral tool-host event consumed by terminal, headless, or test hosts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ToolHostEvent {
    /// A hosted tool execution started.
    Started {
        /// Tool run identity.
        run_id: ToolRunId,
        /// Validated tool request crossing the adapter boundary.
        request: animus_adapters::ToolRequest,
    },
    /// Hosted tool progress was observed.
    Progress {
        /// Tool run identity.
        run_id: ToolRunId,
        /// Bounded progress record.
        progress: ToolProgress,
    },
    /// A bounded output chunk was observed.
    Output {
        /// Tool run identity.
        run_id: ToolRunId,
        /// Bounded model-visible output plus safe metadata.
        output: animus_adapters::tool::BoundedOutput,
    },
    /// The tool requested a finite host approval decision.
    ApprovalRequested {
        /// Approval request with finite choices and a correlation identity.
        request: ApprovalRequest,
    },
    /// The host returned a correlated approval command.
    CommandObserved {
        /// Tool run identity.
        run_id: ToolRunId,
        /// Correlated host command.
        command: ToolHostCommand,
    },
    /// The host cancelled an interactive wait.
    Cancelled {
        /// Tool run identity.
        run_id: ToolRunId,
        /// Safe cancellation reason.
        reason: String,
    },
    /// The tool adapter returned a final response.
    Finished {
        /// Tool run identity.
        run_id: ToolRunId,
        /// Validated tool response.
        response: animus_adapters::ToolResponse,
    },
    /// The host or adapter failed closed.
    Failed {
        /// Tool run identity.
        run_id: ToolRunId,
        /// Typed host failure.
        failure: ToolHostFailure,
    },
}
