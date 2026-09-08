//! Deterministic incremental parser for OpenAI-compatible chat-completions SSE.
//!
//! Z.ai's coding-plan endpoint serves this protocol and has no Responses API.
//! The parser emits the same canonical model stream frames as the Responses
//! parser so the existing worker, retry policy, and measurement stack reuse.

use crate::stream::{
    FIRST_ORDER, ORDER_INCREMENT, OpenAiRetryStreamProgress, OpenAiStreamBounds, OpenAiStreamError,
    OpenAiStreamFailureClass, OpenAiStreamOutcome, OpenAiStreamTerminal, OutcomeParts, STREAM_DONE,
    bound_error, correlation, find_separator, lifecycle, malformed, parse_sse_fields,
};
use crate::tool::{OpenAiCompletedToolCall, OpenAiToolDispatch};
use serde_json::Value;
use std::collections::BTreeMap;

const RESPONSE_ID_REASON: &str = "chat-completions SSE response identity is missing or mismatched";
const TERMINAL_REASON: &str = "chat-completions SSE terminal state is invalid";
const TOOL_CORRELATION_REASON: &str = "chat-completions SSE function-call correlation is invalid";
const UNSUPPORTED_CHOICE_REASON: &str = "chat-completions SSE choice shape is unsupported";
const CANCELLED_REASON: &str = "cancelled";

struct ChatToolState {
    tool_call_id: String,
    tool_name: Option<String>,
    arguments: String,
}

/// Borrowed frame parts before canonical validation.
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
    tool_states: BTreeMap<u64, ChatToolState>,
    tool_calls: Vec<crate::tool::OpenAiCompletedToolCall>,
    usage_emitted: bool,
    terminal: Option<OpenAiStreamTerminal>,
    stream_end_seen: bool,
    pending_finish: Option<String>,
    frames: Vec<animus_adapters::ModelStreamFrame>,
    retry_progress: OpenAiRetryStreamProgress,
}

