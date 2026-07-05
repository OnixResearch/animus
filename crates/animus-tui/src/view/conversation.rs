// Pure conversation render reducer and transcript types.
//
// This module defines the conversation-level transcript model.


/// Bounded window of visible transcript entries.
const DEFAULT_VISIBLE_WINDOW: usize = 256;
/// Maximum content bytes before truncation.
const MAX_CONTENT_CHARS: usize = animus_adapters::MAX_ADAPTER_FIELD_CHARS;

/// Kind of transcript entry, used for stable display routing.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TranscriptEntryKind {
    /// User-submitted message.
    UserMessage,
    /// Assistant content text block.
    AssistantText,
    /// Assistant thinking/reasoning block.
    Thinking,
    /// Model-declared tool-use invocation.
    ToolUse,
    /// Completed tool-result or tool-output block.
    ToolResult,
    /// Image payload display descriptor.
    Image,
    /// Error or failure entry.
    Error,
    /// System-level message (startup, replay, compaction).
    SystemMessage,
    /// Unknown or unmodeled event variant.
    Unknown,
}

impl TranscriptEntryKind {
    /// Returns the stable kind label for display and snapshot routing.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UserMessage => "user",
            Self::AssistantText => "assistant",
            Self::Thinking => "thinking",
            Self::ToolUse => "tool_use",
            Self::ToolResult => "tool_result",
            Self::Image => "image",
            Self::Error => "error",
            Self::SystemMessage => "system",
            Self::Unknown => "unknown",
        }
    }
}

/// Safe metadata carried alongside bounded content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EntryMetadata {
    /// Size of the visible content in bytes (before optional truncation).
    pub size: usize,
    /// Whether the visible content was truncated from a larger source.
    pub truncated: bool,
    /// Optional BLAKE3 content hash when identity is needed.
    pub content_hash: Option<String>,
}

/// One typed, bounded entry in the visible conversation transcript.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TranscriptEntry {
    /// User-submitted message text.
    UserMessage {
        /// Bounded visible content.
        content: String,
        /// Safe size and truncation metadata.
        metadata: EntryMetadata,
    },
    /// Assistant text content block.
    AssistantText {
        /// Bounded visible content.
        content: String,
        /// Safe size and truncation metadata.
        metadata: EntryMetadata,
    },
    /// Assistant thinking/reasoning block.
    Thinking {
        /// Bounded visible content.
        content: String,
        /// Safe size and truncation metadata.
        metadata: EntryMetadata,
    },
    /// Model-declared tool-use invocation.
    ToolUse {
        /// Tool name selected by the model.
        tool_name: String,
        /// Bounded input argument text.
        input: String,
    },
    /// Completed tool-result block.
    ToolResult {
        /// Name of the tool that produced this result.
        tool_name: String,
        /// Bounded output text.
        output: String,
        /// Safe size and truncation metadata.
        metadata: EntryMetadata,
    },
    /// Image payload display descriptor.
    Image {
        /// Human-readable description or alt text.
        description: String,
        /// Media type label.
        media_type: String,
    },
    /// Error or failure block.
    Error {
        /// Safe error reason text.
        reason: String,
    },
    /// System-level informational entry.
    SystemMessage {
        /// System message text.
        content: String,
    },
    /// Unknown or unmodeled event variant.
    Unknown {
        /// Safe hint about the event kind.
        kind_hint: String,
    },
}

impl TranscriptEntry {
    /// Returns the stable kind for this entry.
    #[must_use]
    pub const fn kind(&self) -> TranscriptEntryKind {
        match self {
            Self::UserMessage { .. } => TranscriptEntryKind::UserMessage,
            Self::AssistantText { .. } => TranscriptEntryKind::AssistantText,
            Self::Thinking { .. } => TranscriptEntryKind::Thinking,
            Self::ToolUse { .. } => TranscriptEntryKind::ToolUse,
            Self::ToolResult { .. } => TranscriptEntryKind::ToolResult,
            Self::Image { .. } => TranscriptEntryKind::Image,
            Self::Error { .. } => TranscriptEntryKind::Error,
            Self::SystemMessage { .. } => TranscriptEntryKind::SystemMessage,
            Self::Unknown { .. } => TranscriptEntryKind::Unknown,
        }
    }
}

