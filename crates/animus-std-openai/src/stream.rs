use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Deserialize;
use serde_json::Value;

use crate::{HostError, OpenAiCompletedToolCall, OpenAiToolDispatch};

const EVENT_SEPARATOR: &[u8] = b"\n\n";
const CRLF_EVENT_SEPARATOR: &[u8] = b"\r\n\r\n";
const EVENT_PREFIX: &str = "event:";
const DATA_PREFIX: &str = "data:";
const STREAM_DONE: &str = "[DONE]";
const FIRST_ORDER: u64 = 1;
const ORDER_INCREMENT: u64 = 1;
const MAX_STREAM_CHUNKS: usize = 65_536;
const MAX_STREAM_EVENTS: usize = 65_536;
const MAX_EVENT_BYTES: usize = 1_048_576;
const MAX_TOOL_ARGUMENT_BYTES: usize = 262_144;
const MAX_TOOL_CALLS: usize = 64;

const RESPONSE_CREATED: &str = "response.created";
const RESPONSE_IN_PROGRESS: &str = "response.in_progress";
const OUTPUT_TEXT_DELTA: &str = "response.output_text.delta";
const OUTPUT_TEXT_DONE: &str = "response.output_text.done";
const CONTENT_PART_ADDED: &str = "response.content_part.added";
const CONTENT_PART_DONE: &str = "response.content_part.done";
const REASONING_SUMMARY_PART_ADDED: &str = "response.reasoning_summary_part.added";
const REASONING_SUMMARY_PART_DONE: &str = "response.reasoning_summary_part.done";
const REASONING_SUMMARY_DELTA: &str = "response.reasoning_summary_text.delta";
const REASONING_SUMMARY_DONE: &str = "response.reasoning_summary_text.done";
const OUTPUT_ITEM_ADDED: &str = "response.output_item.added";
const OUTPUT_ITEM_DONE: &str = "response.output_item.done";
const FUNCTION_ARGUMENTS_DELTA: &str = "response.function_call_arguments.delta";
const FUNCTION_ARGUMENTS_DONE: &str = "response.function_call_arguments.done";
const RESPONSE_COMPLETED: &str = "response.completed";
const RESPONSE_FAILED: &str = "response.failed";
const RESPONSE_INCOMPLETE: &str = "response.incomplete";
const ERROR_EVENT: &str = "error";
const FUNCTION_CALL_TYPE: &str = "function_call";
const MESSAGE_TYPE: &str = "message";
const REASONING_TYPE: &str = "reasoning";

const MALFORMED_SSE_REASON: &str = "OpenAI Responses SSE is malformed";
const EVENT_NAME_MISMATCH_REASON: &str = "OpenAI Responses SSE event name does not match JSON type";
const UNKNOWN_EVENT_REASON: &str = "OpenAI Responses SSE event is outside the reviewed cohort";
const RESPONSE_ID_REASON: &str = "OpenAI Responses SSE response identity is missing or mismatched";
const RESPONSE_LIFECYCLE_REASON: &str = "OpenAI Responses SSE lifecycle is invalid";
const SEQUENCE_REASON: &str = "OpenAI Responses SSE sequence is invalid";
const TERMINAL_REASON: &str = "OpenAI Responses SSE terminal state is invalid";
const TERMINAL_BODY_REASON: &str = "OpenAI Responses SSE terminal response is malformed";
const TOOL_CORRELATION_REASON: &str = "OpenAI Responses SSE function-call correlation is invalid";
const TOOL_ARGUMENT_REASON: &str = "OpenAI Responses SSE function-call arguments are invalid";
const TOOL_ARGUMENT_BOUND_REASON: &str =
    "OpenAI Responses SSE function-call arguments exceed the bound";
const TOOL_CALL_BOUND_REASON: &str = "OpenAI Responses SSE function-call count exceeds the bound";
const STREAM_BOUND_REASON: &str = "OpenAI Responses SSE exceeds a stream bound";
const DUPLICATE_USAGE_REASON: &str = "OpenAI Responses SSE emitted duplicate usage";
const REASONING_EVENT_REASON: &str =
    "OpenAI Responses SSE reasoning event is unsafe or unsupported";
const PROVIDER_FAILURE_REASON: &str = "openai_provider_failure";
const PROVIDER_INCOMPLETE_REASON: &str = "openai_provider_incomplete";
const PROVIDER_ERROR_REASON: &str = "openai_provider_error";
const CANCELLED_REASON: &str = "cancelled";

/// Explicit byte and event bounds for one production SSE parse.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct OpenAiStreamBounds {
    /// Aggregate stream byte limit.
    pub stream_bytes: usize,
    /// One SSE event byte limit.
    pub event_bytes: usize,
    /// Parsed SSE event count limit.
    pub events: usize,
    /// Aggregate function arguments limit for one call.
    pub tool_argument_bytes: usize,
    /// Function-call count limit for one response.
    pub tool_calls: usize,
}

