use super::support::*;
use super::*;

#[test]
fn workflow_manifest_accepts_typed_recipe_with_toolsets() {
    let workflow = minimal_workflow();

    assert_eq!(workflow.identity(), WORKFLOW_ID);
    assert_eq!(workflow.activities().len(), ACTIVITY_COUNT);
    assert_eq!(workflow.parameters().len(), PARAMETER_COUNT);
    assert_eq!(workflow.model_profile(), MODEL_PROFILE);
    assert_eq!(workflow.toolsets()[FIRST_ITEM_INDEX], TOOLSET_READONLY);
    assert_eq!(
        workflow.validation_gates()[FIRST_ITEM_INDEX],
        VALIDATION_GATE
    );
    assert_eq!(workflow.retry_policy().max_attempts(), RETRY_ATTEMPTS);
    assert_eq!(workflow.response_schema().name(), RESPONSE_SCHEMA);
    assert_eq!(workflow.provenance().status(), RecipeTrustStatus::Admitted);
    assert_eq!(
        workflow.provenance().identity_hash().len(),
        TRUST_HASH_CHARS
    );
}

#[test]
fn parameter_binding_accepts_declared_variables_defaults_and_file_refs() {
    let workflow = minimal_workflow();
    let inputs = inputs_using_default();
    let binding = bind_recipe_parameters(RecipeBindingDraft {
        workflow: &workflow,
        template: workflow.initial_prompt(),
        inputs: &inputs,
    })
    .expect("binding should validate");

    assert!(
        binding
            .template_variables()
            .iter()
            .any(|variable| variable == TOPIC_PARAM)
    );
    assert!(
        binding
            .template_variables()
            .iter()
            .any(|variable| variable == "recipe.identity")
    );
    assert!(binding.bound_parameters().iter().any(|input| {
        input.name() == MODE_PARAM
            && input.value() == MODE_REVIEW
            && input.kind() == RecipeParameterInputKind::DefaultValue
    }));
    assert!(binding.bound_parameters().iter().any(|input| {
        input.name() == FILE_PARAM
            && input.value() == FILE_REF
            && input.kind() == RecipeParameterInputKind::FileContentRef
    }));
}

#[test]
fn structured_output_accepts_required_fields() {
    let workflow = minimal_workflow();
    let output = RecipeOutputDraft {
        fields: &[
            RecipeOutputFieldDraft {
                name: OUTPUT_STATUS,
                kind: "string",
            },
            RecipeOutputFieldDraft {
                name: OUTPUT_WARNINGS,
                kind: "array",
            },
        ],
    };

    let outcome = validate_recipe_output(workflow.response_schema(), &output)
        .expect("output fields should parse");

    assert_eq!(outcome.status(), RecipeAutomationStatus::Success);
    assert_eq!(outcome.failure_reason(), None);
}

#[test]
fn retry_plan_records_pending_and_denied_host_effect_requests() {
    let workflow = minimal_workflow();
    let pending = plan_recipe_retry(RecipeRetryPlanDraft {
        policy: workflow.retry_policy(),
        denied_capabilities: &[],
        denied_sandboxes: &[],
    });
    let denied = plan_recipe_retry(RecipeRetryPlanDraft {
        policy: workflow.retry_policy(),
        denied_capabilities: &[CAPABILITY_TOOL],
        denied_sandboxes: &[],
    });

    assert_eq!(
        pending.requests()[FIRST_ITEM_INDEX].status(),
        RecipeRetryRequestStatus::PendingHostEffect
    );
    assert_eq!(
        denied.requests()[FIRST_ITEM_INDEX].status(),
        RecipeRetryRequestStatus::Denied {
            reason: RecipeRetryDenialReason::Capability,
        }
    );
}

#[test]
fn admitted_trust_allows_bounded_recipe_influence() {
    let receipt = RecipeTrustReceipt::new(&admitted_provenance()).expect("receipt should parse");
    let influence = RecipeInfluence::evaluate(&receipt);

    assert_eq!(influence.status(), RecipeInfluenceStatus::Active);
    assert!(influence.prompt_allowed());
    assert!(influence.tools_allowed());
    assert!(influence.model_allowed());
    assert!(influence.memory_allowed());
    assert!(influence.workflow_permissions_allowed());
}
