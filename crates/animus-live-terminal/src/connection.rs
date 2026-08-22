//! Explicit daemon and direct connection adapters.

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

const TRANSPORT_VERSION: u16 = 1;
const TRANSPORT_DEADLINE_MS: u64 = 30_000;
const MAX_WIRE_PRIVATE_BYTES: u32 = animus_local_daemon::MAX_PRIVATE_EVENT_BYTES;
const PRIVATE_RUNTIME_EVENT_CLASS: &str = "runtime_event";

/// Client-facing service port. Implementations route only and own no session effects.
pub trait TerminalConnectionPort {
    /// Returns one snapshot before later event polling.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalConnectionError`] when attachment or snapshot decoding fails.
    fn attach(
        &mut self,
        root_id: &str,
        client_id: &str,
        maximum_events: u32,
    ) -> Result<crate::ClientSnapshot, TerminalConnectionError>;

    /// Refreshes one existing attachment snapshot before returning later events.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalConnectionError`] when the attachment is stale or unavailable.
    fn refresh(
        &mut self,
        root_id: &str,
        client_id: &str,
        maximum_events: u32,
    ) -> Result<crate::ClientSnapshot, TerminalConnectionError>;

    /// Returns later events after the supplied sequence under a finite count.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalConnectionError`] for connection, protocol, or bound failure.
    fn events_after(
        &mut self,
        root_id: &str,
        sequence: u64,
        maximum_events: u32,
    ) -> Result<Vec<crate::ClientSessionEvent>, TerminalConnectionError>;

    /// Routes one typed command descriptor without executing provider, tool, Steel, or session work.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalConnectionError`] when the selected service rejects the command.
    fn route(
        &mut self,
        root_id: &str,
        generation: u64,
        command: &crate::ClientCommand,
    ) -> Result<String, TerminalConnectionError>;

    /// Requests a generation-fenced worker stop through the supervisor.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalConnectionError`] when the worker identity is stale or stop fails.
    fn stop(&mut self, root_id: &str, generation: u64) -> Result<(), TerminalConnectionError>;

    /// Removes only this client attachment lease.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalConnectionError`] when detach delivery fails.
    fn detach(&mut self, root_id: &str, client_id: &str) -> Result<(), TerminalConnectionError>;
}

/// Explicit in-process adapter over one injected supervisor service.
pub struct DirectTerminalConnection<Worker> {
    service: animus_local_daemon::LocalSupervisorService<Worker>,
    attached_root: Option<String>,
    attached_client: Option<String>,
}

impl<Worker> DirectTerminalConnection<Worker> {
    /// Builds direct mode from one explicitly injected service.
    #[must_use]
    pub const fn new(service: animus_local_daemon::LocalSupervisorService<Worker>) -> Self {
        Self {
            service,
            attached_root: None,
            attached_client: None,
        }
    }
}

