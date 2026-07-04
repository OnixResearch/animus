/// Queue capabilities advertised by a host adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueCapabilities {
    wait: bool,
    stream: bool,
    poll: bool,
    send_input: bool,
}

impl QueueCapabilities {
    /// Builds a capability set for queue interaction modes.
    #[must_use]
    pub const fn new(wait: bool, stream: bool, poll: bool, send_input: bool) -> Self {
        Self {
            wait,
            stream,
            poll,
            send_input,
        }
    }

    /// Returns whether wait is supported.
    #[must_use]
    pub const fn wait(self) -> bool {
        self.wait
    }

    /// Returns whether stream is supported.
    #[must_use]
    pub const fn stream(self) -> bool {
        self.stream
    }

    /// Returns whether poll is supported.
    #[must_use]
    pub const fn poll(self) -> bool {
        self.poll
    }

    /// Returns whether send-input is supported.
    #[must_use]
    pub const fn send_input(self) -> bool {
        self.send_input
    }
}

/// Optional queue capability requested by an interaction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueCapabilityKind {
    /// Wait for task completion.
    Wait,
    /// Stream task progress.
    Stream,
    /// Poll task status.
    Poll,
    /// Send input to a task.
    SendInput,
}

/// Queue interaction mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueInteractionKind {
    /// Wait for completion.
    Wait,
    /// Stream status or log progress.
    Stream,
    /// Poll current state.
    Poll,
    /// Send input through a backend-supported channel.
    SendInput,
}

impl QueueInteractionKind {
    const fn capability(self) -> QueueCapabilityKind {
        match self {
            Self::Wait => QueueCapabilityKind::Wait,
            Self::Stream => QueueCapabilityKind::Stream,
            Self::Poll => QueueCapabilityKind::Poll,
            Self::SendInput => QueueCapabilityKind::SendInput,
        }
    }
}

/// Borrowed interaction request input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueInteractionDraft<'a> {
    /// Interaction mode.
    pub kind: QueueInteractionKind,
    /// Target task ID.
    pub task_id: &'a str,
    /// Safe timeout policy label.
    pub timeout_policy: &'a str,
    /// Capabilities advertised by the host.
    pub capabilities: QueueCapabilities,
    /// Optional BLAKE3 input hash for send-input requests.
    pub input_hash: Option<&'a str>,
}

/// Validated queue wait, stream, poll, or send-input request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueInteractionRequest {
    kind: QueueInteractionKind,
    task_id: String,
    timeout_policy: String,
    input_hash: Option<crate::SafeHash>,
}

impl QueueInteractionRequest {
    /// Validates an interaction request against advertised host capabilities.
    ///
    /// # Errors
    ///
    /// Returns [`QueueContractError::UnsupportedCapability`] when the host did
    /// not advertise the requested mode.
    pub fn new(draft: QueueInteractionDraft<'_>) -> Result<Self, QueueContractError> {
        validate_capability(draft.kind, draft.capabilities)?;
        let input_hash = draft.input_hash.map(crate::SafeHash::new).transpose()?;
        if draft.kind == QueueInteractionKind::SendInput && input_hash.is_none() {
            return Err(QueueContractError::MissingInputHash);
        }
        let task_id = crate::validation::normalized_adapter_field(TASK_ID_FIELD, draft.task_id)?;
        let timeout_policy = crate::validation::normalized_adapter_field(
            TIMEOUT_POLICY_FIELD,
            draft.timeout_policy,
        )?;
        Ok(Self {
            kind: draft.kind,
            task_id,
            timeout_policy,
            input_hash,
        })
    }

    /// Returns the interaction kind.
    #[must_use]
    pub const fn kind(&self) -> QueueInteractionKind {
        self.kind
    }

    /// Returns the task ID.
    #[must_use]
    pub fn task_id(&self) -> &str {
        self.task_id.as_str()
    }

    /// Returns the timeout policy label.
    #[must_use]
    pub fn timeout_policy(&self) -> &str {
        self.timeout_policy.as_str()
    }

    /// Returns the optional safe input hash.
    #[must_use]
    pub const fn input_hash(&self) -> Option<&crate::SafeHash> {
        self.input_hash.as_ref()
    }
}

/// Borrowed log request input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct QueueLogRequestDraft<'a> {
    /// Task ID to inspect.
    pub task_id: &'a str,
    /// Requested model-visible bounds.
    pub bounds: QueueLogBounds,
}

/// Validated request to read one bounded task log viewport.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueLogRequest {
    task_id: String,
    bounds: QueueLogBounds,
}

impl QueueLogRequest {
    /// Validates a bounded log request.
    ///
    /// # Errors
    ///
    /// Returns [`QueueContractError`] when the task ID is not adapter-safe.
    pub fn new(draft: QueueLogRequestDraft<'_>) -> Result<Self, QueueContractError> {
        let task_id = crate::validation::normalized_adapter_field(TASK_ID_FIELD, draft.task_id)?;
        Ok(Self {
            task_id,
            bounds: draft.bounds,
        })
    }

    /// Returns the task ID.
    #[must_use]
    pub fn task_id(&self) -> &str {
        self.task_id.as_str()
    }

    /// Returns requested bounds.
    #[must_use]
    pub const fn bounds(&self) -> QueueLogBounds {
        self.bounds
    }
}

/// Request variants sent to an injected background queue service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueueServiceRequest {
    /// Enqueue a task.
    Enqueue(QueueEnqueueRequest),
    /// Apply or validate a control operation.
    Control(QueueControlRequest),
    /// Wait, stream, poll, or send input.
    Interaction(QueueInteractionRequest),
    /// Read a bounded log viewport.
    ReadLog(QueueLogRequest),
    /// List queue groups and group status.
    ListGroups,
}

/// Response variants returned by an injected background queue service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueueServiceResponse {
    /// A task was enqueued or observed.
    Task(QueueTaskObservation),
    /// A control request was accepted by the host queue service.
    ControlAccepted(QueueOperationKind),
    /// An interaction request was accepted by the host queue service.
    InteractionAccepted(QueueInteractionKind),
    /// A bounded log viewport was returned.
    Log(QueueLogViewport),
    /// Group statuses were returned.
    Groups(Vec<QueueGroupStatus>),
}

/// Host-injected background queue service contract.
pub trait BackgroundQueueService:
    crate::ComponentAdapter<QueueServiceRequest, crate::ServiceOutcome<QueueServiceResponse>>
{
}

impl<T> BackgroundQueueService for T where
    T: crate::ComponentAdapter<QueueServiceRequest, crate::ServiceOutcome<QueueServiceResponse>>
{
}
