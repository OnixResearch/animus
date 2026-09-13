pub(super) const EVENT_SEPARATOR: &[u8] = b"\n\n";
pub(super) const CRLF_EVENT_SEPARATOR: &[u8] = b"\r\n\r\n";
pub(super) const EVENT_PREFIX: &str = "event:";
pub(super) const DATA_PREFIX: &str = "data:";
pub(super) const STREAM_DONE: &str = "[DONE]";
pub(super) const FIRST_ORDER: u64 = 1;
pub(super) const ORDER_INCREMENT: u64 = 1;
pub(super) const MAX_STREAM_CHUNKS: u64 = 65_536;
pub(super) const MAX_STREAM_EVENTS: u64 = 65_536;
pub(super) const MAX_EVENT_BYTES: u64 = 1_048_576;
pub(super) const MAX_TOOL_ARGUMENT_BYTES: u64 = 262_144;
pub(super) const MAX_TOOL_CALLS: u64 = 64;

pub(super) const MALFORMED_SSE_REASON: &str = "OpenAI Responses SSE is malformed";
pub(super) const EVENT_NAME_MISMATCH_REASON: &str =
    "OpenAI Responses SSE event name does not match JSON type";
pub(super) const UNKNOWN_EVENT_REASON: &str =
    "OpenAI Responses SSE event is outside the reviewed cohort";
pub(super) const RESPONSE_ID_REASON: &str =
    "OpenAI Responses SSE response identity is missing or mismatched";
pub(super) const RESPONSE_LIFECYCLE_REASON: &str = "OpenAI Responses SSE lifecycle is invalid";
pub(super) const SEQUENCE_REASON: &str = "OpenAI Responses SSE sequence is invalid";
pub(super) const TERMINAL_REASON: &str = "OpenAI Responses SSE terminal state is invalid";
pub(super) const TERMINAL_BODY_REASON: &str = "OpenAI Responses SSE terminal response is malformed";
pub(super) const TOOL_CORRELATION_REASON: &str =
    "OpenAI Responses SSE function-call correlation is invalid";
pub(super) const TOOL_ARGUMENT_REASON: &str =
    "OpenAI Responses SSE function-call arguments are invalid";
pub(super) const TOOL_ARGUMENT_BOUND_REASON: &str =
    "OpenAI Responses SSE function-call arguments exceed the bound";
pub(super) const TOOL_CALL_BOUND_REASON: &str =
    "OpenAI Responses SSE function-call count exceeds the bound";
pub(super) const STREAM_BOUND_REASON: &str = "OpenAI Responses SSE exceeds a stream bound";
pub(super) const DUPLICATE_USAGE_REASON: &str = "OpenAI Responses SSE emitted duplicate usage";
pub(super) const REASONING_EVENT_REASON: &str =
    "OpenAI Responses SSE reasoning event is unsafe or unsupported";
pub(super) const PROVIDER_FAILURE_REASON: &str = "openai_provider_failure";
pub(super) const PROVIDER_INCOMPLETE_REASON: &str = "openai_provider_incomplete";
pub(super) const PROVIDER_ERROR_REASON: &str = "openai_provider_error";
pub(super) const CANCELLED_REASON: &str = "cancelled";
