// Tool-call panels for the conversation render model: tool cards, diff views,
// progress rendering, and approval prompts.
//
// This module composes on the existing tool host bridge, hashed-edit-protocol
// DTOs, and tool-gateway-delivery receipts. No new reusable-crate contracts
// are added.

/// Label for the approval prompt block.
const APPROVAL_PROMPT_LABEL: &str = "-- approval needed --";


// ---------------------------------------------------------------------------
// Tool call card
// ---------------------------------------------------------------------------

/// Renderable tool-call card derived from tool host events.
///
/// Cards are collapsible and carry bounded output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolCallCard {
    /// Tool name.
    tool_name: String,
    /// Tool input summary.
    input_summary: String,
    /// Bounded output text, if any.
    output: Option<String>,
    /// Progress state if the tool is long-running.
    progress: Option<ToolCardProgress>,
    /// Whether the tool call is collapsed in the display.
    collapsed: bool,
}

/// Progress state for a long-running tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolCardProgress {
    /// Units completed.
    pub completed: u32,
    /// Total units to complete, if known.
    pub total: Option<u32>,
}

impl ToolCallCard {
    /// Builds a new tool-call card from a tool request.
    #[must_use]
    pub fn from_request(request: &animus_adapters::ToolRequest) -> Self {
        let input = request.input();
        Self {
            tool_name: String::from(request.name()),
            input_summary: input.chars().take(120).collect(),
            output: None,
            progress: None,
            collapsed: false,
        }
    }

    /// Updates the card with new output.
    pub fn set_output(&mut self, output: &str) {
        self.output = Some(String::from(output));
    }

    /// Updates the card with progress information.
    pub fn set_progress(&mut self, completed: u32, total: Option<u32>) {
        self.progress = Some(ToolCardProgress { completed, total });
    }

    /// Toggles collapsed state.
    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }

    /// Returns the tool name.
    #[must_use]
    pub fn tool_name(&self) -> &str {
        self.tool_name.as_str()
    }

    /// Returns the input summary.
    #[must_use]
    pub fn input_summary(&self) -> &str {
        self.input_summary.as_str()
    }

    /// Returns the bounded output, if any.
    #[must_use]
    pub fn output(&self) -> Option<&str> {
        self.output.as_deref()
    }

    /// Returns the progress, if any.
    #[must_use]
    pub const fn progress(&self) -> Option<ToolCardProgress> {
        self.progress
    }

    /// Returns whether the card is collapsed.
    #[must_use]
    pub const fn is_collapsed(&self) -> bool {
        self.collapsed
    }
}

// ---------------------------------------------------------------------------
// Diff view
// ---------------------------------------------------------------------------

/// A single line in a file-edit diff view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiffLine {
    /// Context line (unchanged).
    Context(String),
    /// Added line.
    Added(String),
    /// Removed line.
    Removed(String),
}

/// File-edit diff view rendered from hashed-edit-protocol DTOs.
///
/// The TUI never reconstructs edits from raw file reads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffView {
    /// Editor or tool label.
    source_label: String,
    /// Bounded diff lines.
    lines: Vec<DiffLine>,
    /// Whether the diff is collapsed.
    collapsed: bool,
}

impl DiffView {
    /// Builds a diff view from a source label and diff lines.
    #[must_use]
    pub fn new(source_label: &str, lines: Vec<DiffLine>) -> Self {
        Self {
            source_label: String::from(source_label),
            lines,
            collapsed: false,
        }
    }

    /// Returns the source label.
    #[must_use]
    pub fn source_label(&self) -> &str {
        self.source_label.as_str()
    }

    /// Returns the diff lines.
    #[must_use]
    pub fn lines(&self) -> &[DiffLine] {
        self.lines.as_slice()
    }

    /// Toggles collapsed state.
    pub fn toggle_collapsed(&mut self) {
        self.collapsed = !self.collapsed;
    }

    /// Returns whether the diff is collapsed.
    #[must_use]
    pub const fn is_collapsed(&self) -> bool {
        self.collapsed
    }
}

// ---------------------------------------------------------------------------
// Tool panel state
// ---------------------------------------------------------------------------

