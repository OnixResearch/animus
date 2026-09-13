const FEATURE_ENDPOINT: &str = "endpoint";
const FEATURE_PROVIDER_FAMILY: &str = "provider_family";
const FEATURE_STREAMING: &str = "streaming";
const FEATURE_IMAGE: &str = "image";
const FEATURE_TOOLS: &str = "tools";
const FEATURE_STRUCTURED_OUTPUT: &str = "structured_output";
const FEATURE_STOP_SEQUENCES: &str = "stop_sequences";
const FEATURE_MESSAGE_ROLE: &str = "message_role";
const FEATURE_TOOL_CORRELATION: &str = "tool_correlation";
const REASONING_NONE: &str = "none";
const RESPONSE_FORMAT_TEXT: &str = "text";
const INPUT_TEXT_TYPE: &str = "input_text";
const TEMPERATURE_SCALE_MILLIS: f64 = 1_000.0;
const METADATA_DUPLICATE_REASON: &str = "OpenAI request metadata contains a duplicate key";

#[derive(serde::Serialize)]
struct RequestBody {
    model: String,
    input: Vec<RequestMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<RequestReasoning>,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "std::collections::BTreeMap::is_empty")]
    metadata: std::collections::BTreeMap<String, String>,
}

#[derive(serde::Serialize)]
struct RequestMessage {
    role: String,
    content: Vec<RequestContent>,
}

#[derive(serde::Serialize)]
struct RequestContent {
    #[serde(rename = "type")]
    kind: &'static str,
    text: String,
}

#[derive(serde::Serialize)]
struct RequestReasoning {
    effort: String,
}

/// Builds deterministic JSON bytes for the admitted text-only live cohort.
///
/// # Errors
///
/// Returns [`crate::HostError`] before transport when the plan cannot be preserved exactly.
pub fn build_openai_request_body(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
) -> Result<Vec<u8>, crate::HostError> {
    // r[impl agent_harness_runtime.live_openai_provider_host.request_projection]
    validate_plan(config, plan)?;
    let request = plan.request();
    let body = RequestBody {
        model: String::from(request.model()),
        input: project_messages(request)?,
        reasoning: project_reasoning(config, request.reasoning_effort()),
        stream: false,
        max_output_tokens: request.max_output_tokens(),
        temperature: request
            .temperature_millis()
            .map(|value| f64::from(value) / TEMPERATURE_SCALE_MILLIS),
        metadata: project_metadata(request.metadata())?,
    };
    serde_json::to_vec(&body).map_err(|_error| crate::HostError::RequestEncoding)
}

fn validate_plan(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
) -> Result<(), crate::HostError> {
    if std::cmp::PartialEq::ne(plan.endpoint(), config.endpoint_identity()) {
        return Err(crate::HostError::UnsupportedPlan {
            feature: FEATURE_ENDPOINT,
        });
    }
    if std::cmp::PartialEq::ne(&plan.api_kind(), &animus_std::ApiKind::OpenAiResponses) {
        return Err(crate::HostError::UnsupportedPlan {
            feature: FEATURE_PROVIDER_FAMILY,
        });
    }
    let request = plan.request();
    if request.stream() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: FEATURE_STREAMING,
        });
    }
    if !request.tools().is_empty() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: FEATURE_TOOLS,
        });
    }
    if std::cmp::PartialEq::ne(request.response_format(), RESPONSE_FORMAT_TEXT) {
        return Err(crate::HostError::UnsupportedPlan {
            feature: FEATURE_STRUCTURED_OUTPUT,
        });
    }
    if !request.stop_sequences().is_empty() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: FEATURE_STOP_SEQUENCES,
        });
    }
    assert_eq!(plan.endpoint(), config.endpoint_identity());
    assert!(!request.stream());
    Ok(())
}

fn project_messages(
    request: &animus_std::OpenAiResponsesDispatchRequest,
) -> Result<Vec<RequestMessage>, crate::HostError> {
    request.messages().iter().map(project_message).collect()
}

fn project_message(
    message: &animus_std::OpenAiResponsesMessage,
) -> Result<RequestMessage, crate::HostError> {
    if !is_supported_role(message.role()) {
        return Err(crate::HostError::UnsupportedPlan {
            feature: FEATURE_MESSAGE_ROLE,
        });
    }
    if message.tool_call_id().is_some() {
        return Err(crate::HostError::UnsupportedPlan {
            feature: FEATURE_TOOL_CORRELATION,
        });
    }
    let content = message
        .content()
        .iter()
        .map(project_content)
        .collect::<Result<Vec<_>, _>>()?;
    assert_eq!(content.len(), message.content().len());
    assert!(message.tool_call_id().is_none());
    Ok(RequestMessage {
        role: String::from(message.role()),
        content,
    })
}

fn project_content(
    content: &animus_std::OpenAiResponsesContentPart,
) -> Result<RequestContent, crate::HostError> {
    match content {
        animus_std::OpenAiResponsesContentPart::Text { text } => Ok(RequestContent {
            kind: INPUT_TEXT_TYPE,
            text: String::clone(text),
        }),
        animus_std::OpenAiResponsesContentPart::ImageDescriptor { .. } => {
            Err(crate::HostError::UnsupportedPlan {
                feature: FEATURE_IMAGE,
            })
        }
    }
}

fn project_reasoning(config: &crate::OpenAiHostConfig, effort: &str) -> Option<RequestReasoning> {
    debug_assert!(effort == REASONING_NONE || !effort.is_empty());
    config
        .projected_reasoning_effort(effort)
        .map(|projected| RequestReasoning {
            effort: String::from(projected),
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
            return Err(crate::HostError::MalformedResponse {
                reason: METADATA_DUPLICATE_REASON,
            });
        }
    }
    Ok(output)
}

fn is_supported_role(role: &str) -> bool {
    matches!(role, "system" | "developer" | "user" | "assistant")
}