impl OpenAiStreamBounds {
    /// Builds bounded defaults for the reviewed cohort.
    #[must_use]
    pub const fn reviewed(max_stream_bytes: usize) -> Self {
        Self {
            stream_bytes: max_stream_bytes,
            event_bytes: if max_stream_bytes < MAX_EVENT_BYTES {
                max_stream_bytes
            } else {
                MAX_EVENT_BYTES
            },
            events: MAX_STREAM_EVENTS,
            tool_argument_bytes: MAX_TOOL_ARGUMENT_BYTES,
            tool_calls: MAX_TOOL_CALLS,
        }
    }

    fn validate(self) -> Result<(), HostError> {
        let valid = self.stream_bytes > 0
            && self.event_bytes > 0
            && self.event_bytes <= self.stream_bytes
            && self.events > 0
            && self.events <= MAX_STREAM_EVENTS
            && self.tool_argument_bytes > 0
            && self.tool_argument_bytes <= MAX_TOOL_ARGUMENT_BYTES
            && self.tool_calls > 0
            && self.tool_calls <= MAX_TOOL_CALLS;
        if !valid {
            return Err(HostError::InvalidToolCohort {
                reason: STREAM_BOUND_REASON,
            });
        }
        Ok(())
    }
}

/// Safe production parser failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenAiStreamFailureClass {
    /// SSE framing or JSON syntax was malformed.
    Malformed,
    /// A configured body, event, count, or argument bound was exceeded.
    BoundExceeded,
    /// Provider lifecycle or sequence was invalid.
    Lifecycle,
    /// Response, output-item, or function-call correlation failed.
    Correlation,
    /// The provider emitted an event outside the reviewed cohort.
    UnsupportedEvent,
    /// Reasoning data did not use the exposed summary event.
    UnsafeReasoning,
    /// The provider ended with a failed or incomplete response.
    ProviderFailure,
    /// Explicit cancellation ended parsing.
    Cancelled,
    /// Canonical stream-frame validation failed.
    Canonical,
}

/// Safe production parser failure without provider payloads.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenAiStreamError {
    class: OpenAiStreamFailureClass,
    reason: &'static str,
}

impl OpenAiStreamError {
    fn new(class: OpenAiStreamFailureClass, reason: &'static str) -> Self {
        Self { class, reason }
    }

    /// Returns the safe parser failure class.
    #[must_use]
    pub const fn class(&self) -> OpenAiStreamFailureClass {
        self.class
    }

    /// Returns a stable safe reason.
    #[must_use]
    pub const fn reason(&self) -> &'static str {
        self.reason
    }
}

impl fmt::Debug for OpenAiStreamError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenAiStreamError")
            .field("class", &self.class)
            .field("reason", &self.reason)
            .finish()
    }
}

impl fmt::Display for OpenAiStreamError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.reason)
    }
}

impl std::error::Error for OpenAiStreamError {}

/// Terminal classification for one parsed provider stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenAiStreamTerminal {
    /// The provider completed normally.
    Completed,
    /// The caller cancelled parsing.
    Cancelled,
    /// The provider emitted a failure or incomplete terminal event.
    Failed,
}

/// Safe stream progress for existing provider retry policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OpenAiRetryStreamProgress {
    /// Transport started but no semantic frame was emitted.
    TransportStarted,
    /// Assistant content was emitted.
    Content,
    /// Provider-exposed reasoning summary was emitted.
    Reasoning,
    /// A function-call delta was emitted.
    ToolCall,
    /// Usage or terminal metadata was emitted without earlier semantic text.
    Other,
}

/// Bounded result from the production `OpenAI` Responses SSE parser.
#[derive(Clone)]
pub struct OpenAiStreamOutcome {
    provider_response_id: String,
    frames: Vec<animus_adapters::ModelStreamFrame>,
    tool_calls: Vec<OpenAiCompletedToolCall>,
    terminal: OpenAiStreamTerminal,
    retry_progress: OpenAiRetryStreamProgress,
}

impl OpenAiStreamOutcome {
    /// Returns the host-local provider response identity.
    #[must_use]
    pub fn provider_response_id(&self) -> &str {
        self.provider_response_id.as_str()
    }

    /// Returns ordered canonical model stream frames.
    #[must_use]
    pub fn frames(&self) -> &[animus_adapters::ModelStreamFrame] {
        self.frames.as_slice()
    }

    /// Returns completed correlated provider function calls.
    #[must_use]
    pub fn tool_calls(&self) -> &[OpenAiCompletedToolCall] {
        self.tool_calls.as_slice()
    }

    /// Returns the terminal stream classification.
    #[must_use]
    pub const fn terminal(&self) -> OpenAiStreamTerminal {
        self.terminal
    }

    /// Returns the safe retry progress classification.
    #[must_use]
    pub const fn retry_progress(&self) -> OpenAiRetryStreamProgress {
        self.retry_progress
    }

