impl super::OpenAiResponsesSseParser {
    pub(super) fn process_json_event(
        &mut self,
        kind: &str,
        value: &serde_json::Value,
    ) -> Result<(), crate::OpenAiStreamError> {
        debug_assert!(self.terminal.is_none());
        debug_assert!(!self.stream_end_seen);
        match kind {
            "response.created" => self.start_response(value),
            "response.in_progress" => self.progress_response(value),
            "response.output_text.delta" => {
                self.text_delta(value, animus_adapters::ModelStreamFrameKind::ContentDelta)
            }
            "response.output_text.done"
            | "response.content_part.added"
            | "response.content_part.done" => {
                let is_reasoning = value
                    .get("part")
                    .and_then(|part| part.get("type"))
                    .and_then(serde_json::Value::as_str)
                    == Some("reasoning_text");
                let expected = if is_reasoning {
                    crate::codec::fields::OutputItemKind::Reasoning
                } else {
                    crate::codec::fields::OutputItemKind::Message
                };
                self.output_lifecycle(value, expected)
            }
            "response.reasoning_summary_text.delta" | "response.reasoning_text.delta" => {
                self.text_delta(value, animus_adapters::ModelStreamFrameKind::ThoughtDelta)
            }
            "response.reasoning_summary_text.done"
            | "response.reasoning_summary_part.added"
            | "response.reasoning_summary_part.done"
            | "response.reasoning_text.done" => {
                self.output_lifecycle(value, crate::codec::fields::OutputItemKind::Reasoning)
            }
            "response.output_item.added" => self.output_item_added(value),
            "response.function_call_arguments.delta" => self.function_arguments_delta(value),
            "response.function_call_arguments.done" => self.function_arguments_done(value),
            "response.output_item.done" => self.output_item_done(value),
            "response.completed" => self.response_completed(value),
            "response.failed" => {
                self.provider_terminal(value, crate::codec::vocabulary::PROVIDER_FAILURE_REASON)
            }
            "response.incomplete" => {
                self.provider_terminal(value, crate::codec::vocabulary::PROVIDER_INCOMPLETE_REASON)
            }
            "error" => {
                self.provider_terminal(value, crate::codec::vocabulary::PROVIDER_ERROR_REASON)
            }
            _unknown if kind.contains("reasoning") => Err(crate::OpenAiStreamError::new(
                crate::OpenAiStreamFailureClass::UnsafeReasoning,
                crate::codec::vocabulary::REASONING_EVENT_REASON,
            )),
            _unknown => Err(crate::OpenAiStreamError::new(
                crate::OpenAiStreamFailureClass::UnsupportedEvent,
                crate::codec::vocabulary::UNKNOWN_EVENT_REASON,
            )),
        }
    }

    fn start_response(
        &mut self,
        value: &serde_json::Value,
    ) -> Result<(), crate::OpenAiStreamError> {
        if self.lifecycle_started {
            return Err(crate::codec::contracts::lifecycle(
                crate::codec::vocabulary::RESPONSE_LIFECYCLE_REASON,
            ));
        }
        let response = crate::codec::fields::required_object(value, "response")?;
        let response_id = crate::codec::fields::required_string(response, "id")?;
        if response.get("status").and_then(serde_json::Value::as_str) != Some("in_progress") {
            return Err(crate::codec::contracts::lifecycle(
                crate::codec::vocabulary::RESPONSE_LIFECYCLE_REASON,
            ));
        }
        self.response_id = Some(String::from(response_id));
        self.lifecycle_started = true;
        Ok(())
    }

    fn progress_response(
        &mut self,
        value: &serde_json::Value,
    ) -> Result<(), crate::OpenAiStreamError> {
        self.require_started()?;
        let response = crate::codec::fields::required_object(value, "response")?;
        self.check_response_id(crate::codec::fields::required_string(response, "id")?)
    }

