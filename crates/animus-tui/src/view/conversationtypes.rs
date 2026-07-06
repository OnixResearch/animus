// Conversation transcript types: kind enum, metadata, and bounded entry variants.

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
    pub size: u64,
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
