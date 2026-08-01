//! Authority-scoped blocking host for external agent control.
//!
//! The host owns explicit in-memory synchronization and a bounded event journal.
//! It owns no filesystem, network, process, credential, terminal, clock, global
//! registry, or persistence authority.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

use std::collections::VecDeque;
use std::num::NonZeroUsize;
use std::sync::{Arc, Condvar, Mutex, MutexGuard};

const NEXT_REVISION_INCREMENT: u64 = 1;

/// Explicit synchronized control host for one exact run.
#[derive(Clone, Debug)]
pub struct AgentControlHub {
    shared: Arc<SharedControl>,
}

#[derive(Debug)]
struct SharedControl {
    inner: Mutex<HostState>,
    changed: Condvar,
    event_capacity: usize,
}

#[derive(Debug)]
struct HostState {
    control: animus_runtime::AgentControlState,
    events: VecDeque<animus_adapters::AgentControlEvent>,
    closed: bool,
}

impl AgentControlHub {
    /// Builds an open in-memory host for one exact run.
    #[must_use]
    pub fn new(run_id: animus_adapters::AgentControlRunId, event_capacity: NonZeroUsize) -> Self {
        Self {
            shared: Arc::new(SharedControl {
                inner: Mutex::new(HostState {
                    control: animus_runtime::AgentControlState::new(run_id),
                    events: VecDeque::with_capacity(event_capacity.get()),
                    closed: false,
                }),
                changed: Condvar::new(),
                event_capacity: event_capacity.get(),
            }),
        }
    }

    /// Returns a cloned command publisher bound to this host.
    #[must_use]
    pub fn publisher(&self) -> AgentControlPublisher {
        AgentControlPublisher {
            shared: Arc::clone(&self.shared),
        }
    }

    /// Returns a cloned runtime waiter bound to this host.
    #[must_use]
    pub fn waiter(&self) -> AgentControlWaiter {
        AgentControlWaiter {
            shared: Arc::clone(&self.shared),
        }
    }

    /// Returns a cloned event subscriber bound to this host.
    #[must_use]
    pub fn subscriber(&self) -> AgentControlSubscriber {
        AgentControlSubscriber {
            shared: Arc::clone(&self.shared),
        }
    }

    /// Returns a metadata-only synchronized state snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`AgentControlHostError`] when synchronization fails.
    pub fn snapshot(&self) -> Result<AgentControlSnapshot, AgentControlHostError> {
        let state = lock_host(&self.shared)?;
        Ok(snapshot(&state))
    }

    /// Marks the host terminal, closes it, and wakes all waiters.
    ///
    /// # Errors
    ///
    /// Returns [`AgentControlHostError`] when synchronization or reduction fails.
    pub fn close(&self) -> Result<AgentControlSnapshot, AgentControlHostError> {
        let mut state = lock_host(&self.shared)?;
        if !state.closed {
            let transition = animus_runtime::reduce_agent_control_terminal(&state.control)
                .map_err(AgentControlHostError::Reduction)?;
            let event = transition.event().cloned();
            state.control = transition.into_state();
            if let Some(event) = event {
                record_event(&mut state, self.shared.event_capacity, event);
            }
            state.closed = true;
            self.shared.changed.notify_all();
        }
        Ok(snapshot(&state))
    }
}

/// Clonable external command publisher for one explicit host.
#[derive(Clone, Debug)]
pub struct AgentControlPublisher {
    shared: Arc<SharedControl>,
}

impl AgentControlPublisher {
    /// Applies one exact revision-fenced command and publishes its event.
    ///
    /// # Errors
    ///
    /// Returns [`AgentControlHostError`] for closed, stale, conflicting, or invalid commands.
    pub fn publish(
        &self,
        command: &animus_adapters::AgentControlCommand,
    ) -> Result<AgentControlAcknowledgement, AgentControlHostError> {
        // r[impl agent_harness_runtime.agent_control.host]
        let mut state = lock_host(&self.shared)?;
        if state.closed {
            return Err(AgentControlHostError::Closed);
        }
        let transition = animus_runtime::reduce_agent_control_command(&state.control, command)
            .map_err(AgentControlHostError::Reduction)?;
        let event = transition.event().cloned();
        let disposition = transition.disposition();
        state.control = transition.into_state();
        if let Some(event) = event {
            record_event(&mut state, self.shared.event_capacity, event);
            self.shared.changed.notify_all();
        }
        Ok(AgentControlAcknowledgement {
            revision: state.control.revision(),
            phase: state.control.phase(),
            disposition,
        })
    }

