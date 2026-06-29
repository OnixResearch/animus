pub(super) const SUMMARY: &str = "intent: build reliable systems";
pub(super) const MODEL_MESSAGE: &str = "model answer";
pub(super) const TOOL_NAME: &str = "search";
pub(super) const TOOL_INPUT: &str = "query";
pub(super) const TOOL_OUTPUT: &str = "tool result";
pub(super) const ADAPTER_FAILURE: &str = "adapter failed";
pub(super) const INVALID_CONFIG_TURNS: usize = 0;
pub(super) const NO_ADAPTER_CALLS: usize = 0;
pub(super) const EXPECTED_ADAPTER_CALLS: usize = 1;
pub(super) const INITIAL_TURN_BUDGET: usize = 1;
pub(super) const FOLLOWUP_TURN_BUDGET: usize = 2;
pub(super) const BLANK_TEXT: &str = "   ";
pub(super) const RESPOND_PROGRAM: &str = r#"
(define (animus-decide)
  (animus-respond "model answer"))
"#;
pub(super) const STOP_PROGRAM: &str = r"
(define (animus-decide)
  (animus-stop))
";
pub(super) const MODEL_THEN_RESPOND_PROGRAM: &str = r"
(define (animus-decide)
  (if animus-last-model-message
      (animus-respond animus-last-model-message)
      (animus-model animus-summary)))
";
pub(super) const TOOL_THEN_RESPOND_PROGRAM: &str = r#"
(define (animus-decide)
  (if animus-last-tool-output
      (animus-respond animus-last-tool-output)
      (animus-tool "search" "query")))
"#;
pub(super) const INVALID_DECISION_PROGRAM: &str = r#"
(define (animus-decide)
  "not a decision")
"#;
pub(super) const INVALID_LOAD_PROGRAM: &str = "(define (";
pub(super) const SPINNING_MODEL_PROGRAM: &str = r"
(define (animus-decide)
  (animus-model animus-summary))
";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct AdapterFailure;

impl core::fmt::Display for AdapterFailure {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter.write_str(ADAPTER_FAILURE)
    }
}

pub(super) struct ModelStub {
    fail: bool,
    pub(super) calls: usize,
}

impl ModelStub {
    pub(super) fn ok() -> Self {
        Self {
            fail: false,
            calls: NO_ADAPTER_CALLS,
        }
    }

    pub(super) fn failing() -> Self {
        Self {
            fail: true,
            calls: NO_ADAPTER_CALLS,
        }
    }
}

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for ModelStub
{
    type Error = AdapterFailure;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        assert_eq!(request.prompt(), SUMMARY);
        self.calls = self.calls.saturating_add(EXPECTED_ADAPTER_CALLS);
        if self.fail {
            return Err(AdapterFailure);
        }

        animus_adapters::ModelResponse::new(MODEL_MESSAGE).map_err(|_error| AdapterFailure)
    }
}

pub(super) struct ToolStub {
    fail: bool,
    pub(super) calls: usize,
}

impl ToolStub {
    pub(super) fn ok() -> Self {
        Self {
            fail: false,
            calls: NO_ADAPTER_CALLS,
        }
    }

    pub(super) fn failing() -> Self {
        Self {
            fail: true,
            calls: NO_ADAPTER_CALLS,
        }
    }
}

impl animus_adapters::ComponentAdapter<animus_adapters::ToolRequest, animus_adapters::ToolResponse>
    for ToolStub
{
    type Error = AdapterFailure;

    fn call(
        &mut self,
        request: &animus_adapters::ToolRequest,
    ) -> Result<animus_adapters::ToolResponse, Self::Error> {
        assert_eq!(request.name(), TOOL_NAME);
        assert_eq!(request.input(), TOOL_INPUT);
        self.calls = self.calls.saturating_add(EXPECTED_ADAPTER_CALLS);
        if self.fail {
            return Err(AdapterFailure);
        }

        animus_adapters::ToolResponse::new(TOOL_OUTPUT).map_err(|_error| AdapterFailure)
    }
}

pub(super) fn run_program_with_initial_turn_budget(
    program: &str,
    model: &mut ModelStub,
    tool: &mut ToolStub,
) -> crate::RunReport {
    super::super::run_with_steel_program(
        program,
        &runtime_input(),
        runtime_config_with_initial_turn_budget(),
        super::super::RuntimeAdapters::new(model, tool),
    )
}

pub(super) fn run_program_with_followup_turn_budget(
    program: &str,
    model: &mut ModelStub,
    tool: &mut ToolStub,
) -> crate::RunReport {
    super::super::run_with_steel_program(
        program,
        &runtime_input(),
        runtime_config_with_followup_turn_budget(),
        super::super::RuntimeAdapters::new(model, tool),
    )
}

fn runtime_input() -> crate::RuntimeInput {
    crate::RuntimeInput::new(SUMMARY).expect("summary should be valid")
}

fn runtime_config_with_initial_turn_budget() -> crate::RuntimeConfig {
    crate::RuntimeConfig::from_max_turn_count(INITIAL_TURN_BUDGET)
        .expect("one max turn should be valid")
}

fn runtime_config_with_followup_turn_budget() -> crate::RuntimeConfig {
    crate::RuntimeConfig::from_max_turn_count(FOLLOWUP_TURN_BUDGET)
        .expect("two max turns should be valid")
}
