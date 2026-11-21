//! Qualification attachments produced from the scripted job: byte-exact checked fixtures, honest
//! denominators and costs, and the equal-budget comparison.

mod support;

const JOB: &str = "job:campaign-manager-qualification-fixture";
const PRODUCER_REVISION: &str = "animus-4071c65e-qualification";

fn scripted_model(route: &str) -> animus_runtime::qualification::ModelFacts {
    animus_runtime::qualification::ModelFacts {
        deployment: animus_runtime::qualification::ModelDeployment::Scripted,
        version_basis: animus_runtime::qualification::VersionBasis::Declared,
        version: Some(String::from(route)),
        weights_identity: None,
    }
}

/// Runs the scripted job over `set` once.
fn scripted_job(
    set: &animus_campaign_manager_eval::TaskSet,
) -> Result<animus_campaign_manager_eval::JobReport, support::Failure> {
    let mut policies = animus_campaign_manager_eval::ScriptedPolicy::ALL;
    let mut routes: Vec<animus_campaign_manager_eval::Route<'_>> = policies
        .iter_mut()
        .map(|policy| animus_campaign_manager_eval::Route {
            identity: String::from(policy.identity()),
            model: policy,
        })
        .collect();
    Ok(animus_campaign_manager_eval::run_job(
        set,
        &mut routes,
        JOB,
    )?)
}

/// The fixture binding of `route` under `keys`.
fn binding<'a>(
    keys: &'a support::SubjectKeys,
    route: &'a str,
) -> animus_campaign_manager_eval::QualificationBinding<'a> {
    animus_campaign_manager_eval::QualificationBinding {
        route,
        lattice_definition_identity: &keys.definition,
        lattice_profile_revision: &keys.revision,
        task_family: support::TASK_FAMILY,
        model: scripted_model(route),
        label: animus_runtime::qualification::EvidenceLabel::Fixture,
        producer_revision: PRODUCER_REVISION,
        observed_at_ms: support::OBSERVED_AT_MS,
        valid_until_ms: support::VALID_UNTIL_MS,
    }
}

/// The signal-greedy attachment projected from `report` over `set`.
fn greedy_attachment(
    set: &animus_campaign_manager_eval::TaskSet,
    report: &animus_campaign_manager_eval::JobReport,
) -> Result<
    animus_runtime::qualification::Attachment,
    animus_campaign_manager_eval::QualificationError,
> {
    let route = animus_campaign_manager_eval::ScriptedPolicy::SignalGreedy.identity();
    let keys = support::qualification_keys(set, route);
    animus_campaign_manager_eval::task_family_attachment(set, report, &binding(&keys, route))
}

/// Runs the scripted job once and produces each scripted route's attachment.
fn produce() -> Result<Vec<animus_runtime::qualification::Attachment>, support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let job = scripted_job(&set)?;
    assert!(job.failures.is_empty(), "the scripted job completes");
    assert_eq!(
        job.routes.len(),
        animus_campaign_manager_eval::ScriptedPolicy::ALL.len()
    );
    animus_campaign_manager_eval::ScriptedPolicy::ALL
        .iter()
        .map(|policy| {
            let keys = support::qualification_keys(&set, policy.identity());
            animus_campaign_manager_eval::task_family_attachment(
                &set,
                &job,
                &binding(&keys, policy.identity()),
            )
            .map_err(|_| support::Failure::Unexpected("the attachment was refused"))
        })
        .collect()
}

