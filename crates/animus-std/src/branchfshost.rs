//! Optional abort-only `BranchFS` speculative workspace host.
//!
//! The pure core in this module plans disposable sibling candidates. Process,
//! FUSE, filesystem, and Jujutsu effects are injected through [`Host`].

use blake3::Hasher;
use std::collections::BTreeSet;

/// Reviewed upstream `BranchFS` revision.
pub const BRANCHFS_REVISION: &str = "a4b6592d31aacb4d2f94d0c80f43d47d38063fa9";
/// Reviewed BranchFS/Jujutsu compatibility cohort.
pub const COHORT: &str = "branchfs-a4b6592-jj-abort-only-v1";
const PLAN_DOMAIN: &str = "animus.branchfs.plan.v1";
const RECEIPT_DOMAIN: &str = "animus.branchfs.receipt.v1";
const MAX_CANDIDATES: usize = 16;

/// Pure host facts gathered by the imperative preflight shell.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostFacts<'a> {
    /// Observed `BranchFS` revision identity.
    pub branchfs_revision: &'a str,
    /// FUSE device and mount helper are usable.
    pub fuse_ready: bool,
    /// Bounded mount/create/read/write/abort/unmount smoke passed.
    pub behavior_smoke_passed: bool,
    /// Jujutsu executable and source workspace are usable.
    pub jj_ready: bool,
    /// Durable source identity observed by Jujutsu.
    pub source_identity: &'a str,
}

/// Typed fail-closed preflight result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Preflight {
    /// Exact reviewed cohort is usable.
    Ready,
    /// Adapter is not selected.
    Disabled,
    /// Executable identity differs from the reviewed revision.
    CohortMismatch,
    /// FUSE is unavailable.
    FuseUnavailable,
    /// Functional behavior smoke failed.
    BehaviorUnavailable,
    /// Jujutsu or its source workspace is unavailable.
    JujutsuUnavailable,
    /// Source identity differs from candidate admission.
    SourceDrift,
}

/// Evaluates declared host facts without performing effects.
#[must_use]
pub fn evaluate_preflight(enabled: bool, expected_source: &str, facts: HostFacts<'_>) -> Preflight {
    if !enabled {
        return Preflight::Disabled;
    }
    if facts.branchfs_revision != BRANCHFS_REVISION {
        return Preflight::CohortMismatch;
    }
    if !facts.fuse_ready {
        return Preflight::FuseUnavailable;
    }
    if !facts.behavior_smoke_passed {
        return Preflight::BehaviorUnavailable;
    }
    if !facts.jj_ready {
        return Preflight::JujutsuUnavailable;
    }
    if facts.source_identity != expected_source {
        return Preflight::SourceDrift;
    }
    Preflight::Ready
}

/// Admitted abort-only operation. There is deliberately no commit variant.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Operation {
    /// Mount one source view.
    Mount,
    /// Create a root-level sibling candidate.
    Create {
        /// Safe candidate label.
        candidate: String,
    },
    /// Hand the virtual candidate path to an injected executor.
    Execute {
        /// Safe candidate label.
        candidate: String,
    },
    /// Capture a bounded metadata manifest.
    Manifest {
        /// Safe candidate label.
        candidate: String,
    },
    /// Materialize a selected view into a fresh Jujutsu workspace.
    Materialize {
        /// Safe candidate label.
        candidate: String,
    },
    /// Abort one disposable candidate.
    Abort {
        /// Safe candidate label.
        candidate: String,
    },
    /// Cleanly unmount after every candidate has a known state.
    Unmount,
}

/// Pure deterministic sibling-candidate plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Plan {
    source_identity: String,
    candidates: Vec<String>,
    operations: Vec<Operation>,
    identity: String,
}

/// Plan rejection before any mount or command.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PlanError {
    /// No candidate was supplied.
    EmptyCandidates,
    /// Candidate count exceeds the reviewed bound.
    TooManyCandidates,
    /// A candidate label is empty, duplicated, nested, absolute, or reserved.
    InvalidCandidate,
    /// Durable source identity is absent.
    MissingSourceIdentity,
    /// Direct commit or nested candidate behavior was requested.
    UnsupportedOperation,
}

