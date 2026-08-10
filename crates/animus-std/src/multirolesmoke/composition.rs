//! Exact model-profile composition for the packaged multi-role smoke.

use animus_runtime::agent_roles::{
    AgentRoleBoundModelPlan, AgentRoleModelCompositionInput, AgentRoleProfile,
    compose_agent_role_model_plan,
};
use animus_runtime::{
    ModelInvocationAuthority, ModelInvocationCompositionInput, ModelMemoryAccessMode,
    ModelProfileSpecificity, ModelResourceCeiling, ModelRuntimeBinding,
    ModelRuntimeProfileResolution, ModelRuntimeProfileSet, ModelRuntimeTargetDraft,
    ModelSkillCompositionResource, ResolvedModelRuntimeProfile,
};

use super::{domain_hash, error};

const ERROR_MODEL_PROFILE: &str = "model-profile";
const ERROR_MODEL_COMPOSITION: &str = "model-composition";
const PROVIDER_ID: &str = "mock";
const MODEL_ID: &str = "mock-text";
const REVIEWER_ROLE: &str = "reviewer";
const BALANCED_ROUTER_ROLE: &str = "balanced";
const POLICY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const MEMORY_POLICY_HASH: &str = "9999999999999999999999999999999999999999999999999999999999999999";
const TOOL_CATALOG_HASH_DOMAIN: &str = "animus.multi-role-smoke.tool-catalog.v1";
const PROMPT_HASH_DOMAIN: &str = "animus.multi-role-smoke.prompt-source.v1";
const SKILL_HASH_DOMAIN: &str = "animus.multi-role-smoke.skill.v1";
const TOOL_NAME: &str = "inspect_context";
const TOOL_CAPABILITY: &str = "observe_read";
const TOOL_EXECUTOR: &str = "executor.inspect-context";
const SKILL_NAME: &str = "proof-search";
const HOST_PROMPT: &str = "host-policy";
const MODEL_PROMPT: &str = "model-guidance";
const PROMPT_MODE: &str = "standard";
const WORKSPACE_SCOPE: &str = "workspace:project";
const MODEL_TARGET: &str = "mock:mock-text";
const HOST_PRIORITY: u16 = 10;
const MODEL_PRIORITY: u16 = 20;
const PROMPT_SOURCE_UNITS: u32 = 8;
const PROMPT_BUDGET_UNITS: u32 = 64;
const MODEL_CONTEXT_UNITS: u32 = 4_096;
const TOOL_VISIBLE_CHARS: u32 = 1_024;
const TOOL_OUTPUT_BYTES: u64 = 4_096;
const SECTION_ORDER: &[animus_adapters::PromptSectionKind] = &[
    animus_adapters::PromptSectionKind::Policy,
    animus_adapters::PromptSectionKind::Context,
];
const PROMPT_GUIDELINES: &[&str] = &["Use only bounded host-supplied context."];
const ARGUMENT_NAMES: &[&str] = &["query"];
const SKILL_TAGS: &[&str] = &["proof"];
const NO_RELATED_SKILLS: &[&str] = &[];
const MODEL_TARGETS: &[&str] = &[MODEL_TARGET];
const PROMPT_MODES: &[&str] = &[PROMPT_MODE];
const PROMPT_LABELS: &[&str] = &[HOST_PROMPT, MODEL_PROMPT];
const SKILL_LABELS: &[&str] = &[SKILL_NAME];
const MEMORY_POLICY_HASHES: &[&str] = &[MEMORY_POLICY_HASH];
const TOOL_LABELS: &[&str] = &[TOOL_NAME];
const TOOL_CAPABILITIES: &[&str] = &[TOOL_CAPABILITY];

