use animus_adapters::pijul::{
    PIJUL_CLI_RELEASE, PIJUL_COHORT_IDENTITY, PijulBackendService, PijulBoundedExecutor,
    PijulConfirmation, PijulExecutable, PijulExecutionOutcome, PijulExecutionRequest,
    PijulExecutionStatus, PijulLoadedOutput, PijulOperation, PijulRequest, PijulServiceIssue,
    PijulServiceIssueKind,
};
use animus_runtime::pijul::parse_pijul_profile_json;

use super::*;

const PROFILE_JSON: &str = include_str!("../../../../config/generated/pijul-vcs-adapter.json");
const REPOSITORY_IDENTITY: &str =
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const CHANGE_HASH: &str = "QRYOAJLYA5KO7JAYU6LMYHPU3NSZCTUMYEPCK3HNZIRIDZDWSMBAC";
const ZERO_EXIT_CODE: i32 = 0;
const FAILURE_EXIT_CODE: i32 = 2;
const DIFF_RECORD_COUNT: usize = 2;

#[derive(Clone, Debug)]
struct FakeExecutor {
    requests: Vec<PijulExecutionRequest>,
    outcome: PijulExecutionOutcome,
}

impl PijulBoundedExecutor for FakeExecutor {
    fn execute(
        &mut self,
        request: &PijulExecutionRequest,
    ) -> Result<PijulExecutionOutcome, PijulServiceIssue> {
        self.requests.push(request.clone());
        Ok(self.outcome.clone())
    }
}

#[derive(Clone, Debug, Default)]
struct UnavailableExecutor {
    calls: usize,
}

impl PijulBoundedExecutor for UnavailableExecutor {
    fn execute(
        &mut self,
        _request: &PijulExecutionRequest,
    ) -> Result<PijulExecutionOutcome, PijulServiceIssue> {
        self.calls = self.calls.saturating_add(1);
        Err(PijulServiceIssue::new(
            PijulServiceIssueKind::Unavailable,
            "injected-executor",
        ))
    }
}

fn profile() -> PijulProfile {
    parse_pijul_profile_json(PROFILE_JSON).expect("generated profile must validate")
}

fn executable() -> PijulExecutable {
    PijulExecutable {
        path: "/nix/store/exact-pijul/bin/pijul".to_owned(),
        identity: PIJUL_COHORT_IDENTITY.to_owned(),
        release: PIJUL_CLI_RELEASE.to_owned(),
    }
}

fn request(operation: PijulOperation) -> PijulRequest {
    PijulRequest {
        operation,
        repository_path: "/explicit/repository".to_owned(),
        repository_identity: REPOSITORY_IDENTITY.to_owned(),
        channel: Some("main".to_owned()),
        change_selector: None,
        confirmation: PijulConfirmation::NotRequired,
    }
}

fn outcome(output: &[u8]) -> PijulExecutionOutcome {
    PijulExecutionOutcome {
        status: PijulExecutionStatus::Exited,
        exit_code: Some(ZERO_EXIT_CODE),
        standard_output: output.to_vec(),
        standard_error: Vec::new(),
        stdout_truncated: false,
        stderr_truncated: false,
    }
}

fn service(output: &[u8]) -> PijulCliService<FakeExecutor> {
    PijulCliService::new(
        FakeExecutor {
            requests: Vec::new(),
            outcome: outcome(output),
        },
        profile(),
        executable(),
    )
    .expect("exact cohort must construct")
}

#[test]
fn log_plan_uses_hash_only_json_without_identity_or_state_loading() {
    let request = request(PijulOperation::Log);
    let plan =
        plan_pijul_invocation(&profile(), &executable(), &request).expect("log query must plan");
    assert_eq!(plan.arguments.first().map(String::as_str), Some("log"));
    assert!(
        plan.arguments
            .iter()
            .any(|argument| argument == "--hash-only")
    );
    assert!(plan.arguments.iter().any(|argument| argument == "json"));
    assert!(!plan.arguments.iter().any(|argument| argument == "--state"));
    assert!(
        !plan
            .arguments
            .iter()
            .any(|argument| argument == "--description")
    );
    assert!(!plan.arguments.iter().any(|argument| argument == "--files"));
    assert!(plan.environment.is_empty());
    assert!(!plan.inherit_environment);
    assert!(plan.standard_input.is_empty());
}

