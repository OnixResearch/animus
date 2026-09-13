const OUTPUT_TEXT_TYPE: &str = "output_text";
const REFUSAL_TYPE: &str = "refusal";
const MESSAGE_TYPE: &str = "message";
const REASONING_TYPE: &str = "reasoning";
const ASSISTANT_ROLE: &str = "assistant";
const STATUS_COMPLETED: &str = "completed";
const FINISH_REASON_STOP: &str = "stop";
const FINISH_REASON_REFUSAL: &str = "refusal";
const TEXT_SEPARATOR: &str = "\n";
const MALFORMED_JSON_REASON: &str = "OpenAI response JSON is malformed";
const INCOMPLETE_RESPONSE_REASON: &str = "OpenAI response is not complete";
const OUTPUT_SHAPE_REASON: &str = "OpenAI response output shape is unsupported";
const OUTPUT_ROLE_REASON: &str = "OpenAI response message role is not assistant";
const OUTPUT_CONTENT_REASON: &str = "OpenAI response has no supported content";
const OUTPUT_COMBINATION_REASON: &str = "OpenAI response combines incompatible output kinds";

#[derive(serde::Deserialize)]
struct ResponseBody {
    status: String,
    output: Vec<ResponseOutput>,
    usage: Option<ResponseUsage>,
}

#[derive(serde::Deserialize)]
struct ResponseOutput {
    #[serde(rename = "type")]
    kind: String,
    role: Option<String>,
    content: Option<Vec<ResponseContent>>,
}

#[derive(serde::Deserialize)]
struct ResponseContent {
    #[serde(rename = "type")]
    kind: String,
    text: Option<String>,
    refusal: Option<String>,
}

#[derive(Clone, Copy, serde::Deserialize)]
struct ResponseUsage {
    input_tokens: u32,
    output_tokens: u32,
    input_tokens_details: Option<ResponseInputTokenDetails>,
}

#[derive(Clone, Copy, serde::Deserialize)]
struct ResponseInputTokenDetails {
    cached_tokens: u32,
}

#[derive(Default)]
struct ResponseProjection {
    text_parts: Vec<String>,
    refusal: Option<String>,
}

/// Normalizes one bounded successful `OpenAI` Responses body through existing canonical contracts.
///
/// # Errors
///
/// Returns [`crate::HostError`] when the body is oversized, malformed, incomplete, or unsupported.
///
/// # Panics
///
/// Panics if an internal accepted-response invariant fails after validation.
pub fn normalize_openai_response(
    plan: &animus_std::ProviderDispatchPlan,
    body: &[u8],
    max_response_bytes: u64,
) -> Result<animus_std::ProviderDispatchOutcome, crate::HostError> {
    // r[impl agent_harness_runtime.live_openai_provider_host.response_normalization]
    validate_body_bound(body, max_response_bytes)?;
    let response: ResponseBody =
        serde_json::from_slice(body).map_err(|_error| crate::HostError::MalformedResponse {
            reason: MALFORMED_JSON_REASON,
        })?;
    if response.status != STATUS_COMPLETED {
        return Err(crate::HostError::MalformedResponse {
            reason: INCOMPLETE_RESPONSE_REASON,
        });
    }
    let projection = project_response_output(response.output.as_slice())?;
    assert_eq!(response.status, STATUS_COMPLETED);
    assert_ne!(
        projection.text_parts.is_empty(),
        projection.refusal.is_none()
    );
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
        .map_err(|source| crate::HostError::CanonicalNormalization { source })
}

fn project_response_output(
    output: &[ResponseOutput],
) -> Result<ResponseProjection, crate::HostError> {
    let mut projection = ResponseProjection::default();
    for item in output {
        match item.kind.as_str() {
            MESSAGE_TYPE => project_message_output(item, &mut projection)?,
            REASONING_TYPE => {}
            _other => {
                return Err(crate::HostError::MalformedResponse {
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
) -> Result<(), crate::HostError> {
    if output.role.as_deref() != Some(ASSISTANT_ROLE) {
        return Err(crate::HostError::MalformedResponse {
            reason: OUTPUT_ROLE_REASON,
        });
    }
    let content = output
        .content
        .as_deref()
        .ok_or(crate::HostError::MalformedResponse {
            reason: OUTPUT_CONTENT_REASON,
        })?;
    assert_eq!(output.role.as_deref(), Some(ASSISTANT_ROLE));
    assert!(output.content.is_some());
    for item in content {
        match item.kind.as_str() {
            OUTPUT_TEXT_TYPE => push_output_text(item, projection)?,
            REFUSAL_TYPE => set_refusal(item, projection)?,
            _other => {
                return Err(crate::HostError::MalformedResponse {
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
) -> Result<(), crate::HostError> {
    let text = content
        .text
        .as_ref()
        .ok_or(crate::HostError::MalformedResponse {
            reason: OUTPUT_CONTENT_REASON,
        })?;
    projection.text_parts.push(text.clone());
    Ok(())
}

fn set_refusal(
    content: &ResponseContent,
    projection: &mut ResponseProjection,
) -> Result<(), crate::HostError> {
    if projection.refusal.is_some() {
        return Err(crate::HostError::MalformedResponse {
            reason: OUTPUT_COMBINATION_REASON,
        });
    }
    let refusal = content
        .refusal
        .as_ref()
        .ok_or(crate::HostError::MalformedResponse {
            reason: OUTPUT_CONTENT_REASON,
        })?;
    projection.refusal = Some(refusal.clone());
    Ok(())
}

fn validate_response_projection(projection: &ResponseProjection) -> Result<(), crate::HostError> {
    let has_text = !Vec::is_empty(&projection.text_parts);
    let has_refusal = projection.refusal.is_some();
    if has_text == has_refusal {
        return Err(crate::HostError::MalformedResponse {
            reason: if has_text {
                OUTPUT_COMBINATION_REASON
            } else {
                OUTPUT_CONTENT_REASON
            },
        });
    }
    Ok(())
}

fn validate_body_bound(body: &[u8], limit: u64) -> Result<(), crate::HostError> {
    let body_len =
        u64::try_from(body.len()).map_err(|_error| crate::HostError::ResponseTooLarge { limit })?;
    if body_len > limit {
        return Err(crate::HostError::ResponseTooLarge { limit });
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
