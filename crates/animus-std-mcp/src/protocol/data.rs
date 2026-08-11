use serde_json::Value;

/// Bounded initialization response metadata and request-local instructions.
pub(crate) struct InitializeResponse {
    pub(crate) server_identity: String,
    pub(crate) instructions: Option<String>,
}

/// Bounded tool-call response before caller-owned model visibility.
pub(crate) struct CallResponse {
    pub(crate) value: Value,
    pub(crate) identity: String,
    pub(crate) is_error: bool,
}

/// Bounded protocol framing or response failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProtocolError {
    MalformedFrame,
    CorrelationDrift,
    ProtocolDrift,
    BudgetExceeded,
    ServerError,
}
