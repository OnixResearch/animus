#[test]
fn render_model_reduces_tool_events_to_snapshot() {
    // r[verify agent_harness_runtime.ratatui_tool_host.render_model.test_backend]
    let request = tool_request();
    let run_id = crate::ToolRunId::new(RUN_ID).expect("run ID should validate");
    let approval = approval_request(&[
        crate::ToolCommandChoice::Approve,
        crate::ToolCommandChoice::Deny,
    ]);
    let progress = crate::ToolProgress::new(COMPLETED_UNITS, Some(TOTAL_UNITS))
        .expect("progress should validate");
    let output = bounded_output();
    let mut model = crate::ToolRenderModel::new();

    model.apply_tool_event(&crate::ToolHostEvent::Started { run_id, request });
    model.apply_tool_event(&crate::ToolHostEvent::Progress {
        run_id: crate::ToolRunId::new(RUN_ID).expect("run ID should validate"),
        progress,
    });
    model.apply_tool_event(&crate::ToolHostEvent::Output {
        run_id: crate::ToolRunId::new(RUN_ID).expect("run ID should validate"),
        output,
    });
    model.apply_tool_event(&crate::ToolHostEvent::ApprovalRequested { request: approval });
    let snapshot = crate::render_text_snapshot(&model);

    assert!(snapshot.contains(SNAPSHOT_WAITING));
    assert!(snapshot.contains(SNAPSHOT_TOOL));
    assert!(snapshot.contains(SNAPSHOT_PROGRESS));
    assert!(snapshot.contains(SNAPSHOT_OUTPUT));
    assert!(snapshot.contains(SNAPSHOT_APPROVAL));
    assert!(snapshot.contains(SNAPSHOT_CHOICES));
}

#[test]
fn runtime_events_reduce_to_tool_snapshot() {
    // r[verify agent_harness_runtime.ratatui_tool_host.render_model]
    let request = tool_request();
    let response = tool_response();
    let mut model = crate::ToolRenderModel::new();

    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ToolRequested { request });
    let running_snapshot = crate::render_text_snapshot(&model);
    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ToolResponded { response });
    let final_snapshot = crate::render_text_snapshot(&model);

    assert!(running_snapshot.contains(SNAPSHOT_RUNNING));
    assert!(running_snapshot.contains(SNAPSHOT_TOOL));
    assert!(final_snapshot.contains(SNAPSHOT_FINAL));
}

#[test]
fn runtime_denial_renders_failure_without_terminal_backend() {
    // r[verify agent_harness_runtime.ratatui_tool_host.render_model.test_backend]
    let request = tool_request();
    let mut model = crate::ToolRenderModel::new();

    model.apply_runtime_event(&animus_runtime::RuntimeEvent::ToolDenied {
        request,
        reason: String::from(DENIED_REASON),
    });
    let snapshot = crate::render_text_snapshot(&model);

    assert!(snapshot.contains(SNAPSHOT_FAILURE_DENIED));
}
