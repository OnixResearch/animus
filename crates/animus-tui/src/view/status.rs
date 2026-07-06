// Session context status rail: session picker, model picker, context gauge,
// status bar, cost overlay, and auxiliary panels.
//
// All state is projected from accepted DTOs; the TUI does not own sessions,
// models, credentials, or persistence.

/// Default label for unnamed sessions.
const UNNAMED_SESSION: &str = "(unnamed session)";
/// Default label for unselected models.
const NO_MODEL_SELECTED: &str = "(no model selected)";
/// Maximum model label chars displayed.
const MAX_MODEL_LABEL_CHARS: usize = 64;
/// Maximum context gauge label width.
const MAX_GAUGE_LABEL_WIDTH: usize = 32;

// ---------------------------------------------------------------------------
// Status rail state
// ---------------------------------------------------------------------------

/// Composite status rail state projected from ledger and registry DTOs.
#[derive(Clone, Debug, PartialEq)]
pub struct StatusRailState {
    /// Current session label.
    session_label: String,
    /// Available session labels for the picker.
    available_sessions: Vec<String>,
    /// Current model label.
    model_label: String,
    /// Available model labels for the picker.
    available_models: Vec<String>,
    /// Context usage ratio (0.0 = empty, 1.0 = full).
    context_ratio: f64,
    /// Context budget label.
    context_label: String,
    /// Status bar text.
    status_text: String,
    /// Cost display text.
    cost_text: String,
    /// Git/branch display text.
    git_branch_text: String,
    /// Environment label.
    environment_label: String,
}

impl StatusRailState {
    /// Builds an idle status rail state.
    #[must_use]
    pub fn idle() -> Self {
        Self {
            session_label: String::from(UNNAMED_SESSION),
            available_sessions: Vec::new(),
            model_label: String::from(NO_MODEL_SELECTED),
            available_models: Vec::new(),
            context_ratio: 0.0,
            context_label: String::from("context: 0"),
            status_text: String::from("idle"),
            cost_text: String::new(),
            git_branch_text: String::new(),
            environment_label: String::new(),
        }
    }

    /// Sets the current session label.
    pub fn set_session(&mut self, label: &str) {
        self.session_label = String::from(label);
    }

    /// Sets available sessions for the picker.
    pub fn set_available_sessions(&mut self, sessions: Vec<String>) {
        self.available_sessions = sessions;
    }

    /// Sets the current model label (projected, no credentials).
    pub fn set_model(&mut self, label: &str) {
        let bounded = label
            .chars()
            .take(MAX_MODEL_LABEL_CHARS)
            .collect();
        self.model_label = bounded;
    }

    /// Sets available model labels (projected, no credentials).
    pub fn set_available_models(&mut self, models: Vec<String>) {
        self.available_models = models
            .into_iter()
            .map(|m| m.chars().take(MAX_MODEL_LABEL_CHARS).collect())
            .collect();
    }

    /// Sets the context gauge from a ratio and a label.
    ///
    /// `ratio` is clamped to [0.0, 1.0].
    pub fn set_context(&mut self, ratio: f64, label: &str) {
        self.context_ratio = ratio.clamp(0.0, 1.0);
        let bounded = label
            .chars()
            .take(MAX_GAUGE_LABEL_WIDTH)
            .collect();
        self.context_label = bounded;
    }

    /// Sets the status bar text.
    pub fn set_status(&mut self, status: &str) {
        self.status_text = String::from(status);
    }

    /// Sets the cost overlay text.
    pub fn set_cost(&mut self, cost: &str) {
        self.cost_text = String::from(cost);
    }

    /// Sets the git/branch text.
    pub fn set_git_branch(&mut self, text: &str) {
        self.git_branch_text = String::from(text);
    }

    /// Sets the environment label.
    pub fn set_environment(&mut self, label: &str) {
        self.environment_label = String::from(label);
    }

    /// Returns the current session label.
    #[must_use]
    pub fn session_label(&self) -> &str {
        self.session_label.as_str()
    }

    /// Returns the available session labels.
    #[must_use]
    pub fn available_sessions(&self) -> &[String] {
        self.available_sessions.as_slice()
    }

    /// Returns the current model label.
    #[must_use]
    pub fn model_label(&self) -> &str {
        self.model_label.as_str()
    }

    /// Returns the available model labels.
    #[must_use]
    pub fn available_models(&self) -> &[String] {
        self.available_models.as_slice()
    }

    /// Returns the context usage ratio.
    #[must_use]
    pub const fn context_ratio(&self) -> f64 {
        self.context_ratio
    }

    /// Returns the context label.
    #[must_use]
    pub fn context_label(&self) -> &str {
        self.context_label.as_str()
    }

    /// Returns the status text.
    #[must_use]
    pub fn status_text(&self) -> &str {
        self.status_text.as_str()
    }

    /// Returns the cost text.
    #[must_use]
    pub fn cost_text(&self) -> &str {
        self.cost_text.as_str()
    }

    /// Returns the git/branch text.
    #[must_use]
    pub fn git_branch_text(&self) -> &str {
        self.git_branch_text.as_str()
    }

