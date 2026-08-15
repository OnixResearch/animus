//! Positive and negative fixtures for agent trace export.
//!
//! Negative cases: non-monotonic sequences, unbounded items, invalid labels,
//! duplicate performance summaries, and evidence material leaks.

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

use super::{
    AgentTrace, LITE_CEILING_BYTES, PerformanceSource, TraceContentMode, TraceError, TraceItem,
    TraceItemKind, bound_item, build_trace_receipt, project_performance_accounting, validate_trace,
    validate_trace_evidence_text,
};

fn item(agent_id: &str, sequence: u64, kind: TraceItemKind, content: Option<&[u8]>) -> TraceItem {
    TraceItem {
        agent_id: agent_id.to_string(),
        sequence,
        kind,
        wall_time_millis: sequence * 1_000_000,
        content: content.map(|bytes| bound_item(TraceContentMode::Lite, bytes)),
    }
}

fn trace(agent_id: &str, items: Vec<TraceItem>) -> AgentTrace {
    AgentTrace {
        agent_id: agent_id.to_string(),
        items,
        mode: TraceContentMode::Lite,
    }
}

#[test]
fn positive_lite_projection_retains_exact_metrics_and_sequence() {
    // r[verify agent_harness_runtime.trace_export.wire]
    // r[verify agent_harness_runtime.trace_export.bounds]
    let long = alloc::vec![b'x'; LITE_CEILING_BYTES * 2];
    let items = vec![
        item("alpha", 1, TraceItemKind::Prose, Some(b"hello")),
        item("alpha", 2, TraceItemKind::Call, Some(&long)),
    ];
    let trace = trace("alpha", items);
    assert!(validate_trace(&trace).is_ok());
    let bounded = trace.items[1].content.as_ref().expect("content");
    assert_eq!(bounded.exact_len, (LITE_CEILING_BYTES * 2) as u64);
    assert_eq!(bounded.snippet.len(), LITE_CEILING_BYTES);
}

#[test]
fn positive_content_free_accounting_is_one_summary_per_agent() {
    // r[verify agent_harness_runtime.trace_export.accounting]
    let sources = vec![
        PerformanceSource {
            agent_id: "alpha".to_string(),
            provider_prompt_count: 3,
            total_tokens: 1_000,
            cache_read_tokens: 250,
            missing_evidence_marker: None,
        },
        PerformanceSource {
            agent_id: "beta".to_string(),
            provider_prompt_count: 1,
            total_tokens: 2_000,
            cache_read_tokens: 0,
            missing_evidence_marker: Some("provider-usage-unavailable".to_string()),
        },
    ];
    let summaries = project_performance_accounting(&sources).expect("accounting");
    assert_eq!(summaries.len(), 2);
    let alpha = summaries
        .iter()
        .find(|summary| summary.agent_id == "alpha")
        .expect("alpha");
    // 250/1000 * 1M = 250000 fixed point.
    assert_eq!(alpha.cache_read_ratio_fixed_point, 250_000);
    let beta = summaries
        .iter()
        .find(|summary| summary.agent_id == "beta")
        .expect("beta");
    assert_eq!(beta.missing_evidence_markers.len(), 1);
}

#[test]
fn positive_receipt_is_metadata_only_with_cross_agent_overview() {
    // r[verify agent_harness_runtime.trace_export.receipts]
    let items = vec![item("alpha", 1, TraceItemKind::Activation, None)];
    let trace = trace("alpha", items);
    let receipt = build_trace_receipt(&[&trace], 1).expect("receipt");
    assert_eq!(receipt.item_count, 1);
    assert_eq!(receipt.mode, "lite");
    assert!(receipt.identity.starts_with('b'));
    assert!(validate_trace_evidence_text(&receipt.identity).is_ok());
    assert!(
        validate_trace_evidence_text("raw prompt and provider payload leaked").is_err(),
        "evidence with raw material must be rejected"
    );
}

#[test]
fn negative_non_monotonic_sequence_is_rejected() {
    // r[verify agent_harness_runtime.trace_export.fail_closed]
    let items = vec![
        item("alpha", 2, TraceItemKind::Prose, None),
        item("alpha", 1, TraceItemKind::Call, None),
    ];
    assert_eq!(
        validate_trace(&trace("alpha", items)),
        Err(TraceError::NonMonotonicSequence)
    );
}

#[test]
fn negative_unbounded_and_foreign_items_fail_closed() {
    // r[verify agent_harness_runtime.trace_export.fail_closed]
    let mut items = vec![item("alpha", 1, TraceItemKind::Prose, None)];
    // A bound-violating snippet (content len below exact_len due to fake exact).
    let mut oversized = item("alpha", 2, TraceItemKind::Prose, None);
    oversized.content = Some(super::BoundedText {
        exact_len: 4,
        snippet: alloc::vec![b'x'; 8],
    });
    items.push(oversized);
    let bad = trace("alpha", items);
    assert_eq!(validate_trace(&bad), Err(TraceError::UnboundedItem));
    // A foreign agent label in a timeline is rejected.
    let mut foreign = vec![item("alpha", 1, TraceItemKind::Prose, None)];
    foreign.push(item("beta", 2, TraceItemKind::Prose, None));
    assert_eq!(
        validate_trace(&trace("alpha", foreign)),
        Err(TraceError::InvalidLabel)
    );
}

#[test]
fn negative_duplicate_and_empty_sources_fail_closed() {
    // r[verify agent_harness_runtime.trace_export.fail_closed]
    let sources = vec![
        PerformanceSource {
            agent_id: "alpha".to_string(),
            provider_prompt_count: 1,
            total_tokens: 0,
            cache_read_tokens: 0,
            missing_evidence_marker: None,
        },
        PerformanceSource {
            agent_id: "alpha".to_string(),
            provider_prompt_count: 2,
            total_tokens: 0,
            cache_read_tokens: 0,
            missing_evidence_marker: None,
        },
    ];
    assert_eq!(
        project_performance_accounting(&sources),
        Err(TraceError::DuplicateSummary)
    );
    // Empty-label source fails closed.
    let empty = vec![PerformanceSource {
        agent_id: String::new(),
        provider_prompt_count: 1,
        total_tokens: 1,
        cache_read_tokens: 1,
        missing_evidence_marker: None,
    }];
    assert_eq!(
        project_performance_accounting(&empty),
        Err(TraceError::InvalidLabel)
    );
}

#[test]
fn negative_zero_total_is_a_defensive_ratio_not_an_error() {
    // r[verify agent_harness_runtime.trace_export.accounting]
    let sources = vec![PerformanceSource {
        agent_id: "alpha".to_string(),
        provider_prompt_count: 1,
        total_tokens: 0,
        cache_read_tokens: 0,
        missing_evidence_marker: None,
    }];
    let summaries = project_performance_accounting(&sources).expect("accounting");
    assert_eq!(summaries[0].cache_read_ratio_fixed_point, 0);
}