/// Composite tool panel state for the conversation render model.
///
/// Manages active tool cards, pending approval prompts, and diff views.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolPanelState {
    /// Active tool call cards.
    /// Maximum tool cards to prevent unbounded growth.
    max_cards: usize,
    active_cards: Vec<ToolCallCard>,
    /// Maximum diff views to prevent unbounded growth.
    max_diffs: usize,
    diff_views: Vec<DiffView>,
    /// Approval prompt text, if any.
    approval_prompt: Option<String>,
    /// Available approval choices.
    approval_choices: Vec<String>,
}

impl ToolPanelState {
    /// Builds an empty tool panel state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            max_cards: 64,
            active_cards: Vec::new(),
            max_diffs: 32,
            diff_views: Vec::new(),
            approval_prompt: None,
            approval_choices: Vec::new(),
        }
    }

    /// Adds or updates a tool-call card.
    pub fn add_card(&mut self, card: ToolCallCard) {
        // Replace existing card for the same tool, or push (bounded)
        if let Some(existing) = self
            .active_cards
            .iter_mut()
            .find(|c| c.tool_name() == card.tool_name())
        {
            *existing = card;
        } else if self.active_cards.len() < self.max_cards {
            self.active_cards.push(card);
        }
    }

    /// Clears all tool state.
    pub fn clear(&mut self) {
        self.active_cards.clear();
        self.diff_views.clear();
        self.approval_prompt = None;
        self.approval_choices.clear();
    }

    /// Returns the active tool cards.
    #[must_use]
    pub fn active_cards(&self) -> &[ToolCallCard] {
        self.active_cards.as_slice()
    }

    /// Returns the current diff views.
    #[must_use]
    pub fn diff_views(&self) -> &[DiffView] {
        self.diff_views.as_slice()
    }

    /// Sets the approval prompt.
    pub fn set_approval(&mut self, prompt: &str, choices: &[String]) {
        self.approval_prompt = Some(String::from(prompt));
        self.approval_choices = choices.to_vec();
    }

    /// Returns the approval prompt text, if any.
    #[must_use]
    pub fn approval_prompt(&self) -> Option<&str> {
        self.approval_prompt.as_deref()
    }

    /// Returns the approval choices.
    #[must_use]
    pub fn approval_choices(&self) -> &[String] {
        self.approval_choices.as_slice()
    }

    /// Returns whether an approval prompt is active.
    #[must_use]
    pub fn has_pending_approval(&self) -> bool {
        self.approval_prompt.is_some()
    }
}

impl Default for ToolPanelState {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Tool panel snapshot rendering
// ---------------------------------------------------------------------------

/// Renders a deterministic text snapshot of the tool panel state.
///
/// No terminal backend required.
#[must_use]
pub fn render_tool_panel_snapshot(panel: &ToolPanelState) -> String {
    const MAX_RENDER_LINES: usize = 512;
    let mut lines: Vec<String> = Vec::with_capacity(128);
    let mut line_count: usize = 0;

    // Tool cards
    for card in panel.active_cards() {
        let collapsed = if card.is_collapsed() { " [collapsed]" } else { "" };
        if line_count >= MAX_RENDER_LINES { break; }
        lines.push(format!("tool: {}{collapsed}", card.tool_name())); line_count += 1;
        if line_count >= MAX_RENDER_LINES { break; }
        lines.push(format!("  input: {}", card.input_summary())); line_count += 1;
        if let Some(progress) = card.progress() {
            if line_count >= MAX_RENDER_LINES { break; }
            if let Some(total) = progress.total {
                lines.push(format!("  progress: {}/{}", progress.completed, total)); line_count += 1;
            } else {
                lines.push(format!("  progress: {}/?", progress.completed)); line_count += 1;
            }
        }
        if let Some(output) = card.output() {
            if line_count >= MAX_RENDER_LINES { break; }
            lines.push(format!("  output: {output}")); line_count += 1;
        }
    }

    // Diff views
    for diff in panel.diff_views() {
        let collapsed = if diff.is_collapsed() { " [collapsed]" } else { "" };
        if line_count >= MAX_RENDER_LINES { break; }
        lines.push(format!("diff: {}{collapsed}", diff.source_label())); line_count += 1;
        for line in diff.lines() {
            if line_count >= MAX_RENDER_LINES { break; }
            match line {
                DiffLine::Context(text) => { lines.push(format!("  {text}")); line_count += 1; }
                DiffLine::Added(text) => { lines.push(format!("+ {text}")); line_count += 1; }
                DiffLine::Removed(text) => { lines.push(format!("- {text}")); line_count += 1; }
            }
        }
    }

    // Approval
    if let Some(prompt) = panel.approval_prompt() {
        if line_count < MAX_RENDER_LINES {
            lines.push(String::from(APPROVAL_PROMPT_LABEL)); line_count += 1;
        }
        if line_count < MAX_RENDER_LINES {
            lines.push(format!("  prompt: {prompt}")); line_count += 1;
        }
        for choice in panel.approval_choices() {
            if line_count >= MAX_RENDER_LINES { break; }
            lines.push(format!("  choice: {choice}")); line_count += 1;
        }
    }

    if lines.is_empty() {
        lines.push(String::from("(no active tools)"));
    }

    lines.join("\n")
}

#[cfg(test)]
mod tool_panel_tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Tool call card tests
    // -----------------------------------------------------------------------