pub(super) fn compose_role_model_plan(
    model_profiles: &ModelRuntimeProfileSet,
    role: &AgentRoleProfile,
) -> Result<AgentRoleBoundModelPlan, String> {
    let router_role = (role.role_id() == REVIEWER_ROLE).then_some(BALANCED_ROUTER_ROLE);
    let capability_identity = exact_capability_identity(model_profiles, router_role)?;
    let resolved = resolve_profile(model_profiles, router_role, capability_identity.as_str())?;
    let prompts = prompt_sources(&resolved)?;
    let prompt_policy = prompt_policy()?;
    let skill_manifest = skill_manifest()?;
    let skill_resources = skill_resources(&resolved, &skill_manifest);
    let memory_policy = memory_policy()?;
    let memory_policy_resource = (resolved.profile().memory_access_mode()
        != ModelMemoryAccessMode::Disabled)
        .then_some(&memory_policy);
    let tool_catalog = tool_catalog()?;
    let executor_bindings = executor_bindings()?;
    let capabilities = capability_matrix();
    let capability_decision = capability_decision();
    let base_ceiling = base_ceiling();
    let base_ceilings = [base_ceiling];
    let catalog_identity = domain_hash(TOOL_CATALOG_HASH_DOMAIN, TOOL_NAME);
    let output_contracts = [role.output_contract()];
    let budget_policies = [role.budget_policy_ref()];
    let workspace_scopes = [WORKSPACE_SCOPE];

    compose_agent_role_model_plan(AgentRoleModelCompositionInput {
        profile: role,
        model: ModelInvocationCompositionInput {
            resolved: &resolved,
            capabilities: &capabilities,
            capability_decision: &capability_decision,
            prompt_sources: prompts.as_slice(),
            prompt_policy: &prompt_policy,
            skills: skill_resources.as_slice(),
            memory_policy: memory_policy_resource,
            tool_catalog: &tool_catalog,
            executor_bindings: executor_bindings.as_slice(),
            tool_catalog_identity: catalog_identity.as_str(),
            authority: ModelInvocationAuthority {
                ceilings: &base_ceilings,
                approved_tools: &[],
                policy_hash: POLICY_HASH,
            },
        },
        admitted_output_contracts: &output_contracts,
        admitted_budget_policies: &budget_policies,
        admitted_workspace_scopes: &workspace_scopes,
    })
    .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))
}

fn exact_capability_identity(
    profiles: &ModelRuntimeProfileSet,
    router_role: Option<&str>,
) -> Result<String, String> {
    let mut identity = None;
    for profile in profiles.profiles() {
        let ModelRuntimeBinding::Exact {
            provider_id,
            model_id,
            router_role: bound_role,
            capability_identity,
        } = profile.binding()
        else {
            continue;
        };
        let is_match = provider_id == PROVIDER_ID
            && model_id == MODEL_ID
            && bound_role.as_deref() == router_role;
        if is_match {
            if identity.is_some() {
                return Err(error(ERROR_MODEL_PROFILE));
            }
            identity = Some(capability_identity.clone());
        }
    }
    identity.ok_or_else(|| error(ERROR_MODEL_PROFILE))
}

fn resolve_profile(
    profiles: &ModelRuntimeProfileSet,
    router_role: Option<&str>,
    capability_identity: &str,
) -> Result<Box<ResolvedModelRuntimeProfile>, String> {
    let resolution = animus_runtime::resolve_model_runtime_profile(
        profiles,
        ModelRuntimeTargetDraft {
            provider_id: PROVIDER_ID,
            model_id: MODEL_ID,
            router_role,
            capability_identity,
        },
    )
    .map_err(|_failure| error(ERROR_MODEL_PROFILE))?;
    let ModelRuntimeProfileResolution::Resolved(resolved) = resolution else {
        return Err(error(ERROR_MODEL_PROFILE));
    };
    let expected_specificity = if router_role.is_some() {
        ModelProfileSpecificity::ExactTargetAndRole
    } else {
        ModelProfileSpecificity::ExactTarget
    };
    if resolved.specificity() != expected_specificity {
        return Err(error(ERROR_MODEL_PROFILE));
    }
    Ok(resolved)
}

