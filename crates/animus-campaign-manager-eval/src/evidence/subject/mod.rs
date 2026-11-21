//! The complete evaluated subject, projected from the task set, the route and the host binding.
//!
//! Observed components come from the exact inputs every trial request carried. Declared
//! components come from the lane's fixed configuration. Components the lane cannot identify (the
//! prompt, tool implementation, executable, and model controls of a scripted route) stay
//! unavailable rather than receiving a placeholder identity.

/// Order in which the job plans trials.
const SELECTION_POLICY: &str = "task-major-route-minor";
/// Isolation every trial runs under.
const ISOLATION: &str = "in-memory-choregraph-history";
/// Network posture every trial plans with.
const NETWORK: &str = "none";
/// Role label of an initial scenario candidate in the corpus identity.
const CANDIDATE_ROLE: &str = "candidate";
/// Role label of an offered scenario candidate in the corpus identity.
const OFFER_ROLE: &str = "offer";
/// Marker of the task's target candidate in the corpus identity.
const TARGET_MARK: &str = "target";
/// Marker of any other candidate in the corpus identity.
const OTHER_MARK: &str = "other";

/// The subject of `binding.route` over `set`.
pub(super) fn project(
    set: &crate::TaskSet,
    binding: &super::QualificationBinding<'_>,
) -> animus_runtime::qualification::QualificationSubject {
    let components: Vec<animus_runtime::qualification::Component> =
        animus_runtime::qualification::ComponentKind::ALL
            .into_iter()
            .map(|kind| component(set, binding.route, kind))
            .collect();
    debug_assert_eq!(
        components.len(),
        animus_runtime::qualification::ComponentKind::ALL.len()
    );
    animus_runtime::qualification::QualificationSubject {
        lattice_definition_identity: String::from(binding.lattice_definition_identity),
        lattice_profile_revision: String::from(binding.lattice_profile_revision),
        task_family: String::from(binding.task_family),
        components,
        model: binding.model.clone(),
    }
}

/// One component; `None` parts mean the lane cannot identify it.
fn component(
    set: &crate::TaskSet,
    route: &str,
    kind: animus_runtime::qualification::ComponentKind,
) -> animus_runtime::qualification::Component {
    let Some((basis, parts)) = parts(set, route, kind) else {
        return animus_runtime::qualification::Component {
            kind,
            basis: animus_runtime::qualification::ComponentBasis::Unavailable,
            identity: None,
        };
    };
    let bytes: Vec<&[u8]> = parts.iter().map(String::as_bytes).collect();
    let framed = animus_runtime::qualification::framed_identity(&bytes);
    debug_assert!(!parts.is_empty());
    debug_assert_eq!(bytes.len(), parts.len());
    animus_runtime::qualification::Component {
        kind,
        basis,
        identity: Some(animus_runtime::qualification::component_identity(
            kind,
            framed.as_bytes(),
        )),
    }
}

