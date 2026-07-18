//! Exact-cohort ACP JSON-RPC codec with Animus-owned safe mappings.

#![allow(missing_docs)]

use std::borrow::Cow;

use agent_client_protocol::{self as acp, Side as _};
use serde::Deserialize;
use serde_json::value::RawValue;

use crate::core::domain_hash;
use crate::model::{
    ACP_MAX_SAFE_LABEL_BYTES, ACP_PROTOCOL_VERSION, ACP_WORKSPACE_HASH_DOMAIN,
    AcpClientPermissionDecision, AcpClientPermissionRequest, AcpDecodedRequest, AcpFailureClass,
    AcpFeature, AcpHostMessage, AcpPromptPart, AcpProtocolAction, AcpProtocolError,
    AcpProtocolResponse, AcpStopClass, AcpUpdate, AcpUpdateKind,
};

const JSON_RPC_VERSION: &str = "2.0";
const JSON_RPC_SERVER_ERROR_CODE: i64 = -32_000;
const FIELD_FRAME: &str = "frame";
const FIELD_CORRELATION: &str = "correlation";
const FIELD_REQUEST: &str = "request";
const CORRELATION_NUMBER_PREFIX: &str = "rpc.number.";
const CORRELATION_STRING_PREFIX: &str = "rpc.string.";
const PERMISSION_ALLOW_ONCE: &str = "allow_once";
const PERMISSION_ALLOW_ALWAYS: &str = "allow_always";
const PERMISSION_DENY_ONCE: &str = "deny_once";
const PERMISSION_DENY_ALWAYS: &str = "deny_always";
const METHOD_ANIMUS_UPDATE: &str = "_animus/session_update";
const TOOL_STATUS_TITLE_FALLBACK: &str = "tool";
const SAFE_RESOURCE_NAME_HEX_CHARS: usize = 32;

#[derive(Debug, Deserialize)]
struct RawFrame<'a> {
    #[serde(default)]
    id: Option<acp::RequestId>,
    #[serde(default)]
    method: Option<Cow<'a, str>>,
    #[serde(default, borrow)]
    params: Option<&'a RawValue>,
    #[serde(default, borrow)]
    result: Option<&'a RawValue>,
    #[serde(default)]
    error: Option<acp::Error>,
    #[serde(default)]
    jsonrpc: Option<Cow<'a, str>>,
}

/// # Errors
/// Returns a typed error when a frame is malformed, unsupported, or unsafe to retain.
pub fn decode_acp_frame(frame: &[u8]) -> Result<AcpDecodedRequest, AcpProtocolError> {
    let text = core::str::from_utf8(frame)
        .map_err(|_error| codec_error(AcpFailureClass::MalformedFrame, FIELD_FRAME))?;
    let raw: RawFrame<'_> = serde_json::from_str(text)
        .map_err(|_error| codec_error(AcpFailureClass::MalformedFrame, FIELD_FRAME))?;
    if raw.jsonrpc.as_deref() != Some(JSON_RPC_VERSION)
        || raw.result.is_some()
        || raw.error.is_some()
    {
        return Err(codec_error(AcpFailureClass::MalformedFrame, FIELD_FRAME));
    }
    let Some(method) = raw.method.as_deref() else {
        return Err(codec_error(AcpFailureClass::MalformedFrame, FIELD_FRAME));
    };
    match raw.id {
        Some(request_id) => decode_request(request_id, method, raw.params),
        None => decode_notification(method, raw.params),
    }
}

/// # Errors
/// Returns a typed error when the message correlation or JSON payload cannot be encoded safely.
pub fn encode_acp_message(message: &AcpHostMessage) -> Result<Vec<u8>, AcpProtocolError> {
    let value = match message {
        AcpHostMessage::Response {
            correlation_id,
            payload,
        } => serde_json::json!({
            "jsonrpc": JSON_RPC_VERSION,
            "id": request_id_value(correlation_id)?,
            "result": payload,
        }),
        AcpHostMessage::ErrorResponse {
            correlation_id,
            failure,
        } => serde_json::json!({
            "jsonrpc": JSON_RPC_VERSION,
            "id": request_id_value(correlation_id)?,
            "error": {
                "code": JSON_RPC_SERVER_ERROR_CODE,
                "message": failure.as_str(),
            },
        }),
        AcpHostMessage::Notification { method, payload } => serde_json::json!({
            "jsonrpc": JSON_RPC_VERSION,
            "method": method,
            "params": payload,
        }),
        AcpHostMessage::Request {
            correlation_id,
            method,
            payload,
        } => serde_json::json!({
            "jsonrpc": JSON_RPC_VERSION,
            "id": request_id_value(correlation_id)?,
            "method": method,
            "params": payload,
        }),
    };
    serde_json::to_vec(&value)
        .map_err(|_error| codec_error(AcpFailureClass::WriteFailure, FIELD_FRAME))
}

