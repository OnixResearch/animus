//! Pure chat request projection. The same preparation binds bytes and replay state.

const TEMPERATURE_MILLIS_PER_UNIT: f64 = 1_000.0;
const MAX_REQUEST_BYTES: usize = 2_097_152;

pub(crate) struct Prepared {
    pub(crate) body: Vec<u8>,
    pub(crate) replay: super::history::Replay,
}

/// Builds one admitted Coding Plan request without performing I/O.
///
/// # Errors
/// Returns [`crate::HostError`] for unsupported fields, missing history, or byte bounds.
pub fn build_zai_chat_completions_request_body(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: &crate::tool::OpenAiToolDispatch,
) -> Result<Vec<u8>, crate::HostError> {
    prepare(config, plan, dispatch).map(|prepared| prepared.body)
}

pub(crate) fn prepare(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: &crate::tool::OpenAiToolDispatch,
) -> Result<Prepared, crate::HostError> {
    super::admission::validate(config, plan, dispatch)?;
    let request = plan.request();
    let tools = tools(request, dispatch)?;
    let schemas = serde_json::to_vec(&tools).map_err(|_error| crate::HostError::RequestEncoding)?;
    let schema_identity = blake3::hash(&schemas).to_hex().to_string();
    let replay = match dispatch.continuation_parts() {
        None => super::history::Replay::new(plan, initial_messages(request)?, schema_identity)?,
        Some((binding, result)) => binding
            .chat_replay
            .as_ref()
            .ok_or_else(super::history::denied)?
            .resume(plan, &schema_identity, (binding, result))?,
    };
    let thinking = request.reasoning_effort() != "none";
    let mut body = serde_json::json!({
        "model": request.model(), "messages": replay.messages(), "tools": tools,
        "tool_choice": crate::tool::project_chat_tool_choice(dispatch.policy().choice(), request)?,
        "parallel_tool_calls": false, "stream": true, "max_tokens": request.max_output_tokens(),
        "thinking": {"type": if thinking { "enabled" } else { "disabled" }, "clear_thinking": false},
    });
    if thinking {
        body["reasoning_effort"] =
            serde_json::Value::String(String::from(request.reasoning_effort()));
    }
    if let Some(temperature) = request.temperature_millis() {
        body["temperature"] =
            serde_json::json!(f64::from(temperature) / TEMPERATURE_MILLIS_PER_UNIT);
    }
    let body = serde_json::to_vec(&body).map_err(|_error| crate::HostError::RequestEncoding)?;
    if body.len() > MAX_REQUEST_BYTES {
        return Err(crate::HostError::UnsupportedPlan {
            feature: "chat-request-bound",
        });
    }
    Ok(Prepared { body, replay })
}

fn tools(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    dispatch: &crate::tool::OpenAiToolDispatch,
) -> Result<Vec<serde_json::Value>, crate::HostError> {
    let schemas = crate::tool::schema_map(dispatch.schemas())?;
    request
        .tools()
        .iter()
        .map(|tool| {
            let parameters = schemas.get(tool.input_schema_label()).ok_or(
                crate::HostError::InvalidToolSchema {
                    reason: "chat tool schema binding is missing",
                },
            )?;
            Ok(serde_json::json!({"type": "function", "function": {
            "name": tool.name(), "description": tool.description(), "parameters": parameters}}))
        })
        .collect()
}

fn initial_messages(
    request: &animus_std::OpenAiResponsesDispatchRequest,
) -> Result<Vec<serde_json::Value>, crate::HostError> {
    request
        .messages()
        .iter()
        .map(|message| {
            if message.role() == "tool" || message.tool_call_id().is_some() {
                return Err(crate::HostError::UnsupportedPlan {
                    feature: crate::tool::FEATURE_TOOL_RESULT_SHAPE,
                });
            }
            let content = message
                .content()
                .iter()
                .map(|part| match part {
                    animus_std::OpenAiResponsesContentPart::Text { text } => Ok(text.as_str()),
                    animus_std::OpenAiResponsesContentPart::ImageDescriptor { .. } => {
                        Err(crate::HostError::UnsupportedPlan { feature: "image" })
                    }
                })
                .collect::<Result<Vec<_>, crate::HostError>>()?
                .concat();
            Ok(serde_json::json!({"role": message.role(), "content": content}))
        })
        .collect()
}
