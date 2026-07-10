//! Host-owned model-evidence report ingestion seam.
//!
//! This module plans ingestion of already-produced verifier, conversion,
//! kernel-proof, and benchmark reports. Collection, filesystem access, process
//! execution, GPU access, persistence, and presentation remain host-owned;
//! reusable crates receive only validated `animus-adapters` DTOs.

#![cfg_attr(animus_octet, allow(tigerstyle::numeric_units))]

const REPORT_LABEL_FIELD: animus_adapters::FieldName =
    animus_adapters::FieldName::new("std_model_evidence_report_label");
const UNIX_ROOT_PREFIX: &str = "/";
const MIN_POLICY_VERSION: u32 = 1;

/// Host service readiness for model-evidence ingestion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelEvidenceIngestServiceState {
    /// A host ingestion service is available.
    Available,
    /// Host policy denied evidence ingestion.
    Denied,
    /// No host evidence service was injected.
    Missing,
    /// A host service exists but is currently unavailable.
    Unavailable,
}

/// Borrowed host request for planning model-evidence ingestion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelEvidenceIngestRequest<'a> {
    /// Safe report label used in receipts instead of a raw host path.
    pub safe_report_label: &'a str,
    /// Host policy revision for this ingestion boundary.
    pub policy_version: u32,
    /// Service readiness declared by the host.
    pub service_state: ModelEvidenceIngestServiceState,
}

/// Safe plan handed to an injected model-evidence source.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ModelEvidenceIngestPlan {
    safe_report_label: String,
    policy_version: u32,
}

impl ModelEvidenceIngestPlan {
    /// Returns the safe report label.
    #[must_use]
    pub fn safe_report_label(&self) -> &str {
        self.safe_report_label.as_str()
    }

    /// Returns the host policy revision.
    #[must_use]
    pub const fn policy_version(&self) -> u32 {
        self.policy_version
    }
}

/// Errors raised by the std model-evidence ingestion seam.
#[derive(Debug)]
pub enum ModelEvidenceIngestError {
    /// Host policy denied ingestion.
    Denied,
    /// No service was injected.
    MissingService,
    /// The service is unavailable.
    ServiceUnavailable,
    /// Request metadata was malformed or unsafe.
    InvalidRequest {
        /// Safe reason.
        reason: &'static str,
    },
    /// Adapter DTO validation failed.
    AdapterValidation {
        /// Source validation error.
        source: animus_adapters::ModelEvidenceError,
    },
    /// Injected source failed with a safe reason.
    SourceFailed {
        /// Safe reason.
        reason: &'static str,
    },
}

impl core::fmt::Display for ModelEvidenceIngestError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Denied => formatter.write_str("model evidence ingestion denied"),
            Self::MissingService => formatter.write_str("model evidence ingestion service missing"),
            Self::ServiceUnavailable => {
                formatter.write_str("model evidence ingestion service unavailable")
            }
            Self::InvalidRequest { reason } => {
                write!(
                    formatter,
                    "invalid model evidence ingestion request: {reason}"
                )
            }
            Self::AdapterValidation { source } => {
                write!(formatter, "model evidence report invalid: {source}")
            }
            Self::SourceFailed { reason } => {
                write!(formatter, "model evidence source failed: {reason}")
            }
        }
    }
}

impl std::error::Error for ModelEvidenceIngestError {}

/// Host-owned model-evidence source service.
pub trait ModelEvidenceSource {
    /// Loads or receives one already-planned model-evidence report.
    ///
    /// # Errors
    ///
    /// Returns [`ModelEvidenceIngestError`] when host policy, filesystem access,
    /// verifier collection, GPU/runtime access, persistence, presentation, or
    /// adapter DTO validation fails.
    fn ingest(
        &mut self,
        plan: &ModelEvidenceIngestPlan,
    ) -> Result<animus_adapters::ModelEvidenceReport, ModelEvidenceIngestError>;
}

/// Host adapter shell over an injected model-evidence source.
pub struct ModelEvidenceIngestHost<T> {
    source: T,
}

