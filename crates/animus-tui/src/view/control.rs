// Agent control and queue dashboard: agent status, start/stop/interrupt controls,
// background-work queue view, subagent/peers panels, and process panels.
//
// All state is projected from accepted DTOs; controls issue typed commands
// to the runtime/host without holding ambient authority.

#[allow(unknown_lints, non_trait_imports)]
use animus_adapters::{
    HostEvent, HostEventKind, HostSelector, HostSemanticStatus,
    HostTopologySnapshot,
};

/// Label for the idle status state.
const IDLE_STATUS_LABEL: &str = "idle";
/// Label for the busy/heavy-load status.
const BUSY_STATUS_LABEL: &str = "busy";
/// Label for streaming status.
const STREAMING_STATUS_LABEL: &str = "streaming";
/// Label for waiting-for-approval status.
const WAITING_STATUS_LABEL: &str = "waiting";
/// Label for error status.
const ERROR_STATUS_LABEL: &str = "error";
/// Label for topology revision display.
const TOPOLOGY_REVISION_LABEL: &str = "topology: rev";
/// Label for pending-topology status.
const TOPOLOGY_PENDING_LABEL: &str = "topology: pending";
/// Label for stale-selector diagnostic.
const STALE_SELECTOR_MSG: &str = "topology-mismatch";

/// Semantic agent status following status-authority precedence.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentStatus {
    /// Agent is idle.
    Idle,
    /// Agent is busy processing.
    Busy,
    /// Agent is streaming output.
    Streaming,
    /// Agent is waiting for user approval.
    Waiting,
    /// Agent encountered an error.
    Error,
}

impl AgentStatus {
    /// Returns the stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => IDLE_STATUS_LABEL,
            Self::Busy => BUSY_STATUS_LABEL,
            Self::Streaming => STREAMING_STATUS_LABEL,
            Self::Waiting => WAITING_STATUS_LABEL,
            Self::Error => ERROR_STATUS_LABEL,
        }
    }
}

/// Typed control command issued to the runtime/host.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ControlCommand {
    /// Start the agent.
    Start,
    /// Stop the current operation.
    Stop,
    /// Interrupt the current operation.
    Interrupt,
    /// Cancel a specific queue task.
    CancelTask(String),
    /// Inspect a specific queue task.
    InspectTask(String),
}

/// Agent control dashboard state.
#[derive(Clone, Debug, PartialEq)]
pub struct AgentDashboardState {
    /// Semantic agent status.
    status: AgentStatus,
    /// Whether the agent can be started.
    can_start: bool,
    /// Whether the agent can be stopped.
    can_stop: bool,
    /// Background work queue items.
    queue_items: Vec<QueueItem>,
    /// Subagent/peers list.
    peers: Vec<String>,
    /// Active processes.
    processes: Vec<String>,
    /// Current topology snapshot for control resolution.
    topology: Option<HostTopologySnapshot>,
    /// Current topology revision to detect stale selectors.
    topology_revision: u64,
    /// Tracked selectors that should be re-resolved on topology changes.
    tracked_selectors: Vec<HostSelector>,
}

impl AgentDashboardState {
    /// Builds an empty agent dashboard state.
    #[must_use]
    pub fn idle() -> Self {
        Self {
            status: AgentStatus::Idle,
            can_start: true,
            can_stop: false,
            queue_items: Vec::new(),
            peers: Vec::new(),
            processes: Vec::new(),
            topology: None,
            topology_revision: 0,
            tracked_selectors: Vec::new(),
        }
    }

    /// Sets the agent status and updates control availability.
    pub fn set_status(&mut self, status: AgentStatus) {
        self.status = status;
        match status {
            AgentStatus::Idle => {
                self.can_start = true;
                self.can_stop = false;
            }
            AgentStatus::Busy | AgentStatus::Streaming | AgentStatus::Waiting => {
                self.can_start = false;
                self.can_stop = true;
            }
            AgentStatus::Error => {
                self.can_start = true;
                self.can_stop = true;
            }
        }
    }

