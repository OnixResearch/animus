//! Bounded chat-completions lifecycle over explicit bytes and request history.

use crate::stream::{
    FIRST_ORDER, OpenAiRetryStreamProgress, OpenAiStreamBounds, OpenAiStreamError,
    OpenAiStreamFailureClass, OpenAiStreamOutcome, OpenAiStreamTerminal, OutcomeParts, STREAM_DONE,
    bound_error, correlation, find_separator, lifecycle, malformed, parse_sse_fields,
};
use crate::tool::OpenAiToolDispatch;

mod admission;
mod event;
mod frames;
pub(crate) mod history;
pub(crate) mod request;
mod toolcall;

pub use request::build_zai_chat_completions_request_body;

const RESPONSE_ID_REASON: &str = "chat-completions SSE response identity is missing or mismatched";
const TERMINAL_REASON: &str = "chat-completions SSE terminal state is invalid";
const TOOL_CORRELATION_REASON: &str = "chat-completions SSE function-call correlation is invalid";
const UNSUPPORTED_CHOICE_REASON: &str = "chat-completions SSE choice shape is unsupported";
const CANCELLED_REASON: &str = "cancelled";
const STREAM_BOUND_REASON: &str = "chat-completions SSE exceeds a stream bound";
const MALFORMED_SSE_REASON: &str = "chat-completions SSE is malformed";

struct ChatToolState {
    tool_call_id: String,
    tool_name: Option<String>,
    arguments: String,
}

struct ChatFrameParts<'a> {
    kind: animus_adapters::ModelStreamFrameKind,
    body: Option<&'a str>,
    tool_call_id: Option<&'a str>,
    usage: Option<animus_adapters::ModelStreamUsage>,
    visibility: animus_adapters::ModelStreamVisibility,
}

/// Deterministic incremental production parser for chat-completions SSE bytes.
pub struct OpenAiChatCompletionsSseParser {
    model_call_id: String,
    dispatch: OpenAiToolDispatch,
    bounds: OpenAiStreamBounds,
    buffer: Vec<u8>,
    total_bytes: usize,
    event_count: usize,
    next_order: u64,
    response_id: Option<String>,
    tool_states: std::collections::BTreeMap<u64, ChatToolState>,
    tool_calls: Vec<crate::tool::OpenAiCompletedToolCall>,
    usage_emitted: bool,
    error: Option<OpenAiStreamError>,
    terminal: Option<OpenAiStreamTerminal>,
    stream_end_seen: bool,
    pending_finish: Option<String>,
    frames: Vec<animus_adapters::ModelStreamFrame>,
    retry_progress: OpenAiRetryStreamProgress,
    replay: Option<history::Replay>,
}

impl OpenAiChatCompletionsSseParser {
    /// Builds a parser without request history. Its calls cannot resume a chat request.
    ///
    /// # Errors
    /// Returns [`OpenAiStreamError`] when parser bounds are invalid.
    pub fn new(
        dispatch: OpenAiToolDispatch,
        bounds: OpenAiStreamBounds,
    ) -> Result<Self, OpenAiStreamError> {
        bounds.validate().map_err(|_error| {
            OpenAiStreamError::new(OpenAiStreamFailureClass::BoundExceeded, STREAM_BOUND_REASON)
        })?;
        Ok(Self {
            model_call_id: String::from(dispatch.model_call_id()),
            dispatch,
            bounds,
            buffer: Vec::new(),
            total_bytes: 0,
            event_count: 0,
            next_order: FIRST_ORDER,
            response_id: None,
            tool_states: std::collections::BTreeMap::new(),
            tool_calls: Vec::new(),
            usage_emitted: false,
            error: None,
            terminal: None,
            stream_end_seen: false,
            pending_finish: None,
            frames: Vec::new(),
            retry_progress: OpenAiRetryStreamProgress::TransportStarted,
            replay: None,
        })
    }

    /// Binds an admitted request to the parser for offline or injected stream acquisition.
    ///
    /// # Errors
    /// Returns [`crate::HostError`] for request, history, or parser admission failures.
    pub fn for_request(
        config: &crate::OpenAiHostConfig,
        plan: &animus_std::ProviderDispatchPlan,
        dispatch: OpenAiToolDispatch,
        bounds: OpenAiStreamBounds,
    ) -> Result<Self, crate::HostError> {
        let prepared = request::prepare(config, plan, &dispatch)?;
        Self::new(dispatch, bounds)
            .map(|parser| parser.with_history(prepared.replay))
            .map_err(|source| crate::HostError::Stream { source })
    }

    pub(crate) fn with_history(mut self, replay: history::Replay) -> Self {
        self.replay = Some(replay);
        self
    }

