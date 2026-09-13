#[test]
pub(super) fn production_entrypoint_runs_prompt_tool_result_continuation_and_final_commit() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.verification]
    // r[verify agent_harness_runtime.live_steel_agent_shell.tool_loop.complete]
    let config = animus_live_steel_agent::parse_live_steel_agent_config(super::CONFIG)
        .expect("fixture configuration");
    let provider = animus_live_steel_agent::FixtureStreamProvider::new(
        [super::TOOL_STREAM.to_vec(), super::FINAL_STREAM.to_vec()],
        config.max_provider_response_bytes(),
    );
    let kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("fixture kernel");
    let composition = animus_live_steel_agent::LiveSteelAgentComposition::new(
        animus_live_steel_agent::LiveSteelAgentCompositionInput {
            config,
            provider,
            session_persistence: super::persistence::Journal::default(),
            snapshot_persistence: super::persistence::Snapshots::default(),
            kernel,
            host_executor: super::ContextExecutor::default(),
            kernel_persistence: (),
        },
    )
    .expect("composition");
    let output = composition
        .run_prompt("Use Steel to calculate.")
        .expect("complete tool loop");

    assert_eq!(
        output.receipt().provider_dispatch_count(),
        super::EXPECTED_PROVIDER_DISPATCHES
    );
    assert_eq!(
        output.receipt().tool_dispatch_count(),
        super::EXPECTED_TOOL_DISPATCHES
    );
    assert_output(&output);
}

#[test]
pub(super) fn persistent_session_drives_incrementally_and_finishes_with_private_events() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.persistent_session]
    let config = animus_live_steel_agent::parse_live_steel_agent_config(super::CONFIG)
        .expect("fixture configuration");
    let provider = animus_live_steel_agent::FixtureStreamProvider::new(
        [super::TOOL_STREAM.to_vec(), super::FINAL_STREAM.to_vec()],
        config.max_provider_response_bytes(),
    );
    let kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("fixture kernel");
    let composition = animus_live_steel_agent::LiveSteelAgentComposition::new(
        animus_live_steel_agent::LiveSteelAgentCompositionInput {
            config,
            provider,
            session_persistence: super::persistence::Journal::default(),
            snapshot_persistence: super::persistence::Snapshots::default(),
            kernel,
            host_executor: super::ContextExecutor::default(),
            kernel_persistence: (),
        },
    )
    .expect("composition");
    let mut session = composition.open_session().expect("persistent session");
    session
        .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
            correlation_id: super::CORRELATION_A,
            text: "Use Steel to calculate.",
        })
        .expect("begin prompt");
    assert_eq!(
        session.status(),
        animus_live_steel_agent::PersistentStatus::Running
    );
    for _step in 0..animus_runtime::coding_session_host::coding_session_effect_step_bound() {
        if session.status() == animus_live_steel_agent::PersistentStatus::Completed {
            break;
        }
        assert!(session.drive_one().expect("one bounded effect"));
    }
    assert_eq!(
        session.status(),
        animus_live_steel_agent::PersistentStatus::Completed
    );
    let first_batch = session
        .events_after(animus_live_steel_agent::LiveSteelEventRead {
            cursor: 0,
            maximum: animus_live_steel_agent::MAX_LIVE_SESSION_EVENT_BATCH,
        })
        .expect("bounded private events");
    assert!(!first_batch.runtime_events().is_empty());
    let output = session.finish().expect("completed output");
    assert_eq!(output.final_text(), "3");
    assert_eq!(
        first_batch.next_cursor(),
        u32::try_from(output.runtime_events().len()).expect("bounded event cursor")
    );
}

fn assert_output(output: &animus_live_steel_agent::LiveSteelAgentRunOutput) {
    assert_eq!(output.final_text(), "3");
    // r[verify agent_harness_runtime.live_terminal_agent_host.private_events.source]
    assert_eq!(
        u32::try_from(output.runtime_events().len()).expect("bounded event count"),
        output.receipt().event_count()
    );
    assert!(
        output
            .runtime_events()
            .iter()
            .any(|event| matches!(event, animus_runtime::RuntimeEvent::ToolRequested { .. }))
    );
    assert!(
        output
            .runtime_events()
            .iter()
            .any(|event| matches!(event, animus_runtime::RuntimeEvent::ToolResponded { .. }))
    );
    assert!(
        output
            .runtime_events()
            .iter()
            .any(|event| matches!(event, animus_runtime::RuntimeEvent::ModelResponded { .. }))
    );
    assert!(
        output
            .runtime_events()
            .iter()
            .any(|event| matches!(event, animus_runtime::RuntimeEvent::FinalResponse { .. }))
    );
    assert_eq!(
        output.receipt().committed_message_count(),
        super::EXPECTED_MESSAGES
    );
    assert_eq!(
        output.receipt().provider_dispatch_count(),
        super::EXPECTED_PROVIDER_DISPATCHES
    );
    assert_eq!(
        output.receipt().tool_dispatch_count(),
        super::EXPECTED_TOOL_DISPATCHES
    );
    assert_eq!(
        output.receipt().host_observation_count(),
        super::EXPECTED_ZERO_HOST_EFFECTS
    );
    let rendered = output.receipt().render();
    for secret in [
        "Use Steel",
        "(+ 1 2)",
        "call_fixture",
        "resp_tool_fixture",
        "sk-test",
    ] {
        assert!(!rendered.contains(secret));
    }
}
