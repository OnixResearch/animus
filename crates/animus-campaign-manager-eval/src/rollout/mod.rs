//! The Harbor environment service: one trial alternates manager turns with host planner steps
//! on a scenario campaign.

mod advising;
mod hosting;

/// Turn records reserved per trial: the most turns a task may run.
const TURN_CAPACITY: usize = 24;

/// What one manager turn produced.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TurnOutcome {
    /// Campaign admitted the advice and the host published its events.
    Admitted {
        /// Control events published.
        events: u32,
    },
    /// The reply asked for no change, so Campaign admitted nothing.
    Unchanged,
    /// The manager or Campaign refused the reply; nothing was published.
    Refused,
    /// The model route failed to answer.
    ModelFailed,
}

/// What the host did after a manager turn.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostAction {
    /// The planner selected a candidate and the host ran it and published the result.
    Selected {
        /// The signal the run revealed.
        signal: u32,
        /// Whether the run recorded the task's finding.
        is_target: bool,
    },
    /// The planner had nothing to select yet.
    Waited,
    /// The planner stopped the campaign and the host published the stop.
    Stopped,
}

/// One turn of a trial.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TurnRecord {
    /// One-based turn number.
    pub turn: u32,
    /// The manager's turn.
    pub outcome: TurnOutcome,
    /// The host's step, absent when the manager stopped the campaign.
    pub action: Option<HostAction>,
}

/// A finished trial: its campaign and its turns.
#[derive(Clone, Debug)]
pub struct TrialRun {
    /// The trial's campaign after its last turn.
    pub campaign: crate::TrialCampaign,
    /// Every turn in order.
    pub turns: Vec<TurnRecord>,
}

/// Runs `task` as trial `trial`: up to `task.turns` manager turns with `model`, each followed by
/// one host planner step, until the campaign stops.
///
/// # Errors
///
/// Returns the [`crate::HostFailure`] of a host step Campaign or the session refused.
// r[impl agent_harness_runtime.campaign_manager_eval.trial]
pub fn run_turns<M: animus_adapters::ModelAdapter>(
    set: &crate::TaskSet,
    task: &crate::EvalTask,
    trial: &str,
    model: &mut M,
) -> Result<TrialRun, crate::HostFailure> {
    let mut campaign = crate::TrialCampaign::open(task, trial)?;
    let setup = advising::manager_setup(set, task, campaign.session.state())?;
    let profile = hosting::selection_profile(campaign.session.state())?;
    let mut turns = Vec::with_capacity(TURN_CAPACITY);
    for turn in 1..=task.turns {
        if campaign.session.state().stopped().is_some() {
            break;
        }
        let outcome = advising::manager_turn(&mut campaign, &setup, model)?;
        let action = if campaign.session.state().stopped().is_some() {
            None
        } else {
            Some(hosting::host_step(&mut campaign, &profile)?)
        };
        turns.push(TurnRecord {
            turn,
            outcome,
            action,
        });
    }
    debug_assert!(turns.len() <= TURN_CAPACITY);
    debug_assert!(!turns.is_empty());
    Ok(TrialRun { campaign, turns })
}

/// The Harbor environment service of the campaign-manager lane.
pub struct CampaignEnvironment<'job, 'model> {
    set: &'job crate::TaskSet,
    routes: &'job mut [crate::Route<'model>],
    store: &'job crate::ArtifactStore,
}

impl<'job, 'model> CampaignEnvironment<'job, 'model> {
    /// An environment over `set`'s tasks and `routes`, recording artifacts into `store`.
    pub const fn new(
        set: &'job crate::TaskSet,
        routes: &'job mut [crate::Route<'model>],
        store: &'job crate::ArtifactStore,
    ) -> Self {
        Self { set, routes, store }
    }
}

impl animus_adapters::harbor::EvaluationEnvironmentService for CampaignEnvironment<'_, '_> {
    fn execute(
        &mut self,
        request: &animus_adapters::harbor::TrialRequest,
    ) -> Result<animus_adapters::harbor::TrialObservation, animus_adapters::harbor::OutcomeClass>
    {
        let task = self
            .set
            .task(&request.task_identity)
            .ok_or(animus_adapters::harbor::OutcomeClass::Denied)?;
        let route = self
            .routes
            .iter_mut()
            .find(|route| route.identity == request.model_route_identity)
            .ok_or(animus_adapters::harbor::OutcomeClass::Unavailable)?;
        let mut model = crate::RouteModel(&mut *route.model);
        let run = run_turns(self.set, task, &request.trial_identity, &mut model)
            .map_err(|_| animus_adapters::harbor::OutcomeClass::Malformed)?;
        let trajectory = crate::project_turns(&request.trial_identity, &run.turns)
            .map_err(|_| animus_adapters::harbor::OutcomeClass::Malformed)?;
        let trajectory_identity = trajectory.identity.clone();
        let artifact = crate::TrialArtifact::capture(&run, trajectory);
        self.store.insert(&request.trial_identity, artifact);
        Ok(animus_adapters::harbor::TrialObservation {
            trial_identity: request.trial_identity.clone(),
            outcome: animus_adapters::harbor::OutcomeClass::Completed,
            trajectory_identity: Some(trajectory_identity),
            rewards: Vec::new(),
            artifacts: request.artifacts.clone(),
        })
    }
}