    /// Reduces all accepted frames into the existing runtime replay-safety state.
    #[must_use]
    pub fn retry_stream_state(&self) -> animus_runtime::provider_retry::ProviderRetryStreamState {
        self.frames.iter().fold(
            animus_runtime::provider_retry::ProviderRetryStreamState::TransportStarted,
            |state, frame| {
                let semantic = match frame.kind() {
                    animus_adapters::ModelStreamFrameKind::ContentDelta => {
                        animus_runtime::provider_retry::ProviderRetrySemanticFrame::Content
                    }
                    animus_adapters::ModelStreamFrameKind::ThoughtDelta => {
                        animus_runtime::provider_retry::ProviderRetrySemanticFrame::Reasoning
                    }
                    animus_adapters::ModelStreamFrameKind::ToolCallDelta => {
                        animus_runtime::provider_retry::ProviderRetrySemanticFrame::ToolCall
                    }
                    animus_adapters::ModelStreamFrameKind::Usage
                    | animus_adapters::ModelStreamFrameKind::Finish
                    | animus_adapters::ModelStreamFrameKind::Cancelled
                    | animus_adapters::ModelStreamFrameKind::Failure => {
                        animus_runtime::provider_retry::ProviderRetrySemanticFrame::Other
                    }
                };
                animus_runtime::provider_retry::observe_provider_retry_stream(
                    state,
                    animus_runtime::provider_retry::ProviderRetryStreamObservation::Semantic(
                        semantic,
                    ),
                )
            },
        )
    }
}

/// Renders metadata-only evidence for one tool-capable stream outcome.
#[must_use]
pub fn render_openai_tool_stream_evidence(outcome: &OpenAiStreamOutcome) -> String {
    let terminal = match outcome.terminal {
        OpenAiStreamTerminal::Completed => "completed",
        OpenAiStreamTerminal::Cancelled => "cancelled",
        OpenAiStreamTerminal::Failed => "failed",
    };
    let provider_response_identity = blake3::hash(outcome.provider_response_id.as_bytes());
    format!(
        "LIVE_OPENAI_TOOL_STREAM status={terminal}\nframe_count={}\ntool_call_count={}\nprovider_response_b3={}\nraw_response=omitted\ntool_arguments=omitted\ntool_output=omitted\ncredential=omitted\nreasoning=omitted",
        outcome.frames.len(),
        outcome.tool_calls.len(),
        provider_response_identity.to_hex(),
    )
}

impl fmt::Debug for OpenAiStreamOutcome {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenAiStreamOutcome")
            .field("provider_response_id", &"[REDACTED]")
            .field("frame_count", &self.frames.len())
            .field("tool_call_count", &self.tool_calls.len())
            .field("terminal", &self.terminal)
            .field("retry_progress", &self.retry_progress)
            .finish()
    }
}

/// Deterministic incremental production parser for `OpenAI` Responses SSE bytes.
pub struct OpenAiResponsesSseParser {
    model_call_id: String,
    dispatch: OpenAiToolDispatch,
    bounds: OpenAiStreamBounds,
    buffer: Vec<u8>,
    total_bytes: usize,
    event_count: usize,
    next_sequence: Option<u64>,
    next_order: u64,
    response_id: Option<String>,
    lifecycle_started: bool,
    terminal: Option<OpenAiStreamTerminal>,
    stream_end_seen: bool,
    frames: Vec<animus_adapters::ModelStreamFrame>,
    output_items: BTreeMap<String, OutputItemKind>,
    completed_output_items: BTreeSet<String>,
    tool_states: BTreeMap<String, ToolState>,
    tool_calls: Vec<OpenAiCompletedToolCall>,
    usage_emitted: bool,
    retry_progress: OpenAiRetryStreamProgress,
}

