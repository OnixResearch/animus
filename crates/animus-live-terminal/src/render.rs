//! Deterministic Ratatui projection over existing Animus view reducers.

use ratatui::layout::{Constraint, Layout};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};

/// Glyph shown before the editable prompt content.
const PROMPT_GLYPH: &str = "❯ ";
/// Block cursor drawn after the prompt content.
const CURSOR_GLYPH: &str = "▌";
/// Horizontal separator fill for the prompt divider row.
const SEPARATOR_FILL: &str = "─";
/// Streaming marker shown while provider text is still arriving.
const PENDING_GLYPH: &str = "… ";

/// Renders the current terminal-client view through any Ratatui backend.
///
/// The projection mirrors a coding-agent shell: one status rail, the bounded
/// conversation transcript scrolled to its tail, warnings, and a pinned prompt
/// footer.
///
/// # Errors
///
/// Returns an I/O error when the selected backend cannot draw.
pub fn render_terminal_view<Backend: ratatui::backend::Backend>(
    terminal: &mut ratatui::Terminal<Backend>,
    view: &crate::TerminalViewState,
) -> Result<(), Backend::Error> {
    terminal
        .draw(|frame| {
            let rows = Layout::vertical([
                Constraint::Length(1),
                Constraint::Min(3),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(frame.area());
            frame.render_widget(status_rail(view), rows[0]);
            frame.render_widget(transcript_paragraph(view, rows[1]), rows[1]);
            frame.render_widget(separator(rows[2].width), rows[2]);
            frame.render_widget(prompt_footer(view), rows[3]);
        })
        .map(|_completed| ())
}

/// Produces one deterministic plain-text view snapshot for `TestBackend`.
///
/// The snapshot keeps the historical flat shape so downstream assertions stay
/// stable; the styled projection renders the same state.
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
    if let Some(inspection) = view.state_inspection() {
        lines.extend([
            format!("worker-generation: {}", inspection.worker_generation),
            format!("restore-status: {}", inspection.restore_status.as_str()),
            format!("session-head: {}", inspection.session_head_identity),
            format!("kernel-head: {}", inspection.kernel_head_identity),
            format!(
                "inspected-queue-generation: {}",
                inspection.queue_generation
            ),
            format!("inspected-queue-head: {}", inspection.queue_identity),
            format!("committed-messages: {}", inspection.committed_message_count),
            format!("queued-items: {}", inspection.queued_item_count),
            format!("active-children: {}", inspection.active_child_count),
        ]);
    }
    lines.push(animus_tui::render_streaming_snapshot(view.conversation()));
    lines.join("\n")
}

// ---------------------------------------------------------------------------
// Status rail
// ---------------------------------------------------------------------------

fn dim_label(text: &str) -> Span<'static> {
    Span::styled(format!(" {text} "), Style::new().fg(Color::DarkGray))
}

fn plain_value(text: &str) -> Span<'static> {
    Span::raw(text.to_owned())
}

fn tool_status_span(label: &'static str) -> Span<'static> {
    let style = match label {
        "idle" => Style::new().fg(Color::DarkGray),
        "running" | "waiting_for_approval" | "waiting_for_completion_evidence" => {
            Style::new().fg(Color::Yellow)
        }
        "finished" => Style::new().fg(Color::Green),
        "cancelled" => Style::new().fg(Color::Blue),
        "failed" => Style::new().fg(Color::Red),
        _unknown => Style::new(),
    };
    Span::styled(label, style)
}

fn status_rail(view: &crate::TerminalViewState) -> Line<'static> {
    let frontend = view.coding_session().state();
    let mut spans = vec![
        Span::styled("◆", Style::new().fg(Color::Cyan)),
        dim_label("animus"),
        plain_value(view.lifecycle_status()),
        dim_label("session"),
        plain_value(&frontend.lifecycle_status),
        dim_label("messages"),
        plain_value(&frontend.committed_message_count.to_string()),
        dim_label("queue"),
        plain_value(&frontend.queue_count.to_string()),
        dim_label("tool"),
        tool_status_span(view.tools().status().as_str()),
    ];
    if let Some((generation, _identity)) = view.queue_conflict() {
        spans.push(Span::styled(
            format!(" queue-conflict:{generation}"),
            Style::new().fg(Color::Yellow),
        ));
    }
    if let Some((child_id, status)) = view.child_status() {
        spans.push(Span::styled(
            format!(" child:{child_id}[{status}]"),
            Style::new().fg(Color::Blue),
        ));
    }
    Line::from(spans)
}

