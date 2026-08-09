use alloc::{format, string::String};

use super::super::GuardedAction;
use super::decision::HostAuthorityIssue;

const BLAKE3_HEX_BYTES: usize = 64;
const BLAKE3_PREFIX: &str = "b3:";
const BASALT_BLAKE3_PREFIX: &str = "blake3:";
const REQUEST_IDENTITY_DOMAIN: &[u8] = b"animus/host-authority-request/v1";
const BINDING_IDENTITY_DOMAIN: &[u8] = b"animus/agent-authority-binding/v1";
const HARD_MAX_IDENTITY_BYTES: usize = 512;
const HARD_MAX_RESOURCE_BYTES: usize = 4_096;
const HARD_MAX_CHECKED_FACTS: u32 = 4_096;
const MINIMUM_CHECKED_FACTS: u32 = 1;
const DID_PREFIX: &str = "did:";

/// Freshness posture supplied with one admitted binding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BindingFreshness {
    /// The host observed the binding under the current selected policy.
    Current,
    /// The binding is known to be stale.
    Stale,
    /// The host could not establish current freshness.
    Unavailable,
}

/// Named bounds carried by each generic authority request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostAuthorityBounds {
    /// Byte limit for one identity or safe label.
    pub identity_capacity_bytes: usize,
    /// Byte limit for the logical resource identifier.
    pub resource_ceiling_bytes: usize,
    /// Count limit for checked provider facts.
    pub checked_fact_allowance: u32,
}

impl HostAuthorityBounds {
    /// Return true when all request bounds are positive and below hard limits.
    #[must_use]
    pub const fn is_valid(self) -> bool {
        self.identity_capacity_bytes > 0
            && self.identity_capacity_bytes <= HARD_MAX_IDENTITY_BYTES
            && self.resource_ceiling_bytes > 0
            && self.resource_ceiling_bytes <= HARD_MAX_RESOURCE_BYTES
            && self.checked_fact_allowance >= MINIMUM_CHECKED_FACTS
            && self.checked_fact_allowance <= HARD_MAX_CHECKED_FACTS
    }
}

/// Caller-supplied facts for one agent-definition to UCAN-principal binding.
#[derive(Clone, Copy, Debug)]
pub struct AgentAuthorityBindingDraft<'a> {
    /// Informational display label. This field never grants authority.
    pub display_label: Option<&'a str>,
    /// UCAN audience DID used as the cryptographic principal.
    pub audience_did: &'a str,
    /// Exact admitted agent-definition BLAKE3 identity.
    pub agent_definition_identity: &'a str,
    /// Binding policy BLAKE3 identity.
    pub binding_policy_identity: &'a str,
    /// Current freshness BLAKE3 identity.
    pub freshness_identity: &'a str,
    /// Subject BLAKE3 identity.
    pub subject_identity: &'a str,
    /// Task BLAKE3 identity.
    pub task_identity: &'a str,
    /// Actor BLAKE3 identity.
    pub actor_identity: &'a str,
    /// Semantic role-plan BLAKE3 identity.
    pub role_plan_identity: &'a str,
    /// Session BLAKE3 identity.
    pub session_identity: &'a str,
    /// Host-observed freshness posture.
    pub freshness: BindingFreshness,
    /// Named limits for this binding.
    pub bounds: HostAuthorityBounds,
}

/// Validated agent-definition to UCAN-principal binding.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentAuthorityBinding {
    display_label: Option<String>,
    audience_did: String,
    agent_definition_identity: String,
    binding_policy_identity: String,
    freshness_identity: String,
    subject_identity: String,
    task_identity: String,
    actor_identity: String,
    role_plan_identity: String,
    session_identity: String,
    binding_identity: String,
    bounds: HostAuthorityBounds,
}

impl AgentAuthorityBinding {
    /// Return the non-authoritative display label.
    #[must_use]
    pub fn display_label(&self) -> Option<&str> {
        self.display_label.as_deref()
    }

    /// Return the UCAN audience DID.
    #[must_use]
    pub fn audience_did(&self) -> &str {
        self.audience_did.as_str()
    }

    /// Return the exact agent-definition identity.
    #[must_use]
    pub fn agent_definition_identity(&self) -> &str {
        self.agent_definition_identity.as_str()
    }

    /// Return the binding-policy identity.
    #[must_use]
    pub fn binding_policy_identity(&self) -> &str {
        self.binding_policy_identity.as_str()
    }

    /// Return the freshness identity.
    #[must_use]
    pub fn freshness_identity(&self) -> &str {
        self.freshness_identity.as_str()
    }

    /// Return the subject identity.
    #[must_use]
    pub fn subject_identity(&self) -> &str {
        self.subject_identity.as_str()
    }

