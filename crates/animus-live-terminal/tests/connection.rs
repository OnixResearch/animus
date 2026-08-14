//! Explicit daemon and injected direct connection tests.

use animus_live_terminal::{
    ClientCommand, DaemonRequest, DaemonRequestDispatcher, DaemonResponse,
    DaemonTerminalConnection, DirectTerminalConnection, TerminalConnectionError,
    TerminalConnectionPort,
};
use std::sync::{Arc, Mutex};

const CONFIG: &[u8] = include_bytes!("../../../config/generated/local-daemon-supervisor.json");
const TERMINAL_CONFIG: &[u8] =
    include_bytes!("../../../config/generated/live-terminal-agent-host.json");
const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const MAX_FRAME_BYTES: u32 = 1_048_576;
const MAX_EVENTS: u32 = 8;
const MAX_WORKERS: u16 = 8;
const GENERATION: u64 = 1;
const DAEMON_REQUEST_COUNT: usize = 4;

#[test]
fn explicit_daemon_client_routes_typed_private_command_without_discovery_or_fallback() {
    let directory = tempfile::tempdir().expect("endpoint directory");
    let endpoint = animus_local_daemon::LocalEndpoint::new(directory.path(), "terminal.sock")
        .expect("explicit endpoint");
    let endpoint_identity = String::from(endpoint.identity());
    let payloads = Arc::new(Mutex::new(Vec::new()));
    let config = animus_local_daemon::parse_local_daemon_config(CONFIG).expect("daemon config");
    let registry = animus_local_daemon::WorkerRegistry::new(
        vec![record(animus_local_daemon::WorkerLifecycle::Ready)],
        MAX_WORKERS,
    )
    .expect("registry");
    let service = animus_local_daemon::LocalSupervisorService::new(
        config,
        registry,
        FixtureWorker {
            payloads: Arc::clone(&payloads),
        },
    );
    let listener = animus_local_daemon::LocalListener::bind(endpoint.clone()).expect("listener");
    let server = std::thread::spawn(move || {
        let mut dispatcher = DaemonRequestDispatcher::new(service);
        let (mut client, _peer) = listener.accept().expect("accept");
        for _request_index in 0..DAEMON_REQUEST_COUNT {
            let request = client
                .read_frame::<DaemonRequest>(MAX_FRAME_BYTES)
                .expect("request frame");
            let response = dispatcher.dispatch(request.payload);
            client
                .write_frame(
                    &animus_local_daemon::TransportFrame {
                        version: request.version,
                        endpoint_identity: request.endpoint_identity,
                        generation: request.generation,
                        correlation_id: request.correlation_id,
                        payload: response,
                    },
                    MAX_FRAME_BYTES,
                )
                .expect("response frame");
        }
        listener.close().expect("listener close");
    });

    let mut connection = DaemonTerminalConnection::connect(&endpoint, GENERATION, MAX_FRAME_BYTES)
        .expect("explicit daemon connection");
    let premature = ClientCommand::Prompt {
        correlation_id: String::from(HASH),
        text: String::from("must not route"),
    };
    assert_eq!(
        connection.route("root-direct", GENERATION, &premature),
        Err(TerminalConnectionError::Rejected)
    );
    let snapshot = connection
        .attach("root-direct", "client-daemon", MAX_EVENTS)
        .expect("snapshot-first attach");
    assert_eq!(snapshot.events.len(), 1);
    let forged = ClientCommand::Prompt {
        correlation_id: String::from("forged"),
        text: String::from("must not route"),
    };
    assert_eq!(
        connection.route("root-direct", GENERATION, &forged),
        Err(TerminalConnectionError::Rejected)
    );
    assert!(matches!(
        snapshot.events[0],
        animus_live_terminal::ClientSessionEvent::Observation { sequence: 2, .. }
    ));
    let later = connection
        .events_after("root-direct", 2, MAX_EVENTS)
        .expect("later daemon events");
    assert!(matches!(
        later[0],
        animus_live_terminal::ClientSessionEvent::Observation { sequence: 3, .. }
    ));
    let command = ClientCommand::Prompt {
        correlation_id: String::from(HASH),
        text: String::from("private daemon prompt"),
    };
    assert_eq!(
        connection
            .route("root-direct", GENERATION, &command)
            .expect("route"),
        HASH
    );
    connection
        .detach("root-direct", "client-daemon")
        .expect("detach only");
    server.join().expect("server");
    let payloads = payloads.lock().expect("payload lock");
    assert_eq!(payloads.len(), 1);
    assert!(
        std::str::from_utf8(payloads[0].as_slice())
            .expect("typed payload")
            .contains("private daemon prompt")
    );
    assert_eq!(endpoint.identity(), endpoint_identity);
}

