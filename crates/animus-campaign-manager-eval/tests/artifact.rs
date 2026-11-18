//! The frozen local scorer's artifact admission: the selected fixture is admitted under its
//! canonical identity, and tampered, unselected, missing, and oversized artifacts are refused
//! before scoring.

mod support;

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.artifact]
fn the_selected_fixture_is_admitted_under_its_canonical_identity() -> Result<(), support::Failure> {
    let protocol = animus_campaign_manager_eval::embedded_comparison_protocol()
        .map_err(|_| support::Failure::Unexpected("the protocol is refused"))?;
    let fixture = animus_campaign_manager_eval::embedded_local_scorer_fixture();
    let identity = support::frozen::fixture_identity()?;
    assert_eq!(
        protocol.artifact_identity, identity,
        "the protocol selects the fixture"
    );
    let admitted = animus_campaign_manager_eval::admit_artifact(fixture, &identity)
        .map_err(|_| support::Failure::Unexpected("the fixture is refused"))?;
    assert_eq!(admitted.weights, [-10, 9, 5, 0, 0, 3]);
    let reordered = serde_json::to_vec_pretty(&support::frozen::fixture_json()?)?;
    assert_eq!(
        animus_campaign_manager_eval::artifact_identity(&reordered).ok(),
        Some(identity),
        "spacing and key order do not change the identity"
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.artifact]
fn tampered_artifacts_are_refused_before_scoring() -> Result<(), support::Failure> {
    let cases: [(
        support::frozen::JsonEdit,
        animus_campaign_manager_eval::ArtifactError,
    ); 9] = [
        (
            |value| value["unexpected"] = serde_json::json!(1),
            animus_campaign_manager_eval::ArtifactError::Malformed,
        ),
        (
            |value| value["schema"] = serde_json::json!("animus.other.v1"),
            animus_campaign_manager_eval::ArtifactError::Schema,
        ),
        (
            |value| value["policy"] = serde_json::json!("mlp-v1"),
            animus_campaign_manager_eval::ArtifactError::Policy,
        ),
        (
            |value| value["features"][0] = serde_json::json!("signal"),
            animus_campaign_manager_eval::ArtifactError::Features,
        ),
        (
            |value| value["action_schema"] = serde_json::json!("animus.other-action.v1"),
            animus_campaign_manager_eval::ArtifactError::Action,
        ),
        (
            |value| value["weights"][1] = serde_json::json!(2_000_000),
            animus_campaign_manager_eval::ArtifactError::Parameters,
        ),
        (
            |value| value["weights"] = serde_json::json!([1, 2, 3]),
            animus_campaign_manager_eval::ArtifactError::Parameters,
        ),
        (
            |value| value["rank_bounds"] = serde_json::json!({"min": 5, "max": 5}),
            animus_campaign_manager_eval::ArtifactError::RankBounds,
        ),
        (
            |value| value["runtime"]["artifact_api"] = serde_json::json!(2),
            animus_campaign_manager_eval::ArtifactError::Runtime,
        ),
    ];
    for (edit, expected) in cases {
        let (bytes, identity) = support::frozen::edited(edit)?;
        assert_eq!(
            animus_campaign_manager_eval::admit_artifact(&bytes, &identity),
            Err(expected)
        );
    }
    let (budgetless, identity) = support::frozen::edited(|value| {
        value["runtime"]["operation_budget"] = serde_json::json!(0);
    })?;
    assert_eq!(
        animus_campaign_manager_eval::admit_artifact(&budgetless, &identity),
        Err(animus_campaign_manager_eval::ArtifactError::Runtime)
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.artifact]
fn unselected_missing_and_oversized_artifacts_are_refused() -> Result<(), support::Failure> {
    let (reweighted, _) =
        support::frozen::edited(|value| value["weights"][1] = serde_json::json!(10))?;
    let selected = support::frozen::fixture_identity()?;
    assert_eq!(
        animus_campaign_manager_eval::admit_artifact(&reweighted, &selected),
        Err(animus_campaign_manager_eval::ArtifactError::Identity)
    );
    assert_eq!(
        animus_campaign_manager_eval::admit_artifact(&[], &selected),
        Err(animus_campaign_manager_eval::ArtifactError::Unavailable)
    );
    assert_eq!(
        animus_campaign_manager_eval::admit_artifact(&vec![b' '; 20_000], &selected),
        Err(animus_campaign_manager_eval::ArtifactError::TooLarge)
    );
    Ok(())
}
