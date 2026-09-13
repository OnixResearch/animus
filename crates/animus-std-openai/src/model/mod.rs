mod admission;
#[cfg(any(feature = "reqwest-client", test))]
mod credential;
mod failure;
pub use admission::{
    LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY, LOCAL_DSPARK_RESPONSES_URL,
    OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY, OFFICIAL_OPENAI_RESPONSES_URL,
    OPENROUTER_RESPONSES_ENDPOINT_IDENTITY, OPENROUTER_RESPONSES_URL, OpenAiHostConfig,
};
#[cfg(any(feature = "reqwest-client", test))]
pub use credential::OpenAiApiKey;
pub use failure::{CancellationState, HostError, TransportErrorClass};
