#![no_std]
#![forbid(unsafe_code)]
//! Pure, non-authoritative per-agent usage-cost accounting projection.
//!
//! This family admits typed usage records, bounds each record to an agent and
//! session, prices records against the model that served them with a
//! documented fallback, and projects an approximate per-agent subtree figure.
//! Every projection is explicitly marked as a non-authoritative estimate, not
//! provider billing reconciliation. Hosts own usage ingestion, price catalogs,
//! ledger persistence, and billing reconciliation.

extern crate alloc;

use alloc::collections::BTreeSet;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Accounting schema identity.
pub const ACCOUNTING_SCHEMA: &str = "animus.usage-accounting.v1";
/// Receipt hash domain.
pub const ACCOUNTING_RECEIPT_DOMAIN: &str = "animus/usage-accounting-receipt";
/// Scale for price units: price is micro-dollars per one million tokens.
pub const PRICE_SCALE: u128 = 1_000_000;
/// Explicit non-authoritative estimate label.
pub const NON_AUTHORITATIVE: &str = "non-authoritative-estimate";
/// Maximum safe label length in bytes.
pub const MAX_LABEL_BYTES: usize = 160;

/// One typed usage record bound to an agent and session.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UsageRecord {
    /// Agent identity.
    pub agent_id: String,
    /// Session identity.
    pub session_id: String,
    /// Model that served the record.
    pub model_id: String,
    /// Input token count.
    pub input_tokens: u64,
    /// Output token count.
    pub output_tokens: u64,
    /// Prompt-cache read token count.
    pub cache_read_tokens: u64,
}

/// Registry price metadata for one model, in micro-dollars per million tokens.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelPriceEntry {
    /// Model identity.
    pub model_id: String,
    /// Input price per million tokens in micro-dollars.
    pub input_price_per_million: u64,
    /// Output price per million tokens in micro-dollars.
    pub output_price_per_million: u64,
}

/// Reviewed pricing policy with a documented fallback.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PricingPolicy {
    /// Exact registry price metadata.
    pub price_catalog: Vec<ModelPriceEntry>,
    /// Documented fallback input price per million tokens in micro-dollars.
    pub fallback_input_price_per_million: u64,
    /// Documented fallback output price per million tokens in micro-dollars.
    pub fallback_output_price_per_million: u64,
    /// When true, a missing registry price fails the summary instead of using
    /// the documented fallback.
    pub deny_missing_prices: bool,
}

/// One authenticated same-session creator relationship.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CreatorBinding {
    /// Child agent identity.
    pub child: String,
    /// Parent agent identity.
    pub parent: String,
    /// Session that contains the relationship.
    pub session: String,
}

/// Per-agent accounting summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentAccountSummary {
    /// Agent identity.
    pub agent_id: String,
    /// Number of admitted records.
    pub record_count: u32,
    /// Total input tokens.
    pub input_tokens: u64,
    /// Total output tokens.
    pub output_tokens: u64,
    /// Approximate cost in micro-dollars.
    pub cost_estimate_micro_usd: u128,
    /// Whether any record used the documented fallback price.
    pub used_fallback: bool,
    /// Number of records whose required price was missing.
    pub missing_price_count: u32,
    /// Number of distinct sessions.
    pub session_count: u32,
}

/// One agent subtree cost projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubtreeProjection {
    /// Root agent identity.
    pub agent_id: String,
    /// Approximate subtree cost in micro-dollars.
    pub subtree_cost_micro_usd: u128,
    /// Included descendant agent identities.
    pub included_agents: Vec<String>,
    /// Number of attributed records.
    pub record_count: u32,
}

/// Complete non-authoritative accounting result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountingProjection {
    /// One summary per included agent.
    pub summaries: Vec<AgentAccountSummary>,
    /// Approximate subtree projections.
    pub subtrees: Vec<SubtreeProjection>,
    /// Unattributed records excluded from all subtrees.
    pub unattributed_record_count: u32,
    /// Explicit estimate marker.
    pub estimate: &'static str,
}

/// Metadata-only accounting receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AccountingReceipt {
    /// Canonical BLAKE3 receipt identity.
    pub identity: String,
    /// Schema identity.
    pub schema: &'static str,
    /// Summary count.
    pub summary_count: u32,
    /// Subtree count.
    pub subtree_count: u32,
    /// Record count.
    pub record_count: u32,
    /// Missing-price count.
    pub missing_price_count: u32,
    /// Estimate marker.
    pub estimate: &'static str,
}

