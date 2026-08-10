//! Deterministic injected host services for the completion smoke.

use crate::{
    AgentCheckExecutionRequest, AgentCheckExecutionService, CairnCompletionSummary,
    CairnCompletionValidationRequest, CairnCompletionValidationService,
    ChangedArtifactObservationRequest, ChangedArtifactObservationService,
    CompletionArtifactPersistenceRequest, CompletionArtifactPersistenceService,
    FinalSubjectHashRequest, FinalSubjectHashService, HostArtifactObservation,
    HostCheckExecutionObservation, ObservedChangedArtifact,
};

use super::{CompletionCairnFactoryRequest, CompletionCairnServiceFactory, domain_hash};

const CHANGED_PATH: &str = "docs/authoritative-completion-smoke.md";
const CHANGED_SURFACE: &str = "documentation";
const EXECUTION_EVIDENCE_DOMAIN: &str = "animus.completion-smoke.execution-evidence.v1";
const EXECUTION_RECEIPT_DOMAIN: &str = "animus.completion-smoke.execution-receipt.v1";
const OUTPUT_HASH_DOMAIN: &str = "animus.completion-smoke.output.v1";
const ARTIFACT_HASH_DOMAIN: &str = "animus.completion-smoke.artifact.v1";
const SUBJECT_HASH_DOMAIN: &str = "animus.completion-smoke.final-subject.v1";
const CAIRN_INPUT_DOMAIN: &str = "animus.completion-smoke.cairn-input.v1";
const CAIRN_RECEIPT_DOMAIN: &str = "animus.completion-smoke.cairn-receipt.v1";
const CAIRN_SCHEMA: &str = "cairn.agent-completion.v2";
const AUTHORITATIVE: &str = "authoritative";
const PASS: &str = "PASS";
const ARTIFACT_BYTE_COUNT: u64 = 64;

pub(super) struct FixtureArtifactObserver {
    content_hash: String,
    pub calls: usize,
}

impl FixtureArtifactObserver {
    pub(super) fn new(content_hash: String) -> Self {
        Self {
            content_hash,
            calls: 0,
        }
    }
}

impl ChangedArtifactObservationService for FixtureArtifactObserver {
    type Error = ();

    fn observe(
        &mut self,
        _request: ChangedArtifactObservationRequest<'_>,
    ) -> Result<Vec<ObservedChangedArtifact>, Self::Error> {
        self.calls = self.calls.saturating_add(1);
        Ok(vec![ObservedChangedArtifact {
            path: CHANGED_PATH.to_string(),
            surface: CHANGED_SURFACE.to_string(),
            content_hash: self.content_hash.clone(),
        }])
    }
}

#[derive(Default)]
pub(super) struct FixtureSubjectHasher {
    pub calls: usize,
}

impl FinalSubjectHashService for FixtureSubjectHasher {
    type Error = ();

    fn final_subject_hash(
        &mut self,
        request: FinalSubjectHashRequest<'_>,
    ) -> Result<String, Self::Error> {
        self.calls = self.calls.saturating_add(1);
        Ok(domain_hash(
            SUBJECT_HASH_DOMAIN,
            &[
                request.changed_artifact_hash,
                request.registry_hash,
                request.policy_hash,
            ],
        ))
    }
}

#[derive(Default)]
pub(super) struct FixtureCheckExecutor {
    pub calls: usize,
}

impl AgentCheckExecutionService for FixtureCheckExecutor {
    type Error = ();

    fn execute(
        &mut self,
        request: AgentCheckExecutionRequest<'_>,
    ) -> Result<HostCheckExecutionObservation, Self::Error> {
        self.calls = self.calls.saturating_add(1);
        let check_id = request.check.check_id();
        Ok(HostCheckExecutionObservation {
            status: animus_runtime::authoritative_completion::CheckStatus::Passed,
            evidence_hash: domain_hash(EXECUTION_EVIDENCE_DOMAIN, &[check_id]),
            execution_receipt_hash: domain_hash(EXECUTION_RECEIPT_DOMAIN, &[check_id]),
            exit_code: Some(0),
            signal: None,
            timed_out: false,
            cleanup_complete: true,
            stdout_bytes: 0,
            stderr_bytes: 0,
            stdout_hash: domain_hash(OUTPUT_HASH_DOMAIN, &[check_id, "stdout"]),
            stderr_hash: domain_hash(OUTPUT_HASH_DOMAIN, &[check_id, "stderr"]),
            artifacts: vec![HostArtifactObservation {
                path: format!("target/completion-smoke/{check_id}.receipt"),
                hash: domain_hash(ARTIFACT_HASH_DOMAIN, &[check_id]),
            }],
        })
    }
}

