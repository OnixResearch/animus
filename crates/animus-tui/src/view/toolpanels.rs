// Tool panel state and snapshot renderer.
// Tool cards, FileLine, DiffView moved to toolcards.rs.

const APPROVAL_PROMPT_LABEL: &str = "-- approval needed --";

/// Composite tool panel state for the conversation render model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolPanelState {
    /// Maximum tool cards to prevent unbounded growth.
    max_cards: usize,
    /// Active tool call cards.
    active_cards: Vec<ToolCallCard>,
    /// Maximum diff views to prevent unbounded growth.
    max_diffs: usize,
    /// Current diff views.
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

/// Renders a deterministic text snapshot of the tool panel state.
/// No terminal backend required.
///
/// # Panics
///
/// Panics when `panel` has a pending approval with zero choices.
#[must_use]
pub fn render_tool_panel_snapshot(panel: &ToolPanelState) -> String {
    const MAX_RENDER_LINES: usize = 512;
    if panel.has_pending_approval() {
        assert!(!panel.approval_choices().is_empty());
    }
    let mut lines: Vec<String> = Vec::with_capacity(128);
    let mut line_count: usize = 0;

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

    for diff in panel.diff_views() {
        let collapsed = if diff.is_collapsed() { " [collapsed]" } else { "" };
        if line_count >= MAX_RENDER_LINES { break; }
        lines.push(format!("diff: {}{collapsed}", diff.source_label())); line_count += 1;
        for line in diff.lines() {
            if line_count >= MAX_RENDER_LINES { break; }
            match line {
                FileLine::Context(text) => { lines.push(format!("  {text}")); line_count += 1; }
                FileLine::Added(text) => { lines.push(format!("+ {text}")); line_count += 1; }
                FileLine::Removed(text) => { lines.push(format!("- {text}")); line_count += 1; }
            }
        }
    }

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