impl OpenAiResponsesSseParser {
    /// Builds one parser from explicit model-call admission and bounds.
    ///
    /// # Errors
    ///
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
            next_sequence: None,
            next_order: FIRST_ORDER,
            response_id: None,
            lifecycle_started: false,
            terminal: None,
            stream_end_seen: false,
            frames: Vec::new(),
            output_items: BTreeMap::new(),
            completed_output_items: BTreeSet::new(),
            tool_states: BTreeMap::new(),
            tool_calls: Vec::new(),
            usage_emitted: false,
            retry_progress: OpenAiRetryStreamProgress::TransportStarted,
        })
    }

    /// Consumes one admitted provider byte chunk.
    ///
    /// # Errors
    ///
    /// Returns [`OpenAiStreamError`] for malformed framing, bounds, lifecycle, correlation, or unsupported events.
    pub fn push_chunk(&mut self, chunk: &[u8]) -> Result<(), OpenAiStreamError> {
        // r[impl agent_harness_runtime.tool_capable_openai_host.streaming]
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
        self.process_complete_events()
    }

    /// Emits a canonical cancelled terminal frame.
    ///
    /// # Errors
    ///
    /// Returns [`OpenAiStreamError`] if the stream already ended.
    pub fn cancel(&mut self) -> Result<(), OpenAiStreamError> {
        if self.terminal.is_some() {
            return Err(lifecycle(TERMINAL_REASON));
        }
        let frame = self.terminal_frame(
            animus_adapters::ModelStreamFrameKind::Cancelled,
            Some(CANCELLED_REASON),
        )?;
        self.frames.push(frame);
        self.terminal = Some(OpenAiStreamTerminal::Cancelled);
        Ok(())
    }

    /// Completes parsing and validates the canonical frame stream.
    ///
    /// # Errors
    ///
    /// Returns [`OpenAiStreamError`] for trailing bytes, missing terminal state, unfinished calls, or invalid frames.
    pub fn finish(mut self) -> Result<OpenAiStreamOutcome, OpenAiStreamError> {
        self.process_complete_events()?;
        if !self.buffer.is_empty() {
            if self.buffer.len() > self.bounds.event_bytes {
                return Err(bound_error());
            }
            let event_bytes = std::mem::take(&mut self.buffer);
            self.event_count = self.event_count.checked_add(1).ok_or_else(bound_error)?;
            if self.event_count > self.bounds.events {
                return Err(bound_error());
            }
            self.process_event(event_bytes.as_slice())?;
        }
        let terminal = self.terminal.ok_or_else(|| lifecycle(TERMINAL_REASON))?;
        if terminal == OpenAiStreamTerminal::Completed
            && (self.tool_states.values().any(|state| !state.done)
                || self.output_items.len() != self.completed_output_items.len())
        {
            return Err(correlation(TOOL_CORRELATION_REASON));
        }
        animus_adapters::validate_model_stream_frames(self.frames.as_slice()).map_err(
            |_error| {
                OpenAiStreamError::new(OpenAiStreamFailureClass::Canonical, MALFORMED_SSE_REASON)
            },
        )?;
        let provider_response_id = match terminal {
            OpenAiStreamTerminal::Cancelled if self.response_id.is_none() => String::new(),
            OpenAiStreamTerminal::Completed
            | OpenAiStreamTerminal::Failed
            | OpenAiStreamTerminal::Cancelled => self
                .response_id
                .ok_or_else(|| correlation(RESPONSE_ID_REASON))?,
        };
        Ok(OpenAiStreamOutcome {
            provider_response_id,
            frames: self.frames,
            tool_calls: self.tool_calls,
            terminal,
            retry_progress: self.retry_progress,
        })
    }

    fn process_complete_events(&mut self) -> Result<(), OpenAiStreamError> {
        while let Some((separator_index, separator_len)) = find_separator(self.buffer.as_slice()) {
            if separator_index == 0 || separator_index > self.bounds.event_bytes {
                return Err(bound_error());
            }
            let event_bytes = self.buffer[..separator_index].to_vec();
            self.buffer.drain(..separator_index + separator_len);
            self.event_count = self.event_count.checked_add(1).ok_or_else(bound_error)?;
            if self.event_count > self.bounds.events {
                return Err(bound_error());
            }
            self.process_event(event_bytes.as_slice())?;
        }
        if self.buffer.len() > self.bounds.event_bytes {
            return Err(bound_error());
        }
        Ok(())
    }

    fn process_event(&mut self, bytes: &[u8]) -> Result<(), OpenAiStreamError> {
        let text = std::str::from_utf8(bytes).map_err(|_error| malformed(MALFORMED_SSE_REASON))?;
        // r[impl agent_harness_runtime.responses_sse_framing.keepalive]
        // A record whose every line starts with the comment prefix carries no fields.
        // The record already consumed one event-count unit, so bounds stay charged.
        if text
            .lines()
            .all(|line| line.trim_end_matches('\r').starts_with(':'))
        {
            return Ok(());
        }
        let (event_name, data) = parse_sse_fields(text)?;
        // r[impl agent_harness_runtime.responses_sse_framing.termination]
        if data == STREAM_DONE {
            if event_name.is_some()
                || self.terminal != Some(OpenAiStreamTerminal::Completed)
                || self.stream_end_seen
            {
                return Err(lifecycle(TERMINAL_REASON));
            }
            self.stream_end_seen = true;
            debug_assert_eq!(self.terminal, Some(OpenAiStreamTerminal::Completed));
            return Ok(());
        }
        if self.terminal.is_some() {
            return Err(lifecycle(TERMINAL_REASON));
        }
        debug_assert!(!self.stream_end_seen);
        let envelope: EventEnvelope =
            serde_json::from_str(data).map_err(|_error| malformed(MALFORMED_SSE_REASON))?;
        // r[impl agent_harness_runtime.responses_sse_framing.metadata]
        // r[impl agent_harness_runtime.responses_sse_framing.guards]
        if event_name.is_some_and(|name| name != envelope.kind) {
            return Err(malformed(EVENT_NAME_MISMATCH_REASON));
        }
        self.check_sequence(envelope.sequence_number)?;
        self.process_json_event(envelope.kind.as_str(), &envelope.value)
    }

    fn process_json_event(&mut self, kind: &str, value: &Value) -> Result<(), OpenAiStreamError> {
        match kind {
            RESPONSE_CREATED => self.start_response(value),
            RESPONSE_IN_PROGRESS => self.progress_response(value),
            OUTPUT_TEXT_DELTA => {
                self.text_delta(value, animus_adapters::ModelStreamFrameKind::ContentDelta)
            }
            OUTPUT_TEXT_DONE | CONTENT_PART_ADDED | CONTENT_PART_DONE => {
                self.output_lifecycle(value, OutputItemKind::Message)
            }
            REASONING_SUMMARY_DELTA => {
                self.text_delta(value, animus_adapters::ModelStreamFrameKind::ThoughtDelta)
            }
            REASONING_SUMMARY_DONE | REASONING_SUMMARY_PART_ADDED | REASONING_SUMMARY_PART_DONE => {
                self.output_lifecycle(value, OutputItemKind::Reasoning)
            }
            OUTPUT_ITEM_ADDED => self.output_item_added(value),
            FUNCTION_ARGUMENTS_DELTA => self.function_arguments_delta(value),
            FUNCTION_ARGUMENTS_DONE => self.function_arguments_done(value),
            OUTPUT_ITEM_DONE => self.output_item_done(value),
            RESPONSE_COMPLETED => self.response_completed(value),
            RESPONSE_FAILED => self.provider_terminal(value, PROVIDER_FAILURE_REASON),
            RESPONSE_INCOMPLETE => self.provider_terminal(value, PROVIDER_INCOMPLETE_REASON),
            ERROR_EVENT => self.provider_terminal(value, PROVIDER_ERROR_REASON),
            _unknown if kind.contains("reasoning") => Err(OpenAiStreamError::new(
                OpenAiStreamFailureClass::UnsafeReasoning,
                REASONING_EVENT_REASON,
            )),
            _unknown => Err(OpenAiStreamError::new(
                OpenAiStreamFailureClass::UnsupportedEvent,
                UNKNOWN_EVENT_REASON,
            )),
        }
    }

    fn check_sequence(&mut self, sequence: u64) -> Result<(), OpenAiStreamError> {
        if let Some(expected) = self.next_sequence {
            if sequence != expected {
                return Err(lifecycle(SEQUENCE_REASON));
            }
        }
        self.next_sequence = Some(
            sequence
                .checked_add(ORDER_INCREMENT)
                .ok_or_else(|| lifecycle(SEQUENCE_REASON))?,
        );
        Ok(())
    }

    fn start_response(&mut self, value: &Value) -> Result<(), OpenAiStreamError> {
        if self.lifecycle_started {
            return Err(lifecycle(RESPONSE_LIFECYCLE_REASON));
        }
        let response = response_object(value)?;
        let response_id = required_string(response, "id")?;
        if response.get("status").and_then(Value::as_str) != Some("in_progress") {
            return Err(lifecycle(RESPONSE_LIFECYCLE_REASON));
        }
        self.response_id = Some(String::from(response_id));
        self.lifecycle_started = true;
        Ok(())
    }

    fn progress_response(&mut self, value: &Value) -> Result<(), OpenAiStreamError> {
        self.require_started()?;
        let response = response_object(value)?;
        self.check_response_id(required_string(response, "id")?)
    }

    fn text_delta(
        &mut self,
        value: &Value,
        kind: animus_adapters::ModelStreamFrameKind,
    ) -> Result<(), OpenAiStreamError> {
        self.require_started()?;
        if value.get("response_id").and_then(Value::as_str).is_some() {
            self.check_response_id(required_string(value, "response_id")?)?;
        }
        let expected_item_kind = if kind == animus_adapters::ModelStreamFrameKind::ContentDelta {
            OutputItemKind::Message
        } else {
            OutputItemKind::Reasoning
        };
        self.require_output_item(required_string(value, "item_id")?, expected_item_kind)?;
        if kind == animus_adapters::ModelStreamFrameKind::ThoughtDelta
            && value.get("summary_index").and_then(Value::as_u64).is_none()
        {
            return Err(OpenAiStreamError::new(
                OpenAiStreamFailureClass::UnsafeReasoning,
                REASONING_EVENT_REASON,
            ));
        }
        let delta = required_string(value, "delta")?;
        let visibility = if kind == animus_adapters::ModelStreamFrameKind::ContentDelta {
            self.retry_progress = OpenAiRetryStreamProgress::Content;
            animus_adapters::ModelStreamVisibility::AssistantVisible
        } else {
            self.retry_progress = OpenAiRetryStreamProgress::Reasoning;
            animus_adapters::ModelStreamVisibility::Redacted
        };
        let frame = self.frame(FrameParts {
            kind,
            body: Some(delta),
            tool_call_id: None,
            usage: None,
            visibility,
        })?;
        self.frames.push(frame);
        Ok(())
    }

    fn output_lifecycle(
        &self,
        value: &Value,
        expected: OutputItemKind,
    ) -> Result<(), OpenAiStreamError> {
        self.require_started()?;
        self.require_output_item(required_string(value, "item_id")?, expected)
    }

    fn output_item_added(&mut self, value: &Value) -> Result<(), OpenAiStreamError> {
        self.require_started()?;
        let item = required_object(value, "item")?;
        let output_item_id = required_string(item, "id")?;
        let item_kind = OutputItemKind::parse(required_string(item, "type")?)?;
        if self
            .output_items
            .insert(String::from(output_item_id), item_kind)
            .is_some()
        {
            return Err(correlation(TOOL_CORRELATION_REASON));
        }
        if item_kind != OutputItemKind::FunctionCall {
            return Ok(());
        }
        let tool_call_id = required_string(item, "call_id")?;
        let tool_name = required_string(item, "name")?;
        if !self
            .dispatch
            .capabilities()
            .tools()
            .iter()
            .any(|tool| tool.name() == tool_name)
        {
            return Err(correlation(TOOL_CORRELATION_REASON));
        }
        if self.tool_states.len() >= self.bounds.tool_calls
            || self.tool_states.contains_key(output_item_id)
            || self
                .tool_states
                .values()
                .any(|state| state.tool_call_id == tool_call_id)
        {
            return Err(correlation(TOOL_CALL_BOUND_REASON));
        }
        let initial_arguments = item.get("arguments").and_then(Value::as_str).unwrap_or("");
        if initial_arguments.len() > self.bounds.tool_argument_bytes {
            return Err(bound_tool_argument());
        }
        self.tool_states.insert(
            String::from(output_item_id),
            ToolState {
                tool_call_id: String::from(tool_call_id),
                tool_name: String::from(tool_name),
                arguments: String::from(initial_arguments),
                arguments_done: false,
                done: false,
            },
        );
        Ok(())
    }

    fn function_arguments_delta(&mut self, value: &Value) -> Result<(), OpenAiStreamError> {
        self.require_started()?;
        let item_id = required_string(value, "item_id")?;
        let delta = required_string(value, "delta")?;
        let state = self
            .tool_states
            .get_mut(item_id)
            .ok_or_else(|| correlation(TOOL_CORRELATION_REASON))?;
        if state.arguments_done || state.done {
            return Err(correlation(TOOL_CORRELATION_REASON));
        }
        let new_len = state
            .arguments
            .len()
            .checked_add(delta.len())
            .ok_or_else(bound_tool_argument)?;
        if new_len > self.bounds.tool_argument_bytes {
            return Err(bound_tool_argument());
        }
        state.arguments.push_str(delta);
        let tool_call_id = state.tool_call_id.clone();
        self.retry_progress = OpenAiRetryStreamProgress::ToolCall;
        let frame = self.frame(FrameParts {
            kind: animus_adapters::ModelStreamFrameKind::ToolCallDelta,
            body: Some(delta),
            tool_call_id: Some(tool_call_id.as_str()),
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })?;
        self.frames.push(frame);
        Ok(())
    }

    fn function_arguments_done(&mut self, value: &Value) -> Result<(), OpenAiStreamError> {
        self.require_started()?;
        let item_id = required_string(value, "item_id")?;
        let arguments = required_string(value, "arguments")?;
        let state = self
            .tool_states
            .get_mut(item_id)
            .ok_or_else(|| correlation(TOOL_CORRELATION_REASON))?;
        if state.arguments_done || state.done || state.arguments != arguments {
            return Err(correlation(TOOL_CORRELATION_REASON));
        }
        serde_json::from_str::<Value>(arguments)
            .map_err(|_error| correlation(TOOL_ARGUMENT_REASON))?;
        state.arguments_done = true;
        Ok(())
    }

    fn output_item_done(&mut self, value: &Value) -> Result<(), OpenAiStreamError> {
        self.require_started()?;
        let item = required_object(value, "item")?;
        let output_item_id = required_string(item, "id")?;
        let item_kind = OutputItemKind::parse(required_string(item, "type")?)?;
        self.require_output_item(output_item_id, item_kind)?;
        if !self
            .completed_output_items
            .insert(String::from(output_item_id))
        {
            return Err(correlation(TOOL_CORRELATION_REASON));
        }
        if item_kind != OutputItemKind::FunctionCall {
            return Ok(());
        }
        let tool_call_id = required_string(item, "call_id")?;
        let tool_name = required_string(item, "name")?;
        let arguments = required_string(item, "arguments")?;
        let state = self
            .tool_states
            .get_mut(output_item_id)
            .ok_or_else(|| correlation(TOOL_CORRELATION_REASON))?;
        if !state.arguments_done
            || state.done
            || state.tool_call_id != tool_call_id
            || state.tool_name != tool_name
            || state.arguments != arguments
        {
            return Err(correlation(TOOL_CORRELATION_REASON));
        }
        state.done = true;
        let response_id = self
            .response_id
            .as_deref()
            .ok_or_else(|| correlation(RESPONSE_ID_REASON))?;
        self.tool_calls.push(OpenAiCompletedToolCall {
            model_call_id: self.model_call_id.clone(),
            provider_response_id: String::from(response_id),
            output_item_id: String::from(output_item_id),
            tool_call_id: String::from(tool_call_id),
            tool_name: String::from(tool_name),
            arguments_json: String::from(arguments),
        });
        Ok(())
    }

    fn response_completed(&mut self, value: &Value) -> Result<(), OpenAiStreamError> {
        self.require_started()?;
        let response = response_object(value)?;
        self.check_response_id(required_string(response, "id")?)?;
        if response.get("status").and_then(Value::as_str) != Some("completed") {
            return Err(lifecycle(TERMINAL_BODY_REASON));
        }
        if let Some(usage) = response.get("usage") {
            self.emit_usage(usage)?;
        }
        let frame = self.terminal_frame(animus_adapters::ModelStreamFrameKind::Finish, None)?;
        self.frames.push(frame);
        self.terminal = Some(OpenAiStreamTerminal::Completed);
        Ok(())
    }

    fn provider_terminal(
        &mut self,
        value: &Value,
        reason: &'static str,
    ) -> Result<(), OpenAiStreamError> {
        self.require_started()?;
        if let Some(response) = value.get("response") {
            let response = response
                .as_object()
                .ok_or_else(|| malformed(TERMINAL_BODY_REASON))?;
            if let Some(response_id) = response.get("id").and_then(Value::as_str) {
                self.check_response_id(response_id)?;
            }
        }
        let frame =
            self.terminal_frame(animus_adapters::ModelStreamFrameKind::Failure, Some(reason))?;
        self.frames.push(frame);
        self.terminal = Some(OpenAiStreamTerminal::Failed);
        if self.retry_progress == OpenAiRetryStreamProgress::TransportStarted {
            self.retry_progress = OpenAiRetryStreamProgress::Other;
        }
        Ok(())
    }

    fn emit_usage(&mut self, usage: &Value) -> Result<(), OpenAiStreamError> {
        if self.usage_emitted {
            return Err(lifecycle(DUPLICATE_USAGE_REASON));
        }
        let input = required_u32(usage, "input_tokens")?;
        let output = required_u32(usage, "output_tokens")?;
        let frame = self.frame(FrameParts {
            kind: animus_adapters::ModelStreamFrameKind::Usage,
            body: None,
            tool_call_id: None,
            usage: Some(animus_adapters::ModelStreamUsage::new(input, output)),
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })?;
        self.frames.push(frame);
        self.usage_emitted = true;
        if self.retry_progress == OpenAiRetryStreamProgress::TransportStarted {
            self.retry_progress = OpenAiRetryStreamProgress::Other;
        }
        Ok(())
    }

    fn frame(
        &mut self,
        parts: FrameParts<'_>,
    ) -> Result<animus_adapters::ModelStreamFrame, OpenAiStreamError> {
        let order = self.next_order;
        self.next_order = self
            .next_order
            .checked_add(ORDER_INCREMENT)
            .ok_or_else(|| lifecycle(SEQUENCE_REASON))?;
        animus_adapters::ModelStreamFrame::from_draft(animus_adapters::ModelStreamFrameDraft {
            call_id: self.model_call_id.as_str(),
            order,
            kind: parts.kind,
            body: parts.body,
            tool_call_id: parts.tool_call_id,
            usage: parts.usage,
            visibility: parts.visibility,
        })
        .map_err(|_error| {
            OpenAiStreamError::new(OpenAiStreamFailureClass::Canonical, MALFORMED_SSE_REASON)
        })
    }

    fn terminal_frame(
        &mut self,
        kind: animus_adapters::ModelStreamFrameKind,
        body: Option<&str>,
    ) -> Result<animus_adapters::ModelStreamFrame, OpenAiStreamError> {
        self.frame(FrameParts {
            kind,
            body,
            tool_call_id: None,
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })
    }

    fn require_started(&self) -> Result<(), OpenAiStreamError> {
        if self.lifecycle_started {
            Ok(())
        } else {
            Err(lifecycle(RESPONSE_LIFECYCLE_REASON))
        }
    }

    fn check_response_id(&self, actual: &str) -> Result<(), OpenAiStreamError> {
        if self.response_id.as_deref() == Some(actual) {
            Ok(())
        } else {
            Err(correlation(RESPONSE_ID_REASON))
        }
    }

    fn require_output_item(
        &self,
        item_id: &str,
        expected: OutputItemKind,
    ) -> Result<(), OpenAiStreamError> {
        if self.output_items.get(item_id) == Some(&expected)
            && !self.completed_output_items.contains(item_id)
        {
            Ok(())
        } else {
            Err(correlation(TOOL_CORRELATION_REASON))
        }
    }
}

