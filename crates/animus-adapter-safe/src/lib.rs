//! Independent no-std safe receipt contract family for Animus adapter boundaries.
//!
//! This crate contains implementation-free DTOs, validation helpers, leak
//! classes, bounded counts, and BLAKE3 artifact identities for safe receipts.
//! Hosts still own storage, clocks, filesystems, processes, networks,
//! credentials, terminal state, and receipt persistence.

#![no_std]
#![cfg_attr(dylint_lib = "octet", feature(register_tool))]
#![cfg_attr(dylint_lib = "octet", register_tool(tigerstyle))]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::string::ToString;

const FIELD_ARTIFACT_HASH: &str = "artifact_hash";
const HASH_LENGTH_REASON: &str = "artifact hash must use lowercase BLAKE3 hex";
const BLAKE3_HASH_BYTES: usize = 32;
const HEX_CHARS_PER_BYTE: usize = 2;
const HEX_RADIX: usize = 16;
const HEX_NIBBLE_SHIFT: u8 = 4;
const HEX_LOW_NIBBLE_MASK: u8 = 0x0f;
const LOWER_HEX: &[u8; HEX_RADIX] = b"0123456789abcdef";
const REVIEW_PREFIX: &str = "safe_receipt";
const REVIEW_SEPARATOR: char = ' ';
const MAX_COLLECTION_ITEMS: usize = 32;
const NUL_CHARACTER: char = '\0';
const TOKEN_PREFIX: &str = "sk-";
const GITHUB_TOKEN_PREFIX: &str = "ghp_";
const TOKEN_ASSIGNMENT_MARKER: &str = "token=";
const API_KEY_MARKER: &str = "api_key";
const API_KEY_DASH_MARKER: &str = "api-key";
const BEARER_MARKER: &str = "bearer ";
const RAW_PROMPT_MARKER: &str = "prompt:";
const SYSTEM_PROMPT_MARKER: &str = "system prompt";
const USER_PROMPT_MARKER: &str = "user:";
const PROVIDER_CHOICES_MARKER: &str = "\"choices\"";
const PROVIDER_MESSAGES_MARKER: &str = "\"messages\"";
const PROVIDER_MODEL_MARKER: &str = "\"model\"";
const TOOL_INPUT_MARKER: &str = "tool_input:";
const TOOL_INPUT_TEXT_MARKER: &str = "tool input:";
const TOOL_OUTPUT_MARKER: &str = "tool_output:";
const TOOL_OUTPUT_TEXT_MARKER: &str = "tool output:";
const MODEL_OUTPUT_MARKER: &str = "model_output:";
const MODEL_OUTPUT_TEXT_MARKER: &str = "model output:";
const ASSISTANT_OUTPUT_MARKER: &str = "assistant:";
const ROOT_PATH_MARKER: &str = "/";
const HOME_PATH_MARKER: &str = "/home/";
const TMP_PATH_MARKER: &str = "/tmp/";
const ETC_PATH_MARKER: &str = "/etc/";
const USER_PATH_MARKER: &str = "\\users\\";
const JSON_OBJECT_PREFIX: &str = "{";
const LABELS_FIELD: &str = "receipt_labels";
const COUNTS_FIELD: &str = "receipt_counts";
const HASHES_FIELD: &str = "receipt_artifact_hashes";
const REDACTIONS_FIELD: &str = "receipt_redactions";
const BACKEND_LABEL_FIELD: MetadataField = MetadataField::new("receipt_backend_label");
const INTERCHANGE_LABEL_FIELD: MetadataField = MetadataField::new("receipt_interchange_label");
const LABEL_FIELD: MetadataField = MetadataField::new("receipt_label");
const COUNT_LABEL_FIELD: MetadataField = MetadataField::new("receipt_count_label");

const PROMPT_MARKERS: &[&str] = &[RAW_PROMPT_MARKER, SYSTEM_PROMPT_MARKER, USER_PROMPT_MARKER];
const PROVIDER_MARKERS: &[&str] = &[
    PROVIDER_CHOICES_MARKER,
    PROVIDER_MESSAGES_MARKER,
    PROVIDER_MODEL_MARKER,
];
const CREDENTIAL_MARKERS: &[&str] = &[
    TOKEN_PREFIX,
    GITHUB_TOKEN_PREFIX,
    TOKEN_ASSIGNMENT_MARKER,
    API_KEY_MARKER,
    API_KEY_DASH_MARKER,
    BEARER_MARKER,
];
const PATH_MARKERS: &[&str] = &[
    HOME_PATH_MARKER,
    TMP_PATH_MARKER,
    ETC_PATH_MARKER,
    USER_PATH_MARKER,
];
const TOOL_INPUT_MARKERS: &[&str] = &[TOOL_INPUT_MARKER, TOOL_INPUT_TEXT_MARKER];
const TOOL_OUTPUT_MARKERS: &[&str] = &[TOOL_OUTPUT_MARKER, TOOL_OUTPUT_TEXT_MARKER];
const MODEL_OUTPUT_MARKERS: &[&str] = &[
    MODEL_OUTPUT_MARKER,
    MODEL_OUTPUT_TEXT_MARKER,
    ASSISTANT_OUTPUT_MARKER,
];