#[test]
fn injected_direct_service_receives_private_payload_only_when_route_is_called() {
    let payloads = Arc::new(Mutex::new(Vec::new()));
    let worker = FixtureWorker {
        payloads: Arc::clone(&payloads),
    };
    let config = animus_local_daemon::parse_local_daemon_config(CONFIG).expect("daemon config");
    let registry = animus_local_daemon::WorkerRegistry::new(
        vec![record(animus_local_daemon::WorkerLifecycle::Ready)],
        MAX_WORKERS,
    )
    .expect("registry");
    let service = animus_local_daemon::LocalSupervisorService::new(config, registry, worker);
    let mut connection = DirectTerminalConnection::new(service);
    let premature = ClientCommand::Prompt {
        correlation_id: String::from(HASH),
        text: String::from("must not route"),
    };
    assert_eq!(
        connection.route("root-direct", GENERATION, &premature),
        Err(TerminalConnectionError::Rejected)
    );
    assert!(payloads.lock().expect("payload lock").is_empty());
    let snapshot = connection
        .attach("root-direct", "client-direct", MAX_EVENTS)
        .expect("direct attach");
    assert_eq!(snapshot.events.len(), 1);
    assert!(payloads.lock().expect("payload lock").is_empty());
    let forged = ClientCommand::Prompt {
        correlation_id: String::from("forged"),
        text: String::from("must not route"),
    };
    assert_eq!(
        connection.route("root-direct", GENERATION, &forged),
        Err(TerminalConnectionError::Rejected)
    );
    assert!(payloads.lock().expect("payload lock").is_empty());

    let command = ClientCommand::Prompt {
        correlation_id: String::from(HASH),
        text: String::from("private direct prompt"),
    };
    connection
        .route("root-direct", GENERATION, &command)
        .expect("direct route");
    let payloads = payloads.lock().expect("payload lock");
    assert_eq!(payloads.len(), 1);
    assert!(
        std::str::from_utf8(payloads[0].as_slice())
            .expect("payload UTF-8")
            .contains("private direct prompt")
    );
}

#[test]
fn daemon_dispatcher_uses_the_supervisor_service_and_tracks_one_attachment() {
    let payloads = Arc::new(Mutex::new(Vec::new()));
    let worker = FixtureWorker {
        payloads: Arc::clone(&payloads),
    };
    let config = animus_local_daemon::parse_local_daemon_config(CONFIG).expect("daemon config");
    let registry = animus_local_daemon::WorkerRegistry::new(
        vec![record(animus_local_daemon::WorkerLifecycle::Ready)],
        MAX_WORKERS,
    )
    .expect("registry");
    let service = animus_local_daemon::LocalSupervisorService::new(config, registry, worker);
    let mut dispatcher = DaemonRequestDispatcher::new(service);
    let attached = dispatcher.dispatch(DaemonRequest::Attach {
        root_id: String::from("root-direct"),
        client_id: String::from("client-dispatch"),
        maximum_events: MAX_EVENTS,
    });
    assert!(matches!(
        attached,
        DaemonResponse::Snapshot { ref events, .. } if events[0].sequence == 2
    ));
    let events = dispatcher.dispatch(DaemonRequest::EventsAfter {
        root_id: String::from("root-direct"),
        sequence: 2,
        maximum_events: MAX_EVENTS,
    });
    assert!(matches!(
        events,
        DaemonResponse::Events { ref events } if events[0].sequence == 3
    ));
    assert_eq!(
        dispatcher.dispatch(DaemonRequest::Command {
            root_id: String::from("root-direct"),
            generation: GENERATION,
            correlation_id: String::from(HASH),
            class: String::from("provider_execute"),
            private_payload: vec![1],
        }),
        DaemonResponse::Failed {
            class: String::from("protocol"),
        }
    );
    assert!(payloads.lock().expect("payload lock").is_empty());
    let command = ClientCommand::Prompt {
        correlation_id: String::from(HASH),
        text: String::from("private dispatcher prompt"),
    };
    let routed = dispatcher.dispatch(DaemonRequest::Command {
        root_id: String::from("root-direct"),
        generation: GENERATION,
        correlation_id: String::from(command.correlation_id()),
        class: String::from(command.class()),
        private_payload: serde_json::to_vec(&command).expect("typed command"),
    });
    assert_eq!(
        routed,
        DaemonResponse::CommandAccepted {
            safe_response_identity: String::from(HASH),
        }
    );
    assert_eq!(payloads.lock().expect("payload lock").len(), 1);
    assert_eq!(
        dispatcher.dispatch(DaemonRequest::Detach {
            root_id: String::from("root-direct"),
            client_id: String::from("client-dispatch"),
        }),
        DaemonResponse::Detached
    );
    assert_eq!(
        dispatcher.dispatch(DaemonRequest::EventsAfter {
            root_id: String::from("root-direct"),
            sequence: 3,
            maximum_events: MAX_EVENTS,
        }),
        DaemonResponse::Failed {
            class: String::from("rejected"),
        }
    );
    assert_eq!(
        dispatcher.dispatch(DaemonRequest::Command {
            root_id: String::from("root-direct"),
            generation: GENERATION,
            correlation_id: String::from(HASH),
            class: String::from("prompt"),
            private_payload: Vec::new(),
        }),
        DaemonResponse::Failed {
            class: String::from("rejected"),
        }
    );
}