pub(crate) fn host_messages_for_actions(
    actions: &[AcpProtocolAction],
) -> Result<Vec<AcpHostMessage>, AcpProtocolError> {
    let mut messages = Vec::with_capacity(actions.len());
    for action in actions {
        match action {
            AcpProtocolAction::Respond {
                correlation_id,
                response,
            } => messages.push(host_response(correlation_id, response)?),
            AcpProtocolAction::Update(update) => messages.push(host_update(update)?),
            AcpProtocolAction::Terminate { .. } => {}
        }
    }
    Ok(messages)
}

pub(crate) fn permission_request_message(
    request: &AcpClientPermissionRequest,
) -> Result<AcpHostMessage, AcpProtocolError> {
    validate_safe_correlation(request.request_id.as_str())?;
    let options = vec![
        acp::PermissionOption::new(
            PERMISSION_ALLOW_ONCE,
            "Allow once",
            acp::PermissionOptionKind::AllowOnce,
        ),
        acp::PermissionOption::new(
            PERMISSION_ALLOW_ALWAYS,
            "Allow for this request",
            acp::PermissionOptionKind::AllowAlways,
        ),
        acp::PermissionOption::new(
            PERMISSION_DENY_ONCE,
            "Deny once",
            acp::PermissionOptionKind::RejectOnce,
        ),
        acp::PermissionOption::new(
            PERMISSION_DENY_ALWAYS,
            "Deny for this request",
            acp::PermissionOptionKind::RejectAlways,
        ),
    ];
    let tool_update = acp::ToolCallUpdate::new(
        request.tool_correlation_identity.clone(),
        acp::ToolCallUpdateFields::new()
            .title(request.tool_name.clone())
            .status(acp::ToolCallStatus::Pending),
    );
    let payload = serde_json::to_value(acp::RequestPermissionRequest::new(
        request.session_id.clone(),
        tool_update,
        options,
    ))
    .map_err(|_error| codec_error(AcpFailureClass::WriteFailure, FIELD_REQUEST))?;
    Ok(AcpHostMessage::Request {
        correlation_id: request.request_id.clone(),
        method: String::from(acp::CLIENT_METHOD_NAMES.session_request_permission),
        payload,
    })
}

pub(crate) fn decode_permission_response(
    expected_correlation: &str,
    frame: &[u8],
) -> Result<AcpClientPermissionDecision, AcpProtocolError> {
    let text = core::str::from_utf8(frame)
        .map_err(|_error| codec_error(AcpFailureClass::MalformedFrame, FIELD_FRAME))?;
    let raw: RawFrame<'_> = serde_json::from_str(text)
        .map_err(|_error| codec_error(AcpFailureClass::MalformedFrame, FIELD_FRAME))?;
    if raw.jsonrpc.as_deref() != Some(JSON_RPC_VERSION)
        || raw.method.is_some()
        || raw.params.is_some()
        || raw.error.is_some()
    {
        return Err(codec_error(AcpFailureClass::MalformedFrame, FIELD_FRAME));
    }
    let Some(request_id) = raw.id else {
        return Err(codec_error(
            AcpFailureClass::UnknownCorrelation,
            FIELD_CORRELATION,
        ));
    };
    let observed = match &request_id {
        acp::RequestId::Str(value) if value.as_str() == expected_correlation => {
            String::from(expected_correlation)
        }
        _other => normalize_request_id(request_id)?,
    };
    if observed != expected_correlation {
        return Err(codec_error(
            AcpFailureClass::UnknownCorrelation,
            FIELD_CORRELATION,
        ));
    }
    let result = raw
        .result
        .ok_or_else(|| codec_error(AcpFailureClass::MalformedFrame, FIELD_FRAME))?;
    let response: acp::RequestPermissionResponse = serde_json::from_str(result.get())
        .map_err(|_error| codec_error(AcpFailureClass::MalformedFrame, FIELD_FRAME))?;
    permission_decision(response.outcome)
}

