//! Supervisor composition over injected worker and event ports.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

const MAX_EVENT_CLASS_BYTES: usize = 256;
const STATE_INSPECTION_IDENTITY_DOMAIN: &str = "animus.local-daemon.state-inspection.v1";
const IDENTITY_SEPARATOR: &[u8] = b"\0";

/// Safe snapshot metadata routed before event flow.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerSnapshot {
    /// Root identity.
    pub root_id: String,
    /// Worker identity.
    pub worker_id: String,
    /// Worker generation.
    pub generation: u64,
    /// Latest event sequence.
    pub event_sequence: u64,
    /// Safe restore status.
    pub restore_status: crate::RestoreStatus,
}

/// Bounded metadata from one worker-owned state inspection.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct WorkerStateInspection {
    /// Current worker generation.
    pub worker_generation: u64,
    /// Current durable restore status.
    pub restore_status: crate::RestoreStatus,
    /// BLAKE3 identity of the current session head.
    pub session_head_identity: String,
    /// BLAKE3 identity of the current kernel head.
    pub kernel_head_identity: String,
    /// Current queue generation. Zero identifies the initial empty queue.
    pub queue_generation: u64,
    /// BLAKE3 identity of the current queue state.
    pub queue_identity: String,
    /// Committed conversation message count.
    pub committed_message_count: u32,
    /// Current queued item count.
    pub queued_item_count: u32,
    /// Current active child count.
    pub active_child_count: u32,
}

/// Exact facts that bind one state payload to an ordered worker request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StateInspectionIdentityFacts<'a> {
    /// Root session identity.
    pub root_id: &'a str,
    /// Monotonic worker event sequence.
    pub sequence: u64,
    /// BLAKE3 state-inspection request identity.
    pub correlation_id: &'a str,
}

impl WorkerStateInspection {
    /// Validates the metadata-only inspection boundary.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError::Bound`] for zero generations, blocked restore,
    /// or malformed BLAKE3 identities.
    pub fn validate(&self) -> Result<(), ServiceError> {
        if self.worker_generation == 0 || self.restore_status == crate::RestoreStatus::Blocked {
            return Err(ServiceError::Bound);
        }
        validate_blake3_identity(self.session_head_identity.as_str())?;
        validate_blake3_identity(self.kernel_head_identity.as_str())?;
        validate_blake3_identity(self.queue_identity.as_str())
    }

    /// Computes the exact BLAKE3 identity for one ordered correlated state event.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError::Bound`] for malformed state, root, sequence, or correlation facts.
    pub fn observation_identity(
        &self,
        facts: StateInspectionIdentityFacts<'_>,
    ) -> Result<String, ServiceError> {
        self.validate()?;
        validate_event_class(facts.root_id)?;
        if facts.sequence == 0 {
            return Err(ServiceError::Bound);
        }
        validate_blake3_identity(facts.correlation_id)?;
        let sequence_text = facts.sequence.to_string();
        let worker_generation_text = self.worker_generation.to_string();
        let queue_generation_text = self.queue_generation.to_string();
        let committed_message_count_text = self.committed_message_count.to_string();
        let queued_item_count_text = self.queued_item_count.to_string();
        let active_child_count_text = self.active_child_count.to_string();
        let mut hasher = blake3::Hasher::new();
        for part in [
            STATE_INSPECTION_IDENTITY_DOMAIN,
            facts.root_id,
            sequence_text.as_str(),
            facts.correlation_id,
            worker_generation_text.as_str(),
            self.restore_status.as_str(),
            self.session_head_identity.as_str(),
            self.kernel_head_identity.as_str(),
            queue_generation_text.as_str(),
            self.queue_identity.as_str(),
            committed_message_count_text.as_str(),
            queued_item_count_text.as_str(),
            active_child_count_text.as_str(),
        ] {
            hasher.update(part.as_bytes());
            hasher.update(IDENTITY_SEPARATOR);
        }
        Ok(hasher.finalize().to_hex().to_string())
    }
}

/// Safe bounded worker event metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorkerEvent {
    /// One safe class and observation identity.
    Observation {
        /// Root identity.
        root_id: String,
        /// Monotonic event sequence.
        sequence: u64,
        /// Safe event class.
        class: String,
        /// Metadata-only BLAKE3 observation identity.
        observation_identity: String,
    },
    /// One typed metadata-only state inspection.
    StateInspection {
        /// Root identity.
        root_id: String,
        /// Monotonic event sequence.
        sequence: u64,
        /// BLAKE3 identity of the state-inspection request.
        correlation_id: String,
        /// Metadata-only BLAKE3 observation identity.
        observation_identity: String,
        /// Bounded worker state.
        state: WorkerStateInspection,
    },
}

