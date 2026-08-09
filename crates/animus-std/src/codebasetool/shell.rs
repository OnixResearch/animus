use animus_adapters::codegraph::CodeGraphQuery;
use animus_runtime::tool_executor_kit::{
    ToolExecutionOutcome, ToolExecutionPlan, ToolExecutionPlanRequest, ToolExecutorAdmission,
    ToolExecutorBinding, ToolExecutorOutput, ToolExecutorReadiness, ToolExecutorService,
    ToolExecutorServiceFailure, execute_admitted_tool, plan_tool_execution,
};
use animus_runtime::{
    RuntimeCapabilityDescriptorDraft, RuntimeCapabilityRegistry, RuntimeCapabilityServiceKind,
};

use super::{
    CODEBASE_QUERY_CAPABILITY, CODEBASE_QUERY_EXECUTOR_BINDING, CodebaseToolError,
    build_codebase_query_catalog, decode_codebase_query, render_codebase_query_result,
};
use crate::CodeGraphHost;

/// Thin read-only executor over one caller-constructed capability-scoped graph.
pub struct CodebaseQueryToolExecutor {
    host: CodeGraphHost,
}

impl CodebaseQueryToolExecutor {
    /// Wraps one explicit fresh or stale graph without discovering host state.
    #[must_use]
    pub const fn new(host: CodeGraphHost) -> Self {
        Self { host }
    }

    /// Returns the wrapped graph for explicit host event updates.
    #[must_use]
    pub const fn host(&self) -> &CodeGraphHost {
        &self.host
    }

    /// Returns the wrapped graph mutably for explicit host event updates.
    #[must_use]
    pub const fn host_mut(&mut self) -> &mut CodeGraphHost {
        &mut self.host
    }
}

impl ToolExecutorService for CodebaseQueryToolExecutor {
    fn binding_label(&self) -> &str {
        CODEBASE_QUERY_EXECUTOR_BINDING
    }

    fn execute(
        &mut self,
        _plan: &ToolExecutionPlan,
        request: &animus_adapters::ToolRequest,
    ) -> Result<ToolExecutorOutput, ToolExecutorServiceFailure> {
        let query = decode_codebase_query(request)
            .map_err(|_error| ToolExecutorServiceFailure::ExecutionFailed)?;
        let result = self
            .host
            .query(&CodeGraphQuery {
                symbol: query.symbol(),
                kind: query.kind(),
                max_results: query.max_results(),
            })
            .map_err(|_error| ToolExecutorServiceFailure::ExecutionFailed)?;
        let body = render_codebase_query_result(&result)
            .map_err(|_error| ToolExecutorServiceFailure::ExecutionFailed)?;
        Ok(ToolExecutorOutput::new(
            body.as_str(),
            None,
            animus_adapters::tool::OutputRedactionClass::Safe,
        ))
    }
}

/// Plans and executes one exact request through the production executor kit.
///
/// # Errors
///
/// Returns [`CodebaseToolError`] for catalog, capability, admission, request-binding, query, output, or receipt failures.
pub fn execute_codebase_query_tool(
    executor: &mut CodebaseQueryToolExecutor,
    request: &animus_adapters::ToolRequest,
    admission: ToolExecutorAdmission,
) -> Result<ToolExecutionOutcome, CodebaseToolError> {
    // r[impl agent_harness_runtime.read_only_codebase_tool.request_binding]
    // r[impl agent_harness_runtime.read_only_codebase_tool.outcome]
    let catalog = build_codebase_query_catalog()?;
    let registry = RuntimeCapabilityRegistry::from_drafts(&[RuntimeCapabilityDescriptorDraft {
        label: CODEBASE_QUERY_CAPABILITY,
        service_kind: RuntimeCapabilityServiceKind::Tool.as_str(),
        scope: None,
    }])
    .map_err(|_error| CodebaseToolError::Contract)?;
    let binding = ToolExecutorBinding::new(
        CODEBASE_QUERY_EXECUTOR_BINDING,
        ToolExecutorReadiness::Ready,
    )?;
    let plan = plan_tool_execution(ToolExecutionPlanRequest {
        request,
        catalog: &catalog,
        capabilities: &registry,
        executors: &[binding],
        admission,
    });
    execute_admitted_tool(executor, &plan, request).map_err(Into::into)
}
