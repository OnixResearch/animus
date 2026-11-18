//! The predeclared comparison protocol of the local scorer route and its adoption rule.
//!
//! The protocol is the checked projection of `config/campaign-manager-local-scorer-eval.ncl`; the
//! fixture artifact is the checked projection of `config/campaign-manager-local-scorer-fixture.ncl`.
//! Both are embedded, so selecting them needs no filesystem access.

/// Schema of the comparison protocol.
pub const COMPARISON_SCHEMA: &str = "animus.campaign-manager-local-scorer-eval.v1";
/// Most random-ranking seeds one protocol may declare.
const MAX_RANDOM_SEEDS: usize = 16;
/// Fixed-point scale of the adoption thresholds.
const PER_MILLE: f64 = 1_000.0;

/// The checked fixture artifact, exported from `config/campaign-manager-local-scorer-fixture.ncl`.
const EMBEDDED_FIXTURE: &str =
    include_str!("../../../../config/generated/campaign-manager-local-scorer-fixture.json");
/// The checked protocol, exported from `config/campaign-manager-local-scorer-eval.ncl`.
const EMBEDDED_PROTOCOL: &str =
    include_str!("../../../../config/generated/campaign-manager-local-scorer-eval.json");

/// The adoption rule, fixed before any arm runs.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionRule {
    /// Smallest gain of the scorer's mean primary outcome over the baseline's, per mille.
    pub min_gain_over_baseline_per_mille: u32,
    /// Whether the scorer must also reach the random arms' mean.
    pub requires_random_parity: bool,
    /// Largest share of scorer calls that may fall back, per mille.
    pub max_fallback_per_mille: u32,
}

/// The predeclared comparison.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparisonProtocol {
    /// Protocol schema.
    pub schema: String,
    /// Identity of the selected policy artifact.
    pub artifact_identity: String,
    /// Model-route identity of the grant-matched baseline.
    pub baseline: String,
    /// Model-route identities reported as non-comparable references.
    pub references: Vec<String>,
    /// One random-ranking route per seed.
    pub random_seeds: Vec<u64>,
    /// The metric the adoption rule compares.
    pub primary_outcome: String,
    /// The adoption rule.
    pub adoption: AdoptionRule,
    /// What the comparison does not establish.
    pub non_claims: Vec<String>,
}

/// Why a protocol was refused.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProtocolError {
    /// The projection is not one protocol document.
    Malformed,
    /// Another protocol schema.
    Schema,
    /// The artifact identity is not a canonical `b3:` identity.
    ArtifactIdentity,
    /// The baseline is not the grant-matched `explore-only` route.
    Baseline,
    /// A reference is not a scripted route, or repeats the baseline.
    References,
    /// No seed, a repeated seed, or too many seeds.
    Seeds,
    /// The primary outcome is not the found metric.
    Outcome,
    /// A threshold exceeds 1000 per mille.
    Adoption,
    /// No non-claim is declared.
    NonClaims,
}

/// The adoption decision of one comparison.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdoptionDecision {
    /// Every condition of the rule holds on this synthetic lane.
    Go,
    /// Some condition of the rule fails.
    NoGo,
}

/// Measured means that the adoption rule compares.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AdoptionInput {
    /// The scorer's mean primary outcome.
    pub scorer: f64,
    /// The baseline's mean primary outcome.
    pub baseline: f64,
    /// The random arms' mean primary outcome.
    pub random: f64,
    /// Share of scorer calls that fell back, per mille.
    pub scorer_fallback_per_mille: u32,
}

/// The embedded fixture artifact's bytes.
#[must_use]
pub fn embedded_local_scorer_fixture() -> &'static [u8] {
    EMBEDDED_FIXTURE.as_bytes()
}

/// The embedded comparison protocol, revalidated.
///
/// # Errors
///
/// Returns the [`ProtocolError`] of a protocol that does not hold.
pub fn embedded_comparison_protocol() -> Result<ComparisonProtocol, ProtocolError> {
    parse_comparison_protocol(EMBEDDED_PROTOCOL)
}

/// Parses and checks one comparison protocol.
///
/// # Errors
///
/// Returns the first [`ProtocolError`] that refuses the protocol.
// r[impl agent_harness_runtime.campaign_manager_eval.local_scorer.evaluation]
pub fn parse_comparison_protocol(text: &str) -> Result<ComparisonProtocol, ProtocolError> {
    let protocol: ComparisonProtocol =
        serde_json::from_str(text).map_err(|_| ProtocolError::Malformed)?;
    if protocol.schema != COMPARISON_SCHEMA {
        return Err(ProtocolError::Schema);
    }
    animus_eval_core::validate_blake3_identity(&protocol.artifact_identity)
        .map_err(|_| ProtocolError::ArtifactIdentity)?;
    let baseline = crate::FALLBACK_POLICY.identity();
    if protocol.baseline != baseline {
        return Err(ProtocolError::Baseline);
    }
    let is_scripted = |identity: &String| {
        crate::ScriptedPolicy::ALL
            .iter()
            .any(|policy| policy.identity() == identity)
    };
    if protocol
        .references
        .iter()
        .any(|identity| identity == baseline || !is_scripted(identity))
    {
        return Err(ProtocolError::References);
    }
    let mut seeds = protocol.random_seeds.clone();
    seeds.sort_unstable();
    seeds.dedup();
    if seeds.is_empty()
        || seeds.len() != protocol.random_seeds.len()
        || seeds.len() > MAX_RANDOM_SEEDS
    {
        return Err(ProtocolError::Seeds);
    }
    if protocol.primary_outcome != crate::METRIC_FOUND {
        return Err(ProtocolError::Outcome);
    }
    let rule = protocol.adoption;
    if rule.min_gain_over_baseline_per_mille > 1_000 || rule.max_fallback_per_mille > 1_000 {
        return Err(ProtocolError::Adoption);
    }
    if protocol.non_claims.is_empty() {
        return Err(ProtocolError::NonClaims);
    }
    debug_assert!(!protocol.random_seeds.is_empty());
    debug_assert!(protocol.random_seeds.len() <= MAX_RANDOM_SEEDS);
    debug_assert!(
        protocol
            .references
            .iter()
            .all(|identity| *identity != protocol.baseline)
    );
    Ok(protocol)
}

/// Applies `rule` to measured means.
#[must_use]
pub fn decide_adoption(rule: &AdoptionRule, input: &AdoptionInput) -> AdoptionDecision {
    let gain = f64::from(rule.min_gain_over_baseline_per_mille) / PER_MILLE;
    if input.scorer < input.baseline + gain {
        return AdoptionDecision::NoGo;
    }
    if rule.requires_random_parity && input.scorer < input.random {
        return AdoptionDecision::NoGo;
    }
    if input.scorer_fallback_per_mille > rule.max_fallback_per_mille {
        return AdoptionDecision::NoGo;
    }
    AdoptionDecision::Go
}