#[test]
// r[verify agent_harness_runtime.qualification_attachment.producer]
fn the_scripted_job_emits_the_checked_attachments() -> Result<(), support::Failure> {
    let produced = produce()?;
    let [greedy, explore, malformed] = produced.as_slice() else {
        return Err(support::Failure::Unexpected("three scripted routes"));
    };
    assert_eq!(
        std::str::from_utf8(&greedy.canonical).ok(),
        Some(support::GREEDY_FIXTURE.trim_end())
    );
    assert_eq!(
        std::str::from_utf8(&explore.canonical).ok(),
        Some(support::EXPLORE_FIXTURE.trim_end())
    );
    let greedy_outcomes = greedy
        .document
        .outcomes
        .ok_or(support::Failure::Unexpected("outcomes"))?;
    assert_eq!((greedy_outcomes.planned, greedy_outcomes.accepted), (3, 3));
    let explore_outcomes = explore
        .document
        .outcomes
        .ok_or(support::Failure::Unexpected("outcomes"))?;
    assert_eq!(
        (explore_outcomes.planned, explore_outcomes.accepted),
        (3, 1)
    );
    assert_eq!(explore_outcomes.rejected, 2);
    assert_eq!(
        greedy.document.evidence.source_identity,
        explore.document.evidence.source_identity
    );
    assert_eq!(
        greedy.document.subject.lattice_definition_identity,
        explore.document.subject.lattice_definition_identity
    );
    assert_ne!(
        greedy.document.subject.lattice_profile_revision,
        explore.document.subject.lattice_profile_revision
    );
    assert!(malformed.document.outcomes.is_some());
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.qualification_attachment.measures]
fn produced_measures_keep_unknown_costs_and_compare_accepted_completions()
-> Result<(), support::Failure> {
    let produced = produce()?;
    let [greedy, explore, malformed] = produced.as_slice() else {
        return Err(support::Failure::Unexpected("three scripted routes"));
    };
    assert_eq!(
        animus_runtime::qualification::cost_per_accepted(greedy),
        animus_runtime::qualification::CostPerAccepted::UnknownCost
    );
    assert_eq!(
        animus_runtime::qualification::cost_per_accepted(malformed),
        animus_runtime::qualification::CostPerAccepted::NoAcceptedCompletion
    );
    assert_eq!(
        animus_runtime::qualification::compare_equal_budget(explore, greedy),
        Ok(
            animus_runtime::qualification::EqualBudgetComparison::CandidateMoreAccepted {
                baseline: 1,
                candidate: 3
            }
        )
    );
    assert_eq!(
        animus_runtime::qualification::compare_equal_budget(greedy, malformed),
        Ok(
            animus_runtime::qualification::EqualBudgetComparison::NoAcceptedGain {
                baseline: 3,
                candidate: 0
            }
        )
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.qualification_attachment.producer]
fn the_producer_refuses_task_sets_and_trials_the_job_did_not_run() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let job = scripted_job(&set)?;
    assert!(greedy_attachment(&set, &job).is_ok());

    let mut edited = set.clone();
    let first = edited
        .tasks
        .first_mut()
        .ok_or(support::Failure::Unexpected("a task"))?;
    first.objective.push_str(" Edited.");
    let mut renamed = set.clone();
    for task in &mut renamed.tasks {
        task.identity.push_str("-renamed");
    }
    let mut fewer = set.clone();
    fewer.tasks.pop();
    for other in [&edited, &renamed, &fewer] {
        assert_eq!(
            greedy_attachment(other, &job).err(),
            Some(animus_campaign_manager_eval::QualificationError::TaskSetMismatch)
        );
    }

    // A report relabelled with the other set's identity still lists the trials that ran.
    for other in [&renamed, &fewer] {
        let mut relabelled = job.clone();
        relabelled.task_set_identity = scripted_job(other)?.task_set_identity;
        assert_eq!(
            greedy_attachment(other, &relabelled).err(),
            Some(animus_campaign_manager_eval::QualificationError::TrialMismatch)
        );
    }

    let mut overplanned = job.clone();
    overplanned.receipt.planned = overplanned.receipt.planned.saturating_add(1);
    let mut dropped = job.clone();
    dropped.receipt.trajectories.clear();
    let trial = job
        .receipt
        .trajectories
        .first()
        .and_then(|trajectory| trajectory.strip_prefix("atif:"))
        .ok_or(support::Failure::Unexpected("a trajectory"))?;
    let mut doubled = job.clone();
    doubled
        .failures
        .push(animus_campaign_manager_eval::TrialFailure {
            trial: String::from(trial),
            task: String::from("task:unplanned"),
            route: String::from(
                animus_campaign_manager_eval::ScriptedPolicy::SignalGreedy.identity(),
            ),
            error: animus_std::harbor::HostError::EvidenceMismatch,
        });
    for tampered in [&overplanned, &dropped, &doubled] {
        assert_eq!(
            greedy_attachment(&set, tampered).err(),
            Some(animus_campaign_manager_eval::QualificationError::TrialMismatch)
        );
    }
    Ok(())
}
