//! Outcome counts and measures of one route, from the job's existing metrics owners.
//!
//! Accepted completions come from the separate verifier's reduced `found` reward. Refused trials
//! stay in the denominator as failed, cancelled or incomplete, and every unmeasured cost or time
//! stays unknown.

/// Cost unit the lane would report; the scripted lane measures no cost.
const COST_UNIT: &str = "campaign-cost-units";
/// Absolute tolerance when counting accepted trials from the reduced mean.
const COUNT_TOLERANCE: f64 = 1e-9;

/// Outcome counts of the route over its planned trials.
pub(super) fn outcomes(
    report: &crate::JobReport,
    summary: &crate::RouteSummary,
    route: &str,
    planned: usize,
) -> Result<animus_runtime::qualification::Outcomes, super::QualificationError> {
    let completed = u64::from(summary.completed);
    let accepted = accepted(summary)?;
    let mut counts = animus_runtime::qualification::Outcomes {
        planned: u64::try_from(planned).map_err(|_| super::QualificationError::Bound)?,
        accepted,
        rejected: completed
            .checked_sub(accepted)
            .ok_or(super::QualificationError::Bound)?,
        failed: 0,
        cancelled: 0,
        incomplete: 0,
    };
    for failure in report
        .failures
        .iter()
        .filter(|failure| failure.route == route)
    {
        let slot = match failure_class(failure.error) {
            FailureClass::Cancelled => &mut counts.cancelled,
            FailureClass::Incomplete => &mut counts.incomplete,
            FailureClass::Failed => &mut counts.failed,
        };
        *slot = slot
            .checked_add(1)
            .ok_or(super::QualificationError::Bound)?;
    }
    debug_assert!(counts.accepted <= completed);
    debug_assert!(counts.rejected <= completed);
    Ok(counts)
}

/// Accepted trials: the verifier's binary `found` reward, summed through its reduced mean.
fn accepted(summary: &crate::RouteSummary) -> Result<u64, super::QualificationError> {
    let Some(found) = summary
        .metrics
        .iter()
        .find(|metric| metric.metric == crate::METRIC_FOUND)
    else {
        return if summary.completed == 0 {
            Ok(0)
        } else {
            Err(super::QualificationError::InexactAcceptedCount)
        };
    };
    let total = found.mean * f64::from(found.count);
    let exact = (0..=found.count).find(|count| (f64::from(*count) - total).abs() < COUNT_TOLERANCE);
    debug_assert!(found.count <= summary.completed);
    debug_assert!(total.is_finite());
    exact
        .map(u64::from)
        .ok_or(super::QualificationError::InexactAcceptedCount)
}

/// Outcome class of one refused trial.
enum FailureClass {
    Cancelled,
    Incomplete,
    Failed,
}

fn failure_class(error: animus_std::harbor::HostError) -> FailureClass {
    match error {
        animus_std::harbor::HostError::Environment(class)
        | animus_std::harbor::HostError::Verifier(class) => match class {
            animus_adapters::harbor::OutcomeClass::Cancelled => FailureClass::Cancelled,
            animus_adapters::harbor::OutcomeClass::TimedOut => FailureClass::Incomplete,
            animus_adapters::harbor::OutcomeClass::Completed
            | animus_adapters::harbor::OutcomeClass::Unavailable
            | animus_adapters::harbor::OutcomeClass::Denied
            | animus_adapters::harbor::OutcomeClass::Malformed
            | animus_adapters::harbor::OutcomeClass::Stale => FailureClass::Failed,
        },
        animus_std::harbor::HostError::EvidenceMismatch
        | animus_std::harbor::HostError::SharedVerifier => FailureClass::Failed,
    }
}

/// Coverage and measures; every unmeasured cost and time stays unknown.
pub(super) fn measures(
    set: &crate::TaskSet,
    outcomes: &animus_runtime::qualification::Outcomes,
) -> Result<animus_runtime::qualification::Measures, super::QualificationError> {
    let tasks_covered = outcomes
        .accepted
        .checked_add(outcomes.rejected)
        .ok_or(super::QualificationError::Bound)?;
    let tasks_declared =
        u64::try_from(set.tasks.len()).map_err(|_| super::QualificationError::Bound)?;
    debug_assert!(outcomes.accepted <= tasks_covered);
    debug_assert!(tasks_covered <= outcomes.planned);
    Ok(animus_runtime::qualification::Measures {
        repetitions: super::REPETITIONS,
        tasks_covered,
        tasks_declared,
        time_to_acceptance_ms: None,
        covered_time: animus_runtime::qualification::CoveredTime {
            queue_ms: None,
            repair_ms: None,
            verification_ms: None,
        },
        costs: animus_runtime::qualification::Costs {
            unit: String::from(COST_UNIT),
            model: None,
            tool: None,
            manager: None,
            verifier: None,
            unknown_remainder: true,
        },
        review_interventions: None,
        invalid_actions: None,
        false_completion_attempts: None,
    })
}
