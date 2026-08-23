//! Bounded event-loop orchestration tests.

use animus_live_terminal::{
    ClientCommand, ClientSessionEvent, ClientSnapshot, LiveTerminalRequest,
    TerminalConnectionError, TerminalConnectionPort, TerminalExit, TerminalHostError,
    TerminalHostPort, TerminalInput, preflight_live_terminal_activation, run_attached_terminal,
    run_attached_terminal_with_activation,
};
use std::collections::VecDeque;

const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-terminal-agent-host.json");
const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const FIRST_SEQUENCE: u64 = 1;
const MAXIMUM_ITERATIONS: u64 = 16;
const IDLE_BATCH_TEST_ITERATIONS: u64 = 25;
const IDLE_BATCH_TEST_INPUT_COUNT: usize = 25;
const EXPECTED_IDLE_EVENT_POLLS: u64 = 2;
const INPUT_POLL_TEST_ITERATIONS: u64 = 2;
const EXPECTED_INPUT_EVENT_POLLS: u64 = 2;
const INITIAL_WIDTH: u16 = 80;
const INITIAL_HEIGHT: u16 = 24;
const RESIZED_WIDTH: u16 = 100;
const RESIZED_HEIGHT: u16 = 30;

#[test]
fn input_resize_streaming_commands_and_detach_are_bounded() {
    let config = direct_config();
    let mut connection = FixtureConnection::new(vec![
        vec![ClientSessionEvent::Runtime {
            sequence: FIRST_SEQUENCE,
            event: animus_runtime::RuntimeEvent::SessionStateChanged {
                session_id: String::from("session-host"),
                run_id: Some(String::from("run-host")),
                status: String::from("streaming"),
                queue_count: 1,
            },
        }],
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ]);
    let mut host = FixtureHost::new([
        Some(TerminalInput::Character('h')),
        Some(TerminalInput::Character('i')),
        Some(TerminalInput::Submit),
        Some(TerminalInput::Resize {
            width: RESIZED_WIDTH,
            height: RESIZED_HEIGHT,
        }),
        Some(TerminalInput::Quit),
    ]);
    let (state, exit) = run_attached_terminal(LiveTerminalRequest {
        config: &config,
        connection: &mut connection,
        host: &mut host,
        maximum_iterations: MAXIMUM_ITERATIONS,
    })
    .expect("bounded terminal loop");

    assert_eq!(exit, TerminalExit::Normal);
    assert_eq!(state.view().viewport(), (RESIZED_WIDTH, RESIZED_HEIGHT));
    assert_eq!(state.view().coding_session().state().queue_count, 1);
    assert_eq!(connection.commands.len(), 1);
    assert!(matches!(
        connection.commands[0],
        ClientCommand::Prompt { ref text, .. } if text == "hi"
    ));
    assert!(connection.detached);
    assert_eq!(host.poll_count, 5);
    assert!(host.render_count <= host.poll_count + 1);
}

#[test]
fn queue_conflict_signal_disconnect_and_backend_errors_keep_typed_exit_paths() {
    let config = direct_config();
    let mut conflict_connection = FixtureConnection::new(vec![
        vec![ClientSessionEvent::QueueConflict {
            generation: 2,
            queue_identity: String::from(HASH),
        }],
        Vec::new(),
    ]);
    let mut conflict_host = FixtureHost::new([None, Some(TerminalInput::Quit)]);
    let (_state, exit) = run_attached_terminal(LiveTerminalRequest {
        config: &config,
        connection: &mut conflict_connection,
        host: &mut conflict_host,
        maximum_iterations: MAXIMUM_ITERATIONS,
    })
    .expect("conflict flow");
    assert_eq!(exit, TerminalExit::Normal);
    assert!(conflict_connection.refreshed);
    assert!(conflict_connection.commands.is_empty());

    let mut signal_connection = FixtureConnection::new(vec![Vec::new()]);
    let mut signal_host = FixtureHost::new([None]);
    signal_host.signalled = true;
    let (_state, exit) = run_attached_terminal(LiveTerminalRequest {
        config: &config,
        connection: &mut signal_connection,
        host: &mut signal_host,
        maximum_iterations: MAXIMUM_ITERATIONS,
    })
    .expect("signal flow");
    assert_eq!(exit, TerminalExit::Signal);
    assert!(signal_connection.detached);

    let mut cancelled_connection = FixtureConnection::new(vec![Vec::new()]);
    let mut cancelled_host = FixtureHost::new([None]);
    cancelled_host.cancelled = true;
    let (_state, exit) = run_attached_terminal(LiveTerminalRequest {
        config: &config,
        connection: &mut cancelled_connection,
        host: &mut cancelled_host,
        maximum_iterations: MAXIMUM_ITERATIONS,
    })
    .expect("cancellation flow");
    assert_eq!(exit, TerminalExit::Cancelled);
    assert!(cancelled_connection.detached);

    let mut disconnected = FixtureConnection::new(vec![vec![ClientSessionEvent::Disconnected]]);
    let mut disconnect_host = FixtureHost::new([None]);
    let (_state, exit) = run_attached_terminal(LiveTerminalRequest {
        config: &config,
        connection: &mut disconnected,
        host: &mut disconnect_host,
        maximum_iterations: MAXIMUM_ITERATIONS,
    })
    .expect("disconnect flow");
    assert_eq!(exit, TerminalExit::Disconnected);
    assert!(disconnected.detached);

    let mut backend_connection = FixtureConnection::new(Vec::new());
    let mut backend_host = FixtureHost::new([None]);
    backend_host.fail_render = true;
    assert!(matches!(
        run_attached_terminal(LiveTerminalRequest {
            config: &config,
            connection: &mut backend_connection,
            host: &mut backend_host,
            maximum_iterations: MAXIMUM_ITERATIONS,
        }),
        Err(TerminalHostError::Terminal)
    ));
    assert!(backend_connection.detached);
}

