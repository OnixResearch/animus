//! Fail-closed watch admission checks.

use alloc::vec::Vec;

use crate::graph::{AgentLabel, WatchDenial, WatchGraph};

/// Bounded watch-validation rejection class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatchValidationIssue {
    /// The known-agent set is empty.
    EmptyKnownAgentSet,
    /// A graph edge or status record names an unknown agent.
    UnknownAgent,
    /// A graph contains duplicate directed edges.
    DuplicateEdge,
    /// An edge or status generation regresses.
    NonMonotonicGeneration,
    /// A status record is malformed.
    MalformedStatus,
    /// A transition denial was recorded.
    Rejected(WatchDenial),
}

impl WatchValidationIssue {
    /// Returns the stable issue label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EmptyKnownAgentSet => "empty_known_agent_set",
            Self::UnknownAgent => "unknown_agent",
            Self::DuplicateEdge => "duplicate_edge",
            Self::NonMonotonicGeneration => "non_monotonic_generation",
            Self::MalformedStatus => "malformed_status",
            Self::Rejected(denial) => denial.as_str(),
        }
    }
}

/// Validates the known agent set before any watch transition.
///
/// # Errors
///
/// Returns [`WatchValidationIssue::EmptyKnownAgentSet`] for an empty set and
/// [`WatchValidationIssue::UnknownAgent`] for a duplicate label.
pub fn validate_known_agents(known_agents: &[AgentLabel]) -> Result<(), WatchValidationIssue> {
    if known_agents.is_empty() {
        return Err(WatchValidationIssue::EmptyKnownAgentSet);
    }
    for (index, agent) in known_agents.iter().enumerate() {
        if known_agents[..index].iter().any(|prior| prior == agent) {
            return Err(WatchValidationIssue::UnknownAgent);
        }
    }
    Ok(())
}

/// Validates a watch graph against explicit admissions before effects.
///
/// # Errors
///
/// Returns [`WatchValidationIssue::EmptyKnownAgentSet`],
/// [`WatchValidationIssue::UnknownAgent`], or
/// [`WatchValidationIssue::DuplicateEdge`] for a rejected graph.
pub fn validate_watch_graph(
    graph: &WatchGraph,
    known_agents: &[AgentLabel],
) -> Result<(), WatchValidationIssue> {
    validate_known_agents(known_agents)?;
    for edge in graph.edges() {
        if !known_agents.contains(&edge.watcher) || !known_agents.contains(&edge.target) {
            return Err(WatchValidationIssue::UnknownAgent);
        }
    }
    for (index, edge) in graph.edges().iter().enumerate() {
        if graph.edges()[..index]
            .iter()
            .any(|prior| prior.watcher == edge.watcher && prior.target == edge.target)
        {
            return Err(WatchValidationIssue::DuplicateEdge);
        }
    }
    Ok(())
}

/// Validates a status record generation against the watch graph generation.
///
/// # Errors
///
/// Returns [`WatchValidationIssue::NonMonotonicGeneration`] for a regressed
/// generation.
pub fn validate_status_generation(
    graph_generation: u64,
    status_generation: u64,
) -> Result<(), WatchValidationIssue> {
    if status_generation < graph_generation {
        return Err(WatchValidationIssue::NonMonotonicGeneration);
    }
    Ok(())
}

/// Projects a denial into a validation issue.
#[must_use]
pub const fn denial_issue(denial: WatchDenial) -> WatchValidationIssue {
    WatchValidationIssue::Rejected(denial)
}

/// Returns whether a watch graph is acyclic (no directed cycle).
#[must_use]
pub fn watch_graph_is_acyclic(graph: &WatchGraph) -> bool {
    let agents = collect_agents(graph);
    agents.iter().all(|agent| !reaches_self(graph, agent))
}

fn collect_agents(graph: &WatchGraph) -> Vec<AgentLabel> {
    let mut agents = Vec::new();
    for edge in graph.edges() {
        if !agents.contains(&edge.watcher) {
            agents.push(edge.watcher.clone());
        }
        if !agents.contains(&edge.target) {
            agents.push(edge.target.clone());
        }
    }
    agents
}

fn reaches_self(graph: &WatchGraph, agent: &AgentLabel) -> bool {
    crate::graph::target_can_reach(graph, agent, agent)
}