    /// Return the task identity.
    #[must_use]
    pub fn task_identity(&self) -> &str {
        self.task_identity.as_str()
    }

    /// Return the actor identity.
    #[must_use]
    pub fn actor_identity(&self) -> &str {
        self.actor_identity.as_str()
    }

    /// Return the semantic role-plan identity.
    #[must_use]
    pub fn role_plan_identity(&self) -> &str {
        self.role_plan_identity.as_str()
    }

    /// Return the session identity.
    #[must_use]
    pub fn session_identity(&self) -> &str {
        self.session_identity.as_str()
    }

    /// Return the complete binding identity.
    #[must_use]
    pub fn binding_identity(&self) -> &str {
        self.binding_identity.as_str()
    }

    /// Return the admitted binding bounds.
    #[must_use]
    pub const fn bounds(&self) -> HostAuthorityBounds {
        self.bounds
    }
}

/// Admit one exact agent-definition to UCAN-principal binding.
///
/// # Errors
///
/// Returns a stable issue when a field, bound, DID, BLAKE3 identity, or freshness fact is unsafe.
pub fn admit_agent_authority_binding(
    draft: AgentAuthorityBindingDraft<'_>,
) -> Result<AgentAuthorityBinding, HostAuthorityIssue> {
    if !draft.bounds.is_valid() {
        return Err(HostAuthorityIssue::InvalidBound);
    }
    if draft.freshness != BindingFreshness::Current {
        return Err(HostAuthorityIssue::StaleBinding);
    }
    if !valid_did(draft.audience_did, draft.bounds.identity_capacity_bytes) {
        return Err(HostAuthorityIssue::InvalidPrincipal);
    }
    let identities = [
        draft.agent_definition_identity,
        draft.binding_policy_identity,
        draft.freshness_identity,
        draft.subject_identity,
        draft.task_identity,
        draft.actor_identity,
        draft.role_plan_identity,
        draft.session_identity,
    ];
    if identities
        .iter()
        .any(|value| !valid_blake3_identity(value, draft.bounds.identity_capacity_bytes))
    {
        return Err(HostAuthorityIssue::InvalidIdentity);
    }
    if draft.display_label.is_some_and(|value| {
        !valid_safe_label(value, draft.bounds.identity_capacity_bytes)
            || value == draft.audience_did
    }) {
        return Err(HostAuthorityIssue::UnsafeDisplayLabel);
    }
    let binding_identity = hash_fields(
        BINDING_IDENTITY_DOMAIN,
        &[
            draft.audience_did,
            draft.agent_definition_identity,
            draft.binding_policy_identity,
            draft.freshness_identity,
            draft.subject_identity,
            draft.task_identity,
            draft.actor_identity,
            draft.role_plan_identity,
            draft.session_identity,
        ],
    )?;
    Ok(AgentAuthorityBinding {
        display_label: draft.display_label.map(String::from),
        audience_did: String::from(draft.audience_did),
        agent_definition_identity: String::from(draft.agent_definition_identity),
        binding_policy_identity: String::from(draft.binding_policy_identity),
        freshness_identity: String::from(draft.freshness_identity),
        subject_identity: String::from(draft.subject_identity),
        task_identity: String::from(draft.task_identity),
        actor_identity: String::from(draft.actor_identity),
        role_plan_identity: String::from(draft.role_plan_identity),
        session_identity: String::from(draft.session_identity),
        binding_identity,
        bounds: draft.bounds,
    })
}

/// Reject duplicate or aliased admitted bindings.
///
/// # Errors
///
/// Returns [`HostAuthorityIssue::DuplicateBinding`] when an identity or principal-definition pair repeats.
pub fn validate_agent_authority_binding_set(
    bindings: &[AgentAuthorityBinding],
) -> Result<(), HostAuthorityIssue> {
    for (index, binding) in bindings.iter().enumerate() {
        if bindings[..index].iter().any(|prior| {
            prior.binding_identity == binding.binding_identity
                || prior.audience_did == binding.audience_did
                    && prior.agent_definition_identity == binding.agent_definition_identity
        }) {
            return Err(HostAuthorityIssue::DuplicateBinding);
        }
    }
    Ok(())
}

