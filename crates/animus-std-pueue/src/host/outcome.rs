use animus_adapters::{QueueServiceRequest, QueueServiceResponse, ServiceOutcome};

use super::evidence;
use crate::PueueProjectionError;
use crate::command::PueueCommandError;
use crate::process::{PueueProcessDisposition, PueueProcessOutput, PueueProcessRequest};

pub(super) type HostResult = Result<QueueServiceResponse, HostFailure>;

pub(super) enum HostPlan {
    Enqueue(PueueProcessRequest),
    Control(PueueProcessRequest),
    ObserveTask {
        wait_until_terminal: bool,
        status: PueueProcessRequest,
    },
    Log(PueueProcessRequest),
    Groups(PueueProcessRequest),
}

pub(super) enum ProcessSuccess {
    Completed(Vec<u8>),
    ExitFailed,
}

#[derive(Clone, Copy)]
pub(super) enum HostFailure {
    Denied,
    Unavailable,
    Cancelled,
    TimedOut,
    Malformed,
}

pub(super) const fn task_is_terminal(status: animus_adapters::QueueTaskStatus) -> bool {
    matches!(
        status,
        animus_adapters::QueueTaskStatus::Completed
            | animus_adapters::QueueTaskStatus::Failed
            | animus_adapters::QueueTaskStatus::Cancelled
    )
}

pub(super) fn classify_output(output: PueueProcessOutput) -> Result<ProcessSuccess, HostFailure> {
    match output.disposition {
        PueueProcessDisposition::Succeeded => Ok(ProcessSuccess::Completed(output.stdout)),
        PueueProcessDisposition::ExitFailed => Ok(ProcessSuccess::ExitFailed),
        PueueProcessDisposition::TimedOut => Err(HostFailure::TimedOut),
        PueueProcessDisposition::Cancelled => Err(HostFailure::Cancelled),
        PueueProcessDisposition::OutputLimitExceeded => Err(HostFailure::Malformed),
    }
}

pub(super) const fn map_command_error(error: PueueCommandError) -> HostFailure {
    match error {
        PueueCommandError::Denied | PueueCommandError::MissingProfileField => HostFailure::Denied,
        PueueCommandError::BoundConversion | PueueCommandError::NonUtf8Path => {
            HostFailure::Unavailable
        }
    }
}

pub(super) const fn map_projection_error(_error: PueueProjectionError) -> HostFailure {
    HostFailure::Malformed
}

pub(super) fn map_host_result(
    request: &QueueServiceRequest,
    result: HostResult,
) -> ServiceOutcome<QueueServiceResponse> {
    match result {
        Ok(response) => evidence::completed(request, response),
        Err(HostFailure::Denied) => evidence::denied(request),
        Err(HostFailure::Unavailable) => evidence::unavailable(request),
        Err(HostFailure::Cancelled) => evidence::cancelled(request),
        Err(HostFailure::TimedOut) => evidence::timed_out(request),
        Err(HostFailure::Malformed) => evidence::malformed(request),
    }
}
