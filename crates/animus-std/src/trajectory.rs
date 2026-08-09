//! Thin JSON protocol shell for an injected trajectory normalizer executor.

use animus_adapters::trajectory::{
    TrajectoryDiagnostic, TrajectoryDiagnosticCode, TrajectoryNormalizeRequest,
    TrajectoryNormalizeResult, TrajectoryNormalizerService, TrajectoryPolicy, TrajectoryRecord,
    TrajectoryServiceIssue, TrajectoryToolCall,
};
use serde::{Deserialize, Serialize};

const HELPER_PROTOCOL_VERSION: u8 = 1;
const SINGLE_REQUEST_COUNT: usize = 1;

/// Terminal outcome from a caller-owned bounded executor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrajectoryExecutionStatus {
    /// Child exited successfully.
    Succeeded,
    /// Child exited unsuccessfully.
    Failed,
    /// Deadline expired.
    TimedOut,
    /// Caller cancelled execution.
    Cancelled,
}

/// Explicit bounded child request. It contains no executable path lookup behavior.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrajectoryExecutionRequest {
    /// Exact helper artifact identity.
    pub helper_identity: String,
    /// Typed protocol request sent on bounded stdin.
    pub stdin: String,
    /// Maximum accepted stdout bytes.
    pub max_stdout_bytes: usize,
    /// Maximum accepted stderr bytes.
    pub max_stderr_bytes: usize,
    /// Named deadline class interpreted by the caller.
    pub deadline_class: &'static str,
    /// Named memory class interpreted by the caller.
    pub memory_class: &'static str,
}

/// Bounded child observation returned by an injected executor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrajectoryExecutionOutcome {
    /// Terminal class.
    pub status: TrajectoryExecutionStatus,
    /// Bounded stdout.
    pub stdout: String,
    /// Retained stderr byte count. Stderr content is never accepted here.
    pub stderr_bytes: usize,
    /// Whether stdout was truncated.
    pub stdout_truncated: bool,
    /// Whether stderr was truncated.
    pub stderr_truncated: bool,
}

/// Caller-owned bounded execution service.
pub trait TrajectoryExecutionService {
    /// Runs one explicit helper request.
    ///
    /// # Errors
    ///
    /// Returns a bounded service issue without helper output or transcript text.
    fn execute(
        &mut self,
        request: &TrajectoryExecutionRequest,
    ) -> Result<TrajectoryExecutionOutcome, TrajectoryServiceIssue>;
}

/// Injected normalizer implementation over a caller-owned executor.
pub struct InjectedTrajectoryNormalizer<'a> {
    policy: &'a TrajectoryPolicy,
    executor: &'a mut dyn TrajectoryExecutionService,
}

impl<'a> InjectedTrajectoryNormalizer<'a> {
    /// Creates a thin host shell. No discovery or effects occur.
    #[must_use]
    pub const fn new(
        policy: &'a TrajectoryPolicy,
        executor: &'a mut dyn TrajectoryExecutionService,
    ) -> Self {
        Self { policy, executor }
    }
}

impl TrajectoryNormalizerService for InjectedTrajectoryNormalizer<'_> {
    fn normalize(
        &mut self,
        request: &TrajectoryNormalizeRequest,
    ) -> Result<TrajectoryNormalizeResult, TrajectoryServiceIssue> {
        if request.helper_identity != self.policy.helper_identity {
            return Err(TrajectoryServiceIssue::CohortMismatch);
        }
        if request.transcript.len() > self.policy.bounds.max_transcript_bytes {
            return Err(TrajectoryServiceIssue::RequestBound);
        }
        let stdin = render_request(self.policy, request)?;
        let execution_request = TrajectoryExecutionRequest {
            helper_identity: request.helper_identity.clone(),
            stdin,
            max_stdout_bytes: self.policy.bounds.max_output_bytes,
            max_stderr_bytes: self.policy.bounds.max_output_bytes,
            deadline_class: "trajectory-short",
            memory_class: "trajectory-bounded",
        };
        let outcome = self.executor.execute(&execution_request)?;
        validate_outcome(self.policy, &outcome)?;
        parse_response(outcome.stdout.as_str())
    }
}

