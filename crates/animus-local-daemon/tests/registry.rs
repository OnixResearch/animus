//! Capability-relative registry publication tests.

use animus_local_daemon::{
    RegistryError, RegistryStore, RestoreStatus, WorkerLifecycle, WorkerRegistry,
    WorkerRegistryRecord,
};

const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const MAX_WORKERS: u16 = 8;

#[test]
fn registry_round_trip_is_atomic_and_metadata_only() {
    let temporary = tempfile::tempdir().expect("temporary root");
    let root = cap_std::fs::Dir::open_ambient_dir(temporary.path(), cap_std::ambient_authority())
        .expect("capability root");
    let store = RegistryStore::new(&root, "workers.json", MAX_WORKERS).expect("registry store");
    let empty = store.load().expect("empty registry");
    assert!(empty.workers.is_empty());
    let registry = WorkerRegistry::new(vec![record()], MAX_WORKERS).expect("registry");
    let identity = store.publish(&registry).expect("publish registry");
    assert_eq!(identity.len(), animus_adapter_daemon::BLAKE3_HEX_CHARS);
    assert_eq!(store.load(), Ok(registry));
    let bytes = std::fs::read(temporary.path().join("workers.json")).expect("registry bytes");
    let text = String::from_utf8(bytes).expect("registry text");
    for forbidden in ["prompt", "transcript", "credential", "tool_body"] {
        assert!(!text.contains(forbidden));
    }
}

#[test]
#[cfg(unix)]
fn unsafe_name_symlink_and_invalid_registry_fail_closed() {
    use std::os::unix::fs::symlink;

    let temporary = tempfile::tempdir().expect("temporary root");
    let root = cap_std::fs::Dir::open_ambient_dir(temporary.path(), cap_std::ambient_authority())
        .expect("capability root");
    assert!(matches!(
        RegistryStore::new(&root, "../workers.json", MAX_WORKERS),
        Err(RegistryError::InvalidInput)
    ));
    std::fs::write(temporary.path().join("target"), b"{}\n").expect("target");
    symlink("target", temporary.path().join("workers.json")).expect("symlink");
    let store = RegistryStore::new(&root, "workers.json", MAX_WORKERS).expect("registry store");
    assert_eq!(store.load(), Err(RegistryError::InvalidState));
}

fn record() -> WorkerRegistryRecord {
    WorkerRegistryRecord {
        root_id: String::from("root-a"),
        worker_id: String::from("worker-a"),
        generation: 1,
        process_id: 42,
        process_identity: String::from(HASH),
        endpoint_identity: String::from(HASH),
        session_head_identity: String::from(HASH),
        lifecycle: WorkerLifecycle::Ready,
        restore_status: RestoreStatus::Restored,
        attachments: Vec::new(),
        event_sequence: 1,
        receipt_identity: String::from(HASH),
    }
}
