#[test]
fn hosted_tool_emits_lifecycle_events() {
    // r[verify agent_harness_runtime.ratatui_tool_host.bridge.progress]
    let request = tool_request();
    let run_id = crate::ToolRunId::new(RUN_ID).expect("run ID should validate");
    let mut tool = CountingTool::default();
    let mut sink = RecordingSink::default();

    let outcome = crate::run_hosted_tool(&run_id, &mut tool, &request, &mut sink);

    assert!(outcome.is_completed());
    assert_eq!(tool.calls, expected_call_count());
    assert!(matches!(
        sink.events.first(),
        Some(crate::ToolHostEvent::Started { .. })
    ));
    assert!(matches!(
        sink.events.last(),
        Some(crate::ToolHostEvent::Finished { .. })
    ));
}

#[test]
fn denied_guarded_tool_does_not_execute_adapter() {
    // r[verify agent_harness_runtime.ratatui_tool_host.commands.denied]
    let request = tool_request();
    let approval = approval_request(&[
        crate::ToolCommandChoice::Approve,
        crate::ToolCommandChoice::Deny,
    ]);
    let mut tool = CountingTool::default();
    let mut sink = RecordingSink::default();
    let mut commands = StaticCommandSource::new(crate::ToolHostCommand::Deny {
        correlation_id: crate::ToolCorrelationId::new(CORRELATION_ID)
            .expect("correlation should validate"),
    });

    let outcome = crate::run_guarded_tool(crate::GuardedToolRun {
        approval: &approval,
        tool: &mut tool,
        request: &request,
        sink: &mut sink,
        commands: &mut commands,
    });

    assert!(matches!(outcome, crate::HostedToolOutcome::Denied { .. }));
    assert_eq!(tool.calls, no_call_count());
    assert!(matches!(
        sink.events.last(),
        Some(crate::ToolHostEvent::Failed { .. })
    ));
}

#[test]
fn mismatched_command_identity_fails_closed() {
    // r[verify agent_harness_runtime.ratatui_tool_host.failure_isolation.dropped_consumer]
    let request = tool_request();
    let approval = approval_request(&[
        crate::ToolCommandChoice::Approve,
        crate::ToolCommandChoice::Deny,
    ]);
    let mut tool = CountingTool::default();
    let mut sink = RecordingSink::default();
    let mut commands = StaticCommandSource::new(crate::ToolHostCommand::Approve {
        correlation_id: crate::ToolCorrelationId::new(MISMATCHED_CORRELATION_ID)
            .expect("correlation should validate"),
    });

    let outcome = crate::run_guarded_tool(crate::GuardedToolRun {
        approval: &approval,
        tool: &mut tool,
        request: &request,
        sink: &mut sink,
        commands: &mut commands,
    });

    assert_eq!(tool.calls, no_call_count());
    assert_eq!(
        outcome.failure().expect("failure expected").class(),
        crate::ToolHostFailureClass::InvalidCommand
    );
}

#[test]
fn dropped_consumer_fails_before_adapter_execution() {
    // r[verify agent_harness_runtime.ratatui_tool_host.failure_isolation.dropped_consumer]
    let request = tool_request();
    let run_id = crate::ToolRunId::new(RUN_ID).expect("run ID should validate");
    let mut tool = CountingTool::default();
    let mut sink = FailingSink;

    let outcome = crate::run_hosted_tool(&run_id, &mut tool, &request, &mut sink);

    assert_eq!(tool.calls, no_call_count());
    assert_eq!(
        outcome.failure().expect("failure expected").class(),
        crate::ToolHostFailureClass::DroppedConsumer
    );
}

#[test]
fn closed_command_source_fails_before_adapter_execution() {
    // r[verify agent_harness_runtime.ratatui_tool_host.failure_isolation.dropped_consumer]
    let request = tool_request();
    let approval = approval_request(&[
        crate::ToolCommandChoice::Approve,
        crate::ToolCommandChoice::Deny,
    ]);
    let mut tool = CountingTool::default();
    let mut sink = RecordingSink::default();
    let mut commands = FailingCommandSource;

    let outcome = crate::run_guarded_tool(crate::GuardedToolRun {
        approval: &approval,
        tool: &mut tool,
        request: &request,
        sink: &mut sink,
        commands: &mut commands,
    });

    assert_eq!(tool.calls, no_call_count());
    assert_eq!(
        outcome.failure().expect("failure expected").class(),
        crate::ToolHostFailureClass::ClosedCommandChannel
    );
}