impl<Worker: animus_local_daemon::WorkerSessionPort> TerminalConnectionPort
    for DirectTerminalConnection<Worker>
{
    fn attach(
        &mut self,
        root_id: &str,
        client_id: &str,
        maximum_events: u32,
    ) -> Result<crate::ClientSnapshot, TerminalConnectionError> {
        let (snapshot, events) = self
            .service
            .attach(root_id, client_id, maximum_events)
            .map_err(|_error| TerminalConnectionError::Unavailable)?;
        let core = snapshot_from_daemon(snapshot, events)?;
        self.attached_root = Some(String::from(root_id));
        self.attached_client = Some(String::from(client_id));
        Ok(core)
    }

    fn refresh(
        &mut self,
        root_id: &str,
        client_id: &str,
        maximum_events: u32,
    ) -> Result<crate::ClientSnapshot, TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id)
            || self.attached_client.as_deref() != Some(client_id)
        {
            return Err(TerminalConnectionError::Rejected);
        }
        let (snapshot, events) = self
            .service
            .refresh(root_id, client_id, maximum_events)
            .map_err(|_error| TerminalConnectionError::Rejected)?;
        snapshot_from_daemon(snapshot, events)
    }

    fn events_after(
        &mut self,
        root_id: &str,
        sequence: u64,
        maximum_events: u32,
    ) -> Result<Vec<crate::ClientSessionEvent>, TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id) {
            return Err(TerminalConnectionError::Rejected);
        }
        let client_id = self
            .attached_client
            .as_deref()
            .ok_or(TerminalConnectionError::Rejected)?;
        self.service
            .events_after(animus_local_daemon::EventBatchRequest {
                root_id,
                client_id,
                sequence,
                maximum_events,
            })
            .map_err(|_error| TerminalConnectionError::Rejected)?
            .into_iter()
            .map(client_event_from_worker)
            .collect()
    }

    fn route(
        &mut self,
        root_id: &str,
        generation: u64,
        command: &crate::ClientCommand,
    ) -> Result<String, TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id) {
            return Err(TerminalConnectionError::Rejected);
        }
        crate::core::validate_command(command, MAX_WIRE_PRIVATE_BYTES)
            .map_err(|_error| TerminalConnectionError::Rejected)?;
        let private_payload =
            serde_json::to_vec(command).map_err(|_error| TerminalConnectionError::Protocol)?;
        self.service
            .route(&animus_local_daemon::WorkerCommand {
                root_id: String::from(root_id),
                generation,
                correlation_id: String::from(command.correlation_id()),
                class: String::from(command.class()),
                private_payload,
            })
            .map_err(|_error| TerminalConnectionError::Rejected)
    }

    fn stop(&mut self, root_id: &str, generation: u64) -> Result<(), TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id) {
            return Err(TerminalConnectionError::Rejected);
        }
        self.service
            .stop(root_id, generation)
            .map_err(|_error| TerminalConnectionError::Rejected)
    }

    fn detach(&mut self, root_id: &str, client_id: &str) -> Result<(), TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id)
            || self.attached_client.as_deref() != Some(client_id)
        {
            return Err(TerminalConnectionError::Rejected);
        }
        self.service
            .detach(root_id, client_id)
            .map_err(|_error| TerminalConnectionError::Rejected)?;
        self.attached_root = None;
        self.attached_client = None;
        Ok(())
    }
}

/// Daemon-side bounded dispatcher over one explicit supervisor service.
pub struct DaemonRequestDispatcher<Worker> {
    service: animus_local_daemon::LocalSupervisorService<Worker>,
    attachment: Option<(String, String)>,
}

impl<Worker: animus_local_daemon::WorkerSessionPort> DaemonRequestDispatcher<Worker> {
    /// Creates one dispatcher for one local client connection.
    #[must_use]
    pub const fn new(service: animus_local_daemon::LocalSupervisorService<Worker>) -> Self {
        Self {
            service,
            attachment: None,
        }
    }

    /// Removes the current connection attachment after disconnect.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalConnectionError`] when the recorded lease cannot be removed.
    pub fn disconnect(&mut self) -> Result<(), TerminalConnectionError> {
        let Some((root_id, client_id)) = self.attachment.take() else {
            return Ok(());
        };
        self.service
            .detach(root_id.as_str(), client_id.as_str())
            .map_err(|_error| TerminalConnectionError::Rejected)
    }

    /// Dispatches one bounded protocol request without executing worker-owned session effects.
    #[must_use]
    pub fn dispatch(&mut self, request: DaemonRequest) -> DaemonResponse {
        self.try_dispatch(request)
            .unwrap_or_else(|error| DaemonResponse::Failed {
                class: String::from(error.as_str()),
            })
    }

