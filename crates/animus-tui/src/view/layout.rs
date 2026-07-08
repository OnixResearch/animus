// Pane/split layout: host-only pane-tree model consuming host-control-protocol
// layout DTOs. Layout types stay in the TUI crate; mismatches fail closed
// with a typed diagnostic.

#[cfg_attr(animus_octet, allow(non_trait_imports))]
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

const LAYOUT_STACK_INITIAL_CAPACITY: usize = 16;

struct LayoutStackEntry<'a> {
    item: &'a LayoutTree,
    depth: usize,
}

/// Iterative renderer that avoids recursion.
#[allow(unknown_lints, borrowed_argument_types)]
fn render_tree_iterative(tree: &LayoutTree, lines: &mut Vec<String>) {
    debug_assert!(!tree.node_id().is_empty(), "empty tree");

    // Pre-allocate with a reasonable initial capacity.
    let mut stack: Vec<LayoutStackEntry<'_>> = Vec::with_capacity(LAYOUT_STACK_INITIAL_CAPACITY);
    stack.push(LayoutStackEntry {
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
                    stack.push(LayoutStackEntry {
                        item: child,
                        depth: entry.depth.wrapping_add(1),
                    });
                }
            }
        }
    }
}
