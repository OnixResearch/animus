use std::path::Path;

use animus_runtime::tool_executor_kit::ToolExecutorAdmission;
use cap_std::fs::Dir;

use super::{
    CODEBASE_QUERY_TOOL_NAME, CodebaseQueryToolExecutor, CodebaseToolError,
    build_codebase_query_request, execute_codebase_query_tool,
};
use crate::{CodeGraphHost, parse_code_graph_profile_json};

const CORRELATION_ID: &str = "packaged-codebase-query";
const STATUS_PREFIX: &str = "CODEBASE_TOOL_SMOKE status=passed";
const REDACTION_MARKER: &str = "raw_output=omitted";

/// Borrowed explicit inputs for one packaged smoke run.
pub struct CodebaseToolSmokeRequest<'a> {
    /// Caller-selected root path.
    pub root_path: &'a Path,
    /// Caller-selected capability-root-relative Rust files.
    pub relative_paths: &'a [String],
    /// Exact syntactic symbol name.
    pub symbol: &'a str,
    /// Exact admitted symbol kind.
    pub kind: &'a str,
    /// Finite query result limit.
    pub max_results: usize,
}

/// Runs one packaged query over an explicit root and relative file list.
///
/// # Errors
///
/// Returns [`CodebaseToolError`] when root opening, profile admission, indexing, planning, execution, output bounding, or receipt validation fails.
pub fn run_packaged_codebase_tool_smoke(
    input: &CodebaseToolSmokeRequest<'_>,
) -> Result<String, CodebaseToolError> {
    // r[impl agent_harness_runtime.read_only_codebase_tool.smoke]
    let profile = parse_code_graph_profile_json(include_bytes!(
        "../../../../config/generated/codebase-graph-profile.json"
    ))
    .map_err(|_error| CodebaseToolError::HostFailed)?;
    let root = Dir::open_ambient_dir(input.root_path, cap_std::ambient_authority())
        .map_err(|_error| CodebaseToolError::HostFailed)?;
    let host = CodeGraphHost::index_rust_files(&root, profile, input.relative_paths)
        .map_err(|_error| CodebaseToolError::HostFailed)?;
    let request =
        build_codebase_query_request(input.symbol, input.kind, input.max_results, CORRELATION_ID)?;
    let mut executor = CodebaseQueryToolExecutor::new(host);
    let outcome =
        execute_codebase_query_tool(&mut executor, &request, ToolExecutorAdmission::admitted())?;
    let receipt = outcome.receipt();
    Ok(format!(
        "{STATUS_PREFIX} tool={} output_bytes={} output_b3={} receipt_b3={} {REDACTION_MARKER}\n",
        CODEBASE_QUERY_TOOL_NAME,
        receipt.output_byte_count(),
        receipt.output_hash(),
        receipt.receipt_hash(),
    ))
}