impl WorkerEvent {
    /// Constructs one exact correlated state-inspection event.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError::Bound`] when state or correlation facts are invalid.
    pub fn state_inspection(
        facts: StateInspectionIdentityFacts<'_>,
        state: WorkerStateInspection,
    ) -> Result<Self, ServiceError> {
        let observation_identity = state.observation_identity(facts)?;
        Ok(Self::StateInspection {
            root_id: String::from(facts.root_id),
            sequence: facts.sequence,
            correlation_id: String::from(facts.correlation_id),
            observation_identity,
            state,
        })
    }

    /// Returns the event root identity.
    #[must_use]
    pub fn root_id(&self) -> &str {
        match self {
            Self::Observation { root_id, .. } | Self::StateInspection { root_id, .. } => {
                root_id.as_str()
            }
        }
    }

    /// Returns the monotonic event sequence.
    #[must_use]
    pub const fn sequence(&self) -> u64 {
        match self {
            Self::Observation { sequence, .. } | Self::StateInspection { sequence, .. } => {
                *sequence
            }
        }
    }

    /// Returns the metadata-only observation identity.
    #[must_use]
    pub fn observation_identity(&self) -> &str {
        match self {
            Self::Observation {
                observation_identity,
                ..
            }
            | Self::StateInspection {
                observation_identity,
                ..
            } => observation_identity.as_str(),
        }
    }

    fn validate(
        &self,
        worker_generation: u64,
        restore_status: crate::RestoreStatus,
    ) -> Result<(), ServiceError> {
        if self.sequence() == 0 {
            return Err(ServiceError::Bound);
        }
        validate_blake3_identity(self.observation_identity())?;
        match self {
            Self::Observation { class, .. } => validate_event_class(class.as_str()),
            Self::StateInspection {
                root_id,
                sequence,
                correlation_id,
                observation_identity,
                state,
            } => {
                if state.worker_generation != worker_generation
                    || state.restore_status != restore_status
                    || state.observation_identity(StateInspectionIdentityFacts {
                        root_id,
                        sequence: *sequence,
                        correlation_id,
                    })? != *observation_identity
                {
                    return Err(ServiceError::WorkerIdentity);
                }
                Ok(())
            }
        }
    }
}

/// Typed command routed to the worker-owned session service.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorkerCommand {
    /// Root identity.
    pub root_id: String,
    /// Worker generation.
    pub generation: u64,
    /// Correlation identity.
    pub correlation_id: String,
    /// Safe command class.
    pub class: String,
    /// Private typed command payload routed unchanged to the worker.
    ///
    /// The supervisor does not parse or retain this payload. Daemon and terminal
    /// receipts must omit it.
    pub private_payload: Vec<u8>,
}

/// Worker-owned session port. The supervisor can route only.
pub trait WorkerSessionPort {
    /// Returns safe current snapshot metadata.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when the worker cannot supply current metadata.
    fn snapshot(&mut self, root_id: &str) -> Result<WorkerSnapshot, ServiceError>;
    /// Returns events after one sequence under a finite count.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when bounded event retrieval fails.
    fn events_after(
        &mut self,
        root_id: &str,
        sequence: u64,
        maximum: u32,
    ) -> Result<Vec<WorkerEvent>, ServiceError>;
    /// Routes a command without executing it in the supervisor.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when the worker rejects or cannot route the command.
    fn route(&mut self, command: &WorkerCommand) -> Result<String, ServiceError>;
    /// Requests authorized worker stop.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when stop request delivery fails.
    fn request_stop(&mut self, root_id: &str, generation: u64) -> Result<(), ServiceError>;
    /// Requests one admitted replacement generation.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when replacement startup or restore fails.
    fn retry(&mut self, root_id: &str, generation: u64) -> Result<WorkerSnapshot, ServiceError>;
}

/// One bounded later-event request for an existing client attachment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EventBatchRequest<'a> {
    /// Root session identity.
    pub root_id: &'a str,
    /// Existing attached client identity.
    pub client_id: &'a str,
    /// Last sequence observed by the client.
    pub sequence: u64,
    /// Maximum events admitted for this call.
    pub maximum_events: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AttachedClient {
    attachment: crate::Attachment,
    delivery_scope_lease: crate::ClientDeliveryScopeLease,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct EventBatchValidation<'a> {
    root_id: &'a str,
    first_sequence: u64,
    maximum_events: usize,
    worker_generation: u64,
    restore_status: crate::RestoreStatus,
}

