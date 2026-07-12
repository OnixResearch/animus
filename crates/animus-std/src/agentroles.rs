//! Host-owned loading for generated capability-bound semantic agent roles.

use animus_runtime::agent_roles::{
    AgentRoleBudgetPolicyDraft, AgentRoleCatalog, AgentRoleCatalogDraft, AgentRoleClass,
    AgentRoleContextPosture, AgentRoleEvidencePosture, AgentRoleIssue, AgentRoleMutationPosture,
    AgentRoleOutputKind, AgentRoleProfileDraft, AgentRoleSeparationDimension,
    AgentRoleSeparationPolicyDraft, AgentRoleToolsetDraft,
};
use animus_runtime::{
    ModelProfileAdmissionStatus, ModelProfileSourceScope, ModelProfileTrustStatus,
};
use serde::Deserialize;
use std::path::Path;

/// Stable JSON failure category for generated semantic role configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentRoleJsonErrorKind {
    /// JSON syntax was malformed.
    Syntax,
    /// JSON values did not match the exact generated DTO shape.
    Data,
    /// JSON ended before a complete value was available.
    EndOfInput,
}

/// Safe host-shell error for loading generated semantic role profiles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AgentRoleConfigError {
    /// The configured file could not be read.
    Read(std::io::ErrorKind),
    /// The configured bytes were not an exact generated DTO.
    Json(AgentRoleJsonErrorKind),
    /// A generated enum discriminant was unsupported.
    UnsupportedValue,
    /// Runtime validation rejected the owned catalog.
    Validation(AgentRoleIssue),
}

/// Parse and validate generated role JSON without filesystem or environment access.
///
/// # Errors
///
/// Returns a bounded error class when JSON shape, enum values, canonical identities,
/// trust/admission state, references, separation policy, or class invariants fail.
pub fn parse_agent_role_profiles_json(
    input: &str,
) -> Result<AgentRoleCatalog, AgentRoleConfigError> {
    let generated: GeneratedCatalog =
        serde_json::from_str(input).map_err(|error| classify_json_error(&error))?;
    validate_generated(&generated)
}

