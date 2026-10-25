//! The separate verifier: it reloads the trial's published control projection from its Choregraph
//! history and grades the replayed state.
//!
//! `found` and `efficiency` come only from Campaign's replay of the control history. The
//! environment's claims do not count; a final state that differs from the replay refuses the
//! trial as stale. `refusal-rate` comes from the environment's turn ledger, so a job reports it
//! as advisory.

/// 1 when the replayed campaign recorded the task's finding from a run of its target, else 0.
pub const METRIC_FOUND: &str = "found";
/// For a found target, `(budget + 1 - selections) / budget`; else 0.
pub const METRIC_EFFICIENCY: &str = "efficiency";
/// Share of turns whose reply was refused or whose model failed (advisory); a reply that asks
/// for no change is neither.
pub const METRIC_REFUSALS: &str = "refusal-rate";
/// Reward of a met goal.
const REWARD_MET: f64 = 1.0;
/// Reward of a missed goal.
const REWARD_MISSED: f64 = 0.0;

/// The separate verifier of the campaign-manager lane.
pub struct CampaignVerifier<'job> {
    set: &'job crate::TaskSet,
    store: &'job crate::ArtifactStore,
}

impl<'job> CampaignVerifier<'job> {
    /// A verifier of `set`'s tasks reading artifacts from `store`.
    #[must_use]
    pub const fn new(set: &'job crate::TaskSet, store: &'job crate::ArtifactStore) -> Self {
        Self { set, store }
    }
}

impl animus_adapters::harbor::VerifierService for CampaignVerifier<'_> {
    // r[impl agent_harness_runtime.campaign_manager_eval.verifier]
    fn verify(
        &mut self,
        request: &animus_adapters::harbor::TrialRequest,
    ) -> Result<animus_adapters::harbor::TrialObservation, animus_adapters::harbor::OutcomeClass>
    {
        let task = self
            .set
            .task(&request.task_identity)
            .ok_or(animus_adapters::harbor::OutcomeClass::Denied)?;
        let artifact = self
            .store
            .get(&request.trial_identity)
            .ok_or(animus_adapters::harbor::OutcomeClass::Stale)?;
        let session = campaign_choregraph::ControlSession::load(
            artifact.graph,
            artifact.branch,
            artifact.payloads,
            artifact.envelope,
        )
        .map_err(|_| animus_adapters::harbor::OutcomeClass::Malformed)?;
        if session.state().state_id() != artifact.claimed_state_id {
            return Err(animus_adapters::harbor::OutcomeClass::Stale);
        }
        Ok(animus_adapters::harbor::TrialObservation {
            trial_identity: request.trial_identity.clone(),
            outcome: animus_adapters::harbor::OutcomeClass::Completed,
            trajectory_identity: Some(artifact.trajectory.identity),
            rewards: grade(
                task,
                &request.trial_identity,
                session.state(),
                &artifact.turns,
            ),
            artifacts: request.artifacts.clone(),
        })
    }
}

/// The rewards of trial `trial` of `task` from its replayed `state` and turn ledger.
#[must_use]
pub fn grade(
    task: &crate::EvalTask,
    trial: &str,
    state: &campaign_core::FrontierState,
    turns: &[crate::TurnRecord],
) -> Vec<(String, f64)> {
    let is_found = target_found(task, trial, state);
    let efficiency = if is_found {
        efficiency(task.selection_budget, state.selection_count())
    } else {
        REWARD_MISSED
    };
    let found = if is_found { REWARD_MET } else { REWARD_MISSED };
    vec![
        (String::from(METRIC_FOUND), found),
        (String::from(METRIC_EFFICIENCY), efficiency),
        (String::from(METRIC_REFUSALS), refusal_rate(turns)),
    ]
}

/// Whether `state` recorded `task`'s finding and ran the target's candidate.
fn target_found(task: &crate::EvalTask, trial: &str, state: &campaign_core::FrontierState) -> bool {
    let Some(target) = task
        .candidates
        .iter()
        .chain(&task.offers)
        .find(|candidate| candidate.is_target)
    else {
        return false;
    };
    let finding_id = crate::history::bounds::finding_id(&task.identity, target);
    let operation = crate::history::bounds::id(
        campaign_core::IdKind::OperationProfile,
        &format!("{trial}/{}", target.key),
    );
    debug_assert_eq!(finding_id.kind, campaign_core::IdKind::Finding);
    debug_assert_eq!(operation.kind, campaign_core::IdKind::OperationProfile);
    let is_recorded = state
        .findings()
        .iter()
        .any(|finding| finding.finding_id == finding_id);
    let has_run = state.candidates().iter().any(|entry| {
        entry.candidate.operation_profile_id == operation && entry.selection_count > 0
    });
    is_recorded && has_run
}

/// `(budget + 1 - selections) / budget`, clamped to `[0, 1]`.
fn efficiency(budget: u32, selections: u64) -> f64 {
    if budget == 0 {
        return REWARD_MISSED;
    }
    let spared = u64::from(budget)
        .saturating_add(1)
        .saturating_sub(selections)
        .min(u64::from(budget));
    let Ok(spared) = u32::try_from(spared) else {
        return REWARD_MISSED;
    };
    f64::from(spared) / f64::from(budget)
}

/// Share of `turns` whose reply was refused or whose model failed.
fn refusal_rate(turns: &[crate::TurnRecord]) -> f64 {
    let failed = turns
        .iter()
        .filter(|record| {
            matches!(
                record.outcome,
                crate::TurnOutcome::Refused | crate::TurnOutcome::ModelFailed
            )
        })
        .count();
    let (Ok(failed), Ok(total)) = (u32::try_from(failed), u32::try_from(turns.len())) else {
        return REWARD_MET;
    };
    if total == 0 {
        return REWARD_MISSED;
    }
    f64::from(failed) / f64::from(total)
}