/// Explicit local supervisor service.
pub struct LocalSupervisorService<Worker> {
    config: crate::ValidatedLocalDaemonConfig,
    registry: crate::WorkerRegistry,
    attachments: BTreeMap<String, AttachedClient>,
    worker: Worker,
}

impl<Worker: WorkerSessionPort> LocalSupervisorService<Worker> {
    /// Builds the supervisor from explicit configuration, registry state, and worker port.
    #[must_use]
    pub fn new(
        config: crate::ValidatedLocalDaemonConfig,
        registry: crate::WorkerRegistry,
        worker: Worker,
    ) -> Self {
        Self {
            config,
            registry,
            attachments: BTreeMap::new(),
            worker,
        }
    }

    /// Attaches snapshot-first and registers one bounded client lease.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] for denied routing, worker mismatch, or bounds.
    pub fn attach(
        &mut self,
        root_id: &str,
        client_id: &str,
        maximum_events: u32,
    ) -> Result<(WorkerSnapshot, Vec<WorkerEvent>), ServiceError> {
        crate::decide_routing(
            &self.registry,
            &crate::RoutingRequest::Attach {
                root_id: String::from(root_id),
                client_id: String::from(client_id),
                max_events: maximum_events,
            },
            self.config.max_attachments(),
            self.config.max_events(),
        )
        .map_err(|_error| ServiceError::Decision)?;
        let record = self.registry.find(root_id).ok_or(ServiceError::Decision)?;
        let delivery_scope_lease = crate::plan_client_delivery_scope_lease(record, client_id)
            .map_err(|_error| ServiceError::ComponentScope)?;
        let snapshot = self.worker.snapshot(root_id)?;
        if snapshot.restore_status != record.restore_status {
            return Err(ServiceError::WorkerIdentity);
        }
        if snapshot.worker_id != record.worker_id
            || snapshot.generation != record.generation
            || snapshot.restore_status == crate::RestoreStatus::Blocked
        {
            return Err(ServiceError::WorkerIdentity);
        }
        let events = self
            .worker
            .events_after(root_id, snapshot.event_sequence, maximum_events)?;
        let maximum_events =
            usize::try_from(maximum_events).map_err(|_error| ServiceError::Bound)?;
        let last_sequence = validate_event_batch(
            events.as_slice(),
            EventBatchValidation {
                root_id,
                first_sequence: snapshot.event_sequence,
                maximum_events,
                worker_generation: record.generation,
                restore_status: record.restore_status,
            },
        )?;
        self.attachments.insert(
            format!("{root_id}\0{client_id}"),
            AttachedClient {
                attachment: crate::Attachment {
                    client_id: String::from(client_id),
                    last_sequence,
                },
                delivery_scope_lease,
            },
        );
        Ok((snapshot, events))
    }

    /// Returns one bounded later event batch for an existing client attachment.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when the attachment is absent or event facts violate bounds.
    pub fn events_after(
        &mut self,
        request: EventBatchRequest<'_>,
    ) -> Result<Vec<WorkerEvent>, ServiceError> {
        if request.maximum_events == 0 || request.maximum_events > self.config.max_events() {
            return Err(ServiceError::Bound);
        }
        let record = self
            .registry
            .find(request.root_id)
            .ok_or(ServiceError::Decision)?;
        let worker_generation = record.generation;
        let restore_status = record.restore_status;
        let key = format!("{}\0{}", request.root_id, request.client_id);
        let attachment = self
            .attachments
            .get_mut(key.as_str())
            .ok_or(ServiceError::UnknownAttachment)?;
        if attachment.attachment.last_sequence != request.sequence
            || attachment.delivery_scope_lease.worker_generation.get() != worker_generation
        {
            return Err(ServiceError::WorkerIdentity);
        }
        let events =
            self.worker
                .events_after(request.root_id, request.sequence, request.maximum_events)?;
        let maximum_events =
            usize::try_from(request.maximum_events).map_err(|_error| ServiceError::Bound)?;
        attachment.attachment.last_sequence = validate_event_batch(
            events.as_slice(),
            EventBatchValidation {
                root_id: request.root_id,
                first_sequence: request.sequence,
                maximum_events,
                worker_generation,
                restore_status,
            },
        )?;
        Ok(events)
    }

    /// Detaches one client lease without stopping worker execution.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when routing is denied or no lease exists.
    pub fn detach(&mut self, root_id: &str, client_id: &str) -> Result<(), ServiceError> {
        crate::decide_routing(
            &self.registry,
            &crate::RoutingRequest::Detach {
                root_id: String::from(root_id),
                client_id: String::from(client_id),
            },
            self.config.max_attachments(),
            self.config.max_events(),
        )
        .map_err(|_error| ServiceError::Decision)?;
        self.attachments
            .remove(format!("{root_id}\0{client_id}").as_str())
            .ok_or(ServiceError::UnknownAttachment)?;
        Ok(())
    }

