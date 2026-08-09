//! Optional host shell for the reviewed Pijul CLI cohort.
//!
//! The shell plans exact argv and delegates execution to a caller-injected
//! bounded executor. It does not discover executables or repositories, search
//! `PATH`, inherit environment values, open `.pijul` databases itself, access
//! networks, read identity keys, persist observations, or dispatch mutations.

use std::collections::BTreeMap;

use animus_adapters::pijul::{
    PIJUL_CLI_RELEASE, PIJUL_COHORT_IDENTITY, PijulBackendResponse, PijulBackendService,
    PijulBoundedExecutor, PijulExecutable, PijulExecutionBounds, PijulExecutionRequest,
    PijulExecutionStatus, PijulLoadedChannel, PijulLoadedDiffEntry, PijulLoadedLogEntry,
    PijulLoadedOutput, PijulOperation, PijulRequest, PijulServiceIssue, PijulServiceIssueKind,
};
use animus_runtime::pijul::{
    PijulProfile, PijulValidationIssue, validate_pijul_request, validate_pijul_response,
};
use serde::Deserialize;

const ZERO_EXIT_CODE: i32 = 0;
const CHANNEL_CURRENT_PREFIX: &str = "* ";
const CHANNEL_OTHER_PREFIX: &str = "  ";
const EXECUTABLE_ABSOLUTE_FIELD: &str = "executable-path";
const EXECUTABLE_IDENTITY_FIELD: &str = "executable-identity";
const EXECUTABLE_RELEASE_FIELD: &str = "executable-release";
const QUERY_SERVICE_FIELD: &str = "query-service";
const PROTOCOL_FIELD: &str = "machine-output";
const EXECUTION_FIELD: &str = "bounded-execution";

/// Injected Pijul CLI query service bound to one exact profile and executable.
#[derive(Debug)]
pub struct PijulCliService<E> {
    executor: E,
    profile: PijulProfile,
    executable: PijulExecutable,
}

impl<E> PijulCliService<E> {
    /// Creates an exact-cohort service without executing a child.
    ///
    /// # Errors
    ///
    /// Returns a safe issue when the executable path is not absolute or its
    /// identity or release does not match the reviewed cohort.
    pub fn new(
        executor: E,
        profile: PijulProfile,
        executable: PijulExecutable,
    ) -> Result<Self, PijulServiceIssue> {
        validate_executable(&executable)?;
        Ok(Self {
            executor,
            profile,
            executable,
        })
    }

    /// Returns the pure validated profile.
    #[must_use]
    pub const fn profile(&self) -> &PijulProfile {
        &self.profile
    }

    /// Returns the exact executable binding.
    #[must_use]
    pub const fn executable(&self) -> &PijulExecutable {
        &self.executable
    }

    /// Consumes the service and returns the injected executor.
    #[must_use]
    pub fn into_executor(self) -> E {
        self.executor
    }
}

impl<E: PijulBoundedExecutor> PijulBackendService for PijulCliService<E> {
    fn execute(
        &mut self,
        request: &PijulRequest,
    ) -> Result<PijulBackendResponse, PijulServiceIssue> {
        validate_pijul_request(&self.profile, request)
            .map_err(|error| map_validation_issue(&error))?;
        let plan = plan_pijul_invocation(&self.profile, &self.executable, request)?;
        let execution = self.executor.execute(&plan)?;
        validate_execution_for_parse(&execution)?;
        let loaded = parse_machine_output(request.operation, &execution.standard_output)?;
        let response = PijulBackendResponse {
            operation: request.operation,
            execution,
            loaded: Some(loaded),
        };
        validate_pijul_response(&self.profile, request, &response)
            .map_err(|error| map_validation_issue(&error))?;
        Ok(response)
    }
}