    /// Consumes one provider chunk. An error permanently rejects this parser.
    ///
    /// # Errors
    /// Returns [`OpenAiStreamError`] for malformed framing, bounds, or correlation errors.
    pub fn push_chunk(&mut self, chunk: &[u8]) -> Result<(), OpenAiStreamError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        let result = self.consume_chunk(chunk);
        if let Err(error) = &result {
            self.error = Some(error.clone());
        }
        result
    }

    fn consume_chunk(&mut self, chunk: &[u8]) -> Result<(), OpenAiStreamError> {
        if self.stream_end_seen || self.terminal == Some(OpenAiStreamTerminal::Cancelled) {
            return Err(lifecycle(TERMINAL_REASON));
        }
        if chunk.is_empty() {
            return Err(malformed(MALFORMED_SSE_REASON));
        }
        self.total_bytes = self
            .total_bytes
            .checked_add(chunk.len())
            .ok_or_else(bound_error)?;
        if self.total_bytes > self.bounds.stream_bytes {
            return Err(bound_error());
        }
        self.buffer.extend_from_slice(chunk);
        while let Some((index, separator_len)) = find_separator(&self.buffer) {
            if index == 0 || index > self.bounds.event_bytes {
                return Err(bound_error());
            }
            let bytes = self.buffer[..index].to_vec();
            self.buffer.drain(..index + separator_len);
            self.event_count = self.event_count.checked_add(1).ok_or_else(bound_error)?;
            if self.event_count > self.bounds.events {
                return Err(bound_error());
            }
            self.process_event(&bytes)?;
        }
        if self.buffer.len() > self.bounds.event_bytes {
            return Err(bound_error());
        }
        Ok(())
    }

    /// Emits a cancelled terminal frame and discards unfinished request history.
    ///
    /// # Errors
    /// Returns [`OpenAiStreamError`] if parsing failed or the stream already ended.
    pub fn cancel(&mut self) -> Result<(), OpenAiStreamError> {
        if let Some(error) = &self.error {
            return Err(error.clone());
        }
        if self.terminal.is_some() {
            return Err(lifecycle(TERMINAL_REASON));
        }
        let frame = self.frame(&ChatFrameParts {
            kind: animus_adapters::ModelStreamFrameKind::Cancelled,
            body: Some(CANCELLED_REASON),
            tool_call_id: None,
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })?;
        self.frames.push(frame);
        self.buffer.clear();
        self.tool_states.clear();
        self.tool_calls.clear();
        self.replay = None;
        self.terminal = Some(OpenAiStreamTerminal::Cancelled);
        Ok(())
    }

    /// Validates terminal framing and binds completed calls to bounded request history.
    ///
    /// # Errors
    /// Returns [`OpenAiStreamError`] for trailing bytes, missing DONE, or invalid history.
    pub fn finish(mut self) -> Result<OpenAiStreamOutcome, OpenAiStreamError> {
        if let Some(error) = self.error.take() {
            return Err(error);
        }
        if !self.buffer.is_empty() {
            return Err(malformed(MALFORMED_SSE_REASON));
        }
        let terminal = self.terminal.ok_or_else(|| lifecycle(TERMINAL_REASON))?;
        if terminal != OpenAiStreamTerminal::Cancelled && !self.stream_end_seen {
            return Err(lifecycle(TERMINAL_REASON));
        }
        if terminal == OpenAiStreamTerminal::Completed && self.pending_finish.take().is_some() {
            let frame = self.frame(&ChatFrameParts {
                kind: animus_adapters::ModelStreamFrameKind::Finish,
                body: None,
                tool_call_id: None,
                usage: None,
                visibility: animus_adapters::ModelStreamVisibility::Redacted,
            })?;
            self.frames.push(frame);
        }
        animus_adapters::validate_model_stream_frames(&self.frames).map_err(|_error| {
            OpenAiStreamError::new(OpenAiStreamFailureClass::Canonical, MALFORMED_SSE_REASON)
        })?;
        if terminal == OpenAiStreamTerminal::Completed && !self.tool_calls.is_empty() {
            let [call] = self.tool_calls.as_mut_slice() else {
                return Err(correlation(TOOL_CORRELATION_REASON));
            };
            if let Some(replay) = self.replay.take() {
                call.chat_replay = Some(std::sync::Arc::new(replay.complete(&self.frames, call)?));
            }
        }
        let provider_response_id = match self.response_id {
            Some(id) => id,
            None if terminal == OpenAiStreamTerminal::Cancelled => String::new(),
            None => return Err(correlation(RESPONSE_ID_REASON)),
        };
        Ok(OpenAiStreamOutcome::assemble(OutcomeParts {
            provider_response_id,
            frames: self.frames,
            tool_calls: self.tool_calls,
            terminal,
            retry_progress: self.retry_progress,
        }))
    }

    fn process_event(&mut self, bytes: &[u8]) -> Result<(), OpenAiStreamError> {
        if self.stream_end_seen || self.terminal == Some(OpenAiStreamTerminal::Cancelled) {
            return Err(lifecycle(TERMINAL_REASON));
        }
        let text = std::str::from_utf8(bytes).map_err(|_error| malformed(MALFORMED_SSE_REASON))?;
        if text
            .lines()
            .all(|line| line.trim_end_matches('\r').starts_with(':'))
        {
            return Ok(());
        }
        let (event_name, data) = parse_sse_fields(text)?;
        if data == STREAM_DONE {
            if event_name.is_some() || self.terminal.is_none() || self.stream_end_seen {
                return Err(lifecycle(TERMINAL_REASON));
            }
            self.stream_end_seen = true;
            return Ok(());
        }
        if event_name.is_some() {
            return Err(malformed(MALFORMED_SSE_REASON));
        }
        let value: serde_json::Value =
            serde_json::from_str(data).map_err(|_error| malformed(MALFORMED_SSE_REASON))?;
        let object = value
            .as_object()
            .ok_or_else(|| malformed(MALFORMED_SSE_REASON))?;
        self.accept_chunk_object(object)
    }

    fn bind_response_id(&mut self, id: &str) -> Result<(), OpenAiStreamError> {
        if id.is_empty() {
            return Err(correlation(RESPONSE_ID_REASON));
        }
        match self.response_id.as_deref() {
            None => self.response_id = Some(String::from(id)),
            Some(bound) if bound == id => {}
            Some(_) => return Err(correlation(RESPONSE_ID_REASON)),
        }
        Ok(())
    }
}

fn required_string<'a>(
    object: &'a serde_json::Map<String, serde_json::Value>,
    field: &str,
) -> Result<&'a str, OpenAiStreamError> {
    object
        .get(field)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| correlation(RESPONSE_ID_REASON))
}
