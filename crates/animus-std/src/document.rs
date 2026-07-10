//! Host-owned document-ingest extraction seam.
//!
//! This module plans local document extraction requests and delegates all
//! filesystem reads, optional extractor execution, timeout enforcement, process
//! isolation, redaction, and receipt writes to an injected host service. The
//! reusable report DTO is validated by `animus-adapters` before it crosses back
//! into runtime code.

#![cfg_attr(
    animus_octet,
    allow(
        tigerstyle::compound_condition,
        tigerstyle::numeric_units,
        tigerstyle::path_segment_repetition
    )
)]

const SOURCE_LABEL_FIELD: animus_adapters::FieldName =
    animus_adapters::FieldName::new("std_document_source_label");
const TIMEOUT_CLASS_FIELD: animus_adapters::FieldName =
    animus_adapters::FieldName::new("std_document_timeout_class");
const EMPTY_PATH_COMPONENT: &str = "";
const PARENT_PATH_COMPONENT: &str = "..";
const UNIX_ROOT_PREFIX: &str = "/";
const WINDOWS_DRIVE_MARKER: &str = ":";
const MIN_CAP: u32 = 1;

/// Host service readiness for document extraction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DocumentExtractionServiceState {
    /// A host extraction service is available.
    Available,
    /// Host policy denied document extraction.
    Denied,
    /// No host extraction service was injected.
    Missing,
    /// A host service exists but is currently unavailable.
    Unavailable,
}

/// Host process-isolation requirement for document extraction.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DocumentExtractionIsolationRequirement {
    /// Host policy does not require process isolation for this request.
    NotRequired,
    /// Host policy requires the extractor to run in an isolated process boundary.
    Required,
}

impl DocumentExtractionIsolationRequirement {
    /// Returns the stable isolation requirement label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequired => "not_required",
            Self::Required => "required",
        }
    }
}

/// Borrowed host request for planning document extraction.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentExtractionRequest<'a> {
    /// Safe source label used in receipts instead of a raw host path.
    pub safe_source_label: &'a str,
    /// Host-owned local path relative to the configured extraction root.
    pub relative_path: &'a std::path::Path,
    /// Input byte cap enforced by the host shell.
    pub input_byte_cap: u32,
    /// Output byte cap enforced by the host shell.
    pub output_byte_cap: u32,
    /// Maximum document count enforced by the host shell.
    pub document_count_cap: u32,
    /// Timeout class enforced by the host shell.
    pub timeout_class: &'a str,
    /// Isolation requirement enforced by the host shell.
    pub isolation: DocumentExtractionIsolationRequirement,
    /// Service readiness declared by the host.
    pub service_state: DocumentExtractionServiceState,
}

/// Safe plan handed to an injected document extractor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentExtractionPlan {
    safe_source_label: String,
    relative_path: std::path::PathBuf,
    input_byte_cap: u32,
    output_byte_cap: u32,
    document_count_cap: u32,
    timeout_class: String,
    isolation: DocumentExtractionIsolationRequirement,
}

impl DocumentExtractionPlan {
    /// Returns the safe source label.
    #[must_use]
    pub fn safe_source_label(&self) -> &str {
        self.safe_source_label.as_str()
    }

    /// Returns the host-owned relative path.
    #[must_use]
    pub fn relative_path(&self) -> &std::path::Path {
        self.relative_path.as_path()
    }

    /// Returns the input byte cap.
    #[must_use]
    pub const fn input_byte_cap(&self) -> u32 {
        self.input_byte_cap
    }

    /// Returns the output byte cap.
    #[must_use]
    pub const fn output_byte_cap(&self) -> u32 {
        self.output_byte_cap
    }

    /// Returns the document count cap.
    #[must_use]
    pub const fn document_count_cap(&self) -> u32 {
        self.document_count_cap
    }

    /// Returns the timeout class.
    #[must_use]
    pub fn timeout_class(&self) -> &str {
        self.timeout_class.as_str()
    }

