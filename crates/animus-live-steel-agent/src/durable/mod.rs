pub(crate) mod recovery;
// Explicit durable session and Steel-kernel store construction.

pub(super) const STORE_POLICY_JSON: &str =
    include_str!("../../../../config/generated/durable-session-store-policy.json");
pub(super) const SESSION_STORE_ID: &str = "store.live-steel-agent.session";
pub(super) const KERNEL_STORE_ID: &str = "store.live-steel-agent.kernel";
pub(super) const SNAPSHOT_STORE_ID: &str = "store.live-steel-agent.model-request-snapshots";
pub(super) const SESSION_LEASE_ID: &str = "lease.live-steel-agent.session";
pub(super) const KERNEL_LEASE_ID: &str = "lease.live-steel-agent.kernel";
pub(super) const SNAPSHOT_LEASE_ID: &str = "lease.live-steel-agent.model-request-snapshots";
pub(super) const SNAPSHOT_JOURNAL_NAME: &str = "model-request-snapshots.journal";
pub(super) const SNAPSHOT_LOCK_NAME: &str = "model-request-snapshots.lock";
pub(super) const RECEIPT_POLICY: &str = "metadata-only";

/// Safe durable-composition failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveStoreError {
    /// The generated store policy failed deterministic parsing.
    Policy,
    /// The session store could not open safely.
    Session,
    /// The private kernel store could not open safely.
    Kernel,
    /// The private model-request snapshot store could not open safely.
    Snapshot,
    /// A present kernel snapshot failed strict effect-disabled restore.
    KernelRestore,
}

impl core::fmt::Display for LiveStoreError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "live durable store rejected: {self:?}")
    }
}

impl std::error::Error for LiveStoreError {}

/// Owned policy and descriptors that must outlive open durable services.
pub struct LiveStoreDefinitions {
    /// Generated and parsed durable-store policy.
    pub policy: animus_runtime::durable_session_store::DurableSessionStorePolicy,
    /// Main coding-session descriptor.
    pub session: animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
    /// Dedicated private Steel-kernel descriptor.
    pub kernel: animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
    /// Dedicated private model-request snapshot descriptor.
    pub snapshot: animus_runtime::durable_session_store::DurableSessionStoreDescriptor,
}

/// Loads exact store definitions from generated policy and validated app config.
///
/// # Errors
///
/// Returns [`crate::LiveStoreError::Policy`] when generated policy parsing fails.
pub fn store_definitions(
    config: &crate::ValidatedLiveSteelAgentConfig,
) -> Result<LiveStoreDefinitions, LiveStoreError> {
    let policy = animus_std::parse_durable_session_store_policy_json(STORE_POLICY_JSON)
        .map_err(|_error| LiveStoreError::Policy)?;
    let policy_hash = policy.policy_hash().to_owned();
    let descriptor = |store_id: &str, session_id: &str| {
        animus_runtime::durable_session_store::DurableSessionStoreDescriptor {
        store_id: String::from(store_id),
        location_capability: String::from(config.store_capability_label()),
        session_id: String::from(session_id),
        branch_id: String::from(config.branch_id()),
        policy_hash: policy_hash.clone(),
        durability: animus_runtime::durable_session_store::DurableSessionDurability::SyncAllEachCommit,
        locking: animus_runtime::durable_session_store::DurableSessionLockPolicy::ExclusiveCreateNew,
        private_data: animus_runtime::durable_session_store::DurableSessionPrivacyPosture::ReplayPrivate,
        encryption: animus_runtime::durable_session_store::DurableSessionEncryptionPosture::AbsentExplicit,
        encryption_service_available: false,
        receipt_policy: String::from(RECEIPT_POLICY),
    }
    };
    let session = descriptor(SESSION_STORE_ID, config.application_id());
    let kernel = descriptor(KERNEL_STORE_ID, config.session_identity());
    let snapshot = descriptor(SNAPSHOT_STORE_ID, config.application_id());
    debug_assert_ne!(session.store_id, kernel.store_id);
    debug_assert_eq!(snapshot.session_id, session.session_id);
    Ok(LiveStoreDefinitions {
        policy,
        session,
        kernel,
        snapshot,
    })
}

