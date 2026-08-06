//! Pure session branch explorer reducer, deterministic snapshots, and thin host port.

#![allow(missing_docs, clippy::missing_errors_doc)]

use std::collections::BTreeSet;

use animus_runtime::branch_explorer::{
    BranchExplorerOperation, BranchPlanStatus, BranchProjectionSnapshot, BranchSelectedRead,
    plan_branch_operation,
};

const EMPTY_FOCUS_INDEX: usize = 0;
const INDENT_UNIT: &str = "  ";
const MAX_DIAGNOSTIC_BYTES: usize = 160;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BranchExplorerMode {
    Map,
    Read,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BranchExplorerAction {
    FocusNext,
    FocusPrevious,
    ToggleCollapse,
    ShowMap,
    ShowRead,
    Inspect,
    Activate,
    Fork,
    Bookmark { label: Option<String> },
    Reorder { ordered_child_ids: Vec<String> },
    Compare,
    Refresh,
    ApplyProjection(Box<BranchProjectionSnapshot>),
    HostSucceeded,
    HostFailed { diagnostic: String },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BranchExplorerIntent {
    Inspect {
        target_entry_id: String,
    },
    Activate {
        operation_id: String,
        target_entry_id: String,
    },
    Fork {
        operation_id: String,
        target_entry_id: String,
    },
    Bookmark {
        expected_revision_id: String,
        target_entry_id: String,
        label: Option<String>,
    },
    Reorder {
        expected_revision_id: String,
        parent_entry_id: String,
        ordered_child_ids: Vec<String>,
    },
    Compare {
        left_entry_id: String,
        right_entry_id: String,
    },
    Refresh {
        expected_revision_id: String,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BranchExplorerState {
    pub mode: BranchExplorerMode,
    pub projection: BranchProjectionSnapshot,
    pub selected_read: Option<BranchSelectedRead>,
    pub focused_entry_id: String,
    pub collapsed_entry_ids: BTreeSet<String>,
    pub comparison_left_id: Option<String>,
    pub stale: bool,
    pub awaiting_host: bool,
    pub diagnostic: Option<String>,
}

impl BranchExplorerState {
    #[must_use]
    pub fn new(projection: BranchProjectionSnapshot) -> Self {
        let focused_entry_id = projection
            .presentation_order
            .get(EMPTY_FOCUS_INDEX)
            .cloned()
            .unwrap_or_default();
        Self {
            mode: BranchExplorerMode::Map,
            projection,
            selected_read: None,
            focused_entry_id,
            collapsed_entry_ids: BTreeSet::new(),
            comparison_left_id: None,
            stale: false,
            awaiting_host: false,
            diagnostic: None,
        }
    }

    #[must_use]
    pub fn with_selected_read(mut self, selected_read: BranchSelectedRead) -> Self {
        self.selected_read = Some(selected_read);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BranchExplorerReduction {
    pub state: BranchExplorerState,
    pub intent: Option<BranchExplorerIntent>,
}

/// Purely reduce one explorer action.
#[must_use]
pub fn reduce_branch_explorer(
    state: &BranchExplorerState,
    action: BranchExplorerAction,
) -> BranchExplorerReduction {
    let mut next = state.clone();
    let intent = match action {
        BranchExplorerAction::FocusNext => {
            move_focus(&mut next, FocusDirection::Next);
            None
        }
        BranchExplorerAction::FocusPrevious => {
            move_focus(&mut next, FocusDirection::Previous);
            None
        }
        BranchExplorerAction::ToggleCollapse => {
            toggle_collapse(&mut next);
            None
        }
        BranchExplorerAction::ShowMap => {
            next.mode = BranchExplorerMode::Map;
            None
        }
        BranchExplorerAction::ShowRead => {
            next.mode = BranchExplorerMode::Read;
            None
        }
        BranchExplorerAction::Inspect => Some(BranchExplorerIntent::Inspect {
            target_entry_id: next.focused_entry_id.clone(),
        }),
        BranchExplorerAction::Activate => {
            operation_intent(&mut next, BranchExplorerOperation::ActivateLeaf)
        }
        BranchExplorerAction::Fork => {
            operation_intent(&mut next, BranchExplorerOperation::ForkFromNode)
        }
        BranchExplorerAction::Bookmark { label } => Some(BranchExplorerIntent::Bookmark {
            expected_revision_id: next.projection.revision_id.clone(),
            target_entry_id: next.focused_entry_id.clone(),
            label,
        }),
        BranchExplorerAction::Reorder { ordered_child_ids } => {
            Some(BranchExplorerIntent::Reorder {
                expected_revision_id: next.projection.revision_id.clone(),
                parent_entry_id: next.focused_entry_id.clone(),
                ordered_child_ids,
            })
        }
        BranchExplorerAction::Compare => comparison_intent(&mut next),
        BranchExplorerAction::Refresh => {
            next.awaiting_host = true;
            Some(BranchExplorerIntent::Refresh {
                expected_revision_id: next.projection.revision_id.clone(),
            })
        }
        BranchExplorerAction::ApplyProjection(projection) => {
            apply_projection(&mut next, *projection);
            None
        }
        BranchExplorerAction::HostSucceeded => {
            next.awaiting_host = false;
            next.diagnostic = None;
            None
        }
        BranchExplorerAction::HostFailed { diagnostic } => {
            next.awaiting_host = false;
            next.stale = true;
            next.diagnostic = Some(bound_diagnostic(diagnostic.as_str()));
            None
        }
    };
    if intent.is_some() {
        next.awaiting_host = true;
    }
    BranchExplorerReduction {
        state: next,
        intent,
    }
}

/// Render one deterministic metadata-only explorer text snapshot.
#[must_use]
pub fn render_branch_explorer_snapshot(state: &BranchExplorerState) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "branch-explorer mode={} revision={} external={} stale={} pending={}",
        mode_label(state.mode),
        state.projection.revision_id,
        state.projection.external_state.as_str(),
        state.stale,
        state.awaiting_host
    ));
    if let Some(diagnostic) = state.diagnostic.as_deref() {
        lines.push(format!("diagnostic={diagnostic}"));
    }
    match state.mode {
        BranchExplorerMode::Map => render_map(state, &mut lines),
        BranchExplorerMode::Read => render_read(state, &mut lines),
    }
    if state.projection.truncation.omitted_node_count > 0 {
        lines.push(format!(
            "truncated omitted={} limit={}",
            state.projection.truncation.omitted_node_count, state.projection.truncation.node_limit
        ));
    }
    lines.join("\n")
}

/// Host-owned intent dispatcher.
pub trait BranchExplorerHostPort {
    type Failure;

    fn dispatch_branch_intent(
        &mut self,
        intent: &BranchExplorerIntent,
    ) -> Result<(), Self::Failure>;
}

/// Dispatch one typed intent without giving the reducer host authority.
///
/// # Errors
/// Returns the host port's bounded failure.
pub fn dispatch_branch_explorer_intent<Host: BranchExplorerHostPort>(
    host: &mut Host,
    intent: &BranchExplorerIntent,
) -> Result<(), Host::Failure> {
    host.dispatch_branch_intent(intent)
}

#[derive(Clone, Copy)]
enum FocusDirection {
    Next,
    Previous,
}

fn move_focus(state: &mut BranchExplorerState, direction: FocusDirection) {
    let visible = visible_entry_ids(state);
    if visible.is_empty() {
        state.diagnostic = Some(String::from("empty_projection"));
        return;
    }
    let current = visible
        .iter()
        .position(|entry_id| *entry_id == state.focused_entry_id)
        .unwrap_or(EMPTY_FOCUS_INDEX);
    let next = match direction {
        FocusDirection::Next => current
            .saturating_add(1)
            .min(visible.len().saturating_sub(1)),
        FocusDirection::Previous => current.saturating_sub(1),
    };
    state.focused_entry_id.clone_from(&visible[next]);
}

fn toggle_collapse(state: &mut BranchExplorerState) {
    if !state
        .collapsed_entry_ids
        .remove(state.focused_entry_id.as_str())
    {
        state
            .collapsed_entry_ids
            .insert(state.focused_entry_id.clone());
    }
}

fn operation_intent(
    state: &mut BranchExplorerState,
    operation: BranchExplorerOperation,
) -> Option<BranchExplorerIntent> {
    let plan = plan_branch_operation(
        &state.projection,
        operation,
        state.projection.revision_id.as_str(),
        state.projection.active_leaf_id.as_str(),
        state.focused_entry_id.as_str(),
    );
    if plan.status == BranchPlanStatus::Blocked {
        state.diagnostic = plan.diagnostic.map(|issue| format!("blocked:{issue:?}"));
        return None;
    }
    match operation {
        BranchExplorerOperation::ActivateLeaf => Some(BranchExplorerIntent::Activate {
            operation_id: plan.operation_id,
            target_entry_id: plan.target_entry_id,
        }),
        BranchExplorerOperation::ForkFromNode => Some(BranchExplorerIntent::Fork {
            operation_id: plan.operation_id,
            target_entry_id: plan.target_entry_id,
        }),
        BranchExplorerOperation::Inspect => Some(BranchExplorerIntent::Inspect {
            target_entry_id: plan.target_entry_id,
        }),
    }
}

fn comparison_intent(state: &mut BranchExplorerState) -> Option<BranchExplorerIntent> {
    let Some(left_entry_id) = state.comparison_left_id.take() else {
        state.comparison_left_id = Some(state.focused_entry_id.clone());
        return None;
    };
    if left_entry_id == state.focused_entry_id {
        state.diagnostic = Some(String::from("comparison_requires_distinct_targets"));
        return None;
    }
    Some(BranchExplorerIntent::Compare {
        left_entry_id,
        right_entry_id: state.focused_entry_id.clone(),
    })
}

fn apply_projection(state: &mut BranchExplorerState, projection: BranchProjectionSnapshot) {
    let compatible = projection.session_id == state.projection.session_id
        && projection.policy_id == state.projection.policy_id;
    if !compatible {
        state.awaiting_host = false;
        state.stale = true;
        state.diagnostic = Some(String::from("projection_identity_mismatch"));
        return;
    }
    let focused_exists = projection
        .nodes
        .iter()
        .any(|node| node.entry_id == state.focused_entry_id);
    state.projection = projection;
    state.awaiting_host = false;
    state.stale = false;
    state.diagnostic = None;
    state.collapsed_entry_ids.retain(|entry_id| {
        state
            .projection
            .nodes
            .iter()
            .any(|node| node.entry_id == *entry_id)
    });
    if !focused_exists {
        state.focused_entry_id = state
            .projection
            .presentation_order
            .get(EMPTY_FOCUS_INDEX)
            .cloned()
            .unwrap_or_default();
        state.diagnostic = Some(String::from("focused_entry_removed"));
    }
}

fn visible_entry_ids(state: &BranchExplorerState) -> Vec<String> {
    state
        .projection
        .presentation_order
        .iter()
        .filter(|entry_id| !has_collapsed_ancestor(state, entry_id.as_str()))
        .cloned()
        .collect()
}

fn has_collapsed_ancestor(state: &BranchExplorerState, entry_id: &str) -> bool {
    let mut cursor = state
        .projection
        .nodes
        .iter()
        .find(|node| node.entry_id == entry_id)
        .and_then(|node| node.parent_id.as_deref());
    while let Some(parent_id) = cursor {
        if state.collapsed_entry_ids.contains(parent_id) {
            return true;
        }
        cursor = state
            .projection
            .nodes
            .iter()
            .find(|node| node.entry_id == parent_id)
            .and_then(|node| node.parent_id.as_deref());
    }
    false
}

fn render_map(state: &BranchExplorerState, lines: &mut Vec<String>) {
    for entry_id in visible_entry_ids(state) {
        let Some(node) = state
            .projection
            .nodes
            .iter()
            .find(|node| node.entry_id == entry_id)
        else {
            continue;
        };
        let focus = if node.entry_id == state.focused_entry_id {
            ">"
        } else {
            " "
        };
        let active = if node.is_active_path { "A" } else { "-" };
        let leaf = if node.is_leaf { "L" } else { "N" };
        let bookmark = if node.is_bookmarked { "B" } else { "-" };
        let collapsed = if state.collapsed_entry_ids.contains(&node.entry_id) {
            "C"
        } else {
            "-"
        };
        let indent_count = usize::try_from(node.depth).unwrap_or(usize::MAX);
        let indent = INDENT_UNIT.repeat(indent_count);
        lines.push(format!(
            "{focus}{indent}[{active}{leaf}{bookmark}{collapsed}] {} class={} seq={} children={} labels={}",
            node.entry_id,
            node.entry_class.as_str(),
            node.storage_sequence,
            node.child_count,
            node.labels.join(",")
        ));
    }
}

fn render_read(state: &BranchExplorerState, lines: &mut Vec<String>) {
    if let Some(read) = state.selected_read.as_ref() {
        lines.push(format!(
            "read selected={} conversation_projection={}",
            read.selected_entry_id, read.conversation_projection_id
        ));
    } else {
        lines.push(String::from("read unavailable=selected_projection_missing"));
    }
}

fn mode_label(mode: BranchExplorerMode) -> &'static str {
    match mode {
        BranchExplorerMode::Map => "map",
        BranchExplorerMode::Read => "read",
    }
}

fn bound_diagnostic(value: &str) -> String {
    value.chars().take(MAX_DIAGNOSTIC_BYTES).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_runtime::branch_explorer::{
        BranchEntryClass, BranchProjectionNode, BranchProjectionTruncation, ExternalStatePosture,
    };

    const ROOT_SEQUENCE: u32 = 1;
    const INTERNAL_SEQUENCE: u32 = 2;
    const LEFT_SEQUENCE: u32 = 3;
    const RIGHT_SEQUENCE: u32 = 4;

    #[test]
    fn deterministic_map_snapshot_exposes_required_indicators() {
        let state = BranchExplorerState::new(projection());
        let snapshot = render_branch_explorer_snapshot(&state);
        let repeated = render_branch_explorer_snapshot(&state);
        assert_eq!(snapshot, repeated);
        assert!(snapshot.contains("external=shared_world"));
        assert!(snapshot.contains("[ALB-] left"));
        assert!(snapshot.contains("truncated omitted=2"));
    }

    #[test]
    fn collapse_hides_descendants_and_focus_remains_bounded() {
        let state = focus(BranchExplorerState::new(projection()), "internal");
        let collapsed = reduce_branch_explorer(&state, BranchExplorerAction::ToggleCollapse).state;
        let snapshot = render_branch_explorer_snapshot(&collapsed);
        assert!(!snapshot.contains(" left class="));
        assert!(!snapshot.contains(" right class="));
        let moved = reduce_branch_explorer(&collapsed, BranchExplorerAction::FocusNext).state;
        assert_eq!(moved.focused_entry_id, "internal");
    }

    #[test]
    fn internal_activation_blocks_before_host_dispatch() {
        let state = focus(BranchExplorerState::new(projection()), "internal");
        let reduced = reduce_branch_explorer(&state, BranchExplorerAction::Activate);
        assert!(reduced.intent.is_none());
        assert!(
            reduced
                .state
                .diagnostic
                .expect("diagnostic")
                .contains("NonLeaf")
        );
    }

    #[test]
    fn comparison_selects_two_distinct_targets() {
        let left = focus(BranchExplorerState::new(projection()), "left");
        let selected = reduce_branch_explorer(&left, BranchExplorerAction::Compare).state;
        let right = focus(selected, "right");
        let compared = reduce_branch_explorer(&right, BranchExplorerAction::Compare);
        assert_eq!(
            compared.intent,
            Some(BranchExplorerIntent::Compare {
                left_entry_id: String::from("left"),
                right_entry_id: String::from("right"),
            })
        );
    }

    #[test]
    fn refresh_and_host_failure_preserve_validated_projection() {
        let state = BranchExplorerState::new(projection());
        let refresh = reduce_branch_explorer(&state, BranchExplorerAction::Refresh);
        assert!(refresh.state.awaiting_host);
        let failed = reduce_branch_explorer(
            &refresh.state,
            BranchExplorerAction::HostFailed {
                diagnostic: String::from("host rejected refresh"),
            },
        );
        assert_eq!(failed.state.projection, state.projection);
        assert!(failed.state.stale);
    }

    #[test]
    fn incompatible_refresh_does_not_replace_projection() {
        let state = BranchExplorerState::new(projection());
        let mut foreign = projection();
        foreign.session_id = String::from("foreign");
        let reduced = reduce_branch_explorer(
            &state,
            BranchExplorerAction::ApplyProjection(Box::new(foreign)),
        );
        assert_eq!(reduced.state.projection, state.projection);
        assert!(reduced.state.stale);
    }

    #[test]
    fn host_port_failure_is_returned_without_state_mutation() {
        let mut host = FailingHost;
        let intent = BranchExplorerIntent::Inspect {
            target_entry_id: String::from("left"),
        };
        assert_eq!(
            dispatch_branch_explorer_intent(&mut host, &intent),
            Err("injected")
        );
    }

    fn focus(mut state: BranchExplorerState, entry_id: &str) -> BranchExplorerState {
        state.focused_entry_id = String::from(entry_id);
        state
    }

    fn projection() -> BranchProjectionSnapshot {
        BranchProjectionSnapshot {
            schema: String::from("schema"),
            projection_id: String::from("projection"),
            policy_id: String::from("policy"),
            session_id: String::from("session"),
            revision_id: String::from("revision"),
            ledger_generation: 1,
            active_leaf_id: String::from("left"),
            external_state: ExternalStatePosture::SharedWorld,
            isolation_evidence_id: None,
            nodes: vec![
                node("root", None, ROOT_SEQUENCE, 0, false, true, false),
                node(
                    "internal",
                    Some("root"),
                    INTERNAL_SEQUENCE,
                    1,
                    false,
                    true,
                    false,
                ),
                node("left", Some("internal"), LEFT_SEQUENCE, 2, true, true, true),
                node(
                    "right",
                    Some("internal"),
                    RIGHT_SEQUENCE,
                    2,
                    true,
                    false,
                    false,
                ),
            ],
            presentation_order: vec![
                String::from("root"),
                String::from("internal"),
                String::from("left"),
                String::from("right"),
            ],
            truncation: BranchProjectionTruncation {
                omitted_node_count: 2,
                node_limit: 4,
            },
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn node(
        entry_id: &str,
        parent_id: Option<&str>,
        storage_sequence: u32,
        depth: u32,
        is_leaf: bool,
        is_active_path: bool,
        is_bookmarked: bool,
    ) -> BranchProjectionNode {
        BranchProjectionNode {
            entry_id: String::from(entry_id),
            parent_id: parent_id.map(String::from),
            entry_class: BranchEntryClass::Assistant,
            storage_sequence,
            depth,
            child_count: u32::from(!is_leaf),
            is_leaf,
            is_active_path,
            is_bookmarked,
            labels: Vec::new(),
            artifact_reference_ids: Vec::new(),
            effect_receipt_ids: Vec::new(),
            model_profile_id: None,
            usage_units: None,
        }
    }

    struct FailingHost;

    impl BranchExplorerHostPort for FailingHost {
        type Failure = &'static str;

        fn dispatch_branch_intent(
            &mut self,
            _intent: &BranchExplorerIntent,
        ) -> Result<(), Self::Failure> {
            Err("injected")
        }
    }
}
