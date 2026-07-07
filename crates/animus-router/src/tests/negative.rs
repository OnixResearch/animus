use super::*;

#[derive(Clone)]
struct BoundsCase {
    max_candidates: u32,
    min_success_quorum: u32,
    max_repair_attempts: Option<u32>,
    expected: crate::ValidationError,
}

const UPPERCASE_POLICY_HASH: &str =
    "A000000000000000000000000000000000000000000000000000000000000000";

#[test]
fn recipe_contracts_reject_invalid_shapes() {
    assert!(matches!(
        crate::Recipe::new(crate::RecipeDraft {
            id: RECIPE_SINGLE,
            family: "single",
            roles: &["primary", "primary"],
            output_contract: "json",
            max_candidates: MAX_CANDIDATES_ONE,
            max_parallelism: MAX_PARALLELISM_ONE,
            min_success_quorum: QUORUM_ONE,
            timeout_class: "standard",
            max_synthesis_attempts: SYNTHESIS_NONE,
            max_repair_attempts: Some(REPAIR_NONE),
            fallback_policy: "deny",
        }),
        Err(crate::ValidationError::DuplicateRole { .. })
    ));
    assert!(matches!(
        crate::Role::parse(BAD_VALUE),
        Err(crate::AdapterContractError::UnsupportedRouterValue { .. })
    ));
    assert!(matches!(
        crate::OutputContract::parse(BAD_VALUE),
        Err(crate::AdapterContractError::UnsupportedRouterValue { .. })
    ));
    assert!(matches!(
        crate::RouteFamily::parse(BAD_VALUE),
        Err(crate::AdapterContractError::UnsupportedRouterValue { .. })
    ));
}

#[test]
fn signal_projection_rejects_unsupported_labels() {
    let error = crate::Signals::project(crate::SignalDraft {
        task_shape: BAD_VALUE,
        ..signal_draft()
    })
    .expect_err("unsupported task shape should fail");

    assert!(matches!(
        error,
        crate::AdapterContractError::UnsupportedRouterValue { .. }
    ));
}

#[test]
fn deterministic_selection_denies_unsafe_eligibility() {
    let local_only = signals(crate::SignalDraft {
        risk_band: "strict",
        privacy_band: "local_only",
        ..signal_draft()
    });
    let recipes = [recipe(RecipeFixture {
        id: RECIPE_FUSION,
        family: "fusion",
        roles: &["judge", "finalizer"],
    })];
    let catalog = catalog();

    assert_eq!(
        crate::select_recipe(&local_only, &recipes, &catalog, &budget()),
        Err(crate::SelectionDenial::NoEligibleModel)
    );
}

#[test]
fn execution_bounds_reject_invalid_plans() {
    let cases = [
        BoundsCase {
            max_candidates: ZERO_COUNT,
            min_success_quorum: QUORUM_ONE,
            max_repair_attempts: Some(REPAIR_NONE),
            expected: crate::ValidationError::UnboundedFanOut,
        },
        BoundsCase {
            max_candidates: MAX_CANDIDATES_ONE,
            min_success_quorum: ZERO_COUNT,
            max_repair_attempts: Some(REPAIR_NONE),
            expected: crate::ValidationError::ZeroQuorum,
        },
        BoundsCase {
            max_candidates: MAX_CANDIDATES_ONE,
            min_success_quorum: QUORUM_TWO,
            max_repair_attempts: Some(REPAIR_NONE),
            expected: crate::ValidationError::ImpossibleQuorum,
        },
        BoundsCase {
            max_candidates: MAX_CANDIDATES_ONE,
            min_success_quorum: QUORUM_ONE,
            max_repair_attempts: None,
            expected: crate::ValidationError::UnlimitedRepair,
        },
    ];

    assert_eq!(cases.len(), 4);
    assert!(cases.iter().any(|case| case.max_candidates == ZERO_COUNT));
    for case in &cases {
        assert_bounds_error(case);
    }
}

#[test]
fn router_receipt_leak_fixtures_fail_closed() {
    let cases = [
        ("raw prompt: hello", crate::ReceiptLeak::RawPrompt),
        ("provider json { }", crate::ReceiptLeak::ProviderPayload),
        ("worker output body", crate::ReceiptLeak::WorkerOutput),
        ("judge rationale body", crate::ReceiptLeak::JudgeRationale),
        ("hidden reasoning text", crate::ReceiptLeak::HiddenReasoning),
        ("api_key=secret", crate::ReceiptLeak::Credential),
        ("/home/user/private", crate::ReceiptLeak::FullPath),
        (
            "unbounded trace follows",
            crate::ReceiptLeak::UnboundedTrace,
        ),
    ];

    for (text, expected) in cases {
        assert_eq!(crate::validate_receipt_text(text), Err(expected));
    }
    assert!(
        crate::validate_receipt_text("recipe=remom quorum=met redactions=trace_bounded").is_ok()
    );
}

#[test]
fn router_receipt_rejects_uppercase_policy_hash() {
    let error = crate::Receipt::new(crate::ReceiptDraft {
        recipe_id: RECIPE_REMOM,
        signal_buckets: &["reasoning"],
        selected_models: &[MODEL_WORKER],
        worker_count: WORKER_COUNT,
        success_count: SUCCESS_COUNT,
        quorum_class: "met",
        fallback_policy: "deny",
        policy_hash: UPPERCASE_POLICY_HASH,
        duration_bucket: "bounded",
        redactions: &["prompt_omitted"],
    });

    assert!(matches!(
        error,
        Err(crate::ValidationError::Contract(
            crate::AdapterContractError::InvalidEvidenceHash { .. }
        ))
    ));
}

fn assert_bounds_error(case: &BoundsCase) {
    assert_eq!(
        crate::Bounds::new(crate::BoundsDraft {
            max_candidates: case.max_candidates,
            max_parallelism: MAX_PARALLELISM_ONE,
            min_success_quorum: case.min_success_quorum,
            timeout_class: "standard",
            max_synthesis_attempts: SYNTHESIS_NONE,
            max_repair_attempts: case.max_repair_attempts,
            fallback_policy: "deny",
        }),
        Err(case.expected.clone())
    );
}
