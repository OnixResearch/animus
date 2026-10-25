//! Admitted campaign-manager turns over real Choregraph history with a scripted model.

mod support;

#[test]
// r[verify agent_harness_runtime.campaign_manager.turn]
// r[verify agent_harness_runtime.campaign_manager.brief]
// r[verify agent_harness_runtime.campaign_manager.publication]
fn scored_advice_is_published_and_steers_the_next_pick() -> Result<(), support::Failure> {
    let (fixture, ids) = support::frontier(&["alpha", "beta", "gamma"])?;
    let setup = support::setup(fixture.session.state(), support::full_terms()?)?;
    let favourite = support::handle(fixture.session.state(), &setup, ids[2])?;
    let other = support::handle(fixture.session.state(), &setup, ids[0])?;
    let reply = format!(
        "Here is my advice:\n```json\n{{\"scores\":[{{\"candidate\":\"{favourite}\",\"ranks\":[9,1]}},{{\"candidate\":\"{other}\",\"ranks\":[2,0]}}]}}\n```"
    );
    let mut model = support::ScriptedModel::replying(&reply);
    let labels = [animus_campaign_manager::CandidateLabel {
        candidate_id: ids[2],
        label: String::from("fuzz the decoder with nested arrays"),
    }];
    let menu = animus_campaign_manager::CandidateMenu {
        labels: &labels,
        offers: &[],
    };
    let turn = animus_campaign_manager::advise(&mut model, fixture.session.state(), &setup, menu)?;
    let prompt = model
        .prompts
        .first()
        .ok_or(support::Failure::Unexpected("the model was not asked"))?;
    assert!(prompt.contains("Objective: Find the parser input that hangs the decoder."));
    assert!(prompt.contains(&format!(
        "{favourite} unscored, picked 0 times: fuzz the decoder"
    )));
    assert!(prompt.contains("Allowed per reply: add offers (at most 16)"));
    assert!(
        !prompt.contains(&ids[2].to_string()),
        "the prompt never shows Campaign identities"
    );
    assert_eq!(turn.admission.events.len(), 2);
    let mut store = support::MemoryStore::durable();
    let session = animus_campaign_manager::publish_events(
        fixture.session,
        &turn.admission.events,
        &mut store,
    )?;
    assert_eq!(store.writes, 2);
    assert_eq!(
        session.state().state_id(),
        turn.admission.receipt.state_after_id
    );
    campaign_core::validate_advisory_receipt(&turn.admission.receipt)?;
    assert_eq!(support::selected(&session)?, ids[2]);
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager.proposal]
// r[verify agent_harness_runtime.campaign_manager.publication]
fn a_full_manager_reply_adds_retires_and_stops() -> Result<(), support::Failure> {
    let (fixture, ids) = support::frontier(&["alpha", "beta"])?;
    let setup = support::setup(fixture.session.state(), support::full_terms()?)?;
    let retired = support::handle(fixture.session.state(), &setup, ids[1])?;
    let offers = [animus_campaign_manager::CandidateOffer {
        record: fixture.record("delta"),
        label: String::from("replay the corpus with a smaller stack"),
    }];
    let reply = format!(
        r#"{{"add":[{{"offer":"o1","ranks":[5,5]}}],"retire":[{{"candidate":"{retired}","reason":"dead end"}}],"stop":{{"reason":"objective met"}}}}"#
    );
    let mut model = support::ScriptedModel::replying(&reply);
    let menu = animus_campaign_manager::CandidateMenu {
        labels: &[],
        offers: &offers,
    };
    let turn = animus_campaign_manager::advise(&mut model, fixture.session.state(), &setup, menu)?;
    assert!(
        model
            .prompts
            .iter()
            .any(|prompt| prompt.contains("o1: replay the corpus with a smaller stack"))
    );
    let kinds: Vec<&str> = turn.admission.events.iter().map(support::kind).collect();
    assert_eq!(kinds, ["batch", "guidance", "retirement", "stop"]);
    let session = support::publish(fixture.session, &turn)?;
    let stop = session
        .state()
        .stopped()
        .ok_or(support::Failure::Unexpected("the campaign did not stop"))?;
    assert!(matches!(
        stop.reason,
        campaign_core::StopReason::ConsumerRequested { .. }
    ));
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager.proposal]
fn a_prune_reply_takes_candidates_out_of_selection() -> Result<(), support::Failure> {
    let (fixture, ids) = support::frontier(&["alpha", "beta", "gamma"])?;
    let setup = support::setup(fixture.session.state(), support::full_terms()?)?;
    let first = support::handle(fixture.session.state(), &setup, ids[0])?;
    let second = support::handle(fixture.session.state(), &setup, ids[1])?;
    let survivor = support::handle(fixture.session.state(), &setup, ids[2])?;
    let reply = format!(
        r#"{{"scores":[{{"candidate":"{survivor}","ranks":[1,1]}}],"prune":{{"candidates":["{first}","{second}"],"reason":"unpromising"}}}}"#
    );
    let turn = support::advise_once(&fixture, &setup, &reply)?;
    let kinds: Vec<&str> = turn.admission.events.iter().map(support::kind).collect();
    assert_eq!(kinds, ["guidance", "prune"]);
    let session = support::publish(fixture.session, &turn)?;
    let brief = campaign_core::brief_advisor(session.state(), &setup.mandate)?;
    let listed: Vec<campaign_core::Id> = brief
        .candidates
        .iter()
        .map(|entry| entry.candidate.candidate_id)
        .collect();
    assert_eq!(listed, [ids[2]], "pruned candidates leave the brief");
    assert_eq!(brief.pruned_candidates, 2);
    assert_eq!(support::selected(&session)?, ids[2]);
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager.proposal]
fn rescores_raise_as_guidance_lower_as_revisions_and_repeats_drop() -> Result<(), support::Failure>
{
    let (fixture, ids) = support::frontier(&["alpha", "beta", "gamma"])?;
    let setup = support::setup(fixture.session.state(), support::full_terms()?)?;
    let first = scores(&fixture.session, &setup, &ids, [[4, 4], [4, 4], [4, 4]])?;
    let turn = support::advise_once(&fixture, &setup, &first)?;
    assert_eq!(turn.proposal.guidance.len(), 3);
    let rescored = support::Fixture {
        session: support::publish(fixture.session, &turn)?,
    };
    let second = scores(&rescored.session, &setup, &ids, [[6, 0], [1, 0], [4, 4]])?;
    let turn = support::advise_once(&rescored, &setup, &second)?;
    let raised: Vec<campaign_core::Id> = turn
        .proposal
        .guidance
        .iter()
        .map(|record| record.candidate_id)
        .collect();
    let lowered: Vec<campaign_core::Id> = turn
        .proposal
        .revisions
        .iter()
        .map(|revision| revision.candidate_id)
        .collect();
    assert_eq!(raised, [ids[0]], "a raised score is new guidance");
    assert_eq!(
        lowered,
        [ids[1]],
        "a lowered score is a revision; a repeat is dropped"
    );
    let session = support::publish(rescored.session, &turn)?;
    assert_eq!(support::selected(&session)?, ids[0]);
    Ok(())
}

/// A reply scoring each of `ids` with the matching ranks, by the handles of the current brief.
fn scores(
    session: &campaign_choregraph::ControlSession,
    setup: &animus_campaign_manager::ManagerSetup,
    ids: &[campaign_core::Id],
    ranks: [[i64; 2]; 3],
) -> Result<String, support::Failure> {
    let entries = ids
        .iter()
        .zip(ranks)
        .map(|(id, [first, second])| {
            let handle = support::handle(session.state(), setup, *id)?;
            Ok(format!(
                r#"{{"candidate":"{handle}","ranks":[{first},{second}]}}"#
            ))
        })
        .collect::<Result<Vec<String>, support::Failure>>()?;
    Ok(format!(r#"{{"scores":[{}]}}"#, entries.join(",")))
}
