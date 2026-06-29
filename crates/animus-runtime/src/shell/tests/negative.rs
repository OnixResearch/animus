#[test]
fn invalid_steel_decision_fails_before_adapter_dispatch() {
    let mut model = super::support::ModelStub::ok();
    let mut tool = super::support::ToolStub::ok();
    let result = super::support::run_program_with_initial_turn_budget(
        super::support::INVALID_DECISION_PROGRAM,
        &mut model,
        &mut tool,
    );

    assert!(matches!(
        result.status(),
        crate::RuntimeStatus::Failed {
            failure: crate::RuntimeFailure::Fabric { .. }
        }
    ));
    assert_eq!(model.calls, super::support::NO_ADAPTER_CALLS);
    assert_eq!(tool.calls, super::support::NO_ADAPTER_CALLS);
}

#[test]
fn invalid_steel_load_fails_before_adapter_dispatch() {
    let mut model = super::support::ModelStub::ok();
    let mut tool = super::support::ToolStub::ok();
    let result = super::support::run_program_with_initial_turn_budget(
        super::support::INVALID_LOAD_PROGRAM,
        &mut model,
        &mut tool,
    );

    assert!(matches!(
        result.status(),
        crate::RuntimeStatus::Failed {
            failure: crate::RuntimeFailure::Fabric { .. }
        }
    ));
    assert_eq!(model.calls, super::support::NO_ADAPTER_CALLS);
    assert_eq!(tool.calls, super::support::NO_ADAPTER_CALLS);
}

#[test]
fn model_adapter_failure_stops_safely() {
    let mut model = super::support::ModelStub::failing();
    let mut tool = super::support::ToolStub::ok();
    let result = super::support::run_program_with_followup_turn_budget(
        super::support::MODEL_THEN_RESPOND_PROGRAM,
        &mut model,
        &mut tool,
    );

    assert!(matches!(
        result.status(),
        crate::RuntimeStatus::Failed {
            failure: crate::RuntimeFailure::ModelAdapter { message }
        } if message == super::support::ADAPTER_FAILURE
    ));
    assert_eq!(model.calls, super::support::EXPECTED_ADAPTER_CALLS);
    assert_eq!(tool.calls, super::support::NO_ADAPTER_CALLS);
}

#[test]
fn tool_adapter_failure_stops_safely() {
    let mut model = super::support::ModelStub::ok();
    let mut tool = super::support::ToolStub::failing();
    let result = super::support::run_program_with_followup_turn_budget(
        super::support::TOOL_THEN_RESPOND_PROGRAM,
        &mut model,
        &mut tool,
    );

    assert!(matches!(
        result.status(),
        crate::RuntimeStatus::Failed {
            failure: crate::RuntimeFailure::ToolAdapter { message }
        } if message == super::support::ADAPTER_FAILURE
    ));
    assert_eq!(model.calls, super::support::NO_ADAPTER_CALLS);
    assert_eq!(tool.calls, super::support::EXPECTED_ADAPTER_CALLS);
}

#[test]
fn max_turn_exhaustion_fails_closed_without_extra_adapter_call() {
    let mut model = super::support::ModelStub::ok();
    let mut tool = super::support::ToolStub::ok();
    let result = super::support::run_program_with_followup_turn_budget(
        super::support::SPINNING_MODEL_PROGRAM,
        &mut model,
        &mut tool,
    );

    assert_eq!(
        result.status(),
        &crate::RuntimeStatus::MaxTurnsExceeded {
            max_turns: super::support::FOLLOWUP_TURN_BUDGET,
        }
    );
    assert_eq!(model.calls, super::support::FOLLOWUP_TURN_BUDGET);
    assert_eq!(tool.calls, super::support::NO_ADAPTER_CALLS);
}

#[test]
fn runtime_input_rejects_empty_summary() {
    let error = crate::RuntimeInput::new(super::support::BLANK_TEXT)
        .expect_err("blank summary should fail");

    assert!(matches!(error, crate::RuntimeError::EmptyInput { .. }));
}

#[test]
fn runtime_config_rejects_zero_turn_limit() {
    let error = crate::RuntimeConfig::from_max_turn_count(super::support::INVALID_CONFIG_TURNS)
        .expect_err("zero max turns should fail");

    assert!(matches!(error, crate::RuntimeError::InvalidConfig { .. }));
}
