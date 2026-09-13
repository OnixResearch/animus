//! Durable live-shell restart and restore checks.

const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
const TOOL_STREAM: &[u8] =
    include_bytes!("../../animus-std-openai/tests/fixtures/openai-responses-tool-call.sse");
const FINAL_STREAM: &[u8] =
    include_bytes!("../../animus-std-openai/tests/fixtures/openai-responses-final.sse");
const FIRST_PROMPT: &str = "Use Steel for the first durable turn.";
const SECOND_PROMPT: &str = "Use Steel after restart.";
const INITIAL_AUTHORITY_BINDING_COUNT: u32 = 1;
const TURN_MESSAGES_PER_RUN: u32 = 4;
const FIRST_RUN_TOTAL_MESSAGES: u32 = INITIAL_AUTHORITY_BINDING_COUNT + TURN_MESSAGES_PER_RUN;
const SECOND_RUN_TOTAL_MESSAGES: u32 = FIRST_RUN_TOTAL_MESSAGES + TURN_MESSAGES_PER_RUN;

#[derive(Default)]
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
fn durable_restart_restores_session_and_kernel_safe_points() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.session.restart]
    // r[verify agent_harness_runtime.live_steel_agent_shell.verification]
    let temp = tempfile::tempdir().expect("temporary store root");
    let root = cap_std::fs::Dir::open_ambient_dir(temp.path(), cap_std::ambient_authority())
        .expect("capability root");
    let base = animus_live_steel_agent::parse_live_steel_agent_config(CONFIG).expect("config");
    let definitions = animus_live_steel_agent::store_definitions(&base).expect("store definitions");
    assert!(
        !animus_live_steel_agent::session_resume_required(&root, &base, &definitions)
            .expect("empty session")
    );

    let first_output = run_once(&root, &base, &definitions, FIRST_PROMPT, false);
    assert_eq!(
        first_output.receipt().committed_message_count(),
        FIRST_RUN_TOTAL_MESSAGES
    );
    assert!(
        animus_live_steel_agent::session_resume_required(&root, &base, &definitions)
            .expect("committed session")
    );

    let second_output = run_once(&root, &base, &definitions, SECOND_PROMPT, true);
    assert_eq!(
        second_output.receipt().committed_message_count(),
        SECOND_RUN_TOTAL_MESSAGES
    );
    assert_ne!(
        first_output.receipt().kernel_head_identity(),
        second_output.receipt().kernel_head_identity()
    );
    assert_eq!(second_output.final_text(), "3");
}

#[test]
fn interrupted_restore_returns_the_prior_committed_kernel_without_effect_replay() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.session.restart]
    let temp = tempfile::tempdir().expect("temporary store root");
    let root = cap_std::fs::Dir::open_ambient_dir(temp.path(), cap_std::ambient_authority())
        .expect("capability root");
    let config = animus_live_steel_agent::parse_live_steel_agent_config(CONFIG).expect("config");
    let definitions =
        animus_live_steel_agent::store_definitions(&config).expect("store definitions");
    let persistence =
        animus_live_steel_agent::open_kernel_persistence(&root, &config, &definitions)
            .expect("kernel store");
    let mut kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("kernel");
    let completed = kernel.evaluate("(+ 1 2)").expect("committed cell");
    assert!(matches!(
        completed,
        animus_steel::SteelRlmEvaluation::Value { .. }
    ));
    let snapshot = kernel
        .private_journal_snapshot(config.session_identity(), config.authority_generation())
        .expect("snapshot");
    let expected_head = snapshot.committed_head_identity().to_owned();
    let mut persistence = persistence;
    persistence
        .commit_snapshot(&snapshot)
        .expect("snapshot commit");
    persistence.close().expect("close first kernel store");

    let reopened = animus_live_steel_agent::open_kernel_persistence(&root, &config, &definitions)
        .expect("reopen kernel");
    let restored = animus_live_steel_agent::restore_interrupted_kernel(
        &config,
        &reopened,
        animus_runtime::active_turn_recovery::ActiveTurnRecoveryAction::PlanInterruptedToolRepair,
    )
    .expect("interrupted restore");
    let restored_head = restored
        .private_journal_snapshot(config.session_identity(), config.authority_generation())
        .expect("restored snapshot")
        .committed_head_identity()
        .to_owned();
    assert_eq!(restored_head, expected_head);
    reopened.close().expect("close restored kernel store");
}

