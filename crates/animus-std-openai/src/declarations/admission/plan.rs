pub(in crate::declarations) fn validate(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: &super::OpenAiToolDispatch,
) -> Result<(), crate::HostError> {
    if plan.endpoint() != config.endpoint_identity() {
        return crate::declarations::unsupported("endpoint");
    }
    if plan.api_kind() != animus_std::ApiKind::OpenAiResponses {
        return crate::declarations::unsupported("provider_family");
    }
    let request = plan.request();
    validate_request(request, dispatch)?;
    validate_roles(request, dispatch.capabilities())?;
    validate_tools(request, dispatch.capabilities(), dispatch.schemas())?;
    validate_stream_channels(request, dispatch.capabilities())?;
    super::prior::validate(dispatch, request)
}

fn validate_request(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    dispatch: &super::OpenAiToolDispatch,
) -> Result<(), crate::HostError> {
    if std::cmp::PartialEq::ne(request.model(), dispatch.capabilities().model_id()) {
        return crate::declarations::unsupported("tool_capability");
    }
    if !request.stream() {
        return crate::declarations::unsupported("streaming");
    }
    if request.tools().is_empty() {
        return crate::declarations::unsupported("tool_capability");
    }
    if std::cmp::PartialEq::ne(request.response_format(), "text") {
        return crate::declarations::unsupported("structured_output");
    }
    if !request.stop_sequences().is_empty() {
        return crate::declarations::unsupported("stop_sequences");
    }
    if dispatch.policy().parallel_tool_calls() {
        return crate::declarations::unsupported("parallel_tool_calls");
    }
    debug_assert!(request.stream());
    debug_assert!(!dispatch.policy().parallel_tool_calls());
    Ok(())
}

fn validate_roles(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    capabilities: &crate::OpenAiToolCapabilityMatrix,
) -> Result<(), crate::HostError> {
    for message in request.messages() {
        let role =
            animus_adapters::CanonicalModelRole::parse(message.role()).map_err(|_error| {
                crate::HostError::UnsupportedPlan {
                    feature: "message_role",
                }
            })?;
        if !capabilities.roles().contains(&role) {
            return crate::declarations::unsupported("message_role");
        }
    }
    Ok(())
}

fn validate_tools(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    capabilities: &crate::OpenAiToolCapabilityMatrix,
    schemas: &[crate::OpenAiToolSchemaBinding],
) -> Result<(), crate::HostError> {
    if request.tools().len() != schemas.len() {
        return crate::declarations::unsupported("tool_schema");
    }
    for tool in request.tools() {
        let capability = crate::OpenAiToolCapability::new(tool.name(), tool.input_schema_label())?;
        if !capabilities.tools().contains(&capability)
            || !schemas
                .iter()
                .any(|binding| binding.label() == tool.input_schema_label())
        {
            return crate::declarations::unsupported("tool_capability");
        }
    }
    Ok(())
}

fn validate_stream_channels(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    capabilities: &crate::OpenAiToolCapabilityMatrix,
) -> Result<(), crate::HostError> {
    let mut required = vec![
        crate::OpenAiToolStreamChannel::Content,
        crate::OpenAiToolStreamChannel::ToolCall,
        crate::OpenAiToolStreamChannel::Usage,
        crate::OpenAiToolStreamChannel::Finish,
    ];
    if request.reasoning_effort() != "none" {
        Vec::push(
            &mut required,
            crate::OpenAiToolStreamChannel::ReasoningSummary,
        );
    }
    if required
        .iter()
        .any(|channel| !capabilities.stream_channels().contains(channel))
    {
        return crate::declarations::unsupported("stream_channel");
    }
    assert!(
        capabilities
            .stream_channels()
            .contains(&crate::OpenAiToolStreamChannel::ToolCall)
    );
    assert!(
        capabilities
            .stream_channels()
            .contains(&crate::OpenAiToolStreamChannel::Finish)
    );
    Ok(())
}
