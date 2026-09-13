pub(crate) mod dispatch;
mod inventory;
mod opening;
pub(crate) mod policy;
mod prerequisite;
#[cfg(test)]
mod recovery_controls;
pub(crate) mod stages;

const QUEUE_POLICY_LABEL: &str = "live-steel-agent-queue";
const QUEUE_SOURCE: &str = "coding-session-host";
pub(crate) const RUN_ID_PREFIX: &str = "run.live-steel-agent.";
pub(crate) const ENTRY_ID_PREFIX: &str = "entry.live-steel-agent.user.";
const SUMMARY: &str = "live Steel agent summary omitted";
const LIVE_STEEL_TOOL_EXECUTOR_BINDING: &str = "executor.live-steel-agent.steel-repl";
const LIVE_STEEL_TOOL_LABEL: &str = "Steel REPL";
const LIVE_STEEL_TOOL_DESCRIPTION: &str = "Evaluates one bounded Steel expression.";
const LIVE_STEEL_TOOL_PROMPT: &str = "Use steel_repl for one bounded Steel expression.";
const LIVE_STEEL_TOOL_GUIDELINE: &str = "Supply one code argument.";
const LIVE_STEEL_TOOL_PROVENANCE: &str = "maintained-live-composition";
const LIVE_STEEL_TOOL_RESULT_SHAPE: &str = "bounded-text.v1";
const LIVE_PROVIDER_TRANSPORT_ID: &str = "openai-responses-sse-v1";
const LIVE_MODEL_DIALECT_ID: &str = "openai-responses-tools-v1";
const MAX_UTF8_BYTES_PER_CHAR: u64 = 4;

/// Coding-session persistence with explicit lease release.
pub trait SessionPersistence:
    animus_std::coding_session_host::CodingSessionPersistenceService
{
    /// Releases the session persistence lease.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when release is uncertain.
    fn close(self) -> Result<(), LiveSteelAgentError>
    where
        Self: Sized;
}

impl<Synchronizer> SessionPersistence
    for animus_std::coding_session_host::DurableCodingSessionPersistence<'_, Synchronizer>
where
    Synchronizer: animus_std::DurableSessionSynchronizer,
{
    fn close(self) -> Result<(), LiveSteelAgentError> {
        self.close().map_err(|_error| LiveSteelAgentError::Session)
    }
}

/// Model-request snapshot persistence with explicit lease release.
pub trait SnapshotPersistence:
    animus_std::model_request_snapshot_host::ModelRequestSnapshotStorePort
{
    /// Releases the snapshot persistence lease.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when release is uncertain.
    fn close(self) -> Result<(), LiveSteelAgentError>
    where
        Self: Sized;
}

impl<Synchronizer> SnapshotPersistence
    for animus_std::model_request_snapshot_host::DurableModelRequestSnapshotStore<'_, Synchronizer>
where
    Synchronizer: animus_std::DurableSessionSynchronizer,
{
    fn close(self) -> Result<(), LiveSteelAgentError> {
        self.close().map_err(|_error| LiveSteelAgentError::Session)
    }
}

/// Complete explicit inputs for one application composition.
pub struct LiveSteelAgentCompositionInput<
    Provider,
    Executor,
    SessionPersistence,
    SnapshotPersistence,
    KernelPersistence,
> {
    /// Validated deterministic application configuration.
    pub config: crate::ValidatedLiveSteelAgentConfig,
    /// Explicit provider stream service.
    pub provider: Provider,
    /// Durable coding-session persistence.
    pub session_persistence: SessionPersistence,
    /// Durable private model-request snapshot persistence.
    pub snapshot_persistence: SnapshotPersistence,
    /// Fresh or effect-disabled restored Steel kernel.
    pub kernel: animus_steel::SteelRlmKernel,
    /// Exact host-effect executor, usually `RlmHostExecutor`.
    pub host_executor: Executor,
    /// Private committed kernel safe-point persistence.
    pub kernel_persistence: KernelPersistence,
}

/// Typed application construction or run failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveSteelAgentError {
    /// Provider request, stream, or continuation failed.
    Provider,
    /// Steel tool, host effect, or safe-point persistence failed.
    Tool,
    /// Coding-session state, service, reducer, or ordering failed.
    Session,
    /// Authority or queue policy construction failed.
    Authority,
    /// Selected Steel skills could not load atomically.
    Skill,
    /// Final receipt or kernel safe-point projection failed.
    Receipt,
}

impl core::fmt::Display for LiveSteelAgentError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "live Steel agent failed: {self:?}")
    }
}

impl std::error::Error for LiveSteelAgentError {}

/// Completed bounded application output.
pub struct LiveSteelAgentRunOutput {
    pub(crate) final_text: String,
    pub(crate) runtime_events: Vec<animus_runtime::RuntimeEvent>,
    pub(crate) receipt: crate::LiveSteelAgentRunReceipt,
}

impl LiveSteelAgentRunOutput {
    /// Returns the final assistant text. This private result is separate from evidence.
    #[must_use]
    pub fn final_text(&self) -> &str {
        self.final_text.as_str()
    }

    /// Returns the ordered private runtime events for a worker-owned delivery path.
    ///
    /// These events are not evidence and never enter the metadata-only receipt.
    // r[impl agent_harness_runtime.live_terminal_agent_host.private_events.source]
    #[must_use]
    pub fn runtime_events(&self) -> &[animus_runtime::RuntimeEvent] {
        self.runtime_events.as_slice()
    }

    /// Returns the metadata-only run receipt.
    #[must_use]
    pub const fn receipt(&self) -> &crate::LiveSteelAgentRunReceipt {
        &self.receipt
    }
}

/// One visible product composition root.
pub struct LiveSteelAgentComposition<
    Provider,
    Executor,
    SessionPersistence,
    SnapshotPersistence,
    KernelPersistence,
> {
    config: crate::ValidatedLiveSteelAgentConfig,
    provider: Provider,
    session_persistence: SessionPersistence,
    snapshot_persistence: SnapshotPersistence,
    tool: crate::SteelToolService<Executor, KernelPersistence>,
}
