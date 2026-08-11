use std::ffi::OsString;
use std::path::PathBuf;

use crate::McpLimits;

/// Owned process termination scope.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProcessScope {
    /// Unix process group; descendants receive the teardown signal.
    ProcessGroup,
    /// Direct child only; no descendant teardown claim.
    DirectChild,
}

/// Terminal process teardown observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TeardownObservation {
    /// No process was started.
    NotStarted,
    /// A live generation was retained because no teardown was required.
    Retained,
    /// Child exited and owned cleanup completed.
    Clean,
    /// Forced owned termination completed.
    Forced,
    /// Terminal owned-process evidence is absent.
    Uncertain,
}

impl TeardownObservation {
    pub(crate) const fn terminal(self) -> bool {
        matches!(self, Self::NotStarted | Self::Clean | Self::Forced)
    }
}

/// Exact process launch facts supplied by the caller-owned authority shell.
pub(crate) struct ProcessLaunch {
    pub(crate) executable: PathBuf,
    pub(crate) arguments: Vec<OsString>,
    pub(crate) environment: Vec<(OsString, OsString)>,
    pub(crate) current_directory: PathBuf,
    pub(crate) limits: McpLimits,
}

/// Bounded process or standard-input transport failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum ProcessError {
    Launch,
    StartCleanup,
    StartCleanupUncertain,
    MissingPipe,
    ReaderLaunch,
    Write,
    Flush,
    Read,
    MalformedFrame,
    OversizedFrame,
    Timeout,
    Cancelled,
    EarlyExit,
    Kill,
    Reap,
}

/// Stderr identity and bounded size metadata without stderr text.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct StderrObservation {
    pub(crate) identity: String,
    pub(crate) byte_count: usize,
    pub(crate) truncated: bool,
}
