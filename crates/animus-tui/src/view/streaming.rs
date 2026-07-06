// Streaming markdown view: rendering deltas, images, and scrollback on top
// of the conversation render model.
//
// This module is downstream-only — it consumes the conversation render model,
// adapter DTOs, and runtime events without adding new reusable-crate contracts.

/// Maximum display width for image descriptions.
const MAX_IMAGE_DESCRIPTION_CHARS: u64 = 256;
/// Valid 64-char BLAKE3 hex content hash for test fixtures.
#[cfg(test)]
const VALID_CONTENT_HASH: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

// ---------------------------------------------------------------------------
// Image display
// ---------------------------------------------------------------------------

/// Bounded display descriptor for a multimodal image payload.
/// Carries safe metadata from adapter DTOs; raw provider bytes are never retained.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ImageDisplay {
    /// Human-readable description or alt text, bounded.
    description: String,
    /// Media type label, bounded.
    media_type: String,
    /// True when the image was truncated or bounded.
    bounded: bool,
}

impl ImageDisplay {
    /// Builds a bounded image display descriptor from a validated image payload handle.
    /// Only safe metadata is retained; raw provider bytes, provider file IDs, and
    /// credentials are never stored in the display descriptor.
    #[must_use]
    pub fn from_handle(
        handle: &animus_adapters::ModelImagePayloadHandle,
        description: Option<&str>,
        fallback_description: &str,
    ) -> Self {
        let desc = description
            .unwrap_or(fallback_description)
            .chars()
            .take(usize::try_from(MAX_IMAGE_DESCRIPTION_CHARS).unwrap_or(256))
            .collect();
        let media = handle.media_type().as_str();
        Self {
            description: desc,
            media_type: String::from(media),
            bounded: false,
        }
    }

    /// Returns the bounded description text.
    #[must_use]
    pub fn description(&self) -> &str {
        self.description.as_str()
    }

    /// Returns the bounded media type label.
    #[must_use]
    pub fn media_type(&self) -> &str {
        self.media_type.as_str()
    }

    /// Returns true when the image was bounded/truncated.
    #[must_use]
    pub const fn is_bounded(&self) -> bool {
        self.bounded
    }
}

// ---------------------------------------------------------------------------
// Streaming snapshot
// ---------------------------------------------------------------------------

const PENDING_TEXT_LABEL: &str = "[pending text: ";
const PENDING_THINKING_LABEL: &str = "[pending thinking: ";
const PENDING_SUFFIX: &str = "]";
const TOTAL_ENTRIES_LABEL: &str = "[total entries: ";
const EVICTED_LABEL: &str = ", evicted: ";

/// Renders a deterministic text snapshot of the conversation model's streaming
/// state, including pending accumulated content and scrollback metadata.
/// No terminal backend is required.
///
/// # Panics
///
/// Panics when `model.window_size()` is zero.
#[must_use]
pub fn render_streaming_snapshot(model: &super::ConversationRenderModel) -> String {
    assert!(model.window_size() > 0);
    let conversation_part = super::render_conversation_snapshot(model);
    let mut overlay_lines: Vec<String> = Vec::new();

    if let Some(pending) = model.pending_text() {
        let content = pending.chars().take(80).collect::<String>();
        overlay_lines.push(format!("{PENDING_TEXT_LABEL}{content}{PENDING_SUFFIX}"));
    }
    if let Some(pending) = model.pending_thinking() {
        let content = pending.chars().take(80).collect::<String>();
        overlay_lines.push(format!("{PENDING_THINKING_LABEL}{content}{PENDING_SUFFIX}"));
    }
    overlay_lines.push(format!(
        "{TOTAL_ENTRIES_LABEL}{}{EVICTED_LABEL}{}]",
        model.total_entry_count(),
        model.evicted_count(),
    ));

    if overlay_lines.is_empty() {
        conversation_part
    } else {
        format!("{}\n{}", overlay_lines.join("\n"), conversation_part)
    }
}

// ---------------------------------------------------------------------------
// Scrollback metadata
// ---------------------------------------------------------------------------

/// Bounded scrollback metadata for the visible window.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ScrollbackMetadata {
    /// Number of entries currently visible in the window.
    pub visible_count: u64,
    /// Number of entries evicted from the front of the window.
    pub evicted_count: u64,
    /// Maximum window size.
    pub window_size: u64,
}

impl ScrollbackMetadata {
    /// Captures scrollback metadata from a conversation render model.
    #[must_use]
    pub fn from_model(model: &super::ConversationRenderModel) -> Self {
        Self {
            visible_count: model.transcript().len() as u64,
            evicted_count: model.evicted_count(),
            window_size: model.window_size(),
        }
    }

    /// Returns true when entries have been evicted.
    #[must_use]
    pub const fn has_evicted(&self) -> bool {
        self.evicted_count > 0
    }

    /// Returns the proportion of the window that is filled, as a f64 ratio.
    #[must_use]
    pub fn fill_ratio(&self) -> f64 {
        if self.window_size == 0 {
            0.0
        } else {
            #[allow(clippy::cast_precision_loss)]
            let ratio = self.visible_count as f64 / self.window_size as f64;
            ratio
        }
    }
}

// Tests moved to streamingtest.rs to reduce file length.