    #[allow(
        clippy::too_many_lines,
        reason = "one exhaustive protocol match keeps every request variant fail-closed"
    )]
    fn try_dispatch(
        &mut self,
        request: DaemonRequest,
    ) -> Result<DaemonResponse, TerminalConnectionError> {
        match request {
            DaemonRequest::Attach {
                root_id,
                client_id,
                maximum_events,
            } => {
                if self.attachment.is_some() {
                    return Err(TerminalConnectionError::Rejected);
                }
                let (snapshot, events) = self
                    .service
                    .attach(root_id.as_str(), client_id.as_str(), maximum_events)
                    .map_err(|_error| TerminalConnectionError::Rejected)?;
                let response = DaemonResponse::Snapshot {
                    snapshot: DaemonSnapshot {
                        root_id: snapshot.root_id,
                        worker_id: snapshot.worker_id,
                        generation: snapshot.generation,
                        event_sequence: snapshot.event_sequence,
                        restore_status: snapshot.restore_status,
                    },
                    events: events.into_iter().map(daemon_event_from_worker).collect(),
                };
                self.attachment = Some((root_id, client_id));
                Ok(response)
            }
            DaemonRequest::Refresh {
                root_id,
                client_id,
                maximum_events,
            } => {
                if self.attachment.as_ref() != Some(&(root_id.clone(), client_id.clone())) {
                    return Err(TerminalConnectionError::Rejected);
                }
                let (snapshot, events) = self
                    .service
                    .refresh(root_id.as_str(), client_id.as_str(), maximum_events)
                    .map_err(|_error| TerminalConnectionError::Rejected)?;
                Ok(DaemonResponse::Snapshot {
                    snapshot: DaemonSnapshot {
                        root_id: snapshot.root_id,
                        worker_id: snapshot.worker_id,
                        generation: snapshot.generation,
                        event_sequence: snapshot.event_sequence,
                        restore_status: snapshot.restore_status,
                    },
                    events: events.into_iter().map(daemon_event_from_worker).collect(),
                })
            }
            DaemonRequest::EventsAfter {
                root_id,
                sequence,
                maximum_events,
            } => {
                let (attached_root, client_id) = self
                    .attachment
                    .as_ref()
                    .ok_or(TerminalConnectionError::Rejected)?;
                if attached_root != &root_id {
                    return Err(TerminalConnectionError::Rejected);
                }
                let events = self
                    .service
                    .events_after(animus_local_daemon::EventBatchRequest {
                        root_id: root_id.as_str(),
                        client_id: client_id.as_str(),
                        sequence,
                        maximum_events,
                    })
                    .map_err(|_error| TerminalConnectionError::Rejected)?;
                Ok(DaemonResponse::Events {
                    events: events.into_iter().map(daemon_event_from_worker).collect(),
                })
            }
            DaemonRequest::Command {
                root_id,
                generation,
                correlation_id,
                class,
                private_payload,
            } => {
                let (attached_root, _client_id) = self
                    .attachment
                    .as_ref()
                    .ok_or(TerminalConnectionError::Rejected)?;
                if attached_root != &root_id {
                    return Err(TerminalConnectionError::Rejected);
                }
                validate_hash(correlation_id.as_str())?;
                let maximum_payload = usize::try_from(MAX_WIRE_PRIVATE_BYTES)
                    .map_err(|_error| TerminalConnectionError::Protocol)?;
                if !valid_command_class(class.as_str())
                    || private_payload.is_empty()
                    || private_payload.len() > maximum_payload
                {
                    return Err(TerminalConnectionError::Protocol);
                }
                let safe_response_identity = self
                    .service
                    .route(&animus_local_daemon::WorkerCommand {
                        root_id,
                        generation,
                        correlation_id,
                        class,
                        private_payload,
                    })
                    .map_err(|_error| TerminalConnectionError::Rejected)?;
                Ok(DaemonResponse::CommandAccepted {
                    safe_response_identity,
                })
            }
            DaemonRequest::Stop {
                root_id,
                generation,
            } => {
                let (attached_root, _client_id) = self
                    .attachment
                    .as_ref()
                    .ok_or(TerminalConnectionError::Rejected)?;
                if attached_root != &root_id {
                    return Err(TerminalConnectionError::Rejected);
                }
                self.service
                    .stop(root_id.as_str(), generation)
                    .map_err(|_error| TerminalConnectionError::Rejected)?;
                Ok(DaemonResponse::Stopped)
            }
            DaemonRequest::Detach { root_id, client_id } => {
                if self.attachment.as_ref() != Some(&(root_id.clone(), client_id.clone())) {
                    return Err(TerminalConnectionError::Rejected);
                }
                self.service
                    .detach(root_id.as_str(), client_id.as_str())
                    .map_err(|_error| TerminalConnectionError::Rejected)?;
                self.attachment = None;
                Ok(DaemonResponse::Detached)
            }
        }
    }
}