    /// Returns the isolation requirement.
    #[must_use]
    pub const fn isolation(&self) -> DocumentExtractionIsolationRequirement {
        self.isolation
    }
}

/// Errors raised by the std document-ingest seam.
#[derive(Debug)]
pub enum DocumentExtractionError {
    /// Host policy denied extraction.
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
        source: animus_adapters::DocumentIngestError,
    },
    /// Injected extractor failed with a safe reason.
    ExtractorFailed {
        /// Safe reason.
        reason: &'static str,
    },
}

impl core::fmt::Display for DocumentExtractionError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Denied => formatter.write_str("document extraction denied"),
            Self::MissingService => formatter.write_str("document extraction service missing"),
            Self::ServiceUnavailable => {
                formatter.write_str("document extraction service unavailable")
            }
            Self::InvalidRequest { reason } => {
                write!(formatter, "invalid document extraction request: {reason}")
            }
            Self::AdapterValidation { source } => {
                write!(formatter, "document ingest report invalid: {source}")
            }
            Self::ExtractorFailed { reason } => {
                write!(formatter, "document extractor failed: {reason}")
            }
        }
    }
}

impl std::error::Error for DocumentExtractionError {}

/// Host-owned document extractor service.
pub trait DocumentExtractor {
    /// Extracts one already-planned local document request.
    ///
    /// # Errors
    ///
    /// Returns [`DocumentExtractionError`] when host policy, filesystem access,
    /// extractor execution, timeout enforcement, process isolation, redaction,
    /// receipt persistence, or adapter DTO validation fails.
    fn extract(
        &mut self,
        plan: &DocumentExtractionPlan,
    ) -> Result<animus_adapters::DocumentIngestReport, DocumentExtractionError>;
}

/// Host adapter shell over an injected document extractor.
pub struct DocumentExtractionHost<T> {
    extractor: T,
}

impl<T> DocumentExtractionHost<T> {
    /// Builds a host shell over caller-owned extractor state.
    #[must_use]
    pub const fn new(extractor: T) -> Self {
        Self { extractor }
    }

    /// Returns the wrapped extractor.
    #[must_use]
    pub const fn extractor(&self) -> &T {
        &self.extractor
    }

    /// Returns the wrapped extractor mutably.
    #[must_use]
    pub const fn extractor_mut(&mut self) -> &mut T {
        &mut self.extractor
    }
}

impl<T> DocumentExtractionHost<T>
where
    T: DocumentExtractor,
{
    /// Plans and runs one host-owned document extraction.
    ///
    /// # Errors
    ///
    /// Returns [`DocumentExtractionError`] when planning fails or the injected
    /// extractor cannot safely return a validated report.
    pub fn extract(
        &mut self,
        request: &DocumentExtractionRequest<'_>,
    ) -> Result<animus_adapters::DocumentIngestReport, DocumentExtractionError> {
        let plan = plan_document_extraction(request)?;
        self.extractor.extract(&plan)
    }
}

/// Plans document extraction without reading files or invoking extractors.
///
/// # Errors
///
/// Returns [`DocumentExtractionError`] for denied, missing, unavailable, unsafe,
/// absolute, escaping, or zero-budget requests.
pub fn plan_document_extraction(
    request: &DocumentExtractionRequest<'_>,
) -> Result<DocumentExtractionPlan, DocumentExtractionError> {
    validate_service_state(request.service_state)?;
    validate_relative_path(request.relative_path)?;
    let safe_source_label = normalize_safe_field(SOURCE_LABEL_FIELD, request.safe_source_label)?;
    let timeout_class = normalize_safe_field(TIMEOUT_CLASS_FIELD, request.timeout_class)?;
    validate_caps(request)?;
    Ok(DocumentExtractionPlan {
        safe_source_label,
        relative_path: request.relative_path.to_path_buf(),
        input_byte_cap: request.input_byte_cap,
        output_byte_cap: request.output_byte_cap,
        document_count_cap: request.document_count_cap,
        timeout_class,
        isolation: request.isolation,
    })
}

