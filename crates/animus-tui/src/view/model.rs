/// Pure render model for a TUI-compatible tool host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolRenderModel {
    status: ToolSessionStatus,
    active_tool: Option<ToolPanel>,
    approval: Option<ToolApprovalPanel>,
    final_output: Option<String>,
    failure: Option<crate::ToolHostFailure>,
}

impl ToolRenderModel {
    /// Builds an idle render model.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            status: ToolSessionStatus::Idle,
            active_tool: None,
            approval: None,
            final_output: None,
            failure: None,
        }
    }

    /// Applies one UI-neutral tool-host event to the render model.
    pub fn apply_tool_event(&mut self, event: &crate::ToolHostEvent) {
        match event {
            crate::ToolHostEvent::Started { run_id, request } => self.start_tool(run_id, request),
            crate::ToolHostEvent::Progress { progress, .. } => self.update_progress(*progress),
            crate::ToolHostEvent::Output { output, .. } => self.update_output(output.visible_text()),
            crate::ToolHostEvent::ApprovalRequested { request } => self.wait_for_approval(request),
            crate::ToolHostEvent::CommandObserved { command, .. } => self.observe_command(command),
            crate::ToolHostEvent::Cancelled { reason, .. } => self.cancel(reason),
            crate::ToolHostEvent::Finished { response, .. } => self.finish(response.output()),
            crate::ToolHostEvent::Failed { failure, .. } => self.fail(failure),
        }
    }

    /// Applies one runtime event to the render model without terminal I/O.
    pub fn apply_runtime_event(&mut self, event: &animus_runtime::RuntimeEvent) {
        match event {
            animus_runtime::RuntimeEvent::ToolRequested { request } => {
                self.status = ToolSessionStatus::Running;
                self.active_tool = Some(ToolPanel::runtime_tool(request));
                self.failure = None;
            }
            animus_runtime::RuntimeEvent::ToolResponded { response } => self.finish(response.output()),
            animus_runtime::RuntimeEvent::ToolDenied { reason, .. } => self.deny_tool(reason),
            animus_runtime::RuntimeEvent::CompletionCandidateRequested { .. } => {
                self.status = ToolSessionStatus::WaitingForCompletionEvidence;
                self.final_output = None;
            }
            animus_runtime::RuntimeEvent::Cancelled { reason } => self.cancel(reason),
            animus_runtime::RuntimeEvent::AdapterError { component, message }
                if *component == animus_runtime::AdapterComponent::Tool =>
            {
                self.fail_tool_adapter(message);
            }
            animus_runtime::RuntimeEvent::Shutdown { status } => {
                self.status = terminal_status(*status);
            }
            _other => {}
        }
    }

    /// Returns the current session status.
    #[must_use]
    pub const fn status(&self) -> ToolSessionStatus {
        self.status
    }

    /// Returns the active-tool panel when one is visible.
    #[must_use]
    pub const fn active_tool(&self) -> Option<&ToolPanel> {
        self.active_tool.as_ref()
    }

    /// Returns the approval prompt when one is pending.
    #[must_use]
    pub const fn approval(&self) -> Option<&ToolApprovalPanel> {
        self.approval.as_ref()
    }

    /// Returns the final bounded output when a tool completed.
    #[must_use]
    pub fn final_output(&self) -> Option<&str> {
        self.final_output.as_deref()
    }

    /// Returns the latest typed failure when one is visible.
    #[must_use]
    pub const fn failure(&self) -> Option<&crate::ToolHostFailure> {
        self.failure.as_ref()
    }

    fn start_tool(&mut self, run_id: &crate::ToolRunId, request: &animus_adapters::ToolRequest) {
        self.status = ToolSessionStatus::Running;
        self.active_tool = Some(ToolPanel::from_request(run_id, request));
        self.approval = None;
        self.final_output = None;
        self.failure = None;
    }

    fn update_progress(&mut self, progress: crate::ToolProgress) {
        if let Some(panel) = &mut self.active_tool {
            panel.progress = Some(progress);
        }
    }

    fn update_output(&mut self, visible_text: &str) {
        if let Some(panel) = &mut self.active_tool {
            panel.last_output = Some(String::from(visible_text));
        }
    }

    fn wait_for_approval(&mut self, request: &crate::ApprovalRequest) {
        self.status = ToolSessionStatus::WaitingForApproval;
        self.approval = Some(ToolApprovalPanel::from_request(request));
    }

    fn observe_command(&mut self, command: &crate::ToolHostCommand) {
        match command.choice() {
            crate::ToolCommandChoice::Approve | crate::ToolCommandChoice::Continue => {
                self.status = ToolSessionStatus::Running;
                self.approval = None;
            }
            crate::ToolCommandChoice::Deny => self.status = ToolSessionStatus::Failed,
            crate::ToolCommandChoice::Cancel => self.status = ToolSessionStatus::Cancelled,
        }
    }

    fn cancel(&mut self, reason: &str) {
        self.status = ToolSessionStatus::Cancelled;
        self.approval = None;
        self.failure = Some(safe_failure(
            crate::ToolHostFailureClass::Cancelled,
            FailureText {
                reason,
                fallback: CANCELLED_REASON,
            },
        ));
    }

    fn finish(&mut self, output: &str) {
        self.status = ToolSessionStatus::Finished;
        self.approval = None;
        self.final_output = Some(String::from(output));
        self.failure = None;
    }

    fn fail(&mut self, failure: &crate::ToolHostFailure) {
        self.status = ToolSessionStatus::Failed;
        self.approval = None;
        self.final_output = None;
        self.failure = Some(failure.clone());
    }

    fn deny_tool(&mut self, reason: &str) {
        self.status = ToolSessionStatus::Failed;
        self.final_output = None;
        self.failure = Some(safe_failure(
            crate::ToolHostFailureClass::Denied,
            FailureText {
                reason,
                fallback: TOOL_DENIED_REASON,
            },
        ));
    }

    fn fail_tool_adapter(&mut self, message: &str) {
        self.status = ToolSessionStatus::Failed;
        self.active_tool
            .get_or_insert_with(ToolPanel::unknown_runtime_tool);
        self.failure = Some(safe_failure(
            crate::ToolHostFailureClass::Adapter,
            FailureText {
                reason: message,
                fallback: TOOL_ADAPTER_FAILED_REASON,
            },
        ));
    }
}

impl Default for ToolRenderModel {
    fn default() -> Self {
        Self::new()
    }
}
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
