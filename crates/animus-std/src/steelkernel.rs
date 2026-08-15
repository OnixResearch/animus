//! Private durable Steel kernel snapshot host over the existing session-store mechanism.

use animus_runtime::durable_session_store::{
    DurableSessionPrivateEntryDraft, DurableSessionRecordKind, DurableSessionStoreDescriptor,
    DurableSessionStorePolicy, DurableSessionTransactionDraft, plan_durable_session_transaction,
};

const STEEL_KERNEL_ENTRY_PREFIX: &str = "steel.kernel.";
const FIRST_DURABLE_SEQUENCE: u64 = 1;
const NEXT_DURABLE_SEQUENCE: u64 = 1;

/// Typed durable Steel kernel host issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DurableSteelKernelHostIssue {
    /// No committed Steel kernel snapshot was present.
    SnapshotMissing,
    /// The private snapshot session differed from the store descriptor.
    SessionMismatch,
    /// The durable store rejected planning or commit.
    StoreRejected,
    /// Kernel journal admission or replay failed.
    RestoreRejected,
    /// Durable sequence arithmetic overflowed.
    BoundExceeded,
}

/// Bounded durable Steel kernel host error.
#[derive(Debug)]
pub struct DurableSteelKernelHostError {
    issue: DurableSteelKernelHostIssue,
}

impl DurableSteelKernelHostError {
    const fn new(issue: DurableSteelKernelHostIssue) -> Self {
        Self { issue }
    }

    /// Returns the safe issue class.
    #[must_use]
    pub const fn issue(&self) -> DurableSteelKernelHostIssue {
        self.issue
    }
}

impl core::fmt::Display for DurableSteelKernelHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "durable Steel kernel host failed: {:?}",
            self.issue
        )
    }
}

impl std::error::Error for DurableSteelKernelHostError {}

/// Explicit current facts required for one restore attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DurableSteelKernelRestoreRequest<'a> {
    /// Exact BLAKE3 session identity.
    pub session_identity: &'a str,
    /// Exact host-owned kernel generation.
    pub kernel_generation: u64,
    /// Exact committed private kernel chain head.
    pub committed_head_identity: &'a str,
    /// Durable active-turn recovery posture.
    pub recovery_posture: animus_steel::SteelKernelRecoveryPosture,
    /// Optional active-turn recovery action for an interrupted cell.
    pub active_turn_action: Option<animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction>,
}

/// Dedicated private kernel journal backed by the existing durable-store host.
pub struct DurableSteelKernelPersistence<'store, Synchronizer>
where
    Synchronizer: super::durablefile::DurableSessionSynchronizer,
{
    store: super::durablefile::DurableSessionLocalStore<'store, Synchronizer>,
    policy: &'store DurableSessionStorePolicy,
    descriptor: &'store DurableSessionStoreDescriptor,
}