#[derive(Clone, Copy)]
struct FrameParts<'a> {
    kind: animus_adapters::ModelStreamFrameKind,
    body: Option<&'a str>,
    tool_call_id: Option<&'a str>,
    usage: Option<animus_adapters::ModelStreamUsage>,
    visibility: animus_adapters::ModelStreamVisibility,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum OutputItemKind {
    Message,
    Reasoning,
    FunctionCall,
}

impl OutputItemKind {
    fn parse(value: &str) -> Result<Self, OpenAiStreamError> {
        match value {
            MESSAGE_TYPE => Ok(Self::Message),
            REASONING_TYPE => Ok(Self::Reasoning),
            FUNCTION_CALL_TYPE => Ok(Self::FunctionCall),
            _unsupported => Err(OpenAiStreamError::new(
                OpenAiStreamFailureClass::UnsupportedEvent,
                UNKNOWN_EVENT_REASON,
            )),
        }
    }
}

#[derive(Clone, Debug)]
struct ToolState {
    tool_call_id: String,
    tool_name: String,
    arguments: String,
    arguments_done: bool,
    done: bool,
}

#[derive(Deserialize)]
struct EventEnvelope {
    #[serde(rename = "type")]
    kind: String,
    sequence_number: u64,
    #[serde(flatten)]
    value: Value,
}

