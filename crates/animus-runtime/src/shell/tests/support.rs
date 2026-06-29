pub(super) const SUMMARY: &str = "intent: build reliable systems";
pub(super) const MODEL_MESSAGE: &str = "model answer";
pub(super) const TOOL_NAME: &str = "search";
pub(super) const TOOL_INPUT: &str = "query";
pub(super) const TOOL_OUTPUT: &str = "tool result";
pub(super) const ADAPTER_FAILURE: &str = "adapter failed";
pub(super) const ABSENT_SERVICE: &str = "absent";
pub(super) const DENIED_SERVICE: &str = "denied";
pub(super) const INVALID_CONFIG_TURNS: usize = 0;
pub(super) const NO_ADAPTER_CALLS: usize = 0;
pub(super) const EXPECTED_ADAPTER_CALLS: usize = 1;
pub(super) const INITIAL_TURN_BUDGET: usize = 1;
pub(super) const FOLLOWUP_TURN_BUDGET: usize = 2;
const FOLLOWUP_TURN_BUDGET_U32: u32 = 2;
pub(super) const BLANK_TEXT: &str = "   ";
pub(super) const STEEL_POLICY_PROFILE: &str = "default-loop";
pub(super) const STEEL_POLICY_SCRIPT: &str = "scripts/default-loop.scm";
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

pub(super) fn run_policy_program_with_followup_turn_budget(
    program: &str,
    model: &mut ModelStub,
    tool: &mut ToolStub,
) -> crate::RunReport {
    let profile = policy_profile_for_program(program, FOLLOWUP_TURN_BUDGET_U32);
    let script = policy_script_for_program(program);
    let policy = super::super::SteelPolicyRuntime::new(&profile, script);
    super::super::run_with_steel_policy(
        &policy,
        &runtime_input(),
        runtime_config_with_followup_turn_budget(),
        super::super::RuntimeAdapters::new(model, tool),
    )
}

pub(super) fn run_policy_program_with_bad_profile(
    program: &str,
    model: &mut ModelStub,
    tool: &mut ToolStub,
) -> crate::RunReport {
    let profile = policy_profile_for_program(program, FOLLOWUP_TURN_BUDGET_U32);
    let script = animus_steel::SteelPolicyScript::new("other-loop", STEEL_POLICY_SCRIPT, program);
    let policy = super::super::SteelPolicyRuntime::new(&profile, script);
    super::super::run_with_steel_policy(
        &policy,
        &runtime_input(),
        runtime_config_with_followup_turn_budget(),
        super::super::RuntimeAdapters::new(model, tool),
    )
}

pub(super) fn run_program_with_missing_model_service(
    program: &str,
    tool: &mut ToolStub,
) -> crate::RunReport {
    super::super::run_with_steel_program(
        program,
        &runtime_input(),
        runtime_config_with_followup_turn_budget(),
        super::super::RuntimeAdapters::<ModelStub, ToolStub>::new_optional(None, Some(tool)),
    )
}

pub(super) fn run_program_with_missing_tool_service(
    program: &str,
    model: &mut ModelStub,
) -> crate::RunReport {
    super::super::run_with_steel_program(
        program,
        &runtime_input(),
        runtime_config_with_followup_turn_budget(),
        super::super::RuntimeAdapters::<ModelStub, ToolStub>::new_optional(Some(model), None),
    )
}

pub(super) fn runtime_input() -> crate::RuntimeInput {
    crate::RuntimeInput::new(SUMMARY).expect("summary should be valid")
}

fn policy_profile_for_program(program: &str, max_turns: u32) -> animus_steel::SteelPolicyProfile {
    let script_hash = animus_steel::steel_script_hash(program);
    animus_steel::SteelPolicyProfile::from_draft(&animus_steel::SteelPolicyProfileDraft {
        name: STEEL_POLICY_PROFILE,
        enabled: true,
        is_default: true,
        script_label: STEEL_POLICY_SCRIPT,
        script_hash: script_hash.as_str(),
        max_turns,
        allowed_host_actions: &["model", "tool", "respond", "stop"],
        fallback_mode: "block",
        receipt_redaction: "hash-only",
    })
    .expect("policy fixture should be valid")
}

fn policy_script_for_program(program: &str) -> animus_steel::SteelPolicyScript<'_> {
    animus_steel::SteelPolicyScript::new(STEEL_POLICY_PROFILE, STEEL_POLICY_SCRIPT, program)
}

fn runtime_config_with_initial_turn_budget() -> crate::RuntimeConfig {
    crate::RuntimeConfig::from_max_turn_count(INITIAL_TURN_BUDGET)
        .expect("one max turn should be valid")
}

pub(super) fn runtime_config_with_followup_turn_budget() -> crate::RuntimeConfig {
    crate::RuntimeConfig::from_max_turn_count(FOLLOWUP_TURN_BUDGET)
        .expect("two max turns should be valid")
}
