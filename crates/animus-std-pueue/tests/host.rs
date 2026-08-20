//! Positive and negative pueue host boundary tests.

#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![cfg_attr(
    animus_octet,
    allow(
        tigerstyle::assertion_density,
        tigerstyle::excessive_file_length,
        tigerstyle::no_panic,
        tigerstyle::no_unwrap,
        tigerstyle::non_trait_imports,
        tigerstyle::numeric_units,
        reason = "the host integration matrix uses assertion panics, explicit fixture expects, and task identifiers across paired scenarios"
    )
)]

mod support;

use animus_adapters::{
    ComponentAdapter, OutcomeKind, QueueControlDraft, QueueControlRequest, QueueEnqueueDraft,
    QueueEnqueueRequest, QueueInteractionDraft, QueueInteractionKind, QueueInteractionRequest,
    QueueLogBounds, QueueOperationKind, QueueServiceRequest, QueueServiceResponse,
    QueueTargetDraft, QueueTargetKind, QueueTaskStatus, ServiceOutcome,
};
use animus_std_pueue::{PueueHost, parse_pueue_host_config};

const ROOT: &str = "/tmp/animus-pueue-host-test";
const EXPECTED_PROCESS_CALLS: usize = 2;

#[test]
fn enqueue_uses_reviewed_profile_and_cleared_namespace_environment() {
    let config = fixture_config();
    let bounds = config.limits().log_bounds().expect("log bounds");
    let request = QueueServiceRequest::Enqueue(
        QueueEnqueueRequest::new(QueueEnqueueDraft {
            command_label: "fixture-check",
            group: Some("build"),
            environment_policy: config.environment_policy(),
            timeout_policy: config.timeout_policy(),
            output_bounds: bounds,
        })
        .expect("enqueue request"),
    );
    let process = support::FakeProcess::with_outputs(vec![
        support::success(support::status_fixture().as_str()),
        support::success("42\n"),
    ]);
    let mut host = PueueHost::new(config, process);

    let outcome = host.call(&request).expect("infallible queue host");
    let ServiceOutcome::Completed { value, receipt } = outcome else {
        panic!("enqueue must complete");
    };
    let QueueServiceResponse::Task(task) = value else {
        panic!("enqueue must return a task");
    };
    assert_eq!(task.task_id(), "42");
    assert_eq!(task.group(), "build");
    assert_eq!(receipt.outcome(), OutcomeKind::Completed);
    assert!(receipt.input_hash().is_some());
    assert!(receipt.output_hash().is_some());

    let calls = &host.process().calls;
    assert_eq!(calls.len(), EXPECTED_PROCESS_CALLS);
    assert_enqueue_process_call(&calls[1]);
}

#[test]
fn missing_daemon_fails_closed_without_ambient_startup() {
    let config = fixture_config();
    let process = support::FakeProcess::with_outputs(vec![support::exit_failed()]);
    let mut host = PueueHost::new(config, process);

    let outcome = host
        .call(&QueueServiceRequest::ListGroups)
        .expect("infallible queue host");
    assert_eq!(outcome.kind(), OutcomeKind::Unavailable);
    assert_eq!(host.process().calls.len(), 1);
    assert_eq!(
        host.process().calls[0].program().to_string_lossy(),
        support::FAKE_PUEUE
    );
}

#[test]
fn malformed_backend_output_returns_redacted_failure_class() {
    let config = fixture_config();
    let process = support::FakeProcess::with_outputs(vec![
        support::success(support::status_fixture().as_str()),
        support::success("not-json"),
    ]);
    let mut host = PueueHost::new(config, process);

    let outcome = host
        .call(&QueueServiceRequest::ListGroups)
        .expect("infallible queue host");
    assert_eq!(outcome.kind(), OutcomeKind::MalformedOutput);
    assert!(outcome.receipt().output_hash().is_none());
}

