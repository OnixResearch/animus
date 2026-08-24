//! Host-owned loading for generated per-model runtime profiles.

use animus_runtime::{
    ModelMemoryAccessMode, ModelNamedResourceReferenceDraft, ModelProfileAdmissionStatus,
    ModelProfileEnablementPolicy, ModelProfileSourceScope, ModelProfileTrustStatus,
    ModelPromptSourceReferenceDraft, ModelRuntimeBindingDraft, ModelRuntimeBudgetPolicyDraft,
    ModelRuntimeProfileDraft, ModelRuntimeProfileIssue, ModelRuntimeProfileSet,
    ModelRuntimeProfileSetDraft, ModelSkillReferenceDraft, ModelSkillResourcePosture,
    ModelToolsetDraft, NativeToolCallingMode,
};
use serde::Deserialize;
use std::path::Path;

/// Stable JSON failure category for generated model-profile configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelRuntimeProfileJsonErrorKind {
    /// JSON syntax was malformed.
    Syntax,
    /// JSON values did not match the exact generated DTO shape.
    Data,
    /// JSON ended before a complete value was available.
    EndOfInput,
}

/// Safe host-shell error for loading generated model profiles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelRuntimeProfileConfigError {
    /// The configured file could not be read.
    Read(std::io::ErrorKind),
    /// The configured bytes were not an exact generated DTO.
    Json(ModelRuntimeProfileJsonErrorKind),
    /// A generated enum or binding discriminant was unsupported.
    UnsupportedValue,
    /// Runtime validation rejected the owned profile set.
    Validation(ModelRuntimeProfileIssue),
}

/// Parse and validate generated profile JSON without filesystem or environment access.
///
/// # Errors
///
/// Returns a bounded error class when JSON shape, enum values, canonical identities,
/// trust/admission state, references, or profile invariants fail validation.
pub fn parse_model_runtime_profiles_json(
    input: &str,
) -> Result<ModelRuntimeProfileSet, ModelRuntimeProfileConfigError> {
    let generated: GeneratedSet =
        serde_json::from_str(input).map_err(|error| classify_json_error(&error))?;
    validate_generated(&generated)
}