/// Caller-supplied generic request facts for one guarded action.
#[derive(Clone, Copy, Debug)]
pub struct HostAuthorityRequestDraft<'a> {
    /// Guarded action class.
    pub action: GuardedAction,
    /// Exact provider contract identifier.
    pub contract: &'a str,
    /// Logical capability resource. Physical paths are not admitted.
    pub resource: &'a str,
    /// Exact capability ability.
    pub ability: &'a str,
    /// Current admitted agent binding.
    pub binding: &'a AgentAuthorityBinding,
    /// Current agent-definition identity.
    pub agent_definition_identity: &'a str,
    /// Current subject identity.
    pub subject_identity: &'a str,
    /// Current task identity.
    pub task_identity: &'a str,
    /// Current actor identity.
    pub actor_identity: &'a str,
    /// Current semantic role-plan identity.
    pub role_plan_identity: &'a str,
    /// Current session identity.
    pub session_identity: &'a str,
    /// Selected provider-policy identity.
    pub policy_identity: &'a str,
    /// Current grant-set identity.
    pub grant_identity: &'a str,
    /// Expected provider operation-profile identity.
    pub operation_profile_identity: &'a str,
    /// Safe correlation identity.
    pub correlation_identity: &'a str,
    /// Named request limits.
    pub bounds: HostAuthorityBounds,
}

/// Validated implementation-neutral request passed to an authority provider.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HostAuthorityRequest {
    action: GuardedAction,
    contract: String,
    resource: String,
    ability: String,
    audience_did: String,
    agent_definition_identity: String,
    binding_identity: String,
    binding_policy_identity: String,
    freshness_identity: String,
    subject_identity: String,
    task_identity: String,
    actor_identity: String,
    role_plan_identity: String,
    session_identity: String,
    policy_identity: String,
    grant_identity: String,
    operation_profile_identity: String,
    correlation_identity: String,
    request_identity: String,
    bounds: HostAuthorityBounds,
}

impl HostAuthorityRequest {
    /// Return the guarded action class.
    #[must_use]
    pub const fn action(&self) -> GuardedAction {
        self.action
    }
    /// Return the provider contract.
    #[must_use]
    pub fn contract(&self) -> &str {
        self.contract.as_str()
    }
    /// Return the logical resource.
    #[must_use]
    pub fn resource(&self) -> &str {
        self.resource.as_str()
    }
    /// Return the requested ability.
    #[must_use]
    pub fn ability(&self) -> &str {
        self.ability.as_str()
    }
    /// Return the UCAN audience DID.
    #[must_use]
    pub fn audience_did(&self) -> &str {
        self.audience_did.as_str()
    }
    /// Return the agent-definition identity.
    #[must_use]
    pub fn agent_definition_identity(&self) -> &str {
        self.agent_definition_identity.as_str()
    }
    /// Return the binding identity.
    #[must_use]
    pub fn binding_identity(&self) -> &str {
        self.binding_identity.as_str()
    }
    /// Return the binding policy identity.
    #[must_use]
    pub fn binding_policy_identity(&self) -> &str {
        self.binding_policy_identity.as_str()
    }
    /// Return the freshness identity.
    #[must_use]
    pub fn freshness_identity(&self) -> &str {
        self.freshness_identity.as_str()
    }
    /// Return the subject identity.
    #[must_use]
    pub fn subject_identity(&self) -> &str {
        self.subject_identity.as_str()
    }
    /// Return the task identity.
    #[must_use]
    pub fn task_identity(&self) -> &str {
        self.task_identity.as_str()
    }
    /// Return the actor identity.
    #[must_use]
    pub fn actor_identity(&self) -> &str {
        self.actor_identity.as_str()
    }
    /// Return the role-plan identity.
    #[must_use]
    pub fn role_plan_identity(&self) -> &str {
        self.role_plan_identity.as_str()
    }
    /// Return the session identity.
    #[must_use]
    pub fn session_identity(&self) -> &str {
        self.session_identity.as_str()
    }
    /// Return the selected provider-policy identity.
    #[must_use]
    pub fn policy_identity(&self) -> &str {
        self.policy_identity.as_str()
    }
    /// Return the grant-set identity.
    #[must_use]
    pub fn grant_identity(&self) -> &str {
        self.grant_identity.as_str()
    }
    /// Return the expected operation-profile identity.
    #[must_use]
    pub fn operation_profile_identity(&self) -> &str {
        self.operation_profile_identity.as_str()
    }
    /// Return the correlation identity.
    #[must_use]
    pub fn correlation_identity(&self) -> &str {
        self.correlation_identity.as_str()
    }
    /// Return the complete generic request identity.
    #[must_use]
    pub fn request_identity(&self) -> &str {
        self.request_identity.as_str()
    }
    /// Return the named request bounds.
    #[must_use]
    pub const fn bounds(&self) -> HostAuthorityBounds {
        self.bounds
    }
}

