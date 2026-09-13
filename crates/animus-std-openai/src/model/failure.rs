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
    /// The admitted endpoint client could not be constructed.
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
    /// The transport shell failed without exposing raw transport data.
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
    /// Exact tool-cohort metadata was invalid.
    InvalidToolCohort {
        /// Safe metadata failure reason.
        reason: &'static str,
    },
    /// A reviewed JSON tool schema was invalid.
    InvalidToolSchema {
        /// Safe schema failure reason.
        reason: &'static str,
    },
    /// Provider continuation identities or committed tool-result facts did not match.
    Correlation {
        /// Safe correlation failure reason.
        reason: &'static str,
    },
    /// The production Responses SSE parser rejected the stream.
    #[cfg(feature = "tool-streaming")]
    Stream {
        /// Safe parser failure.
        source: crate::OpenAiStreamError,
    },
    /// Existing canonical provider normalization rejected the projection.
    CanonicalNormalization {
        /// Safe provider dispatch error.
        source: animus_std::ProviderDispatchError,
    },
}

impl std::fmt::Display for HostError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfiguration { reason }
            | Self::InvalidCredential { reason }
            | Self::MalformedResponse { reason }
            | Self::InvalidToolCohort { reason }
            | Self::InvalidToolSchema { reason }
            | Self::Correlation { reason } => formatter.write_str(reason),
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
            #[cfg(feature = "tool-streaming")]
            Self::Stream { source } => write!(formatter, "OpenAI stream failed: {source}"),
            Self::CanonicalNormalization { source } => {
                write!(formatter, "OpenAI response normalization failed: {source}")
            }
        }
    }
}

impl std::error::Error for HostError {}