    /// Returns the environment label.
    #[must_use]
    pub fn environment_label(&self) -> &str {
        self.environment_label.as_str()
    }
}

// ---------------------------------------------------------------------------
// Status rail snapshot rendering
// ---------------------------------------------------------------------------

/// Renders a deterministic text snapshot of the status rail.
///
/// # Panics
///
/// Panics when `rail.session_label()` is empty.
#[must_use]
pub fn render_status_rail_snapshot(rail: &StatusRailState) -> String {
    assert!(!rail.session_label().is_empty());
    let mut lines = Vec::with_capacity(16);
    lines.push(format!("session: {}", rail.session_label()));
    lines.push(format!("model: {}", rail.model_label()));
    lines.push(format!("context: {} ({:.1}%)", rail.context_label(), rail.context_ratio() * 100.0));
    lines.push(format!("status: {}", rail.status_text()));
    if !rail.cost_text().is_empty() {
        lines.push(format!("cost: {}", rail.cost_text()));
    }
    if !rail.git_branch_text().is_empty() {
        lines.push(format!("git: {}", rail.git_branch_text()));
    }
    if !rail.environment_label().is_empty() {
        lines.push(format!("env: {}", rail.environment_label()));
    }
    if rail.available_sessions().len() > 1 {
        lines.push(format!("sessions: {}", rail.available_sessions().join(", ")));
    }
    if !rail.available_models().is_empty() {
        lines.push(format!("models: {}", rail.available_models().join(", ")));
    }
    lines.join("\n")
}

#[cfg(test)]
#[cfg(test)]
fn idle_rail() -> StatusRailState {
    StatusRailState::idle()
}

#[cfg(test)]
fn set_long_text(rail: &mut StatusRailState, text: &str) {
    rail.set_model(text);
}

mod status_rail_tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn idle_rail_has_sensible_defaults() {
        let rail = idle_rail();
        assert_eq!(rail.session_label(), UNNAMED_SESSION);
        assert_eq!(rail.model_label(), NO_MODEL_SELECTED);
        assert_eq!(rail.status_text(), "idle");
        assert!((rail.context_ratio() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn session_label_updates() {
        let mut rail = idle_rail();
        rail.set_session("workspace-session-1");
        assert_eq!(rail.session_label(), "workspace-session-1");
    }

    #[test]
    fn available_sessions_listed() {
        let mut rail = idle_rail();
        rail.set_available_sessions(vec![
            String::from("session-a"),
            String::from("session-b"),
        ]);
        assert_eq!(rail.available_sessions().len(), 2);
    }

    #[test]
    fn model_label_bounded() {
        let mut rail = idle_rail();
        let long = "x".repeat(200);
        set_long_text(&mut rail, &long);
        assert!(rail.model_label().len() <= MAX_MODEL_LABEL_CHARS);
    }

    #[test]
    fn context_ratio_clamped() {
        let mut rail = idle_rail();
        rail.set_context(1.5, "over budget");
        assert!((rail.context_ratio() - 1.0).abs() < f64::EPSILON);
        rail.set_context(-0.5, "negative");
        assert!((rail.context_ratio() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn status_text_updates() {
        let mut rail = idle_rail();
        rail.set_status("streaming");
        assert_eq!(rail.status_text(), "streaming");
    }

    #[test]
    fn cost_and_git_are_optional() {
        let rail = idle_rail();
        assert!(rail.cost_text().is_empty());
        assert!(rail.git_branch_text().is_empty());
    }

    #[test]
    fn idle_rail_snapshot() {
        let rail = idle_rail();
        let snapshot = render_status_rail_snapshot(&rail);
        assert!(snapshot.contains("session: (unnamed session)"));
        assert!(snapshot.contains("status: idle"));
    }

    #[test]
    fn populated_rail_snapshot() {
        let mut rail = idle_rail();
        rail.set_session("work");
        rail.set_model("gpt-5.5");
        rail.set_context(0.75, "high");
        rail.set_status("streaming");
        rail.set_cost("~$0.02/turn");
        rail.set_git_branch("main");
        let snapshot = render_status_rail_snapshot(&rail);
        assert!(snapshot.contains("session: work"));
        assert!(snapshot.contains("model: gpt-5.5"));
        assert!(snapshot.contains("status: streaming"));
        assert!(snapshot.contains("git: main"));
    }

    #[test]
    fn missing_session_does_not_panic() {
        let mut rail = idle_rail();
        rail.set_session("");
        assert_eq!(rail.session_label(), "");
    }

    #[test]
    fn oversize_context_does_not_panic() {
        let mut rail = idle_rail();
        let long = "x".repeat(200);
        rail.set_context(0.5, &long);
        assert!(rail.context_label().len() <= MAX_GAUGE_LABEL_WIDTH);
    }

    #[test]
    fn empty_available_sessions() {
        let rail = idle_rail();
        assert!(rail.available_sessions().is_empty());
    }

    #[test]
    fn model_label_does_not_grow_unbounded() {
        let mut rail = idle_rail();
        let long = "x".repeat(200);
        set_long_text(&mut rail, &long);
        assert!(rail.model_label().len() <= MAX_MODEL_LABEL_CHARS);
    }
}