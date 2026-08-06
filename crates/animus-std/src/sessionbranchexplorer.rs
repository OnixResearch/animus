//! Generated branch explorer policy loading and deterministic fixture host.

use animus_runtime::branch_explorer::{
    BranchAnnotationPort, BranchAnnotationsInput, BranchContinuationPort, BranchEvidencePort,
    BranchExplorerPolicy, BranchExplorerPolicyInput, BranchExplorerReceipt,
    BranchExplorerSourceInput, BranchOperationPlan, BranchProjectionPort, BranchSelectionPort,
};
use serde::Deserialize;

/// Safe policy configuration error.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionBranchExplorerConfigError {
    Read(std::io::ErrorKind),
    Json,
    Validation(animus_runtime::branch_explorer::BranchExplorerIssue),
}

/// Parse and strictly revalidate generated policy JSON.
///
/// # Errors
/// Returns a bounded JSON or policy error class.
pub fn parse_session_branch_explorer_policy_json(
    input: &str,
) -> Result<BranchExplorerPolicy, SessionBranchExplorerConfigError> {
    let value: GeneratedPolicy =
        serde_json::from_str(input).map_err(|_| SessionBranchExplorerConfigError::Json)?;
    animus_runtime::branch_explorer::validate_branch_explorer_policy(&value.into_input())
        .map_err(|error| SessionBranchExplorerConfigError::Validation(error.issue()))
}

/// Load generated policy from one explicit host path.
///
/// # Errors
/// Returns a bounded read, JSON, or policy error class.
pub fn load_session_branch_explorer_policy(
    path: impl AsRef<std::path::Path>,
) -> Result<BranchExplorerPolicy, SessionBranchExplorerConfigError> {
    let input = std::fs::read_to_string(path)
        .map_err(|error| SessionBranchExplorerConfigError::Read(error.kind()))?;
    parse_session_branch_explorer_policy_json(input.as_str())
}

/// Deterministic host failure class.
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionBranchFixtureFailure {
    StaleRevision,
    StaleActiveLeaf,
    Injected,
    BlockedPlan,
}

/// Disposable in-memory host for annotation and active-leaf contract tests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionBranchMemoryHost {
    source: BranchExplorerSourceInput,
    receipts: Vec<String>,
    dispatched_operations: Vec<String>,
    fail_next: bool,
}

impl SessionBranchMemoryHost {
    /// Create a fixture from explicit source facts.
    #[must_use]
    pub const fn new(source: BranchExplorerSourceInput) -> Self {
        Self {
            source,
            receipts: Vec::new(),
            dispatched_operations: Vec::new(),
            fail_next: false,
        }
    }

    /// Inject one failure before the next mutating call.
    pub fn fail_next(&mut self) {
        self.fail_next = true;
    }

    /// Return the current active leaf.
    #[must_use]
    pub fn active_leaf_id(&self) -> &str {
        self.source.active_leaf_id.as_str()
    }

    /// Return current annotation facts.
    #[must_use]
    pub const fn annotations(&self) -> &BranchAnnotationsInput {
        &self.source.annotations
    }

    /// Return recorded receipt identities.
    #[must_use]
    pub fn receipt_ids(&self) -> &[String] {
        self.receipts.as_slice()
    }

    /// Return dispatched append-only operation identities.
    #[must_use]
    pub fn dispatched_operation_ids(&self) -> &[String] {
        self.dispatched_operations.as_slice()
    }

    fn take_failure(&mut self) -> Result<(), SessionBranchFixtureFailure> {
        if self.fail_next {
            self.fail_next = false;
            return Err(SessionBranchFixtureFailure::Injected);
        }
        Ok(())
    }
}

impl BranchProjectionPort for SessionBranchMemoryHost {
    type Failure = SessionBranchFixtureFailure;

    fn load_branch_source(&self) -> Result<BranchExplorerSourceInput, Self::Failure> {
        Ok(self.source.clone())
    }
}

impl BranchAnnotationPort for SessionBranchMemoryHost {
    type Failure = SessionBranchFixtureFailure;

    fn compare_and_set_annotations(
        &mut self,
        expected_revision_id: &str,
        annotations: &BranchAnnotationsInput,
    ) -> Result<(), Self::Failure> {
        self.take_failure()?;
        if self.source.revision_id != expected_revision_id
            || annotations.revision_id != expected_revision_id
        {
            return Err(SessionBranchFixtureFailure::StaleRevision);
        }
        self.source.annotations = annotations.clone();
        Ok(())
    }
}

impl BranchSelectionPort for SessionBranchMemoryHost {
    type Failure = SessionBranchFixtureFailure;

    fn compare_and_set_active_leaf(
        &mut self,
        expected_revision_id: &str,
        expected_active_leaf_id: &str,
        target_leaf_id: &str,
    ) -> Result<(), Self::Failure> {
        self.take_failure()?;
        if self.source.revision_id != expected_revision_id {
            return Err(SessionBranchFixtureFailure::StaleRevision);
        }
        if self.source.active_leaf_id != expected_active_leaf_id {
            return Err(SessionBranchFixtureFailure::StaleActiveLeaf);
        }
        self.source.active_leaf_id = String::from(target_leaf_id);
        Ok(())
    }
}

impl BranchContinuationPort for SessionBranchMemoryHost {
    type Failure = SessionBranchFixtureFailure;

