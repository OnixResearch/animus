use crate::{
    CancellationState, HostError, OpenAiApiKey, OpenAiHostConfig, TransportErrorClass,
    build_openai_request_body, normalize_openai_response,
};
#[cfg(feature = "tool-streaming")]
use crate::{
    OpenAiRetryStreamProgress, OpenAiStreamBounds, OpenAiStreamOutcome,
    build_tool_capable_openai_request_body,
};

#[cfg(all(test, feature = "tool-streaming"))]
mod deadline;
#[cfg(all(test, feature = "tool-streaming"))]
mod tls;

const HTTP_SUCCESS_START: u16 = 200;
const HTTP_SUCCESS_END_EXCLUSIVE: u16 = 300;

/// Pollable cancellation service for one provider stream.
#[cfg(feature = "tool-streaming")]
pub trait StreamingCancellation {
    /// Returns true when the caller cancelled the exact in-flight model call.
    fn is_cancelled(&self) -> bool;
}

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

#[cfg(feature = "tool-streaming")]
pub(crate) struct StreamingWireResponse {
    pub(crate) status: u16,
    pub(crate) body: Box<dyn std::io::Read>,
}

#[cfg(feature = "tool-streaming")]
pub(crate) trait StreamingTransport {
    fn open_stream(
        &self,
        request: &WireRequest<'_>,
        api_key: &OpenAiApiKey,
    ) -> Result<StreamingWireResponse, TransportErrorClass>;
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

/// Borrowed authority and correlation input for one tool stream.
#[cfg(feature = "tool-streaming")]
pub struct OpenAiToolStreamRequest<'a> {
    /// Admitted provider dispatch plan.
    pub plan: &'a animus_std::ProviderDispatchPlan,
    /// Tool cohort and continuation correlation.
    pub dispatch: crate::OpenAiToolDispatch,
    /// Explicit credential bound to the plan.
    pub api_key: &'a OpenAiApiKey,
    /// Pollable cancellation for the exact call.
    pub cancellation: &'a dyn StreamingCancellation,
}

#[cfg(feature = "tool-streaming")]
pub(crate) struct StreamingExecutionRequest<'a> {
    pub(crate) config: &'a OpenAiHostConfig,
    pub(crate) plan: &'a animus_std::ProviderDispatchPlan,
    pub(crate) dispatch: crate::OpenAiToolDispatch,
    pub(crate) api_key: &'a OpenAiApiKey,
    pub(crate) cancellation: &'a dyn StreamingCancellation,
}

#[cfg(feature = "tool-streaming")]
pub(crate) fn execute_tool_stream_with_transport<T: StreamingTransport>(
    transport: &T,
    input: StreamingExecutionRequest<'_>,
) -> Result<OpenAiStreamOutcome, HostError> {
    if input.cancellation.is_cancelled() {
        return Err(HostError::Cancelled);
    }
    if input.api_key.source_label() != input.plan.credential_source_label() {
        return Err(HostError::CredentialBindingMismatch);
    }
    let body = build_tool_capable_openai_request_body(input.config, input.plan, &input.dispatch)?;
    let request = WireRequest {
        endpoint_url: input.config.endpoint_url(),
        body,
        max_response_bytes: input.config.max_response_bytes(),
    };
    let mut response = transport
        .open_stream(&request, input.api_key)
        .map_err(|class| HostError::Transport { class })?;
    if !(HTTP_SUCCESS_START..HTTP_SUCCESS_END_EXCLUSIVE).contains(&response.status) {
        return Err(HostError::HttpStatus {
            status: response.status,
        });
    }
    let mut parser = crate::OpenAiResponsesSseParser::new(
        input.dispatch,
        OpenAiStreamBounds::reviewed(usize::try_from(input.config.max_response_bytes()).map_err(
            |_error| HostError::ResponseTooLarge {
                limit: input.config.max_response_bytes(),
            },
        )?),
    )
    .map_err(|source| HostError::Stream { source })?;
    stream_reader_into_parser(response.body.as_mut(), &mut parser, input.cancellation)?;
    parser
        .finish()
        .map_err(|source| HostError::Stream { source })
}

#[cfg(feature = "tool-streaming")]
pub(crate) fn execute_chat_stream_with_transport<T: StreamingTransport>(
    transport: &T,
    input: StreamingExecutionRequest<'_>,
) -> Result<OpenAiStreamOutcome, HostError> {
    if input.cancellation.is_cancelled() {
        return Err(HostError::Cancelled);
    }
    if input.api_key.source_label() != input.plan.credential_source_label() {
        return Err(HostError::CredentialBindingMismatch);
    }
    let prepared = crate::chat::request::prepare(input.config, input.plan, &input.dispatch)?;
    let mut parser = crate::OpenAiChatCompletionsSseParser::new(
        input.dispatch,
        OpenAiStreamBounds::reviewed(usize::try_from(input.config.max_response_bytes()).map_err(
            |_error| HostError::ResponseTooLarge { limit: input.config.max_response_bytes() },
        )?),
    ).map_err(|source| HostError::Stream { source })?.with_history(prepared.replay);
    let request = WireRequest {
        endpoint_url: input.config.endpoint_url(), body: prepared.body,
        max_response_bytes: input.config.max_response_bytes(),
    };
    if input.cancellation.is_cancelled() { return Err(HostError::Cancelled); }
    let mut response = transport.open_stream(&request, input.api_key)
        .map_err(|class| HostError::Transport { class })?;
    if !(HTTP_SUCCESS_START..HTTP_SUCCESS_END_EXCLUSIVE).contains(&response.status) {
        return Err(HostError::HttpStatus { status: response.status });
    }
    stream_reader_into_parser(response.body.as_mut(), &mut parser, input.cancellation)?;
    parser
        .finish()
        .map_err(|source| HostError::Stream { source })
}

