//! Converting the model's reply into a Campaign advisory proposal bound to its brief.

mod ranking;

/// Frame label of the source-facts identity of one model reply.
const REPLY_FACTS_LABEL: &[u8] = b"animus-campaign-manager-reply-v1";
/// Frame label of a reason identity the manager derives from a reason label.
const REASON_LABEL: &[u8] = b"animus-campaign-manager-reason-v1";
/// Reason label of a rescore that lowers a candidate's ranks.
const RESCORE_REASON: &str = "advisor-rescore";

/// Parses `message`, the model's whole reply to `rendered`, and builds its advisory proposal for
/// `state` under `setup`.
///
/// Handles resolve only to what `rendered` presented, and every rank vector must have the
/// presented length and magnitude. A score of an unscored candidate, or one that raises its
/// ranks, becomes a guidance record; one that lowers them becomes an explicit revision; one that
/// repeats them is dropped. The proposal binds `message`, the model's whole reply, as its source
/// facts.
///
/// # Errors
///
/// Returns the parsing refusal of [`crate::parse_reply`], `UnknownHandle`, `RankLength`,
/// `RankMagnitude`, `InvalidText` for a reason label, and Campaign's refusal when a record or the
/// proposal does not derive.
// r[impl agent_harness_runtime.campaign_manager.proposal]
pub fn build_proposal(
    state: &campaign_core::FrontierState,
    setup: &crate::ManagerSetup,
    rendered: &crate::RenderedBrief,
    message: &str,
) -> Result<campaign_core::AdvisoryProposal, crate::ManagerError> {
    let reply = crate::parse_reply(message)?;
    let brief = &rendered.brief;
    let handles = &rendered.handles;
    let source_facts_id = campaign_core::identity::derive_id_with(
        campaign_core::IdKind::SourceFacts,
        REPLY_FACTS_LABEL,
        |writer| Ok(writer.bytes(message.as_bytes())),
    )?;
    let context = ranking::Context {
        state,
        handles,
        source_facts_id,
        rescore_reason_id: reason_id(setup, RESCORE_REASON)?,
    };
    let mut proposal = campaign_core::AdvisoryProposal {
        proposal_id: campaign_core::Id::absent(),
        campaign_id: brief.campaign_id,
        state_id: brief.state_id,
        brief_id: brief.brief_id,
        mandate_id: setup.mandate.mandate_id,
        advisor_id: setup.mandate.advisor_id,
        source_facts_id,
        candidates: alloc::vec::Vec::new(),
        guidance: alloc::vec::Vec::new(),
        revisions: alloc::vec::Vec::new(),
        retirements: alloc::vec::Vec::new(),
        prune: None,
        stop_reason_id: None,
    };
    ranking::add_scores(&context, &reply.scores, &mut proposal)?;
    ranking::add_offers(&context, &reply.add, &mut proposal)?;
    add_removals(setup, handles, &reply, &mut proposal)?;
    debug_assert_eq!(proposal.brief_id, brief.brief_id);
    debug_assert_eq!(proposal.source_facts_id, source_facts_id);
    Ok(proposal.derive()?)
}

/// Adds the reply's retirements, prune, and stop request.
fn add_removals(
    setup: &crate::ManagerSetup,
    handles: &crate::BriefHandles,
    reply: &crate::ManagerReply,
    proposal: &mut campaign_core::AdvisoryProposal,
) -> Result<(), crate::ManagerError> {
    debug_assert!(proposal.retirements.is_empty());
    for retire in &reply.retire {
        proposal
            .retirements
            .push(campaign_core::AdvisoryRetirement {
                candidate_id: candidate_id(handles, &retire.candidate)?,
                reason_id: reason_id(setup, &retire.reason)?,
            });
    }
    if let Some(prune) = &reply.prune {
        let candidates = prune
            .candidates
            .iter()
            .map(|handle| candidate_id(handles, handle))
            .collect::<Result<alloc::vec::Vec<_>, _>>()?;
        proposal.prune = Some(campaign_core::AdvisoryPrune {
            candidates,
            reason_id: reason_id(setup, &prune.reason)?,
        });
    }
    if let Some(stop) = &reply.stop {
        proposal.stop_reason_id = Some(reason_id(setup, &stop.reason)?);
    }
    debug_assert_eq!(proposal.retirements.len(), reply.retire.len());
    debug_assert_eq!(proposal.stop_reason_id.is_some(), reply.stop.is_some());
    Ok(())
}

/// The candidate a `c<n>` handle names.
fn candidate_id(
    handles: &crate::BriefHandles,
    handle: &str,
) -> Result<campaign_core::Id, crate::ManagerError> {
    handles
        .candidate(handle)
        .map(|entry| entry.candidate.candidate_id)
        .ok_or_else(|| crate::ManagerError::UnknownHandle {
            handle: alloc::string::String::from(handle),
        })
}

/// The `Reason` identity of a reason label, scoped to the manager's advisor identity.
fn reason_id(
    setup: &crate::ManagerSetup,
    label: &str,
) -> Result<campaign_core::Id, crate::ManagerError> {
    crate::host::check_text(label, crate::MAX_REASON_CHARS, "reason")?;
    Ok(campaign_core::identity::derive_id_with(
        campaign_core::IdKind::Reason,
        REASON_LABEL,
        |writer| Ok(writer.id(setup.mandate.advisor_id).bytes(label.as_bytes())),
    )?)
}
