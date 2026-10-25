//! One trial's campaign over fresh in-memory Choregraph history, opened the way a host opens one,
//! with the task's candidates admitted and its offers derived but not yet admitted.

pub(crate) mod bounds;

/// Events the trial's history graph admits.
const GRAPH_EVENTS: u32 = 1_024;
/// Control events one projection loads; covers every event a trial can publish.
const PROJECTION_EVENTS: u32 = 512;
const PARENT_LINKS: u32 = 4;
const METADATA_ENTRIES: u32 = 4;
const LABEL_BYTES: u32 = 64;
const CONTROL_BRANCH: &str = "campaign-manager-eval-control";
const ROOT_GENERATION: u64 = 0;
const ADMITTED_GENERATION: u64 = 1;
const DOMAIN: &[u8] = b"animus-campaign-manager-eval";
/// Declared cost of every scenario candidate, in selections.
const UNIT_COST: u64 = 1;
/// Depth of a candidate under the root.
const CANDIDATE_DEPTH: u64 = 1;

/// Why a host step of a trial failed.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HostFailure {
    /// Campaign refused a host-built event.
    Campaign(campaign_core::CampaignError),
    /// Choregraph history or the control session refused a step.
    Session(campaign_choregraph::AdapterError),
    /// The publication fence refused to release a selection for execution.
    Fence,
    /// A selection named a candidate the scenario does not hold.
    UnknownCandidate,
    /// A count exceeded its representation.
    Bound,
}

impl From<campaign_core::CampaignError> for HostFailure {
    fn from(error: campaign_core::CampaignError) -> Self {
        Self::Campaign(error)
    }
}

impl From<campaign_choregraph::AdapterError> for HostFailure {
    fn from(error: campaign_choregraph::AdapterError) -> Self {
        Self::Session(error)
    }
}

impl From<campaign_choregraph::HistoryError> for HostFailure {
    fn from(error: campaign_choregraph::HistoryError) -> Self {
        Self::Session(campaign_choregraph::AdapterError::from(error))
    }
}

/// One scenario candidate or offer with its Campaign record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateEntry {
    /// The derived candidate identity.
    pub candidate_id: campaign_core::Id,
    /// The derived candidate record.
    pub record: campaign_core::CandidateRecord,
    /// The scenario candidate.
    pub source: crate::TaskCandidate,
    /// Whether the manager must add it before it can run.
    pub is_offer: bool,
}

/// One trial's open campaign and its scenario entries.
#[derive(Clone, Debug)]
pub struct TrialCampaign {
    /// The campaign's control session.
    pub session: campaign_choregraph::ControlSession,
    /// Every candidate and offer of the task.
    pub entries: Vec<CandidateEntry>,
    /// The task's Harbor identity.
    pub task_identity: String,
    /// The trial's Harbor identity.
    pub trial_identity: String,
    /// The latest signal the host observed per run candidate.
    pub observed: std::collections::BTreeMap<campaign_core::Id, u32>,
}

impl TrialCampaign {
    /// Opens `task`'s campaign for `trial` and admits its candidates.
    ///
    /// # Errors
    ///
    /// Returns the [`HostFailure`] of the history, Campaign, or session step that refused.
    pub fn open(task: &crate::EvalTask, trial: &str) -> Result<Self, HostFailure> {
        let session = open_session(task)?;
        let admission = session.state().admission().clone();
        let entries = task
            .candidates
            .iter()
            .map(|source| (source, false))
            .chain(task.offers.iter().map(|source| (source, true)))
            .map(|(source, is_offer)| entry(&admission, trial, source, is_offer))
            .collect::<Result<Vec<_>, _>>()?;
        let records = entries
            .iter()
            .filter(|entry| !entry.is_offer)
            .map(|entry| entry.record.clone())
            .collect();
        let event = campaign_core::admit_candidates(
            session.state(),
            records,
            bounds::admission_rule(),
            bounds::coalescing_policy_id(),
        )?;
        let (session, _publication, _evidence) = publish(session, event)?;
        debug_assert_eq!(session.state().candidates().len(), task.candidates.len());
        Ok(Self {
            session,
            entries,
            task_identity: task.identity.clone(),
            trial_identity: String::from(trial),
            observed: std::collections::BTreeMap::new(),
        })
    }

    /// The scenario entry of `candidate_id`.
    #[must_use]
    pub fn entry(&self, candidate_id: campaign_core::Id) -> Option<&CandidateEntry> {
        self.entries
            .iter()
            .find(|entry| entry.candidate_id == candidate_id)
    }
}

