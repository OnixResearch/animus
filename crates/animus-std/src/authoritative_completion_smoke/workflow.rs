//! Pure completion orchestration over validated inputs and injected fixture services.

use crate::multirolesmoke::{JudgeDecision, compose_multi_role_advisory};
use crate::{
    AgentCheckExecutionRequest, AgentCheckExecutionService, CairnCompletionValidationRequest,
    CairnCompletionValidationService, ChangedArtifactObservationRequest,
    ChangedArtifactObservationService, CompletionArtifactPersistenceRequest,
    CompletionArtifactPersistenceService, FinalSubjectHashRequest, FinalSubjectHashService,
};
use animus_runtime::authoritative_completion::{
    AgentCheckArtifactDraft, AgentCheckExecutionDraft, AgentCheckRegistry, AgentCheckSelection,
    AgentCheckSelectionRequest, AuthoritativeCompletionCandidateDraft,
    AuthoritativeCompletionOutcome, CairnCompletionSummaryDraft, ChangedArtifactDraft,
    CheckCapability, CheckTier, CompletionCandidateSource, admit_completion_candidate,
    admit_execution_results, evaluate_authoritative_completion, select_required_agent_checks,
};

use super::fixtures::{
    FixtureArtifactObserver, FixtureArtifactPersister, FixtureCairnValidator, FixtureCheckExecutor,
    FixtureSubjectHasher, artifact_byte_count,
};
use super::{CompletionFixture, HeadState, domain_hash, error};

const ERROR_ROLE_ADVISORY: &str = "role-advisory";
const ERROR_ROLE_SELF_ATTESTATION: &str = "role-self-attestation";
const ERROR_JUDGE_VETO: &str = "judge-veto";
const ERROR_HEAD_DIRTY: &str = "head-dirty";
const ERROR_ARTIFACT_OBSERVATION: &str = "artifact-observation";
const ERROR_SUBJECT_HASH: &str = "subject-hash";
const ERROR_SELECTION: &str = "check-selection";
const ERROR_EXECUTION: &str = "check-execution";
const ERROR_PERSISTENCE: &str = "artifact-persistence";
const ERROR_CAIRN: &str = "cairn-validation";
const ERROR_COMPLETION: &str = "completion";
const ERROR_FAILURE_NOT_BLOCKED: &str = "failure-not-blocked";
const WORK_ID: &str = "authoritative-completion-smoke";
const COMPLETION_REQUEST: &str = "completion-request";
const WORKSPACE_LABEL: &str = "repository-root";
const SUBJECT_SEED_DOMAIN: &str = "animus.completion-smoke.subject-seed.v1";
const STALE_SUBJECT_DOMAIN: &str = "animus.completion-smoke.stale-subject.v1";
const CONTRACT_DOMAIN: &str = "animus.completion-smoke.contract.v1";
const ROLE_CONTRACT: &str = "contract";
const ROLE_RESULTS: &str = "results";
const AVAILABLE_CAPABILITIES: &[CheckCapability] = &[CheckCapability::Nix, CheckCapability::Cairn];

#[derive(Clone, Copy)]
pub(super) struct LoadedCompletionInput<'a> {
    pub roles: &'a animus_runtime::agent_roles::AgentRoleCatalog,
    pub model_profiles: &'a animus_runtime::ModelRuntimeProfileSet,
    pub registry: &'a AgentCheckRegistry,
    pub subject: &'a str,
    pub head_state: HeadState,
    pub fixture: CompletionFixture,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct CompletionReport {
    pub role_advisory_hash: String,
    pub judge_decision: &'static str,
    pub registry_hash: String,
    pub selection_hash: String,
    pub final_subject_hash: String,
    pub execution_evidence_hash: String,
    pub cairn_receipt_hash: String,
    pub candidate_hash: String,
    pub completion_receipt_hash: String,
    pub required_check_count: usize,
    pub passed_check_count: usize,
    pub service_calls: HostServiceCalls,
    pub authoritative_completion: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct HostServiceCalls {
    pub artifact_observer: usize,
    pub subject_hasher: usize,
    pub check_executor: usize,
    pub artifact_persister: usize,
    pub cairn_validator: usize,
}

struct ReportInput<'a> {
    fixture: CompletionFixture,
    advisory: &'a crate::multirolesmoke::MultiRoleAdvisory,
    selection: &'a AgentCheckSelection,
    evidence: &'a animus_runtime::authoritative_completion::ExecutionEvidenceReport,
    candidate: &'a animus_runtime::authoritative_completion::AuthoritativeCompletionCandidate,
    outcome: AuthoritativeCompletionOutcome,
    service_calls: HostServiceCalls,
}

