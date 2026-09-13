mod input;

const TEMPERATURE_MILLI_SCALE: f64 = 1_000.0;

#[derive(serde::Serialize)]
struct ToolRequestBody {
    model: String,
    input: Vec<serde_json::Value>,
    tools: Vec<WireTool>,
    tool_choice: serde_json::Value,
    parallel_tool_calls: bool,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<WireReasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    metadata: std::collections::BTreeMap<String, String>,
}

#[derive(serde::Serialize)]
struct WireTool {
    #[serde(rename = "type")]
    kind: &'static str,
    name: String,
    description: String,
    parameters: serde_json::Value,
    strict: bool,
}

#[derive(serde::Serialize)]
struct WireReasoning {
    effort: String,
    summary: &'static str,
}

/// Builds deterministic JSON for the reviewed streaming tool cohort.
///
/// # Errors
/// Returns [`crate::HostError`] before transport for unsupported, lossy, stale, or mismatched input.
pub fn build_tool_capable_openai_request_body(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: &super::OpenAiToolDispatch,
) -> Result<Vec<u8>, crate::HostError> {
    // r[impl agent_harness_runtime.tool_capable_openai_host.capabilities]
    // r[impl agent_harness_runtime.tool_capable_openai_host.projection]
    super::admission::plan::validate(config, plan, dispatch)?;
    let request = plan.request();
    debug_assert!(request.stream());
    debug_assert!(!dispatch.policy().parallel_tool_calls());
    let schema_map = super::shapes::schema_map(dispatch.schemas())?;
    let tools = request
        .tools()
        .iter()
        .map(|tool| project_tool(tool, &schema_map))
        .collect::<Result<Vec<_>, _>>()?;
    let (input, previous_response_id) = input::project(request, dispatch)?;
    let body = ToolRequestBody {
        model: String::from(request.model()),
        input,
        tools,
        tool_choice: dispatch.policy().choice().project(request)?,
        parallel_tool_calls: dispatch.policy().parallel_tool_calls(),
        stream: true,
        previous_response_id,
        reasoning: project_reasoning(config, request.reasoning_effort()),
        max_output_tokens: request.max_output_tokens(),
        temperature: request
            .temperature_millis()
            .map(|value| f64::from(value) / TEMPERATURE_MILLI_SCALE),
        metadata: project_metadata(request.metadata())?,
    };
    serde_json::to_vec(&body).map_err(|_error| crate::HostError::RequestEncoding)
}

fn project_tool(
    tool: &animus_std::OpenAiResponsesTool,
    schemas: &std::collections::BTreeMap<&str, &serde_json::Value>,
) -> Result<WireTool, crate::HostError> {
    let parameters =
        schemas
            .get(tool.input_schema_label())
            .ok_or(crate::HostError::InvalidToolSchema {
                reason: super::SCHEMA_SHAPE_REASON,
            })?;
    Ok(WireTool {
        kind: "function",
        name: String::from(tool.name()),
        description: String::from(tool.description()),
        parameters: (*parameters).clone(),
        strict: true,
    })
}

fn project_reasoning(config: &crate::OpenAiHostConfig, effort: &str) -> Option<WireReasoning> {
    debug_assert!(effort == "none" || !effort.is_empty());
    config
        .projected_reasoning_effort(effort)
        .map(|projected| WireReasoning {
            effort: String::from(projected),
            summary: "auto",
        })
}

fn project_metadata(
    metadata: &[animus_std::OpenAiResponsesMetadata],
) -> Result<std::collections::BTreeMap<String, String>, crate::HostError> {
    let mut output = std::collections::BTreeMap::new();
    for entry in metadata {
        if output
            .insert(String::from(entry.key()), String::from(entry.value()))
            .is_some()
        {
            return Err(crate::HostError::InvalidToolCohort {
                reason: super::DUPLICATE_CAPABILITY_REASON,
            });
        }
    }
    Ok(output)
}
