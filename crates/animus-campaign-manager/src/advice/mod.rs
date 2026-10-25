//! One manager turn: brief, model call, proposal, and Campaign review.

/// The advice of one turn: the proposal the reply produced and Campaign's admission of it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagerTurn {
    /// The proposal built from the model's reply.
    pub proposal: campaign_core::AdvisoryProposal,
    /// Campaign's admission: the control events to publish in order, and the receipt binding them
    /// to the proposal.
    pub admission: campaign_core::AdvisoryAdmission,
}

/// Runs one manager turn on `state`: briefs the campaign under `setup`'s mandate, asks `model` for
/// advice once, converts the reply into a proposal, and has Campaign review it.
///
/// The turn performs no effect beyond the one injected model call. It returns the admitted control
/// events without publishing them; the host publishes them in order, for example with
/// [`crate::publish_events`].
///
/// # Errors
///
/// Returns `Model` when the model adapter fails, and `Refused` with Campaign's
/// `CampaignStopped` for a stopped campaign (before any model call), the rendering refusals of
/// [`crate::render_brief`], the parsing and conversion refusals of [`crate::build_proposal`], or
/// Campaign's review refusal.
// r[impl agent_harness_runtime.campaign_manager.turn]
pub fn advise<M: animus_adapters::ModelAdapter>(
    model: &mut M,
    state: &campaign_core::FrontierState,
    setup: &crate::ManagerSetup,
    menu: crate::CandidateMenu<'_>,
) -> Result<ManagerTurn, crate::TurnFailure<M::Error>> {
    if state.stopped().is_some() {
        return Err(campaign_core::CampaignError::CampaignStopped.into());
    }
    let brief = campaign_core::brief_advisor(state, &setup.mandate)?;
    let rendered = crate::render_brief(state, &brief, setup, menu)?;
    let response = model
        .call(&rendered.request)
        .map_err(crate::TurnFailure::Model)?;
    let proposal = crate::build_proposal(state, setup, &rendered, response.message())?;
    let admission = campaign_core::review_advisory_proposal(
        state,
        campaign_core::AdvisoryReview {
            mandate: setup.mandate.clone(),
            proposal: proposal.clone(),
            admission_rule: setup.admission_rule,
            coalescing_policy_id: setup.coalescing_policy_id,
        },
    )?;
    debug_assert_eq!(admission.receipt.proposal_id, proposal.proposal_id);
    debug_assert_eq!(admission.receipt.state_before_id, state.state_id());
    Ok(ManagerTurn {
        proposal,
        admission,
    })
}
