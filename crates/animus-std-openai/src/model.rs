use std::fmt;
use std::time::Duration;

#[cfg(any(feature = "reqwest-client", test))]
use zeroize::Zeroizing;

/// Exact endpoint identity admitted by the first live `OpenAI` host cohort.
pub const OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY: &str = "openai-responses";
/// Exact HTTPS URL admitted by the first live `OpenAI` host cohort.
pub const OFFICIAL_OPENAI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";

#[cfg(any(feature = "reqwest-client", test))]
const MAX_API_KEY_CHARS: usize = 4_096;
const KIBIBYTE_BYTES: u64 = 1_024;
const MEBIBYTE_BYTES: u64 = KIBIBYTE_BYTES * KIBIBYTE_BYTES;
const MAX_RESPONSE_MEBIBYTES: u64 = 16;
const MAX_RESPONSE_BYTES: u64 = MAX_RESPONSE_MEBIBYTES * MEBIBYTE_BYTES;
#[cfg(any(feature = "reqwest-client", test))]
const EMPTY_API_KEY_REASON: &str = "OpenAI API key must be non-empty";
#[cfg(any(feature = "reqwest-client", test))]
const LONG_API_KEY_REASON: &str = "OpenAI API key exceeds the supported length";
#[cfg(any(feature = "reqwest-client", test))]
const UNSAFE_API_KEY_REASON: &str = "OpenAI API key contains whitespace or control characters";
#[cfg(any(feature = "reqwest-client", test))]
const EMPTY_CREDENTIAL_LABEL_REASON: &str = "credential source label must be non-empty";
const ENDPOINT_IDENTITY_REASON: &str = "OpenAI endpoint identity is not admitted";
const ENDPOINT_URL_REASON: &str = "OpenAI endpoint URL is not admitted";
const TIMEOUT_REASON: &str = "OpenAI timeout must be non-zero";
const RESPONSE_LIMIT_REASON: &str = "OpenAI response limit is outside the supported range";

/// Explicit cancellation observation for one provider attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationState {
    /// The caller admits provider contact.
    Active,
    /// The caller cancelled before provider contact.
    Cancelled,
}

/// Safe transport failure class without provider bodies or credentials.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportErrorClass {
    /// The HTTPS client could not be constructed.
    ClientConfiguration,
    /// The explicit deadline expired.
    Timeout,
    /// The request failed before a response was available.
    Unavailable,
    /// The bounded response body could not be read.
    ResponseRead,
}

/// Typed failure from live `OpenAI` admission, transport, or normalization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HostError {
    /// Explicit host configuration was not admitted.
    InvalidConfiguration {
        /// Safe configuration failure reason.
        reason: &'static str,
    },
    /// The supplied API key was not admitted.
    InvalidCredential {
        /// Safe credential failure reason.
        reason: &'static str,
    },
    /// The credential source does not match the admitted dispatch plan.
    CredentialBindingMismatch,
    /// Cancellation blocked provider contact.
    Cancelled,
    /// The plan requests a field that the live cohort cannot preserve.
    UnsupportedPlan {
        /// Stable unsupported feature label.
        feature: &'static str,
    },
    /// Deterministic request JSON encoding failed.
    RequestEncoding,
    /// The HTTPS shell failed without exposing raw transport data.
    Transport {
        /// Safe transport failure class.
        class: TransportErrorClass,
    },
    /// The provider returned a non-success status.
    HttpStatus {
        /// Numeric HTTP status.
        status: u16,
    },
    /// The bounded response exceeded the admitted byte limit.
    ResponseTooLarge {
        /// Admitted maximum response bytes.
        limit: u64,
    },
    /// The successful response shape was malformed or unsupported.
    MalformedResponse {
        /// Safe response failure reason.
        reason: &'static str,
    },
    /// Existing canonical provider normalization rejected the projection.
    CanonicalNormalization {
        /// Safe provider dispatch error.
        source: animus_std::ProviderDispatchError,
    },
}

impl fmt::Display for HostError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidConfiguration { reason }
            | Self::InvalidCredential { reason }
            | Self::MalformedResponse { reason } => formatter.write_str(reason),
            Self::CredentialBindingMismatch => {
                formatter.write_str("credential source does not match provider dispatch plan")
            }
            Self::Cancelled => formatter.write_str("OpenAI dispatch cancelled before contact"),
            Self::UnsupportedPlan { feature } => {
                write!(formatter, "unsupported live OpenAI feature: {feature}")
            }
            Self::RequestEncoding => formatter.write_str("OpenAI request encoding failed"),
            Self::Transport { class } => write!(formatter, "OpenAI transport failed: {class:?}"),
            Self::HttpStatus { status } => {
                write!(formatter, "OpenAI returned HTTP status {status}")
            }
            Self::ResponseTooLarge { limit } => {
                write!(formatter, "OpenAI response exceeded {limit} bytes")
            }
            Self::CanonicalNormalization { source } => {
                write!(formatter, "OpenAI response normalization failed: {source}")
            }
        }
    }
}

