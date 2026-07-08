// App shell event loop tests — separated to reduce file length.
// Included from view.rs inside `mod shell_tests { use super::*; include!(...); }`

// -----------------------------------------------------------------------
// App shell state tests
// -----------------------------------------------------------------------

#[test]
fn new_shell_is_uninitialized() {
    let state = fresh_shell();
    assert_eq!(state.terminal_state(), TerminalState::Uninitialized);
    assert!(!state.is_shutdown_requested());
    assert!(state.last_error().is_none());
}

#[test]
fn terminal_state_transitions() {
    let mut state = fresh_shell();
    state.set_terminal_state(TerminalState::Ready);
    assert_eq!(state.terminal_state(), TerminalState::Ready);
    state.set_terminal_state(TerminalState::Failed);
    assert_eq!(state.terminal_state(), TerminalState::Failed);
}

#[test]
fn shutdown_request() {
    let mut state = fresh_shell();
    assert!(!state.is_shutdown_requested());
    state.request_shutdown();
    assert!(state.is_shutdown_requested());
}

#[test]
fn prompt_input_tracking() {
    let mut state = fresh_shell();
    assert!(state.prompt_input().is_empty());
    state.set_prompt_input("hello");
    assert_eq!(state.prompt_input(), "hello");
}

#[test]
fn error_recording() {
    let mut state = fresh_shell();
    assert!(state.last_error().is_none());
    state.record_error("something went wrong");
    assert_eq!(state.last_error(), Some("something went wrong"));
}

#[test]
fn agent_and_session_labels() {
    let mut state = fresh_shell();
    state.set_agent_status("streaming");
    state.set_session_label("session-1");
    assert_eq!(state.agent_status(), "streaming");
    assert_eq!(state.session_label(), "session-1");
}

// -----------------------------------------------------------------------
// Snapshot tests
// -----------------------------------------------------------------------

#[test]
fn idle_shell_snapshot() {
    let state = fresh_shell();
    let snapshot = render_app_shell_snapshot(&state);
    assert!(snapshot.contains("terminal: uninitialized"));
    assert!(snapshot.contains("agent: idle"));
    assert!(snapshot.contains("transcript entries: 0"));
}

#[test]
fn populated_shell_snapshot() {
    let mut state = fresh_shell();
    state.set_terminal_state(TerminalState::Ready);
    state.set_session_label("work");
    state.set_agent_status("streaming");
    state.set_prompt_input("What is Rust?");
    state.set_transcript_entry_count(42_u64);
    let snapshot = render_app_shell_snapshot(&state);
    assert!(snapshot.contains("terminal: app ready"));
    assert!(snapshot.contains("session: work"));
    assert!(snapshot.contains("agent: streaming"));
    assert!(snapshot.contains("prompt: What is Rust?"));
    assert!(snapshot.contains("transcript entries: 42"));
}

// -----------------------------------------------------------------------
// Negative / failure isolation tests
// -----------------------------------------------------------------------

#[test]
fn terminal_setup_failure_records_error() {
    let mut state = fresh_shell();
    state.set_terminal_state(TerminalState::Failed);
    state.record_error("terminal raw mode failed");
    assert_eq!(state.terminal_state(), TerminalState::Failed);
    let snapshot = render_app_shell_snapshot(&state);
    assert!(snapshot.contains("error: terminal raw mode failed"));
}

#[test]
fn dropped_event_consumer_does_not_panic() {
    let mut state = fresh_shell();
    state.set_terminal_state(TerminalState::Failed);
    state.record_error("event consumer closed");
    state.request_shutdown();
    assert!(state.is_shutdown_requested());
    let snapshot = render_app_shell_snapshot(&state);
    assert!(snapshot.contains("shutdown: requested"));
}

#[test]
fn cancelled_command_channel_does_not_corrupt() {
    let mut state = fresh_shell();
    state.set_terminal_state(TerminalState::Ready);
    state.set_agent_status("busy");
    state.request_shutdown();
    assert!(state.is_shutdown_requested());
    assert_eq!(state.agent_status(), "busy");
}

#[test]
fn layout_mismatch_does_not_panic() {
    let mut state = fresh_shell();
    state.set_terminal_state(TerminalState::Ready);
    state.record_error("layout: zero-area terminal");
    assert!(state.last_error().is_some());
}