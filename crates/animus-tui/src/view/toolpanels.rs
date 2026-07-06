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
///
/// # Panics
///
/// Panics when `panel` has a pending approval with zero choices.
#[must_use]
pub fn render_tool_panel_snapshot(panel: &ToolPanelState) -> String {
    const MAX_RENDER_LINES: usize = 512;
    assert!(!panel.has_pending_approval() || !panel.approval_choices().is_empty());
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
// Tests moved to toolpaneltest.rs to reduce file length.