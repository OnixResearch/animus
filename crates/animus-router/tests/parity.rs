//! Parity tests verifying that every public item re-exported from
//! `animus-adapters::router` resolves to the live `animus-router` type
//! with the same API shape after the crate-extraction barrier.

const EXPECTED_ROUTER_ITEM_COUNT: usize = 30;
const EXPECTED_ALIAS_COUNT: usize = 17;

fn assert_type<T>() {}

/// Every re-exported type name compiles from the original path.
#[test]
fn adapter_reexport_types_resolve() {
    assert_type::<animus_adapters::router::Budget>();
    assert_type::<animus_adapters::router::Bounds>();
    assert_type::<animus_adapters::router::CatalogEntry>();
    assert_type::<animus_adapters::router::DeadlineClass>();
    assert_type::<animus_adapters::router::DurationBucket>();
    assert_type::<animus_adapters::router::FallbackEligibility>();
    assert_type::<animus_adapters::router::FallbackPolicy>();
    assert_type::<animus_adapters::router::OutputContract>();
    assert_type::<animus_adapters::router::PrivacyBand>();
    assert_type::<animus_adapters::router::QuorumClass>();
    assert_type::<animus_adapters::router::Receipt>();
    assert_type::<animus_adapters::router::Redaction>();
    assert_type::<animus_adapters::router::Recipe>();
    assert_type::<animus_adapters::router::Role>();
    assert_type::<animus_adapters::router::RouteFamily>();
    assert_type::<animus_adapters::router::SelectedPlan>();
    assert_type::<animus_adapters::router::SelectionDenial>();
    assert_type::<animus_adapters::router::SignalBand>();
    assert_type::<animus_adapters::router::Signals>();
    assert_type::<animus_adapters::router::TaskShape>();
    assert_type::<animus_adapters::router::RouteResult>();
    assert_type::<animus_adapters::router::RouteResultStatus>();
    assert_type::<animus_adapters::router::ValidationError>();
    assert_type::<animus_adapters::router::ReceiptLeak>();
    assert_type::<animus_adapters::router::BoundsDraft<'_>>();
    assert_type::<animus_adapters::router::CatalogEntryDraft<'_>>();
    assert_type::<animus_adapters::router::ReceiptDraft<'_>>();
    assert_type::<animus_adapters::router::RecipeDraft<'_>>();
    assert_type::<animus_adapters::router::SignalDraft<'_>>();
    assert_type::<animus_adapters::router::ResultDraft<'_>>();

    let _select_recipe = animus_adapters::router::select_recipe;
    let _validate_receipt_text = animus_adapters::router::validate_receipt_text;

    assert_eq!(EXPECTED_ROUTER_ITEM_COUNT, router_item_names().len());
}

/// Every re-exported type also compiles from the direct `animus_router` path.
#[test]
fn direct_router_types_resolve() {
    assert_type::<animus_router::Budget>();
    assert_type::<animus_router::Bounds>();
    assert_type::<animus_router::CatalogEntry>();
    assert_type::<animus_router::DeadlineClass>();
    assert_type::<animus_router::DurationBucket>();
    assert_type::<animus_router::FallbackEligibility>();
    assert_type::<animus_router::FallbackPolicy>();
    assert_type::<animus_router::OutputContract>();
    assert_type::<animus_router::PrivacyBand>();
    assert_type::<animus_router::QuorumClass>();
    assert_type::<animus_router::Receipt>();
    assert_type::<animus_router::Redaction>();
    assert_type::<animus_router::Recipe>();
    assert_type::<animus_router::Role>();
    assert_type::<animus_router::RouteFamily>();
    assert_type::<animus_router::SelectedPlan>();
    assert_type::<animus_router::SelectionDenial>();
    assert_type::<animus_router::SignalBand>();
    assert_type::<animus_router::Signals>();
    assert_type::<animus_router::TaskShape>();
    assert_type::<animus_router::RouteResult>();
    assert_type::<animus_router::RouteResultStatus>();
    assert_type::<animus_router::ValidationError>();
    assert_type::<animus_router::ReceiptLeak>();
    assert_type::<animus_router::BoundsDraft<'_>>();
    assert_type::<animus_router::CatalogEntryDraft<'_>>();
    assert_type::<animus_router::ReceiptDraft<'_>>();
    assert_type::<animus_router::RecipeDraft<'_>>();
    assert_type::<animus_router::SignalDraft<'_>>();
    assert_type::<animus_router::ResultDraft<'_>>();

    let _select_recipe = animus_router::select_recipe;
    let _validate_receipt_text = animus_router::validate_receipt_text;

    assert_eq!(EXPECTED_ROUTER_ITEM_COUNT, router_item_names().len());
}

/// Verifies the facade type aliases in `animus-adapters` still compile.
#[test]
fn facade_aliases_resolve() {
    assert_type::<animus_adapters::ExecutionBounds>();
    assert_type::<animus_adapters::ExecutionBoundsDraft<'_>>();
    assert_type::<animus_adapters::ModelCatalogEntry>();
    assert_type::<animus_adapters::ModelCatalogEntryDraft<'_>>();
    assert_type::<animus_adapters::ModelRole>();
    assert_type::<animus_adapters::RequestSignalDraft<'_>>();
    assert_type::<animus_adapters::RequestSignals>();
    assert_type::<animus_adapters::RouterBudget>();
    assert_type::<animus_adapters::RouterReceipt>();
    assert_type::<animus_adapters::RouterRedaction>();
    assert_type::<animus_adapters::RouterRecipe>();
    assert_type::<animus_adapters::RouterRecipeDraft<'_>>();
    assert_type::<animus_adapters::RouterValidationError>();
    assert_type::<animus_adapters::SelectedRoutePlan>();
    assert_type::<animus_adapters::TimeoutClass>();
    assert_type::<animus_adapters::RouteResultDraft<'_>>();
    assert_type::<animus_adapters::RouterReceiptLeak>();

    let _validate_router_receipt_text = animus_adapters::validate_router_receipt_text;

    assert_eq!(EXPECTED_ALIAS_COUNT, alias_names().len());
}

fn router_item_names() -> [&'static str; EXPECTED_ROUTER_ITEM_COUNT] {
    [
        "Budget",
        "Bounds",
        "CatalogEntry",
        "DeadlineClass",
        "DurationBucket",
        "FallbackEligibility",
        "FallbackPolicy",
        "OutputContract",
        "PrivacyBand",
        "QuorumClass",
        "Receipt",
        "Redaction",
        "Recipe",
        "Role",
        "RouteFamily",
        "SelectedPlan",
        "SelectionDenial",
        "SignalBand",
        "Signals",
        "TaskShape",
        "RouteResult",
        "RouteResultStatus",
        "ValidationError",
        "ReceiptLeak",
        "BoundsDraft",
        "CatalogEntryDraft",
        "ReceiptDraft",
        "RecipeDraft",
        "SignalDraft",
        "ResultDraft",
    ]
}

fn alias_names() -> [&'static str; EXPECTED_ALIAS_COUNT] {
    [
        "ExecutionBounds",
        "ExecutionBoundsDraft",
        "ModelCatalogEntry",
        "ModelCatalogEntryDraft",
        "ModelRole",
        "RequestSignalDraft",
        "RequestSignals",
        "RouterBudget",
        "RouterReceipt",
        "RouterRedaction",
        "RouterRecipe",
        "RouterRecipeDraft",
        "RouterValidationError",
        "SelectedRoutePlan",
        "TimeoutClass",
        "RouteResultDraft",
        "RouterReceiptLeak",
    ]
}
