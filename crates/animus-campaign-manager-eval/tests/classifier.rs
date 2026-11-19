//! The frozen-policy tree route over the shared `animus-frozen-policy` leaf: class-mapped ranks
//! in brief order with no offers, typed fallbacks for every refusal, leaf receipts per candidate,
//! real manager trials, and its predeclared protocol.

mod support;

/// The route's tree fixture admitted under the protocol's selection.
fn fixture_route() -> Result<animus_campaign_manager_eval::ClassifierRoute, support::Failure> {
    let protocol = animus_campaign_manager_eval::embedded_frozen_policy_protocol()
        .map_err(|_| support::Failure::Unexpected("the protocol is refused"))?;
    let admitted = animus_frozen_policy::admit_policy(
        animus_campaign_manager_eval::embedded_frozen_policy_fixture(),
        &protocol.artifact_identity,
    );
    Ok(animus_campaign_manager_eval::ClassifierRoute::new(
        admitted,
        &protocol.artifact_identity,
    ))
}

/// The tree fixture with `edit` applied, admitted under its own identity and selected by it.
fn edited_route(
    edit: impl FnOnce(&mut serde_json::Value),
) -> Result<animus_campaign_manager_eval::ClassifierRoute, support::Failure> {
    let mut value: serde_json::Value =
        serde_json::from_slice(animus_campaign_manager_eval::embedded_frozen_policy_fixture())?;
    edit(&mut value);
    let bytes = serde_json::to_vec(&value)?;
    let identity = animus_frozen_policy::policy_identity(&bytes)
        .map_err(|_| support::Failure::Unexpected("the edit is not a document"))?;
    Ok(animus_campaign_manager_eval::ClassifierRoute::new(
        animus_frozen_policy::admit_policy(&bytes, &identity),
        &identity,
    ))
}

/// The last decision `route` recorded.
fn last(
    route: &animus_campaign_manager_eval::ClassifierRoute,
) -> Result<&animus_campaign_manager_eval::DecisionRecord, support::Failure> {
    route
        .decisions()
        .last()
        .ok_or(support::Failure::Unexpected("no decision was recorded"))
}