    /// Sets the queue items.
    pub fn set_queue_items(&mut self, items: Vec<QueueItem>) {
        self.queue_items = items;
    }

    /// Sets the peer list.
    pub fn set_peers(&mut self, peers: Vec<String>) {
        self.peers = peers;
    }

    /// Sets the process list.
    pub fn set_processes(&mut self, processes: Vec<String>) {
        self.processes = processes;
    }

    /// Returns the current agent status.
    #[must_use]
    pub const fn status(&self) -> AgentStatus {
        self.status
    }

    /// Returns whether start is available.
    #[must_use]
    pub const fn can_start(&self) -> bool {
        self.can_start
    }

    /// Returns whether stop is available.
    #[must_use]
    pub const fn can_stop(&self) -> bool {
        self.can_stop
    }

    /// Returns the queue items.
    #[must_use]
    pub fn queue_items(&self) -> &[QueueItem] {
        self.queue_items.as_slice()
    }

    /// Returns the peer list.
    #[must_use]
    pub fn peers(&self) -> &[String] {
        self.peers.as_slice()
    }

    /// Returns the process list.
    #[must_use]
    pub fn processes(&self) -> &[String] {
        self.processes.as_slice()
    }

    // -------- Topology-aware control resolution --------

    /// Returns the current topology revision.
    #[must_use]
    pub const fn topology_revision(&self) -> u64 {
        self.topology_revision
    }

    /// Returns the current topology snapshot, if any.
    #[must_use]
    pub const fn topology(&self) -> Option<&HostTopologySnapshot> {
        self.topology.as_ref()
    }

    /// Returns tracked selectors bound to the current topology.
    #[must_use]
    pub fn tracked_selectors(&self) -> &[HostSelector] {
        self.tracked_selectors.as_slice()
    }

    /// Applies a topology snapshot, re-resolving stale selectors.
    ///
    /// Selectors bound to an older revision are replaced with fresh selectors
    /// from the new snapshot when the resource identity still exists.
    /// If a tracked selector's resource has been removed, it remains tracked
    /// but `check_selector_freshness` reports a mismatch on the next control.
    pub fn apply_topology_snapshot(&mut self, snapshot: HostTopologySnapshot) {
        let new_revision = snapshot.revision();
        if new_revision <= self.topology_revision && self.topology.is_some() {
            return;
        }
        let mut resolved: Vec<HostSelector> = Vec::with_capacity(self.tracked_selectors.len());
        for selector in self.tracked_selectors.drain(..) {
            if selector.revision() < new_revision {
                if let Some(fresh) = find_selector_in_snapshot(&snapshot, &selector) {
                    resolved.push(fresh);
                } else {
                    resolved.push(selector);
                }
            } else {
                resolved.push(selector);
            }
        }
        self.tracked_selectors = resolved;
        self.topology = Some(snapshot);
        self.topology_revision = new_revision;
    }

    /// Handles a host-control-protocol event, updating dashboard state.
    ///
    /// `AgentStatus` events update the semantic status and control availability.
    /// Other event kinds are acknowledged but don't change dashboard state.
    pub fn handle_host_event(&mut self, event: &HostEvent) {
        match event.kind() {
            HostEventKind::AgentStatus => {
                if let Some(status) = event.status() {
                    self.status = match status {
                        HostSemanticStatus::Running => AgentStatus::Busy,
                        HostSemanticStatus::Waiting => AgentStatus::Waiting,
                        HostSemanticStatus::Idle | HostSemanticStatus::Done => AgentStatus::Idle,
                        HostSemanticStatus::Failed => AgentStatus::Error,
                    };
                    match self.status {
                        AgentStatus::Idle | AgentStatus::Error => {
                            self.can_start = true;
                            self.can_stop = false;
                        }
                        AgentStatus::Busy | AgentStatus::Streaming | AgentStatus::Waiting => {
                            self.can_start = false;
                            self.can_stop = true;
                        }
                    }
                }
            }
            HostEventKind::PaneLifecycle
            | HostEventKind::OutputMatch
            | HostEventKind::Cancelled
            | HostEventKind::Timeout
            | HostEventKind::HostError => {
                // Acknowledged but handled by the runtime, not the dashboard.
            }
        }
    }

