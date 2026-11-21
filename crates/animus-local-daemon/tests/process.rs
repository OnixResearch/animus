//! Worker process tree lifecycle tests.

use animus_local_daemon::{
    ProcessObservation, ProcessTreeLauncher, WorkerLaunchRequest, WorkerProcessError,
};
use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const STOP_DEADLINE_MS: u64 = 2000;

/// The worker's environment is explicitly empty, so its shell resolves no
/// external command (NixOS `sh` falls back to `PATH=/no-such-path`). A
/// subshell running a builtin-only loop keeps a real process tree alive until
/// the group teardown under test, without reading any ambient environment.
const LIVE_TREE: &str = "( while :; do :; done ) & wait";

#[test]
fn worker_launch_uses_explicit_environment_and_complete_group_teardown() {
    let request = WorkerLaunchRequest {
        executable: PathBuf::from("/bin/sh"),
        arguments: vec![String::from("-c"), String::from(LIVE_TREE)],
        environment: BTreeMap::new(),
        current_directory: PathBuf::from("/"),
        root_id: String::from("root-a"),
        worker_id: String::from("worker-a"),
        generation: 1,
        endpoint_identity: String::from(HASH),
    };
    let mut worker = ProcessTreeLauncher::launch(&request).expect("launch worker");
    assert!(worker.identity().process_id > 0);
    assert_eq!(worker.observe(), Ok(ProcessObservation::Running));
    assert!(matches!(
        worker.stop_and_reap(Duration::from_millis(STOP_DEADLINE_MS)),
        Ok(ProcessObservation::Exited(_))
    ));
}

#[test]
fn relative_executable_ambient_inputs_and_zero_generation_are_denied() {
    let invalid = WorkerLaunchRequest {
        executable: PathBuf::from("sh"),
        arguments: Vec::new(),
        environment: BTreeMap::new(),
        current_directory: PathBuf::from("/"),
        root_id: String::from("root-a"),
        worker_id: String::from("worker-a"),
        generation: 0,
        endpoint_identity: String::from(HASH),
    };
    assert!(matches!(
        ProcessTreeLauncher::launch(&invalid),
        Err(WorkerProcessError::InvalidRequest)
    ));
}
