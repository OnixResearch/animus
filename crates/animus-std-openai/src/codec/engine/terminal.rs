#[derive(Clone, Copy)]
pub(super) struct FrameParts<'a> {
    pub(super) kind: animus_adapters::ModelStreamFrameKind,
    pub(super) body: Option<&'a str>,
    pub(super) tool_call_id: Option<&'a str>,
    pub(super) usage: Option<animus_adapters::ModelStreamUsage>,
    pub(super) visibility: animus_adapters::ModelStreamVisibility,
}

impl super::OpenAiResponsesSseParser {
    pub(super) fn response_completed(
        &mut self,
        value: &serde_json::Value,
    ) -> Result<(), crate::OpenAiStreamError> {
        self.require_started()?;
        let response = crate::codec::fields::required_object(value, "response")?;
        self.check_response_id(crate::codec::fields::required_string(response, "id")?)?;
        if response.get("status").and_then(serde_json::Value::as_str) != Some("completed") {
            return Err(crate::codec::contracts::lifecycle(
                crate::codec::vocabulary::TERMINAL_BODY_REASON,
            ));
        }
        if let Some(usage) = response.get("usage") {
            self.emit_usage(usage)?;
        }
        let frame = self.terminal_frame(animus_adapters::ModelStreamFrameKind::Finish, None)?;
        self.frames.push(frame);
        self.terminal = Some(crate::OpenAiStreamTerminal::Completed);
        Ok(())
    }

    pub(super) fn provider_terminal(
        &mut self,
        value: &serde_json::Value,
        reason: &'static str,
    ) -> Result<(), crate::OpenAiStreamError> {
        self.require_started()?;
        if let Some(response) = value.get("response") {
            let response = response.as_object().ok_or_else(|| {
                crate::codec::contracts::malformed(crate::codec::vocabulary::TERMINAL_BODY_REASON)
            })?;
            if let Some(response_id) = response.get("id").and_then(serde_json::Value::as_str) {
                self.check_response_id(response_id)?;
            }
        }
        let frame =
            self.terminal_frame(animus_adapters::ModelStreamFrameKind::Failure, Some(reason))?;
        self.frames.push(frame);
        self.terminal = Some(crate::OpenAiStreamTerminal::Failed);
        if self.retry_progress == crate::OpenAiRetryStreamProgress::TransportStarted {
            self.retry_progress = crate::OpenAiRetryStreamProgress::Other;
        }
        Ok(())
    }

    fn emit_usage(&mut self, usage: &serde_json::Value) -> Result<(), crate::OpenAiStreamError> {
        if self.usage_emitted {
            return Err(crate::codec::contracts::lifecycle(
                crate::codec::vocabulary::DUPLICATE_USAGE_REASON,
            ));
        }
        let input = crate::codec::fields::required_u32(usage, "input_tokens")?;
        let output = crate::codec::fields::required_u32(usage, "output_tokens")?;
        let frame = self.frame(FrameParts {
            kind: animus_adapters::ModelStreamFrameKind::Usage,
            body: None,
            tool_call_id: None,
            usage: Some(animus_adapters::ModelStreamUsage::new(input, output)),
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })?;
        self.frames.push(frame);
        self.usage_emitted = true;
        if self.retry_progress == crate::OpenAiRetryStreamProgress::TransportStarted {
            self.retry_progress = crate::OpenAiRetryStreamProgress::Other;
        }
        Ok(())
    }

    pub(super) fn frame(
        &mut self,
        parts: FrameParts<'_>,
    ) -> Result<animus_adapters::ModelStreamFrame, crate::OpenAiStreamError> {
        let order = self.next_order;
        let next_order = order
            .checked_add(crate::codec::vocabulary::ORDER_INCREMENT)
            .ok_or_else(|| {
                crate::codec::contracts::lifecycle(crate::codec::vocabulary::SEQUENCE_REASON)
            })?;
        let frame =
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
                crate::OpenAiStreamError::new(
                    crate::OpenAiStreamFailureClass::Canonical,
                    crate::codec::vocabulary::MALFORMED_SSE_REASON,
                )
            })?;
        self.next_order = next_order;
        Ok(frame)
    }

    pub(super) fn terminal_frame(
        &mut self,
        kind: animus_adapters::ModelStreamFrameKind,
        body: Option<&str>,
    ) -> Result<animus_adapters::ModelStreamFrame, crate::OpenAiStreamError> {
        self.frame(FrameParts {
            kind,
            body,
            tool_call_id: None,
            usage: None,
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })
    }
}
