mod spans;

use super::super::*;
use super::support::*;

#[test]
fn request_rejects_missing_name() {
    let error = Request::new(BLANK_TEXT, INPUT).expect_err("blank tool name should fail");

    assert_eq!(
        error,
        crate::AdapterContractError::EmptyField {
            field: super::super::NAME_FIELD.as_str(),
        }
    );
}

#[test]
fn definition_metadata_rejects_missing_schema_and_empty_guidelines() {
    let missing_schema = Definition::validate(&DefinitionDraft {
        parameter_schema_identity: None,
        ..definition_draft(DefinitionStatus::Active, NAME)
    })
    .expect_err("missing schema identity should fail");
    assert_eq!(
        missing_schema,
        ContractError::MissingField {
            field: super::super::SCHEMA_FIELD.as_str(),
        }
    );

    let empty_guidelines = Definition::validate(&DefinitionDraft {
        prompt_guidelines: &[],
        ..definition_draft(DefinitionStatus::Active, NAME)
    })
    .expect_err("empty guidelines should fail");
    assert_eq!(
        empty_guidelines,
        ContractError::Adapter(crate::AdapterContractError::EmptyCollection {
            field: super::super::PROMPT_GUIDELINE_FIELD.as_str(),
        })
    );
}

#[test]
fn malformed_or_unknown_arguments_fail_after_preparation() {
    let unknown_fields = [ArgumentFieldDraft {
        name: Some(EXTRA_ARGUMENT),
        value: Some(ARGUMENT_VALUE),
    }];
    let unknown = PreparedArguments::prepare(&ArgumentPreparationRequest {
        current_schema_identity: SCHEMA_ID,
        allowed_field_names: &[CURRENT_ARGUMENT],
        compatibility_rules: &[],
        fields: &unknown_fields,
    })
    .expect_err("unknown current field should fail");
    assert_eq!(
        unknown,
        ContractError::UnknownArgument {
            name: super::super::Text::from(EXTRA_ARGUMENT),
        }
    );

    let missing_value_fields = [ArgumentFieldDraft {
        name: Some(CURRENT_ARGUMENT),
        value: None,
    }];
    let missing_value = PreparedArguments::prepare(&ArgumentPreparationRequest {
        current_schema_identity: SCHEMA_ID,
        allowed_field_names: &[CURRENT_ARGUMENT],
        compatibility_rules: &[],
        fields: &missing_value_fields,
    })
    .expect_err("missing argument value should fail");
    assert_eq!(
        missing_value,
        ContractError::MissingField {
            field: super::super::ARGUMENT_VALUE_FIELD.as_str(),
        }
    );
}

#[test]
fn unsafe_output_reference_is_rejected() {
    let error = BoundedOutput::new(
        &OutputDraft {
            body: OUTPUT,
            full_output_reference: Some(UNSAFE_REFERENCE),
            hash_requirement: HashRequirement::Required,
        },
        OutputPolicy {
            visible_char_budget: VISIBLE_BUDGET,
        },
    )
    .expect_err("absolute references should fail");

    assert_eq!(
        error,
        ContractError::UnsafeReference {
            reference: super::super::Text::from(UNSAFE_REFERENCE),
        }
    );
}

#[test]
fn missing_executor_and_denied_capability_fail_without_fallback() {
    let active_definition = definition(DefinitionStatus::Active, NAME);
    let missing_executor = evaluate_dispatch(DispatchRequest {
        definition: Some(&active_definition),
        executor_status: ExecutorStatus::Missing,
        capability_status: CapabilityStatus::Allowed,
        output_boundary_status: OutputBoundaryStatus::Bounded,
    });
    assert_eq!(missing_executor.status(), DispatchStatus::Failed);
    assert_eq!(
        missing_executor.failure_class(),
        Some(FailureClass::MissingExecutor)
    );
    assert!(!missing_executor.fallback_attempted());

    let denied = evaluate_dispatch(DispatchRequest {
        definition: Some(&active_definition),
        executor_status: ExecutorStatus::Available,
        capability_status: CapabilityStatus::Denied,
        output_boundary_status: OutputBoundaryStatus::Bounded,
    });
    assert_eq!(denied.status(), DispatchStatus::Failed);
    assert_eq!(denied.failure_class(), Some(FailureClass::DeniedCapability));
    assert!(!denied.fallback_attempted());
}