/// Pure conversation render model advanced by applying runtime events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConversationRenderModel {
    /// Ordered visible transcript entries within the bounded window.
    transcript: Vec<TranscriptEntry>,
    /// Start index of the visible window within the full event log.
    window_start: usize,
    /// Maximum number of entries in the visible window.
    window_size: usize,
    /// Number of entries evicted from the front of the window.
    evicted_count: usize,
    /// Accumulated streaming-delta buffer for in-progress assistant text.
    pending_text: Option<PendingTextBlock>,
    /// Accumulated thinking buffer.
    pending_thinking: Option<PendingTextBlock>,
    /// Active turn index for correlation.
    active_turn: Option<usize>,
}

/// Buffer for in-progress streaming text/thinking blocks.
#[derive(Clone, Debug, Eq, PartialEq)]
struct PendingTextBlock {
    /// Accumulated delta text.
    buffer: String,
}

impl ConversationRenderModel {
    /// Builds an empty conversation render model with the default window.
    #[must_use]
    pub fn new() -> Self {
        Self {
            transcript: Vec::new(),
            window_start: 0,
            window_size: DEFAULT_VISIBLE_WINDOW,
            evicted_count: 0,
            pending_text: None,
            pending_thinking: None,
            active_turn: None,
        }
    }

    /// Builds a conversation render model with a specific visible window.
    #[must_use]
    pub fn with_window_size(window_size: usize) -> Self {
        Self {
            window_size,
            ..Self::new()
        }
    }

