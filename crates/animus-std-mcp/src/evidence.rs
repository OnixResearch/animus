use std::collections::BTreeSet;

use crate::config::{REQUIRED_NON_CLAIMS, framed_hash, validate_hash, validate_safe_label};
use crate::{LifecyclePhase, TeardownState};

const RECEIPT_DOMAIN: &str = "animus.mcp-client-host.receipt.v1";
const MAX_RECEIPT_CAPABILITIES: usize = 64;

/// Sensitive or overclaim field detected before receipt projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptLeakClass {
    /// No sensitive field is present.
    None,
    /// Raw JSON-RPC frame.
    RawFrame,
    /// Raw tool arguments.
    Arguments,
    /// Raw tool result.
    Result,
    /// Server instruction body.
    Instructions,
    /// Credential material.
    Credential,
    /// Environment value.
    EnvironmentValue,
    /// Executable or socket path.
    PrivatePath,
    /// Provider data.
    ProviderData,
    /// Unrestricted error text.
    UnrestrictedError,
    /// Unsupported trust or readiness claim.
    Overclaim,
}

/// Bounded cancellation metadata for one receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CancellationReceiptState {
    /// No cancellation was requested.
    NotRequested,
    /// A current cancellation request was observed.
    Requested,
    /// Cancellation reached terminal process teardown.
    Terminal,
}

/// Borrowed metadata-only MCP host receipt facts.
#[derive(Clone, Copy)]
pub struct McpHostReceiptDraft<'a> {
    /// Exact server identity.
    pub server_id: &'a str,
    /// Exact validated configuration identity.
    pub configuration_id: &'a str,
    /// Exact protocol cohort.
    pub protocol_id: &'a str,
    /// Exact server generation.
    pub generation: u64,
    /// Exact process-attempt identity.
    pub process_attempt_id: &'a str,
    /// Exact tool name, when this is a call receipt.
    pub tool_name: Option<&'a str>,
    /// Exact tool schema identity, when this is a call receipt.
    pub schema_identity: Option<&'a str>,
    /// Current capability labels.
    pub capability_labels: &'a [&'a str],
    /// Exact approval identity, when applicable.
    pub approval_identity: Option<&'a str>,
    /// Exact call identity, when applicable.
    pub call_identity: Option<&'a str>,
    /// Exact output-bound identity, when applicable.
    pub output_bound_identity: Option<&'a str>,
    /// Exact output identity, when applicable.
    pub output_identity: Option<&'a str>,
    /// Bounded cancellation state.
    pub cancellation: CancellationReceiptState,
    /// Current lifecycle phase.
    pub lifecycle: LifecyclePhase,
    /// Current teardown classification.
    pub teardown: TeardownState,
    /// Bounded status label.
    pub status: &'a str,
    /// Mandatory redaction policy identity.
    pub redaction_identity: &'a str,
    /// Mandatory non-claims.
    pub non_claims: &'a [&'a str],
    /// Sensitive or overclaim candidate classification.
    pub leak: ReceiptLeakClass,
}

/// Metadata-only MCP host receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpHostReceipt {
    identity: String,
    server_id: String,
    configuration_id: String,
    protocol_id: String,
    generation: u64,
    process_attempt_id: String,
    tool_name: Option<String>,
    schema_identity: Option<String>,
    capability_count: usize,
    approval_identity: Option<String>,
    call_identity: Option<String>,
    output_bound_identity: Option<String>,
    output_identity: Option<String>,
    cancellation: CancellationReceiptState,
    lifecycle: LifecyclePhase,
    teardown: TeardownState,
    status: String,
    redaction_identity: String,
    non_claims: Vec<String>,
}

impl McpHostReceipt {
    /// Returns the receipt identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the server identity.
    #[must_use]
    pub fn server_id(&self) -> &str {
        self.server_id.as_str()
    }

    /// Returns the server generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns the exact approval identity, when applicable.
    #[must_use]
    pub fn approval_identity(&self) -> Option<&str> {
        self.approval_identity.as_deref()
    }

    /// Returns the exact call identity, when applicable.
    #[must_use]
    pub fn call_identity(&self) -> Option<&str> {
        self.call_identity.as_deref()
    }

    /// Returns the output-bound identity, when applicable.
    #[must_use]
    pub fn output_bound_identity(&self) -> Option<&str> {
        self.output_bound_identity.as_deref()
    }

    /// Returns the bounded output identity, when applicable.
    #[must_use]
    pub fn output_identity(&self) -> Option<&str> {
        self.output_identity.as_deref()
    }

    /// Returns the bounded cancellation state.
    #[must_use]
    pub const fn cancellation(&self) -> CancellationReceiptState {
        self.cancellation
    }

    /// Returns the redaction policy identity.
    #[must_use]
    pub fn redaction_identity(&self) -> &str {
        self.redaction_identity.as_str()
    }

    /// Returns the lifecycle phase.
    #[must_use]
    pub const fn lifecycle(&self) -> LifecyclePhase {
        self.lifecycle
    }

    /// Returns the teardown classification.
    #[must_use]
    pub const fn teardown(&self) -> TeardownState {
        self.teardown
    }

    /// Returns the capability count without exposing grants.
    #[must_use]
    pub const fn capability_count(&self) -> usize {
        self.capability_count
    }

