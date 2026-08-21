//! Positive and negative pueue executor-kit tests.

#![cfg_attr(animus_octet, feature(register_tool))]
#![cfg_attr(animus_octet, register_tool(tigerstyle))]
#![cfg_attr(
    animus_octet,
    allow(
        tigerstyle::no_unwrap,
        tigerstyle::non_trait_imports,
        reason = "executor-kit tests use explicit expect messages while asserting admitted and denied boundary outcomes"
    )
)]

mod support;

use animus_runtime::tool_executor_kit::{
    ToolCapabilityAdmission, ToolExecutionFailureClass, ToolExecutorAdmission,
    ToolExecutorKitError, ToolExecutorServiceFailure,
};
use animus_std_pueue::{
    PueueHost, PueueToolError, PueueToolExecutor, PueueToolRequestDraft, build_pueue_tool_request,
    execute_pueue_tool, parse_pueue_host_config,
};

const ROOT: &str = "/tmp/animus-pueue-tool-test";
const CORRELATION_ID: &str = "pueue-tool-test";
const GROUP_PARALLELISM: u32 = 4;
const TASK_ID: u64 = 42;
const EXPECTED_PROCESS_CALLS: usize = 2;

#[test]
fn admitted_group_request_runs_through_executor_kit() {
    let process = support::FakeProcess::with_outputs(vec![
        support::success(support::status_fixture().as_str()),
        support::success(support::status_fixture().as_str()),
    ]);
    let mut executor = fixture_executor(process);
    let request = tool_request(r#"{"operation":"groups"}"#);

    let outcome = execute_pueue_tool(&mut executor, &request, ToolExecutorAdmission::admitted())
        .expect("admitted pueue tool");

    assert!(outcome.response().output().contains("build"));
    assert!(outcome.response().output().contains("running"));
    assert_eq!(
        executor.host().process().calls.len(),
        EXPECTED_PROCESS_CALLS
    );
}

#[test]
fn exact_group_parallelism_tool_request_reaches_only_the_named_group() {
    let process = support::FakeProcess::with_outputs(vec![
        support::success(support::status_fixture().as_str()),
        support::success(""),
    ]);
    let mut executor = fixture_executor(process);
    let request = tool_request(
        format!(
            r#"{{"operation":"control","action":"set_group_parallelism","target":{{"kind":"group","group":"build"}},"parallelism":{GROUP_PARALLELISM}}}"#
        )
        .as_str(),
    );

    execute_pueue_tool(&mut executor, &request, ToolExecutorAdmission::admitted())
        .expect("admitted group parallelism");

    assert_eq!(
        executor.host().process().calls.len(),
        EXPECTED_PROCESS_CALLS
    );
    let args = executor.host().process().calls[1].args();
    assert!(args.ends_with(&[
        String::from("parallel"),
        String::from("--group"),
        String::from("build"),
        GROUP_PARALLELISM.to_string(),
    ]));
}

#[test]
fn missing_parallelism_and_task_parallelism_fail_before_process_effects() {
    let cases = [
        String::from(
            r#"{"operation":"control","action":"set_group_parallelism","target":{"kind":"group","group":"build"}}"#,
        ),
        format!(
            r#"{{"operation":"control","action":"set_group_parallelism","target":{{"kind":"task","task_id":{TASK_ID}}},"parallelism":{GROUP_PARALLELISM}}}"#
        ),
    ];
    for request_json in cases {
        let process = support::FakeProcess::default();
        let mut executor = fixture_executor(process);
        let request = tool_request(request_json.as_str());

        let error = execute_pueue_tool(&mut executor, &request, ToolExecutorAdmission::admitted())
            .expect_err("invalid parallelism request");

        assert!(matches!(
            error,
            PueueToolError::ExecutorKit(ToolExecutorKitError::ServiceFailed {
                failure: ToolExecutorServiceFailure::ExecutionFailed
            })
        ));
        assert!(executor.host().process().calls.is_empty());
    }
}

#[test]
fn raw_command_field_fails_before_process_effects() {
    let process = support::FakeProcess::default();
    let mut executor = fixture_executor(process);
    let request = tool_request(
        r#"{"operation":"enqueue","profile":"fixture-check","command":"echo unsafe"}"#,
    );

    let error = execute_pueue_tool(&mut executor, &request, ToolExecutorAdmission::admitted())
        .expect_err("raw command must fail");

    assert!(matches!(
        error,
        PueueToolError::ExecutorKit(ToolExecutorKitError::ServiceFailed {
            failure: ToolExecutorServiceFailure::ExecutionFailed
        })
    ));
    assert!(executor.host().process().calls.is_empty());
}

#[test]
fn denied_capability_never_calls_the_host() {
    let process = support::FakeProcess::default();
    let mut executor = fixture_executor(process);
    let request = tool_request(r#"{"operation":"groups"}"#);
    let mut admission = ToolExecutorAdmission::admitted();
    admission.capability = ToolCapabilityAdmission::Denied;

    let error = execute_pueue_tool(&mut executor, &request, admission)
        .expect_err("denied capability must fail");

    assert!(matches!(
        error,
        PueueToolError::ExecutorKit(ToolExecutorKitError::PlanDenied {
            failure: ToolExecutionFailureClass::DeniedCapability
        })
    ));
    assert!(executor.host().process().calls.is_empty());
}

#[test]
fn log_output_is_redacted_before_model_visibility() {
    let logs = serde_json::json!({
        "42": {"output": "/home/private/API_KEY=secret"}
    })
    .to_string();
    let process = support::FakeProcess::with_outputs(vec![
        support::success(support::status_fixture().as_str()),
        support::success(logs.as_str()),
    ]);
    let mut executor = fixture_executor(process);
    let request =
        tool_request(r#"{"operation":"logs","task_id":42,"max_lines":8,"max_bytes":1024}"#);

    let outcome = execute_pueue_tool(&mut executor, &request, ToolExecutorAdmission::admitted())
        .expect("redacted log tool");
    let output = outcome.response().output();

    assert!(!output.contains("/home/"));
    assert!(!output.contains("API_KEY="));
    assert!(output.contains("[redacted]"));
}

fn tool_request(request_json: &str) -> animus_adapters::ToolRequest {
    build_pueue_tool_request(PueueToolRequestDraft {
        request_json,
        namespace_target: support::NAMESPACE_ID,
        correlation_id: CORRELATION_ID,
    })
    .expect("tool request")
}

fn fixture_executor(process: support::FakeProcess) -> PueueToolExecutor<support::FakeProcess> {
    let text = support::config_json(
        ROOT,
        support::FAKE_PUEUE,
        support::FAKE_PUEUED,
        support::FAKE_JOB,
    );
    let config = parse_pueue_host_config(text.as_str()).expect("valid fixture config");
    PueueToolExecutor::new(PueueHost::new(config, process))
}