/// Typed accounting error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccountingError {
    /// A label is empty, oversized, or contains control characters.
    InvalidLabel,
    /// A record or binding is malformed.
    MalformedRecord,
    /// A required registry price is missing.
    MissingRequiredPrice,
    /// A public count or cost total overflowed.
    Overflow,
    /// A duplicate agent summary was produced.
    DuplicateAgentSummary,
}

impl AccountingError {
    /// Returns the stable error label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidLabel => "invalid_label",
            Self::MalformedRecord => "malformed_record",
            Self::MissingRequiredPrice => "missing_required_price",
            Self::Overflow => "overflow",
            Self::DuplicateAgentSummary => "duplicate_agent_summary",
        }
    }
}

fn validate_label(value: &str) -> Result<(), AccountingError> {
    if value.is_empty() || value.len() > MAX_LABEL_BYTES || value.chars().any(char::is_control) {
        return Err(AccountingError::InvalidLabel);
    }
    Ok(())
}

fn validate_record(record: &UsageRecord) -> Result<(), AccountingError> {
    validate_label(record.agent_id.as_str())?;
    validate_label(record.session_id.as_str())?;
    validate_label(record.model_id.as_str())?;
    Ok(())
}

fn price_of(policy: &PricingPolicy, model_id: &str) -> Option<(u64, u64)> {
    policy
        .price_catalog
        .iter()
        .find(|entry| entry.model_id == model_id)
        .map(|entry| {
            (
                entry.input_price_per_million,
                entry.output_price_per_million,
            )
        })
}

fn token_cost(tokens: u64, price_per_million: u64) -> u128 {
    let tokens = u128::from(tokens);
    let price = u128::from(price_per_million);
    tokens.saturating_mul(price) / PRICE_SCALE
}

/// Projects one per-agent accounting summary set from typed inputs.
///
/// # Errors
///
/// Returns a typed error for malformed records, missing required prices,
/// overflow, or duplicate agent labels.
pub fn project_accounting(
    records: &[UsageRecord],
    policy: &PricingPolicy,
) -> Result<AccountingProjection, AccountingError> {
    let mut ordering = Vec::<String>::new();
    let mut by_agent = alloc::collections::BTreeMap::<String, Vec<&UsageRecord>>::new();
    for record in records {
        validate_record(record)?;
        if !by_agent.contains_key(record.agent_id.as_str()) {
            ordering.push(record.agent_id.clone());
        }
        by_agent
            .entry(record.agent_id.clone())
            .or_default()
            .push(record);
    }
    let mut summaries = Vec::with_capacity(ordering.len());
    let mut missing_price_count = 0_u32;
    for agent_id in &ordering {
        let agent_records = by_agent
            .get(agent_id)
            .ok_or(AccountingError::MalformedRecord)?;
        let (input_tokens, output_tokens, cost, used_fallback, missing) =
            aggregate(agent_records, policy)?;
        missing_price_count = missing_price_count.saturating_add(missing);
        summaries.push(AgentAccountSummary {
            agent_id: agent_id.clone(),
            record_count: u32::try_from(agent_records.len())
                .map_err(|_| AccountingError::Overflow)?,
            input_tokens,
            output_tokens,
            cost_estimate_micro_usd: cost,
            used_fallback,
            missing_price_count: missing,
            session_count: distinct_session_count(agent_records),
        });
    }
    Ok(AccountingProjection {
        summaries,
        subtrees: Vec::new(),
        unattributed_record_count: 0,
        estimate: NON_AUTHORITATIVE,
    })
}

fn aggregate(
    records: &[&UsageRecord],
    policy: &PricingPolicy,
) -> Result<(u64, u64, u128, bool, u32), AccountingError> {
    let mut input_tokens = 0_u64;
    let mut output_tokens = 0_u64;
    let mut cost = 0_u128;
    let mut used_fallback = false;
    let mut missing = 0_u32;
    for record in records {
        input_tokens = input_tokens
            .checked_add(record.input_tokens)
            .ok_or(AccountingError::Overflow)?;
        output_tokens = output_tokens
            .checked_add(record.output_tokens)
            .ok_or(AccountingError::Overflow)?;
        let (input_price, output_price) = match price_of(policy, record.model_id.as_str()) {
            Some(prices) => prices,
            None if policy.deny_missing_prices => {
                missing = missing.saturating_add(1);
                continue;
            }
            None => {
                used_fallback = true;
                (
                    policy.fallback_input_price_per_million,
                    policy.fallback_output_price_per_million,
                )
            }
        };
        cost = cost
            .checked_add(token_cost(record.input_tokens, input_price))
            .and_then(|value| value.checked_add(token_cost(record.cache_read_tokens, input_price)))
            .and_then(|value| value.checked_add(token_cost(record.output_tokens, output_price)))
            .ok_or(AccountingError::Overflow)?;
    }
    Ok((input_tokens, output_tokens, cost, used_fallback, missing))
}