/// Answers the shared brief and checks that the route fell back with `expected`.
fn assert_falls_back(
    route: &mut animus_campaign_manager_eval::ClassifierRoute,
    expected: animus_campaign_manager_eval::Disposition,
) -> Result<(), support::Failure> {
    let reply = route
        .answer(support::frozen::BRIEF)
        .map_err(|_| support::Failure::Unexpected("the route refused"))?;
    assert_eq!(
        reply,
        animus_campaign_manager_eval::FALLBACK_POLICY.reply(support::frozen::BRIEF)
    );
    let record = last(route)?;
    assert_eq!(record.disposition, expected);
    assert_eq!(
        record.fallback,
        Some(animus_campaign_manager_eval::FALLBACK_POLICY)
    );
    assert_eq!(
        record.cost.fallback_candidates, 3,
        "the fallback is charged"
    );
    assert!(record.scores.is_empty());
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.frozen_policy.campaign_route]
fn the_tree_route_ranks_classes_in_brief_order_and_adds_nothing() -> Result<(), support::Failure> {
    let mut route = fixture_route()?;
    assert!(route.identity().starts_with("model:frozen-policy:"));
    let reply = route
        .answer(support::frozen::BRIEF)
        .map_err(|_| support::Failure::Unexpected("the route refused"))?;
    assert_eq!(
        reply,
        r#"{"scores":[{"candidate":"c1","ranks":[7]},{"candidate":"c2","ranks":[20]},{"candidate":"c3","ranks":[2]}],"add":[]}"#,
        "executed candidates follow their signal; the leading family's unexecuted member is exploited"
    );
    let record = last(&route)?;
    assert_eq!(
        record.disposition,
        animus_campaign_manager_eval::Disposition::Scored
    );
    assert_eq!(record.fallback, None);
    assert_eq!(record.scores, vec![7, 20, 2]);
    assert_eq!(record.cost.feature_operations, 18);
    assert_eq!(
        record.cost.inference_operations, 5,
        "one split per executed candidate, three for the exploited one"
    );
    assert_eq!(record.receipts.len(), 3);
    for receipt in &record.receipts {
        assert!(animus_frozen_policy::is_b3_identity(receipt));
    }
    let first_receipts = record.receipts.clone();
    route
        .answer(support::frozen::BRIEF)
        .map_err(|_| support::Failure::Unexpected("the route refused"))?;
    assert_eq!(
        last(&route)?.receipts,
        first_receipts,
        "a frozen policy decides one brief identically"
    );
    let weak = support::frozen::BRIEF.replace("[signal 7]", "[signal 5]");
    let reply = route
        .answer(&weak)
        .map_err(|_| support::Failure::Unexpected("the route refused"))?;
    assert!(
        reply.contains(r#"{"candidate":"c2","ranks":[10]}"#),
        "a weak lead is explored, not exploited: {reply}"
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.frozen_policy.campaign_route]
// r[verify agent_harness_runtime.frozen_policy.failure]
fn refused_stale_and_mismatched_artifacts_fall_back() -> Result<(), support::Failure> {
    let mut missing = animus_campaign_manager_eval::ClassifierRoute::new(
        Err(animus_frozen_policy::AdmissionError::Unavailable),
        "b3:unused",
    );
    assert_eq!(missing.identity(), "model:frozen-policy:refused");
    assert_falls_back(
        &mut missing,
        animus_campaign_manager_eval::Disposition::Unavailable,
    )?;
    let mut refused = animus_campaign_manager_eval::ClassifierRoute::new(
        Err(animus_frozen_policy::AdmissionError::Identity),
        "b3:unused",
    );
    assert_falls_back(
        &mut refused,
        animus_campaign_manager_eval::Disposition::IncompatibleArtifact,
    )?;
    let mut relabelled = edited_route(|value| {
        value["action_schema"]["actions"][2] = serde_json::json!("exploit-hard");
    })?;
    assert_falls_back(
        &mut relabelled,
        animus_campaign_manager_eval::Disposition::IncompatibleArtifact,
    )?;
    let protocol = animus_campaign_manager_eval::embedded_frozen_policy_protocol()
        .map_err(|_| support::Failure::Unexpected("the protocol is refused"))?;
    let admitted = animus_frozen_policy::admit_policy(
        animus_campaign_manager_eval::embedded_frozen_policy_fixture(),
        &protocol.artifact_identity,
    );
    let mut stale = animus_campaign_manager_eval::ClassifierRoute::new(
        admitted,
        "b3:0000000000000000000000000000000000000000000000000000000000000000",
    );
    assert_falls_back(&mut stale, animus_campaign_manager_eval::Disposition::Stale)?;
    assert_eq!(
        last(&stale)?.receipts.len(),
        1,
        "the first refused candidate stops the brief"
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.frozen_policy.campaign_route]
// r[verify agent_harness_runtime.frozen_policy.failure]
fn abstention_budget_and_malformed_briefs_fall_back() -> Result<(), support::Failure> {
    let mut abstaining = edited_route(|value| {
        value["bounds"]["abstain_below"] = serde_json::json!(200);
    })?;
    assert_falls_back(
        &mut abstaining,
        animus_campaign_manager_eval::Disposition::Abstained,
    )?;
    let mut over_budget = edited_route(|value| {
        let hidden = vec![vec![1_i64; 6]; 16];
        let output = vec![vec![1_i64; 16]; 3];
        value["model"] = serde_json::json!({
            "kind": "mlp-per-mille-v1",
            "layers": [
                {"activation": "relu", "biases": vec![0; 16], "weights": hidden},
                {"activation": "identity", "biases": vec![0; 3], "weights": output},
            ],
        });
        value["bounds"]["operation_budget"] = serde_json::json!(144);
    })?;
    assert_falls_back(
        &mut over_budget,
        animus_campaign_manager_eval::Disposition::OperationBudget,
    )?;
    assert_eq!(
        last(&over_budget)?.cost.inference_operations,
        0,
        "the deadline refuses before inference"
    );
    let mut route = fixture_route()?;
    let malformed = support::frozen::BRIEF.replace("[signal 2]", "[signal 10]");
    let reply = route
        .answer(&malformed)
        .map_err(|_| support::Failure::Unexpected("the route refused"))?;
    assert_eq!(
        reply,
        animus_campaign_manager_eval::FALLBACK_POLICY.reply(&malformed)
    );
    assert_eq!(
        last(&route)?.disposition,
        animus_campaign_manager_eval::Disposition::MalformedBrief
    );
    assert!(
        last(&route)?.receipts.is_empty(),
        "nothing reached the leaf"
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.frozen_policy.campaign_route]
// r[verify agent_harness_runtime.frozen_policy.verification]
fn the_tree_route_completes_real_manager_trials_without_fallback() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let mut route = fixture_route()?;
    let identity = String::from(route.identity());
    let mut routes = vec![animus_campaign_manager_eval::Route {
        identity: identity.clone(),
        model: &mut route,
    }];
    let summary = animus_campaign_manager_eval::run_job(&set, &mut routes, "job:test-tree")?;
    drop(routes);
    assert!(summary.failures.is_empty());
    assert_eq!(support::route(&summary, &identity)?.completed, 3);
    assert!(!route.decisions().is_empty());
    assert!(
        route
            .decisions()
            .iter()
            .all(|record| record.disposition == animus_campaign_manager_eval::Disposition::Scored),
        "real briefs fit the declared projection and classes"
    );
    let refusals = support::mean(
        support::route(&summary, &identity)?,
        animus_campaign_manager_eval::METRIC_REFUSALS,
    )?;
    assert!(support::is_zero(refusals));
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.frozen_policy.evaluation]
fn the_frozen_policy_protocol_selects_the_tree_fixture() -> Result<(), support::Failure> {
    let protocol = animus_campaign_manager_eval::embedded_frozen_policy_protocol()
        .map_err(|_| support::Failure::Unexpected("the protocol is refused"))?;
    assert_eq!(
        protocol.scorer_arm(),
        Some(animus_campaign_manager_eval::ScorerArm::FrozenPolicy)
    );
    assert_eq!(
        protocol.schema,
        animus_campaign_manager_eval::FROZEN_POLICY_COMPARISON_SCHEMA
    );
    let identity = animus_frozen_policy::policy_identity(
        animus_campaign_manager_eval::embedded_frozen_policy_fixture(),
    )
    .map_err(|_| support::Failure::Unexpected("the fixture is not a document"))?;
    assert_eq!(protocol.artifact_identity, identity);
    let linear = animus_campaign_manager_eval::embedded_comparison_protocol()
        .map_err(|_| support::Failure::Unexpected("the protocol is refused"))?;
    assert_eq!(
        linear.scorer_arm(),
        Some(animus_campaign_manager_eval::ScorerArm::LocalScorer)
    );
    assert_eq!(
        protocol.adoption, linear.adoption,
        "both arms face the same predeclared rule"
    );
    Ok(())
}
