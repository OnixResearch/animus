//! One open campaign over fresh Choregraph history, admitted the way a host opens one.

const GRAPH_EVENTS: u32 = 512;
const PARENT_LINKS: u32 = 4;
const METADATA_ENTRIES: u32 = 4;
const LABEL_BYTES: u32 = 64;
const PROJECTION_EVENTS: u32 = 64;
const CONTROL_BRANCH: &str = "campaign-manager-control";
const ROOT_GENERATION: u64 = 0;
const ADMITTED_GENERATION: u64 = 1;
const DOMAIN: &[u8] = b"animus-campaign-manager-tests";
/// Every count bound of the test campaign.
const BOUND: u64 = 64;
/// The test campaign's rank-magnitude bound.
const RANK_MAGNITUDE: u64 = 1_000_000;
/// Units of the test campaign's one resource budget.
const BUDGET_UNITS: u64 = 64;
/// Declared cost of every test candidate.
const UNIT_COST: u64 = 1;
/// Depth of a candidate under the root.
const CANDIDATE_DEPTH: u64 = 1;

/// A deterministic test identity of `kind`.
pub fn id(kind: campaign_core::IdKind, label: &str) -> campaign_core::Id {
    campaign_core::derive_id(kind, label.as_bytes(), &[])
}

/// The tests' candidate admission rule.
pub const fn admission_rule() -> campaign_core::AdmissionRule {
    campaign_core::AdmissionRule {
        duplicate_candidate: campaign_core::DuplicateCandidateRule::Reject,
    }
}

/// One open campaign.
pub struct Fixture {
    /// The campaign's control session.
    pub session: campaign_choregraph::ControlSession,
}

impl Fixture {
    /// The record of a new candidate under the root, labelled `label`.
    pub fn record(&self, label: &str) -> campaign_core::CandidateRecord {
        candidate_record(self.session.state().admission(), label)
    }

    /// Publishes one host-built control event durably.
    pub fn publish(
        &mut self,
        event: campaign_core::CampaignControlEvent,
    ) -> Result<(), super::Failure> {
        let mut store = super::MemoryStore::durable();
        self.session =
            animus_campaign_manager::publish_events(self.session.clone(), &[event], &mut store)?;
        assert_eq!(store.writes, 1, "one event is one durable write");
        Ok(())
    }
}

/// Opens a campaign and publishes one candidate per label; returns the candidate identities in
/// label order.
pub fn frontier(labels: &[&str]) -> Result<(Fixture, Vec<campaign_core::Id>), super::Failure> {
    let mut fixture = open()?;
    let records: Vec<campaign_core::CandidateRecord> =
        labels.iter().map(|label| fixture.record(label)).collect();
    let ids = records
        .iter()
        .map(|record| record.clone().derive().map(|derived| derived.candidate_id))
        .collect::<Result<Vec<_>, _>>()?;
    let event = campaign_core::admit_candidates(
        fixture.session.state(),
        records,
        admission_rule(),
        id(campaign_core::IdKind::CoalescingPolicy, "coalesce"),
    )?;
    fixture.publish(event)?;
    assert_eq!(fixture.session.state().candidates().len(), labels.len());
    Ok((fixture, ids))
}

/// Opens a campaign with one root moment on fresh Choregraph history.
fn open() -> Result<Fixture, super::Failure> {
    let history_bounds = campaign_choregraph::HistoryLimits::new(
        GRAPH_EVENTS,
        PARENT_LINKS,
        METADATA_ENTRIES,
        LABEL_BYTES,
        PROJECTION_EVENTS,
    )?;
    let root = campaign_choregraph::HistoryGraph::new(history_bounds)?.admit(root_draft())?;
    let branch = campaign_choregraph::BranchRef {
        name: String::from(CONTROL_BRANCH),
        head: root.event,
        generation: ROOT_GENERATION,
    };
    let envelope = envelope();
    let adapter_id = envelope.adapter_id;
    let projection = campaign_choregraph::derive_projection_binding(
        &root.graph,
        &branch,
        envelope.projection_schema_id,
        adapter_id,
        envelope.control_domain_id,
    )?;
    let event = campaign_core::admit_campaign(admission(&envelope, projection, bounds()?))?;
    let control_root = campaign_choregraph::admit_control_root(
        &root.graph,
        event,
        envelope.projection_schema_id,
        adapter_id,
        envelope.control_domain_id,
    )?;
    let admitted = campaign_choregraph::BranchRef {
        head: control_root.envelope.history_event_id,
        generation: ADMITTED_GENERATION,
        ..branch
    };
    let session = campaign_choregraph::ControlSession::open(control_root, admitted, envelope)?;
    assert_eq!(session.state().admission().adapter_id, adapter_id);
    assert!(session.graph().contains(root.event));
    Ok(Fixture { session })
}

