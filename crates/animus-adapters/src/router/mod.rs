//! Forwarding shim to the dedicated `animus-router` crate.
//!
//! This module re-exports every public item from `animus-router` so that
//! existing dependents inside `animus-adapters` and the rest of the workspace
//! compile without churn. The shim will be removed once direct consumers
//! migrate to `animus-router` directly.

pub use animus_router::{
    Bounds, BoundsDraft, Budget, CatalogEntry, CatalogEntryDraft, DeadlineClass, DurationBucket,
    FallbackEligibility, FallbackPolicy, OutputContract, PrivacyBand, QuorumClass, Receipt,
    ReceiptDraft, ReceiptLeak, Recipe, RecipeDraft, Redaction, RequestMetadata,
    RequestMetadataDraft, ResultDraft, Role, RouteFamily, RouteResult, RouteResultStatus,
    RuleSignalSource, SelectedPlan, SelectionDenial, SignalBand, SignalDraft,
    SignalProjectionError, SignalSource, Signals, TaskShape, ValidationError, project_signals,
    select_recipe, validate_receipt_text,
};
