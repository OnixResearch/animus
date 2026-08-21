//! Supervisor and worker composition tests.

use std::cell::Cell;
use std::rc::Rc;

use animus_local_daemon::{
    HealthFacts, LocalSupervisorService, PrivateEventIdentityFacts, RestoreStatus, ServiceError,
    StateInspectionIdentityFacts, WorkerCommand, WorkerEvent, WorkerLifecycle, WorkerPrivateEvent,
    WorkerRegistry, WorkerRegistryRecord, WorkerSessionPort, WorkerSnapshot, WorkerStateInspection,
    parse_local_daemon_config,
};

const CONFIG: &[u8] = include_bytes!("../../../config/generated/local-daemon-supervisor.json");
const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const MAX_WORKERS: u16 = 8;
const SECOND_EVENT_OFFSET: u64 = 2;
const PRIVATE_EVENT_CLASS: &str = "runtime_event";
const PRIVATE_EVENT_CONTENT: &[u8] = b"private transcript content";

#[derive(Default)]
struct FixtureWorker {
    routed: usize,
    stopped: Rc<Cell<usize>>,
    retried: usize,
    state_inspection: Option<WorkerStateInspection>,
    invalid_state_inspection: Option<WorkerStateInspection>,
    private_event: Option<WorkerPrivateEvent>,
    unordered_events: bool,
}

impl WorkerSessionPort for FixtureWorker {
    fn snapshot(&mut self, root_id: &str) -> Result<WorkerSnapshot, ServiceError> {
        Ok(WorkerSnapshot {
            root_id: String::from(root_id),
            worker_id: String::from("worker-a"),
            generation: 1,
            event_sequence: 4,
            restore_status: RestoreStatus::Restored,
        })
    }

    fn events_after(
        &mut self,
        root_id: &str,
        sequence: u64,
        _maximum: u32,
    ) -> Result<Vec<WorkerEvent>, ServiceError> {
        if self.unordered_events {
            return Ok(vec![
                WorkerEvent::Observation {
                    root_id: String::from(root_id),
                    sequence: sequence + SECOND_EVENT_OFFSET,
                    class: String::from("later"),
                    observation_identity: String::from(HASH),
                },
                WorkerEvent::Observation {
                    root_id: String::from(root_id),
                    sequence: sequence + 1,
                    class: String::from("earlier"),
                    observation_identity: String::from(HASH),
                },
            ]);
        }
        let event = if let Some(event) = self.private_event.as_ref() {
            WorkerEvent::Private(event.clone())
        } else if let Some(state) = self.invalid_state_inspection.as_ref() {
            WorkerEvent::StateInspection {
                root_id: String::from(root_id),
                sequence: sequence + 1,
                correlation_id: String::from(HASH),
                observation_identity: String::from(HASH),
                state: state.clone(),
            }
        } else if let Some(state) = self.state_inspection.as_ref() {
            WorkerEvent::state_inspection(
                StateInspectionIdentityFacts {
                    root_id,
                    sequence: sequence + 1,
                    correlation_id: HASH,
                },
                state.clone(),
            )
            .expect("valid state inspection fixture")
        } else {
            WorkerEvent::Observation {
                root_id: String::from(root_id),
                sequence: sequence + 1,
                class: String::from("health"),
                observation_identity: String::from(HASH),
            }
        };
        Ok(vec![event])
    }

    fn route(&mut self, command: &WorkerCommand) -> Result<String, ServiceError> {
        self.routed += 1;
        Ok(command.correlation_id.clone())
    }

    fn request_stop(&mut self, _root_id: &str, _generation: u64) -> Result<(), ServiceError> {
        self.stopped.set(self.stopped.get().saturating_add(1));
        Ok(())
    }

    fn retry(&mut self, root_id: &str, generation: u64) -> Result<WorkerSnapshot, ServiceError> {
        self.retried += 1;
        Ok(WorkerSnapshot {
            root_id: String::from(root_id),
            worker_id: String::from("worker-b"),
            generation,
            event_sequence: 0,
            restore_status: RestoreStatus::Restored,
        })
    }
}

