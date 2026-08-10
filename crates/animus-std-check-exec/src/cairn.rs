//! Pinned Cairn completion evaluation over captured selected-check observations.

use animus_std::{
    CairnCompletionSummary, CairnCompletionValidationRequest, CairnCompletionValidationService,
    CompletionCairnFactoryRequest, CompletionCairnServiceFactory, HostCheckExecutionObservation,
};
use serde_json::{Value, json};

/// Safe pinned-Cairn preparation or request failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CairnServiceError {
    /// The selected-check and observation counts differ.
    ObservationCountMismatch,
    /// The workflow request differs from the prepared Cairn report.
    RequestMismatch,
    /// A report count cannot be represented by the Animus summary DTO.
    CountOverflow,
}

impl CairnServiceError {
    /// Return a stable safe issue label.
    #[must_use]
    pub const fn issue(self) -> &'static str {
        match self {
            Self::ObservationCountMismatch => "observation-count-mismatch",
            Self::RequestMismatch => "request-mismatch",
            Self::CountOverflow => "count-overflow",
        }
    }
}

/// Factory backed by the exact pinned Cairn completion core.
#[derive(Default)]
pub struct PinnedCairnFactory;

impl CompletionCairnServiceFactory for PinnedCairnFactory {
    type Error = CairnServiceError;
    type Service = PinnedCairnService;

    fn create(
        &mut self,
        request: CompletionCairnFactoryRequest<'_>,
    ) -> Result<Self::Service, Self::Error> {
        // r[impl agent_harness_runtime.real_check_completion_host_smoke.cairn]
        if request.selection.required_checks().len() != request.observations.len() {
            return Err(CairnServiceError::ObservationCountMismatch);
        }
        let contract = contract_value(request.selection);
        let results = results_value(request.selection, request.observations);
        let report = cairn_core::evaluate_authoritative_agent_completion(
            &contract,
            &results,
            request.selection.final_subject_hash(),
            request.selection.policy_hash(),
        );
        let non_passing_count = u32::try_from(report.non_passing_checks.len())
            .map_err(|_failure| CairnServiceError::CountOverflow)?;
        let diagnostic_count = u32::try_from(report.diagnostics.len())
            .map_err(|_failure| CairnServiceError::CountOverflow)?;
        let summary = CairnCompletionSummary {
            schema: report.schema,
            classification: report.classification,
            repository_id: report.repository_id,
            registry_id: report.registry_id,
            registry_hash: report.registry_hash,
            selection_hash: report.selection_hash,
            policy_hash: report.policy_hash,
            final_subject_hash: report.final_subject_hash,
            required_checks: report.required_checks,
            passed_checks: report.passed_checks,
            non_passing_count,
            diagnostic_count,
            valid: report.valid,
            verdict: report.verdict,
            input_hash: report.input_hash,
            receipt_hash: report.receipt_hash,
        };
        Ok(PinnedCairnService {
            expected_contract_hash: request.contract_hash.to_string(),
            expected_results_hash: request.evidence.evidence_hash().to_string(),
            expected_subject_hash: request.selection.final_subject_hash().to_string(),
            summary,
        })
    }
}

/// Prepared pinned-Cairn summary service.
pub struct PinnedCairnService {
    expected_contract_hash: String,
    expected_results_hash: String,
    expected_subject_hash: String,
    summary: CairnCompletionSummary,
}

impl CairnCompletionValidationService for PinnedCairnService {
    type Error = CairnServiceError;

    fn validate_completion(
        &mut self,
        request: CairnCompletionValidationRequest<'_>,
    ) -> Result<CairnCompletionSummary, Self::Error> {
        if request.contract_hash != self.expected_contract_hash
            || request.results_hash != self.expected_results_hash
            || request.final_subject_hash != self.expected_subject_hash
        {
            return Err(CairnServiceError::RequestMismatch);
        }
        Ok(self.summary.clone())
    }
}

fn contract_value(
    selection: &animus_runtime::authoritative_completion::AgentCheckSelection,
) -> Value {
    let required_checks = selection
        .required_checks()
        .iter()
        .map(|check| {
            json!({
                "check_id": check.check_id(),
                "command": check.command(),
                "command_hash": check.command_hash(),
                "prerequisites": check.prerequisites(),
            })
        })
        .collect::<Vec<_>>();
    json!({
        "schema": "cairn.agent-completion.v2",
        "repository_id": selection.repository_id(),
        "registry_id": selection.registry_id(),
        "registry_hash": selection.registry_hash(),
        "selection_hash": selection.selection_hash(),
        "policy_hash": selection.policy_hash(),
        "final_subject_hash": selection.final_subject_hash(),
        "required_checks": required_checks,
    })
}

fn results_value(
    selection: &animus_runtime::authoritative_completion::AgentCheckSelection,
    observations: &[HostCheckExecutionObservation],
) -> Value {
    let rows = selection
        .required_checks()
        .iter()
        .zip(observations)
        .map(|(check, observation)| {
            let artifacts = observation
                .artifacts
                .iter()
                .map(|artifact| json!({ "path": artifact.path, "hash": artifact.hash }))
                .collect::<Vec<_>>();
            json!({
                "check_id": check.check_id(),
                "status": observation.status.as_str(),
                "command": check.command(),
                "command_hash": check.command_hash(),
                "subject_hash": selection.final_subject_hash(),
                "registry_hash": selection.registry_hash(),
                "selection_hash": selection.selection_hash(),
                "policy_hash": selection.policy_hash(),
                "evidence_hash": observation.evidence_hash,
                "execution_receipt_hash": observation.execution_receipt_hash,
                "artifacts": artifacts,
                "diagnostic": null,
            })
        })
        .collect::<Vec<_>>();
    Value::Array(rows)
}
