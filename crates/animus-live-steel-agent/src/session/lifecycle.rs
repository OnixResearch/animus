//! Lifecycle observation and lease release for the persistent live session.

impl<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
    super::PersistentLiveHost<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
where
    Provider: crate::ProviderStreamService,
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: crate::SessionPersistence,
    SnapshotStore: crate::SnapshotPersistence,
    KernelPersistence: crate::KernelSafePointPersistence,
{
    /// Returns the safe current lifecycle status.
    #[must_use]
    pub fn status(&self) -> super::PersistentStatus {
        match self.host.state_snapshot().terminal() {
            Some(animus_runtime::coding_session_host::CodingSessionTerminalKind::Completed) => {
                super::PersistentStatus::Completed
            }
            Some(animus_runtime::coding_session_host::CodingSessionTerminalKind::Cancelled) => {
                super::PersistentStatus::Cancelled
            }
            Some(animus_runtime::coding_session_host::CodingSessionTerminalKind::Blocked) => {
                super::PersistentStatus::Blocked
            }
            None if self.host.state_snapshot().run().is_some()
                || self.host.state_snapshot().pending_effect().is_some() =>
            {
                super::PersistentStatus::Running
            }
            None => super::PersistentStatus::Idle,
        }
    }

    /// Closes a completed session and returns its private output and safe receipt.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] unless the run completed or any
    /// persistence lease and tool safe point cannot close cleanly.
    pub fn finish(self) -> Result<crate::LiveSteelAgentRunOutput, crate::LiveSteelAgentError> {
        if self.status() != super::PersistentStatus::Completed {
            return match self.close() {
                Ok(()) => Err(crate::LiveSteelAgentError::Session),
                Err(error) => Err(error),
            };
        }
        let final_text_result = self
            .host
            .message_snapshot()
            .iter()
            .rev()
            .find(|message| {
                message.kind()
                    == animus_runtime::coding_session_host::CodingSessionMessageKind::Assistant
            })
            .map(|message| String::from(message.body()))
            .ok_or(crate::LiveSteelAgentError::Session);
        let committed_message_count_result = u32::try_from(self.host.message_snapshot().len())
            .map_err(|_error| crate::LiveSteelAgentError::Receipt);
        let runtime_events = self.host.event_snapshot().to_vec();
        let event_count_result = u32::try_from(runtime_events.len())
            .map_err(|_error| crate::LiveSteelAgentError::Receipt);
        let services = self.host.into_services();
        let provider_dispatch_count = services.provider.dispatch_count;
        let tool_dispatch_count = services.tool.dispatch_count;
        let host_observation_count = services.tool.tool.host_observation_count();
        let kernel_head_result = services
            .tool
            .tool
            .kernel()
            .private_journal_snapshot(
                self.config.session_identity(),
                self.config.authority_generation(),
            )
            .map(|snapshot| String::from(snapshot.committed_head_identity()))
            .map_err(|_error| crate::LiveSteelAgentError::Receipt);
        let session_close = services.persistence.close();
        let snapshot_close = services.provider.snapshot_store.close();
        let tool_close = services
            .tool
            .tool
            .close()
            .map_err(|_error| crate::LiveSteelAgentError::Tool);
        session_close?;
        snapshot_close?;
        tool_close?;
        let final_text = final_text_result?;
        let committed_message_count = committed_message_count_result?;
        let event_count = event_count_result?;
        let kernel_head_identity = kernel_head_result?;
        let receipt = crate::LiveSteelAgentRunReceipt::completed(
            &crate::receipt::LiveSteelAgentReceiptDraft {
                config_identity: self.config.config_identity(),
                session_identity: self.config.session_identity(),
                committed_message_count,
                provider_dispatch_count,
                tool_dispatch_count,
                host_observation_count,
                event_count,
                kernel_head_identity: kernel_head_identity.as_str(),
            },
        );
        Ok(crate::LiveSteelAgentRunOutput {
            final_text,
            runtime_events,
            receipt,
        })
    }

    /// Releases all owned leases without producing a completion receipt.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when any session, snapshot, or
    /// kernel resource cannot close cleanly.
    pub fn close(self) -> Result<(), crate::LiveSteelAgentError> {
        let services = self.host.into_services();
        let session_close = services.persistence.close();
        let snapshot_close = services.provider.snapshot_store.close();
        let tool_close = services
            .tool
            .tool
            .close()
            .map_err(|_error| crate::LiveSteelAgentError::Tool);
        session_close?;
        snapshot_close?;
        tool_close
    }
}
