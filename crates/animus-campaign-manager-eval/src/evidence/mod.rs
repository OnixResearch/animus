//! Task-family qualification attachments projected from one finished job.
//!
//! The producer reads three inputs: a [`crate::JobReport`] that [`crate::run_job`] already
//! returned, the task set the job ran, and a host binding that names the evaluated route's
//! Lattice subject keys and freshness. It replans the same task-major trial matrix to attribute
//! trials to the route. It counts independently accepted completions from the separate
//! verifier's reduced `found` metric, and it keeps failed, cancelled and incomplete trials in the
//! denominator. It runs no trial, model or verifier and adds no evaluation engine.
// r[impl agent_harness_runtime.qualification_attachment.producer]

mod counts;
mod subject;

/// Producer token every attachment of this lane carries.
pub const QUALIFICATION_PRODUCER: &str = "animus-campaign-manager-eval";
/// Domain of the source-run identity.
const SOURCE_DOMAIN: &str = "animus.campaign-manager-eval.qualification-source.v1";
/// Trials per task and route in one job.
const REPETITIONS: u64 = 1;
/// Reference kind of an ATIF trajectory projection.
const TRAJECTORY_REFERENCE: &str = "atif-trajectory";
/// Prefix the lane puts before a trial identity to name its trajectory.
const TRAJECTORY_PREFIX: &str = "atif:";
/// Spelling prefix of native trial identities.
const NATIVE_DIGEST_PREFIX: &str = "b3:";
/// Spelling prefix of attachment identities.
const ATTACHMENT_DIGEST_PREFIX: &str = "blake3:";
/// Parts of the source identity before the trajectory identities.
const SOURCE_HEADER_PARTS: usize = 4;
/// Modes this lane never measures, sorted.
const MISSING_MODES: [&str; 7] = [
    "context-output-sampling-controls",
    "cost-accounting",
    "live-model-route",
    "prompt-identity",
    "review-burden",
    "time-to-acceptance",
    "tool-implementation-identity",
];

/// The host's binding of one evaluated route to its subject keys and freshness.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualificationBinding<'a> {
    /// Model-route identity of the evaluated route, as the job named it.
    pub route: &'a str,
    /// Lattice definition identity, `blake3:<hex>`.
    pub lattice_definition_identity: &'a str,
    /// Lattice profile revision, bare 64 lowercase hex.
    pub lattice_profile_revision: &'a str,
    /// Task family the task set evaluates.
    pub task_family: &'a str,
    /// Model version facts of the route.
    pub model: animus_runtime::qualification::ModelFacts,
    /// Fixture or live evidence.
    pub label: animus_runtime::qualification::EvidenceLabel,
    /// Producer revision token.
    pub producer_revision: &'a str,
    /// Host-supplied observation time, milliseconds since the epoch.
    pub observed_at_ms: u64,
    /// First time the evidence is no longer current, milliseconds since the epoch.
    pub valid_until_ms: u64,
}

/// Why no attachment was produced.
#[derive(Clone, Debug, PartialEq)]
pub enum QualificationError {
    /// The report holds no summary for the bound route.
    UnknownRoute,
    /// The reduced `found` mean is not an exact count.
    InexactAcceptedCount,
    /// A count exceeded its representation.
    Bound,
    /// The trial matrix could not be replanned.
    Plan(crate::JobError),
    /// The projected document failed attachment validation.
    Attachment(animus_runtime::qualification::Rejection),
}

/// Projects one `task_family_evaluation` attachment for `binding.route` from a finished job.
///
/// # Errors
///
/// Returns [`QualificationError`] when the route is absent, counts are inexact or unbounded, the
/// matrix cannot be replanned, or the document fails attachment validation.
pub fn task_family_attachment(
    set: &crate::TaskSet,
    report: &crate::JobReport,
    binding: &QualificationBinding<'_>,
) -> Result<animus_runtime::qualification::Attachment, QualificationError> {
    let summary = report
        .routes
        .iter()
        .find(|summary| summary.route == binding.route)
        .ok_or(QualificationError::UnknownRoute)?;
    let identities: Vec<String> = report
        .routes
        .iter()
        .map(|route| route.route.clone())
        .collect();
    let plans = crate::matrix::plan(set, &identities).map_err(QualificationError::Plan)?;
    let trials: Vec<&animus_runtime::harbor_eval::TrialPlan> = plans
        .iter()
        .filter(|plan| plan.model == binding.route)
        .collect();
    let outcomes = counts::outcomes(report, summary, binding.route, trials.len())?;
    let subject = subject::project(set, binding);
    debug_assert_eq!(
        subject.components.len(),
        animus_runtime::qualification::ComponentKind::ALL.len()
    );
    debug_assert!(trials.len() <= plans.len());
    let document = animus_runtime::qualification::AttachmentDocument {
        evidence: source(&subject, &report.receipt)?,
        subject,
        outcomes: Some(outcomes),
        measures: Some(counts::measures(set, &outcomes)?),
        scope: scope(set, report, &trials),
        provenance: provenance(binding),
        references: references(report, &trials),
    };
    animus_runtime::qualification::encode_attachment(&document)
        .map_err(QualificationError::Attachment)
}

