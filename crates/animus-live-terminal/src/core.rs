//! Pure terminal-client state, command, and evidence contracts.

use serde::{Deserialize, Serialize};

const RECEIPT_SCHEMA: &str = "animus.live-terminal-receipt.v1";
const RECEIPT_DOMAIN: &str = "animus.live-terminal-receipt.identity.v1";
const COMMAND_DOMAIN: &str = "animus.live-terminal-command.v1";
const HASH_SEPARATOR: &[u8] = b"\0";
const NON_CLAIMS: &[&str] = &[
    "not-arbitrary-terminal-compatibility",
    "not-remote-security",
    "not-crash-proof-restoration",
    "not-pty-migration",
    "not-security-sandbox",
    "not-production-readiness",
    "not-release-eligibility",
];
const BLAKE3_HEX_CHARS: usize = 64;
const MAX_LABEL_BYTES: usize = 256;
const CONNECTION_DAEMON: &str = "daemon";
const CONNECTION_DIRECT: &str = "direct";

/// Terminal input normalized before pure reduction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalInput {
    /// Insert one character into the bounded prompt.
    Character(char),
    /// Remove the previous prompt character.
    Backspace,
    /// Submit the current prompt.
    Submit,
    /// Request correlated session cancellation.
    Cancel,
    /// Request client shutdown without stopping worker execution.
    Quit,
    /// Move prompt cursor left.
    Left,
    /// Move prompt cursor right.
    Right,
    /// Move prompt cursor to its first position.
    Home,
    /// Move prompt cursor after its final character.
    End,
    /// Record an admitted terminal resize.
    Resize {
        /// Width in cells.
        width: u16,
        /// Height in cells.
        height: u16,
    },
}

/// Approval decision produced by a UI control.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ApprovalDecision {
    /// Approve the pending operation.
    Approve,
    /// Deny the pending operation.
    Deny,
}

/// Child-session control produced by a UI control.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChildAction {
    /// Inspect bounded child status.
    Inspect,
    /// Request child cancellation through the session owner.
    Cancel,
}

/// Session control produced by a UI control.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SessionAction {
    /// Request a fresh private snapshot through the selected API.
    RefreshSnapshot,
    /// Detach this client without stopping worker execution.
    Detach,
    /// Request authorized worker stop.
    Stop,
}

/// Typed descriptor sent through a selected client API.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ClientCommand {
    /// Submit private prompt text to the session owner.
    Prompt {
        /// Correlation identity.
        correlation_id: String,
        /// Private prompt text. Receipts omit it.
        text: String,
    },
    /// Edit one queued item with exact compare-and-swap facts.
    QueueEdit {
        /// Correlation identity.
        correlation_id: String,
        /// Queue item identity.
        item_id: String,
        /// Expected queue generation.
        expected_generation: u64,
        /// Expected queue BLAKE3 identity.
        expected_queue_identity: String,
        /// Private replacement text. Receipts omit it.
        replacement: String,
    },
    /// Remove one queued item with exact compare-and-swap facts.
    QueueRemove {
        /// Correlation identity.
        correlation_id: String,
        /// Queue item identity.
        item_id: String,
        /// Expected queue generation.
        expected_generation: u64,
        /// Expected queue BLAKE3 identity.
        expected_queue_identity: String,
    },
    /// Answer one pending approval request.
    Approval {
        /// Correlation identity from the request.
        correlation_id: String,
        /// Explicit approval decision.
        decision: ApprovalDecision,
    },
    /// Request correlated session cancellation.
    Cancel {
        /// Correlation identity.
        correlation_id: String,
    },
    /// Request an explicit worker-authorized kernel clear.
    KernelClear {
        /// Correlation identity.
        correlation_id: String,
    },
    /// Request bounded metadata-only worker state inspection.
    StateInspection {
        /// Correlation identity.
        correlation_id: String,
    },
    /// Route one child control through the session owner.
    Child {
        /// Correlation identity.
        correlation_id: String,
        /// Child identity.
        child_id: String,
        /// Requested child action.
        action: ChildAction,
    },
    /// Route one session control through the selected client API.
    Session {
        /// Correlation identity.
        correlation_id: String,
        /// Requested session action.
        action: SessionAction,
    },
}

