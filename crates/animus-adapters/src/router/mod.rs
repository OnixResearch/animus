//! Forwarding shim to the dedicated `animus-router` crate.
//!
//! This module re-exports every public item from `animus-router` so that
//! existing dependents inside `animus-adapters` and the rest of the workspace
//! compile without churn. The shim will be removed once direct consumers
//! migrate to `animus-router` directly.

pub use animus_router::{
    Bounds, BoundsDraft, Budget, CatalogEntry, CatalogEntryDraft, DeadlineClass, DurationBucket,
    FallbackEligibility, FallbackPolicy, OutputContract, PrivacyBand, QuorumClass, Receipt,
    ReceiptDraft, ReceiptLeak, Recipe, RecipeDraft, Redaction, ResultDraft, Role, RouteFamily,
    RouteResult, RouteResultStatus, SelectedPlan, SelectionDenial, SignalBand, SignalDraft,
    Signals, TaskShape, ValidationError, select_recipe, validate_receipt_text,
};
