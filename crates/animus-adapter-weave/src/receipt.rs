//! Metadata-only operation receipts.

use alloc::format;
use alloc::string::String;

/// Domain separator for receipt identities.
const RECEIPT_DOMAIN: &str = "animus_adapter_weave_receipt_v1";
/// Length of a lowercase BLAKE3 hex identity.
const BLAKE3_HEX_CHARS: usize = 64;

/// The class of one admitted weave operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WeaveOperationKind {
    /// A node was added.
    Add,
    /// Node content was replaced.
    Edit,
    /// A node was split into two nodes.
    Split,
    /// A node was merged into its parent.
    Merge,
    /// A node was moved to new parents.
    Move,
    /// The bookmark flag changed.
    Bookmark,
    /// The active flag changed.
    Activate,
    /// A node was removed.
    Remove,
}

impl WeaveOperationKind {
    /// Returns the stable lowercase label of the operation class.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Add => "add",
            Self::Edit => "edit",
            Self::Split => "split",
            Self::Merge => "merge",
            Self::Move => "move",
            Self::Bookmark => "bookmark",
            Self::Activate => "activate",
            Self::Remove => "remove",
        }
    }
}

/// Bounded weave counts observed after one operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WeaveCounts {
    /// Number of nodes in the weave.
    pub nodes: usize,
    /// Number of root nodes in the weave.
    pub roots: usize,
    /// Number of active nodes in the weave.
    pub active: usize,
    /// Number of bookmarked nodes in the weave.
    pub bookmarked: usize,
}

/// A metadata-only receipt for one admitted weave operation.
///
/// The receipt binds the operation class, the target identifier, bounded
/// weave counts, and the policy identity. It never contains node text,
/// prompts, provider payloads, credentials, or paths.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaveOperationReceipt {
    operation: WeaveOperationKind,
    target: u128,
    counts: WeaveCounts,
    policy_identity: String,
    receipt_identity: String,
}

impl WeaveOperationReceipt {
    /// Creates a receipt and derives its domain-separated BLAKE3 identity.
    #[must_use]
    pub fn new(
        operation: WeaveOperationKind,
        target: u128,
        counts: WeaveCounts,
        policy_identity: String,
    ) -> Self {
        let receipt_identity = Self::derive_identity(operation, target, counts, &policy_identity);
        Self {
            operation,
            target,
            counts,
            policy_identity,
            receipt_identity,
        }
    }

    fn derive_identity(
        operation: WeaveOperationKind,
        target: u128,
        counts: WeaveCounts,
        policy_identity: &str,
    ) -> String {
        let canonical = format!(
            "{RECEIPT_DOMAIN}|op={}|target={target}|nodes={}|roots={}|active={}|bookmarked={}|policy={policy_identity}",
            operation.as_str(),
            counts.nodes,
            counts.roots,
            counts.active,
            counts.bookmarked
        );
        format!("{}", blake3::hash(canonical.as_bytes()).to_hex())
    }

    /// Returns the operation class.
    #[must_use]
    pub fn operation(&self) -> WeaveOperationKind {
        self.operation
    }

    /// Returns the target node identifier.
    #[must_use]
    pub fn target(&self) -> u128 {
        self.target
    }

    /// Returns the bounded weave counts after the operation.
    #[must_use]
    pub fn counts(&self) -> WeaveCounts {
        self.counts
    }

    /// Returns the policy identity bound into the receipt.
    #[must_use]
    pub fn policy_identity(&self) -> &str {
        &self.policy_identity
    }

    /// Returns the domain-separated BLAKE3 receipt identity.
    #[must_use]
    pub fn receipt_identity(&self) -> &str {
        &self.receipt_identity
    }

    /// Returns `true` when every identity field is lowercase BLAKE3 hex.
    #[must_use]
    pub fn has_valid_identities(&self) -> bool {
        is_blake3_hex(&self.policy_identity) && is_blake3_hex(&self.receipt_identity)
    }
}

/// Returns `true` when the value is exactly 64 lowercase hex characters.
#[must_use]
pub fn is_blake3_hex(value: &str) -> bool {
    value.len() == BLAKE3_HEX_CHARS
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
