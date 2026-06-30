#[derive(Debug, Eq, PartialEq)]
pub(super) enum Step {
    Continue { state: crate::TurnState },
    Complete { report: crate::RunReport },
}

pub(super) fn run_planned_step<Fabric, Model, Tool>(
    config: crate::RuntimeConfig,
    state: crate::TurnState,
    fabric: &mut Fabric,
    adapters: &mut super::RuntimeAdapters<'_, Model, Tool>,
) -> Step
where
    Fabric: crate::LoopFabric,
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    match crate::plan_next_turn(config, state) {
        crate::TurnPlan::Evaluate { state: ready, turn } => {
            run_ready_turn(ready, &turn, fabric, adapters)
        }
        crate::TurnPlan::Complete { report } => Step::Complete { report },
    }
}

fn run_ready_turn<Fabric, Model, Tool>(
    state: crate::TurnState,
    turn: &animus_steel::LoopTurn,
    fabric: &mut Fabric,
    adapters: &mut super::RuntimeAdapters<'_, Model, Tool>,
) -> Step
where
    Fabric: crate::LoopFabric,
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    let decision = match fabric.decide(turn) {
        Ok(decision) => decision,
        Err(error) => {
            return Step::Complete {
                report: crate::complete_with_fabric_error(state, error.to_string()),
            };
        }
    };

    apply_decision_step(crate::apply_decision(state, decision), adapters)
}

fn apply_decision_step<Model, Tool>(
    step: crate::DecisionStep,
    adapters: &mut super::RuntimeAdapters<'_, Model, Tool>,
) -> Step
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    match step {
        crate::DecisionStep::Dispatch { state, request } => {
            apply_dispatch_step(prepare_dispatch(state, request, adapters), adapters)
        }
        crate::DecisionStep::Complete { report } => Step::Complete { report },
    }
}

enum DispatchStep {
    Dispatch {
        state: crate::TurnState,
        request: crate::AdapterRequest,
    },
    Complete {
        report: crate::RunReport,
    },
}

fn apply_dispatch_step<Model, Tool>(
    step: DispatchStep,
    adapters: &mut super::RuntimeAdapters<'_, Model, Tool>,
) -> Step
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    match step {
        DispatchStep::Dispatch { state, request } => apply_adapter_step(
            crate::apply_adapter_outcome(state, dispatch_adapter(request, adapters)),
        ),
        DispatchStep::Complete { report } => Step::Complete { report },
    }
}

fn prepare_dispatch<Model, Tool>(
    state: crate::TurnState,
    request: crate::AdapterRequest,
    adapters: &mut super::RuntimeAdapters<'_, Model, Tool>,
) -> DispatchStep {
    match request {
        crate::AdapterRequest::Model { request } => {
            prepare_model_dispatch(state, request, &mut adapters.middleware)
        }
        crate::AdapterRequest::Tool { request } => {
            prepare_tool_dispatch(state, request, &mut adapters.middleware)
        }
    }
}

fn prepare_model_dispatch(
    state: crate::TurnState,
    request: animus_adapters::ModelRequest,
    middleware: &mut crate::RuntimeMiddlewareStack<'_>,
) -> DispatchStep {
    match middleware.prepare_model_dispatch(request) {
        crate::hooks::chain::MiddlewareOutcome::Accepted(request) => DispatchStep::Dispatch {
            state,
            request: crate::AdapterRequest::Model { request },
        },
        crate::hooks::chain::MiddlewareOutcome::Blocked {
            value,
            hook,
            reason,
        } => DispatchStep::Complete {
            report: crate::complete_with_guard_block(
                state,
                crate::AdapterRequest::Model { request: value },
                hook,
                reason,
            ),
        },
        crate::hooks::chain::MiddlewareOutcome::Failed { value, error } => DispatchStep::Complete {
            report: crate::complete_with_guard_failure(
                state,
                crate::AdapterRequest::Model { request: value },
                &error,
            ),
        },
    }
}

fn prepare_tool_dispatch(
    state: crate::TurnState,
    request: animus_adapters::ToolRequest,
    middleware: &mut crate::RuntimeMiddlewareStack<'_>,
) -> DispatchStep {
    match middleware.prepare_tool_dispatch(request) {
        crate::hooks::chain::MiddlewareOutcome::Accepted(request) => DispatchStep::Dispatch {
            state,
            request: crate::AdapterRequest::Tool { request },
        },
        crate::hooks::chain::MiddlewareOutcome::Blocked {
            value,
            hook,
            reason,
        } => DispatchStep::Complete {
            report: crate::complete_with_guard_block(
                state,
                crate::AdapterRequest::Tool { request: value },
                hook,
                reason,
            ),
        },
        crate::hooks::chain::MiddlewareOutcome::Failed { value, error } => DispatchStep::Complete {
            report: crate::complete_with_guard_failure(
                state,
                crate::AdapterRequest::Tool { request: value },
                &error,
            ),
        },
    }
}

fn apply_adapter_step(step: crate::AdapterStep) -> Step {
    match step {
        crate::AdapterStep::Continue { state } => Step::Continue { state },
        crate::AdapterStep::Complete { report } => Step::Complete { report },
    }
}

fn dispatch_adapter<Model, Tool>(
    request: crate::AdapterRequest,
    adapters: &mut super::RuntimeAdapters<'_, Model, Tool>,
) -> crate::AdapterOutcome
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    match request {
        crate::AdapterRequest::Model { request } => match adapters.model.as_deref_mut() {
            Some(model) => model.call(&request).map_or_else(
                |error| crate::AdapterOutcome::Err {
                    component: crate::AdapterComponent::Model,
                    message: error.to_string(),
                },
                |response| crate::AdapterOutcome::ModelOk { response },
            ),
            None => missing_service(crate::AdapterComponent::Model),
        },
        crate::AdapterRequest::Tool { request } => match adapters.tool.as_deref_mut() {
            Some(tool) => tool.call(&request).map_or_else(
                |error| crate::AdapterOutcome::Err {
                    component: crate::AdapterComponent::Tool,
                    message: error.to_string(),
                },
                |response| crate::AdapterOutcome::ToolOk { response },
            ),
            None => missing_service(crate::AdapterComponent::Tool),
        },
    }
}

fn missing_service(component: crate::AdapterComponent) -> crate::AdapterOutcome {
    crate::AdapterOutcome::Blocked {
        component,
        outcome: animus_adapters::OutcomeKind::Absent,
    }
}
