use animus_adapters::{GuardedAction, HostAuthorityIssue, HostAuthorityRequest};
use basalt::EnforcementRequest;
use basalt::ucan_enforcement::{EnforcementAction, UcanEnforcementReason};

/// Reviewed Basalt source revision used by this adapter.
pub const BASALT_SOURCE_REVISION: &str = "b10a0465bc3ee966a4864d6e779184f4eae0cd61";
/// Reviewed vendored UCAN source revision used by Basalt.
pub const BASALT_UCAN_REVISION: &str = "3a2aa043068a82663cf72ec05ed5d56a7d65ecd5";
/// BLAKE3 identity of Basalt's checked generated policy artifact.
pub const BASALT_POLICY_IDENTITY: &str =
    "b3:5475d5473ec98cbf052a83512836b0739f5dad3e7b9b0ed201eb3094b1cf39bf";
/// BLAKE3 identity of Basalt's reviewed opt-in process execution policy.
/// Execution-purpose admission requires this identity, not the default policy.
pub const BASALT_PROCESS_EXECUTION_POLICY_IDENTITY: &str =
    "b3:c90838cfc1f3c8f5f926f9ee98a05d203d2c0dc2199657514537c37bff3d8c7d";
/// BLAKE3 identity of the reviewed vendored UCAN support catalog.
pub const BASALT_UCAN_CATALOG_IDENTITY: &str =
    "b3:fe77fd63076eaef555b74e702e4c14e483bdba70151ccc955c75b17d97412427";
/// License identifier that downstream adopters must review explicitly.
pub const BASALT_LICENSE: &str = "AGPL-3.0-or-later";

const DOCUMENT_READ_CONTRACT: &str = "document-read";
const DOCUMENT_READ_ABILITY: &str = "document/read";
const DOCUMENT_WRITE_CONTRACT: &str = "document-write";
const DOCUMENT_WRITE_ABILITY: &str = "document/write";
const PROCESS_EXECUTE_CONTRACT: &str = "process-execute";
const PROCESS_EXECUTE_ABILITY: &str = "process/execute";
const PROCESS_EXECUTE_RESOURCE_PREFIX: &str = "onix://process-execution/";

/// Pure exact translation from Animus vocabulary to Basalt vocabulary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BasaltRequestPlan {
    /// Closed reviewed Basalt action.
    pub action: EnforcementAction,
    /// Exact Basalt enforcement request.
    pub enforcement: EnforcementRequest,
    /// Expected BLAKE3 operation-profile identity.
    pub operation_profile_identity: String,
}

/// Translate one generic request through the closed supported-action table.
///
/// # Errors
///
/// Returns [`HostAuthorityIssue::UnsupportedAction`] unless the action, contract,
/// ability, and operation-profile identity match a reviewed mapping exactly.
pub fn plan_basalt_request(
    request: &HostAuthorityRequest,
) -> Result<BasaltRequestPlan, HostAuthorityIssue> {
    let action = match (request.action(), request.contract(), request.ability()) {
        (GuardedAction::Filesystem, DOCUMENT_READ_CONTRACT, DOCUMENT_READ_ABILITY) => {
            EnforcementAction::DocumentRead
        }
        (GuardedAction::Filesystem, DOCUMENT_WRITE_CONTRACT, DOCUMENT_WRITE_ABILITY) => {
            EnforcementAction::DocumentWrite
        }
        (GuardedAction::Exec, PROCESS_EXECUTE_CONTRACT, PROCESS_EXECUTE_ABILITY) => {
            EnforcementAction::ProcessExecute
        }
        _ => return Err(HostAuthorityIssue::UnsupportedAction),
    };
    if action == EnforcementAction::ProcessExecute
        && !request
            .resource()
            .starts_with(PROCESS_EXECUTE_RESOURCE_PREFIX)
    {
        return Err(HostAuthorityIssue::UnsupportedAction);
    }
    let expected_policy_identity = required_policy_identity(action);
    if request.policy_identity() != expected_policy_identity {
        return Err(HostAuthorityIssue::UnsupportedAction);
    }
    let operation_profile_identity = operation_profile_identity(action);
    if request.operation_profile_identity() != operation_profile_identity {
        return Err(HostAuthorityIssue::UnsupportedAction);
    }
    Ok(BasaltRequestPlan {
        action,
        enforcement: EnforcementRequest::new(
            request.contract(),
            request.resource(),
            request.ability(),
        ),
        operation_profile_identity,
    })
}

