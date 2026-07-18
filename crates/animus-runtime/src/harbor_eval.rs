//! Pure deterministic planning, ATIF projection, reward reduction, and receipts.

use std::collections::BTreeSet;

/// Maximum admitted trials in one local job.
pub const MAX_TRIALS: usize = 256;
/// Maximum admitted concurrency.
pub const MAX_CONCURRENCY: usize = 16;
/// Maximum ATIF steps.
pub const MAX_ATIF_STEPS: usize = 4_096;
/// Maximum tool calls or observations in one step.
pub const MAX_STEP_ITEMS: usize = 128;
/// Maximum safe identity length.
pub const MAX_IDENTITY_BYTES: usize = 256;

/// Explicit network posture.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NetworkPosture {
    /// No network access.
    None,
    /// Explicit bounded private network access.
    Private,
}

/// Immutable task profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Task<'a> {
    /// Task identity.
    pub identity: &'a str,
    /// Environment identity.
    pub environment: &'a str,
    /// Verifier identity.
    pub verifier: &'a str,
    /// Rubric identity.
    pub rubric: &'a str,
    /// Network posture.
    pub network: NetworkPosture,
    /// Separate verifier requirement.
    pub separate_verifier: bool,
}

/// Agent/model route pair.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Route<'a> {
    /// Agent identity.
    pub agent: &'a str,
    /// Model route identity.
    pub model: &'a str,
}

/// Deterministic trial plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrialPlan {
    /// Stable identity.
    pub identity: String,
    /// Task identity.
    pub task: String,
    /// Agent identity.
    pub agent: String,
    /// Model identity.
    pub model: String,
    /// Plan order.
    pub order: usize,
}

/// Planner admission error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanError {
    /// Empty, duplicate, or unsafe identity.
    Identity,
    /// Missing separate verifier isolation.
    VerifierIsolation,
    /// Empty or excessive matrix.
    TrialLimit,
    /// Invalid concurrency.
    Concurrency,
    /// Resume evidence did not bind a current trial.
    StaleResume,
}

/// Build a stable task-major, route-minor trial matrix.
///
/// # Errors
/// Returns a fail-closed planning error for malformed inputs, bounds, or stale resume entries.
pub fn plan_trials(
    tasks: &[Task<'_>],
    routes: &[Route<'_>],
    concurrency: usize,
    completed: &[&str],
) -> Result<Vec<TrialPlan>, PlanError> {
    if concurrency == 0 || concurrency > MAX_CONCURRENCY {
        return Err(PlanError::Concurrency);
    }
    if tasks.is_empty()
        || routes.is_empty()
        || tasks.len().saturating_mul(routes.len()) > MAX_TRIALS
    {
        return Err(PlanError::TrialLimit);
    }
    let mut identities = BTreeSet::new();
    let mut plans = Vec::new();
    for task in tasks {
        if !task.separate_verifier {
            return Err(PlanError::VerifierIsolation);
        }
        for value in [task.identity, task.environment, task.verifier, task.rubric] {
            if !safe(value) {
                return Err(PlanError::Identity);
            }
        }
        for route in routes {
            if !safe(route.agent) || !safe(route.model) {
                return Err(PlanError::Identity);
            }
            let identity = digest(
                "animus.harbor.trial.v1",
                &[
                    task.identity,
                    route.agent,
                    route.model,
                    task.environment,
                    task.verifier,
                    task.rubric,
                ],
            );
            if !identities.insert(identity.clone()) {
                return Err(PlanError::Identity);
            }
            plans.push(TrialPlan {
                identity,
                task: task.identity.into(),
                agent: route.agent.into(),
                model: route.model.into(),
                order: plans.len(),
            });
        }
    }
    if completed.iter().any(|value| !identities.contains(*value)) {
        return Err(PlanError::StaleResume);
    }
    plans.retain(|plan| !completed.contains(&plan.identity.as_str()));
    Ok(plans)
}

/// ATIF source role.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AtifSource {
    /// User input.
    User,
    /// Agent output.
    Agent,
    /// System event.
    System,
}

/// ATIF tool-call reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolCall<'a> {
    /// Call identity.
    pub id: &'a str,
    /// Function identity.
    pub function: &'a str,
}

/// ATIF observation reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservationRef<'a> {
    /// Source call identity when correlated.
    pub source_call_id: Option<&'a str>,
    /// Embedded subagent trajectory identity.
    pub subagent_trajectory_id: Option<&'a str>,
}

/// Bounded ATIF step facts; raw message/reasoning/tool payloads stay outside safe projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtifStep<'a> {
    /// One-based step identity.
    pub id: usize,
    /// Source role.
    pub source: AtifSource,
    /// Tool calls.
    pub calls: &'a [ToolCall<'a>],
    /// Observations.
    pub observations: &'a [ObservationRef<'a>],
    /// Whether hidden reasoning was present and omitted.
    pub hidden_reasoning_omitted: bool,
    /// Terminal marker.
    pub terminal: bool,
}

