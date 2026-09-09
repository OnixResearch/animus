//! Closed Coding Plan cohort admission before byte encoding or transport.

pub(super) fn validate(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: &crate::tool::OpenAiToolDispatch,
) -> Result<(), crate::HostError> {
    if config.endpoint_identity() != crate::ZAI_CODING_PLAN_COMPLETIONS_ENDPOINT_IDENTITY
        || plan.endpoint() != config.endpoint_identity()
    {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_ENDPOINT,
        });
    }
    if plan.api_kind() != animus_std::ApiKind::OpenAiChat
        || dispatch
            .continuation_parts()
            .is_some_and(|(binding, _result)| binding.api_kind != animus_std::ApiKind::OpenAiChat)
    {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_PROVIDER_FAMILY,
        });
    }
    let request = plan.request();
    if request.model() != dispatch.capabilities().model_id() || request.tools().is_empty() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_TOOL_CAPABILITY,
        });
    }
    if !request.stream() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_STREAMING,
        });
    }
    if request.response_format() != crate::tool::RESPONSE_FORMAT_TEXT {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_STRUCTURED_OUTPUT,
        });
    }
    if !request.stop_sequences().is_empty() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_STOP_SEQUENCES,
        });
    }
    if dispatch.policy().parallel_tool_calls() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: crate::tool::FEATURE_PARALLEL_TOOL_CALLS,
        });
    }
    if !matches!(request.reasoning_effort(), "none" | "low" | "high" | "max") {
        return Err(crate::HostError::UnsupportedPlan {
            feature: "chat-reasoning-effort",
        });
    }
    if request.max_output_tokens().is_none_or(|limit| limit == 0) || !request.metadata().is_empty()
    {
        return Err(crate::HostError::UnsupportedPlan {
            feature: "chat-output-or-metadata",
        });
    }
    if request
        .messages()
        .iter()
        .any(|message| !matches!(message.role(), "system" | "user" | "assistant" | "tool"))
    {
        return Err(crate::HostError::UnsupportedPlan {
            feature: "chat-role",
        });
    }
    crate::tool::validate_roles(request, dispatch.capabilities())?;
    crate::tool::validate_tools(request, dispatch.capabilities(), dispatch.schemas())?;
    crate::tool::validate_stream_channels(request, dispatch.capabilities())?;
    crate::tool::validate_continuation(dispatch, request)
}