fn daemon_event_from_worker(event: animus_local_daemon::WorkerEvent) -> DaemonEvent {
    match event {
        animus_local_daemon::WorkerEvent::Observation {
            sequence,
            class,
            observation_identity,
            ..
        } => DaemonEvent::Observation {
            sequence,
            class,
            observation_identity,
        },
        animus_local_daemon::WorkerEvent::StateInspection {
            sequence,
            correlation_id,
            observation_identity,
            state,
            ..
        } => DaemonEvent::StateInspection {
            sequence,
            correlation_id,
            observation_identity,
            state,
        },
        animus_local_daemon::WorkerEvent::Private(event) => {
            let (sequence, class, payload_identity, private_payload) = event.into_wire_parts();
            DaemonEvent::Private {
                sequence,
                class,
                payload_identity,
                private_payload,
            }
        }
    }
}

/// Explicit Unix daemon connection with no endpoint discovery or fallback.
pub struct DaemonTerminalConnection {
    client: animus_local_daemon::LocalClient,
    endpoint_identity: String,
    generation: u64,
    max_frame_bytes: u32,
    attached_root: Option<String>,
    attached_client: Option<String>,
}

impl DaemonTerminalConnection {
    /// Connects to one caller-constructed local endpoint.
    ///
    /// # Errors
    ///
    /// Returns [`TerminalConnectionError`] when connection or deadline setup fails.
    pub fn connect(
        endpoint: &animus_local_daemon::LocalEndpoint,
        generation: u64,
        max_frame_bytes: u32,
    ) -> Result<Self, TerminalConnectionError> {
        if generation == 0 || max_frame_bytes == 0 {
            return Err(TerminalConnectionError::Protocol);
        }
        let client = animus_local_daemon::LocalClient::connect(endpoint)
            .map_err(|_error| TerminalConnectionError::Unavailable)?;
        client
            .set_deadline(Duration::from_millis(TRANSPORT_DEADLINE_MS))
            .map_err(|_error| TerminalConnectionError::Unavailable)?;
        Ok(Self {
            client,
            endpoint_identity: String::from(endpoint.identity()),
            generation,
            max_frame_bytes,
            attached_root: None,
            attached_client: None,
        })
    }

    fn transact(
        &mut self,
        correlation_id: &str,
        request: &DaemonRequest,
    ) -> Result<DaemonResponse, TerminalConnectionError> {
        let frame = animus_local_daemon::TransportFrame {
            version: TRANSPORT_VERSION,
            endpoint_identity: self.endpoint_identity.clone(),
            generation: self.generation,
            correlation_id: String::from(correlation_id),
            payload: request,
        };
        self.client
            .write_frame(&frame, self.max_frame_bytes)
            .map_err(|_error| TerminalConnectionError::Protocol)?;
        let response = self
            .client
            .read_frame::<DaemonResponse>(self.max_frame_bytes)
            .map_err(|_error| TerminalConnectionError::Unavailable)?;
        if response.version != TRANSPORT_VERSION
            || response.endpoint_identity != self.endpoint_identity
            || response.generation != self.generation
            || response.correlation_id != correlation_id
        {
            return Err(TerminalConnectionError::Protocol);
        }
        Ok(response.payload)
    }
}

