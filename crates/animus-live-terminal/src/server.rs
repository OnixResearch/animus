//! Bounded authenticated daemon request loop over the local transport.

/// Explicit finite daemon serving policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DaemonServerLimits {
    /// Exact worker generation admitted on every frame.
    pub generation: u64,
    /// Expected local user identity from kernel peer credentials.
    pub expected_user_id: u32,
    /// Maximum accepted client connections.
    pub maximum_clients: u16,
    /// Maximum requests read from one client.
    pub maximum_requests_per_client: u32,
    /// Maximum encoded frame bytes.
    pub maximum_frame_bytes: u32,
    /// Read and write deadline in milliseconds.
    pub deadline_ms: u64,
}

impl DaemonServerLimits {
    fn validate(self) -> Result<Self, DaemonServerError> {
        if self.generation == 0
            || self.maximum_clients == 0
            || self.maximum_requests_per_client == 0
            || self.maximum_frame_bytes == 0
            || self.deadline_ms == 0
        {
            return Err(DaemonServerError::Bound);
        }
        Ok(self)
    }
}

/// Metadata-only daemon serving outcome.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DaemonServerOutcome {
    /// Accepted authenticated clients.
    pub accepted_clients: u16,
    /// Dispatched request frames.
    pub dispatched_requests: u32,
    /// Connections that ended without an explicit detach request.
    pub disconnected_clients: u16,
}

/// Runs one finite local daemon server over an injected dispatcher.
///
/// Each connection shares the same supervisor through the dispatcher. Abrupt
/// disconnect removes its attachment before the next client is accepted.
///
/// # Errors
///
/// Returns [`DaemonServerError`] for invalid bounds, peer mismatch, protocol
/// drift, transport failure, counter overflow, or attachment cleanup failure.
pub fn run_bounded_daemon<Worker: animus_local_daemon::WorkerSessionPort>(
    listener: animus_local_daemon::LocalListener,
    dispatcher: &mut crate::DaemonRequestDispatcher<Worker>,
    limits: DaemonServerLimits,
) -> Result<DaemonServerOutcome, DaemonServerError> {
    let limits = limits.validate()?;
    let endpoint_identity = String::from(listener.endpoint().identity());
    let mut outcome = DaemonServerOutcome {
        accepted_clients: 0,
        dispatched_requests: 0,
        disconnected_clients: 0,
    };
    for _client_index in 0..limits.maximum_clients {
        let (mut client, peer) = listener.accept().map_err(DaemonServerError::Transport)?;
        if peer.user_id != limits.expected_user_id {
            return Err(DaemonServerError::Peer);
        }
        client
            .set_deadline(std::time::Duration::from_millis(limits.deadline_ms))
            .map_err(DaemonServerError::Transport)?;
        outcome.accepted_clients = outcome
            .accepted_clients
            .checked_add(1)
            .ok_or(DaemonServerError::Overflow)?;
        let explicitly_detached = serve_client(
            &mut client,
            dispatcher,
            endpoint_identity.as_str(),
            limits,
            &mut outcome,
        )?;
        if !explicitly_detached {
            dispatcher
                .disconnect()
                .map_err(|_error| DaemonServerError::Cleanup)?;
            outcome.disconnected_clients = outcome
                .disconnected_clients
                .checked_add(1)
                .ok_or(DaemonServerError::Overflow)?;
        }
    }
    listener.close().map_err(DaemonServerError::Transport)?;
    Ok(outcome)
}

#[allow(
    clippy::too_many_arguments,
    reason = "the daemon shell receives four explicit capabilities plus its bounded outcome"
)]
fn serve_client<Worker: animus_local_daemon::WorkerSessionPort>(
    client: &mut animus_local_daemon::LocalClient,
    dispatcher: &mut crate::DaemonRequestDispatcher<Worker>,
    endpoint_identity: &str,
    limits: DaemonServerLimits,
    outcome: &mut DaemonServerOutcome,
) -> Result<bool, DaemonServerError> {
    for _request_index in 0..limits.maximum_requests_per_client {
        let request = match client.read_frame::<crate::DaemonRequest>(limits.maximum_frame_bytes) {
            Ok(request) => request,
            Err(_error) => return Ok(false),
        };
        if request.endpoint_identity != endpoint_identity || request.generation != limits.generation
        {
            return Err(DaemonServerError::Protocol);
        }
        let response_payload = dispatcher.dispatch(request.payload);
        let explicitly_detached = response_payload == crate::DaemonResponse::Detached;
        let response = animus_local_daemon::TransportFrame {
            version: request.version,
            endpoint_identity: request.endpoint_identity,
            generation: request.generation,
            correlation_id: request.correlation_id,
            payload: response_payload,
        };
        client
            .write_frame(&response, limits.maximum_frame_bytes)
            .map_err(DaemonServerError::Transport)?;
        outcome.dispatched_requests = outcome
            .dispatched_requests
            .checked_add(1)
            .ok_or(DaemonServerError::Overflow)?;
        if explicitly_detached {
            return Ok(true);
        }
    }
    Ok(false)
}

/// Bounded daemon server failure class.
#[derive(Debug)]
pub enum DaemonServerError {
    /// A finite policy value was zero.
    Bound,
    /// Kernel peer identity did not match the configured local user.
    Peer,
    /// Frame endpoint or generation differed from the server.
    Protocol,
    /// A serving counter overflowed.
    Overflow,
    /// Local transport failed.
    Transport(animus_local_daemon::TransportError),
    /// Disconnect lease cleanup failed.
    Cleanup,
}

impl core::fmt::Display for DaemonServerError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "bounded daemon server failed: {self:?}")
    }
}

