//! The evaluation task set: scenario campaigns with hidden candidate outcomes, authored in typed
//! Nickel and revalidated here from the checked generated projection.

/// Schema of the generated task set.
pub const TASK_SET_SCHEMA: &str = "animus.campaign-manager-eval-tasks.v1";
/// Largest signal a scenario candidate reveals when executed.
pub const MAX_SIGNAL: u32 = 9;
/// Most turns one task may run.
pub const MAX_TURNS: u32 = 24;
/// Most candidates plus offers one task may hold.
pub const MAX_CANDIDATES: usize = 32;
/// Characters a base label may use, leaving room for the host's signal annotation.
const LABEL_ROOM: usize = 80;
/// The checked generated task set, exported from `config/campaign-manager-eval-tasks.ncl`.
const EMBEDDED: &str =
    include_str!("../../../../config/generated/campaign-manager-eval-tasks.json");

/// Pinned Harbor cohort the task set claims compatibility with.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HarborPins {
    /// Harbor release.
    pub release: String,
    /// Harbor commit.
    pub revision: String,
    /// Harbor task schema.
    pub task_schema: String,
    /// ATIF schema.
    pub atif_schema: String,
}

/// Per-proposal limits of the manager's full mandate; the stop grant is Campaign's fixed limit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MandateLimits {
    /// Candidates one brief lists.
    pub brief_window: u64,
    /// Candidate records per batch.
    pub candidate_batch: u64,
    /// Guidance records per proposal.
    pub guidance: u64,
    /// Revisions per proposal.
    pub revision: u64,
    /// Retirements per proposal.
    pub retirement: u64,
    /// Prune targets per proposal.
    pub prune: u64,
}

/// One scenario candidate or offer.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskCandidate {
    /// Task-unique key.
    pub key: String,
    /// Label the model sees, `family/approach`.
    pub label: String,
    /// Signal its execution reveals.
    pub signal: u32,
    /// Whether its execution records the task's finding.
    pub is_target: bool,
}

/// One scenario campaign.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvalTask {
    /// Harbor task identity.
    pub identity: String,
    /// The manager's objective.
    pub objective: String,
    /// Selections the campaign's budget allows.
    pub selection_budget: u32,
    /// Manager turns the trial runs at most.
    pub turns: u32,
    /// Candidates admitted when the campaign opens.
    pub candidates: Vec<TaskCandidate>,
    /// Candidates the manager may add.
    pub offers: Vec<TaskCandidate>,
}

/// The evaluation task set.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskSet {
    /// Schema, [`TASK_SET_SCHEMA`].
    pub schema: String,
    /// Task-set profile identity.
    pub profile_id: String,
    /// Pinned Harbor cohort.
    pub harbor: HarborPins,
    /// Agent identity of every route.
    pub agent: String,
    /// Environment identity.
    pub environment: String,
    /// Verifier identity; differs from the environment.
    pub verifier: String,
    /// Rubric identity.
    pub rubric: String,
    /// The manager's mandate limits.
    pub mandate: MandateLimits,
    /// Rank dimensions the manager scores in.
    pub rank_dimensions: u32,
    /// The tasks, in plan order.
    pub tasks: Vec<EvalTask>,
    /// Explicit non-claims.
    pub non_claims: Vec<String>,
}

/// Why a task set is refused.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TaskError {
    /// The projection is not the task-set JSON shape.
    Malformed,
    /// The schema or the Harbor cohort drifted.
    Drift,
    /// An identity or key is unsafe.
    Identity,
    /// The environment and verifier identities are equal.
    SharedVerifier,
    /// A task identity or a candidate key repeats.
    Duplicate,
    /// A task's objective, label, signal, budget, turns, offers, or target is out of shape.
    Shape {
        /// The task.
        task: String,
    },
}

impl TaskSet {
    /// The task with `identity`.
    #[must_use]
    pub fn task(&self, identity: &str) -> Option<&EvalTask> {
        self.tasks.iter().find(|task| task.identity == identity)
    }