/// Return the reviewed policy identity one action must select.
#[must_use]
pub const fn required_policy_identity(action: EnforcementAction) -> &'static str {
    match action {
        EnforcementAction::ProcessExecute => BASALT_PROCESS_EXECUTION_POLICY_IDENTITY,
        _ => BASALT_POLICY_IDENTITY,
    }
}

/// Return the exact generic BLAKE3 identity for one Basalt operation profile.
#[must_use]
pub fn operation_profile_identity(action: EnforcementAction) -> String {
    let digest = *action.operation_profile_id().as_bytes();
    format!("b3:{}", blake3::Hash::from_bytes(digest).to_hex())
}

/// Return a stable provider identity bound to the reviewed Basalt revision.
#[must_use]
pub fn basalt_provider_identity() -> String {
    domain_hash(
        b"animus/basalt-provider/v1",
        BASALT_SOURCE_REVISION.as_bytes(),
    )
}

/// Return a redacted metadata hash for one principal DID.
#[must_use]
pub fn principal_hash(audience_did: &str) -> String {
    domain_hash(b"animus/basalt-principal/v1", audience_did.as_bytes())
}

/// Return a redacted metadata hash for one logical resource.
#[must_use]
pub fn resource_hash(resource: &str) -> String {
    domain_hash(b"animus/basalt-resource/v1", resource.as_bytes())
}

/// Convert a Basalt policy reference into the generic identity form.
#[must_use]
pub fn normalize_basalt_identity(identity: &str) -> Option<String> {
    if let Some(hex) = identity.strip_prefix("blake3:") {
        return Some(format!("b3:{hex}"));
    }
    identity.starts_with("b3:").then(|| String::from(identity))
}

/// Map one Basalt denial into the stable generic issue vocabulary.
#[must_use]
pub const fn map_basalt_reason(reason: UcanEnforcementReason) -> HostAuthorityIssue {
    match reason {
        UcanEnforcementReason::TimeDenied => HostAuthorityIssue::StaleVerification,
        UcanEnforcementReason::ReplayDenied => HostAuthorityIssue::ReplayDenied,
        UcanEnforcementReason::OperationMismatch | UcanEnforcementReason::ProtocolProfileDenied => {
            HostAuthorityIssue::UnsupportedAction
        }
        UcanEnforcementReason::PolicyDenied => HostAuthorityIssue::PolicyDenied,
        UcanEnforcementReason::BackendUnavailable
        | UcanEnforcementReason::InternalAuthorityDenied => HostAuthorityIssue::ProviderUnavailable,
        UcanEnforcementReason::Allowed => HostAuthorityIssue::DecisionMismatch,
        UcanEnforcementReason::OversizedCredential
        | UcanEnforcementReason::MalformedCredential
        | UcanEnforcementReason::SignatureDenied
        | UcanEnforcementReason::KeyResolutionDenied
        | UcanEnforcementReason::ProofDenied
        | UcanEnforcementReason::CaveatDenied
        | UcanEnforcementReason::ScopeDenied
        | UcanEnforcementReason::Revoked
        | UcanEnforcementReason::HolderDenied
        | UcanEnforcementReason::LimitExceeded
        | UcanEnforcementReason::UnverifiedAuthorityFields => {
            HostAuthorityIssue::VerificationDenied
        }
    }
}

fn domain_hash(domain: &[u8], value: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain);
    hasher.update(&(value.len() as u64).to_be_bytes());
    hasher.update(value);
    format!("b3:{}", hasher.finalize().to_hex())
}
