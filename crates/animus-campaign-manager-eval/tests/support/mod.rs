//! Shared support of the evaluation-lane tests.

#![allow(
    dead_code,
    reason = "each integration-test binary uses a subset of the shared support"
)]

/// Why a test step failed.
#[derive(Debug)]
pub enum Failure {
    /// The task set was refused.
    Task(animus_campaign_manager_eval::TaskError),
    /// The job could not run.
    Job(animus_campaign_manager_eval::JobError),
    /// A host step of a trial failed.
    Host(animus_campaign_manager_eval::HostFailure),
    /// A trajectory was refused.
    Atif(animus_runtime::harbor_eval::AtifError),
    /// JSON could not be read or written.
    Json,
    /// A step had another outcome than the test expects.
    Unexpected(&'static str),
}

impl From<animus_campaign_manager_eval::TaskError> for Failure {
    fn from(error: animus_campaign_manager_eval::TaskError) -> Self {
        Self::Task(error)
    }
}

impl From<animus_campaign_manager_eval::JobError> for Failure {
    fn from(error: animus_campaign_manager_eval::JobError) -> Self {
        Self::Job(error)
    }
}

impl From<animus_campaign_manager_eval::HostFailure> for Failure {
    fn from(error: animus_campaign_manager_eval::HostFailure) -> Self {
        Self::Host(error)
    }
}

impl From<animus_runtime::harbor_eval::AtifError> for Failure {
    fn from(error: animus_runtime::harbor_eval::AtifError) -> Self {
        Self::Atif(error)
    }
}

impl From<serde_json::Error> for Failure {
    fn from(_: serde_json::Error) -> Self {
        Self::Json
    }
}

/// The embedded task set as JSON, for tests that change one field.
pub fn task_set_json() -> Result<serde_json::Value, Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let text = include_str!("../../../../config/generated/campaign-manager-eval-tasks.json");
    let value: serde_json::Value = serde_json::from_str(text)?;
    assert_eq!(
        value["tasks"].as_array().map(Vec::len),
        Some(set.tasks.len())
    );
    Ok(value)
}

/// The refusal of the task set `value`, or `None` when it is admitted.
pub fn refusal(
    value: &serde_json::Value,
) -> Result<Option<animus_campaign_manager_eval::TaskError>, Failure> {
    let text = serde_json::to_string(value)?;
    Ok(animus_campaign_manager_eval::parse_task_set(&text).err())
}

/// The task with `identity` of `set`.
pub fn task<'set>(
    set: &'set animus_campaign_manager_eval::TaskSet,
    identity: &str,
) -> Result<&'set animus_campaign_manager_eval::EvalTask, Failure> {
    set.task(identity)
        .ok_or(Failure::Unexpected("the task set lacks the task"))
}

/// `route`'s summary in `summary`.
pub fn route<'job>(
    summary: &'job animus_campaign_manager_eval::JobReport,
    route: &str,
) -> Result<&'job animus_campaign_manager_eval::RouteSummary, Failure> {
    summary
        .routes
        .iter()
        .find(|entry| entry.route == route)
        .ok_or(Failure::Unexpected("the report lacks the route"))
}

/// The mean of `metric` in `route`.
pub fn mean(
    route: &animus_campaign_manager_eval::RouteSummary,
    metric: &str,
) -> Result<f64, Failure> {
    route
        .metrics
        .iter()
        .find(|entry| entry.metric == metric)
        .map(|entry| entry.mean)
        .ok_or(Failure::Unexpected("the route lacks the metric"))
}

/// Whether `value` is 1 within float tolerance.
pub fn is_one(value: f64) -> bool {
    (value - 1.0).abs() < f64::EPSILON
}

/// Whether `value` is 0 within float tolerance.
pub fn is_zero(value: f64) -> bool {
    value.abs() < f64::EPSILON
}

/// The identities one trial request names.
pub struct TrialNames<'name> {
    /// Trial identity.
    pub trial: &'name str,
    /// Task identity.
    pub task: &'name str,
    /// Model-route identity.
    pub route: &'name str,
}

/// The trial request of `names` under `set`.
pub fn request(
    set: &animus_campaign_manager_eval::TaskSet,
    names: &TrialNames<'_>,
) -> animus_adapters::harbor::TrialRequest {
    animus_adapters::harbor::TrialRequest {
        trial_identity: String::from(names.trial),
        task_identity: String::from(names.task),
        agent_identity: set.agent.clone(),
        model_route_identity: String::from(names.route),
        environment_identity: set.environment.clone(),
        verifier_identity: set.verifier.clone(),
        artifacts: animus_campaign_manager_eval::ARTIFACT_NAMES
            .map(String::from)
            .to_vec(),
    }
}

/// A route that cannot be reached.
pub struct Offline;

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for Offline
{
    type Error = animus_campaign_manager_eval::RouteFailure;

    fn call(
        &mut self,
        _request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        Err(animus_campaign_manager_eval::RouteFailure::Unavailable)
    }
}
