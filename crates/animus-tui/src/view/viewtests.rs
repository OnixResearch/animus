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