fn distinct_session_count(records: &[&UsageRecord]) -> u32 {
    let sessions = records
        .iter()
        .map(|record| record.session_id.as_str())
        .collect::<BTreeSet<_>>();
    u32::try_from(sessions.len()).unwrap_or(u32::MAX)
}

/// Projects approximate per-agent subtree figures from authenticated
/// same-session creator relationships.
///
/// # Errors
///
/// Returns a typed error for malformed bindings, unknown agents, or overflow.
pub fn project_subtrees(
    summaries: &[AgentAccountSummary],
    bindings: &[CreatorBinding],
) -> Result<Vec<SubtreeProjection>, AccountingError> {
    for binding in bindings {
        validate_label(binding.child.as_str())?;
        validate_label(binding.parent.as_str())?;
        validate_label(binding.session.as_str())?;
    }
    let known_agents = summaries
        .iter()
        .map(|summary| summary.agent_id.clone())
        .collect::<alloc::collections::BTreeSet<_>>();
    let mut subtrees = Vec::with_capacity(summaries.len());
    for summary in summaries {
        let mut included = Vec::<String>::new();
        let mut pending = alloc::collections::VecDeque::from([summary.agent_id.clone()]);
        while let Some(agent) = pending.pop_front() {
            if included.contains(&agent) {
                continue;
            }
            included.push(agent.clone());
            for binding in bindings {
                if binding.parent == agent
                    && binding.session == summary_session(binding, summary)
                    && known_agents.contains(&binding.child)
                {
                    push_unseen(&mut pending, &included, binding.child.clone());
                }
            }
        }
        let mut cost = summary.cost_estimate_micro_usd;
        let mut records = summary.record_count;
        for included_agent in included.iter().skip(1) {
            let Some(child) = summaries
                .iter()
                .find(|entry| &entry.agent_id == included_agent)
            else {
                continue;
            };
            cost = cost
                .checked_add(child.cost_estimate_micro_usd)
                .ok_or(AccountingError::Overflow)?;
            records = records
                .checked_add(child.record_count)
                .ok_or(AccountingError::Overflow)?;
        }
        subtrees.push(SubtreeProjection {
            agent_id: summary.agent_id.clone(),
            subtree_cost_micro_usd: cost,
            included_agents: included,
            record_count: records,
        });
    }
    Ok(subtrees)
}

fn summary_session(binding: &CreatorBinding, _summary: &AgentAccountSummary) -> String {
    // The binding session is authoritative for the parent-child relationship.
    binding.session.clone()
}

fn push_unseen(pending: &mut alloc::collections::VecDeque<String>, seen: &[String], value: String) {
    if !seen.contains(&value) {
        pending.push_back(value);
    }
}

/// Builds a metadata-only accounting receipt.
///
/// # Errors
///
/// Returns [`AccountingError::Overflow`] for a count overflow.
pub fn build_accounting_receipt(
    projection: &AccountingProjection,
) -> Result<AccountingReceipt, AccountingError> {
    let summary_count =
        u32::try_from(projection.summaries.len()).map_err(|_| AccountingError::Overflow)?;
    let subtree_count =
        u32::try_from(projection.subtrees.len()).map_err(|_| AccountingError::Overflow)?;
    let record_count = projection.summaries.iter().fold(0_u32, |total, summary| {
        total.saturating_add(summary.record_count)
    });
    let missing_price_count = projection.summaries.iter().fold(0_u32, |total, summary| {
        total.saturating_add(summary.missing_price_count)
    });
    let identity = crate::hash_parts(
        ACCOUNTING_RECEIPT_DOMAIN,
        &[
            ACCOUNTING_SCHEMA,
            &summary_count.to_string(),
            &subtree_count.to_string(),
            &record_count.to_string(),
            &missing_price_count.to_string(),
            projection.estimate,
        ],
    );
    Ok(AccountingReceipt {
        identity,
        schema: ACCOUNTING_SCHEMA,
        summary_count,
        subtree_count,
        record_count,
        missing_price_count,
        estimate: projection.estimate,
    })
}

/// Rejects raw usage material in accounting evidence text.
///
/// # Errors
///
/// Returns [`AccountingError::InvalidLabel`] when a leak sentinel appears.
pub fn validate_accounting_evidence_text(value: &str) -> Result<(), AccountingError> {
    let lower = value.to_ascii_lowercase();
    for forbidden in [
        "provider payload",
        "raw usage",
        "credential",
        "/home/",
        "reasoning text",
    ] {
        if lower.contains(forbidden) {
            return Err(AccountingError::InvalidLabel);
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