impl ClientCommand {
    /// Returns the safe command class.
    #[must_use]
    pub const fn class(&self) -> &'static str {
        match self {
            Self::Prompt { .. } => "prompt",
            Self::QueueEdit { .. } => "queue_edit",
            Self::QueueRemove { .. } => "queue_remove",
            Self::Approval { .. } => "approval",
            Self::Cancel { .. } => "cancel",
            Self::KernelClear { .. } => "kernel_clear",
            Self::StateInspection { .. } => "state_inspection",
            Self::Child { .. } => "child",
            Self::Session { .. } => "session",
        }
    }

    /// Returns the command correlation identity.
    #[must_use]
    pub fn correlation_id(&self) -> &str {
        match self {
            Self::Prompt { correlation_id, .. }
            | Self::QueueEdit { correlation_id, .. }
            | Self::QueueRemove { correlation_id, .. }
            | Self::Approval { correlation_id, .. }
            | Self::Cancel { correlation_id }
            | Self::KernelClear { correlation_id }
            | Self::StateInspection { correlation_id }
            | Self::Child { correlation_id, .. }
            | Self::Session { correlation_id, .. } => correlation_id.as_str(),
        }
    }
}

/// Explicit UI control intent before command construction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ControlIntent {
    /// Edit one queued item.
    QueueEdit {
        /// Queue item identity.
        item_id: String,
        /// Expected queue generation.
        expected_generation: u64,
        /// Expected queue BLAKE3 identity.
        expected_queue_identity: String,
        /// Private replacement text.
        replacement: String,
    },
    /// Remove one queued item.
    QueueRemove {
        /// Queue item identity.
        item_id: String,
        /// Expected queue generation.
        expected_generation: u64,
        /// Expected queue BLAKE3 identity.
        expected_queue_identity: String,
    },
    /// Answer one approval.
    Approval {
        /// Approval correlation identity.
        correlation_id: String,
        /// Explicit decision.
        decision: ApprovalDecision,
    },
    /// Request cancellation.
    Cancel,
    /// Request an explicit worker-authorized kernel clear.
    KernelClear,
    /// Request bounded metadata-only worker state inspection.
    StateInspection,
    /// Control one child.
    Child {
        /// Child identity.
        child_id: String,
        /// Child action.
        action: ChildAction,
    },
    /// Control the current session.
    Session(SessionAction),
}

/// Safe event projected from the selected session API.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ClientSessionEvent {
    /// Apply one existing runtime event through accepted TUI reducers.
    Runtime(animus_runtime::RuntimeEvent),
    /// Record safe metadata from a daemon worker event.
    Observation {
        /// Monotonic event sequence.
        sequence: u64,
        /// Safe event class.
        class: String,
        /// BLAKE3 observation identity.
        observation_identity: String,
    },
    /// Record one typed metadata-only worker state inspection.
    StateInspection {
        /// Monotonic event sequence.
        sequence: u64,
        /// BLAKE3 identity of the state-inspection request.
        correlation_id: String,
        /// BLAKE3 observation identity.
        observation_identity: String,
        /// Bounded worker state.
        state: animus_local_daemon::WorkerStateInspection,
    },
    /// Record one pending approval correlation.
    ApprovalRequested {
        /// Approval correlation identity.
        correlation_id: String,
    },
    /// Record a queue compare-and-swap conflict.
    QueueConflict {
        /// Current queue generation.
        generation: u64,
        /// Current queue BLAKE3 identity.
        queue_identity: String,
    },
    /// Record bounded child state.
    ChildState {
        /// Child identity.
        child_id: String,
        /// Safe child status.
        status: String,
    },
    /// Record a typed connection loss.
    Disconnected,
    /// Record a bounded safe client error class.
    Failed {
        /// Safe error class.
        class: String,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct StateInspectionObservation<'a> {
    sequence: u64,
    correlation_id: &'a str,
    observation_identity: &'a str,
    state: &'a animus_local_daemon::WorkerStateInspection,
}

impl<'a> StateInspectionObservation<'a> {
    fn from_event(event: &'a ClientSessionEvent) -> Result<Self, TerminalCoreError> {
        let ClientSessionEvent::StateInspection {
            sequence,
            correlation_id,
            observation_identity,
            state,
        } = event
        else {
            return Err(TerminalCoreError::Identity);
        };
        Ok(Self {
            sequence: *sequence,
            correlation_id,
            observation_identity,
            state,
        })
    }
}

/// Snapshot returned before later event polling.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ClientSnapshot {
    /// Root identity.
    pub root_id: String,
    /// Worker identity.
    pub worker_id: String,
    /// Worker generation.
    pub generation: u64,
    /// Latest observed sequence.
    pub event_sequence: u64,
    /// Safe restore status.
    pub restore_status: animus_local_daemon::RestoreStatus,
    /// Initial bounded view events.
    pub events: Vec<ClientSessionEvent>,
}

