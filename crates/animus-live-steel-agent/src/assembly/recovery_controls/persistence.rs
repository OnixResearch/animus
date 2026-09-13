//! In-memory snapshot storage with explicit write refusals.

const CURSOR_INCREMENT: u64 = 1;

#[derive(Default)]
pub(super) struct Snapshots {
    pub(super) committed:
        Vec<animus_runtime::model_request_snapshot::CommittedModelRequestSnapshot>,
    pub(super) terminal:
        Vec<animus_runtime::model_request_snapshot::ModelRequestProviderTerminalObservation>,
    pub(super) refuse_snapshot: bool,
    pub(super) refuse_terminal: bool,
}

impl crate::SnapshotPersistence for Snapshots {
    fn close(self) -> Result<(), crate::LiveSteelAgentError> {
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
        if self.refuse_snapshot {
            return Err(
                animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed,
            );
        }
        let cursor = u64::try_from(self.committed.len())
            .ok()
            .and_then(|value| value.checked_add(CURSOR_INCREMENT))
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
        if self.refuse_terminal {
            return Err(
                animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed,
            );
        }
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