pub(super) fn run_completion_workflow(
    input: LoadedCompletionInput<'_>,
) -> Result<CompletionReport, String> {
    // r[impl agent_harness_runtime.authoritative_completion_host_smoke.host_services]
    // r[impl agent_harness_runtime.authoritative_completion_host_smoke.completion]
    let (subject_seed, advisory) = prepare_role_advisory(input)?;
    let mut artifact_observer = FixtureArtifactObserver::new(subject_seed.clone());
    let artifacts = artifact_observer
        .observe(ChangedArtifactObservationRequest {
            workspace_label: WORKSPACE_LABEL,
        })
        .map_err(|_failure| error(ERROR_ARTIFACT_OBSERVATION))?;
    let changed = artifacts
        .iter()
        .map(|artifact| ChangedArtifactDraft {
            path: artifact.path.as_str(),
            surface: artifact.surface.as_str(),
            content_hash: artifact.content_hash.as_str(),
        })
        .collect::<Vec<_>>();
    let probe = select_checks(input.registry, changed.as_slice(), subject_seed.as_str());
    if !probe.is_ready() {
        return Err(error(ERROR_SELECTION));
    }
    let mut subject_hasher = FixtureSubjectHasher::default();
    let final_subject = subject_hasher
        .final_subject_hash(FinalSubjectHashRequest {
            changed_artifact_hash: probe.changed_artifact_hash(),
            registry_hash: probe.registry_hash(),
            policy_hash: probe.policy_hash(),
        })
        .map_err(|_failure| error(ERROR_SUBJECT_HASH))?;
    let selection = select_checks(input.registry, changed.as_slice(), final_subject.as_str());
    if !selection.is_ready() || selection.changed_artifact_hash() != probe.changed_artifact_hash() {
        return Err(error(ERROR_SELECTION));
    }

    let execution_subject = if input.fixture == CompletionFixture::StaleSubject {
        domain_hash(STALE_SUBJECT_DOMAIN, &[selection.final_subject_hash()])
    } else {
        selection.final_subject_hash().to_string()
    };
    let mut executor = FixtureCheckExecutor::default();
    let mut observations = execute_checks(&selection, &mut executor)?;
    if input.fixture == CompletionFixture::MissingResult {
        observations.pop();
    }
    let evidence = admit_fixture_results(&selection, observations.as_slice(), &execution_subject);
    let candidate = admit_completion_candidate(&AuthoritativeCompletionCandidateDraft {
        source: CompletionCandidateSource::Typed,
        request: COMPLETION_REQUEST,
        work_id: WORK_ID,
        subject_hash: selection.final_subject_hash(),
    })
    .map_err(|_failure| error(ERROR_COMPLETION))?;

    let contract_hash = domain_hash(
        CONTRACT_DOMAIN,
        &[
            selection.registry_hash(),
            selection.selection_hash(),
            selection.final_subject_hash(),
        ],
    );
    let mut persister = FixtureArtifactPersister::default();
    persist_completion_artifacts(
        &mut persister,
        contract_hash.as_str(),
        evidence.evidence_hash(),
    )?;
    let mut cairn = FixtureCairnValidator::new(&selection, &evidence);
    let summary = if input.fixture == CompletionFixture::MissingCairn {
        None
    } else {
        Some(
            cairn
                .validate_completion(CairnCompletionValidationRequest {
                    contract_hash: contract_hash.as_str(),
                    results_hash: evidence.evidence_hash(),
                    final_subject_hash: selection.final_subject_hash(),
                })
                .map_err(|_failure| error(ERROR_CAIRN))?,
        )
    };
    let outcome = evaluate_with_summary(&candidate, &selection, &evidence, summary.as_ref());
    finish_report(ReportInput {
        fixture: input.fixture,
        advisory: &advisory,
        selection: &selection,
        evidence: &evidence,
        candidate: &candidate,
        outcome,
        service_calls: HostServiceCalls {
            artifact_observer: artifact_observer.calls,
            subject_hasher: subject_hasher.calls,
            check_executor: executor.calls,
            artifact_persister: persister.calls,
            cairn_validator: cairn.calls,
        },
    })
}