/// Normalizes through an explicitly supplied implementation-free service.
///
/// # Errors
///
/// Returns unavailable before any fallback when no service is supplied.
pub fn normalize_with_service(
    service: Option<&mut dyn TrajectoryNormalizerService>,
    request: &TrajectoryNormalizeRequest,
) -> Result<TrajectoryNormalizeResult, TrajectoryServiceIssue> {
    service
        .ok_or(TrajectoryServiceIssue::Unavailable)?
        .normalize(request)
}

/// Generated profile parse or cohort validation issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TrajectoryProfileIssue {
    /// JSON or field shape is malformed.
    Json,
    /// Exact reviewed cohort or mandatory policy posture differs.
    Cohort,
    /// Pure Rust policy validation rejected the projected values.
    Policy,
}

/// Parses generated Nickel JSON and revalidates the exact reviewed policy.
///
/// # Errors
///
/// Returns a bounded issue for malformed JSON, unknown fields, cohort drift, or
/// pure policy validation failure.
pub fn parse_trajectory_policy_json(
    value: &str,
) -> Result<TrajectoryPolicy, TrajectoryProfileIssue> {
    let root: WireProfileRoot =
        serde_json::from_str(value).map_err(|_error| TrajectoryProfileIssue::Json)?;
    let profile = root.trajectory_transcript_adapter;
    if profile.schema != "animus.trajectory-transcript-adapter.v1"
        || profile.enabled_by_default
        || profile.cohort.package != animus_adapters::trajectory::TRAJECTORY_PACKAGE
        || profile.cohort.version != animus_adapters::trajectory::TRAJECTORY_VERSION
        || profile.cohort.revision != animus_adapters::trajectory::TRAJECTORY_REVISION
        || profile.cohort.trajectory_schema != animus_adapters::trajectory::TRAJECTORY_SCHEMA
        || profile.cohort.license != animus_adapters::trajectory::TRAJECTORY_LICENSE
        || profile.helper_protocol != "trajectory-bridge-v1"
        || profile.timeout_class != "trajectory-short"
        || profile.memory_class != "trajectory-bounded"
        || profile.projection != "ledger-and-batch-drafts"
        || profile.receipt_schema != "animus.trajectory-transcript-receipt.v1"
        || profile.content.reasoning != "omit"
        || profile.content.path_metadata != "omit"
        || !profile
            .non_claims
            .iter()
            .map(String::as_str)
            .eq(REQUIRED_PROFILE_NON_CLAIMS)
    {
        return Err(TrajectoryProfileIssue::Cohort);
    }
    let sources = profile
        .cohort
        .sources
        .iter()
        .map(|source| profile_source(source))
        .collect::<Result<Vec<_>, _>>()?;
    let diagnostics = profile
        .diagnostics
        .into_iter()
        .map(|row| {
            Ok(animus_adapters::trajectory::TrajectoryDiagnosticPolicy {
                code: TrajectoryDiagnosticCode::parse(row.code.as_str())
                    .ok_or(TrajectoryProfileIssue::Cohort)?,
                disposition: profile_disposition(row.disposition.as_str())?,
            })
        })
        .collect::<Result<Vec<_>, TrajectoryProfileIssue>>()?;
    let policy = TrajectoryPolicy {
        helper_identity: profile.cohort.helper_blake3,
        schema_identity: profile.cohort.trajectory_schema_blake3,
        sources,
        bounds: animus_adapters::trajectory::TrajectoryBounds {
            max_transcript_bytes: profile.bounds.max_transcript_bytes,
            max_output_bytes: profile.bounds.max_output_bytes,
            max_records: profile.bounds.max_records,
            max_content_bytes: profile.bounds.max_content_bytes,
            max_tool_calls: profile.bounds.max_tool_calls,
            max_tool_argument_bytes: profile.bounds.max_tool_argument_bytes,
            max_tool_result_bytes: profile.bounds.max_tool_result_bytes,
            max_diagnostics: profile.bounds.max_diagnostics,
        },
        diagnostics,
        reasoning: profile_content_disposition(profile.content.reasoning.as_str())?,
        tool_arguments: profile_content_disposition(profile.content.tool_arguments.as_str())?,
        tool_results: profile_content_disposition(profile.content.tool_results.as_str())?,
        path_metadata: profile_content_disposition(profile.content.path_metadata.as_str())?,
    };
    animus_runtime::trajectory::validate_policy(&policy)
        .map_err(|_issue| TrajectoryProfileIssue::Policy)?;
    Ok(policy)
}

