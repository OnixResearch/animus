use super::super::*;
use super::support::*;

#[test]
fn request_accepts_current_shape_contract() {
    // r[verify agent_harness_runtime.tool_calling_breaking.current_shape]
    let request = request();

    assert_eq!(request.name(), NAME);
    assert_eq!(request.schema_identity(), SCHEMA_ID);
    assert_eq!(request.correlation_id(), CORRELATION_ID);
    assert_eq!(request.output_policy().visible_char_budget, VISIBLE_BUDGET);
    assert_eq!(request.arguments()[FIRST_INDEX].name(), CURRENT_ARGUMENT);
    assert_eq!(request.arguments()[FIRST_INDEX].value(), ARGUMENT_VALUE);
    assert_eq!(request.mutation_targets()[FIRST_INDEX], TARGET_FILE);
    assert_eq!(
        request.required_capability_labels()[FIRST_INDEX],
        CAPABILITY_READ
    );
    assert_eq!(request.approval_posture(), ApprovalPosture::PreApproved);
    assert_eq!(request.argument_summary(), "query=adapter contracts");
    assert_eq!(Request::contract_identity(), TOOL_REQUEST_CONTRACT_IDENTITY);
}

#[test]
fn response_accepts_bounded_result_contract() {
    // r[verify agent_harness_runtime.tool_calling_breaking.bounded_result]
    let response = response();

    assert_eq!(response.output(), OUTPUT);
    assert_eq!(response.model_visible_text(), OUTPUT);
    assert_eq!(response.status(), OutputStatus::Complete);
    assert_eq!(response.byte_count(), OUTPUT.len() as u64);
    assert_eq!(response.char_count(), OUTPUT.chars().count() as u64);
    assert_eq!(response.line_count(), crate::MIN_ADAPTER_FIELD_CHARS as u64);
    assert_eq!(response.redaction_class(), OutputRedactionClass::Safe);
    assert!(response.content_hash().is_some());
    assert_eq!(
        Response::contract_identity(),
        TOOL_RESPONSE_CONTRACT_IDENTITY
    );
}

#[test]
fn definition_metadata_builds_attributed_prompt_context_for_active_tools() {
    let active = definition(DefinitionStatus::Active, NAME);
    let inactive = definition(DefinitionStatus::Inactive, INACTIVE_NAME);
    let untrusted = definition(DefinitionStatus::Untrusted, "unsafe_tool");
    let context = build_prompt_context(&[active, inactive, untrusted]);

    assert_eq!(context.len(), PROMPT_CONTEXT_ENTRY_COUNT);
    assert_eq!(context[FIRST_INDEX].tool_name(), NAME);
    assert_eq!(context[FIRST_INDEX].kind(), PromptContextKind::Snippet);
    assert_eq!(context[FIRST_INDEX].text(), PROMPT_SNIPPET);
    assert_eq!(context[SECOND_INDEX].tool_name(), NAME);
    assert_eq!(context[SECOND_INDEX].kind(), PromptContextKind::Guideline);
    assert_eq!(context[SECOND_INDEX].text(), PROMPT_GUIDELINE);
}

#[test]
fn current_arguments_prepare_without_compatibility_rules() {
    let prepared = prepared_arguments();

    assert_eq!(prepared.schema_identity(), SCHEMA_ID);
    assert_eq!(prepared.fields()[FIRST_INDEX].name(), CURRENT_ARGUMENT);
    assert_eq!(prepared.fields()[FIRST_INDEX].value(), ARGUMENT_VALUE);
}

#[test]
fn oversized_output_is_bounded_and_hashed() {
    let bounded = BoundedOutput::new(
        &OutputDraft {
            body: LONG_OUTPUT,
            full_output_reference: Some(SAFE_REFERENCE),
            hash_requirement: HashRequirement::Optional,
        },
        OutputPolicy {
            visible_char_budget: VISIBLE_BUDGET,
        },
    )
    .expect("oversized output should truncate safely");

    assert_eq!(bounded.status(), OutputStatus::Truncated);
    let visible_budget = usize::try_from(VISIBLE_BUDGET).expect("visible budget fits usize");
    assert_eq!(bounded.visible_text().chars().count(), visible_budget);
    assert_ne!(bounded.visible_text(), LONG_OUTPUT);
    assert_eq!(bounded.char_count(), LONG_OUTPUT_CHAR_COUNT);
    assert_eq!(bounded.line_count(), LONG_OUTPUT_LINE_COUNT);
    assert_eq!(bounded.full_output_reference(), Some(SAFE_REFERENCE));
    assert!(bounded.content_hash().is_some());
}