impl std::error::Error for DaemonServerError {}

#[cfg(test)]
mod tests {
    use crate::TerminalConnectionPort as _;
    use std::os::unix::fs::MetadataExt as _;

    const CONFIG: &[u8] = include_bytes!("../../../config/generated/local-daemon-supervisor.json");
    const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const GENERATION: u64 = 1;
    const MAX_FRAME_BYTES: u32 = 1_048_576;
    const MAX_EVENTS: u32 = 8;
    const MAX_WORKERS: u16 = 8;
    const MAX_CLIENTS: u16 = 1;
    const MAX_REQUESTS: u32 = 2;
    const DEADLINE_MS: u64 = 1_000;

    #[test]
    fn bounded_server_accepts_attach_and_explicit_detach() {
        let directory = tempfile::tempdir().expect("server endpoint directory");
        let endpoint = animus_local_daemon::LocalEndpoint::new(directory.path(), "server.sock")
            .expect("server endpoint");
        let listener =
            animus_local_daemon::LocalListener::bind(endpoint.clone()).expect("server listener");
        let user_id = directory
            .path()
            .metadata()
            .expect("endpoint root metadata")
            .uid();
        let server = std::thread::spawn(move || {
            let mut dispatcher = crate::DaemonRequestDispatcher::new(fixture_service());
            super::run_bounded_daemon(
                listener,
                &mut dispatcher,
                super::DaemonServerLimits {
                    generation: GENERATION,
                    expected_user_id: user_id,
                    maximum_clients: MAX_CLIENTS,
                    maximum_requests_per_client: MAX_REQUESTS,
                    maximum_frame_bytes: MAX_FRAME_BYTES,
                    deadline_ms: DEADLINE_MS,
                },
            )
            .expect("bounded server")
        });
        let mut connection =
            crate::DaemonTerminalConnection::connect(&endpoint, GENERATION, MAX_FRAME_BYTES)
                .expect("daemon connection");
        connection
            .attach("root-server", "client-server", MAX_EVENTS)
            .expect("server attach");
        connection
            .detach("root-server", "client-server")
            .expect("server detach");
        let outcome = server.join().expect("server join");
        assert_eq!(outcome.accepted_clients, MAX_CLIENTS);
        assert_eq!(outcome.dispatched_requests, MAX_REQUESTS);
        assert_eq!(outcome.disconnected_clients, 0);
    }

    #[test]
    fn zero_server_bound_fails_before_transport() {
        let limits = super::DaemonServerLimits {
            generation: GENERATION,
            expected_user_id: 0,
            maximum_clients: 0,
            maximum_requests_per_client: MAX_REQUESTS,
            maximum_frame_bytes: MAX_FRAME_BYTES,
            deadline_ms: DEADLINE_MS,
        };
        assert!(matches!(
            limits.validate(),
            Err(super::DaemonServerError::Bound)
        ));
    }

    fn fixture_service() -> animus_local_daemon::LocalSupervisorService<FixtureWorker> {
        let config = animus_local_daemon::parse_local_daemon_config(CONFIG).expect("daemon config");
        let registry = animus_local_daemon::WorkerRegistry::new(
            vec![animus_local_daemon::WorkerRegistryRecord {
                root_id: String::from("root-server"),
                worker_id: String::from("worker-server"),
                generation: GENERATION,
                process_id: 1,
                process_identity: String::from(HASH),
                endpoint_identity: String::from(HASH),
                session_head_identity: String::from(HASH),
                lifecycle: animus_local_daemon::WorkerLifecycle::Ready,
                restore_status: animus_local_daemon::RestoreStatus::Fresh,
                attachments: Vec::new(),
                event_sequence: 0,
                receipt_identity: String::from(HASH),
            }],
            MAX_WORKERS,
        )
        .expect("server registry");
        animus_local_daemon::LocalSupervisorService::new(config, registry, FixtureWorker)
    }

    struct FixtureWorker;

    impl animus_local_daemon::WorkerSessionPort for FixtureWorker {
        fn snapshot(
            &mut self,
            root_id: &str,
        ) -> Result<animus_local_daemon::WorkerSnapshot, animus_local_daemon::ServiceError>
        {
            Ok(animus_local_daemon::WorkerSnapshot {
                root_id: String::from(root_id),
                worker_id: String::from("worker-server"),
                generation: GENERATION,
                event_sequence: 0,
                restore_status: animus_local_daemon::RestoreStatus::Fresh,
            })
        }

        fn events_after(
            &mut self,
            _root_id: &str,
            _sequence: u64,
            _maximum: u32,
        ) -> Result<Vec<animus_local_daemon::WorkerEvent>, animus_local_daemon::ServiceError>
        {
            Ok(Vec::new())
        }

        fn route(
            &mut self,
            _command: &animus_local_daemon::WorkerCommand,
        ) -> Result<String, animus_local_daemon::ServiceError> {
            Ok(String::from(HASH))
        }

        fn request_stop(
            &mut self,
            _root_id: &str,
            _generation: u64,
        ) -> Result<(), animus_local_daemon::ServiceError> {
            Ok(())
        }

        fn retry(
            &mut self,
            root_id: &str,
            generation: u64,
        ) -> Result<animus_local_daemon::WorkerSnapshot, animus_local_daemon::ServiceError>
        {
            Ok(animus_local_daemon::WorkerSnapshot {
                root_id: String::from(root_id),
                worker_id: String::from("worker-server"),
                generation,
                event_sequence: 0,
                restore_status: animus_local_daemon::RestoreStatus::Fresh,
            })
        }
    }
}
