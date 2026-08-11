use serde_json::Value;

use super::{
    CallResponse, InitializeResponse, JSON_RPC_VERSION, MCP_PROTOCOL_VERSION, ProtocolError,
};
use crate::McpLimits;
use crate::config::framed_hash;

const SERVER_INFO_DOMAIN: &str = "animus.mcp-client-host.server-info.v1";
const CALL_RESULT_DOMAIN: &str = "animus.mcp-client-host.call-result.v1";
const MAX_CONTENT_ITEMS: usize = 64;

pub(crate) fn parse_initialize_response(
    bytes: &[u8],
    expected_id: u64,
    limits: McpLimits,
) -> Result<InitializeResponse, ProtocolError> {
    let value = parse_envelope(bytes, expected_id, limits.max_frame_bytes)?;
    let result = value
        .get("result")
        .and_then(Value::as_object)
        .ok_or(ProtocolError::MalformedFrame)?;
    if result.get("protocolVersion").and_then(Value::as_str) != Some(MCP_PROTOCOL_VERSION) {
        return Err(ProtocolError::ProtocolDrift);
    }
    let capabilities = result
        .get("capabilities")
        .and_then(Value::as_object)
        .ok_or(ProtocolError::MalformedFrame)?;
    let tools = capabilities
        .get("tools")
        .and_then(Value::as_object)
        .ok_or(ProtocolError::ProtocolDrift)?;
    if tools
        .get("listChanged")
        .is_some_and(|value| !value.is_boolean())
    {
        return Err(ProtocolError::MalformedFrame);
    }
    let server_info = result
        .get("serverInfo")
        .and_then(Value::as_object)
        .ok_or(ProtocolError::MalformedFrame)?;
    if !server_info.get("name").is_some_and(Value::is_string)
        || !server_info.get("version").is_some_and(Value::is_string)
    {
        return Err(ProtocolError::MalformedFrame);
    }
    let server_info_bytes =
        serde_json::to_vec(server_info).map_err(|_| ProtocolError::MalformedFrame)?;
    let server_identity = framed_hash(SERVER_INFO_DOMAIN, server_info_bytes.as_slice());
    let instructions = match result.get("instructions") {
        Some(value) => Some(String::from(
            value.as_str().ok_or(ProtocolError::MalformedFrame)?,
        )),
        None => None,
    };
    if instructions.as_ref().is_some_and(|text| {
        text.is_empty() || text.len() > limits.max_instruction_bytes || text.contains('\0')
    }) {
        return Err(ProtocolError::BudgetExceeded);
    }
    Ok(InitializeResponse {
        server_identity,
        instructions,
    })
}

pub(crate) fn parse_call_response(
    bytes: &[u8],
    expected_id: u64,
    limits: McpLimits,
) -> Result<CallResponse, ProtocolError> {
    let value = parse_envelope(bytes, expected_id, limits.max_frame_bytes)?;
    let result = value
        .get("result")
        .cloned()
        .ok_or(ProtocolError::MalformedFrame)?;
    let is_error = validate_call_result(&result)?;
    let canonical = serde_json::to_vec(&result).map_err(|_| ProtocolError::MalformedFrame)?;
    if canonical.len() > limits.max_result_bytes {
        return Err(ProtocolError::BudgetExceeded);
    }
    let identity = framed_hash(CALL_RESULT_DOMAIN, canonical.as_slice());
    Ok(CallResponse {
        value: result,
        identity,
        is_error,
    })
}

fn validate_call_result(result: &Value) -> Result<bool, ProtocolError> {
    let object = result.as_object().ok_or(ProtocolError::MalformedFrame)?;
    let content = object
        .get("content")
        .and_then(Value::as_array)
        .ok_or(ProtocolError::MalformedFrame)?;
    if content.len() > MAX_CONTENT_ITEMS {
        return Err(ProtocolError::BudgetExceeded);
    }
    for item in content {
        validate_content_item(item)?;
    }
    if object
        .get("structuredContent")
        .is_some_and(|value| !value.is_object())
        || object.get("_meta").is_some_and(|value| !value.is_object())
    {
        return Err(ProtocolError::MalformedFrame);
    }
    match object.get("isError") {
        Some(value) => value.as_bool().ok_or(ProtocolError::MalformedFrame),
        None => Ok(false),
    }
}

fn validate_content_item(item: &Value) -> Result<(), ProtocolError> {
    let object = item.as_object().ok_or(ProtocolError::MalformedFrame)?;
    let item_type = object
        .get("type")
        .and_then(Value::as_str)
        .ok_or(ProtocolError::MalformedFrame)?;
    let valid = match item_type {
        "text" => object.get("text").is_some_and(Value::is_string),
        "image" | "audio" => {
            object.get("data").is_some_and(Value::is_string)
                && object.get("mimeType").is_some_and(Value::is_string)
        }
        "resource_link" => {
            object.get("name").is_some_and(Value::is_string)
                && object.get("uri").is_some_and(Value::is_string)
        }
        "resource" => object.get("resource").is_some_and(Value::is_object),
        _ => false,
    };
    if !valid {
        return Err(ProtocolError::MalformedFrame);
    }
    Ok(())
}

fn parse_envelope(
    bytes: &[u8],
    expected_id: u64,
    max_frame_bytes: usize,
) -> Result<Value, ProtocolError> {
    if bytes.is_empty() || bytes.len() > max_frame_bytes {
        return Err(ProtocolError::BudgetExceeded);
    }
    let value =
        serde_json::from_slice::<Value>(bytes).map_err(|_| ProtocolError::MalformedFrame)?;
    let object = value.as_object().ok_or(ProtocolError::MalformedFrame)?;
    if object.get("jsonrpc").and_then(Value::as_str) != Some(JSON_RPC_VERSION) {
        return Err(ProtocolError::MalformedFrame);
    }
    if object.get("id").and_then(Value::as_u64) != Some(expected_id) {
        return Err(ProtocolError::CorrelationDrift);
    }
    if object.contains_key("error") {
        return Err(ProtocolError::ServerError);
    }
    Ok(value)
}
