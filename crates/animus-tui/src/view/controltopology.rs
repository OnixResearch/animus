// Topology-aware control resolution: connects the agent control dashboard
// to host-control-protocol `HostEvent`/`HostTopologySnapshot` DTOs so
// controls target the correct workspace, tab, pane, or agent and re-resolve
// stale selectors after topology changes.
//
// Included via `include!` from control.rs — all items are in the same module
// and share imports declared in control.rs.

/// Label for topology revision display.
const TOPOLOGY_REVISION_LABEL: &str = "topology: rev";
/// Label for pending-topology status.
const TOPOLOGY_PENDING_LABEL: &str = "topology: pending";
/// Label for stale-selector diagnostic.
const STALE_SELECTOR_MSG: &str = "topology-mismatch";

// ---------------------------------------------------------------------------
// Topology methods on AgentDashboardState
// ---------------------------------------------------------------------------

impl AgentDashboardState {
    /// Returns the current topology revision.
    #[must_use]
    pub const fn topology_revision(&self) -> u64 {
        self.topology_revision
    }

    /// Returns the current topology snapshot, if any.
    #[must_use]
    pub const fn topology(&self) -> Option<&animus_adapters::HostTopologySnapshot> {
        self.topology.as_ref()
    }

    /// Returns tracked selectors bound to the current topology.
    #[must_use]
    pub fn tracked_selectors(&self) -> &[animus_adapters::HostSelector] {
        self.tracked_selectors.as_slice()
    }

    /// Applies a topology snapshot, re-resolving stale selectors.
    ///
    /// Selectors bound to an older revision are replaced with fresh selectors
    /// from the new snapshot when the resource identity still exists.
    /// If a tracked selector's resource has been removed, it remains tracked
    /// but `check_selector_freshness` reports a mismatch on the next control.
    pub fn apply_topology_snapshot(&mut self, snapshot: animus_adapters::HostTopologySnapshot) {
        let new_revision = snapshot.revision();
        if new_revision <= self.topology_revision && self.topology.is_some() {
            return;
        }
        let mut resolved: Vec<animus_adapters::HostSelector> =
            Vec::with_capacity(self.tracked_selectors.len());
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
    pub fn handle_host_event(&mut self, event: &animus_adapters::HostEvent) {
        match event.kind() {
            animus_adapters::HostEventKind::AgentStatus => {
                if let Some(status) = event.status() {
                    self.status = match status {
                        animus_adapters::HostSemanticStatus::Idle
                        | animus_adapters::HostSemanticStatus::Done => AgentStatus::Idle,
                        animus_adapters::HostSemanticStatus::Running => AgentStatus::Busy,
                        animus_adapters::HostSemanticStatus::Waiting => AgentStatus::Waiting,
                        animus_adapters::HostSemanticStatus::Failed => AgentStatus::Error,
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
            animus_adapters::HostEventKind::PaneLifecycle
            | animus_adapters::HostEventKind::OutputMatch
            | animus_adapters::HostEventKind::Cancelled
            | animus_adapters::HostEventKind::Timeout
            | animus_adapters::HostEventKind::HostError => {}
        }
    }

    /// Returns an error when a selector is stale (bound to an older revision).
    ///
    /// # Errors
    ///
    /// Returns `"topology-mismatch"` when the selector revision is stale.
    pub fn check_selector_freshness(
        &self,
        selector: &animus_adapters::HostSelector,
    ) -> Result<(), &'static str> {
        if let Some(ref snapshot) = self.topology {
            if selector.revision() < snapshot.revision() {
                return Err(STALE_SELECTOR_MSG);
            }
        }
        Ok(())
    }

    /// Re-resolves a stale selector against the current topology snapshot.
    #[must_use]
    pub fn resolve_selector(
        &self,
        selector: &animus_adapters::HostSelector,
    ) -> Option<animus_adapters::HostSelector> {
        self.topology.as_ref().and_then(|snapshot| {
            if selector.revision() >= snapshot.revision() {
                return Some(selector.clone());
            }
            find_selector_in_snapshot(snapshot, selector)
        })
    }
}

// ---------------------------------------------------------------------------
// Topology rendering
// ---------------------------------------------------------------------------

/// Appends topology info to the snapshot lines buffer.
#[allow(unknown_lints, borrowed_argument_types)]
pub fn append_topology_lines(dashboard: &AgentDashboardState, lines: &mut Vec<String>) {
    if let Some(snapshot) = dashboard.topology() {
        lines.push(format!("{} {}", TOPOLOGY_REVISION_LABEL, snapshot.revision()));
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
}

// ---------------------------------------------------------------------------
// Internal helpers
// ---------------------------------------------------------------------------

/// Finds a matching selector in a topology snapshot by host + resource identity.
fn find_selector_in_snapshot(
    snapshot: &animus_adapters::HostTopologySnapshot,
    original: &animus_adapters::HostSelector,
) -> Option<animus_adapters::HostSelector> {
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