//! Explicit generated-policy loader and injected session-tree artifact writer.

type Path = std::path::Path;
type SessionTreeExportArtifact = animus_runtime::session_tree_export::SessionTreeExportArtifact;
type SessionTreeExportError = animus_runtime::session_tree_export::SessionTreeExportError;
type SessionTreeExportPolicy = animus_runtime::session_tree_export::SessionTreeExportPolicy;
type SessionTreeExportPolicyInput =
    animus_runtime::session_tree_export::SessionTreeExportPolicyInput;
type SessionTreeExportWriteEvidence<'a> =
    animus_runtime::session_tree_export::SessionTreeExportWriteEvidence<'a>;

const GENERATED_POLICY_FILE: &str = "config/generated/session-tree-export-policy.json";
const PRIVATE_FILE_MODE: u32 = 0o600;
const MIN_ARTIFACT_BYTES: u32 = 1;
const MAX_CAPABILITY_LABEL_CHARS: usize = 128;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct SessionTreeExportPolicyJson {
    schema: String,
    policy_id: String,
    max_entries: u32,
    max_depth: u32,
    max_summary_chars: u32,
    max_html_bytes: u32,
    max_safe_lines_bytes: u32,
    allowed_entry_kinds: Vec<String>,
    formats: Vec<String>,
    branch_visibility: String,
    tool_metadata_posture: String,
    custom_entry_posture: String,
    receipt_fields: Vec<String>,
    content_posture: String,
    path_posture: String,
    malformed_input: String,
    scripts: String,
    remote_resources: String,
    policy_hash: String,
}

/// Generated-policy JSON error class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionTreeExportJsonErrorKind {
    /// Input was not valid JSON.
    InvalidJson,
    /// Input contained unknown or malformed fields.
    InvalidShape,
}

/// Generated-policy loading or validation error.
#[derive(Debug)]
pub enum SessionTreeExportConfigError {
    /// Explicit policy file could not be read.
    Read(std::io::Error),
    /// JSON decoding failed.
    Json {
        /// Bounded error class.
        kind: SessionTreeExportJsonErrorKind,
    },
    /// Runtime policy validation failed.
    Policy(SessionTreeExportError),
}

impl core::fmt::Display for SessionTreeExportConfigError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Read(_error) => write!(formatter, "failed to read {GENERATED_POLICY_FILE}"),
            Self::Json { kind } => write!(formatter, "session tree export JSON rejected: {kind:?}"),
            Self::Policy(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for SessionTreeExportConfigError {}

/// Parses and revalidates generated session-tree export policy JSON.
///
/// # Errors
///
/// Returns [`SessionTreeExportConfigError`] for malformed JSON, unknown fields,
/// or failed runtime policy validation.
pub fn parse_session_tree_export_policy_json(
    text: &str,
) -> Result<SessionTreeExportPolicy, SessionTreeExportConfigError> {
    let decoded: SessionTreeExportPolicyJson =
        serde_json::from_str(text).map_err(|error| classify_json_error(&error))?;
    let input = SessionTreeExportPolicyInput {
        schema: decoded.schema,
        policy_id: decoded.policy_id,
        max_entries: decoded.max_entries,
        max_depth: decoded.max_depth,
        max_summary_chars: decoded.max_summary_chars,
        max_html_bytes: decoded.max_html_bytes,
        max_safe_lines_bytes: decoded.max_safe_lines_bytes,
        allowed_entry_kinds: decoded.allowed_entry_kinds,
        formats: decoded.formats,
        branch_visibility: decoded.branch_visibility,
        tool_metadata_posture: decoded.tool_metadata_posture,
        custom_entry_posture: decoded.custom_entry_posture,
        receipt_fields: decoded.receipt_fields,
        content_posture: decoded.content_posture,
        path_posture: decoded.path_posture,
        malformed_input: decoded.malformed_input,
        scripts: decoded.scripts,
        remote_resources: decoded.remote_resources,
        policy_hash: decoded.policy_hash,
    };
    animus_runtime::session_tree_export::validate_session_tree_export_policy(&input)
        .map_err(SessionTreeExportConfigError::Policy)
}

/// Loads generated policy from an explicit caller-selected path.
///
/// # Errors
///
/// Returns [`SessionTreeExportConfigError`] for read, JSON, or policy failures.
pub fn load_session_tree_export_policy(
    path: &Path,
) -> Result<SessionTreeExportPolicy, SessionTreeExportConfigError> {
    let text = std::fs::read_to_string(path).map_err(SessionTreeExportConfigError::Read)?;
    parse_session_tree_export_policy_json(text.as_str())
}

fn classify_json_error(error: &serde_json::Error) -> SessionTreeExportConfigError {
    let kind = if error.is_syntax() || error.is_eof() {
        SessionTreeExportJsonErrorKind::InvalidJson
    } else {
        SessionTreeExportJsonErrorKind::InvalidShape
    };
    SessionTreeExportConfigError::Json { kind }
}

/// Explicit host write request.
#[derive(Clone, Copy, Debug)]
pub struct SessionTreeExportWriteRequest<'a> {
    /// Capability label resolved by the host to a destination.
    pub destination_capability: &'a str,
    /// Whether an existing destination may be replaced.
    pub allow_overwrite: bool,
    /// Requested private file mode.
    pub file_mode: u32,
    /// Maximum admitted artifact bytes.
    pub max_bytes: u32,
    /// Exact rendered artifact.
    pub artifact: &'a SessionTreeExportArtifact,
}

