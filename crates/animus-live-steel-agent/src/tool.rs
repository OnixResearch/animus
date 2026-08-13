use std::time::Instant;

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
    /// Returns [`SteelToolError::Persistence`] when release is uncertain.
    pub fn close(self) -> Result<(), SteelToolError>
    where
        Persistence: KernelSafePointPersistence,
    {
        self.persistence.close()
    }
}

impl<Executor, Persistence> SteelToolService<Executor, Persistence>
where
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: KernelSafePointPersistence,
{
    /// Executes one exact request through the complete resumable path.
    ///
    /// # Errors
    ///
    /// Returns [`SteelToolError`] for request drift, bridge failure, host
    /// failure, output bounds, cancellation, or safe-point persistence failure.
    pub fn execute(
        &mut self,
        request: &animus_adapters::ToolRequest,
    ) -> Result<animus_adapters::ToolResponse, SteelToolError> {
        validate_request(request)?;
        let source = request.arguments()[0].value();
        let cell_identity = hash_parts(
            CELL_IDENTITY_DOMAIN,
            &[
                self.binding.session_identity.as_str(),
                request.correlation_id(),
                source,
            ],
        );
        let started = Instant::now();
        let mut control = ExplicitBridgeControl {
            cancelled: self.cancelled,
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
        .map_err(|error| match error.detail() {
            animus_runtime::rlm_kernel::RlmBridgeErrorDetail::Execution(_) => SteelToolError::Host,
            animus_runtime::rlm_kernel::RlmBridgeErrorDetail::Contract(
                animus_adapter_rlm_kernel::Error::BridgeCancelled,
            ) => SteelToolError::Cancelled,
            animus_runtime::rlm_kernel::RlmBridgeErrorDetail::Contract(_)
            | animus_runtime::rlm_kernel::RlmBridgeErrorDetail::Steel(_) => {
                SteelToolError::Evaluation
            }
        })?;
        self.host_observation_count = self.host_observation_count.saturating_add(
            u64::try_from(completed.observations().len()).unwrap_or(u64::MAX),
        );
        let value = match completed.evaluation() {
            animus_steel::SteelRlmEvaluation::Value { value, .. } => value.as_str(),
            animus_steel::SteelRlmEvaluation::HostRequest { .. } => {
                return Err(SteelToolError::Evaluation);
            }
        };
        let bounded = animus_adapters::tool::BoundedOutput::new(
            &animus_adapters::tool::OutputDraft {
                body: value,
                full_output_reference: None,
                hash_requirement: animus_adapters::tool::HashRequirement::Required,
            },
            request.output_policy(),
        )
        .map_err(|_error| SteelToolError::Output)?;
        let journal = self
            .kernel
            .private_journal_snapshot(
                self.binding.session_identity.as_str(),
                self.binding.generation,
            )
            .map_err(|_error| SteelToolError::Persistence)?;
        self.persistence.persist(&journal)?;
        Ok(animus_adapters::ToolResponse::from_bounded(
            bounded,
            animus_adapters::tool::OutputRedactionClass::Safe,
        ))
    }
}

/// Private safe-point persistence port owned by the application shell.
pub trait KernelSafePointPersistence {
    /// Persists one complete committed kernel journal atomically.
    ///
    /// # Errors
    ///
    /// Returns [`SteelToolError::Persistence`] when publication is not confirmed.
    fn persist(
        &mut self,
        journal: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<(), SteelToolError>;

    /// Releases the persistence lease.
    ///
    /// # Errors
    ///
    /// Returns [`SteelToolError::Persistence`] when release is uncertain.
    fn close(self) -> Result<(), SteelToolError>
    where
        Self: Sized;
}

impl KernelSafePointPersistence for () {
    fn persist(
        &mut self,
        _journal: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<(), SteelToolError> {
        Ok(())
    }

    fn close(self) -> Result<(), SteelToolError> {
        Ok(())
    }
}

impl<Synchronizer> KernelSafePointPersistence
    for animus_std::DurableSteelKernelPersistence<'_, Synchronizer>
where
    Synchronizer: animus_std::DurableSessionSynchronizer,
{
    fn persist(
        &mut self,
        journal: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<(), SteelToolError> {
        self.commit_snapshot(journal)
            .map(|_outcome| ())
            .map_err(|_error| SteelToolError::Persistence)
    }

    fn close(self) -> Result<(), SteelToolError> {
        self.close().map_err(|_error| SteelToolError::Persistence)
    }
}

struct ExplicitBridgeControl {
    cancelled: bool,
    started: Instant,
}

impl animus_runtime::rlm_kernel::RlmBridgeControl for ExplicitBridgeControl {
    fn observe(
        &mut self,
        _checkpoint: animus_runtime::rlm_kernel::RlmBridgeCheckpoint,
    ) -> animus_runtime::rlm_kernel::RlmBridgeControlObservation {
        let elapsed = self.started.elapsed().as_millis();
        animus_runtime::rlm_kernel::RlmBridgeControlObservation {
            cancelled: self.cancelled,
            elapsed_ms: u64::try_from(elapsed).unwrap_or(u64::MAX),
        }
    }
}

fn validate_request(request: &animus_adapters::ToolRequest) -> Result<(), SteelToolError> {
    let exact = request.name() == animus_std_openai::STEEL_REPL_TOOL_NAME
        && request.schema_identity() == animus_std_openai::STEEL_REPL_TOOL_SCHEMA
        && request.approval_posture() != animus_adapters::tool::ApprovalPosture::Denied
        && request.arguments().len() == 1
        && request.arguments()[0].name() == "code";
    if !exact {
        return Err(SteelToolError::Request);
    }
    Ok(())
}

fn hash_parts(domain: &str, parts: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    for part in parts {
        let length = u64::try_from(part.len()).unwrap_or(u64::MAX);
        hasher.update(&length.to_le_bytes());
        hasher.update(part.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}
