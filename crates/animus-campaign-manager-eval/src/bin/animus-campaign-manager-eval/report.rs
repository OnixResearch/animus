//! The comparison's metadata-only report: per-arm completion, rewards and latency, the scorer's
//! dispositions and cost units, and the adoption decision with its non-claims.

/// Per-mille scale of the fallback share.
const PER_MILLE: u32 = 1_000;

/// What one arm is in the comparison.
#[derive(Clone, Copy)]
enum Role {
    /// The grant-matched baseline.
    Baseline,
    /// A broader-grant reference, which the decision does not compare.
    Reference,
    /// The frozen local scorer.
    Scorer,
    /// One seeded random-ranking arm.
    Random,
}

impl Role {
    const fn label(self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Reference => "reference(broader-grant,non-comparable)",
            Self::Scorer => "scorer",
            Self::Random => "random",
        }
    }
}

/// Renders the report of `finished`, the job of `arms` under `protocol`.
pub(super) fn render(
    protocol: &animus_campaign_manager_eval::ComparisonProtocol,
    finished: &animus_campaign_manager_eval::JobReport,
    arms: &super::Arms,
) -> Result<String, String> {
    let input = adoption_input(finished, arms)?;
    let decision = match animus_campaign_manager_eval::decide_adoption(&protocol.adoption, &input) {
        animus_campaign_manager_eval::AdoptionDecision::Go => "go",
        animus_campaign_manager_eval::AdoptionDecision::NoGo => "no-go",
    };
    let mut lines = vec![
        String::from("LOCAL_SCORER_COMPARISON status=completed"),
        format!(
            "job={} protocol={}",
            finished.receipt.job_identity, protocol.schema
        ),
        format!("artifact={} provenance=fixture", protocol.artifact_identity),
        format!(
            "planned={} completed={} failed={}",
            finished.receipt.planned,
            finished.receipt.completed,
            finished.failures.len()
        ),
    ];
    lines.extend(arm_lines(finished, arms));
    lines.extend(finished.failures.iter().map(|failure| {
        format!(
            "failure task={} route={} error={:?}",
            failure.task, failure.route, failure.error
        )
    }));
    lines.push(format!(
        "decision={decision} rule=gain>={}permille,random-parity={},fallback<={}permille scorer={:.3} baseline={:.3} random_mean={:.3} fallback={}permille",
        protocol.adoption.min_gain_over_baseline_per_mille,
        protocol.adoption.requires_random_parity,
        protocol.adoption.max_fallback_per_mille,
        input.scorer,
        input.baseline,
        input.random,
        input.scorer_fallback_per_mille
    ));
    lines.push(String::from(
        "prompts=omitted replies=omitted labels=omitted latency=non-deterministic",
    ));
    lines.push(format!("non_claims={}", protocol.non_claims.join(",")));
    lines.push(String::new());
    debug_assert!(lines.iter().all(|line| !line.contains('\n')));
    debug_assert!(input.scorer_fallback_per_mille <= PER_MILLE);
    Ok(lines.join("\n"))
}

/// One line per arm in job order, with the scorer's dispositions after its own line.
fn arm_lines(
    finished: &animus_campaign_manager_eval::JobReport,
    arms: &super::Arms,
) -> Vec<String> {
    let mut lines = vec![arm_line(finished, Role::Baseline, &arms.baseline)];
    lines.extend(
        arms.references
            .iter()
            .map(|reference| arm_line(finished, Role::Reference, reference)),
    );
    lines.push(arm_line(finished, Role::Scorer, &arms.scorer));
    lines.push(scorer_line(arms.scorer.inner.decisions()));
    lines.extend(
        arms.randoms
            .iter()
            .map(|random| arm_line(finished, Role::Random, random)),
    );
    lines
}

/// One arm's completion, mean rewards, and latency.
fn arm_line<A>(
    finished: &animus_campaign_manager_eval::JobReport,
    role: Role,
    arm: &super::Timed<A>,
) -> String {
    let summary = finished
        .routes
        .iter()
        .find(|route| route.route == arm.identity);
    let completed = summary.map_or(0, |route| route.completed);
    let metrics: Vec<String> = summary
        .map(|route| {
            route
                .metrics
                .iter()
                .map(|metric| format!("{}={:.3}", metric.metric, metric.mean))
                .collect()
        })
        .unwrap_or_default();
    let (p50, p99, max) = percentiles(&arm.latencies);
    debug_assert!(p50 <= p99);
    debug_assert!(p99 <= max);
    format!(
        "arm role={} route={} completed={completed} {} latency_ns_p50={} latency_ns_p99={} latency_ns_max={}",
        role.label(),
        arm.identity,
        metrics.join(" "),
        p50.as_nanos(),
        p99.as_nanos(),
        max.as_nanos()
    )
}

