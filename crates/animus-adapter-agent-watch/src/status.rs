//! Watcher-only semantic status and sanitized retry progress.

use alloc::string::String;
use alloc::vec::Vec;

use crate::graph::{AgentLabel, WatchEdge, WatchGraph};

/// Self-reported semantic agent status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatchStatus {
    /// No turn is active.
    Idle,
    /// A turn is active.
    Working,
    /// The agent waits on a concrete blocker.
    Blocked,
    /// The agent waits for human approval or confirmation.
    WaitingApproval,
    /// The last visible turn completed.
    Completed,
}

impl WatchStatus {
    /// Returns the stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Working => "working",
            Self::Blocked => "blocked",
            Self::WaitingApproval => "waiting_approval",
            Self::Completed => "completed",
        }
    }
}

/// Bounded provider-retry error class shown to watchers.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetryErrorClass {
    /// Provider rate-limited the request.
    RateLimited,
    /// Provider returned a transient failure.
    Transient,
    /// Provider service was unavailable.
    Unavailable,
    /// Provider returned an unknown error class.
    Unknown,
}

impl RetryErrorClass {
    /// Returns the stable error-class label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RateLimited => "rate_limited",
            Self::Transient => "transient",
            Self::Unavailable => "unavailable",
            Self::Unknown => "unknown",
        }
    }
}

/// Sanitized retry progress without raw provider payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SanitizedRetryProgress {
    /// One-based attempt count.
    pub attempt: u32,
    /// Remaining allowed attempts.
    pub remaining: u32,
    /// Bounded error class of the last failure.
    pub last_error_class: Option<RetryErrorClass>,
}

impl SanitizedRetryProgress {
    /// Returns whether the progress record is well-formed.
    #[must_use]
    pub const fn is_well_formed(self) -> bool {
        self.attempt >= 1 && self.remaining >= 1
    }
}

/// Host-supplied semantic status input for one session agent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentStatusInput {
    /// Session agent identity.
    pub agent_id: AgentLabel,
    /// Self-reported semantic status.
    pub status: WatchStatus,
    /// Sanitized retry progress when the agent is retrying.
    pub retry: Option<SanitizedRetryProgress>,
    /// Monotonic status generation.
    pub generation: u64,
}

/// Watcher-visible redacted status projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatcherStatusView {
    /// Watched agent identity.
    pub agent_id: AgentLabel,
    /// Semantic status only. Hidden prompts and raw provider data never appear.
    pub status: WatchStatus,
    /// Sanitized retry progress.
    pub retry: Option<SanitizedRetryProgress>,
    /// Status generation.
    pub generation: u64,
}

/// Projects one watcher-only status view per directed watch edge.
///
/// Each watcher sees only the agents it watches. An agent that is watched but
/// has no status input is a fail-closed error, as is an unknown status input.
///
/// # Errors
///
/// Returns [`crate::graph::WatchError::InvalidLabel`] for malformed labels.
pub fn project_watcher_status(
    graph: &WatchGraph,
    inputs: &[AgentStatusInput],
) -> Result<Vec<WatcherStatusView>, crate::graph::WatchError> {
    for input in inputs {
        if let Some(progress) = input.retry {
            if !progress.is_well_formed() {
                return Err(crate::graph::WatchError::InvalidLabel);
            }
        }
    }
    let mut views = Vec::with_capacity(graph.edges().len());
    for WatchEdge { target, .. } in graph.edges() {
        let input = inputs
            .iter()
            .find(|candidate| &candidate.agent_id == target)
            .ok_or(crate::graph::WatchError::InvalidLabel)?;
        views.push(WatcherStatusView {
            agent_id: input.agent_id.clone(),
            status: input.status,
            retry: input.retry,
            generation: input.generation,
        });
    }
    views.sort_by(|left, right| left.agent_id.cmp(&right.agent_id));
    views.dedup_by(|left, right| left.agent_id == right.agent_id);
    Ok(views)
}

/// Returns whether a status record violates the monotonic generation rule.
#[must_use]
pub fn generation_is_monotonic(prior: u64, next: u64) -> bool {
    next >= prior
}

/// Bounded watcher-visible status string without raw content.
///
/// Returns `"<agent>:<status>"` with optional retry counters.
#[must_use]
pub fn status_view_line(view: &WatcherStatusView) -> String {
    let mut line = alloc::format!(
        "{}:{}:{}",
        view.agent_id.as_str(),
        view.status.as_str(),
        view.generation
    );
    if let Some(progress) = view.retry {
        line = alloc::format!(
            "{}:{}:{}",
            line,
            progress.attempt,
            progress
                .last_error_class
                .map_or(retry_error_label_none(), RetryErrorClass::as_str)
        );
    }
    line
}

const fn retry_error_label_none() -> &'static str {
    "none"
}