fn prepare_role_advisory(
    input: LoadedCompletionInput<'_>,
) -> Result<(String, crate::multirolesmoke::MultiRoleAdvisory), String> {
    let subject_seed = domain_hash(SUBJECT_SEED_DOMAIN, &[input.subject]);
    let judge_decision = judge_decision(input.fixture);
    let advisory = compose_multi_role_advisory(
        input.roles,
        input.model_profiles,
        subject_seed.as_str(),
        judge_decision,
    )
    .map_err(|_failure| error(ERROR_ROLE_ADVISORY))?;
    if advisory.authoritative_completion || advisory.role_count == 0 {
        return Err(error(ERROR_ROLE_ADVISORY));
    }
    enforce_pre_completion_boundaries(input.fixture, input.head_state, &advisory)?;
    Ok((subject_seed, advisory))
}

fn judge_decision(fixture: CompletionFixture) -> JudgeDecision {
    if fixture == CompletionFixture::JudgeVeto {
        JudgeDecision::Veto
    } else {
        JudgeDecision::Select
    }
}

fn enforce_pre_completion_boundaries(
    fixture: CompletionFixture,
    head_state: HeadState,
    advisory: &crate::multirolesmoke::MultiRoleAdvisory,
) -> Result<(), String> {
    // r[impl agent_harness_runtime.authoritative_completion_host_smoke.role_boundary]
    if fixture == CompletionFixture::RoleSelfAttestation {
        let rejected = admit_completion_candidate(&AuthoritativeCompletionCandidateDraft {
            source: CompletionCandidateSource::Typed,
            request: advisory.advisory_hash.as_str(),
            work_id: WORK_ID,
            subject_hash: advisory.advisory_hash.as_str(),
        })
        .is_err();
        if rejected {
            return Err(error(ERROR_ROLE_SELF_ATTESTATION));
        }
        return Err(error(ERROR_FAILURE_NOT_BLOCKED));
    }
    if advisory.judge_decision == JudgeDecision::Veto {
        return Err(error(ERROR_JUDGE_VETO));
    }
    if head_state == HeadState::Dirty {
        return Err(error(ERROR_HEAD_DIRTY));
    }
    Ok(())
}

