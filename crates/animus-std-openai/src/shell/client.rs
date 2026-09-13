/// One explicit blocking client for an admitted Responses endpoint.
#[cfg(feature = "reqwest-client")]
pub struct ReqwestOpenAiHost {
    config: crate::OpenAiHostConfig,
    client: reqwest::blocking::Client,
}

#[cfg(feature = "reqwest-client")]
impl ReqwestOpenAiHost {
    /// Builds the exact bounded client without redirects or retries.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HostError`] when the client cannot be constructed.
    pub fn new(config: crate::OpenAiHostConfig) -> Result<Self, crate::HostError> {
        let client = reqwest::blocking::Client::builder()
            .timeout(config.timeout())
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .https_only(config.https_only())
            .build()
            .map_err(|_error| crate::HostError::Transport {
                class: crate::TransportErrorClass::ClientConfiguration,
            })?;
        Ok(Self { config, client })
    }

    /// Sends one admitted `OpenAI` Responses request.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HostError`] for admission, cancellation, transport, status, bound, or response failures.
    pub fn execute(
        &self,
        plan: &animus_std::ProviderDispatchPlan,
        api_key: &crate::OpenAiApiKey,
        cancellation: crate::CancellationState,
    ) -> Result<animus_std::ProviderDispatchOutcome, crate::HostError> {
        // r[impl agent_harness_runtime.live_openai_provider_host.transport]
        // r[impl agent_harness_runtime.live_openai_provider_host.transport.single_attempt]
        super::execute_with_transport(
            self,
            super::ExecutionRequest {
                config: &self.config,
                plan,
                api_key,
                cancellation,
            },
        )
    }

    /// Sends one admitted streaming tool request through the production parser.
    ///
    /// # Errors
    ///
    /// Returns [`crate::HostError`] for admission, cancellation, transport, status, bound, or stream failures.
    #[cfg(feature = "tool-streaming")]
    pub fn execute_tool_stream(
        &self,
        request: super::OpenAiToolStreamRequest<'_>,
    ) -> Result<crate::OpenAiStreamOutcome, crate::HostError> {
        // r[impl agent_harness_runtime.tool_capable_openai_host.authority]
        let outcome = super::execute_tool_stream_with_transport(
            self,
            super::StreamingExecutionRequest {
                config: &self.config,
                plan: request.plan,
                dispatch: request.dispatch,
                api_key: request.api_key,
                cancellation: request.cancellation,
            },
        )?;
        let _progress: crate::OpenAiRetryStreamProgress = outcome.retry_progress();
        Ok(outcome)
    }

    /// Returns the admitted host configuration.
    #[must_use]
    pub const fn config(&self) -> &crate::OpenAiHostConfig {
        &self.config
    }
}

#[cfg(feature = "reqwest-client")]
impl super::Transport for ReqwestOpenAiHost {
    fn send(
        &self,
        request: &super::WireRequest<'_>,
        api_key: &crate::OpenAiApiKey,
    ) -> Result<super::WireResponse, crate::TransportErrorClass> {
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
        let read_limit_bytes = request
            .max_response_bytes
            .checked_add(1)
            .ok_or(crate::TransportErrorClass::ResponseRead)?;
        let mut bounded = response.take(read_limit_bytes);
        let mut body = Vec::new();
        bounded
            .read_to_end(&mut body)
            .map_err(|_error| crate::TransportErrorClass::ResponseRead)?;
        Ok(super::WireResponse { status, body })
    }
}

#[cfg(feature = "tool-streaming")]
impl super::StreamingTransport for ReqwestOpenAiHost {
    fn open_stream(
        &self,
        request: &super::WireRequest<'_>,
        api_key: &crate::OpenAiApiKey,
    ) -> Result<super::StreamingWireResponse, crate::TransportErrorClass> {
        let response = self
            .client
            .post(request.endpoint_url)
            .bearer_auth(api_key.expose_value())
            .header(reqwest::header::ACCEPT, "text/event-stream")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(request.body.clone())
            .send()
            .map_err(|error| classify_reqwest_error(&error))?;
        Ok(super::StreamingWireResponse {
            status: response.status().as_u16(),
            body: Box::new(response),
        })
    }
}

#[cfg(feature = "reqwest-client")]
fn classify_reqwest_error(error: &reqwest::Error) -> crate::TransportErrorClass {
    if error.is_timeout() {
        crate::TransportErrorClass::Timeout
    } else {
        crate::TransportErrorClass::Unavailable
    }
}
