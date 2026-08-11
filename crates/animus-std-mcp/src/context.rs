use crate::config::{framed_hash, validate_safe_label};

const INSTRUCTION_DOMAIN: &str = "animus.mcp-client-host.instructions.v1";
const INSTRUCTION_LABEL: &str = "mcp-server-instructions";
const CONTRIBUTION_PREFIX: &str = "mcp-instructions";

/// Request-local server instructions plus existing Animus contribution metadata.
pub struct InstructionContribution {
    body: String,
    contribution: animus_runtime::request_context::Contribution,
    payload_identity: String,
}

impl InstructionContribution {
    /// Returns the request-local body for immediate prompt assembly only.
    #[must_use]
    pub fn body(&self) -> &str {
        self.body.as_str()
    }

    /// Returns the admitted existing request-context contribution.
    #[must_use]
    pub const fn contribution(&self) -> &animus_runtime::request_context::Contribution {
        &self.contribution
    }

    /// Returns the body identity without exposing the body in evidence.
    #[must_use]
    pub fn payload_identity(&self) -> &str {
        self.payload_identity.as_str()
    }
}

/// Bounded server-instruction admission failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InstructionContributionError {
    /// Server identity or instruction encoding was malformed.
    InvalidInput,
    /// Instruction bytes exceeded the current-request bound.
    BudgetExceeded,
    /// Existing request-context admission rejected the contribution.
    ContextRejected,
}

/// Admits server instructions only as untrusted advisory current-request context.
///
/// # Errors
///
/// Returns a bounded error without copying instructions into receipts or durable state.
// r[impl agent_harness_runtime.mcp_client_host.context]
pub fn admit_instructions(
    server_id: &str,
    generation: u64,
    instructions: String,
    max_instruction_bytes: usize,
) -> Result<InstructionContribution, InstructionContributionError> {
    validate_safe_label(server_id).map_err(|_| InstructionContributionError::InvalidInput)?;
    if generation == 0 || instructions.contains('\0') {
        return Err(InstructionContributionError::InvalidInput);
    }
    if instructions.is_empty() || instructions.len() > max_instruction_bytes {
        return Err(InstructionContributionError::BudgetExceeded);
    }
    let budget_cost = u32::try_from(instructions.len())
        .map_err(|_| InstructionContributionError::BudgetExceeded)?;
    let payload_identity = framed_hash(INSTRUCTION_DOMAIN, instructions.as_bytes());
    let contribution_id = format!("{CONTRIBUTION_PREFIX}-{server_id}-{generation}");
    let contribution = animus_runtime::request_context::contribution_from_owner(
        animus_runtime::request_context::ContributionDraft {
            contribution_id: contribution_id.as_str(),
            source: animus_runtime::request_context::ContributionSource::Mcp,
            trust: animus_runtime::request_context::ContributionTrust::Untrusted,
            posture: animus_runtime::request_context::ContributionPosture::Advisory,
            priority: 0,
            budget_cost,
            payload_identity: payload_identity.as_str(),
            deduplication_key: payload_identity.as_str(),
            supersedes: &[],
            conflict_group: None,
            lifetime: animus_runtime::request_context::ContributionLifetime::CurrentRequest,
            sensitivity: animus_runtime::request_context::ContributionSensitivity::RequestLocal,
            safe_label: INSTRUCTION_LABEL,
        },
    )
    .map_err(|_| InstructionContributionError::ContextRejected)?;
    Ok(InstructionContribution {
        body: instructions,
        contribution,
        payload_identity,
    })
}
