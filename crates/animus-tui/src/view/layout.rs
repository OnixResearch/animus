// Pane/split layout: host-only pane-tree model consuming host-control-protocol
// layout DTOs. Layout types stay in the TUI crate; mismatches fail closed
// with a typed diagnostic.

#[allow(unknown_lints, non_trait_imports)]
use animus_adapters::LayoutAxis;

/// Typed layout diagnostic for mismatch/failure cases.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayoutDiagnostic {
    /// A pane node id is absent from the current host topology.
    PaneAbsentFromTopology(String),
    /// A split axis label is not supported.
    UnsupportedAxis(String),
    /// The layout tree references a node that duplicates an existing node id.
    DuplicateNodeId(String),
    /// The layout apply request has no child panes in a split.
    EmptySplit,
}

impl LayoutDiagnostic {
    /// Returns a stable diagnostic label for testing and rendering.
    #[must_use]
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::PaneAbsentFromTopology(_) => "pane-absent-from-topology",
            Self::UnsupportedAxis(_) => "unsupported-axis",
            Self::DuplicateNodeId(_) => "duplicate-node-id",
            Self::EmptySplit => "empty-split",
        }
    }
}

/// Host-only renderable pane view derived from a layout node.
#[derive(Clone, Debug, Eq, PartialEq)]
#[allow(unknown_lints, path_segment_repetition)]
pub struct PaneView {
    /// Pane node identity matched from the host-control protocol.
    node_id: String,
    /// Optional pane label.
    label: Option<String>,
    /// Whether this pane has host focus.
    is_focused: bool,
    /// Whether this pane is zoomed.
    is_zoomed: bool,
}

impl PaneView {
    /// Builds a pane view from layout and topology metadata.
    #[must_use]
    pub fn new(
        node_id: &str,
        label: Option<String>,
        is_focused: bool,
        is_zoomed: bool,
    ) -> Self {
        Self {
            node_id: String::from(node_id),
            label,
            is_focused,
            is_zoomed,
        }
    }

    /// Returns the pane node identity.
    #[must_use]
    pub fn node_id(&self) -> &str {
        self.node_id.as_str()
    }

    /// Returns the optional pane label.
    #[must_use]
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns whether this pane has host focus.
    #[must_use]
    pub const fn is_focused(&self) -> bool {
        self.is_focused
    }

    /// Returns whether this pane is zoomed.
    #[must_use]
    pub const fn is_zoomed(&self) -> bool {
        self.is_zoomed
    }
}

/// Host-only renderable layout tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LayoutTree {
    /// A pane leaf.
    Pane(PaneView),
    /// A split with child layout trees along an axis.
    Split {
        /// Split node identity.
        node_id: String,
        /// Split axis.
        axis: LayoutAxis,
        /// Child layout trees.
        children: Vec<LayoutTree>,
    },
}

impl LayoutTree {
    /// Returns the node identity for this layout tree node.
    #[must_use]
    pub fn node_id(&self) -> &str {
        match self {
            Self::Pane(pane) => pane.node_id(),
            Self::Split { node_id, .. } => node_id.as_str(),
        }
    }

    /// Returns the focused pane node id when one exists in the tree.
    #[must_use]
    pub fn focused_pane_id(&self) -> Option<&str> {
        match self {
            Self::Pane(pane) => {
                if pane.is_focused() {
                    Some(pane.node_id())
                } else {
                    None
                }
            }
            Self::Split { children, .. } => {
                for child in children {
                    if let Some(id) = child.focused_pane_id() {
                        return Some(id);
                    }
                }
                None
            }
        }
    }
}

/// Resolves a layout apply request against a host topology snapshot,
/// producing a renderable layout tree or a typed diagnostic.
///
/// # Errors
///
/// Returns [`LayoutDiagnostic`] when:
/// - A pane node id is absent from the host topology.
/// - A split axis is unsupported.
/// - Duplicate node ids exist in the layout tree.
/// - A split has no child nodes.
pub fn resolve_layout(
    request: &animus_adapters::LayoutApplyRequest,
    topology: &animus_adapters::HostTopologySnapshot,
) -> Result<LayoutTree, LayoutDiagnostic> {
    let mut seen_ids: Vec<String> = Vec::new();
    resolve_node(request.root(), topology, &mut seen_ids)
}

