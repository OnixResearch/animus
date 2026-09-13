/// Exact endpoint identity admitted by the official `OpenAI` host cohort.
pub const OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY: &str = "openai-responses";
/// Exact HTTPS URL admitted by the official `OpenAI` host cohort.
pub const OFFICIAL_OPENAI_RESPONSES_URL: &str = "https://api.openai.com/v1/responses";
/// Exact endpoint identity admitted by the `OpenRouter` Responses cohort.
pub const OPENROUTER_RESPONSES_ENDPOINT_IDENTITY: &str = "openrouter-responses";
/// Exact HTTPS URL admitted by the `OpenRouter` Responses cohort.
pub const OPENROUTER_RESPONSES_URL: &str = "https://openrouter.ai/api/v1/responses";
/// Exact endpoint identity admitted by the private `local-dspark` cohort.
pub const LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY: &str = "local-dspark-responses";
/// Exact private HTTP URL admitted by the `local-dspark` cohort.
pub const LOCAL_DSPARK_RESPONSES_URL: &str = "http://192.168.1.244:8888/v1/responses";

const MAX_RESPONSE_BYTES: u64 = 16_777_216;
const DISABLED_REASONING_EFFORT: &str = "none";
const ENDPOINT_IDENTITY_REASON: &str = "Responses endpoint identity is not admitted";
const ENDPOINT_URL_REASON: &str = "Responses endpoint URL does not match its identity";
const TIMEOUT_REASON: &str = "OpenAI timeout must be non-zero";
const RESPONSE_LIMIT_REASON: &str = "OpenAI response limit is outside the supported range";

/// Explicit configuration for one admitted OpenAI-compatible Responses host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiHostConfig {
    endpoint_identity: String,
    endpoint_url: String,
    timeout: std::time::Duration,
    max_response_bytes: u64,
}

impl OpenAiHostConfig {
    /// Admits one exact endpoint pair and bounded transport configuration.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HostError`] when the endpoint, timeout, or response limit is not admitted.
    pub fn new(
        endpoint_identity: &str,
        endpoint_url: &str,
        timeout: std::time::Duration,
        max_response_bytes: u64,
    ) -> Result<Self, crate::HostError> {
        // r[impl agent_harness_runtime.live_openai_provider_host.boundary]
        validate_config(
            (endpoint_identity, endpoint_url),
            timeout,
            max_response_bytes,
        )?;
        Ok(Self {
            endpoint_identity: String::from(endpoint_identity),
            endpoint_url: String::from(endpoint_url),
            timeout,
            max_response_bytes,
        })
    }

    /// Returns the exact endpoint identity.
    #[must_use]
    pub fn endpoint_identity(&self) -> &str {
        self.endpoint_identity.as_str()
    }

    /// Returns the exact endpoint URL.
    #[must_use]
    pub fn endpoint_url(&self) -> &str {
        self.endpoint_url.as_str()
    }

    #[cfg(any(test, feature = "reqwest-client"))]
    pub(crate) fn https_only(&self) -> bool {
        self.endpoint_identity != LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY
    }

    pub(crate) fn projected_reasoning_effort<'a>(&self, effort: &'a str) -> Option<&'a str> {
        if effort == DISABLED_REASONING_EFFORT
            && self.endpoint_identity != LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY
        {
            None
        } else {
            Some(effort)
        }
    }

    /// Returns the explicit request timeout.
    #[must_use]
    pub const fn timeout(&self) -> std::time::Duration {
        self.timeout
    }

    /// Returns the admitted maximum response bytes.
    #[must_use]
    pub const fn max_response_bytes(&self) -> u64 {
        self.max_response_bytes
    }
}

fn validate_config(
    endpoint: (&str, &str),
    timeout: std::time::Duration,
    max_response_bytes: u64,
) -> Result<(), crate::HostError> {
    // r[impl agent_harness_runtime.packaged_live_openrouter_smoke.endpoint]
    // r[impl agent_harness_runtime.local_dspark_responses.endpoint]
    // r[impl agent_harness_runtime.local_dspark_responses.transport]
    let (endpoint_identity, endpoint_url) = endpoint;
    let expected_url = match endpoint_identity {
        OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY => Ok(OFFICIAL_OPENAI_RESPONSES_URL),
        OPENROUTER_RESPONSES_ENDPOINT_IDENTITY => Ok(OPENROUTER_RESPONSES_URL),
        LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY => Ok(LOCAL_DSPARK_RESPONSES_URL),
        _unsupported => Err(crate::HostError::InvalidConfiguration {
            reason: ENDPOINT_IDENTITY_REASON,
        }),
    }?;
    if endpoint_url != expected_url {
        return Err(crate::HostError::InvalidConfiguration {
            reason: ENDPOINT_URL_REASON,
        });
    }
    if timeout.is_zero() {
        return Err(crate::HostError::InvalidConfiguration {
            reason: TIMEOUT_REASON,
        });
    }
    if max_response_bytes == 0 || max_response_bytes > MAX_RESPONSE_BYTES {
        return Err(crate::HostError::InvalidConfiguration {
            reason: RESPONSE_LIMIT_REASON,
        });
    }
    assert_eq!(endpoint_url, expected_url);
    assert!(!timeout.is_zero());
    Ok(())
}
