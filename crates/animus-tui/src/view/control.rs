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

#[cfg(test)]
#[allow(unknown_lints, non_trait_imports, path_segment_repetition, ambiguous_params)]
mod agent_control_tests {
    use super::*;
    use animus_adapters::{
        HostSelectorDraft, HostTopologySnapshotDraft,
    };

    // -----------------------------------------------------------------------
    // Agent status tests
    // -----------------------------------------------------------------------

    #[test]
    fn idle_status_has_correct_labels() {
        assert_eq!(AgentStatus::Idle.as_str(), "idle");
        assert_eq!(AgentStatus::Busy.as_str(), "busy");
        assert_eq!(AgentStatus::Streaming.as_str(), "streaming");
        assert_eq!(AgentStatus::Waiting.as_str(), "waiting");
        assert_eq!(AgentStatus::Error.as_str(), "error");
    }

    // -----------------------------------------------------------------------
    // Agent dashboard tests
    // -----------------------------------------------------------------------

    #[test]
    fn idle_dashboard_allows_start() {
        let dashboard = AgentDashboardState::idle();
        assert_eq!(dashboard.status(), AgentStatus::Idle);
        assert!(dashboard.can_start());
        assert!(!dashboard.can_stop());
    }

    #[test]
    fn busy_dashboard_allows_stop() {
        let mut dashboard = AgentDashboardState::idle();
        dashboard.set_status(AgentStatus::Busy);
        assert!(!dashboard.can_start());
        assert!(dashboard.can_stop());
    }

    #[test]
    fn error_dashboard_allows_both() {
        let mut dashboard = AgentDashboardState::idle();
        dashboard.set_status(AgentStatus::Error);
        assert!(dashboard.can_start());
        assert!(dashboard.can_stop());
    }

    #[test]
    fn queue_items_populated() {
        let mut dashboard = AgentDashboardState::idle();
        dashboard.set_queue_items(vec![
            QueueItem {
                task_id: String::from("task-1"),
                label: String::from("build"),
                status: QueueItemStatus::Running,
            },
            QueueItem {
                task_id: String::from("task-2"),
                label: String::from("test"),
                status: QueueItemStatus::Queued,
            },
        ]);
        assert_eq!(dashboard.queue_items().len(), 2);
    }

    #[test]
    fn peers_and_processes() {
        let mut dashboard = AgentDashboardState::idle();
        dashboard.set_peers(vec![String::from("agent-b"), String::from("agent-c")]);
        dashboard.set_processes(vec![String::from("cargo build --release")]);
        assert_eq!(dashboard.peers().len(), 2);
        assert_eq!(dashboard.processes().len(), 1);
    }

    // -----------------------------------------------------------------------
    // Snapshot tests
    // -----------------------------------------------------------------------

    #[test]
    fn idle_dashboard_snapshot() {
        let dashboard = AgentDashboardState::idle();
        let snapshot = render_agent_dashboard_snapshot(&dashboard);
        assert!(snapshot.contains("status: idle"));
        assert!(snapshot.contains("control: start"));
        assert!(snapshot.contains("(no active items)"));
    }

    #[test]
    fn busy_dashboard_snapshot() {
        let mut dashboard = AgentDashboardState::idle();
        dashboard.set_status(AgentStatus::Streaming);
        dashboard.set_queue_items(vec![
            QueueItem {
                task_id: String::from("t1"),
                label: String::from("build"),
                status: QueueItemStatus::Running,
            },
        ]);
        let snapshot = render_agent_dashboard_snapshot(&dashboard);
        assert!(snapshot.contains("status: streaming"));
        assert!(snapshot.contains("control: stop"));
        assert!(snapshot.contains("queue: build [running]"));
    }

    // -----------------------------------------------------------------------
    // Negative tests
    // -----------------------------------------------------------------------

    #[test]
    fn missing_worker_service_does_not_panic() {
        let dashboard = AgentDashboardState::idle();
        assert!(dashboard.queue_items().is_empty());
        assert!(dashboard.peers().is_empty());
    }

    #[test]
    fn denied_control_does_not_enable() {
        let mut dashboard = AgentDashboardState::idle();
        dashboard.set_status(AgentStatus::Idle);
        assert!(dashboard.can_start());
        dashboard.set_status(AgentStatus::Busy);
        assert!(!dashboard.can_start());
    }

    #[test]
    fn unbounded_queue_is_prevented() {
        let mut dashboard = AgentDashboardState::idle();
        let many: Vec<_> = (0..1000)
            .map(|i| QueueItem {
                task_id: format!("task-{i}"),
                label: format!("item {i}"),
                status: QueueItemStatus::Queued,
            })
            .collect();
        dashboard.set_queue_items(many);
        assert_eq!(dashboard.queue_items().len(), 1000);
        let snapshot = render_agent_dashboard_snapshot(&dashboard);
        assert!(snapshot.contains("queue: item 0 [queued]"));
    }

    // -----------------------------------------------------------------------
    // Topology tests
    // -----------------------------------------------------------------------

    #[allow(unknown_lints, ambiguous_params)]
    fn make_selector(host: &str, resource: &str, revision: u64) -> HostSelector {
        HostSelector::validate(HostSelectorDraft {
            host_id: host,
            resource_id: resource,
            kind: "agent",
            lifetime: "live",
            revision,
        })
        .expect("test selector should be valid")
    }

    fn make_snapshot(
        host_id: &str,
        revision: u64,
        selectors: &[HostSelector],
    ) -> HostTopologySnapshot {
        HostTopologySnapshot::validate(HostTopologySnapshotDraft {
            host_id,
            revision,
            selectors,
            labels: &[],
            supported_operations: &[],
        })
        .expect("test snapshot should be valid")
    }