/// Parses admitted chunks through the production parser.
///
/// # Errors
///
/// Returns [`OpenAiStreamError`] for malformed, unsupported, over-limit, stale, or incomplete streams.
pub fn parse_openai_responses_sse_chunks(
    dispatch: OpenAiToolDispatch,
    bounds: OpenAiStreamBounds,
    chunks: &[&[u8]],
) -> Result<OpenAiStreamOutcome, OpenAiStreamError> {
    if chunks.is_empty() || chunks.len() > MAX_STREAM_CHUNKS {
        return Err(bound_error());
    }
    let mut parser = OpenAiResponsesSseParser::new(dispatch, bounds)?;
    for chunk in chunks {
        parser.push_chunk(chunk)?;
    }
    parser.finish()
}

fn parse_sse_fields(record: &str) -> Result<(Option<&str>, &str), OpenAiStreamError> {
    let mut event = None;
    let mut data = None;
    for line in record.lines() {
        let line = line.trim_end_matches('\r');
        if let Some(value) = line.strip_prefix(EVENT_PREFIX) {
            if event.replace(value.trim_start()).is_some() {
                return Err(malformed(MALFORMED_SSE_REASON));
            }
        } else if let Some(value) = line.strip_prefix(DATA_PREFIX) {
            if data.replace(value.trim_start()).is_some() {
                return Err(malformed(MALFORMED_SSE_REASON));
            }
        } else if !line.starts_with(':') {
            return Err(malformed(MALFORMED_SSE_REASON));
        }
    }
    match (event, data) {
        (event, Some(data)) if !event.is_some_and(str::is_empty) && !data.is_empty() => {
            debug_assert!(!data.is_empty());
            debug_assert!(event.is_none_or(|name| !name.is_empty()));
            Ok((event, data))
        }
        _missing => Err(malformed(MALFORMED_SSE_REASON)),
    }
}