/// The scenario entry of `source` under `admission`, its identities scoped to `trial`.
fn entry(
    admission: &campaign_core::CampaignAdmission,
    trial: &str,
    source: &crate::TaskCandidate,
    is_offer: bool,
) -> Result<CandidateEntry, HostFailure> {
    let scope = format!("{trial}/{}", source.key);
    let record = campaign_core::CandidateRecord {
        candidate_id: campaign_core::Id::absent(),
        campaign_id: admission.campaign_id,
        parent_moment_id: admission.roots[0].moment_id,
        operation_profile_id: bounds::id(campaign_core::IdKind::OperationProfile, &scope),
        operation_schema_id: admission.operation_schema_ids[0],
        source_adapter_id: admission.adapter_id,
        depth: CANDIDATE_DEPTH,
        cost_profile_id: bounds::id(campaign_core::IdKind::CostProfile, &source.key),
        declared_costs: vec![campaign_core::ResourceAmount {
            dimension_id: admission.resource_budgets[0].dimension_id,
            amount: UNIT_COST,
        }],
        operation_generation_profile_id: bounds::id(campaign_core::IdKind::Policy, "generation"),
    }
    .derive()?;
    debug_assert_eq!(record.candidate_id.kind, campaign_core::IdKind::Candidate);
    debug_assert_eq!(record.campaign_id, admission.campaign_id);
    Ok(CandidateEntry {
        candidate_id: record.candidate_id,
        record,
        source: source.clone(),
        is_offer,
    })
}

/// Opens a control session whose campaign has one root moment on fresh history.
fn open_session(
    task: &crate::EvalTask,
) -> Result<campaign_choregraph::ControlSession, HostFailure> {
    let history_bounds = campaign_choregraph::HistoryLimits::new(
        GRAPH_EVENTS,
        PARENT_LINKS,
        METADATA_ENTRIES,
        LABEL_BYTES,
        PROJECTION_EVENTS,
    )?;
    let draft = campaign_choregraph::EventDraft::v1(
        campaign_choregraph::EventClass::Application,
        campaign_choregraph::Digest::derive(DOMAIN, task.identity.as_bytes()),
        Vec::new(),
        campaign_choregraph::Digest::derive(DOMAIN, b"workspace-moment"),
    );
    let root = campaign_choregraph::HistoryGraph::new(history_bounds)?.admit(draft)?;
    let branch = campaign_choregraph::BranchRef {
        name: String::from(CONTROL_BRANCH),
        head: root.event,
        generation: ROOT_GENERATION,
    };
    let envelope = bounds::envelope(PROJECTION_EVENTS);
    let projection = campaign_choregraph::derive_projection_binding(
        &root.graph,
        &branch,
        envelope.projection_schema_id,
        envelope.adapter_id,
        envelope.control_domain_id,
    )?;
    let admission = bounds::admission(
        &envelope,
        projection,
        bounds::exploration_caps()?,
        task.selection_budget,
    );
    let control_root = campaign_choregraph::admit_control_root(
        &root.graph,
        campaign_core::admit_campaign(admission)?,
        envelope.projection_schema_id,
        envelope.adapter_id,
        envelope.control_domain_id,
    )?;
    let admitted = campaign_choregraph::BranchRef {
        head: control_root.envelope.history_event_id,
        generation: ADMITTED_GENERATION,
        ..branch
    };
    let session = campaign_choregraph::ControlSession::open(control_root, admitted, envelope)?;
    debug_assert!(session.graph().contains(root.event));
    debug_assert!(session.state().stopped().is_none());
    Ok(session)
}

/// Publishes one control event, observing its history event and branch update as durable.
///
/// # Errors
///
/// Returns the session refusal's adapter error.
pub(crate) fn publish(
    session: campaign_choregraph::ControlSession,
    event: campaign_core::CampaignControlEvent,
) -> Result<
    (
        campaign_choregraph::ControlSession,
        campaign_choregraph::ControlPublication,
        campaign_choregraph::DurablePublicationEvidence,
    ),
    HostFailure,
> {
    let publication = session.plan_publication(event)?;
    let plan = publication.branch_plan();
    let evidence = campaign_choregraph::DurablePublicationEvidence {
        history_event_id: publication.history_event_id(),
        branch_before: plan.before.clone(),
        branch_after: plan.after.clone(),
        durable_event: true,
        durable_branch_update: true,
    };
    let session = session
        .commit(&publication, &evidence)
        .map_err(|refusal| HostFailure::Session(refusal.into_parts().1))?;
    Ok((session, publication, evidence))
}
