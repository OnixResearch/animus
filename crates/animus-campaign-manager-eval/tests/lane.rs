//! The evaluation lane end to end: the job over scripted routes, single trials, and the
//! separate verifier's refusals.

mod support;

const PARSER_TASK: &str = "task:config-parser-crash";

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.job]
// r[verify agent_harness_runtime.campaign_manager_eval.verifier]
fn the_job_separates_good_partial_and_broken_managers() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let mut policies = animus_campaign_manager_eval::ScriptedPolicy::ALL;
    let mut routes: Vec<animus_campaign_manager_eval::Route<'_>> = policies
        .iter_mut()
        .map(|policy| animus_campaign_manager_eval::Route {
            identity: String::from(policy.identity()),
            model: policy,
        })
        .collect();
    let summary = animus_campaign_manager_eval::run_job(&set, &mut routes, "job:test")?;
    let planned = u32::try_from(set.tasks.len().saturating_mul(routes.len()))
        .map_err(|_| support::Failure::Unexpected("the matrix is too large"))?;
    assert_eq!(summary.receipt.planned, planned);
    assert_eq!(summary.receipt.completed, planned);
    assert!(summary.failures.is_empty());
    assert_eq!(
        summary.receipt.trajectories.len(),
        set.tasks.len().saturating_mul(routes.len())
    );
    let greedy = animus_campaign_manager_eval::ScriptedPolicy::SignalGreedy.identity();
    let partial = animus_campaign_manager_eval::ScriptedPolicy::ExploreOnly.identity();
    let broken = animus_campaign_manager_eval::ScriptedPolicy::Malformed.identity();
    let found = animus_campaign_manager_eval::METRIC_FOUND;
    let refusals = animus_campaign_manager_eval::METRIC_REFUSALS;
    let greedy = support::route(&summary, greedy)?;
    let partial = support::route(&summary, partial)?;
    let broken = support::route(&summary, broken)?;
    assert!(support::is_one(support::mean(greedy, found)?));
    assert!(support::mean(partial, found)? < support::mean(greedy, found)?);
    assert!(support::is_zero(support::mean(broken, found)?));
    assert!(support::is_one(support::mean(broken, refusals)?));
    assert!(support::is_zero(support::mean(greedy, refusals)?));
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.trial]
// r[verify agent_harness_runtime.campaign_manager_eval.trajectory]
fn a_trial_reaches_an_offered_target_and_projects_its_trajectory() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let task = support::task(&set, PARSER_TASK)?;
    let mut policy = animus_campaign_manager_eval::ScriptedPolicy::SignalGreedy;
    let run = animus_campaign_manager_eval::run_turns(&set, task, "trial:parser", &mut policy)?;
    let state = run.campaign.session.state();
    let stop = state
        .stopped()
        .ok_or(support::Failure::Unexpected("the campaign did not stop"))?;
    assert!(matches!(
        stop.reason,
        campaign_core::StopReason::PolicyRequested { .. }
    ));
    assert_eq!(state.findings().len(), 1);
    assert!(
        state.candidates().len() > task.candidates.len(),
        "the manager added offers"
    );
    let rewards = animus_campaign_manager_eval::grade(task, "trial:parser", state, &run.turns);
    assert!(rewards.contains(&(
        String::from(animus_campaign_manager_eval::METRIC_FOUND),
        1.0
    )));
    let projection = animus_campaign_manager_eval::project_turns("trial:parser", &run.turns)?;
    let host_steps = run
        .turns
        .iter()
        .filter(|record| record.action.is_some())
        .count();
    assert_eq!(projection.steps, run.turns.len().saturating_add(host_steps));
    assert_eq!(projection.calls, projection.steps);
    assert_eq!(
        animus_campaign_manager_eval::project_turns("trial:empty", &[]),
        Err(animus_runtime::harbor_eval::AtifError::Bound)
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.trial]
fn a_partial_manager_misses_a_target_it_never_adds() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let task = support::task(&set, PARSER_TASK)?;
    let mut policy = animus_campaign_manager_eval::ScriptedPolicy::ExploreOnly;
    let run = animus_campaign_manager_eval::run_turns(&set, task, "trial:partial", &mut policy)?;
    let state = run.campaign.session.state();
    assert!(state.findings().is_empty());
    assert_eq!(
        state.candidates().len(),
        task.candidates.len(),
        "no offer was added"
    );
    let stop = state
        .stopped()
        .ok_or(support::Failure::Unexpected("the campaign did not stop"))?;
    assert!(matches!(
        stop.reason,
        campaign_core::StopReason::BudgetExhausted { .. }
    ));
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.verifier]
fn the_verifier_refuses_missing_stale_and_unknown_trials() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let store = animus_campaign_manager_eval::ArtifactStore::new();
    let mut policy = animus_campaign_manager_eval::ScriptedPolicy::SignalGreedy;
    let mut routes = [animus_campaign_manager_eval::Route {
        identity: String::from(policy.identity()),
        model: &mut policy,
    }];
    let route = routes[0].identity.clone();
    let request = support::request(
        &set,
        &support::TrialNames {
            trial: "trial:verified",
            task: PARSER_TASK,
            route: &route,
        },
    );
    let mut verifier = animus_campaign_manager_eval::CampaignVerifier::new(&set, &store);
    let missing = animus_adapters::harbor::VerifierService::verify(&mut verifier, &request);
    assert_eq!(
        missing.err(),
        Some(animus_adapters::harbor::OutcomeClass::Stale)
    );
    let mut environment =
        animus_campaign_manager_eval::CampaignEnvironment::new(&set, &mut routes, &store);
    let subject =
        animus_adapters::harbor::EvaluationEnvironmentService::execute(&mut environment, &request);
    assert_eq!(
        subject.map(|observation| observation.outcome),
        Ok(animus_adapters::harbor::OutcomeClass::Completed)
    );
    let verdict = animus_adapters::harbor::VerifierService::verify(&mut verifier, &request);
    assert_eq!(
        verdict.map(|observation| observation.outcome),
        Ok(animus_adapters::harbor::OutcomeClass::Completed)
    );
    let mut artifact = store
        .get("trial:verified")
        .ok_or(support::Failure::Unexpected(
            "the environment left no artifact",
        ))?;
    artifact.claimed_state_id = campaign_core::Id::absent();
    store.insert("trial:verified", artifact);
    let stale = animus_adapters::harbor::VerifierService::verify(&mut verifier, &request);
    assert_eq!(
        stale.err(),
        Some(animus_adapters::harbor::OutcomeClass::Stale)
    );
    let unknown = support::request(
        &set,
        &support::TrialNames {
            trial: "trial:verified",
            task: "task:unknown",
            route: &route,
        },
    );
    let denied = animus_adapters::harbor::VerifierService::verify(&mut verifier, &unknown);
    assert_eq!(
        denied.err(),
        Some(animus_adapters::harbor::OutcomeClass::Denied)
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.verifier]
fn a_history_from_another_trial_does_not_verify() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let task = support::task(&set, PARSER_TASK)?;
    let mut policy = animus_campaign_manager_eval::ScriptedPolicy::SignalGreedy;
    let first = animus_campaign_manager_eval::run_turns(&set, task, "trial:first", &mut policy)?;
    let second = animus_campaign_manager_eval::run_turns(&set, task, "trial:second", &mut policy)?;
    let projection = animus_campaign_manager_eval::project_turns("trial:first", &first.turns)?;
    let mut artifact = animus_campaign_manager_eval::TrialArtifact::capture(&first, projection);
    artifact.branch = second.campaign.session.branch().clone();
    let store = animus_campaign_manager_eval::ArtifactStore::new();
    store.insert("trial:first", artifact);
    let request = support::request(
        &set,
        &support::TrialNames {
            trial: "trial:first",
            task: PARSER_TASK,
            route: "model:any",
        },
    );
    let mut verifier = animus_campaign_manager_eval::CampaignVerifier::new(&set, &store);
    let verdict = animus_adapters::harbor::VerifierService::verify(&mut verifier, &request);
    assert_eq!(
        verdict.err(),
        Some(animus_adapters::harbor::OutcomeClass::Malformed),
        "a foreign branch head cannot load against this graph"
    );
    assert_ne!(
        first.campaign.session.branch().head,
        second.campaign.session.branch().head
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.trial]
fn unreachable_routes_and_unknown_tasks_fail_closed() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let store = animus_campaign_manager_eval::ArtifactStore::new();
    let mut offline = support::Offline;
    let mut routes = [animus_campaign_manager_eval::Route {
        identity: String::from("model:offline"),
        model: &mut offline,
    }];
    let mut environment =
        animus_campaign_manager_eval::CampaignEnvironment::new(&set, &mut routes, &store);
    let unknown_route = support::request(
        &set,
        &support::TrialNames {
            trial: "trial:a",
            task: PARSER_TASK,
            route: "model:missing",
        },
    );
    let unknown_task = support::request(
        &set,
        &support::TrialNames {
            trial: "trial:b",
            task: "task:missing",
            route: "model:offline",
        },
    );
    let offline_trial = support::request(
        &set,
        &support::TrialNames {
            trial: "trial:c",
            task: PARSER_TASK,
            route: "model:offline",
        },
    );
    let mut run = |request| {
        animus_adapters::harbor::EvaluationEnvironmentService::execute(&mut environment, request)
            .map(|observation| observation.outcome)
    };
    assert_eq!(
        run(&unknown_route),
        Err(animus_adapters::harbor::OutcomeClass::Unavailable)
    );
    assert_eq!(
        run(&unknown_task),
        Err(animus_adapters::harbor::OutcomeClass::Denied)
    );
    assert_eq!(
        run(&offline_trial),
        Ok(animus_adapters::harbor::OutcomeClass::Completed)
    );
    let artifact = store.get("trial:c").ok_or(support::Failure::Unexpected(
        "the offline trial left no artifact",
    ))?;
    assert!(
        artifact
            .turns
            .iter()
            .all(|record| record.outcome == animus_campaign_manager_eval::TurnOutcome::ModelFailed)
    );
    Ok(())
}