/// Existing TUI projections plus bounded terminal client state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalViewState {
    coding_session: animus_tui::CodingSessionRenderModel,
    conversation: animus_tui::ConversationRenderModel,
    tools: animus_tui::ToolRenderModel,
    prompt: animus_tui::PromptInputState,
    lifecycle_status: String,
    child_status: Option<(String, String)>,
    queue_conflict: Option<(u64, String)>,
    pending_approval: Option<String>,
    state_inspection: Option<animus_local_daemon::WorkerStateInspection>,
    viewport: (u16, u16),
}

impl TerminalViewState {
    fn new(viewport: (u16, u16)) -> Self {
        Self {
            coding_session: animus_tui::CodingSessionRenderModel::new(),
            conversation: animus_tui::ConversationRenderModel::new(),
            tools: animus_tui::ToolRenderModel::new(),
            prompt: animus_tui::PromptInputState::new(),
            lifecycle_status: String::from("connecting"),
            child_status: None,
            queue_conflict: None,
            pending_approval: None,
            state_inspection: None,
            viewport,
        }
    }

    fn apply_runtime_event(&mut self, event: &animus_runtime::RuntimeEvent) {
        self.coding_session.apply_runtime_event(event);
        self.conversation.apply_runtime_event(event);
        self.tools.apply_runtime_event(event);
        event.kind().as_str().clone_into(&mut self.lifecycle_status);
    }

    /// Returns the existing coding-session projection.
    #[must_use]
    pub const fn coding_session(&self) -> &animus_tui::CodingSessionRenderModel {
        &self.coding_session
    }

    /// Returns the existing conversation projection.
    #[must_use]
    pub const fn conversation(&self) -> &animus_tui::ConversationRenderModel {
        &self.conversation
    }

    /// Returns the existing tool projection.
    #[must_use]
    pub const fn tools(&self) -> &animus_tui::ToolRenderModel {
        &self.tools
    }

    /// Returns current private prompt state for rendering only.
    #[must_use]
    pub const fn prompt(&self) -> &animus_tui::PromptInputState {
        &self.prompt
    }

    /// Returns safe lifecycle status.
    #[must_use]
    pub fn lifecycle_status(&self) -> &str {
        self.lifecycle_status.as_str()
    }

    /// Returns the last bounded child status.
    #[must_use]
    pub fn child_status(&self) -> Option<(&str, &str)> {
        self.child_status
            .as_ref()
            .map(|(identity, status)| (identity.as_str(), status.as_str()))
    }

    /// Returns current queue conflict metadata.
    #[must_use]
    pub fn queue_conflict(&self) -> Option<(u64, &str)> {
        self.queue_conflict
            .as_ref()
            .map(|(generation, identity)| (*generation, identity.as_str()))
    }

    /// Returns the latest metadata-only worker state inspection.
    #[must_use]
    pub const fn state_inspection(&self) -> Option<&animus_local_daemon::WorkerStateInspection> {
        self.state_inspection.as_ref()
    }

    /// Returns the current viewport.
    #[must_use]
    pub const fn viewport(&self) -> (u16, u16) {
        self.viewport
    }
}

/// Pure state for one attached terminal client.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalClientState {
    root_id: String,
    worker_id: String,
    generation: u64,
    restore_status: animus_local_daemon::RestoreStatus,
    last_sequence: u64,
    command_index: u64,
    view: TerminalViewState,
    dirty: bool,
    shutdown_requested: bool,
    input_count: u64,
    event_count: u64,
    command_count: u64,
    render_count: u64,
}

impl TerminalClientState {
    /// Creates state from one validated snapshot before event polling.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalCoreError`] for invalid identities, dimensions, or snapshot events.
    pub fn from_snapshot(
        snapshot: &ClientSnapshot,
        viewport: (u16, u16),
        limits: TerminalLimits,
    ) -> Result<Self, TerminalCoreError> {
        validate_label(snapshot.root_id.as_str())?;
        validate_label(snapshot.worker_id.as_str())?;
        if snapshot.generation == 0
            || snapshot.restore_status == animus_local_daemon::RestoreStatus::Blocked
        {
            return Err(TerminalCoreError::Snapshot);
        }
        validate_viewport(viewport, limits)?;
        let mut state = Self {
            root_id: snapshot.root_id.clone(),
            worker_id: snapshot.worker_id.clone(),
            generation: snapshot.generation,
            restore_status: snapshot.restore_status,
            last_sequence: snapshot.event_sequence,
            command_index: 0,
            view: TerminalViewState::new(viewport),
            dirty: true,
            shutdown_requested: false,
            input_count: 0,
            event_count: 0,
            command_count: 0,
            render_count: 0,
        };
        state.view.lifecycle_status = String::from("attached");
        apply_event_batch(&mut state, snapshot.events.as_slice(), limits.event_batch)?;
        Ok(state)
    }