fn prompt_sources(
    resolved: &ResolvedModelRuntimeProfile,
) -> Result<Vec<animus_adapters::PromptSource>, String> {
    let mut sources = Vec::with_capacity(resolved.profile().prompt_sources().len());
    for reference in resolved.profile().prompt_sources() {
        let (kind, section, priority) = match reference.label() {
            HOST_PROMPT => (
                animus_adapters::PromptSourceKind::PolicyProfile,
                animus_adapters::PromptSectionKind::Policy,
                HOST_PRIORITY,
            ),
            MODEL_PROMPT => (
                animus_adapters::PromptSourceKind::HostContext,
                animus_adapters::PromptSectionKind::Context,
                MODEL_PRIORITY,
            ),
            _unsupported => return Err(error(ERROR_MODEL_COMPOSITION)),
        };
        let requirement = if reference.required() {
            animus_adapters::PromptSourceRequirement::Required
        } else {
            animus_adapters::PromptSourceRequirement::Optional
        };
        let body_hash = domain_hash(PROMPT_HASH_DOMAIN, reference.label());
        let source = animus_adapters::PromptSource::validate(&animus_adapters::PromptSourceDraft {
            kind,
            scope: animus_adapters::PromptResourceScope::Host,
            trust_status: animus_adapters::PromptSourceTrustStatus::Trusted,
            admission_status: animus_adapters::PromptSourceAdmissionStatus::Admitted,
            requirement,
            priority: animus_adapters::PromptSourcePriority::new(priority),
            budget_class: animus_adapters::PromptBudgetClass::Tiny,
            redaction_class: animus_adapters::PromptRedactionClass::Public,
            label: reference.label(),
            section,
            body_hash: body_hash.as_str(),
            requested_units: PROMPT_SOURCE_UNITS,
        })
        .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))?;
        sources.push(source);
    }
    Ok(sources)
}

fn prompt_policy() -> Result<animus_adapters::PromptAssemblyPolicy, String> {
    animus_adapters::PromptAssemblyPolicy::validate(&animus_adapters::PromptAssemblyPolicyDraft {
        mode: animus_adapters::PromptAssemblyMode::AdmittedSources,
        section_order: SECTION_ORDER,
        section_budget_units: PROMPT_BUDGET_UNITS,
        policy_hash: POLICY_HASH,
    })
    .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))
}

fn skill_manifest() -> Result<animus_adapters::SkillManifest, String> {
    let skill_hash = domain_hash(SKILL_HASH_DOMAIN, SKILL_NAME);
    animus_adapters::SkillManifest::validate(&animus_adapters::SkillManifestDraft {
        name: Some(SKILL_NAME),
        description: Some("Search bounded proof families."),
        version: Some("1.0.0"),
        category: Some("reasoning"),
        tags: SKILL_TAGS,
        related_skills: NO_RELATED_SKILLS,
        provenance: Some("host-reviewed"),
        trust_status: Some(animus_adapters::SkillManifestTrustStatus::Trusted.as_str()),
        content_identity: Some(skill_hash.as_str()),
    })
    .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))
}

fn skill_resources<'a>(
    resolved: &ResolvedModelRuntimeProfile,
    manifest: &'a animus_adapters::SkillManifest,
) -> Vec<ModelSkillCompositionResource<'a>> {
    if resolved.profile().skills().is_empty() {
        return Vec::new();
    }
    vec![ModelSkillCompositionResource {
        manifest,
        selected_body: None,
        selection_profile_hash: None,
        selection_target_hash: None,
        invocation_admitted: true,
        dependencies_admitted: true,
        budget_admitted: true,
    }]
}

fn memory_policy() -> Result<animus_adapters::OptInMemoryPolicy, String> {
    let controls = animus_adapters::MemoryScopeControls {
        user: animus_adapters::MemoryOptInControl::Enabled,
        project: animus_adapters::MemoryOptInControl::Enabled,
        thread: animus_adapters::MemoryOptInControl::Enabled,
    };
    animus_adapters::OptInMemoryPolicy::validate(&animus_adapters::OptInMemoryPolicyDraft {
        use_controls: controls,
        generation_controls: controls,
        policy_hash: MEMORY_POLICY_HASH,
    })
    .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))
}