impl TerminalConnectionPort for DaemonTerminalConnection {
    fn attach(
        &mut self,
        root_id: &str,
        client_id: &str,
        maximum_events: u32,
    ) -> Result<crate::ClientSnapshot, TerminalConnectionError> {
        let correlation_id = request_identity("attach", root_id, client_id, 0);
        let response = self.transact(
            correlation_id.as_str(),
            &DaemonRequest::Attach {
                root_id: String::from(root_id),
                client_id: String::from(client_id),
                maximum_events,
            },
        )?;
        let DaemonResponse::Snapshot { snapshot, events } = response else {
            return Err(TerminalConnectionError::Protocol);
        };
        let core = snapshot.into_core(events)?;
        self.attached_root = Some(String::from(root_id));
        self.attached_client = Some(String::from(client_id));
        Ok(core)
    }

    fn refresh(
        &mut self,
        root_id: &str,
        client_id: &str,
        maximum_events: u32,
    ) -> Result<crate::ClientSnapshot, TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id)
            || self.attached_client.as_deref() != Some(client_id)
        {
            return Err(TerminalConnectionError::Rejected);
        }
        let correlation_id = request_identity("refresh", root_id, client_id, 0);
        let response = self.transact(
            correlation_id.as_str(),
            &DaemonRequest::Refresh {
                root_id: String::from(root_id),
                client_id: String::from(client_id),
                maximum_events,
            },
        )?;
        let DaemonResponse::Snapshot { snapshot, events } = response else {
            return Err(TerminalConnectionError::Protocol);
        };
        snapshot.into_core(events)
    }

    fn events_after(
        &mut self,
        root_id: &str,
        sequence: u64,
        maximum_events: u32,
    ) -> Result<Vec<crate::ClientSessionEvent>, TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id) {
            return Err(TerminalConnectionError::Rejected);
        }
        let correlation_id = request_identity("events", root_id, "terminal", sequence);
        let response = self.transact(
            correlation_id.as_str(),
            &DaemonRequest::EventsAfter {
                root_id: String::from(root_id),
                sequence,
                maximum_events,
            },
        )?;
        let DaemonResponse::Events { events } = response else {
            return Err(TerminalConnectionError::Protocol);
        };
        events
            .into_iter()
            .map(|event| event.into_core(root_id))
            .collect()
    }

    fn route(
        &mut self,
        root_id: &str,
        generation: u64,
        command: &crate::ClientCommand,
    ) -> Result<String, TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id) {
            return Err(TerminalConnectionError::Rejected);
        }
        crate::core::validate_command(command, MAX_WIRE_PRIVATE_BYTES)
            .map_err(|_error| TerminalConnectionError::Rejected)?;
        let response = self.transact(
            command.correlation_id(),
            &DaemonRequest::Command {
                root_id: String::from(root_id),
                generation,
                correlation_id: String::from(command.correlation_id()),
                class: String::from(command.class()),
                private_payload: serde_json::to_vec(command)
                    .map_err(|_error| TerminalConnectionError::Protocol)?,
            },
        )?;
        let DaemonResponse::CommandAccepted {
            safe_response_identity,
        } = response
        else {
            return Err(TerminalConnectionError::Rejected);
        };
        validate_hash(safe_response_identity.as_str())?;
        Ok(safe_response_identity)
    }

    fn stop(&mut self, root_id: &str, generation: u64) -> Result<(), TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id) {
            return Err(TerminalConnectionError::Rejected);
        }
        let correlation_id = request_identity("stop", root_id, "terminal", generation);
        let response = self.transact(
            correlation_id.as_str(),
            &DaemonRequest::Stop {
                root_id: String::from(root_id),
                generation,
            },
        )?;
        if response != DaemonResponse::Stopped {
            return Err(TerminalConnectionError::Rejected);
        }
        Ok(())
    }

    fn detach(&mut self, root_id: &str, client_id: &str) -> Result<(), TerminalConnectionError> {
        if self.attached_root.as_deref() != Some(root_id)
            || self.attached_client.as_deref() != Some(client_id)
        {
            return Err(TerminalConnectionError::Rejected);
        }
        let correlation_id = request_identity("detach", root_id, client_id, 0);
        let response = self.transact(
            correlation_id.as_str(),
            &DaemonRequest::Detach {
                root_id: String::from(root_id),
                client_id: String::from(client_id),
            },
        )?;
        if response != DaemonResponse::Detached {
            return Err(TerminalConnectionError::Rejected);
        }
        self.attached_root = None;
        self.attached_client = None;
        Ok(())
    }
}