#[allow(unknown_lints, borrowed_argument_types)]
fn resolve_node(
    node: &animus_adapters::LayoutNode,
    topology: &animus_adapters::HostTopologySnapshot,
    seen_ids: &mut Vec<String>,
) -> Result<LayoutTree, LayoutDiagnostic> {
    let node_id = node.node_id();
    debug_assert!(!node_id.is_empty(), "resolve_node called with empty node id");
    if seen_ids.contains(&String::from(node_id)) {
        return Err(LayoutDiagnostic::DuplicateNodeId(String::from(node_id)));
    }
    seen_ids.push(String::from(node_id));

    #[allow(unknown_lints, bool_naming)]
    match node {
        animus_adapters::LayoutNode::Pane {
            node_id,
            label,
            focused: is_focused,
            zoomed: is_zoomed,
            ..
        } => {
            let pane_in_topology = topology.selectors().iter().any(
                |selector: &animus_adapters::HostSelector| {
                    selector.resource_id() == node_id.as_str()
                },
            );
            if !pane_in_topology {
                return Err(LayoutDiagnostic::PaneAbsentFromTopology(
                    node_id.clone(),
                ));
            }

            Ok(LayoutTree::Pane(PaneView::new(
                node_id,
                label.clone(),
                *is_focused,
                *is_zoomed,
            )))
        }
        animus_adapters::LayoutNode::Split {
            node_id: _,
            axis,
            children,
        } => {
            if children.is_empty() {
                return Err(LayoutDiagnostic::EmptySplit);
            }

            let resolved_children: Vec<LayoutTree> = children
                .iter()
                .map(|child| resolve_node(child, topology, seen_ids))
                .collect::<Result<Vec<_>, _>>()?;

            Ok(LayoutTree::Split {
                node_id: String::from(node.node_id()),
                axis: *axis,
                children: resolved_children,
            })
        }
    }
}

/// Renders a deterministic text representation of a layout tree.
#[must_use]
pub fn render_layout_tree_snapshot(tree: &LayoutTree) -> String {
    let mut lines: Vec<String> = Vec::new();
    let node_id = tree.node_id();
    debug_assert!(!node_id.is_empty(), "empty node id");
    render_tree_iterative(tree, &mut lines);
    lines.join("\n")
}

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

/// Iterative renderer that avoids recursion.
#[allow(unknown_lints, borrowed_argument_types)]
fn render_tree_iterative<'a>(tree: &'a LayoutTree, lines: &mut Vec<String>) {
    debug_assert!(tree.node_id().len() > 0, "empty tree");

    struct StackEntry<'a> {
        item: &'a LayoutTree,
        depth: usize,
    }

    // Pre-allocate with a reasonable initial capacity.
    let mut stack: Vec<StackEntry<'_>> = Vec::with_capacity(16);
    stack.push(StackEntry {
        item: tree,
        depth: 0,
    });

    while let Some(entry) = stack.pop() {
        let indent = "  ".repeat(entry.depth);
        match entry.item {
            LayoutTree::Pane(pane) => {
                let focus_mark = if pane.is_focused() { " [focused]" } else { "" };
                let zoom_mark = if pane.is_zoomed() { " [zoomed]" } else { "" };
                lines.push(format!(
                    "{indent}pane: {}{}{}",
                    pane.node_id(),
                    focus_mark,
                    zoom_mark,
                ));
            }
            LayoutTree::Split {
                node_id,
                axis,
                children,
            } => {
                let axis_label = match axis {
                    LayoutAxis::Horizontal => "horizontal",
                    LayoutAxis::Vertical => "vertical",
                };
                lines.push(format!("{indent}split: {node_id} [{axis_label}]"));
                for child in children.iter().rev() {
                    stack.push(StackEntry {
                        item: child,
                        depth: entry.depth.wrapping_add(1),
                    });
                }
            }
        }
    }
}