fn select_checks(
    registry: &AgentCheckRegistry,
    changed: &[ChangedArtifactDraft<'_>],
    final_subject_hash: &str,
) -> AgentCheckSelection {
    select_required_agent_checks(AgentCheckSelectionRequest {
        registry,
        changed_artifacts: changed,
        available_capabilities: AVAILABLE_CAPABILITIES,
        maximum_tier: CheckTier::Quick,
        final_subject_hash,
    })
}

fn execute_checks(
    selection: &AgentCheckSelection,
    executor: &mut FixtureCheckExecutor,
) -> Result<Vec<crate::HostCheckExecutionObservation>, String> {
    let mut observations = Vec::with_capacity(selection.required_checks().len());
    for check in selection.required_checks() {
        let observation = executor
            .execute(AgentCheckExecutionRequest {
                check,
                registry_hash: selection.registry_hash(),
                selection_hash: selection.selection_hash(),
                policy_hash: selection.policy_hash(),
                final_subject_hash: selection.final_subject_hash(),
            })
            .map_err(|_failure| error(ERROR_EXECUTION))?;
        observations.push(observation);
    }
    Ok(observations)
}

fn admit_fixture_results(
    selection: &AgentCheckSelection,
    observations: &[crate::HostCheckExecutionObservation],
    execution_subject: &str,
) -> animus_runtime::authoritative_completion::ExecutionEvidenceReport {
    let selected = &selection.required_checks()[..observations.len()];
    let command_refs = selected
        .iter()
        .map(|check| {
            check
                .command()
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let artifacts = observations
        .iter()
        .map(|observation| {
            observation
                .artifacts
                .iter()
                .map(|artifact| AgentCheckArtifactDraft {
                    path: artifact.path.as_str(),
                    hash: artifact.hash.as_str(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let drafts = selected
        .iter()
        .enumerate()
        .map(|(index, check)| {
            let observation = &observations[index];
            AgentCheckExecutionDraft {
                check_id: check.check_id(),
                status: observation.status,
                command: command_refs[index].as_slice(),
                command_hash: check.command_hash(),
                subject_hash: execution_subject,
                registry_hash: selection.registry_hash(),
                selection_hash: selection.selection_hash(),
                policy_hash: selection.policy_hash(),
                evidence_hash: observation.evidence_hash.as_str(),
                execution_receipt_hash: observation.execution_receipt_hash.as_str(),
                exit_code: observation.exit_code,
                signal: observation.signal,
                timed_out: observation.timed_out,
                cleanup_complete: observation.cleanup_complete,
                stdout_bytes: observation.stdout_bytes,
                stderr_bytes: observation.stderr_bytes,
                stdout_hash: observation.stdout_hash.as_str(),
                stderr_hash: observation.stderr_hash.as_str(),
                artifacts: artifacts[index].as_slice(),
                diagnostic: None,
            }
        })
        .collect::<Vec<_>>();
    admit_execution_results(selection, drafts.as_slice())
}

fn persist_completion_artifacts(
    persister: &mut FixtureArtifactPersister,
    contract_hash: &str,
    results_hash: &str,
) -> Result<(), String> {
    for request in [
        CompletionArtifactPersistenceRequest {
            role: ROLE_CONTRACT,
            content_hash: contract_hash,
            byte_count: artifact_byte_count(),
        },
        CompletionArtifactPersistenceRequest {
            role: ROLE_RESULTS,
            content_hash: results_hash,
            byte_count: artifact_byte_count(),
        },
    ] {
        persister
            .persist(request)
            .map_err(|_failure| error(ERROR_PERSISTENCE))?;
    }
    Ok(())
}

fn evaluate_with_summary(
    candidate: &animus_runtime::authoritative_completion::AuthoritativeCompletionCandidate,
    selection: &AgentCheckSelection,
    evidence: &animus_runtime::authoritative_completion::ExecutionEvidenceReport,
    summary: Option<&crate::CairnCompletionSummary>,
) -> AuthoritativeCompletionOutcome {
    let required_refs = summary.map(|value| {
        value
            .required_checks
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    });
    let passed_refs = summary.map(|value| {
        value
            .passed_checks
            .iter()
            .map(String::as_str)
            .collect::<Vec<_>>()
    });
    let draft = summary.map(|value| CairnCompletionSummaryDraft {
        schema: value.schema.as_str(),
        classification: value.classification.as_str(),
        repository_id: value.repository_id.as_str(),
        registry_id: value.registry_id.as_str(),
        registry_hash: value.registry_hash.as_str(),
        selection_hash: value.selection_hash.as_str(),
        policy_hash: value.policy_hash.as_str(),
        final_subject_hash: value.final_subject_hash.as_str(),
        required_checks: required_refs.as_deref().unwrap_or(&[]),
        passed_checks: passed_refs.as_deref().unwrap_or(&[]),
        non_passing_count: value.non_passing_count,
        diagnostic_count: value.diagnostic_count,
        valid: value.valid,
        verdict: value.verdict.as_str(),
        input_hash: value.input_hash.as_str(),
        receipt_hash: value.receipt_hash.as_str(),
    });
    evaluate_authoritative_completion(candidate, selection, evidence, draft.as_ref())
}

fn finish_report(input: ReportInput<'_>) -> Result<CompletionReport, String> {
    match input.outcome {
        AuthoritativeCompletionOutcome::Verified(receipt) => {
            if input.fixture != CompletionFixture::Admitted {
                return Err(error(ERROR_FAILURE_NOT_BLOCKED));
            }
            Ok(CompletionReport {
                role_advisory_hash: input.advisory.advisory_hash.clone(),
                judge_decision: input.advisory.judge_decision.as_str(),
                registry_hash: input.selection.registry_hash().to_string(),
                selection_hash: input.selection.selection_hash().to_string(),
                final_subject_hash: input.selection.final_subject_hash().to_string(),
                execution_evidence_hash: input.evidence.evidence_hash().to_string(),
                cairn_receipt_hash: receipt.cairn_receipt_hash().to_string(),
                candidate_hash: input.candidate.candidate_hash().to_string(),
                completion_receipt_hash: receipt.receipt_hash().to_string(),
                required_check_count: receipt.required_checks().len(),
                passed_check_count: receipt.passed_checks().len(),
                service_calls: input.service_calls,
                authoritative_completion: receipt.authoritative_completion(),
            })
        }
        AuthoritativeCompletionOutcome::Blocked(_blockers) => {
            if input.fixture == CompletionFixture::Admitted {
                Err(error(ERROR_COMPLETION))
            } else {
                Err(error(input.fixture.issue()))
            }
        }
    }
}
