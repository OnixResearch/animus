//! Refused campaign-manager turns and failed publications: nothing reaches the control branch
//! that Campaign's review did not admit, and an invalid setup never reaches the model.

mod support;

#[test]
// r[verify agent_harness_runtime.campaign_manager.reply]
// r[verify agent_harness_runtime.campaign_manager.refusals]
fn malformed_replies_are_refused_before_review() -> Result<(), support::Failure> {
    let (fixture, _ids) = support::frontier(&["alpha"])?;
    let setup = support::setup(fixture.session.state(), support::full_terms()?)?;
    let malformed = [
        "I would focus on alpha.",
        r#"{"scores":[],"extra":1}"#,
        r#"{"scores":[{"candidate":"c1","ranks":[1.5,1]}]}"#,
        r#"{"stop":"now"}"#,
    ];
    for reply in malformed {
        assert_eq!(
            animus_campaign_manager::parse_reply(reply),
            Err(animus_campaign_manager::ManagerError::MalformedReply),
            "{reply}"
        );
        assert_eq!(
            support::advise_once(&fixture, &setup, reply).err(),
            Some(animus_campaign_manager::TurnFailure::Refused(
                animus_campaign_manager::ManagerError::MalformedReply
            )),
            "{reply}"
        );
    }
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager.refusals]
fn replies_outside_the_brief_are_refused() -> Result<(), support::Failure> {
    let (fixture, ids) = support::frontier(&["alpha", "beta"])?;
    let setup = support::setup(fixture.session.state(), support::full_terms()?)?;
    let listed = support::handle(fixture.session.state(), &setup, ids[0])?;
    let unknown = |handle: &str| animus_campaign_manager::ManagerError::UnknownHandle {
        handle: String::from(handle),
    };
    let cases = [
        (
            String::from(r#"{"scores":[{"candidate":"c9","ranks":[1,1]}]}"#),
            unknown("c9"),
        ),
        (
            String::from(r#"{"scores":[{"candidate":"c01","ranks":[1,1]}]}"#),
            unknown("c01"),
        ),
        (String::from(r#"{"add":[{"offer":"o1"}]}"#), unknown("o1")),
        (
            format!(r#"{{"scores":[{{"candidate":"{listed}","ranks":[1]}}]}}"#),
            animus_campaign_manager::ManagerError::RankLength {
                handle: listed.clone(),
            },
        ),
        (
            format!(r#"{{"scores":[{{"candidate":"{listed}","ranks":[1,9999999999]}}]}}"#),
            animus_campaign_manager::ManagerError::RankMagnitude {
                handle: listed.clone(),
            },
        ),
        (
            format!(r#"{{"retire":[{{"candidate":"{listed}","reason":"two\nlines"}}]}}"#),
            animus_campaign_manager::ManagerError::InvalidText { field: "reason" },
        ),
    ];
    for (reply, expected) in cases {
        assert!(
            animus_campaign_manager::parse_reply(&reply).is_ok(),
            "{reply} is well-formed"
        );
        assert_eq!(
            support::advise_once(&fixture, &setup, &reply).err(),
            Some(animus_campaign_manager::TurnFailure::Refused(expected)),
            "{reply}"
        );
    }
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager.refusals]
fn ungranted_advice_and_model_failures_refuse_the_turn() -> Result<(), support::Failure> {
    let (fixture, _ids) = support::frontier(&["alpha"])?;
    let mut scorer_terms = support::full_terms()?;
    scorer_terms
        .grants
        .retain(|grant| grant.action == campaign_core::AdvisoryActionKind::Guidance);
    let scorer = support::setup(fixture.session.state(), scorer_terms)?;
    assert_eq!(
        support::advise_once(&fixture, &scorer, r#"{"stop":{"reason":"done"}}"#).err(),
        Some(animus_campaign_manager::TurnFailure::Refused(
            animus_campaign_manager::ManagerError::Campaign(
                campaign_core::CampaignError::UngrantedAdvice {
                    action: campaign_core::AdvisoryActionKind::Stop
                }
            )
        ))
    );
    let mut silent = support::ScriptedModel::silent();
    let failure = animus_campaign_manager::advise(
        &mut silent,
        fixture.session.state(),
        &scorer,
        support::no_menu(),
    );
    assert_eq!(
        failure.err(),
        Some(animus_campaign_manager::TurnFailure::Model(
            "model unavailable"
        ))
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager.refusals]
fn invalid_setups_and_menus_never_reach_the_model() -> Result<(), support::Failure> {
    let (fixture, ids) = support::frontier(&["alpha"])?;
    let setup = support::setup(fixture.session.state(), support::full_terms()?)?;
    let mut long_objective = setup.clone();
    long_objective.objective =
        "x".repeat(animus_campaign_manager::MAX_OBJECTIVE_CHARS.saturating_add(1));
    let mut no_ranks = setup.clone();
    no_ranks.rank_dimensions = 0;
    let offer = animus_campaign_manager::CandidateOffer {
        record: fixture.record("offered"),
        label: String::from("an offered candidate"),
    };
    let crowded = vec![offer; animus_campaign_manager::MAX_OFFERS.saturating_add(1)];
    let bad_label = [animus_campaign_manager::CandidateLabel {
        candidate_id: ids[0],
        label: String::from("tab\tlabel"),
    }];
    let crowded_menu = animus_campaign_manager::CandidateMenu {
        labels: &[],
        offers: &crowded,
    };
    let labelled_menu = animus_campaign_manager::CandidateMenu {
        labels: &bad_label,
        offers: &[],
    };
    let cases = [
        (
            &long_objective,
            support::no_menu(),
            animus_campaign_manager::ManagerError::InvalidText { field: "objective" },
        ),
        (
            &no_ranks,
            support::no_menu(),
            animus_campaign_manager::ManagerError::RankDimensions,
        ),
        (
            &setup,
            crowded_menu,
            animus_campaign_manager::ManagerError::TooManyOffers,
        ),
        (
            &setup,
            labelled_menu,
            animus_campaign_manager::ManagerError::InvalidText { field: "label" },
        ),
    ];
    for (case_setup, menu, expected) in cases {
        let mut unasked = support::ScriptedModel::replying(r#"{"scores":[]}"#);
        let refusal = animus_campaign_manager::advise(
            &mut unasked,
            fixture.session.state(),
            case_setup,
            menu,
        );
        assert_eq!(
            refusal.err(),
            Some(animus_campaign_manager::TurnFailure::Refused(expected))
        );
        assert!(
            unasked.prompts.is_empty(),
            "an invalid setup never reaches the model"
        );
    }
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager.refusals]
fn a_stopped_campaign_is_refused_before_any_model_call() -> Result<(), support::Failure> {
    let (mut fixture, _ids) = support::frontier(&["alpha"])?;
    let setup = support::setup(fixture.session.state(), support::full_terms()?)?;
    let reason_id = support::id(campaign_core::IdKind::Reason, "host stop");
    let stop = campaign_core::admit_stop(
        fixture.session.state(),
        campaign_core::StopReason::ConsumerRequested { reason_id },
    )?;
    fixture.publish(stop)?;
    let mut model = support::ScriptedModel::replying(r#"{"scores":[]}"#);
    let refusal = animus_campaign_manager::advise(
        &mut model,
        fixture.session.state(),
        &setup,
        support::no_menu(),
    );
    assert_eq!(
        refusal.err(),
        Some(animus_campaign_manager::TurnFailure::Refused(
            animus_campaign_manager::ManagerError::Campaign(
                campaign_core::CampaignError::CampaignStopped
            )
        ))
    );
    assert!(model.prompts.is_empty(), "the model was never called");
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.campaign_manager.publication]
fn a_store_failure_keeps_the_committed_prefix() -> Result<(), support::Failure> {
    let (fixture, ids) = support::frontier(&["alpha", "beta"])?;
    let setup = support::setup(fixture.session.state(), support::full_terms()?)?;
    let first = support::handle(fixture.session.state(), &setup, ids[0])?;
    let second = support::handle(fixture.session.state(), &setup, ids[1])?;
    let reply = format!(
        r#"{{"scores":[{{"candidate":"{first}","ranks":[3,3]}},{{"candidate":"{second}","ranks":[2,2]}}]}}"#
    );
    let turn = support::advise_once(&fixture, &setup, &reply)?;
    let before = fixture.session.state().control_event_count();
    let mut store = support::MemoryStore::failing_at(1);
    let Err(failure) = animus_campaign_manager::publish_events(
        fixture.session,
        &turn.admission.events,
        &mut store,
    ) else {
        return Err(support::Failure::Unexpected(
            "the failing store published every event",
        ));
    };
    assert_eq!(failure.committed, 1);
    assert_eq!(
        failure.cause,
        animus_campaign_manager::PublicationCause::Store("store unavailable")
    );
    assert_eq!(
        failure.session.state().control_event_count(),
        before.saturating_add(1)
    );
    let remaining = turn
        .admission
        .events
        .get(1..)
        .ok_or(support::Failure::Unexpected("the turn has no second event"))?;
    let resumed = animus_campaign_manager::publish_events(
        failure.session,
        remaining,
        &mut support::MemoryStore::durable(),
    )?;
    assert_eq!(
        resumed.state().state_id(),
        turn.admission.receipt.state_after_id
    );
    Ok(())
}
