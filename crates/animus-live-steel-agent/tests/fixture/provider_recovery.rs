use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

const PROMPT: &str = "Use Steel to calculate.";
const COUNT_INCREMENT: u32 = 1;
const FIRST_CALL_COUNT: u32 = 1;
const SECOND_CALL_COUNT: u32 = 2;
const SECOND_SNAPSHOT_COMMIT: u32 = 2;
const SECOND_TERMINAL_COMMIT: u32 = 2;
const FINAL_TEXT: &str = "3";

#[test]
fn continuation_snapshot_refusal_happens_before_provider_contact() {
    let calls = Rc::new(Cell::new(0));
    let mut session = session_with(
        controlled_provider(
            [super::TOOL_STREAM.to_vec(), super::FINAL_STREAM.to_vec()],
            Rc::clone(&calls),
            None,
        ),
        ControlledSnapshotPersistence::fail_snapshot(SECOND_SNAPSHOT_COMMIT),
    )
    .open_session()
    .expect("session");
    session
        .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
            correlation_id: super::CORRELATION_A,
            text: PROMPT,
        })
        .expect("begin prompt");

    assert!(session.drive_pending().is_err());
    assert_eq!(calls.get(), FIRST_CALL_COUNT);
    let _ignored = session.drive_one();
    assert_eq!(calls.get(), FIRST_CALL_COUNT);
}

#[test]
fn continuation_provider_failure_blocks_a_second_provider_call() {
    let calls = Rc::new(Cell::new(0));
    let mut session = session_with(
        controlled_provider(
            [super::TOOL_STREAM.to_vec()],
            Rc::clone(&calls),
            Some(SECOND_CALL_COUNT),
        ),
        ControlledSnapshotPersistence::normal(),
    )
    .open_session()
    .expect("session");
    session
        .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
            correlation_id: super::CORRELATION_A,
            text: PROMPT,
        })
        .expect("begin prompt");

    assert!(session.drive_pending().is_err());
    assert_eq!(calls.get(), SECOND_CALL_COUNT);
    let _ignored = session.drive_one();
    assert_eq!(calls.get(), SECOND_CALL_COUNT);
}

#[test]
fn continuation_terminal_refusal_blocks_a_second_provider_call() {
    let calls = Rc::new(Cell::new(0));
    let mut session = session_with(
        controlled_provider(
            [super::TOOL_STREAM.to_vec(), super::FINAL_STREAM.to_vec()],
            Rc::clone(&calls),
            None,
        ),
        ControlledSnapshotPersistence::fail_terminal(SECOND_TERMINAL_COMMIT),
    )
    .open_session()
    .expect("session");
    session
        .begin_prompt(animus_live_steel_agent::LiveSteelPrompt {
            correlation_id: super::CORRELATION_A,
            text: PROMPT,
        })
        .expect("begin prompt");

    assert!(session.drive_pending().is_err());
    assert_eq!(calls.get(), SECOND_CALL_COUNT);
    let _ignored = session.drive_one();
    assert_eq!(calls.get(), SECOND_CALL_COUNT);
}

#[test]
fn successful_continuation_reaches_the_final_response() {
    let calls = Rc::new(Cell::new(0));
    let output = session_with(
        controlled_provider(
            [super::TOOL_STREAM.to_vec(), super::FINAL_STREAM.to_vec()],
            Rc::clone(&calls),
            None,
        ),
        ControlledSnapshotPersistence::normal(),
    )
    .run_prompt(PROMPT)
    .expect("successful continuation");

    assert_eq!(output.final_text(), FINAL_TEXT);
    assert_eq!(calls.get(), SECOND_CALL_COUNT);
}

struct ControlledProvider {
    streams: VecDeque<Vec<u8>>,
    calls: Rc<Cell<u32>>,
    fail_on_call: Option<u32>,
    max_response_bytes: u64,
}

impl animus_live_steel_agent::ProviderStreamService for ControlledProvider {
    fn stream(
        &mut self,
        _plan: &animus_std::ProviderDispatchPlan,
        dispatch: animus_std_openai::OpenAiToolDispatch,
    ) -> Result<animus_std_openai::OpenAiStreamOutcome, animus_live_steel_agent::ProviderStreamError>
    {
        let next = self
            .calls
            .get()
            .checked_add(COUNT_INCREMENT)
            .ok_or(animus_live_steel_agent::ProviderStreamError::Unavailable)?;
        self.calls.set(next);
        if self.fail_on_call == Some(next) {
            return Err(animus_live_steel_agent::ProviderStreamError::Provider);
        }
        let stream = self
            .streams
            .pop_front()
            .ok_or(animus_live_steel_agent::ProviderStreamError::Unavailable)?;
        animus_std_openai::parse_openai_responses_sse_chunks(
            dispatch,
            animus_std_openai::OpenAiStreamBounds::reviewed(self.max_response_bytes),
            &[stream.as_slice()],
        )
        .map_err(|_error| animus_live_steel_agent::ProviderStreamError::Provider)
    }
}