/// One explicit blocking client for an admitted Responses endpoint.
#[cfg(feature = "reqwest-client")]
pub struct ReqwestOpenAiHost {
    config: OpenAiHostConfig,
    client: reqwest::blocking::Client,
}

#[cfg(feature = "reqwest-client")]
impl ReqwestOpenAiHost {
    fn client_builder(config: &OpenAiHostConfig) -> reqwest::blocking::ClientBuilder {
        reqwest::blocking::Client::builder()
            .timeout(config.timeout())
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .https_only(config.https_only())
    }

    /// Builds the exact bounded client without redirects or retries.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when the client cannot be constructed.
    pub fn new(config: OpenAiHostConfig) -> Result<Self, HostError> {
        let client =
            Self::client_builder(&config)
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

    /// Sends one admitted streaming tool request through the production parser.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] for admission, cancellation, transport, status, bound, or stream failures.
    #[cfg(feature = "tool-streaming")]
    pub fn execute_tool_stream(
        &self,
        request: OpenAiToolStreamRequest<'_>,
    ) -> Result<OpenAiStreamOutcome, HostError> {
        // r[impl agent_harness_runtime.tool_capable_openai_host.authority]
        let outcome = execute_tool_stream_with_transport(
            self,
            StreamingExecutionRequest {
                config: &self.config,
                plan: request.plan,
                dispatch: request.dispatch,
                api_key: request.api_key,
                cancellation: request.cancellation,
            },
        )?;
        let _progress: OpenAiRetryStreamProgress = outcome.retry_progress();
        Ok(outcome)
    }

    /// Executes one bounded chat-completions stream for the Z.ai cohort.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] under the same transport, bound, and parser rules
    /// as [`Self::execute_tool_stream`].
    #[cfg(feature = "tool-streaming")]
    pub fn execute_chat_stream(
        &self,
        request: OpenAiToolStreamRequest<'_>,
    ) -> Result<OpenAiStreamOutcome, HostError> {
        let outcome = execute_chat_stream_with_transport(
            self,
            StreamingExecutionRequest {
                config: &self.config,
                plan: request.plan,
                dispatch: request.dispatch,
                api_key: request.api_key,
                cancellation: request.cancellation,
            },
        )?;
        let _progress: OpenAiRetryStreamProgress = outcome.retry_progress();
        Ok(outcome)
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
            // The blocking client timeout renews on each read. The request
            // timeout also follows the async response body until completion.
            .timeout(self.config.timeout())
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

#[cfg(feature = "tool-streaming")]
impl StreamingTransport for ReqwestOpenAiHost {
    fn open_stream(
        &self,
        request: &WireRequest<'_>,
        api_key: &OpenAiApiKey,
    ) -> Result<StreamingWireResponse, TransportErrorClass> {
        let response = self
            .client
            .post(request.endpoint_url)
            .bearer_auth(api_key.expose_value())
            .timeout(self.config.timeout())
            .header(reqwest::header::ACCEPT, "text/event-stream")
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .body(request.body.clone())
            .send()
            .map_err(|error| classify_reqwest_error(&error))?;
        Ok(StreamingWireResponse {
            status: response.status().as_u16(),
            body: Box::new(response),
        })
    }
}

#[cfg(feature = "tool-streaming")]
pub(crate) trait StreamParserSink {
    fn push_chunk(&mut self, chunk: &[u8]) -> Result<(), crate::stream::OpenAiStreamError>;
    fn cancel(&mut self) -> Result<(), crate::stream::OpenAiStreamError>;
}

#[cfg(feature = "tool-streaming")]
impl StreamParserSink for crate::OpenAiResponsesSseParser {
    fn push_chunk(&mut self, chunk: &[u8]) -> Result<(), crate::stream::OpenAiStreamError> {
        Self::push_chunk(self, chunk)
    }
    fn cancel(&mut self) -> Result<(), crate::stream::OpenAiStreamError> {
        Self::cancel(self)
    }
}

#[cfg(feature = "tool-streaming")]
impl StreamParserSink for crate::OpenAiChatCompletionsSseParser {
    fn push_chunk(&mut self, chunk: &[u8]) -> Result<(), crate::stream::OpenAiStreamError> {
        Self::push_chunk(self, chunk)
    }
    fn cancel(&mut self) -> Result<(), crate::stream::OpenAiStreamError> {
        Self::cancel(self)
    }
}

#[cfg(feature = "tool-streaming")]
fn stream_reader_into_parser<P: StreamParserSink>(
    reader: &mut dyn std::io::Read,
    parser: &mut P,
    cancellation: &dyn StreamingCancellation,
) -> Result<(), HostError> {
    const STREAM_READ_BUFFER_BYTES: usize = 8_192;
    let mut buffer = [0_u8; STREAM_READ_BUFFER_BYTES];
    loop {
        if cancellation.is_cancelled() {
            parser
                .cancel()
                .map_err(|source| HostError::Stream { source })?;
            return Ok(());
        }
        let read = reader
            .read(&mut buffer)
            .map_err(|_error| HostError::Transport {
                class: TransportErrorClass::ResponseRead,
            })?;
        if read == 0 {
            return Ok(());
        }
        parser
            .push_chunk(&buffer[..read])
            .map_err(|source| HostError::Stream { source })?;
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