impl std::error::Error for HostError {}

/// Explicit configuration for the live `OpenAI` host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiHostConfig {
    endpoint_identity: String,
    endpoint_url: String,
    timeout: Duration,
    max_response_bytes: u64,
}

impl OpenAiHostConfig {
    /// Admits one exact `OpenAI` endpoint and bounded transport configuration.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when the endpoint, timeout, or response limit is not admitted.
    pub fn new(
        endpoint_identity: &str,
        endpoint_url: &str,
        timeout: Duration,
        max_response_bytes: u64,
    ) -> Result<Self, HostError> {
        // r[impl agent_harness_runtime.live_openai_provider_host.boundary]
        validate_config(endpoint_identity, endpoint_url, timeout, max_response_bytes)?;
        Ok(Self {
            endpoint_identity: String::from(endpoint_identity),
            endpoint_url: String::from(endpoint_url),
            timeout,
            max_response_bytes,
        })
    }

    /// Returns the exact endpoint identity.
    #[must_use]
    pub fn endpoint_identity(&self) -> &str {
        self.endpoint_identity.as_str()
    }

    /// Returns the exact HTTPS endpoint URL.
    #[must_use]
    pub fn endpoint_url(&self) -> &str {
        self.endpoint_url.as_str()
    }

    /// Returns the explicit request timeout.
    #[must_use]
    pub const fn timeout(&self) -> Duration {
        self.timeout
    }

    /// Returns the admitted maximum response bytes.
    #[must_use]
    pub const fn max_response_bytes(&self) -> u64 {
        self.max_response_bytes
    }
}

/// Explicit `OpenAI` API key and its safe source label.
///
/// The key value is non-cloneable, redacted from debug output, and zeroized on drop.
#[cfg(any(feature = "reqwest-client", test))]
pub struct OpenAiApiKey {
    source_label: String,
    value: Zeroizing<String>,
}

#[cfg(any(feature = "reqwest-client", test))]
impl OpenAiApiKey {
    /// Admits an explicitly supplied API key.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when the source label or value is invalid.
    pub fn new(source_label: &str, value: String) -> Result<Self, HostError> {
        if source_label.is_empty() {
            return Err(HostError::InvalidCredential {
                reason: EMPTY_CREDENTIAL_LABEL_REASON,
            });
        }
        if value.is_empty() {
            return Err(HostError::InvalidCredential {
                reason: EMPTY_API_KEY_REASON,
            });
        }
        if value.chars().count() > MAX_API_KEY_CHARS {
            return Err(HostError::InvalidCredential {
                reason: LONG_API_KEY_REASON,
            });
        }
        if value
            .chars()
            .any(|character| character.is_whitespace() || character.is_control())
        {
            return Err(HostError::InvalidCredential {
                reason: UNSAFE_API_KEY_REASON,
            });
        }
        Ok(Self {
            source_label: String::from(source_label),
            value: Zeroizing::new(value),
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
impl fmt::Debug for OpenAiApiKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenAiApiKey")
            .field("source_label", &self.source_label)
            .field("value", &"[REDACTED]")
            .finish()
    }
}

fn validate_config(
    endpoint_identity: &str,
    endpoint_url: &str,
    timeout: Duration,
    max_response_bytes: u64,
) -> Result<(), HostError> {
    if endpoint_identity != OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY {
        return Err(HostError::InvalidConfiguration {
            reason: ENDPOINT_IDENTITY_REASON,
        });
    }
    if endpoint_url != OFFICIAL_OPENAI_RESPONSES_URL {
        return Err(HostError::InvalidConfiguration {
            reason: ENDPOINT_URL_REASON,
        });
    }
    if timeout.is_zero() {
        return Err(HostError::InvalidConfiguration {
            reason: TIMEOUT_REASON,
        });
    }
    if max_response_bytes == 0 || max_response_bytes > MAX_RESPONSE_BYTES {
        return Err(HostError::InvalidConfiguration {
            reason: RESPONSE_LIMIT_REASON,
        });
    }
    Ok(())
}