struct ControlledSnapshotPersistence {
    inner: super::persistence::MemorySnapshotPersistence,
    snapshot_commits: u32,
    terminal_commits: u32,
    fail_snapshot_on: Option<u32>,
    fail_terminal_on: Option<u32>,
}

impl ControlledSnapshotPersistence {
    fn normal() -> Self {
        Self {
            inner: super::persistence::MemorySnapshotPersistence::default(),
            snapshot_commits: 0,
            terminal_commits: 0,
            fail_snapshot_on: None,
            fail_terminal_on: None,
        }
    }

    fn fail_snapshot(commit: u32) -> Self {
        Self {
            fail_snapshot_on: Some(commit),
            ..Self::normal()
        }
    }

    fn fail_terminal(commit: u32) -> Self {
        Self {
            fail_terminal_on: Some(commit),
            ..Self::normal()
        }
    }
}

impl animus_live_steel_agent::SnapshotPersistence for ControlledSnapshotPersistence {
    fn close(self) -> Result<(), animus_live_steel_agent::LiveSteelAgentError> {
        Ok(())
    }
}

impl animus_std::model_request_snapshot_host::ModelRequestSnapshotStorePort
    for ControlledSnapshotPersistence
{
    fn commit_snapshot(
        &mut self,
        plan: &animus_runtime::model_request_snapshot::ModelRequestSnapshotCommitPlan,
    ) -> Result<
        animus_runtime::model_request_snapshot::ModelRequestSnapshotCommitObservation,
        animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue,
    > {
        self.snapshot_commits = self.snapshot_commits.checked_add(COUNT_INCREMENT).ok_or(
            animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed,
        )?;
        if self.fail_snapshot_on == Some(self.snapshot_commits) {
            return Err(
                animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed,
            );
        }
        self.inner.commit_snapshot(plan)
    }

    fn commit_terminal(
        &mut self,
        observation: &animus_runtime::model_request_snapshot::ModelRequestProviderTerminalObservation,
    ) -> Result<(), animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue> {
        self.terminal_commits = self.terminal_commits.checked_add(COUNT_INCREMENT).ok_or(
            animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed,
        )?;
        if self.fail_terminal_on == Some(self.terminal_commits) {
            return Err(
                animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue::Failed,
            );
        }
        self.inner.commit_terminal(observation)
    }

    fn load_committed_snapshot(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<
        Option<animus_runtime::model_request_snapshot::CommittedModelRequestSnapshot>,
        animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue,
    > {
        self.inner.load_committed_snapshot(snapshot_identity)
    }

    fn load_terminal_observation(
        &mut self,
        snapshot_identity: &str,
    ) -> Result<
        Option<animus_runtime::model_request_snapshot::ModelRequestProviderTerminalObservation>,
        animus_std::model_request_snapshot_host::ModelRequestSnapshotStoreIssue,
    > {
        self.inner.load_terminal_observation(snapshot_identity)
    }
}

fn controlled_provider(
    streams: impl IntoIterator<Item = Vec<u8>>,
    calls: Rc<Cell<u32>>,
    fail_on_call: Option<u32>,
) -> ControlledProvider {
    let config = animus_live_steel_agent::parse_live_steel_agent_config(super::CONFIG)
        .expect("configuration");
    ControlledProvider {
        streams: streams.into_iter().collect(),
        calls,
        fail_on_call,
        max_response_bytes: config.max_provider_response_bytes(),
    }
}

fn session_with(
    provider: ControlledProvider,
    snapshot_persistence: ControlledSnapshotPersistence,
) -> animus_live_steel_agent::LiveSteelAgentComposition<
    ControlledProvider,
    super::ContextExecutor,
    super::persistence::MemoryPersistence,
    ControlledSnapshotPersistence,
    (),
> {
    let config = animus_live_steel_agent::parse_live_steel_agent_config(super::CONFIG)
        .expect("configuration");
    let kernel = animus_steel::SteelRlmKernel::new(config.rlm_policy().clone()).expect("kernel");
    animus_live_steel_agent::LiveSteelAgentComposition::new(
        animus_live_steel_agent::LiveSteelAgentCompositionInput {
            config,
            provider,
            session_persistence: super::persistence::MemoryPersistence::default(),
            snapshot_persistence,
            kernel,
            host_executor: super::ContextExecutor::default(),
            kernel_persistence: (),
        },
    )
    .expect("composition")
}
