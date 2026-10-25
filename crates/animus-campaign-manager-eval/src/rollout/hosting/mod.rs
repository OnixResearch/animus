//! The host's half of a turn: one planner step that runs one selection against the scenario's
//! hidden outcomes, or publishes the planner's stop.

/// Findings after which the planner's stopping rule stops the campaign.
const FINDING_LIMIT: u64 = 1;

/// The ranked selection profile that stops at the first finding.
pub(super) fn selection_profile(
    state: &campaign_core::FrontierState,
) -> Result<campaign_core::SelectionProfile, crate::HostFailure> {
    Ok(campaign_core::SelectionProfile {
        profile_id: campaign_core::Id::absent(),
        scheduling_policy_id: crate::history::bounds::id(
            campaign_core::IdKind::SchedulingProfile,
            "rank",
        ),
        admission_policy_id: crate::history::bounds::admission_rule().admission_policy_id()?,
        coalescing_policy_id: crate::history::bounds::coalescing_policy_id(),
        mode: campaign_core::SelectionMode::Ranked,
        decay: campaign_core::SelectionCountDecay::None,
        stopping: campaign_core::StoppingRule {
            finding_limit: Some(FINDING_LIMIT),
        },
        implementation: campaign_core::PolicyImplementation::ProductionCompiled {
            policy_id: state.admission().policy_id,
        },
    }
    .derive()?)
}

/// One host planner step: select and run one candidate, wait, or publish the planner's stop.
pub(super) fn host_step(
    campaign: &mut crate::TrialCampaign,
    profile: &campaign_core::SelectionProfile,
) -> Result<super::HostAction, crate::HostFailure> {
    let action = match plan(campaign, profile)? {
        campaign_core::NextDecision::Select(selection) => execute(campaign, profile, &selection)?,
        campaign_core::NextDecision::Wait(_) => super::HostAction::Waited,
        campaign_core::NextDecision::Stop(stop) => {
            publish_stop(campaign, stop.reason)?;
            super::HostAction::Stopped
        }
    };
    let is_target = matches!(
        action,
        super::HostAction::Selected {
            is_target: true,
            ..
        }
    );
    if is_target {
        settle_stop(campaign, profile)?;
    }
    let state = campaign.session.state();
    debug_assert!(action != super::HostAction::Stopped || state.stopped().is_some());
    debug_assert!(!is_target || !state.findings().is_empty());
    Ok(action)
}

/// Publishes the planner's stop once the target's finding is recorded.
fn settle_stop(
    campaign: &mut crate::TrialCampaign,
    profile: &campaign_core::SelectionProfile,
) -> Result<(), crate::HostFailure> {
    if let campaign_core::NextDecision::Stop(stop) = plan(campaign, profile)? {
        publish_stop(campaign, stop.reason)?;
    }
    Ok(())
}

/// Publishes a stop for `reason`.
fn publish_stop(
    campaign: &mut crate::TrialCampaign,
    reason: campaign_core::StopReason,
) -> Result<(), crate::HostFailure> {
    let event = campaign_core::admit_stop(campaign.session.state(), reason)?;
    let (session, _publication, _evidence) =
        crate::history::publish(campaign.session.clone(), event)?;
    campaign.session = session;
    debug_assert!(campaign.session.state().stopped().is_some());
    Ok(())
}

/// The planner's next decision under `profile`.
fn plan(
    campaign: &crate::TrialCampaign,
    profile: &campaign_core::SelectionProfile,
) -> Result<campaign_core::NextDecision, crate::HostFailure> {
    let session = &campaign.session;
    let state = session.state();
    let request = campaign_core::SelectionRequest {
        request_id: campaign_core::derive_id(
            campaign_core::IdKind::Request,
            b"animus-campaign-manager-eval-request",
            &[state.state_id()],
        ),
        campaign_id: state.admission().campaign_id,
        state_id: state.state_id(),
        profile: profile.clone(),
        entropy: None,
        expected_generation: session.binding().generation,
        expected_head_id: session.binding().head_id,
    };
    let planned = session.plan_next(campaign_choregraph::PublishedSelectionRequest { request })?;
    debug_assert_eq!(planned.state_id, state.state_id());
    Ok(planned.decision)
}

/// Publishes `selection`, runs it against the scenario once the fence releases it, and publishes
/// its result.
fn execute(
    campaign: &mut crate::TrialCampaign,
    profile: &campaign_core::SelectionProfile,
    selection: &campaign_core::SelectionDecision,
) -> Result<super::HostAction, crate::HostFailure> {
    release(campaign, profile, selection)?;
    let source = campaign
        .entry(selection.candidate_id)
        .ok_or(crate::HostFailure::UnknownCandidate)?
        .source
        .clone();
    settle(campaign, selection, &source)?;
    campaign
        .observed
        .insert(selection.candidate_id, source.signal);
    Ok(super::HostAction::Selected {
        signal: source.signal,
        is_target: source.is_target,
    })
}

/// Publishes `selection` and requires the publication fence to release it for execution.
fn release(
    campaign: &mut crate::TrialCampaign,
    profile: &campaign_core::SelectionProfile,
    selection: &campaign_core::SelectionDecision,
) -> Result<(), crate::HostFailure> {
    let event = campaign_core::CampaignControlEvent::SelectionPublished(
        campaign_core::SelectionEvent::new(selection.clone(), profile.clone(), None)?,
    );
    let (session, publication, evidence) =
        crate::history::publish(campaign.session.clone(), event)?;
    campaign.session = session;
    if campaign_choregraph::can_execute_selection_after_publication(
        selection,
        &publication,
        &evidence,
    )? {
        Ok(())
    } else {
        Err(crate::HostFailure::Fence)
    }
}

/// Publishes the result of running `source` for `selection`.
fn settle(
    campaign: &mut crate::TrialCampaign,
    selection: &campaign_core::SelectionDecision,
    source: &crate::TaskCandidate,
) -> Result<(), crate::HostFailure> {
    let state = campaign.session.state();
    let observation = campaign_core::ResultObservation {
        result_id: campaign_core::Id::absent(),
        campaign_id: state.admission().campaign_id,
        selection_id: selection.selection_id,
        parent_moment_id: selection.parent_moment_id,
        operation_id: selection.operation_id,
        adapter_id: state.admission().adapter_id,
        payload_id: campaign_core::derive_id(
            campaign_core::IdKind::EventPayload,
            format!("{}/result", campaign.trial_identity).as_bytes(),
            &[selection.selection_id],
        ),
        children: Vec::new(),
        findings: findings(source, &campaign.task_identity),
        derived_records: Vec::new(),
    }
    .derive()?;
    debug_assert_eq!(observation.selection_id, selection.selection_id);
    let event = campaign_core::admit_result(state, observation)?;
    let (session, _publication, _evidence) =
        crate::history::publish(campaign.session.clone(), event)?;
    campaign.session = session;
    debug_assert!(campaign.session.state().outstanding().is_empty());
    Ok(())
}

/// The finding a run of `source` records: the task's target finding, or none.
fn findings(source: &crate::TaskCandidate, task_identity: &str) -> Vec<campaign_core::FindingRef> {
    if !source.is_target {
        return Vec::new();
    }
    vec![campaign_core::FindingRef {
        finding_id: crate::history::bounds::finding_id(task_identity, source),
        priority_facts_id: crate::history::bounds::id(
            campaign_core::IdKind::SourceFacts,
            &source.key,
        ),
    }]
}
