pub(super) fn project(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    dispatch: &crate::OpenAiToolDispatch,
) -> Result<(Vec<serde_json::Value>, Option<String>), crate::HostError> {
    match dispatch.continuation_parts() {
        None => Ok((
            request
                .messages()
                .iter()
                .map(project_regular_message)
                .collect::<Result<Vec<_>, _>>()?,
            None,
        )),
        Some((binding, result)) => Ok((
            vec![serde_json::json!({
                "type": "function_call_output", "call_id": binding.tool_call_id(), "output": result.output(),
            })],
            Some(String::from(binding.provider_response_id())),
        )),
    }
}

fn project_regular_message(
    message: &animus_std::OpenAiResponsesMessage,
) -> Result<serde_json::Value, crate::HostError> {
    if message.role() == "tool" || message.tool_call_id().is_some() {
        return crate::declarations::unsupported("tool_result_shape");
    }
    let mut content = Vec::with_capacity(message.content().len());
    for part in message.content() {
        match part {
            animus_std::OpenAiResponsesContentPart::Text { text } => {
                content.push(serde_json::json!({"type": "input_text", "text": text}));
            }
            animus_std::OpenAiResponsesContentPart::ImageDescriptor { .. } => {
                return crate::declarations::unsupported("image");
            }
        }
    }
    Ok(serde_json::json!({"role": message.role(), "content": content}))
}
