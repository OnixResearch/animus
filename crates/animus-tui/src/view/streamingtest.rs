// Streaming markdown view tests.
// Included from view.rs inside `mod streaming_tests { use super::*; include!(...); }`

/// Helper: build a content-delta stream frame.
fn content_delta(call_id: &str, order: u64, body: &str) -> animus_adapters::ModelStreamFrame {
    animus_adapters::ModelStreamFrame::from_draft(animus_adapters::ModelStreamFrameDraft {
        call_id,
        order,
        kind: animus_adapters::ModelStreamFrameKind::ContentDelta,
        body: Some(body),
        tool_call_id: None,
        usage: None,
        visibility: animus_adapters::ModelStreamVisibility::AssistantVisible,
    })
    .expect("frame should validate")
}

/// Helper: build a valid image handle.
fn test_image_handle(media_type: &str) -> animus_adapters::ModelImagePayloadHandle {
    animus_adapters::ModelImagePayloadHandle::from_draft(
        animus_adapters::ModelImagePayloadHandleDraft {
            kind: animus_adapters::ModelImagePayloadHandleKind::ProviderFileId,
            safe_handle: "safe_abc",
            content_hash: VALID_CONTENT_HASH,
            media_type,
        },
    )
    .expect("handle should validate")
}

// -----------------------------------------------------------------------
// Image display tests
// -----------------------------------------------------------------------

#[test]
fn image_display_from_handle_bounds_description() {
    let long_desc = "a".repeat(512);
    let handle = test_image_handle("image/png");
    let display = ImageDisplay::from_handle(&handle, Some(&long_desc), "fallback");
    assert_eq!(
        display.description().len(),
        usize::try_from(MAX_IMAGE_DESCRIPTION_CHARS).unwrap_or(256)
    );
    assert!(display.description().len() < long_desc.len());
}

#[test]
fn image_display_without_description_uses_fallback() {
    let handle = test_image_handle("image/png");
    let display = ImageDisplay::from_handle(&handle, None, "default image");
    assert_eq!(display.description(), "default image");
    assert_eq!(display.media_type(), "image/png");
}

#[test]
fn image_display_rejects_raw_provider_bytes() {
    let handle = test_image_handle("image/png");
    let display = ImageDisplay::from_handle(&handle, None, "image");
    assert!(!display.description().contains("safe_abc"), "safe handle leaked");
}

#[test]
fn image_display_stable_media_type_from_enum() {
    let handle = test_image_handle("image/jpeg");
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
    let frame = content_delta(call_id.as_str(), 1, "accumulating content...");
    let mut model_with_frame = model.clone();
    assert_eq!(model_with_frame, model);
    model_with_frame.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame {
        frame,
    });
    assert_ne!(model_with_frame, model);
    let snapshot = render_streaming_snapshot(&model_with_frame);
    assert!(snapshot.contains("[pending text:"), "pending text should appear");
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
    assert_eq!(model.transcript().len(), 2);
}

#[test]
fn streaming_snapshot_after_stream_shows_accumulated_content() {
    let call_id = animus_adapters::ModelStreamCallId::new("call-acc").unwrap();
    let mut model = ConversationRenderModel::new();

    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamStarted {
        call_id: call_id.clone(),
    });

    for (i, part) in ["Hello ", "world, ", "this is ", "streaming!"].iter().enumerate() {
        let frame = content_delta(
            call_id.as_str(),
            i.saturating_add(1) as u64,
            part,
        );
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame });
    }

    let before_snapshot = render_streaming_snapshot(&model);
    assert!(before_snapshot.contains("[pending text:"), "pending text before flush");

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
    let frame = content_delta("orphan-call", 1, "orphaned delta");
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame });
    let snapshot = render_streaming_snapshot(&model);
    assert!(snapshot.contains("(empty transcript)"));
}

#[test]
fn out_of_order_finish_does_not_corrupt_transcript() {
    let mut model = ConversationRenderModel::new();
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamEnded {
        call_id: animus_adapters::ModelStreamCallId::new("empty-finish").unwrap(),
        status: animus_adapters::ModelStreamFrameKind::Finish,
    });
    let snapshot = render_conversation_snapshot(&model);
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
    assert!((meta.fill_ratio() - 1.0).abs() < f64::EPSILON);
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