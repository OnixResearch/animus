//! Private replay values. No persistence, logging, transport, or credential values.

pub(super) const MAX_HISTORY_BYTES: usize = 1_048_576;
pub(super) const MAX_HISTORY_MESSAGES: usize = 64;
const HISTORY_FEATURE: &str = "chat-continuation-history";

#[cfg(test)]
mod tests;

#[derive(Clone, Eq, PartialEq)]
pub(crate) struct Replay {
    model: String,
    provider: String,
    endpoint: String,
    credential_source: String,
    reasoning_effort: String,
    schema_identity: String,
    messages: Vec<serde_json::Value>,
}

impl Replay {
    pub(super) fn new(
        plan: &animus_std::ProviderDispatchPlan,
        messages: Vec<serde_json::Value>,
        schema_identity: String,
    ) -> Result<Self, crate::HostError> {
        check_messages(&messages)?;
        Ok(Self {
            model: String::from(plan.request().model()),
            provider: String::from(plan.provider_id()),
            endpoint: String::from(plan.endpoint()),
            credential_source: String::from(plan.credential_source_label()),
            reasoning_effort: String::from(plan.request().reasoning_effort()),
            schema_identity,
            messages,
        })
    }

    pub(super) fn messages(&self) -> &[serde_json::Value] {
        &self.messages
    }

    pub(super) fn resume(
        &self,
        plan: &animus_std::ProviderDispatchPlan,
        schema_identity: &str,
        continuation: (&crate::tool::OpenAiContinuationBinding, &crate::tool::OpenAiToolResult),
    ) -> Result<Self, crate::HostError> {
        let (binding, result) = continuation;
        if self.model != plan.request().model()
            || self.provider != plan.provider_id()
            || self.endpoint != plan.endpoint()
            || self.credential_source != plan.credential_source_label()
            || self.reasoning_effort != plan.request().reasoning_effort()
            || self.schema_identity != schema_identity
        {
            return Err(denied());
        }
        let assistant = self.messages.last().ok_or_else(denied)?;
        let calls = assistant["tool_calls"].as_array().ok_or_else(denied)?;
        let [call] = calls.as_slice() else {
            return Err(denied());
        };
        if assistant["role"] != "assistant"
            || call["id"] != binding.tool_call_id()
            || call["function"]["name"] != binding.tool_name()
            || call["function"]["arguments"] != binding.arguments_json()
        {
            return Err(denied());
        }
        let mut next = self.clone();
        next.messages.push(
            serde_json::json!({"role": "tool", "tool_call_id": binding.tool_call_id(),
            "content": result.output()}),
        );
        check_messages(&next.messages)?;
        Ok(next)
    }

    pub(super) fn complete(
        mut self,
        frames: &[animus_adapters::ModelStreamFrame],
        call: &crate::tool::OpenAiCompletedToolCall,
    ) -> Result<Self, super::OpenAiStreamError> {
        if self.messages.iter().any(|message| {
            message["tool_calls"]
                .as_array()
                .is_some_and(|calls| calls.iter().any(|old| old["id"] == call.tool_call_id))
        }) {
            return Err(super::correlation(super::TOOL_CORRELATION_REASON));
        }
        let content: String = frames
            .iter()
            .filter(|frame| frame.kind() == animus_adapters::ModelStreamFrameKind::ContentDelta)
            .filter_map(|frame| frame.body())
            .collect();
        let reasoning: String = frames
            .iter()
            .filter(|frame| frame.kind() == animus_adapters::ModelStreamFrameKind::ThoughtDelta)
            .filter_map(|frame| frame.body())
            .collect();
        let mut assistant = serde_json::json!({"role": "assistant", "content": content,
            "tool_calls": [{"id": call.tool_call_id, "type": "function", "function": {
                "name": call.tool_name, "arguments": call.arguments_json}}]});
        if self.reasoning_effort != "none" || !reasoning.is_empty() {
            assistant["reasoning_content"] = serde_json::Value::String(reasoning);
        }
        self.messages.push(assistant);
        check_messages(&self.messages).map_err(|_error| super::bound_error())?;
        Ok(self)
    }
}

pub(super) fn denied() -> crate::HostError {
    crate::HostError::UnsupportedPlan {
        feature: HISTORY_FEATURE,
    }
}

fn check_messages(messages: &[serde_json::Value]) -> Result<(), crate::HostError> {
    if messages.is_empty() || messages.len() > MAX_HISTORY_MESSAGES {
        return Err(denied());
    }
    let encoded =
        serde_json::to_vec(messages).map_err(|_error| crate::HostError::RequestEncoding)?;
    if encoded.len() > MAX_HISTORY_BYTES {
        return Err(denied());
    }
    Ok(())
}
