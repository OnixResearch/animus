/// Renderable active-tool panel state derived without terminal I/O.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolPanel {
    run_id: String,
    tool_name: String,
    progress: Option<crate::ToolProgress>,
    last_output: Option<String>,
}

impl ToolPanel {
    fn from_request(run_id: &crate::ToolRunId, request: &animus_adapters::ToolRequest) -> Self {
        Self {
            run_id: String::from(run_id.as_str()),
            tool_name: String::from(request.name()),
            progress: None,
            last_output: None,
        }
    }

    fn runtime_tool(request: &animus_adapters::ToolRequest) -> Self {
        Self {
            run_id: String::from(RUNTIME_TOOL_RUN_ID),
            tool_name: String::from(request.name()),
            progress: None,
            last_output: None,
        }
    }

    fn unknown_runtime_tool() -> Self {
        Self {
            run_id: String::from(RUNTIME_TOOL_RUN_ID),
            tool_name: String::from(UNKNOWN_TOOL_NAME),
            progress: None,
            last_output: None,
        }
    }

    /// Returns the run identity rendered for this panel.
    #[must_use]
    pub fn run_id(&self) -> &str {
        self.run_id.as_str()
    }

    /// Returns the tool name rendered for this panel.
    #[must_use]
    pub fn tool_name(&self) -> &str {
        self.tool_name.as_str()
    }

    /// Returns the latest progress rendered for this panel.
    #[must_use]
    pub const fn progress(&self) -> Option<crate::ToolProgress> {
        self.progress
    }

    /// Returns the latest bounded output rendered for this panel.
    #[must_use]
    pub fn last_output(&self) -> Option<&str> {
        self.last_output.as_deref()
    }
}

/// Renderable approval-prompt panel derived from an approval request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolApprovalPanel {
    run_id: String,
    correlation_id: String,
    prompt: String,
    choices: Vec<String>,
}

impl ToolApprovalPanel {
    fn from_request(request: &crate::ApprovalRequest) -> Self {
        Self {
            run_id: String::from(request.run_id().as_str()),
            correlation_id: String::from(request.correlation_id().as_str()),
            prompt: String::from(request.prompt()),
            choices: request
                .choices()
                .iter()
                .map(|choice| String::from(choice.as_str()))
                .collect(),
        }
    }

    /// Returns the run identity associated with this approval prompt.
    #[must_use]
    pub fn run_id(&self) -> &str {
        self.run_id.as_str()
    }

    /// Returns the correlation identity for the pending approval command.
    #[must_use]
    pub fn correlation_id(&self) -> &str {
        self.correlation_id.as_str()
    }

    /// Returns the safe approval prompt text.
    #[must_use]
    pub fn prompt(&self) -> &str {
        self.prompt.as_str()
    }

    /// Returns the finite choice labels exposed by the approval prompt.
    #[must_use]
    pub fn choices(&self) -> &[String] {
        self.choices.as_slice()
    }
}