/// Builds daemon mode only from an explicit capability-root path and configured endpoint name.
///
/// # Errors
///
/// Returns [`TerminalConnectionError`] for direct-mode configuration or endpoint failure.
pub fn connect_explicit_daemon(
    config: &crate::ValidatedTerminalConfig,
    endpoint_root: &Path,
    generation: u64,
    max_frame_bytes: u32,
) -> Result<DaemonTerminalConnection, TerminalConnectionError> {
    if config.connection_mode() != crate::ConnectionMode::Daemon {
        return Err(TerminalConnectionError::WrongMode);
    }
    let endpoint = animus_local_daemon::LocalEndpoint::new(endpoint_root, config.endpoint_name())
        .map_err(|_error| TerminalConnectionError::Unavailable)?;
    DaemonTerminalConnection::connect(&endpoint, generation, max_frame_bytes)
}

/// Bounded daemon protocol request.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DaemonRequest {
    /// Request snapshot-first attachment.
    Attach {
        /// Root identity.
        root_id: String,
        /// Client identity.
        client_id: String,
        /// Maximum initial events.
        maximum_events: u32,
    },
    /// Refresh one existing snapshot-first attachment.
    Refresh {
        /// Root identity.
        root_id: String,
        /// Existing client identity.
        client_id: String,
        /// Maximum later events.
        maximum_events: u32,
    },
    /// Request later events.
    EventsAfter {
        /// Root identity.
        root_id: String,
        /// Last observed sequence.
        sequence: u64,
        /// Maximum later events.
        maximum_events: u32,
    },
    /// Route one typed command class.
    Command {
        /// Root identity.
        root_id: String,
        /// Worker generation.
        generation: u64,
        /// Command correlation identity.
        correlation_id: String,
        /// Safe command class.
        class: String,
        /// Private typed command payload for the worker-owned session service.
        private_payload: Vec<u8>,
    },
    /// Request generation-fenced worker stop.
    Stop {
        /// Root identity.
        root_id: String,
        /// Expected worker generation.
        generation: u64,
    },
    /// Remove one client attachment lease.
    Detach {
        /// Root identity.
        root_id: String,
        /// Client identity.
        client_id: String,
    },
}

/// Bounded daemon protocol response.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DaemonResponse {
    /// Snapshot returned before optional later events.
    Snapshot {
        /// Safe snapshot metadata.
        snapshot: DaemonSnapshot,
        /// Safe initial events.
        events: Vec<DaemonEvent>,
    },
    /// Later safe events.
    Events {
        /// Bounded events.
        events: Vec<DaemonEvent>,
    },
    /// Typed command was accepted for worker routing.
    CommandAccepted {
        /// BLAKE3 identity of safe response metadata.
        safe_response_identity: String,
    },
    /// Worker stop was accepted by its current generation.
    Stopped,
    /// Client lease was removed without stopping worker execution.
    Detached,
    /// Typed bounded failure.
    Failed {
        /// Safe failure class.
        class: String,
    },
}

/// Daemon snapshot wire shape.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DaemonSnapshot {
    /// Root identity.
    pub root_id: String,
    /// Worker identity.
    pub worker_id: String,
    /// Worker generation.
    pub generation: u64,
    /// Latest event sequence.
    pub event_sequence: u64,
    /// Safe restore status.
    pub restore_status: animus_local_daemon::RestoreStatus,
}

impl DaemonSnapshot {
    fn into_core(
        self,
        events: Vec<DaemonEvent>,
    ) -> Result<crate::ClientSnapshot, TerminalConnectionError> {
        let core_events = events
            .into_iter()
            .map(|event| event.into_core(self.root_id.as_str()))
            .collect::<Result<_, _>>()?;
        Ok(crate::ClientSnapshot {
            root_id: self.root_id,
            worker_id: self.worker_id,
            generation: self.generation,
            event_sequence: self.event_sequence,
            restore_status: self.restore_status,
            events: core_events,
        })
    }
}

