//! Pure adapter core over the `universal-weave` DAG weave.

use alloc::string::String;
use alloc::vec::Vec;

use universal_weave::hashbrown::DefaultHashBuilder;
use universal_weave::independent::{IndependentNode, IndependentWeave as DagWeave};
use universal_weave::indexmap::IndexSet;
use universal_weave::{
    ActivePathWeave, BookmarkableWeave, DiscreteWeave, IndependentWeave as IndependentWeaveOps,
    Node, SemiIndependentWeave, Weave,
};

use crate::content::WeaveNodeContent;
use crate::error::WeaveAdapterError;
use crate::policy::WeaveAdapterPolicy;
use crate::receipt::{WeaveCounts, WeaveOperationKind, WeaveOperationReceipt};
use crate::snapshot::{WeaveSnapshot, WeaveSnapshotEntry, content_digest};

/// The node key type used by the adapter.
type Key = u128;
/// The hasher used by the wrapped weave.
type Hasher = DefaultHashBuilder;
/// The wrapped DAG weave.
type Dag = DagWeave<Key, WeaveNodeContent, (), Hasher>;
/// The wrapped DAG node.
type DagNode = IndependentNode<Key, WeaveNodeContent, Hasher>;

/// A pure, bounded adapter over the `universal-weave` DAG weave.
///
/// The adapter owns one weave instance and a validated policy. Every public
/// mutation validates the request against the policy before touching the
/// weave, and every admitted mutation returns a metadata-only receipt. The
/// adapter never performs I/O and never touches the neutral session ledger.
#[derive(Debug, Clone)]
pub struct WeaveAdapter {
    policy: WeaveAdapterPolicy,
    policy_identity: String,
    weave: Dag,
}

impl WeaveAdapter {
    /// Creates an empty adapter over a validated policy.
    #[must_use]
    pub fn new(policy: WeaveAdapterPolicy) -> Self {
        let policy_identity = policy.identity();
        Self {
            policy,
            policy_identity,
            weave: Dag::default(),
        }
    }

    /// Returns the validated policy.
    #[must_use]
    pub fn policy(&self) -> WeaveAdapterPolicy {
        self.policy
    }

    /// Returns the policy identity bound into receipts.
    #[must_use]
    pub fn policy_identity(&self) -> &str {
        &self.policy_identity
    }

    /// Returns the number of nodes in the weave.
    #[must_use]
    pub fn len(&self) -> usize {
        self.weave.len()
    }

