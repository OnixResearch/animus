
// Pure conversation render reducer: advances transcript state from runtime events.

/// Bounded window of visible transcript entries.
const DEFAULT_VISIBLE_WINDOW: u64 = 256;
/// Maximum content bytes before truncation.
const MAX_CONTENT_CHARS: u64 = animus_adapters::MAX_ADAPTER_FIELD_CHARS as u64;
/// usize version of the max content limit for slice indexing.
const MAX_CONTENT_USIZE: usize = animus_adapters::MAX_ADAPTER_FIELD_CHARS;
/// Padding entry reserved for the eviction banner line.
const ENTRY_CAPACITY_PAD: usize = 1;

/// Pure conversation render model advanced by applying runtime events.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConversationRenderModel {
    /// Ordered visible transcript entries within the bounded window.
    transcript: Vec<TranscriptEntry>,
    /// Start index of the visible window within the full event log.
    window_start: u64,
    /// Maximum number of entries in the visible window.
    window_size: u64,
    /// Number of entries evicted from the front of the window.
    evicted_count: u64,
    /// Accumulated streaming-delta buffer for in-progress assistant text.
    pending_text: Option<PendingTextBlock>,
    /// Accumulated thinking buffer.
    pending_thinking: Option<PendingTextBlock>,
    /// Active turn index for correlation.
    active_turn: Option<u64>,
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
        const { assert!(DEFAULT_VISIBLE_WINDOW > 0); }
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
    pub fn with_window_size(window_size: u64) -> Self {
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
            animus_runtime::RuntimeEvent::ModelStreamStarted { .. } => {}
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
            _other => {}
        }
    }

    /// Returns the ordered visible transcript entries.
    #[must_use]
    pub fn transcript(&self) -> &[TranscriptEntry] {
        self.transcript.as_slice()
    }

    /// Returns the number of entries evicted from the visible window.
    #[must_use]
    pub const fn evicted_count(&self) -> u64 {
        self.evicted_count
    }

    /// Returns the maximum visible window size.
    #[must_use]
    pub const fn window_size(&self) -> u64 {
        self.window_size
    }

    /// Returns the total number of entries ever recorded (including evicted).
    #[must_use]
    pub fn total_entry_count(&self) -> u64 {
        self.evicted_count.saturating_add(self.transcript.len() as u64)
    }

    /// Applies a model stream frame to accumulate deltas.
    fn apply_stream_frame(&mut self, frame: &animus_adapters::ModelStreamFrame) {
        match frame.kind() {
            animus_adapters::ModelStreamFrameKind::ContentDelta => {
                let body = frame.body().unwrap_or("");
                let pending = self.pending_text.get_or_insert_with(|| PendingTextBlock {
                    buffer: String::new(),
                });
                pending.buffer.push_str(body);
            }
            animus_adapters::ModelStreamFrameKind::ThoughtDelta => {
                let body = frame.body().unwrap_or("");
                let pending = self.pending_thinking.get_or_insert_with(|| PendingTextBlock {
                    buffer: String::new(),
                });
                pending.buffer.push_str(body);
            }
            animus_adapters::ModelStreamFrameKind::ToolCallDelta
            | animus_adapters::ModelStreamFrameKind::Usage => {}
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
    #[allow(clippy::cast_possible_truncation)]
    fn push_entry(&mut self, entry: TranscriptEntry) {
        self.transcript.push(entry);
        // SAFETY: DEFAULT_VISIBLE_WINDOW (256) fits in any usize
        const { assert!(DEFAULT_VISIBLE_WINDOW <= usize::MAX as u64); }
        if (self.transcript.len() as u64) > self.window_size {
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