/// ATIF trajectory facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtifTrajectory<'a> {
    /// Exact schema version.
    pub schema: &'a str,
    /// Safe trajectory identity.
    pub identity: &'a str,
    /// Ordered steps.
    pub steps: &'a [AtifStep<'a>],
    /// Declared embedded subagent identities.
    pub subagents: &'a [&'a str],
}

/// Safe ATIF projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AtifProjection {
    /// Trajectory identity.
    pub identity: String,
    /// Step count.
    pub steps: usize,
    /// Tool-call count.
    pub calls: usize,
    /// Observation count.
    pub observations: usize,
    /// Hidden-reasoning redaction count.
    pub hidden_reasoning_redactions: usize,
}

/// ATIF validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AtifError {
    /// Unsupported version.
    Version,
    /// Invalid step sequence or missing terminal state.
    Sequence,
    /// Duplicate/orphan tool observation.
    Correlation,
    /// Invalid subagent lineage.
    Lineage,
    /// Collection or identity bound exceeded.
    Bound,
}

/// Validate exact `ATIF-v1.7` facts and project metadata only.
///
/// # Errors
/// Returns a deterministic schema, sequence, correlation, lineage, or bound error.
pub fn project_atif(value: &AtifTrajectory<'_>) -> Result<AtifProjection, AtifError> {
    if value.schema != animus_adapters::harbor::ATIF_SCHEMA {
        return Err(AtifError::Version);
    }
    if !safe(value.identity) || value.steps.is_empty() || value.steps.len() > MAX_ATIF_STEPS {
        return Err(AtifError::Bound);
    }
    let subagents: BTreeSet<_> = value.subagents.iter().copied().collect();
    if subagents.len() != value.subagents.len() || value.subagents.iter().any(|item| !safe(item)) {
        return Err(AtifError::Lineage);
    }
    let mut calls_total = 0usize;
    let mut observations_total = 0usize;
    let mut redactions = 0usize;
    for (index, step) in value.steps.iter().enumerate() {
        if step.id != index + 1
            || step.calls.len() > MAX_STEP_ITEMS
            || step.observations.len() > MAX_STEP_ITEMS
        {
            return Err(AtifError::Sequence);
        }
        let calls: BTreeSet<_> = step.calls.iter().map(|call| call.id).collect();
        if calls.len() != step.calls.len()
            || step
                .calls
                .iter()
                .any(|call| !safe(call.id) || !safe(call.function))
        {
            return Err(AtifError::Correlation);
        }
        for observation in step.observations {
            if observation
                .source_call_id
                .is_some_and(|id| !calls.contains(id))
            {
                return Err(AtifError::Correlation);
            }
            if observation
                .subagent_trajectory_id
                .is_some_and(|id| !subagents.contains(id))
            {
                return Err(AtifError::Lineage);
            }
        }
        calls_total += step.calls.len();
        observations_total += step.observations.len();
        redactions += usize::from(step.hidden_reasoning_omitted);
    }
    if !value.steps.last().is_some_and(|step| step.terminal) {
        return Err(AtifError::Sequence);
    }
    Ok(AtifProjection {
        identity: value.identity.into(),
        steps: value.steps.len(),
        calls: calls_total,
        observations: observations_total,
        hidden_reasoning_redactions: redactions,
    })
}

/// Attributed reward input.
#[derive(Clone, Debug, PartialEq)]
pub struct Reward<'a> {
    /// Trial identity.
    pub trial: &'a str,
    /// Metric name.
    pub metric: &'a str,
    /// Finite value.
    pub value: f64,
    /// Verifier identity.
    pub verifier: &'a str,
    /// Whether this is advisory judgment.
    pub advisory: bool,
}

/// Deterministic reward summary.
#[derive(Clone, Debug, PartialEq)]
pub struct RewardSummary {
    /// Metric name.
    pub metric: String,
    /// Ordered finite mean.
    pub mean: f64,
    /// Value count.
    pub count: u32,
    /// Advisory value count.
    pub advisory_count: u32,
}

/// Reward error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RewardError {
    /// Missing or malformed identity.
    Identity,
    /// Duplicate trial/metric reward.
    Duplicate,
    /// Non-finite value.
    NonFinite,
}