    /// Returns `true` when the weave holds no nodes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.weave.is_empty()
    }

    /// Returns `true` when a node with the identifier exists.
    #[must_use]
    pub fn contains(&self, id: u128) -> bool {
        self.weave.contains(&id)
    }

    /// Returns the text of one node, when it exists.
    ///
    /// This is a host-facing read, not evidence. Receipts and snapshots never
    /// contain this text.
    #[must_use]
    pub fn node_text(&self, id: u128) -> Option<&str> {
        self.weave.get(&id).map(|node| node.contents().text())
    }

    /// Runs the upstream internal consistency check.
    #[must_use]
    pub fn check_consistency(&self) -> bool {
        self.weave.validate()
    }

    /// Returns the active path from the deepest active node to a root.
    #[must_use]
    pub fn active_path(&mut self) -> Vec<u128> {
        let mut path = Vec::new();
        self.weave.get_active_path(&mut path);
        path
    }

    /// Adds a node with caller-supplied identifier, parents, and text.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the policy node limit is reached, the text
    /// exceeds the content bound, the fan-in bound is exceeded, the
    /// identifier already exists, a parent is missing, or the upstream weave
    /// rejects the insertion. The weave stays unchanged on error.
    pub fn add_node(
        &mut self,
        id: u128,
        parents: &[u128],
        text: &str,
    ) -> Result<WeaveOperationReceipt, WeaveAdapterError> {
        if self.weave.len() >= self.policy.node_limit() {
            return Err(WeaveAdapterError::NodeLimitReached {
                limit: self.policy.node_limit(),
            });
        }
        self.check_content(text)?;
        self.check_parents(parents)?;
        if self.weave.contains(&id) {
            return Err(WeaveAdapterError::NodeExists { id });
        }
        for parent in parents {
            if !self.weave.contains(parent) {
                return Err(WeaveAdapterError::ParentMissing { id: *parent });
            }
        }
        let node = DagNode {
            id,
            from: parents.iter().copied().collect(),
            to: IndexSet::default(),
            active: false,
            bookmarked: false,
            contents: WeaveNodeContent::new(String::from(text)),
        };
        if !self.weave.insert(node) {
            return Err(WeaveAdapterError::OperationRejected { id });
        }
        Ok(self.receipt(WeaveOperationKind::Add, id))
    }

    /// Replaces the text of one node.
    ///
    /// # Errors
    ///
    /// Returns [`WeaveAdapterError::NodeMissing`] when the node does not exist
    /// or [`WeaveAdapterError::ContentTooLarge`] when the text exceeds the
    /// content bound. The weave stays unchanged on error.
    pub fn edit_node(
        &mut self,
        id: u128,
        text: &str,
    ) -> Result<WeaveOperationReceipt, WeaveAdapterError> {
        if !self.weave.contains(&id) {
            return Err(WeaveAdapterError::NodeMissing { id });
        }
        self.check_content(text)?;
        let replacement = WeaveNodeContent::new(String::from(text));
        let updated = self
            .weave
            .get_contents_mut(&id, |contents| *contents = replacement);
        if updated.is_none() {
            return Err(WeaveAdapterError::NodeMissing { id });
        }
        Ok(self.receipt(WeaveOperationKind::Edit, id))
    }

    /// Splits one node at a byte index, creating a child with `new_id`.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the node is missing, the new identifier
    /// already exists, the policy node limit is reached, or the split point
    /// is outside the content or not on a UTF-8 boundary. The weave stays
    /// unchanged on error.
    pub fn split_node(
        &mut self,
        id: u128,
        at: usize,
        new_id: u128,
    ) -> Result<WeaveOperationReceipt, WeaveAdapterError> {
        if !self.weave.contains(&id) {
            return Err(WeaveAdapterError::NodeMissing { id });
        }
        if self.weave.contains(&new_id) {
            return Err(WeaveAdapterError::NodeExists { id: new_id });
        }
        if self.weave.len() >= self.policy.node_limit() {
            return Err(WeaveAdapterError::NodeLimitReached {
                limit: self.policy.node_limit(),
            });
        }
        if !self.weave.split(&id, at, new_id) {
            return Err(WeaveAdapterError::SplitRejected { id });
        }
        Ok(self.receipt(WeaveOperationKind::Split, new_id))
    }

    /// Merges one node into its parent, returning the merged identifier.
    ///
    /// # Errors
    ///
    /// Returns [`WeaveAdapterError::NodeMissing`] when the node does not exist
    /// or [`WeaveAdapterError::MergeRejected`] when the node has no single
    /// parent to merge into. The weave stays unchanged on error.
    pub fn merge_with_parent(
        &mut self,
        id: u128,
    ) -> Result<WeaveOperationReceipt, WeaveAdapterError> {
        if !self.weave.contains(&id) {
            return Err(WeaveAdapterError::NodeMissing { id });
        }
        let merged = self
            .weave
            .merge_with_parent(&id)
            .ok_or(WeaveAdapterError::MergeRejected { id })?;
        Ok(self.receipt(WeaveOperationKind::Merge, merged))
    }

    /// Moves one node to a new set of parents.
    ///
    /// # Errors
    ///
    /// Returns a typed error when the node is missing, the fan-in bound is
    /// exceeded, a parent is missing, or the move would break consistency
    /// (for example by creating a cycle). The weave stays unchanged on error.
    pub fn move_node(
        &mut self,
        id: u128,
        new_parents: &[u128],
    ) -> Result<WeaveOperationReceipt, WeaveAdapterError> {
        if !self.weave.contains(&id) {
            return Err(WeaveAdapterError::NodeMissing { id });
        }
        self.check_parents(new_parents)?;
        for parent in new_parents {
            if !self.weave.contains(parent) {
                return Err(WeaveAdapterError::ParentMissing { id: *parent });
            }
        }
        if !self.weave.move_to(&id, new_parents) {
            return Err(WeaveAdapterError::MoveRejected { id });
        }
        Ok(self.receipt(WeaveOperationKind::Move, id))
    }

    /// Sets the bookmark flag of one node.
    ///
    /// # Errors
    ///
    /// Returns [`WeaveAdapterError::NodeMissing`] when the node does not exist
    /// or [`WeaveAdapterError::OperationRejected`] when the upstream weave
    /// rejects the change.
    pub fn set_bookmarked(
        &mut self,
        id: u128,
        value: bool,
    ) -> Result<WeaveOperationReceipt, WeaveAdapterError> {
        if !self.weave.contains(&id) {
            return Err(WeaveAdapterError::NodeMissing { id });
        }
        if !self.weave.set_bookmarked(&id, value) {
            return Err(WeaveAdapterError::OperationRejected { id });
        }
        Ok(self.receipt(WeaveOperationKind::Bookmark, id))
    }

    /// Sets the active flag of one node.
    ///
    /// # Errors
    ///
    /// Returns [`WeaveAdapterError::NodeMissing`] when the node does not exist
    /// or [`WeaveAdapterError::OperationRejected`] when the upstream weave
    /// rejects the change.
    pub fn set_active(
        &mut self,
        id: u128,
        value: bool,
    ) -> Result<WeaveOperationReceipt, WeaveAdapterError> {
        if !self.weave.contains(&id) {
            return Err(WeaveAdapterError::NodeMissing { id });
        }
        if !self.weave.set_active(&id, value) {
            return Err(WeaveAdapterError::OperationRejected { id });
        }
        Ok(self.receipt(WeaveOperationKind::Activate, id))
    }

    /// Removes one node.
    ///
    /// # Errors
    ///
    /// Returns [`WeaveAdapterError::NodeMissing`] when the node does not exist
    /// or [`WeaveAdapterError::OperationRejected`] when the upstream weave
    /// rejects the removal.
    pub fn remove_node(&mut self, id: u128) -> Result<WeaveOperationReceipt, WeaveAdapterError> {
        if !self.weave.contains(&id) {
            return Err(WeaveAdapterError::NodeMissing { id });
        }
        if self.weave.remove(&id).is_none() {
            return Err(WeaveAdapterError::OperationRejected { id });
        }
        Ok(self.receipt(WeaveOperationKind::Remove, id))
    }

    /// Projects a redacted, deterministically ordered snapshot of the weave.
    #[must_use]
    pub fn snapshot(&mut self) -> WeaveSnapshot {
        let mut ids = Vec::new();
        self.weave.get_ordered_identifiers(&mut ids);
        let mut entries = Vec::with_capacity(ids.len());
        for id in ids {
            if let Some(node) = self.weave.get(&id) {
                entries.push(WeaveSnapshotEntry {
                    id,
                    parent_count: node.from().len(),
                    child_count: node.to().len(),
                    active: node.is_active(),
                    bookmarked: self.weave.contains_bookmark(&id),
                    content_bytes: node.contents().len(),
                    content_digest: content_digest(node.contents().text()),
                });
            }
        }
        WeaveSnapshot::new(entries)
    }

    fn check_content(&self, text: &str) -> Result<(), WeaveAdapterError> {
        if text.len() > self.policy.content_byte_limit() {
            return Err(WeaveAdapterError::ContentTooLarge {
                limit: self.policy.content_byte_limit(),
                actual: text.len(),
            });
        }
        Ok(())
    }

    fn check_parents(&self, parents: &[u128]) -> Result<(), WeaveAdapterError> {
        if parents.len() > self.policy.parent_fan_in_limit() {
            return Err(WeaveAdapterError::TooManyParents {
                limit: self.policy.parent_fan_in_limit(),
                actual: parents.len(),
            });
        }
        Ok(())
    }

    fn receipt(&self, operation: WeaveOperationKind, target: u128) -> WeaveOperationReceipt {
        let counts = WeaveCounts {
            nodes: self.weave.len(),
            roots: self.weave.roots().len(),
            active: self.weave.active().len(),
            bookmarked: self.weave.bookmarks().len(),
        };
        WeaveOperationReceipt::new(operation, target, counts, self.policy_identity.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROOT: u128 = 1;
    const CHILD: u128 = 2;
    const GRANDCHILD: u128 = 3;
    const EXTRA: u128 = 4;
    const MISSING: u128 = 99;

    fn policy() -> WeaveAdapterPolicy {
        WeaveAdapterPolicy::new(8, 64, 2).expect("policy")
    }

    fn adapter() -> WeaveAdapter {
        WeaveAdapter::new(policy())
    }

    fn adapter_with_root() -> WeaveAdapter {
        let mut adapter = adapter();
        adapter
            .add_node(ROOT, &[], "hello world")
            .expect("add root");
        adapter
    }

    #[test]
    fn add_root_node_reports_counts() {
        let mut adapter = adapter();
        let receipt = adapter.add_node(ROOT, &[], "hello").expect("add");
        assert_eq!(receipt.operation(), WeaveOperationKind::Add);
        assert_eq!(receipt.target(), ROOT);
        assert_eq!(receipt.counts().nodes, 1);
        assert_eq!(receipt.counts().roots, 1);
        assert!(receipt.has_valid_identities());
        assert_eq!(adapter.node_text(ROOT), Some("hello"));
        assert!(adapter.check_consistency());
    }

    #[test]
    fn add_child_links_parent() {
        let mut adapter = adapter_with_root();
        adapter
            .add_node(CHILD, &[ROOT], "child text")
            .expect("add child");
        let snapshot = adapter.snapshot();
        let child = snapshot
            .entries
            .iter()
            .find(|entry| entry.id == CHILD)
            .expect("child entry");
        assert_eq!(child.parent_count, 1);
        let root = snapshot
            .entries
            .iter()
            .find(|entry| entry.id == ROOT)
            .expect("root entry");
        assert_eq!(root.child_count, 1);
        assert!(adapter.check_consistency());
    }

    #[test]
    fn edit_replaces_text() {
        let mut adapter = adapter_with_root();
        let receipt = adapter.edit_node(ROOT, "replacement").expect("edit");
        assert_eq!(receipt.operation(), WeaveOperationKind::Edit);
        assert_eq!(adapter.node_text(ROOT), Some("replacement"));
    }

    #[test]
    fn split_preserves_content() {
        let mut adapter = adapter_with_root();
        let receipt = adapter.split_node(ROOT, 5, CHILD).expect("split");
        assert_eq!(receipt.operation(), WeaveOperationKind::Split);
        assert_eq!(adapter.node_text(ROOT), Some("hello"));
        assert_eq!(adapter.node_text(CHILD), Some(" world"));
        assert!(adapter.check_consistency());
    }

    #[test]
    fn split_on_multibyte_boundary() {
        let mut adapter = adapter();
        adapter.add_node(ROOT, &[], "aéb").expect("add");
        adapter.split_node(ROOT, 3, CHILD).expect("split");
        assert_eq!(adapter.node_text(ROOT), Some("aé"));
        assert_eq!(adapter.node_text(CHILD), Some("b"));
    }

    #[test]
    fn merge_concatenates_with_parent() {
        let mut adapter = adapter_with_root();
        adapter.split_node(ROOT, 5, CHILD).expect("split");
        let receipt = adapter.merge_with_parent(CHILD).expect("merge");
        assert_eq!(receipt.operation(), WeaveOperationKind::Merge);
        assert_eq!(receipt.target(), ROOT);
        assert_eq!(adapter.node_text(ROOT), Some("hello world"));
        assert!(!adapter.contains(CHILD));
    }

    #[test]
    fn move_reparents_node() {
        let mut adapter = adapter_with_root();
        adapter.add_node(CHILD, &[ROOT], "child").expect("child");
        adapter
            .add_node(GRANDCHILD, &[], "second root")
            .expect("second root");
        let receipt = adapter.move_node(CHILD, &[GRANDCHILD]).expect("move");
        assert_eq!(receipt.operation(), WeaveOperationKind::Move);
        let snapshot = adapter.snapshot();
        let child = snapshot
            .entries
            .iter()
            .find(|entry| entry.id == CHILD)
            .expect("child entry");
        assert_eq!(child.parent_count, 1);
        assert!(adapter.check_consistency());
    }

    #[test]
    fn bookmark_and_activate_flags_report() {
        let mut adapter = adapter_with_root();
        adapter.add_node(CHILD, &[ROOT], "child").expect("child");
        let receipt = adapter.set_bookmarked(CHILD, true).expect("bookmark");
        assert_eq!(receipt.counts().bookmarked, 1);
        let receipt = adapter.set_active(CHILD, true).expect("activate");
        assert_eq!(receipt.operation(), WeaveOperationKind::Activate);
        assert!(receipt.counts().active >= 1);
        let path = adapter.active_path();
        assert!(path.contains(&CHILD));
    }

    #[test]
    fn remove_detaches_node() {
        let mut adapter = adapter_with_root();
        adapter.add_node(CHILD, &[ROOT], "child").expect("child");
        let receipt = adapter.remove_node(CHILD).expect("remove");
        assert_eq!(receipt.operation(), WeaveOperationKind::Remove);
        assert_eq!(receipt.counts().nodes, 1);
        assert!(!adapter.contains(CHILD));
        assert!(adapter.check_consistency());
    }

    #[test]
    fn receipts_are_deterministic() {
        let mut first = adapter();
        let mut second = adapter();
        let run = |adapter: &mut WeaveAdapter| {
            adapter.add_node(ROOT, &[], "hello").expect("add");
            adapter
                .add_node(CHILD, &[ROOT], "child")
                .expect("add child");
            adapter.set_bookmarked(CHILD, true).expect("bookmark")
        };
        let one = run(&mut first);
        let two = run(&mut second);
        assert_eq!(one.receipt_identity(), two.receipt_identity());
    }

    #[test]
    fn snapshot_is_redacted_and_stable() {
        let mut adapter = adapter_with_root();
        adapter.add_node(CHILD, &[ROOT], "child").expect("child");
        let first = adapter.snapshot();
        let second = adapter.snapshot();
        assert_eq!(first.topology_digest, second.topology_digest);
        assert_eq!(first.entries.len(), 2);
        for entry in &first.entries {
            assert!(entry.content_digest.len() == 64);
        }
    }

    #[test]
    fn zero_policy_bounds_are_rejected() {
        assert!(matches!(
            WeaveAdapterPolicy::new(0, 64, 2),
            Err(WeaveAdapterError::InvalidPolicy {
                field: "node_limit"
            })
        ));
        assert!(matches!(
            WeaveAdapterPolicy::new(8, 0, 2),
            Err(WeaveAdapterError::InvalidPolicy {
                field: "content_bytes"
            })
        ));
        assert!(matches!(
            WeaveAdapterPolicy::new(8, 64, 0),
            Err(WeaveAdapterError::InvalidPolicy {
                field: "parent_fan_in"
            })
        ));
    }

    #[test]
    fn node_limit_is_enforced() {
        let mut adapter = WeaveAdapter::new(WeaveAdapterPolicy::new(1, 64, 2).expect("policy"));
        adapter.add_node(ROOT, &[], "one").expect("add");
        let before = adapter.snapshot();
        assert!(matches!(
            adapter.add_node(CHILD, &[], "two"),
            Err(WeaveAdapterError::NodeLimitReached { limit: 1 })
        ));
        assert_eq!(adapter.snapshot().topology_digest, before.topology_digest);
    }

    #[test]
    fn content_bound_is_enforced() {
        let mut adapter = adapter();
        let oversized = "x".repeat(65);
        assert!(matches!(
            adapter.add_node(ROOT, &[], &oversized),
            Err(WeaveAdapterError::ContentTooLarge {
                limit: 64,
                actual: 65
            })
        ));
        assert!(adapter.is_empty());
    }

    #[test]
    fn parent_fan_in_bound_is_enforced() {
        let mut adapter = adapter_with_root();
        adapter.add_node(CHILD, &[], "two").expect("child");
        adapter
            .add_node(GRANDCHILD, &[], "three")
            .expect("grandchild");
        assert!(matches!(
            adapter.add_node(EXTRA, &[ROOT, CHILD, GRANDCHILD], "extra"),
            Err(WeaveAdapterError::TooManyParents {
                limit: 2,
                actual: 3
            })
        ));
        assert!(!adapter.contains(EXTRA));
    }

    #[test]
    fn duplicate_identifier_is_rejected() {
        let mut adapter = adapter_with_root();
        assert!(matches!(
            adapter.add_node(ROOT, &[], "again"),
            Err(WeaveAdapterError::NodeExists { id: ROOT })
        ));
        assert_eq!(adapter.node_text(ROOT), Some("hello world"));
    }

    #[test]
    fn missing_parent_is_rejected() {
        let mut adapter = adapter_with_root();
        assert!(matches!(
            adapter.add_node(CHILD, &[MISSING], "orphan"),
            Err(WeaveAdapterError::ParentMissing { id: MISSING })
        ));
        assert!(!adapter.contains(CHILD));
    }

    #[test]
    fn missing_node_operations_are_rejected() {
        let mut adapter = adapter_with_root();
        assert!(matches!(
            adapter.edit_node(MISSING, "text"),
            Err(WeaveAdapterError::NodeMissing { id: MISSING })
        ));
        assert!(matches!(
            adapter.split_node(MISSING, 1, CHILD),
            Err(WeaveAdapterError::NodeMissing { id: MISSING })
        ));
        assert!(matches!(
            adapter.merge_with_parent(MISSING),
            Err(WeaveAdapterError::NodeMissing { id: MISSING })
        ));
        assert!(matches!(
            adapter.move_node(MISSING, &[ROOT]),
            Err(WeaveAdapterError::NodeMissing { id: MISSING })
        ));
        assert!(matches!(
            adapter.set_bookmarked(MISSING, true),
            Err(WeaveAdapterError::NodeMissing { id: MISSING })
        ));
        assert!(matches!(
            adapter.set_active(MISSING, true),
            Err(WeaveAdapterError::NodeMissing { id: MISSING })
        ));
        assert!(matches!(
            adapter.remove_node(MISSING),
            Err(WeaveAdapterError::NodeMissing { id: MISSING })
        ));
    }

    #[test]
    fn out_of_range_splits_are_rejected_without_mutation() {
        let mut adapter = adapter_with_root();
        let before = adapter.snapshot();
        assert!(matches!(
            adapter.split_node(ROOT, 0, CHILD),
            Err(WeaveAdapterError::SplitRejected { id: ROOT })
        ));
        assert!(matches!(
            adapter.split_node(ROOT, 11, CHILD),
            Err(WeaveAdapterError::SplitRejected { id: ROOT })
        ));
        assert_eq!(adapter.snapshot().topology_digest, before.topology_digest);
        assert_eq!(adapter.node_text(ROOT), Some("hello world"));
    }

    #[test]
    fn non_boundary_split_is_rejected_without_mutation() {
        let mut adapter = adapter();
        adapter.add_node(ROOT, &[], "aéb").expect("add");
        let before = adapter.snapshot();
        assert!(matches!(
            adapter.split_node(ROOT, 2, CHILD),
            Err(WeaveAdapterError::SplitRejected { id: ROOT })
        ));
        assert_eq!(adapter.snapshot().topology_digest, before.topology_digest);
    }

    #[test]
    fn split_with_existing_new_identifier_is_rejected() {
        let mut adapter = adapter_with_root();
        adapter.add_node(CHILD, &[ROOT], "child").expect("child");
        assert!(matches!(
            adapter.split_node(ROOT, 5, CHILD),
            Err(WeaveAdapterError::NodeExists { id: CHILD })
        ));
    }

    #[test]
    fn merge_without_parent_is_rejected() {
        let mut adapter = adapter_with_root();
        assert!(matches!(
            adapter.merge_with_parent(ROOT),
            Err(WeaveAdapterError::MergeRejected { id: ROOT })
        ));
        assert_eq!(adapter.node_text(ROOT), Some("hello world"));
    }

    #[test]
    fn cyclic_move_is_rejected_without_mutation() {
        let mut adapter = adapter_with_root();
        adapter.add_node(CHILD, &[ROOT], "child").expect("child");
        let before = adapter.snapshot();
        assert!(matches!(
            adapter.move_node(ROOT, &[CHILD]),
            Err(WeaveAdapterError::MoveRejected { id: ROOT })
        ));
        assert_eq!(adapter.snapshot().topology_digest, before.topology_digest);
    }

    #[test]
    fn receipts_and_snapshots_never_contain_node_text() {
        let marker = "prompt:do-not-leak sk-test-token";
        let mut adapter = adapter();
        let receipt = adapter.add_node(ROOT, &[], marker).expect("add");
        assert!(!receipt.receipt_identity().contains(marker));
        assert!(!receipt.policy_identity().contains(marker));
        let snapshot = adapter.snapshot();
        for entry in &snapshot.entries {
            assert!(!entry.content_digest.contains(marker));
        }
        let debug = alloc::format!("{receipt:?} {snapshot:?}");
        assert!(!debug.contains(marker));
        assert!(!debug.contains("sk-test-token"));
    }
}