// ---------------------------------------------------------------------------
// Transcript
// ---------------------------------------------------------------------------

fn labelled_lines(prefix: &str, color: Color, content: &str) -> Vec<Line<'static>> {
    let indent = " ".repeat(prefix.chars().count());
    let body_lines: Vec<&str> = content.split('\n').collect();
    let mut lines = Vec::with_capacity(body_lines.len());
    for (index, text) in body_lines.iter().enumerate() {
        if index == 0 {
            lines.push(Line::from(vec![
                Span::styled(prefix.to_owned(), Style::new().fg(color)),
                Span::raw((*text).to_owned()),
            ]));
        } else {
            lines.push(Line::from(vec![
                Span::raw(indent.clone()),
                Span::raw((*text).to_owned()),
            ]));
        }
    }
    if lines.is_empty() {
        lines.push(Line::from(vec![Span::styled(
            prefix.to_owned(),
            Style::new().fg(color),
        )]));
    }
    lines
}

fn entry_lines(entry: &animus_tui::TranscriptEntry) -> Vec<Line<'static>> {
    match entry {
        animus_tui::TranscriptEntry::UserMessage { content, .. } => {
            labelled_lines(" you › ", Color::Cyan, content)
        }
        animus_tui::TranscriptEntry::AssistantText { content, .. } => {
            labelled_lines(" animus › ", Color::Green, content)
        }
        animus_tui::TranscriptEntry::Thinking { content, .. } => {
            labelled_lines(" · thinking › ", Color::DarkGray, content)
        }
        animus_tui::TranscriptEntry::ToolUse { tool_name, input } => {
            labelled_lines(&format!(" ⚙ {tool_name} › "), Color::Yellow, input)
        }
        animus_tui::TranscriptEntry::ToolResult {
            tool_name, output, ..
        } => labelled_lines(&format!(" ✔ {tool_name} › "), Color::Green, output),
        animus_tui::TranscriptEntry::Image {
            description,
            media_type,
        } => labelled_lines(
            &format!(" ▣ image [{media_type}] › "),
            Color::Magenta,
            description,
        ),
        animus_tui::TranscriptEntry::Error { reason } => {
            labelled_lines(" ✗ error › ", Color::Red, reason)
        }
        animus_tui::TranscriptEntry::SystemMessage { content } => {
            labelled_lines(" ● system › ", Color::Blue, content)
        }
        animus_tui::TranscriptEntry::Unknown { .. } => Vec::new(),
    }
}

fn warning_lines(view: &crate::TerminalViewState) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    if let Some((generation, _identity)) = view.queue_conflict() {
        lines.push(Line::from(Span::styled(
            format!(" ! queue conflict on generation {generation}; resolve before queue edits"),
            Style::new().fg(Color::Yellow),
        )));
    }
    if let Some((child_id, status)) = view.child_status() {
        lines.push(Line::from(Span::styled(
            format!(" • child {child_id} is {status}"),
            Style::new().fg(Color::Blue),
        )));
    }
    if let Some(inspection) = view.state_inspection() {
        lines.push(Line::from(Span::styled(
            format!(
                " worker-generation {} restored {}; committed {} queued {} children {}",
                inspection.worker_generation,
                inspection.restore_status.as_str(),
                inspection.committed_message_count,
                inspection.queued_item_count,
                inspection.active_child_count,
            ),
            Style::new().fg(Color::DarkGray),
        )));
        lines.push(Line::from(Span::styled(
            format!(" session-head {}", inspection.session_head_identity),
            Style::new().fg(Color::DarkGray),
        )));
    }
    lines
}

fn pending_lines(view: &crate::TerminalViewState) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    if let Some(pending) = view.conversation().pending_thinking() {
        lines.push(Line::from(Span::styled(
            format!("{PENDING_GLYPH}thinking {pending}"),
            Style::new().fg(Color::DarkGray),
        )));
    }
    if let Some(pending) = view.conversation().pending_text() {
        lines.push(Line::from(Span::styled(
            format!("{PENDING_GLYPH}{pending}"),
            Style::new().fg(Color::Yellow),
        )));
    }
    lines
}