#[derive(Default)]
pub(super) struct FixtureArtifactPersister {
    pub calls: usize,
}

impl CompletionArtifactPersistenceService for FixtureArtifactPersister {
    type Error = ();

    fn persist(
        &mut self,
        request: CompletionArtifactPersistenceRequest<'_>,
    ) -> Result<HostArtifactObservation, Self::Error> {
        self.calls = self.calls.saturating_add(1);
        if request.byte_count != ARTIFACT_BYTE_COUNT {
            return Err(());
        }
        Ok(HostArtifactObservation {
            path: format!("target/completion-smoke/{}.json", request.role),
            hash: request.content_hash.to_string(),
        })
    }
}

pub(super) struct FixtureCairnValidator {
    repository_id: String,
    registry_id: String,
    registry_hash: String,
    selection_hash: String,
    policy_hash: String,
    final_subject_hash: String,
    required_checks: Vec<String>,
    passed_checks: Vec<String>,
    expected_results_hash: String,
    pub calls: usize,
}

impl FixtureCairnValidator {
    fn new(
        selection: &animus_runtime::authoritative_completion::AgentCheckSelection,
        evidence: &animus_runtime::authoritative_completion::ExecutionEvidenceReport,
    ) -> Self {
        Self {
            repository_id: selection.repository_id().to_string(),
            registry_id: selection.registry_id().to_string(),
            registry_hash: selection.registry_hash().to_string(),
            selection_hash: selection.selection_hash().to_string(),
            policy_hash: selection.policy_hash().to_string(),
            final_subject_hash: selection.final_subject_hash().to_string(),
            required_checks: selection
                .required_checks()
                .iter()
                .map(|check| check.check_id().to_string())
                .collect(),
            passed_checks: evidence.passed_checks().to_vec(),
            expected_results_hash: evidence.evidence_hash().to_string(),
            calls: 0,
        }
    }
}

pub(super) struct FixtureCairnValidatorFactory;

impl CompletionCairnServiceFactory for FixtureCairnValidatorFactory {
    type Error = ();
    type Service = FixtureCairnValidator;

    fn create(
        &mut self,
        request: CompletionCairnFactoryRequest<'_>,
    ) -> Result<Self::Service, Self::Error> {
        Ok(FixtureCairnValidator::new(
            request.selection,
            request.evidence,
        ))
    }
}

impl CairnCompletionValidationService for FixtureCairnValidator {
    type Error = ();

    fn validate_completion(
        &mut self,
        request: CairnCompletionValidationRequest<'_>,
    ) -> Result<CairnCompletionSummary, Self::Error> {
        self.calls = self.calls.saturating_add(1);
        if request.final_subject_hash != self.final_subject_hash
            || request.results_hash != self.expected_results_hash
        {
            return Err(());
        }
        let input_hash = domain_hash(
            CAIRN_INPUT_DOMAIN,
            &[
                request.contract_hash,
                request.results_hash,
                request.final_subject_hash,
            ],
        );
        let receipt_hash = domain_hash(CAIRN_RECEIPT_DOMAIN, &[input_hash.as_str()]);
        Ok(CairnCompletionSummary {
            schema: CAIRN_SCHEMA.to_string(),
            classification: AUTHORITATIVE.to_string(),
            repository_id: self.repository_id.clone(),
            registry_id: self.registry_id.clone(),
            registry_hash: self.registry_hash.clone(),
            selection_hash: self.selection_hash.clone(),
            policy_hash: self.policy_hash.clone(),
            final_subject_hash: self.final_subject_hash.clone(),
            required_checks: self.required_checks.clone(),
            passed_checks: self.passed_checks.clone(),
            non_passing_count: 0,
            diagnostic_count: 0,
            valid: true,
            verdict: PASS.to_string(),
            input_hash,
            receipt_hash,
        })
    }
}

pub(super) const fn artifact_byte_count() -> u64 {
    ARTIFACT_BYTE_COUNT
}