impl Plan {
    /// Builds an abort-only plan. Candidate labels are sorted so caller order
    /// cannot affect the operation identity.
    ///
    /// # Errors
    ///
    /// Returns [`PlanError`] when source or candidates violate the bounded,
    /// root-level, abort-only contract.
    pub fn new(
        source_identity: &str,
        candidates: &[&str],
        requests_commit: bool,
        requests_nested: bool,
    ) -> Result<Self, PlanError> {
        if source_identity.is_empty() {
            return Err(PlanError::MissingSourceIdentity);
        }
        if requests_commit || requests_nested {
            return Err(PlanError::UnsupportedOperation);
        }
        if candidates.is_empty() {
            return Err(PlanError::EmptyCandidates);
        }
        if candidates.len() > MAX_CANDIDATES {
            return Err(PlanError::TooManyCandidates);
        }
        let mut labels = candidates
            .iter()
            .map(|label| (*label).to_owned())
            .collect::<Vec<_>>();
        labels.sort();
        let unique = labels.iter().collect::<BTreeSet<_>>();
        if unique.len() != labels.len() || labels.iter().any(|label| !valid_label(label)) {
            return Err(PlanError::InvalidCandidate);
        }
        let mut operations = vec![Operation::Mount];
        for candidate in &labels {
            operations.push(Operation::Create {
                candidate: candidate.clone(),
            });
            operations.push(Operation::Execute {
                candidate: candidate.clone(),
            });
            operations.push(Operation::Manifest {
                candidate: candidate.clone(),
            });
        }
        let identity = hash_parts(PLAN_DOMAIN, source_identity, &labels);
        Ok(Self {
            source_identity: source_identity.to_owned(),
            candidates: labels,
            operations,
            identity,
        })
    }

    /// Returns the exact durable source identity bound at admission.
    #[must_use]
    pub fn source_identity(&self) -> &str {
        &self.source_identity
    }
    /// Returns sorted admitted candidate labels.
    #[must_use]
    pub fn candidates(&self) -> &[String] {
        &self.candidates
    }
    /// Returns initial operations. Promotion and cleanup are selected later.
    #[must_use]
    pub fn operations(&self) -> &[Operation] {
        &self.operations
    }
    /// Returns the domain-separated deterministic plan identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }
}

fn valid_label(label: &str) -> bool {
    !label.is_empty()
        && label.len() <= 64
        && !label.starts_with('@')
        && !label.starts_with('.')
        && !label.contains('/')
        && !label.contains('\\')
        && !label.contains('\0')
        && label
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_')
}

/// Pure winner-promotion validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Promotion {
    /// Materialization is accepted; the winner may now be aborted.
    Accepted,
    /// Selection did not name an admitted sibling.
    UnknownWinner,
    /// Durable source changed after admission.
    SourceDrift,
    /// Fresh workspace content differs from the winner view.
    ManifestMismatch,
    /// Destination is not a fresh Jujutsu workspace.
    DestinationNotFresh,
}

/// Bounded promotion evidence observed by the imperative shell.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PromotionEvidence<'a> {
    /// Selected candidate identity.
    pub selected: &'a str,
    /// Current durable source identity.
    pub current_source: &'a str,
    /// Selected view's bounded manifest identity.
    pub winner_manifest: &'a str,
    /// Fresh workspace's bounded manifest identity.
    pub copied_manifest: &'a str,
    /// Whether the destination was freshly created by Jujutsu.
    pub destination_is_fresh: bool,
}

/// Validates promotion evidence before disposable winner cleanup.
#[must_use]
pub fn validate_promotion(plan: &Plan, evidence: PromotionEvidence<'_>) -> Promotion {
    if !plan
        .candidates
        .iter()
        .any(|candidate| candidate == evidence.selected)
    {
        return Promotion::UnknownWinner;
    }
    if evidence.current_source != plan.source_identity {
        return Promotion::SourceDrift;
    }
    if !evidence.destination_is_fresh {
        return Promotion::DestinationNotFresh;
    }
    if evidence.winner_manifest != evidence.copied_manifest {
        return Promotion::ManifestMismatch;
    }
    Promotion::Accepted
}

