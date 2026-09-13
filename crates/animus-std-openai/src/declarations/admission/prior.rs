pub(super) fn validate(
    dispatch: &super::OpenAiToolDispatch,
    request: &animus_std::OpenAiResponsesDispatchRequest,
) -> Result<(), crate::HostError> {
    match dispatch.continuation_parts() {
        None => {
            if request
                .messages()
                .iter()
                .any(|message| message.role() == "tool")
            {
                return Err(crate::HostError::Correlation {
                    reason: crate::declarations::INITIAL_TOOL_RESULT_REASON,
                });
            }
            Ok(())
        }
        Some((binding, result)) => {
            if dispatch.model_call_id() != binding.model_call_id() || !result.matches(binding) {
                return Err(crate::HostError::Correlation {
                    reason: crate::declarations::CONTINUATION_MISMATCH_REASON,
                });
            }
            validate_message(request, binding, result)
        }
    }
}

fn validate_message(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    binding: &crate::OpenAiContinuationBinding,
    result: &crate::OpenAiToolResult,
) -> Result<(), crate::HostError> {
    let [message] = request.messages() else {
        return Err(crate::HostError::Correlation {
            reason: crate::declarations::CONTINUATION_MESSAGE_REASON,
        });
    };
    if message.role() != "tool" || message.tool_call_id() != Some(binding.tool_call_id()) {
        return Err(crate::HostError::Correlation {
            reason: crate::declarations::CONTINUATION_MESSAGE_REASON,
        });
    }
    if tool_result_text(message)? != result.output() {
        return Err(crate::HostError::Correlation {
            reason: crate::declarations::TOOL_RESULT_VALUE_REASON,
        });
    }
    Ok(())
}

fn tool_result_text(
    message: &animus_std::OpenAiResponsesMessage,
) -> Result<&str, crate::HostError> {
    let [part] = message.content() else {
        return Err(crate::HostError::Correlation {
            reason: crate::declarations::TOOL_RESULT_TEXT_REASON,
        });
    };
    match part {
        animus_std::OpenAiResponsesContentPart::Text { text } => Ok(String::as_str(text)),
        animus_std::OpenAiResponsesContentPart::ImageDescriptor { .. } => {
            Err(crate::HostError::Correlation {
                reason: crate::declarations::TOOL_RESULT_TEXT_REASON,
            })
        }
    }
}
