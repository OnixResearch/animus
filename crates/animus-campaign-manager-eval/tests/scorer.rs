//! The frozen local scorer route: guidance-only replies in brief order, visible fallbacks with
//! their dispositions, bounded decision records, and real manager trials.

mod support;

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.route]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.schema]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.record]
fn the_scorer_answers_with_guidance_only_in_brief_order() -> Result<(), support::Failure> {
    let mut route = support::frozen::fixture_route()?;
    let reply = route
        .answer(support::frozen::BRIEF)
        .map_err(|_| support::Failure::Unexpected("the scorer refused"))?;
    assert_eq!(
        reply,
        r#"{"scores":[{"candidate":"c1","ranks":[13]},{"candidate":"c2","ranks":[16]},{"candidate":"c3","ranks":[4]}],"add":[]}"#
    );
    let record = support::frozen::last(&route)?;
    assert_eq!(
        record.disposition,
        animus_campaign_manager_eval::Disposition::Scored
    );
    assert_eq!(record.fallback, None);
    assert_eq!(
        record.features,
        vec![
            [1000, 777, 777, 500, 333, 1000],
            [0, 0, 777, 500, 333, 1000],
            [1000, 222, 222, 0, 333, 0],
        ]
    );
    assert_eq!(record.scores, vec![13, 16, 4]);
    assert_eq!(record.cost.inference_operations, 18);
    assert_eq!(record.cost.fallback_candidates, 0);
    assert!(
        record
            .artifact
            .as_deref()
            .is_some_and(|identity| identity.starts_with("b3:"))
    );
    let again = route
        .answer(support::frozen::BRIEF)
        .map_err(|_| support::Failure::Unexpected("the scorer refused"))?;
    assert_eq!(
        again, reply,
        "a frozen policy answers one brief identically"
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.fallback]
fn malformed_briefs_fall_back_to_explore_only_visibly() -> Result<(), support::Failure> {
    let briefs = [
        support::frozen::BRIEF.replace("[signal 2]", "[signal 10]"),
        support::frozen::BRIEF.replace("each from -1000000 to 1000000", "from small to large"),
        support::frozen::BRIEF
            .lines()
            .filter(|line| !line.starts_with('c'))
            .fold(String::new(), |mut text, line| {
                text.push_str(line);
                text.push('\n');
                text
            }),
    ];
    for brief in briefs {
        let mut route = support::frozen::fixture_route()?;
        let reply = route
            .answer(&brief)
            .map_err(|_| support::Failure::Unexpected("the scorer refused"))?;
        assert_eq!(
            reply,
            animus_campaign_manager_eval::FALLBACK_POLICY.reply(&brief)
        );
        let record = support::frozen::last(&route)?;
        assert_eq!(
            record.disposition,
            animus_campaign_manager_eval::Disposition::MalformedBrief
        );
        assert_eq!(
            record.fallback,
            Some(animus_campaign_manager_eval::FALLBACK_POLICY)
        );
        assert!(record.scores.is_empty());
    }
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.fallback]
fn out_of_range_and_budget_decisions_fall_back() -> Result<(), support::Failure> {
    let (loud, loud_identity) =
        support::frozen::edited(|value| value["bias"] = serde_json::json!(1_000_000))?;
    let (tight, tight_identity) = support::frozen::edited(|value| {
        value["runtime"]["operation_budget"] = serde_json::json!(6);
    })?;
    let narrow_brief =
        support::frozen::BRIEF.replace("each from -1000000 to 1000000", "each from -50 to 50");
    let cases = [
        (
            support::frozen::admitted_route(&loud, &loud_identity),
            support::frozen::BRIEF.to_owned(),
            animus_campaign_manager_eval::Disposition::OutOfRange,
        ),
        (
            support::frozen::admitted_route(&tight, &tight_identity),
            support::frozen::BRIEF.to_owned(),
            animus_campaign_manager_eval::Disposition::OperationBudget,
        ),
        (
            support::frozen::fixture_route()?,
            narrow_brief,
            animus_campaign_manager_eval::Disposition::OutOfRange,
        ),
    ];
    for (mut route, brief, expected) in cases {
        let reply = route
            .answer(&brief)
            .map_err(|_| support::Failure::Unexpected("the scorer refused"))?;
        assert_eq!(
            reply,
            animus_campaign_manager_eval::FALLBACK_POLICY.reply(&brief)
        );
        let record = support::frozen::last(&route)?;
        assert_eq!(record.disposition, expected);
        assert_eq!(
            record.cost.fallback_candidates, 3,
            "the fallback's work is charged"
        );
        assert_eq!(record.features.len(), 3, "the projection is retained");
    }
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.fallback]
fn refused_and_missing_artifacts_fall_back_with_their_disposition() -> Result<(), support::Failure>
{
    let cases = [
        (
            animus_campaign_manager_eval::ArtifactError::Identity,
            animus_campaign_manager_eval::Disposition::IncompatibleArtifact,
        ),
        (
            animus_campaign_manager_eval::ArtifactError::Unavailable,
            animus_campaign_manager_eval::Disposition::Unavailable,
        ),
    ];
    for (refusal, expected) in cases {
        let mut route = animus_campaign_manager_eval::LocalScorerRoute::new(Err(refusal));
        assert_eq!(route.identity(), "model:local-scorer:refused");
        let reply = route
            .answer(support::frozen::BRIEF)
            .map_err(|_| support::Failure::Unexpected("the route refused"))?;
        assert_eq!(
            reply,
            animus_campaign_manager_eval::FALLBACK_POLICY.reply(support::frozen::BRIEF)
        );
        let record = support::frozen::last(&route)?;
        assert_eq!(record.disposition, expected);
        assert_eq!(record.artifact, None);
        assert!(record.features.is_empty());
    }
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.record]
fn decision_records_are_bounded() -> Result<(), support::Failure> {
    let mut route = support::frozen::fixture_route()?;
    for _ in 0..animus_campaign_manager_eval::MAX_DECISIONS {
        route
            .answer(support::frozen::BRIEF)
            .map_err(|_| support::Failure::Unexpected("the scorer refused early"))?;
    }
    assert_eq!(
        route.decisions().len(),
        animus_campaign_manager_eval::MAX_DECISIONS
    );
    assert_eq!(
        route.answer(support::frozen::BRIEF),
        Err(animus_campaign_manager_eval::RouteFailure::Rejected)
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.route]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.verification]
fn the_scorer_completes_real_manager_trials_without_fallback() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let mut scorer = support::frozen::fixture_route()?;
    let mut baseline = animus_campaign_manager_eval::FALLBACK_POLICY;
    let identity = String::from(scorer.identity());
    let mut routes = vec![
        animus_campaign_manager_eval::Route {
            identity: String::from(baseline.identity()),
            model: &mut baseline,
        },
        animus_campaign_manager_eval::Route {
            identity: identity.clone(),
            model: &mut scorer,
        },
    ];
    let summary =
        animus_campaign_manager_eval::run_job(&set, &mut routes, "job:test-local-scorer")?;
    drop(routes);
    assert!(summary.failures.is_empty());
    assert_eq!(support::route(&summary, &identity)?.completed, 3);
    let decisions = scorer.decisions();
    assert!(!decisions.is_empty());
    assert!(
        decisions
            .iter()
            .all(|record| record.disposition == animus_campaign_manager_eval::Disposition::Scored),
        "real briefs fit the declared projection"
    );
    let refusals = support::mean(
        support::route(&summary, &identity)?,
        animus_campaign_manager_eval::METRIC_REFUSALS,
    )?;
    assert!(
        support::is_zero(refusals),
        "the manager admits every scored reply"
    );
    Ok(())
}