#[test]
fn snapshot_first_attach_detach_command_stop_and_retry_use_worker_port() {
    let config = parse_local_daemon_config(CONFIG).expect("config");
    let registry =
        WorkerRegistry::new(vec![record(WorkerLifecycle::Ready)], MAX_WORKERS).expect("registry");
    let worker = FixtureWorker::default();
    let stopped = Rc::clone(&worker.stopped);
    let mut service = LocalSupervisorService::new(config, registry, worker);
    let (snapshot, events) = service.attach("root-a", "client-a", 8).expect("attach");
    assert_eq!(snapshot.event_sequence, 4);
    assert_eq!(events[0].sequence(), 5);
    assert_eq!(service.attachment_count(), 1);
    let client_scope_lease = service
        .client_delivery_scope_lease("root-a", "client-a")
        .expect("client delivery scope lease");
    assert!(!client_scope_lease.owns_worker_scope());
    let later = service
        .events_after(animus_local_daemon::EventBatchRequest {
            root_id: "root-a",
            client_id: "client-a",
            sequence: 5,
            maximum_events: 8,
        })
        .expect("later events");
    assert_eq!(later[0].sequence(), 6);
    service.detach("root-a", "client-a").expect("detach");
    assert_eq!(service.attachment_count(), 0);
    assert!(
        service
            .client_delivery_scope_lease("root-a", "client-a")
            .is_none()
    );
    assert_eq!(stopped.get(), 0);
    assert_eq!(
        service
            .route(&WorkerCommand {
                root_id: String::from("root-a"),
                generation: 1,
                correlation_id: String::from("command-a"),
                class: String::from("prompt"),
                private_payload: br#"{"kind":"prompt"}"#.to_vec(),
            })
            .expect("route"),
        "command-a"
    );
    service.stop("root-a", 1).expect("stop");
    assert_eq!(stopped.get(), 1);

    let config = parse_local_daemon_config(CONFIG).expect("config");
    let registry =
        WorkerRegistry::new(vec![record(WorkerLifecycle::Stopped)], MAX_WORKERS).expect("registry");
    let mut retry_service = LocalSupervisorService::new(config, registry, FixtureWorker::default());
    let replacement = retry_service
        .retry(
            "root-a",
            HealthFacts {
                process_exists: false,
                identity_matches: false,
                zombie: false,
                endpoint_connected: false,
                health_response: false,
            },
        )
        .expect("retry");
    assert_eq!(replacement.generation, 2);
}

#[test]
fn stale_generation_duplicate_detach_and_healthy_retry_fail_closed() {
    let config = parse_local_daemon_config(CONFIG).expect("config");
    let registry =
        WorkerRegistry::new(vec![record(WorkerLifecycle::Ready)], MAX_WORKERS).expect("registry");
    let mut service = LocalSupervisorService::new(config, registry, FixtureWorker::default());
    assert_eq!(
        service.route(&WorkerCommand {
            root_id: String::from("root-a"),
            generation: 2,
            correlation_id: String::from("command-a"),
            class: String::from("prompt"),
            private_payload: br#"{"kind":"prompt"}"#.to_vec(),
        }),
        Err(ServiceError::WorkerIdentity)
    );
    assert_eq!(
        service.detach("root-a", "missing-client"),
        Err(ServiceError::UnknownAttachment)
    );
    assert_eq!(
        service.events_after(animus_local_daemon::EventBatchRequest {
            root_id: "root-a",
            client_id: "missing-client",
            sequence: 4,
            maximum_events: 8,
        }),
        Err(ServiceError::UnknownAttachment)
    );
    assert_eq!(
        service.retry(
            "root-a",
            HealthFacts {
                process_exists: true,
                identity_matches: true,
                zombie: false,
                endpoint_connected: true,
                health_response: true,
            },
        ),
        Err(ServiceError::RetryDenied)
    );
}

#[test]
fn metadata_only_state_inspection_routes_and_invalid_state_fails_closed() {
    let config = parse_local_daemon_config(CONFIG).expect("config");
    let registry =
        WorkerRegistry::new(vec![record(WorkerLifecycle::Ready)], MAX_WORKERS).expect("registry");
    let inspection = state_inspection();
    let worker = FixtureWorker {
        state_inspection: Some(inspection.clone()),
        ..FixtureWorker::default()
    };
    let mut service = LocalSupervisorService::new(config, registry, worker);
    let (_snapshot, events) = service
        .attach("root-a", "inspection-client", 1)
        .expect("state inspection event");
    assert_eq!(
        events,
        vec![
            WorkerEvent::state_inspection(
                StateInspectionIdentityFacts {
                    root_id: "root-a",
                    sequence: 5,
                    correlation_id: HASH,
                },
                inspection,
            )
            .expect("expected state inspection")
        ]
    );

    let config = parse_local_daemon_config(CONFIG).expect("config");
    let registry =
        WorkerRegistry::new(vec![record(WorkerLifecycle::Ready)], MAX_WORKERS).expect("registry");
    let invalid = WorkerStateInspection {
        session_head_identity: String::from("private-session-content"),
        ..state_inspection()
    };
    let worker = FixtureWorker {
        invalid_state_inspection: Some(invalid),
        ..FixtureWorker::default()
    };
    let mut service = LocalSupervisorService::new(config, registry, worker);
    assert_eq!(
        service.attach("root-a", "invalid-inspection-client", 1),
        Err(ServiceError::Bound)
    );
}

