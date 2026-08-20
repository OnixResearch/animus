//! Isolated real-daemon pueue integration evidence.

#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![cfg_attr(
    animus_octet,
    allow(
        tigerstyle::ambient_env,
        tigerstyle::no_panic,
        tigerstyle::no_unwrap,
        tigerstyle::non_trait_imports,
        tigerstyle::numeric_units,
        reason = "the ignored real-daemon test reads explicit focused-rail executable bindings and uses assertion failures as test verdicts"
    )
)]

mod support;

use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use animus_adapters::{
    ComponentAdapter, OutcomeKind, QueueEnqueueDraft, QueueEnqueueRequest, QueueInteractionDraft,
    QueueInteractionKind, QueueInteractionRequest, QueueServiceRequest, QueueServiceResponse,
    QueueTaskStatus, ServiceOutcome,
};
use animus_std_pueue::{BoundedExecPueueProcessService, PueueHost, parse_pueue_host_config};

const PUEUE_ENV: &str = "ANIMUS_PUEUE_BIN";
const PUEUED_ENV: &str = "ANIMUS_PUEUED_BIN";
const TIMEOUT_ENV: &str = "ANIMUS_PUEUE_TIMEOUT_BIN";
const JOB_ENV: &str = "ANIMUS_PUEUE_JOB_BIN";
const DAEMON_SETTLE_DELAY_MS: u64 = 500;
const TASK_SETTLE_DELAY_MS: u64 = 500;

#[test]
#[ignore = "focused rail supplies pinned pueue executables"]
fn isolated_real_daemon_executes_one_reviewed_profile() {
    let pueue = required_path(PUEUE_ENV);
    let pueued = required_path(PUEUED_ENV);
    let timeout = required_path(TIMEOUT_ENV);
    let job = required_path(JOB_ENV);
    let root = tempfile::tempdir().expect("temporary root");
    let namespace_root = root.path().join("isolated");
    std::fs::create_dir_all(&namespace_root).expect("isolated root");
    let config_path = namespace_root.join("pueue.yml");
    write_pueue_config(&config_path, &namespace_root);
    start_daemon(&pueued, &config_path, &namespace_root);
    std::thread::sleep(std::time::Duration::from_millis(DAEMON_SETTLE_DELAY_MS));
    let _guard = DaemonGuard::new(pueue.clone(), config_path.clone(), namespace_root.clone());

    let config_text = support::config_json_from_paths(support::ConfigPaths {
        root: namespace_root.to_str().expect("isolated root path"),
        pueue: pueue.to_str().expect("pueue path"),
        pueued: pueued.to_str().expect("pueued path"),
        timeout: timeout.to_str().expect("timeout path"),
        job: job.to_str().expect("job path"),
    });
    let config = parse_pueue_host_config(config_text.as_str()).expect("real daemon config");
    let capabilities = config.capabilities();
    let environment_policy = String::from(config.environment_policy());
    let timeout_policy = String::from(config.timeout_policy());
    let log_bounds = config.limits().log_bounds().expect("log bounds");
    let mut host = PueueHost::new(
        config,
        BoundedExecPueueProcessService::new(Arc::new(AtomicBool::new(false))),
    );

    let enqueue = QueueServiceRequest::Enqueue(
        QueueEnqueueRequest::new(QueueEnqueueDraft {
            command_label: "fixture-check",
            group: None,
            environment_policy: environment_policy.as_str(),
            timeout_policy: timeout_policy.as_str(),
            output_bounds: log_bounds,
        })
        .expect("enqueue request"),
    );
    let enqueue_outcome = host.call(&enqueue).expect("infallible host");
    assert_eq!(
        enqueue_outcome.kind(),
        OutcomeKind::Completed,
        "enqueue outcome"
    );
    let task_id = completed_task_id(enqueue_outcome);
    std::thread::sleep(std::time::Duration::from_millis(TASK_SETTLE_DELAY_MS));
    let poll = QueueInteractionRequest::new(QueueInteractionDraft {
        kind: QueueInteractionKind::Poll,
        task_id: task_id.as_str(),
        timeout_policy: timeout_policy.as_str(),
        capabilities,
        input_hash: None,
    })
    .expect("poll request");
    let poll_outcome = host
        .call(&QueueServiceRequest::Interaction(poll))
        .expect("poll");
    assert_eq!(poll_outcome.kind(), OutcomeKind::Completed, "poll outcome");
    let status = completed_task(poll_outcome);

    assert_eq!(status.status(), QueueTaskStatus::Completed);
    assert_eq!(status.group(), "default");
}

fn completed_task_id(outcome: ServiceOutcome<QueueServiceResponse>) -> String {
    String::from(completed_task(outcome).task_id())
}

fn completed_task(
    outcome: ServiceOutcome<QueueServiceResponse>,
) -> animus_adapters::QueueTaskObservation {
    let kind = outcome.kind();
    let ServiceOutcome::Completed { value, .. } = outcome else {
        panic!("queue request must complete: {kind:?}");
    };
    let QueueServiceResponse::Task(task) = value else {
        panic!("task response required");
    };
    task
}

fn required_path(key: &str) -> PathBuf {
    PathBuf::from(std::env::var(key).unwrap_or_else(|_error| panic!("missing {key}")))
}

fn write_pueue_config(path: &Path, root: &Path) {
    let text = format!(
        "shared:\n  pueue_directory: {}\n  runtime_directory: {}\n",
        root.display(),
        root.display()
    );
    std::fs::write(path, text).expect("pueue config");
}

fn start_daemon(pueued: &Path, config: &Path, root: &Path) {
    let output = std::process::Command::new(pueued)
        .env_clear()
        .env("HOME", root)
        .env("PUEUE_CONFIG_PATH", config)
        .current_dir(root)
        .arg("--daemonize")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .expect("pueued startup");
    assert!(output.success());
}

struct DaemonGuard {
    pueue: PathBuf,
    config: PathBuf,
    root: PathBuf,
}

impl DaemonGuard {
    const fn new(pueue: PathBuf, config: PathBuf, root: PathBuf) -> Self {
        Self {
            pueue,
            config,
            root,
        }
    }
}

impl Drop for DaemonGuard {
    fn drop(&mut self) {
        let _shutdown = std::process::Command::new(&self.pueue)
            .env_clear()
            .env("HOME", &self.root)
            .env("PUEUE_CONFIG_PATH", &self.config)
            .current_dir(&self.root)
            .args(["--color", "never", "shutdown"])
            .output();
    }
}
