#[cfg(any(feature = "reqwest-client", test))]
const MAX_API_KEY_CHARS: usize = 4_096;
#[cfg(any(feature = "reqwest-client", test))]
const EMPTY_API_KEY_REASON: &str = "OpenAI API key must be non-empty";
#[cfg(any(feature = "reqwest-client", test))]
const LONG_API_KEY_REASON: &str = "OpenAI API key exceeds the supported length";
#[cfg(any(feature = "reqwest-client", test))]
const UNSAFE_API_KEY_REASON: &str = "OpenAI API key contains whitespace or control characters";
#[cfg(any(feature = "reqwest-client", test))]
const EMPTY_CREDENTIAL_LABEL_REASON: &str = "credential source label must be non-empty";
/// Explicit `OpenAI` API key and its safe source label.
///
/// The key value is non-cloneable, redacted from debug output, and zeroized on drop.
#[cfg(any(feature = "reqwest-client", test))]
pub struct OpenAiApiKey {
    source_label: String,
    value: zeroize::Zeroizing<String>,
}

#[cfg(any(feature = "reqwest-client", test))]
impl OpenAiApiKey {
    /// Admits an explicitly supplied API key.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HostError`] when the source label or value is invalid.
    pub fn new(source_label: &str, value: String) -> Result<Self, crate::HostError> {
        if source_label.is_empty() {
            return Err(crate::HostError::InvalidCredential {
                reason: EMPTY_CREDENTIAL_LABEL_REASON,
            });
        }
        if value.is_empty() {
            return Err(crate::HostError::InvalidCredential {
                reason: EMPTY_API_KEY_REASON,
            });
        }
        if value.chars().count() > MAX_API_KEY_CHARS {
            return Err(crate::HostError::InvalidCredential {
                reason: LONG_API_KEY_REASON,
            });
        }
        if value
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
        {
            return Err(crate::HostError::InvalidCredential {
                reason: UNSAFE_API_KEY_REASON,
            });
        }
        Ok(Self {
            source_label: String::from(source_label),
            value: zeroize::Zeroizing::new(value),
        })
    }

    /// Returns the safe credential source label.
    #[must_use]
    pub fn source_label(&self) -> &str {
        self.source_label.as_str()
    }

    pub(crate) fn expose_value(&self) -> &str {
        self.value.as_str()
    }
}

#[cfg(any(feature = "reqwest-client", test))]
impl std::fmt::Debug for OpenAiApiKey {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("OpenAiApiKey")
            .field("source_label", &self.source_label)
            .field("value", &"[REDACTED]")
            .finish()
    }
}
