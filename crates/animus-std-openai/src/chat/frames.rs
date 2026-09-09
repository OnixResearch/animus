//! Canonical frame construction and bounded usage conversion.

impl super::OpenAiChatCompletionsSseParser {
    pub(super) fn text_frame(
        &mut self,
        kind: animus_adapters::ModelStreamFrameKind,
        text: &str,
    ) -> Result<(), super::OpenAiStreamError> {
        if text.is_empty() {
            return Ok(());
        }
        let visibility = if kind == animus_adapters::ModelStreamFrameKind::ContentDelta {
            self.retry_progress = super::OpenAiRetryStreamProgress::Content;
            animus_adapters::ModelStreamVisibility::AssistantVisible
        } else {
            self.retry_progress = super::OpenAiRetryStreamProgress::Reasoning;
            animus_adapters::ModelStreamVisibility::Redacted
        };
        let frame = self.frame(&super::ChatFrameParts {
            kind,
            body: Some(text),
            tool_call_id: None,
            usage: None,
            visibility,
        })?;
        self.frames.push(frame);
        Ok(())
    }

    pub(super) fn accept_usage(
        &mut self,
        object: &serde_json::Map<String, serde_json::Value>,
    ) -> Result<(), super::OpenAiStreamError> {
        let usage = object
            .get("usage")
            .filter(|usage| !usage.is_null())
            .ok_or_else(|| super::malformed(super::MALFORMED_SSE_REASON))?;
        self.accept_usage_value(usage)
    }

    pub(super) fn accept_usage_value(
        &mut self,
        usage: &serde_json::Value,
    ) -> Result<(), super::OpenAiStreamError> {
        if self.usage_emitted {
            return Err(super::correlation(super::RESPONSE_ID_REASON));
        }
        let object = usage
            .as_object()
            .ok_or_else(|| super::malformed(super::MALFORMED_SSE_REASON))?;
        let input = required_u32(object, "prompt_tokens")?;
        let output = required_u32(object, "completion_tokens")?;
        let frame = self.frame(&super::ChatFrameParts {
            kind: animus_adapters::ModelStreamFrameKind::Usage,
            body: None,
            tool_call_id: None,
            usage: Some(animus_adapters::ModelStreamUsage::new(input, output)),
            visibility: animus_adapters::ModelStreamVisibility::Redacted,
        })?;
        self.frames.push(frame);
        self.usage_emitted = true;
        if self.retry_progress == super::OpenAiRetryStreamProgress::TransportStarted {
            self.retry_progress = super::OpenAiRetryStreamProgress::Other;
        }
        Ok(())
    }

    pub(super) fn frame(
        &mut self,
        parts: &super::ChatFrameParts<'_>,
    ) -> Result<animus_adapters::ModelStreamFrame, super::OpenAiStreamError> {
        let order = self.next_order;
        self.next_order = self
            .next_order
            .checked_add(crate::stream::ORDER_INCREMENT)
            .ok_or_else(|| super::lifecycle(super::TERMINAL_REASON))?;
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
            super::OpenAiStreamError::new(
                super::OpenAiStreamFailureClass::Canonical,
                super::MALFORMED_SSE_REASON,
            )
        })
    }
}

fn required_u32(
    object: &serde_json::Map<String, serde_json::Value>,
    field: &str,
) -> Result<u32, super::OpenAiStreamError> {
    object
        .get(field)
        .and_then(serde_json::Value::as_u64)
        .and_then(|value| u32::try_from(value).ok())
        .ok_or_else(|| super::malformed(super::MALFORMED_SSE_REASON))
}