/// Read generated profile JSON from an explicit host path and validate it.
///
/// # Errors
///
/// Returns a bounded read, JSON, unsupported-value, or runtime-validation error.
pub fn load_model_runtime_profiles(
    path: impl AsRef<Path>,
) -> Result<ModelRuntimeProfileSet, ModelRuntimeProfileConfigError> {
    let input = std::fs::read_to_string(path.as_ref())
        .map_err(|error| ModelRuntimeProfileConfigError::Read(error.kind()))?;
    parse_model_runtime_profiles_json(input.as_str())
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedSet {
    schema: String,
    enablement: String,
    explicit_default_profile_id: String,
    budget_policies: Vec<GeneratedBudget>,
    toolsets: Vec<GeneratedToolset>,
    profiles: Vec<GeneratedProfile>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedBudget {
    policy_id: String,
    max_context_units: u32,
    max_tool_calls: u32,
    max_parallel_tool_calls: u16,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedToolset {
    toolset_id: String,
    tool_names: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedProfile {
    profile_id: String,
    dialect_id: String,
    binding: GeneratedBinding,
    source_scope: String,
    trust_status: String,
    admission_status: String,
    project_profile_admitted: bool,
    prompt_mode_ref: String,
    prompt_sources: Vec<GeneratedPromptReference>,
    skills: Vec<GeneratedSkillReference>,
    memory_access_mode: String,
    memory_policy_hash: String,
    tools: Vec<GeneratedNamedReference>,
    toolsets: Vec<GeneratedNamedReference>,
    tool_calling_mode: String,
    context_budget_policy: String,
    tool_budget_policy: String,
    policy_hash: String,
    profile_hash: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedBinding {
    kind: String,
    provider_id: String,
    model_id: String,
    router_role: String,
    capability_identity: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedPromptReference {
    label: String,
    required: bool,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedSkillReference {
    name: String,
    posture: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedNamedReference {
    name: String,
    required: bool,
}

fn classify_json_error(error: &serde_json::Error) -> ModelRuntimeProfileConfigError {
    let kind = match error.classify() {
        serde_json::error::Category::Syntax => ModelRuntimeProfileJsonErrorKind::Syntax,
        serde_json::error::Category::Data | serde_json::error::Category::Io => {
            ModelRuntimeProfileJsonErrorKind::Data
        }
        serde_json::error::Category::Eof => ModelRuntimeProfileJsonErrorKind::EndOfInput,
    };
    ModelRuntimeProfileConfigError::Json(kind)
}

fn validate_generated(
    generated: &GeneratedSet,
) -> Result<ModelRuntimeProfileSet, ModelRuntimeProfileConfigError> {
    let budgets: Vec<ModelRuntimeBudgetPolicyDraft<'_>> = generated
        .budget_policies
        .iter()
        .map(|policy| ModelRuntimeBudgetPolicyDraft {
            policy_id: policy.policy_id.as_str(),
            max_context_units: policy.max_context_units,
            max_tool_calls: policy.max_tool_calls,
            max_parallel_tool_calls: policy.max_parallel_tool_calls,
        })
        .collect();
    let tool_names: Vec<Vec<&str>> = generated
        .toolsets
        .iter()
        .map(|toolset| toolset.tool_names.iter().map(String::as_str).collect())
        .collect();
    let toolsets: Vec<ModelToolsetDraft<'_>> = generated
        .toolsets
        .iter()
        .enumerate()
        .map(|(index, toolset)| ModelToolsetDraft {
            toolset_id: toolset.toolset_id.as_str(),
            tool_names: tool_names[index].as_slice(),
        })
        .collect();
    let prompt_refs = generated_prompt_refs(generated);
    let skill_refs = generated_skill_refs(generated)?;
    let tool_refs = generated_named_refs(generated, |profile| profile.tools.as_slice());
    let toolset_refs = generated_named_refs(generated, |profile| profile.toolsets.as_slice());
    let profiles: Vec<ModelRuntimeProfileDraft<'_>> = generated
        .profiles
        .iter()
        .enumerate()
        .map(|(index, profile)| {
            generated_profile_draft(
                profile,
                GeneratedProfileResources {
                    prompt_sources: prompt_refs[index].as_slice(),
                    skills: skill_refs[index].as_slice(),
                    tools: tool_refs[index].as_slice(),
                    toolsets: toolset_refs[index].as_slice(),
                },
            )
        })
        .collect::<Result<Vec<_>, _>>()?;
    let enablement = parse_enablement(generated.enablement.as_str())?;

    ModelRuntimeProfileSet::validate(&ModelRuntimeProfileSetDraft {
        schema: generated.schema.as_str(),
        enablement,
        explicit_default_profile_id: Some(generated.explicit_default_profile_id.as_str()),
        budget_policies: budgets.as_slice(),
        toolsets: toolsets.as_slice(),
        profiles: profiles.as_slice(),
    })
    .map_err(|error| ModelRuntimeProfileConfigError::Validation(error.issue()))
}

fn generated_prompt_refs(
    generated: &GeneratedSet,
) -> Vec<Vec<ModelPromptSourceReferenceDraft<'_>>> {
    generated
        .profiles
        .iter()
        .map(|profile| {
            profile
                .prompt_sources
                .iter()
                .map(|reference| ModelPromptSourceReferenceDraft {
                    label: reference.label.as_str(),
                    required: reference.required,
                })
                .collect()
        })
        .collect()
}

fn generated_skill_refs(
    generated: &GeneratedSet,
) -> Result<Vec<Vec<ModelSkillReferenceDraft<'_>>>, ModelRuntimeProfileConfigError> {
    generated
        .profiles
        .iter()
        .map(|profile| {
            profile
                .skills
                .iter()
                .map(|reference| {
                    Ok(ModelSkillReferenceDraft {
                        name: reference.name.as_str(),
                        posture: parse_skill_posture(reference.posture.as_str())?,
                    })
                })
                .collect()
        })
        .collect()
}

fn generated_named_refs<'a, F>(
    generated: &'a GeneratedSet,
    select: F,
) -> Vec<Vec<ModelNamedResourceReferenceDraft<'a>>>
where
    F: Fn(&'a GeneratedProfile) -> &'a [GeneratedNamedReference],
{
    generated
        .profiles
        .iter()
        .map(|profile| {
            select(profile)
                .iter()
                .map(|reference| ModelNamedResourceReferenceDraft {
                    name: reference.name.as_str(),
                    required: reference.required,
                })
                .collect()
        })
        .collect()
}

#[derive(Clone, Copy)]
struct GeneratedProfileResources<'a> {
    prompt_sources: &'a [ModelPromptSourceReferenceDraft<'a>],
    skills: &'a [ModelSkillReferenceDraft<'a>],
    tools: &'a [ModelNamedResourceReferenceDraft<'a>],
    toolsets: &'a [ModelNamedResourceReferenceDraft<'a>],
}

fn generated_profile_draft<'a>(
    profile: &'a GeneratedProfile,
    resources: GeneratedProfileResources<'a>,
) -> Result<ModelRuntimeProfileDraft<'a>, ModelRuntimeProfileConfigError> {
    Ok(ModelRuntimeProfileDraft {
        profile_id: profile.profile_id.as_str(),
        dialect_id: profile.dialect_id.as_str(),
        binding: parse_binding(&profile.binding)?,
        source_scope: parse_source_scope(profile.source_scope.as_str())?,
        trust_status: parse_trust(profile.trust_status.as_str())?,
        admission_status: parse_admission(profile.admission_status.as_str())?,
        project_profile_admitted: profile.project_profile_admitted,
        prompt_mode_ref: profile.prompt_mode_ref.as_str(),
        prompt_sources: resources.prompt_sources,
        skills: resources.skills,
        memory_access_mode: parse_memory_access(profile.memory_access_mode.as_str())?,
        memory_policy_hash: profile.memory_policy_hash.as_str(),
        tools: resources.tools,
        toolsets: resources.toolsets,
        tool_calling_mode: parse_tool_mode(profile.tool_calling_mode.as_str())?,
        context_budget_policy: profile.context_budget_policy.as_str(),
        tool_budget_policy: profile.tool_budget_policy.as_str(),
        policy_hash: profile.policy_hash.as_str(),
        profile_hash: profile.profile_hash.as_str(),
    })
}

