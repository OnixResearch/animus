/// Runtime plan before evaluating the next loop-fabric turn.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TurnPlan {
    /// Evaluate the loop fabric with this turn input.
    Evaluate {
        /// State that will consume the fabric decision.
        state: TurnState,
        /// Typed loop-fabric turn input.
        turn: animus_steel::LoopTurn,
    },
    /// Runtime execution has completed.
    Complete {
        /// Final runtime report.
        report: crate::RunReport,
    },
}

/// Pending adapter request selected by a validated loop decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterRequest {
    /// Model adapter request.
    Model {
        /// Validated model request.
        request: animus_adapters::ModelRequest,
    },
    /// Tool adapter request.
    Tool {
        /// Validated tool request.
        request: animus_adapters::ToolRequest,
    },
}

/// Step returned after applying a loop decision to pure state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DecisionStep {
    /// A model or tool adapter must be called by the imperative shell.
    Dispatch {
        /// State after recording the requested adapter event.
        state: TurnState,
        /// Adapter request to dispatch.
        request: AdapterRequest,
    },
    /// Runtime execution has completed.
    Complete {
        /// Final runtime report.
        report: crate::RunReport,
    },
}

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
        state: TurnState,
    },
    /// Runtime execution has completed.
    Complete {
        /// Final runtime report.
        report: crate::RunReport,
    },
}

/// Pure deterministic runtime-loop state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TurnState {
    summary: String,
    turns_completed: usize,
    last_model_message: Option<String>,
    last_tool_output: Option<String>,
    events: Vec<crate::RuntimeEvent>,
}

impl TurnState {
    /// Builds initial turn state from validated runtime input.
    #[must_use]
    pub fn new(input: &crate::RuntimeInput) -> Self {
        Self {
            summary: String::from(input.summary()),
            turns_completed: 0,
            last_model_message: None,
            last_tool_output: None,
            events: vec![crate::RuntimeEvent::LoopStarted {
                summary: String::from(input.summary()),
            }],
        }
    }

    /// Returns the input summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        self.summary.as_str()
    }

    /// Returns the number of completed fabric decisions.
    #[must_use]
    const fn turns_completed(&self) -> usize {
        self.turns_completed
    }

    /// Returns the last model message, if any.
    #[must_use]
    pub fn last_model_message(&self) -> Option<&str> {
        self.last_model_message.as_deref()
    }

    /// Returns the last tool output, if any.
    #[must_use]
    pub fn last_tool_output(&self) -> Option<&str> {
        self.last_tool_output.as_deref()
    }

    /// Returns ordered runtime events accumulated so far.
    #[must_use]
    pub fn events(&self) -> &[crate::RuntimeEvent] {
        self.events.as_slice()
    }

    fn next_turn(&self) -> Result<animus_steel::LoopTurn, crate::RuntimeError> {
        animus_steel::LoopTurn::new(
            self.summary(),
            self.last_model_message(),
            self.last_tool_output(),
        )
        .map_err(|source| crate::RuntimeError::LoopTurn {
            message: source.to_string(),
        })
    }

    fn complete(self, status: crate::RuntimeStatus) -> crate::RunReport {
        crate::RunReport::new(self.events, status)
    }

    fn with_event(mut self, event: crate::RuntimeEvent) -> Self {
        self.events.push(event);
        self
    }
}

/// Plans the next fabric turn or max-turn completion without side effects.
#[must_use]
pub fn plan_next_turn(config: crate::RuntimeConfig, state: TurnState) -> TurnPlan {
    if state.turns_completed() >= config.max_turns().count() {
        let max_turns = config.max_turns().count();
        let state = state.with_event(crate::RuntimeEvent::MaxTurnsExceeded { max_turns });
        return TurnPlan::Complete {
            report: state.complete(crate::RuntimeStatus::MaxTurnsExceeded { max_turns }),
        };
    }

    match state.next_turn() {
        Ok(turn) => TurnPlan::Evaluate { state, turn },
        Err(error) => TurnPlan::Complete {
            report: complete_with_fabric_error(state, error.to_string()),
        },
    }
}

/// Applies a validated loop decision to pure state.
#[must_use]
pub fn apply_decision(state: TurnState, decision: animus_steel::LoopDecision) -> DecisionStep {
    match decision {
        animus_steel::LoopDecision::CallModel { request } => {
            let state = state.with_event(crate::RuntimeEvent::ModelRequested {
                request: request.clone(),
            });
            DecisionStep::Dispatch {
                state,
                request: AdapterRequest::Model { request },
            }
        }
        animus_steel::LoopDecision::CallTool { request } => {
            let state = state.with_event(crate::RuntimeEvent::ToolRequested {
                request: request.clone(),
            });
            DecisionStep::Dispatch {
                state,
                request: AdapterRequest::Tool { request },
            }
        }
        animus_steel::LoopDecision::Respond { response } => {
            let state = state.with_event(crate::RuntimeEvent::FinalResponse {
                response: response.clone(),
            });
            DecisionStep::Complete {
                report: state.complete(crate::RuntimeStatus::FinalResponse { response }),
            }
        }
        animus_steel::LoopDecision::Stop => {
            let state = state.with_event(crate::RuntimeEvent::Stopped);
            DecisionStep::Complete {
                report: state.complete(crate::RuntimeStatus::Stopped),
            }
        }
    }
}

/// Applies an adapter outcome to pure state.
#[must_use]
pub fn apply_adapter_outcome(state: TurnState, outcome: AdapterOutcome) -> AdapterStep {
    match outcome {
        AdapterOutcome::ModelOk { response } => model_success(state, response),
        AdapterOutcome::ToolOk { response } => tool_success(state, response),
        AdapterOutcome::Err { component, message } => adapter_failure(state, component, message),
    }
}

/// Completes a runtime report after a fabric failure.
#[must_use]
pub fn complete_with_fabric_error(state: TurnState, message: String) -> crate::RunReport {
    let state = state.with_event(crate::RuntimeEvent::FabricError {
        message: message.clone(),
    });
    state.complete(crate::RuntimeStatus::Failed {
        failure: crate::RuntimeFailure::Fabric { message },
    })
}

fn model_success(mut state: TurnState, response: animus_adapters::ModelResponse) -> AdapterStep {
    state.turns_completed = state.turns_completed.saturating_add(1);
    state.last_model_message = Some(String::from(response.message()));
    state
        .events
        .push(crate::RuntimeEvent::ModelResponded { response });
    AdapterStep::Continue { state }
}

fn tool_success(mut state: TurnState, response: animus_adapters::ToolResponse) -> AdapterStep {
    state.turns_completed = state.turns_completed.saturating_add(1);
    state.last_tool_output = Some(String::from(response.output()));
    state
        .events
        .push(crate::RuntimeEvent::ToolResponded { response });
    AdapterStep::Continue { state }
}

fn adapter_failure(
    state: TurnState,
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
