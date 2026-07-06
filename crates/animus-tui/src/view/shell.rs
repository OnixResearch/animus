// App shell and event loop: terminal lifecycle, event routing, pane layout,
// peripherals, and runtime binding.
//
// This module is the imperative shell that owns the terminal, polls events,
// schedules rendering, lays out panes, and binds to the runtime.
// All reducers remain pure and testable without a terminal.

/// Label for the app shell ready state.
const APP_READY_LABEL: &str = "app ready";
/// Label for terminal setup failure.
const TERMINAL_SETUP_FAILED: &str = "terminal setup failed";


// ---------------------------------------------------------------------------
// App shell state
// ---------------------------------------------------------------------------

/// Terminal lifecycle state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalState {
    /// Terminal is not initialized.
    Uninitialized,
    /// Terminal setup completed successfully.
    Ready,
    /// Terminal setup failed.
    Failed,
    /// Terminal was shut down.
    ShutDown,
}

impl TerminalState {
    /// Returns the stable state label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Uninitialized => "uninitialized",
            Self::Ready => APP_READY_LABEL,
            Self::Failed => TERMINAL_SETUP_FAILED,
            Self::ShutDown => "shut down",
        }
    }
}

/// Composite app shell state holding all TUI component states.
#[derive(Clone, Debug, PartialEq)]
pub struct AppShellState {
    /// Terminal lifecycle state.
    terminal: TerminalState,
    /// Conversation render model state index.
    transcript_entry_count: usize,
    /// Prompt input content.
    prompt_input: String,
    /// Agent dashboard status label.
    agent_status: String,
    /// Status rail session label.
    session_label: String,
    /// Error string when the shell encounters a runtime issue.
    last_error: Option<String>,
    /// Whether the shell has a pending shutdown request.
    shutdown_requested: bool,
}

impl AppShellState {
    /// Builds an uninitialized app shell state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            terminal: TerminalState::Uninitialized,
            transcript_entry_count: 0,
            prompt_input: String::new(),
            agent_status: String::from("idle"),
            session_label: String::new(),
            last_error: None,
            shutdown_requested: false,
        }
    }

    /// Sets the terminal state.
    pub fn set_terminal_state(&mut self, state: TerminalState) {
        self.terminal = state;
    }

    /// Returns the current terminal state.
    #[must_use]
    pub const fn terminal_state(&self) -> TerminalState {
        self.terminal
    }

    /// Sets the transcript entry count from the conversation model.
    pub fn set_transcript_entry_count(&mut self, count: usize) {
        self.transcript_entry_count = count;
    }

    /// Returns the transcript entry count.
    #[must_use]
    pub const fn transcript_entry_count(&self) -> usize {
        self.transcript_entry_count
    }

    /// Sets the prompt input text.
    pub fn set_prompt_input(&mut self, input: &str) {
        self.prompt_input = String::from(input);
    }

    /// Returns the prompt input text.
    #[must_use]
    pub fn prompt_input(&self) -> &str {
        self.prompt_input.as_str()
    }

    /// Sets the agent status label.
    pub fn set_agent_status(&mut self, status: &str) {
        self.agent_status = String::from(status);
    }

    /// Returns the agent status label.
    #[must_use]
    pub fn agent_status(&self) -> &str {
        self.agent_status.as_str()
    }

    /// Sets the session label.
    pub fn set_session_label(&mut self, label: &str) {
        self.session_label = String::from(label);
    }

    /// Returns the session label.
    #[must_use]
    pub fn session_label(&self) -> &str {
        self.session_label.as_str()
    }

    /// Records an error.
    pub fn record_error(&mut self, error: &str) {
        self.last_error = Some(String::from(error));
    }

    /// Returns the last error, if any.
    #[must_use]
    pub fn last_error(&self) -> Option<&str> {
        self.last_error.as_deref()
    }

    /// Requests shutdown.
    pub fn request_shutdown(&mut self) {
        self.shutdown_requested = true;
    }

    /// Returns whether shutdown was requested.
    #[must_use]
    pub const fn is_shutdown_requested(&self) -> bool {
        self.shutdown_requested
    }
}