    /// Applies one generation-stable refreshed snapshot to the existing view.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalCoreError`] for identity drift, cursor drift, blocked restore,
    /// or malformed refreshed events.
    pub fn apply_refresh(
        &mut self,
        snapshot: &ClientSnapshot,
        maximum_events: u32,
    ) -> Result<(), TerminalCoreError> {
        if snapshot.root_id != self.root_id
            || snapshot.worker_id != self.worker_id
            || snapshot.generation != self.generation
            || snapshot.event_sequence != self.last_sequence
            || snapshot.restore_status == animus_local_daemon::RestoreStatus::Blocked
        {
            return Err(TerminalCoreError::Snapshot);
        }
        self.restore_status = snapshot.restore_status;
        apply_event_batch(self, snapshot.events.as_slice(), maximum_events)?;
        self.dirty = true;
        Ok(())
    }

    /// Returns the current view projection.
    #[must_use]
    pub const fn view(&self) -> &TerminalViewState {
        &self.view
    }

    /// Returns the latest event sequence.
    #[must_use]
    pub const fn last_sequence(&self) -> u64 {
        self.last_sequence
    }

    /// Records a completed render after the shell performs it.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalCoreError::Overflow`] when the render count cannot advance.
    pub fn record_rendered(&mut self) -> Result<(), TerminalCoreError> {
        self.render_count = self
            .render_count
            .checked_add(1)
            .ok_or(TerminalCoreError::Overflow)?;
        self.dirty = false;
        Ok(())
    }

    /// Returns whether rendering is needed.
    #[must_use]
    pub const fn is_dirty(&self) -> bool {
        self.dirty
    }

    /// Returns whether client shutdown was requested.
    #[must_use]
    pub const fn shutdown_requested(&self) -> bool {
        self.shutdown_requested
    }

    /// Projects metadata-only completion evidence after cleanup.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalCoreError`] when cleanup is incomplete or counts overflow.
    pub fn receipt(
        &self,
        facts: TerminalReceiptFacts<'_>,
    ) -> Result<LiveTerminalReceipt, TerminalCoreError> {
        validate_hash(facts.config_identity)?;
        if !matches!(facts.connection, CONNECTION_DAEMON | CONNECTION_DIRECT) {
            return Err(TerminalCoreError::Identity);
        }
        facts.cleanup.validate_complete()?;
        let (width, height) = self.view.viewport;
        let material = format!(
            "{RECEIPT_DOMAIN}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}\0{}",
            facts.config_identity,
            self.root_id,
            self.worker_id,
            self.generation,
            facts.connection,
            facts.exit.as_str(),
            self.input_count,
            self.event_count,
            self.command_count,
            self.render_count,
            width,
            height,
            facts.cleanup.completed_transitions,
            facts.cleanup.attempted_actions,
            facts.cleanup.completed_actions,
            facts.cleanup.failed_actions,
        );
        Ok(LiveTerminalReceipt {
            schema: RECEIPT_SCHEMA,
            receipt_identity: blake3::hash(material.as_bytes()).to_hex().to_string(),
            config_identity: String::from(facts.config_identity),
            root_identity: blake3::hash(self.root_id.as_bytes()).to_hex().to_string(),
            worker_identity: blake3::hash(self.worker_id.as_bytes()).to_hex().to_string(),
            generation: self.generation,
            connection: facts.connection,
            exit: facts.exit,
            viewport_width: width,
            viewport_height: height,
            input_count: self.input_count,
            event_count: self.event_count,
            command_count: self.command_count,
            render_count: self.render_count,
            cleanup: facts.cleanup,
            non_claims: NON_CLAIMS,
        })
    }

    fn next_correlation(&mut self, class: &str) -> Result<String, TerminalCoreError> {
        self.command_index = self
            .command_index
            .checked_add(1)
            .ok_or(TerminalCoreError::Overflow)?;
        let mut hasher = blake3::Hasher::new();
        hasher.update(COMMAND_DOMAIN.as_bytes());
        hasher.update(HASH_SEPARATOR);
        hasher.update(self.root_id.as_bytes());
        hasher.update(HASH_SEPARATOR);
        hasher.update(self.generation.to_string().as_bytes());
        hasher.update(HASH_SEPARATOR);
        hasher.update(self.command_index.to_string().as_bytes());
        hasher.update(HASH_SEPARATOR);
        hasher.update(class.as_bytes());
        Ok(hasher.finalize().to_hex().to_string())
    }

    fn record_command(&mut self) -> Result<(), TerminalCoreError> {
        self.command_count = self
            .command_count
            .checked_add(1)
            .ok_or(TerminalCoreError::Overflow)?;
        Ok(())
    }
}

