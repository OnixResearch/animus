#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ToolState {
    tool_call_id: String,
    tool_name: String,
    arguments: String,
    arguments_done: bool,
    pub(super) done: bool,
}

impl super::OpenAiResponsesSseParser {
    pub(super) fn register_function(
        &mut self,
        output_item_id: &str,
        item: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), crate::OpenAiStreamError> {
        let tool_call_id = crate::codec::fields::required_string(item, "call_id")?;
        let tool_name = crate::codec::fields::required_string(item, "name")?;
        if !self
            .dispatch
            .capabilities()
            .tools()
            .iter()
            .any(|tool| tool.name() == tool_name)
        {
            return Err(crate::codec::contracts::correlation(
                crate::codec::vocabulary::TOOL_CORRELATION_REASON,
            ));
        }
        if crate::codec::contracts::portable_len(self.tool_states.len())? >= self.bounds.tool_calls
            || self.tool_states.contains_key(output_item_id)
            || self
                .tool_states
                .values()
                .any(|state| state.tool_call_id == tool_call_id)
        {
            return Err(crate::codec::contracts::correlation(
                crate::codec::vocabulary::TOOL_CALL_BOUND_REASON,
            ));
        }
        let initial_arguments = item
            .get("arguments")
            .and_then(serde_json::Value::as_str)
            .map_or("", |value| value);
        if crate::codec::contracts::portable_len(initial_arguments.len())?
            > self.bounds.tool_argument_bytes
        {
            return Err(crate::codec::contracts::bound_tool_argument());
        }
        debug_assert!(!self.tool_states.contains_key(output_item_id));
        debug_assert!(
            !self
                .tool_states
                .values()
                .any(|state| state.tool_call_id == tool_call_id)
        );
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

    pub(super) fn function_arguments_delta(
        &mut self,
        value: &serde_json::Value,
    ) -> Result<(), crate::OpenAiStreamError> {
        self.require_started()?;
        let item_id = crate::codec::fields::required_string(value, "item_id")?;
        let delta = crate::codec::fields::required_string(value, "delta")?;
        let state = self.tool_states.get_mut(item_id).ok_or_else(|| {
            crate::codec::contracts::correlation(crate::codec::vocabulary::TOOL_CORRELATION_REASON)
        })?;
        if state.arguments_done || state.done {
            return Err(crate::codec::contracts::correlation(
                crate::codec::vocabulary::TOOL_CORRELATION_REASON,
            ));
        }
        let new_len = crate::codec::contracts::portable_len(state.arguments.len())?
            .checked_add(crate::codec::contracts::portable_len(delta.len())?)
            .ok_or_else(crate::codec::contracts::bound_tool_argument)?;
        if new_len > self.bounds.tool_argument_bytes {
            return Err(crate::codec::contracts::bound_tool_argument());
        }
        debug_assert!(new_len <= self.bounds.tool_argument_bytes);
        debug_assert!(!state.arguments_done);
        state.arguments.push_str(delta);
        let tool_call_id = state.tool_call_id.clone();
        let frame = self.frame(super::terminal::FrameParts {
            kind: animus_adapters::ModelStreamFrameKind::ToolCallDelta,
            body: Some(delta),
            tool_call_id: Some(tool_call_id.as_str()),
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })?;
        self.retry_progress = crate::OpenAiRetryStreamProgress::ToolCall;
        self.frames.push(frame);
        Ok(())
    }

    pub(super) fn function_arguments_done(
        &mut self,
        value: &serde_json::Value,
    ) -> Result<(), crate::OpenAiStreamError> {
        self.require_started()?;
        let item_id = crate::codec::fields::required_string(value, "item_id")?;
        let arguments = crate::codec::fields::required_string(value, "arguments")?;
        let state = self.tool_states.get_mut(item_id).ok_or_else(|| {
            crate::codec::contracts::correlation(crate::codec::vocabulary::TOOL_CORRELATION_REASON)
        })?;
        if state.arguments_done || state.done || state.arguments != arguments {
            return Err(crate::codec::contracts::correlation(
                crate::codec::vocabulary::TOOL_CORRELATION_REASON,
            ));
        }
        serde_json::from_str::<serde_json::Value>(arguments).map_err(|_error| {
            crate::codec::contracts::correlation(crate::codec::vocabulary::TOOL_ARGUMENT_REASON)
        })?;
        state.arguments_done = true;
        Ok(())
    }

    pub(super) fn complete_function(
        &mut self,
        output_item_id: &str,
        item: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), crate::OpenAiStreamError> {
        let tool_call_id = crate::codec::fields::required_string(item, "call_id")?;
        let tool_name = crate::codec::fields::required_string(item, "name")?;
        let arguments = crate::codec::fields::required_string(item, "arguments")?;
        let state = self.tool_states.get_mut(output_item_id).ok_or_else(|| {
            crate::codec::contracts::correlation(crate::codec::vocabulary::TOOL_CORRELATION_REASON)
        })?;
        let has_wrong_identity = state.tool_call_id != tool_call_id || state.tool_name != tool_name;
        let has_wrong_arguments = !state.arguments_done || state.arguments != arguments;
        if state.done || has_wrong_identity || has_wrong_arguments {
            return Err(crate::codec::contracts::correlation(
                crate::codec::vocabulary::TOOL_CORRELATION_REASON,
            ));
        }
        let response_id = self.response_id.as_deref().ok_or_else(|| {
            crate::codec::contracts::correlation(crate::codec::vocabulary::RESPONSE_ID_REASON)
        })?;
        debug_assert!(state.arguments_done);
        debug_assert_eq!(state.arguments, arguments);
        state.done = true;
        self.tool_calls.push(crate::OpenAiCompletedToolCall {
            model_call_id: self.model_call_id.clone(),
            provider_response_id: String::from(response_id),
            output_item_id: String::from(output_item_id),
            tool_call_id: String::from(tool_call_id),
            tool_name: String::from(tool_name),
            arguments_json: String::from(arguments),
        });
        Ok(())
    }
}
