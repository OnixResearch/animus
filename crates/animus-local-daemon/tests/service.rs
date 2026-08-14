//! Supervisor and worker composition tests.

use animus_local_daemon::{
    HealthFacts, LocalSupervisorService, RestoreStatus, ServiceError, WorkerCommand, WorkerEvent,
    WorkerLifecycle, WorkerRegistry, WorkerRegistryRecord, WorkerSessionPort, WorkerSnapshot,
    parse_local_daemon_config,
};

const CONFIG: &[u8] = include_bytes!("../../../config/generated/local-daemon-supervisor.json");
const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const MAX_WORKERS: u16 = 8;

#[derive(Default)]
struct FixtureWorker {
    routed: usize,
    stopped: usize,
    retried: usize,
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
        Ok(vec![WorkerEvent {
            root_id: String::from(root_id),
            sequence: sequence + 1,
            class: String::from("health"),
            observation_identity: String::from(HASH),
        }])
    }

    fn route(&mut self, command: &WorkerCommand) -> Result<String, ServiceError> {
        self.routed += 1;
        Ok(command.correlation_id.clone())
    }

    fn request_stop(&mut self, _root_id: &str, _generation: u64) -> Result<(), ServiceError> {
        self.stopped += 1;
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
    let mut service = LocalSupervisorService::new(config, registry, FixtureWorker::default());
    let (snapshot, events) = service.attach("root-a", "client-a", 8).expect("attach");
    assert_eq!(snapshot.event_sequence, 4);
    assert_eq!(events[0].sequence, 5);
    assert_eq!(service.attachment_count(), 1);
    service.detach("root-a", "client-a").expect("detach");
    assert_eq!(service.attachment_count(), 0);
    assert_eq!(
        service
            .route(&WorkerCommand {
                root_id: String::from("root-a"),
                generation: 1,
                correlation_id: String::from("command-a"),
                class: String::from("prompt"),
            })
            .expect("route"),
        "command-a"
    );
    service.stop("root-a", 1).expect("stop");

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
        }),
        Err(ServiceError::WorkerIdentity)
    );
    assert_eq!(
        service.detach("root-a", "missing-client"),
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
