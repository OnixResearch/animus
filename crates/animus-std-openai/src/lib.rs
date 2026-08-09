//! Authority-scoped live OpenAI-compatible Responses host.
//!
//! The pure core admits reviewed text-only, non-streaming Responses cohorts.
//! The optional `reqwest-client` shell owns one explicit HTTPS attempt. This
//! crate does not discover providers, models, endpoints, credentials, retries,
//! fallbacks, tools, image payloads, structured schemas, or streaming services.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod core;
mod model;
#[cfg(any(feature = "reqwest-client", test))]
mod shell;
mod smoke;

pub use core::{build_openai_request_body, normalize_openai_response};
#[cfg(any(feature = "reqwest-client", test))]
pub use model::OpenAiApiKey;
pub use model::{
    CancellationState, HostError, OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
    OFFICIAL_OPENAI_RESPONSES_URL, OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
    OPENROUTER_RESPONSES_URL, OpenAiHostConfig, TransportErrorClass,
};
#[cfg(feature = "reqwest-client")]
pub use shell::ReqwestOpenAiHost;
pub use smoke::{
    LiveOpenAiSmokeError, plan_live_openai_smoke, plan_live_openrouter_smoke,
    render_live_openai_smoke_evidence, render_live_openrouter_smoke_evidence,
};
#[cfg(feature = "reqwest-client")]
pub use smoke::{run_packaged_live_openai_smoke, run_packaged_live_openrouter_smoke};

#[cfg(test)]
mod tests;