/// Safe metadata-only disposable candidate receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateReceipt {
    /// Receipt identity.
    pub identity: String,
    /// Plan identity.
    pub plan_identity: String,
    /// Candidate label.
    pub candidate: String,
    /// Manifest identity, never the raw manifest.
    pub manifest_identity: String,
    /// Cleanup outcome label.
    pub cleanup: &'static str,
    /// Compatibility cohort.
    pub cohort: &'static str,
}

/// Projects a bounded candidate receipt.
#[must_use]
pub fn candidate_receipt(
    plan: &Plan,
    candidate: &str,
    manifest_identity: &str,
    cleanup: &'static str,
) -> CandidateReceipt {
    let identity = hash_parts(
        RECEIPT_DOMAIN,
        plan.identity(),
        &[
            candidate.to_owned(),
            manifest_identity.to_owned(),
            cleanup.to_owned(),
        ],
    );
    CandidateReceipt {
        identity,
        plan_identity: plan.identity.clone(),
        candidate: candidate.to_owned(),
        manifest_identity: manifest_identity.to_owned(),
        cleanup,
        cohort: COHORT,
    }
}

/// Safe metadata-only durable promotion receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PromotionReceipt {
    /// Domain-separated receipt identity.
    pub identity: String,
    /// Selected candidate label.
    pub selected_candidate: String,
    /// Fresh Jujutsu workspace identity.
    pub workspace_identity: String,
    /// Fresh Jujutsu change identity.
    pub change_identity: String,
    /// Copied bounded manifest identity.
    pub manifest_identity: String,
    /// Compatibility cohort.
    pub cohort: &'static str,
}

/// Projects durable handoff evidence separately from disposable candidates.
#[must_use]
pub fn promotion_receipt(selected: &str, materialization: &Materialization) -> PromotionReceipt {
    let parts = [
        materialization.workspace_identity.clone(),
        materialization.change_identity.clone(),
        materialization.manifest_identity.clone(),
    ];
    PromotionReceipt {
        identity: hash_parts(RECEIPT_DOMAIN, selected, &parts),
        selected_candidate: selected.to_owned(),
        workspace_identity: materialization.workspace_identity.clone(),
        change_identity: materialization.change_identity.clone(),
        manifest_identity: materialization.manifest_identity.clone(),
        cohort: COHORT,
    }
}

/// Rendered command with fixed executable and bounded arguments.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandSpec {
    /// Reviewed executable name.
    pub executable: &'static str,
    /// Arguments; host paths are supplied only by the imperative shell.
    pub arguments: Vec<String>,
}

/// Renders only reviewed `BranchFS` operations. Since [`Operation`] has no
/// commit variant, admitted rendering cannot invoke `branchfs commit`.
#[must_use]
pub fn render_branchfs(
    operation: &Operation,
    source: &str,
    mount: &str,
    storage: &str,
) -> Option<CommandSpec> {
    let arguments = match operation {
        Operation::Mount => vec![
            "mount".into(),
            "--base".into(),
            source.into(),
            "--storage".into(),
            storage.into(),
            mount.into(),
        ],
        Operation::Create { candidate } => vec![
            "create".into(),
            "--storage".into(),
            storage.into(),
            candidate.clone(),
            mount.into(),
            "--parent".into(),
            "main".into(),
        ],
        Operation::Unmount => vec![
            "unmount".into(),
            "--storage".into(),
            storage.into(),
            mount.into(),
        ],
        Operation::Execute { .. }
        | Operation::Manifest { .. }
        | Operation::Materialize { .. }
        | Operation::Abort { .. } => return None,
    };
    Some(CommandSpec {
        executable: "branchfs",
        arguments,
    })
}

/// Result of capability-scoped materialization into a fresh Jujutsu workspace.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Materialization {
    /// Fresh workspace manifest identity.
    pub manifest_identity: String,
    /// Safe Jujutsu workspace identity.
    pub workspace_identity: String,
    /// Safe Jujutsu change identity.
    pub change_identity: String,
    /// Destination was freshly created and was not the operator workspace.
    pub destination_is_fresh: bool,
}

