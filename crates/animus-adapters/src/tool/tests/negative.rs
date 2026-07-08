mod spans;

use super::super::*;
use super::support::*;

#[test]
fn request_rejects_missing_current_shape_fields() {
    // r[verify agent_harness_runtime.tool_calling_breaking.current_shape.rejects_legacy]
    let missing_name_arguments = prepared_arguments();
    let missing_name = Request::from_draft(&RequestDraft {
        name: None,
        arguments: Some(&missing_name_arguments),
        correlation_id: Some(CORRELATION_ID),
        output_policy: OutputPolicy {
            visible_char_budget: VISIBLE_BUDGET,
        },
        mutation_targets: &[],
        required_capability_labels: &[CAPABILITY_READ],
        approval_posture: APPROVAL_PRE_APPROVED,
    })
    .expect_err("missing tool name should fail");
    assert_eq!(
        missing_name,
        ContractError::MissingField {
            field: super::super::NAME_FIELD.as_str(),
        }
    );

    let missing_arguments = Request::from_draft(&RequestDraft {
        name: Some(NAME),
        arguments: None,
        correlation_id: Some(CORRELATION_ID),
        output_policy: OutputPolicy {
            visible_char_budget: VISIBLE_BUDGET,
        },
        mutation_targets: &[],
        required_capability_labels: &[CAPABILITY_READ],
        approval_posture: APPROVAL_PRE_APPROVED,
    })
    .expect_err("missing arguments should fail");
    assert_eq!(
        missing_arguments,
        ContractError::MissingField {
            field: super::super::ARGUMENT_NAME_FIELD.as_str(),
        }
    );
}

#[test]
fn request_rejects_missing_schema_identity_capabilities_and_bad_approval() {
    // r[verify agent_harness_runtime.tool_calling_breaking.current_shape.rejects_legacy]
    let fields = [ArgumentFieldDraft {
        name: Some(CURRENT_ARGUMENT),
        value: Some(ARGUMENT_VALUE),
    }];
    let missing_schema = PreparedArguments::prepare(&ArgumentPreparationRequest {
        current_schema_identity: BLANK_TEXT,
        allowed_field_names: &[CURRENT_ARGUMENT],
        compatibility_rules: &[],
        fields: &fields,
    })
    .expect_err("blank schema identity should fail");
    assert!(matches!(missing_schema, ContractError::Adapter(_)));

    let arguments = prepared_arguments();
    let missing_caps = Request::from_draft(&RequestDraft {
        name: Some(NAME),
        arguments: Some(&arguments),
        correlation_id: Some(CORRELATION_ID),
        output_policy: OutputPolicy {
            visible_char_budget: VISIBLE_BUDGET,
        },
        mutation_targets: &[],
        required_capability_labels: &[],
        approval_posture: APPROVAL_PRE_APPROVED,
    })
    .expect_err("missing capability labels should fail");
    assert_eq!(
        missing_caps,
        ContractError::Adapter(crate::AdapterContractError::EmptyCollection {
            field: super::super::CAPABILITY_FIELD.as_str(),
        })
    );

    let bad_approval = Request::from_draft(&RequestDraft {
        name: Some(NAME),
        arguments: Some(&arguments),
        correlation_id: Some(CORRELATION_ID),
        output_policy: OutputPolicy {
            visible_char_budget: VISIBLE_BUDGET,
        },
        mutation_targets: &[],
        required_capability_labels: &[CAPABILITY_READ],
        approval_posture: "ambient-approval",
    })
    .expect_err("unsupported approval should fail");
    assert_eq!(
        bad_approval,
        ContractError::UnsupportedApprovalPosture {
            posture: super::super::Text::from("ambient-approval"),
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
fn malformed_unknown_arguments_fail() {
    // r[verify agent_harness_runtime.tool_calling_breaking.current_shape.rejects_legacy]
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
}

#[test]
fn duplicate_current_arguments_fail() {
    let duplicate_fields = [
        ArgumentFieldDraft {
            name: Some(CURRENT_ARGUMENT),
            value: Some(ARGUMENT_VALUE),
        },
        ArgumentFieldDraft {
            name: Some(CURRENT_ARGUMENT),
            value: Some(ARGUMENT_VALUE),
        },
    ];
    assert!(
        duplicate_fields
            .iter()
            .all(|field| field.name == Some(CURRENT_ARGUMENT))
    );
    let duplicate = PreparedArguments::prepare(&ArgumentPreparationRequest {
        current_schema_identity: SCHEMA_ID,
        allowed_field_names: &[CURRENT_ARGUMENT],
        compatibility_rules: &[],
        fields: &duplicate_fields,
    })
    .expect_err("duplicate current fields should fail");
    assert_eq!(
        duplicate,
        ContractError::DuplicateArgument {
            name: super::super::Text::from(CURRENT_ARGUMENT),
        }
    );
}

#[test]
fn missing_argument_values_fail() {
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
fn legacy_argument_compatibility_rules_fail() {
    let legacy_fields = [ArgumentFieldDraft {
        name: Some(LEGACY_ARGUMENT),
        value: Some(ARGUMENT_VALUE),
    }];
    let legacy_rules = [ArgumentCompatibilityRule {
        legacy_name: LEGACY_ARGUMENT,
        current_name: CURRENT_ARGUMENT,
    }];
    let legacy = PreparedArguments::prepare(&ArgumentPreparationRequest {
        current_schema_identity: SCHEMA_ID,
        allowed_field_names: &[CURRENT_ARGUMENT],
        compatibility_rules: &legacy_rules,
        fields: &legacy_fields,
    })
    .expect_err("legacy compatibility should be disabled");
    assert_eq!(legacy, ContractError::LegacyCompatibilityDisabled);
}

#[test]
fn unsafe_output_reference_and_raw_body_hazards_are_rejected() {
    // r[verify agent_harness_runtime.tool_calling_breaking.bounded_result.no_raw_body]
    let unsafe_reference = BoundedOutput::new(
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
        unsafe_reference,
        ContractError::UnsafeReference {
            reference: super::super::Text::from(UNSAFE_REFERENCE),
        }
    );

    let nul_output = BoundedOutput::new(
        &OutputDraft {
            body: "bad\0output",
            full_output_reference: None,
            hash_requirement: HashRequirement::Required,
        },
        OutputPolicy {
            visible_char_budget: VISIBLE_BUDGET,
        },
    )
    .expect_err("NUL output should fail");
    assert!(matches!(nul_output, ContractError::Adapter(_)));
}

#[test]
fn missing_executor_denied_capability_and_unsafe_output_fail_without_fallback() {
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

    let unsafe_output = evaluate_dispatch(DispatchRequest {
        definition: Some(&active_definition),
        executor_status: ExecutorStatus::Available,
        capability_status: CapabilityStatus::Allowed,
        output_boundary_status: OutputBoundaryStatus::UnsafeReference,
    });
    assert_eq!(unsafe_output.status(), DispatchStatus::Failed);
    assert_eq!(
        unsafe_output.failure_class(),
        Some(FailureClass::UnsafeOutputReference)
    );
    assert!(!unsafe_output.fallback_attempted());
}
