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
    turn: &crate::RuntimeLoopTurn,
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
        crate::AdapterRequest::Model { request } => dispatch_model_adapter(&request, adapters),
        crate::AdapterRequest::Tool { request } => dispatch_tool_adapter(&request, adapters),
    }
}

fn dispatch_model_adapter<Model, Tool>(
    request: &animus_adapters::ModelRequest,
    adapters: &mut super::RuntimeAdapters<'_, Model, Tool>,
) -> crate::AdapterOutcome
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    // r[impl agent_harness_runtime.model_dispatch.runtime_gate]
    // r[impl agent_harness_runtime.scoped_capabilities.exact_resolution]
    debug_assert!(!request.prompt().is_empty());
    let plan = crate::plan_model_dispatch(crate::ModelDispatchPlanRequest {
        request,
        required_capability: crate::RuntimeCapabilityDescriptorDraft::default_model(),
        registry: &adapters.capabilities,
        policy: crate::ModelDispatchPolicyDraft::compatibility_defaults(),
    });
    debug_assert_eq!(
        plan.request_contract_identity(),
        animus_adapters::MODEL_REQUEST_CONTRACT_IDENTITY
    );
    if !plan.is_admitted() {
        return crate::AdapterOutcome::ModelCall {
            outcome: failed_outcome(
                model_plan_failure_class(plan.issue()),
                model_plan_failure_message(plan.issue()),
            ),
        };
    }
    match adapters.model.as_deref_mut() {
        Some(model) => model.call(request).map_or_else(
            |error| crate::AdapterOutcome::ModelCall {
                outcome: failed_outcome(
                    animus_adapters::AdapterCallFailureClass::ExecutionFailure,
                    error.to_string().as_str(),
                ),
            },
            |response| crate::AdapterOutcome::ModelCall {
                outcome: animus_adapters::AdapterCallOutcome::completed(response),
            },
        ),
        None => missing_service(crate::AdapterComponent::Model),
    }
}

fn dispatch_tool_adapter<Model, Tool>(
    request: &animus_adapters::ToolRequest,
    adapters: &mut super::RuntimeAdapters<'_, Model, Tool>,
) -> crate::AdapterOutcome
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    // r[impl agent_harness_runtime.scoped_capabilities.exact_resolution]
    debug_assert!(!request.name().is_empty());
    let Some(required) = default_tool_capability() else {
        return missing_service(crate::AdapterComponent::Tool);
    };
    debug_assert!(!required.label().is_empty());
    if !matches!(
        adapters.capabilities.resolve_exact(&required),
        crate::RuntimeCapabilityResolution::Resolved { .. }
    ) {
        return missing_service(crate::AdapterComponent::Tool);
    }
    match adapters.tool.as_deref_mut() {
        Some(tool) => tool.call(request).map_or_else(
            |error| crate::AdapterOutcome::ToolCall {
                outcome: failed_outcome(
                    animus_adapters::AdapterCallFailureClass::ExecutionFailure,
                    error.to_string().as_str(),
                ),
            },
            |response| crate::AdapterOutcome::ToolCall {
                outcome: animus_adapters::AdapterCallOutcome::completed(response),
            },
        ),
        None => missing_service(crate::AdapterComponent::Tool),
    }
}

fn default_tool_capability() -> Option<crate::RuntimeCapabilityDescriptor> {
    crate::RuntimeCapabilityDescriptor::from_draft(
        crate::RuntimeCapabilityDescriptorDraft::default_tool(),
    )
    .ok()
}

fn missing_service(component: crate::AdapterComponent) -> crate::AdapterOutcome {
    match component {
        crate::AdapterComponent::Model => crate::AdapterOutcome::ModelCall {
            outcome: failed_outcome(animus_adapters::AdapterCallFailureClass::Absent, "absent"),
        },
        crate::AdapterComponent::Tool => crate::AdapterOutcome::ToolCall {
            outcome: failed_outcome(animus_adapters::AdapterCallFailureClass::Absent, "absent"),
        },
    }
}

fn model_plan_failure_message(issue: crate::ModelDispatchPlanIssue) -> &'static str {
    match issue {
        crate::ModelDispatchPlanIssue::MissingProvider => "absent",
        other => other.as_str(),
    }
}

fn model_plan_failure_class(
    issue: crate::ModelDispatchPlanIssue,
) -> animus_adapters::AdapterCallFailureClass {
    match issue {
        crate::ModelDispatchPlanIssue::MissingProvider => {
            animus_adapters::AdapterCallFailureClass::Absent
        }
        crate::ModelDispatchPlanIssue::DeniedCapability
        | crate::ModelDispatchPlanIssue::BudgetOverflow
        | crate::ModelDispatchPlanIssue::FallbackDenied => {
            animus_adapters::AdapterCallFailureClass::Denied
        }
        crate::ModelDispatchPlanIssue::Cancelled => {
            animus_adapters::AdapterCallFailureClass::Cancelled
        }
        crate::ModelDispatchPlanIssue::TimedOut => {
            animus_adapters::AdapterCallFailureClass::TimedOut
        }
        crate::ModelDispatchPlanIssue::None
        | crate::ModelDispatchPlanIssue::InvalidCapability
        | crate::ModelDispatchPlanIssue::MalformedRequestShape
        | crate::ModelDispatchPlanIssue::UnsupportedStreamPolicy
        | crate::ModelDispatchPlanIssue::MissingReceiptPolicy => {
            animus_adapters::AdapterCallFailureClass::MalformedOutput
        }
    }
}

fn failed_outcome<Response>(
    class: animus_adapters::AdapterCallFailureClass,
    message: &str,
) -> animus_adapters::AdapterCallOutcome<Response> {
    animus_adapters::AdapterCallOutcome::failed(
        class,
        animus_adapters::AdapterCallFailureMetadataDraft {
            message,
            correlation_id: None,
            receipt_reference: None,
        },
    )
    .unwrap_or_else(|_error| fallback_failed_outcome(class))
}

fn fallback_failed_outcome<Response>(
    class: animus_adapters::AdapterCallFailureClass,
) -> animus_adapters::AdapterCallOutcome<Response> {
    animus_adapters::AdapterCallOutcome::fallback_failed(class)
}