    /// Returns an error when a selector is stale (bound to an older revision).
    ///
    /// # Errors
    ///
    /// Returns `"topology-mismatch"` when the selector revision is stale.
    pub fn check_selector_freshness(&self, selector: &HostSelector) -> Result<(), &'static str> {
        if let Some(ref snapshot) = self.topology {
            if selector.revision() < snapshot.revision() {
                return Err(STALE_SELECTOR_MSG);
            }
        }
        Ok(())
    }

    /// Re-resolves a stale selector against the current topology snapshot.
    #[must_use]
    pub fn resolve_selector(&self, selector: &HostSelector) -> Option<HostSelector> {
        self.topology.as_ref().and_then(|snapshot| {
            if selector.revision() >= snapshot.revision() {
                return Some(selector.clone());
            }
            find_selector_in_snapshot(snapshot, selector)
        })
    }
}

// ---------------------------------------------------------------------------
// Queue item
// ---------------------------------------------------------------------------

/// A background work queue item.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueueItem {
    /// Task identifier.
    pub task_id: String,
    /// Task label or description.
    pub label: String,
    /// Task status.
    pub status: QueueItemStatus,
}

/// Status of a queue item.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum QueueItemStatus {
    /// Queued and waiting.
    Queued,
    /// Running.
    Running,
    /// Completed successfully.
    Completed,
    /// Failed.
    Failed,
    /// Cancelled.
    Cancelled,
}

impl QueueItemStatus {
    /// Returns the stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Queued => "queued",
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Failed => "failed",
            Self::Cancelled => "cancelled",
        }
    }
}

// ---------------------------------------------------------------------------
// Free functions
// ---------------------------------------------------------------------------

/// Finds a matching selector in a topology snapshot by host + resource identity.
fn find_selector_in_snapshot(
    snapshot: &HostTopologySnapshot,
    original: &HostSelector,
) -> Option<HostSelector> {
    snapshot.selectors().iter().find_map(|candidate| {
        if candidate.host_id() == original.host_id()
            && candidate.resource_id() == original.resource_id()
        {
            Some(candidate.clone())
        } else {
            None
        }
    })
}

// ---------------------------------------------------------------------------
// Agent control snapshot rendering
// ---------------------------------------------------------------------------

/// Renders a deterministic text snapshot of the agent dashboard.
#[must_use]
pub fn render_agent_dashboard_snapshot(dashboard: &AgentDashboardState) -> String {
    let mut lines: Vec<String> = Vec::with_capacity(32);
    debug_assert!(dashboard.status().as_str().len() > 0, "empty status");
    lines.push(format!("status: {}", dashboard.status().as_str()));
    if dashboard.can_start() {
        lines.push(String::from("control: start"));
    }
    if dashboard.can_stop() {
        lines.push(String::from("control: stop"));
    }

    // Topology
    if let Some(snapshot) = dashboard.topology() {
        lines.push(format!(
            "{} {}",
            TOPOLOGY_REVISION_LABEL,
            snapshot.revision()
        ));
        for selector in dashboard.tracked_selectors() {
            lines.push(format!(
                "selector: {}/{}",
                selector.host_id(),
                selector.resource_id(),
            ));
        }
    } else {
        lines.push(String::from(TOPOLOGY_PENDING_LABEL));
    }

    // Queue
    for item in dashboard.queue_items() {
        lines.push(format!(
            "queue: {} [{}]",
            item.label, item.status.as_str()
        ));
    }

    // Peers
    for peer in dashboard.peers() {
        lines.push(format!("peer: {peer}"));
    }

    // Processes
    for proc in dashboard.processes() {
        lines.push(format!("process: {proc}"));
    }

    if dashboard.queue_items().is_empty()
        && dashboard.peers().is_empty()
        && dashboard.processes().is_empty()
    {
        lines.push(String::from("(no active items)"));
    }

    lines.join("\n")
}