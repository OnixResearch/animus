use super::{ModelRequestSnapshotStoreIssue, ModelRequestSnapshotStorePort, snapshot};

const SNAPSHOT_ENVELOPE_PREFIX: &[u8] = b"animus.model-request-snapshot.store-envelope.v1\n";
const BLAKE3_HEX_BYTES: usize = 64;
const TERMINAL_ENTRY_IDENTITY_CHARS: usize = BLAKE3_HEX_BYTES;
const TERMINAL_ENTRY_PREFIX: &str = "request-terminal.";
const FIRST_SEQUENCE: u64 = 1;
const SEQUENCE_INCREMENT: u64 = 1;

/// Private durable-store adapter for snapshots and terminal observations.
///
/// The caller supplies an already opened capability-local durable store. A
/// dedicated journal can keep snapshot lineage separate from transcript leaf
/// lineage while reusing the same private framing, synchronization, locking,
/// recovery, and bounds owner.
pub struct DurableModelRequestSnapshotStore<'store, Synchronizer>
where
    Synchronizer: super::super::durablefile::DurableSessionSynchronizer,
{
    store: super::super::durablefile::DurableSessionLocalStore<'store, Synchronizer>,
    policy: &'store animus_runtime::durable_session_store::DurableSessionStorePolicy,
    descriptor: &'store animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
}

impl<'store, Synchronizer> DurableModelRequestSnapshotStore<'store, Synchronizer>
where
    Synchronizer: super::super::durablefile::DurableSessionSynchronizer,
{
    /// Binds an explicit durable store, policy, and descriptor.
    #[must_use]
    pub const fn new(
        store: super::super::durablefile::DurableSessionLocalStore<'store, Synchronizer>,
        policy: &'store animus_runtime::durable_session_store::DurableSessionStorePolicy,
        descriptor: &'store animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
    ) -> Self {
        Self {
            store,
            policy,
            descriptor,
        }
    }

    /// Releases the explicit writer lease.
    ///
    /// # Errors
    ///
    /// Returns a durable-store error when lease cleanup is uncertain.
    pub fn close(self) -> Result<(), super::super::durablefile::DurableSessionHostError> {
        self.store.close()
    }

    fn commit_private_entry(
        &mut self,
        entry_id: &str,
        kind: animus_runtime::durable_session_store::DurableSessionRecordKind,
        payload: &[u8],
    ) -> Result<
        super::super::durablefile::DurableSessionCommitOutcome,
        ModelRequestSnapshotStoreIssue,
    > {
        let recovery = self.store.recovery();
        let prior_head_hash = String::from(recovery.committed_head_hash());
        let prior_active_leaf_id = recovery.active_leaf_id().map(String::from);
        let first_sequence =
            recovery
                .committed_frames()
                .last()
                .map_or(Ok(FIRST_SEQUENCE), |frame| {
                    frame
                        .sequence()
                        .checked_add(SEQUENCE_INCREMENT)
                        .ok_or(ModelRequestSnapshotStoreIssue::Failed)
                })?;
        let entry = [
            animus_runtime::durable_session_store::DurableSessionPrivateEntryDraft {
                entry_id,
                parent_id: prior_active_leaf_id.as_deref(),
                kind,
                payload,
            },
        ];
        let transaction = animus_runtime::durable_session_store::plan_durable_session_transaction(
            self.policy,
            &animus_runtime::durable_session_store::DurableSessionTransactionDraft {
                descriptor: self.descriptor,
                prior_head_hash: prior_head_hash.as_str(),
                prior_active_leaf_id: prior_active_leaf_id.as_deref(),
                first_sequence,
                active_leaf_id: entry_id,
                entries: &entry,
            },
        )
        .map_err(|error| map_core_store_issue(error.issue()))?;
        self.store
            .commit(&transaction)
            .map_err(|error| map_host_store_issue(error.issue()))
    }
}

impl<Synchronizer> ModelRequestSnapshotStorePort
    for DurableModelRequestSnapshotStore<'_, Synchronizer>
