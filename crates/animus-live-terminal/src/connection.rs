//! Explicit daemon and direct connection adapters.

use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

const TRANSPORT_VERSION: u16 = 1;
const TRANSPORT_DEADLINE_MS: u64 = 30_000;
const MAX_WIRE_PRIVATE_BYTES: u32 = 65_536;

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
            .map(|event| {
                validate_hash(event.observation_identity.as_str())?;
                Ok(crate::ClientSessionEvent::Observation {
                    sequence: event.sequence,
                    class: event.class,
                    observation_identity: event.observation_identity,
                })
            })
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

    /// Dispatches one bounded protocol request without executing worker-owned session effects.
    #[must_use]
    pub fn dispatch(&mut self, request: DaemonRequest) -> DaemonResponse {
        self.try_dispatch(request)
            .unwrap_or_else(|error| DaemonResponse::Failed {
                class: String::from(error.as_str()),
            })
    }

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
    DaemonEvent {
        sequence: event.sequence,
        class: event.class,
        observation_identity: event.observation_identity,
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
        events.into_iter().map(DaemonEvent::into_core).collect()
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
        Ok(crate::ClientSnapshot {
            root_id: self.root_id,
            worker_id: self.worker_id,
            generation: self.generation,
            event_sequence: self.event_sequence,
            restore_status: self.restore_status,
            events: events
                .into_iter()
                .map(DaemonEvent::into_core)
                .collect::<Result<_, _>>()?,
        })
    }
}

/// Daemon event wire shape.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct DaemonEvent {
    /// Monotonic event sequence.
    pub sequence: u64,
    /// Safe event class.
    pub class: String,
    /// BLAKE3 observation identity.
    pub observation_identity: String,
}

impl DaemonEvent {
    fn into_core(self) -> Result<crate::ClientSessionEvent, TerminalConnectionError> {
        validate_hash(self.observation_identity.as_str())?;
        if self.sequence == 0 || self.class.is_empty() {
            return Err(TerminalConnectionError::Protocol);
        }
        Ok(crate::ClientSessionEvent::Observation {
            sequence: self.sequence,
            class: self.class,
            observation_identity: self.observation_identity,
        })
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
            .map(|event| {
                validate_hash(event.observation_identity.as_str())?;
                Ok(crate::ClientSessionEvent::Observation {
                    sequence: event.sequence,
                    class: event.class,
                    observation_identity: event.observation_identity,
                })
            })
            .collect::<Result<_, TerminalConnectionError>>()?,
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
