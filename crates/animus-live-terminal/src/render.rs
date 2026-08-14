//! Deterministic Ratatui projection over existing Animus view reducers.

/// Renders the current terminal-client view through any Ratatui backend.
///
/// # Errors
///
/// Returns an I/O error when the selected backend cannot draw.
pub fn render_terminal_view<Backend: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<Backend>,
    view: &crate::TerminalViewState,
) -> Result<(), Backend::Error> {
    let snapshot = render_text_projection(view);
    terminal
        .draw(|frame| {
            let block = ratatui::widgets::Block::new()
                .title("Animus Live Terminal")
                .borders(ratatui::widgets::Borders::ALL);
            let paragraph = ratatui::widgets::Paragraph::new(snapshot.as_str())
                .block(block)
                .wrap(ratatui::widgets::Wrap { trim: false });
            frame.render_widget(paragraph, frame.area());
        })
        .map(|_completed| ())
}

/// Produces one deterministic view snapshot for `TestBackend` and live rendering.
#[must_use]
pub fn render_text_projection(view: &crate::TerminalViewState) -> String {
    let frontend = view.coding_session().state();
    let (width, height) = view.viewport();
    let mut lines = vec![
        format!("status: {}", view.lifecycle_status()),
        format!("viewport: {width}x{height}"),
        format!("session: {}", frontend.lifecycle_status),
        format!("messages: {}", frontend.committed_message_count),
        format!("queue: {}", frontend.queue_count),
        format!("queue-generation: {}", frontend.queue_generation.value()),
        format!("tool: {:?}", view.tools().status()),
        format!("prompt: {}", view.prompt().content()),
    ];
    if let Some((generation, _identity)) = view.queue_conflict() {
        lines.push(format!("queue-conflict-generation: {generation}"));
    }
    if let Some((child_id, status)) = view.child_status() {
        lines.push(format!("child: {child_id} [{status}]"));
    }
    lines.push(animus_tui::render_streaming_snapshot(view.conversation()));
    lines.join("\n")
}