/// Injected imperative host boundary.
pub trait Host {
    /// Executes one already-admitted `BranchFS` command.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] without exposing raw command output.
    fn execute(&mut self, command: &CommandSpec) -> Result<(), HostError>;

    /// Runs one candidate through the existing admitted agent executor.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when candidate execution cannot complete.
    fn execute_candidate(&mut self, candidate: &str) -> Result<(), HostError>;

    /// Computes a bounded manifest identity for one candidate view.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] for control namespaces, escaping links, special
    /// files, budget exhaustion, or filesystem failure.
    fn manifest_identity(&mut self, candidate: &str) -> Result<String, HostError>;

    /// Reads the current durable source identity through Jujutsu.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] if Jujutsu cannot inspect the bound source.
    fn current_source_identity(&mut self) -> Result<String, HostError>;

    /// Materializes one winner into a fresh Jujutsu workspace through bounded
    /// capability-scoped copy operations.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] for unsafe entries, copy failure, or Jujutsu
    /// workspace failure. A failure must not target the operator workspace.
    fn materialize_winner(
        &mut self,
        candidate: &str,
        source_identity: &str,
    ) -> Result<Materialization, HostError>;

    /// Aborts one virtual candidate using the typed control-file boundary.
    ///
    /// # Errors
    ///
    /// Returns [`HostError::CleanupRequired`] if the candidate remains.
    fn abort_candidate(&mut self, candidate: &str) -> Result<(), HostError>;
}

/// Host execution failure without raw command output.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostError {
    /// Executable was unavailable.
    Unavailable,
    /// Command or bounded filesystem operation failed.
    Failed,
    /// Unsafe control path, link, special file, or budget overflow was found.
    UnsafeMaterialization,
    /// Cleanup failed and operator action is required.
    CleanupRequired,
}

/// Bounded shell request around a previously admitted pure plan.
pub struct RunRequest<'a> {
    /// Pure admitted plan.
    pub plan: &'a Plan,
    /// Host-owned source path.
    pub source_path: &'a str,
    /// Host-owned mount path.
    pub mount_path: &'a str,
    /// Host-owned private backend storage path.
    pub storage_path: &'a str,
    /// Candidate selected by an external portfolio policy.
    pub selected_candidate: &'a str,
}

/// Safe successful durable handoff.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunOutcome {
    /// Selected candidate label.
    pub selected_candidate: String,
    /// Fresh Jujutsu workspace identity.
    pub workspace_identity: String,
    /// Fresh Jujutsu change identity.
    pub change_identity: String,
    /// Promotion manifest identity.
    pub manifest_identity: String,
    /// Candidate receipts after known cleanup.
    pub candidate_receipts: Vec<CandidateReceipt>,
    /// Separate durable promotion receipt.
    pub promotion_receipt: PromotionReceipt,
}

/// Shell lifecycle failure stage.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RunStage {
    /// Initial mount.
    Mount,
    /// Candidate creation.
    Create,
    /// Candidate agent execution.
    Execute,
    /// Bounded manifest capture.
    Manifest,
    /// Durable source identity recheck.
    SourceFreshness,
    /// Fresh Jujutsu materialization.
    Materialize,
    /// Candidate abort cleanup.
    Abort,
    /// Final unmount cleanup.
    Unmount,
}

/// Safe lifecycle failure with bounded recovery metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RunFailure {
    /// Failed stage.
    pub stage: RunStage,
    /// Candidate labels that may require explicit cleanup.
    pub retained_candidates: Vec<String>,
    /// Underlying typed host failure when applicable.
    pub host_error: Option<HostError>,
}

struct CandidateState {
    created: Vec<String>,
    manifests: Vec<(String, String)>,
}

