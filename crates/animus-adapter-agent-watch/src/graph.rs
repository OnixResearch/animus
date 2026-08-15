//! Session-local typed watch-graph state and deterministic transitions.

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::{MAX_WATCH_LABEL_BYTES, hash_parts};

/// One safe agent label within a session-local watch scope.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct AgentLabel(String);

impl AgentLabel {
    /// Validates a bounded safe agent label.
    ///
    /// # Errors
    ///
    /// Returns [`WatchError::InvalidLabel`] for empty, oversized, or
    /// control-character labels.
    pub fn parse(value: &str) -> Result<Self, WatchError> {
        if value.is_empty()
            || value.len() > MAX_WATCH_LABEL_BYTES
            || value.chars().any(char::is_control)
        {
            return Err(WatchError::InvalidLabel);
        }
        Ok(Self(value.to_string()))
    }

    /// Returns the bounded label text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// One typed directed watch edge from a watcher to a watched target.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatchEdge {
    /// Watcher agent.
    pub watcher: AgentLabel,
    /// Watched target agent.
    pub target: AgentLabel,
    /// Monotonic watch-process generation that created the edge.
    pub generation: u64,
}

/// Immutable session-local watch graph.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WatchGraph {
    edges: Vec<WatchEdge>,
}

impl WatchGraph {
    /// Returns the directed watch edges.
    #[must_use]
    pub fn edges(&self) -> &[WatchEdge] {
        self.edges.as_slice()
    }

    /// Returns whether the graph has no edges.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.edges.is_empty()
    }

    /// Returns a deterministic identity over the sorted edge set.
    #[must_use]
    pub fn identity(&self) -> String {
        let mut watchers = self
            .edges
            .iter()
            .map(|edge| {
                alloc::format!(
                    "{}->{}:{}",
                    edge.watcher.as_str(),
                    edge.target.as_str(),
                    edge.generation
                )
            })
            .collect::<Vec<_>>();
        watchers.sort();
        let parts = watchers.iter().map(String::as_str).collect::<Vec<_>>();
        hash_parts("animus/agent-watch-graph", &parts)
    }
}

/// A watch transition requested on a session-local graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WatchAction {
    /// Enable one watcher-to-target edge.
    Enable {
        /// Watcher agent.
        watcher: AgentLabel,
        /// Watched target agent.
        target: AgentLabel,
        /// Monotonic process generation.
        generation: u64,
    },
    /// Disable the edge from a watcher to a target.
    Disable {
        /// Watcher agent.
        watcher: AgentLabel,
        /// Watched target agent.
        target: AgentLabel,
        /// Monotonic process generation.
        generation: u64,
    },
}

/// Bounded watch-transition denial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatchDenial {
    /// An edge names an agent outside the known session set.
    UnknownAgent,
    /// The enable would create a duplicate existing edge.
    DuplicateEdge,
    /// The disable names an edge that is not present.
    MissingEdge,
    /// The transition generation is older than the recorded edge generation.
    StaleGeneration,
    /// The enable would close a directed cycle.
    CycleWouldForm,
}

impl WatchDenial {
    /// Returns the stable denial label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::UnknownAgent => "unknown_agent",
            Self::DuplicateEdge => "duplicate_edge",
            Self::MissingEdge => "missing_edge",
            Self::StaleGeneration => "stale_generation",
            Self::CycleWouldForm => "cycle_would_form",
        }
    }
}

/// Watch transition outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WatchResult {
    /// The transition produced a new deterministic graph state.
    Applied(Box<WatchGraph>),
    /// The transition was rejected without changing watch state.
    Rejected(WatchDenial),
}

/// Applies one watch transition to an owned graph state.
///
/// Every rejection leaves the input state unchanged. Cycle-closing enables are
/// rejected atomically. The action does not touch processes, sockets,
/// persistence, or providers.
#[must_use]
pub fn apply_watch_action(
    state: &WatchGraph,
    action: &WatchAction,
    known_agents: &[AgentLabel],
) -> WatchResult {
    match action {
        WatchAction::Enable {
            watcher,
            target,
            generation,
        } => {
            if !known_agents.contains(watcher) || !known_agents.contains(target) {
                return WatchResult::Rejected(WatchDenial::UnknownAgent);
            }
            apply_enable(state, watcher, target, *generation)
        }
        WatchAction::Disable {
            watcher,
            target,
            generation,
        } => {
            if !known_agents.contains(watcher) || !known_agents.contains(target) {
                return WatchResult::Rejected(WatchDenial::UnknownAgent);
            }
            apply_disable(state, watcher, target, *generation)
        }
    }
}

fn apply_enable(
    state: &WatchGraph,
    watcher: &AgentLabel,
    target: &AgentLabel,
    generation: u64,
) -> WatchResult {
    if let Some(existing) = state
        .edges
        .iter()
        .find(|edge| &edge.watcher == watcher && &edge.target == target)
    {
        if generation < existing.generation {
            return WatchResult::Rejected(WatchDenial::StaleGeneration);
        }
        return WatchResult::Rejected(WatchDenial::DuplicateEdge);
    }
    if target_can_reach(state, target, watcher) {
        return WatchResult::Rejected(WatchDenial::CycleWouldForm);
    }
    let mut edges = state.edges.clone();
    edges.push(WatchEdge {
        watcher: watcher.clone(),
        target: target.clone(),
        generation,
    });
    WatchResult::Applied(Box::new(WatchGraph { edges }))
}

fn apply_disable(
    state: &WatchGraph,
    watcher: &AgentLabel,
    target: &AgentLabel,
    generation: u64,
) -> WatchResult {
    let matches = state
        .edges
        .iter()
        .filter(|edge| &edge.watcher == watcher && &edge.target == target)
        .collect::<Vec<_>>();
    let Some(existing) = matches.first().copied() else {
        return WatchResult::Rejected(WatchDenial::MissingEdge);
    };
    if existing.generation > generation {
        return WatchResult::Rejected(WatchDenial::StaleGeneration);
    }
    let edges = state
        .edges
        .iter()
        .filter(|edge| !(edge.watcher == *watcher && edge.target == *target))
        .cloned()
        .collect::<Vec<_>>();
    WatchResult::Applied(Box::new(WatchGraph { edges }))
}

/// Returns whether `from` can reach `to` through one or more directed watch
/// edges. A node does not count as reaching itself without a real path.
#[must_use]
pub fn target_can_reach(state: &WatchGraph, from: &AgentLabel, to: &AgentLabel) -> bool {
    let mut pending = Vec::new();
    for edge in &state.edges {
        if edge.watcher == *from {
            pending.push(edge.target.clone());
        }
    }
    let mut visited = alloc::collections::BTreeSet::new();
    while let Some(current) = pending.pop() {
        if current == *to {
            return true;
        }
        if !visited.insert(current.clone()) {
            continue;
        }
        for edge in &state.edges {
            if edge.watcher == current {
                pending.push(edge.target.clone());
            }
        }
    }
    false
}

/// Typed watch-graph construction error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WatchError {
    /// The label is empty, oversized, or contains control characters.
    InvalidLabel,
}
