//! `#![no_std]` model-router decision core for `animus`.
//!
//! This crate owns recipe/role/output-contract DTOs, signal projection, recipe
//! selection, execution-plan materialization, route-result aggregation, and
//! safe receipt assembly. It defines its own validation helpers locally to avoid a circular
//! dependency on `animus-adapters` and depends on the BLAKE3 workspace crate for receipt
//! identity.

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

// ── Local validation helpers (inlined to avoid module-resolution issues) ──────

use alloc::string::String;

const NUL_CHARACTER: char = '\0';

/// Smallest accepted non-empty adapter field length after trimming.
pub(crate) const MIN_ADAPTER_FIELD_CHARS: usize = 1;

/// Largest accepted adapter field length.
pub(crate) const MAX_ADAPTER_FIELD_CHARS: usize = 8_192;

const _: () = assert!(MAX_ADAPTER_FIELD_CHARS >= MIN_ADAPTER_FIELD_CHARS);

/// A validated adapter field name constant used for error diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FieldName {
    value: &'static str,
}

impl FieldName {
    #[must_use]
    pub(crate) const fn new(value: &'static str) -> Self {
        Self { value }
    }

    #[must_use]
    pub(crate) const fn as_str(self) -> &'static str {
        self.value
    }
}

/// Contract errors raised before data crosses a router boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AdapterContractError {
    /// A required field contained no user-visible characters.
    EmptyField {
        /// Boundary field name.
        field: &'static str,
    },
    /// A field exceeded [`MAX_ADAPTER_FIELD_CHARS`].
    FieldTooLong {
        /// Boundary field name.
        field: &'static str,
        /// Maximum accepted character count.
        max_chars: usize,
        /// Actual character count.
        actual_chars: usize,
    },
    /// A field contained a NUL character.
    NulCharacter {
        /// Boundary field name.
        field: &'static str,
        /// Byte index where the NUL character starts.
        byte_index: usize,
    },
    /// A model-router field value was not supported.
    UnsupportedRouterValue {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported value after normalization.
        value: String,
    },
    /// A required collection contained no values.
    EmptyCollection {
        /// Boundary collection field name.
        field: &'static str,
    },
    /// A safe evidence hash failed deterministic validation.
    InvalidEvidenceHash {
        /// Boundary field name.
        field: &'static str,
        /// Human-readable validation reason.
        reason: &'static str,
    },
}

impl core::fmt::Display for AdapterContractError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptyField { field } | Self::EmptyCollection { field } => {
                write!(formatter, "{field} must not be empty")
            }
            Self::FieldTooLong {
                field,
                max_chars,
                actual_chars,
            } => write!(
                formatter,
                "{field} must be at most {max_chars} characters; got {actual_chars}"
            ),
            Self::NulCharacter { field, byte_index } => write!(
                formatter,
                "{field} contains a NUL character at byte index {byte_index}"
            ),
            Self::UnsupportedRouterValue { field, value } => write!(
                formatter,
                "{field} contains unsupported model-router value {value}"
            ),
            Self::InvalidEvidenceHash { field, reason } => {
                write!(formatter, "{field} is invalid: {reason}")
            }
        }
    }
}

/// Normalizes an adapter field value by trimming whitespace and rejecting
/// empty, too-long, or NUL-containing inputs.
pub(crate) fn normalized_adapter_field(
    field_name: FieldName,
    field_text: &str,
) -> Result<String, AdapterContractError> {
    let raw_chars = field_text.chars().count();
    let visible_chars = field_text.trim().chars().count();
    assert!(
        visible_chars <= raw_chars,
        "trimmed field character count must fit in raw character count"
    );

    if visible_chars < MIN_ADAPTER_FIELD_CHARS {
        return Err(AdapterContractError::EmptyField {
            field: field_name.as_str(),
        });
    }

    if raw_chars > MAX_ADAPTER_FIELD_CHARS {
        return Err(AdapterContractError::FieldTooLong {
            field: field_name.as_str(),
            max_chars: MAX_ADAPTER_FIELD_CHARS,
            actual_chars: raw_chars,
        });
    }

    if let Some((byte_index, _character)) = field_text
        .char_indices()
        .find(|(_byte_index, character)| *character == NUL_CHARACTER)
    {
        return Err(AdapterContractError::NulCharacter {
            field: field_name.as_str(),
            byte_index,
        });
    }

    Ok(String::from(field_text.trim()))
}