impl<'store, Synchronizer> DurableSteelKernelPersistence<'store, Synchronizer>
where
    Synchronizer: super::durablefile::DurableSessionSynchronizer,
{
    /// Binds one dedicated durable store to private Steel kernel snapshots.
    #[must_use]
    pub const fn new(
        store: super::durablefile::DurableSessionLocalStore<'store, Synchronizer>,
        policy: &'store DurableSessionStorePolicy,
        descriptor: &'store DurableSessionStoreDescriptor,
    ) -> Self {
        Self {
            store,
            policy,
            descriptor,
        }
    }

    /// Returns the current outer durable-store committed head.
    #[must_use]
    pub fn durable_head_identity(&self) -> &str {
        self.store.recovery().committed_head_hash()
    }

    /// Returns the latest committed private kernel chain head.
    ///
    /// # Errors
    ///
    /// Returns [`DurableSteelKernelHostError`] when no valid snapshot entry exists.
    pub fn latest_kernel_head_identity(&self) -> Result<&str, DurableSteelKernelHostError> {
        self.store
            .recovery()
            .committed_frames()
            .iter()
            .rev()
            .find(|frame| frame.kind() == DurableSessionRecordKind::SteelKernelState)
            .and_then(|frame| frame.entry_id().strip_prefix(STEEL_KERNEL_ENTRY_PREFIX))
            .filter(|identity| !identity.is_empty())
            .ok_or_else(|| {
                DurableSteelKernelHostError::new(DurableSteelKernelHostIssue::SnapshotMissing)
            })
    }

    /// Commits one complete private kernel safe-point snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`DurableSteelKernelHostError`] before success for session drift,
    /// sequence overflow, transaction rejection, append failure, or sync failure.
    // r[impl agent_harness_runtime.steel_kernel_restore.journal]
    // r[impl agent_harness_runtime.steel_kernel_restore.safe_points]
    pub fn commit_snapshot(
        &mut self,
        snapshot: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<super::durablefile::DurableSessionCommitOutcome, DurableSteelKernelHostError> {
        if snapshot.session_identity() != self.descriptor.session_id {
            return Err(DurableSteelKernelHostError::new(
                DurableSteelKernelHostIssue::SessionMismatch,
            ));
        }
        let recovery = self.store.recovery();
        let first_sequence =
            recovery
                .committed_frames()
                .last()
                .map_or(Ok(FIRST_DURABLE_SEQUENCE), |frame| {
                    frame.sequence().checked_add(NEXT_DURABLE_SEQUENCE).ok_or(
                        DurableSteelKernelHostError::new(
                            DurableSteelKernelHostIssue::BoundExceeded,
                        ),
                    )
                })?;
        let entry_id = format!(
            "{STEEL_KERNEL_ENTRY_PREFIX}{}",
            snapshot.committed_head_identity()
        );
        let entry = DurableSessionPrivateEntryDraft {
            entry_id: entry_id.as_str(),
            parent_id: recovery.active_leaf_id(),
            kind: DurableSessionRecordKind::SteelKernelState,
            payload: snapshot.private_bytes(),
        };
        let transaction = plan_durable_session_transaction(
            self.policy,
            &DurableSessionTransactionDraft {
                descriptor: self.descriptor,
                prior_head_hash: recovery.committed_head_hash(),
                prior_active_leaf_id: recovery.active_leaf_id(),
                first_sequence,
                active_leaf_id: entry_id.as_str(),
                entries: &[entry],
            },
        )
        .map_err(|_error| {
            DurableSteelKernelHostError::new(DurableSteelKernelHostIssue::StoreRejected)
        })?;
        self.store.commit(&transaction).map_err(|_error| {
            DurableSteelKernelHostError::new(DurableSteelKernelHostIssue::StoreRejected)
        })
    }

    /// Restores the most recent committed private kernel snapshot.
    ///
    /// The host passes exact current session, generation, compatibility, and
    /// committed-head facts. Steel replay receives no effect executor.
    ///
    /// # Errors
    ///
    /// Returns [`DurableSteelKernelHostError`] when no committed snapshot exists,
    /// current admission differs, or effect-disabled replay fails.
    // r[impl agent_harness_runtime.steel_kernel_restore.replay]
    // r[impl agent_harness_runtime.steel_kernel_restore.admission]
    pub fn restore_latest(
        &self,
        policy: &animus_adapter_rlm_kernel::RlmKernelPolicy,
        request: DurableSteelKernelRestoreRequest<'_>,
    ) -> Result<
        (
            animus_steel::SteelRlmKernel,
            animus_steel::SteelKernelRestoreReceipt,
        ),
        DurableSteelKernelHostError,
    > {
        validate_recovery_request(request)?;
        let frame = self
            .store
            .recovery()
            .committed_frames()
            .iter()
            .rev()
            .find(|frame| frame.kind() == DurableSessionRecordKind::SteelKernelState)
            .ok_or_else(|| {
                DurableSteelKernelHostError::new(DurableSteelKernelHostIssue::SnapshotMissing)
            })?;
        animus_steel::SteelRlmKernel::restore_private_journal(
            policy,
            animus_steel::SteelKernelRestoreAdmission {
                session_identity: request.session_identity,
                kernel_generation: request.kernel_generation,
                committed_head_identity: request.committed_head_identity,
                recovery_posture: request.recovery_posture,
            },
            frame.private_payload(),
        )
        .map_err(|_error| {
            DurableSteelKernelHostError::new(DurableSteelKernelHostIssue::RestoreRejected)
        })
    }

    /// Explicitly releases the underlying durable writer lease.
    ///
    /// # Errors
    ///
    /// Returns [`DurableSteelKernelHostError`] when lease release is uncertain.
    pub fn close(self) -> Result<(), DurableSteelKernelHostError> {
        self.store.close().map_err(|_error| {
            DurableSteelKernelHostError::new(DurableSteelKernelHostIssue::StoreRejected)
        })
    }
}

fn validate_recovery_request(
    request: DurableSteelKernelRestoreRequest<'_>,
) -> Result<(), DurableSteelKernelHostError> {
    let valid = match request.recovery_posture {
        animus_steel::SteelKernelRecoveryPosture::Clean => request.active_turn_action.is_none(),
        animus_steel::SteelKernelRecoveryPosture::InterruptedCell => matches!(
            request.active_turn_action,
            Some(
                animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction::PlanInterruptedToolRepair
                    | animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction::Parked
                    | animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction::Blocked
                    | animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction::ObserveTerminal
                    | animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction::CleanupRequired
            )
        ),
    };
    if valid {
        Ok(())
    } else {
        Err(DurableSteelKernelHostError::new(
            DurableSteelKernelHostIssue::RestoreRejected,
        ))
    }
}

#[cfg(test)]
#[path = "steelkernel/tests.rs"]
mod tests;