fn parse_binding(
    binding: &GeneratedBinding,
) -> Result<ModelRuntimeBindingDraft<'_>, ModelRuntimeProfileConfigError> {
    match binding.kind.as_str() {
        "exact" => Ok(ModelRuntimeBindingDraft::Exact {
            provider_id: binding.provider_id.as_str(),
            model_id: binding.model_id.as_str(),
            router_role: (!binding.router_role.is_empty()).then_some(binding.router_role.as_str()),
            capability_identity: binding.capability_identity.as_str(),
        }),
        "default" => Ok(ModelRuntimeBindingDraft::Default),
        _ => Err(ModelRuntimeProfileConfigError::UnsupportedValue),
    }
}

fn parse_enablement(
    value: &str,
) -> Result<ModelProfileEnablementPolicy, ModelRuntimeProfileConfigError> {
    match value {
        "required" => Ok(ModelProfileEnablementPolicy::Required),
        "compatibility_disabled" => Ok(ModelProfileEnablementPolicy::CompatibilityDisabled),
        _ => Err(ModelRuntimeProfileConfigError::UnsupportedValue),
    }
}

fn parse_source_scope(
    value: &str,
) -> Result<ModelProfileSourceScope, ModelRuntimeProfileConfigError> {
    match value {
        "host" => Ok(ModelProfileSourceScope::Host),
        "project" => Ok(ModelProfileSourceScope::Project),
        _ => Err(ModelRuntimeProfileConfigError::UnsupportedValue),
    }
}

fn parse_trust(value: &str) -> Result<ModelProfileTrustStatus, ModelRuntimeProfileConfigError> {
    match value {
        "trusted" => Ok(ModelProfileTrustStatus::Trusted),
        "untrusted" => Ok(ModelProfileTrustStatus::Untrusted),
        _ => Err(ModelRuntimeProfileConfigError::UnsupportedValue),
    }
}

fn parse_admission(
    value: &str,
) -> Result<ModelProfileAdmissionStatus, ModelRuntimeProfileConfigError> {
    match value {
        "admitted" => Ok(ModelProfileAdmissionStatus::Admitted),
        "denied" => Ok(ModelProfileAdmissionStatus::Denied),
        _ => Err(ModelRuntimeProfileConfigError::UnsupportedValue),
    }
}

