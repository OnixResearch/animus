const MAX_IDENTIFIER_BYTES: usize = 64;
const ASCII_LOWERCASE_A: char = 'a';
const ASCII_LOWERCASE_Z: char = 'z';
const ASCII_UPPERCASE_A: char = 'A';
const ASCII_UPPERCASE_Z: char = 'Z';
const ASCII_DIGIT_ZERO: char = '0';
const ASCII_DIGIT_NINE: char = '9';
const ID_DASH: char = '-';
const ID_UNDERSCORE: char = '_';
const ID_DOT: char = '.';
const ID_COLON: char = ':';

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum IdField {
    Provider,
    Model,
    CredentialSource,
}

impl IdField {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Provider => super::PROVIDER_ID_FIELD,
            Self::Model => super::MODEL_ID_FIELD,
            Self::CredentialSource => super::CREDENTIAL_SOURCE_FIELD,
        }
    }
}

/// Validated provider identifier.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ProviderId(String);

impl ProviderId {
    /// Builds a provider id after shell-boundary validation.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when the id is empty, too long, or unsafe.
    pub fn new(raw: &str) -> Result<Self, super::RegistryError> {
        validate_id(IdField::Provider, raw)?;
        Ok(Self(String::from(raw)))
    }

    /// Returns the validated id text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Validated model identifier.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ModelId(String);

impl ModelId {
    /// Builds a model id after shell-boundary validation.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when the id is empty, too long, or unsafe.
    pub fn new(raw: &str) -> Result<Self, super::RegistryError> {
        validate_id(IdField::Model, raw)?;
        Ok(Self(String::from(raw)))
    }

    /// Returns the validated id text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

pub(super) fn validate_id(field: IdField, raw: &str) -> Result<(), super::RegistryError> {
    let field_label = field.as_str();
    debug_assert!(!field_label.is_empty());
    debug_assert!(field_label.is_ascii());

    if raw.is_empty() {
        return Err(super::RegistryError::InvalidIdentifier {
            field: field_label,
            reason: super::INVALID_ID_EMPTY_REASON,
        });
    }

    if raw.len() > MAX_IDENTIFIER_BYTES {
        return Err(super::RegistryError::InvalidIdentifier {
            field: field_label,
            reason: super::INVALID_ID_LENGTH_REASON,
        });
    }

    if !raw.chars().all(is_safe_id_char) {
        return Err(super::RegistryError::InvalidIdentifier {
            field: field_label,
            reason: super::INVALID_ID_CHAR_REASON,
        });
    }

    Ok(())
}

fn is_safe_id_char(character: char) -> bool {
    matches!(
        character,
        ASCII_LOWERCASE_A..=ASCII_LOWERCASE_Z
            | ASCII_UPPERCASE_A..=ASCII_UPPERCASE_Z
            | ASCII_DIGIT_ZERO..=ASCII_DIGIT_NINE
            | ID_DASH
            | ID_UNDERSCORE
            | ID_DOT
            | ID_COLON
    )
}
