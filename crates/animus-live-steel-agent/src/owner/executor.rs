impl<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
    super::PersistentLiveHost<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
where
    Provider: crate::ProviderStreamService,
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor + super::LiveSessionExecutorControl,
    Persistence: crate::SessionPersistence,
    SnapshotStore: crate::SnapshotPersistence,
    KernelPersistence: crate::KernelSafePointPersistence,
{
    /// Delegates one exact approval decision to the retained executor owner.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError::Tool`] for malformed or rejected controls.
    pub fn answer_approval(
        &mut self,
        correlation_id: &str,
        decision: super::LiveApprovalDecision,
    ) -> Result<(), crate::LiveSteelAgentError> {
        super::admission::require_blake3_identity(correlation_id)?;
        self.host
            .tool_service_mut()
            .host_executor_mut()
            .answer_approval(correlation_id, decision)
            .map_err(|_error| crate::LiveSteelAgentError::Tool)
    }

    /// Returns metadata-only child status through the retained executor owner.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError::Tool`] for malformed or unknown child identity.
    pub fn child_status(
        &mut self,
        child_id: &str,
    ) -> Result<super::LiveChildStatus, crate::LiveSteelAgentError> {
        super::admission::require_control_label(child_id)?;
        self.host
            .tool_service_mut()
            .host_executor_mut()
            .child_status(child_id)
            .map_err(|_error| crate::LiveSteelAgentError::Tool)
    }

    /// Requests child cancellation through the retained executor owner.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError::Tool`] for malformed or unknown child identity.
    pub fn cancel_child(
        &mut self,
        child_id: &str,
    ) -> Result<super::LiveChildStatus, crate::LiveSteelAgentError> {
        super::admission::require_control_label(child_id)?;
        self.host
            .tool_service_mut()
            .host_executor_mut()
            .cancel_child(child_id)
            .map_err(|_error| crate::LiveSteelAgentError::Tool)
    }
}
