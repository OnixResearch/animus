/// Borrowed workspace-edit operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EditOperationDraft<'a> {
    /// Replace one inclusive tagged line span.
    Replace {
        /// First selected tag.
        start_tag: &'a str,
        /// Last selected tag.
        end_tag: &'a str,
        /// Logical LF-delimited replacement text.
        replacement: &'a str,
    },
    /// Delete one inclusive tagged line span.
    Delete {
        /// First selected tag.
        start_tag: &'a str,
        /// Last selected tag.
        end_tag: &'a str,
    },
    /// Insert text at one explicit visible tagged boundary.
    Insert {
        /// Adjacent line tag.
        line_tag: &'a str,
        /// Selected boundary side.
        side: crate::model::BoundarySide,
        /// Logical LF-delimited insertion text.
        text: &'a str,
    },
    /// Insert text at an explicitly visible end boundary.
    InsertEnd {
        /// Logical LF-delimited insertion text.
        text: &'a str,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct AppliedOperation {
    pub(crate) start: usize,
    pub(crate) end: usize,
    pub(crate) is_insertion: bool,
    pub(crate) replacement: Vec<u8>,
    pub(crate) kind: &'static str,
    pub(crate) first_line: u32,
    pub(crate) last_line: u32,
    pub(crate) replacement_identity: String,
}

/// Exact deterministic byte-application plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EditPlan {
    pub(crate) snapshot: crate::model::AdmittedSnapshot,
    pub(crate) after_bytes: Vec<u8>,
    pub(crate) after_identity: String,
    pub(crate) plan_identity: String,
    pub(crate) operations: Vec<AppliedOperation>,
    pub(crate) operation_count: u32,
}

impl EditPlan {
    /// Return the admitted snapshot.
    #[must_use]
    pub const fn snapshot(&self) -> &crate::model::AdmittedSnapshot {
        &self.snapshot
    }

    /// Return the exact bytes approved for staging.
    #[must_use]
    pub fn after_bytes(&self) -> &[u8] {
        &self.after_bytes
    }

    /// Return the complete after-content BLAKE3 identity.
    #[must_use]
    pub fn after_identity(&self) -> &str {
        &self.after_identity
    }

    /// Return the canonical edit-plan identity.
    #[must_use]
    pub fn plan_identity(&self) -> &str {
        &self.plan_identity
    }

    /// Return the operation count.
    #[must_use]
    pub const fn operation_count(&self) -> u32 {
        self.operation_count
    }
}
