use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{HostError, OpenAiHostConfig};

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
const OUTPUT_TEXT_TYPE: &str = "output_text";
const REFUSAL_TYPE: &str = "refusal";
const MESSAGE_TYPE: &str = "message";
const REASONING_TYPE: &str = "reasoning";
const ASSISTANT_ROLE: &str = "assistant";
const STATUS_COMPLETED: &str = "completed";
const FINISH_REASON_STOP: &str = "stop";
const FINISH_REASON_REFUSAL: &str = "refusal";
const TEXT_SEPARATOR: &str = "\n";
const TEMPERATURE_SCALE_MILLIS: f64 = 1_000.0;
const MALFORMED_JSON_REASON: &str = "OpenAI response JSON is malformed";
const INCOMPLETE_RESPONSE_REASON: &str = "OpenAI response is not complete";
const OUTPUT_SHAPE_REASON: &str = "OpenAI response output shape is unsupported";
const OUTPUT_ROLE_REASON: &str = "OpenAI response message role is not assistant";
const OUTPUT_CONTENT_REASON: &str = "OpenAI response has no supported content";
const OUTPUT_COMBINATION_REASON: &str = "OpenAI response combines incompatible output kinds";
const METADATA_DUPLICATE_REASON: &str = "OpenAI request metadata contains a duplicate key";

#[derive(Serialize)]
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
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    metadata: BTreeMap<String, String>,
}

#[derive(Serialize)]
struct RequestMessage {
    role: String,
    content: Vec<RequestContent>,
}

#[derive(Serialize)]
struct RequestContent {
    #[serde(rename = "type")]
    kind: &'static str,
    text: String,
}

#[derive(Serialize)]
struct RequestReasoning {
    effort: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    status: String,
    output: Vec<ResponseOutput>,
    usage: Option<ResponseUsage>,
}

#[derive(Deserialize)]
struct ResponseOutput {
    #[serde(rename = "type")]
    kind: String,
    role: Option<String>,
    content: Option<Vec<ResponseContent>>,
}

#[derive(Deserialize)]
struct ResponseContent {
    #[serde(rename = "type")]
    kind: String,
    text: Option<String>,
    refusal: Option<String>,
}

#[derive(Clone, Copy, Deserialize)]
struct ResponseUsage {
    input_tokens: u32,
    output_tokens: u32,
    input_tokens_details: Option<ResponseInputTokenDetails>,
}

#[derive(Clone, Copy, Deserialize)]
struct ResponseInputTokenDetails {
    cached_tokens: u32,
}

#[derive(Default)]
struct ResponseProjection {
    text_parts: Vec<String>,
    refusal: Option<String>,
}

/// Builds deterministic JSON bytes for the admitted text-only live cohort.
///
/// # Errors
///
/// Returns [`HostError`] before transport when the plan cannot be preserved exactly.
pub fn build_openai_request_body(
    config: &OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
) -> Result<Vec<u8>, HostError> {
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
    serde_json::to_vec(&body).map_err(|_error| HostError::RequestEncoding)
}