fn parse_skill_posture(
    value: &str,
) -> Result<ModelSkillResourcePosture, ModelRuntimeProfileConfigError> {
    match value {
        "metadata_only" => Ok(ModelSkillResourcePosture::MetadataOnly),
        "selectable" => Ok(ModelSkillResourcePosture::Selectable),
        "required_selected" => Ok(ModelSkillResourcePosture::RequiredSelected),
        _ => Err(ModelRuntimeProfileConfigError::UnsupportedValue),
    }
}

fn parse_memory_access(
    value: &str,
) -> Result<ModelMemoryAccessMode, ModelRuntimeProfileConfigError> {
    match value {
        "disabled" => Ok(ModelMemoryAccessMode::Disabled),
        "read_only" => Ok(ModelMemoryAccessMode::ReadOnly),
        "read_write" => Ok(ModelMemoryAccessMode::ReadWrite),
        _ => Err(ModelRuntimeProfileConfigError::UnsupportedValue),
    }
}

fn parse_tool_mode(value: &str) -> Result<NativeToolCallingMode, ModelRuntimeProfileConfigError> {
    match value {
        "disabled" => Ok(NativeToolCallingMode::Disabled),
        "optional" => Ok(NativeToolCallingMode::Optional),
        "capability_required" => Ok(NativeToolCallingMode::CapabilityRequired),
        _ => Err(ModelRuntimeProfileConfigError::UnsupportedValue),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATED_JSON: &str =
        include_str!("../../../config/generated/model-runtime-profiles.json");
    const ALTERNATE_PROFILE_HASH: &str =
        "0000000000000000000000000000000000000000000000000000000000000000";
    const MISSING_PROFILE_PATH: &str = "animus-explicit-missing-model-runtime-profiles.json";

    #[test]
    fn generated_profiles_load_into_validated_owned_runtime_values() {
        let profiles = parse_model_runtime_profiles_json(GENERATED_JSON)
            .expect("checked generated profiles should load");

        assert!(!profiles.profiles().is_empty());
        assert!(profiles.explicit_default_profile_id().is_some());
    }

    #[test]
    fn missing_explicit_profile_path_fails_without_discovery() {
        let error = load_model_runtime_profiles(MISSING_PROFILE_PATH)
            .expect_err("missing explicit profile path must fail closed");

        assert_eq!(
            error,
            ModelRuntimeProfileConfigError::Read(std::io::ErrorKind::NotFound)
        );
    }

    #[test]
    fn malformed_unknown_and_stale_generated_profiles_fail_closed() {
        assert_eq!(
            parse_model_runtime_profiles_json("{").expect_err("truncated JSON"),
            ModelRuntimeProfileConfigError::Json(ModelRuntimeProfileJsonErrorKind::EndOfInput)
        );

        let unknown =
            GENERATED_JSON.replacen("\"schema\":", "\"unknown_field\": true, \"schema\":", 1);
        assert_eq!(
            parse_model_runtime_profiles_json(unknown.as_str()).expect_err("unknown field"),
            ModelRuntimeProfileConfigError::Json(ModelRuntimeProfileJsonErrorKind::Data)
        );

        let unsupported_memory = GENERATED_JSON.replacen(
            "\"memory_access_mode\": \"disabled\"",
            "\"memory_access_mode\": \"ambient\"",
            1,
        );
        assert_eq!(
            parse_model_runtime_profiles_json(unsupported_memory.as_str())
                .expect_err("unsupported memory mode"),
            ModelRuntimeProfileConfigError::UnsupportedValue
        );

        let profiles = parse_model_runtime_profiles_json(GENERATED_JSON)
            .expect("checked generated profiles should load");
        let profile_hash = profiles
            .profiles()
            .first()
            .expect("generated profiles should not be empty")
            .profile_hash();
        let stale = GENERATED_JSON.replacen(profile_hash, ALTERNATE_PROFILE_HASH, 1);
        assert_eq!(
            parse_model_runtime_profiles_json(stale.as_str()).expect_err("stale hash"),
            ModelRuntimeProfileConfigError::Validation(
                ModelRuntimeProfileIssue::StaleProfileIdentity
            )
        );
    }
}
