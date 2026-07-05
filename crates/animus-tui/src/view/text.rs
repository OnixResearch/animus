/// Renders a deterministic text snapshot of the pure tool render model.
#[must_use]
pub fn render_text_snapshot(model: &ToolRenderModel) -> String {
    const {
        assert!(!SESSION_PREFIX.is_empty());
        assert!(!TOOL_PREFIX.is_empty());
    };
    let mut lines = Vec::new();
    lines.push(format!("{SESSION_PREFIX}{}", model.status().as_str()));
    let lines = with_tool_lines(lines, model);
    let mut lines = with_approval_lines(lines, model);
    if model
        .active_tool()
        .and_then(ToolPanel::last_output)
        .is_none()
    {
        lines.push(String::from(NO_OUTPUT));
    }
    if let Some(output) = model.final_output() {
        lines.push(format!("{FINAL_PREFIX}{output}"));
    }
    if let Some(failure) = model.failure() {
        lines.push(format!(
            "{FAILURE_PREFIX}{}:{}",
            failure.class().as_str(),
            failure.reason()
        ));
    }
    lines.join(&LINE_SEPARATOR.to_string())
}

fn with_tool_lines(mut lines: Vec<String>, model: &ToolRenderModel) -> Vec<String> {
    match model.active_tool() {
        Some(panel) => {
            lines.push(format!("{TOOL_PREFIX}{}", panel.tool_name()));
            lines.push(format!("{RUN_PREFIX}{}", panel.run_id()));
            if let Some(progress) = panel.progress() {
                lines.push(format!("{PROGRESS_PREFIX}{}", progress_label(progress)));
            }
            if let Some(output) = panel.last_output() {
                lines.push(format!("{OUTPUT_PREFIX}{output}"));
            }
        }
        None => lines.push(String::from(NO_ACTIVE_TOOL)),
    }
    lines
}

fn with_approval_lines(mut lines: Vec<String>, model: &ToolRenderModel) -> Vec<String> {
    match model.approval() {
        Some(approval) => {
            lines.push(format!("{APPROVAL_PREFIX}{}", approval.prompt()));
            lines.push(format!("{CHOICES_PREFIX}{}", approval.choices().join(",")));
        }
        None => lines.push(String::from(NO_APPROVAL_PROMPT)),
    }
    lines
}

fn progress_label(progress: crate::ToolProgress) -> String {
    match progress.total_units() {
        Some(total) => format!("{}/{}", progress.completed_units(), total),
        None => format!("{}/{}", progress.completed_units(), UNKNOWN_PROGRESS_TOTAL),
    }
}

const fn terminal_status(status: animus_runtime::RuntimeTerminalKind) -> ToolSessionStatus {
    match status {
        animus_runtime::RuntimeTerminalKind::FinalResponse
        | animus_runtime::RuntimeTerminalKind::Stopped => ToolSessionStatus::Finished,
        animus_runtime::RuntimeTerminalKind::MaxTurnsExceeded
        | animus_runtime::RuntimeTerminalKind::Failed => ToolSessionStatus::Failed,
    }
}

#[derive(Clone, Copy)]
struct FailureText<'reason> {
    reason: &'reason str,
    fallback: &'static str,
}

fn safe_failure(
    class: crate::ToolHostFailureClass,
    text: FailureText<'_>,
) -> crate::ToolHostFailure {
    crate::ToolHostFailure::new(class, text.reason)
        .unwrap_or_else(|_validation_error| crate::ToolHostFailure::from_static_reason(class, text.fallback))
}