fn decode_request(
    request_id: acp::RequestId,
    method: &str,
    params: Option<&RawValue>,
) -> Result<AcpDecodedRequest, AcpProtocolError> {
    let correlation_id = normalize_request_id(request_id)?;
    let request = acp::AgentSide::decode_request(method, params)
        .map_err(|_error| codec_error(AcpFailureClass::MalformedFrame, FIELD_REQUEST))?;
    match request {
        acp::ClientRequest::InitializeRequest(arguments) => Ok(AcpDecodedRequest::Initialize {
            correlation_id,
            protocol_version: arguments.protocol_version.to_string(),
            required_features: required_client_features(&arguments),
        }),
        acp::ClientRequest::NewSessionRequest(arguments) => {
            validate_absolute_workspace(arguments.cwd.as_path())?;
            Ok(AcpDecodedRequest::NewSession {
                correlation_id,
                workspace_identity: workspace_identity(arguments.cwd.as_path()),
                mcp_server_count: bounded_count(arguments.mcp_servers.len())?,
            })
        }
        acp::ClientRequest::LoadSessionRequest(arguments) => {
            validate_absolute_workspace(arguments.cwd.as_path())?;
            Ok(AcpDecodedRequest::LoadSession {
                correlation_id,
                session_reference: arguments.session_id.to_string(),
                workspace_identity: workspace_identity(arguments.cwd.as_path()),
                mcp_server_count: bounded_count(arguments.mcp_servers.len())?,
            })
        }
        acp::ClientRequest::PromptRequest(arguments) => Ok(AcpDecodedRequest::Prompt {
            correlation_id,
            session_id: arguments.session_id.to_string(),
            parts: prompt_parts(arguments.prompt.as_slice())?,
        }),
        _unsupported => Ok(AcpDecodedRequest::Unsupported {
            correlation_id: Some(correlation_id),
            method: safe_method_label(method),
        }),
    }
}

fn decode_notification(
    method: &str,
    params: Option<&RawValue>,
) -> Result<AcpDecodedRequest, AcpProtocolError> {
    let notification = acp::AgentSide::decode_notification(method, params)
        .map_err(|_error| codec_error(AcpFailureClass::MalformedFrame, FIELD_REQUEST))?;
    match notification {
        acp::ClientNotification::CancelNotification(arguments) => Ok(AcpDecodedRequest::Cancel {
            session_id: arguments.session_id.to_string(),
        }),
        _unsupported => Ok(AcpDecodedRequest::Unsupported {
            correlation_id: None,
            method: safe_method_label(method),
        }),
    }
}

fn required_client_features(_request: &acp::InitializeRequest) -> Vec<AcpFeature> {
    Vec::new()
}

fn prompt_parts(blocks: &[acp::ContentBlock]) -> Result<Vec<AcpPromptPart>, AcpProtocolError> {
    let mut parts = Vec::with_capacity(blocks.len());
    for block in blocks {
        match block {
            acp::ContentBlock::Text(text) => parts.push(AcpPromptPart::Text(text.text.clone())),
            acp::ContentBlock::ResourceLink(resource) => {
                let safe_name_hash =
                    domain_hash(ACP_WORKSPACE_HASH_DOMAIN, &[resource.name.as_str()]);
                let resource_identity =
                    domain_hash(ACP_WORKSPACE_HASH_DOMAIN, &[resource.uri.as_str()]);
                let suffix = safe_name_hash
                    .get(..SAFE_RESOURCE_NAME_HEX_CHARS)
                    .ok_or_else(|| codec_error(AcpFailureClass::InvalidPrompt, FIELD_REQUEST))?;
                parts.push(AcpPromptPart::ResourceLink {
                    safe_name: format!("resource.{suffix}"),
                    resource_identity,
                });
            }
            _unsupported => {
                return Err(codec_error(
                    AcpFailureClass::UnsupportedFeature,
                    FIELD_REQUEST,
                ));
            }
        }
    }
    Ok(parts)
}