/// Purely plans exact argv and finite bounds for one admitted query.
///
/// # Errors
///
/// Returns a safe issue for any unsupported or protected operation.
pub fn plan_pijul_invocation(
    profile: &PijulProfile,
    executable: &PijulExecutable,
    request: &PijulRequest,
) -> Result<PijulExecutionRequest, PijulServiceIssue> {
    validate_executable(executable)?;
    validate_pijul_request(profile, request).map_err(|error| map_validation_issue(&error))?;
    let mut arguments = Vec::new();
    match request.operation {
        PijulOperation::Log => {
            arguments.push("log".to_owned());
            push_repository_and_channel(&mut arguments, request);
            arguments.push("--hash-only".to_owned());
            arguments.push("--output-format".to_owned());
            arguments.push("json".to_owned());
            arguments.push("--limit".to_owned());
            arguments.push(profile.bounds().max_log_entries.to_string());
        }
        PijulOperation::Diff => {
            arguments.push("diff".to_owned());
            push_repository_and_channel(&mut arguments, request);
            arguments.push("--json".to_owned());
        }
        PijulOperation::ChannelList => {
            arguments.push("channel".to_owned());
            push_repository(&mut arguments, request);
        }
        PijulOperation::Change
        | PijulOperation::Record
        | PijulOperation::Unrecord
        | PijulOperation::Apply
        | PijulOperation::Fork
        | PijulOperation::Reset
        | PijulOperation::ChannelMutation => {
            return Err(PijulServiceIssue::new(
                PijulServiceIssueKind::UnsupportedOperation,
                QUERY_SERVICE_FIELD,
            ));
        }
    }
    Ok(PijulExecutionRequest {
        executable: executable.clone(),
        arguments,
        working_directory: request.repository_path.clone(),
        standard_input: Vec::new(),
        environment: Vec::new(),
        inherit_environment: false,
        bounds: PijulExecutionBounds {
            max_stdin_bytes: profile.bounds().max_stdin_bytes,
            max_stdout_bytes: profile.bounds().max_stdout_bytes,
            max_stderr_bytes: profile.bounds().max_stderr_bytes,
            max_memory_bytes: profile.bounds().max_memory_bytes,
            max_cpu_milliseconds: profile.bounds().max_cpu_milliseconds,
            deadline_milliseconds: profile.bounds().deadline_milliseconds,
        },
    })
}

fn push_repository_and_channel(arguments: &mut Vec<String>, request: &PijulRequest) {
    push_repository(arguments, request);
    if let Some(channel) = &request.channel {
        arguments.push("--channel".to_owned());
        arguments.push(channel.clone());
    }
}

fn push_repository(arguments: &mut Vec<String>, request: &PijulRequest) {
    arguments.push("--repository".to_owned());
    arguments.push(request.repository_path.clone());
}

fn validate_executable(executable: &PijulExecutable) -> Result<(), PijulServiceIssue> {
    if !is_absolute_path(&executable.path) {
        return Err(PijulServiceIssue::new(
            PijulServiceIssueKind::InvalidRequest,
            EXECUTABLE_ABSOLUTE_FIELD,
        ));
    }
    if executable.identity != PIJUL_COHORT_IDENTITY {
        return Err(PijulServiceIssue::new(
            PijulServiceIssueKind::CohortMismatch,
            EXECUTABLE_IDENTITY_FIELD,
        ));
    }
    if executable.release != PIJUL_CLI_RELEASE {
        return Err(PijulServiceIssue::new(
            PijulServiceIssueKind::CohortMismatch,
            EXECUTABLE_RELEASE_FIELD,
        ));
    }
    Ok(())
}

fn validate_execution_for_parse(
    execution: &animus_adapters::pijul::PijulExecutionOutcome,
) -> Result<(), PijulServiceIssue> {
    if execution.stdout_truncated || execution.stderr_truncated {
        return Err(PijulServiceIssue::new(
            PijulServiceIssueKind::BoundExceeded,
            EXECUTION_FIELD,
        ));
    }
    if execution.status != PijulExecutionStatus::Exited
        || execution.exit_code != Some(ZERO_EXIT_CODE)
    {
        return Err(PijulServiceIssue::new(
            PijulServiceIssueKind::ExecutorFailure,
            EXECUTION_FIELD,
        ));
    }
    Ok(())
}

fn parse_machine_output(
    operation: PijulOperation,
    bytes: &[u8],
) -> Result<PijulLoadedOutput, PijulServiceIssue> {
    match operation {
        PijulOperation::Log => parse_hash_log(bytes).map(PijulLoadedOutput::Log),
        PijulOperation::Diff => parse_diff(bytes).map(PijulLoadedOutput::Diff),
        PijulOperation::ChannelList => parse_channels(bytes).map(PijulLoadedOutput::Channels),
        PijulOperation::Change
        | PijulOperation::Record
        | PijulOperation::Unrecord
        | PijulOperation::Apply
        | PijulOperation::Fork
        | PijulOperation::Reset
        | PijulOperation::ChannelMutation => Err(PijulServiceIssue::new(
            PijulServiceIssueKind::UnsupportedOperation,
            PROTOCOL_FIELD,
        )),
    }
}