where
    Synchronizer: super::super::durablefile::DurableSessionSynchronizer,
{
    fn commit_snapshot(
        &mut self,
        plan: &snapshot::ModelRequestSnapshotCommitPlan,
    ) -> Result<snapshot::ModelRequestSnapshotCommitObservation, ModelRequestSnapshotStoreIssue>
    {
        let request = plan.snapshot();
        if request.session_id() != self.descriptor.session_id
            || request.branch_id() != self.descriptor.branch_id
        {
            return Err(ModelRequestSnapshotStoreIssue::Stale);
        }
        let payload = encode_snapshot_envelope(plan)?;
        let outcome = self.commit_private_entry(
            plan.entry_id(),
            animus_runtime::durable_session_store::DurableSessionRecordKind::ModelRequestSnapshot,
            payload.as_slice(),
        )?;
        let durable_cursor = u64::try_from(self.store.recovery().committed_frames().len())
            .map_err(|_overflow| ModelRequestSnapshotStoreIssue::Failed)?;
        Ok(snapshot::ModelRequestSnapshotCommitObservation {
            plan_identity: String::from(plan.plan_identity()),
            snapshot_identity: String::from(request.snapshot_identity()),
            session_id: String::from(request.session_id()),
            branch_id: String::from(request.branch_id()),
            active_leaf_id: String::from(request.active_leaf_id()),
            turn_generation: request.turn_generation(),
            request_sequence: request.request_sequence(),
            new_head_hash: outcome.new_head_hash,
            durable_cursor,
            complete: true,
            synchronized: outcome.synchronized,
        })
    }

    fn commit_terminal(
        &mut self,
        observation: &snapshot::ModelRequestProviderTerminalObservation,
    ) -> Result<(), ModelRequestSnapshotStoreIssue> {
        let payload = snapshot::model_request_provider_terminal_private_payload(observation)
            .map_err(|_error| ModelRequestSnapshotStoreIssue::Failed)?;
        let identity_prefix = observation
            .observation_identity
            .get(..TERMINAL_ENTRY_IDENTITY_CHARS)
            .ok_or(ModelRequestSnapshotStoreIssue::Failed)?;
        let entry_id = format!("{TERMINAL_ENTRY_PREFIX}{identity_prefix}");
        self.commit_private_entry(
            entry_id.as_str(),
            animus_runtime::durable_session_store::DurableSessionRecordKind::ModelRequestTerminal,
            payload.as_slice(),
        )?;
        Ok(())
    }

    fn load_committed_snapshot(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<Option<snapshot::CommittedModelRequestSnapshot>, ModelRequestSnapshotStoreIssue>
    {
        for frame in self.store.recovery().committed_frames().iter().rev() {
            if frame.kind()
                != animus_runtime::durable_session_store::DurableSessionRecordKind::ModelRequestSnapshot
            {
                continue;
            }
            let (snapshot, prior_head_hash) = decode_snapshot_envelope(frame.private_payload())?;
            if snapshot.snapshot_identity() != snapshot_identity {
                continue;
            }
            let plan =
                snapshot::plan_model_request_snapshot_commit(snapshot, prior_head_hash.as_str())
                    .map_err(|_error| ModelRequestSnapshotStoreIssue::Failed)?;
            if plan.entry_id() != frame.entry_id() {
                return Err(ModelRequestSnapshotStoreIssue::Stale);
            }
            let observation = snapshot::ModelRequestSnapshotCommitObservation {
                plan_identity: String::from(plan.plan_identity()),
                snapshot_identity: String::from(plan.snapshot().snapshot_identity()),
                session_id: String::from(plan.snapshot().session_id()),
                branch_id: String::from(plan.snapshot().branch_id()),
                active_leaf_id: String::from(plan.snapshot().active_leaf_id()),
                turn_generation: plan.snapshot().turn_generation(),
                request_sequence: plan.snapshot().request_sequence(),
                new_head_hash: String::from(frame.frame_hash()),
                durable_cursor: frame.sequence(),
                complete: true,
                synchronized: true,
            };
            return snapshot::observe_model_request_snapshot_commit(plan, &observation)
                .map(Some)
                .map_err(|_error| ModelRequestSnapshotStoreIssue::Failed);
        }
        Ok(None)
    }

    fn load_terminal_observation(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<
        Option<snapshot::ModelRequestProviderTerminalObservation>,
        ModelRequestSnapshotStoreIssue,
    > {
        for frame in self.store.recovery().committed_frames().iter().rev() {
            if frame.kind()
                != animus_runtime::durable_session_store::DurableSessionRecordKind::ModelRequestTerminal
            {
                continue;
            }
            let observation = snapshot::decode_model_request_provider_terminal_private_payload(
                frame.private_payload(),
            )
            .map_err(|_error| ModelRequestSnapshotStoreIssue::Failed)?;
            if observation.snapshot_identity == snapshot_identity {
                return Ok(Some(observation));
            }
        }
        Ok(None)
    }
}

fn encode_snapshot_envelope(
    plan: &snapshot::ModelRequestSnapshotCommitPlan,
) -> Result<Vec<u8>, ModelRequestSnapshotStoreIssue> {
    if plan.plan_identity().len() != BLAKE3_HEX_BYTES
        || plan.prior_head_hash().len() != BLAKE3_HEX_BYTES
    {
        return Err(ModelRequestSnapshotStoreIssue::Failed);
    }
    let capacity = SNAPSHOT_ENVELOPE_PREFIX
        .len()
        .checked_add(BLAKE3_HEX_BYTES)
        .and_then(|value| value.checked_add(BLAKE3_HEX_BYTES))
        .and_then(|value| value.checked_add(plan.snapshot().private_payload().len()))
        .ok_or(ModelRequestSnapshotStoreIssue::Failed)?;
    let mut payload = Vec::with_capacity(capacity);
    payload.extend_from_slice(SNAPSHOT_ENVELOPE_PREFIX);
    payload.extend_from_slice(plan.plan_identity().as_bytes());
    payload.extend_from_slice(plan.prior_head_hash().as_bytes());
    payload.extend_from_slice(plan.snapshot().private_payload());
    Ok(payload)
}

fn decode_snapshot_envelope(
    payload: &[u8],
) -> Result<(snapshot::ModelRequestSnapshot, String), ModelRequestSnapshotStoreIssue> {
    let header_bytes = SNAPSHOT_ENVELOPE_PREFIX
        .len()
        .checked_add(BLAKE3_HEX_BYTES)
        .and_then(|value| value.checked_add(BLAKE3_HEX_BYTES))
        .ok_or(ModelRequestSnapshotStoreIssue::Failed)?;
    if payload.len() <= header_bytes || !payload.starts_with(SNAPSHOT_ENVELOPE_PREFIX) {
        return Err(ModelRequestSnapshotStoreIssue::Failed);
    }
    let plan_start = SNAPSHOT_ENVELOPE_PREFIX.len();
    let prior_start = plan_start
        .checked_add(BLAKE3_HEX_BYTES)
        .ok_or(ModelRequestSnapshotStoreIssue::Failed)?;
    let snapshot_start = prior_start
        .checked_add(BLAKE3_HEX_BYTES)
        .ok_or(ModelRequestSnapshotStoreIssue::Failed)?;
    let encoded_plan_identity = core::str::from_utf8(
        payload
            .get(plan_start..prior_start)
            .ok_or(ModelRequestSnapshotStoreIssue::Failed)?,
    )
    .map_err(|_error| ModelRequestSnapshotStoreIssue::Failed)?;
    let prior_head_hash = core::str::from_utf8(
        payload
            .get(prior_start..snapshot_start)
            .ok_or(ModelRequestSnapshotStoreIssue::Failed)?,
    )
    .map_err(|_error| ModelRequestSnapshotStoreIssue::Failed)?;
    let decoded = snapshot::decode_model_request_snapshot_private_payload(
        payload
            .get(snapshot_start..)
            .ok_or(ModelRequestSnapshotStoreIssue::Failed)?,
    )
    .map_err(|_error| ModelRequestSnapshotStoreIssue::Failed)?;
    let plan = snapshot::plan_model_request_snapshot_commit(decoded.clone(), prior_head_hash)
        .map_err(|_error| ModelRequestSnapshotStoreIssue::Failed)?;
    if plan.plan_identity() != encoded_plan_identity {
        return Err(ModelRequestSnapshotStoreIssue::Stale);
    }
    Ok((decoded, String::from(prior_head_hash)))
}

const fn map_core_store_issue(
    issue: animus_runtime::durable_session_store::DurableSessionStoreIssue,
) -> ModelRequestSnapshotStoreIssue {
    use animus_runtime::durable_session_store::DurableSessionStoreIssue as Issue;
    match issue {
        Issue::StaleHead | Issue::SequenceGap | Issue::IntegrityMismatch => {
            ModelRequestSnapshotStoreIssue::Stale
        }
        Issue::BoundExceeded => ModelRequestSnapshotStoreIssue::Partial,
        _other => ModelRequestSnapshotStoreIssue::Failed,
    }
}

const fn map_host_store_issue(
    issue: super::super::durablefile::DurableSessionHostIssue,
) -> ModelRequestSnapshotStoreIssue {
    use super::super::durablefile::DurableSessionHostIssue as Issue;
    match issue {
        Issue::LockConflict | Issue::StaleLease => ModelRequestSnapshotStoreIssue::Unavailable,
        Issue::StaleHead => ModelRequestSnapshotStoreIssue::Stale,
        Issue::SynchronizationFailed | Issue::BoundExceeded => {
            ModelRequestSnapshotStoreIssue::Partial
        }
        _other => ModelRequestSnapshotStoreIssue::Failed,
    }
}
