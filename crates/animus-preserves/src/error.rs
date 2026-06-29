/// Error returned by Preserves bridge operations.
#[derive(Debug)]
pub enum Failure {
    /// Adapter DTO validation failed while converting a decoded boundary message.
    AdapterContract {
        /// Adapter contract validation error.
        source: animus_adapters::AdapterContractError,
    },
    /// Runtime DTO validation failed while converting a decoded boundary message.
    RuntimeContract {
        /// Runtime contract validation error.
        source: animus_runtime::RuntimeError,
    },
    /// A decoded runtime payload used an unsupported tag or field value.
    InvalidRuntimePayload {
        /// Payload field name.
        field: &'static str,
        /// Human-readable validation reason.
        reason: &'static str,
    },
    /// Preserves serialization or deserialization failed.
    Preserves {
        /// Preserves codec error.
        source: preserves::error::Error,
    },
    /// Preserves text writer returned bytes that were not valid UTF-8.
    Utf8 {
        /// UTF-8 conversion error.
        source: std::string::FromUtf8Error,
    },
    /// Caller requested a different boundary message kind.
    WrongKind {
        /// Expected boundary kind.
        expected: &'static str,
        /// Actual boundary kind.
        actual: &'static str,
    },
}

impl Failure {
    pub(crate) fn wrong_kind(expected: crate::BoundaryKind, actual: crate::BoundaryKind) -> Self {
        Self::WrongKind {
            expected: expected.as_str(),
            actual: actual.as_str(),
        }
    }
}

impl From<animus_adapters::AdapterContractError> for Failure {
    fn from(source: animus_adapters::AdapterContractError) -> Self {
        Self::AdapterContract { source }
    }
}

impl From<animus_runtime::RuntimeError> for Failure {
    fn from(source: animus_runtime::RuntimeError) -> Self {
        Self::RuntimeContract { source }
    }
}

impl From<preserves::error::Error> for Failure {
    fn from(source: preserves::error::Error) -> Self {
        Self::Preserves { source }
    }
}

impl From<std::string::FromUtf8Error> for Failure {
    fn from(source: std::string::FromUtf8Error) -> Self {
        Self::Utf8 { source }
    }
}

impl std::fmt::Display for Failure {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AdapterContract { source } => {
                write!(formatter, "adapter contract failed: {source}")
            }
            Self::RuntimeContract { source } => {
                write!(formatter, "runtime contract failed: {source}")
            }
            Self::InvalidRuntimePayload { field, reason } => {
                write!(formatter, "invalid runtime payload field {field}: {reason}")
            }
            Self::Preserves { source } => write!(formatter, "preserves codec failed: {source}"),
            Self::Utf8 { source } => write!(formatter, "preserves text was not UTF-8: {source}"),
            Self::WrongKind { expected, actual } => {
                write!(
                    formatter,
                    "expected boundary message kind {expected}; got {actual}"
                )
            }
        }
    }
}

impl std::error::Error for Failure {}