/// The basis and identity parts of `kind`, or `None` when the lane cannot identify it.
fn parts(
    set: &crate::TaskSet,
    route: &str,
    kind: animus_runtime::qualification::ComponentKind,
) -> Option<(animus_runtime::qualification::ComponentBasis, Vec<String>)> {
    let observed = animus_runtime::qualification::ComponentBasis::Observed;
    let declared = animus_runtime::qualification::ComponentBasis::Declared;
    let selected = match kind {
        animus_runtime::qualification::ComponentKind::AgentDefinition => {
            Some((observed, vec![set.agent.clone()]))
        }
        animus_runtime::qualification::ComponentKind::Budget => Some((observed, budget(set))),
        animus_runtime::qualification::ComponentKind::Corpus => Some((observed, corpus(set))),
        animus_runtime::qualification::ComponentKind::Environment => {
            Some((observed, vec![set.environment.clone()]))
        }
        animus_runtime::qualification::ComponentKind::Harness => Some((observed, harness(set))),
        animus_runtime::qualification::ComponentKind::ModelRoute => {
            Some((observed, vec![String::from(route)]))
        }
        animus_runtime::qualification::ComponentKind::TaskProtocol => {
            Some((observed, protocol(set)))
        }
        animus_runtime::qualification::ComponentKind::Verifier => Some((observed, verifier(set))),
        animus_runtime::qualification::ComponentKind::EvaluationConfig => {
            Some((declared, evaluation()))
        }
        animus_runtime::qualification::ComponentKind::Permissions => Some((declared, mandate(set))),
        animus_runtime::qualification::ComponentKind::Isolation => {
            Some((declared, vec![String::from(ISOLATION)]))
        }
        animus_runtime::qualification::ComponentKind::Network => {
            Some((declared, vec![String::from(NETWORK)]))
        }
        animus_runtime::qualification::ComponentKind::SelectionPolicy => {
            Some((declared, vec![String::from(SELECTION_POLICY)]))
        }
        animus_runtime::qualification::ComponentKind::Topology => {
            Some((declared, vec![crate::matrix::CONCURRENCY.to_string()]))
        }
        animus_runtime::qualification::ComponentKind::ContextControls
        | animus_runtime::qualification::ComponentKind::Executable
        | animus_runtime::qualification::ComponentKind::MemoryPolicy
        | animus_runtime::qualification::ComponentKind::OutputControls
        | animus_runtime::qualification::ComponentKind::Prompt
        | animus_runtime::qualification::ComponentKind::SamplingControls
        | animus_runtime::qualification::ComponentKind::SemanticRole
        | animus_runtime::qualification::ComponentKind::Skills
        | animus_runtime::qualification::ComponentKind::Strategy
        | animus_runtime::qualification::ComponentKind::ToolImplementation
        | animus_runtime::qualification::ComponentKind::ToolSchemas => None,
    };
    debug_assert!(
        selected
            .as_ref()
            .is_none_or(|(_, identity_parts)| !identity_parts.is_empty())
    );
    debug_assert!(!route.is_empty());
    selected
}

fn harness(set: &crate::TaskSet) -> Vec<String> {
    let pins = &set.harbor;
    vec![
        pins.release.clone(),
        pins.revision.clone(),
        pins.task_schema.clone(),
        pins.atif_schema.clone(),
    ]
}

fn protocol(set: &crate::TaskSet) -> Vec<String> {
    vec![
        set.schema.clone(),
        set.profile_id.clone(),
        set.rubric.clone(),
        set.rank_dimensions.to_string(),
    ]
}

fn verifier(set: &crate::TaskSet) -> Vec<String> {
    vec![
        set.verifier.clone(),
        set.rubric.clone(),
        String::from(crate::METRIC_FOUND),
    ]
}

fn evaluation() -> Vec<String> {
    crate::ARTIFACT_NAMES
        .iter()
        .map(|name| String::from(*name))
        .chain([
            String::from(crate::METRIC_FOUND),
            super::REPETITIONS.to_string(),
        ])
        .collect()
}

fn mandate(set: &crate::TaskSet) -> Vec<String> {
    let terms = &set.mandate;
    [
        terms.brief_window,
        terms.candidate_batch,
        terms.guidance,
        terms.revision,
        terms.retirement,
        terms.prune,
    ]
    .iter()
    .map(u64::to_string)
    .collect()
}

fn budget(set: &crate::TaskSet) -> Vec<String> {
    set.tasks
        .iter()
        .flat_map(|task| {
            [
                task.identity.clone(),
                task.selection_budget.to_string(),
                task.turns.to_string(),
            ]
        })
        .collect()
}

/// The exact tasks the job ran: objectives, candidates, offers, hidden signals and targets.
fn corpus(set: &crate::TaskSet) -> Vec<String> {
    set.tasks
        .iter()
        .flat_map(|task| {
            let entries = task
                .candidates
                .iter()
                .map(|entry| (CANDIDATE_ROLE, entry))
                .chain(task.offers.iter().map(|entry| (OFFER_ROLE, entry)))
                .flat_map(|(role, entry)| {
                    let mark = if entry.is_target {
                        TARGET_MARK
                    } else {
                        OTHER_MARK
                    };
                    [
                        String::from(role),
                        entry.key.clone(),
                        entry.label.clone(),
                        entry.signal.to_string(),
                        String::from(mark),
                    ]
                });
            [task.identity.clone(), task.objective.clone()]
                .into_iter()
                .chain(entries)
        })
        .collect()
}
