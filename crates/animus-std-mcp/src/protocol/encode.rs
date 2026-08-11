use serde_json::{Value, json};

use super::{JSON_RPC_VERSION, MCP_PROTOCOL_VERSION, ProtocolError};

const CLIENT_NAME: &str = "animus";
const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");
const CANCEL_REASON: &str = "cancelled-by-current-host-policy";

pub(crate) fn encode_initialize(max_frame_bytes: usize) -> Result<Vec<u8>, ProtocolError> {
    encode(
        &json!({
            "jsonrpc": JSON_RPC_VERSION,
            "id": super::INITIALIZE_RESPONSE_ID,
            "method": "initialize",
            "params": {
                "protocolVersion": MCP_PROTOCOL_VERSION,
                "capabilities": {},
                "clientInfo": {
                    "name": CLIENT_NAME,
                    "version": CLIENT_VERSION,
                },
            },
        }),
        max_frame_bytes,
    )
}

pub(crate) fn encode_initialized(max_frame_bytes: usize) -> Result<Vec<u8>, ProtocolError> {
    encode(
        &json!({
            "jsonrpc": JSON_RPC_VERSION,
            "method": "notifications/initialized",
        }),
        max_frame_bytes,
    )
}

pub(crate) fn encode_list_tools(max_frame_bytes: usize) -> Result<Vec<u8>, ProtocolError> {
    encode(
        &json!({
            "jsonrpc": JSON_RPC_VERSION,
            "id": super::LIST_TOOLS_RESPONSE_ID,
            "method": "tools/list",
            "params": {},
        }),
        max_frame_bytes,
    )
}

pub(crate) fn encode_call(
    request_id: u64,
    tool_name: &str,
    arguments: &Value,
    max_frame_bytes: usize,
) -> Result<Vec<u8>, ProtocolError> {
    encode(
        &json!({
            "jsonrpc": JSON_RPC_VERSION,
            "id": request_id,
            "method": "tools/call",
            "params": {
                "name": tool_name,
                "arguments": arguments,
            },
        }),
        max_frame_bytes,
    )
}

pub(crate) fn encode_cancel(
    request_id: u64,
    max_frame_bytes: usize,
) -> Result<Vec<u8>, ProtocolError> {
    encode(
        &json!({
            "jsonrpc": JSON_RPC_VERSION,
            "method": "notifications/cancelled",
            "params": {
                "requestId": request_id,
                "reason": CANCEL_REASON,
            },
        }),
        max_frame_bytes,
    )
}

fn encode(value: &Value, max_frame_bytes: usize) -> Result<Vec<u8>, ProtocolError> {
    let bytes = serde_json::to_vec(value).map_err(|_| ProtocolError::MalformedFrame)?;
    if bytes.is_empty() || bytes.len() > max_frame_bytes || bytes.contains(&b'\n') {
        return Err(ProtocolError::BudgetExceeded);
    }
    Ok(bytes)
}
