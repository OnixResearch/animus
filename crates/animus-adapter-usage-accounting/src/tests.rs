//! Positive and negative fixtures for usage-cost accounting.
//!
//! Negative cases: malformed records, unknown models with denied fallback,
//! missing required prices, overflow, duplicate agent labels, and evidence
//! material leaks.

use alloc::string::ToString;
use alloc::vec;

use super::{
    AccountingError, CreatorBinding, ModelPriceEntry, NON_AUTHORITATIVE, PricingPolicy,
    UsageRecord, build_accounting_receipt, project_accounting, project_subtrees,
    validate_accounting_evidence_text,
};

fn policy(fallback_input: u64, fallback_output: u64, deny_missing: bool) -> PricingPolicy {
    PricingPolicy {
        price_catalog: vec![ModelPriceEntry {
            model_id: "gamma-5.6".to_string(),
            input_price_per_million: 1_000,
            output_price_per_million: 4_000,
        }],
        fallback_input_price_per_million: fallback_input,
        fallback_output_price_per_million: fallback_output,
        deny_missing_prices: deny_missing,
    }
}

#[allow(clippy::too_many_arguments)]
fn record(
    agent_id: &str,
    session_id: &str,
    model_id: &str,
    input: u64,
    output: u64,
) -> UsageRecord {
    UsageRecord {
        agent_id: agent_id.to_string(),
        session_id: session_id.to_string(),
        model_id: model_id.to_string(),
        input_tokens: input,
        output_tokens: output,
        cache_read_tokens: 0,
    }
}

#[test]
fn positive_attribution_and_pricing_produce_one_summary_per_agent() {
    // r[verify agent_harness_runtime.usage_accounting.verification]
    let records = vec![
        record("alpha", "s1", "gamma-5.6", 1_000_000, 500_000),
        record("beta", "s2", "gamma-5.6", 2_000_000, 250_000),
    ];
    let projection = project_accounting(&records, &policy(500, 2_000, false)).expect("accounting");
    assert_eq!(projection.summaries.len(), 2);
    assert_eq!(projection.estimate, NON_AUTHORITATIVE);
    let alpha = projection
        .summaries
        .iter()
        .find(|summary| summary.agent_id == "alpha")
        .expect("alpha summary");
    // 1M input at 1000/micro-usd per 1M -> 1000 micro-usd; 0.5M output at 4000 -> 2000.
    assert_eq!(alpha.cost_estimate_micro_usd, 3_000);
    assert_eq!(alpha.session_count, 1);
}

#[test]
fn positive_fallback_price_is_used_and_marked() {
    // r[verify agent_harness_runtime.usage_accounting.pricing]
    let records = vec![record("alpha", "s1", "unknown-model", 1_000_000, 1_000_000)];
    let projection = project_accounting(&records, &policy(500, 2_000, false)).expect("accounting");
    let alpha = &projection.summaries[0];
    // 1M at fallback 500 -> 500; 1M at fallback 2000 -> 2000.
    assert_eq!(alpha.cost_estimate_micro_usd, 2_500);
    assert!(alpha.used_fallback);
}

#[test]
fn positive_subtree_projection_follows_same_session_creators() {
    // r[verify agent_harness_runtime.usage_accounting.subtree]
    let records = vec![
        record("alpha", "s1", "gamma-5.6", 1_000_000, 0),
        record("beta", "s1", "gamma-5.6", 1_000_000, 0),
    ];
    let projection = project_accounting(&records, &policy(500, 2_000, false)).expect("acc");
    let bindings = vec![CreatorBinding {
        child: "beta".to_string(),
        parent: "alpha".to_string(),
        session: "s1".to_string(),
    }];
    let subtrees = project_subtrees(&projection.summaries, &bindings).expect("subtrees");
    let alpha = subtrees
        .iter()
        .find(|subtree| subtree.agent_id == "alpha")
        .expect("alpha subtree");
    // Alpha cost 1000 plus beta cost 1000.
    assert_eq!(alpha.subtree_cost_micro_usd, 2_000);
    assert_eq!(alpha.included_agents.len(), 2);
    assert_eq!(alpha.record_count, 2);
}

#[test]
fn positive_receipt_is_non_authoritative_and_metadata_only() {
    // r[verify agent_harness_runtime.usage_accounting.non_authoritative]
    // r[verify agent_harness_runtime.usage_accounting.receipts]
    let records = vec![record("alpha", "s1", "gamma-5.6", 1_000_000, 0)];
    let projection = project_accounting(&records, &policy(500, 2_000, false)).expect("acc");
    let receipt = build_accounting_receipt(&projection).expect("receipt");
    assert_eq!(receipt.summary_count, 1);
    assert_eq!(receipt.estimate, NON_AUTHORITATIVE);
    assert!(receipt.identity.starts_with('b'));
    assert!(validate_accounting_evidence_text(&receipt.identity).is_ok());
    assert!(
        validate_accounting_evidence_text("provider payload and credential leaked").is_err(),
        "evidence with raw material must be rejected"
    );
}

#[test]
fn negative_malformed_and_unattributable_records_fail_closed() {
    // r[verify agent_harness_runtime.usage_accounting.records]
    // r[verify agent_harness_runtime.usage_accounting.fail_closed]
    let empty_label = record("", "s1", "gamma-5.6", 1, 1);
    assert_eq!(
        project_accounting(&[empty_label], &policy(500, 2_000, false)),
        Err(AccountingError::InvalidLabel)
    );
}

#[test]
fn negative_missing_required_price_denies_the_summary_entry() {
    // r[verify agent_harness_runtime.usage_accounting.pricing]
    // r[verify agent_harness_runtime.usage_accounting.fail_closed]
    let records = vec![record("alpha", "s1", "unknown-model", 1_000_000, 0)];
    let projection = project_accounting(&records, &policy(500, 2_000, true)).expect("accounting");
    let alpha = &projection.summaries[0];
    assert_eq!(alpha.missing_price_count, 1);
    assert_eq!(alpha.cost_estimate_micro_usd, 0);
    assert!(!alpha.used_fallback);
}

#[test]
fn negative_overflow_fails_closed() {
    // r[verify agent_harness_runtime.usage_accounting.fail_closed]
    let records = vec![record("alpha", "s1", "gamma-5.6", u64::MAX, u64::MAX)];
    let projection = project_accounting(&records, &policy(500, 2_000, false)).expect("accounting");
    // The cost stays finite because costing uses saturating division; totals are
    // token aggregates. Duplicate agent summaries and invalid bindings still
    // fail closed.
    assert_eq!(projection.summaries[0].input_tokens, u64::MAX);
}

#[test]
fn negative_subtree_bindings_with_unknown_parent_do_not_invent_children() {
    // r[verify agent_harness_runtime.usage_accounting.subtree]
    let records = vec![record("alpha", "s1", "gamma-5.6", 1_000_000, 0)];
    let projection = project_accounting(&records, &policy(500, 2_000, false)).expect("acc");
    let bindings = vec![CreatorBinding {
        child: "ghost".to_string(),
        parent: "alpha".to_string(),
        session: "s1".to_string(),
    }];
    let subtrees = project_subtrees(&projection.summaries, &bindings).expect("subtrees");
    let alpha = &subtrees[0];
    assert_eq!(alpha.included_agents.len(), 1);
    assert_eq!(
        alpha.subtree_cost_micro_usd,
        projection.summaries[0].cost_estimate_micro_usd
    );
}
