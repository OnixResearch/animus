//! One evaluation job: the task × route trial matrix, run through Animus's Harbor slice.

/// Trials planned to run at once; the in-memory lane runs them in order.
const CONCURRENCY: u32 = 1;
/// Artifact identities every trial request declares.
pub const ARTIFACT_NAMES: [&str; 2] = ["artifact:control-history", "artifact:turn-ledger"];

/// A trial that did not complete.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TrialFailure {
    /// Trial identity.
    pub trial: String,
    /// Task identity.
    pub task: String,
    /// Model-route identity.
    pub route: String,
    /// The host's refusal.
    pub error: animus_std::harbor::HostError,
}

/// One route's reduced rewards.
#[derive(Clone, Debug, PartialEq)]
pub struct RouteSummary {
    /// Model-route identity.
    pub route: String,
    /// Trials of the route that completed.
    pub completed: u32,
    /// Mean reward per metric, in metric order.
    pub metrics: Vec<animus_runtime::harbor_eval::RewardSummary>,
}

/// A finished job.
#[derive(Clone, Debug, PartialEq)]
pub struct JobReport {
    /// The metadata-only evaluation receipt.
    pub receipt: animus_runtime::harbor_eval::EvaluationReceipt,
    /// Per-route reward summaries, in route order.
    pub routes: Vec<RouteSummary>,
    /// Trials that did not complete.
    pub failures: Vec<TrialFailure>,
}

/// Why a job could not run.
#[derive(Clone, Debug, PartialEq)]
pub enum JobError {
    /// The trial matrix could not be planned.
    Plan(animus_runtime::harbor_eval::PlanError),
    /// The verifier's rewards could not be reduced.
    Reward(animus_runtime::harbor_eval::RewardError),
}

/// A completed trial's plan and verifier rewards.
struct Graded {
    trial: String,
    route: String,
    rewards: Vec<(String, f64)>,
}

/// Runs every task of `set` against every route in `routes` as job `job`.
///
/// # Errors
///
/// Returns [`JobError::Plan`] for an unplannable matrix and [`JobError::Reward`] for rewards the
/// reducer refuses. A trial whose environment or verifier refuses is reported in
/// [`JobReport::failures`].
// r[impl agent_harness_runtime.campaign_manager_eval.job]
pub fn run_job(
    set: &crate::TaskSet,
    routes: &mut [crate::Route<'_>],
    job: &str,
) -> Result<JobReport, JobError> {
    let identities: Vec<String> = routes.iter().map(|route| route.identity.clone()).collect();
    let plans = plan(set, &identities)?;
    let store = crate::ArtifactStore::new();
    let mut environment = crate::CampaignEnvironment::new(set, routes, &store);
    let mut verifier = crate::CampaignVerifier::new(set, &store);
    let mut graded = Vec::with_capacity(plans.len());
    let mut failures = Vec::with_capacity(plans.len());
    for plan in &plans {
        let request = request(set, plan);
        match animus_std::harbor::execute_separate_verifier(
            &request,
            &mut environment,
            &mut verifier,
        ) {
            Ok((_subject, verdict)) => graded.push(Graded {
                trial: plan.identity.clone(),
                route: plan.model.clone(),
                rewards: verdict.rewards,
            }),
            Err(error) => failures.push(TrialFailure {
                trial: plan.identity.clone(),
                task: plan.task.clone(),
                route: plan.model.clone(),
                error,
            }),
        }
    }
    let summaries = identities
        .iter()
        .map(|route| summarize(set, route, &graded))
        .collect::<Result<Vec<_>, _>>()?;
    let trajectories: Vec<animus_runtime::harbor_eval::AtifProjection> = graded
        .iter()
        .filter_map(|trial| store.get(&trial.trial).map(|artifact| artifact.trajectory))
        .collect();
    let completed = u32::try_from(graded.len())
        .map_err(|_| JobError::Plan(animus_runtime::harbor_eval::PlanError::TrialLimit))?;
    let receipt = animus_runtime::harbor_eval::receipt(job, &plans, completed, &trajectories)
        .map_err(JobError::Plan)?;
    debug_assert_eq!(graded.len().saturating_add(failures.len()), plans.len());
    debug_assert_eq!(trajectories.len(), graded.len());
    Ok(JobReport {
        receipt,
        routes: summaries,
        failures,
    })
}

/// The task-major, route-minor trial matrix.
fn plan(
    set: &crate::TaskSet,
    identities: &[String],
) -> Result<Vec<animus_runtime::harbor_eval::TrialPlan>, JobError> {
    let tasks: Vec<animus_runtime::harbor_eval::Task<'_>> = set
        .tasks
        .iter()
        .map(|task| animus_runtime::harbor_eval::Task {
            identity: &task.identity,
            environment: &set.environment,
            verifier: &set.verifier,
            rubric: &set.rubric,
            network: animus_runtime::harbor_eval::NetworkPosture::None,
            separate_verifier: true,
        })
        .collect();
    let routes: Vec<animus_runtime::harbor_eval::Route<'_>> = identities
        .iter()
        .map(|model| animus_runtime::harbor_eval::Route {
            agent: &set.agent,
            model,
        })
        .collect();
    debug_assert_eq!(tasks.len(), set.tasks.len());
    debug_assert_eq!(routes.len(), identities.len());
    animus_runtime::harbor_eval::plan_trials(&tasks, &routes, CONCURRENCY, &[])
        .map_err(JobError::Plan)
}

/// The Harbor trial request of `plan`.
fn request(
    set: &crate::TaskSet,
    plan: &animus_runtime::harbor_eval::TrialPlan,
) -> animus_adapters::harbor::TrialRequest {
    animus_adapters::harbor::TrialRequest {
        trial_identity: plan.identity.clone(),
        task_identity: plan.task.clone(),
        agent_identity: plan.agent.clone(),
        model_route_identity: plan.model.clone(),
        environment_identity: set.environment.clone(),
        verifier_identity: set.verifier.clone(),
        artifacts: ARTIFACT_NAMES.map(String::from).to_vec(),
    }
}

/// `route`'s completed trials and mean rewards.
fn summarize(
    set: &crate::TaskSet,
    route: &str,
    graded: &[Graded],
) -> Result<RouteSummary, JobError> {
    let trials: Vec<&Graded> = graded.iter().filter(|trial| trial.route == route).collect();
    let rewards: Vec<animus_runtime::harbor_eval::Reward<'_>> = trials
        .iter()
        .flat_map(|trial| {
            trial
                .rewards
                .iter()
                .map(|(metric, value)| animus_runtime::harbor_eval::Reward {
                    trial: &trial.trial,
                    metric,
                    verifier: &set.verifier,
                    value: *value,
                    advisory: metric == crate::METRIC_REFUSALS,
                })
        })
        .collect();
    debug_assert!(trials.iter().all(|trial| trial.route == route));
    let metrics =
        animus_runtime::harbor_eval::reduce_rewards(&rewards).map_err(JobError::Reward)?;
    debug_assert!(metrics.len() <= rewards.len());
    Ok(RouteSummary {
        route: String::from(route),
        completed: u32::try_from(trials.len())
            .map_err(|_| JobError::Plan(animus_runtime::harbor_eval::PlanError::TrialLimit))?,
        metrics,
    })
}