/// Validates a host-produced report draft at the shell boundary.
///
/// # Errors
///
/// Returns [`DocumentExtractionError::AdapterValidation`] when the report fails
/// reusable DTO validation.
pub fn validate_document_report_at_shell(
    draft: animus_adapters::DocumentIngestReportDraft<'_>,
) -> Result<animus_adapters::DocumentIngestReport, DocumentExtractionError> {
    animus_adapters::validate_document_ingest_report(draft)
        .map_err(|source| DocumentExtractionError::AdapterValidation { source })
}

fn validate_service_state(
    state: DocumentExtractionServiceState,
) -> Result<(), DocumentExtractionError> {
    match state {
        DocumentExtractionServiceState::Available => Ok(()),
        DocumentExtractionServiceState::Denied => Err(DocumentExtractionError::Denied),
        DocumentExtractionServiceState::Missing => Err(DocumentExtractionError::MissingService),
        DocumentExtractionServiceState::Unavailable => {
            Err(DocumentExtractionError::ServiceUnavailable)
        }
    }
}

fn validate_caps(request: &DocumentExtractionRequest<'_>) -> Result<(), DocumentExtractionError> {
    if request.input_byte_cap < MIN_CAP
        || request.output_byte_cap < MIN_CAP
        || request.document_count_cap < MIN_CAP
    {
        return Err(DocumentExtractionError::InvalidRequest {
            reason: "document extraction caps must be positive",
        });
    }
    Ok(())
}

fn validate_relative_path(path: &std::path::Path) -> Result<(), DocumentExtractionError> {
    let path_text = path.to_string_lossy();
    if path_text.is_empty()
        || path_text.starts_with(UNIX_ROOT_PREFIX)
        || path_text.contains(WINDOWS_DRIVE_MARKER)
        || path.components().any(is_escape_component)
    {
        return Err(DocumentExtractionError::InvalidRequest {
            reason: "document extraction path must be relative and non-escaping",
        });
    }
    Ok(())
}

fn is_escape_component(component: std::path::Component<'_>) -> bool {
    matches!(component, std::path::Component::ParentDir)
        || component.as_os_str() == EMPTY_PATH_COMPONENT
        || component.as_os_str() == PARENT_PATH_COMPONENT
}