/// Bounded call passed to the injected writer service.
#[derive(Clone, Copy, Debug)]
pub struct SessionTreeExportWriteCall<'a> {
    /// Capability label resolved by the service.
    pub destination_capability: &'a str,
    /// Overwrite posture.
    pub allow_overwrite: bool,
    /// Exact private file mode.
    pub file_mode: u32,
    /// Exact artifact BLAKE3 identity.
    pub artifact_hash: &'a str,
    /// Artifact bytes, transient for this call.
    pub artifact_bytes: &'a [u8],
}

/// Safe writer outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionTreeExportWriteOutcome {
    /// Exact artifact BLAKE3 identity observed by the writer.
    pub artifact_hash: String,
    /// Bytes written by the service.
    pub bytes_written: u32,
    /// Whether final replacement/creation committed.
    pub committed: bool,
    /// Whether requested synchronization completed.
    pub synchronized: bool,
}

/// Injected destination-capability writer.
pub trait SessionTreeExportWriteService {
    /// Host-private writer error.
    type Error;

    /// Writes one already-admitted artifact.
    ///
    /// # Errors
    ///
    /// Returns the service's host-private error when the write cannot complete.
    fn write(
        &mut self,
        call: &SessionTreeExportWriteCall<'_>,
    ) -> Result<SessionTreeExportWriteOutcome, Self::Error>;
}

/// Host write-shell failure.
#[derive(Debug, Eq, PartialEq)]
pub enum SessionTreeExportWriteError<ServiceError> {
    /// No writer service was injected.
    MissingService,
    /// Request or service outcome violated exact admission.
    InvalidWrite,
    /// Injected service returned its host-private error.
    Service(ServiceError),
}

/// Validates write admission, invokes the injected service once, and validates
/// its exact committed outcome.
///
/// # Errors
///
/// Returns [`SessionTreeExportWriteError`] before invocation for malformed
/// admission and after invocation for service failure or mismatched outcome.
// r[impl agent_harness_runtime.session_tree_export.host_write]
pub fn execute_session_tree_export_write<Service>(
    service: Option<&mut Service>,
    request: SessionTreeExportWriteRequest<'_>,
) -> Result<SessionTreeExportWriteOutcome, SessionTreeExportWriteError<Service::Error>>
where
    Service: SessionTreeExportWriteService,
{
    validate_write_request(&request)?;
    let Some(service) = service else {
        return Err(SessionTreeExportWriteError::MissingService);
    };
    let call = SessionTreeExportWriteCall {
        destination_capability: request.destination_capability,
        allow_overwrite: request.allow_overwrite,
        file_mode: request.file_mode,
        artifact_hash: request.artifact.artifact_hash(),
        artifact_bytes: request.artifact.bytes(),
    };
    let outcome = service
        .write(&call)
        .map_err(SessionTreeExportWriteError::Service)?;
    validate_write_outcome(request.artifact, &outcome)?;
    Ok(outcome)
}

/// Projects exact validated write evidence for pure receipt construction.
///
/// # Errors
///
/// Returns [`SessionTreeExportWriteError::InvalidWrite`] unless the request and
/// outcome satisfy the same exact admission checks as the write shell.
pub fn project_session_tree_export_write_evidence<'a>(
    request: &'a SessionTreeExportWriteRequest<'a>,
    outcome: &'a SessionTreeExportWriteOutcome,
    status: &'a str,
) -> Result<
    SessionTreeExportWriteEvidence<'a>,
    SessionTreeExportWriteError<core::convert::Infallible>,
> {
    validate_write_request(request)?;
    validate_write_outcome(request.artifact, outcome)?;
    Ok(SessionTreeExportWriteEvidence {
        destination_capability: request.destination_capability,
        allow_overwrite: request.allow_overwrite,
        file_mode: request.file_mode,
        max_bytes: request.max_bytes,
        artifact_hash: outcome.artifact_hash.as_str(),
        bytes_written: outcome.bytes_written,
        committed: outcome.committed,
        synchronized: outcome.synchronized,
        status,
    })
}

fn validate_write_request<ServiceError>(
    request: &SessionTreeExportWriteRequest<'_>,
) -> Result<(), SessionTreeExportWriteError<ServiceError>> {
    let label = request.destination_capability;
    let is_label_bounded = !label.is_empty() && label.len() <= MAX_CAPABILITY_LABEL_CHARS;
    let has_safe_label_chars = label.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | ':' | '-')
    });
    let byte_count = request.artifact.byte_count();
    let is_budget_valid =
        request.max_bytes >= MIN_ARTIFACT_BYTES && byte_count <= request.max_bytes;
    if !is_label_bounded || !has_safe_label_chars {
        return Err(SessionTreeExportWriteError::InvalidWrite);
    }
    if request.file_mode != PRIVATE_FILE_MODE || !is_budget_valid {
        return Err(SessionTreeExportWriteError::InvalidWrite);
    }
    Ok(())
}

fn validate_write_outcome<ServiceError>(
    artifact: &SessionTreeExportArtifact,
    outcome: &SessionTreeExportWriteOutcome,
) -> Result<(), SessionTreeExportWriteError<ServiceError>> {
    let has_exact_identity = outcome.artifact_hash == artifact.artifact_hash();
    let has_exact_size = outcome.bytes_written == artifact.byte_count();
    if !has_exact_identity || !has_exact_size {
        return Err(SessionTreeExportWriteError::InvalidWrite);
    }
    if !outcome.committed || !outcome.synchronized {
        return Err(SessionTreeExportWriteError::InvalidWrite);
    }
    Ok(())
}

#[cfg(test)]
#[path = "sessiontreeexport/tests.rs"]
mod tests;
