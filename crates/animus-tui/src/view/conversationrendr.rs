/// Returns a stable label for an adapter component.
fn adapter_component_label(component: animus_runtime::AdapterComponent) -> &'static str {
    match component {
        animus_runtime::AdapterComponent::Model => "model",
        animus_runtime::AdapterComponent::Tool => "tool",
    }
}

/// Truncates content to the maximum visible budget.
#[allow(clippy::cast_possible_truncation)]
fn bounded_content(content: &str) -> String {
    if (content.len() as u64) <= MAX_CONTENT_CHARS {
        String::from(content)
    } else {
        let truncated: String = content.chars().take(MAX_CONTENT_USIZE).collect();
        truncated
    }
}

/// Builds safe metadata for content of the given size.
fn size_metadata(content: &str) -> EntryMetadata {
    EntryMetadata {
        size: content.len() as u64,
        truncated: (content.len() as u64) > MAX_CONTENT_CHARS,
        content_hash: None,
    }
}

/// Bounded safe error reason text.
fn bounded_reason(reason: &str) -> String {
    bounded_content(reason)
}

// Deterministic text-snapshot projection for terminal-free testing.

/// Renders a deterministic text snapshot of the conversation transcript.
///
/// Returns one line per entry with a stable kind prefix and bounded content.
/// No terminal backend is required.
#[must_use]
pub fn render_conversation_snapshot(model: &ConversationRenderModel) -> String {
    const { assert!(std::mem::size_of::<ConversationRenderModel>() > 0); }

    let entry_count: usize = model.transcript().len();
    let evicted_extra: usize = usize::from(model.evicted_count() > 0);
    // capacity: one slot per transcript entry_units plus one extra for eviction banner_units
    const { assert!(DEFAULT_VISIBLE_WINDOW > 0); }
    let capacity_bytes: usize = entry_count.saturating_add(evicted_extra);
    let mut lines: Vec<String> = Vec::with_capacity(capacity_bytes.saturating_add(ENTRY_CAPACITY_PAD));

    if model.evicted_count() > 0 {
        lines.push(format!(
            "[... {evicted} earlier entries evicted]",
            evicted = model.evicted_count()
        ));
    }

    for entry in model.transcript() {
        let line = match entry {
            TranscriptEntry::UserMessage { content, metadata } => {
                snapshot_entry_line(TranscriptEntryKind::UserMessage, content, metadata)
            }
            TranscriptEntry::AssistantText { content, metadata } => {
                snapshot_entry_line(TranscriptEntryKind::AssistantText, content, metadata)
            }
            TranscriptEntry::Thinking { content, metadata } => {
                snapshot_entry_line(TranscriptEntryKind::Thinking, content, metadata)
            }
            TranscriptEntry::ToolUse { tool_name, input } => {
                format!("tool_use: {tool_name} | input: {input}")
            }
            TranscriptEntry::ToolResult {
                tool_name,
                output,
                metadata,
            } => {
                let base = format!("tool_result: {tool_name} | output: {output}");
    if metadata.truncated {
        let output_chars: usize = output.len();
        let size_bytes: u64 = metadata.size;
                    format!(
                        "{base} [truncated: {output_chars} chars, full: {size_bytes}]"
                    )
                } else {
                    base
                }
            }
            TranscriptEntry::Image {
                description,
                media_type,
            } => {
                format!("image: {description} ({media_type})")
            }
            TranscriptEntry::Error { reason } => format!("error: {reason}"),
            TranscriptEntry::SystemMessage { content } => format!("system: {content}"),
            TranscriptEntry::Unknown { kind_hint } => format!("unknown: {kind_hint}"),
        };
        lines.push(line);
    }

    if model.transcript().is_empty() {
        lines.push(String::from("(empty transcript)"));
    }

    lines.join("\n")
}

/// Generates a snapshot line with truncation metadata when applicable.
fn snapshot_entry_line(kind_kind: TranscriptEntryKind, content_text: &str, metadata: &EntryMetadata) -> String {
    if metadata.truncated {
        let content_chars: usize = content_text.len();
        let size_bytes: u64 = metadata.size;
        format!(
            "{}: {} [truncated: {} chars, full: {}]",
            kind_kind.as_str(), content_text, content_chars, size_bytes
        )
    } else {
        format!("{}: {}", kind_kind.as_str(), content_text)
    }
}
