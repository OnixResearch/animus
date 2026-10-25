//! Scores and added offers: guidance records, revisions, and new candidates.

/// Guidance generation of a candidate's first record.
const FIRST_GENERATION: u64 = 1;
/// Step from a record's generation to its successor's.
const NEXT_GENERATION: u64 = 1;

/// Facts every score conversion of one reply shares.
pub(super) struct Context<'reply> {
    /// The reviewed state.
    pub(super) state: &'reply campaign_core::FrontierState,
    /// The handles and score domain the brief presented.
    pub(super) handles: &'reply crate::BriefHandles,
    /// Source facts of the reply.
    pub(super) source_facts_id: campaign_core::Id,
    /// Reason of a rescore that lowers ranks.
    pub(super) rescore_reason_id: campaign_core::Id,
}

impl Context<'_> {
    /// A guidance record of `candidate_id` at `generation` with checked `ranks`.
    fn record(
        &self,
        candidate_id: campaign_core::Id,
        generation: u64,
        ranks: alloc::vec::Vec<i64>,
    ) -> campaign_core::GuidanceRecord {
        let admission = self.state.admission();
        campaign_core::GuidanceRecord {
            guidance_id: campaign_core::Id::absent(),
            campaign_id: admission.campaign_id,
            candidate_id,
            generation,
            ranks,
            source_facts_id: self.source_facts_id,
            ordering_profile_id: admission.guidance_profile_id,
            adapter_id: admission.adapter_id,
        }
    }

    /// `ranks` after checking their count and magnitude against the presented domain.
    fn checked_ranks(
        &self,
        handle: &str,
        ranks: &[i64],
    ) -> Result<alloc::vec::Vec<i64>, crate::ManagerError> {
        let has_presented_shape = u32::try_from(ranks.len())
            .is_ok_and(|dimensions| dimensions == self.handles.rank_dimensions());
        if !has_presented_shape {
            return Err(crate::ManagerError::RankLength {
                handle: alloc::string::String::from(handle),
            });
        }
        let magnitude = self.handles.rank_magnitude();
        if ranks.iter().any(|rank| rank.unsigned_abs() > magnitude) {
            return Err(crate::ManagerError::RankMagnitude {
                handle: alloc::string::String::from(handle),
            });
        }
        Ok(ranks.to_vec())
    }
}

/// Adds the reply's scores of presented candidates as guidance records or revisions.
// r[impl agent_harness_runtime.campaign_manager.proposal]
pub(super) fn add_scores(
    context: &Context<'_>,
    scores: &[crate::ScoreReply],
    proposal: &mut campaign_core::AdvisoryProposal,
) -> Result<(), crate::ManagerError> {
    for score in scores {
        let entry = context.handles.candidate(&score.candidate).ok_or_else(|| {
            crate::ManagerError::UnknownHandle {
                handle: score.candidate.clone(),
            }
        })?;
        let ranks = context.checked_ranks(&score.candidate, &score.ranks)?;
        let candidate_id = entry.candidate.candidate_id;
        let Some(latest) = &entry.latest_guidance else {
            let record = context.record(candidate_id, FIRST_GENERATION, ranks);
            proposal.guidance.push(record);
            continue;
        };
        let generation = latest
            .generation
            .checked_add(NEXT_GENERATION)
            .ok_or(campaign_core::CampaignError::ArithmeticOverflow)?;
        match ranks.as_slice().cmp(latest.ranks.as_slice()) {
            core::cmp::Ordering::Greater => {
                proposal
                    .guidance
                    .push(context.record(candidate_id, generation, ranks));
            }
            core::cmp::Ordering::Less => {
                proposal
                    .revisions
                    .push(campaign_core::GuidanceRevisionRequest {
                        candidate_id,
                        successor: context.record(candidate_id, generation, ranks),
                        invalidated_guidance_id: latest.guidance_id,
                        reason_id: context.rescore_reason_id,
                    });
            }
            core::cmp::Ordering::Equal => {}
        }
    }
    debug_assert!(proposal.guidance.len() <= scores.len());
    debug_assert!(proposal.revisions.len() <= scores.len());
    Ok(())
}

/// Adds the reply's chosen offers as a candidate batch, with first scores where given.
pub(super) fn add_offers(
    context: &Context<'_>,
    added: &[crate::AddReply],
    proposal: &mut campaign_core::AdvisoryProposal,
) -> Result<(), crate::ManagerError> {
    for add in added {
        let offer = context.handles.offer(&add.offer).ok_or_else(|| {
            crate::ManagerError::UnknownHandle {
                handle: add.offer.clone(),
            }
        })?;
        let record = offer.record.clone().derive()?;
        if let Some(ranks) = &add.ranks {
            let ranks = context.checked_ranks(&add.offer, ranks)?;
            let guidance = context.record(record.candidate_id, FIRST_GENERATION, ranks);
            proposal.guidance.push(guidance);
        }
        proposal.candidates.push(record);
    }
    debug_assert!(proposal.candidates.len() >= added.len());
    Ok(())
}