/// Safe BLAKE3 hash used by router receipts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SafeHash {
    value: String,
}

impl SafeHash {
    /// Character length of a lowercase BLAKE3 receipt hash in hex form.
    pub(crate) const HEX_CHARS: usize = 64;

    /// Builds a validated receipt hash.
    pub(crate) fn new(value: &str) -> Result<Self, AdapterContractError> {
        let field = FieldName::new("router_effect_receipt_hash");
        validate_hash(field, value)?;
        Ok(Self {
            value: String::from(value),
        })
    }

    #[must_use]
    pub(crate) fn as_str(&self) -> &str {
        self.value.as_str()
    }
}

fn validate_hash(field: FieldName, value: &str) -> Result<(), AdapterContractError> {
    if value.is_empty() {
        return Err(AdapterContractError::EmptyField {
            field: field.as_str(),
        });
    }
    if value.len() != SafeHash::HEX_CHARS {
        return Err(AdapterContractError::InvalidEvidenceHash {
            field: field.as_str(),
            reason: "effect hash must use the configured BLAKE3 hex length",
        });
    }
    if !value
        .chars()
        .all(|character| character.is_ascii_hexdigit() && !character.is_ascii_uppercase())
    {
        return Err(AdapterContractError::InvalidEvidenceHash {
            field: field.as_str(),
            reason: "effect hash must contain lowercase hex only",
        });
    }
    Ok(())
}

// ── Sub-modules ──────────────────────────────────────────────────────────────

mod choose;
mod limit;
pub(crate) mod outcome;
mod projection;
mod schema;

/// Evidence module (edition-2024 file-style entry point).
mod evidence;

#[cfg(test)]
#[path = "tests/mod.rs"]
mod test_support;

// ── Public re-exports ────────────────────────────────────────────────────────

pub use choose::{
    Budget, CatalogEntry, CatalogEntryDraft, SelectedPlan, SelectionDenial, select_recipe,
};
pub use evidence::{
    DurationBucket, QuorumClass, Receipt, ReceiptDraft, ReceiptLeak, Redaction,
    validate_receipt_text,
};
pub use limit::{Bounds, BoundsDraft, DeadlineClass, FallbackPolicy};
pub use outcome::{ResultDraft, RouteResult, RouteResultStatus};
pub use projection::{
    FallbackEligibility, PrivacyBand, SignalBand, SignalDraft, Signals, TaskShape,
};
pub use schema::{OutputContract, Recipe, RecipeDraft, Role, RouteFamily};

// ── Shared helpers ───────────────────────────────────────────────────────────

const HASH_SEPARATOR: &str = "|";

/// Router validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationError {
    /// Generic adapter field contract error.
    Contract(AdapterContractError),
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

impl From<AdapterContractError> for ValidationError {
    fn from(error: AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

pub(crate) fn parse_value<T>(
    field_name: FieldName,
    value: &str,
    parser: impl FnOnce(&str) -> Option<T>,
) -> Result<T, AdapterContractError> {
    let normalized = normalized_adapter_field(field_name, value)?;
    parser(normalized.as_str()).ok_or_else(|| AdapterContractError::UnsupportedRouterValue {
        field: field_name.as_str(),
        value: normalized,
    })
}

pub(crate) fn hash_parts(parts: &[&str]) -> alloc::string::String {
    let mut text = alloc::string::String::new();
    let part_count = parts.len();
    assert!(part_count > 0, "hash input must name at least one part");
    assert!(HASH_SEPARATOR.len() == 1, "hash separator must stay stable");
    text.reserve(part_count.saturating_mul(MIN_ADAPTER_FIELD_CHARS));
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            text.push_str(HASH_SEPARATOR);
        }
        text.push_str(part);
    }
    alloc::format!("{}", blake3::hash(text.as_bytes()).to_hex())
}
