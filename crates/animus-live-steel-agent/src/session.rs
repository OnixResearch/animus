//! Persistent worker-owned live-session shell.

mod control;
mod lifecycle;

/// Maximum private runtime events returned by one incremental snapshot.
pub const MAX_LIVE_SESSION_EVENT_BATCH: u32 = 256;

pub(crate) const QUEUED_ITEM_ID_PREFIX: &str = "queue.live-steel-agent.";

/// Safe lifecycle status for one persistent live session.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PersistentStatus {
    /// No run or pending effect exists.
    Idle,
    /// A run or effect remains active.
    Running,
    /// The current run completed.
    Completed,
    /// The current run was cancelled.
    Cancelled,
    /// The current run failed closed.
    Blocked,
}

/// Correlated private prompt accepted by the persistent session owner.
#[derive(Clone, Copy)]
pub struct LiveSteelPrompt<'a> {
    /// BLAKE3 correlation identity.
    pub correlation_id: &'a str,
    /// Private prompt text.
    pub text: &'a str,
}

/// Correlated private follow-up or steering input.
#[derive(Clone, Copy)]
pub struct LiveSteelQueuedInput<'a> {
    /// BLAKE3 correlation identity.
    pub correlation_id: &'a str,
    /// Private queued text.
    pub text: &'a str,
}

/// Exact queue edit request from the worker-owned private command path.
#[derive(Clone, Copy)]
pub struct LiveSteelQueueEdit<'a> {
    /// Queue item identity.
    pub item_id: &'a str,
    /// Expected queue generation.
    pub expected_generation: u64,
    /// Expected queue BLAKE3 identity.
    pub expected_queue_identity: &'a str,
    /// Private replacement text.
    pub replacement: &'a str,
}

/// Exact queue removal request from the worker-owned private command path.
#[derive(Clone, Copy)]
pub struct LiveSteelQueueRemoval<'a> {
    /// Queue item identity.
    pub item_id: &'a str,
    /// Expected queue generation.
    pub expected_generation: u64,
    /// Expected queue BLAKE3 identity.
    pub expected_queue_identity: &'a str,
}

/// Explicit worker-owned approval decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveApprovalDecision {
    /// Admit the correlated pending operation.
    Approve,
    /// Deny the correlated pending operation.
    Deny,
}

/// Safe child lifecycle status returned by the executor owner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LiveChildStatus {
    /// Child work is queued or running.
    Active,
    /// Child work completed.
    Completed,
    /// Child cancellation was accepted or completed.
    Cancelled,
    /// Child work failed, timed out, or blocked.
    Failed,
}

/// Narrow control port retained by the exact persistent host executor.
pub trait LiveSessionExecutorControl {
    /// Host-specific control failure.
    type Error;

    /// Answers one exact pending approval correlation.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when no matching pending approval exists.
    fn answer_approval(
        &mut self,
        correlation_id: &str,
        decision: LiveApprovalDecision,
    ) -> Result<(), Self::Error>;

    /// Returns metadata-only status for one exact child identity.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the child is unknown or cannot be observed.
    fn child_status(&mut self, child_id: &str) -> Result<LiveChildStatus, Self::Error>;

    /// Requests cancellation for one exact child identity.
    ///
    /// # Errors
    ///
    /// Returns [`Self::Error`] when the child is unknown or cancellation fails.
    fn cancel_child(&mut self, child_id: &str) -> Result<LiveChildStatus, Self::Error>;
}

/// Cursor and maximum for one bounded private event read.
#[derive(Clone, Copy)]
pub struct LiveSteelEventRead {
    /// Prior event cursor.
    pub cursor: u32,
    /// Maximum events to return.
    pub maximum: u32,
}

/// Bounded private runtime-event batch after one caller cursor.
pub struct PrivateEventBatch {
    pub(crate) next_cursor: u32,
    pub(crate) events: Vec<animus_runtime::RuntimeEvent>,
}

impl PrivateEventBatch {
    /// Returns the cursor after this batch.
    #[must_use]
    pub const fn next_cursor(&self) -> u32 {
        self.next_cursor
    }

    /// Returns the exact ordered private runtime events.
    #[must_use]
    pub fn runtime_events(&self) -> &[animus_runtime::RuntimeEvent] {
        self.events.as_slice()
    }
}

pub(crate) type PersistentCodingHost<
    Provider,
    Executor,
    Persistence,
    SnapshotStore,
    KernelPersistence,
> = animus_std::coding_session_host::CodingSessionHost<
    crate::shell::ProviderAdapter<Provider, SnapshotStore>,
    crate::shell::ToolAdapter<Executor, KernelPersistence>,
    Persistence,
    animus_std::coding_session_host::AllowCodingSessionPolicy,
    animus_std::coding_session_host::AdmittedCodingSessionAuthority,
    animus_std::coding_session_host::ObservedCodingSessionCancellation,
    animus_std::coding_session_host::AvailableCodingSessionTimer,
    animus_std::coding_session_host::FakeCodingSessionSummarizer,
    crate::shell::MetadataEventSink,
>;

/// Persistent live session with one visible owner for all session effects.
pub struct PersistentLiveHost<Provider, Executor, Persistence, SnapshotStore, KernelPersistence> {
    pub(crate) config: std::rc::Rc<crate::ValidatedLiveSteelAgentConfig>,
    pub(crate) host:
        PersistentCodingHost<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>,
}

impl<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
    PersistentLiveHost<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
{
    pub(crate) fn new(
        config: std::rc::Rc<crate::ValidatedLiveSteelAgentConfig>,
        host: PersistentCodingHost<
            Provider,
            Executor,
            Persistence,
            SnapshotStore,
            KernelPersistence,
        >,
    ) -> Self {
        Self { config, host }
    }
}