    /// Routes one typed command to the worker-owned service.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] for invalid routing or worker failure.
    pub fn route(&mut self, command: &WorkerCommand) -> Result<String, ServiceError> {
        let decision = crate::decide_routing(
            &self.registry,
            &crate::RoutingRequest::Command {
                root_id: command.root_id.clone(),
                correlation_id: command.correlation_id.clone(),
            },
            self.config.max_attachments(),
            self.config.max_events(),
        )
        .map_err(|_error| ServiceError::Decision)?;
        let crate::RoutingDecision::Routed { worker_id, .. } = decision else {
            return Err(ServiceError::Decision);
        };
        let record = self
            .registry
            .find(command.root_id.as_str())
            .ok_or(ServiceError::Decision)?;
        if worker_id != record.worker_id || command.generation != record.generation {
            return Err(ServiceError::WorkerIdentity);
        }
        self.worker.route(command)
    }

    /// Requests stop without depending on client attachment state.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] for unknown roots, stale generation, or worker failure.
    pub fn stop(&mut self, root_id: &str, generation: u64) -> Result<(), ServiceError> {
        let record = self.registry.find(root_id).ok_or(ServiceError::Decision)?;
        let scope_plan =
            crate::plan_worker_root_scope(record).map_err(|_error| ServiceError::ComponentScope)?;
        if scope_plan.generation.get() != generation {
            return Err(ServiceError::WorkerIdentity);
        }
        self.worker.request_stop(root_id, generation)
    }

    /// Requests explicit retry only after recovery admits replacement.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] for denied retry, generation drift, or worker failure.
    pub fn retry(
        &mut self,
        root_id: &str,
        facts: crate::HealthFacts,
    ) -> Result<WorkerSnapshot, ServiceError> {
        let record = self.registry.find(root_id).ok_or(ServiceError::Decision)?;
        let decision = crate::decide_recovery(record.lifecycle, record.restore_status, facts);
        if !decision.retry_allowed {
            return Err(ServiceError::RetryDenied);
        }
        let generation =
            crate::next_generation(record.generation).map_err(|_error| ServiceError::Decision)?;
        self.worker.retry(root_id, generation)
    }

    /// Returns one client-owned delivery lease without worker-scope ownership.
    #[must_use]
    pub fn client_delivery_scope_lease(
        &self,
        root_id: &str,
        client_id: &str,
    ) -> Option<&crate::ClientDeliveryScopeLease> {
        self.attachments
            .get(format!("{root_id}\0{client_id}").as_str())
            .map(|attached| &attached.delivery_scope_lease)
    }

    /// Returns the current attachment count.
    #[must_use]
    pub fn attachment_count(&self) -> usize {
        self.attachments.len()
    }
}

/// Safe supervisor service failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceError {
    /// Pure routing or recovery decision denied the operation.
    Decision,
    /// Worker identity or generation differs.
    WorkerIdentity,
    /// Bounded event flow was violated.
    Bound,
    /// Client lease does not exist.
    UnknownAttachment,
    /// Explicit retry is not admitted in current state.
    RetryDenied,
    /// Worker or client scope mapping was invalid.
    ComponentScope,
    /// Worker-owned service failed.
    Worker,
}

impl core::fmt::Display for ServiceError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "local supervisor service rejected: {self:?}")
    }
}

impl std::error::Error for ServiceError {}

fn validate_event_batch(
    events: &[WorkerEvent],
    validation: EventBatchValidation<'_>,
) -> Result<u64, ServiceError> {
    if events.len() > validation.maximum_events {
        return Err(ServiceError::Bound);
    }
    let mut previous_sequence = validation.first_sequence;
    for event in events {
        if event.root_id() != validation.root_id || event.sequence() <= previous_sequence {
            return Err(ServiceError::Bound);
        }
        event.validate(validation.worker_generation, validation.restore_status)?;
        previous_sequence = event.sequence();
    }
    Ok(previous_sequence)
}

fn validate_blake3_identity(value: &str) -> Result<(), ServiceError> {
    if value.len() != animus_adapter_daemon::BLAKE3_HEX_CHARS
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(ServiceError::Bound);
    }
    Ok(())
}

fn validate_event_class(value: &str) -> Result<(), ServiceError> {
    if value.is_empty()
        || value.len() > MAX_EVENT_CLASS_BYTES
        || value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return Err(ServiceError::Bound);
    }
    Ok(())
}
