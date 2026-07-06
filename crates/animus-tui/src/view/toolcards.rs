// Tool call card, diff view types — split from toolpanels.rs to reduce file length.

/// Renderable tool-call card derived from tool host events.
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
    /// Whether the tool call is collapsed.
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

/// A single line in a file-edit diff view.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FileLine {
    /// Context line (unchanged).
    Context(String),
    /// Added line.
    Added(String),
    /// Removed line.
    Removed(String),
}

/// File-edit diff view rendered from hashed-edit-protocol DTOs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiffView {
    /// Editor or tool label.
    source_label: String,
    /// Bounded diff lines.
    lines: Vec<FileLine>,
    /// Whether the diff is collapsed.
    collapsed: bool,
}

impl DiffView {
    /// Builds a diff view from a source label and diff lines.
    #[must_use]
    pub fn new(source_label: &str, lines: Vec<FileLine>) -> Self {
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
    pub fn lines(&self) -> &[FileLine] {
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