/// Explicit facts needed to project one metadata-only terminal receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalReceiptFacts<'a> {
    /// Validated terminal configuration identity.
    pub config_identity: &'a str,
    /// Selected explicit connection class.
    pub connection: &'static str,
    /// Complete terminal cleanup observation.
    pub cleanup: CleanupObservation,
    /// Terminal exit class.
    pub exit: TerminalExit,
}

/// Named finite event-loop limits.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TerminalLimits {
    /// Maximum session events in one loop iteration.
    pub event_batch: u32,
    /// Maximum private prompt bytes.
    pub prompt_bytes: u32,
    /// Maximum viewport width.
    pub width: u16,
    /// Maximum viewport height.
    pub height: u16,
}

/// One explicit event-loop iteration input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalIteration<'events> {
    /// At most one terminal input observation.
    pub input: Option<TerminalInput>,
    /// One bounded session event batch.
    pub session_events: &'events [ClientSessionEvent],
    /// Whether caller cancellation was observed.
    pub cancelled: bool,
    /// Whether a supported signal was observed.
    pub signalled: bool,
}

/// Pure plan for shell effects after one iteration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TerminalIterationPlan {
    /// At most one command to route through the selected connection.
    pub command: Option<ClientCommand>,
    /// Whether the shell must render the updated view.
    pub render: bool,
    /// Terminal exit class when this iteration ends the client.
    pub exit: Option<TerminalExit>,
}

/// Reduces one bounded loop iteration.
///
/// # Errors
///
/// Returns [`TerminalCoreError`] for event floods, malformed facts, bounds, or overflow.
pub fn reduce_iteration(
    state: &mut TerminalClientState,
    iteration: &TerminalIteration<'_>,
    limits: TerminalLimits,
) -> Result<TerminalIterationPlan, TerminalCoreError> {
    if iteration.session_events.len()
        > usize::try_from(limits.event_batch).map_err(|_error| TerminalCoreError::Bound)?
    {
        return Err(TerminalCoreError::EventFlood);
    }
    let mut command = None;
    if let Some(input) = iteration.input {
        state.input_count = state
            .input_count
            .checked_add(1)
            .ok_or(TerminalCoreError::Overflow)?;
        command = reduce_input(state, input, limits)?;
    }
    apply_event_batch(state, iteration.session_events, limits.event_batch)?;
    if command.is_none() && state.view.queue_conflict.is_some() {
        let correlation_id = state.next_correlation("refresh_snapshot")?;
        command = Some(ClientCommand::Session {
            correlation_id,
            action: SessionAction::RefreshSnapshot,
        });
        state.view.queue_conflict = None;
    }
    if let Some(command) = &command {
        validate_command(command, limits.prompt_bytes)?;
        state.record_command()?;
    }
    let exit = if iteration.signalled {
        state.shutdown_requested = true;
        Some(TerminalExit::Signal)
    } else if iteration.cancelled {
        state.shutdown_requested = true;
        Some(TerminalExit::Cancelled)
    } else if iteration
        .session_events
        .iter()
        .any(|event| matches!(event, ClientSessionEvent::Disconnected))
    {
        state.shutdown_requested = true;
        Some(TerminalExit::Disconnected)
    } else if state.shutdown_requested {
        Some(TerminalExit::Normal)
    } else {
        None
    };
    Ok(TerminalIterationPlan {
        command,
        render: state.dirty,
        exit,
    })
}

/// Converts one explicit UI control into a command descriptor without executing it.
///
/// # Errors
///
/// Returns [`TerminalCoreError`] for malformed identities, hashes, zero generations, or bounds.
pub fn command_from_control(
    state: &mut TerminalClientState,
    intent: ControlIntent,
    max_prompt_bytes: u32,
) -> Result<ClientCommand, TerminalCoreError> {
    let command = match intent {
        ControlIntent::QueueEdit {
            item_id,
            expected_generation,
            expected_queue_identity,
            replacement,
        } => ClientCommand::QueueEdit {
            correlation_id: state.next_correlation("queue_edit")?,
            item_id,
            expected_generation,
            expected_queue_identity,
            replacement,
        },
        ControlIntent::QueueRemove {
            item_id,
            expected_generation,
            expected_queue_identity,
        } => ClientCommand::QueueRemove {
            correlation_id: state.next_correlation("queue_remove")?,
            item_id,
            expected_generation,
            expected_queue_identity,
        },
        ControlIntent::Approval {
            correlation_id,
            decision,
        } => ClientCommand::Approval {
            correlation_id,
            decision,
        },
        ControlIntent::Cancel => ClientCommand::Cancel {
            correlation_id: state.next_correlation("cancel")?,
        },
        ControlIntent::KernelClear => ClientCommand::KernelClear {
            correlation_id: state.next_correlation("kernel_clear")?,
        },
        ControlIntent::StateInspection => ClientCommand::StateInspection {
            correlation_id: state.next_correlation("state_inspection")?,
        },
        ControlIntent::Child { child_id, action } => ClientCommand::Child {
            correlation_id: state.next_correlation("child")?,
            child_id,
            action,
        },
        ControlIntent::Session(action) => ClientCommand::Session {
            correlation_id: state.next_correlation("session")?,
            action,
        },
    };
    validate_command(&command, max_prompt_bytes)?;
    state.record_command()?;
    Ok(command)
}

