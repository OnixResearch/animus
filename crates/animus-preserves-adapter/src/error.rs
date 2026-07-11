/// Fail-closed adapter Preserves bridge error without raw payload material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Text syntax could not be decoded.
    MalformedText,
    /// Binary syntax could not be decoded.
    MalformedBinary,
    /// A typed record could not be serialized.
    Serialization,
    /// The schema version is unsupported.
    UnsupportedSchemaVersion {
        /// Received version.
        version: u16,
        /// Supported version.
        supported: u16,
    },
    /// The record declares a non-adapter family.
    WrongSchemaFamily {
        /// Received family label.
        actual: &'static str,
    },
    /// Compatibility metadata has too many fields.
    TooManyCompatibilityFields {
        /// Maximum supported field count.
        limit: usize,
    },
    /// Compatibility metadata is unsafe or malformed.
    UnsafeCompatibilityMetadata {
        /// Safe diagnostic reason that never includes the value.
        reason: &'static str,
    },
    /// The decoded adapter payload failed its owning DTO validator.
    InvalidAdapterPayload {
        /// Stable payload kind.
        kind: &'static str,
        /// Safe diagnostic reason.
        reason: &'static str,
    },
}

impl core::fmt::Display for Error {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MalformedText => formatter.write_str("malformed Preserves adapter text"),
            Self::MalformedBinary => formatter.write_str("malformed Preserves adapter binary"),
            Self::Serialization => {
                formatter.write_str("failed to serialize Preserves adapter record")
            }
            Self::UnsupportedSchemaVersion { version, supported } => write!(
                formatter,
                "unsupported adapter schema version {version}; supported version is {supported}"
            ),
            Self::WrongSchemaFamily { actual } => write!(
                formatter,
                "expected adapter_message schema family; got {actual}"
            ),
            Self::TooManyCompatibilityFields { limit } => write!(
                formatter,
                "compatibility metadata exceeds the {limit}-field limit"
            ),
            Self::UnsafeCompatibilityMetadata { reason } => formatter.write_str(reason),
            Self::InvalidAdapterPayload { kind, reason } => {
                write!(formatter, "invalid {kind} adapter payload: {reason}")
            }
        }
    }
}

impl std::error::Error for Error {}
