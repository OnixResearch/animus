//! Validated single-choice chat events and final-delta ordering.

use super::{
    MALFORMED_SSE_REASON, OpenAiChatCompletionsSseParser, OpenAiStreamError, RESPONSE_ID_REASON,
    TERMINAL_REASON, UNSUPPORTED_CHOICE_REASON, correlation, lifecycle, malformed, required_string,
};
use serde_json::{Map, Value};

const SELECTED_CHOICE_INDEX: u64 = 0;
const ASSISTANT_ROLE: &str = "assistant";
const CHUNK_OBJECT: &str = "chat.completion.chunk";

pub(super) fn tool_function(
    object: &Map<String, Value>,
) -> Result<Option<&Map<String, Value>>, OpenAiStreamError> {
    if object
        .keys()
        .any(|key| !matches!(key.as_str(), "index" | "id" | "type" | "function"))
        || object
            .get("type")
            .is_some_and(|kind| kind.as_str() != Some("function"))
        || object
            .get("id")
            .is_some_and(|id| id.as_str().is_none_or(str::is_empty))
    {
        return Err(malformed(super::TOOL_CORRELATION_REASON));
    }
    let function = object.get("function").and_then(Value::as_object);
    if object.contains_key("function") && function.is_none() {
        return Err(malformed(super::TOOL_CORRELATION_REASON));
    }
    if let Some(function) = function {
        if function
            .iter()
            .any(|(key, value)| !matches!(key.as_str(), "name" | "arguments") || !value.is_string())
        {
            return Err(malformed(super::TOOL_CORRELATION_REASON));
        }
    }
    Ok(function)
}

impl OpenAiChatCompletionsSseParser {
    pub(super) fn accept_chunk_object(
        &mut self,
        object: &Map<String, Value>,
    ) -> Result<(), OpenAiStreamError> {
        self.bind_response_id(required_string(object, "id")?)?;
        if object
            .get("model")
            .is_some_and(|model| model.as_str() != Some(self.dispatch.capabilities().model_id()))
            || object
                .get("object")
                .is_some_and(|kind| kind.as_str() != Some(CHUNK_OBJECT))
        {
            return Err(correlation(RESPONSE_ID_REASON));
        }
        let choices = object
            .get("choices")
            .and_then(Value::as_array)
            .ok_or_else(|| malformed(UNSUPPORTED_CHOICE_REASON))?;
        if choices.is_empty() {
            return self.accept_usage(object);
        }
        if self.terminal.is_some() {
            return Err(lifecycle(TERMINAL_REASON));
        }
        let [choice] = choices.as_slice() else {
            return Err(malformed(UNSUPPORTED_CHOICE_REASON));
        };
        if choice.get("index").and_then(Value::as_u64) != Some(SELECTED_CHOICE_INDEX) {
            return Err(correlation(UNSUPPORTED_CHOICE_REASON));
        }
        let reason = match choice.get("finish_reason") {
            None | Some(Value::Null) => None,
            Some(Value::String(reason)) => Some(reason.as_str()),
            Some(_) => return Err(malformed(MALFORMED_SSE_REASON)),
        };
        let delta = choice
            .get("delta")
            .and_then(Value::as_object)
            .ok_or_else(|| malformed(MALFORMED_SSE_REASON))?;
        self.accept_delta(delta)?;
        if let Some(reason) = reason {
            self.finish_stream(reason)?;
        }
        if let Some(usage) = object.get("usage").filter(|usage| !usage.is_null()) {
            self.accept_usage_value(usage)?;
        }
        Ok(())
    }

    fn accept_delta(&mut self, delta: &Map<String, Value>) -> Result<(), OpenAiStreamError> {
        if delta.keys().any(|key| {
            !matches!(
                key.as_str(),
                "role" | "content" | "reasoning_content" | "tool_calls"
            )
        }) {
            return Err(malformed(UNSUPPORTED_CHOICE_REASON));
        }
        if delta
            .get("role")
            .is_some_and(|role| role.as_str() != Some(ASSISTANT_ROLE))
        {
            return Err(malformed(UNSUPPORTED_CHOICE_REASON));
        }
        for (field, kind) in [
            (
                "content",
                animus_adapters::ModelStreamFrameKind::ContentDelta,
            ),
            (
                "reasoning_content",
                animus_adapters::ModelStreamFrameKind::ThoughtDelta,
            ),
        ] {
            if let Some(value) = delta.get(field).filter(|value| !value.is_null()) {
                let text = value
                    .as_str()
                    .ok_or_else(|| malformed(MALFORMED_SSE_REASON))?;
                self.text_frame(kind, text)?;
            }
        }
        if let Some(value) = delta.get("tool_calls").filter(|value| !value.is_null()) {
            let calls = value
                .as_array()
                .ok_or_else(|| malformed(MALFORMED_SSE_REASON))?;
            for call in calls {
                self.tool_call_delta(call)?;
            }
        }
        Ok(())
    }
}