impl OpenAiChatCompletionsSseParser {
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
            next_order: FIRST_ORDER,
            response_id: None,
            tool_states: BTreeMap::new(),
            tool_calls: Vec::new(),
            usage_emitted: false,
            terminal: None,
            stream_end_seen: false,
            pending_finish: None,
            frames: Vec::new(),
            retry_progress: OpenAiRetryStreamProgress::TransportStarted,
        })
    }

    /// Consumes one admitted provider byte chunk.
    ///
    /// # Errors
    ///
    /// Returns [`OpenAiStreamError`] for malformed framing, bounds, correlation,
    /// or unsupported event shapes.
    pub fn push_chunk(&mut self, chunk: &[u8]) -> Result<(), OpenAiStreamError> {
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

    /// Emits a canonical cancelled terminal frame.
    ///
    /// # Errors
    ///
    /// Returns [`OpenAiStreamError`] if the stream already ended.
    pub fn cancel(&mut self) -> Result<(), OpenAiStreamError> {
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
        self.terminal = Some(OpenAiStreamTerminal::Cancelled);
        Ok(())
    }

    /// Completes parsing and validates the canonical frame stream.
    ///
    /// # Errors
    ///
    /// Returns [`OpenAiStreamError`] for trailing bytes, missing terminal state,
    /// or invalid frames.
    pub fn finish(mut self) -> Result<OpenAiStreamOutcome, OpenAiStreamError> {
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
        animus_adapters::validate_model_stream_frames(self.frames.as_slice()).map_err(
            |_error| {
                OpenAiStreamError::new(OpenAiStreamFailureClass::Canonical, MALFORMED_SSE_REASON)
            },
        )?;
        let provider_response_id = self
            .response_id
            .ok_or_else(|| correlation(RESPONSE_ID_REASON))?;
        Ok(OpenAiStreamOutcome::assemble(OutcomeParts {
            provider_response_id,
            frames: self.frames,
            tool_calls: self.tool_calls,
            terminal,
            retry_progress: self.retry_progress,
        }))
    }

    fn process_event(&mut self, bytes: &[u8]) -> Result<(), OpenAiStreamError> {
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
        let value: Value =
            serde_json::from_str(data).map_err(|_error| malformed(MALFORMED_SSE_REASON))?;
        let object = value
            .as_object()
            .ok_or_else(|| malformed(MALFORMED_SSE_REASON))?;
        self.bind_response_id(required_string(object, "id")?)?;
        if self.terminal.is_some() {
            // Only the usage-only record that follows a finished stream is admitted.
            if object
                .get("choices")
                .and_then(Value::as_array)
                .is_some_and(Vec::is_empty)
            {
                return self.accept_usage(object);
            }
            return Err(lifecycle(TERMINAL_REASON));
        }
        if let Some(reason) = finish_reason(object)? {
            self.finish_stream(reason)?;
            if let Some(usage) = object.get("usage").filter(|usage| !usage.is_null()) {
                self.accept_usage_value(usage)?;
            }
            return Ok(());
        }
        let choices = object.get("choices").and_then(Value::as_array);
        let Some(choices) = choices else {
            return self.accept_usage(object);
        };
        if choices.is_empty() {
            return self.accept_usage(object);
        }
        let [choice] = choices.as_slice() else {
            return Err(malformed(UNSUPPORTED_CHOICE_REASON));
        };
        if let Some(usage) = choice.get("usage").filter(|usage| !usage.is_null()) {
            self.accept_usage_value(usage)?;
        }
        let delta = choice.get("delta").and_then(Value::as_object);
        let Some(delta) = delta else {
            return Ok(());
        };
        if let Some(content) = delta.get("content").and_then(Value::as_str) {
            self.text_frame(animus_adapters::ModelStreamFrameKind::ContentDelta, content)?;
        }
        if let Some(reasoning) = delta.get("reasoning_content").and_then(Value::as_str) {
            self.text_frame(
                animus_adapters::ModelStreamFrameKind::ThoughtDelta,
                reasoning,
            )?;
        }
        if let Some(calls) = delta.get("tool_calls").and_then(Value::as_array) {
            for call in calls {
                self.tool_call_delta(call)?;
            }
        }
        Ok(())
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

    fn text_frame(
        &mut self,
        kind: animus_adapters::ModelStreamFrameKind,
        text: &str,
    ) -> Result<(), OpenAiStreamError> {
        if text.is_empty() {
            return Ok(());
        }
        let (visibility, progress) = if kind == animus_adapters::ModelStreamFrameKind::ContentDelta
        {
            self.retry_progress = OpenAiRetryStreamProgress::Content;
            (
                animus_adapters::ModelStreamVisibility::AssistantVisible,
                0_u8,
            )
        } else {
            self.retry_progress = OpenAiRetryStreamProgress::Reasoning;
            (animus_adapters::ModelStreamVisibility::Redacted, 1_u8)
        };
        debug_assert!(progress <= 1);
        let frame = self.frame(&ChatFrameParts {
            kind,
            body: Some(text),
            tool_call_id: None,
            usage: None,
            visibility,
        })?;
        self.frames.push(frame);
        Ok(())
    }

    fn tool_call_delta(&mut self, call: &Value) -> Result<(), OpenAiStreamError> {
        let object = call
            .as_object()
            .ok_or_else(|| malformed(MALFORMED_SSE_REASON))?;
        let index = object.get("index").and_then(Value::as_u64).ok_or_else(|| {
            OpenAiStreamError::new(
                OpenAiStreamFailureClass::Correlation,
                TOOL_CORRELATION_REASON,
            )
        })?;
        let function = object.get("function").and_then(Value::as_object);
        let name = function
            .and_then(|function| function.get("name"))
            .and_then(Value::as_str);
        let arguments = function
            .and_then(|function| function.get("arguments"))
            .and_then(Value::as_str)
            .unwrap_or("");
        if !self.tool_states.contains_key(&index)
            && self.tool_states.len() >= self.bounds.tool_calls
        {
            return Err(bound_error());
        }
        let (tool_call_id, arguments_delta) = {
            let state = self
                .tool_states
                .entry(index)
                .or_insert_with(|| ChatToolState {
                    tool_call_id: object
                        .get("id")
                        .and_then(Value::as_str)
                        .unwrap_or_default()
                        .to_owned(),
                    tool_name: None,
                    arguments: String::new(),
                });
            if let Some(id) = object.get("id").and_then(Value::as_str) {
                if state.tool_call_id.is_empty() {
                    state.tool_call_id = String::from(id);
                } else if state.tool_call_id != id {
                    return Err(correlation(TOOL_CORRELATION_REASON));
                }
            }
            if state.tool_call_id.is_empty() {
                return Err(correlation(TOOL_CORRELATION_REASON));
            }
            if let Some(name) = name {
                match state.tool_name.as_deref() {
                    None => {
                        let admitted = self
                            .dispatch
                            .capabilities()
                            .tools()
                            .iter()
                            .any(|tool| tool.name() == name);
                        if !admitted || name.is_empty() {
                            return Err(correlation(TOOL_CORRELATION_REASON));
                        }
                        state.tool_name = Some(String::from(name));
                    }
                    Some(bound) if bound == name => {}
                    Some(_) => return Err(correlation(TOOL_CORRELATION_REASON)),
                }
            }
            if state.arguments.len().saturating_add(arguments.len())
                > self.bounds.tool_argument_bytes
            {
                return Err(bound_error());
            }
            state.arguments.push_str(arguments);
            (state.tool_call_id.clone(), arguments)
        };
        if arguments_delta.is_empty() {
            return Ok(());
        }
        let frame = self.frame(&ChatFrameParts {
            kind: animus_adapters::ModelStreamFrameKind::ToolCallDelta,
            body: Some(arguments_delta),
            tool_call_id: Some(tool_call_id.as_str()),
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })?;
        self.frames.push(frame);
        self.retry_progress = OpenAiRetryStreamProgress::ToolCall;
        Ok(())
    }

    fn finish_stream(&mut self, reason: &str) -> Result<(), OpenAiStreamError> {
        if self.terminal.is_some() {
            return Err(lifecycle(TERMINAL_REASON));
        }
        match reason {
            "stop" if self.tool_states.is_empty() => {}
            "tool_calls" => {
                let indexes: Vec<u64> = self.tool_states.keys().copied().collect();
                for index in indexes {
                    self.complete_tool_call(index)?;
                }
            }
            _ => {
                let frame = self.frame(&ChatFrameParts {
                    kind: animus_adapters::ModelStreamFrameKind::Failure,
                    body: Some(reason),
                    tool_call_id: None,
                    usage: None,
                    visibility: animus_adapters::ModelStreamVisibility::Redacted,
                })?;
                self.frames.push(frame);
                self.terminal = Some(OpenAiStreamTerminal::Failed);
                return Ok(());
            }
        }
        if self.retry_progress == OpenAiRetryStreamProgress::TransportStarted {
            self.retry_progress = OpenAiRetryStreamProgress::Other;
        }
        self.pending_finish = Some(String::from(reason));
        self.terminal = Some(OpenAiStreamTerminal::Completed);
        Ok(())
    }

    fn complete_tool_call(&mut self, index: u64) -> Result<(), OpenAiStreamError> {
        let bounds = self.bounds;
        let response_id = self
            .response_id
            .clone()
            .ok_or_else(|| correlation(RESPONSE_ID_REASON))?;
        let Some(state) = self.tool_states.remove(&index) else {
            return Err(correlation(TOOL_CORRELATION_REASON));
        };
        let admitted = state.tool_name.as_deref().is_some_and(|name| {
            self.dispatch
                .capabilities()
                .tools()
                .iter()
                .any(|tool| tool.name() == name)
        });
        if state.tool_call_id.is_empty() || !admitted {
            return Err(correlation(TOOL_CORRELATION_REASON));
        }
        debug_assert!(state.arguments.len() <= bounds.tool_argument_bytes);
        self.tool_calls.push(OpenAiCompletedToolCall {
            model_call_id: self.model_call_id.clone(),
            provider_response_id: response_id,
            output_item_id: state.tool_call_id.clone(),
            tool_call_id: state.tool_call_id,
            tool_name: state.tool_name.unwrap_or_default(),
            arguments_json: state.arguments,
        });
        Ok(())
    }

    fn accept_usage(
        &mut self,
        object: &serde_json::Map<String, Value>,
    ) -> Result<(), OpenAiStreamError> {
        let usage = object.get("usage").filter(|usage| !usage.is_null());
        let Some(usage) = usage else {
            return Err(malformed(MALFORMED_SSE_REASON));
        };
        self.accept_usage_value(usage)
    }

    fn accept_usage_value(&mut self, usage: &Value) -> Result<(), OpenAiStreamError> {
        if self.usage_emitted {
            return Err(correlation(RESPONSE_ID_REASON));
        }
        let object = usage
            .as_object()
            .ok_or_else(|| malformed(MALFORMED_SSE_REASON))?;
        let input = required_u32(object, "prompt_tokens")?;
        let output = required_u32(object, "completion_tokens")?;
        let frame = self.frame(&ChatFrameParts {
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
        parts: &ChatFrameParts<'_>,
    ) -> Result<animus_adapters::ModelStreamFrame, OpenAiStreamError> {
        let order = self.next_order;
        self.next_order = self
            .next_order
            .checked_add(ORDER_INCREMENT)
            .ok_or_else(|| lifecycle(TERMINAL_REASON))?;
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
}

const STREAM_BOUND_REASON: &str = "chat-completions SSE exceeds a stream bound";
const MALFORMED_SSE_REASON: &str = "chat-completions SSE is malformed";

fn finish_reason(
    object: &serde_json::Map<String, Value>,
) -> Result<Option<&str>, OpenAiStreamError> {
    let choice = object.get("choices").and_then(Value::as_array);
    let Some(choices) = choice else {
        return Ok(None);
    };
    let [choice] = choices.as_slice() else {
        return Err(malformed(UNSUPPORTED_CHOICE_REASON));
    };
    match choice.get("finish_reason") {
        None | Some(Value::Null) => Ok(None),
        Some(Value::String(reason)) => Ok(Some(reason.as_str())),
        Some(_) => Err(malformed(MALFORMED_SSE_REASON)),
    }
}

fn required_string<'a>(
    object: &'a serde_json::Map<String, Value>,
    field: &str,
) -> Result<&'a str, OpenAiStreamError> {
    object
        .get(field)
        .and_then(Value::as_str)
        .ok_or_else(|| correlation(RESPONSE_ID_REASON))
}

fn required_u32(
    object: &serde_json::Map<String, Value>,
    field: &str,
) -> Result<u32, OpenAiStreamError> {
    object
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| malformed(MALFORMED_SSE_REASON))
}

/// Builds one Z.ai coding-plan chat-completions request body.
///
/// # Errors
///
/// Returns [`crate::HostError`] when the plan, dispatch, or cohort is unsupported.
fn validate_zai_chat_plan(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: &OpenAiToolDispatch,
) -> Result<(), crate::HostError> {
    use crate::tool::{
        validate_continuation, validate_roles, validate_stream_channels, validate_tools,
    };

    if config.endpoint_identity() != crate::ZAI_CODING_PLAN_COMPLETIONS_ENDPOINT_IDENTITY
        || plan.endpoint() != config.endpoint_identity()
    {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_ENDPOINT,
        });
    }
    if plan.api_kind() != animus_std::ApiKind::OpenAiChat {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_PROVIDER_FAMILY,
        });
    }
    let request = plan.request();
    if request.model() != dispatch.capabilities().model_id() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_TOOL_CAPABILITY,
        });
    }
    if !request.stream() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_STREAMING,
        });
    }
    if request.tools().is_empty() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_TOOL_CAPABILITY,
        });
    }
    if request.response_format() != crate::tool::RESPONSE_FORMAT_TEXT {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_STRUCTURED_OUTPUT,
        });
    }
    if !request.stop_sequences().is_empty() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_STOP_SEQUENCES,
        });
    }
    if dispatch.policy().parallel_tool_calls() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_PARALLEL_TOOL_CALLS,
        });
    }
    validate_roles(request, dispatch.capabilities())?;
    validate_tools(request, dispatch.capabilities(), dispatch.schemas())?;
    validate_stream_channels(request, dispatch.capabilities())?;
    validate_continuation(dispatch, request)
}

