//! Deterministic bounded Responses decoding.

mod contracts;
mod engine;
mod fields;
mod reports;
mod vocabulary;

pub use contracts::{OpenAiStreamBounds, OpenAiStreamError, OpenAiStreamFailureClass};
pub use engine::{OpenAiResponsesSseParser, parse_openai_responses_sse_chunks};
pub use reports::{
    OpenAiRetryStreamProgress, OpenAiStreamOutcome, OpenAiStreamTerminal,
    render_openai_tool_stream_evidence,
};