#[test]
fn cancellation_stops_before_daemon_or_queue_effects() {
    let config = fixture_config();
    let process = support::FakeProcess::with_outputs(vec![support::cancelled()]);
    let mut host = PueueHost::new(config, process);

    let outcome = host
        .call(&QueueServiceRequest::ListGroups)
        .expect("infallible queue host");
    assert_eq!(outcome.kind(), OutcomeKind::Cancelled);
    assert_eq!(host.process().calls.len(), 1);
}

#[test]
fn timed_out_process_returns_typed_timeout() {
    let config = fixture_config();
    let process = support::FakeProcess::with_outputs(vec![support::timed_out()]);
    let mut host = PueueHost::new(config, process);

    let outcome = host
        .call(&QueueServiceRequest::ListGroups)
        .expect("infallible queue host");
    assert_eq!(outcome.kind(), OutcomeKind::TimedOut);
}

#[test]
fn oversized_backend_output_returns_malformed_class() {
    let config = fixture_config();
    let process = support::FakeProcess::with_outputs(vec![support::output_limited()]);
    let mut host = PueueHost::new(config, process);

    let outcome = host
        .call(&QueueServiceRequest::ListGroups)
        .expect("infallible queue host");
    assert_eq!(outcome.kind(), OutcomeKind::MalformedOutput);
}

#[test]
fn explicit_control_targets_map_to_exact_pueue_commands() {
    let cases = [
        (
            QueueOperationKind::Pause,
            QueueTargetDraft {
                kind: QueueTargetKind::Group,
                label: Some("build"),
            },
            &["pause", "--group", "build"][..],
        ),
        (
            QueueOperationKind::Resume,
            QueueTargetDraft {
                kind: QueueTargetKind::DefaultGroup,
                label: None,
            },
            &["start", "--group", "default"][..],
        ),
        (
            QueueOperationKind::Restart,
            QueueTargetDraft {
                kind: QueueTargetKind::Task,
                label: Some("42"),
            },
            &["restart", "42"][..],
        ),
        (
            QueueOperationKind::Cancel,
            QueueTargetDraft {
                kind: QueueTargetKind::AllGroups,
                label: None,
            },
            &["kill", "--all"][..],
        ),
        (
            QueueOperationKind::Clean,
            QueueTargetDraft {
                kind: QueueTargetKind::AllGroups,
                label: None,
            },
            &["clean"][..],
        ),
    ];
    for (operation, target, expected_tail) in cases {
        let request = QueueServiceRequest::Control(
            QueueControlRequest::new(QueueControlDraft {
                operation,
                target: Some(target),
            })
            .expect("control request"),
        );
        let process = support::FakeProcess::with_outputs(vec![
            support::success(support::status_fixture().as_str()),
            support::success(""),
        ]);
        let mut host = PueueHost::new(fixture_config(), process);

        let outcome = host.call(&request).expect("infallible queue host");
        assert_eq!(outcome.kind(), OutcomeKind::Completed);
        assert!(arguments_end_with(
            host.process().calls[1].args(),
            expected_tail
        ));
    }
}

#[test]
fn unsupported_parallelism_control_is_denied_before_process_effects() {
    let request = QueueServiceRequest::Control(
        QueueControlRequest::new(QueueControlDraft {
            operation: QueueOperationKind::SetGroupParallelism,
            target: Some(QueueTargetDraft {
                kind: QueueTargetKind::Group,
                label: Some("build"),
            }),
        })
        .expect("control request"),
    );
    let mut host = PueueHost::new(fixture_config(), support::FakeProcess::default());

    let outcome = host.call(&request).expect("infallible queue host");
    assert_eq!(outcome.kind(), OutcomeKind::Denied);
    assert!(host.process().calls.is_empty());
}

