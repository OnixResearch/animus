//! The local-scorer comparison lane: seeded random ranking, the predeclared protocol's drift
//! refusals, and the adoption rule.

mod support;

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.baseline]
fn random_ranking_repeats_per_seed_within_bounds_and_adds_nothing() -> Result<(), support::Failure>
{
    let bounds = animus_campaign_manager_eval::RankBounds::new(-100, 100)
        .ok_or(support::Failure::Unexpected("bounds refused"))?;
    let mut first = animus_campaign_manager_eval::RandomRankRoute::new(7, bounds);
    let mut repeat = animus_campaign_manager_eval::RandomRankRoute::new(7, bounds);
    let mut other = animus_campaign_manager_eval::RandomRankRoute::new(8, bounds);
    let replies: Vec<String> = (0..16)
        .map(|_| first.reply(support::frozen::BRIEF))
        .collect();
    let repeated: Vec<String> = (0..16)
        .map(|_| repeat.reply(support::frozen::BRIEF))
        .collect();
    let others: Vec<String> = (0..16)
        .map(|_| other.reply(support::frozen::BRIEF))
        .collect();
    assert_eq!(replies, repeated);
    assert_ne!(replies, others);
    for reply in &replies {
        let value: serde_json::Value = serde_json::from_str(reply)?;
        assert_eq!(value["add"], serde_json::json!([]));
        let ranks: Vec<i64> = value["scores"]
            .as_array()
            .ok_or(support::Failure::Unexpected("no scores"))?
            .iter()
            .filter_map(|score| score["ranks"][0].as_i64())
            .collect();
        assert_eq!(ranks.len(), 3);
        assert!(ranks.iter().all(|rank| bounds.contains(*rank)));
    }
    assert!(animus_campaign_manager_eval::RankBounds::new(5, 5).is_none());
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.evaluation]
fn the_comparison_protocol_refuses_drift() -> Result<(), support::Failure> {
    let text = include_str!("../../../config/generated/campaign-manager-local-scorer-eval.json");
    let base: serde_json::Value = serde_json::from_str(text)?;
    assert!(animus_campaign_manager_eval::parse_comparison_protocol(text).is_ok());
    let cases: [(
        support::frozen::JsonEdit,
        animus_campaign_manager_eval::ProtocolError,
    ); 9] = [
        (
            |value| value["schema"] = serde_json::json!("animus.other.v1"),
            animus_campaign_manager_eval::ProtocolError::Schema,
        ),
        (
            |value| value["artifact_identity"] = serde_json::json!("blake3:00"),
            animus_campaign_manager_eval::ProtocolError::ArtifactIdentity,
        ),
        (
            |value| value["baseline"] = serde_json::json!("model:scripted:signal-greedy"),
            animus_campaign_manager_eval::ProtocolError::Baseline,
        ),
        (
            |value| value["references"] = serde_json::json!(["model:scripted:explore-only"]),
            animus_campaign_manager_eval::ProtocolError::References,
        ),
        (
            |value| value["random_seeds"] = serde_json::json!([1, 1]),
            animus_campaign_manager_eval::ProtocolError::Seeds,
        ),
        (
            |value| value["random_seeds"] = serde_json::json!([]),
            animus_campaign_manager_eval::ProtocolError::Seeds,
        ),
        (
            |value| value["primary_outcome"] = serde_json::json!("efficiency"),
            animus_campaign_manager_eval::ProtocolError::Outcome,
        ),
        (
            |value| value["adoption"]["max_fallback_per_mille"] = serde_json::json!(1001),
            animus_campaign_manager_eval::ProtocolError::Adoption,
        ),
        (
            |value| value["non_claims"] = serde_json::json!([]),
            animus_campaign_manager_eval::ProtocolError::NonClaims,
        ),
    ];
    for (edit, expected) in cases {
        let mut value = base.clone();
        edit(&mut value);
        let changed = serde_json::to_string(&value)?;
        assert_eq!(
            animus_campaign_manager_eval::parse_comparison_protocol(&changed),
            Err(expected)
        );
    }
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager_eval.local_scorer.evaluation]
fn the_adoption_rule_reports_go_and_no_go() {
    let rule = animus_campaign_manager_eval::AdoptionRule {
        min_gain_over_baseline_per_mille: 100,
        requires_random_parity: true,
        max_fallback_per_mille: 0,
    };
    let winning = animus_campaign_manager_eval::AdoptionInput {
        scorer: 0.9,
        baseline: 0.6,
        random: 0.5,
        scorer_fallback_per_mille: 0,
    };
    assert_eq!(
        animus_campaign_manager_eval::decide_adoption(&rule, &winning),
        animus_campaign_manager_eval::AdoptionDecision::Go
    );
    let small_gain = animus_campaign_manager_eval::AdoptionInput {
        scorer: 0.65,
        ..winning
    };
    let below_random = animus_campaign_manager_eval::AdoptionInput {
        random: 0.95,
        ..winning
    };
    let falling_back = animus_campaign_manager_eval::AdoptionInput {
        scorer_fallback_per_mille: 1,
        ..winning
    };
    for input in [small_gain, below_random, falling_back] {
        assert_eq!(
            animus_campaign_manager_eval::decide_adoption(&rule, &input),
            animus_campaign_manager_eval::AdoptionDecision::NoGo
        );
    }
}
