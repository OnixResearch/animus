mod data;
mod decode;
mod encode;

pub(crate) use data::{CallResponse, InitializeResponse, ProtocolError};
pub(crate) use decode::{parse_call_response, parse_initialize_response};
pub(crate) use encode::{
    encode_call, encode_cancel, encode_initialize, encode_initialized, encode_list_tools,
};

/// Exact reviewed MCP protocol cohort.
pub const MCP_PROTOCOL_VERSION: &str = "2025-06-18";
pub(crate) const JSON_RPC_VERSION: &str = "2.0";
pub(crate) const INITIALIZE_RESPONSE_ID: u64 = 1;
pub(crate) const LIST_TOOLS_RESPONSE_ID: u64 = 2;
pub(crate) const FIRST_CALL_RESPONSE_ID: u64 = 3;
