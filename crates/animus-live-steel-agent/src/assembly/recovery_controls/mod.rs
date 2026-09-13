//! Effect-aware recovery controls for private live-shell adapters.

use animus_std::coding_session_host::{CodingSessionPersistenceService, CodingSessionToolService};

const CONFIG: &[u8] = include_bytes!("../../../../../config/generated/live-steel-agent-shell.json");
const TOOL_STREAM: &[u8] =
    include_bytes!("../../../../animus-std-openai/tests/fixtures/openai-responses-tool-call.sse");
const PROMPT: &str = "Use Steel to calculate.";
const CORRELATION_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const GOOD_HEAD: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const BAD_HEAD: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const TOOL_RESULT_STAGE_LIMIT: u32 = 8;
const FIRST_DURABLE_CURSOR: u64 = 1;

mod continuations;
mod persistence;

#[test]
fn crossed_tool_commit_retains_pending_stage_until_exact_observation() {
    let services = pending_tool_result_session().host.into_services();
    let mut tool = services.tool;
    let mut crossed = exact_observation(&tool, true);
    crossed.result_identity = String::from(BAD_HEAD);

    assert_eq!(
        tool.observe_committed(&crossed),
        Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)
    );
    assert!(tool.pending_stage.is_some());
    let observation = exact_observation(&tool, true);
    tool.observe_committed(&observation)
        .expect("exact observation completes once");
    assert!(tool.pending_stage.is_none());
    assert_eq!(
        tool.observe_committed(&observation),
        Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)
    );
}

#[test]
fn unsynchronized_tool_commit_retains_pending_stage() {
    let services = pending_tool_result_session().host.into_services();
    let mut tool = services.tool;
    let observation = exact_observation(&tool, false);

    assert_eq!(
        tool.observe_committed(&observation),
        Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)
    );
    assert!(tool.pending_stage.is_some());
}

#[test]
fn missing_tool_commit_observation_is_rejected_without_pending_stage() {
    let services = pending_tool_result_session().host.into_services();
    let mut tool = services.tool;
    let observation = exact_observation(&tool, true);
    tool.observe_committed(&observation)
        .expect("first observation completes");
    assert_eq!(
        tool.observe_committed(&observation),
        Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)
    );
    assert!(tool.pending_stage.is_none());
}

#[test]
fn duplicate_result_binding_does_not_consume_the_pending_stage() {
    let services = pending_tool_result_session().host.into_services();
    let mut tool = services.tool;
    let output = tool
        .pending_stage
        .as_ref()
        .expect("pending stage")
        .outcome()
        .response()
        .output()
        .to_owned();
    tool.shared
        .borrow_mut()
        .pending_mut()
        .expect("pending provider call")
        .bind_result(output.as_str())
        .expect("manual first bind");
    let observation = exact_observation(&tool, true);

    assert_eq!(
        tool.observe_committed(&observation),
        Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)
    );
    assert!(tool.pending_stage.is_some());
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

struct TestSessionPersistence {
    inner: animus_std::coding_session_host::MemoryCodingSessionPersistence,
}

impl crate::SessionPersistence for TestSessionPersistence {
    fn close(self) -> Result<(), crate::LiveSteelAgentError> {
        Ok(())
    }
}

impl CodingSessionPersistenceService for TestSessionPersistence {
    fn commit(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionCommitRequest,
    ) -> Result<
        animus_std::coding_session_host::CodingSessionCommitObservation,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        self.inner.commit(request)
    }

    fn recover(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionRecoveryRequest,
    ) -> Result<
        animus_runtime::coding_session_host::CodingSessionRecoverySnapshot,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        self.inner.recover(request)
    }
}

fn pending_tool_result_session() -> crate::PersistentLiveHost<
    crate::FixtureStreamProvider,
    DenyHost,
    TestSessionPersistence,
    persistence::Snapshots,
    (),
> {
    let config = crate::parse_live_steel_agent_config(CONFIG).expect("configuration");
    let provider = crate::FixtureStreamProvider::new(
        [TOOL_STREAM.to_vec()],
        config.max_provider_response_bytes(),
    );
    let kernel = animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("kernel");
    let composition =
        super::LiveSteelAgentComposition::new(super::LiveSteelAgentCompositionInput {
            config,
            provider,
            session_persistence: TestSessionPersistence {
                inner: animus_std::coding_session_host::MemoryCodingSessionPersistence::empty(),
            },
            snapshot_persistence: persistence::Snapshots::default(),
            kernel,
            host_executor: DenyHost,
            kernel_persistence: (),
        })
        .expect("composition");
    let mut session = composition.open_session().expect("session");
    session
        .begin_prompt(crate::LiveSteelPrompt {
            correlation_id: CORRELATION_A,
            text: PROMPT,
        })
        .expect("begin prompt");
    for _step in 0..TOOL_RESULT_STAGE_LIMIT {
        if matches!(
            session.host.state_snapshot().pending_effect(),
            Some(animus_runtime::coding_session_host::CodingSessionEffectRequest::Commit(request))
                if request.tool_response_after_commit().is_some()
        ) {
            break;
        }
        session.drive_one().expect("drive to tool result commit");
    }
    assert!(session.host.tool_service_mut().pending_stage.is_some());
    assert!(matches!(
        session.host.state_snapshot().pending_effect(),
        Some(animus_runtime::coding_session_host::CodingSessionEffectRequest::Commit(request))
            if request.tool_response_after_commit().is_some()
    ));
    session
}

fn exact_observation<Executor, Persistence>(
    tool: &super::stages::ToolAdapter<Executor, Persistence>,
    synchronized: bool,
) -> animus_std::coding_session_host::CodingSessionToolCommitObservation {
    let pending = tool.pending_stage.as_ref().expect("pending stage");
    let result_identity = pending
        .outcome()
        .response()
        .content_hash()
        .expect("tool response hash");
    assert!(!pending.plan().plan_identity().is_empty());
    assert!(!result_identity.is_empty());
    animus_std::coding_session_host::CodingSessionToolCommitObservation {
        effect_id: String::from("effect.tool.commit"),
        run_id: String::from("run.tool.commit"),
        result_identity: String::from(result_identity),
        commit_owner_identity:
            animus_runtime::tool_execution_stages::coding_session_tool_commit_owner_identity(),
        durable_head_identity: String::from(GOOD_HEAD),
        active_leaf_id: String::from("entry.tool.commit"),
        durable_cursor: FIRST_DURABLE_CURSOR,
        synchronized,
    }
}