#[test]
fn bounded_wait_polls_until_terminal_status() {
    let config = fixture_config();
    let task_id = "42";
    let timeout_policy = String::from(config.timeout_policy());
    let request = QueueServiceRequest::Interaction(
        QueueInteractionRequest::new(QueueInteractionDraft {
            kind: QueueInteractionKind::Wait,
            task_id,
            timeout_policy: timeout_policy.as_str(),
            capabilities: config.capabilities(),
            input_hash: None,
        })
        .expect("wait request"),
    );
    let mut terminal: serde_json::Value =
        serde_json::from_str(support::status_fixture().as_str()).expect("status fixture");
    terminal["tasks"][task_id]["status"] = serde_json::json!({
        "Done": {"result": "Success"}
    });
    let process = support::FakeProcess::with_outputs(vec![
        support::success(support::status_fixture().as_str()),
        support::success(support::status_fixture().as_str()),
        support::success(terminal.to_string().as_str()),
    ]);
    let mut host = PueueHost::new(config, process);

    let outcome = host.call(&request).expect("infallible queue host");
    let ServiceOutcome::Completed { value, .. } = outcome else {
        panic!("wait request must complete");
    };
    let QueueServiceResponse::Task(task) = value else {
        panic!("wait request must return a task");
    };
    assert_eq!(task.status(), QueueTaskStatus::Completed);
}

#[test]
fn bounded_log_projection_records_full_output_identity() {
    let config = fixture_config();
    let bounds = QueueLogBounds::new(2, 128).expect("log bounds");
    let log_request =
        animus_adapters::QueueLogRequest::new(animus_adapters::QueueLogRequestDraft {
            task_id: "42",
            bounds,
        })
        .expect("log request");
    let logs = serde_json::json!({
        "42": {"output": "first\nsecond\nthird"}
    })
    .to_string();
    let process = support::FakeProcess::with_outputs(vec![
        support::success(support::status_fixture().as_str()),
        support::success(logs.as_str()),
    ]);
    let mut host = PueueHost::new(config, process);

    let outcome = host
        .call(&QueueServiceRequest::ReadLog(log_request))
        .expect("infallible queue host");
    let ServiceOutcome::Completed { value, .. } = outcome else {
        panic!("log request must complete");
    };
    let QueueServiceResponse::Log(viewport) = value else {
        panic!("log request must return viewport");
    };
    assert_eq!(viewport.visible_text(), "second\nthird");
    assert!(viewport.truncated());
    assert!(viewport.full_output_hash().is_some());
    assert_eq!(viewport.safe_reference(), Some("pueue-job-42-log"));
}

fn assert_enqueue_process_call(enqueue: &animus_std_pueue::PueueProcessRequest) {
    assert_eq!(enqueue.program().to_string_lossy(), support::FAKE_PUEUE);
    assert!(enqueue.args().iter().any(|argument| argument == "add"));
    assert!(enqueue.args().iter().any(|argument| argument == "--"));
    assert!(
        enqueue
            .args()
            .iter()
            .any(|argument| argument == support::FAKE_TIMEOUT)
    );
    assert!(
        enqueue
            .args()
            .iter()
            .any(|argument| argument == support::FAKE_JOB)
    );
    for required_key in ["PUEUE_CONFIG_PATH", "HOME", "ANIMUS_TASK"] {
        assert!(
            enqueue
                .environment()
                .iter()
                .any(|(key, _value)| key == required_key)
        );
    }
    assert!(
        !enqueue
            .environment()
            .iter()
            .any(|(key, _value)| key == "PATH")
    );
}

fn arguments_end_with(actual: &[String], expected: &[&str]) -> bool {
    actual
        .iter()
        .rev()
        .take(expected.len())
        .map(String::as_str)
        .eq(expected.iter().rev().copied())
}

fn fixture_config() -> animus_std_pueue::PueueHostConfig {
    let text = support::config_json(
        ROOT,
        support::FAKE_PUEUE,
        support::FAKE_PUEUED,
        support::FAKE_JOB,
    );
    parse_pueue_host_config(text.as_str()).expect("valid fixture config")
}
