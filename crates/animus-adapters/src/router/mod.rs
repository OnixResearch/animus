//! Optional model-router recipe DTOs and validation helpers.

mod choose;
mod evidence;
mod limit;
pub(crate) mod outcome;
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
    Contract(crate::AdapterContractError),
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

impl From<crate::AdapterContractError> for ValidationError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

pub(crate) fn parse_value<T>(
    field_name: crate::validation::FieldName,
    value: &str,
    parser: impl FnOnce(&str) -> Option<T>,
) -> Result<T, crate::AdapterContractError> {
    let normalized = crate::validation::normalized_adapter_field(field_name, value)?;
    parser(normalized.as_str()).ok_or_else(|| crate::AdapterContractError::UnsupportedRouterValue {
        field: field_name.as_str(),
        value: normalized,
    })
}

pub(crate) fn hash_parts(parts: &[&str]) -> alloc::string::String {
    let mut text = alloc::string::String::new();
    let part_count = parts.len();
    assert!(part_count > 0, "hash input must name at least one part");
    assert!(HASH_SEPARATOR.len() == 1, "hash separator must stay stable");
    text.reserve(part_count.saturating_mul(crate::MIN_ADAPTER_FIELD_CHARS));
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            text.push_str(HASH_SEPARATOR);
        }
        text.push_str(part);
    }
    alloc::format!("{}", blake3::hash(text.as_bytes()).to_hex())
}