    /// The full-manager advisory terms of the mandate limits.
    ///
    /// # Errors
    ///
    /// Returns `InvalidConfig` when Campaign refuses the terms.
    pub fn terms(&self) -> campaign_core::CampaignResult<campaign_core::AdvisoryTerms> {
        let mandate = self.mandate;
        campaign_core::AdvisoryTerms {
            brief_window: mandate.brief_window,
            grants: vec![
                grant(
                    campaign_core::AdvisoryActionKind::CandidateBatch,
                    mandate.candidate_batch,
                ),
                grant(
                    campaign_core::AdvisoryActionKind::Guidance,
                    mandate.guidance,
                ),
                grant(
                    campaign_core::AdvisoryActionKind::Revision,
                    mandate.revision,
                ),
                grant(
                    campaign_core::AdvisoryActionKind::Retirement,
                    mandate.retirement,
                ),
                grant(campaign_core::AdvisoryActionKind::Prune, mandate.prune),
                grant(
                    campaign_core::AdvisoryActionKind::Stop,
                    campaign_core::STOP_GRANT_LIMIT,
                ),
            ],
        }
        .canonical()
    }
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

/// The checked task set embedded in the crate.
///
/// # Errors
///
/// Returns the [`TaskError`] of [`parse_task_set`].
pub fn embedded_task_set() -> Result<TaskSet, TaskError> {
    parse_task_set(EMBEDDED)
}

/// Parses and revalidates one generated task-set projection.
///
/// # Errors
///
/// Returns a [`TaskError`] for malformed JSON, schema or Harbor drift, unsafe identities, a shared
/// verifier, duplicates, or a task out of shape.
// r[impl agent_harness_runtime.campaign_manager_eval.tasks]
pub fn parse_task_set(text: &str) -> Result<TaskSet, TaskError> {
    let set: TaskSet = serde_json::from_str(text).map_err(|_| TaskError::Malformed)?;
    check_header(&set)?;
    let mut identities = std::collections::BTreeSet::new();
    for task in &set.tasks {
        if !identities.insert(task.identity.as_str()) {
            return Err(TaskError::Duplicate);
        }
        check_task(task)?;
    }
    debug_assert_eq!(identities.len(), set.tasks.len());
    debug_assert!(!set.tasks.is_empty());
    Ok(set)
}

/// Checks the schema, Harbor pins, identities, and verifier separation.
fn check_header(set: &TaskSet) -> Result<(), TaskError> {
    let pins = HarborPins {
        release: String::from(animus_adapters::harbor::HARBOR_RELEASE),
        revision: String::from(animus_adapters::harbor::HARBOR_REVISION),
        task_schema: String::from(animus_adapters::harbor::HARBOR_TASK_SCHEMA),
        atif_schema: String::from(animus_adapters::harbor::ATIF_SCHEMA),
    };
    if set.schema != TASK_SET_SCHEMA || set.harbor != pins {
        return Err(TaskError::Drift);
    }
    let identities = [
        &set.profile_id,
        &set.agent,
        &set.environment,
        &set.verifier,
        &set.rubric,
    ];
    if !identities
        .iter()
        .all(|identity| animus_eval_core::is_safe_identity(identity))
    {
        return Err(TaskError::Identity);
    }
    if set.environment == set.verifier {
        return Err(TaskError::SharedVerifier);
    }
    if set.tasks.is_empty() || set.rank_dimensions == 0 {
        return Err(TaskError::Malformed);
    }
    debug_assert_ne!(set.environment, set.verifier);
    debug_assert_eq!(set.schema, TASK_SET_SCHEMA);
    Ok(())
}

/// Checks one task's identities, keys, text, signals, budget, turns, offers, and single target.
fn check_task(task: &EvalTask) -> Result<(), TaskError> {
    let shape = || TaskError::Shape {
        task: task.identity.clone(),
    };
    if !animus_eval_core::is_safe_identity(&task.identity) {
        return Err(TaskError::Identity);
    }
    let entries = || task.candidates.iter().chain(&task.offers);
    let mut keys = std::collections::BTreeSet::new();
    for entry in entries() {
        if !animus_eval_core::is_safe_identity(&entry.key) {
            return Err(TaskError::Identity);
        }
        if !keys.insert(entry.key.as_str()) {
            return Err(TaskError::Duplicate);
        }
        if !is_plain_text(&entry.label, LABEL_ROOM) || entry.signal > MAX_SIGNAL {
            return Err(shape());
        }
    }
    let is_bounded = task.turns <= MAX_TURNS && keys.len() <= MAX_CANDIDATES;
    let has_room = task.offers.len() <= animus_campaign_manager::MAX_OFFERS;
    let is_runnable = task.selection_budget > 0 && task.turns > 0 && !task.candidates.is_empty();
    let has_objective = is_plain_text(
        &task.objective,
        animus_campaign_manager::MAX_OBJECTIVE_CHARS,
    );
    let targets = entries().filter(|entry| entry.is_target).count();
    if !(is_bounded && has_room && is_runnable && has_objective) || targets != 1 {
        return Err(shape());
    }
    debug_assert_eq!(targets, 1);
    debug_assert!(keys.len() <= MAX_CANDIDATES);
    Ok(())
}

/// Whether `text` is non-empty, at most `room` characters, and free of control characters.
fn is_plain_text(text: &str, room: usize) -> bool {
    !text.is_empty() && text.chars().count() <= room && !text.chars().any(char::is_control)
}
