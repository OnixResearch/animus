pub(super) const OVER_BOUND_EVENT_BATCH: u32 = 257;
#[test]
pub(super) fn persistent_session_interleaves_queue_mutation_and_cancellation() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.persistent_session.controls]
    let mut session = fixture_session();
    session
        .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
            correlation_id: super::CORRELATION_A,
            text: "first private prompt",
        })
        .expect("begin prompt");
    assert!(session.drive_one().expect("commit prompt"));
    session
        .begin_follow_up(animus_live_steel_agent::LiveSteelQueuedInput {
            correlation_id: super::CORRELATION_B,
            text: "queued private follow-up",
        })
        .expect("queue follow-up");
    let before = session.queue_snapshot().expect("safe queue snapshot");
    let item_id = before.entries.first().expect("queued item").item_id.clone();
    session
        .begin_queue_edit(animus_live_steel_agent::LiveSteelQueueEdit {
            item_id: item_id.as_str(),
            expected_generation: before.generation.value(),
            expected_queue_identity: before.queue_identity.as_str(),
            replacement: "edited private follow-up",
        })
        .expect("exact queue edit");
    let after = session.queue_snapshot().expect("edited queue snapshot");
    assert_ne!(before.queue_identity, after.queue_identity);
    let stale = session.begin_queue_removal(animus_live_steel_agent::LiveSteelQueueRemoval {
        item_id: item_id.as_str(),
        expected_generation: before.generation.value(),
        expected_queue_identity: before.queue_identity.as_str(),
    });
    assert!(stale.is_err());
    session.begin_cancel().expect("begin cancellation");
    session.drive_pending().expect("observe cancellation");
    assert_eq!(
        session.status(),
        animus_live_steel_agent::PersistentStatus::Cancelled
    );
    let events = session
        .events_after(animus_live_steel_agent::LiveSteelEventRead {
            cursor: 0,
            maximum: animus_live_steel_agent::MAX_LIVE_SESSION_EVENT_BATCH,
        })
        .expect("private event batch");
    assert!(events.runtime_events().iter().any(|event| matches!(
        event,
        animus_runtime::RuntimeEvent::SessionQueueMutationObserved { .. }
    )));
    session.close().expect("release cancelled session");
}

#[test]
pub(super) fn persistent_session_rejects_malformed_identity_and_event_bounds() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.persistent_session.failure]
    let mut session = fixture_session();
    session
        .answer_approval(
            super::CORRELATION_A,
            animus_live_steel_agent::LiveApprovalDecision::Approve,
        )
        .expect("approval delegation");
    assert_eq!(
        session.child_status("child-active").expect("child status"),
        animus_live_steel_agent::LiveChildStatus::Active
    );
    assert_eq!(
        session.cancel_child("child-active").expect("child cancel"),
        animus_live_steel_agent::LiveChildStatus::Cancelled
    );
    assert!(
        session
            .answer_approval(
                super::INVALID_CORRELATION,
                animus_live_steel_agent::LiveApprovalDecision::Deny
            )
            .is_err()
    );
    assert!(session.child_status("missing child").is_err());
    assert!(
        session
            .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
                correlation_id: super::INVALID_CORRELATION,
                text: "private prompt",
            })
            .is_err()
    );
    assert!(
        session
            .events_after(animus_live_steel_agent::LiveSteelEventRead {
                cursor: 0,
                maximum: 0,
            })
            .is_err()
    );
    assert!(
        session
            .events_after(animus_live_steel_agent::LiveSteelEventRead {
                cursor: 0,
                maximum: OVER_BOUND_EVENT_BATCH,
            })
            .is_err()
    );
    assert!(
        session
            .events_after(animus_live_steel_agent::LiveSteelEventRead {
                cursor: 1,
                maximum: 1,
            })
            .is_err()
    );
    assert_eq!(
        session.status(),
        animus_live_steel_agent::PersistentStatus::Idle
    );
    session.close().expect("release idle session");
}

fn fixture_session() -> animus_live_steel_agent::PersistentLiveHost<
    animus_live_steel_agent::FixtureStreamProvider,
    super::ContextExecutor,
    super::persistence::MemoryPersistence,
    super::persistence::MemorySnapshotPersistence,
    (),
> {
    let config = animus_live_steel_agent::parse_live_steel_agent_config(super::CONFIG)
        .expect("fixture configuration");
    let provider = animus_live_steel_agent::FixtureStreamProvider::new(
        [super::FINAL_STREAM.to_vec()],
        config.max_provider_response_bytes(),
    );
    let kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("fixture kernel");
    let composition = animus_live_steel_agent::LiveSteelAgentComposition::new(
        animus_live_steel_agent::LiveSteelAgentCompositionInput {
            config,
            provider,
            session_persistence: super::persistence::MemoryPersistence::default(),
            snapshot_persistence: super::persistence::MemorySnapshotPersistence::default(),
            kernel,
            host_executor: super::ContextExecutor::default(),
            kernel_persistence: (),
        },
    )
    .expect("composition");
    composition.open_session().expect("persistent session")
}
