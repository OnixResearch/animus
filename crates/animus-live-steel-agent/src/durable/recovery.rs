/// Returns whether the coding-session journal contains committed state.
///
/// # Errors
///
/// Returns [`crate::LiveStoreError::Session`] when the snapshot cannot be read safely.
pub fn session_resume_required(
    root: &cap_std::fs::Dir,
    config: &crate::ValidatedLiveSteelAgentConfig,
    definitions: &super::LiveStoreDefinitions,
) -> Result<bool, super::LiveStoreError> {
    match animus_std::read_durable_session_snapshot(
        root,
        config.session_journal(),
        &definitions.policy,
        &definitions.session,
    ) {
        Ok(recovery) => Ok(!recovery.committed_frames().is_empty()),
        Err(error) if error.issue() == animus_std::DurableSessionHostIssue::IoFailure => Ok(false),
        Err(_error) => Err(super::LiveStoreError::Session),
    }
}

/// Restores a present committed kernel snapshot without any effect executor.
///
/// An empty journal creates a fresh kernel and loads the reviewed skills. A
/// present journal must restore exactly; it never falls back to fresh state.
///
/// # Errors
///
/// Returns [`crate::LiveStoreError::KernelRestore`] for present-snapshot drift or replay failure.
pub fn restore_or_create_kernel(
    config: &crate::ValidatedLiveSteelAgentConfig,
    persistence: &animus_std::DurableSteelKernelPersistence<'_, animus_std::FileSynchronizer>,
) -> Result<animus_steel::SteelRlmKernel, super::LiveStoreError> {
    if persistence.durable_head_identity()
        == animus_runtime::durable_session_store::DURABLE_SESSION_EMPTY_HEAD_HASH
    {
        let mut kernel = animus_steel::SteelRlmKernel::new(config.rlm_policy().clone())
            .map_err(|_error| super::LiveStoreError::KernelRestore)?;
        let plan = animus_adapter_rlm_kernel::resolve_steel_skill_modules(
            config.skill_modules(),
            animus_adapter_rlm_kernel::SteelSkillModuleAuthority {
                session: &[],
                profile: &[],
                skill: &[],
                host: &[],
            },
        )
        .map_err(|_error| super::LiveStoreError::KernelRestore)?;
        kernel
            .load_skill_modules(&plan)
            .map_err(|_error| super::LiveStoreError::KernelRestore)?;
        debug_assert_eq!(
            kernel.skill_module_set_identity(),
            Some(plan.module_set_identity())
        );
        debug_assert!(!plan.module_set_identity().is_empty());
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
/// Returns [`crate::LiveStoreError::KernelRestore`] for absent snapshots, invalid repair actions,
/// policy drift, journal drift, or effect-disabled replay failure.
pub fn restore_interrupted_kernel(
    config: &crate::ValidatedLiveSteelAgentConfig,
    persistence: &animus_std::DurableSteelKernelPersistence<'_, animus_std::FileSynchronizer>,
    action: animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction,
) -> Result<animus_steel::SteelRlmKernel, super::LiveStoreError> {
    restore_kernel(
        config,
        persistence,
        animus_steel::SteelKernelRecoveryPosture::InterruptedCell,
        Some(action),
    )
}

pub(super) fn restore_kernel(
    config: &crate::ValidatedLiveSteelAgentConfig,
    persistence: &animus_std::DurableSteelKernelPersistence<'_, animus_std::FileSynchronizer>,
    recovery_posture: animus_steel::SteelKernelRecoveryPosture,
    active_turn_action: Option<animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction>,
) -> Result<animus_steel::SteelRlmKernel, super::LiveStoreError> {
    let kernel_head = persistence
        .latest_kernel_head_identity()
        .map_err(|_error| super::LiveStoreError::KernelRestore)?;
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
        .map_err(|_error| super::LiveStoreError::KernelRestore)
}
