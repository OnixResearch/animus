const NO_PLANNING_STEPS_REMAINING: usize = 0;
const PLANNING_GUARD_EXHAUSTED: &str = "runtime planning step guard exhausted";

/// Borrowed model/tool adapter bundle used by runtime runners.
pub struct RuntimeAdapters<'a, Model, Tool> {
    model: &'a mut Model,
    tool: &'a mut Tool,
}

impl<'a, Model, Tool> RuntimeAdapters<'a, Model, Tool> {
    /// Builds a borrowed adapter bundle.
    #[must_use]
    pub fn new(model: &'a mut Model, tool: &'a mut Tool) -> Self {
        Self { model, tool }
    }
}

#[derive(Debug, Eq, PartialEq)]
enum RunStep {
    Continue { state: crate::TurnState },
    Complete { report: crate::RunReport },
}

/// Runs the agent loop with caller-supplied fabric and adapters.
#[must_use]
pub fn run_agent_loop<Fabric, Model, Tool>(
    input: &crate::RuntimeInput,
    config: crate::RuntimeConfig,
    fabric: &mut Fabric,
    mut adapters: RuntimeAdapters<'_, Model, Tool>,
) -> crate::RunReport
where
    Fabric: crate::LoopFabric,
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    let mut state = crate::TurnState::new(input);
    let mut planning_steps_remaining = config
        .max_turns()
        .count()
        .saturating_add(crate::MIN_MAX_TURNS.count());

    while planning_steps_remaining != NO_PLANNING_STEPS_REMAINING {
        planning_steps_remaining =
            planning_steps_remaining.saturating_sub(crate::MIN_MAX_TURNS.count());
        match run_planned_step(config, state, fabric, &mut adapters) {
            RunStep::Continue { state: next_state } => state = next_state,
            RunStep::Complete { report } => return report,
        }
    }

    crate::complete_with_fabric_error(state, String::from(PLANNING_GUARD_EXHAUSTED))
}

/// Runs the agent loop with a Steel program as the default loop fabric.
#[must_use]
pub fn run_with_steel_program<Model, Tool>(
    program: &str,
    input: &crate::RuntimeInput,
    config: crate::RuntimeConfig,
    adapters: RuntimeAdapters<'_, Model, Tool>,
) -> crate::RunReport
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    match animus_steel::SteelLoopFabric::new(program) {
        Ok(mut fabric) => run_agent_loop(input, config, &mut fabric, adapters),
        Err(error) => {
            crate::complete_with_fabric_error(crate::TurnState::new(input), error.to_string())
        }
    }
}

fn run_planned_step<Fabric, Model, Tool>(
    config: crate::RuntimeConfig,
    state: crate::TurnState,
    fabric: &mut Fabric,
    adapters: &mut RuntimeAdapters<'_, Model, Tool>,
) -> RunStep
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
        crate::TurnPlan::Complete { report } => RunStep::Complete { report },
    }
}

fn run_ready_turn<Fabric, Model, Tool>(
    state: crate::TurnState,
    turn: &animus_steel::LoopTurn,
    fabric: &mut Fabric,
    adapters: &mut RuntimeAdapters<'_, Model, Tool>,
) -> RunStep
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
            return RunStep::Complete {
                report: crate::complete_with_fabric_error(state, error.to_string()),
            };
        }
    };

    apply_decision_step(crate::apply_decision(state, decision), adapters)
}

fn apply_decision_step<Model, Tool>(
    step: crate::DecisionStep,
    adapters: &mut RuntimeAdapters<'_, Model, Tool>,
) -> RunStep
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    match step {
        crate::DecisionStep::Dispatch { state, request } => apply_adapter_step(
            crate::apply_adapter_outcome(state, dispatch_adapter(request, adapters)),
        ),
        crate::DecisionStep::Complete { report } => RunStep::Complete { report },
    }
}

fn apply_adapter_step(step: crate::AdapterStep) -> RunStep {
    match step {
        crate::AdapterStep::Continue { state } => RunStep::Continue { state },
        crate::AdapterStep::Complete { report } => RunStep::Complete { report },
    }
}

fn dispatch_adapter<Model, Tool>(
    request: crate::AdapterRequest,
    adapters: &mut RuntimeAdapters<'_, Model, Tool>,
) -> crate::AdapterOutcome
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    match request {
        crate::AdapterRequest::Model { request } => adapters.model.call(&request).map_or_else(
            |error| crate::AdapterOutcome::Err {
                component: crate::AdapterComponent::Model,
                message: error.to_string(),
            },
            |response| crate::AdapterOutcome::ModelOk { response },
        ),
        crate::AdapterRequest::Tool { request } => adapters.tool.call(&request).map_or_else(
            |error| crate::AdapterOutcome::Err {
                component: crate::AdapterComponent::Tool,
                message: error.to_string(),
            },
            |response| crate::AdapterOutcome::ToolOk { response },
        ),
    }
}

#[cfg(test)]
mod tests;