    #[test]
    fn card_from_request_has_tool_name_and_input() {
        let request = animus_adapters::ToolRequest::new("search", "query: hello")
            .expect("request should validate");
        let card = ToolCallCard::from_request(&request);
        assert_eq!(card.tool_name(), "search");
        assert!(card.input_summary().contains("query: hello"));
        assert!(card.output().is_none());
    }

    #[test]
    fn card_output_updates() {
        let request = animus_adapters::ToolRequest::new("read", "path: Cargo.toml")
            .expect("request should validate");
        let mut card = ToolCallCard::from_request(&request);
        card.set_output("[package]\nname = \"animus\"");
        assert_eq!(card.output(), Some("[package]\nname = \"animus\""));
    }

    #[test]
    fn card_progress_updates() {
        let request = animus_adapters::ToolRequest::new("build", ".")
            .expect("request should validate");
        let mut card = ToolCallCard::from_request(&request);
        card.set_progress(5, Some(10));
        assert_eq!(card.progress(), Some(ToolCardProgress { completed: 5, total: Some(10) }));
    }

    #[test]
    fn card_toggle_collapsed() {
        let request = animus_adapters::ToolRequest::new("test", "all")
            .expect("request should validate");
        let mut card = ToolCallCard::from_request(&request);
        assert!(!card.is_collapsed());
        card.toggle_collapsed();
        assert!(card.is_collapsed());
        card.toggle_collapsed();
        assert!(!card.is_collapsed());
    }

    #[test]
    fn long_input_is_truncated() {
        let long_input = "a".repeat(500);
        let request = animus_adapters::ToolRequest::new("tool", &long_input)
            .expect("request should validate");
        let card = ToolCallCard::from_request(&request);
        assert!(card.input_summary().len() <= 120);
    }

    // -----------------------------------------------------------------------
    // Diff view tests
    // -----------------------------------------------------------------------

    #[test]
    fn diff_view_renders_lines() {
        let diff = DiffView::new(
            "editor",
            vec![
                DiffLine::Context(String::from("line 1")),
                DiffLine::Added(String::from("new line")),
                DiffLine::Removed(String::from("old line")),
            ],
        );
        assert_eq!(diff.source_label(), "editor");
        assert_eq!(diff.lines().len(), 3);
    }

    #[test]
    fn diff_view_toggle_collapsed() {
        let mut diff = DiffView::new("editor", vec![]);
        assert!(!diff.is_collapsed());
        diff.toggle_collapsed();
        assert!(diff.is_collapsed());
    }

    // -----------------------------------------------------------------------
    // Tool panel state tests
    // -----------------------------------------------------------------------

    #[test]
    fn empty_panel_shows_no_tools() {
        let panel = ToolPanelState::new();
        assert!(panel.active_cards().is_empty());
        assert!(!panel.has_pending_approval());
    }