fn host_response(
    correlation_id: &str,
    response: &AcpProtocolResponse,
) -> Result<AcpHostMessage, AcpProtocolError> {
    let payload = match response {
        AcpProtocolResponse::Initialized {
            protocol_version, ..
        } => {
            if protocol_version != ACP_PROTOCOL_VERSION {
                return Err(codec_error(
                    AcpFailureClass::UnsupportedProtocol,
                    FIELD_REQUEST,
                ));
            }
            serde_json::to_value(
                acp::InitializeResponse::new(acp::ProtocolVersion::V1)
                    .agent_capabilities(acp::AgentCapabilities::new().load_session(true))
                    .agent_info(acp::Implementation::new(
                        "animus",
                        env!("CARGO_PKG_VERSION"),
                    )),
            )
        }
        AcpProtocolResponse::SessionBound { session_id, loaded } => {
            if *loaded {
                serde_json::to_value(acp::LoadSessionResponse::new())
            } else {
                serde_json::to_value(acp::NewSessionResponse::new(session_id.clone()))
            }
        }
        AcpProtocolResponse::PromptCompleted { stop } => {
            serde_json::to_value(acp::PromptResponse::new(stop_reason(*stop)))
        }
        AcpProtocolResponse::Failure { failure } => {
            return Ok(AcpHostMessage::ErrorResponse {
                correlation_id: String::from(correlation_id),
                failure: *failure,
            });
        }
    }
    .map_err(|_error| codec_error(AcpFailureClass::WriteFailure, FIELD_REQUEST))?;
    Ok(AcpHostMessage::Response {
        correlation_id: String::from(correlation_id),
        payload,
    })
}

fn host_update(update: &AcpUpdate) -> Result<AcpHostMessage, AcpProtocolError> {
    if let Some(session_update) = stable_session_update(update)? {
        let notification = acp::SessionNotification::new(update.session_id.clone(), session_update);
        let payload = serde_json::to_value(notification)
            .map_err(|_error| codec_error(AcpFailureClass::WriteFailure, FIELD_REQUEST))?;
        return Ok(AcpHostMessage::Notification {
            method: String::from(acp::CLIENT_METHOD_NAMES.session_update),
            payload,
        });
    }
    let payload = serde_json::json!({
        "sequence": update.sequence,
        "kind": update.kind.as_str(),
        "sessionId": update.session_id,
        "status": update.status,
        "safeText": update.safe_text,
        "correlationIdentity": update.correlation_identity,
    });
    Ok(AcpHostMessage::Notification {
        method: String::from(METHOD_ANIMUS_UPDATE),
        payload,
    })
}

fn stable_session_update(
    update: &AcpUpdate,
) -> Result<Option<acp::SessionUpdate>, AcpProtocolError> {
    let session_update =
        match update.kind {
            AcpUpdateKind::AgentMessage => {
                let text = update.safe_text.clone().ok_or_else(|| {
                    codec_error(AcpFailureClass::ProjectionRejected, FIELD_REQUEST)
                })?;
                acp::SessionUpdate::AgentMessageChunk(acp::ContentChunk::new(
                    acp::ContentBlock::Text(acp::TextContent::new(text)),
                ))
            }
            AcpUpdateKind::ToolStarted => {
                let correlation = required_update_correlation(update)?;
                let title = update.safe_text.clone().ok_or_else(|| {
                    codec_error(AcpFailureClass::ProjectionRejected, FIELD_REQUEST)
                })?;
                acp::SessionUpdate::ToolCall(
                    acp::ToolCall::new(correlation, title).status(acp::ToolCallStatus::InProgress),
                )
            }
            AcpUpdateKind::ToolCompleted => {
                let correlation = required_update_correlation(update)?;
                acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
                    correlation,
                    acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::Completed),
                ))
            }
            AcpUpdateKind::ToolDenied => {
                let correlation = required_update_correlation(update)?;
                acp::SessionUpdate::ToolCallUpdate(acp::ToolCallUpdate::new(
                    correlation,
                    acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::Failed),
                ))
            }
            _metadata => return Ok(None),
        };
    Ok(Some(session_update))
}

fn required_update_correlation(update: &AcpUpdate) -> Result<String, AcpProtocolError> {
    update
        .correlation_identity
        .clone()
        .ok_or_else(|| codec_error(AcpFailureClass::ProjectionRejected, FIELD_CORRELATION))
}