fn transcript_spans(view: &crate::TerminalViewState) -> Vec<Line<'static>> {
    let mut lines = vec![Line::from(Span::styled(
        format!(
            "[total entries: {}, evicted: {}]",
            view.conversation().total_entry_count(),
            view.conversation().evicted_count(),
        ),
        Style::new().fg(Color::DarkGray),
    ))];
    lines.extend(warning_lines(view));
    lines.extend(pending_lines(view));
    for entry in view.conversation().transcript() {
        lines.extend(entry_lines(entry));
    }
    lines
}

/// Wraps styled lines to one width, then keeps only the trailing window that
/// fits the given height so the newest conversation content stays visible.
fn fitted_lines(lines: Vec<Line<'static>>, width: u16, height: u16) -> Vec<Line<'static>> {
    let width = usize::from(width.max(1));
    let height = usize::from(height.max(1));
    let mut wrapped: Vec<Line<'static>> = Vec::with_capacity(lines.len());
    for line in lines {
        wrapped.extend(wrap_line_to_width(line, width));
    }
    if wrapped.len() > height {
        wrapped.drain(..wrapped.len() - height);
    }
    wrapped
}

/// Flattens one styled line into width-sized styled lines without losing any
/// character or per-span styling.
fn wrap_line_to_width(line: Line<'static>, width: usize) -> Vec<Line<'static>> {
    let base_style = line.style;
    let mut flat: Vec<(char, Style)> = Vec::with_capacity(128);
    for span in &line.spans {
        let style = base_style.patch(span.style);
        for char in span.content.chars() {
            flat.push((char, style));
        }
    }
    if flat.is_empty() {
        return vec![line];
    }
    flat.chunks(width).map(rebuild_styled_line).collect()
}

/// Groups one width-sized cell run back into consecutive same-styled spans.
fn rebuild_styled_line(cells: &[(char, Style)]) -> Line<'static> {
    let mut spans: Vec<Span<'static>> = Vec::new();
    let mut current_text = String::new();
    let mut current_style: Option<Style> = None;
    for (char, style) in cells {
        if current_style == Some(*style) {
            current_text.push(*char);
        } else {
            if !current_text.is_empty() {
                spans.push(Span::styled(
                    current_text.clone(),
                    current_style.unwrap_or_default(),
                ));
            }
            current_text.clear();
            current_text.push(*char);
            current_style = Some(*style);
        }
    }
    if !current_text.is_empty() {
        spans.push(Span::styled(
            current_text,
            current_style.unwrap_or_default(),
        ));
    }
    Line::from(spans)
}

fn separator(width: u16) -> Line<'static> {
    Line::from(Span::styled(
        SEPARATOR_FILL.repeat(usize::from(width)),
        Style::new().fg(Color::DarkGray),
    ))
}

fn prompt_footer(view: &crate::TerminalViewState) -> Line<'static> {
    let content = view.prompt().content();
    let mut spans = vec![Span::styled(
        PROMPT_GLYPH,
        Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
    )];
    if content.is_empty() {
        spans.push(Span::styled(CURSOR_GLYPH, Style::new().fg(Color::DarkGray)));
    } else {
        spans.push(Span::raw(content.to_owned()));
        spans.push(Span::styled(CURSOR_GLYPH, Style::new().fg(Color::Cyan)));
    }
    Line::from(spans)
}

fn shell_block() -> Block<'static> {
    Block::new()
        .borders(Borders::ALL)
        .title(Span::styled(
            "Animus Live Terminal",
            Style::new().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ))
        .title(Span::styled(
            "  /state /clear /refresh /detach",
            Style::new().fg(Color::DarkGray),
        ))
}

fn transcript_paragraph(
    view: &crate::TerminalViewState,
    area: ratatui::layout::Rect,
) -> Paragraph<'static> {
    let inner_width = area.width.saturating_sub(2);
    let inner_height = area.height.saturating_sub(2);
    let lines = fitted_lines(transcript_spans(view), inner_width, inner_height);
    Paragraph::new(lines).block(shell_block())
}
