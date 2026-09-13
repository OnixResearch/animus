mod recovery;

impl<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
    super::LiveSteelAgentComposition<
        Provider,
        Executor,
        Persistence,
        SnapshotStore,
        KernelPersistence,
    >
where
    Provider: crate::ProviderStreamService,
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: super::SessionPersistence,
    SnapshotStore: super::SnapshotPersistence,
    KernelPersistence: crate::evaluation::ports::KernelSafePointPersistence,
{
    /// Selects and binds the provider, Steel kernel, bridge, host executor,
    /// skills, control, events, receipt, and coding-session adapters.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] before a provider or host effect when
    /// skill, kernel, or composition admission fails.
    pub fn new(
        input: super::LiveSteelAgentCompositionInput<
            Provider,
            Executor,
            Persistence,
            SnapshotStore,
            KernelPersistence,
        >,
    ) -> Result<Self, super::LiveSteelAgentError> {
        // r[impl agent_harness_runtime.live_steel_agent_shell.composition]
        let capabilities = input
            .config
            .admitted_operations()
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>();
        let authority = animus_adapter_rlm_kernel::SteelSkillModuleAuthority {
            session: capabilities.as_slice(),
            profile: capabilities.as_slice(),
            skill: capabilities.as_slice(),
            host: capabilities.as_slice(),
        };
        let plan = animus_adapter_rlm_kernel::resolve_steel_skill_modules(
            input.config.skill_modules(),
            authority,
        )
        .map_err(|_error| super::LiveSteelAgentError::Skill)?;
        let mut kernel = input.kernel;
        match kernel.skill_module_set_identity() {
            Some(identity) if identity != plan.module_set_identity() => {
                return Err(super::LiveSteelAgentError::Skill);
            }
            Some(_identity) => {}
            None => {
                kernel
                    .load_skill_modules(&plan)
                    .map_err(|_error| super::LiveSteelAgentError::Skill)?;
            }
        }
        let tool = crate::SteelToolService::new(
            kernel,
            input.host_executor,
            input.kernel_persistence,
            crate::SteelToolBinding {
                session_identity: String::from(input.config.session_identity()),
                generation: input.config.authority_generation(),
            },
        );
        Ok(Self {
            config: input.config,
            provider: input.provider,
            session_persistence: input.session_persistence,
            snapshot_persistence: input.snapshot_persistence,
            tool,
        })
    }

    /// Opens one persistent session that keeps provider, tool, and persistence
    /// authority inside the live shell while a worker interleaves commands.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when resume or host construction fails.
    pub fn open_session(
        self,
    ) -> Result<
        crate::PersistentLiveHost<
            Provider,
            Executor,
            Persistence,
            SnapshotStore,
            KernelPersistence,
        >,
        super::LiveSteelAgentError,
    > {
        // r[impl agent_harness_runtime.live_steel_agent_shell.persistent_session]
        let shared = std::rc::Rc::new(std::cell::RefCell::new(
            crate::adapters::responses::continuation::SharedProviderState::default(),
        ));
        let config = std::rc::Rc::new(self.config);
        let services = animus_std::coding_session_host::CodingSessionHostServices {
            provider: super::dispatch::ProviderAdapter {
                provider: self.provider,
                snapshot_store: self.snapshot_persistence,
                config: std::rc::Rc::clone(&config),
                shared: std::rc::Rc::clone(&shared),
                dispatch_count: 0,
            },
            tool: super::stages::ToolAdapter {
                tool: self.tool,
                config: std::rc::Rc::clone(&config),
                shared,
                dispatch_count: 0,
                pending_stage: None,
            },
            persistence: self.session_persistence,
            policy: animus_std::coding_session_host::AllowCodingSessionPolicy,
            authority: animus_std::coding_session_host::AdmittedCodingSessionAuthority {
                observation_count: 0,
            },
            cancellation: animus_std::coding_session_host::ObservedCodingSessionCancellation,
            timer: animus_std::coding_session_host::AvailableCodingSessionTimer,
            summarizer: animus_std::coding_session_host::FakeCodingSessionSummarizer {
                summary: String::from(super::SUMMARY),
            },
            events: super::policy::MetadataEventSink,
        };
        let state = super::policy::new_state(config.as_ref())?;
        let host = animus_std::coding_session_host::CodingSessionHost::new(state, services)
            .map_err(|_error| super::LiveSteelAgentError::Session)?;
        let needs_resume = config.session_resume_required();
        let mut session = crate::PersistentLiveHost::new(config, host);
        if !needs_resume {
            return Ok(session);
        }
        if session
            .host
            .resume_with_gate(&mut recovery::ReadOnly)
            .is_err()
        {
            session.close()?;
            return Err(super::LiveSteelAgentError::Session);
        }
        Ok(session)
    }

    /// Runs one prompt through `CodingSessionHost` until a terminal result.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] for provider, authority, persistence,
    /// tool, cancellation, bounds, ordering, or receipt failures.
    pub fn run_prompt(
        self,
        prompt: &str,
    ) -> Result<super::LiveSteelAgentRunOutput, super::LiveSteelAgentError> {
        // r[impl agent_harness_runtime.live_steel_agent_shell.tool_loop]
        // r[impl agent_harness_runtime.live_steel_agent_shell.session]
        let prompt_identity = blake3::hash(prompt.as_bytes()).to_hex().to_string();
        let mut session = self.open_session()?;
        session.begin_prompt(crate::LiveSteelPrompt {
            correlation_id: prompt_identity.as_str(),
            text: prompt,
        })?;
        session.drive_pending()?;
        session.finish()
    }
}
