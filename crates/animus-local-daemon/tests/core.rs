//! Pure supervisor lifecycle and routing tests.

use animus_local_daemon::{
    HealthFacts, LocalDaemonError, RecoveryKind, RestoreStatus, RoutingDecision, RoutingRequest,
    StopFacts, WorkerLifecycle, WorkerRegistry, WorkerRegistryRecord, admit_ready, decide_recovery,
    decide_routing, decide_stop, next_generation,
};

const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const OTHER_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const MAX_WORKERS: u16 = 8;
const MAX_ATTACHMENTS: u16 = 4;
const MAX_EVENTS: u32 = 64;

#[test]
fn ready_worker_routes_snapshot_attach_detach_and_command_without_execution() {
    let worker = record(WorkerLifecycle::Ready, RestoreStatus::Restored);
    let registry = WorkerRegistry::new(vec![worker.clone()], MAX_WORKERS).expect("registry");

    assert_eq!(
        decide_routing(
            &registry,
            &RoutingRequest::Snapshot {
                root_id: String::from("root-a"),
            },
            MAX_ATTACHMENTS,
            MAX_EVENTS,
        ),
        Ok(RoutingDecision::Snapshot {
            worker_id: String::from("worker-a"),
            generation: 1,
            event_sequence: 7,
        })
    );
    for request in [
        RoutingRequest::Attach {
            root_id: String::from("root-a"),
            client_id: String::from("client-a"),
            max_events: MAX_EVENTS,
        },
        RoutingRequest::Detach {
            root_id: String::from("root-a"),
            client_id: String::from("client-a"),
        },
    ] {
        let decision = decide_routing(&registry, &request, MAX_ATTACHMENTS, MAX_EVENTS)
            .expect("route attachment");
        match decision {
            RoutingDecision::Attached {
                execution_continues,
                ..
            }
            | RoutingDecision::Detached {
                execution_continues,
                ..
            } => assert!(execution_continues),
            RoutingDecision::Snapshot { .. } | RoutingDecision::Routed { .. } => {
                panic!("unexpected decision")
            }
        }
    }
    assert!(matches!(
        decide_routing(
            &registry,
            &RoutingRequest::Command {
                root_id: String::from("root-a"),
                correlation_id: String::from("command-a"),
            },
            MAX_ATTACHMENTS,
            MAX_EVENTS,
        ),
        Ok(RoutingDecision::Routed { .. })
    ));
}

#[test]
fn recovery_distinguishes_stale_identity_zombie_disconnect_and_restore_block() {
    let ready = WorkerLifecycle::Ready;
    let restored = RestoreStatus::Restored;
    let base = HealthFacts {
        process_exists: true,
        identity_matches: true,
        zombie: false,
        endpoint_connected: true,
        health_response: true,
    };
    assert_eq!(
        decide_recovery(ready, restored, base).kind,
        RecoveryKind::Healthy
    );
    assert_eq!(
        decide_recovery(
            ready,
            restored,
            HealthFacts {
                process_exists: false,
                ..base
            },
        )
        .kind,
        RecoveryKind::Stale
    );
    let mismatch = decide_recovery(
        ready,
        restored,
        HealthFacts {
            identity_matches: false,
            ..base
        },
    );
    assert_eq!(mismatch.kind, RecoveryKind::IdentityMismatch);
    assert!(!mismatch.teardown_required);
    assert_eq!(
        decide_recovery(
            ready,
            restored,
            HealthFacts {
                zombie: true,
                ..base
            },
        )
        .kind,
        RecoveryKind::Zombie
    );
    assert_eq!(
        decide_recovery(
            ready,
            restored,
            HealthFacts {
                endpoint_connected: false,
                ..base
            },
        )
        .kind,
        RecoveryKind::Disconnected
    );
    assert_eq!(
        decide_recovery(ready, RestoreStatus::Blocked, base).kind,
        RecoveryKind::RestoreBlocked
    );
}

#[test]
fn stop_finalization_requires_authority_and_complete_tree_observation() {
    assert_eq!(
        decide_stop(StopFacts {
            authorized: false,
            root_exited: true,
            descendants_exited: true,
            root_reaped: true,
        }),
        Err(LocalDaemonError::AuthorityDenied)
    );
    let pending = decide_stop(StopFacts {
        authorized: true,
        root_exited: true,
        descendants_exited: false,
        root_reaped: true,
    })
    .expect("pending stop");
    assert_eq!(pending.lifecycle, WorkerLifecycle::Stopping);
    assert!(pending.continue_finalization);
    assert!(!pending.commit_stopped);
    let complete = decide_stop(StopFacts {
        authorized: true,
        root_exited: true,
        descendants_exited: true,
        root_reaped: true,
    })
    .expect("complete stop");
    assert_eq!(complete.lifecycle, WorkerLifecycle::Stopped);
    assert!(complete.commit_stopped);
}

#[test]
fn readiness_retry_and_registry_fail_closed() {
    let expected = record(WorkerLifecycle::Starting, RestoreStatus::Fresh);
    let mut observed = expected.clone();
    observed.lifecycle = WorkerLifecycle::Ready;
    observed.restore_status = RestoreStatus::Restored;
    assert_eq!(admit_ready(&expected, &observed), Ok(observed.clone()));
    observed.process_identity = String::from(OTHER_HASH);
    assert_eq!(
        admit_ready(&expected, &observed),
        Err(LocalDaemonError::IdentityMismatch)
    );
    assert_eq!(
        next_generation(u64::MAX),
        Err(LocalDaemonError::GenerationOverflow)
    );
    assert_eq!(
        WorkerRegistry::new(
            vec![record(WorkerLifecycle::Ready, RestoreStatus::Restored); 2],
            MAX_WORKERS
        ),
        Err(LocalDaemonError::DuplicateWorker)
    );
}

fn record(lifecycle: WorkerLifecycle, restore_status: RestoreStatus) -> WorkerRegistryRecord {
    WorkerRegistryRecord {
        root_id: String::from("root-a"),
        worker_id: String::from("worker-a"),
        generation: 1,
        process_id: 42,
        process_identity: String::from(HASH),
        endpoint_identity: String::from(HASH),
        session_head_identity: String::from(HASH),
        lifecycle,
        restore_status,
        attachments: Vec::new(),
        event_sequence: 7,
        receipt_identity: String::from(HASH),
    }
}