impl<T> ModelEvidenceIngestHost<T> {
    /// Builds a host shell over caller-owned source state.
    #[must_use]
    pub const fn new(source: T) -> Self {
        Self { source }
    }

    /// Returns the wrapped source.
    #[must_use]
    pub const fn source(&self) -> &T {
        &self.source
    }

    /// Returns the wrapped source mutably.
    #[must_use]
    pub const fn source_mut(&mut self) -> &mut T {
        &mut self.source
    }
}

impl<T> ModelEvidenceIngestHost<T>
where
    T: ModelEvidenceSource,
{
    /// Plans and runs one host-owned model-evidence ingestion.
    ///
    /// # Errors
    ///
    /// Returns [`ModelEvidenceIngestError`] when planning fails or the injected
    /// source cannot safely return a validated report.
    pub fn ingest(
        &mut self,
        request: &ModelEvidenceIngestRequest<'_>,
    ) -> Result<animus_adapters::ModelEvidenceReport, ModelEvidenceIngestError> {
        let plan = plan_model_evidence_ingest(request)?;
        self.source.ingest(&plan)
    }
}

/// Plans model-evidence ingestion without reading files or invoking verifiers.
///
/// # Errors
///
/// Returns [`ModelEvidenceIngestError`] for denied, missing, unavailable, unsafe,
/// raw-path, or zero-version requests.
pub fn plan_model_evidence_ingest(
    request: &ModelEvidenceIngestRequest<'_>,
) -> Result<ModelEvidenceIngestPlan, ModelEvidenceIngestError> {
    validate_service_state(request.service_state)?;
    if request.policy_version < MIN_POLICY_VERSION {
        return Err(ModelEvidenceIngestError::InvalidRequest {
            reason: "model evidence policy version must be positive",
        });
    }
    let safe_report_label = normalize_safe_field(REPORT_LABEL_FIELD, request.safe_report_label)?;
    Ok(ModelEvidenceIngestPlan {
        safe_report_label,
        policy_version: request.policy_version,
    })
}

/// Validates a host-produced model-evidence report draft at the shell boundary.
///
/// # Errors
///
/// Returns [`ModelEvidenceIngestError::AdapterValidation`] when the report fails
/// reusable DTO validation.
pub fn validate_model_evidence_at_shell(
    draft: animus_adapters::ModelEvidenceReportDraft<'_>,
) -> Result<animus_adapters::ModelEvidenceReport, ModelEvidenceIngestError> {
    animus_adapters::validate_model_evidence_report(draft)
        .map_err(|source| ModelEvidenceIngestError::AdapterValidation { source })
}

fn validate_service_state(
    state: ModelEvidenceIngestServiceState,
) -> Result<(), ModelEvidenceIngestError> {
    match state {
        ModelEvidenceIngestServiceState::Available => Ok(()),
        ModelEvidenceIngestServiceState::Denied => Err(ModelEvidenceIngestError::Denied),
        ModelEvidenceIngestServiceState::Missing => Err(ModelEvidenceIngestError::MissingService),
        ModelEvidenceIngestServiceState::Unavailable => {
            Err(ModelEvidenceIngestError::ServiceUnavailable)
        }
    }
}

