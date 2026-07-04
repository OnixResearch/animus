fn finish_tool<Sink>(
    run_id: &crate::ToolRunId,
    response: animus_adapters::ToolResponse,
    sink: &mut Sink,
) -> HostedToolOutcome
where
    Sink: ToolHostEventSink,
    Sink::Error: core::fmt::Display,
{
    if emit_event(
        sink,
        crate::ToolHostEvent::Finished {
            run_id: run_id.clone(),
            response: response.clone(),
        },
    )
    .is_err()
    {
        return failed(
            crate::ToolHostFailureClass::DroppedConsumer,
            EVENT_CONSUMER_UNAVAILABLE,
        );
    }
    HostedToolOutcome::Completed { response }
}

fn fail_adapter<Sink>(
    run_id: &crate::ToolRunId,
    error: &impl core::fmt::Display,
    sink: &mut Sink,
) -> HostedToolOutcome
where
    Sink: ToolHostEventSink,
{
    let failure = display_failure(crate::ToolHostFailureClass::Adapter, error, ADAPTER_FAILED);
    let _ignored = emit_event(
        sink,
        crate::ToolHostEvent::Failed {
            run_id: run_id.clone(),
            failure: failure.clone(),
        },
    );
    HostedToolOutcome::Failed { failure }
}

fn emit_failure<Sink>(run_id: &crate::ToolRunId, failure: &crate::ToolHostFailure, sink: &mut Sink)
where
    Sink: ToolHostEventSink,
{
    let _ignored = sink.emit(crate::ToolHostEvent::Failed {
        run_id: run_id.clone(),
        failure: failure.clone(),
    });
}

fn emit_event<Sink>(sink: &mut Sink, event: crate::ToolHostEvent) -> Result<(), Sink::Error>
where
    Sink: ToolHostEventSink,
{
    sink.emit(event)
}

fn failed(class: crate::ToolHostFailureClass, reason: &'static str) -> HostedToolOutcome {
    HostedToolOutcome::Failed {
        failure: static_failure(class, reason),
    }
}

fn static_failure(
    class: crate::ToolHostFailureClass,
    reason: &'static str,
) -> crate::ToolHostFailure {
    crate::ToolHostFailure::from_static_reason(class, reason)
}

fn display_failure(
    class: crate::ToolHostFailureClass,
    error: &impl core::fmt::Display,
    fallback: &'static str,
) -> crate::ToolHostFailure {
    crate::ToolHostFailure::new(class, &error.to_string())
        .unwrap_or_else(|_validation_error| static_failure(class, fallback))
}
