mod checkpoint;
pub(crate) mod ports;

const FIRST_NESTING_DEPTH: u32 = 1;
const CELL_IDENTITY_DOMAIN: &str = "animus.live-steel-agent.cell.v1";

/// Steel tool failure without source, arguments, or result bodies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SteelToolError {
    /// Tool request shape or exact provider correlation was invalid.
    Request,
    /// Steel or resumable bridge evaluation failed.
    Evaluation,
    /// Host effect execution failed.
    Host,
    /// Explicit cancellation stopped the bridge before safe completion.
    Cancelled,
    /// Model-visible output could not satisfy the request bound.
    Output,
    /// Kernel safe-point persistence failed.
    Persistence,
}

impl core::fmt::Display for SteelToolError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "Steel tool execution rejected: {self:?}")
    }
}

impl std::error::Error for SteelToolError {}

/// Executes `steel_repl` through the resumable bridge and injected host executor.
pub struct SteelToolService<Executor, Persistence> {
    kernel: animus_steel::SteelRlmKernel,
    executor: Executor,
    persistence: Persistence,
    binding: SteelToolBinding,
    cancelled: bool,
    host_observation_count: u64,
    pending_stage_journal: Option<animus_steel::SteelKernelPrivateJournal>,
}

/// Exact private-journal binding for one Steel tool service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SteelToolBinding {
    /// Exact BLAKE3 session identity.
    pub session_identity: String,
    /// Current host-owned authority generation.
    pub generation: u64,
}

impl<Executor, Persistence> SteelToolService<Executor, Persistence> {
    /// Builds the exact Steel, host-effect, and private persistence owner.
    #[must_use]
    pub fn new(
        kernel: animus_steel::SteelRlmKernel,
        executor: Executor,
        persistence: Persistence,
        binding: SteelToolBinding,
    ) -> Self {
        Self {
            kernel,
            executor,
            persistence,
            binding,
            cancelled: false,
            host_observation_count: 0,
            pending_stage_journal: None,
        }
    }

    /// Sets explicit cancellation for later bridge checkpoints.
    pub const fn set_cancelled(&mut self, cancelled: bool) {
        self.cancelled = cancelled;
    }

    /// Returns the committed kernel.
    #[must_use]
    pub const fn kernel(&self) -> &animus_steel::SteelRlmKernel {
        &self.kernel
    }

    /// Returns the exact host executor for worker-owned control delegation.
    #[must_use]
    pub const fn host_executor_mut(&mut self) -> &mut Executor {
        &mut self.executor
    }

    /// Returns mutable persistence for controlled recovery tests.
    #[must_use]
    pub const fn persistence_mut(&mut self) -> &mut Persistence {
        &mut self.persistence
    }

    /// Returns the count of completed host effects.
    #[must_use]
    pub const fn host_observation_count(&self) -> u64 {
        self.host_observation_count
    }

    /// Releases the private persistence lease after the final safe point.
    ///
    /// # Errors
    ///
    /// Returns [`crate::SteelToolError::Persistence`] when release is uncertain.
    pub fn close(self) -> Result<(), SteelToolError>
    where
        Persistence: ports::KernelSafePointPersistence,
    {
        if self.pending_stage_journal.is_some() {
            return Err(SteelToolError::Persistence);
        }
        self.persistence.close()
    }
}

impl<Executor, Persistence> SteelToolService<Executor, Persistence>
where
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: ports::KernelSafePointPersistence,
{
    /// Executes one exact request through the complete resumable path.
    ///
    /// # Errors
    ///
    /// Returns [`crate::SteelToolError`] for request drift, bridge failure, host
    /// failure, output bounds, cancellation, or safe-point persistence failure.
    pub fn execute(
        &mut self,
        request: &animus_adapters::ToolRequest,
    ) -> Result<animus_adapters::ToolResponse, SteelToolError> {
        let value = self.execute_stage_value(request)?;
        let bounded = animus_adapters::tool::BoundedOutput::new(
            &animus_adapters::tool::OutputDraft {
                body: value.as_str(),
                full_output_reference: None,
                hash_requirement: animus_adapters::tool::HashRequirement::Required,
            },
            request.output_policy(),
        )
        .map_err(|_error| SteelToolError::Output)?;
        self.persist_staged_safe_point()?;
        Ok(animus_adapters::ToolResponse::from_bounded(
            bounded,
            animus_adapters::tool::OutputRedactionClass::Safe,
        ))
    }

    pub(crate) fn execute_stage_value(
        &mut self,
        request: &animus_adapters::ToolRequest,
    ) -> Result<String, SteelToolError> {
        if self.pending_stage_journal.is_some() {
            return Err(SteelToolError::Persistence);
        }
        checkpoint::validate_request(request)?;
        let source = request.arguments()[0].value();
        let cell_identity = checkpoint::hash_parts(
            CELL_IDENTITY_DOMAIN,
            &[
                self.binding.session_identity.as_str(),
                request.correlation_id(),
                source,
            ],
        )?;
        let clock = cap_std::time::MonotonicClock::new(cap_std::ambient_authority());
        let started = clock.now();
        let mut control = checkpoint::ExplicitBridgeControl {
            cancelled: self.cancelled,
            clock,
            started,
        };
        // r[impl agent_harness_runtime.live_steel_agent_shell.tool_loop.marker_only]
        let completed = animus_runtime::rlm_kernel::execute_bridged_cell(
            &mut self.kernel,
            animus_runtime::rlm_kernel::RlmBridgeExecutionRequest {
                source,
                session_identity: self.binding.session_identity.as_str(),
                cell_identity: cell_identity.as_str(),
                nesting_depth: FIRST_NESTING_DEPTH,
            },
            &mut self.executor,
            &mut control,
        )
        .map_err(|error| checkpoint::map_bridge_error(&error))?;
        let observed_count = u64::try_from(completed.observations().len())
            .map_err(|_error| SteelToolError::Output)?;
        self.host_observation_count = self
            .host_observation_count
            .checked_add(observed_count)
            .ok_or(SteelToolError::Output)?;
        let value = match completed.evaluation() {
            animus_steel::SteelRlmEvaluation::Value { value, .. } => value.clone(),
            animus_steel::SteelRlmEvaluation::HostRequest { .. } => {
                return Err(SteelToolError::Evaluation);
            }
        };
        let journal = self
            .kernel
            .private_journal_snapshot(
                self.binding.session_identity.as_str(),
                self.binding.generation,
            )
            .map_err(|_error| SteelToolError::Persistence)?;
        self.pending_stage_journal = Some(journal);
        Ok(value)
    }

    pub(crate) fn persist_staged_safe_point(&mut self) -> Result<(), SteelToolError> {
        let journal = self
            .pending_stage_journal
            .take()
            .ok_or(SteelToolError::Persistence)?;
        if self.persistence.persist(&journal).is_err() {
            self.pending_stage_journal = Some(journal);
            return Err(SteelToolError::Persistence);
        }
        Ok(())
    }
}