fn parse_hash_log(bytes: &[u8]) -> Result<Vec<PijulLoadedLogEntry>, PijulServiceIssue> {
    let hashes: Vec<String> = serde_json::from_slice(bytes).map_err(|_error| protocol_issue())?;
    Ok(hashes
        .into_iter()
        .map(|hash| PijulLoadedLogEntry {
            hash,
            state: None,
            authors: Vec::new(),
            timestamp: None,
            message: None,
            description: None,
            paths: Vec::new(),
        })
        .collect())
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct DiffStatus {
    operation: String,
    line: Option<u64>,
}

fn parse_diff(bytes: &[u8]) -> Result<Vec<PijulLoadedDiffEntry>, PijulServiceIssue> {
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let by_path: BTreeMap<String, Vec<DiffStatus>> =
        serde_json::from_slice(bytes).map_err(|_error| protocol_issue())?;
    let record_count = by_path.values().map(Vec::len).sum();
    let mut entries = Vec::with_capacity(record_count);
    for (path, statuses) in by_path {
        for status in statuses {
            entries.push(PijulLoadedDiffEntry {
                path: path.clone(),
                operation: status.operation,
                line: status.line,
            });
        }
    }
    Ok(entries)
}

fn parse_channels(bytes: &[u8]) -> Result<Vec<PijulLoadedChannel>, PijulServiceIssue> {
    let text = std::str::from_utf8(bytes).map_err(|_error| protocol_issue())?;
    let mut channels = Vec::new();
    for line in text.lines() {
        if let Some(name) = line.strip_prefix(CHANNEL_CURRENT_PREFIX) {
            channels.push(parse_channel_line(name, true)?);
        } else if let Some(name) = line.strip_prefix(CHANNEL_OTHER_PREFIX) {
            channels.push(parse_channel_line(name, false)?);
        } else if !line.is_empty() {
            return Err(protocol_issue());
        }
    }
    Ok(channels)
}

fn parse_channel_line(
    name: &str,
    is_current: bool,
) -> Result<PijulLoadedChannel, PijulServiceIssue> {
    if name.is_empty() || name.trim() != name {
        return Err(protocol_issue());
    }
    Ok(PijulLoadedChannel {
        name: name.to_owned(),
        is_current,
    })
}

fn map_validation_issue(error: &animus_runtime::pijul::PijulValidationError) -> PijulServiceIssue {
    let kind = match error.issue() {
        PijulValidationIssue::CohortMismatch => PijulServiceIssueKind::CohortMismatch,
        PijulValidationIssue::UnsupportedOperation
        | PijulValidationIssue::ProtectedMutationDenied => {
            PijulServiceIssueKind::UnsupportedOperation
        }
        PijulValidationIssue::BoundExceeded => PijulServiceIssueKind::BoundExceeded,
        PijulValidationIssue::ProtocolMismatch
        | PijulValidationIssue::InvalidIdentity
        | PijulValidationIssue::InvalidChannel
        | PijulValidationIssue::InvalidStatus
        | PijulValidationIssue::InteractiveOutput
        | PijulValidationIssue::UnsafeEvidence
        | PijulValidationIssue::AmbiguousPrefix => PijulServiceIssueKind::ProtocolFailure,
        PijulValidationIssue::InvalidExitStatus => PijulServiceIssueKind::ExecutorFailure,
        PijulValidationIssue::MalformedProfile
        | PijulValidationIssue::InvalidBound
        | PijulValidationIssue::InvalidRequest
        | PijulValidationIssue::ProjectionRejected => PijulServiceIssueKind::InvalidRequest,
    };
    PijulServiceIssue::new(kind, error.field())
}

fn protocol_issue() -> PijulServiceIssue {
    PijulServiceIssue::new(PijulServiceIssueKind::ProtocolFailure, PROTOCOL_FIELD)
}

fn is_absolute_path(value: &str) -> bool {
    std::path::Path::new(value).is_absolute()
}

#[cfg(test)]
mod tests;