/// Builds one Z.ai coding-plan chat-completions request body.
///
/// # Errors
///
/// Returns [`crate::HostError`] when the plan, dispatch, or cohort is unsupported.
pub fn build_zai_chat_completions_request_body(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: &OpenAiToolDispatch,
) -> Result<Vec<u8>, crate::HostError> {
    use crate::tool::schema_map;

    validate_zai_chat_plan(config, plan, dispatch)?;
    let request = plan.request();
    let schemas = schema_map(dispatch.schemas())?;
    let tools = request
        .tools()
        .iter()
        .map(|tool| {
            let parameters = schemas.get(tool.input_schema_label()).ok_or(
                crate::HostError::InvalidToolSchema {
                    reason: "chat tool schema binding is missing",
                },
            )?;
            Ok(serde_json::json!({
                "type": "function",
                "function": {
                    "name": tool.name(),
                    "description": tool.description(),
                    "parameters": parameters,
                },
            }))
        })
        .collect::<Result<Vec<_>, crate::HostError>>()?;
    let messages = match dispatch.continuation_parts() {
        None => request
            .messages()
            .iter()
            .map(|message| {
                if message.role() == "tool" || message.tool_call_id().is_some() {
                    return Err(crate::HostError::UnsupportedPlan {
                        feature: crate::tool::FEATURE_TOOL_RESULT_SHAPE,
                    });
                }
                let text = message
                    .content()
                    .iter()
                    .map(|part| match part {
                        animus_std::OpenAiResponsesContentPart::Text { text } => Ok(text.as_str()),
                        animus_std::OpenAiResponsesContentPart::ImageDescriptor { .. } => {
                            Err(crate::HostError::UnsupportedPlan { feature: "image" })
                        }
                    })
                    .collect::<Result<Vec<&str>, crate::HostError>>()?
                    .concat();
                Ok(serde_json::json!({ "role": message.role(), "content": text }))
            })
            .collect::<Result<Vec<_>, crate::HostError>>()?,
        Some((binding, result)) => vec![
            serde_json::json!({
                "role": "assistant",
                "content": serde_json::Value::Null,
                "tool_calls": [{
                    "id": binding.tool_call_id(),
                    "type": "function",
                    "function": {
                        "name": binding.tool_name(),
                        "arguments": binding.arguments_json(),
                    },
                }],
            }),
            serde_json::json!({
                "role": "tool",
                "tool_call_id": binding.tool_call_id(),
                "content": result.output(),
            }),
        ],
    };
    let body = serde_json::json!({
        "model": request.model(),
        "messages": messages,
        "tools": tools,
        "tool_choice": crate::tool::project_chat_tool_choice(
            dispatch.policy().choice(),
            request,
        )?,
        "parallel_tool_calls": false,
        "stream": true,
        "max_tokens": request.max_output_tokens(),
        "temperature": request
            .temperature_millis()
            .map(|value| f64::from(value) / 1_000.0),
    });
    serde_json::to_vec(&body).map_err(|_error| crate::HostError::RequestEncoding)
}