    /// Returns the current metadata-only state snapshot.
    ///
    /// # Errors
    ///
    /// Returns [`AgentControlHostError`] when synchronization fails.
    pub fn snapshot(&self) -> Result<AgentControlSnapshot, AgentControlHostError> {
        let state = lock_host(&self.shared)?;
        Ok(snapshot(&state))
    }
}

/// Clonable blocking waiter for runtime safe checkpoints.
#[derive(Clone, Debug)]
pub struct AgentControlWaiter {
    shared: Arc<SharedControl>,
}

impl animus_adapters::AgentControlWait for AgentControlWaiter {
    fn wait_until_runnable(
        &mut self,
        checkpoint: &animus_adapters::AgentControlCheckpoint,
    ) -> Result<animus_adapters::AgentControlPermit, animus_adapters::AgentControlWaitError> {
        let mut state =
            self.shared.inner.lock().map_err(|_poisoned| {
                animus_adapters::AgentControlWaitError::SynchronizationFailed
            })?;
        loop {
            if state.closed {
                return Err(animus_adapters::AgentControlWaitError::Closed);
            }
            let transition =
                animus_runtime::reduce_agent_control_checkpoint(&state.control, checkpoint)
                    .map_err(map_wait_reduction_error)?;
            let event = transition.event().cloned();
            let decision = transition.decision();
            state.control = transition.into_state();
            if let Some(event) = event {
                record_event(&mut state, self.shared.event_capacity, event);
                self.shared.changed.notify_all();
            }
            match decision {
                animus_runtime::AgentControlCheckpointDecision::Continue => {
                    return Ok(animus_adapters::AgentControlPermit::Continue);
                }
                animus_runtime::AgentControlCheckpointDecision::Cancel => {
                    return Ok(animus_adapters::AgentControlPermit::Cancel);
                }
                animus_runtime::AgentControlCheckpointDecision::Wait => {
                    state = self.shared.changed.wait(state).map_err(|_poisoned| {
                        animus_adapters::AgentControlWaitError::SynchronizationFailed
                    })?;
                }
            }
        }
    }
}

/// Clonable cursor-based event subscriber for one explicit host.
#[derive(Clone, Debug)]
pub struct AgentControlSubscriber {
    shared: Arc<SharedControl>,
}

impl AgentControlSubscriber {
    /// Returns the next event after a revision without blocking.
    ///
    /// # Errors
    ///
    /// Returns [`AgentControlHostError`] when synchronization fails.
    pub fn next_after(
        &self,
        revision: u64,
    ) -> Result<AgentControlSubscriptionOutcome, AgentControlHostError> {
        let state = lock_host(&self.shared)?;
        Ok(next_subscription_outcome(&state, revision))
    }

    /// Blocks without polling until an event, lag outcome, or closure follows a revision.
    ///
    /// # Errors
    ///
    /// Returns [`AgentControlHostError`] when synchronization fails.
    pub fn wait_after(
        &self,
        revision: u64,
    ) -> Result<AgentControlSubscriptionOutcome, AgentControlHostError> {
        let mut state = lock_host(&self.shared)?;
        loop {
            let outcome = next_subscription_outcome(&state, revision);
            if outcome != AgentControlSubscriptionOutcome::Pending {
                return Ok(outcome);
            }
            state = self
                .shared
                .changed
                .wait(state)
                .map_err(|_poisoned| AgentControlHostError::SynchronizationFailed)?;
        }
    }
}

/// Metadata-only state returned by the explicit control host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentControlSnapshot {
    run_id: animus_adapters::AgentControlRunId,
    revision: u64,
    phase: animus_adapters::AgentControlPhase,
    closed: bool,
}

impl AgentControlSnapshot {
    /// Returns the exact run identity.
    #[must_use]
    pub const fn run_id(&self) -> &animus_adapters::AgentControlRunId {
        &self.run_id
    }

    /// Returns the current revision.
    #[must_use]
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    /// Returns the current phase.
    #[must_use]
    pub const fn phase(&self) -> animus_adapters::AgentControlPhase {
        self.phase
    }

    /// Reports whether the host is explicitly closed.
    #[must_use]
    pub const fn is_closed(&self) -> bool {
        self.closed
    }
}

