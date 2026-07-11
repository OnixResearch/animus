const MAX_COMPATIBILITY_FIELDS: usize = 8;
const MAX_FIELD_NAME_BYTES: usize = 64;
const MAX_FIELD_VALUE_BYTES: usize = 256;
const EMPTY_FIELD_REASON: &str = "compatibility metadata names must be non-empty";
const INVALID_NAME_REASON: &str =
    "compatibility metadata names must use safe identifier characters";
const UNBOUNDED_FIELD_REASON: &str =
    "compatibility metadata exceeds the bounded preservation limit";
const UNSAFE_FIELD_REASON: &str = "compatibility metadata must not preserve raw prompts, provider payloads, credentials, file bodies, tool bodies, or unbounded output";
const PRIVATE_PATH_REASON: &str = "compatibility metadata must not preserve full private paths";
const PROVIDER_JSON_REASON: &str = "compatibility metadata must not preserve provider payload JSON";
const JSON_OBJECT_START: char = '{';
const JSON_ARRAY_START: char = '[';
const SAFE_FIELD_DASH: char = '-';
const SAFE_FIELD_UNDERSCORE: char = '_';
const SAFE_FIELD_DOT: char = '.';
const PRIVATE_PATH_MARKERS: &[&str] = &["/home/", "/users/", ":\\"];
const UNSAFE_MARKERS: &[&str] = &[
    "raw_prompt",
    "provider_payload",
    "provider_json",
    "credential",
    "secret",
    "api_key",
    "file_body",
    "tool_input",
    "tool_output",
    "model_output",
    "unbounded_output",
    "unbounded_log",
    "begin private key",
];

/// Safe bounded compatibility metadata for adapter records.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct CompatibilityField {
    name: String,
    value: String,
}

impl CompatibilityField {
    /// Builds a compatibility field after validating its name and value.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] when the field is unsafe or unbounded.
    pub fn new(name: &str, value: &str) -> Result<Self, crate::Error> {
        let field = Self {
            name: String::from(name),
            value: String::from(value),
        };
        field.validate()?;
        Ok(field)
    }

    /// Returns the safe field name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the safe field value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    pub(crate) fn validate(&self) -> Result<(), crate::Error> {
        validate_name(&self.name)?;
        validate_value(&self.name, &self.value)
    }
}

pub(crate) fn validate_metadata(fields: &[CompatibilityField]) -> Result<(), crate::Error> {
    if fields.len() > MAX_COMPATIBILITY_FIELDS {
        return Err(crate::Error::TooManyCompatibilityFields {
            limit: MAX_COMPATIBILITY_FIELDS,
        });
    }
    for field in fields {
        field.validate()?;
    }
    Ok(())
}

fn validate_name(name: &str) -> Result<(), crate::Error> {
    if name.is_empty() {
        return Err(unsafe_metadata(EMPTY_FIELD_REASON));
    }
    if name.len() > MAX_FIELD_NAME_BYTES {
        return Err(unsafe_metadata(UNBOUNDED_FIELD_REASON));
    }
    if !name.chars().all(is_safe_name_character) {
        return Err(unsafe_metadata(INVALID_NAME_REASON));
    }
    if contains_unsafe_marker(name) {
        return Err(unsafe_metadata(UNSAFE_FIELD_REASON));
    }
    Ok(())
}

fn validate_value(name: &str, value: &str) -> Result<(), crate::Error> {
    if value.len() > MAX_FIELD_VALUE_BYTES {
        return Err(unsafe_metadata(UNBOUNDED_FIELD_REASON));
    }
    let normalized_name = name.to_ascii_lowercase();
    let normalized_value = value.to_ascii_lowercase();
    if contains_private_path(&normalized_value) {
        return Err(unsafe_metadata(PRIVATE_PATH_REASON));
    }
    if (normalized_name.contains("provider") || normalized_name.contains("payload"))
        && looks_like_json(value)
    {
        return Err(unsafe_metadata(PROVIDER_JSON_REASON));
    }
    if contains_unsafe_marker(&normalized_name) || contains_unsafe_marker(&normalized_value) {
        return Err(unsafe_metadata(UNSAFE_FIELD_REASON));
    }
    Ok(())
}

const fn is_safe_name_character(character: char) -> bool {
    character.is_ascii_alphanumeric()
        || character == SAFE_FIELD_DASH
        || character == SAFE_FIELD_UNDERSCORE
        || character == SAFE_FIELD_DOT
}

fn contains_private_path(value: &str) -> bool {
    PRIVATE_PATH_MARKERS
        .iter()
        .any(|marker| value.contains(marker))
}

fn contains_unsafe_marker(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    UNSAFE_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

fn looks_like_json(value: &str) -> bool {
    matches!(
        value.trim_start().chars().next(),
        Some(JSON_OBJECT_START | JSON_ARRAY_START)
    )
}

const fn unsafe_metadata(reason: &'static str) -> crate::Error {
    crate::Error::UnsafeCompatibilityMetadata { reason }
}