fn find_separator(bytes: &[u8]) -> Option<(usize, usize)> {
    let lf = bytes
        .windows(EVENT_SEPARATOR.len())
        .position(|window| window == EVENT_SEPARATOR)
        .map(|index| (index, EVENT_SEPARATOR.len()));
    let crlf = bytes
        .windows(CRLF_EVENT_SEPARATOR.len())
        .position(|window| window == CRLF_EVENT_SEPARATOR)
        .map(|index| (index, CRLF_EVENT_SEPARATOR.len()));
    match (lf, crlf) {
        (Some(left), Some(right)) => Some(if left.0 <= right.0 { left } else { right }),
        (Some(value), None) | (None, Some(value)) => Some(value),
        (None, None) => None,
    }
}

fn response_object(value: &Value) -> Result<&serde_json::Map<String, Value>, OpenAiStreamError> {
    required_object(value, "response")
}

fn required_object<'a>(
    value: &'a Value,
    key: &str,
) -> Result<&'a serde_json::Map<String, Value>, OpenAiStreamError> {
    value
        .get(key)
        .and_then(Value::as_object)
        .ok_or_else(|| malformed(MALFORMED_SSE_REASON))
}

trait JsonFields {
    fn field(&self, key: &str) -> Option<&Value>;
}

impl JsonFields for Value {
    fn field(&self, key: &str) -> Option<&Value> {
        self.get(key)
    }
}