/// The root exploration moment's history draft.
fn root_draft() -> campaign_choregraph::EventDraft {
    campaign_choregraph::EventDraft::v1(
        campaign_choregraph::EventClass::Application,
        campaign_choregraph::Digest::derive(DOMAIN, b"root-workspace"),
        Vec::new(),
        campaign_choregraph::Digest::derive(DOMAIN, b"workspace-moment"),
    )
}

/// The projection identities and bound of the control history.
fn envelope() -> campaign_choregraph::ControlEnvelope {
    campaign_choregraph::ControlEnvelope {
        projection_schema_id: id(campaign_core::IdKind::Projection, "projection-schema"),
        adapter_id: id(campaign_core::IdKind::Adapter, "adapter"),
        control_domain_id: id(campaign_core::IdKind::ControlDomain, "domain"),
        maximum_events: PROJECTION_EVENTS,
    }
}

/// The campaign admission of the test campaign.
fn admission(
    envelope: &campaign_choregraph::ControlEnvelope,
    projection: campaign_core::ProjectionBinding,
    bounds: campaign_core::ExplorationBounds,
) -> campaign_core::CampaignAdmission {
    campaign_core::CampaignAdmission {
        campaign_id: campaign_core::Id::absent(),
        schema_version: campaign_core::CAMPAIGN_SCHEMA_VERSION,
        policy_id: id(campaign_core::IdKind::Policy, "policy"),
        guidance_profile_id: id(campaign_core::IdKind::GuidanceProfile, "guidance"),
        entropy_profile_id: id(campaign_core::IdKind::EntropyProfile, "entropy"),
        budget_profile_id: bounds.bounds_id,
        adapter_id: envelope.adapter_id,
        control_domain_id: envelope.control_domain_id,
        roots: vec![campaign_core::RootMoment {
            moment_id: id(campaign_core::IdKind::RootMoment, "root"),
            projection_id: projection.projection_id,
            adapter_id: envelope.adapter_id,
        }],
        projection,
        bounds,
        resource_budgets: vec![campaign_core::ResourceAmount {
            dimension_id: id(campaign_core::IdKind::ResourceDimension, "cpu"),
            amount: BUDGET_UNITS,
        }],
        operation_schema_ids: vec![id(campaign_core::IdKind::OperationSchema, "operation")],
    }
}

/// The test campaign's exploration bounds.
fn bounds() -> Result<campaign_core::ExplorationBounds, super::Failure> {
    Ok(campaign_core::ExplorationBounds {
        bounds_id: campaign_core::Id::absent(),
        max_roots: BOUND,
        max_candidates: BOUND,
        max_candidate_batch: BOUND,
        max_retained_candidates: BOUND,
        max_selections: BOUND,
        max_guidance_records: BOUND,
        max_outstanding: BOUND,
        max_depth: BOUND,
        max_child_moments: BOUND,
        max_derived_records: BOUND,
        max_rank_dimensions: BOUND,
        max_rank_magnitude: RANK_MAGNITUDE,
        max_findings: BOUND,
        max_control_events: BOUND,
        max_resource_dimensions: BOUND,
        max_operation_schemas: BOUND,
        max_prune_batch: BOUND,
    }
    .derive()?)
}

/// A new candidate under the admission's root, labelled `label`.
fn candidate_record(
    admission: &campaign_core::CampaignAdmission,
    label: &str,
) -> campaign_core::CandidateRecord {
    campaign_core::CandidateRecord {
        candidate_id: campaign_core::Id::absent(),
        campaign_id: admission.campaign_id,
        parent_moment_id: admission.roots[0].moment_id,
        operation_profile_id: id(campaign_core::IdKind::OperationProfile, label),
        operation_schema_id: admission.operation_schema_ids[0],
        source_adapter_id: admission.adapter_id,
        depth: CANDIDATE_DEPTH,
        cost_profile_id: id(campaign_core::IdKind::CostProfile, label),
        declared_costs: vec![campaign_core::ResourceAmount {
            dimension_id: admission.resource_budgets[0].dimension_id,
            amount: UNIT_COST,
        }],
        operation_generation_profile_id: id(campaign_core::IdKind::Policy, "generation"),
    }
}
