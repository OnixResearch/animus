use super::*;

#[test]
fn positive_model_matches_production_for_allowed_authority_paths() {
    let model_request = capability_request!(GuardedAction::Model, MODEL_RESOURCE, MODEL_ABILITY);
    let model_contract = policy_contract!(GuardedAction::Model, MODEL_PREFIX, MODEL_ABILITY);
    let model_grant = ucan_grant!(MODEL_PREFIX, MODEL_ABILITY);
    let model_caveats = [LocalCaveat::new(
        LocalPolicyLayer::Project,
        LocalCaveatDecision::Allow,
    )];
    assert_model_matches_production(AuthorizationContext {
        request: &model_request,
        contract: &model_contract,
        grants: &[model_grant],
        caveats: &model_caveats,
        sandbox: &SandboxConstraint::NotRequired,
    });

    let exec_request = capability_request!(GuardedAction::Exec, EXEC_RESOURCE, EXEC_ABILITY);
    let exec_contract = policy_contract!(GuardedAction::Exec, EXEC_PREFIX, EXEC_ABILITY);
    let exec_grant = ucan_grant!(EXEC_PREFIX, EXEC_ABILITY);
    let exec_sandbox = SandboxConstraint::required(safe_hash(), SandboxState::Enforceable);
    assert_model_matches_production(AuthorizationContext {
        request: &exec_request,
        contract: &exec_contract,
        grants: &[exec_grant],
        caveats: &[],
        sandbox: &exec_sandbox,
    });
}

#[test]
fn negative_model_matches_production_for_rejected_counterexamples() {
    let contract_request =
        capability_request!(GuardedAction::Filesystem, FILE_RESOURCE, OTHER_ABILITY);
    let contract = policy_contract!(GuardedAction::Filesystem, FILE_PREFIX, FILE_ABILITY);
    let wrong_grant = ucan_grant!(FILE_PREFIX, OTHER_ABILITY);
    assert_model_matches_production(AuthorizationContext {
        request: &contract_request,
        contract: &contract,
        grants: &[wrong_grant],
        caveats: &[],
        sandbox: &SandboxConstraint::NotRequired,
    });

    let tool_request = capability_request!(GuardedAction::Tool, TOOL_RESOURCE, TOOL_ABILITY);
    let tool_contract = policy_contract!(GuardedAction::Tool, TOOL_PREFIX, TOOL_ABILITY);
    let local_deny_grant = ucan_grant!(TOOL_PREFIX, TOOL_ABILITY);
    let local_denies = [LocalCaveat::new(
        LocalPolicyLayer::Harness,
        LocalCaveatDecision::Deny,
    )];
    assert_model_matches_production(AuthorizationContext {
        request: &tool_request,
        contract: &tool_contract,
        grants: &[local_deny_grant],
        caveats: &local_denies,
        sandbox: &SandboxConstraint::NotRequired,
    });
    assert_model_matches_production(AuthorizationContext {
        request: &tool_request,
        contract: &tool_contract,
        grants: &[],
        caveats: &[],
        sandbox: &SandboxConstraint::NotRequired,
    });

    let exec_request = capability_request!(GuardedAction::Exec, EXEC_RESOURCE, EXEC_ABILITY);
    let exec_contract = policy_contract!(GuardedAction::Exec, EXEC_PREFIX, EXEC_ABILITY);
    let exec_grant = ucan_grant!(EXEC_PREFIX, EXEC_ABILITY);
    let malformed = SandboxConstraint::required(safe_hash(), SandboxState::Malformed);
    let unavailable = SandboxConstraint::required(safe_hash(), SandboxState::Unavailable);
    assert_model_matches_production(AuthorizationContext {
        request: &exec_request,
        contract: &exec_contract,
        grants: core::slice::from_ref(&exec_grant),
        caveats: &[],
        sandbox: &malformed,
    });
    assert_model_matches_production(AuthorizationContext {
        request: &exec_request,
        contract: &exec_contract,
        grants: &[exec_grant],
        caveats: &[],
        sandbox: &unavailable,
    });
}

fn assert_model_matches_production(context: AuthorizationContext<'_>) {
    let model_input = AuthorizationModelInput::from_context(&context);
    let model_issue = model_authorization_issue(model_input);
    let production_receipt = authorize(context);
    assert_eq!(production_receipt.issue(), model_issue);
    assert_eq!(
        production_receipt.is_allowed(),
        model_issue == AuthorizationIssue::None
    );
}
