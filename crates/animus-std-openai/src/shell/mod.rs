#[cfg(feature = "reqwest-client")]
mod client;
#[cfg(feature = "reqwest-client")]
pub use client::ReqwestOpenAiHost;
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
        api_key: &crate::OpenAiApiKey,
    ) -> Result<WireResponse, crate::TransportErrorClass>;
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
        api_key: &crate::OpenAiApiKey,
    ) -> Result<StreamingWireResponse, crate::TransportErrorClass>;
}

#[derive(Clone, Copy)]
pub(crate) struct ExecutionRequest<'a> {
    pub(crate) config: &'a crate::OpenAiHostConfig,
    pub(crate) plan: &'a animus_std::ProviderDispatchPlan,
    pub(crate) api_key: &'a crate::OpenAiApiKey,
    pub(crate) cancellation: crate::CancellationState,
}

pub(crate) fn execute_with_transport<T: Transport>(
    transport: &T,
    input: ExecutionRequest<'_>,
) -> Result<animus_std::ProviderDispatchOutcome, crate::HostError> {
    if input.cancellation == crate::CancellationState::Cancelled {
        return Err(crate::HostError::Cancelled);
    }
    if input.api_key.source_label() != input.plan.credential_source_label() {
        return Err(crate::HostError::CredentialBindingMismatch);
    }
    assert_eq!(input.cancellation, crate::CancellationState::Active);
    assert_eq!(
        input.api_key.source_label(),
        input.plan.credential_source_label()
    );
    let body = crate::build_openai_request_body(input.config, input.plan)?;
    let request = WireRequest {
        endpoint_url: input.config.endpoint_url(),
        body,
        max_response_bytes: input.config.max_response_bytes(),
    };
    let response = transport
        .send(&request, input.api_key)
        .map_err(|class| crate::HostError::Transport { class })?;
    if !(HTTP_SUCCESS_START..HTTP_SUCCESS_END_EXCLUSIVE).contains(&response.status) {
        return Err(crate::HostError::HttpStatus {
            status: response.status,
        });
    }
    crate::normalize_openai_response(
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
    pub api_key: &'a crate::OpenAiApiKey,
    /// Pollable cancellation for the exact call.
    pub cancellation: &'a dyn StreamingCancellation,
}

#[cfg(feature = "tool-streaming")]
pub(crate) struct StreamingExecutionRequest<'a> {
    pub(crate) config: &'a crate::OpenAiHostConfig,
    pub(crate) plan: &'a animus_std::ProviderDispatchPlan,
    pub(crate) dispatch: crate::OpenAiToolDispatch,
    pub(crate) api_key: &'a crate::OpenAiApiKey,
    pub(crate) cancellation: &'a dyn StreamingCancellation,
}

#[cfg(feature = "tool-streaming")]
pub(crate) fn execute_tool_stream_with_transport<T: StreamingTransport>(
    transport: &T,
    input: StreamingExecutionRequest<'_>,
) -> Result<crate::OpenAiStreamOutcome, crate::HostError> {
    if input.cancellation.is_cancelled() {
        return Err(crate::HostError::Cancelled);
    }
    if input.api_key.source_label() != input.plan.credential_source_label() {
        return Err(crate::HostError::CredentialBindingMismatch);
    }
    assert_eq!(
        input.api_key.source_label(),
        input.plan.credential_source_label()
    );
    let body =
        crate::build_tool_capable_openai_request_body(input.config, input.plan, &input.dispatch)?;
    assert_eq!(input.plan.endpoint(), input.config.endpoint_identity());
    let request = WireRequest {
        endpoint_url: input.config.endpoint_url(),
        body,
        max_response_bytes: input.config.max_response_bytes(),
    };
    let mut response = transport
        .open_stream(&request, input.api_key)
        .map_err(|class| crate::HostError::Transport { class })?;
    if !(HTTP_SUCCESS_START..HTTP_SUCCESS_END_EXCLUSIVE).contains(&response.status) {
        return Err(crate::HostError::HttpStatus {
            status: response.status,
        });
    }
    let mut parser = crate::OpenAiResponsesSseParser::new(
        input.dispatch,
        crate::OpenAiStreamBounds::reviewed(input.config.max_response_bytes()),
    )
    .map_err(|source| crate::HostError::Stream { source })?;
    stream_reader_into_parser(response.body.as_mut(), &mut parser, input.cancellation)?;
    parser
        .finish()
        .map_err(|source| crate::HostError::Stream { source })
}

#[cfg(feature = "tool-streaming")]
fn stream_reader_into_parser(
    reader: &mut dyn std::io::Read,
    parser: &mut crate::OpenAiResponsesSseParser,
    cancellation: &dyn StreamingCancellation,
) -> Result<(), crate::HostError> {
    const STREAM_READ_BUFFER_BYTES: usize = 8_192;
    let mut buffer = [0_u8; STREAM_READ_BUFFER_BYTES];
    loop {
        if cancellation.is_cancelled() {
            parser
                .cancel()
                .map_err(|source| crate::HostError::Stream { source })?;
            return Ok(());
        }
        let read = reader
            .read(&mut buffer)
            .map_err(|_error| crate::HostError::Transport {
                class: crate::TransportErrorClass::ResponseRead,
            })?;
        if read == 0 {
            return Ok(());
        }
        let chunk = buffer.get(..read).ok_or(crate::HostError::Transport {
            class: crate::TransportErrorClass::ResponseRead,
        })?;
        assert!(!chunk.is_empty());
        assert_eq!(chunk.len(), read);
        parser
            .push_chunk(chunk)
            .map_err(|source| crate::HostError::Stream { source })?;
    }
}
