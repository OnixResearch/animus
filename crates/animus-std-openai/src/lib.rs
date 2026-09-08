//! Authority-scoped live OpenAI-compatible Responses host.
//!
//! The default pure core admits reviewed text-only Responses cohorts. The
//! non-default `tool-streaming` feature adds exact function declarations,
//! correlated function outputs, and bounded production SSE parsing. The
//! optional shell owns one explicit admitted endpoint attempt. This crate does not discover
//! providers, models, endpoints, credentials, retries, or fallback targets.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

#[cfg(feature = "tool-streaming")]
mod chat;
mod core;
mod model;
#[cfg(any(feature = "reqwest-client", test))]
mod shell;
mod smoke;
#[cfg(feature = "tool-streaming")]
mod stream;
#[cfg(feature = "tool-streaming")]
mod tool;

#[cfg(feature = "tool-streaming")]
pub use chat::OpenAiChatCompletionsSseParser;
pub use core::{build_openai_request_body, normalize_openai_response};
#[cfg(any(feature = "reqwest-client", test))]
pub use model::OpenAiApiKey;
pub use model::{
    CancellationState, HostError, LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY,
    LOCAL_DSPARK_RESPONSES_URL, OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
    OFFICIAL_OPENAI_RESPONSES_URL, OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
    OPENROUTER_RESPONSES_URL, OpenAiHostConfig, TransportErrorClass,
    ZAI_CODING_PLAN_COMPLETIONS_ENDPOINT_IDENTITY, ZAI_CODING_PLAN_COMPLETIONS_URL,
};
#[cfg(feature = "reqwest-client")]
pub use shell::ReqwestOpenAiHost;
#[cfg(feature = "tool-streaming")]
pub use shell::{OpenAiToolStreamRequest, StreamingCancellation};
pub use smoke::{
    LOCAL_DSPARK_SMOKE_MODEL_ID, LiveOpenAiSmokeError, OPENROUTER_SMOKE_MODEL_ID,
    plan_live_local_dspark_smoke, plan_live_openai_smoke, plan_live_openrouter_smoke,
    render_live_local_dspark_smoke_evidence, render_live_openai_smoke_evidence,
    render_live_openrouter_smoke_evidence,
};
#[cfg(feature = "reqwest-client")]
pub use smoke::{
    run_packaged_live_local_dspark_smoke, run_packaged_live_openai_smoke,
    run_packaged_live_openrouter_smoke,
};
#[cfg(feature = "tool-streaming")]
pub use stream::{
    OpenAiResponsesSseParser, OpenAiRetryStreamProgress, OpenAiStreamBounds, OpenAiStreamError,
    OpenAiStreamFailureClass, OpenAiStreamOutcome, OpenAiStreamTerminal,
    parse_openai_responses_sse_chunks, render_openai_tool_stream_evidence,
};
#[cfg(feature = "tool-streaming")]
pub use tool::{
    OpenAiCompletedToolCall, OpenAiContinuationBinding, OpenAiToolCapability,
    OpenAiToolCapabilityMatrix, OpenAiToolChoice, OpenAiToolDispatch, OpenAiToolPolicy,
    OpenAiToolResult, OpenAiToolSchemaBinding, OpenAiToolStreamChannel, STEEL_REPL_TOOL_NAME,
    STEEL_REPL_TOOL_SCHEMA, build_tool_capable_openai_request_body,
};

#[cfg(test)]
mod tests;