fn reduce_input(
    state: &mut TerminalClientState,
    input: TerminalInput,
    limits: TerminalLimits,
) -> Result<Option<ClientCommand>, TerminalCoreError> {
    match input {
        TerminalInput::Character(character) => {
            let next_bytes = state
                .view
                .prompt
                .content()
                .len()
                .checked_add(character.len_utf8())
                .ok_or(TerminalCoreError::Overflow)?;
            if next_bytes
                > usize::try_from(limits.prompt_bytes).map_err(|_error| TerminalCoreError::Bound)?
            {
                return Err(TerminalCoreError::Bound);
            }
            state.view.prompt.type_char(character);
            state.dirty = true;
            Ok(None)
        }
        TerminalInput::Backspace => {
            state.view.prompt.backspace();
            state.dirty = true;
            Ok(None)
        }
        TerminalInput::Left => {
            state.view.prompt.move_left();
            state.dirty = true;
            Ok(None)
        }
        TerminalInput::Right => {
            state.view.prompt.move_right();
            state.dirty = true;
            Ok(None)
        }
        TerminalInput::Home => {
            state.view.prompt.move_home();
            state.dirty = true;
            Ok(None)
        }
        TerminalInput::End => {
            state.view.prompt.move_end();
            state.dirty = true;
            Ok(None)
        }
        TerminalInput::Submit => {
            let text = state.view.prompt.submit();
            state.dirty = true;
            if text.is_empty() {
                return Ok(None);
            }
            Ok(Some(ClientCommand::Prompt {
                correlation_id: state.next_correlation("prompt")?,
                text,
            }))
        }
        TerminalInput::Cancel => Ok(Some(ClientCommand::Cancel {
            correlation_id: state.next_correlation("cancel")?,
        })),
        TerminalInput::Quit => {
            state.shutdown_requested = true;
            Ok(None)
        }
        TerminalInput::Resize { width, height } => {
            validate_viewport((width, height), limits)?;
            state.view.viewport = (width, height);
            state.dirty = true;
            Ok(None)
        }
    }
}

fn apply_event_batch(
    state: &mut TerminalClientState,
    events: &[ClientSessionEvent],
    maximum: u32,
) -> Result<(), TerminalCoreError> {
    if events.len() > usize::try_from(maximum).map_err(|_error| TerminalCoreError::Bound)? {
        return Err(TerminalCoreError::EventFlood);
    }
    for event in events {
        match event {
            ClientSessionEvent::Runtime(runtime) => state.view.apply_runtime_event(runtime),
            ClientSessionEvent::Observation {
                sequence,
                class,
                observation_identity,
            } => {
                if *sequence <= state.last_sequence {
                    return Err(TerminalCoreError::Sequence);
                }
                validate_label(class.as_str())?;
                validate_hash(observation_identity.as_str())?;
                state.last_sequence = *sequence;
                state.view.lifecycle_status.clone_from(class);
            }
            ClientSessionEvent::StateInspection { .. } => {
                apply_state_inspection(state, StateInspectionObservation::from_event(event)?)?;
            }
            ClientSessionEvent::ApprovalRequested { correlation_id } => {
                validate_label(correlation_id.as_str())?;
                state.view.pending_approval = Some(correlation_id.clone());
                state.view.lifecycle_status = String::from("approval_requested");
            }
            ClientSessionEvent::QueueConflict {
                generation,
                queue_identity,
            } => {
                if *generation == 0 {
                    return Err(TerminalCoreError::Sequence);
                }
                validate_hash(queue_identity.as_str())?;
                state.view.queue_conflict = Some((*generation, queue_identity.clone()));
                state.view.lifecycle_status = String::from("queue_conflict");
            }
            ClientSessionEvent::ChildState { child_id, status } => {
                validate_label(child_id.as_str())?;
                validate_label(status.as_str())?;
                state.view.child_status = Some((child_id.clone(), status.clone()));
            }
            ClientSessionEvent::Disconnected => {
                state.view.lifecycle_status = String::from("disconnected");
            }
            ClientSessionEvent::Failed { class } => {
                validate_label(class.as_str())?;
                state.view.lifecycle_status.clone_from(class);
            }
        }
        state.event_count = state
            .event_count
            .checked_add(1)
            .ok_or(TerminalCoreError::Overflow)?;
        state.dirty = true;
    }
    Ok(())
}