/// Validate and bind one implementation-neutral authority request.
///
/// # Errors
///
/// Returns a stable issue before any provider or host effect is called.
pub fn admit_host_authority_request(
    draft: HostAuthorityRequestDraft<'_>,
) -> Result<HostAuthorityRequest, HostAuthorityIssue> {
    if !draft.bounds.is_valid() || draft.bounds != draft.binding.bounds {
        return Err(HostAuthorityIssue::InvalidBound);
    }
    if !valid_safe_label(draft.contract, draft.bounds.identity_capacity_bytes)
        || !valid_safe_label(draft.ability, draft.bounds.identity_capacity_bytes)
        || !valid_safe_label(
            draft.correlation_identity,
            draft.bounds.identity_capacity_bytes,
        )
    {
        return Err(HostAuthorityIssue::InvalidActionBinding);
    }
    if !valid_resource(draft.resource, draft.bounds.resource_ceiling_bytes) {
        return Err(HostAuthorityIssue::InvalidResource);
    }
    for identity in [
        draft.policy_identity,
        draft.grant_identity,
        draft.operation_profile_identity,
    ] {
        if !valid_blake3_identity(identity, draft.bounds.identity_capacity_bytes) {
            return Err(HostAuthorityIssue::InvalidIdentity);
        }
    }
    if draft.agent_definition_identity != draft.binding.agent_definition_identity
        || draft.subject_identity != draft.binding.subject_identity
        || draft.task_identity != draft.binding.task_identity
        || draft.actor_identity != draft.binding.actor_identity
        || draft.role_plan_identity != draft.binding.role_plan_identity
        || draft.session_identity != draft.binding.session_identity
    {
        return Err(HostAuthorityIssue::BindingMismatch);
    }
    let request_identity = hash_fields(
        REQUEST_IDENTITY_DOMAIN,
        &[
            draft.action.as_str(),
            draft.contract,
            draft.resource,
            draft.ability,
            draft.binding.audience_did.as_str(),
            draft.binding.binding_identity.as_str(),
            draft.agent_definition_identity,
            draft.subject_identity,
            draft.task_identity,
            draft.actor_identity,
            draft.role_plan_identity,
            draft.session_identity,
            draft.policy_identity,
            draft.grant_identity,
            draft.operation_profile_identity,
            draft.correlation_identity,
        ],
    )?;
    Ok(HostAuthorityRequest {
        action: draft.action,
        contract: String::from(draft.contract),
        resource: String::from(draft.resource),
        ability: String::from(draft.ability),
        audience_did: draft.binding.audience_did.clone(),
        agent_definition_identity: String::from(draft.agent_definition_identity),
        binding_identity: draft.binding.binding_identity.clone(),
        binding_policy_identity: draft.binding.binding_policy_identity.clone(),
        freshness_identity: draft.binding.freshness_identity.clone(),
        subject_identity: String::from(draft.subject_identity),
        task_identity: String::from(draft.task_identity),
        actor_identity: String::from(draft.actor_identity),
        role_plan_identity: String::from(draft.role_plan_identity),
        session_identity: String::from(draft.session_identity),
        policy_identity: String::from(draft.policy_identity),
        grant_identity: String::from(draft.grant_identity),
        operation_profile_identity: String::from(draft.operation_profile_identity),
        correlation_identity: String::from(draft.correlation_identity),
        request_identity,
        bounds: draft.bounds,
    })
}

fn valid_blake3_identity(value: &str, max_bytes: usize) -> bool {
    let hex = value
        .strip_prefix(BLAKE3_PREFIX)
        .or_else(|| value.strip_prefix(BASALT_BLAKE3_PREFIX));
    !value.is_empty()
        && value.len() <= max_bytes
        && hex.is_some_and(|hex| {
            hex.len() == BLAKE3_HEX_BYTES
                && hex
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        })
}

fn valid_did(value: &str, max_bytes: usize) -> bool {
    value.starts_with(DID_PREFIX)
        && valid_safe_label(value, max_bytes)
        && !value.contains('*')
        && !value.contains(char::is_whitespace)
}

fn valid_safe_label(value: &str, max_bytes: usize) -> bool {
    !value.is_empty()
        && value.len() <= max_bytes
        && value.trim() == value
        && !value.contains('*')
        && !value.chars().any(char::is_control)
}

fn valid_resource(value: &str, max_bytes: usize) -> bool {
    valid_safe_label(value, max_bytes)
        && !value.starts_with('/')
        && !value.starts_with('~')
        && !value.contains("../")
        && !value.contains("..\\")
        && !value.contains("file://")
}

fn hash_fields(domain: &[u8], fields: &[&str]) -> Result<String, HostAuthorityIssue> {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain);
    for field in fields {
        let length = u64::try_from(field.len()).map_err(|_| HostAuthorityIssue::InvalidBound)?;
        hasher.update(&length.to_be_bytes());
        hasher.update(field.as_bytes());
    }
    Ok(format!("{BLAKE3_PREFIX}{}", hasher.finalize().to_hex()))
}
