//! Durable interrupted-write controls. These do not simulate a process kill.

#[cfg(test)]
#[path = "recovery/checkpoint.rs"]
mod checkpoint;

#[cfg(test)]
mod cases {
    const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
    const TOOL_STREAM: &[u8] =
        include_bytes!("../../animus-std-openai/tests/fixtures/openai-responses-tool-call.sse");
    const FINAL_STREAM: &[u8] =
        include_bytes!("../../animus-std-openai/tests/fixtures/openai-responses-final.sse");
    const PROMPT: &str = "Use Steel for the interrupted turn.";
    const CORRELATION: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const INTERRUPT_STEP_LIMIT: u32 = 8;
    const NO_PROVIDER_CALLS: u32 = 0;

    /// Closed protocol: interrupt at the kernel commit or finish the session commit.
    #[derive(Clone, Copy)]
    #[doc = "`sealed_enum`"]
    enum Boundary {
        KernelCommit,
        FinalCommit,
    }

    struct DenyHost;

    impl animus_runtime::rlm_kernel::RlmKernelHostExecutor for DenyHost {
        type Error = &'static str;

        fn execute(
            &mut self,
            _request: &animus_adapter_rlm_kernel::HostRequest,
        ) -> Result<String, Self::Error> {
            Err("unexpected host request")
        }
    }

    #[test]
    fn crash_window_reopen_refuses_unproved_tool_continuation() {
        let temp = tempfile::tempdir().expect("temporary store root");
        let root = cap_std::fs::Dir::open_ambient_dir(temp.path(), cap_std::ambient_authority())
            .expect("capability root");
        let base = animus_live_steel_agent::parse_live_steel_agent_config(CONFIG).expect("config");
        let definitions = animus_live_steel_agent::store_definitions(&base).expect("definitions");
        close_at_boundary(
            &root,
            &base.with_session_resume_required(false),
            &definitions,
            Boundary::KernelCommit,
        );
        let calls = std::rc::Rc::new(std::cell::Cell::new(NO_PROVIDER_CALLS));
        let result = resume_and_close(
            &root,
            &base.with_session_resume_required(true),
            &definitions,
            std::rc::Rc::clone(&calls),
        );
        assert_eq!(
            result,
            Err(animus_live_steel_agent::LiveSteelAgentError::Session)
        );
        assert_eq!(calls.get(), NO_PROVIDER_CALLS);
    }

    #[test]
    fn clean_committed_pair_reopens_normally_without_provider_contact() {
        let temp = tempfile::tempdir().expect("temporary store root");
        let root = cap_std::fs::Dir::open_ambient_dir(temp.path(), cap_std::ambient_authority())
            .expect("capability root");
        let base = animus_live_steel_agent::parse_live_steel_agent_config(CONFIG).expect("config");
        let definitions = animus_live_steel_agent::store_definitions(&base).expect("definitions");
        close_at_boundary(
            &root,
            &base.with_session_resume_required(false),
            &definitions,
            Boundary::FinalCommit,
        );
        let calls = std::rc::Rc::new(std::cell::Cell::new(NO_PROVIDER_CALLS));
        let result = resume_and_close(
            &root,
            &base.with_session_resume_required(true),
            &definitions,
            std::rc::Rc::clone(&calls),
        );
        assert_eq!(result, Ok(animus_live_steel_agent::PersistentStatus::Idle));
        assert_eq!(calls.get(), NO_PROVIDER_CALLS);
    }