fn apply_state_inspection(
    state: &mut TerminalClientState,
    observation: StateInspectionObservation<'_>,
) -> Result<(), TerminalCoreError> {
    observation
        .state
        .validate()
        .map_err(|_error| TerminalCoreError::Identity)?;
    validate_state_inspection_context(state, observation)?;
    validate_state_inspection_identity(state.root_id.as_str(), observation)?;
    state.last_sequence = observation.sequence;
    state.view.state_inspection = Some(observation.state.clone());
    state.view.lifecycle_status = String::from("state_inspection");
    Ok(())
}

fn validate_state_inspection_context(
    state: &TerminalClientState,
    observation: StateInspectionObservation<'_>,
) -> Result<(), TerminalCoreError> {
    if observation.sequence <= state.last_sequence {
        return Err(TerminalCoreError::Sequence);
    }
    if observation.state.worker_generation != state.generation
        || observation.state.restore_status != state.restore_status
    {
        return Err(TerminalCoreError::Sequence);
    }
    if let Some(current) = state.view.state_inspection.as_ref()
        && observation.state.queue_generation < current.queue_generation
    {
        return Err(TerminalCoreError::Sequence);
    }
    Ok(())
}

fn validate_state_inspection_identity(
    root_id: &str,
    observation: StateInspectionObservation<'_>,
) -> Result<(), TerminalCoreError> {
    validate_hash(observation.correlation_id)?;
    validate_hash(observation.observation_identity)?;
    let expected_identity = observation
        .state
        .observation_identity(animus_local_daemon::StateInspectionIdentityFacts {
            root_id,
            sequence: observation.sequence,
            correlation_id: observation.correlation_id,
        })
        .map_err(|_error| TerminalCoreError::Identity)?;
    if observation.observation_identity != expected_identity {
        return Err(TerminalCoreError::Identity);
    }
    Ok(())
}

pub(crate) fn validate_command(
    command: &ClientCommand,
    max_prompt_bytes: u32,
) -> Result<(), TerminalCoreError> {
    validate_hash(command.correlation_id())?;
    match command {
        ClientCommand::Prompt { text, .. } => validate_private_text(text, max_prompt_bytes),
        ClientCommand::QueueEdit {
            item_id,
            expected_generation,
            expected_queue_identity,
            replacement,
            ..
        } => {
            validate_label(item_id)?;
            if *expected_generation == 0 {
                return Err(TerminalCoreError::Command);
            }
            validate_hash(expected_queue_identity)?;
            validate_private_text(replacement, max_prompt_bytes)
        }
        ClientCommand::QueueRemove {
            item_id,
            expected_generation,
            expected_queue_identity,
            ..
        } => {
            validate_label(item_id)?;
            if *expected_generation == 0 {
                return Err(TerminalCoreError::Command);
            }
            validate_hash(expected_queue_identity)
        }
        ClientCommand::Approval { .. }
        | ClientCommand::Cancel { .. }
        | ClientCommand::KernelClear { .. }
        | ClientCommand::StateInspection { .. }
        | ClientCommand::Session { .. } => Ok(()),
        ClientCommand::Child { child_id, .. } => validate_label(child_id),
    }
}

fn validate_private_text(value: &str, maximum: u32) -> Result<(), TerminalCoreError> {
    if value.is_empty()
        || value.len() > usize::try_from(maximum).map_err(|_error| TerminalCoreError::Bound)?
        || value.contains('\0')
    {
        return Err(TerminalCoreError::Command);
    }
    Ok(())
}

fn validate_label(value: &str) -> Result<(), TerminalCoreError> {
    if value.is_empty()
        || value.len() > MAX_LABEL_BYTES
        || value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return Err(TerminalCoreError::Identity);
    }
    Ok(())
}

fn validate_hash(value: &str) -> Result<(), TerminalCoreError> {
    if value.len() != BLAKE3_HEX_CHARS || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(TerminalCoreError::Identity);
    }
    Ok(())
}

fn validate_viewport(
    viewport: (u16, u16),
    limits: TerminalLimits,
) -> Result<(), TerminalCoreError> {
    let (width, height) = viewport;
    if width == 0 || width > limits.width || height == 0 || height > limits.height {
        return Err(TerminalCoreError::Bound);
    }
    Ok(())
}