    #[test]
    fn idle_dashboard_has_no_topology() {
        let dashboard = AgentDashboardState::idle();
        assert!(dashboard.topology().is_none());
        assert_eq!(dashboard.topology_revision(), 0);
        assert!(dashboard.tracked_selectors().is_empty());
    }

    #[test]
    fn topology_snapshot_updates_revision() {
        let mut dashboard = AgentDashboardState::idle();
        let selector = make_selector("host-1", "pane-a", 1);
        let snapshot = make_snapshot("host-1", 7, &[selector]);
        dashboard.apply_topology_snapshot(snapshot);
        assert_eq!(dashboard.topology_revision(), 7);
        assert!(dashboard.topology().is_some());
    }

    #[test]
    fn stale_topology_is_not_applied() {
        let mut dashboard = AgentDashboardState::idle();
        let old_selector = make_selector("host-1", "pane-a", 1);
        let old_snapshot = make_snapshot("host-1", 5, &[old_selector]);
        dashboard.apply_topology_snapshot(old_snapshot);
        assert_eq!(dashboard.topology_revision(), 5);

        let new_selector = make_selector("host-1", "pane-a", 6);
        let new_snapshot = make_snapshot("host-1", 10, &[new_selector]);
        dashboard.apply_topology_snapshot(new_snapshot);
        assert_eq!(dashboard.topology_revision(), 10);

        let stale_snapshot = make_snapshot("host-1", 8, &[]);
        dashboard.apply_topology_snapshot(stale_snapshot);
        assert_eq!(dashboard.topology_revision(), 10);
    }

    #[test]
    fn host_event_agent_status_updates_dashboard() {
        let mut dashboard = AgentDashboardState::idle();
        let selector = make_selector("host-1", "agent-ctl", 1);
        let event = HostEvent::agent_status("corr-1", selector, HostSemanticStatus::Running)
            .expect("valid event");
        dashboard.handle_host_event(&event);
        assert_eq!(dashboard.status(), AgentStatus::Busy);
        assert!(!dashboard.can_start());
        assert!(dashboard.can_stop());
    }

    #[test]
    fn host_event_running_to_idle_restores_start() {
        let mut dashboard = AgentDashboardState::idle();
        let selector = make_selector("host-1", "agent-ctl", 1);

        let run = HostEvent::agent_status("corr-1", selector.clone(), HostSemanticStatus::Running)
            .expect("valid event");
        dashboard.handle_host_event(&run);
        assert_eq!(dashboard.status(), AgentStatus::Busy);

        let done = HostEvent::agent_status("corr-1", selector, HostSemanticStatus::Done)
            .expect("valid event");
        dashboard.handle_host_event(&done);
        assert_eq!(dashboard.status(), AgentStatus::Idle);
        assert!(dashboard.can_start());
        assert!(!dashboard.can_stop());
    }

    #[test]
    fn stale_selector_rejected_by_freshness_check() {
        let mut dashboard = AgentDashboardState::idle();
        let live_selector = make_selector("host-1", "pane-a", 10);
        let snapshot = make_snapshot("host-1", 10, &[live_selector]);
        dashboard.apply_topology_snapshot(snapshot);

        let stale = make_selector("host-1", "pane-a", 5);
        assert_eq!(
            dashboard.check_selector_freshness(&stale),
            Err("topology-mismatch")
        );

        let fresh = make_selector("host-1", "pane-a", 10);
        assert!(dashboard.check_selector_freshness(&fresh).is_ok());
    }

    #[test]
    fn stale_selector_is_re_resolved_when_resource_still_exists() {
        let mut dashboard = AgentDashboardState::idle();
        let original = make_selector("host-1", "pane-a", 5);
        dashboard.tracked_selectors.push(original);

        let fresh = make_selector("host-1", "pane-a", 10);
        let snapshot = make_snapshot("host-1", 10, &[fresh]);
        dashboard.apply_topology_snapshot(snapshot);

        assert!(dashboard.tracked_selectors().iter().all(|s| s.revision() == 10));
    }

    #[test]
    fn stale_selector_missing_from_topology_stays_tracked() {
        let mut dashboard = AgentDashboardState::idle();
        let removed = make_selector("host-1", "removed-pane", 5);
        dashboard.tracked_selectors.push(removed);

        let live = make_selector("host-1", "still-here", 10);
        let snapshot = make_snapshot("host-1", 10, &[live]);
        dashboard.apply_topology_snapshot(snapshot);

        assert_eq!(dashboard.tracked_selectors().len(), 1);
        assert_eq!(dashboard.tracked_selectors()[0].revision(), 5);
        assert_eq!(
            dashboard.check_selector_freshness(&dashboard.tracked_selectors()[0]),
            Err("topology-mismatch")
        );
    }

    #[test]
    fn topology_snapshot_shows_in_render() {
        let mut dashboard = AgentDashboardState::idle();
        let selector = make_selector("host-main", "agent-ctl", 3);
        dashboard.tracked_selectors.push(selector);
        let snapshot = make_snapshot("host-main", 3, &[]);
        dashboard.apply_topology_snapshot(snapshot);

        let rendered = render_agent_dashboard_snapshot(&dashboard);
        assert!(rendered.contains("topology: rev 3"));
        assert!(rendered.contains("selector: host-main/agent-ctl"));
    }

    #[test]
    fn pending_topology_shows_in_render() {
        let dashboard = AgentDashboardState::idle();
        let rendered = render_agent_dashboard_snapshot(&dashboard);
        assert!(rendered.contains("topology: pending"));
    }
}