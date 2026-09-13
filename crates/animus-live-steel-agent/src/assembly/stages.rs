pub(crate) struct ToolAdapter<Executor, Persistence> {
    pub(crate) tool: crate::SteelToolService<Executor, Persistence>,
    pub(super) config: std::rc::Rc<crate::ValidatedLiveSteelAgentConfig>,
    pub(super) shared: std::rc::Rc<
        std::cell::RefCell<crate::adapters::responses::continuation::SharedProviderState>,
    >,
    pub(crate) dispatch_count: u32,
    pub(super) pending_stage:
        Option<animus_std::tool_execution_stage_host::PendingStagedToolExecution>,
}

impl<Executor, Persistence> ToolAdapter<Executor, Persistence> {
    pub(crate) const fn host_executor_mut(&mut self) -> &mut Executor {
        self.tool.host_executor_mut()
    }
}

impl<Executor, Persistence> animus_std::coding_session_host::CodingSessionToolService
    for ToolAdapter<Executor, Persistence>
where
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: crate::evaluation::ports::KernelSafePointPersistence,
{
    fn dispatch(
        &mut self,
        request: &animus_runtime::coding_session_host::CodingSessionToolRequest,
    ) -> Result<
        animus_adapters::ToolResponse,
        animus_runtime::coding_session_host::CodingSessionServiceIssue,
    > {
        if self.pending_stage.is_some() {
            return Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale);
        }
        self.dispatch_count = self
            .dispatch_count
            .checked_add(1)
            .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)?;
        let correlation = self
            .shared
            .borrow()
            .pending()
            .map(|pending| String::from(pending.correlation_id()))
            .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)?;
        if request.request().correlation_id() != correlation {
            return Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale);
        }
        let catalog =
            super::inventory::live_steel_tool_catalog(request.request(), self.config.as_ref())?;
        let capabilities = super::inventory::live_steel_tool_capabilities()?;
        let executor_bindings = [animus_runtime::tool_executor_kit::ToolExecutorBinding::new(
            super::LIVE_STEEL_TOOL_EXECUTOR_BINDING,
            animus_runtime::tool_executor_kit::ToolExecutorReadiness::Ready,
        )
        .map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed
        })?];
        let authority =
            crate::build_expected_authority(self.config.as_ref()).map_err(|_error| {
                animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied
            })?;
        let commit_owner =
            animus_runtime::tool_execution_stages::coding_session_tool_commit_owner_identity();
        let stage_input = animus_std::tool_execution_stage_host::StagedToolExecutionInput {
            tool: animus_runtime::tool_executor_kit::ToolExecutionPlanRequest {
                request: request.request(),
                catalog: &catalog,
                capabilities: &capabilities,
                executors: &executor_bindings,
                admission: animus_runtime::tool_executor_kit::ToolExecutorAdmission::admitted(),
            },
            authority_seal_identity: authority.unit.unit_identity(),
            commit_owner_identity: commit_owner.as_str(),
            handlers: &[],
        };
        let mut stage_services = live_noop_stage_services();
        let mut executor = LiveSteelToolKitExecutor {
            tool: &mut self.tool,
        };
        let pending = animus_std::tool_execution_stage_host::execute_staged_tool(
            &mut executor,
            &mut stage_services,
            &stage_input,
        )
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)?;
        executor.persist_safe_point()?;
        let response = pending.outcome().response().clone();
        self.pending_stage = Some(pending);
        Ok(response)
    }

    fn observe_committed(
        &mut self,
        observation: &animus_std::coding_session_host::CodingSessionToolCommitObservation,
    ) -> Result<(), animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        let pending = self
            .pending_stage
            .take()
            .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)?;
        let stage_observation =
            animus_runtime::tool_execution_stages::ToolExecutionStageCommitObservation {
                stage_plan_identity: String::from(pending.plan().plan_identity()),
                result_identity: observation.result_identity.clone(),
                commit_owner_identity: observation.commit_owner_identity.clone(),
                durable_head_identity: observation.durable_head_identity.clone(),
                complete: true,
                synchronized: observation.synchronized,
            };
        let mut post_commit = animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers;
        let completed = animus_std::tool_execution_stage_host::complete_staged_tool_commit(
            &mut post_commit,
            pending,
            &stage_observation,
        )
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)?;
        self.shared
            .borrow_mut()
            .pending_mut()
            .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Stale)?
            .bind_result(completed.outcome.response().output())
            .map_err(super::policy::service_issue)
    }
}

pub(super) struct LiveSteelToolKitExecutor<'tool, Executor, Persistence> {
    pub(super) tool: &'tool mut crate::SteelToolService<Executor, Persistence>,
}

impl<Executor, Persistence> LiveSteelToolKitExecutor<'_, Executor, Persistence>
where
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: crate::evaluation::ports::KernelSafePointPersistence,
{
    fn persist_safe_point(
        &mut self,
    ) -> Result<(), animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        self.tool.persist_staged_safe_point().map_err(|_error| {
            animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed
        })
    }
}

impl<Executor, Persistence> animus_runtime::tool_executor_kit::ToolExecutorService
    for LiveSteelToolKitExecutor<'_, Executor, Persistence>
where
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: crate::evaluation::ports::KernelSafePointPersistence,
{
    fn binding_label(&self) -> &str {
        super::LIVE_STEEL_TOOL_EXECUTOR_BINDING
    }

    fn execute(
        &mut self,
        _plan: &animus_runtime::tool_executor_kit::ToolExecutionPlan,
        request: &animus_adapters::ToolRequest,
    ) -> Result<
        animus_runtime::tool_executor_kit::ToolExecutorOutput,
        animus_runtime::tool_executor_kit::ToolExecutorServiceFailure,
    > {
        let value = self
            .tool
            .execute_stage_value(request)
            .map_err(execution_failure)?;
        Ok(animus_runtime::tool_executor_kit::ToolExecutorOutput::new(
            value.as_str(),
            None,
            animus_adapters::tool::OutputRedactionClass::Safe,
        ))
    }
}

const fn execution_failure(
    error: crate::SteelToolError,
) -> animus_runtime::tool_executor_kit::ToolExecutorServiceFailure {
    if matches!(error, crate::SteelToolError::Cancelled) {
        animus_runtime::tool_executor_kit::ToolExecutorServiceFailure::Cancelled
    } else {
        animus_runtime::tool_executor_kit::ToolExecutorServiceFailure::ExecutionFailed
    }
}

pub(super) fn live_noop_stage_services()
-> animus_std::tool_execution_stage_host::ToolExecutionStageServices<
    animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
    animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
    animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
    animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
> {
    animus_std::tool_execution_stage_host::ToolExecutionStageServices {
        pre_admission: animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
        pre_execution: animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
        output: animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
        post_commit: animus_std::tool_execution_stage_host::NoToolExecutionStageHandlers,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn explicit_cancellation_retains_its_distinct_failure() {
        assert_eq!(
            super::execution_failure(crate::SteelToolError::Cancelled),
            animus_runtime::tool_executor_kit::ToolExecutorServiceFailure::Cancelled,
        );
    }

    #[test]
    fn other_tool_errors_cannot_report_completion_or_cancellation() {
        for error in [
            crate::SteelToolError::Request,
            crate::SteelToolError::Evaluation,
            crate::SteelToolError::Host,
            crate::SteelToolError::Output,
            crate::SteelToolError::Persistence,
        ] {
            assert_eq!(
                super::execution_failure(error),
                animus_runtime::tool_executor_kit::ToolExecutorServiceFailure::ExecutionFailed,
            );
        }
    }
}
