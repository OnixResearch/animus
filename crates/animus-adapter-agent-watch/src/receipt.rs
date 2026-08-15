//! Metadata-only watch receipts.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::graph::WatchDenial;
use crate::{WATCH_RECEIPT_DOMAIN, WATCH_SCHEMA, hash_parts};

/// Metadata-only watch receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WatchReceipt {
    /// Canonical BLAKE3 receipt identity.
    pub identity: String,
    /// BLAKE3 watch-graph identity.
    pub graph_identity: String,
    /// Number of directed watch edges.
    pub edge_count: u32,
    /// Number of projected watcher views.
    pub view_count: u32,
    /// Last transition denial, when one was recorded.
    pub denial: Option<WatchDenial>,
    /// Explicit monotonic generation marker.
    pub generation_marker: u64,
}

/// Builds a metadata-only watch receipt. Contains no prompts, transcripts, or
/// provider payloads by construction.
#[allow(clippy::too_many_arguments)]
#[must_use]
pub fn build_watch_receipt(
    graph_identity: &str,
    edge_count: u32,
    view_count: u32,
    denial: Option<WatchDenial>,
    generation_marker: u64,
) -> WatchReceipt {
    let edges = edge_count.to_string();
    let views = view_count.to_string();
    let generation = generation_marker.to_string();
    let denial_text = denial.map_or("none", WatchDenial::as_str);
    let identity = hash_parts(
        WATCH_RECEIPT_DOMAIN,
        &[
            WATCH_SCHEMA,
            graph_identity,
            edges.as_str(),
            views.as_str(),
            denial_text,
            generation.as_str(),
        ],
    );
    WatchReceipt {
        identity,
        graph_identity: graph_identity.to_string(),
        edge_count,
        view_count,
        denial,
        generation_marker,
    }
}

/// Renders a bounded receipt summary line without raw content.
#[must_use]
pub fn watch_receipt_line(receipt: &WatchReceipt) -> String {
    alloc::format!(
        "{} edges={} views={} denial={} generation={}",
        receipt.identity,
        receipt.edge_count,
        receipt.view_count,
        receipt.denial.map_or("none", WatchDenial::as_str),
        receipt.generation_marker
    )
}

/// Returns forbidden material that must never appear in receipt text.
#[must_use]
pub fn forbidden_receipt_material() -> Vec<&'static str> {
    Vec::from([
        "raw prompt",
        "provider payload",
        "reasoning text",
        "tool output",
        "/home/",
    ])
}

/// Rejects raw material in watch evidence text.
///
/// # Errors
///
/// Returns [`crate::graph::WatchError::InvalidLabel`] when a leak sentinel
/// appears in evidence text.
pub fn validate_watch_evidence_text(value: &str) -> Result<(), crate::graph::WatchError> {
    let lower = value.to_ascii_lowercase();
    if forbidden_receipt_material()
        .iter()
        .any(|item| lower.contains(item))
    {
        return Err(crate::graph::WatchError::InvalidLabel);
    }
    Ok(())
}

/// Re-exports the denial label helper for receipt consumers.
#[must_use]
pub const fn denial_label(denial: WatchDenial) -> &'static str {
    denial.as_str()
}
