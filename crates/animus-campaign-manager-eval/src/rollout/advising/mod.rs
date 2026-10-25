//! The manager's half of a turn: the setup, the menu, one model call through `advise`, and
//! publication of admitted events.

/// Host store that observes every in-memory write as durable.
struct MemoryStore;

impl animus_campaign_manager::DurableControlStore for MemoryStore {
    type Error = core::convert::Infallible;

    fn make_durable(
        &mut self,
        publication: &campaign_choregraph::ControlPublication,
    ) -> Result<campaign_choregraph::DurablePublicationEvidence, Self::Error> {
        let plan = publication.branch_plan();
        Ok(campaign_choregraph::DurablePublicationEvidence {
            history_event_id: publication.history_event_id(),
            branch_before: plan.before.clone(),
            branch_after: plan.after.clone(),
            durable_event: true,
            durable_branch_update: true,
        })
    }
}

/// The manager setup of `task`'s campaign under the task set's full mandate.
pub(in crate::rollout) fn manager_setup(
    set: &crate::TaskSet,
    task: &crate::EvalTask,
    state: &campaign_core::FrontierState,
) -> Result<animus_campaign_manager::ManagerSetup, crate::HostFailure> {
    let mandate = campaign_core::AdvisoryMandate {
        mandate_id: campaign_core::Id::absent(),
        campaign_id: state.admission().campaign_id,
        advisor_id: crate::history::bounds::id(campaign_core::IdKind::Advisor, &set.agent),
        terms: set.terms()?,
    }
    .derive()?;
    Ok(animus_campaign_manager::ManagerSetup {
        mandate,
        objective: task.objective.clone(),
        rank_dimensions: set.rank_dimensions,
        admission_rule: crate::history::bounds::admission_rule(),
        coalescing_policy_id: crate::history::bounds::coalescing_policy_id(),
    })
}

/// One manager turn: the menu, one model call through `advise`, and publication of admitted
/// events.
pub(in crate::rollout) fn manager_turn<M: animus_adapters::ModelAdapter>(
    campaign: &mut crate::TrialCampaign,
    setup: &animus_campaign_manager::ManagerSetup,
    model: &mut M,
) -> Result<crate::TurnOutcome, crate::HostFailure> {
    let state = campaign.session.state();
    let observed = &campaign.observed;
    let is_admitted = |id: campaign_core::Id| {
        state
            .candidates()
            .iter()
            .any(|admitted| admitted.candidate.candidate_id == id)
    };
    let labels: Vec<animus_campaign_manager::CandidateLabel> = campaign
        .entries
        .iter()
        .filter(|entry| is_admitted(entry.candidate_id))
        .map(|entry| animus_campaign_manager::CandidateLabel {
            candidate_id: entry.candidate_id,
            label: observed.get(&entry.candidate_id).map_or_else(
                || entry.source.label.clone(),
                |signal| format!("{} [signal {signal}]", entry.source.label),
            ),
        })
        .collect();
    let offers: Vec<animus_campaign_manager::CandidateOffer> = campaign
        .entries
        .iter()
        .filter(|entry| entry.is_offer && !is_admitted(entry.candidate_id))
        .map(|entry| animus_campaign_manager::CandidateOffer {
            record: entry.record.clone(),
            label: entry.source.label.clone(),
        })
        .collect();
    let menu = animus_campaign_manager::CandidateMenu {
        labels: &labels,
        offers: &offers,
    };
    let turn = match animus_campaign_manager::advise(model, state, setup, menu) {
        Ok(turn) => turn,
        Err(animus_campaign_manager::TurnFailure::Refused(
            animus_campaign_manager::ManagerError::Campaign(
                campaign_core::CampaignError::UnchangedState,
            ),
        )) => return Ok(crate::TurnOutcome::Unchanged),
        Err(animus_campaign_manager::TurnFailure::Refused(_)) => {
            return Ok(crate::TurnOutcome::Refused);
        }
        Err(animus_campaign_manager::TurnFailure::Model(_)) => {
            return Ok(crate::TurnOutcome::ModelFailed);
        }
    };
    let events = &turn.admission.events;
    campaign.session =
        animus_campaign_manager::publish_events(campaign.session.clone(), events, &mut MemoryStore)
            .map_err(|failure| match failure.cause {
                animus_campaign_manager::PublicationCause::Session(error) => {
                    crate::HostFailure::Session(error)
                }
                animus_campaign_manager::PublicationCause::Store(never) => match never {},
            })?;
    debug_assert_eq!(
        campaign.session.state().state_id(),
        turn.admission.receipt.state_after_id
    );
    Ok(crate::TurnOutcome::Admitted {
        events: u32::try_from(events.len()).map_err(|_| crate::HostFailure::Bound)?,
    })
}
