//! Explicit durable session and Steel-kernel store construction.

use animus_runtime::durable_session_store::{
    DurableSessionDurability, DurableSessionEncryptionPosture, DurableSessionLockPolicy,
    DurableSessionPrivacyPosture, DurableSessionStoreDescriptor, DurableSessionStorePolicy,
};
use animus_std::coding_session_host::DurableCodingSessionPersistence;
use animus_std::{
    DurableSessionStoreLocation, DurableSessionStoreOpenRequest, FileSynchronizer,
    open_durable_session_store, parse_durable_session_store_policy_json,
};

const STORE_POLICY_JSON: &str =
    include_str!("../../../config/generated/durable-session-store-policy.json");
const SESSION_STORE_ID: &str = "store.live-steel-agent.session";
const KERNEL_STORE_ID: &str = "store.live-steel-agent.kernel";
const SESSION_LEASE_ID: &str = "lease.live-steel-agent.session";
const KERNEL_LEASE_ID: &str = "lease.live-steel-agent.kernel";
const RECEIPT_POLICY: &str = "metadata-only";

/// Safe durable-composition failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveStoreError {
    /// The generated store policy failed deterministic parsing.
    Policy,
    /// The session store could not open safely.
    Session,
    /// The private kernel store could not open safely.
    Kernel,
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
    pub policy: DurableSessionStorePolicy,
    /// Main coding-session descriptor.
    pub session: DurableSessionStoreDescriptor,
    /// Dedicated private Steel-kernel descriptor.
    pub kernel: DurableSessionStoreDescriptor,
}

/// Loads exact store definitions from generated policy and validated app config.
///
/// # Errors
///
/// Returns [`LiveStoreError::Policy`] when generated policy parsing fails.
pub fn store_definitions(
    config: &crate::ValidatedLiveSteelAgentConfig,
) -> Result<LiveStoreDefinitions, LiveStoreError> {
    let policy = parse_durable_session_store_policy_json(STORE_POLICY_JSON)
        .map_err(|_error| LiveStoreError::Policy)?;
    let policy_hash = policy.policy_hash().to_owned();
    let descriptor = |store_id: &str, session_id: &str| DurableSessionStoreDescriptor {
        store_id: String::from(store_id),
        location_capability: String::from(config.store_capability_label()),
        session_id: String::from(session_id),
        branch_id: String::from(config.branch_id()),
        policy_hash: policy_hash.clone(),
        durability: DurableSessionDurability::SyncAllEachCommit,
        locking: DurableSessionLockPolicy::ExclusiveCreateNew,
        private_data: DurableSessionPrivacyPosture::ReplayPrivate,
        encryption: DurableSessionEncryptionPosture::AbsentExplicit,
        encryption_service_available: false,
        receipt_policy: String::from(RECEIPT_POLICY),
    };
    let session = descriptor(SESSION_STORE_ID, config.application_id());
    let kernel = descriptor(KERNEL_STORE_ID, config.session_identity());
    Ok(LiveStoreDefinitions {
        policy,
        session,
        kernel,
    })
}

/// Opens the main coding-session persistence lease.
///
/// # Errors
///
/// Returns [`LiveStoreError::Session`] when admission, recovery, locking, or I/O fails.
pub fn open_session_persistence<'a>(
    root: &'a cap_std::fs::Dir,
    config: &'a crate::ValidatedLiveSteelAgentConfig,
    definitions: &'a LiveStoreDefinitions,
) -> Result<DurableCodingSessionPersistence<'a, FileSynchronizer>, LiveStoreError> {
    let store = open_durable_session_store(
        &DurableSessionStoreOpenRequest {
            policy: &definitions.policy,
            descriptor: &definitions.session,
            location: DurableSessionStoreLocation {
                root,
                journal_name: config.session_journal(),
                lock_name: config.session_lock(),
            },
            lease_id: SESSION_LEASE_ID,
        },
        FileSynchronizer,
    )
    .map_err(|_error| LiveStoreError::Session)?;
    Ok(DurableCodingSessionPersistence::new(
        store,
        &definitions.policy,
        &definitions.session,
    ))
}

/// Opens the dedicated private Steel-kernel persistence lease.
///
/// # Errors
///
/// Returns [`LiveStoreError::Kernel`] when admission, recovery, locking, or I/O fails.
pub fn open_kernel_persistence<'a>(
    root: &'a cap_std::fs::Dir,
    config: &'a crate::ValidatedLiveSteelAgentConfig,
    definitions: &'a LiveStoreDefinitions,
) -> Result<animus_std::DurableSteelKernelPersistence<'a, FileSynchronizer>, LiveStoreError> {
    let store = open_durable_session_store(
        &DurableSessionStoreOpenRequest {
            policy: &definitions.policy,
            descriptor: &definitions.kernel,
            location: DurableSessionStoreLocation {
                root,
                journal_name: config.kernel_journal(),
                lock_name: config.kernel_lock(),
            },
            lease_id: KERNEL_LEASE_ID,
        },
        FileSynchronizer,
    )
    .map_err(|_error| LiveStoreError::Kernel)?;
    Ok(animus_std::DurableSteelKernelPersistence::new(
        store,
        &definitions.policy,
        &definitions.kernel,
    ))
}