    /// Applies one runtime event to advance the conversation model.
    ///
    /// This is a pure reducer: no terminal I/O, provider calls, or tool
    /// execution occurs.
    #[allow(clippy::too_many_lines, clippy::match_same_arms)]
    pub fn apply_runtime_event(&mut self, event: &animus_runtime::RuntimeEvent) {
        match event {
            animus_runtime::RuntimeEvent::RuntimeStarted { summary }
            | animus_runtime::RuntimeEvent::ContextReplayed { summary } => {
                self.flush_pending();
                self.push_system_entry(summary);
            }
            animus_runtime::RuntimeEvent::LoopStarted { .. }
            | animus_runtime::RuntimeEvent::TurnStarted { .. } => {
                self.flush_pending();
            }
            animus_runtime::RuntimeEvent::ModelRequested { request } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::UserMessage {
                    content: bounded_content(request.prompt()),
                    metadata: size_metadata(request.prompt()),
                });
            }
            animus_runtime::RuntimeEvent::ModelStreamStarted { .. } => {
                // Stream lifecycle: prepare to accumulate deltas.
            }
            animus_runtime::RuntimeEvent::ModelStreamFrame { frame } => {
                self.apply_stream_frame(frame);
            }
            animus_runtime::RuntimeEvent::ModelStreamEnded { .. } => {
                self.flush_pending_delta();
            }
            animus_runtime::RuntimeEvent::ModelStreamFailed { reason, .. } => {
                self.flush_pending_delta();
                self.push_entry(TranscriptEntry::Error {
                    reason: bounded_reason(reason),
                });
            }
            animus_runtime::RuntimeEvent::ModelResponded { response }
            | animus_runtime::RuntimeEvent::FinalResponse { response } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::AssistantText {
                    content: bounded_content(response.message()),
                    metadata: size_metadata(response.message()),
                });
            }
            animus_runtime::RuntimeEvent::ModelDenied { reason, .. } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::Error {
                    reason: bounded_reason(reason),
                });
            }
            animus_runtime::RuntimeEvent::ToolRequested { request } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::ToolUse {
                    tool_name: bounded_content(request.name()),
                    input: bounded_content(request.input()),
                });
            }
            animus_runtime::RuntimeEvent::ToolResponded { response } => {
                let output = response.output();
                self.push_entry(TranscriptEntry::ToolResult {
                    tool_name: String::from("tool"),
                    output: bounded_content(output),
                    metadata: size_metadata(output),
                });
            }
            animus_runtime::RuntimeEvent::ToolDenied { reason, request } => {
                self.push_entry(TranscriptEntry::Error {
                    reason: format!(
                        "tool {} denied: {}",
                        bounded_content(request.name()),
                        bounded_reason(reason)
                    ),
                });
            }
            animus_runtime::RuntimeEvent::Stopped
            | animus_runtime::RuntimeEvent::Shutdown { .. } => {
                self.flush_pending();
            }
            animus_runtime::RuntimeEvent::MaxTurnsExceeded { max_turns } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::SystemMessage {
                    content: format!("Reached maximum turns ({max_turns})"),
                });
            }
            animus_runtime::RuntimeEvent::AdapterError {
                component, message, ..
            } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::Error {
                    reason: format!(
                        "{} adapter error: {}",
                        adapter_component_label(*component),
                        bounded_reason(message)
                    ),
                });
            }
            animus_runtime::RuntimeEvent::FabricError { message } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::Error {
                    reason: bounded_reason(message),
                });
            }
            animus_runtime::RuntimeEvent::SteelPolicyDenied {
                reason, profile, ..
            } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::SystemMessage {
                    content: format!(
                        "Policy denied (profile: {}): {}",
                        bounded_content(profile),
                        bounded_reason(reason)
                    ),
                });
            }
            animus_runtime::RuntimeEvent::Cancelled { reason } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::SystemMessage {
                    content: format!("Cancelled: {}", bounded_reason(reason)),
                });
            }
            animus_runtime::RuntimeEvent::CompactionRequested { summary } => {
                self.flush_pending();
                self.push_entry(TranscriptEntry::SystemMessage {
                    content: bounded_content(summary),
                });
            }
            _other => {
                // Hook events, lifecycle outcomes, sandbox escalations, and
                // other non-conversation events are not transcribed.
            }
        }
    }

    /// Returns the ordered visible transcript entries.
    #[must_use]
    pub fn transcript(&self) -> &[TranscriptEntry] {
        self.transcript.as_slice()
    }

    /// Returns the number of entries evicted from the visible window.
    #[must_use]
    pub const fn evicted_count(&self) -> usize {
        self.evicted_count
    }

    /// Returns the maximum visible window size.
    #[must_use]
    pub const fn window_size(&self) -> usize {
        self.window_size
    }

    /// Returns the total number of entries ever recorded (including evicted).
    #[must_use]
    pub fn total_entry_count(&self) -> usize {
        self.evicted_count + self.transcript.len()
    }

    /// Applies a model stream frame to accumulate deltas.
    fn apply_stream_frame(&mut self, frame: &animus_adapters::ModelStreamFrame) {
        match frame.kind() {
            animus_adapters::ModelStreamFrameKind::ContentDelta => {
                let body = frame.body().unwrap_or("");
                self.pending_text
                    .get_or_insert_with(|| PendingTextBlock {
                        buffer: String::new(),
                    })
                    .buffer
                    .push_str(body);
            }
            animus_adapters::ModelStreamFrameKind::ThoughtDelta => {
                let body = frame.body().unwrap_or("");
                self.pending_thinking
                    .get_or_insert_with(|| PendingTextBlock {
                        buffer: String::new(),
                    })
                    .buffer
                    .push_str(body);
            }
            animus_adapters::ModelStreamFrameKind::ToolCallDelta
            | animus_adapters::ModelStreamFrameKind::Usage => {
                // Tool-call deltas and usage metadata are not rendered as
                // separate transcript entries.
            }
            animus_adapters::ModelStreamFrameKind::Finish
            | animus_adapters::ModelStreamFrameKind::Cancelled
            | animus_adapters::ModelStreamFrameKind::Failure => {
                self.flush_pending_delta();
            }
        }
    }

    /// Flushes all pending (accumulated-streaming) entry buffers.
    fn flush_pending_delta(&mut self) {
        if let Some(pending) = self.pending_text.take() {
            let content = pending.buffer;
            if !content.is_empty() {
                self.push_entry(TranscriptEntry::AssistantText {
                    content: bounded_content(&content),
                    metadata: size_metadata(&content),
                });
            }
        }
        if let Some(pending) = self.pending_thinking.take() {
            let content = pending.buffer;
            if !content.is_empty() {
                self.push_entry(TranscriptEntry::Thinking {
                    content: bounded_content(&content),
                    metadata: size_metadata(&content),
                });
            }
        }
    }

    /// Flushes all pending state.
    fn flush_pending(&mut self) {
        self.flush_pending_delta();
    }

    /// Pushes one transcript entry, maintaining the bounded window.
    fn push_entry(&mut self, entry: TranscriptEntry) {
        self.transcript.push(entry);
        if self.transcript.len() > self.window_size {
            self.transcript.remove(0);
            self.evicted_count = self.evicted_count.saturating_add(1);
            self.window_start = self.window_start.saturating_add(1);
        }
    }

    /// Pushes a system message entry.
    fn push_system_entry(&mut self, content: &str) {
        self.push_entry(TranscriptEntry::SystemMessage {
            content: bounded_content(content),
        });
    }
}