/// The scorer's dispositions and summed deterministic cost units.
fn scorer_line(decisions: &[animus_campaign_manager_eval::DecisionRecord]) -> String {
    let with = |disposition: animus_campaign_manager_eval::Disposition| {
        decisions
            .iter()
            .filter(|record| record.disposition == disposition)
            .count()
    };
    let total = |cost: fn(&animus_campaign_manager_eval::CostUnits) -> u64| {
        decisions
            .iter()
            .map(|record| cost(&record.cost))
            .fold(0_u64, u64::saturating_add)
    };
    debug_assert!(decisions.len() <= animus_campaign_manager_eval::MAX_DECISIONS);
    debug_assert!(
        decisions.iter().all(|record| {
            (record.disposition == animus_campaign_manager_eval::Disposition::Scored)
                == record.fallback.is_none()
        }),
        "exactly the unscored decisions name a fallback"
    );
    format!(
        "scorer decisions={} scored={} malformed_brief={} out_of_range={} operation_budget={} incompatible_artifact={} unavailable={} brief_bytes={} feature_operations={} inference_operations={} fallback_candidates={}",
        decisions.len(),
        with(animus_campaign_manager_eval::Disposition::Scored),
        with(animus_campaign_manager_eval::Disposition::MalformedBrief),
        with(animus_campaign_manager_eval::Disposition::OutOfRange),
        with(animus_campaign_manager_eval::Disposition::OperationBudget),
        with(animus_campaign_manager_eval::Disposition::IncompatibleArtifact),
        with(animus_campaign_manager_eval::Disposition::Unavailable),
        total(|cost| cost.brief_bytes),
        total(|cost| cost.feature_operations),
        total(|cost| cost.inference_operations),
        total(|cost| cost.fallback_candidates),
    )
}

/// The measured means and fallback share that the adoption rule compares.
fn adoption_input(
    finished: &animus_campaign_manager_eval::JobReport,
    arms: &super::Arms,
) -> Result<animus_campaign_manager_eval::AdoptionInput, String> {
    let randoms = arms
        .randoms
        .iter()
        .map(|random| found_mean(finished, &random.identity))
        .collect::<Result<Vec<f64>, String>>()?;
    Ok(animus_campaign_manager_eval::AdoptionInput {
        scorer: found_mean(finished, &arms.scorer.identity)?,
        baseline: found_mean(finished, &arms.baseline.identity)?,
        random: mean(&randoms)?,
        scorer_fallback_per_mille: fallback_per_mille(arms.scorer.inner.decisions())?,
    })
}

/// The mean primary outcome of the route `identity` in `finished`.
fn found_mean(
    finished: &animus_campaign_manager_eval::JobReport,
    identity: &str,
) -> Result<f64, String> {
    finished
        .routes
        .iter()
        .find(|route| route.route == identity)
        .and_then(|route| {
            route
                .metrics
                .iter()
                .find(|metric| metric.metric == animus_campaign_manager_eval::METRIC_FOUND)
        })
        .map(|metric| metric.mean)
        .ok_or_else(|| format!("route {identity} completed no trial"))
}

/// The mean of `values`, which holds one entry per random arm.
fn mean(values: &[f64]) -> Result<f64, String> {
    let count = u32::try_from(values.len()).map_err(|_| String::from("too many random arms"))?;
    if count == 0 {
        return Err(String::from("no random arm"));
    }
    Ok(values.iter().sum::<f64>() / f64::from(count))
}

/// The share of `decisions` that fell back, per mille, rounding down; 0 without decisions.
fn fallback_per_mille(
    decisions: &[animus_campaign_manager_eval::DecisionRecord],
) -> Result<u32, String> {
    let refused = || String::from("the fallback share exceeds its integer range");
    let fallback_count = decisions
        .iter()
        .filter(|record| record.fallback.is_some())
        .count();
    let part = u32::try_from(fallback_count).map_err(|_| refused())?;
    let whole = u32::try_from(decisions.len()).map_err(|_| refused())?;
    if whole == 0 {
        return Ok(0);
    }
    part.checked_mul(PER_MILLE)
        .and_then(|scaled| scaled.checked_div(whole))
        .ok_or_else(refused)
}

/// The 50th and 99th percentile and maximum of `samples`, nearest-rank; zero when empty.
fn percentiles(
    samples: &[std::time::Duration],
) -> (
    std::time::Duration,
    std::time::Duration,
    std::time::Duration,
) {
    let mut sorted = samples.to_vec();
    sorted.sort_unstable();
    let at = |quantile_per_mille: usize| {
        let rank = sorted
            .len()
            .saturating_mul(quantile_per_mille)
            .div_ceil(1_000)
            .saturating_sub(1);
        sorted.get(rank).copied().unwrap_or_default()
    };
    (at(500), at(990), sorted.last().copied().unwrap_or_default())
}