/// Schema identifier attached to safe receipt DTOs.
pub const SCHEMA: &str = "animus.safe-receipt.v1";
/// Largest accepted safe metadata label length.
pub const MAX_LABEL_CHARS: usize = 256;
/// Largest accepted bounded count value in a safe receipt.
pub const MAX_COUNT: u32 = 1_000_000;
/// Character length of a lowercase BLAKE3 artifact hash in hex form.
pub const BLAKE3_HASH_HEX_CHARS: usize = BLAKE3_HASH_BYTES.saturating_mul(HEX_CHARS_PER_BYTE);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MetadataField {
    name: &'static str,
}

impl MetadataField {
    const fn new(name: &'static str) -> Self {
        Self { name }
    }

    const fn name(self) -> &'static str {
        self.name
    }
}

fn ensure_required_collection(field: &'static str, actual_items: usize) -> Result<(), Error> {
    ensure_optional_collection(field, actual_items)?;
    if actual_items == 0 {
        return Err(Error::EmptyCollection { field });
    }
    Ok(())
}

const fn ensure_optional_collection(field: &'static str, actual_items: usize) -> Result<(), Error> {
    if actual_items > MAX_COLLECTION_ITEMS {
        return Err(Error::CollectionTooLarge {
            field,
            max_items: MAX_COLLECTION_ITEMS,
            actual_items,
        });
    }
    Ok(())
}

fn optional_label(
    field: MetadataField,
    value: Option<&str>,
) -> Result<Option<alloc::string::String>, Error> {
    value.map(|label| required_label(field, label)).transpose()
}

fn required_label(field: MetadataField, value: &str) -> Result<alloc::string::String, Error> {
    assert!(
        !field.name().is_empty(),
        "metadata field name must not be empty"
    );
    assert!(
        value.len() >= value.chars().count(),
        "label byte length must cover character count"
    );
    reject_oversized(field, value)?;
    reject_nul(field, value)?;
    let normalized = value.trim();
    reject_empty(field, normalized)?;
    reject_unsafe(field, normalized)?;
    Ok(normalized.to_string())
}

// Animus receipt validation counts Unicode characters through non-const Iterator methods.
#[cfg_attr(dylint_lib = "octet", allow(tigerstyle::missing_const_fn))]
fn reject_oversized(field: MetadataField, value: &str) -> Result<(), Error> {
    let actual_chars = value.chars().count();
    if actual_chars > MAX_LABEL_CHARS {
        return Err(Error::UnsafeMetadata {
            field: field.name(),
            class: LeakClass::UnboundedOutput,
        });
    }
    Ok(())
}

// Animus receipt validation preserves byte offsets through non-const char_indices and find.
#[cfg_attr(dylint_lib = "octet", allow(tigerstyle::missing_const_fn))]
fn reject_nul(field: MetadataField, value: &str) -> Result<(), Error> {
    if let Some((byte_index, _character)) = value
        .char_indices()
        .find(|(_byte_index, character)| *character == NUL_CHARACTER)
    {
        return Err(Error::NulCharacter {
            field: field.name(),
            byte_index,
        });
    }
    Ok(())
}

fn reject_empty(field: MetadataField, value: &str) -> Result<(), Error> {
    if value.is_empty() {
        return Err(Error::EmptyField {
            field: field.name(),
        });
    }
    Ok(())
}

fn reject_unsafe(field: MetadataField, value: &str) -> Result<(), Error> {
    if let Some(class) = leak::classify(value) {
        return Err(Error::UnsafeMetadata {
            field: field.name(),
            class,
        });
    }
    Ok(())
}

// Animus receipt rendering must grow the caller's buffer; a borrowed str cannot provide capacity.
#[cfg_attr(dylint_lib = "octet", allow(tigerstyle::borrowed_argument_types))]
fn push_review_value(text: &mut alloc::string::String, value: &str) {
    if !text.is_empty() {
        text.push(REVIEW_SEPARATOR);
    }
    text.push_str(value);
}

mod classes;
mod digest;
mod fault;
mod leak;
mod receipt;

pub use classes::{IssueCode, LeakClass, Operation, Redaction, Status};
pub use digest::ArtifactHash;
pub use fault::Error;
pub use receipt::{BoundedCount, BoundedCountDraft, Draft, Record};

#[cfg(test)]
#[path = "tests/mod.rs"]
mod tests;