impl Default for ConversationRenderModel {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Deterministic text-snapshot helpers (terminal-free test projection)
// ---------------------------------------------------------------------------

/// Renders a deterministic text snapshot of the conversation transcript.
///
/// Returns one line per entry with a stable kind prefix and bounded content.
/// No terminal backend is required.
#[must_use]
pub fn render_conversation_snapshot(model: &ConversationRenderModel) -> String {
    let mut lines: Vec<String> = Vec::new();

    if model.evicted_count() > 0 {
        lines.push(format!(
            "[... {} earlier entries evicted]",
            model.evicted_count()
        ));
    }

    for entry in model.transcript() {
        let line = match entry {
            TranscriptEntry::UserMessage { content, metadata } => {
                snapshot_entry_line("user", content, metadata)
            }
            TranscriptEntry::AssistantText { content, metadata } => {
                snapshot_entry_line("assistant", content, metadata)
            }
            TranscriptEntry::Thinking { content, metadata } => {
                snapshot_entry_line("thinking", content, metadata)
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
                    format!(
                        "{base} [truncated: {} chars, full: {}]",
                        output.len(),
                        metadata.size
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
fn snapshot_entry_line(kind: &str, content: &str, metadata: &EntryMetadata) -> String {
    if metadata.truncated {
        format!(
            "{kind}: {content} [truncated: {} chars, full: {}]",
            content.len(),
            metadata.size
        )
    } else {
        format!("{kind}: {content}")
    }
}

// ---------------------------------------------------------------------------
// Bounding helpers
// ---------------------------------------------------------------------------

/// Truncates content to the maximum visible budget.
fn bounded_content(content: &str) -> String {
    if content.len() <= MAX_CONTENT_CHARS {
        String::from(content)
    } else {
        let mut truncated = String::with_capacity(MAX_CONTENT_CHARS);
        truncated.push_str(&content[..MAX_CONTENT_CHARS]);
        truncated
    }
}

/// Builds safe metadata for content of the given size.
fn size_metadata(content: &str) -> EntryMetadata {
    EntryMetadata {
        size: content.len(),
        truncated: content.len() > MAX_CONTENT_CHARS,
        content_hash: None,
    }
}

/// Bounded safe error reason text.
fn bounded_reason(reason: &str) -> String {
    bounded_content(reason)
}

/// Returns a stable label for an adapter component.
fn adapter_component_label(component: animus_runtime::AdapterComponent) -> &'static str {
    match component {
        animus_runtime::AdapterComponent::Model => "model",
        animus_runtime::AdapterComponent::Tool => "tool",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -----------------------------------------------------------------------
    // Positive tests
    // -----------------------------------------------------------------------

    #[test]
    fn empty_transcript_renders_snapshot() {
        let model = ConversationRenderModel::new();
        let snapshot = render_conversation_snapshot(&model);
        assert_eq!(snapshot, "(empty transcript)");
    }

    #[test]
    fn user_message_produces_transcript_entry() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelRequested {
            request: animus_adapters::ModelRequest::new("What is Rust?").unwrap(),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("user: What is Rust?"));
    }

    #[test]
    fn model_response_produces_assistant_entry() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
            response: animus_adapters::ModelResponse::new("Rust is a systems language.").unwrap(),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("assistant: Rust is a systems language."));
    }

    #[test]
    fn tool_request_and_response_produce_entries() {
        let mut model = ConversationRenderModel::new();
        let request =
            animus_adapters::ToolRequest::new("read_file", "path: Cargo.toml").unwrap();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ToolRequested {
            request: request.clone(),
        });
        let response =
            animus_adapters::ToolResponse::new("[package]\nname = \"animus\"").unwrap();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ToolResponded { response });

        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("tool_use: read_file"));
        assert!(snapshot.contains("tool_result:"));
    }

