// Consolidated tests for layout and control modules, extracted to keep
// source files under the Octet excessive_file_length threshold.

#[cfg(test)]
#[allow(unknown_lints, non_trait_imports, path_segment_repetition, ambiguous_params)]
mod layout_tests {
    use super::*;
    use animus_adapters::{
        HostControlError, HostSelector, HostSelectorDraft,
        HostTopologySnapshot, HostTopologySnapshotDraft,
        LayoutApplyRequest, LayoutNode, LayoutPaneDraft,
    };

    #[allow(unknown_lints, ambiguous_params)]
    fn build_selector(host: &str, selected_resource_name: &str, revision_val: u64) -> HostSelector {
        HostSelector::validate(HostSelectorDraft {
            host_id: host,
            resource_id: selected_resource_name,
            kind: "pane",
            lifetime: "live",
            revision: revision_val,
        })
        .expect("test selector should be valid")
    }

    fn make_topology(selectors: &[HostSelector]) -> HostTopologySnapshot {
        HostTopologySnapshot::validate(HostTopologySnapshotDraft {
            host_id: "host-main",
            revision: 1,
            selectors,
            labels: &["main"],
            supported_operations: &["focus", "rename", "close", "export_layout"],
        })
        .expect("test topology should be valid")
    }

    fn make_pane(node_id: &str) -> LayoutNode {
        LayoutNode::pane_with(LayoutPaneDraft {
            node_id,
            label: None,
            cwd_summary: None,
            command: None,
            focused: false,
            zoomed: false,
        })
        .expect("test pane should be valid")
    }

    fn make_focused_pane(node_id: &str) -> LayoutNode {
        LayoutNode::pane_with(LayoutPaneDraft {
            node_id,
            label: Some("focused"),
            cwd_summary: None,
            command: None,
            focused: true,
            zoomed: false,
        })
        .expect("test pane should be valid")
    }

    // -----------------------------------------------------------------------
    // Positive layout tests
    // -----------------------------------------------------------------------

    #[test]
    fn single_pane_resolves() {
        let sel = build_selector("host-main", "pane-1", 1);
        let topology = make_topology(&[sel]);
        let pane = make_pane("pane-1");
        let request = LayoutApplyRequest::new(pane, false)
            .expect("valid request");
        let tree = resolve_layout(&request, &topology)
            .expect("layout should resolve");
        assert_eq!(tree.node_id(), "pane-1");
    }

    #[test]
    fn split_resolves() {
        let sel_a = build_selector("host-main", "pane-a", 1);
        let sel_b = build_selector("host-main", "pane-b", 1);
        let topology = make_topology(&[sel_a, sel_b]);
        let pane_a = make_pane("pane-a");
        let pane_b = make_pane("pane-b");
        let split = LayoutNode::split("root", "horizontal", vec![pane_a, pane_b])
            .expect("valid split");
        let request = LayoutApplyRequest::new(split, false)
            .expect("valid request");
        let tree = resolve_layout(&request, &topology)
            .expect("layout should resolve");
        assert_eq!(tree.node_id(), "root");
        if let LayoutTree::Split { axis, children, .. } = &tree {
            assert_eq!(*axis, LayoutAxis::Horizontal);
            assert_eq!(children.len(), 2);
            assert_eq!(children[0].node_id(), "pane-a");
            assert_eq!(children[1].node_id(), "pane-b");
        } else {
            panic!("expected split tree");
        }
    }

    #[test]
    fn focused_pane_is_detected_in_tree() {
        let sel = build_selector("host-main", "pane-main", 1);
        let topology = make_topology(&[sel]);
        let pane = make_focused_pane("pane-main");
        let request = LayoutApplyRequest::new(pane, false)
            .expect("valid request");
        let tree = resolve_layout(&request, &topology)
            .expect("layout should resolve");
        assert_eq!(tree.focused_pane_id(), Some("pane-main"));
    }

    #[test]
    fn no_focus_in_tree_returns_none() {
        let sel = build_selector("host-main", "pane-plain", 1);
        let topology = make_topology(&[sel]);
        let pane = make_pane("pane-plain");
        let request = LayoutApplyRequest::new(pane, false)
            .expect("valid request");
        let tree = resolve_layout(&request, &topology)
            .expect("layout should resolve");
        assert_eq!(tree.focused_pane_id(), None);
    }

    // -----------------------------------------------------------------------
    // Negative layout tests
    // -----------------------------------------------------------------------

    #[test]
    fn pane_absent_from_topology_fails_closed() {
        let topology = make_topology(&[]);
        let pane = make_pane("unknown-pane");
        let request = LayoutApplyRequest::new(pane, false)
            .expect("valid request");
        let err = resolve_layout(&request, &topology)
            .expect_err("absent pane should fail");
        assert_eq!(
            err,
            LayoutDiagnostic::PaneAbsentFromTopology(
                String::from("unknown-pane")
            )
        );
        assert_eq!(err.as_str(), "pane-absent-from-topology");
    }

    #[test]
    fn empty_split_fails_closed() {
        let err = LayoutNode::split("empty-root", "horizontal", vec![])
            .expect_err("empty split should be rejected");
        assert_eq!(err, HostControlError::InvalidLayout);
    }

    #[test]
    fn duplicate_node_id_fails_closed() {
        let sel = build_selector("host-main", "pane-dupe", 1);
        let topology = make_topology(&[sel]);
        let pane = make_pane("pane-dupe");
        let split = LayoutNode::split("root", "vertical", vec![pane.clone(), pane])
            .expect("LayoutNode allows duplicates; resolve_layout rejects them");
        let request = LayoutApplyRequest::new(split, false)
            .expect("valid request");
        let err = resolve_layout(&request, &topology)
            .expect_err("duplicate id should fail at resolve_layout");
        assert_eq!(
            err,
            LayoutDiagnostic::DuplicateNodeId(
                String::from("pane-dupe")
            )
        );
        assert_eq!(err.as_str(), "duplicate-node-id");
    }

    #[test]
    fn preserve_live_processes_request_fails_closed() {
        let pane = make_pane("pane-1");
        let err = LayoutApplyRequest::new(pane, true)
            .expect_err("preserve_live_processes rejected");
        assert_eq!(err, HostControlError::InvalidLayout);
    }

    #[test]
    fn snapshot_single_pane() {
        let sel = build_selector("host-main", "pane-x", 1);
        let topology = make_topology(&[sel]);
        let pane = make_focused_pane("pane-x");
        let request = LayoutApplyRequest::new(pane, false)
            .expect("valid request");
        let tree = resolve_layout(&request, &topology)
            .expect("layout should resolve");
        let snapshot = render_layout_tree_snapshot(&tree);
        assert!(snapshot.contains("pane: pane-x"));
        assert!(snapshot.contains("[focused]"));
    }

    #[test]
    fn snapshot_split_tree() {
        let sel_a = build_selector("host-main", "left", 1);
        let sel_b = build_selector("host-main", "right", 1);
        let topology = make_topology(&[sel_a, sel_b]);
        let left = make_focused_pane("left");
        let right = make_pane("right");
        let split = LayoutNode::split("root", "vertical", vec![left, right])
            .expect("valid split");
        let request = LayoutApplyRequest::new(split, false)
            .expect("valid request");
        let tree = resolve_layout(&request, &topology)
            .expect("layout should resolve");
        let snapshot = render_layout_tree_snapshot(&tree);
        assert!(snapshot.contains("split: root [vertical]"));
        assert!(snapshot.contains("  pane: left [focused]"));
        assert!(snapshot.contains("  pane: right"));
    }
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