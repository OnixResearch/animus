//! Supervisor composition over injected worker and event ports.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

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

/// Safe bounded worker event metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorkerEvent {
    /// Root identity.
    pub root_id: String,
    /// Monotonic event sequence.
    pub sequence: u64,
    /// Safe event class.
    pub class: String,
    /// Metadata-only BLAKE3 observation identity.
    pub observation_identity: String,
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
        if events.len() > maximum_events
            || events.iter().any(|event| {
                event.root_id != root_id
                    || event.sequence <= snapshot.event_sequence
                    || event.observation_identity.len() != animus_adapter_daemon::BLAKE3_HEX_CHARS
            })
        {
            return Err(ServiceError::Bound);
        }
        self.attachments.insert(
            format!("{root_id}\0{client_id}"),
            AttachedClient {
                attachment: crate::Attachment {
                    client_id: String::from(client_id),
                    last_sequence: events
                        .last()
                        .map_or(snapshot.event_sequence, |event| event.sequence),
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
        let key = format!("{}\0{}", request.root_id, request.client_id);
        let attachment = self
            .attachments
            .get_mut(key.as_str())
            .ok_or(ServiceError::UnknownAttachment)?;
        if attachment.attachment.last_sequence != request.sequence {
            return Err(ServiceError::WorkerIdentity);
        }
        let events =
            self.worker
                .events_after(request.root_id, request.sequence, request.maximum_events)?;
        let maximum_events =
            usize::try_from(request.maximum_events).map_err(|_error| ServiceError::Bound)?;
        if events.len() > maximum_events
            || events.iter().any(|event| {
                event.root_id != request.root_id
                    || event.sequence <= request.sequence
                    || event.observation_identity.len() != animus_adapter_daemon::BLAKE3_HEX_CHARS
            })
        {
            return Err(ServiceError::Bound);
        }
        attachment.attachment.last_sequence = events
            .last()
            .map_or(request.sequence, |event| event.sequence);
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
