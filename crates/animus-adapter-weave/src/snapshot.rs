//! Redacted topology snapshots.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Domain separator for topology digests.
const TOPOLOGY_DOMAIN: &str = "animus_adapter_weave_topology_v1";

/// One redacted node entry in a weave snapshot.
///
/// The entry exposes identifiers, fan-in and fan-out counts, flags, content
/// length, and a BLAKE3 content digest. It never contains node text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaveSnapshotEntry {
    /// The node identifier.
    pub id: u128,
    /// The number of parent links.
    pub parent_count: usize,
    /// The number of child links.
    pub child_count: usize,
    /// Whether the node is active.
    pub active: bool,
    /// Whether the node is bookmarked.
    pub bookmarked: bool,
    /// The content length in bytes.
    pub content_bytes: usize,
    /// The BLAKE3 digest of the node text.
    pub content_digest: String,
}

/// A redacted, deterministically ordered view of one weave.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeaveSnapshot {
    /// Bounded per-node entries in stable weave order.
    pub entries: Vec<WeaveSnapshotEntry>,
    /// Domain-separated BLAKE3 digest over ordered identifiers and content digests.
    pub topology_digest: String,
}

impl WeaveSnapshot {
    /// Builds a snapshot from ordered entries and derives the topology digest.
    #[must_use]
    pub fn new(entries: Vec<WeaveSnapshotEntry>) -> Self {
        let topology_digest = Self::derive_topology_digest(&entries);
        Self {
            entries,
            topology_digest,
        }
    }

    fn derive_topology_digest(entries: &[WeaveSnapshotEntry]) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(TOPOLOGY_DOMAIN.as_bytes());
        for entry in entries {
            let line = format!(
                "|{}:{}:{}:{}:{}:{}",
                entry.id,
                entry.parent_count,
                entry.child_count,
                entry.active,
                entry.bookmarked,
                entry.content_digest
            );
            hasher.update(line.as_bytes());
        }
        format!("{}", hasher.finalize().to_hex())
    }
}

/// Returns the BLAKE3 digest of node text for snapshot entries.
#[must_use]
pub fn content_digest(text: &str) -> String {
    format!("{}", blake3::hash(text.as_bytes()).to_hex())
}