/// Read generated role JSON from an explicit host path and validate it.
///
/// Runtime code never evaluates Nickel or discovers ambient configuration paths.
///
/// # Errors
///
/// Returns a bounded read, JSON, unsupported-value, or runtime-validation error.
pub fn load_agent_role_profiles(
    path: impl AsRef<Path>,
) -> Result<AgentRoleCatalog, AgentRoleConfigError> {
    let input = std::fs::read_to_string(path.as_ref())
        .map_err(|error| AgentRoleConfigError::Read(error.kind()))?;
    parse_agent_role_profiles_json(input.as_str())
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedCatalog {
    schema: String,
    known_capabilities: Vec<String>,
    input_contracts: Vec<String>,
    output_contracts: Vec<String>,
    evidence_contracts: Vec<String>,
    budget_policies: Vec<GeneratedBudget>,
    toolsets: Vec<GeneratedToolset>,
    separation_policies: Vec<GeneratedSeparationPolicy>,
    profiles: Vec<GeneratedProfile>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedBudget {
    policy_id: String,
    max_actions: u32,
    max_outputs: u16,
    max_context_units: u32,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedToolset {
    toolset_id: String,
    tool_names: Vec<String>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
#[serde(transparent)]
struct RequirementFlag(bool);

impl RequirementFlag {
    const fn is_required(self) -> bool {
        self.0
    }
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedSeparationPolicy {
    policy_id: String,
    require_distinct_assignment: RequirementFlag,
    require_distinct_actor: RequirementFlag,
    require_distinct_run_thread: RequirementFlag,
    require_distinct_session: RequirementFlag,
    require_distinct_model_profile: RequirementFlag,
    require_distinct_workspace: RequirementFlag,
    require_distinct_mutation_lineage: RequirementFlag,
    require_clean_context: RequirementFlag,
    require_subject_immutable: RequirementFlag,
    require_verifier_immutable: RequirementFlag,
    require_rubric_immutable: RequirementFlag,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedProfile {
    role_id: String,
    class: String,
    purpose: String,
    source_scope: String,
    trust_status: String,
    admission_status: String,
    project_profile_admitted: bool,
    required_capabilities: Vec<String>,
    capability_ceiling: Vec<String>,
    eligible_model_profiles: Vec<String>,
    model_targets: Vec<String>,
    prompt_modes: Vec<String>,
    prompt_sources: Vec<String>,
    skills: Vec<String>,
    memory_policy_hashes: Vec<String>,
    tools: Vec<String>,
    toolsets: Vec<String>,
    tool_capabilities: Vec<String>,
    workspace_scopes: Vec<String>,
    mutation_posture: String,
    context_posture: String,
    evidence_posture: String,
    output_kind: String,
    input_contract: String,
    output_contract: String,
    evidence_contract: String,
    separation_policy_ref: String,
    budget_policy_ref: String,
    policy_hash: String,
    profile_hash: String,
}

fn classify_json_error(error: &serde_json::Error) -> AgentRoleConfigError {
    let kind = match error.classify() {
        serde_json::error::Category::Syntax => AgentRoleJsonErrorKind::Syntax,
        serde_json::error::Category::Data | serde_json::error::Category::Io => {
            AgentRoleJsonErrorKind::Data
        }
        serde_json::error::Category::Eof => AgentRoleJsonErrorKind::EndOfInput,
    };
    AgentRoleConfigError::Json(kind)
}

fn validate_generated(
    generated: &GeneratedCatalog,
) -> Result<AgentRoleCatalog, AgentRoleConfigError> {
    let known_capabilities = string_refs(generated.known_capabilities.as_slice());
    let input_contracts = string_refs(generated.input_contracts.as_slice());
    let output_contracts = string_refs(generated.output_contracts.as_slice());
    let evidence_contracts = string_refs(generated.evidence_contracts.as_slice());
    let budgets = generated
        .budget_policies
        .iter()
        .map(|budget| AgentRoleBudgetPolicyDraft {
            policy_id: budget.policy_id.as_str(),
            max_actions: budget.max_actions,
            max_outputs: budget.max_outputs,
            max_context_units: budget.max_context_units,
        })
        .collect::<Vec<_>>();
    let tool_names = generated
        .toolsets
        .iter()
        .map(|toolset| string_refs(toolset.tool_names.as_slice()))
        .collect::<Vec<_>>();
    let toolsets = generated
        .toolsets
        .iter()
        .enumerate()
        .map(|(index, toolset)| AgentRoleToolsetDraft {
            toolset_id: toolset.toolset_id.as_str(),
            tool_names: tool_names[index].as_slice(),
        })
        .collect::<Vec<_>>();
    let separation_dimensions = generated
        .separation_policies
        .iter()
        .map(generated_separation_dimensions)
        .collect::<Vec<_>>();
    let separation_policies = generated
        .separation_policies
        .iter()
        .enumerate()
        .map(|(index, policy)| AgentRoleSeparationPolicyDraft {
            policy_id: policy.policy_id.as_str(),
            required_dimensions: separation_dimensions[index].as_slice(),
        })
        .collect::<Vec<_>>();
    let resources = generated
        .profiles
        .iter()
        .map(GeneratedProfileResources::from_profile)
        .collect::<Vec<_>>();
    let profiles = generated
        .profiles
        .iter()
        .enumerate()
        .map(|(index, profile)| generated_profile_draft(profile, &resources[index]))
        .collect::<Result<Vec<_>, _>>()?;

    AgentRoleCatalog::validate(&AgentRoleCatalogDraft {
        schema: generated.schema.as_str(),
        known_capabilities: known_capabilities.as_slice(),
        input_contracts: input_contracts.as_slice(),
        output_contracts: output_contracts.as_slice(),
        evidence_contracts: evidence_contracts.as_slice(),
        budget_policies: budgets.as_slice(),
        toolsets: toolsets.as_slice(),
        separation_policies: separation_policies.as_slice(),
        profiles: profiles.as_slice(),
    })
    .map_err(|error| AgentRoleConfigError::Validation(error.issue()))
}

fn string_refs(values: &[String]) -> Vec<&str> {
    values.iter().map(String::as_str).collect()
}

fn generated_separation_dimensions(
    policy: &GeneratedSeparationPolicy,
) -> Vec<AgentRoleSeparationDimension> {
    [
        (
            policy.require_distinct_assignment,
            AgentRoleSeparationDimension::Assignment,
        ),
        (
            policy.require_distinct_actor,
            AgentRoleSeparationDimension::Actor,
        ),
        (
            policy.require_distinct_run_thread,
            AgentRoleSeparationDimension::RunThread,
        ),
        (
            policy.require_distinct_session,
            AgentRoleSeparationDimension::Session,
        ),
        (
            policy.require_distinct_model_profile,
            AgentRoleSeparationDimension::ModelProfile,
        ),
        (
            policy.require_distinct_workspace,
            AgentRoleSeparationDimension::Workspace,
        ),
        (
            policy.require_distinct_mutation_lineage,
            AgentRoleSeparationDimension::MutationLineage,
        ),
        (
            policy.require_clean_context,
            AgentRoleSeparationDimension::CleanContext,
        ),
        (
            policy.require_subject_immutable,
            AgentRoleSeparationDimension::SubjectImmutable,
        ),
        (
            policy.require_verifier_immutable,
            AgentRoleSeparationDimension::VerifierImmutable,
        ),
        (
            policy.require_rubric_immutable,
            AgentRoleSeparationDimension::RubricImmutable,
        ),
    ]
    .into_iter()
    .filter_map(|(required, dimension)| required.is_required().then_some(dimension))
    .collect()
}

struct GeneratedProfileResources<'a> {
    required_capabilities: Vec<&'a str>,
    capability_ceiling: Vec<&'a str>,
    eligible_model_profiles: Vec<&'a str>,
    model_targets: Vec<&'a str>,
    prompt_modes: Vec<&'a str>,
    prompt_sources: Vec<&'a str>,
    skills: Vec<&'a str>,
    memory_policy_hashes: Vec<&'a str>,
    tools: Vec<&'a str>,
    toolsets: Vec<&'a str>,
    tool_capabilities: Vec<&'a str>,
    workspace_scopes: Vec<&'a str>,
}

impl<'a> GeneratedProfileResources<'a> {
    fn from_profile(profile: &'a GeneratedProfile) -> Self {
        Self {
            required_capabilities: string_refs(profile.required_capabilities.as_slice()),
            capability_ceiling: string_refs(profile.capability_ceiling.as_slice()),
            eligible_model_profiles: string_refs(profile.eligible_model_profiles.as_slice()),
            model_targets: string_refs(profile.model_targets.as_slice()),
            prompt_modes: string_refs(profile.prompt_modes.as_slice()),
            prompt_sources: string_refs(profile.prompt_sources.as_slice()),
            skills: string_refs(profile.skills.as_slice()),
            memory_policy_hashes: string_refs(profile.memory_policy_hashes.as_slice()),
            tools: string_refs(profile.tools.as_slice()),
            toolsets: string_refs(profile.toolsets.as_slice()),
            tool_capabilities: string_refs(profile.tool_capabilities.as_slice()),
            workspace_scopes: string_refs(profile.workspace_scopes.as_slice()),
        }
    }
}

fn generated_profile_draft<'a>(
    profile: &'a GeneratedProfile,
    resources: &'a GeneratedProfileResources<'a>,
) -> Result<AgentRoleProfileDraft<'a>, AgentRoleConfigError> {
    Ok(AgentRoleProfileDraft {
        role_id: profile.role_id.as_str(),
        class: parse_role_class(profile.class.as_str())?,
        purpose: profile.purpose.as_str(),
        source_scope: parse_source_scope(profile.source_scope.as_str())?,
        trust_status: parse_trust(profile.trust_status.as_str())?,
        admission_status: parse_admission(profile.admission_status.as_str())?,
        project_profile_admitted: profile.project_profile_admitted,
        required_capabilities: resources.required_capabilities.as_slice(),
        capability_ceiling: resources.capability_ceiling.as_slice(),
        eligible_model_profiles: resources.eligible_model_profiles.as_slice(),
        model_targets: resources.model_targets.as_slice(),
        prompt_modes: resources.prompt_modes.as_slice(),
        prompt_sources: resources.prompt_sources.as_slice(),
        skills: resources.skills.as_slice(),
        memory_policy_hashes: resources.memory_policy_hashes.as_slice(),
        tools: resources.tools.as_slice(),
        toolsets: resources.toolsets.as_slice(),
        tool_capabilities: resources.tool_capabilities.as_slice(),
        workspace_scopes: resources.workspace_scopes.as_slice(),
        mutation_posture: parse_mutation_posture(profile.mutation_posture.as_str())?,
        context_posture: parse_context_posture(profile.context_posture.as_str())?,
        evidence_posture: parse_evidence_posture(profile.evidence_posture.as_str())?,
        output_kind: parse_output_kind(profile.output_kind.as_str())?,
        input_contract: profile.input_contract.as_str(),
        output_contract: profile.output_contract.as_str(),
        evidence_contract: profile.evidence_contract.as_str(),
        separation_policy_ref: profile.separation_policy_ref.as_str(),
        budget_policy_ref: profile.budget_policy_ref.as_str(),
        policy_hash: profile.policy_hash.as_str(),
        profile_hash: profile.profile_hash.as_str(),
    })
}

fn parse_role_class(value: &str) -> Result<AgentRoleClass, AgentRoleConfigError> {
    match value {
        "implementation" => Ok(AgentRoleClass::Implementation),
        "review" => Ok(AgentRoleClass::Review),
        "verification" => Ok(AgentRoleClass::Verification),
        "judgment" => Ok(AgentRoleClass::Judgment),
        "coordination" => Ok(AgentRoleClass::Coordination),
        "observation" => Ok(AgentRoleClass::Observation),
        _ => Err(AgentRoleConfigError::UnsupportedValue),
    }
}

fn parse_source_scope(value: &str) -> Result<ModelProfileSourceScope, AgentRoleConfigError> {
    match value {
        "host" => Ok(ModelProfileSourceScope::Host),
        "project" => Ok(ModelProfileSourceScope::Project),
        _ => Err(AgentRoleConfigError::UnsupportedValue),
    }
}

fn parse_trust(value: &str) -> Result<ModelProfileTrustStatus, AgentRoleConfigError> {
    match value {
        "trusted" => Ok(ModelProfileTrustStatus::Trusted),
        "untrusted" => Ok(ModelProfileTrustStatus::Untrusted),
        _ => Err(AgentRoleConfigError::UnsupportedValue),
    }
}

fn parse_admission(value: &str) -> Result<ModelProfileAdmissionStatus, AgentRoleConfigError> {
    match value {
        "admitted" => Ok(ModelProfileAdmissionStatus::Admitted),
        "denied" => Ok(ModelProfileAdmissionStatus::Denied),
        _ => Err(AgentRoleConfigError::UnsupportedValue),
    }
}

fn parse_mutation_posture(value: &str) -> Result<AgentRoleMutationPosture, AgentRoleConfigError> {
    match value {
        "scoped_intent" => Ok(AgentRoleMutationPosture::ScopedIntent),
        "subject_read_only" => Ok(AgentRoleMutationPosture::SubjectReadOnly),
        "verification_service_only" => Ok(AgentRoleMutationPosture::VerificationServiceOnly),
        "none" => Ok(AgentRoleMutationPosture::None),
        _ => Err(AgentRoleConfigError::UnsupportedValue),
    }
}

fn parse_context_posture(value: &str) -> Result<AgentRoleContextPosture, AgentRoleConfigError> {
    match value {
        "clean_recompose" => Ok(AgentRoleContextPosture::CleanRecompose),
        "bounded_shared" => Ok(AgentRoleContextPosture::BoundedShared),
        "read_only_observation" => Ok(AgentRoleContextPosture::ReadOnlyObservation),
        _ => Err(AgentRoleConfigError::UnsupportedValue),
    }
}

fn parse_evidence_posture(value: &str) -> Result<AgentRoleEvidencePosture, AgentRoleConfigError> {
    match value {
        "candidate" => Ok(AgentRoleEvidencePosture::Candidate),
        "findings" => Ok(AgentRoleEvidencePosture::Findings),
        "check_evidence" => Ok(AgentRoleEvidencePosture::CheckEvidence),
        "judgment" => Ok(AgentRoleEvidencePosture::Judgment),
        "coordination" => Ok(AgentRoleEvidencePosture::Coordination),
        "observation" => Ok(AgentRoleEvidencePosture::Observation),
        _ => Err(AgentRoleConfigError::UnsupportedValue),
    }
}

fn parse_output_kind(value: &str) -> Result<AgentRoleOutputKind, AgentRoleConfigError> {
    match value {
        "implementation_candidate" => Ok(AgentRoleOutputKind::ImplementationCandidate),
        "review_report" => Ok(AgentRoleOutputKind::ReviewReport),
        "verification_report" => Ok(AgentRoleOutputKind::VerificationReport),
        "judgment_report" => Ok(AgentRoleOutputKind::JudgmentReport),
        "coordination_report" => Ok(AgentRoleOutputKind::CoordinationReport),
        "observation_report" => Ok(AgentRoleOutputKind::ObservationReport),
        _ => Err(AgentRoleConfigError::UnsupportedValue),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATED_JSON: &str = include_str!("../../../config/generated/agent-role-profiles.json");
    const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const MISSING_PROFILE_PATH: &str = "animus-explicit-missing-agent-role-profiles.json";

    #[test]
    fn generated_roles_load_into_validated_owned_runtime_values() {
        let roles = parse_agent_role_profiles_json(GENERATED_JSON)
            .expect("checked generated semantic roles should load");

        assert!(roles.profile("implementer").is_some());
        assert!(roles.profile("reviewer").is_some());
        assert!(roles.profile("verifier").is_some());
        assert!(roles.profile("judge").is_some());
    }

    #[test]
    fn missing_explicit_role_path_fails_without_discovery() {
        let error = load_agent_role_profiles(MISSING_PROFILE_PATH)
            .expect_err("missing explicit role path must fail closed");

        assert_eq!(
            error,
            AgentRoleConfigError::Read(std::io::ErrorKind::NotFound)
        );
    }

    #[test]
    fn malformed_unknown_and_stale_generated_roles_fail_closed() {
        assert_eq!(
            parse_agent_role_profiles_json("{").expect_err("truncated JSON"),
            AgentRoleConfigError::Json(AgentRoleJsonErrorKind::EndOfInput)
        );
        let unknown = GENERATED_JSON.replacen("\"schema\":", "\"unknown\": true, \"schema\":", 1);
        assert_eq!(
            parse_agent_role_profiles_json(unknown.as_str()).expect_err("unknown field"),
            AgentRoleConfigError::Json(AgentRoleJsonErrorKind::Data)
        );
        let unsupported =
            GENERATED_JSON.replacen("\"class\": \"review\"", "\"class\": \"admin\"", 1);
        assert_eq!(
            parse_agent_role_profiles_json(unsupported.as_str()).expect_err("unsupported class"),
            AgentRoleConfigError::UnsupportedValue
        );
        let roles = parse_agent_role_profiles_json(GENERATED_JSON)
            .expect("checked generated semantic roles should load");
        let profile_hash = roles
            .profile("reviewer")
            .expect("generated reviewer")
            .profile_hash();
        let stale = GENERATED_JSON.replacen(profile_hash, ZERO_HASH, 1);
        assert_eq!(
            parse_agent_role_profiles_json(stale.as_str()).expect_err("stale profile hash"),
            AgentRoleConfigError::Validation(AgentRoleIssue::StaleIdentity)
        );
    }
}