#[test]
fn valid_hash_log_is_parsed_and_revalidated() {
    let output = format!("[\"{CHANGE_HASH}\"]");
    let mut service = service(output.as_bytes());
    let response = service
        .execute(&request(PijulOperation::Log))
        .expect("reviewed hash log must pass");
    let Some(PijulLoadedOutput::Log(entries)) = response.loaded else {
        panic!("expected loaded log")
    };
    assert_eq!(entries.len(), 1);
    assert!(entries[0].authors.is_empty());
    assert!(entries[0].state.is_none());
    let executor = service.into_executor();
    assert_eq!(executor.requests.len(), 1);
}

#[test]
fn rich_log_json_is_rejected_without_identity_file_authority() {
    let output = format!("[{{\"hash\":\"{CHANGE_HASH}\",\"authors\":[\"private\"]}}]");
    let mut service = service(output.as_bytes());
    let error = service
        .execute(&request(PijulOperation::Log))
        .expect_err("rich log schema must fail");
    assert_eq!(error.kind(), PijulServiceIssueKind::ProtocolFailure);
}

#[test]
fn diff_json_is_flattened_in_stable_path_order() {
    let output = br#"{
      "src/z.rs": [{"operation":"edit","line":2}],
      "src/a.rs": [{"operation":"solve order conflict","line":null}]
    }"#;
    let mut service = service(output);
    let response = service
        .execute(&request(PijulOperation::Diff))
        .expect("reviewed diff JSON must pass");
    let Some(PijulLoadedOutput::Diff(entries)) = response.loaded else {
        panic!("expected loaded diff")
    };
    assert_eq!(entries.len(), DIFF_RECORD_COUNT);
    assert_eq!(entries[0].path, "src/a.rs");
    assert_eq!(entries[1].path, "src/z.rs");
}

#[test]
fn unknown_diff_field_is_rejected() {
    let output = br#"{"src/lib.rs":[{"operation":"edit","line":1,"body":"secret"}]}"#;
    let mut service = service(output);
    let error = service
        .execute(&request(PijulOperation::Diff))
        .expect_err("unknown diff fields must fail");
    assert_eq!(error.kind(), PijulServiceIssueKind::ProtocolFailure);
}

#[test]
fn reviewed_channel_lines_are_parsed() {
    let mut service = service(b"* main\n");
    let response = service
        .execute(&request(PijulOperation::ChannelList))
        .expect("reviewed channel line must pass");
    let Some(PijulLoadedOutput::Channels(channels)) = response.loaded else {
        panic!("expected channels")
    };
    assert_eq!(channels.len(), 1);
    assert!(channels[0].is_current);
}

#[test]
fn unreviewed_channel_text_is_rejected() {
    let mut service = service(b"current: main\n");
    let error = service
        .execute(&request(PijulOperation::ChannelList))
        .expect_err("unreviewed line format must fail");
    assert_eq!(error.kind(), PijulServiceIssueKind::ProtocolFailure);
}

#[test]
fn protected_mutation_has_zero_executor_calls() {
    let mut protected = request(PijulOperation::Record);
    protected.confirmation = PijulConfirmation::Confirmed;
    let mut service = service(b"");
    let error = service
        .execute(&protected)
        .expect_err("record is not dispatched");
    assert_eq!(error.kind(), PijulServiceIssueKind::UnsupportedOperation);
    assert!(service.into_executor().requests.is_empty());
}

#[test]
fn unavailable_injected_executor_returns_typed_failure() {
    let mut service = PijulCliService::new(UnavailableExecutor::default(), profile(), executable())
        .expect("exact service must construct");
    let error = service
        .execute(&request(PijulOperation::Log))
        .expect_err("unavailable executor must fail");
    assert_eq!(error.kind(), PijulServiceIssueKind::Unavailable);
    assert_eq!(service.into_executor().calls, 1);
}

#[test]
fn nonzero_exit_is_rejected_before_parse() {
    let mut service = service(b"not-json");
    service.executor.outcome.exit_code = Some(FAILURE_EXIT_CODE);
    let error = service
        .execute(&request(PijulOperation::Log))
        .expect_err("nonzero exit must fail");
    assert_eq!(error.kind(), PijulServiceIssueKind::ExecutorFailure);
}

#[test]
fn wrong_executable_cohort_is_rejected() {
    let mut wrong = executable();
    wrong.release = "1.0.0-beta.17".to_owned();
    let error = PijulCliService::new(
        FakeExecutor {
            requests: Vec::new(),
            outcome: outcome(b"[]"),
        },
        profile(),
        wrong,
    )
    .expect_err("wrong release must fail");
    assert_eq!(error.kind(), PijulServiceIssueKind::CohortMismatch);
}