    /// Returns mandatory non-claims.
    #[must_use]
    pub fn non_claims(&self) -> &[String] {
        self.non_claims.as_slice()
    }
}

/// Bounded receipt projection failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReceiptError {
    /// A sensitive or unrestricted field was supplied.
    Leak,
    /// A required safe identity was malformed.
    InvalidIdentity,
    /// A capability list was duplicated or over budget.
    InvalidCapabilities,
    /// A mandatory non-claim was absent.
    MissingNonClaim,
}

/// Projects one metadata-only MCP host receipt.
///
/// # Errors
///
/// Returns a bounded error without echoing sensitive candidate values.
// r[impl agent_harness_runtime.mcp_client_host.evidence]
pub fn project_receipt(draft: McpHostReceiptDraft<'_>) -> Result<McpHostReceipt, ReceiptError> {
    if draft.leak != ReceiptLeakClass::None {
        return Err(ReceiptError::Leak);
    }
    validate_draft_identities(&draft)?;
    let capabilities = validate_capabilities(draft.capability_labels)?;
    let non_claims = validate_non_claims(draft.non_claims)?;
    let identity = receipt_identity(&draft, capabilities.len());
    Ok(McpHostReceipt {
        identity,
        server_id: String::from(draft.server_id),
        configuration_id: String::from(draft.configuration_id),
        protocol_id: String::from(draft.protocol_id),
        generation: draft.generation,
        process_attempt_id: String::from(draft.process_attempt_id),
        tool_name: draft.tool_name.map(String::from),
        schema_identity: draft.schema_identity.map(String::from),
        capability_count: capabilities.len(),
        approval_identity: draft.approval_identity.map(String::from),
        call_identity: draft.call_identity.map(String::from),
        output_bound_identity: draft.output_bound_identity.map(String::from),
        output_identity: draft.output_identity.map(String::from),
        cancellation: draft.cancellation,
        lifecycle: draft.lifecycle,
        teardown: draft.teardown,
        status: String::from(draft.status),
        redaction_identity: String::from(draft.redaction_identity),
        non_claims,
    })
}

fn validate_draft_identities(draft: &McpHostReceiptDraft<'_>) -> Result<(), ReceiptError> {
    validate_safe_label(draft.server_id).map_err(|_| ReceiptError::InvalidIdentity)?;
    validate_safe_label(draft.protocol_id).map_err(|_| ReceiptError::InvalidIdentity)?;
    validate_safe_label(draft.status).map_err(|_| ReceiptError::InvalidIdentity)?;
    for hash in [
        Some(draft.configuration_id),
        Some(draft.process_attempt_id),
        draft.schema_identity,
        draft.approval_identity,
        draft.call_identity,
        draft.output_bound_identity,
        draft.output_identity,
        Some(draft.redaction_identity),
    ]
    .into_iter()
    .flatten()
    {
        validate_hash(hash).map_err(|_| ReceiptError::InvalidIdentity)?;
    }
    if let Some(tool_name) = draft.tool_name {
        validate_safe_label(tool_name).map_err(|_| ReceiptError::InvalidIdentity)?;
    }
    if draft.generation == 0 {
        return Err(ReceiptError::InvalidIdentity);
    }
    Ok(())
}

fn validate_capabilities(values: &[&str]) -> Result<BTreeSet<String>, ReceiptError> {
    if values.len() > MAX_RECEIPT_CAPABILITIES {
        return Err(ReceiptError::InvalidCapabilities);
    }
    let mut capabilities = BTreeSet::new();
    for value in values {
        validate_safe_label(value).map_err(|_| ReceiptError::InvalidIdentity)?;
        if !capabilities.insert(String::from(*value)) {
            return Err(ReceiptError::InvalidCapabilities);
        }
    }
    Ok(capabilities)
}

fn validate_non_claims(values: &[&str]) -> Result<Vec<String>, ReceiptError> {
    let supplied = values.iter().copied().collect::<BTreeSet<_>>();
    if REQUIRED_NON_CLAIMS
        .iter()
        .any(|required| !supplied.contains(required))
    {
        return Err(ReceiptError::MissingNonClaim);
    }
    Ok(supplied.into_iter().map(String::from).collect())
}

fn receipt_identity(draft: &McpHostReceiptDraft<'_>, capability_count: usize) -> String {
    let mut bytes = Vec::new();
    for value in [
        Some(draft.server_id),
        Some(draft.configuration_id),
        Some(draft.protocol_id),
        Some(draft.process_attempt_id),
        draft.tool_name,
        draft.schema_identity,
        draft.approval_identity,
        draft.call_identity,
        draft.output_bound_identity,
        draft.output_identity,
        Some(draft.status),
        Some(draft.redaction_identity),
    ]
    .into_iter()
    .flatten()
    {
        append_framed(&mut bytes, value.as_bytes());
    }
    bytes.extend_from_slice(&draft.generation.to_le_bytes());
    bytes.extend_from_slice(&(capability_count as u64).to_le_bytes());
    bytes.push(draft.cancellation as u8);
    bytes.push(draft.lifecycle as u8);
    bytes.push(draft.teardown as u8);
    framed_hash(RECEIPT_DOMAIN, bytes.as_slice())
}

fn append_framed(output: &mut Vec<u8>, value: &[u8]) {
    output.extend_from_slice(&(value.len() as u64).to_le_bytes());
    output.extend_from_slice(value);
}
