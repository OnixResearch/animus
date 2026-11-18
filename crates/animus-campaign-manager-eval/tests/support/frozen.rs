//! Shared support of the frozen local scorer tests.

/// A brief in the manager's rendering: one leading parser family, one unexecuted parser member, a
/// weak cache candidate, and one offer.
pub const BRIEF: &str = "Objective: find the crash.\n\
Ranks: 1 integers per score, each from -1000000 to 1000000. Higher ranks run first, compared from the first integer.\n\
Candidates (unscored first, then best first):\n\
c1 unscored, picked 1 times: parser/fuzz [signal 7]\n\
c2 unscored, picked 0 times: parser/grammar\n\
c3 unscored, picked 1 times: cache/keys [signal 2]\n\
Offers you may add:\n\
o1: parser/mutation\n";

/// One edit of a JSON document under test.
pub type JsonEdit = fn(&mut serde_json::Value);

/// The embedded fixture artifact as JSON.
pub fn fixture_json() -> Result<serde_json::Value, super::Failure> {
    Ok(serde_json::from_slice(
        animus_campaign_manager_eval::embedded_local_scorer_fixture(),
    )?)
}

/// The canonical identity of the embedded fixture artifact.
pub fn fixture_identity() -> Result<String, super::Failure> {
    animus_campaign_manager_eval::artifact_identity(
        animus_campaign_manager_eval::embedded_local_scorer_fixture(),
    )
    .map_err(|_| super::Failure::Unexpected("the fixture is not a document"))
}

/// The fixture with `edit` applied, as bytes and their own canonical identity, or the fixture's
/// identity when the edit leaves no document to identify.
pub fn edited(
    edit: impl FnOnce(&mut serde_json::Value),
) -> Result<(Vec<u8>, String), super::Failure> {
    let selected = fixture_identity()?;
    let mut value = fixture_json()?;
    edit(&mut value);
    let bytes = serde_json::to_vec(&value)?;
    let identity = animus_campaign_manager_eval::artifact_identity(&bytes).unwrap_or(selected);
    Ok((bytes, identity))
}

/// A scorer route over `bytes`, admitted under `identity` or refused.
pub fn admitted_route(
    bytes: &[u8],
    identity: &str,
) -> animus_campaign_manager_eval::LocalScorerRoute {
    animus_campaign_manager_eval::LocalScorerRoute::new(
        animus_campaign_manager_eval::admit_artifact(bytes, identity),
    )
}

/// A scorer route over the artifact the comparison protocol selects.
pub fn fixture_route() -> Result<animus_campaign_manager_eval::LocalScorerRoute, super::Failure> {
    let protocol = animus_campaign_manager_eval::embedded_comparison_protocol()
        .map_err(|_| super::Failure::Unexpected("the protocol is refused"))?;
    let fixture = animus_campaign_manager_eval::embedded_local_scorer_fixture();
    Ok(admitted_route(fixture, &protocol.artifact_identity))
}

/// The last decision `route` recorded.
pub fn last(
    route: &animus_campaign_manager_eval::LocalScorerRoute,
) -> Result<&animus_campaign_manager_eval::DecisionRecord, super::Failure> {
    route
        .decisions()
        .last()
        .ok_or(super::Failure::Unexpected("no decision was recorded"))
}
