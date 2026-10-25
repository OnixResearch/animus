//! The trial campaign's admission: identities, exploration bounds, and the selection budget.

/// Count bound of every exploration bound without a dedicated value.
const BOUND: u64 = 64;
/// Guidance records a trial may admit: every candidate rescored on every turn.
const GUIDANCE_RECORDS: u64 = 1_024;
/// Control events a trial may publish; at most eight per turn over [`crate::MAX_TURNS`] turns.
const CONTROL_EVENTS: u64 = 512;
/// The campaign's rank-magnitude bound.
const RANK_MAGNITUDE: u64 = 1_000_000;
/// Rank dimensions the campaign admits.
const RANK_DIMENSIONS: u64 = 4;
const LABEL_IDS: &[u8] = b"animus-campaign-manager-eval";

/// A deterministic identity of `kind` labelled `label`.
pub(crate) fn id(kind: campaign_core::IdKind, label: &str) -> campaign_core::Id {
    let framed = [LABEL_IDS, label.as_bytes()].concat();
    campaign_core::derive_id(kind, &framed, &[])
}

/// The finding a run of `target` in task `task_identity` records; the verifier derives the same.
pub(crate) fn finding_id(task_identity: &str, target: &crate::TaskCandidate) -> campaign_core::Id {
    id(
        campaign_core::IdKind::Finding,
        &format!("{task_identity}/{}", target.key),
    )
}

/// The trials' candidate admission rule.
pub(crate) const fn admission_rule() -> campaign_core::AdmissionRule {
    campaign_core::AdmissionRule {
        duplicate_candidate: campaign_core::DuplicateCandidateRule::Reject,
    }
}

/// The trials' coalescing policy identity.
pub(crate) fn coalescing_policy_id() -> campaign_core::Id {
    id(campaign_core::IdKind::CoalescingPolicy, "coalesce")
}

/// The projection identities and bound of the control history.
pub(super) fn envelope(projection_events: u32) -> campaign_choregraph::ControlEnvelope {
    campaign_choregraph::ControlEnvelope {
        projection_schema_id: id(campaign_core::IdKind::Projection, "projection-schema"),
        adapter_id: id(campaign_core::IdKind::Adapter, "adapter"),
        control_domain_id: id(campaign_core::IdKind::ControlDomain, "domain"),
        maximum_events: projection_events,
    }
}

/// The campaign admission under `caps`, with a budget of `selection_budget` unit-cost selections.
pub(super) fn admission(
    envelope: &campaign_choregraph::ControlEnvelope,
    projection: campaign_core::ProjectionBinding,
    caps: campaign_core::ExplorationBounds,
    selection_budget: u32,
) -> campaign_core::CampaignAdmission {
    campaign_core::CampaignAdmission {
        campaign_id: campaign_core::Id::absent(),
        schema_version: campaign_core::CAMPAIGN_SCHEMA_VERSION,
        policy_id: id(campaign_core::IdKind::Policy, "policy"),
        guidance_profile_id: id(campaign_core::IdKind::GuidanceProfile, "guidance"),
        entropy_profile_id: id(campaign_core::IdKind::EntropyProfile, "entropy"),
        budget_profile_id: caps.bounds_id,
        adapter_id: envelope.adapter_id,
        control_domain_id: envelope.control_domain_id,
        roots: vec![campaign_core::RootMoment {
            moment_id: id(campaign_core::IdKind::RootMoment, "root"),
            projection_id: projection.projection_id,
            adapter_id: envelope.adapter_id,
        }],
        projection,
        bounds: caps,
        resource_budgets: vec![campaign_core::ResourceAmount {
            dimension_id: id(campaign_core::IdKind::ResourceDimension, "selections"),
            amount: u64::from(selection_budget),
        }],
        operation_schema_ids: vec![id(campaign_core::IdKind::OperationSchema, "operation")],
    }
}

/// The trial campaign's exploration caps.
pub(super) fn exploration_caps() -> campaign_core::CampaignResult<campaign_core::ExplorationBounds>
{
    campaign_core::ExplorationBounds {
        bounds_id: campaign_core::Id::absent(),
        max_roots: BOUND,
        max_candidates: BOUND,
        max_candidate_batch: BOUND,
        max_retained_candidates: BOUND,
        max_selections: BOUND,
        max_guidance_records: GUIDANCE_RECORDS,
        max_outstanding: BOUND,
        max_depth: BOUND,
        max_child_moments: BOUND,
        max_derived_records: BOUND,
        max_rank_dimensions: RANK_DIMENSIONS,
        max_rank_magnitude: RANK_MAGNITUDE,
        max_findings: BOUND,
        max_control_events: CONTROL_EVENTS,
        max_resource_dimensions: BOUND,
        max_operation_schemas: BOUND,
        max_prune_batch: BOUND,
    }
    .derive()
}
