/// Executes a noninteractive tool through the existing simple adapter contract.
///
/// # Errors
///
/// Returns the wrapped adapter error when the adapter rejects or cannot satisfy the request.
pub fn run_simple_tool<Tool>(
    tool: &mut Tool,
    request: &animus_adapters::ToolRequest,
) -> Result<animus_adapters::ToolResponse, Tool::Error>
where
    Tool: animus_adapters::ToolAdapter,
{
    tool.call(request)
}

/// Executes a tool adapter while emitting UI-neutral lifecycle events.
#[must_use]
pub fn run_hosted_tool<Tool, Sink>(
    run_id: &crate::ToolRunId,
    tool: &mut Tool,
    request: &animus_adapters::ToolRequest,
    sink: &mut Sink,
) -> HostedToolOutcome
where
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
    Sink: ToolHostEventSink,
    Sink::Error: core::fmt::Display,
{
    assert!(!run_id.as_str().is_empty());
    assert!(!request.name().is_empty());
    if emit_started(run_id, request, sink).is_err() {
        return failed(
            crate::ToolHostFailureClass::DroppedConsumer,
            EVENT_CONSUMER_UNAVAILABLE,
        );
    }
    match run_simple_tool(tool, request) {
        Ok(response) => finish_tool(run_id, response, sink),
        Err(error) => fail_adapter(run_id, &error, sink),
    }
}

/// Executes a guarded tool only after a correlated host approval command allows it.
#[must_use]
pub fn run_guarded_tool<Tool, Sink, Commands>(
    run: GuardedToolRun<'_, Tool, Sink, Commands>,
) -> HostedToolOutcome
where
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
    Sink: ToolHostEventSink,
    Sink::Error: core::fmt::Display,
    Commands: ToolHostCommandSource,
    Commands::Error: core::fmt::Display,
{
    assert!(!run.approval.prompt().is_empty());
    assert!(!run.request.name().is_empty());
    if emit_approval(run.approval, run.sink).is_err() {
        return failed(
            crate::ToolHostFailureClass::DroppedConsumer,
            EVENT_CONSUMER_UNAVAILABLE,
        );
    }
    let command = match read_approval_command(run.approval, run.sink, run.commands) {
        Ok(command) => command,
        Err(outcome) => return outcome,
    };
    if let Err(outcome) = verify_command(run.approval, &command, run.sink) {
        return outcome;
    }
    dispatch_guarded_command(run, command)
}

fn emit_started<Sink>(
    run_id: &crate::ToolRunId,
    request: &animus_adapters::ToolRequest,
    sink: &mut Sink,
) -> Result<(), Sink::Error>
where
    Sink: ToolHostEventSink,
{
    emit_event(
        sink,
        crate::ToolHostEvent::Started {
            run_id: run_id.clone(),
            request: request.clone(),
        },
    )
}

fn emit_approval<Sink>(
    approval: &crate::ApprovalRequest,
    sink: &mut Sink,
) -> Result<(), Sink::Error>
where
    Sink: ToolHostEventSink,
{
    emit_event(
        sink,
        crate::ToolHostEvent::ApprovalRequested {
            request: approval.clone(),
        },
    )
}

fn read_approval_command<Sink, Commands>(
    approval: &crate::ApprovalRequest,
    sink: &mut Sink,
    commands: &mut Commands,
) -> Result<crate::ToolHostCommand, HostedToolOutcome>
where
    Sink: ToolHostEventSink,
    Commands: ToolHostCommandSource,
    Commands::Error: core::fmt::Display,
{
    match commands.decide(approval) {
        Ok(command) => Ok(command),
        Err(error) => {
            let failure = display_failure(
                crate::ToolHostFailureClass::ClosedCommandChannel,
                &error,
                COMMAND_CHANNEL_CLOSED,
            );
            emit_failure(approval.run_id(), &failure, sink);
            Err(HostedToolOutcome::Failed { failure })
        }
    }
}

fn verify_command<Sink>(
    approval: &crate::ApprovalRequest,
    command: &crate::ToolHostCommand,
    sink: &mut Sink,
) -> Result<(), HostedToolOutcome>
where
    Sink: ToolHostEventSink,
{
    if approval.allows_command(command) {
        return Ok(());
    }
    let failure = static_failure(crate::ToolHostFailureClass::InvalidCommand, INVALID_COMMAND);
    emit_failure(approval.run_id(), &failure, sink);
    Err(HostedToolOutcome::Failed { failure })
}

fn dispatch_guarded_command<Tool, Sink, Commands>(
    run: GuardedToolRun<'_, Tool, Sink, Commands>,
    command: crate::ToolHostCommand,
) -> HostedToolOutcome
where
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
    Sink: ToolHostEventSink,
    Sink::Error: core::fmt::Display,
{
    assert!(!run.approval.run_id().as_str().is_empty());
    assert!(!run.request.name().is_empty());
    if emit_observed_command(run.approval, &command, run.sink).is_err() {
        return failed(
            crate::ToolHostFailureClass::DroppedConsumer,
            EVENT_CONSUMER_UNAVAILABLE,
        );
    }
    match command.choice() {
        crate::ToolCommandChoice::Approve | crate::ToolCommandChoice::Continue => {
            run_hosted_tool(run.approval.run_id(), run.tool, run.request, run.sink)
        }
        crate::ToolCommandChoice::Deny => deny_guarded(run.approval, run.sink),
        crate::ToolCommandChoice::Cancel => cancel_guarded(run.approval, run.sink),
    }
}

fn emit_observed_command<Sink>(
    approval: &crate::ApprovalRequest,
    command: &crate::ToolHostCommand,
    sink: &mut Sink,
) -> Result<(), Sink::Error>
where
    Sink: ToolHostEventSink,
{
    emit_event(
        sink,
        crate::ToolHostEvent::CommandObserved {
            run_id: approval.run_id().clone(),
            command: command.clone(),
        },
    )
}

fn deny_guarded<Sink>(approval: &crate::ApprovalRequest, sink: &mut Sink) -> HostedToolOutcome
where
    Sink: ToolHostEventSink,
{
    let failure = static_failure(crate::ToolHostFailureClass::Denied, GUARDED_ACTION_DENIED);
    emit_failure(approval.run_id(), &failure, sink);
    HostedToolOutcome::Denied { failure }
}

fn cancel_guarded<Sink>(approval: &crate::ApprovalRequest, sink: &mut Sink) -> HostedToolOutcome
where
    Sink: ToolHostEventSink,
{
    let failure = static_failure(crate::ToolHostFailureClass::Cancelled, GUARDED_ACTION_CANCELLED);
    let _ignored = emit_event(
        sink,
        crate::ToolHostEvent::Cancelled {
            run_id: approval.run_id().clone(),
            reason: String::from(failure.reason()),
        },
    );
    HostedToolOutcome::Cancelled { failure }
}