/// Daemon event wire shape.
#[derive(Clone, Deserialize, Eq, PartialEq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum DaemonEvent {
    /// One safe class and observation identity.
    Observation {
        /// Monotonic event sequence.
        sequence: u64,
        /// Safe event class.
        class: String,
        /// BLAKE3 observation identity.
        observation_identity: String,
    },
    /// One typed metadata-only worker state inspection.
    StateInspection {
        /// Monotonic event sequence.
        sequence: u64,
        /// BLAKE3 identity of the state-inspection request.
        correlation_id: String,
        /// BLAKE3 observation identity.
        observation_identity: String,
        /// Safe bounded worker state.
        state: animus_local_daemon::WorkerStateInspection,
    },
    /// One bounded private payload routed without supervisor decoding.
    Private {
        /// Monotonic event sequence.
        sequence: u64,
        /// Safe private event class.
        class: String,
        /// BLAKE3 identity of the exact payload and event facts.
        payload_identity: String,
        /// Opaque private payload for terminal-owned decoding.
        private_payload: Vec<u8>,
    },
}

impl DaemonEvent {
    fn into_core(
        self,
        expected_root_id: &str,
    ) -> Result<crate::ClientSessionEvent, TerminalConnectionError> {
        match self {
            Self::Observation {
                sequence,
                class,
                observation_identity,
            } => {
                validate_hash(observation_identity.as_str())?;
                if sequence == 0 || class.is_empty() {
                    return Err(TerminalConnectionError::Protocol);
                }
                Ok(crate::ClientSessionEvent::Observation {
                    sequence,
                    class,
                    observation_identity,
                })
            }
            Self::StateInspection {
                sequence,
                correlation_id,
                observation_identity,
                state,
            } => {
                validate_hash(correlation_id.as_str())?;
                validate_hash(observation_identity.as_str())?;
                if sequence == 0 || state.validate().is_err() {
                    return Err(TerminalConnectionError::Protocol);
                }
                Ok(crate::ClientSessionEvent::StateInspection {
                    sequence,
                    correlation_id,
                    observation_identity,
                    state,
                })
            }
            Self::Private {
                sequence,
                class,
                payload_identity,
                private_payload,
            } => private_client_event(
                animus_local_daemon::PrivateEventIdentityFacts {
                    root_id: expected_root_id,
                    sequence,
                    class: class.as_str(),
                },
                payload_identity.as_str(),
                private_payload.as_slice(),
            ),
        }
    }
}

impl core::fmt::Debug for DaemonEvent {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Observation {
                sequence,
                class,
                observation_identity,
            } => formatter
                .debug_struct("Observation")
                .field("sequence", sequence)
                .field("class", class)
                .field("observation_identity", observation_identity)
                .finish(),
            Self::StateInspection {
                sequence,
                correlation_id,
                observation_identity,
                state,
            } => formatter
                .debug_struct("StateInspection")
                .field("sequence", sequence)
                .field("correlation_id", correlation_id)
                .field("observation_identity", observation_identity)
                .field("state", state)
                .finish(),
            Self::Private {
                sequence,
                class,
                payload_identity,
                private_payload,
            } => formatter
                .debug_struct("Private")
                .field("sequence", sequence)
                .field("class", class)
                .field("payload_identity", payload_identity)
                .field("private_payload_bytes", &private_payload.len())
                .finish(),
        }
    }
}

/// Explicit connection failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminalConnectionError {
    /// Selected endpoint or service was unavailable.
    Unavailable,
    /// Wire framing or typed payload validation failed.
    Protocol,
    /// Worker-owned service rejected the operation.
    Rejected,
    /// Configuration selected a different explicit mode.
    WrongMode,
}

impl core::fmt::Display for TerminalConnectionError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "terminal connection rejected: {self:?}")
    }
}

impl std::error::Error for TerminalConnectionError {}