fn permission_decision(
    outcome: acp::RequestPermissionOutcome,
) -> Result<AcpClientPermissionDecision, AcpProtocolError> {
    match outcome {
        acp::RequestPermissionOutcome::Cancelled => Ok(AcpClientPermissionDecision::Cancelled),
        acp::RequestPermissionOutcome::Selected(selected) => {
            match selected.option_id.to_string().as_str() {
                PERMISSION_ALLOW_ONCE => Ok(AcpClientPermissionDecision::AllowOnce),
                PERMISSION_ALLOW_ALWAYS => Ok(AcpClientPermissionDecision::AllowAlways),
                PERMISSION_DENY_ONCE => Ok(AcpClientPermissionDecision::DenyOnce),
                PERMISSION_DENY_ALWAYS => Ok(AcpClientPermissionDecision::DenyAlways),
                _unknown => Err(codec_error(
                    AcpFailureClass::PermissionWidening,
                    FIELD_REQUEST,
                )),
            }
        }
        _unsupported => Err(codec_error(
            AcpFailureClass::PermissionWidening,
            FIELD_REQUEST,
        )),
    }
}

fn normalize_request_id(request_id: acp::RequestId) -> Result<String, AcpProtocolError> {
    let normalized = match request_id {
        acp::RequestId::Number(value) => format!("{CORRELATION_NUMBER_PREFIX}{value}"),
        acp::RequestId::Str(value) => {
            validate_safe_correlation(value.as_str())?;
            format!("{CORRELATION_STRING_PREFIX}{value}")
        }
        acp::RequestId::Null => {
            return Err(codec_error(
                AcpFailureClass::MalformedFrame,
                FIELD_CORRELATION,
            ));
        }
    };
    validate_safe_correlation(normalized.as_str())?;
    Ok(normalized)
}

fn request_id_value(correlation_id: &str) -> Result<serde_json::Value, AcpProtocolError> {
    if let Some(number) = correlation_id.strip_prefix(CORRELATION_NUMBER_PREFIX) {
        let parsed = number
            .parse::<i64>()
            .map_err(|_error| codec_error(AcpFailureClass::MalformedFrame, FIELD_CORRELATION))?;
        return Ok(serde_json::Value::Number(parsed.into()));
    }
    if let Some(value) = correlation_id.strip_prefix(CORRELATION_STRING_PREFIX) {
        validate_safe_correlation(value)?;
        return Ok(serde_json::Value::String(String::from(value)));
    }
    validate_safe_correlation(correlation_id)?;
    Ok(serde_json::Value::String(String::from(correlation_id)))
}

fn validate_safe_correlation(value: &str) -> Result<(), AcpProtocolError> {
    if value.is_empty()
        || value.len() > ACP_MAX_SAFE_LABEL_BYTES
        || value.contains('\0')
        || value.chars().any(char::is_whitespace)
    {
        Err(codec_error(
            AcpFailureClass::MalformedFrame,
            FIELD_CORRELATION,
        ))
    } else {
        Ok(())
    }
}

fn validate_absolute_workspace(path: &std::path::Path) -> Result<(), AcpProtocolError> {
    if !path.is_absolute() {
        return Err(codec_error(AcpFailureClass::MalformedFrame, FIELD_REQUEST));
    }
    if path.to_str().is_none() {
        return Err(codec_error(AcpFailureClass::MalformedFrame, FIELD_REQUEST));
    }
    Ok(())
}

fn workspace_identity(path: &std::path::Path) -> String {
    let text = path.to_str().unwrap_or(TOOL_STATUS_TITLE_FALLBACK);
    domain_hash(ACP_WORKSPACE_HASH_DOMAIN, &[text])
}

fn safe_method_label(method: &str) -> String {
    let identity = domain_hash(ACP_WORKSPACE_HASH_DOMAIN, &[method]);
    format!("method.{identity}")
}

fn bounded_count(count: usize) -> Result<u32, AcpProtocolError> {
    u32::try_from(count)
        .map_err(|_error| codec_error(AcpFailureClass::MalformedFrame, FIELD_REQUEST))
}

const fn stop_reason(stop: AcpStopClass) -> acp::StopReason {
    match stop {
        AcpStopClass::EndTurn => acp::StopReason::EndTurn,
        AcpStopClass::Cancelled => acp::StopReason::Cancelled,
        AcpStopClass::Refused => acp::StopReason::Refusal,
    }
}

