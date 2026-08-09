//! Non-default read-only codebase graph tool host.

mod core;
mod shell;
mod smoke;

pub use core::{
    CODEBASE_QUERY_ARGUMENT_KIND, CODEBASE_QUERY_ARGUMENT_LIMIT, CODEBASE_QUERY_ARGUMENT_SYMBOL,
    CODEBASE_QUERY_CAPABILITY, CODEBASE_QUERY_EXECUTOR_BINDING, CODEBASE_QUERY_SCHEMA_IDENTITY,
    CODEBASE_QUERY_TOOL_NAME, CodebaseToolError, CodebaseToolQuery, build_codebase_query_catalog,
    build_codebase_query_request, decode_codebase_query, render_codebase_query_result,
};
pub use shell::{CodebaseQueryToolExecutor, execute_codebase_query_tool};
pub use smoke::{CodebaseToolSmokeRequest, run_packaged_codebase_tool_smoke};

#[cfg(test)]
mod tests;