#[test]
fn missing_endpoint_and_mismatched_response_identity_fail_without_fallback() {
    let directory = tempfile::tempdir().expect("endpoint directory");
    let missing = animus_local_daemon::LocalEndpoint::new(directory.path(), "missing.sock")
        .expect("explicit missing endpoint");
    assert!(matches!(
        DaemonTerminalConnection::connect(&missing, GENERATION, MAX_FRAME_BYTES),
        Err(TerminalConnectionError::Unavailable)
    ));

    let mismatch = animus_local_daemon::LocalEndpoint::new(directory.path(), "mismatch.sock")
        .expect("mismatch endpoint");
    let listener = animus_local_daemon::LocalListener::bind(mismatch.clone()).expect("listener");
    let server = std::thread::spawn(move || {
        let (mut client, _peer) = listener.accept().expect("accept");
        let request = client
            .read_frame::<DaemonRequest>(MAX_FRAME_BYTES)
            .expect("request");
        client
            .write_frame(
                &animus_local_daemon::TransportFrame {
                    version: request.version,
                    endpoint_identity: String::from(HASH),
                    generation: request.generation,
                    correlation_id: request.correlation_id,
                    payload: DaemonResponse::Failed {
                        class: String::from("rejected"),
                    },
                },
                MAX_FRAME_BYTES,
            )
            .expect("mismatched response");
        listener.close().expect("listener close");
    });
    let mut connection = DaemonTerminalConnection::connect(&mismatch, GENERATION, MAX_FRAME_BYTES)
        .expect("connect mismatch fixture");
    assert_eq!(
        connection.attach("root-direct", "client-mismatch", MAX_EVENTS),
        Err(TerminalConnectionError::Protocol)
    );
    server.join().expect("server");

    let mut direct: serde_json::Value =
        serde_json::from_slice(TERMINAL_CONFIG).expect("terminal config");
    direct["connection_mode"] = serde_json::Value::String(String::from("direct"));
    let direct = animus_live_terminal::parse_terminal_config(
        serde_json::to_vec(&direct)
            .expect("direct config")
            .as_slice(),
    )
    .expect("direct mode");
    assert!(matches!(
        animus_live_terminal::connect_explicit_daemon(
            &direct,
            directory.path(),
            GENERATION,
            MAX_FRAME_BYTES,
        ),
        Err(TerminalConnectionError::WrongMode)
    ));
}

struct FixtureWorker {
    payloads: Arc<Mutex<Vec<Vec<u8>>>>,
}

impl animus_local_daemon::WorkerSessionPort for FixtureWorker {
    fn snapshot(
        &mut self,
        root_id: &str,
    ) -> Result<animus_local_daemon::WorkerSnapshot, animus_local_daemon::ServiceError> {
        Ok(animus_local_daemon::WorkerSnapshot {
            root_id: String::from(root_id),
            worker_id: String::from("worker-direct"),
            generation: GENERATION,
            event_sequence: 1,
            restore_status: animus_local_daemon::RestoreStatus::Restored,
        })
    }

    fn events_after(
        &mut self,
        root_id: &str,
        sequence: u64,
        _maximum: u32,
    ) -> Result<Vec<animus_local_daemon::WorkerEvent>, animus_local_daemon::ServiceError> {
        Ok(vec![animus_local_daemon::WorkerEvent {
            root_id: String::from(root_id),
            sequence: sequence + 1,
            class: String::from("attached"),
            observation_identity: String::from(HASH),
        }])
    }

    fn route(
        &mut self,
        command: &animus_local_daemon::WorkerCommand,
    ) -> Result<String, animus_local_daemon::ServiceError> {
        self.payloads
            .lock()
            .expect("payload lock")
            .push(command.private_payload.clone());
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
    ) -> Result<animus_local_daemon::WorkerSnapshot, animus_local_daemon::ServiceError> {
        Ok(animus_local_daemon::WorkerSnapshot {
            root_id: String::from(root_id),
            worker_id: String::from("worker-direct"),
            generation,
            event_sequence: 0,
            restore_status: animus_local_daemon::RestoreStatus::Restored,
        })
    }
}

fn record(
    lifecycle: animus_local_daemon::WorkerLifecycle,
) -> animus_local_daemon::WorkerRegistryRecord {
    animus_local_daemon::WorkerRegistryRecord {
        root_id: String::from("root-direct"),
        worker_id: String::from("worker-direct"),
        generation: GENERATION,
        process_id: 42,
        process_identity: String::from(HASH),
        endpoint_identity: String::from(HASH),
        session_head_identity: String::from(HASH),
        lifecycle,
        restore_status: animus_local_daemon::RestoreStatus::Restored,
        attachments: Vec::new(),
        event_sequence: 1,
        receipt_identity: String::from(HASH),
    }
}
