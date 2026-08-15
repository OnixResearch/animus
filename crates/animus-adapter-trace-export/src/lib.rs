#![no_std]
#![forbid(unsafe_code)]
//! Pure, bounded agent trace-export contracts.
//!
//! This family projects validated provider-declared calls, assistant prose,
//! reasoning, directional messages, and activations into a bounded semantic
//! timeline with per-agent journal-sequence authority. Lite mode bounds each
//! content item to a fixed ceiling while retaining exact complete-content
//! metrics. A content-free performance accounting projection exposes provider
//! prompt accounting, fixed-point cache ratios, and missing-evidence coverage
//! markers. Hosts own ledger persistence, live providers, and daemon discovery.

extern crate alloc;

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Trace-export schema identity.
pub const TRACE_SCHEMA: &str = "animus.trace-export.v1";
/// Receipt hash domain.
pub const TRACE_RECEIPT_DOMAIN: &str = "animus/trace-export-receipt";
/// Fixed non-zero lite content ceiling in bytes.
pub const LITE_CEILING_BYTES: usize = 4096;
/// Default lite ceiling label.
pub const LITE_LABEL: &str = "lite";
/// Full content mode label.
pub const FULL_LABEL: &str = "full";
/// Cross-agent overview label.
pub const CROSS_AGENT_OVERVIEW: &str = "cross-agent-wall-time-overview";
/// Maximum safe label length in bytes.
pub const MAX_LABEL_BYTES: usize = 160;
/// Fixed-point cache ratio scale.
pub const CACHE_RATIO_SCALE: u32 = 1_000_000;

/// Restricted semantic item kinds exported from the ledger.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraceItemKind {
    /// Provider-declared tool or model call.
    Call,
    /// Assistant prose.
    Prose,
    /// Reasoning summary.
    Reasoning,
    /// Explicit directional agent message.
    Message,
    /// Agent activation.
    Activation,
}

impl TraceItemKind {
    /// Returns the stable kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Call => "call",
            Self::Prose => "prose",
            Self::Reasoning => "reasoning",
            Self::Message => "message",
            Self::Activation => "activation",
        }
    }
}

/// Content export mode.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraceContentMode {
    /// Bound each content item to the fixed lite ceiling.
    Lite,
    /// Retain complete content.
    Full,
}

impl TraceContentMode {
    /// Returns the stable mode label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Lite => LITE_LABEL,
            Self::Full => FULL_LABEL,
        }
    }
}

/// Bounded content with exact complete-content metrics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedText {
    /// Exact byte length of the original content.
    pub exact_len: u64,
    /// Bound-preserving snippet. Exactly the original bytes when never truncated.
    pub snippet: Vec<u8>,
}

/// One typed trace item with authoritative per-agent sequence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceItem {
    /// Owning agent identity.
    pub agent_id: String,
    /// Monotonic journal sequence inside the agent.
    pub sequence: u64,
    /// Item kind.
    pub kind: TraceItemKind,
    /// Qualified absolute append wall time in milliseconds.
    pub wall_time_millis: u64,
    /// Bounded content when the kind carries content.
    pub content: Option<BoundedText>,
}

/// One agent's ordered semantic timeline.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentTrace {
    /// Agent identity.
    pub agent_id: String,
    /// Ordered items with monotonic journal sequences.
    pub items: Vec<TraceItem>,
    /// Content mode applied at projection time.
    pub mode: TraceContentMode,
}

impl AgentTrace {
    /// Returns whether the item list is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// Content-free performance source for one agent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerformanceSource {
    /// Agent identity.
    pub agent_id: String,
    /// Number of provider prompt turns.
    pub provider_prompt_count: u32,
    /// Total provider tokens.
    pub total_tokens: u64,
    /// Prompt-cache read tokens.
    pub cache_read_tokens: u64,
    /// Explicit missing-evidence coverage marker when evidence is missing.
    pub missing_evidence_marker: Option<String>,
}