/// Normalizes one bounded successful `OpenAI` Responses body through existing canonical contracts.
///
/// # Errors
///
/// Returns [`HostError`] when the body is oversized, malformed, incomplete, or unsupported.
pub fn normalize_openai_response(
    plan: &animus_std::ProviderDispatchPlan,
    body: &[u8],
    max_response_bytes: u64,
) -> Result<animus_std::ProviderDispatchOutcome, HostError> {
    // r[impl agent_harness_runtime.live_openai_provider_host.response_normalization]
    validate_body_bound(body, max_response_bytes)?;
    let response: ResponseBody =
        serde_json::from_slice(body).map_err(|_error| HostError::MalformedResponse {
            reason: MALFORMED_JSON_REASON,
        })?;
    if response.status != STATUS_COMPLETED {
        return Err(HostError::MalformedResponse {
            reason: INCOMPLETE_RESPONSE_REASON,
        });
    }
    let projection = project_response_output(response.output.as_slice())?;
    let usage = response.usage.map(project_usage);
    let text = joined_text(projection.text_parts.as_slice());
    let finish_reason = if projection.refusal.is_some() {
        FINISH_REASON_REFUSAL
    } else {
        FINISH_REASON_STOP
    };
    let draft = animus_adapters::ProviderResponseFixtureDraft {
        family: animus_adapters::ProviderProjectionFamily::OpenAiResponses.as_str(),
        text: text.as_deref(),
        structured_payload: None,
        tool_call_id: None,
        tool_name: None,
        tool_arguments: None,
        refusal: projection.refusal.as_deref(),
        safety: None,
        malformed_output: None,
        usage,
        finish_reason: Some(finish_reason),
        raw_provider_payload: None,
        hidden_reasoning: None,
    };
    animus_std::normalize_openai_responses_fixture_response(plan, draft)
        .map_err(|source| HostError::CanonicalNormalization { source })
}

fn validate_plan(
    config: &OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
) -> Result<(), HostError> {
    if plan.endpoint() != config.endpoint_identity() {
        return Err(HostError::UnsupportedPlan {
            feature: FEATURE_ENDPOINT,
        });
    }
    if plan.api_kind() != animus_std::ApiKind::OpenAiResponses {
        return Err(HostError::UnsupportedPlan {
            feature: FEATURE_PROVIDER_FAMILY,
        });
    }
    let request = plan.request();
    if request.stream() {
        return Err(HostError::UnsupportedPlan {
            feature: FEATURE_STREAMING,
        });
    }
    if !request.tools().is_empty() {
        return Err(HostError::UnsupportedPlan {
            feature: FEATURE_TOOLS,
        });
    }
    if request.response_format() != RESPONSE_FORMAT_TEXT {
        return Err(HostError::UnsupportedPlan {
            feature: FEATURE_STRUCTURED_OUTPUT,
        });
    }
    if !request.stop_sequences().is_empty() {
        return Err(HostError::UnsupportedPlan {
            feature: FEATURE_STOP_SEQUENCES,
        });
    }
    Ok(())
}

fn project_messages(
    request: &animus_std::OpenAiResponsesDispatchRequest,
) -> Result<Vec<RequestMessage>, HostError> {
    request.messages().iter().map(project_message).collect()
}

fn project_message(
    message: &animus_std::OpenAiResponsesMessage,
) -> Result<RequestMessage, HostError> {
    if !is_supported_role(message.role()) {
        return Err(HostError::UnsupportedPlan {
            feature: FEATURE_MESSAGE_ROLE,
        });
    }
    if message.tool_call_id().is_some() {
        return Err(HostError::UnsupportedPlan {
            feature: FEATURE_TOOL_CORRELATION,
        });
    }
    let content = message
        .content()
        .iter()
        .map(project_content)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(RequestMessage {
        role: String::from(message.role()),
        content,
    })
}

fn project_content(
    content: &animus_std::OpenAiResponsesContentPart,
) -> Result<RequestContent, HostError> {
    match content {
        animus_std::OpenAiResponsesContentPart::Text { text } => Ok(RequestContent {
            kind: INPUT_TEXT_TYPE,
            text: text.clone(),
        }),
        animus_std::OpenAiResponsesContentPart::ImageDescriptor { .. } => {
            Err(HostError::UnsupportedPlan {
                feature: FEATURE_IMAGE,
            })
        }
    }
}

fn project_reasoning(config: &OpenAiHostConfig, effort: &str) -> Option<RequestReasoning> {
    debug_assert!(effort == REASONING_NONE || !effort.is_empty());
    config
        .projected_reasoning_effort(effort)
        .map(|projected| RequestReasoning {
            effort: String::from(projected),
        })
}