/// Protocol, verifier and corpus identities bound to the subject, and the job's source identity.
fn source(
    subject: &animus_runtime::qualification::QualificationSubject,
    receipt: &animus_runtime::harbor_eval::EvaluationReceipt,
) -> Result<animus_runtime::qualification::EvidenceSource, QualificationError> {
    let bound = |kind| {
        subject
            .component(kind)
            .and_then(|component| component.identity.clone())
            .ok_or(QualificationError::Bound)
    };
    let planned = receipt.planned.to_string();
    let completed = receipt.completed.to_string();
    let parts: Vec<&[u8]> = [SOURCE_DOMAIN, &receipt.job_identity, &planned, &completed]
        .into_iter()
        .chain(receipt.trajectories.iter().map(String::as_str))
        .map(str::as_bytes)
        .collect();
    debug_assert_eq!(
        parts.len(),
        SOURCE_HEADER_PARTS.saturating_add(receipt.trajectories.len())
    );
    debug_assert!(receipt.completed <= receipt.planned);
    Ok(animus_runtime::qualification::EvidenceSource {
        class: animus_runtime::qualification::EvidenceClass::TaskFamilyEvaluation,
        protocol_identity: bound(animus_runtime::qualification::ComponentKind::TaskProtocol)?,
        verifier_identity: bound(animus_runtime::qualification::ComponentKind::Verifier)?,
        corpus_identity: Some(bound(animus_runtime::qualification::ComponentKind::Corpus)?),
        source_identity: animus_runtime::qualification::framed_identity(&parts),
    })
}

fn scope(
    set: &crate::TaskSet,
    report: &crate::JobReport,
    trials: &[&animus_runtime::harbor_eval::TrialPlan],
) -> animus_runtime::qualification::Scope {
    let mut partitions: Vec<String> = trials.iter().map(|plan| plan.task.clone()).collect();
    partitions.sort_unstable();
    partitions.dedup();
    let mut non_claims: Vec<String> = set
        .non_claims
        .iter()
        .cloned()
        .chain(
            report
                .receipt
                .non_claims
                .iter()
                .map(|claim| String::from(*claim)),
        )
        .collect();
    non_claims.sort_unstable();
    non_claims.dedup();
    debug_assert!(partitions.len() <= trials.len());
    debug_assert!(partitions.is_sorted());
    animus_runtime::qualification::Scope {
        partitions,
        limitations: non_claims,
        missing_modes: MISSING_MODES
            .iter()
            .map(|mode| String::from(*mode))
            .collect(),
    }
}

fn provenance(binding: &QualificationBinding<'_>) -> animus_runtime::qualification::Provenance {
    animus_runtime::qualification::Provenance {
        label: binding.label,
        producer: String::from(QUALIFICATION_PRODUCER),
        producer_revision: String::from(binding.producer_revision),
        observed_at_ms: binding.observed_at_ms,
        valid_until_ms: binding.valid_until_ms,
    }
}

/// The route's ATIF trajectory identities, respelled `blake3:` over the same digest.
fn references(
    report: &crate::JobReport,
    trials: &[&animus_runtime::harbor_eval::TrialPlan],
) -> Vec<animus_runtime::qualification::Reference> {
    let mut references: Vec<animus_runtime::qualification::Reference> = report
        .receipt
        .trajectories
        .iter()
        .filter_map(|trajectory| trajectory.strip_prefix(TRAJECTORY_PREFIX))
        .filter(|trial| trials.iter().any(|plan| plan.identity == *trial))
        .filter_map(|trial| trial.strip_prefix(NATIVE_DIGEST_PREFIX))
        .map(|digest| animus_runtime::qualification::Reference {
            kind: String::from(TRAJECTORY_REFERENCE),
            identity: format!("{ATTACHMENT_DIGEST_PREFIX}{digest}"),
        })
        .collect();
    references.sort_unstable();
    debug_assert!(references.len() <= trials.len());
    debug_assert!(references.is_sorted());
    references
}
