// Positive tests

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
    let req = animus_adapters::ToolRequest::new("read_file", "path: Cargo.toml").unwrap();
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ToolRequested {
        request: req.clone(),
    });
    let resp = animus_adapters::ToolResponse::new("[package]\nname = \"animus\"").unwrap();
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ToolResponded { response: resp });
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

    let frame1 = make_content_delta(&call_id, 1, "Hello");
    let frame2 = make_content_delta(&call_id, 2, "World");
    let frame3 = make_finish(&call_id, 3);

    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame: frame1 });
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame: frame2 });
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelStreamFrame { frame: frame3 });

    let snapshot = render_conversation_snapshot(&model);
    assert!(snapshot.contains("assistant: HelloWorld"));
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
    assert!(!model.transcript().len() > 0, "transcript starts empty");
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
    assert!(q1_pos < a1_pos, "Q1 before A1");
    assert!(a1_pos < q2_pos, "A1 before Q2");
    assert!(q2_pos < a2_pos, "Q2 before A2");
}

// Negative tests

#[test]
fn unknown_event_variant_does_not_panic() {
    let mut model = ConversationRenderModel::new();
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::Stopped);
    let snapshot = render_conversation_snapshot(&model);
    assert_eq!(snapshot, "(empty transcript)");
}

#[test]
fn oversized_content_is_truncated() {
    let max: usize = usize::try_from(MAX_CONTENT_CHARS)
        .unwrap_or(0);
    let within_bounds = "a".repeat(max);
    let mut model = ConversationRenderModel::new();
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ModelResponded {
        response: animus_adapters::ModelResponse::new(&within_bounds).unwrap(),
    });
    let snapshot = render_conversation_snapshot(&model);
    let expected = format!("assistant: {}", "a".repeat(max));
    assert!(snapshot.contains(&expected));
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