#[test]
fn present_kernel_drift_fails_without_fresh_state_fallback() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.session.restart]
    let temp = tempfile::tempdir().expect("temporary store root");
    let root = cap_std::fs::Dir::open_ambient_dir(temp.path(), cap_std::ambient_authority())
        .expect("capability root");
    let config = animus_live_steel_agent::parse_live_steel_agent_config(CONFIG).expect("config");
    let definitions =
        animus_live_steel_agent::store_definitions(&config).expect("store definitions");
    let persistence =
        animus_live_steel_agent::open_kernel_persistence(&root, &config, &definitions)
            .expect("kernel store");
    let mut kernel =
        animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("kernel");
    let completed = kernel.evaluate("(+ 1 2)").expect("committed cell");
    assert!(matches!(
        completed,
        animus_steel::SteelRlmEvaluation::Value { .. }
    ));
    let snapshot = kernel
        .private_journal_snapshot(config.session_identity(), config.authority_generation())
        .expect("snapshot");
    let mut persistence = persistence;
    persistence
        .commit_snapshot(&snapshot)
        .expect("snapshot commit");
    persistence.close().expect("close first kernel store");

    let drifted_json = String::from_utf8(CONFIG.to_vec())
        .expect("UTF-8")
        .replace("\"max_source_chars\": 4096", "\"max_source_chars\": 4097");
    let drifted = animus_live_steel_agent::parse_live_steel_agent_config(drifted_json.as_bytes())
        .expect("drifted config");
    let reopened = animus_live_steel_agent::open_kernel_persistence(&root, &drifted, &definitions)
        .expect("reopen kernel");
    assert!(animus_live_steel_agent::restore_or_create_kernel(&drifted, &reopened).is_err());
    reopened.close().expect("close drift store");
}

#[allow(
    clippy::too_many_arguments,
    reason = "fixture keeps restart inputs explicit"
)]
fn run_once(
    root: &cap_std::fs::Dir,
    base: &animus_live_steel_agent::ValidatedLiveSteelAgentConfig,
    definitions: &animus_live_steel_agent::LiveStoreDefinitions,
    prompt: &str,
    resume_required: bool,
) -> animus_live_steel_agent::LiveSteelAgentRunOutput {
    let config = base.with_session_resume_required(resume_required);
    let session_persistence =
        animus_live_steel_agent::open_session_persistence(root, &config, definitions)
            .expect("session persistence");
    let snapshot_persistence =
        animus_live_steel_agent::open_snapshot_persistence(root, definitions)
            .expect("snapshot persistence");
    let kernel_persistence =
        animus_live_steel_agent::open_kernel_persistence(root, &config, definitions)
            .expect("kernel persistence");
    let kernel = animus_live_steel_agent::restore_or_create_kernel(&config, &kernel_persistence)
        .expect("kernel restore");
    let provider = animus_live_steel_agent::FixtureStreamProvider::new(
        [TOOL_STREAM.to_vec(), FINAL_STREAM.to_vec()],
        config.max_provider_response_bytes(),
    );
    animus_live_steel_agent::LiveSteelAgentComposition::new(
        animus_live_steel_agent::LiveSteelAgentCompositionInput {
            config: config.clone(),
            provider,
            session_persistence,
            snapshot_persistence,
            kernel,
            host_executor: DenyHost,
            kernel_persistence,
        },
    )
    .expect("composition")
    .run_prompt(prompt)
    .expect("durable run")
}