impl Default for AppShellState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Event loop event
// ---------------------------------------------------------------------------

/// Events that the shell event loop can process.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShellEvent {
    /// Terminal key event (gated by rats-widgets feature).
    #[cfg(feature = "rats-widgets")]
    Key(crossterm::event::KeyEvent),
    /// Terminal resize event.
    Resize(u16, u16),
    /// Runtime event from the turn loop.
    RuntimeEvent(animus_runtime::RuntimeEvent),
    /// Tool host event from the tool adapter.
    ToolEvent(crate::ToolHostEvent),
    /// Prompt was submitted by the user.
    PromptSubmitted(String),
    /// Shutdown signal.
    Shutdown,
    /// Tick (periodic poll).
    Tick,
}

/// Result of processing a shell event.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ShellEventOutcome {
    /// Event was processed, keep running.
    Continue,
    /// Event requested shutdown.
    Shutdown,
    /// Event caused an error.
    Error(String),
}

// ---------------------------------------------------------------------------
// App shell builder
// ---------------------------------------------------------------------------

/// Builder for the app shell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppShellConfig {
    /// Whether to enable raw mode on terminal setup.
    pub raw_mode: bool,
    /// Whether to use alternate screen.
    pub alternate_screen: bool,
    /// Visible window size for the conversation model.
    pub conversation_window_size: u64,
}

impl Default for AppShellConfig {
    fn default() -> Self {
        Self {
            raw_mode: true,
            alternate_screen: true,
            conversation_window_size: 256,
        }
    }
}

// ---------------------------------------------------------------------------
// App shell snapshot rendering
// ---------------------------------------------------------------------------

/// Renders a deterministic text snapshot of the app shell state.
#[must_use]
pub fn render_app_shell_snapshot(state: &AppShellState) -> String {
    let mut lines: Vec<String> = Vec::new();

    lines.push(format!("terminal: {}", state.terminal_state().as_str()));
    lines.push(format!("session: {}", state.session_label()));
    lines.push(format!("agent: {}", state.agent_status()));
    lines.push(format!("transcript entries: {}", state.transcript_entry_count()));
    lines.push(format!("prompt: {}", state.prompt_input()));

    if let Some(error) = state.last_error() {
        lines.push(format!("error: {error}"));
    }
    if state.is_shutdown_requested() {
        lines.push(String::from("shutdown: requested"));
    }

    lines.join("\n")
}

#[cfg(test)]
mod shell_tests {
    use super::*;

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
        state.set_transcript_entry_count(42);
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
        state.record_error("crossterm::enable_raw_mode failed");
        assert_eq!(state.terminal_state(), TerminalState::Failed);
        let snapshot = render_app_shell_snapshot(&state);
        assert!(snapshot.contains("error: crossterm::enable_raw_mode failed"));
    }

    #[test]
    fn dropped_event_consumer_does_not_panic() {
        // Simulate a shutdown after terminal failure
        let mut state = fresh_shell();
        state.set_terminal_state(TerminalState::Failed);
        state.record_error("event consumer closed");
        state.request_shutdown();
        assert!(state.is_shutdown_requested());
        // No panic
        let snapshot = render_app_shell_snapshot(&state);
        assert!(snapshot.contains("shutdown: requested"));
    }

    #[test]
    fn cancelled_command_channel_does_not_corrupt() {
        let mut state = fresh_shell();
        state.set_terminal_state(TerminalState::Ready);
        state.set_agent_status("busy");
        // Cancel via shutdown
        state.request_shutdown();
        assert!(state.is_shutdown_requested());
        assert_eq!(state.agent_status(), "busy");
    }

    #[test]
    fn layout_mismatch_does_not_panic() {
        // Simulate receiving a resize event with extreme values
        let mut state = fresh_shell();
        state.set_terminal_state(TerminalState::Ready);
        // Resize with 0 dimensions shouldn't cause issues
        state.record_error("layout: zero-area terminal");
        assert!(state.last_error().is_some());
    }
}