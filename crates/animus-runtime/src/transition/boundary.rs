/// Adapter result normalized for pure transition handling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterOutcome {
    /// Model adapter returned a validated response.
    ModelOk {
        /// Validated model response.
        response: animus_adapters::ModelResponse,
    },
    /// Tool adapter returned a validated response.
    ToolOk {
        /// Validated tool response.
        response: animus_adapters::ToolResponse,
    },
    /// Adapter service failed closed with a typed non-completed outcome.
    Blocked {
        /// Adapter component that was blocked.
        component: crate::AdapterComponent,
        /// Non-completed service outcome.
        outcome: animus_adapters::OutcomeKind,
    },
    /// Adapter returned an error.
    Err {
        /// Adapter component that failed.
        component: crate::AdapterComponent,
        /// Human-readable adapter error message.
        message: String,
    },
}

/// Step returned after applying an adapter outcome to pure state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterStep {
    /// Continue to the next fabric turn.
    Continue {
        /// Updated turn state.
        state: super::TurnState,
    },
    /// Runtime execution has completed.
    Complete {
        /// Final runtime report.
        report: crate::RunReport,
    },
}

/// Applies an adapter outcome to pure state.
#[must_use]
pub fn apply_adapter_outcome(state: super::TurnState, outcome: AdapterOutcome) -> AdapterStep {
    match outcome {
        AdapterOutcome::ModelOk { response } => model_success(state, response),
        AdapterOutcome::ToolOk { response } => tool_success(state, response),
        AdapterOutcome::Blocked { component, outcome } => {
            adapter_blocked(state, component, outcome)
        }
        AdapterOutcome::Err { component, message } => adapter_failure(state, component, message),
    }
}

fn model_success(
    mut state: super::TurnState,
    response: animus_adapters::ModelResponse,
) -> AdapterStep {
    state.turns_completed = state.turns_completed.saturating_add(1);
    state.last_model_message = Some(String::from(response.message()));
    state
        .events
        .push(crate::RuntimeEvent::ModelResponded { response });
    AdapterStep::Continue { state }
}

fn tool_success(
    mut state: super::TurnState,
    response: animus_adapters::ToolResponse,
) -> AdapterStep {
    state.turns_completed = state.turns_completed.saturating_add(1);
    state.last_tool_output = Some(String::from(response.output()));
    state
        .events
        .push(crate::RuntimeEvent::ToolResponded { response });
    AdapterStep::Continue { state }
}

fn adapter_blocked(
    state: super::TurnState,
    component: crate::AdapterComponent,
    outcome: animus_adapters::OutcomeKind,
) -> AdapterStep {
    adapter_failure(state, component, String::from(outcome.as_str()))
}

fn adapter_failure(
    state: super::TurnState,
    component: crate::AdapterComponent,
    message: String,
) -> AdapterStep {
    let failure = match component {
        crate::AdapterComponent::Model => crate::RuntimeFailure::ModelAdapter {
            message: message.clone(),
        },
        crate::AdapterComponent::Tool => crate::RuntimeFailure::ToolAdapter {
            message: message.clone(),
        },
    };
    let state = state.with_event(crate::RuntimeEvent::AdapterError { component, message });
    AdapterStep::Complete {
        report: state.complete(crate::RuntimeStatus::Failed { failure }),
    }
}