fn project_metadata(
    metadata: &[animus_std::OpenAiResponsesMetadata],
) -> Result<BTreeMap<String, String>, HostError> {
    let mut output = BTreeMap::new();
    for entry in metadata {
        if output
            .insert(String::from(entry.key()), String::from(entry.value()))
            .is_some()
        {
            return Err(HostError::MalformedResponse {
                reason: METADATA_DUPLICATE_REASON,
            });
        }
    }
    Ok(output)
}

fn project_response_output(output: &[ResponseOutput]) -> Result<ResponseProjection, HostError> {
    let mut projection = ResponseProjection::default();
    for item in output {
        match item.kind.as_str() {
            MESSAGE_TYPE => project_message_output(item, &mut projection)?,
            REASONING_TYPE => {}
            _other => {
                return Err(HostError::MalformedResponse {
                    reason: OUTPUT_SHAPE_REASON,
                });
            }
        }
    }
    validate_response_projection(&projection)?;
    Ok(projection)
}

fn project_message_output(
    output: &ResponseOutput,
    projection: &mut ResponseProjection,
) -> Result<(), HostError> {
    if output.role.as_deref() != Some(ASSISTANT_ROLE) {
        return Err(HostError::MalformedResponse {
            reason: OUTPUT_ROLE_REASON,
        });
    }
    let content = output
        .content
        .as_deref()
        .ok_or(HostError::MalformedResponse {
            reason: OUTPUT_CONTENT_REASON,
        })?;
    for item in content {
        match item.kind.as_str() {
            OUTPUT_TEXT_TYPE => push_output_text(item, projection)?,
            REFUSAL_TYPE => set_refusal(item, projection)?,
            _other => {
                return Err(HostError::MalformedResponse {
                    reason: OUTPUT_SHAPE_REASON,
                });
            }
        }
    }
    Ok(())
}

fn push_output_text(
    content: &ResponseContent,
    projection: &mut ResponseProjection,
) -> Result<(), HostError> {
    let text = content.text.as_ref().ok_or(HostError::MalformedResponse {
        reason: OUTPUT_CONTENT_REASON,
    })?;
    projection.text_parts.push(text.clone());
    Ok(())
}

fn set_refusal(
    content: &ResponseContent,
    projection: &mut ResponseProjection,
) -> Result<(), HostError> {
    if projection.refusal.is_some() {
        return Err(HostError::MalformedResponse {
            reason: OUTPUT_COMBINATION_REASON,
        });
    }
    let refusal = content
        .refusal
        .as_ref()
        .ok_or(HostError::MalformedResponse {
            reason: OUTPUT_CONTENT_REASON,
        })?;
    projection.refusal = Some(refusal.clone());
    Ok(())
}

fn validate_response_projection(projection: &ResponseProjection) -> Result<(), HostError> {
    let has_text = !projection.text_parts.is_empty();
    let has_refusal = projection.refusal.is_some();
    if has_text == has_refusal {
        return Err(HostError::MalformedResponse {
            reason: if has_text {
                OUTPUT_COMBINATION_REASON
            } else {
                OUTPUT_CONTENT_REASON
            },
        });
    }
    Ok(())
}

fn validate_body_bound(body: &[u8], limit: u64) -> Result<(), HostError> {
    let body_len =
        u64::try_from(body.len()).map_err(|_error| HostError::ResponseTooLarge { limit })?;
    if body_len > limit {
        return Err(HostError::ResponseTooLarge { limit });
    }
    Ok(())
}

fn project_usage(usage: ResponseUsage) -> animus_adapters::CanonicalModelUsageDraft {
    animus_adapters::CanonicalModelUsageDraft {
        input: usage.input_tokens,
        output: usage.output_tokens,
        cached_input: usage
            .input_tokens_details
            .map(|details| details.cached_tokens),
    }
}

fn joined_text(parts: &[String]) -> Option<String> {
    if parts.is_empty() {
        None
    } else {
        Some(parts.join(TEXT_SEPARATOR))
    }
}

fn is_supported_role(role: &str) -> bool {
    matches!(role, "system" | "developer" | "user" | "assistant")
}
