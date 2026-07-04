#[test]
fn approval_request_accepts_finite_choices() {
    // r[verify agent_harness_runtime.ratatui_tool_host.commands]
    let approval = approval_request(&[
        crate::ToolCommandChoice::Approve,
        crate::ToolCommandChoice::Deny,
    ]);

    assert_eq!(approval.run_id().as_str(), RUN_ID);
    assert_eq!(approval.correlation_id().as_str(), CORRELATION_ID);
    assert_eq!(approval.prompt(), APPROVAL_PROMPT);
    assert_eq!(approval.choices().len(), expected_choice_count());
}

#[test]
fn invalid_approval_fixtures_fail_closed() {
    // r[verify agent_harness_runtime.ratatui_tool_host.failure_isolation]
    let bad_run_id = crate::ToolRunId::new(BAD_NUL).expect_err("NUL run IDs must fail");
    let run_id = crate::ToolRunId::new(RUN_ID).expect("run ID should validate");
    let correlation_id =
        crate::ToolCorrelationId::new(CORRELATION_ID).expect("correlation should validate");
    let empty_choices =
        crate::ApprovalRequest::new(run_id, correlation_id, APPROVAL_PROMPT, &[])
            .expect_err("empty choices must fail");
    let invalid_progress = crate::ToolProgress::new(TOO_MANY_COMPLETED_UNITS, Some(TOTAL_UNITS))
        .expect_err("completed progress above total must fail");

    assert_eq!(
        bad_run_id.to_string(),
        "tool-host field run_id is invalid: contains_nul"
    );
    assert_eq!(
        empty_choices.to_string(),
        "tool-host field choices must list finite choices"
    );
    assert_eq!(
        invalid_progress.to_string(),
        "tool-host field progress_total is invalid: completed_exceeds_total"
    );
}

#[test]
fn simple_tool_runs_without_tui_services() {
    // r[verify agent_harness_runtime.ratatui_tool_host.adapter_compat.simple]
    let request = tool_request();
    let mut tool = CountingTool::default();

    let response = crate::run_simple_tool(&mut tool, &request).expect("simple tool should run");

    assert_eq!(response.output(), TOOL_OUTPUT);
    assert_eq!(tool.calls, expected_call_count());
}
