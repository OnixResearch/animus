//! Deterministic `TestBackend` projection tests.

#![cfg(feature = "live-terminal")]

use animus_live_terminal::{
    ClientSessionEvent, ClientSnapshot, TerminalClientState, TerminalLimits, render_terminal_view,
};

const WIDTH: u16 = 80;
const HEIGHT: u16 = 24;

#[test]
fn test_backend_renders_snapshot_first_streaming_and_child_state() {
    let mut state = TerminalClientState::from_snapshot(
        &ClientSnapshot {
            root_id: String::from("root-render"),
            worker_id: String::from("worker-render"),
            generation: 1,
            event_sequence: 0,
            restore_status: animus_local_daemon::RestoreStatus::Restored,
            events: vec![
                ClientSessionEvent::Runtime(animus_runtime::RuntimeEvent::SessionStateChanged {
                    session_id: String::from("session-render"),
                    run_id: Some(String::from("run-render")),
                    status: String::from("streaming"),
                    queue_count: 2,
                }),
                ClientSessionEvent::ChildState {
                    child_id: String::from("child-render"),
                    status: String::from("running"),
                },
            ],
        },
        (WIDTH, HEIGHT),
        limits(),
    )
    .expect("snapshot-first state");
    let backend = ratatui::backend::TestBackend::new(WIDTH, HEIGHT);
    let mut terminal = ratatui::Terminal::new(backend).expect("test backend");
    render_terminal_view(&mut terminal, state.view()).expect("render");
    state.record_rendered().expect("render count");

    let buffer = terminal.backend().buffer();
    let rendered = (0..HEIGHT)
        .map(|row| {
            (0..WIDTH)
                .map(|column| buffer[(column, row)].symbol())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert!(rendered.contains("Animus Live Terminal"));
    assert!(rendered.contains("session: streaming"));
    assert!(rendered.contains("queue: 2"));
    assert!(rendered.contains("child: child-render [running]"));
}

const fn limits() -> TerminalLimits {
    TerminalLimits {
        event_batch: 8,
        prompt_bytes: 1024,
        width: WIDTH,
        height: HEIGHT,
    }
}
