//! Explicit daemon and injected direct connection tests.

use animus_live_terminal::{
    ClientCommand, DaemonRequest, DaemonRequestDispatcher, DaemonResponse,
    DaemonTerminalConnection, DirectTerminalConnection, TerminalClientState,
    TerminalConnectionError, TerminalConnectionPort, TerminalLimits,
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
const INITIAL_EVENT_SEQUENCE: u64 = 1;
const FIRST_EVENT_SEQUENCE: u64 = 2;
const SECOND_EVENT_SEQUENCE: u64 = 3;
const DAEMON_REQUEST_COUNT: usize = 4;
const PRIVATE_DAEMON_REQUEST_COUNT: usize = 1;
const STATE_EVENT_FIELD_COUNT: usize = 5;
const PRIVATE_RUNTIME_EVENT_CLASS: &str = "runtime_event";
const PRIVATE_USER_PROMPT: &str = "private user prompt";
const PRIVATE_ASSISTANT_RESPONSE: &str = "private assistant response";
const PRIVATE_TOOL_NAME: &str = "private_tool";
const PRIVATE_TOOL_ARGUMENT: &str = "query";
const PRIVATE_TOOL_ARGUMENT_VALUE: &str = "private argument";
const PRIVATE_TOOL_RESULT: &str = "private tool result";
const PRIVATE_TOOL_SCHEMA: &str = "schema.private-tool.v1";
const PRIVATE_TOOL_CALL_ID: &str = "private-tool-call";
const PRIVATE_TOOL_MUTATION_TARGET: &str = "resource:private-tool";
const PRIVATE_TOOL_CAPABILITY: &str = "tool.private";
const PRIVATE_TOOL_VISIBLE_BUDGET: u32 = 1_024;
const PRIVATE_SESSION_ID: &str = "session.private";
const PRIVATE_RUN_ID: &str = "run.private";
const PRIVATE_SESSION_STATUS: &str = "active";
const VIEWPORT: (u16, u16) = (80, 24);
const MAX_PROMPT_BYTES: u32 = 4_096;
const MAX_VIEWPORT_WIDTH: u16 = 512;
const MAX_VIEWPORT_HEIGHT: u16 = 256;

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
            private_event_mode: PrivateEventMode::None,
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
    assert_client_state_inspection(&snapshot.events[0], FIRST_EVENT_SEQUENCE);
    let later = connection
        .events_after("root-direct", FIRST_EVENT_SEQUENCE, MAX_EVENTS)
        .expect("later daemon events");
    assert!(matches!(
        later[0],
        animus_live_terminal::ClientSessionEvent::Observation {
            sequence: SECOND_EVENT_SEQUENCE,
            ..
        }
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
        private_event_mode: PrivateEventMode::None,
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
    let malformed_kernel_clear = ClientCommand::KernelClear {
        correlation_id: String::from("forged"),
    };
    assert_eq!(
        connection.route("root-direct", GENERATION, &malformed_kernel_clear),
        Err(TerminalConnectionError::Rejected)
    );
    assert!(payloads.lock().expect("payload lock").is_empty());

    let commands = [
        ClientCommand::Prompt {
            correlation_id: String::from(HASH),
            text: String::from("private direct prompt"),
        },
        ClientCommand::KernelClear {
            correlation_id: String::from(HASH),
        },
        ClientCommand::StateInspection {
            correlation_id: String::from(HASH),
        },
    ];
    for command in &commands {
        connection
            .route("root-direct", GENERATION, command)
            .expect("direct route");
    }
    let payloads = payloads.lock().expect("payload lock");
    assert_eq!(payloads.len(), commands.len());
    assert!(payloads.iter().any(|payload| {
        std::str::from_utf8(payload.as_slice())
            .expect("payload UTF-8")
            .contains("private direct prompt")
    }));
    assert_eq!(
        payloads
            .iter()
            .map(|payload| {
                serde_json::from_slice::<serde_json::Value>(payload).expect("typed command payload")
                    ["kind"]
                    .as_str()
                    .expect("command kind")
                    .to_owned()
            })
            .collect::<Vec<_>>(),
        commands
            .iter()
            .map(|command| command.class().to_owned())
            .collect::<Vec<_>>()
    );
}

#[test]
fn daemon_dispatcher_uses_the_supervisor_service_and_tracks_one_attachment() {
    let payloads = Arc::new(Mutex::new(Vec::new()));
    let worker = FixtureWorker {
        payloads: Arc::clone(&payloads),
        private_event_mode: PrivateEventMode::None,
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
    assert_daemon_state_inspection(&attached, FIRST_EVENT_SEQUENCE);
    let events = dispatcher.dispatch(DaemonRequest::EventsAfter {
        root_id: String::from("root-direct"),
        sequence: FIRST_EVENT_SEQUENCE,
        maximum_events: MAX_EVENTS,
    });
    assert_daemon_observation(&events, SECOND_EVENT_SEQUENCE);
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
    let commands = [
        ClientCommand::Prompt {
            correlation_id: String::from(HASH),
            text: String::from("private dispatcher prompt"),
        },
        ClientCommand::KernelClear {
            correlation_id: String::from(HASH),
        },
        ClientCommand::StateInspection {
            correlation_id: String::from(HASH),
        },
    ];
    for command in &commands {
        let routed = dispatcher.dispatch(DaemonRequest::Command {
            root_id: String::from("root-direct"),
            generation: GENERATION,
            correlation_id: String::from(command.correlation_id()),
            class: String::from(command.class()),
            private_payload: serde_json::to_vec(command).expect("typed command"),
        });
        assert_eq!(
            routed,
            DaemonResponse::CommandAccepted {
                safe_response_identity: String::from(HASH),
            }
        );
    }
    assert_eq!(payloads.lock().expect("payload lock").len(), commands.len());
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
            sequence: SECOND_EVENT_SEQUENCE,
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
// r[verify agent_harness_runtime.live_terminal_agent_host.private_events.render]
fn direct_and_daemon_private_runtime_events_decode_and_render() {
    let mut direct = DirectTerminalConnection::new(fixture_service(PrivateEventMode::Runtime));
    let direct_snapshot = direct
        .attach("root-direct", "client-private-direct", MAX_EVENTS)
        .expect("direct private snapshot");
    assert_private_runtime_render(&direct_snapshot);

    let directory = tempfile::tempdir().expect("private endpoint directory");
    let endpoint = animus_local_daemon::LocalEndpoint::new(directory.path(), "private.sock")
        .expect("private endpoint");
    let listener = animus_local_daemon::LocalListener::bind(endpoint.clone()).expect("listener");
    let server = std::thread::spawn(move || {
        let mut dispatcher =
            DaemonRequestDispatcher::new(fixture_service(PrivateEventMode::Runtime));
        let (mut client, _peer) = listener.accept().expect("accept");
        for _request_index in 0..PRIVATE_DAEMON_REQUEST_COUNT {
            let request = client
                .read_frame::<DaemonRequest>(MAX_FRAME_BYTES)
                .expect("private request frame");
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
                .expect("private response frame");
        }
        listener.close().expect("private listener close");
    });
    let mut daemon = DaemonTerminalConnection::connect(&endpoint, GENERATION, MAX_FRAME_BYTES)
        .expect("private daemon connection");
    let daemon_snapshot = daemon
        .attach("root-direct", "client-private-daemon", MAX_EVENTS)
        .expect("daemon private snapshot");
    assert_private_runtime_render(&daemon_snapshot);
    server.join().expect("private daemon server");
}

#[test]
// r[verify agent_harness_runtime.live_terminal_agent_host.private_events.denied]
fn private_runtime_unknown_class_invalid_codec_forged_identity_and_bounds_fail_closed() {
    for mode in [
        PrivateEventMode::UnknownClass,
        PrivateEventMode::InvalidCodec,
    ] {
        let mut direct = DirectTerminalConnection::new(fixture_service(mode));
        assert_eq!(
            direct.attach("root-direct", "client-private-invalid", MAX_EVENTS),
            Err(TerminalConnectionError::Protocol)
        );
    }

    let valid_payload = runtime_payload(INITIAL_EVENT_SEQUENCE).expect("private payload");
    let forged = animus_live_terminal::DaemonEvent::Private {
        sequence: FIRST_EVENT_SEQUENCE,
        class: String::from(PRIVATE_RUNTIME_EVENT_CLASS),
        payload_identity: String::from(HASH),
        private_payload: valid_payload,
    };
    assert_private_daemon_snapshot_rejected(forged, "forged-private.sock");

    let oversized_len = usize::try_from(animus_local_daemon::MAX_PRIVATE_EVENT_BYTES)
        .expect("private bound")
        .saturating_add(1);
    let oversized = animus_live_terminal::DaemonEvent::Private {
        sequence: FIRST_EVENT_SEQUENCE,
        class: String::from(PRIVATE_RUNTIME_EVENT_CLASS),
        payload_identity: String::from(HASH),
        private_payload: vec![0; oversized_len],
    };
    assert_private_daemon_snapshot_rejected(oversized, "oversized-private.sock");
}

#[test]
fn state_inspection_wire_shape_rejects_extra_private_fields() {
    let state = state_inspection();
    let observation_identity = state
        .observation_identity(animus_local_daemon::StateInspectionIdentityFacts {
            root_id: "root-direct",
            sequence: FIRST_EVENT_SEQUENCE,
            correlation_id: HASH,
        })
        .expect("state event identity");
    let event = animus_live_terminal::DaemonEvent::StateInspection {
        sequence: FIRST_EVENT_SEQUENCE,
        correlation_id: String::from(HASH),
        observation_identity,
        state,
    };
    let encoded = serde_json::to_value(event).expect("state event JSON");
    let object = encoded.as_object().expect("state event object");
    assert_eq!(object.len(), STATE_EVENT_FIELD_COUNT);
    assert_eq!(object["kind"], "state_inspection");
    assert_eq!(object["correlation_id"], HASH);
    assert!(!object.contains_key("prompt"));
    assert!(!object.contains_key("source"));
    assert!(!object.contains_key("credential"));
    assert!(!object.contains_key("path"));

    let mut extra_outer = encoded.clone();
    extra_outer["prompt"] = serde_json::Value::String(String::from("private prompt"));
    assert!(serde_json::from_value::<animus_live_terminal::DaemonEvent>(extra_outer).is_err());
    let mut extra_state = encoded;
    extra_state["state"]["credential"] =
        serde_json::Value::String(String::from("private credential"));
    assert!(serde_json::from_value::<animus_live_terminal::DaemonEvent>(extra_state).is_err());
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

fn assert_client_state_inspection(
    event: &animus_live_terminal::ClientSessionEvent,
    expected_sequence: u64,
) {
    assert!(matches!(
        event,
        animus_live_terminal::ClientSessionEvent::StateInspection { sequence, .. }
            if *sequence == expected_sequence
    ));
}

fn assert_daemon_state_inspection(response: &DaemonResponse, expected_sequence: u64) {
    assert!(matches!(
        response,
        DaemonResponse::Snapshot { events, .. }
            if matches!(&events[0], animus_live_terminal::DaemonEvent::StateInspection {
                sequence,
                ..
            } if *sequence == expected_sequence)
    ));
}

fn assert_daemon_observation(response: &DaemonResponse, expected_sequence: u64) {
    assert!(matches!(
        response,
        DaemonResponse::Events { events }
            if matches!(&events[0], animus_live_terminal::DaemonEvent::Observation {
                sequence,
                ..
            } if *sequence == expected_sequence)
    ));
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum PrivateEventMode {
    #[default]
    None,
    Runtime,
    UnknownClass,
    InvalidCodec,
}

fn fixture_service(
    private_event_mode: PrivateEventMode,
) -> animus_local_daemon::LocalSupervisorService<FixtureWorker> {
    let config = animus_local_daemon::parse_local_daemon_config(CONFIG).expect("daemon config");
    let registry = animus_local_daemon::WorkerRegistry::new(
        vec![record(animus_local_daemon::WorkerLifecycle::Ready)],
        MAX_WORKERS,
    )
    .expect("registry");
    animus_local_daemon::LocalSupervisorService::new(
        config,
        registry,
        FixtureWorker {
            payloads: Arc::new(Mutex::new(Vec::new())),
            private_event_mode,
        },
    )
}

fn private_runtime_events(
    root_id: &str,
    first_sequence: u64,
) -> Result<Vec<animus_local_daemon::WorkerEvent>, animus_local_daemon::ServiceError> {
    runtime_projection_events()
        .into_iter()
        .enumerate()
        .map(|(index, event)| {
            let offset =
                u64::try_from(index).map_err(|_error| animus_local_daemon::ServiceError::Worker)?;
            private_event(
                root_id,
                first_sequence.saturating_add(offset),
                PRIVATE_RUNTIME_EVENT_CLASS,
                runtime_event_payload(&event)?,
            )
        })
        .collect()
}

fn runtime_payload(
    _sequence_before_event: u64,
) -> Result<Vec<u8>, animus_local_daemon::ServiceError> {
    runtime_event_payload(&animus_runtime::RuntimeEvent::ModelRequested {
        request: animus_adapters::ModelRequest::new(PRIVATE_USER_PROMPT)
            .map_err(|_error| animus_local_daemon::ServiceError::Worker)?,
    })
}

fn runtime_event_payload(
    event: &animus_runtime::RuntimeEvent,
) -> Result<Vec<u8>, animus_local_daemon::ServiceError> {
    let message = animus_preserves::BoundaryMessage::from_runtime_event(event);
    animus_preserves::to_versioned_bytes(&message)
        .map_err(|_error| animus_local_daemon::ServiceError::Worker)
}

fn runtime_projection_events() -> Vec<animus_runtime::RuntimeEvent> {
    vec![
        animus_runtime::RuntimeEvent::ModelRequested {
            request: animus_adapters::ModelRequest::new(PRIVATE_USER_PROMPT)
                .expect("private model request"),
        },
        animus_runtime::RuntimeEvent::SessionStateChanged {
            session_id: String::from(PRIVATE_SESSION_ID),
            run_id: Some(String::from(PRIVATE_RUN_ID)),
            status: String::from(PRIVATE_SESSION_STATUS),
            queue_count: 0,
        },
        animus_runtime::RuntimeEvent::ToolRequested {
            request: private_tool_request(),
        },
        animus_runtime::RuntimeEvent::ToolResponded {
            response: private_tool_response(),
        },
        animus_runtime::RuntimeEvent::ModelResponded {
            response: animus_adapters::ModelResponse::new(PRIVATE_ASSISTANT_RESPONSE)
                .expect("private model response"),
        },
    ]
}

fn private_tool_request() -> animus_adapters::ToolRequest {
    let fields = [animus_adapters::tool::ArgumentFieldDraft {
        name: Some(PRIVATE_TOOL_ARGUMENT),
        value: Some(PRIVATE_TOOL_ARGUMENT_VALUE),
    }];
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: PRIVATE_TOOL_SCHEMA,
            allowed_field_names: &[PRIVATE_TOOL_ARGUMENT],
            compatibility_rules: &[],
            fields: &fields,
        },
    )
    .expect("private tool arguments");
    animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(PRIVATE_TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some(PRIVATE_TOOL_CALL_ID),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: PRIVATE_TOOL_VISIBLE_BUDGET,
        },
        mutation_targets: &[PRIVATE_TOOL_MUTATION_TARGET],
        required_capability_labels: &[PRIVATE_TOOL_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .expect("private tool request")
}

fn private_tool_response() -> animus_adapters::ToolResponse {
    let bounded = animus_adapters::tool::BoundedOutput::new(
        &animus_adapters::tool::OutputDraft {
            body: PRIVATE_TOOL_RESULT,
            full_output_reference: None,
            hash_requirement: animus_adapters::tool::HashRequirement::Required,
        },
        animus_adapters::tool::OutputPolicy {
            visible_char_budget: PRIVATE_TOOL_VISIBLE_BUDGET,
        },
    )
    .expect("private tool output");
    animus_adapters::ToolResponse::from_bounded(
        bounded,
        animus_adapters::tool::OutputRedactionClass::Safe,
    )
}

fn private_event(
    root_id: &str,
    sequence: u64,
    class: &str,
    private_payload: Vec<u8>,
) -> Result<animus_local_daemon::WorkerEvent, animus_local_daemon::ServiceError> {
    animus_local_daemon::WorkerPrivateEvent::new(
        animus_local_daemon::PrivateEventIdentityFacts {
            root_id,
            sequence,
            class,
        },
        private_payload,
    )
    .map(animus_local_daemon::WorkerEvent::Private)
    .map_err(|_error| animus_local_daemon::ServiceError::Worker)
}

fn assert_private_runtime_render(snapshot: &animus_live_terminal::ClientSnapshot) {
    let limits = TerminalLimits {
        event_batch: MAX_EVENTS,
        prompt_bytes: MAX_PROMPT_BYTES,
        width: MAX_VIEWPORT_WIDTH,
        height: MAX_VIEWPORT_HEIGHT,
    };
    let state = TerminalClientState::from_snapshot(snapshot, VIEWPORT, limits)
        .expect("private event reduction");
    let rendered = animus_tui::render_streaming_snapshot(state.view().conversation());
    assert!(rendered.contains(PRIVATE_USER_PROMPT));
    assert!(rendered.contains(PRIVATE_ASSISTANT_RESPONSE));
    assert!(rendered.contains(PRIVATE_TOOL_NAME));
    assert!(rendered.contains(PRIVATE_TOOL_RESULT));
    assert_eq!(
        state.view().coding_session().state().lifecycle_status,
        PRIVATE_SESSION_STATUS
    );
}

fn assert_private_daemon_snapshot_rejected(
    event: animus_live_terminal::DaemonEvent,
    endpoint_name: &str,
) {
    let directory = tempfile::tempdir().expect("invalid private endpoint directory");
    let endpoint = animus_local_daemon::LocalEndpoint::new(directory.path(), endpoint_name)
        .expect("invalid private endpoint");
    let listener = animus_local_daemon::LocalListener::bind(endpoint.clone()).expect("listener");
    let server = std::thread::spawn(move || {
        let (mut client, _peer) = listener.accept().expect("accept");
        let request = client
            .read_frame::<DaemonRequest>(MAX_FRAME_BYTES)
            .expect("invalid private request");
        client
            .write_frame(
                &animus_local_daemon::TransportFrame {
                    version: request.version,
                    endpoint_identity: request.endpoint_identity,
                    generation: request.generation,
                    correlation_id: request.correlation_id,
                    payload: DaemonResponse::Snapshot {
                        snapshot: animus_live_terminal::DaemonSnapshot {
                            root_id: String::from("root-direct"),
                            worker_id: String::from("worker-direct"),
                            generation: GENERATION,
                            event_sequence: INITIAL_EVENT_SEQUENCE,
                            restore_status: animus_local_daemon::RestoreStatus::Restored,
                        },
                        events: vec![event],
                    },
                },
                MAX_FRAME_BYTES,
            )
            .expect("invalid private response");
        listener.close().expect("invalid private listener close");
    });
    let mut connection = DaemonTerminalConnection::connect(&endpoint, GENERATION, MAX_FRAME_BYTES)
        .expect("invalid private daemon connection");
    assert_eq!(
        connection.attach("root-direct", "client-private-invalid", MAX_EVENTS),
        Err(TerminalConnectionError::Protocol)
    );
    server.join().expect("invalid private daemon server");
}

struct FixtureWorker {
    payloads: Arc<Mutex<Vec<Vec<u8>>>>,
    private_event_mode: PrivateEventMode,
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
            event_sequence: INITIAL_EVENT_SEQUENCE,
            restore_status: animus_local_daemon::RestoreStatus::Restored,
        })
    }

    fn events_after(
        &mut self,
        root_id: &str,
        sequence: u64,
        _maximum: u32,
    ) -> Result<Vec<animus_local_daemon::WorkerEvent>, animus_local_daemon::ServiceError> {
        match self.private_event_mode {
            PrivateEventMode::Runtime => private_runtime_events(root_id, sequence + 1),
            PrivateEventMode::None if sequence == INITIAL_EVENT_SEQUENCE => {
                Ok(vec![animus_local_daemon::WorkerEvent::state_inspection(
                    animus_local_daemon::StateInspectionIdentityFacts {
                        root_id,
                        sequence: sequence + 1,
                        correlation_id: HASH,
                    },
                    state_inspection(),
                )?])
            }
            PrivateEventMode::None => Ok(vec![animus_local_daemon::WorkerEvent::Observation {
                root_id: String::from(root_id),
                sequence: sequence + 1,
                class: String::from("attached"),
                observation_identity: String::from(HASH),
            }]),
            PrivateEventMode::UnknownClass => Ok(vec![private_event(
                root_id,
                sequence + 1,
                "provider_result",
                runtime_payload(sequence)?,
            )?]),
            PrivateEventMode::InvalidCodec => Ok(vec![private_event(
                root_id,
                sequence + 1,
                PRIVATE_RUNTIME_EVENT_CLASS,
                b"not-preserves".to_vec(),
            )?]),
        }
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

fn state_inspection() -> animus_local_daemon::WorkerStateInspection {
    animus_local_daemon::WorkerStateInspection {
        worker_generation: GENERATION,
        restore_status: animus_local_daemon::RestoreStatus::Restored,
        session_head_identity: String::from(HASH),
        kernel_head_identity: String::from(HASH),
        queue_generation: GENERATION,
        queue_identity: String::from(HASH),
        committed_message_count: 1,
        queued_item_count: 0,
        active_child_count: 0,
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
        event_sequence: INITIAL_EVENT_SEQUENCE,
        receipt_identity: String::from(HASH),
    }
}