/// Content-free performance summary for one agent.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PerformanceSummary {
    /// Agent identity.
    pub agent_id: String,
    /// Provider prompt count.
    pub provider_prompt_count: u32,
    /// Fixed-point cache read ratio scaled by [`CACHE_RATIO_SCALE`].
    pub cache_read_ratio_fixed_point: u32,
    /// Explicit missing-evidence coverage markers.
    pub missing_evidence_markers: Vec<String>,
}

/// Metadata-only trace-export receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TraceExportReceipt {
    /// Canonical BLAKE3 receipt identity.
    pub identity: String,
    /// Schema identity.
    pub schema: &'static str,
    /// Agent count.
    pub agent_count: u32,
    /// Item count.
    pub item_count: u32,
    /// Content-mode label.
    pub mode: &'static str,
    /// Performance summary count.
    pub performance_summary_count: u32,
    /// Cross-agent wall-time overview warning.
    pub cross_agent_overview: &'static str,
}

/// Typed trace-export error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TraceError {
    /// A label is empty, oversized, or contains control characters.
    InvalidLabel,
    /// An item carries content beyond the fixed ceiling.
    UnboundedItem,
    /// The per-agent journal sequence regresses or duplicates.
    NonMonotonicSequence,
    /// A performance source duplicate an agent summary.
    DuplicateSummary,
    /// A public count overflowed.
    Overflow,
}

impl TraceError {
    /// Returns the stable error label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidLabel => "invalid_label",
            Self::UnboundedItem => "unbounded_item",
            Self::NonMonotonicSequence => "non_monotonic_sequence",
            Self::DuplicateSummary => "duplicate_summary",
            Self::Overflow => "overflow",
        }
    }
}

fn validate_label(value: &str) -> Result<(), TraceError> {
    if value.is_empty() || value.len() > MAX_LABEL_BYTES || value.chars().any(char::is_control) {
        return Err(TraceError::InvalidLabel);
    }
    Ok(())
}

/// Bounds content under the selected mode, retaining exact metrics.
#[must_use]
pub fn bound_item(mode: TraceContentMode, content: &[u8]) -> BoundedText {
    match mode {
        TraceContentMode::Full => BoundedText {
            exact_len: u64::try_from(content.len()).unwrap_or(u64::MAX),
            snippet: content.to_vec(),
        },
        TraceContentMode::Lite => {
            let ceiling = content.len().min(LITE_CEILING_BYTES);
            BoundedText {
                exact_len: u64::try_from(content.len()).unwrap_or(u64::MAX),
                snippet: content[..ceiling].to_vec(),
            }
        }
    }
}

/// Validates one agent's timeline before any projection is accepted.
///
/// # Errors
///
/// Returns a typed error for invalid labels, unbounded items, or a
/// non-monotonic journal sequence.
pub fn validate_trace(trace: &AgentTrace) -> Result<(), TraceError> {
    validate_label(trace.agent_id.as_str())?;
    let mut prior_sequence = None;
    for item in &trace.items {
        validate_label(item.agent_id.as_str())?;
        if item.agent_id != trace.agent_id {
            return Err(TraceError::InvalidLabel);
        }
        if let Some(content) = &item.content {
            if trace.mode == TraceContentMode::Lite && content.snippet.len() > LITE_CEILING_BYTES {
                return Err(TraceError::UnboundedItem);
            }
            if u64::try_from(content.snippet.len()).unwrap_or(u64::MAX) > content.exact_len {
                return Err(TraceError::UnboundedItem);
            }
        }
        if let Some(prior) = prior_sequence {
            if item.sequence <= prior {
                return Err(TraceError::NonMonotonicSequence);
            }
        }
        prior_sequence = Some(item.sequence);
    }
    Ok(())
}