fn tool_catalog() -> Result<animus_runtime::tool_executor_kit::ToolCatalog, String> {
    let metadata =
        animus_adapters::tool::Definition::validate(&animus_adapters::tool::DefinitionDraft {
            name: Some(TOOL_NAME),
            label: Some("Inspect context"),
            description: Some("Read bounded host-supplied context."),
            parameter_schema_identity: Some("schema:animus.tool.input.context.v1"),
            prompt_snippet: Some("Inspect bounded context."),
            prompt_guidelines: PROMPT_GUIDELINES,
            source_identity: Some("animus.tool.inspect-context.v1"),
            provenance: Some("host-reviewed"),
            capability_labels: TOOL_CAPABILITIES,
            result_metadata_shape: Some("schema:animus.tool.output.context.v1"),
            status: animus_adapters::tool::DefinitionStatus::Active,
        })
        .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))?;
    let output_policy = animus_runtime::tool_executor_kit::ToolExecutorOutputPolicy::new(
        TOOL_VISIBLE_CHARS,
        TOOL_OUTPUT_BYTES,
        true,
    )
    .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))?;
    let definition = animus_runtime::tool_executor_kit::ToolDefinition::from_draft(
        animus_runtime::tool_executor_kit::ToolDefinitionDraft {
            metadata,
            executor_binding: TOOL_EXECUTOR,
            argument_names: ARGUMENT_NAMES,
            mutation_posture: animus_runtime::tool_executor_kit::ToolMutationPosture::ReadOnly,
            output_policy,
            approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved,
        },
    )
    .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))?;
    animus_runtime::tool_executor_kit::ToolCatalog::new(vec![definition])
        .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))
}

fn executor_bindings() -> Result<Vec<animus_runtime::tool_executor_kit::ToolExecutorBinding>, String>
{
    let binding = animus_runtime::tool_executor_kit::ToolExecutorBinding::new(
        TOOL_EXECUTOR,
        animus_runtime::tool_executor_kit::ToolExecutorReadiness::Ready,
    )
    .map_err(|_failure| error(ERROR_MODEL_COMPOSITION))?;
    Ok(vec![binding])
}

fn capability_matrix() -> animus_runtime::ProviderModelCapabilityMatrix {
    animus_runtime::ProviderModelCapabilityMatrix {
        schema_identity: "animus.model-capability-matrix.v1".to_string(),
        provider_id: PROVIDER_ID.to_string(),
        model_id: MODEL_ID.to_string(),
        roles: vec![animus_adapters::CanonicalModelRole::User],
        content_parts: vec![animus_runtime::ProviderModelContentPartClass::Text],
        tool_calls: false,
        structured_outputs: Vec::new(),
        stream_channels: Vec::new(),
        reasoning_controls: Vec::new(),
        context_budget: MODEL_CONTEXT_UNITS,
        output_budget: MODEL_CONTEXT_UNITS,
        privacy: animus_runtime::ProviderPrivacyPosture::LocalOnly,
    }
}

fn capability_decision() -> animus_runtime::ModelCapabilityNegotiationDecision {
    animus_runtime::ModelCapabilityNegotiationDecision {
        status: animus_runtime::ModelCapabilityDecisionStatus::Admitted,
        provider_id: PROVIDER_ID.to_string(),
        model_id: MODEL_ID.to_string(),
        evidence: vec!["capability-matrix-admitted".to_string()],
        downgrades: Vec::new(),
        issues: Vec::new(),
    }
}

const fn base_ceiling() -> ModelResourceCeiling<'static> {
    ModelResourceCeiling {
        ceiling_label: "host-policy",
        model_targets: MODEL_TARGETS,
        prompt_modes: PROMPT_MODES,
        prompt_sources: PROMPT_LABELS,
        skills: SKILL_LABELS,
        memory_policy_hashes: MEMORY_POLICY_HASHES,
        tools: TOOL_LABELS,
        tool_capabilities: TOOL_CAPABILITIES,
    }
}
