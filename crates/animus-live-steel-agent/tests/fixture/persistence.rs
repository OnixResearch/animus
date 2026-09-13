#[derive(Default)]
pub(super) struct Journal {
    messages: Vec<animus_runtime::coding_session_host::CodingSessionMessage>,
    head_hash: String,
}

impl animus_live_steel_agent::SessionPersistence for Journal {
    fn close(self) -> Result<(), animus_live_steel_agent::LiveSteelAgentError> {
        Ok(())
    }
}

impl animus_std::coding_session_host::CodingSessionPersistenceService for Journal {
    fn commit(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionCommitRequest,
    ) -> Result<
        animus_std::coding_session_host::CodingSessionCommitObservation,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        self.messages.extend(request.messages().iter().cloned());
        let cursor = u64::try_from(self.messages.len()).map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?;
        self.head_hash = format!("{cursor:064x}");
        Ok(
            animus_std::coding_session_host::CodingSessionCommitObservation {
                new_head_hash: self.head_hash.clone(),
                active_leaf_id: String::from(request.proposed_active_leaf_id()),
                durable_cursor: cursor,
                synchronized: true,
            },
        )
    }

    fn recover(
        &mut self,
        _request: &animus_runtime::coding_session_host::CodingSessionRecoveryRequest,
    ) -> Result<
        animus_runtime::coding_session_host::CodingSessionRecoverySnapshot,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        Ok(
            animus_runtime::coding_session_host::CodingSessionRecoverySnapshot {
                status: if self.messages.is_empty() {
                    animus_runtime::coding_session_host::CodingSessionRecoveryStatus::Empty
                } else {
                    animus_runtime::coding_session_host::CodingSessionRecoveryStatus::Clean
                },
                committed_head_hash: if self.head_hash.is_empty() {
                    String::from(
                        animus_runtime::coding_session_host::CODING_SESSION_EMPTY_HEAD_HASH,
                    )
                } else {
                    self.head_hash.clone()
                },
                active_leaf_id: self
                    .messages
                    .last()
                    .map(|message| String::from(message.entry_id())),
                durable_cursor: u64::try_from(self.messages.len()).map_err(|_error| {
                    animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
                })?,
                messages: self.messages.clone(),
            },
        )
    }
}

#[derive(Default)]
pub(super) struct Snapshots {
    committed: Vec<animus_runtime::model_request_snapshot::CommittedModelRequestSnapshot>,
    terminal: Vec<animus_runtime::model_request_snapshot::ModelRequestProviderTerminalObservation>,
}

impl animus_live_steel_agent::SnapshotPersistence for Snapshots {
    fn close(self) -> Result<(), animus_live_steel_agent::LiveSteelAgentError> {
        Ok(())
    }
}

impl animus_std::model_request_snapshot_host::ModelRequestSnapshotStorePort for Snapshots {
    fn commit_snapshot(
        &mut self,
        plan: &animus_runtime::model_request_snapshot::ModelRequestSnapshotCommitPlan,
    ) -> Result<
        animus_runtime::model_request_snapshot::ModelRequestSnapshotCommitObservation,
        animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue,
    > {
        let cursor = u64::try_from(self.committed.len())
            .ok()
            .and_then(|value| value.checked_add(1))
            .ok_or(
                animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed,
            )?;
        let observation =
            animus_runtime::model_request_snapshot::ModelRequestSnapshotCommitObservation {
                plan_identity: String::from(plan.plan_identity()),
                snapshot_identity: String::from(plan.snapshot().snapshot_identity()),
                session_id: String::from(plan.snapshot().session_id()),
                branch_id: String::from(plan.snapshot().branch_id()),
                active_leaf_id: String::from(plan.snapshot().active_leaf_id()),
                turn_generation: plan.snapshot().turn_generation(),
                request_sequence: plan.snapshot().request_sequence(),
                new_head_hash: String::from(plan.plan_identity()),
                durable_cursor: cursor,
                complete: true,
                synchronized: true,
            };
        let committed =
            animus_runtime::model_request_snapshot::observe_model_request_snapshot_commit(
                plan.clone(),
                &observation,
            )
            .map_err(|_error| {
                animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed
            })?;
        self.committed.push(committed);
        Ok(observation)
    }

    fn commit_terminal(
        &mut self,
        observation: &animus_runtime::model_request_snapshot::ModelRequestProviderTerminalObservation,
    ) -> Result<(), animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue> {
        self.terminal.push(observation.clone());
        Ok(())
    }

    fn load_committed_snapshot(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<
        Option<animus_runtime::model_request_snapshot::CommittedModelRequestSnapshot>,
        animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue,
    > {
        Ok(self
            .committed
            .iter()
            .rev()
            .find(|value| value.snapshot().snapshot_identity() == snapshot_identity)
            .cloned())
    }

    fn load_terminal_observation(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<
        Option<animus_runtime::model_request_snapshot::ModelRequestProviderTerminalObservation>,
        animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue,
    > {
        Ok(self
            .terminal
            .iter()
            .rev()
            .find(|value| value.snapshot_identity == snapshot_identity)
            .cloned())
    }
}