#[test]
fn event_flood_and_iteration_exhaustion_fail_closed_without_automatic_mode_switch() {
    let config = direct_config();
    let flood_count = usize::try_from(config.max_event_batch()).expect("batch bound") + 1;
    let mut flood_connection = FixtureConnection::new(vec![vec![
        ClientSessionEvent::Failed {
            class: String::from("bounded_failure"),
        };
        flood_count
    ]]);
    let mut flood_host = FixtureHost::new([None]);
    assert!(matches!(
        run_attached_terminal(LiveTerminalRequest {
            config: &config,
            connection: &mut flood_connection,
            host: &mut flood_host,
            maximum_iterations: MAXIMUM_ITERATIONS,
        }),
        Err(TerminalHostError::Core(
            animus_live_terminal::TerminalCoreError::EventFlood
        ))
    ));
    assert!(flood_connection.detached);

    let mut idle_connection = FixtureConnection::new(vec![Vec::new()]);
    let mut idle_host = FixtureHost::new([None]);
    assert!(matches!(
        run_attached_terminal(LiveTerminalRequest {
            config: &config,
            connection: &mut idle_connection,
            host: &mut idle_host,
            maximum_iterations: 1,
        }),
        Err(TerminalHostError::IterationLimit)
    ));
    assert!(idle_connection.detached);
}

#[test]
fn idle_terminal_batches_event_polls() {
    let config = direct_config();
    let mut connection = FixtureConnection::new(Vec::new());
    let inputs: [Option<TerminalInput>; IDLE_BATCH_TEST_INPUT_COUNT] =
        std::array::from_fn(|_index| None);
    let mut host = FixtureHost::new(inputs);

    let result = run_attached_terminal(LiveTerminalRequest {
        config: &config,
        connection: &mut connection,
        host: &mut host,
        maximum_iterations: IDLE_BATCH_TEST_ITERATIONS,
    });

    assert!(matches!(result, Err(TerminalHostError::IterationLimit)));
    assert_eq!(host.poll_count, IDLE_BATCH_TEST_ITERATIONS);
    assert_eq!(connection.event_poll_count, EXPECTED_IDLE_EVENT_POLLS);
    assert!(connection.detached);
}

#[test]
fn local_input_forces_an_immediate_event_poll() {
    let config = direct_config();
    let mut connection = FixtureConnection::new(Vec::new());
    let mut host = FixtureHost::new([None, Some(TerminalInput::Character('x'))]);

    let result = run_attached_terminal(LiveTerminalRequest {
        config: &config,
        connection: &mut connection,
        host: &mut host,
        maximum_iterations: INPUT_POLL_TEST_ITERATIONS,
    });

    assert!(matches!(result, Err(TerminalHostError::IterationLimit)));
    assert_eq!(host.poll_count, INPUT_POLL_TEST_ITERATIONS);
    assert_eq!(connection.event_poll_count, EXPECTED_INPUT_EVENT_POLLS);
    assert!(connection.detached);
}

#[test]
fn blocked_activation_calls_no_terminal_or_connection_effect() {
    // r[verify agent_harness_runtime.composition_activation_graph.fail_loud]
    let config = direct_config();
    let mut activation = preflight_live_terminal_activation().expect("activation profile");
    activation.status =
        animus_runtime::composition_activation_graph::ActivationDecisionStatus::Blocked;
    let mut connection = FixtureConnection::new(Vec::new());
    let mut host = FixtureHost::new([None]);

    let result = run_attached_terminal_with_activation(
        LiveTerminalRequest {
            config: &config,
            connection: &mut connection,
            host: &mut host,
            maximum_iterations: MAXIMUM_ITERATIONS,
        },
        &activation,
    );

    assert!(matches!(result, Err(TerminalHostError::Activation)));
    assert_eq!(host.viewport_count, 0);
    assert_eq!(host.poll_count, 0);
    assert_eq!(host.render_count, 0);
    assert!(connection.commands.is_empty());
    assert!(!connection.detached);

    let mut forged = preflight_live_terminal_activation().expect("activation profile");
    forged.graph_identity = String::from("forged-activation-graph");
    let forged_result = run_attached_terminal_with_activation(
        LiveTerminalRequest {
            config: &config,
            connection: &mut connection,
            host: &mut host,
            maximum_iterations: MAXIMUM_ITERATIONS,
        },
        &forged,
    );
    assert!(matches!(forged_result, Err(TerminalHostError::Activation)));
    assert_eq!(host.viewport_count, 0);
    assert!(connection.commands.is_empty());
}