fn execute_candidates<H: Host>(
    host: &mut H,
    request: &RunRequest<'_>,
) -> Result<CandidateState, RunFailure> {
    let mut state = CandidateState {
        created: Vec::new(),
        manifests: Vec::new(),
    };
    for candidate in request.plan.candidates() {
        let Some(create) = render_branchfs(
            &Operation::Create {
                candidate: candidate.clone(),
            },
            request.source_path,
            request.mount_path,
            request.storage_path,
        ) else {
            return Err(failure(RunStage::Create, &state.created, None));
        };
        host.execute(&create)
            .map_err(|error| failure(RunStage::Create, &state.created, Some(error)))?;
        state.created.push(candidate.clone());
        host.execute_candidate(candidate)
            .map_err(|error| failure(RunStage::Execute, &state.created, Some(error)))?;
        let manifest = host
            .manifest_identity(candidate)
            .map_err(|error| failure(RunStage::Manifest, &state.created, Some(error)))?;
        state.manifests.push((candidate.clone(), manifest));
    }
    Ok(state)
}

fn promote<H: Host>(
    host: &mut H,
    request: &RunRequest<'_>,
    state: &CandidateState,
) -> Result<Materialization, RunFailure> {
    let current_source = host
        .current_source_identity()
        .map_err(|error| failure(RunStage::SourceFreshness, &state.created, Some(error)))?;
    if current_source != request.plan.source_identity() {
        return Err(failure(RunStage::SourceFreshness, &state.created, None));
    }
    let Some(winner_manifest) = state
        .manifests
        .iter()
        .find(|(candidate, _)| candidate == request.selected_candidate)
        .map(|(_, manifest)| manifest.as_str())
    else {
        return Err(failure(RunStage::Materialize, &state.created, None));
    };
    let materialization = host
        .materialize_winner(request.selected_candidate, request.plan.source_identity())
        .map_err(|error| failure(RunStage::Materialize, &state.created, Some(error)))?;
    let promotion = validate_promotion(
        request.plan,
        PromotionEvidence {
            selected: request.selected_candidate,
            current_source: &current_source,
            winner_manifest,
            copied_manifest: &materialization.manifest_identity,
            destination_is_fresh: materialization.destination_is_fresh,
        },
    );
    if promotion != Promotion::Accepted {
        return Err(failure(RunStage::Materialize, &state.created, None));
    }
    Ok(materialization)
}

fn cleanup<H: Host>(
    host: &mut H,
    request: &RunRequest<'_>,
    state: &CandidateState,
) -> Result<Vec<CandidateReceipt>, RunFailure> {
    let mut receipts = Vec::new();
    for (candidate, manifest) in &state.manifests {
        host.abort_candidate(candidate)
            .map_err(|error| failure(RunStage::Abort, &state.created, Some(error)))?;
        receipts.push(candidate_receipt(
            request.plan,
            candidate,
            manifest,
            "aborted",
        ));
    }
    let Some(unmount) = render_branchfs(
        &Operation::Unmount,
        request.source_path,
        request.mount_path,
        request.storage_path,
    ) else {
        return Err(failure(RunStage::Unmount, &[], None));
    };
    host.execute(&unmount)
        .map_err(|error| failure(RunStage::Unmount, &[], Some(error)))?;
    Ok(receipts)
}

/// Runs the thin imperative shell around pure admission and validation.
///
/// # Errors
///
/// Returns [`RunFailure`] and retains bounded recovery labels if any effect or
/// validation fails. It never invokes `BranchFS` commit.
pub fn run<H: Host>(host: &mut H, request: &RunRequest<'_>) -> Result<RunOutcome, RunFailure> {
    if !request
        .plan
        .candidates
        .iter()
        .any(|candidate| candidate == request.selected_candidate)
    {
        return Err(failure(
            RunStage::Materialize,
            request.plan.candidates(),
            None,
        ));
    }
    let Some(mount) = render_branchfs(
        &Operation::Mount,
        request.source_path,
        request.mount_path,
        request.storage_path,
    ) else {
        return Err(failure(RunStage::Mount, &[], None));
    };
    host.execute(&mount)
        .map_err(|error| failure(RunStage::Mount, &[], Some(error)))?;
    let state = execute_candidates(host, request)?;
    let materialization = promote(host, request, &state)?;
    let receipts = cleanup(host, request, &state)?;
    let durable_receipt = promotion_receipt(request.selected_candidate, &materialization);
    Ok(RunOutcome {
        selected_candidate: request.selected_candidate.to_owned(),
        workspace_identity: materialization.workspace_identity,
        change_identity: materialization.change_identity,
        manifest_identity: materialization.manifest_identity,
        candidate_receipts: receipts,
        promotion_receipt: durable_receipt,
    })
}

