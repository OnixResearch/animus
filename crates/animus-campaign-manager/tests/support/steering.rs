//! Steering helpers: manager setups, brief handles, planner picks, and one-shot turns.

/// Brief window of the full-manager terms.
const BRIEF_WINDOW: u64 = 64;
/// Records per proposal the full-manager terms grant for batches, revisions, and retirements.
const SMALL_GRANT: u64 = 16;
/// Records per proposal the full-manager terms grant for guidance and prune targets.
const LARGE_GRANT: u64 = 64;
/// The manager's objective in every test.
const OBJECTIVE: &str = "Find the parser input that hangs the decoder.";

/// Full-manager terms: every advisory action granted, with the limits of Campaign's default
/// advisory mandate profile.
pub fn full_terms() -> Result<campaign_core::AdvisoryTerms, super::Failure> {
    Ok(campaign_core::AdvisoryTerms {
        brief_window: BRIEF_WINDOW,
        grants: vec![
            grant(
                campaign_core::AdvisoryActionKind::CandidateBatch,
                SMALL_GRANT,
            ),
            grant(campaign_core::AdvisoryActionKind::Guidance, LARGE_GRANT),
            grant(campaign_core::AdvisoryActionKind::Revision, SMALL_GRANT),
            grant(campaign_core::AdvisoryActionKind::Retirement, SMALL_GRANT),
            grant(campaign_core::AdvisoryActionKind::Prune, LARGE_GRANT),
            grant(
                campaign_core::AdvisoryActionKind::Stop,
                campaign_core::STOP_GRANT_LIMIT,
            ),
        ],
    }
    .canonical()?)
}

/// One grant of `action`.
const fn grant(
    action: campaign_core::AdvisoryActionKind,
    max_per_proposal: u64,
) -> campaign_core::AdvisoryGrant {
    campaign_core::AdvisoryGrant {
        action,
        max_per_proposal,
    }
}

/// The manager setup of `state` under `terms`, bound to the test advisor.
pub fn setup(
    state: &campaign_core::FrontierState,
    terms: campaign_core::AdvisoryTerms,
) -> Result<animus_campaign_manager::ManagerSetup, super::Failure> {
    let mandate = campaign_core::AdvisoryMandate {
        mandate_id: campaign_core::Id::absent(),
        campaign_id: state.admission().campaign_id,
        advisor_id: super::id(campaign_core::IdKind::Advisor, "animus-campaign-manager"),
        terms,
    }
    .derive()?;
    Ok(animus_campaign_manager::ManagerSetup {
        mandate,
        objective: String::from(OBJECTIVE),
        rank_dimensions: super::RANK_DIMENSIONS,
        admission_rule: super::admission_rule(),
        coalescing_policy_id: super::id(campaign_core::IdKind::CoalescingPolicy, "coalesce"),
    })
}

/// A menu without labels or offers.
pub const fn no_menu() -> animus_campaign_manager::CandidateMenu<'static> {
    animus_campaign_manager::CandidateMenu {
        labels: &[],
        offers: &[],
    }
}

/// The brief handle the manager gives `candidate_id` in `state` under `setup`.
pub fn handle(
    state: &campaign_core::FrontierState,
    setup: &animus_campaign_manager::ManagerSetup,
    candidate_id: campaign_core::Id,
) -> Result<String, super::Failure> {
    let brief = campaign_core::brief_advisor(state, &setup.mandate)?;
    let position = brief
        .candidates
        .iter()
        .position(|entry| entry.candidate.candidate_id == candidate_id)
        .ok_or(super::Failure::Unexpected("the candidate is not listed"))?;
    Ok(format!("c{}", position.saturating_add(1)))
}

/// The candidate the ranked planner selects next through the session.
pub fn selected(
    session: &campaign_choregraph::ControlSession,
) -> Result<campaign_core::Id, super::Failure> {
    let state = session.state();
    let request = campaign_core::SelectionRequest {
        request_id: super::id(campaign_core::IdKind::Request, "request"),
        campaign_id: state.admission().campaign_id,
        state_id: state.state_id(),
        profile: selection_profile(state)?,
        entropy: None,
        expected_generation: session.binding().generation,
        expected_head_id: session.binding().head_id,
    };
    let plan = session.plan_next(campaign_choregraph::PublishedSelectionRequest { request })?;
    assert_eq!(plan.state_id, state.state_id());
    let campaign_core::NextDecision::Select(selection) = plan.decision else {
        return Err(super::Failure::Unexpected("the planner did not select"));
    };
    assert_eq!(selection.state_before_id, state.state_id());
    Ok(selection.candidate_id)
}

/// The ranked selection profile of the test campaign.
fn selection_profile(
    state: &campaign_core::FrontierState,
) -> Result<campaign_core::SelectionProfile, super::Failure> {
    Ok(campaign_core::SelectionProfile {
        profile_id: campaign_core::Id::absent(),
        scheduling_policy_id: super::id(campaign_core::IdKind::SchedulingProfile, "schedule"),
        admission_policy_id: super::admission_rule().admission_policy_id()?,
        coalescing_policy_id: super::id(campaign_core::IdKind::CoalescingPolicy, "coalesce"),
        mode: campaign_core::SelectionMode::Ranked,
        decay: campaign_core::SelectionCountDecay::None,
        stopping: campaign_core::StoppingRule {
            finding_limit: None,
        },
        implementation: campaign_core::PolicyImplementation::ProductionCompiled {
            policy_id: state.admission().policy_id,
        },
    }
    .derive()?)
}

/// One turn with a model that gives `reply`, over no labels or offers.
pub fn advise_once(
    fixture: &super::Fixture,
    setup: &animus_campaign_manager::ManagerSetup,
    reply: &str,
) -> Result<animus_campaign_manager::ManagerTurn, animus_campaign_manager::TurnFailure<&'static str>>
{
    let mut model = super::ScriptedModel::replying(reply);
    let turn =
        animus_campaign_manager::advise(&mut model, fixture.session.state(), setup, no_menu());
    assert_eq!(
        model.prompts.len(),
        1,
        "a turn over a valid setup asks the model once"
    );
    turn
}

/// Publishes a turn's events with an always-durable store.
pub fn publish(
    session: campaign_choregraph::ControlSession,
    turn: &animus_campaign_manager::ManagerTurn,
) -> Result<campaign_choregraph::ControlSession, super::Failure> {
    let mut store = super::MemoryStore::durable();
    let published =
        animus_campaign_manager::publish_events(session, &turn.admission.events, &mut store)?;
    assert_eq!(
        published.state().state_id(),
        turn.admission.receipt.state_after_id,
        "publication reaches the reviewed state"
    );
    Ok(published)
}

/// Short label of a control event's kind.
pub const fn kind(event: &campaign_core::CampaignControlEvent) -> &'static str {
    match event {
        campaign_core::CampaignControlEvent::CandidatesAdmitted(_) => "batch",
        campaign_core::CampaignControlEvent::GuidanceAdmitted(_) => "guidance",
        campaign_core::CampaignControlEvent::CandidateRetired(_) => "retirement",
        campaign_core::CampaignControlEvent::CandidatesPruned(_) => "prune",
        campaign_core::CampaignControlEvent::StopAccepted(_) => "stop",
        campaign_core::CampaignControlEvent::CampaignAdmitted { .. }
        | campaign_core::CampaignControlEvent::SelectionPublished(_)
        | campaign_core::CampaignControlEvent::ResultObserved(_)
        | campaign_core::CampaignControlEvent::UnobservedClosed(_) => "other",
    }
}