/// Projects one content-free performance summary per included agent.
///
/// Aggregated accounting never carries semantic text, reasoning, tool output,
/// or provider payload bodies.
///
/// # Errors
///
/// Returns [`TraceError::DuplicateSummary`] when a source appears more than
/// once and [`TraceError::Overflow`] on a count overflow.
pub fn project_performance_accounting(
    sources: &[PerformanceSource],
) -> Result<Vec<PerformanceSummary>, TraceError> {
    let mut summaries = Vec::with_capacity(sources.len());
    let mut seen = alloc::collections::BTreeSet::<String>::new();
    for source in sources {
        validate_label(source.agent_id.as_str())?;
        if !seen.insert(source.agent_id.clone()) {
            return Err(TraceError::DuplicateSummary);
        }
        let cache_read_ratio_fixed_point =
            fixed_point_ratio(source.cache_read_tokens, source.total_tokens)?;
        let missing_evidence_markers = source
            .missing_evidence_marker
            .as_ref()
            .map_or_else(Vec::new, |marker| alloc::vec![marker.clone()]);
        summaries.push(PerformanceSummary {
            agent_id: source.agent_id.clone(),
            provider_prompt_count: source.provider_prompt_count,
            cache_read_ratio_fixed_point,
            missing_evidence_markers,
        });
    }
    Ok(summaries)
}

fn fixed_point_ratio(read_tokens: u64, total_tokens: u64) -> Result<u32, TraceError> {
    if total_tokens == 0 {
        return Ok(0);
    }
    let scaled = u128::from(read_tokens)
        .checked_mul(u128::from(CACHE_RATIO_SCALE))
        .ok_or(TraceError::Overflow)?;
    let ratio = scaled / u128::from(total_tokens);
    u32::try_from(ratio).map_err(|_| TraceError::Overflow)
}

/// Builds a metadata-only trace-export receipt.
///
/// # Errors
///
/// Returns [`TraceError::Overflow`] for a count overflow.
pub fn build_trace_receipt(
    traces: &[&AgentTrace],
    performance_summary_count: u32,
) -> Result<TraceExportReceipt, TraceError> {
    let mut agent_ids = alloc::collections::BTreeSet::<String>::new();
    let mut item_count = 0_u32;
    let mode = traces
        .first()
        .map_or(TraceContentMode::Lite, |trace| trace.mode);
    for trace in traces {
        validate_label(trace.agent_id.as_str())?;
        if !agent_ids.insert(trace.agent_id.clone()) {
            // Distinct agent timelines may be supplied separately; counts stay exact.
        }
        item_count = item_count
            .checked_add(u32::try_from(trace.items.len()).map_err(|_| TraceError::Overflow)?)
            .ok_or(TraceError::Overflow)?;
    }
    let agent_count = u32::try_from(agent_ids.len()).map_err(|_| TraceError::Overflow)?;
    let identity = crate::hash_parts(
        TRACE_RECEIPT_DOMAIN,
        &[
            TRACE_SCHEMA,
            &agent_count.to_string(),
            &item_count.to_string(),
            mode.as_str(),
            &performance_summary_count.to_string(),
        ],
    );
    Ok(TraceExportReceipt {
        identity,
        schema: TRACE_SCHEMA,
        agent_count,
        item_count,
        mode: mode.as_str(),
        performance_summary_count,
        cross_agent_overview: CROSS_AGENT_OVERVIEW,
    })
}

/// Rejects raw material in trace evidence text.
///
/// # Errors
///
/// Returns [`TraceError::InvalidLabel`] when a leak sentinel appears.
pub fn validate_trace_evidence_text(value: &str) -> Result<(), TraceError> {
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "provider payload",
        "raw prompt",
        "reasoning text",
        "credential",
        "/home/",
    ] {
        if lower.contains(forbidden) {
            return Err(TraceError::InvalidLabel);
        }
    }
    Ok(())
}

pub(crate) fn hash_parts(domain: &str, parts: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    for part in parts {
        hasher.update(&[0]);
        hasher.update(part.as_bytes());
    }
    alloc::format!("b3:{}", hasher.finalize().to_hex())
}

#[cfg(test)]
mod tests;
