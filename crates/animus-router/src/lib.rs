//! Optional model-router recipe DTOs and validation helpers.
//!
//! This crate is `#![no_std]` with `alloc` only. It owns routing decision
//! logic — recipe/role/output-contract DTOs, signal projection, recipe
//! selection, execution-plan materialization, route-result aggregation,
//! and safe receipt assembly. `animus-adapters` remains the home for
//! implementation-free model/tool/port contracts.

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

mod choose;
mod evidence;
mod limit;
mod outcome;
mod projection;
mod schema;

#[cfg(test)]
mod tests;

pub use choose::{
    Budget, CatalogEntry, CatalogEntryDraft, SelectedPlan, SelectionDenial, select_recipe,
};
pub use evidence::{
    DurationBucket, QuorumClass, Receipt, ReceiptDraft, ReceiptLeak, Redaction,
    validate_receipt_text,
};
pub use limit::{Bounds, BoundsDraft, DeadlineClass, FallbackPolicy};
pub use outcome::{RouteResult, RouteResultStatus};
pub use projection::{
    FallbackEligibility, PrivacyBand, SignalBand, SignalDraft, Signals, TaskShape,
};
pub use schema::{OutputContract, Recipe, RecipeDraft, Role, RouteFamily};

const HASH_SEPARATOR: &str = "|";

/// Router validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationError {
    /// Generic adapter field contract error.
    Contract(animus_adapters::AdapterContractError),
    /// Required role list was empty.
    EmptyRoles,
    /// A role appeared more than once.
    DuplicateRole {
        /// Duplicated role.
        role: Role,
    },
    /// Fan-out or parallelism was unbounded or zero.
    UnboundedFanOut,
    /// Quorum was zero.
    ZeroQuorum,
    /// Quorum required more successes than possible candidates.
    ImpossibleQuorum,
    /// Repair attempts were omitted and therefore unlimited.
    UnlimitedRepair,
    /// Fallback shape was disallowed by bounds.
    FallbackDisallowed,
}

impl From<animus_adapters::AdapterContractError> for ValidationError {
    fn from(error: animus_adapters::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

fn parse_value<T>(
    field_name: animus_adapters::FieldName,
    value: &str,
    parser: impl FnOnce(&str) -> Option<T>,
) -> Result<T, animus_adapters::AdapterContractError> {
    let normalized = animus_adapters::normalized_adapter_field(field_name, value)?;
    parser(normalized.as_str()).ok_or_else(|| {
        animus_adapters::AdapterContractError::UnsupportedRouterValue {
            field: field_name.as_str(),
            value: normalized,
        }
    })
}

fn hash_parts(parts: &[&str]) -> alloc::string::String {
    let mut text = alloc::string::String::new();
    let part_count = parts.len();
    assert!(part_count > 0, "hash input must name at least one part");
    assert!(HASH_SEPARATOR.len() == 1, "hash separator must stay stable");
    text.reserve(part_count.saturating_mul(animus_adapters::MIN_ADAPTER_FIELD_CHARS));
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            text.push_str(HASH_SEPARATOR);
        }
        text.push_str(part);
    }
    alloc::format!("{}", blake3::hash(text.as_bytes()).to_hex())
}