/// One cleanup action attempted by the terminal lifecycle guard.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CleanupAction {
    /// Unregister supported signal handlers.
    SignalHandlers,
    /// Release or clear the Ratatui backend.
    Backend,
    /// Show the cursor.
    Cursor,
    /// Leave the alternate screen.
    AlternateScreen,
    /// Disable raw mode.
    RawMode,
}

/// Metadata-only cleanup observation.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CleanupObservation {
    /// Number of state transitions that completed during startup.
    pub completed_transitions: u32,
    /// Number of inverse actions attempted.
    pub attempted_actions: u32,
    /// Number of inverse actions that completed.
    pub completed_actions: u32,
    /// Number of inverse actions that failed.
    pub failed_actions: u32,
}

impl CleanupObservation {
    fn validate_complete(self) -> Result<(), TerminalCoreError> {
        if self.failed_actions != 0
            || self.attempted_actions != self.completed_transitions
            || self.completed_actions != self.attempted_actions
        {
            return Err(TerminalCoreError::Cleanup);
        }
        Ok(())
    }
}

/// Bounded terminal exit class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalExit {
    /// User requested normal client shutdown.
    Normal,
    /// Caller cancellation was observed.
    Cancelled,
    /// Selected connection disconnected.
    Disconnected,
    /// A supported signal requested shutdown.
    Signal,
    /// A bounded backend or connection error ended the client.
    Failed,
}

impl TerminalExit {
    /// Returns the stable exit label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Cancelled => "cancelled",
            Self::Disconnected => "disconnected",
            Self::Signal => "signal",
            Self::Failed => "failed",
        }
    }
}

/// Metadata-only live-terminal evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LiveTerminalReceipt {
    /// Receipt schema.
    pub schema: &'static str,
    /// BLAKE3 identity of all receipt fields.
    pub receipt_identity: String,
    /// Validated configuration BLAKE3 identity.
    pub config_identity: String,
    /// BLAKE3 identity of the private root label.
    pub root_identity: String,
    /// BLAKE3 identity of the private worker label.
    pub worker_identity: String,
    /// Worker generation.
    pub generation: u64,
    /// Explicit connection mode.
    pub connection: &'static str,
    /// Bounded exit class.
    pub exit: TerminalExit,
    /// Final viewport width.
    pub viewport_width: u16,
    /// Final viewport height.
    pub viewport_height: u16,
    /// Input observation count.
    pub input_count: u64,
    /// Session event count.
    pub event_count: u64,
    /// Routed command count.
    pub command_count: u64,
    /// Completed render count.
    pub render_count: u64,
    /// Complete cleanup observation.
    pub cleanup: CleanupObservation,
    /// Fixed non-claims.
    pub non_claims: &'static [&'static str],
}

impl LiveTerminalReceipt {
    /// Renders safe metadata without input, buffers, prompts, endpoint paths, or content.
    #[must_use]
    pub fn render(&self) -> String {
        let mut lines = vec![format!(
            "LIVE_TERMINAL_RECEIPT schema={} receipt_identity={} config_identity={} root_identity={} worker_identity={} generation={} connection={} exit={} viewport={}x{} input_count={} event_count={} command_count={} render_count={} cleanup_transitions={} cleanup_attempted={} cleanup_completed={} cleanup_failed={} redaction=terminal-input-buffer-prompt-content-endpoint-path-provider-tool-and-session-content-omitted",
            self.schema,
            self.receipt_identity,
            self.config_identity,
            self.root_identity,
            self.worker_identity,
            self.generation,
            self.connection,
            self.exit.as_str(),
            self.viewport_width,
            self.viewport_height,
            self.input_count,
            self.event_count,
            self.command_count,
            self.render_count,
            self.cleanup.completed_transitions,
            self.cleanup.attempted_actions,
            self.cleanup.completed_actions,
            self.cleanup.failed_actions,
        )];
        lines.extend(
            self.non_claims
                .iter()
                .map(|claim| format!("NON_CLAIM surface={claim}")),
        );
        lines.push(String::new());
        lines.join("\n")
    }
}

/// Pure client contract failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalCoreError {
    /// A label or BLAKE3 identity was invalid.
    Identity,
    /// Snapshot identity, generation, or restore facts were invalid.
    Snapshot,
    /// A finite bound was zero or exceeded.
    Bound,
    /// A session event batch exceeded its per-iteration bound.
    EventFlood,
    /// Event sequence was stale, duplicated, or malformed.
    Sequence,
    /// A command descriptor was malformed or exceeded bounds.
    Command,
    /// A finite counter overflowed.
    Overflow,
    /// Terminal cleanup was incomplete.
    Cleanup,
}

impl core::fmt::Display for TerminalCoreError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "live terminal core rejected: {self:?}")
    }
}

impl std::error::Error for TerminalCoreError {}