struct FixtureHost {
    inputs: VecDeque<Option<TerminalInput>>,
    viewport: (u16, u16),
    cancelled: bool,
    signalled: bool,
    fail_render: bool,
    viewport_count: u64,
    poll_count: u64,
    render_count: u64,
}

impl FixtureHost {
    fn new<const COUNT: usize>(inputs: [Option<TerminalInput>; COUNT]) -> Self {
        Self {
            inputs: VecDeque::from(inputs),
            viewport: (INITIAL_WIDTH, INITIAL_HEIGHT),
            cancelled: false,
            signalled: false,
            fail_render: false,
            viewport_count: 0,
            poll_count: 0,
            render_count: 0,
        }
    }
}

impl TerminalHostPort for FixtureHost {
    fn viewport(&mut self) -> Result<(u16, u16), TerminalHostError> {
        self.viewport_count += 1;
        Ok(self.viewport)
    }

    fn poll_input(&mut self, _timeout_ms: u64) -> Result<Option<TerminalInput>, TerminalHostError> {
        self.poll_count += 1;
        Ok(self.inputs.pop_front().flatten())
    }

    fn cancelled(&self) -> bool {
        self.cancelled
    }

    fn signalled(&self) -> bool {
        self.signalled
    }

    fn render(
        &mut self,
        _view: &animus_live_terminal::TerminalViewState,
    ) -> Result<(), TerminalHostError> {
        if self.fail_render {
            return Err(TerminalHostError::Terminal);
        }
        self.render_count += 1;
        Ok(())
    }
}

struct FixtureConnection {
    event_batches: VecDeque<Vec<ClientSessionEvent>>,
    commands: Vec<ClientCommand>,
    event_poll_count: u64,
    refreshed: bool,
    stopped: bool,
    detached: bool,
}

impl FixtureConnection {
    fn new(event_batches: Vec<Vec<ClientSessionEvent>>) -> Self {
        Self {
            event_batches: VecDeque::from(event_batches),
            commands: Vec::new(),
            event_poll_count: 0,
            refreshed: false,
            stopped: false,
            detached: false,
        }
    }
}

impl TerminalConnectionPort for FixtureConnection {
    fn attach(
        &mut self,
        root_id: &str,
        _client_id: &str,
        _maximum_events: u32,
    ) -> Result<ClientSnapshot, TerminalConnectionError> {
        Ok(ClientSnapshot {
            root_id: String::from(root_id),
            worker_id: String::from("worker-host"),
            generation: 1,
            event_sequence: 0,
            restore_status: animus_local_daemon::RestoreStatus::Restored,
            events: Vec::new(),
        })
    }

    fn refresh(
        &mut self,
        root_id: &str,
        _client_id: &str,
        _maximum_events: u32,
    ) -> Result<ClientSnapshot, TerminalConnectionError> {
        self.refreshed = true;
        Ok(ClientSnapshot {
            root_id: String::from(root_id),
            worker_id: String::from("worker-host"),
            generation: 1,
            event_sequence: 0,
            restore_status: animus_local_daemon::RestoreStatus::Restored,
            events: Vec::new(),
        })
    }

    fn events_after(
        &mut self,
        _root_id: &str,
        _sequence: u64,
        _maximum_events: u32,
    ) -> Result<Vec<ClientSessionEvent>, TerminalConnectionError> {
        self.event_poll_count = self.event_poll_count.saturating_add(1);
        Ok(self.event_batches.pop_front().unwrap_or_default())
    }

    fn route(
        &mut self,
        _root_id: &str,
        _generation: u64,
        command: &ClientCommand,
    ) -> Result<String, TerminalConnectionError> {
        self.commands.push(command.clone());
        Ok(String::from(HASH))
    }

    fn stop(&mut self, _root_id: &str, _generation: u64) -> Result<(), TerminalConnectionError> {
        self.stopped = true;
        Ok(())
    }

    fn detach(&mut self, _root_id: &str, _client_id: &str) -> Result<(), TerminalConnectionError> {
        self.detached = true;
        Ok(())
    }
}

fn direct_config() -> animus_live_terminal::ValidatedTerminalConfig {
    let mut config: serde_json::Value = serde_json::from_slice(CONFIG).expect("terminal config");
    config["connection_mode"] = serde_json::Value::String(String::from("direct"));
    animus_live_terminal::parse_terminal_config(
        serde_json::to_vec(&config)
            .expect("direct terminal config")
            .as_slice(),
    )
    .expect("direct mode")
}