    fn dispatch_append_only_fork(
        &mut self,
        plan: &BranchOperationPlan,
    ) -> Result<(), Self::Failure> {
        self.take_failure()?;
        if plan.status != animus_runtime::branch_explorer::BranchPlanStatus::Admitted
            || plan.append_only_branch_operation
                != Some(animus_runtime::SessionBranchOperation::ForkFromMessage)
        {
            return Err(SessionBranchFixtureFailure::BlockedPlan);
        }
        self.dispatched_operations.push(plan.operation_id.clone());
        Ok(())
    }
}

impl BranchEvidencePort for SessionBranchMemoryHost {
    type Failure = SessionBranchFixtureFailure;

    fn record_branch_receipt(
        &mut self,
        receipt: &BranchExplorerReceipt,
    ) -> Result<(), Self::Failure> {
        self.take_failure()?;
        self.receipts.push(receipt.receipt_id.clone());
        Ok(())
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedPolicy {
    schema: String,
    max_nodes: u32,
    max_depth: u32,
    max_label_bytes: u32,
    max_labels_per_node: u32,
    max_ordered_children: u32,
    max_comparison_suffix: u32,
    max_safe_references: u32,
    redaction_first: bool,
    one_replay_parent: bool,
    annotations_are_sidecars: bool,
    external_state_required: bool,
    destructive_actions_allowed: bool,
    receipt_metadata_only: bool,
    non_claims: Vec<String>,
}

impl GeneratedPolicy {
    fn into_input(self) -> BranchExplorerPolicyInput {
        BranchExplorerPolicyInput {
            schema: self.schema,
            max_nodes: self.max_nodes,
            max_depth: self.max_depth,
            max_label_bytes: self.max_label_bytes,
            max_labels_per_node: self.max_labels_per_node,
            max_ordered_children: self.max_ordered_children,
            max_comparison_suffix: self.max_comparison_suffix,
            max_safe_references: self.max_safe_references,
            redaction_first: self.redaction_first,
            one_replay_parent: self.one_replay_parent,
            annotations_are_sidecars: self.annotations_are_sidecars,
            external_state_required: self.external_state_required,
            destructive_actions_allowed: self.destructive_actions_allowed,
            receipt_metadata_only: self.receipt_metadata_only,
            non_claims: self.non_claims,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_runtime::branch_explorer::{
        BranchEntryClass, BranchExplorerSourceInput, BranchLedgerNodeInput,
        BranchSiblingOrderInput, ExternalStatePosture,
    };

    const ROOT_SEQUENCE: u32 = 1;
    const LEAF_SEQUENCE: u32 = 2;

    #[test]
    fn generated_policy_parses_and_malformed_json_fails() {
        assert!(
            parse_session_branch_explorer_policy_json(include_str!(
                "../../../config/generated/session-branch-explorer-policy.json"
            ))
            .is_ok()
        );
        assert_eq!(
            parse_session_branch_explorer_policy_json("{").expect_err("malformed"),
            SessionBranchExplorerConfigError::Json
        );
    }

    #[test]
    fn host_compare_and_set_updates_only_exact_current_facts() {
        let mut host = SessionBranchMemoryHost::new(source());
        host.compare_and_set_active_leaf("revision", "leaf", "root")
            .expect("exact update");
        assert_eq!(host.active_leaf_id(), "root");
        assert_eq!(
            host.compare_and_set_active_leaf("stale", "root", "leaf"),
            Err(SessionBranchFixtureFailure::StaleRevision)
        );
    }

    #[test]
    fn injected_failure_preserves_annotations() {
        let mut host = SessionBranchMemoryHost::new(source());
        let before = host.annotations().clone();
        let updated = BranchAnnotationsInput {
            revision_id: String::from("revision"),
            bookmarks: Vec::new(),
            labels: Vec::new(),
            sibling_orders: vec![BranchSiblingOrderInput {
                parent_entry_id: String::from("root"),
                ordered_child_ids: vec![String::from("leaf")],
            }],
        };
        host.fail_next();
        assert_eq!(
            host.compare_and_set_annotations("revision", &updated),
            Err(SessionBranchFixtureFailure::Injected)
        );
        assert_eq!(host.annotations(), &before);
    }

    fn source() -> BranchExplorerSourceInput {
        BranchExplorerSourceInput {
            session_id: String::from("session"),
            revision_id: String::from("revision"),
            ledger_generation: 1,
            active_leaf_id: String::from("leaf"),
            nodes: vec![
                node("root", None, ROOT_SEQUENCE, false),
                node("leaf", Some("root"), LEAF_SEQUENCE, true),
            ],
            annotations: BranchAnnotationsInput {
                revision_id: String::from("revision"),
                bookmarks: Vec::new(),
                labels: Vec::new(),
                sibling_orders: Vec::new(),
            },
            external_state: Some(ExternalStatePosture::ReadOnly),
            isolation_evidence_id: None,
        }
    }

    fn node(
        entry_id: &str,
        parent_id: Option<&str>,
        storage_sequence: u32,
        claimed_leaf: bool,
    ) -> BranchLedgerNodeInput {
        BranchLedgerNodeInput {
            entry_id: String::from(entry_id),
            parent_id: parent_id.map(String::from),
            extra_parent_id: None,
            entry_class: BranchEntryClass::System,
            storage_sequence,
            claimed_leaf,
            artifact_reference_ids: Vec::new(),
            effect_receipt_ids: Vec::new(),
            model_profile_id: None,
            usage_units: None,
        }
    }
}