impl JsonFields for serde_json::Map<String, Value> {
    fn field(&self, key: &str) -> Option<&Value> {
        self.get(key)
    }
}

fn required_string<'a, T: JsonFields + ?Sized>(
    value: &'a T,
    key: &str,
) -> Result<&'a str, OpenAiStreamError> {
    value
        .field(key)
        .and_then(Value::as_str)
        .filter(|text| !text.is_empty())
        .ok_or_else(|| malformed(MALFORMED_SSE_REASON))
}

fn required_u32(value: &Value, key: &str) -> Result<u32, OpenAiStreamError> {
    let raw = value
        .get(key)
        .and_then(Value::as_u64)
        .ok_or_else(|| malformed(MALFORMED_SSE_REASON))?;
    u32::try_from(raw).map_err(|_error| bound_error())
}

fn malformed(reason: &'static str) -> OpenAiStreamError {
    OpenAiStreamError::new(OpenAiStreamFailureClass::Malformed, reason)
}

fn lifecycle(reason: &'static str) -> OpenAiStreamError {
    OpenAiStreamError::new(OpenAiStreamFailureClass::Lifecycle, reason)
}

fn correlation(reason: &'static str) -> OpenAiStreamError {
    OpenAiStreamError::new(OpenAiStreamFailureClass::Correlation, reason)
}

fn bound_error() -> OpenAiStreamError {
    OpenAiStreamError::new(OpenAiStreamFailureClass::BoundExceeded, STREAM_BOUND_REASON)
}

fn bound_tool_argument() -> OpenAiStreamError {
    OpenAiStreamError::new(
        OpenAiStreamFailureClass::BoundExceeded,
        TOOL_ARGUMENT_BOUND_REASON,
    )
}
