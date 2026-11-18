//! The separate verifier's grading rules over a real trial's replayed state.

mod support;

const PARSER_TASK: &str = "task:config-parser-crash";
const TRIAL: &str = "trial:grading";

/// The refusal rate `grade` reports for a ledger of `outcomes` against `run`'s replayed state.
fn refusal_rate(
    task: &animus_campaign_manager_eval::EvalTask,
    run: &animus_campaign_manager_eval::TrialRun,
    outcomes: &[animus_campaign_manager_eval::TurnOutcome],
) -> Result<f64, support::Failure> {
    let turns: Vec<animus_campaign_manager_eval::TurnRecord> = outcomes
        .iter()
        .zip(1..)
        .map(|(outcome, turn)| animus_campaign_manager_eval::TurnRecord {
            turn,
            outcome: *outcome,
            action: None,
        })
        .collect();
    animus_campaign_manager_eval::grade(task, TRIAL, run.campaign.session.state(), &turns)
        .into_iter()
        .find(|(metric, _)| metric == animus_campaign_manager_eval::METRIC_REFUSALS)
        .map(|(_, value)| value)
        .ok_or(support::Failure::Unexpected("no refusal rate was graded"))
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.verifier]
fn unchanged_replies_are_neither_admitted_nor_refused() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let task = support::task(&set, PARSER_TASK)?;
    let mut policy = animus_campaign_manager_eval::ScriptedPolicy::ExploreOnly;
    let run = animus_campaign_manager_eval::run_turns(&set, task, TRIAL, &mut policy)?;
    let unchanged = animus_campaign_manager_eval::TurnOutcome::Unchanged;
    assert!(
        support::is_zero(refusal_rate(task, &run, &[unchanged, unchanged])?),
        "a reply that asks for no change is not a refusal"
    );
    for counted in [
        animus_campaign_manager_eval::TurnOutcome::Refused,
        animus_campaign_manager_eval::TurnOutcome::ModelFailed,
    ] {
        let rate = refusal_rate(task, &run, &[unchanged, counted])?;
        assert!((rate - 0.5).abs() < f64::EPSILON, "{counted:?}: {rate}");
    }
    Ok(())
}