impl TerminalConnectionError {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Unavailable => "unavailable",
            Self::Protocol => "protocol",
            Self::Rejected => "rejected",
            Self::WrongMode => "wrong_mode",
        }
    }
}

fn snapshot_from_daemon(
    snapshot: animus_local_daemon::WorkerSnapshot,
    events: Vec<animus_local_daemon::WorkerEvent>,
) -> Result<crate::ClientSnapshot, TerminalConnectionError> {
    Ok(crate::ClientSnapshot {
        root_id: snapshot.root_id,
        worker_id: snapshot.worker_id,
        generation: snapshot.generation,
        event_sequence: snapshot.event_sequence,
        restore_status: snapshot.restore_status,
        events: events
            .into_iter()
            .map(client_event_from_worker)
            .collect::<Result<_, TerminalConnectionError>>()?,
    })
}

fn client_event_from_worker(
    event: animus_local_daemon::WorkerEvent,
) -> Result<crate::ClientSessionEvent, TerminalConnectionError> {
    match event {
        animus_local_daemon::WorkerEvent::Observation {
            sequence,
            class,
            observation_identity,
            ..
        } => {
            validate_hash(observation_identity.as_str())?;
            Ok(crate::ClientSessionEvent::Observation {
                sequence,
                class,
                observation_identity,
            })
        }
        animus_local_daemon::WorkerEvent::StateInspection {
            sequence,
            correlation_id,
            observation_identity,
            state,
            ..
        } => {
            validate_hash(correlation_id.as_str())?;
            validate_hash(observation_identity.as_str())?;
            state
                .validate()
                .map_err(|_error| TerminalConnectionError::Protocol)?;
            Ok(crate::ClientSessionEvent::StateInspection {
                sequence,
                correlation_id,
                observation_identity,
                state,
            })
        }
        animus_local_daemon::WorkerEvent::Private(event) => private_client_event(
            animus_local_daemon::PrivateEventIdentityFacts {
                root_id: event.root_id(),
                sequence: event.sequence(),
                class: event.class(),
            },
            event.payload_identity(),
            event.private_payload(),
        ),
    }
}

// r[impl agent_harness_runtime.live_terminal_agent_host.private_events]
fn private_client_event(
    facts: animus_local_daemon::PrivateEventIdentityFacts<'_>,
    payload_identity: &str,
    private_payload: &[u8],
) -> Result<crate::ClientSessionEvent, TerminalConnectionError> {
    if facts.class != PRIVATE_RUNTIME_EVENT_CLASS {
        return Err(TerminalConnectionError::Protocol);
    }
    animus_local_daemon::validate_private_event_payload(
        facts,
        payload_identity,
        private_payload,
        MAX_WIRE_PRIVATE_BYTES,
    )
    .map_err(|_error| TerminalConnectionError::Protocol)?;
    let event = animus_preserves::from_versioned_bytes(private_payload)
        .map_err(|_error| TerminalConnectionError::Protocol)?
        .try_into_runtime_event()
        .map_err(|_error| TerminalConnectionError::Protocol)?;
    Ok(crate::ClientSessionEvent::Runtime {
        sequence: facts.sequence,
        event,
    })
}

fn request_identity(class: &str, root_id: &str, client_id: &str, sequence: u64) -> String {
    let material = format!(
        "animus.live-terminal.daemon-request.v1\0{class}\0{root_id}\0{client_id}\0{sequence}"
    );
    blake3::hash(material.as_bytes()).to_hex().to_string()
}

fn valid_command_class(class: &str) -> bool {
    matches!(
        class,
        "prompt"
            | "queue_edit"
            | "queue_remove"
            | "approval"
            | "cancel"
            | "kernel_clear"
            | "state_inspection"
            | "child"
            | "session"
    )
}

fn validate_hash(value: &str) -> Result<(), TerminalConnectionError> {
    if value.len() != animus_adapter_daemon::BLAKE3_HEX_CHARS
        || !value.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(TerminalConnectionError::Protocol);
    }
    Ok(())
}
