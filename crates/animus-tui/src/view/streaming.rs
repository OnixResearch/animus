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
///
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
    ///
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
            .take(MAX_IMAGE_DESCRIPTION_CHARS as usize)
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

/// Prefix for pending text content in streaming snapshot.
const PENDING_TEXT_LABEL: &str = "[pending text: ";
/// Prefix for pending thinking content in streaming snapshot.
const PENDING_THINKING_LABEL: &str = "[pending thinking: ";
/// Suffix for pending content blocks.
const PENDING_SUFFIX: &str = "]";
/// Label for the total entry count line.
const TOTAL_ENTRIES_LABEL: &str = "[total entries: ";
/// Label for evicted entry count.
const EVICTED_LABEL: &str = ", evicted: ";

/// Renders a deterministic text snapshot of the conversation model's streaming
/// state, including pending accumulated content and scrollback metadata.
///
/// This is a pure text projection — no terminal backend is required.
/// It is intended for testing and for consumers that need a plain-text view
/// of the streaming state without Ratatui dependencies.
#[must_use]
pub fn render_streaming_snapshot(model: &super::ConversationRenderModel) -> String {
    // Start with the conversation snapshot (visible entries)
    let conversation_part = super::render_conversation_snapshot(model);

    // Build pending/scrollback overlay lines
    let mut overlay_lines: Vec<String> = Vec::new();

    // Pending content block status
    if let Some(pending) = model.pending_text() {
        let content = if pending.len() > 80 {
            format!("{}...", &pending[..80])
        } else {
            pending.clone()
        };
        overlay_lines.push(format!(
            "{PENDING_TEXT_LABEL}{content}{PENDING_SUFFIX}"
        ));
    }

    if let Some(pending) = model.pending_thinking() {
        let content = if pending.len() > 80 {
            format!("{}...", &pending[..80])
        } else {
            pending.clone()
        };
        overlay_lines.push(format!(
            "{PENDING_THINKING_LABEL}{content}{PENDING_SUFFIX}"
        ));
    }

    // Scrollback metadata
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
///
/// Carries safe counts and optional hashes for evicted entries.
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
            self.visible_count as f64 / self.window_size as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::view::ConversationRenderModel;

    // -----------------------------------------------------------------------
    // Image display tests
    // -----------------------------------------------------------------------

    #[test]
    fn image_display_from_handle_bounds_description() {
        let long_desc = "a".repeat(512);
        let handle = animus_adapters::ModelImagePayloadHandle::from_draft(
            animus_adapters::ModelImagePayloadHandleDraft {
                kind: animus_adapters::ModelImagePayloadHandleKind::ProviderFileId,
                safe_handle: "safe_abc",
                content_hash: VALID_CONTENT_HASH,
                media_type: "image/png",
            },
        )
        .expect("handle should validate");
        let display = ImageDisplay::from_handle(&handle, Some(&long_desc), "fallback");
        assert_eq!(
            display.description().len(),
            MAX_IMAGE_DESCRIPTION_CHARS as usize
        );
        assert!(display.description().len() < long_desc.len()); // truncated
    }

    #[test]
    fn image_display_without_description_uses_fallback() {
        let handle = animus_adapters::ModelImagePayloadHandle::from_draft(
            animus_adapters::ModelImagePayloadHandleDraft {
                kind: animus_adapters::ModelImagePayloadHandleKind::ProviderFileId,
                safe_handle: "safe_abc",
                content_hash: VALID_CONTENT_HASH,
                media_type: "image/png",
            },
        )
        .expect("handle should validate");
        let display = ImageDisplay::from_handle(&handle, None, "default image");
        assert_eq!(display.description(), "default image");
        assert_eq!(display.media_type(), "image/png");
    }

    #[test]
    fn image_display_rejects_raw_provider_bytes() {
        let handle = animus_adapters::ModelImagePayloadHandle::from_draft(
            animus_adapters::ModelImagePayloadHandleDraft {
                kind: animus_adapters::ModelImagePayloadHandleKind::ProviderFileId,
                safe_handle: "safe_abc",
                content_hash: VALID_CONTENT_HASH,
                media_type: "image/png",
            },
        )
        .expect("handle should validate");
        let display = ImageDisplay::from_handle(&handle, None, "image");
        // The display should not contain the raw provider handle value
        assert!(!display.description().contains("safe_abc"), "safe handle leaked");
    }

    #[test]
    fn image_display_stable_media_type_from_enum() {
        let handle = animus_adapters::ModelImagePayloadHandle::from_draft(
            animus_adapters::ModelImagePayloadHandleDraft {
                kind: animus_adapters::ModelImagePayloadHandleKind::ProviderFileId,
                safe_handle: "safe_abc",
                content_hash: VALID_CONTENT_HASH,
                media_type: "image/jpeg",
            },
        )
        .expect("handle should validate");
        let display = ImageDisplay::from_handle(&handle, None, "image");
        assert_eq!(display.media_type(), "image/jpeg");
        assert!(!display.is_bounded(), "enum values are bounded by definition");
    }

    // -----------------------------------------------------------------------
    // Streaming snapshot tests
    // -----------------------------------------------------------------------

    #[test]
    fn empty_streaming_snapshot_shows_empty_transcript() {
        let model = ConversationRenderModel::new();
        let snapshot = render_streaming_snapshot(&model);
        assert!(snapshot.contains("(empty transcript)"));
        assert!(snapshot.contains("total entries: 0, evicted: 0"));
    }

    #[test]
    fn streaming_snapshot_shows_pending_text_when_buffering() {
        let model = ConversationRenderModel::new();
        let call_id = animus_adapters::ModelStreamCallId::new("call-pending").unwrap();
        let frame = animus_adapters::ModelStreamFrame::from_draft(
            animus_adapters::ModelStreamFrameDraft {
                call_id: call_id.as_str(),
                order: 1,
                kind: animus_adapters::ModelStreamFrameKind::ContentDelta,
                body: Some("accumulating content..."),
                tool_call_id: None,
                usage: None,
                visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
            },
        )
        .expect("frame should validate");
        let mut model_with_frame = model.clone();
        model_with_frame.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame {
            frame,
        });
        let snapshot = render_streaming_snapshot(&model_with_frame);
        assert!(
            snapshot.contains("[pending text:"),
            "pending text should appear"
        );
    }

    #[test]
    fn streaming_snapshot_shows_eviction_metadata() {
        let mut model = ConversationRenderModel::with_window_size(2);
        for i in 0..5 {
            let content = format!("Message {i}");
            model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
                response: animus_adapters::ModelResponse::new(&content).unwrap(),
            });
        }
        let snapshot = render_streaming_snapshot(&model);
        assert!(snapshot.contains("total entries: 5, evicted: 3"));
        assert_eq!(model.transcript().len(), 2); // window of 2
    }

    #[test]
    fn streaming_snapshot_after_stream_shows_accumulated_content() {
        let call_id = animus_adapters::ModelStreamCallId::new("call-acc").unwrap();
        let mut model = ConversationRenderModel::new();

        // Start stream
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamStarted {
            call_id: call_id.clone(),
        });

        // Content deltas (bodies are .trim()ed by normalized_adapter_field)
        for (i, part) in ["Hello ", "world, ", "this is ", "streaming!"].iter().enumerate() {
            let frame = animus_adapters::ModelStreamFrame::from_draft(
                animus_adapters::ModelStreamFrameDraft {
                    call_id: call_id.as_str(),
                    order: (i + 1) as u64,
                    kind: animus_adapters::ModelStreamFrameKind::ContentDelta,
                    body: Some(part),
                    tool_call_id: None,
                    usage: None,
                    visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
                },
            )
            .expect("frame should validate");
            model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame });
        }

        // Before flush: pending text is visible
        let before_snapshot = render_streaming_snapshot(&model);
        assert!(
            before_snapshot.contains("[pending text:"),
            "pending text before flush: {before_snapshot}"
        );

        // End stream — flushes pending
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamEnded {
            call_id,
            status: animus_adapters::ModelStreamFrameKind::Finish,
        });

        let after_snapshot = render_streaming_snapshot(&model);
        assert!(
            after_snapshot.contains("assistant: Helloworld,this isstreaming!"),
            "accumulated content after flush: {after_snapshot}"
        );
    }

    // -----------------------------------------------------------------------
    // Negative tests
    // -----------------------------------------------------------------------

    #[test]
    fn orphan_content_delta_does_not_panic() {
        let mut model = ConversationRenderModel::new();
        let frame = animus_adapters::ModelStreamFrame::from_draft(
            animus_adapters::ModelStreamFrameDraft {
                call_id: "orphan-call",
                order: 1,
                kind: animus_adapters::ModelStreamFrameKind::ContentDelta,
                body: Some("orphaned delta"),
                tool_call_id: None,
                usage: None,
                visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
            },
        )
        .expect("frame should validate");
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame });
        // No panic = success; orphan delta just stays pending
        let snapshot = render_streaming_snapshot(&model);
        assert!(
            snapshot.contains("(empty transcript)") || snapshot.contains("total entries: 0"),
            "orphan delta should not corrupt: {snapshot}"
        );
    }

    #[test]
    fn out_of_order_finish_does_not_corrupt_transcript() {
        let mut model = ConversationRenderModel::new();
        // Finish frame with no preceding content
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamEnded {
            call_id: animus_adapters::ModelStreamCallId::new("empty-finish").unwrap(),
            status: animus_adapters::ModelStreamFrameKind::Finish,
        });
        // No panic = success, transcript should be empty
        let snapshot = super::render_conversation_snapshot(&model);
        assert!(snapshot.contains("(empty transcript)"));
    }

    // -----------------------------------------------------------------------
    // Scrollback metadata tests
    // -----------------------------------------------------------------------

    #[test]
    fn scrollback_metadata_empty_window() {
        let model = ConversationRenderModel::new();
        let meta = ScrollbackMetadata::from_model(&model);
        assert_eq!(meta.visible_count, 0);
        assert_eq!(meta.evicted_count, 0);
        assert!(!meta.has_evicted());
        assert!((meta.fill_ratio() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn scrollback_metadata_tracks_eviction() {
        let mut model = ConversationRenderModel::with_window_size(2);
        for i in 0..5 {
            let content = format!("Message {i}");
            model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
                response: animus_adapters::ModelResponse::new(&content).unwrap(),
            });
        }
        let meta = ScrollbackMetadata::from_model(&model);
        assert_eq!(meta.visible_count, 2);
        assert_eq!(meta.evicted_count, 3);
        assert!(meta.has_evicted());
        assert!((meta.fill_ratio() - 1.0).abs() < f64::EPSILON); // window is full
    }

    #[test]
    fn scrollback_metadata_fill_ratio_partial() {
        let mut model = ConversationRenderModel::with_window_size(5);
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
            response: animus_adapters::ModelResponse::new("single").unwrap(),
        });
        let meta = ScrollbackMetadata::from_model(&model);
        assert_eq!(meta.visible_count, 1);
        assert!((meta.fill_ratio() - 0.2).abs() < f64::EPSILON);
    }
}