/// Metadata-only acknowledgement returned after command publication.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AgentControlAcknowledgement {
    revision: u64,
    phase: animus_adapters::AgentControlPhase,
    disposition: animus_runtime::AgentControlCommandDisposition,
}

impl AgentControlAcknowledgement {
    /// Returns the resulting revision.
    #[must_use]
    pub const fn revision(self) -> u64 {
        self.revision
    }

    /// Returns the resulting phase.
    #[must_use]
    pub const fn phase(self) -> animus_adapters::AgentControlPhase {
        self.phase
    }

    /// Returns whether the command applied or was an exact duplicate.
    #[must_use]
    pub const fn disposition(self) -> animus_runtime::AgentControlCommandDisposition {
        self.disposition
    }
}

/// Bounded cursor read outcome for the host event journal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AgentControlSubscriptionOutcome {
    /// The next retained event after the supplied cursor.
    Event(animus_adapters::AgentControlEvent),
    /// The cursor fell behind bounded retention.
    Lagged {
        /// Oldest retained event revision.
        oldest_available_revision: u64,
    },
    /// No newer event exists yet.
    Pending,
    /// The host closed and no newer retained event remains.
    Closed,
}

/// Typed standard-host error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentControlHostError {
    /// The explicit control host closed.
    Closed,
    /// The synchronization primitive failed.
    SynchronizationFailed,
    /// The pure reducer rejected supplied facts.
    Reduction(animus_runtime::AgentControlReductionError),
}

impl core::fmt::Display for AgentControlHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Closed => formatter.write_str("agent control host closed"),
            Self::SynchronizationFailed => {
                formatter.write_str("agent control synchronization failed")
            }
            Self::Reduction(error) => write!(formatter, "agent control reduction failed: {error}"),
        }
    }
}

impl std::error::Error for AgentControlHostError {}

fn lock_host(shared: &SharedControl) -> Result<MutexGuard<'_, HostState>, AgentControlHostError> {
    shared
        .inner
        .lock()
        .map_err(|_poisoned| AgentControlHostError::SynchronizationFailed)
}

fn snapshot(state: &HostState) -> AgentControlSnapshot {
    AgentControlSnapshot {
        run_id: state.control.run_id().clone(),
        revision: state.control.revision(),
        phase: state.control.phase(),
        closed: state.closed,
    }
}

fn record_event(state: &mut HostState, capacity: usize, event: animus_adapters::AgentControlEvent) {
    debug_assert!(capacity > 0);
    if state.events.len() == capacity {
        let removed = state.events.pop_front();
        debug_assert!(removed.is_some());
    }
    state.events.push_back(event);
    debug_assert!(state.events.len() <= capacity);
}

fn next_subscription_outcome(state: &HostState, revision: u64) -> AgentControlSubscriptionOutcome {
    if let Some(oldest) = state.events.front() {
        let next_revision = revision.saturating_add(NEXT_REVISION_INCREMENT);
        if oldest.revision() > next_revision {
            return AgentControlSubscriptionOutcome::Lagged {
                oldest_available_revision: oldest.revision(),
            };
        }
    }
    if let Some(event) = state
        .events
        .iter()
        .find(|event| event.revision() > revision)
    {
        return AgentControlSubscriptionOutcome::Event(event.clone());
    }
    if state.closed {
        AgentControlSubscriptionOutcome::Closed
    } else {
        AgentControlSubscriptionOutcome::Pending
    }
}

