//! Serial function-call assembly with immutable call identities.

impl super::OpenAiChatCompletionsSseParser {
    pub(super) fn tool_call_delta(
        &mut self,
        call: &serde_json::Value,
    ) -> Result<(), super::OpenAiStreamError> {
        let object = call
            .as_object()
            .ok_or_else(|| super::malformed(super::MALFORMED_SSE_REASON))?;
        let index = object
            .get("index")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| super::correlation(super::TOOL_CORRELATION_REASON))?;
        let function = super::event::tool_function(object)?;
        if let Some(id) = object.get("id").and_then(serde_json::Value::as_str) {
            if self
                .tool_states
                .iter()
                .any(|(bound_index, state)| *bound_index != index && state.tool_call_id == id)
            {
                return Err(super::correlation(super::TOOL_CORRELATION_REASON));
            }
        }
        let name = function
            .and_then(|function| function.get("name"))
            .and_then(serde_json::Value::as_str);
        let arguments = function
            .and_then(|function| function.get("arguments"))
            .and_then(serde_json::Value::as_str)
            .unwrap_or("");
        if !self.tool_states.contains_key(&index)
            && self.tool_states.len() >= self.bounds.tool_calls
        {
            return Err(super::bound_error());
        }
        let state = self
            .tool_states
            .entry(index)
            .or_insert_with(|| super::ChatToolState {
                tool_call_id: object
                    .get("id")
                    .and_then(serde_json::Value::as_str)
                    .unwrap_or_default()
                    .to_owned(),
                tool_name: None,
                arguments: String::new(),
            });
        if let Some(id) = object.get("id").and_then(serde_json::Value::as_str) {
            if state.tool_call_id.is_empty() {
                state.tool_call_id = String::from(id);
            } else if state.tool_call_id != id {
                return Err(super::correlation(super::TOOL_CORRELATION_REASON));
            }
        }
        if state.tool_call_id.is_empty() {
            return Err(super::correlation(super::TOOL_CORRELATION_REASON));
        }
        if let Some(name) = name {
            bind_name(state, name, self.dispatch.capabilities())?;
        }
        if state.arguments.len().saturating_add(arguments.len()) > self.bounds.tool_argument_bytes {
            return Err(super::bound_error());
        }
        state.arguments.push_str(arguments);
        let tool_call_id = state.tool_call_id.clone();
        if arguments.is_empty() {
            return Ok(());
        }
        let frame = self.frame(&super::ChatFrameParts {
            kind: animus_adapters::ModelStreamFrameKind::ToolCallDelta,
            body: Some(arguments),
            tool_call_id: Some(tool_call_id.as_str()),
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })?;
        self.frames.push(frame);
        self.retry_progress = super::OpenAiRetryStreamProgress::ToolCall;
        Ok(())
    }

    pub(super) fn finish_stream(&mut self, reason: &str) -> Result<(), super::OpenAiStreamError> {
        if self.terminal.is_some() {
            return Err(super::lifecycle(super::TERMINAL_REASON));
        }
        match reason {
            "stop" if self.tool_states.is_empty() => {}
            "tool_calls" if !self.tool_states.is_empty() => {
                let indexes: Vec<u64> = self.tool_states.keys().copied().collect();
                for index in indexes {
                    self.complete_tool_call(index)?;
                }
            }
            _ => {
                let frame = self.frame(&super::ChatFrameParts {
                    kind: animus_adapters::ModelStreamFrameKind::Failure,
                    body: Some(reason),
                    tool_call_id: None,
                    usage: None,
                    visibility: animus_adapters::ModelStreamVisibility::Redacted,
                })?;
                self.frames.push(frame);
                self.terminal = Some(super::OpenAiStreamTerminal::Failed);
                return Ok(());
            }
        }
        if self.retry_progress == super::OpenAiRetryStreamProgress::TransportStarted {
            self.retry_progress = super::OpenAiRetryStreamProgress::Other;
        }
        self.pending_finish = Some(String::from(reason));
        self.terminal = Some(super::OpenAiStreamTerminal::Completed);
        Ok(())
    }

    fn complete_tool_call(&mut self, index: u64) -> Result<(), super::OpenAiStreamError> {
        let response_id = self
            .response_id
            .clone()
            .ok_or_else(|| super::correlation(super::RESPONSE_ID_REASON))?;
        let state = self
            .tool_states
            .remove(&index)
            .ok_or_else(|| super::correlation(super::TOOL_CORRELATION_REASON))?;
        let admitted = state.tool_name.as_deref().is_some_and(|name| {
            self.dispatch
                .capabilities()
                .tools()
                .iter()
                .any(|tool| tool.name() == name)
        });
        if state.tool_call_id.is_empty() || !admitted {
            return Err(super::correlation(super::TOOL_CORRELATION_REASON));
        }
        debug_assert!(state.arguments.len() <= self.bounds.tool_argument_bytes);
        let arguments: serde_json::Value = serde_json::from_str(&state.arguments)
            .map_err(|_error| super::malformed(super::MALFORMED_SSE_REASON))?;
        if !arguments.is_object() {
            return Err(super::malformed(super::MALFORMED_SSE_REASON));
        }
        self.tool_calls.push(crate::tool::OpenAiCompletedToolCall {
            model_call_id: self.model_call_id.clone(),
            provider_response_id: response_id,
            output_item_id: state.tool_call_id.clone(),
            tool_call_id: state.tool_call_id,
            tool_name: state.tool_name.unwrap_or_default(),
            arguments_json: state.arguments,
            chat_replay: None,
            api_kind: animus_std::ApiKind::OpenAiChat,
        });
        Ok(())
    }
}

fn bind_name(
    state: &mut super::ChatToolState,
    name: &str,
    capabilities: &crate::tool::OpenAiToolCapabilityMatrix,
) -> Result<(), super::OpenAiStreamError> {
    match state.tool_name.as_deref() {
        None => {
            if name.is_empty() || !capabilities.tools().iter().any(|tool| tool.name() == name) {
                return Err(super::correlation(super::TOOL_CORRELATION_REASON));
            }
            state.tool_name = Some(String::from(name));
        }
        Some(bound) if bound == name => {}
        Some(_) => return Err(super::correlation(super::TOOL_CORRELATION_REASON)),
    }
    Ok(())
}