fn failure(stage: RunStage, retained: &[String], host_error: Option<HostError>) -> RunFailure {
    RunFailure {
        stage,
        retained_candidates: retained.to_vec(),
        host_error,
    }
}

fn hash_parts(domain: &str, first: &str, rest: &[String]) -> String {
    let mut hasher = Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&[0]);
    hasher.update(first.as_bytes());
    for value in rest {
        hasher.update(&[0]);
        hasher.update(value.as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SOURCE: &str = "source:abc";

    #[test]
    fn sibling_plan_is_deterministic_and_abort_only() {
        let first = Plan::new(SOURCE, &["candidate-b", "candidate-a"], false, false).unwrap();
        let second = Plan::new(SOURCE, &["candidate-a", "candidate-b"], false, false).unwrap();
        assert_eq!(first.identity(), second.identity());
        assert_eq!(first.candidates(), &["candidate-a", "candidate-b"]);
        assert!(first.operations().iter().all(|operation| {
            !format!("{operation:?}")
                .to_ascii_lowercase()
                .contains("commit")
        }));
    }

    #[test]
    fn commit_nested_and_unsafe_labels_fail_before_commands() {
        assert_eq!(
            Plan::new(SOURCE, &["a"], true, false),
            Err(PlanError::UnsupportedOperation)
        );
        assert_eq!(
            Plan::new(SOURCE, &["a"], false, true),
            Err(PlanError::UnsupportedOperation)
        );
        assert_eq!(
            Plan::new(SOURCE, &["../escape"], false, false),
            Err(PlanError::InvalidCandidate)
        );
        assert_eq!(
            Plan::new(SOURCE, &["same", "same"], false, false),
            Err(PlanError::InvalidCandidate)
        );
    }

    #[test]
    fn preflight_requires_exact_functional_cohort() {
        let facts = HostFacts {
            branchfs_revision: BRANCHFS_REVISION,
            fuse_ready: true,
            behavior_smoke_passed: true,
            jj_ready: true,
            source_identity: SOURCE,
        };
        assert_eq!(evaluate_preflight(true, SOURCE, facts), Preflight::Ready);
        assert_eq!(
            evaluate_preflight(false, SOURCE, facts),
            Preflight::Disabled
        );
        assert_eq!(
            evaluate_preflight(true, "source:drift", facts),
            Preflight::SourceDrift
        );
        assert_eq!(
            evaluate_preflight(
                true,
                SOURCE,
                HostFacts {
                    fuse_ready: false,
                    ..facts
                }
            ),
            Preflight::FuseUnavailable
        );
    }

    #[test]
    fn promotion_binds_selection_source_and_manifest() {
        let plan = Plan::new(SOURCE, &["winner", "loser"], false, false).unwrap();
        let accepted = PromotionEvidence {
            selected: "winner",
            current_source: SOURCE,
            winner_manifest: "manifest:1",
            copied_manifest: "manifest:1",
            destination_is_fresh: true,
        };
        assert_eq!(validate_promotion(&plan, accepted), Promotion::Accepted);
        assert_eq!(
            validate_promotion(
                &plan,
                PromotionEvidence {
                    selected: "unknown",
                    ..accepted
                }
            ),
            Promotion::UnknownWinner
        );
        assert_eq!(
            validate_promotion(
                &plan,
                PromotionEvidence {
                    current_source: "source:drift",
                    ..accepted
                }
            ),
            Promotion::SourceDrift
        );
        assert_eq!(
            validate_promotion(
                &plan,
                PromotionEvidence {
                    copied_manifest: "manifest:2",
                    ..accepted
                }
            ),
            Promotion::ManifestMismatch
        );
    }

    #[test]
    fn renderer_has_no_commit_command() {
        let operations = [
            Operation::Mount,
            Operation::Create {
                candidate: "a".into(),
            },
            Operation::Unmount,
        ];
        for operation in operations {
            let command = render_branchfs(&operation, "/source", "/mount", "/storage").unwrap();
            assert_eq!(command.executable, "branchfs");
            assert!(
                !command
                    .arguments
                    .iter()
                    .any(|argument| argument == "commit")
            );
        }
        assert!(
            render_branchfs(
                &Operation::Abort {
                    candidate: "a".into()
                },
                "/source",
                "/mount",
                "/storage"
            )
            .is_none()
        );
    }

    #[test]
    fn shell_materializes_winner_then_aborts_every_candidate() {
        let plan = Plan::new(SOURCE, &["winner", "loser"], false, false).unwrap();
        let mut host = FakeHost::default();
        let outcome = run(
            &mut host,
            &RunRequest {
                plan: &plan,
                source_path: "/source",
                mount_path: "/mount",
                storage_path: "/storage",
                selected_candidate: "winner",
            },
        )
        .unwrap();
        assert_eq!(outcome.selected_candidate, "winner");
        assert_eq!(outcome.candidate_receipts.len(), 2);
        assert_eq!(host.aborted, vec!["loser", "winner"]);
        assert!(host.commands.iter().all(|command| {
            !command
                .arguments
                .iter()
                .any(|argument| argument == "commit")
        }));
    }

    #[test]
    fn shell_retains_candidates_on_source_drift() {
        let plan = Plan::new(SOURCE, &["candidate"], false, false).unwrap();
        let mut host = FakeHost {
            current_source: "source:drift",
            ..FakeHost::default()
        };
        let error = run(
            &mut host,
            &RunRequest {
                plan: &plan,
                source_path: "/source",
                mount_path: "/mount",
                storage_path: "/storage",
                selected_candidate: "candidate",
            },
        )
        .unwrap_err();
        assert_eq!(error.stage, RunStage::SourceFreshness);
        assert_eq!(error.retained_candidates, vec!["candidate"]);
        assert!(host.aborted.is_empty());
    }

    struct FakeHost {
        commands: Vec<CommandSpec>,
        aborted: Vec<String>,
        current_source: &'static str,
    }

    impl Default for FakeHost {
        fn default() -> Self {
            Self {
                commands: Vec::new(),
                aborted: Vec::new(),
                current_source: SOURCE,
            }
        }
    }

    impl Host for FakeHost {
        fn execute(&mut self, command: &CommandSpec) -> Result<(), HostError> {
            self.commands.push(command.clone());
            Ok(())
        }

        fn execute_candidate(&mut self, _candidate: &str) -> Result<(), HostError> {
            Ok(())
        }

        fn manifest_identity(&mut self, candidate: &str) -> Result<String, HostError> {
            Ok(format!("manifest:{candidate}"))
        }

        fn current_source_identity(&mut self) -> Result<String, HostError> {
            Ok(self.current_source.to_owned())
        }

        fn materialize_winner(
            &mut self,
            candidate: &str,
            _source_identity: &str,
        ) -> Result<Materialization, HostError> {
            Ok(Materialization {
                manifest_identity: format!("manifest:{candidate}"),
                workspace_identity: "workspace:fresh".into(),
                change_identity: "change:fresh".into(),
                destination_is_fresh: true,
            })
        }

        fn abort_candidate(&mut self, candidate: &str) -> Result<(), HostError> {
            self.aborted.push(candidate.to_owned());
            Ok(())
        }
    }

    #[test]
    fn receipt_contains_hashes_not_host_material() {
        let plan = Plan::new(SOURCE, &["winner"], false, false).unwrap();
        let receipt = candidate_receipt(&plan, "winner", "manifest:hash", "aborted");
        let rendered = format!("{receipt:?}");
        assert!(!rendered.contains("/tmp/"));
        assert!(!rendered.contains("file body"));
        assert_eq!(receipt.cohort, COHORT);
    }
}
