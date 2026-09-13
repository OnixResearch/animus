mod admission;
mod attempts;
mod fixture;
mod normalization;
#[cfg(feature = "tool-streaming")]
mod sse;
const PROVIDER_ID: &str = "openai";
const MODEL_ID: &str = "gpt-5-mini";
const USER_TEXT: &str = "Reply with one safe word.";
const RESPONSE_TEXT: &str = "safe";
const REFUSAL_REASON: &str = "request refused";
const METADATA_KEY: &str = "trace_label";
const METADATA_VALUE: &str = "live-openai-fixture";
const CREDENTIAL_LABEL: &str = "env:ANIMUS_OPENAI_LIVE_API_KEY";
const API_KEY_VALUE: &str = "test-secret-value";
const TOOL_NAME: &str = "lookup";
const TOOL_DESCRIPTION: &str = "Look up one value.";
const TOOL_SCHEMA: &str = "schema.lookup";
const IMAGE_DESCRIPTOR: &str = "image-one";
const IMAGE_MEDIA_TYPE: &str = "image/png";
const IMAGE_HASH: &str = "BLAKE3:image";
const RESPONSE_SCHEMA: &str = "schema.answer";
const STOP_SEQUENCE: &str = "STOP";
const TIMEOUT_SECONDS: u64 = 30;
const MAX_RESPONSE_BYTES: u64 = 65_536;
const OVERSIZED_RESPONSE_BYTES: usize = 65_537;
const OUTPUT_LIMIT: u32 = 64;
const TEMPERATURE_MILLIS: u16 = 250;
const CONTEXT_BUDGET: u32 = 8_192;
const MODEL_OUTPUT_BUDGET: u32 = 4_096;
const RESPONSE_INPUT_TOKENS: u32 = 12;
const RESPONSE_OUTPUT_TOKENS: u32 = 4;
const RESPONSE_CACHED_TOKENS: u32 = 2;
const HTTP_OK: u16 = 200;
const HTTP_UNAUTHORIZED: u16 = 401;
const EXPECTED_CALLS_ONE: usize = 1;
const EXPECTED_CALLS_ZERO: usize = 0;
const FIRST_ITEM_INDEX: usize = 0;

const BASE_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "response:text",
    "reasoning:low",
    "metadata:trace_label=live-openai-fixture",
];
const STREAM_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "response:text",
    "reasoning:low",
    "stream:true",
    "metadata:trace_label=live-openai-fixture",
];
const TOOL_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "tool:lookup",
    "response:text",
    "reasoning:low",
    "metadata:trace_label=live-openai-fixture",
];
const IMAGE_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "image:descriptor",
    "response:text",
    "reasoning:low",
    "metadata:trace_label=live-openai-fixture",
];
const STRUCTURED_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with one safe word.",
    "response:schema.answer",
    "reasoning:low",
    "metadata:trace_label=live-openai-fixture",
];

/// `octet::sealed_enum`: this finite matrix names each supported fixture mutation.
/// Every new case needs its own request and expected projection.
#[derive(Clone, Copy)]
enum RequestCase {
    Base,
    Stream,
    Tool,
    Image,
    Structured,
    Stop,
}
