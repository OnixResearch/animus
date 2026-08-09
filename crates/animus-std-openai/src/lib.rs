//! Authority-scoped live `OpenAI` Responses host.
//!
//! The pure core admits one text-only, non-streaming `OpenAI` Responses cohort.
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

pub use core::{build_openai_request_body, normalize_openai_response};
#[cfg(any(feature = "reqwest-client", test))]
pub use model::OpenAiApiKey;
pub use model::{
    CancellationState, HostError, OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
    OFFICIAL_OPENAI_RESPONSES_URL, OpenAiHostConfig, TransportErrorClass,
};
#[cfg(feature = "reqwest-client")]
pub use shell::ReqwestOpenAiHost;

#[cfg(test)]
mod tests;