/// Reduce rewards in supplied plan order and metric order.
///
/// # Errors
/// Returns identity, duplicate, or non-finite failures without dropping invalid values.
pub fn reduce_rewards(rewards: &[Reward<'_>]) -> Result<Vec<RewardSummary>, RewardError> {
    let mut seen = BTreeSet::new();
    let mut metrics: Vec<RewardSummary> = Vec::new();
    for reward in rewards {
        if !safe(reward.trial) || !safe(reward.metric) || !safe(reward.verifier) {
            return Err(RewardError::Identity);
        }
        if !reward.value.is_finite() {
            return Err(RewardError::NonFinite);
        }
        if !seen.insert((reward.trial, reward.metric)) {
            return Err(RewardError::Duplicate);
        }
        if let Some(summary) = metrics.iter_mut().find(|item| item.metric == reward.metric) {
            summary.mean = (summary.mean * f64::from(summary.count) + reward.value)
                / f64::from(summary.count + 1);
            summary.count += 1;
            summary.advisory_count += u32::from(reward.advisory);
        } else {
            metrics.push(RewardSummary {
                metric: reward.metric.into(),
                mean: reward.value,
                count: 1,
                advisory_count: u32::from(reward.advisory),
            });
        }
    }
    Ok(metrics)
}

/// Metadata-only evaluation receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluationReceipt {
    /// Job identity.
    pub job_identity: String,
    /// Planned trials.
    pub planned: usize,
    /// Completed trials.
    pub completed: usize,
    /// ATIF projection identities.
    pub trajectories: Vec<String>,
    /// Explicit non-claims.
    pub non_claims: Vec<&'static str>,
}

/// Project a bounded receipt from explicit current facts.
///
/// # Errors
/// Returns an identity error for unsafe job identities or impossible completion counts.
pub fn receipt(
    job: &str,
    plans: &[TrialPlan],
    completed: usize,
    trajectories: &[AtifProjection],
) -> Result<EvaluationReceipt, PlanError> {
    if !safe(job) || completed > plans.len() {
        return Err(PlanError::Identity);
    }
    Ok(EvaluationReceipt {
        job_identity: job.into(),
        planned: plans.len(),
        completed,
        trajectories: trajectories
            .iter()
            .map(|item| item.identity.clone())
            .collect(),
        non_claims: vec![
            "not-verifier-soundness",
            "not-benchmark-superiority",
            "not-automatic-completion",
            "not-production-or-release-readiness",
        ],
    })
}

fn safe(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= MAX_IDENTITY_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-_.:".contains(&byte))
}
fn digest(domain: &str, values: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    for value in values {
        hasher.update(&[0]);
        hasher.update(value.as_bytes());
    }
    format!("b3:{}", hasher.finalize().to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn vertical_slice_is_deterministic_and_metadata_only() {
        let tasks = [Task {
            identity: "task:1",
            environment: "env:1",
            verifier: "verifier:1",
            rubric: "rubric:1",
            network: NetworkPosture::None,
            separate_verifier: true,
        }];
        let routes = [Route {
            agent: "agent:1",
            model: "model:1",
        }];
        let plans = plan_trials(&tasks, &routes, 1, &[]).expect("plan");
        let calls = [ToolCall {
            id: "call:1",
            function: "tool:fixture",
        }];
        let observations = [ObservationRef {
            source_call_id: Some("call:1"),
            subagent_trajectory_id: None,
        }];
        let steps = [AtifStep {
            id: 1,
            source: AtifSource::Agent,
            calls: &calls,
            observations: &observations,
            hidden_reasoning_omitted: true,
            terminal: true,
        }];
        let projection = project_atif(&AtifTrajectory {
            schema: "ATIF-v1.7",
            identity: "trajectory:1",
            steps: &steps,
            subagents: &[],
        })
        .expect("atif");
        let rewards = reduce_rewards(&[Reward {
            trial: &plans[0].identity,
            metric: "correctness",
            value: 1.0,
            verifier: "verifier:1",
            advisory: false,
        }])
        .expect("reward");
        assert!((rewards[0].mean - 1.0).abs() < f64::EPSILON);
        assert_eq!(projection.hidden_reasoning_redactions, 1);
        assert_eq!(
            receipt("job:1", &plans, 1, &[projection])
                .expect("receipt")
                .completed,
            1
        );
    }
    #[test]
    fn planning_atif_rewards_and_receipts_fail_closed() {
        let task = Task {
            identity: "task:1",
            environment: "env:1",
            verifier: "verifier:1",
            rubric: "rubric:1",
            network: NetworkPosture::None,
            separate_verifier: false,
        };
        assert_eq!(
            plan_trials(
                &[task],
                &[Route {
                    agent: "agent:1",
                    model: "model:1"
                }],
                1,
                &[]
            ),
            Err(PlanError::VerifierIsolation)
        );
        let steps = [AtifStep {
            id: 2,
            source: AtifSource::Agent,
            calls: &[],
            observations: &[],
            hidden_reasoning_omitted: false,
            terminal: false,
        }];
        assert_eq!(
            project_atif(&AtifTrajectory {
                schema: "ATIF-v1.6",
                identity: "trajectory:1",
                steps: &steps,
                subagents: &[]
            }),
            Err(AtifError::Version)
        );
        assert_eq!(
            reduce_rewards(&[Reward {
                trial: "trial:1",
                metric: "score",
                value: f64::NAN,
                verifier: "verifier:1",
                advisory: true
            }]),
            Err(RewardError::NonFinite)
        );
        assert_eq!(
            receipt("/secret/path", &[], 1, &[]),
            Err(PlanError::Identity)
        );
    }
}
