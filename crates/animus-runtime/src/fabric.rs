/// Swappable loop-fabric boundary used by the runtime shell.
pub trait LoopFabric {
    /// Error returned by the concrete loop fabric.
    type Error: core::fmt::Display;

    /// Produces one validated loop decision for the supplied turn input.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the fabric cannot produce a valid decision.
    fn decide(
        &mut self,
        turn: &crate::RuntimeLoopTurn,
    ) -> Result<crate::RuntimeLoopDecision, Self::Error>;
}

#[cfg(feature = "steel")]
impl LoopFabric for animus_steel::SteelLoopFabric {
    type Error = animus_steel::LoopError;

    fn decide(
        &mut self,
        turn: &crate::RuntimeLoopTurn,
    ) -> Result<crate::RuntimeLoopDecision, Self::Error> {
        let steel_turn = animus_steel::LoopTurn::try_from(turn)?;
        animus_steel::SteelLoopFabric::decide(self, &steel_turn)
            .map(crate::RuntimeLoopDecision::from)
    }
}

#[cfg(test)]
mod tests {
    use super::LoopFabric;

    const SUMMARY: &str = "intent: build reliable systems";
    const MODEL_MESSAGE: &str = "model answer";
    const TOOL_NAME: &str = "search";
    const TOOL_INPUT: &str = "query";
    const TOOL_OUTPUT: &str = "tool result";
    const FABRIC_FAILURE: &str = "malformed decision";
    const NO_ADAPTER_CALLS: usize = 0;
    const ONE_ADAPTER_CALL: usize = 1;
    const FOLLOWUP_TURN_BUDGET: usize = 2;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct DecisionFailure;