/// Opens the main coding-session persistence lease.
///
/// # Errors
///
/// Returns [`crate::LiveStoreError::Session`] when admission, recovery, locking, or I/O fails.
pub fn open_session_persistence<'a>(
    root: &'a cap_std::fs::Dir,
    config: &'a crate::ValidatedLiveSteelAgentConfig,
    definitions: &'a LiveStoreDefinitions,
) -> Result<
    animus_std::coding_session_host::DurableCodingSessionPersistence<
        'a,
        animus_std::FileSynchronizer,
    >,
    LiveStoreError,
> {
    let store = animus_std::open_durable_session_store(
        &animus_std::DurableSessionStoreOpenRequest {
            policy: &definitions.policy,
            descriptor: &definitions.session,
            location: animus_std::DurableSessionStoreLocation {
                root,
                journal_name: config.session_journal(),
                lock_name: config.session_lock(),
            },
            lease_id: SESSION_LEASE_ID,
        },
        animus_std::FileSynchronizer,
    )
    .map_err(|_error| LiveStoreError::Session)?;
    debug_assert!(!config.session_journal().is_empty());
    debug_assert_ne!(config.session_journal(), config.session_lock());
    Ok(
        animus_std::coding_session_host::DurableCodingSessionPersistence::new(
            store,
            &definitions.policy,
            &definitions.session,
        ),
    )
}

/// Opens the dedicated private model-request snapshot persistence lease.
///
/// # Errors
///
/// Returns [`crate::LiveStoreError::Snapshot`] when admission, recovery, locking, or I/O fails.
pub fn open_snapshot_persistence<'a>(
    root: &'a cap_std::fs::Dir,
    definitions: &'a LiveStoreDefinitions,
) -> Result<
    animus_std::model_request_snapshot_host::DurableModelRequestSnapshotStore<
        'a,
        animus_std::FileSynchronizer,
    >,
    LiveStoreError,
> {
    let store = animus_std::open_durable_session_store(
        &animus_std::DurableSessionStoreOpenRequest {
            policy: &definitions.policy,
            descriptor: &definitions.snapshot,
            location: animus_std::DurableSessionStoreLocation {
                root,
                journal_name: SNAPSHOT_JOURNAL_NAME,
                lock_name: SNAPSHOT_LOCK_NAME,
            },
            lease_id: SNAPSHOT_LEASE_ID,
        },
        animus_std::FileSynchronizer,
    )
    .map_err(|_error| LiveStoreError::Snapshot)?;
    debug_assert_eq!(
        definitions.snapshot.policy_hash,
        definitions.policy.policy_hash()
    );
    debug_assert!(!definitions.snapshot.session_id.is_empty());
    Ok(
        animus_std::model_request_snapshot_host::DurableModelRequestSnapshotStore::new(
            store,
            &definitions.policy,
            &definitions.snapshot,
        ),
    )
}

/// Opens the dedicated private Steel-kernel persistence lease.
///
/// # Errors
///
/// Returns [`crate::LiveStoreError::Kernel`] when admission, recovery, locking, or I/O fails.
pub fn open_kernel_persistence<'a>(
    root: &'a cap_std::fs::Dir,
    config: &'a crate::ValidatedLiveSteelAgentConfig,
    definitions: &'a LiveStoreDefinitions,
) -> Result<
    animus_std::DurableSteelKernelPersistence<'a, animus_std::FileSynchronizer>,
    LiveStoreError,
> {
    let store = animus_std::open_durable_session_store(
        &animus_std::DurableSessionStoreOpenRequest {
            policy: &definitions.policy,
            descriptor: &definitions.kernel,
            location: animus_std::DurableSessionStoreLocation {
                root,
                journal_name: config.kernel_journal(),
                lock_name: config.kernel_lock(),
            },
            lease_id: KERNEL_LEASE_ID,
        },
        animus_std::FileSynchronizer,
    )
    .map_err(|_error| LiveStoreError::Kernel)?;
    debug_assert!(!config.kernel_journal().is_empty());
    debug_assert_ne!(config.kernel_journal(), config.kernel_lock());
    Ok(animus_std::DurableSteelKernelPersistence::new(
        store,
        &definitions.policy,
        &definitions.kernel,
    ))
}
