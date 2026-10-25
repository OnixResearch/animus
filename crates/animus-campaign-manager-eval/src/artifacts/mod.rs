//! The job's in-memory artifact store: what an environment run leaves for the separate verifier.
//!
//! The store stands in for the shared filesystem a Harbor trial leaves behind. The environment
//! writes one artifact per trial, and the verifier reads it by trial identity.

/// What one trial leaves for its verifier.
#[derive(Clone, Debug)]
pub struct TrialArtifact {
    /// The trial's Choregraph history.
    pub graph: campaign_choregraph::HistoryGraph,
    /// The control branch at the trial's end.
    pub branch: campaign_choregraph::BranchRef,
    /// The control payloads the branch's events carry.
    pub payloads: Vec<campaign_choregraph::ControlPayloadEnvelope>,
    /// The control envelope the history validates under.
    pub envelope: campaign_choregraph::ControlEnvelope,
    /// The final frontier state identity the environment claims.
    pub claimed_state_id: campaign_core::Id,
    /// The environment's turn ledger.
    pub turns: Vec<crate::TurnRecord>,
    /// The trial's ATIF projection.
    pub trajectory: animus_runtime::harbor_eval::AtifProjection,
}

impl TrialArtifact {
    /// The artifact of a finished run.
    #[must_use]
    pub fn capture(
        run: &crate::TrialRun,
        trajectory: animus_runtime::harbor_eval::AtifProjection,
    ) -> Self {
        let session = &run.campaign.session;
        Self {
            graph: session.graph().clone(),
            branch: session.branch().clone(),
            payloads: session.payloads().to_vec(),
            envelope: *session.envelope(),
            claimed_state_id: session.state().state_id(),
            turns: run.turns.clone(),
            trajectory,
        }
    }
}

/// Artifacts by trial identity.
#[derive(Debug)]
pub struct ArtifactStore {
    entries: core::cell::RefCell<std::collections::BTreeMap<String, TrialArtifact>>,
}

impl ArtifactStore {
    /// An empty store.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            entries: core::cell::RefCell::new(std::collections::BTreeMap::new()),
        }
    }

    /// Records `artifact` as `trial`'s, replacing an earlier one.
    pub fn insert(&self, trial: &str, artifact: TrialArtifact) {
        self.entries
            .borrow_mut()
            .insert(String::from(trial), artifact);
    }

    /// `trial`'s artifact.
    #[must_use]
    pub fn get(&self, trial: &str) -> Option<TrialArtifact> {
        self.entries.borrow().get(trial).cloned()
    }
}

impl Default for ArtifactStore {
    fn default() -> Self {
        Self::new()
    }
}