const REQUIRED_PROFILE_NON_CLAIMS: [&str; 9] = [
    "source-authenticity",
    "transcript-completeness",
    "timestamp-truth",
    "tool-execution-truth",
    "semantic-equivalence",
    "privacy",
    "model-correctness",
    "dataset-export-fitness",
    "release-eligibility",
];

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireProfileRoot {
    trajectory_transcript_adapter: WireProfile,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireProfile {
    schema: String,
    enabled_by_default: bool,
    cohort: WireCohort,
    bounds: WireProfileBounds,
    diagnostics: Vec<WireDiagnosticPolicy>,
    content: WireContentPolicy,
    projection: String,
    helper_protocol: String,
    timeout_class: String,
    memory_class: String,
    receipt_schema: String,
    non_claims: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireCohort {
    package: String,
    version: String,
    revision: String,
    trajectory_schema: String,
    trajectory_schema_blake3: String,
    helper_blake3: String,
    license: String,
    sources: Vec<String>,
}

#[allow(clippy::struct_field_names)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireProfileBounds {
    max_transcript_bytes: usize,
    max_output_bytes: usize,
    max_records: usize,
    max_content_bytes: usize,
    max_tool_calls: usize,
    max_tool_argument_bytes: usize,
    max_tool_result_bytes: usize,
    max_diagnostics: usize,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireDiagnosticPolicy {
    code: String,
    disposition: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireContentPolicy {
    reasoning: String,
    tool_arguments: String,
    tool_results: String,
    path_metadata: String,
}

fn profile_source(
    value: &str,
) -> Result<animus_adapters::trajectory::TrajectorySource, TrajectoryProfileIssue> {
    let upstream = match value {
        "claude_code" => "claude-code",
        "letta_code" => "letta-code",
        other => other,
    };
    animus_adapters::trajectory::TrajectorySource::parse(upstream)
        .ok_or(TrajectoryProfileIssue::Cohort)
}

fn profile_content_disposition(
    value: &str,
) -> Result<animus_adapters::trajectory::TrajectoryContentDisposition, TrajectoryProfileIssue> {
    match value {
        "include" => Ok(animus_adapters::trajectory::TrajectoryContentDisposition::Include),
        "omit" => Ok(animus_adapters::trajectory::TrajectoryContentDisposition::Omit),
        _ => Err(TrajectoryProfileIssue::Cohort),
    }
}

fn profile_disposition(
    value: &str,
) -> Result<animus_adapters::trajectory::DiagnosticDisposition, TrajectoryProfileIssue> {
    match value {
        "accepted_cleanup" => {
            Ok(animus_adapters::trajectory::DiagnosticDisposition::AcceptedCleanup)
        }
        "lossy" => Ok(animus_adapters::trajectory::DiagnosticDisposition::Lossy),
        "rejected" => Ok(animus_adapters::trajectory::DiagnosticDisposition::Rejected),
        _ => Err(TrajectoryProfileIssue::Cohort),
    }
}

#[derive(Serialize)]
struct WireRequestEnvelope<'a> {
    version: u8,
    requests: [WireNormalizeRequest<'a>; SINGLE_REQUEST_COUNT],
}

#[derive(Serialize)]
struct WireNormalizeRequest<'a> {
    source: &'a str,
    transcript: &'a str,
    bounds: WireBounds,
    filters: WireFilters,
}

#[derive(Clone, Copy, Serialize)]
struct WireBounds {
    #[serde(rename = "toolArguments")]
    tool_arguments: WireCharacterBound,
    #[serde(rename = "toolResults")]
    tool_results: WireToolResultBound,
}

#[derive(Clone, Copy, Serialize)]
struct WireCharacterBound {
    #[serde(rename = "maxCharacters")]
    max_characters: usize,
}

#[derive(Clone, Copy, Serialize)]
struct WireToolResultBound {
    #[serde(rename = "maxCharacters")]
    max_characters: usize,
    strategy: &'static str,
}

#[derive(Clone, Copy, Serialize)]
struct WireFilters {
    #[serde(rename = "toolResults")]
    tool_results: &'static str,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireResponseEnvelope {
    version: u8,
    results: Vec<WireResponseItem>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireResponseItem {
    ok: bool,
    result: Option<WireResult>,
    error: Option<WireError>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireError {
    name: String,
    code: String,
    message: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireResult {
    records: Vec<WireRecord>,
    diagnostics: Vec<WireDiagnostic>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireRecord {
    role: String,
    source: Option<String>,
    cwd: Option<String>,
    git_branch: Option<String>,
    model: Option<String>,
    content: Option<serde_json::Value>,
    timestamp: Option<String>,
    tool_calls: Option<Vec<WireToolCall>>,
    tool_call_id: Option<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireToolCall {
    id: String,
    name: String,
    args: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct WireDiagnostic {
    code: String,
    message: String,
    #[serde(rename = "inputLine")]
    input_line: Option<u32>,
    #[serde(rename = "recordIndex")]
    record_index: Option<u32>,
    count: Option<u32>,
}

fn render_request(
    policy: &TrajectoryPolicy,
    request: &TrajectoryNormalizeRequest,
) -> Result<String, TrajectoryServiceIssue> {
    let envelope = WireRequestEnvelope {
        version: HELPER_PROTOCOL_VERSION,
        requests: [WireNormalizeRequest {
            source: request.source.as_str(),
            transcript: request.transcript.as_str(),
            bounds: WireBounds {
                tool_arguments: WireCharacterBound {
                    max_characters: policy.bounds.max_tool_argument_bytes,
                },
                tool_results: WireToolResultBound {
                    max_characters: policy.bounds.max_tool_result_bytes,
                    strategy: "head-tail",
                },
            },
            filters: WireFilters {
                tool_results: if policy.tool_results
                    == animus_adapters::trajectory::TrajectoryContentDisposition::Include
                {
                    "include"
                } else {
                    "omit"
                },
            },
        }],
    };
    serde_json::to_string(&envelope).map_err(|_error| TrajectoryServiceIssue::Protocol)
}

fn validate_outcome(
    policy: &TrajectoryPolicy,
    outcome: &TrajectoryExecutionOutcome,
) -> Result<(), TrajectoryServiceIssue> {
    if outcome.status != TrajectoryExecutionStatus::Succeeded {
        return Err(TrajectoryServiceIssue::Execution);
    }
    if outcome.stdout_truncated
        || outcome.stderr_truncated
        || outcome.stdout.len() > policy.bounds.max_output_bytes
        || outcome.stderr_bytes > policy.bounds.max_output_bytes
    {
        return Err(TrajectoryServiceIssue::Protocol);
    }
    Ok(())
}

fn parse_response(value: &str) -> Result<TrajectoryNormalizeResult, TrajectoryServiceIssue> {
    let envelope: WireResponseEnvelope =
        serde_json::from_str(value).map_err(|_error| TrajectoryServiceIssue::Protocol)?;
    if envelope.version != HELPER_PROTOCOL_VERSION || envelope.results.len() != SINGLE_REQUEST_COUNT
    {
        return Err(TrajectoryServiceIssue::Protocol);
    }
    let item = envelope
        .results
        .into_iter()
        .next()
        .ok_or(TrajectoryServiceIssue::Protocol)?;
    if !item.ok || item.error.is_some() {
        if let Some(error) = item.error {
            let _discarded = (error.name.len(), error.code.len(), error.message.len());
        }
        return Err(TrajectoryServiceIssue::Protocol);
    }
    let result = item.result.ok_or(TrajectoryServiceIssue::Protocol)?;
    let records = result
        .records
        .into_iter()
        .map(convert_record)
        .collect::<Result<Vec<_>, _>>()?;
    let diagnostics = result
        .diagnostics
        .iter()
        .map(convert_diagnostic)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(TrajectoryNormalizeResult {
        records,
        diagnostics,
    })
}

fn convert_record(record: WireRecord) -> Result<TrajectoryRecord, TrajectoryServiceIssue> {
    match record.role.as_str() {
        "meta" => {
            ensure_absent(
                &record,
                &["content", "timestamp", "tool_calls", "tool_call_id"],
            )?;
            Ok(TrajectoryRecord::Meta {
                source: record.source.ok_or(TrajectoryServiceIssue::Protocol)?,
                cwd: record.cwd,
                git_branch: record.git_branch,
                model: record.model,
            })
        }
        "user" | "reasoning" | "assistant" if record.tool_calls.is_none() => {
            ensure_absent(
                &record,
                &["source", "cwd", "git_branch", "model", "tool_call_id"],
            )?;
            let content = string_content(record.content)?;
            let timestamp = record.timestamp.ok_or(TrajectoryServiceIssue::Protocol)?;
            match record.role.as_str() {
                "user" => Ok(TrajectoryRecord::User { content, timestamp }),
                "reasoning" => Ok(TrajectoryRecord::Reasoning { content, timestamp }),
                "assistant" => Ok(TrajectoryRecord::Assistant { content, timestamp }),
                _ => Err(TrajectoryServiceIssue::Protocol),
            }
        }
        "assistant" => {
            ensure_absent(
                &record,
                &["source", "cwd", "git_branch", "model", "tool_call_id"],
            )?;
            if !matches!(record.content, Some(serde_json::Value::Null)) {
                return Err(TrajectoryServiceIssue::Protocol);
            }
            let calls = record
                .tool_calls
                .ok_or(TrajectoryServiceIssue::Protocol)?
                .into_iter()
                .map(|call| TrajectoryToolCall {
                    id: call.id,
                    name: call.name,
                    arguments: call.args,
                })
                .collect();
            Ok(TrajectoryRecord::AssistantToolCalls {
                tool_calls: calls,
                timestamp: record.timestamp.ok_or(TrajectoryServiceIssue::Protocol)?,
            })
        }
        "tool" => {
            ensure_absent(
                &record,
                &["source", "cwd", "git_branch", "model", "tool_calls"],
            )?;
            Ok(TrajectoryRecord::Tool {
                tool_call_id: record
                    .tool_call_id
                    .ok_or(TrajectoryServiceIssue::Protocol)?,
                content: string_content(record.content)?,
                timestamp: record.timestamp.ok_or(TrajectoryServiceIssue::Protocol)?,
            })
        }
        _ => Err(TrajectoryServiceIssue::Protocol),
    }
}

fn convert_diagnostic(
    diagnostic: &WireDiagnostic,
) -> Result<TrajectoryDiagnostic, TrajectoryServiceIssue> {
    let code = TrajectoryDiagnosticCode::parse(diagnostic.code.as_str())
        .ok_or(TrajectoryServiceIssue::Protocol)?;
    let _discarded = (
        diagnostic.message.len(),
        diagnostic.input_line,
        diagnostic.record_index,
    );
    Ok(TrajectoryDiagnostic {
        code,
        count: diagnostic.count,
    })
}

fn string_content(content: Option<serde_json::Value>) -> Result<String, TrajectoryServiceIssue> {
    content
        .and_then(|value| value.as_str().map(String::from))
        .ok_or(TrajectoryServiceIssue::Protocol)
}

fn ensure_absent(record: &WireRecord, fields: &[&str]) -> Result<(), TrajectoryServiceIssue> {
    let present = fields.iter().any(|field| match *field {
        "source" => record.source.is_some(),
        "cwd" => record.cwd.is_some(),
        "git_branch" => record.git_branch.is_some(),
        "model" => record.model.is_some(),
        "content" => record.content.is_some(),
        "timestamp" => record.timestamp.is_some(),
        "tool_calls" => record.tool_calls.is_some(),
        "tool_call_id" => record.tool_call_id.is_some(),
        _ => true,
    });
    if present {
        return Err(TrajectoryServiceIssue::Protocol);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::trajectory::{
        DiagnosticDisposition, TRAJECTORY_HELPER_BLAKE3, TRAJECTORY_SCHEMA_BLAKE3,
        TrajectoryBounds, TrajectoryContentDisposition, TrajectoryDiagnosticPolicy,
        TrajectorySource,
    };

    const POLICY_IDENTITY: &str = "policy:fixture";
    const BLAKE3_HEX_LENGTH: usize = 64;
    const FIXTURE_MAX_TRANSCRIPT_BYTES: usize = 1_024;
    const FIXTURE_MAX_OUTPUT_BYTES: usize = 4_096;
    const FIXTURE_MAX_RECORDS: usize = 16;
    const FIXTURE_MAX_CONTENT_BYTES: usize = 1_024;
    const FIXTURE_MAX_TOOL_CALLS: usize = 16;
    const FIXTURE_MAX_TOOL_ARGUMENT_BYTES: usize = 1_024;
    const FIXTURE_MAX_TOOL_RESULT_BYTES: usize = 1_024;
    const FIXTURE_MAX_DIAGNOSTICS: usize = 16;

    #[test]
    fn generated_policy_parses_and_drift_fails_closed() {
        let source = include_str!("../../../config/generated/trajectory-transcript-adapter.json");
        let parsed = parse_trajectory_policy_json(source).expect("checked policy");
        assert_eq!(parsed.helper_identity, TRAJECTORY_HELPER_BLAKE3);
        let drifted = source.replace("0.2.0", "0.3.0");
        assert_eq!(
            parse_trajectory_policy_json(drifted.as_str()),
            Err(TrajectoryProfileIssue::Cohort)
        );
        let unknown = source.replace(
            "\"enabled_by_default\": false,",
            "\"enabled_by_default\": false, \"unknown\": true,",
        );
        assert_eq!(
            parse_trajectory_policy_json(unknown.as_str()),
            Err(TrajectoryProfileIssue::Json)
        );
    }

    #[test]
    fn injected_executor_receives_one_bounded_request_and_parses_result() {
        let policy = policy();
        let mut executor = FakeExecutor::success(valid_response());
        let mut normalizer = InjectedTrajectoryNormalizer::new(&policy, &mut executor);
        let result = normalizer.normalize(&request()).expect("normalized");
        assert_eq!(result.records.len(), 3);
        assert_eq!(executor.calls, 1);
        assert!(executor.last_stdin.contains("\"source\":\"pi\""));
        assert!(executor.last_stdin.contains("\"version\":1"));
    }

    #[test]
    fn absent_service_and_helper_mismatch_fail_before_execution() {
        assert_eq!(
            normalize_with_service(None, &request()),
            Err(TrajectoryServiceIssue::Unavailable)
        );
        let mut mismatched = request();
        mismatched.helper_identity = String::from("0").repeat(BLAKE3_HEX_LENGTH);
        let policy = policy();
        let mut executor = FakeExecutor::success(valid_response());
        let mut normalizer = InjectedTrajectoryNormalizer::new(&policy, &mut executor);
        assert_eq!(
            normalizer.normalize(&mismatched),
            Err(TrajectoryServiceIssue::CohortMismatch)
        );
        assert_eq!(executor.calls, 0);
    }

    #[test]
    fn failed_truncated_corrupt_and_unknown_protocols_fail_closed() {
        for outcome in [
            TrajectoryExecutionOutcome {
                status: TrajectoryExecutionStatus::Failed,
                stdout: valid_response(),
                stderr_bytes: 0,
                stdout_truncated: false,
                stderr_truncated: false,
            },
            TrajectoryExecutionOutcome {
                status: TrajectoryExecutionStatus::Succeeded,
                stdout: valid_response(),
                stderr_bytes: 0,
                stdout_truncated: true,
                stderr_truncated: false,
            },
            TrajectoryExecutionOutcome {
                status: TrajectoryExecutionStatus::Succeeded,
                stdout: String::from("not-json"),
                stderr_bytes: 0,
                stdout_truncated: false,
                stderr_truncated: false,
            },
            TrajectoryExecutionOutcome {
                status: TrajectoryExecutionStatus::Succeeded,
                stdout: String::from(
                    "{\"version\":1,\"results\":[{\"ok\":true,\"result\":{\"records\":[],\"diagnostics\":[],\"unknown\":true},\"error\":null}]}",
                ),
                stderr_bytes: 0,
                stdout_truncated: false,
                stderr_truncated: false,
            },
        ] {
            let policy = policy();
            let mut executor = FakeExecutor {
                outcome,
                calls: 0,
                last_stdin: String::new(),
            };
            let mut normalizer = InjectedTrajectoryNormalizer::new(&policy, &mut executor);
            assert!(normalizer.normalize(&request()).is_err());
            assert_eq!(executor.calls, 1);
        }
    }

    struct FakeExecutor {
        outcome: TrajectoryExecutionOutcome,
        calls: u32,
        last_stdin: String,
    }

    impl FakeExecutor {
        fn success(stdout: String) -> Self {
            Self {
                outcome: TrajectoryExecutionOutcome {
                    status: TrajectoryExecutionStatus::Succeeded,
                    stdout,
                    stderr_bytes: 0,
                    stdout_truncated: false,
                    stderr_truncated: false,
                },
                calls: 0,
                last_stdin: String::new(),
            }
        }
    }

    impl TrajectoryExecutionService for FakeExecutor {
        fn execute(
            &mut self,
            request: &TrajectoryExecutionRequest,
        ) -> Result<TrajectoryExecutionOutcome, TrajectoryServiceIssue> {
            self.calls = self.calls.saturating_add(1);
            self.last_stdin.clone_from(&request.stdin);
            Ok(self.outcome.clone())
        }
    }

    fn policy() -> TrajectoryPolicy {
        TrajectoryPolicy {
            helper_identity: String::from(TRAJECTORY_HELPER_BLAKE3),
            schema_identity: String::from(TRAJECTORY_SCHEMA_BLAKE3),
            sources: vec![TrajectorySource::Pi],
            bounds: TrajectoryBounds {
                max_transcript_bytes: FIXTURE_MAX_TRANSCRIPT_BYTES,
                max_output_bytes: FIXTURE_MAX_OUTPUT_BYTES,
                max_records: FIXTURE_MAX_RECORDS,
                max_content_bytes: FIXTURE_MAX_CONTENT_BYTES,
                max_tool_calls: FIXTURE_MAX_TOOL_CALLS,
                max_tool_argument_bytes: FIXTURE_MAX_TOOL_ARGUMENT_BYTES,
                max_tool_result_bytes: FIXTURE_MAX_TOOL_RESULT_BYTES,
                max_diagnostics: FIXTURE_MAX_DIAGNOSTICS,
            },
            diagnostics: vec![TrajectoryDiagnosticPolicy {
                code: TrajectoryDiagnosticCode::TimestampsSynthesized,
                disposition: DiagnosticDisposition::AcceptedCleanup,
            }],
            reasoning: TrajectoryContentDisposition::Omit,
            tool_arguments: TrajectoryContentDisposition::Omit,
            tool_results: TrajectoryContentDisposition::Omit,
            path_metadata: TrajectoryContentDisposition::Omit,
        }
    }

    fn request() -> TrajectoryNormalizeRequest {
        TrajectoryNormalizeRequest {
            source: TrajectorySource::Pi,
            transcript: String::from("synthetic"),
            source_generation_identity: String::from("generation:one"),
            policy_identity: String::from(POLICY_IDENTITY),
            helper_identity: String::from(TRAJECTORY_HELPER_BLAKE3),
        }
    }

    fn valid_response() -> String {
        String::from(
            "{\"version\":1,\"results\":[{\"ok\":true,\"result\":{\"records\":[{\"role\":\"meta\",\"source\":\"pi\"},{\"role\":\"user\",\"content\":\"hi\",\"timestamp\":\"2026-08-09T10:00:00Z\"},{\"role\":\"assistant\",\"content\":\"ok\",\"timestamp\":\"2026-08-09T10:00:01Z\"}],\"diagnostics\":[]},\"error\":null}]}",
        )
    }
}