    impl core::fmt::Display for DecisionFailure {
        fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            formatter.write_str(FABRIC_FAILURE)
        }
    }

    struct FakeLoop {
        decisions: Vec<Result<crate::RuntimeLoopDecision, DecisionFailure>>,
        index: usize,
    }

    impl FakeLoop {
        fn new(decisions: Vec<Result<crate::RuntimeLoopDecision, DecisionFailure>>) -> Self {
            Self {
                decisions,
                index: 0,
            }
        }
    }

    impl LoopFabric for FakeLoop {
        type Error = DecisionFailure;

        fn decide(
            &mut self,
            _turn: &crate::RuntimeLoopTurn,
        ) -> Result<crate::RuntimeLoopDecision, Self::Error> {
            let decision = self
                .decisions
                .get(self.index)
                .cloned()
                .unwrap_or(Err(DecisionFailure));
            self.index = self.index.saturating_add(ONE_ADAPTER_CALL);
            decision
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct AdapterFailure;

    impl core::fmt::Display for AdapterFailure {
        fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            formatter.write_str("adapter failed")
        }
    }

    struct ModelStub {
        calls: usize,
    }

    impl ModelStub {
        const fn new() -> Self {
            Self {
                calls: NO_ADAPTER_CALLS,
            }
        }
    }

    impl
        animus_adapters::ComponentAdapter<
            animus_adapters::ModelRequest,
            animus_adapters::ModelResponse,
        > for ModelStub
    {
        type Error = AdapterFailure;

        fn call(
            &mut self,
            request: &animus_adapters::ModelRequest,
        ) -> Result<animus_adapters::ModelResponse, Self::Error> {
            assert_eq!(request.prompt(), SUMMARY);
            self.calls = self.calls.saturating_add(ONE_ADAPTER_CALL);
            animus_adapters::ModelResponse::new(MODEL_MESSAGE).map_err(|_error| AdapterFailure)
        }
    }

    struct ToolStub {
        calls: usize,
    }

    impl ToolStub {
        const fn new() -> Self {
            Self {
                calls: NO_ADAPTER_CALLS,
            }
        }
    }

    impl
        animus_adapters::ComponentAdapter<
            animus_adapters::ToolRequest,
            animus_adapters::ToolResponse,
        > for ToolStub
    {
        type Error = AdapterFailure;

        fn call(
            &mut self,
            request: &animus_adapters::ToolRequest,
        ) -> Result<animus_adapters::ToolResponse, Self::Error> {
            assert_eq!(request.name(), TOOL_NAME);
            assert_eq!(request.input(), TOOL_INPUT);
            self.calls = self.calls.saturating_add(ONE_ADAPTER_CALL);
            animus_adapters::ToolResponse::new(TOOL_OUTPUT).map_err(|_error| AdapterFailure)
        }
    }

    #[test]
    fn non_steel_loop_returns_immediate_final_response() {
        let mut model = ModelStub::new();
        let mut tool = ToolStub::new();
        let mut fabric = FakeLoop::new(vec![Ok(response_decision(MODEL_MESSAGE))]);
        let result = run_fake(&mut fabric, &mut model, &mut tool);

        assert!(matches!(
            result.status(),
            crate::RuntimeStatus::FinalResponse { response } if response.message() == MODEL_MESSAGE
        ));
        assert_eq!(model.calls, NO_ADAPTER_CALLS);
        assert_eq!(tool.calls, NO_ADAPTER_CALLS);
    }

    #[test]
    fn non_steel_loop_dispatches_model_and_tool_paths() {
        let mut model = ModelStub::new();
        let mut tool = ToolStub::new();
        let mut model_fabric = FakeLoop::new(vec![
            Ok(model_decision()),
            Ok(response_decision(MODEL_MESSAGE)),
        ]);
        let model_result = run_fake(&mut model_fabric, &mut model, &mut tool);
        let mut tool_fabric = FakeLoop::new(vec![
            Ok(tool_decision()),
            Ok(response_decision(TOOL_OUTPUT)),
        ]);
        let tool_result = run_fake(&mut tool_fabric, &mut model, &mut tool);

        assert!(matches!(
            model_result.status(),
            crate::RuntimeStatus::FinalResponse { response } if response.message() == MODEL_MESSAGE
        ));
        assert!(matches!(
            tool_result.status(),
            crate::RuntimeStatus::FinalResponse { response } if response.message() == TOOL_OUTPUT
        ));
        assert_eq!(model.calls, ONE_ADAPTER_CALL);
        assert_eq!(tool.calls, ONE_ADAPTER_CALL);
    }

    #[test]
    fn non_steel_loop_fails_closed_for_malformed_decision_and_missing_service() {
        let mut model = ModelStub::new();
        let mut tool = ToolStub::new();
        let mut bad_fabric = FakeLoop::new(vec![Err(DecisionFailure)]);
        let bad_result = run_fake(&mut bad_fabric, &mut model, &mut tool);
        let mut missing_model_fabric = FakeLoop::new(vec![Ok(model_decision())]);
        let missing_model_result = crate::run_agent_loop(
            &runtime_input(),
            runtime_config(),
            &mut missing_model_fabric,
            crate::RuntimeAdapters::<ModelStub, ToolStub>::new_optional(None, Some(&mut tool)),
        );

        assert!(matches!(
            bad_result.status(),
            crate::RuntimeStatus::Failed {
                failure: crate::RuntimeFailure::Fabric { message }
            } if message == FABRIC_FAILURE
        ));
        assert!(matches!(
            missing_model_result.status(),
            crate::RuntimeStatus::Failed {
                failure: crate::RuntimeFailure::ModelAdapter { message }
            } if message == animus_adapters::OutcomeKind::Absent.as_str()
        ));
        assert_eq!(model.calls, NO_ADAPTER_CALLS);
        assert_eq!(tool.calls, NO_ADAPTER_CALLS);
    }

    fn run_fake(
        fabric: &mut FakeLoop,
        model: &mut ModelStub,
        tool: &mut ToolStub,
    ) -> crate::RunReport {
        crate::run_agent_loop(
            &runtime_input(),
            runtime_config(),
            fabric,
            crate::RuntimeAdapters::new(model, tool),
        )
    }

    fn runtime_input() -> crate::RuntimeInput {
        crate::RuntimeInput::new(SUMMARY).expect("summary should validate")
    }

    fn runtime_config() -> crate::RuntimeConfig {
        crate::RuntimeConfig::from_max_turn_count(FOLLOWUP_TURN_BUDGET)
            .expect("turn budget should validate")
    }

    fn model_decision() -> crate::RuntimeLoopDecision {
        crate::RuntimeLoopDecision::from_draft(crate::RuntimeLoopDecisionDraft::model(SUMMARY))
            .expect("model decision should validate")
    }

    fn tool_decision() -> crate::RuntimeLoopDecision {
        crate::RuntimeLoopDecision::from_draft(crate::RuntimeLoopDecisionDraft::tool(
            TOOL_NAME, TOOL_INPUT,
        ))
        .expect("tool decision should validate")
    }

    fn response_decision(message: &str) -> crate::RuntimeLoopDecision {
        crate::RuntimeLoopDecision::from_draft(crate::RuntimeLoopDecisionDraft::respond(message))
            .expect("response decision should validate")
    }
}