const fn codec_error(failure: AcpFailureClass, field: &'static str) -> AcpProtocolError {
    AcpProtocolError { failure, field }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INITIALIZE_FRAME: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":1,"clientCapabilities":{}}}"#;
    const PROMPT_FRAME: &str = r#"{"jsonrpc":"2.0","id":"prompt-1","method":"session/prompt","params":{"sessionId":"session.fixture","prompt":[{"type":"text","text":"bounded"}]}}"#;
    const MALFORMED_FRAME: &str = r#"{"jsonrpc":"2.0","id":1,"method":"initialize""#;
    const NUMERIC_RESPONSE_ID: i64 = 7;

    #[test]
    fn reviewed_sdk_decodes_initialize_and_text_prompt() {
        let initialize = decode_acp_frame(INITIALIZE_FRAME.as_bytes()).expect("initialize frame");
        let prompt = decode_acp_frame(PROMPT_FRAME.as_bytes()).expect("prompt frame");

        assert!(matches!(initialize, AcpDecodedRequest::Initialize { .. }));
        assert!(matches!(prompt, AcpDecodedRequest::Prompt { .. }));
        assert_eq!(initialize.correlation_id(), Some("rpc.number.1"));
        assert_eq!(prompt.correlation_id(), Some("rpc.string.prompt-1"));
    }

    #[test]
    fn malformed_and_unsupported_content_fail_closed() {
        let malformed =
            decode_acp_frame(MALFORMED_FRAME.as_bytes()).expect_err("malformed frame rejected");
        assert_eq!(malformed.failure, AcpFailureClass::MalformedFrame);

        let image = r#"{"jsonrpc":"2.0","id":2,"method":"session/prompt","params":{"sessionId":"session.fixture","prompt":[{"type":"image","data":"raw","mimeType":"image/png"}]}}"#;
        let unsupported = decode_acp_frame(image.as_bytes()).expect_err("image rejected");
        assert_eq!(unsupported.failure, AcpFailureClass::UnsupportedFeature);
    }

    #[test]
    fn initialize_advertises_only_reviewed_stable_capabilities() {
        let message = host_response(
            "rpc.number.1",
            &AcpProtocolResponse::Initialized {
                protocol_version: String::from(ACP_PROTOCOL_VERSION),
                features: AcpFeature::ALL.to_vec(),
            },
        )
        .expect("initialize response");
        let AcpHostMessage::Response { payload, .. } = message else {
            panic!("initialize must produce a response");
        };

        assert_eq!(payload["agentCapabilities"]["loadSession"], true);
        assert_eq!(
            payload["agentCapabilities"]["promptCapabilities"]["image"],
            false
        );
        assert_eq!(
            payload["agentCapabilities"]["promptCapabilities"]["audio"],
            false
        );
        assert_eq!(
            payload["agentCapabilities"]["promptCapabilities"]["embeddedContext"],
            false
        );
    }

    #[test]
    fn stable_tool_updates_omit_raw_input_and_output() {
        let started = host_update(&AcpUpdate {
            sequence: 1,
            kind: AcpUpdateKind::ToolStarted,
            session_id: String::from("session.fixture"),
            safe_text: Some(String::from("fixture_tool")),
            correlation_identity: Some(String::from("tool.fixture")),
            status: String::from("pending"),
        })
        .expect("started update");
        let completed = host_update(&AcpUpdate {
            sequence: 1,
            kind: AcpUpdateKind::ToolCompleted,
            session_id: String::from("session.fixture"),
            safe_text: None,
            correlation_identity: Some(String::from("tool.fixture")),
            status: String::from("completed_output_omitted"),
        })
        .expect("completed update");

        for (message, expected_kind) in [(started, "tool_call"), (completed, "tool_call_update")] {
            let AcpHostMessage::Notification { method, payload } = message else {
                panic!("stable tool update must be a notification");
            };
            assert_eq!(method, acp::CLIENT_METHOD_NAMES.session_update);
            assert_eq!(payload["update"]["sessionUpdate"], expected_kind);
            assert!(payload["update"].get("rawInput").is_none());
            assert!(payload["update"].get("rawOutput").is_none());
        }
    }

    #[test]
    fn response_encoding_preserves_numeric_correlation() {
        let message = AcpHostMessage::Response {
            correlation_id: format!("rpc.number.{NUMERIC_RESPONSE_ID}"),
            payload: serde_json::json!({"ok": true}),
        };
        let encoded = encode_acp_message(&message).expect("encoded response");
        let value: serde_json::Value =
            serde_json::from_slice(encoded.as_slice()).expect("JSON response");

        assert_eq!(value["id"], serde_json::json!(NUMERIC_RESPONSE_ID));
        assert_eq!(value["result"]["ok"], serde_json::json!(true));
    }
}