    #[test]
    fn add_card_to_panel() {
        let mut panel = ToolPanelState::new();
        let request = animus_adapters::ToolRequest::new("build", ".")
            .expect("request should validate");
        panel.add_card(ToolCallCard::from_request(&request));
        assert_eq!(panel.active_cards().len(), 1);
    }

    #[test]
    fn replacing_card_updates_in_place() {
        let mut panel = ToolPanelState::new();
        let req1 = animus_adapters::ToolRequest::new("tool", "first")
            .expect("request should validate");
        panel.add_card(ToolCallCard::from_request(&req1));
        let req2 = animus_adapters::ToolRequest::new("tool", "second")
            .expect("request should validate");
        panel.add_card(ToolCallCard::from_request(&req2));
        assert_eq!(panel.active_cards().len(), 1);
        assert_eq!(panel.active_cards()[0].input_summary(), "second");
    }

    #[test]
    fn approval_prompt_tracking() {
        let mut panel = ToolPanelState::new();
        assert!(!panel.has_pending_approval());
        panel.set_approval("Allow write to /tmp?", &[String::from("approve"), String::from("deny")]);
        assert!(panel.has_pending_approval());
        assert_eq!(panel.approval_choices().len(), 2);
    }

    #[test]
    fn clear_resets_panel() {
        let mut panel = ToolPanelState::new();
        let request = animus_adapters::ToolRequest::new("tool", "test")
            .expect("request should validate");
        panel.add_card(ToolCallCard::from_request(&request));
        panel.set_approval("Proceed?", &[String::from("yes")]);
        panel.clear();
        assert!(panel.active_cards().is_empty());
        assert!(!panel.has_pending_approval());
    }

    // -----------------------------------------------------------------------
    // Snapshot tests
    // -----------------------------------------------------------------------

    #[test]
    fn empty_panel_snapshot() {
        let panel = ToolPanelState::new();
        let snapshot = render_tool_panel_snapshot(&panel);
        assert_eq!(snapshot, "(no active tools)");
    }

    #[test]
    fn panel_snapshot_shows_tools_and_approval() {
        let mut panel = ToolPanelState::new();
        let request = animus_adapters::ToolRequest::new("search", "query: hello")
            .expect("request should validate");
        panel.add_card(ToolCallCard::from_request(&request));
        panel.set_approval("Allow?", &[String::from("approve"), String::from("deny")]);
        let snapshot = render_tool_panel_snapshot(&panel);
        assert!(snapshot.contains("tool: search"));
        assert!(snapshot.contains("-- approval needed --"));
        assert!(snapshot.contains("choice: approve"));
    }

    // -----------------------------------------------------------------------
    // Negative tests
    // -----------------------------------------------------------------------

    #[test]
    fn denied_approval_clears_prompt() {
        let mut panel = ToolPanelState::new();
        panel.set_approval("Allow?", &[String::from("approve"), String::from("deny")]);
        assert!(panel.has_pending_approval());
        panel.clear();
        assert!(!panel.has_pending_approval());
    }

    #[test]
    fn mismatched_correlation_does_not_panic() {
        let mut panel = ToolPanelState::new();
        // Adding a card with different name doesn't replace unrelated cards
        let req1 = animus_adapters::ToolRequest::new("tool_a", "first")
            .expect("request should validate");
        let req2 = animus_adapters::ToolRequest::new("tool_b", "second")
            .expect("request should validate");
        panel.add_card(ToolCallCard::from_request(&req1));
        panel.add_card(ToolCallCard::from_request(&req2));
        assert_eq!(panel.active_cards().len(), 2);
    }

    #[test]
    fn oversized_output_does_not_panic() {
        let mut panel = ToolPanelState::new();
        let request = animus_adapters::ToolRequest::new("tool", "input")
            .expect("request should validate");
        let mut card = ToolCallCard::from_request(&request);
        let huge_output = "x".repeat(100_000);
        card.set_output(&huge_output);
        panel.add_card(card);
        assert!(panel.active_cards()[0].output().is_some());
    }
}