/// Returns whether the coding-session journal contains committed state.
///
/// # Errors
///
/// Returns [`LiveStoreError::Session`] when the snapshot cannot be read safely.
pub fn session_resume_required(
    root: &cap_std::fs::Dir,
    config: &crate::ValidatedLiveSteelAgentConfig,
    definitions: &LiveStoreDefinitions,
) -> Result<bool, LiveStoreError> {
    match animus_std::read_durable_session_snapshot(
        root,
        config.session_journal(),
        &definitions.policy,
        &definitions.session,
    ) {
        Ok(recovery) => Ok(!recovery.committed_frames().is_empty()),
        Err(error) if error.issue() == animus_std::DurableSessionHostIssue::IoFailure => Ok(false),
        Err(_error) => Err(LiveStoreError::Session),
    }
}

/// Restores a present committed kernel snapshot without any effect executor.
///
/// An empty journal creates a fresh kernel and loads the reviewed skills. A
/// present journal must restore exactly; it never falls back to fresh state.
///
/// # Errors
///
/// Returns [`LiveStoreError::KernelRestore`] for present-snapshot drift or replay failure.
pub fn restore_or_create_kernel(
    config: &crate::ValidatedLiveSteelAgentConfig,
    persistence: &animus_std::DurableSteelKernelPersistence<'_, FileSynchronizer>,
) -> Result<animus_steel::SteelRlmKernel, LiveStoreError> {
    use animus_runtime::durable_session_store::DURABLE_SESSION_EMPTY_HEAD_HASH;

    if persistence.durable_head_identity() == DURABLE_SESSION_EMPTY_HEAD_HASH {
        let mut kernel = animus_steel::SteelRlmKernel::new(config.rlm_policy().clone())
            .map_err(|_error| LiveStoreError::KernelRestore)?;
        let plan = animus_adapter_rlm_kernel::resolve_steel_skill_modules(
            config.skill_modules(),
            animus_adapter_rlm_kernel::SteelSkillModuleAuthority {
                session: &[],
                profile: &[],
                skill: &[],
                host: &[],
            },
        )
        .map_err(|_error| LiveStoreError::KernelRestore)?;
        kernel
            .load_skill_modules(&plan)
            .map_err(|_error| LiveStoreError::KernelRestore)?;
        return Ok(kernel);
    }
    restore_kernel(
        config,
        persistence,
        animus_steel::SteelKernelRecoveryPosture::Clean,
        None,
    )
}

/// Restores the prior committed kernel while preserving an interrupted-cell posture.
///
/// # Errors
///
/// Returns [`LiveStoreError::KernelRestore`] for absent snapshots, invalid repair actions,
/// policy drift, journal drift, or effect-disabled replay failure.
pub fn restore_interrupted_kernel(
    config: &crate::ValidatedLiveSteelAgentConfig,
    persistence: &animus_std::DurableSteelKernelPersistence<'_, FileSynchronizer>,
    action: animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction,
) -> Result<animus_steel::SteelRlmKernel, LiveStoreError> {
    restore_kernel(
        config,
        persistence,
        animus_steel::SteelKernelRecoveryPosture::InterruptedCell,
        Some(action),
    )
}

fn restore_kernel(
    config: &crate::ValidatedLiveSteelAgentConfig,
    persistence: &animus_std::DurableSteelKernelPersistence<'_, FileSynchronizer>,
    recovery_posture: animus_steel::SteelKernelRecoveryPosture,
    active_turn_action: Option<animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction>,
) -> Result<animus_steel::SteelRlmKernel, LiveStoreError> {
    let kernel_head = persistence
        .latest_kernel_head_identity()
        .map_err(|_error| LiveStoreError::KernelRestore)?;
    persistence
        .restore_latest(
            config.rlm_policy(),
            animus_std::DurableSteelKernelRestoreRequest {
                session_identity: config.session_identity(),
                kernel_generation: config.authority_generation(),
                committed_head_identity: kernel_head,
                recovery_posture,
                active_turn_action,
            },
        )
        .map(|(kernel, _receipt)| kernel)
        .map_err(|_error| LiveStoreError::KernelRestore)
}