fn normalize_safe_field(
    field: animus_adapters::FieldName,
    value: &str,
) -> Result<String, DocumentExtractionError> {
    let normalized = animus_adapters::normalized_adapter_field(field, value).map_err(|_error| {
        DocumentExtractionError::InvalidRequest {
            reason: "document extraction metadata field is invalid",
        }
    })?;
    if normalized.starts_with(UNIX_ROOT_PREFIX) {
        return Err(DocumentExtractionError::InvalidRequest {
            reason: "document extraction metadata must not contain raw absolute paths",
        });
    }
    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAFE_SOURCE: &str = "artifact:doc:alpha";
    const RELATIVE_PATH: &str = "docs/input.md";
    const TIMEOUT: &str = "short";
    const INPUT_CAP: u32 = 1_024;
    const OUTPUT_CAP: u32 = 512;
    const DOCUMENT_CAP: u32 = 4;
    const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const SECTION_BYTES: u32 = 32;
    const DOCUMENT_COUNT: u32 = 1;
    const EXPECTED_CALLS: u32 = 1;

    #[derive(Default)]
    struct FixtureExtractor {
        calls: u32,
    }

    impl DocumentExtractor for FixtureExtractor {
        fn extract(
            &mut self,
            plan: &DocumentExtractionPlan,
        ) -> Result<animus_adapters::DocumentIngestReport, DocumentExtractionError> {
            self.calls = self.calls.saturating_add(EXPECTED_CALLS);
            let sections = [animus_adapters::DocumentSectionDraft {
                section_id: "section-1",
                heading: "overview",
                summary: "bounded summary",
                byte_count: SECTION_BYTES,
            }];
            let diagnostics = [animus_adapters::DocumentIngestDiagnostic::new(
                animus_adapters::DocumentIngestValidationIssue::InvalidField,
                "fixture",
                "safe fixture diagnostic",
            )];
            let redactions = [
                animus_adapters::DocumentRedactionLabel::ContentBounded,
                animus_adapters::DocumentRedactionLabel::PayloadHashOnly,
            ];
            let non_claims = [
                animus_adapters::DocumentIngestNonClaim::ParserSoundness,
                animus_adapters::DocumentIngestNonClaim::ExtractionCompleteness,
                animus_adapters::DocumentIngestNonClaim::OcrCoverage,
                animus_adapters::DocumentIngestNonClaim::SemanticTruth,
                animus_adapters::DocumentIngestNonClaim::EmbeddedContentSafety,
                animus_adapters::DocumentIngestNonClaim::ReleaseEligibility,
                animus_adapters::DocumentIngestNonClaim::DownstreamTaskCorrectness,
            ];
            validate_document_report_at_shell(animus_adapters::DocumentIngestReportDraft {
                source: animus_adapters::DocumentSourceIdentityDraft {
                    safe_source_label: plan.safe_source_label(),
                    source_hash: HASH,
                    observed_source_hash: HASH,
                },
                admitted_format: "markdown",
                extractor_family: "fixture-extractor",
                status: animus_adapters::DocumentIngestStatus::Completed,
                truncation: animus_adapters::DocumentTruncationStatus::None,
                content_trust: animus_adapters::DocumentContentTrust::UntrustedContext,
                input_bytes: SECTION_BYTES,
                input_byte_cap: plan.input_byte_cap(),
                output_bytes: SECTION_BYTES,
                output_byte_cap: plan.output_byte_cap(),
                document_count: DOCUMENT_COUNT,
                document_count_cap: plan.document_count_cap(),
                timeout_class: plan.timeout_class(),
                isolation: animus_adapters::DocumentIsolationStatus::RequestedAndRan,
                sections: &sections,
                table_cells: &[],
                metadata: &[],
                diagnostics: &diagnostics,
                redactions: &redactions,
                non_claims: &non_claims,
            })
        }
    }

    fn request(path: &std::path::Path) -> DocumentExtractionRequest<'_> {
        DocumentExtractionRequest {
            safe_source_label: SAFE_SOURCE,
            relative_path: path,
            input_byte_cap: INPUT_CAP,
            output_byte_cap: OUTPUT_CAP,
            document_count_cap: DOCUMENT_CAP,
            timeout_class: TIMEOUT,
            isolation: DocumentExtractionIsolationRequirement::Required,
            service_state: DocumentExtractionServiceState::Available,
        }
    }

    #[test]
    fn document_extraction_host_runs_injected_service_for_valid_plan() {
        // r[verify document-ingest-adapters.host-shell]
        let path = std::path::Path::new(RELATIVE_PATH);
        let mut host = DocumentExtractionHost::new(FixtureExtractor::default());
        let report = host
            .extract(&request(path))
            .expect("host extraction should return a validated report");

        assert_eq!(host.extractor().calls, EXPECTED_CALLS);
        assert_eq!(report.source().safe_source_label(), SAFE_SOURCE);
        assert_eq!(
            report.admitted_format(),
            animus_adapters::DocumentFormat::Markdown
        );
    }

    #[test]
    fn document_extraction_planning_fails_closed_for_missing_service_and_escape_path() {
        // r[verify document-ingest-adapters.negative]
        let path = std::path::Path::new(RELATIVE_PATH);
        let mut missing = request(path);
        missing.service_state = DocumentExtractionServiceState::Missing;
        assert!(matches!(
            plan_document_extraction(&missing),
            Err(DocumentExtractionError::MissingService)
        ));

        let escape = std::path::Path::new("../private.pdf");
        assert!(matches!(
            plan_document_extraction(&request(escape)),
            Err(DocumentExtractionError::InvalidRequest { .. })
        ));
    }
}
