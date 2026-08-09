use crate::{
    CancellationState, HostError, OpenAiApiKey, OpenAiHostConfig, TransportErrorClass,
    build_openai_request_body, normalize_openai_response,
};

const HTTP_SUCCESS_START: u16 = 200;
const HTTP_SUCCESS_END_EXCLUSIVE: u16 = 300;

pub(crate) struct WireRequest<'a> {
    pub(crate) endpoint_url: &'a str,
    pub(crate) body: Vec<u8>,
    pub(crate) max_response_bytes: u64,
}

pub(crate) struct WireResponse {
    pub(crate) status: u16,
    pub(crate) body: Vec<u8>,
}

pub(crate) trait Transport {
    fn send(
        &self,
        request: &WireRequest<'_>,
        api_key: &OpenAiApiKey,
    ) -> Result<WireResponse, TransportErrorClass>;
}

#[derive(Clone, Copy)]
pub(crate) struct ExecutionRequest<'a> {
    pub(crate) config: &'a OpenAiHostConfig,
    pub(crate) plan: &'a animus_std::ProviderDispatchPlan,
    pub(crate) api_key: &'a OpenAiApiKey,
    pub(crate) cancellation: CancellationState,
}

pub(crate) fn execute_with_transport<T: Transport>(
    transport: &T,
    input: ExecutionRequest<'_>,
) -> Result<animus_std::ProviderDispatchOutcome, HostError> {
    if input.cancellation == CancellationState::Cancelled {
        return Err(HostError::Cancelled);
    }
    if input.api_key.source_label() != input.plan.credential_source_label() {
        return Err(HostError::CredentialBindingMismatch);
    }
    let body = build_openai_request_body(input.config, input.plan)?;
    let request = WireRequest {
        endpoint_url: input.config.endpoint_url(),
        body,
        max_response_bytes: input.config.max_response_bytes(),
    };
    let response = transport
        .send(&request, input.api_key)
        .map_err(|class| HostError::Transport { class })?;
    if !(HTTP_SUCCESS_START..HTTP_SUCCESS_END_EXCLUSIVE).contains(&response.status) {
        return Err(HostError::HttpStatus {
            status: response.status,
        });
    }
    normalize_openai_response(
        input.plan,
        response.body.as_slice(),
        input.config.max_response_bytes(),
    )
}

/// One explicit blocking HTTPS client for the admitted `OpenAI` Responses endpoint.
#[cfg(feature = "reqwest-client")]
pub struct ReqwestOpenAiHost {
    config: OpenAiHostConfig,
    client: reqwest::blocking::Client,
}

#[cfg(feature = "reqwest-client")]
impl ReqwestOpenAiHost {
    /// Builds the exact bounded HTTPS client without redirects or retries.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when the HTTPS client cannot be constructed.
    pub fn new(config: OpenAiHostConfig) -> Result<Self, HostError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(config.timeout())
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .https_only(true)
            .build()
            .map_err(|_error| HostError::Transport {
                class: TransportErrorClass::ClientConfiguration,
            })?;
        Ok(Self { config, client })
    }

    /// Sends one admitted `OpenAI` Responses request.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] for admission, cancellation, transport, status, bound, or response failures.
    pub fn execute(
        &self,
        plan: &animus_std::ProviderDispatchPlan,
        api_key: &OpenAiApiKey,
        cancellation: CancellationState,
    ) -> Result<animus_std::ProviderDispatchOutcome, HostError> {
        // r[impl agent_harness_runtime.live_openai_provider_host.transport]
        // r[impl agent_harness_runtime.live_openai_provider_host.transport.single_attempt]
        execute_with_transport(
            self,
            ExecutionRequest {
                config: &self.config,
                plan,
                api_key,
                cancellation,
            },
        )
    }

    /// Returns the admitted host configuration.
    #[must_use]
    pub const fn config(&self) -> &OpenAiHostConfig {
        &self.config
    }
}

#[cfg(feature = "reqwest-client")]
impl Transport for ReqwestOpenAiHost {
    fn send(
        &self,
        request: &WireRequest<'_>,
        api_key: &OpenAiApiKey,
    ) -> Result<WireResponse, TransportErrorClass> {
        use std::io::Read as _;

        let response = self
            .client
            .post(request.endpoint_url)
            .bearer_auth(api_key.expose_value())
            .header(reqwest::header::ACCEPT, "application/json")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(request.body.clone())
            .send()
            .map_err(|error| classify_reqwest_error(&error))?;
        let status = response.status().as_u16();
        let read_limit = request
            .max_response_bytes
            .checked_add(1)
            .ok_or(TransportErrorClass::ResponseRead)?;
        let mut bounded = response.take(read_limit);
        let mut body = Vec::new();
        bounded
            .read_to_end(&mut body)
            .map_err(|_error| TransportErrorClass::ResponseRead)?;
        Ok(WireResponse { status, body })
    }
}

#[cfg(feature = "reqwest-client")]
fn classify_reqwest_error(error: &reqwest::Error) -> TransportErrorClass {
    if error.is_timeout() {
        TransportErrorClass::Timeout
    } else {
        TransportErrorClass::Unavailable
    }
}