#[test]
fn bounded_private_event_routes_unchanged_and_debug_omits_payload() {
    let config = parse_local_daemon_config(CONFIG).expect("config");
    let registry =
        WorkerRegistry::new(vec![record(WorkerLifecycle::Ready)], MAX_WORKERS).expect("registry");
    let event = WorkerPrivateEvent::new(
        PrivateEventIdentityFacts {
            root_id: "root-a",
            sequence: 5,
            class: PRIVATE_EVENT_CLASS,
        },
        PRIVATE_EVENT_CONTENT.to_vec(),
    )
    .expect("private event");
    let payload_identity = String::from(event.payload_identity());
    assert!(!format!("{event:?}").contains("private transcript content"));
    let worker = FixtureWorker {
        private_event: Some(event),
        ..FixtureWorker::default()
    };
    let mut service = LocalSupervisorService::new(config, registry, worker);
    let (_snapshot, events) = service
        .attach("root-a", "private-client", 1)
        .expect("attach");
    let WorkerEvent::Private(delivered) = &events[0] else {
        panic!("expected private event");
    };
    assert_eq!(delivered.private_payload(), PRIVATE_EVENT_CONTENT);
    assert_eq!(delivered.payload_identity(), payload_identity);
    assert_eq!(delivered.class(), PRIVATE_EVENT_CLASS);

    assert!(
        WorkerPrivateEvent::new(
            PrivateEventIdentityFacts {
                root_id: "root-a",
                sequence: 5,
                class: PRIVATE_EVENT_CLASS,
            },
            Vec::new(),
        )
        .is_err()
    );
    let oversized_len = usize::try_from(animus_local_daemon::MAX_PRIVATE_EVENT_BYTES)
        .expect("private bound")
        .saturating_add(1);
    assert!(
        WorkerPrivateEvent::new(
            PrivateEventIdentityFacts {
                root_id: "root-a",
                sequence: 5,
                class: PRIVATE_EVENT_CLASS,
            },
            vec![0; oversized_len],
        )
        .is_err()
    );
}

#[test]
fn stale_state_generation_restore_and_event_order_fail_closed() {
    for invalid in [
        WorkerStateInspection {
            worker_generation: SECOND_EVENT_OFFSET,
            ..state_inspection()
        },
        WorkerStateInspection {
            restore_status: RestoreStatus::Fresh,
            ..state_inspection()
        },
    ] {
        let config = parse_local_daemon_config(CONFIG).expect("config");
        let registry = WorkerRegistry::new(vec![record(WorkerLifecycle::Ready)], MAX_WORKERS)
            .expect("registry");
        let worker = FixtureWorker {
            state_inspection: Some(invalid),
            ..FixtureWorker::default()
        };
        let mut service = LocalSupervisorService::new(config, registry, worker);
        assert_eq!(
            service.attach("root-a", "stale-inspection-client", 1),
            Err(ServiceError::WorkerIdentity)
        );
    }

    let config = parse_local_daemon_config(CONFIG).expect("config");
    let registry =
        WorkerRegistry::new(vec![record(WorkerLifecycle::Ready)], MAX_WORKERS).expect("registry");
    let worker = FixtureWorker {
        unordered_events: true,
        ..FixtureWorker::default()
    };
    let mut service = LocalSupervisorService::new(config, registry, worker);
    assert_eq!(
        service.attach("root-a", "unordered-client", 1),
        Err(ServiceError::Bound)
    );
}

fn state_inspection() -> WorkerStateInspection {
    WorkerStateInspection {
        worker_generation: 1,
        restore_status: RestoreStatus::Restored,
        session_head_identity: String::from(HASH),
        kernel_head_identity: String::from(HASH),
        queue_generation: 1,
        queue_identity: String::from(HASH),
        committed_message_count: 1,
        queued_item_count: 0,
        active_child_count: 0,
    }
}

fn record(lifecycle: WorkerLifecycle) -> WorkerRegistryRecord {
    WorkerRegistryRecord {
        root_id: String::from("root-a"),
        worker_id: String::from("worker-a"),
        generation: 1,
        process_id: 42,
        process_identity: String::from(HASH),
        endpoint_identity: String::from(HASH),
        session_head_identity: String::from(HASH),
        lifecycle,
        restore_status: RestoreStatus::Restored,
        attachments: Vec::new(),
        event_sequence: 4,
        receipt_identity: String::from(HASH),
    }
}