    fn close_at_boundary(
        root: &cap_std::fs::Dir,
        config: &animus_live_steel_agent::ValidatedLiveSteelAgentConfig,
        definitions: &animus_live_steel_agent::LiveStoreDefinitions,
        boundary: Boundary,
    ) {
        assert!(!config.session_resume_required());
        let kernel_persistence =
            animus_live_steel_agent::open_kernel_persistence(root, config, definitions)
                .expect("kernel persistence");
        let kernel = animus_live_steel_agent::restore_or_create_kernel(config, &kernel_persistence)
            .expect("kernel restore");
        let has_committed = std::rc::Rc::new(std::cell::Cell::new(false));
        let provider = animus_live_steel_agent::FixtureStreamProvider::new(
            [TOOL_STREAM.to_vec(), FINAL_STREAM.to_vec()],
            config.max_provider_response_bytes(),
        );
        let mut session = animus_live_steel_agent::LiveSteelAgentComposition::new(
            animus_live_steel_agent::LiveSteelAgentCompositionInput {
                config: config.clone(),
                provider,
                session_persistence: animus_live_steel_agent::open_session_persistence(
                    root,
                    config,
                    definitions,
                )
                .expect("session persistence"),
                snapshot_persistence: animus_live_steel_agent::open_snapshot_persistence(
                    root,
                    definitions,
                )
                .expect("snapshot persistence"),
                kernel,
                host_executor: DenyHost,
                kernel_persistence: super::checkpoint::Observed {
                    inner: kernel_persistence,
                    has_committed: std::rc::Rc::clone(&has_committed),
                },
            },
        )
        .expect("composition")
        .open_session()
        .expect("session");
        session
            .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
                correlation_id: CORRELATION,
                text: PROMPT,
            })
            .expect("begin prompt");
        drive_to_boundary(&mut session, boundary, &has_committed);
        assert!(has_committed.get());
        session
            .close()
            .expect("close leases without driving another session effect");
    }

    fn drive_to_boundary<Provider, Persistence, SnapshotStore, KernelPersistence>(
        session: &mut animus_live_steel_agent::PersistentLiveHost<
            Provider,
            DenyHost,
            Persistence,
            SnapshotStore,
            KernelPersistence,
        >,
        boundary: Boundary,
        has_committed: &std::cell::Cell<bool>,
    ) where
        Provider: animus_live_steel_agent::ProviderStreamService,
        Persistence: animus_live_steel_agent::SessionPersistence,
        SnapshotStore: animus_live_steel_agent::SnapshotPersistence,
        KernelPersistence: animus_live_steel_agent::KernelSafePointPersistence,
    {
        match boundary {
            Boundary::KernelCommit => {
                for _step in 0..INTERRUPT_STEP_LIMIT {
                    session.drive_one().expect("drive one effect");
                    if has_committed.get() {
                        break;
                    }
                }
                assert!(has_committed.get());
                let batch = session
                    .events_after(animus_live_steel_agent::LiveSteelEventRead {
                        cursor: 0,
                        maximum: animus_live_steel_agent::MAX_LIVE_SESSION_EVENT_BATCH,
                    })
                    .expect("events before session result commit");
                assert!(!batch.runtime_events().iter().any(|event| matches!(
                    event,
                    animus_runtime::RuntimeEvent::ToolResponded { .. }
                )));
            }
            Boundary::FinalCommit => {
                session.drive_pending().expect("complete prompt");
                assert_eq!(
                    session.status(),
                    animus_live_steel_agent::PersistentStatus::Completed
                );
            }
        }
    }

    fn resume_and_close(
        root: &cap_std::fs::Dir,
        config: &animus_live_steel_agent::ValidatedLiveSteelAgentConfig,
        definitions: &animus_live_steel_agent::LiveStoreDefinitions,
        calls: std::rc::Rc<std::cell::Cell<u32>>,
    ) -> Result<
        animus_live_steel_agent::PersistentStatus,
        animus_live_steel_agent::LiveSteelAgentError,
    > {
        assert!(config.session_resume_required());
        assert_eq!(calls.get(), NO_PROVIDER_CALLS);
        let kernel_persistence =
            animus_live_steel_agent::open_kernel_persistence(root, config, definitions)
                .expect("kernel persistence");
        let kernel = animus_live_steel_agent::restore_or_create_kernel(config, &kernel_persistence)
            .expect("kernel restore");
        let session = animus_live_steel_agent::LiveSteelAgentComposition::new(
            animus_live_steel_agent::LiveSteelAgentCompositionInput {
                config: config.clone(),
                provider: CountingProvider { calls },
                session_persistence: animus_live_steel_agent::open_session_persistence(
                    root,
                    config,
                    definitions,
                )
                .expect("session persistence"),
                snapshot_persistence: animus_live_steel_agent::open_snapshot_persistence(
                    root,
                    definitions,
                )
                .expect("snapshot persistence"),
                kernel,
                host_executor: DenyHost,
                kernel_persistence,
            },
        )
        .expect("composition")
        .open_session()?;
        let status = session.status();
        session.close()?;
        Ok(status)
    }

    struct CountingProvider {
        calls: std::rc::Rc<std::cell::Cell<u32>>,
    }

    impl animus_live_steel_agent::ProviderStreamService for CountingProvider {
        fn stream(
            &mut self,
            _plan: &animus_std::ProviderDispatchPlan,
            _dispatch: animus_std_openai::OpenAiToolDispatch,
        ) -> Result<
            animus_std_openai::OpenAiStreamOutcome,
            animus_live_steel_agent::ProviderStreamError,
        > {
            let next = self
                .calls
                .get()
                .checked_add(1)
                .ok_or(animus_live_steel_agent::ProviderStreamError::Unavailable)?;
            self.calls.set(next);
            Err(animus_live_steel_agent::ProviderStreamError::Provider)
        }
    }
}