fn normalize_safe_field(
    field: animus_adapters::FieldName,
    value: &str,
) -> Result<String, ModelEvidenceIngestError> {
    let normalized = animus_adapters::normalized_adapter_field(field, value).map_err(|_error| {
        ModelEvidenceIngestError::InvalidRequest {
            reason: "model evidence ingestion metadata field is invalid",
        }
    })?;
    if normalized.starts_with(UNIX_ROOT_PREFIX) {
        return Err(ModelEvidenceIngestError::InvalidRequest {
            reason: "model evidence ingestion metadata must not contain raw absolute paths",
        });
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    const REPORT_LABEL: &str = "model-evidence:alpha";
    const POLICY_VERSION: u32 = 1;
    const MODEL_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const CORPUS_HASH: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
    const EXPECTED_CALLS: u32 = 1;
    const SUPPORTING_EVIDENCE_COUNT: u32 = 1;

    #[derive(Default)]
    struct FixtureSource {
        calls: u32,
    }

    impl ModelEvidenceSource for FixtureSource {
        fn ingest(
            &mut self,
            plan: &ModelEvidenceIngestPlan,
        ) -> Result<animus_adapters::ModelEvidenceReport, ModelEvidenceIngestError> {
            self.calls = self.calls.saturating_add(EXPECTED_CALLS);
            let diagnostics = [animus_adapters::ModelEvidenceDiagnostic::new(
                animus_adapters::ModelEvidenceValidationIssue::InvalidField,
                "fixture",
                "safe fixture diagnostic",
            )];
            let capabilities = [animus_adapters::ModelEvidenceCapabilityLabel::AdvisoryRouting];
            let non_claims = [
                animus_adapters::ModelEvidenceNonClaim::ArbitraryRobustness,
                animus_adapters::ModelEvidenceNonClaim::OutputCorrectness,
                animus_adapters::ModelEvidenceNonClaim::HallucinationPrevention,
                animus_adapters::ModelEvidenceNonClaim::VerifierSoundness,
                animus_adapters::ModelEvidenceNonClaim::BenchmarkSuperiority,
                animus_adapters::ModelEvidenceNonClaim::ReleaseEligibility,
                animus_adapters::ModelEvidenceNonClaim::GpuCorrectness,
                animus_adapters::ModelEvidenceNonClaim::WholeSystemSafety,
            ];
            validate_model_evidence_at_shell(animus_adapters::ModelEvidenceReportDraft {
                report_id: plan.safe_report_label(),
                evidence_kind: "bound-report",
                model: animus_adapters::ModelArtifactIdentityDraft {
                    model_label: "model:alpha",
                    model_hash: MODEL_HASH,
                    observed_model_hash: MODEL_HASH,
                },
                property_scope: "classification-bound:local",
                verifier_kind: "gamma-crown",
                verifier_identity: "gamma-crown-fixture",
                input_corpus_hash: CORPUS_HASH,
                result_status: animus_adapters::ModelEvidenceResultStatus::Passed,
                freshness: animus_adapters::ModelEvidenceFreshness::Current,
                supporting_evidence_count: SUPPORTING_EVIDENCE_COUNT,
                diagnostics: &diagnostics,
                capability_labels: &capabilities,
                non_claims: &non_claims,
            })
        }
    }

    fn request() -> ModelEvidenceIngestRequest<'static> {
        ModelEvidenceIngestRequest {
            safe_report_label: REPORT_LABEL,
            policy_version: POLICY_VERSION,
            service_state: ModelEvidenceIngestServiceState::Available,
        }
    }

    #[test]
    fn model_evidence_ingest_host_runs_injected_source_for_valid_plan() {
        // r[verify model-evidence-adapters.host-shell]
        let mut host = ModelEvidenceIngestHost::new(FixtureSource::default());
        let report = host
            .ingest(&request())
            .expect("host evidence source should return a validated report");

        assert_eq!(host.source().calls, EXPECTED_CALLS);
        assert_eq!(report.report_id(), REPORT_LABEL);
        assert_eq!(
            report.evidence_kind(),
            animus_adapters::ModelEvidenceKind::BoundReport
        );
    }

    #[test]
    fn model_evidence_ingest_fails_closed_for_missing_service_and_raw_path_label() {
        // r[verify model-evidence-adapters.negative]
        let mut missing = request();
        missing.service_state = ModelEvidenceIngestServiceState::Missing;
        assert!(matches!(
            plan_model_evidence_ingest(&missing),
            Err(ModelEvidenceIngestError::MissingService)
        ));

        let raw_path = ModelEvidenceIngestRequest {
            safe_report_label: "/home/example/report.json",
            ..request()
        };
        assert!(matches!(
            plan_model_evidence_ingest(&raw_path),
            Err(ModelEvidenceIngestError::InvalidRequest { .. })
        ));
    }
}