#[test]
fn mutation_windows_serialize_conflicting_targets_only() {
    let windows = plan_mutation_windows(&[
        MutationCallDraft {
            call_id: Some(CALL_FIRST),
            mutation_targets: &[TARGET_FILE],
        },
        MutationCallDraft {
            call_id: Some(CALL_SECOND),
            mutation_targets: &[TARGET_FILE],
        },
        MutationCallDraft {
            call_id: Some(CALL_THIRD),
            mutation_targets: &[TARGET_OTHER],
        },
    ])
    .expect("mutation windows should plan");

    assert_eq!(
        windows[FIRST_INDEX].status(),
        MutationWindowStatus::Independent
    );
    assert_eq!(
        windows[SECOND_INDEX].status(),
        MutationWindowStatus::Serialized
    );
    assert_eq!(windows[SECOND_INDEX].waits_for(), Some(CALL_FIRST));
    assert_eq!(
        windows[THIRD_INDEX].status(),
        MutationWindowStatus::Independent
    );
    assert_eq!(windows[THIRD_INDEX].waits_for(), None);
}

#[test]
fn hashed_edit_plan_validates_ordered_spans_and_safe_receipt() {
    let snapshot = hashed_snapshot();
    let spans = [
        span_draft(SpanFixture {
            start_tag: TAG_FIRST,
            end_tag: TAG_FIRST,
            replacement: REPLACEMENT_TEXT,
        }),
        span_draft(SpanFixture {
            start_tag: TAG_THIRD,
            end_tag: TAG_THIRD,
            replacement: DELETE_TEXT,
        }),
    ];
    let request = hashed_request(&spans);
    let plan = hashed::validate_plan(&snapshot, &request).expect("edit plan should validate");

    assert_eq!(plan.resource_class(), RESOURCE_CLASS_FILE);
    assert_eq!(plan.snapshot_hash(), HASH_A);
    assert_eq!(
        plan.newline_policy(),
        hashed::NewlinePolicy::PreserveSnapshot
    );
    assert_eq!(plan.spans().len(), HASHED_EDIT_COUNT);
    assert_eq!(plan.spans()[FIRST_INDEX].start_line(), FIRST_LINE_NUMBER);
    assert_eq!(plan.spans()[SECOND_INDEX].start_line(), THIRD_LINE_NUMBER);
    assert_eq!(plan.spans()[FIRST_INDEX].replacement(), REPLACEMENT_TEXT);
    assert!(!plan.plan_hash().is_empty());

    let receipt = hashed::Receipt::new(&hashed::ReceiptDraft {
        plan: &plan,
        after_snapshot_hash: HASH_D,
        resource_redaction: RESOURCE_REDACTION_HASH_LABEL,
        resource_identity: Some(RESOURCE_ID_HASH),
        diagnostics: &[DIAGNOSTIC_APPLIED],
    })
    .expect("receipt should validate");

    assert_eq!(
        receipt.resource_redaction(),
        hashed::ResourceRedaction::ResourceHash
    );
    assert_eq!(receipt.resource_identity(), Some(RESOURCE_ID_HASH));
    assert_eq!(receipt.before_snapshot_hash(), HASH_A);
    assert_eq!(receipt.after_snapshot_hash(), HASH_D);
    assert_eq!(receipt.spans().len(), HASHED_EDIT_COUNT);
    assert_eq!(receipt.diagnostics()[FIRST_INDEX], DIAGNOSTIC_APPLIED);
    assert!(!receipt.receipt_hash().is_empty());
}

#[test]
fn hashed_edit_validation_runs_without_host_adapter() {
    let snapshot = hashed_snapshot();
    let spans = [span_draft(SpanFixture {
        start_tag: TAG_FIRST,
        end_tag: TAG_FIRST,
        replacement: REPLACEMENT_TEXT,
    })];
    let request = hashed_request(&spans);
    let plan = hashed::validate_plan(&snapshot, &request)
        .expect("pure validation should not require a host adapter");

    assert_eq!(plan.spans().len(), SINGLE_HASHED_EDIT_COUNT);
    assert_eq!(plan.snapshot_hash(), HASH_A);
}
