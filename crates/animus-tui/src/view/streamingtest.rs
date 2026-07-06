// Streaming markdown view tests.
// Included from view.rs inside `mod streaming_tests { use super::*; include!(...); }`

// -----------------------------------------------------------------------
// Image display tests
// -----------------------------------------------------------------------

#[test]
fn image_display_from_handle_bounds_description() {
    let long_desc = "a".repeat(512);
    let handle = img_handle("image/png");
    let display = ImageDisplay::from_handle(&handle, Some(&long_desc), "fallback");
    assert_eq!(
        display.description().len(),
        usize::try_from(MAX_IMAGE_DESCRIPTION_CHARS).unwrap_or(256)
    );
    assert!(display.description().len() < long_desc.len());
}

#[test]
fn image_display_without_description_uses_fallback() {
    let handle = img_handle("image/png");
    let display = ImageDisplay::from_handle(&handle, None, "default image");
    assert_eq!(display.description(), "default image");
    assert_eq!(display.media_type(), "image/png");
}

#[test]
fn image_display_rejects_raw_provider_bytes() {
    let handle = img_handle("image/png");
    let display = ImageDisplay::from_handle(&handle, None, "image");
    assert!(!display.description().contains("safe_abc"), "safe handle leaked");
}

#[test]
fn image_display_stable_media_type_from_enum() {
    let handle = img_handle("image/jpeg");
    let display = ImageDisplay::from_handle(&handle, None, "image");
    assert_eq!(display.media_type(), "image/jpeg");
    assert!(!display.is_bounded(), "enum values are bounded by definition");
}

// -----------------------------------------------------------------------
// Streaming snapshot tests
// -----------------------------------------------------------------------

#[test]
fn empty_streaming_snapshot_shows_empty_transcript() {
    let model = fresh_model();
    let snapshot = render_streaming_snapshot(&model);
    assert!(snapshot.contains("(empty transcript)"));
    assert!(snapshot.contains("total entries: 0, evicted: 0"));
}

#[test]
fn streaming_snapshot_shows_pending_text_when_buffering() {
    let model = fresh_model();
    let cid = call_id("call-pending");
    let frame = delta_frame(cid.as_str(), 1, "accumulating content...");
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
    let mut model = sized_model(2);
    for i in 0..5 {
        let content = format!("Message {i}");
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
            response: msg_response(&content),
        });
    }
    let snapshot = render_streaming_snapshot(&model);
    assert!(snapshot.contains("total entries: 5, evicted: 3"));
    assert_eq!(model.transcript().len(), 2);
}

#[test]
fn streaming_snapshot_after_stream_shows_accumulated_content() {
    let cid = call_id("call-acc");
    let mut model = fresh_model();

    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamStarted {
        call_id: cid.clone(),
    });

    for (i, part) in ["Hello ", "world, ", "this is ", "streaming!"].iter().enumerate() {
        let frame = delta_frame(
            cid.as_str(),
            i.saturating_add(1) as u64,
            part,
        );
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame });
    }

    let before_snapshot = render_streaming_snapshot(&model);
    assert!(before_snapshot.contains("[pending text:"), "pending text before flush");

    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamEnded {
        call_id: cid,
        status: finish_kind(),
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
    let mut model = fresh_model();
    let frame = delta_frame("orphan-call", 1, "orphaned delta");
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame });
    let snapshot = render_streaming_snapshot(&model);
    assert!(snapshot.contains("(empty transcript)"));
}

#[test]
fn out_of_order_finish_does_not_corrupt_transcript() {
    let mut model = fresh_model();
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamEnded {
        call_id: call_id("empty-finish"),
        status: finish_kind(),
    });
    let snapshot = render_conversation_snapshot(&model);
    assert!(snapshot.contains("(empty transcript)"));
}

// -----------------------------------------------------------------------
// Scrollback metadata tests
// -----------------------------------------------------------------------

#[test]
fn scrollback_metadata_empty_window() {
    let model = fresh_model();
    let meta = ScrollbackMetadata::from_model(&model);
    assert_eq!(meta.visible_count, 0);
    assert_eq!(meta.evicted_count, 0);
    assert!(!meta.has_evicted());
    assert!((meta.fill_ratio() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn scrollback_metadata_tracks_eviction() {
    let mut model = sized_model(2);
    for i in 0..5 {
        let content = format!("Message {i}");
        model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
            response: msg_response(&content),
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
    let mut model = sized_model(5);
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
        response: msg_response("single"),
    });
    let meta = ScrollbackMetadata::from_model(&model);
    assert_eq!(meta.visible_count, 1);
    assert!((meta.fill_ratio() - 0.2).abs() < f64::EPSILON);
}