    fn text_delta(
        &mut self,
        value: &serde_json::Value,
        kind: animus_adapters::ModelStreamFrameKind,
    ) -> Result<(), crate::OpenAiStreamError> {
        self.require_started()?;
        if value
            .get("response_id")
            .and_then(serde_json::Value::as_str)
            .is_some()
        {
            self.check_response_id(crate::codec::fields::required_string(value, "response_id")?)?;
        }
        let is_content = kind == animus_adapters::ModelStreamFrameKind::ContentDelta;
        let expected_item_kind = if is_content {
            crate::codec::fields::OutputItemKind::Message
        } else {
            crate::codec::fields::OutputItemKind::Reasoning
        };
        self.require_output_item(
            crate::codec::fields::required_string(value, "item_id")?,
            expected_item_kind,
        )?;
        if !is_content
            && value
                .get("summary_index")
                .and_then(serde_json::Value::as_u64)
                .is_none()
            && value
                .get("content_index")
                .and_then(serde_json::Value::as_u64)
                .is_none()
        {
            return Err(crate::OpenAiStreamError::new(
                crate::OpenAiStreamFailureClass::UnsafeReasoning,
                crate::codec::vocabulary::REASONING_EVENT_REASON,
            ));
        }
        let delta = crate::codec::fields::required_string(value, "delta")?;
        let visibility = if is_content {
            animus_adapters::ModelStreamVisibility::AssistantVisible
        } else {
            animus_adapters::ModelStreamVisibility::Redacted
        };
        let frame = self.frame(super::terminal::FrameParts {
            kind,
            body: Some(delta),
            tool_call_id: None,
            usage: None,
            visibility,
        })?;
        debug_assert_eq!(frame.kind(), kind);
        debug_assert!(self.lifecycle_started);
        self.retry_progress = if is_content {
            crate::OpenAiRetryStreamProgress::Content
        } else {
            crate::OpenAiRetryStreamProgress::Reasoning
        };
        self.frames.push(frame);
        Ok(())
    }

    fn output_lifecycle(
        &self,
        value: &serde_json::Value,
        expected: crate::codec::fields::OutputItemKind,
    ) -> Result<(), crate::OpenAiStreamError> {
        self.require_started()?;
        self.require_output_item(
            crate::codec::fields::required_string(value, "item_id")?,
            expected,
        )
    }

    fn output_item_added(
        &mut self,
        value: &serde_json::Value,
    ) -> Result<(), crate::OpenAiStreamError> {
        self.require_started()?;
        let item = crate::codec::fields::required_object(value, "item")?;
        let output_item_id = crate::codec::fields::required_string(item, "id")?;
        let item_kind = crate::codec::fields::OutputItemKind::parse(
            crate::codec::fields::required_string(item, "type")?,
        )?;
        if self.output_items.contains_key(output_item_id) {
            return Err(crate::codec::contracts::correlation(
                crate::codec::vocabulary::TOOL_CORRELATION_REASON,
            ));
        }
        if item_kind == crate::codec::fields::OutputItemKind::FunctionCall {
            self.register_function(output_item_id, item)?;
        }
        self.output_items
            .insert(String::from(output_item_id), item_kind);
        Ok(())
    }

    fn output_item_done(
        &mut self,
        value: &serde_json::Value,
    ) -> Result<(), crate::OpenAiStreamError> {
        self.require_started()?;
        let item = crate::codec::fields::required_object(value, "item")?;
        let output_item_id = crate::codec::fields::required_string(item, "id")?;
        let item_kind = crate::codec::fields::OutputItemKind::parse(
            crate::codec::fields::required_string(item, "type")?,
        )?;
        self.require_output_item(output_item_id, item_kind)?;
        if item_kind == crate::codec::fields::OutputItemKind::FunctionCall {
            self.complete_function(output_item_id, item)?;
        }
        self.completed_output_items
            .insert(String::from(output_item_id));
        Ok(())
    }
}