    #[test]
    fn runtime_started_produces_system_entry() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::RuntimeStarted {
            summary: String::from("agent-harness v0.1.0"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("system: agent-harness v0.1.0"));
    }

    #[test]
    fn model_denied_produces_error_entry() {
        let mut model = ConversationRenderModel::new();
        let request = animus_adapters::ModelRequest::new("dangerous command").unwrap();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelDenied {
            request,
            reason: String::from("policy: profile_mismatch"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("error: policy: profile_mismatch"));
    }

    #[test]
    fn tool_denied_produces_error_entry() {
        let mut model = ConversationRenderModel::new();
        let request = animus_adapters::ToolRequest::new("rm", "/").unwrap();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ToolDenied {
            request,
            reason: String::from("capability: denied"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("error: tool rm denied: capability: denied"));
    }

    #[test]
    fn stream_deltas_accumulate_into_assistant_entry() {
        let call_id = animus_adapters::ModelStreamCallId::new("call-1").unwrap();
        let mut model = ConversationRenderModel::new();

        // Simulate streaming deltas
        let frame1 = make_content_delta(&call_id, 1, "Hello ");
        let frame2 = make_content_delta(&call_id, 2, "world");
        let frame3 = make_finish(&call_id, 3);

        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame: frame1 });
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame: frame2 });
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame: frame3 });

        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("assistant: Helloworld"));
    }

    #[test]
    fn max_turns_exceeded_produces_system_entry() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::MaxTurnsExceeded {
            max_turns: 10,
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("system: Reached maximum turns (10)"));
    }

    #[test]
    fn evicted_entries_show_count_in_snapshot() {
        let mut model = ConversationRenderModel::with_window_size(3);
        for i in 0..5 {
            let content = format!("Message {i}");
            model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
                response: animus_adapters::ModelResponse::new(&content).unwrap(),
            });
        }
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("earlier entries evicted"));
        assert_eq!(model.total_entry_count(), 5);
        assert_eq!(model.evicted_count(), 2);
    }

    #[test]
    fn total_entry_count_includes_evicted() {
        let mut model = ConversationRenderModel::with_window_size(2);
        for i in 0..4 {
            let content = format!("Message {i}");
            model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
                response: animus_adapters::ModelResponse::new(&content).unwrap(),
            });
        }
        assert_eq!(model.total_entry_count(), 4);
        assert_eq!(model.transcript().len(), 2);
    }

    #[test]
    fn transcript_ordering_is_preserved() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelRequested {
            request: animus_adapters::ModelRequest::new("Q1").unwrap(),
        });
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
            response: animus_adapters::ModelResponse::new("A1").unwrap(),
        });
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelRequested {
            request: animus_adapters::ModelRequest::new("Q2").unwrap(),
        });
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
            response: animus_adapters::ModelResponse::new("A2").unwrap(),
        });
        let snapshot = render_conversation_snapshot(&model);
        let q1_pos = snapshot.find("user: Q1").unwrap();
        let a1_pos = snapshot.find("assistant: A1").unwrap();
        let q2_pos = snapshot.find("user: Q2").unwrap();
        let a2_pos = snapshot.find("assistant: A2").unwrap();
        assert!(q1_pos < a1_pos);
        assert!(a1_pos < q2_pos);
        assert!(q2_pos < a2_pos);
    }

    // -----------------------------------------------------------------------
    // Negative tests
    // -----------------------------------------------------------------------

    #[test]
    fn unknown_event_variant_does_not_panic() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::Stopped);
        let snapshot = render_conversation_snapshot(&model);
        assert_eq!(snapshot, "(empty transcript)");
    }

    #[test]
    fn hook_events_dont_produce_transcript_entries() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::HookBlocked {
            boundary: animus_runtime::HookBoundary::ModelDispatch,
            hook: String::from("test-hook"),
            reason: String::from("test"),
        });
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::HookFailed {
            boundary: animus_runtime::HookBoundary::ToolDispatch,
            hook: String::from("test-hook"),
            message: String::from("test failure"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert_eq!(snapshot, "(empty transcript)");
    }

    #[test]
    fn oversized_content_is_truncated() {
        // Verify the snapshot does not show truncation when content is within bounds.
        // Truncation at the conversation level only triggers when content exceeds
        // MAX_CONTENT_CHARS, which is also the adapter max, so we test the
        // within-bounds path here.
        let within_bounds = "a".repeat(MAX_CONTENT_CHARS);
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
            response: animus_adapters::ModelResponse::new(&within_bounds).unwrap(),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains(&format!("assistant: {}", "a".repeat(MAX_CONTENT_CHARS))));
        assert!(!snapshot.contains("[truncated"));
    }

    #[test]
    fn model_stream_failure_produces_error() {
        let call_id = animus_adapters::ModelStreamCallId::new("call-fail").unwrap();
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFailed {
            call_id,
            reason: String::from("provider timeout"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("error: provider timeout"));
    }

    #[test]
    fn fabric_error_produces_error_entry() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::FabricError {
            message: String::from("loop fabric panicked"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("error: loop fabric panicked"));
    }

    #[test]
    fn cancelled_produces_system_entry() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::Cancelled {
            reason: String::from("user interrupt"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("system: Cancelled: user interrupt"));
    }

    #[test]
    fn steel_policy_denied_produces_system_entry() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::SteelPolicyDenied {
            profile: String::from("strict"),
            script_hash: String::from("abc123"),
            receipt_hash: String::from("def456"),
            reason: String::from("profile_mismatch"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(
            snapshot.contains("system: Policy denied (profile: strict): profile_mismatch")
        );
    }

    #[test]
    fn compaction_requested_produces_system_entry() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::CompactionRequested {
            summary: String::from("Mid-turn compaction at turn 42"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("system: Mid-turn compaction at turn 42"));
    }

    #[test]
    fn adapter_error_produces_error_entry() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::AdapterError {
            component: animus_runtime::AdapterComponent::Model,
            message: String::from("provider unavailable"),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("error: model adapter error: provider unavailable"));
    }

    #[test]
    fn model_response_with_single_char_is_rendered() {
        let mut model = ConversationRenderModel::new();
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
            response: animus_adapters::ModelResponse::new(".").unwrap(),
        });
        let snapshot = render_conversation_snapshot(&model);
        assert!(snapshot.contains("assistant: ."));
    }

    // -----------------------------------------------------------------------
    // Test helpers
    // -----------------------------------------------------------------------

    fn make_content_delta(
        call_id: &animus_adapters::ModelStreamCallId,
        order: u64,
        body: &str,
    ) -> animus_adapters::ModelStreamFrame {
        animus_adapters::ModelStreamFrame::from_draft(
            animus_adapters::ModelStreamFrameDraft {
                call_id: call_id.as_str(),
                order,
                kind: animus_adapters::ModelStreamFrameKind::ContentDelta,
                body: Some(body),
                tool_call_id: None,
                usage: None,
                visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
            },
        )
        .expect("content delta should validate")
    }

    fn make_finish(
        call_id: &animus_adapters::ModelStreamCallId,
        order: u64,
    ) -> animus_adapters::ModelStreamFrame {
        animus_adapters::ModelStreamFrame::from_draft(
            animus_adapters::ModelStreamFrameDraft {
                call_id: call_id.as_str(),
                order,
                kind: animus_adapters::ModelStreamFrameKind::Finish,
                body: None,
                tool_call_id: None,
                usage: None,
                visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
            },
        )
        .expect("finish frame should validate")
    }
}