const fn map_wait_reduction_error(
    error: animus_runtime::AgentControlReductionError,
) -> animus_adapters::AgentControlWaitError {
    match error {
        animus_runtime::AgentControlReductionError::RunMismatch => {
            animus_adapters::AgentControlWaitError::RunMismatch
        }
        animus_runtime::AgentControlReductionError::StaleRevision { .. }
        | animus_runtime::AgentControlReductionError::CommandIdentityConflict
        | animus_runtime::AgentControlReductionError::Cancelling
        | animus_runtime::AgentControlReductionError::Terminal
        | animus_runtime::AgentControlReductionError::RevisionOverflow => {
            animus_adapters::AgentControlWaitError::Unavailable
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::AgentControlWait;
    use std::sync::mpsc;
    use std::time::Duration;

    const RUN_ID: &str = "run.std-control";
    const OTHER_RUN_ID: &str = "run.other";
    const PAUSE_ID: &str = "command.pause";
    const RESUME_ID: &str = "command.resume";
    const SECOND_PAUSE_ID: &str = "command.pause-two";
    const CANCEL_ID: &str = "command.cancel";
    const EVENT_CAPACITY: usize = 2;
    const FIRST_REVISION: u64 = 1;
    const SECOND_REVISION: u64 = 2;
    const THIRD_REVISION: u64 = 3;
    const TEST_WAIT_SECONDS: u64 = 2;

    #[test]
    fn resume_before_wait_uses_state_predicate_without_lost_wakeup() {
        // r[verify agent_harness_runtime.agent_control.host.resume_race]
        let hub = hub();
        let publisher = hub.publisher();
        publish(
            &publisher,
            PAUSE_ID,
            animus_adapters::AgentControlAction::Pause,
        )
        .expect("pause must publish");
        publish(
            &publisher,
            RESUME_ID,
            animus_adapters::AgentControlAction::ResumeRun,
        )
        .expect("resume must publish");
        let mut waiter = hub.waiter();

        assert_eq!(
            waiter.wait_until_runnable(&checkpoint(RUN_ID)),
            Ok(animus_adapters::AgentControlPermit::Continue)
        );
        assert_eq!(
            hub.snapshot().expect("snapshot must succeed").phase(),
            animus_adapters::AgentControlPhase::Running
        );
    }

    #[test]
    fn paused_waiter_blocks_until_resume_and_publishes_checkpoint_event() {
        // r[verify agent_harness_runtime.agent_control.host]
        // r[verify agent_harness_runtime.agent_control.verification.positive]
        let hub = hub();
        let publisher = hub.publisher();
        let subscriber = hub.subscriber();
        publish(
            &publisher,
            PAUSE_ID,
            animus_adapters::AgentControlAction::Pause,
        )
        .expect("pause must publish");
        let mut waiter = hub.waiter();
        let (sender, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let outcome = waiter.wait_until_runnable(&checkpoint(RUN_ID));
            sender.send(outcome).expect("receiver must remain open");
        });

        let paused_event = subscriber
            .wait_after(FIRST_REVISION)
            .expect("paused event must publish");
        assert!(matches!(
            paused_event,
            AgentControlSubscriptionOutcome::Event(ref event)
                if event.kind() == animus_adapters::AgentControlEventKind::Paused
                    && event.revision() == SECOND_REVISION
        ));
        publish(
            &publisher,
            RESUME_ID,
            animus_adapters::AgentControlAction::ResumeRun,
        )
        .expect("resume must publish");
        assert_eq!(
            receiver
                .recv_timeout(Duration::from_secs(TEST_WAIT_SECONDS))
                .expect("waiter must wake"),
            Ok(animus_adapters::AgentControlPermit::Continue)
        );
        worker.join().expect("worker must not panic");
    }

    #[test]
    fn cancellation_wakes_paused_waiter_and_prevents_resume() {
        // r[verify agent_harness_runtime.agent_control.concurrency.cancel]
        let hub = hub();
        let publisher = hub.publisher();
        let subscriber = hub.subscriber();
        publish(
            &publisher,
            PAUSE_ID,
            animus_adapters::AgentControlAction::Pause,
        )
        .expect("pause must publish");
        let mut waiter = hub.waiter();
        let (sender, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let outcome = waiter.wait_until_runnable(&checkpoint(RUN_ID));
            sender.send(outcome).expect("receiver must remain open");
        });
        let paused = subscriber
            .wait_after(FIRST_REVISION)
            .expect("paused event must publish");
        assert!(matches!(paused, AgentControlSubscriptionOutcome::Event(_)));

        publish(
            &publisher,
            CANCEL_ID,
            animus_adapters::AgentControlAction::Cancel,
        )
        .expect("cancel must publish");
        assert_eq!(
            receiver
                .recv_timeout(Duration::from_secs(TEST_WAIT_SECONDS))
                .expect("waiter must wake"),
            Ok(animus_adapters::AgentControlPermit::Cancel)
        );
        assert!(matches!(
            publish(
                &publisher,
                RESUME_ID,
                animus_adapters::AgentControlAction::ResumeRun,
            ),
            Err(AgentControlHostError::Reduction(
                animus_runtime::AgentControlReductionError::Cancelling
            ))
        ));
        worker.join().expect("worker must not panic");
    }

    #[test]
    fn stale_commands_wrong_runs_and_lagging_subscribers_fail_explicitly() {
        // r[verify agent_harness_runtime.agent_control.host.lag]
        // r[verify agent_harness_runtime.agent_control.verification.negative]
        let hub = hub();
        let publisher = hub.publisher();
        let subscriber = hub.subscriber();
        let stale = animus_adapters::AgentControlCommand::new(
            run_id(RUN_ID),
            command_id(PAUSE_ID),
            FIRST_REVISION,
            animus_adapters::AgentControlAction::Pause,
        );
        assert!(matches!(
            publisher.publish(&stale),
            Err(AgentControlHostError::Reduction(
                animus_runtime::AgentControlReductionError::StaleRevision { .. }
            ))
        ));
        assert_eq!(
            hub.waiter().wait_until_runnable(&checkpoint(OTHER_RUN_ID)),
            Err(animus_adapters::AgentControlWaitError::RunMismatch)
        );

        publish(
            &publisher,
            PAUSE_ID,
            animus_adapters::AgentControlAction::Pause,
        )
        .expect("pause must publish");
        publish(
            &publisher,
            RESUME_ID,
            animus_adapters::AgentControlAction::ResumeRun,
        )
        .expect("resume must publish");
        publish(
            &publisher,
            SECOND_PAUSE_ID,
            animus_adapters::AgentControlAction::Pause,
        )
        .expect("second pause must publish");
        assert_eq!(
            subscriber.next_after(0).expect("read must succeed"),
            AgentControlSubscriptionOutcome::Lagged {
                oldest_available_revision: SECOND_REVISION,
            }
        );
        assert_eq!(
            hub.snapshot().expect("snapshot must succeed").revision(),
            THIRD_REVISION
        );
    }

    #[test]
    fn explicit_close_wakes_waiters_without_running_fallback() {
        // r[verify agent_harness_runtime.agent_control.concurrency.close]
        let hub = hub();
        let publisher = hub.publisher();
        let subscriber = hub.subscriber();
        publish(
            &publisher,
            PAUSE_ID,
            animus_adapters::AgentControlAction::Pause,
        )
        .expect("pause must publish");
        let mut waiter = hub.waiter();
        let (sender, receiver) = mpsc::channel();
        let worker = std::thread::spawn(move || {
            let outcome = waiter.wait_until_runnable(&checkpoint(RUN_ID));
            sender.send(outcome).expect("receiver must remain open");
        });
        let paused = subscriber
            .wait_after(FIRST_REVISION)
            .expect("paused event must publish");
        assert!(matches!(paused, AgentControlSubscriptionOutcome::Event(_)));
        let closed = hub.close().expect("close must succeed");

        assert!(closed.is_closed());
        assert_eq!(
            receiver
                .recv_timeout(Duration::from_secs(TEST_WAIT_SECONDS))
                .expect("waiter must wake"),
            Err(animus_adapters::AgentControlWaitError::Closed)
        );
        assert_eq!(
            publisher.publish(&animus_adapters::AgentControlCommand::new(
                run_id(RUN_ID),
                command_id(RESUME_ID),
                closed.revision(),
                animus_adapters::AgentControlAction::ResumeRun,
            )),
            Err(AgentControlHostError::Closed)
        );
        worker.join().expect("worker must not panic");
    }

    fn hub() -> AgentControlHub {
        AgentControlHub::new(
            run_id(RUN_ID),
            NonZeroUsize::new(EVENT_CAPACITY).expect("capacity must be non-zero"),
        )
    }

    fn publish(
        publisher: &AgentControlPublisher,
        id: &str,
        action: animus_adapters::AgentControlAction,
    ) -> Result<AgentControlAcknowledgement, AgentControlHostError> {
        let snapshot = publisher.snapshot()?;
        publisher.publish(&animus_adapters::AgentControlCommand::new(
            snapshot.run_id().clone(),
            command_id(id),
            snapshot.revision(),
            action,
        ))
    }

    fn run_id(value: &str) -> animus_adapters::AgentControlRunId {
        animus_adapters::AgentControlRunId::new(value).expect("run id must validate")
    }

    fn command_id(value: &str) -> animus_adapters::AgentControlCommandId {
        animus_adapters::AgentControlCommandId::new(value).expect("command id must validate")
    }

    fn checkpoint(run: &str) -> animus_adapters::AgentControlCheckpoint {
        animus_adapters::AgentControlCheckpoint::new(
            run_id(run),
            animus_adapters::AgentControlCheckpointKind::BeforeModel,
        )
    }
}
