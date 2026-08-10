//! Deterministic four-role coordinator for the packaged multi-role smoke.

#[cfg(feature = "authoritative-completion-smoke")]
use animus_runtime::agent_roles::AGENT_ROLE_SELECT_ABILITY;
use animus_runtime::agent_roles::{
    AGENT_ROLE_ACTIVATE_ABILITY, AGENT_ROLE_ASSIGN_ABILITY, AGENT_ROLE_CAPABILITY_CONTRACT,
    AGENT_ROLE_EMIT_CANDIDATE_ABILITY, AGENT_ROLE_EMIT_REVIEW_ABILITY,
    AGENT_ROLE_EMIT_VERIFICATION_ABILITY, AGENT_ROLE_VETO_ABILITY,
    AgentRoleActivationResourceInput, AgentRoleAdmissionState, AgentRoleAssignment,
    AgentRoleAssignmentRequest, AgentRoleBoundModelPlan, AgentRoleCapabilityCeilings,
    AgentRoleCapabilityResolution, AgentRoleCapabilityResolutionInput, AgentRoleCatalog,
    AgentRoleDeterministicPolicyState, AgentRoleOutput, AgentRoleOutputAdmission,
    AgentRoleOutputClaim, AgentRoleOutputDraft, AgentRoleOutputEffect,
    AgentRoleOutputResourceInput, AgentRoleOutputStatus, AgentRolePlan, AgentRolePlanInput,
    AgentRoleProfile, AgentRoleReceiptInput, AgentRoleReceiptKind, AgentRoleReceiptSensitiveDraft,
    AgentRoleSeparationDecision, AgentRoleSeparationDimension, AgentRoleSeparationObservation,
    admit_agent_role_assignment, admit_agent_role_capability_use, admit_agent_role_output,
    agent_role_activation_resource, agent_role_output_resource, build_agent_role_plan,
    evaluate_agent_role_separation, project_agent_role_receipt, resolve_agent_role_capabilities,
};
use animus_runtime::authoritative_completion::{
    AuthoritativeCompletionCandidateDraft, CompletionCandidateSource, admit_completion_candidate,
};
use animus_runtime::{ModelRuntimeProfileSet, TypedSubagentCapability};

use super::composition::compose_role_model_plan;
use super::{SmokeFixture, domain_hash, error};

const ERROR_ROLE_MISSING: &str = "role-missing";
const ERROR_SEPARATION: &str = "separation";
const ERROR_ACTIVATION: &str = "activation";
const ERROR_CAPABILITY: &str = "capability";
const ERROR_OUTPUT: &str = "output";
const ERROR_REVIEWER_MUTATION: &str = "reviewer-mutation";
const ERROR_COMPLETION_BOUNDARY: &str = "completion-boundary";
const ERROR_RECEIPT: &str = "receipt";
const IMPLEMENTER: &str = "implementer";
const REVIEWER: &str = "reviewer";
const VERIFIER: &str = "verifier";
const JUDGE: &str = "judge";
const WORK_ID: &str = "multi-role-smoke";
const AUTH_RESOURCE_PREFIX: &str = "role:";
const IDENTITY_DOMAIN: &str = "animus.multi-role-smoke.identity.v1";
const OUTPUT_CONTRACT_DOMAIN: &str = "animus.multi-role-smoke.output-contract.v1";
const EVIDENCE_DOMAIN: &str = "animus.multi-role-smoke.evidence.v1";
const VERIFIER_DOMAIN: &str = "animus.multi-role-smoke.verifier.v1";
const RUBRIC_DOMAIN: &str = "animus.multi-role-smoke.rubric.v1";
const SUBJECT_ORIGIN: &str = "subject-origin";
const BOUNDED_CONTEXT: &str = "bounded-context";
const CANDIDATE_EDIT: &str = "candidate-edit";
const INSPECT_CONTEXT: &str = "inspect_context";
const IMPLEMENTER_INDEX: usize = 0;
const REVIEWER_INDEX: usize = 1;
const VERIFIER_INDEX: usize = 2;
const JUDGE_INDEX: usize = 3;
const STATE_DIMENSIONS: &[AgentRoleSeparationDimension] = &[
    AgentRoleSeparationDimension::CleanContext,
    AgentRoleSeparationDimension::SubjectImmutable,
    AgentRoleSeparationDimension::VerifierImmutable,
    AgentRoleSeparationDimension::RubricImmutable,
];

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum JudgeDecision {
    #[cfg(feature = "authoritative-completion-smoke")]
    Select,
    Veto,
}

#[cfg(feature = "authoritative-completion-smoke")]
impl JudgeDecision {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Select => "select",
            Self::Veto => "veto",
        }
    }
}

#[derive(Clone, Copy)]
struct RoleSpec {
    role_id: &'static str,
    effect: AgentRoleOutputEffect,
    output_ability: &'static str,
}

const ROLE_SPECS: &[RoleSpec] = &[
    RoleSpec {
        role_id: IMPLEMENTER,
        effect: AgentRoleOutputEffect::Candidate,
        output_ability: AGENT_ROLE_EMIT_CANDIDATE_ABILITY,
    },
    RoleSpec {
        role_id: REVIEWER,
        effect: AgentRoleOutputEffect::Finding,
        output_ability: AGENT_ROLE_EMIT_REVIEW_ABILITY,
    },
    RoleSpec {
        role_id: VERIFIER,
        effect: AgentRoleOutputEffect::CheckEvidence,
        output_ability: AGENT_ROLE_EMIT_VERIFICATION_ABILITY,
    },
    RoleSpec {
        role_id: JUDGE,
        effect: AgentRoleOutputEffect::Veto,
        output_ability: AGENT_ROLE_VETO_ABILITY,
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct RoleRecord {
    pub role_id: String,
    pub role_class: &'static str,
    pub model_profile_id: String,
    pub model_plan_hash: String,
    pub assignment_hash: String,
    pub separation_hash: String,
    pub effective_capability_count: usize,
    pub output_kind: &'static str,
    pub output_effect: &'static str,
    pub output_hash: String,
    pub receipt_hash: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct BoundaryReport {
    pub reviewer_mutation_rejected: bool,
    pub completion_claim_rejected: bool,
    pub completion_candidate_rejected: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct WorkflowReport {
    pub roles: Vec<RoleRecord>,
    pub boundaries: BoundaryReport,
    pub authoritative_completion: bool,
}

#[derive(Clone, Copy)]
pub(super) struct JudgeWorkflowInput<'a> {
    pub roles: &'a AgentRoleCatalog,
    pub model_profiles: &'a ModelRuntimeProfileSet,
    pub subject_hash: &'a str,
    pub fixture: SmokeFixture,
    pub judge_decision: JudgeDecision,
}

struct WorkflowContext<'a> {
    catalog: &'a AgentRoleCatalog,
    model_profiles: &'a ModelRuntimeProfileSet,
    subject: &'a str,
    fixture: SmokeFixture,
}

#[derive(Clone, Copy)]
struct RoleBuildRequest<'a> {
    context: &'a WorkflowContext<'a>,
    spec: RoleSpec,
    origin: &'a RoleOrigin,
    evidence: Option<&'a str>,
}

struct RoleActivation<'a> {
    profile: &'a AgentRoleProfile,
    model_plan: AgentRoleBoundModelPlan,
    bindings: RoleBindings,
    separation: AgentRoleSeparationDecision,
    assignment: AgentRoleAssignment,
    capabilities: AgentRoleCapabilityResolution,
    role_plan: AgentRolePlan,
}

struct BuiltRole {
    record: RoleRecord,
    origin: RoleOrigin,
    completion_claim_rejected: bool,
    completion_candidate_rejected: bool,
}

#[derive(Clone, Debug)]
struct RoleOrigin {
    assignment: String,
    actor: String,
    run_thread: String,
    session: String,
    model_profile: String,
    workspace: String,
    mutation_lineage: String,
}

#[derive(Clone, Debug)]
struct RoleBindings {
    assignment_nonce: String,
    task: String,
    actor: String,
    parent_lineage: String,
    run_thread: String,
    session: String,
    workspace: String,
    mutation_lineage: String,
    agent_definition: String,
    grant_set: String,
    freshness: String,
    output_contract: String,
    evidence: String,
    verifier: String,
    rubric: String,
}

#[derive(Clone, Copy)]
struct AssignmentInput<'a> {
    profile: &'a AgentRoleProfile,
    model_plan: &'a AgentRoleBoundModelPlan,
    separation: &'a AgentRoleSeparationDecision,
    subject: &'a str,
    bindings: &'a RoleBindings,
    resource: &'a str,
}

#[derive(Clone, Copy)]
struct SeparationInput<'a> {
    catalog: &'a AgentRoleCatalog,
    profile: &'a AgentRoleProfile,
    model_plan: &'a AgentRoleBoundModelPlan,
    subject: &'a str,
    origin: &'a RoleOrigin,
    bindings: &'a RoleBindings,
    fixture: SmokeFixture,
}

#[derive(Clone, Copy)]
struct OutputContext<'a> {
    spec: RoleSpec,
    activation: &'a RoleActivation<'a>,
    fixture: SmokeFixture,
}

pub(super) fn run_workflow(
    roles: &AgentRoleCatalog,
    model_profiles: &ModelRuntimeProfileSet,
    subject_hash: &str,
    fixture: SmokeFixture,
) -> Result<WorkflowReport, String> {
    run_workflow_with_judge(JudgeWorkflowInput {
        roles,
        model_profiles,
        subject_hash,
        fixture,
        judge_decision: JudgeDecision::Veto,
    })
}

pub(super) fn run_workflow_with_judge(
    input: JudgeWorkflowInput<'_>,
) -> Result<WorkflowReport, String> {
    // r[impl agent_harness_runtime.multi_role_host_smoke.activation]
    // r[impl agent_harness_runtime.multi_role_host_smoke.separation]
    // r[impl agent_harness_runtime.multi_role_host_smoke.outputs]
    let context = WorkflowContext {
        catalog: input.roles,
        model_profiles: input.model_profiles,
        subject: input.subject_hash,
        fixture: input.fixture,
    };
    let genesis = genesis_origin();
    let implementer = build_role(RoleBuildRequest {
        context: &context,
        spec: ROLE_SPECS[IMPLEMENTER_INDEX],
        origin: &genesis,
        evidence: None,
    })?;
    let reviewer = build_role(RoleBuildRequest {
        context: &context,
        spec: ROLE_SPECS[REVIEWER_INDEX],
        origin: &implementer.origin,
        evidence: None,
    })?;
    let verifier = build_role(RoleBuildRequest {
        context: &context,
        spec: ROLE_SPECS[VERIFIER_INDEX],
        origin: &reviewer.origin,
        evidence: None,
    })?;
    let judge = build_role(RoleBuildRequest {
        context: &context,
        spec: judge_role_spec(input.judge_decision),
        origin: &reviewer.origin,
        evidence: Some(verifier.record.output_hash.as_str()),
    })?;

    Ok(WorkflowReport {
        roles: vec![
            implementer.record,
            reviewer.record,
            verifier.record,
            judge.record,
        ],
        boundaries: BoundaryReport {
            reviewer_mutation_rejected: true,
            completion_claim_rejected: judge.completion_claim_rejected,
            completion_candidate_rejected: judge.completion_candidate_rejected,
        },
        authoritative_completion: false,
    })
}

const fn judge_role_spec(decision: JudgeDecision) -> RoleSpec {
    match decision {
        #[cfg(feature = "authoritative-completion-smoke")]
        JudgeDecision::Select => RoleSpec {
            role_id: JUDGE,
            effect: AgentRoleOutputEffect::Select,
            output_ability: AGENT_ROLE_SELECT_ABILITY,
        },
        JudgeDecision::Veto => ROLE_SPECS[JUDGE_INDEX],
    }
}

fn build_role(request: RoleBuildRequest<'_>) -> Result<BuiltRole, String> {
    let activation = activate_role(&request)?;
    enforce_reviewer_mutation_boundary(request.spec, &activation, request.context.fixture)?;
    let output = admit_output(OutputContext {
        spec: request.spec,
        activation: &activation,
        fixture: request.context.fixture,
    })?;
    let role_receipt = project_agent_role_receipt(
        AgentRoleReceiptInput {
            kind: AgentRoleReceiptKind::Output,
            assignment: &activation.assignment,
            profile: activation.profile,
            capability_resolution: Some(&activation.capabilities),
            separation: Some(&activation.separation),
            role_plan: Some(&activation.role_plan),
            handoff: None,
            output: Some(&output),
        },
        AgentRoleReceiptSensitiveDraft::default(),
    )
    .map_err(|_failure| error(ERROR_RECEIPT))?;
    let completion_boundary = if request.spec.role_id == JUDGE {
        completion_boundaries_reject(&activation, &output)?
    } else {
        (true, true)
    };
    let origin = role_origin(&activation);
    let record = role_record(&activation, &output, role_receipt.receipt_hash());
    Ok(BuiltRole {
        record,
        origin,
        completion_claim_rejected: completion_boundary.0,
        completion_candidate_rejected: completion_boundary.1,
    })
}

fn activate_role<'a>(request: &'a RoleBuildRequest<'a>) -> Result<RoleActivation<'a>, String> {
    let profile = request
        .context
        .catalog
        .profile(request.spec.role_id)
        .ok_or_else(|| error(ERROR_ROLE_MISSING))?;
    let model_plan = compose_role_model_plan(request.context.model_profiles, profile)?;
    let bindings = role_bindings(
        request.spec.role_id,
        profile.output_contract(),
        request.origin,
        request.evidence,
    );
    let separation = separation_decision(SeparationInput {
        catalog: request.context.catalog,
        profile,
        model_plan: &model_plan,
        subject: request.context.subject,
        origin: request.origin,
        bindings: &bindings,
        fixture: request.context.fixture,
    })?;
    if !separation.passed() {
        return Err(error(ERROR_SEPARATION));
    }
    let resource = agent_role_activation_resource(AgentRoleActivationResourceInput {
        profile,
        model_plan: &model_plan,
        separation: &separation,
        subject_hash: request.context.subject,
        task_hash: bindings.task.as_str(),
        actor_hash: bindings.actor.as_str(),
        parent_lineage_hash: bindings.parent_lineage.as_str(),
        run_thread_hash: bindings.run_thread.as_str(),
        session_hash: bindings.session.as_str(),
        workspace_hash: bindings.workspace.as_str(),
        mutation_lineage_hash: bindings.mutation_lineage.as_str(),
        agent_definition_hash: bindings.agent_definition.as_str(),
        grant_set_hash: bindings.grant_set.as_str(),
        freshness_hash: bindings.freshness.as_str(),
        output_contract_hash: bindings.output_contract.as_str(),
    })
    .map_err(|_failure| error(ERROR_ACTIVATION))?;
    let assignment = assign_role(AssignmentInput {
        profile,
        model_plan: &model_plan,
        separation: &separation,
        subject: request.context.subject,
        bindings: &bindings,
        resource: resource.as_str(),
    })?;
    let role_ceiling = typed_role_ceiling(profile)?;
    let model_ceiling = model_capability_ceiling(role_ceiling.as_slice(), &model_plan);
    let capabilities = resolve_capabilities(
        &assignment,
        profile,
        role_ceiling.as_slice(),
        model_ceiling.as_slice(),
    )?;
    let role_plan = build_agent_role_plan(AgentRolePlanInput {
        assignment: &assignment,
        profile,
        model_plan: &model_plan,
        capabilities: &capabilities,
        separation: &separation,
    })
    .map_err(|_failure| error(ERROR_ACTIVATION))?;
    Ok(RoleActivation {
        profile,
        model_plan,
        bindings,
        separation,
        assignment,
        capabilities,
        role_plan,
    })
}

fn assign_role(input: AssignmentInput<'_>) -> Result<AgentRoleAssignment, String> {
    let (assignment_request, assignment_receipt) =
        authorized_role_request(input.resource, AGENT_ROLE_ASSIGN_ABILITY, true)?;
    let (activation_request, activation_receipt) =
        authorized_role_request(input.resource, AGENT_ROLE_ACTIVATE_ABILITY, true)?;
    admit_agent_role_assignment(AgentRoleAssignmentRequest {
        profile: input.profile,
        model_plan: input.model_plan,
        separation: input.separation,
        subject_hash: input.subject,
        task_hash: input.bindings.task.as_str(),
        actor_hash: input.bindings.actor.as_str(),
        parent_lineage_hash: input.bindings.parent_lineage.as_str(),
        run_thread_hash: input.bindings.run_thread.as_str(),
        session_hash: input.bindings.session.as_str(),
        workspace_hash: input.bindings.workspace.as_str(),
        mutation_lineage_hash: input.bindings.mutation_lineage.as_str(),
        agent_definition_hash: input.bindings.agent_definition.as_str(),
        grant_set_hash: input.bindings.grant_set.as_str(),
        freshness_hash: input.bindings.freshness.as_str(),
        output_contract_hash: input.bindings.output_contract.as_str(),
        assignment_request: &assignment_request,
        assignment_receipt: &assignment_receipt,
        activation_request: &activation_request,
        activation_receipt: &activation_receipt,
    })
    .map_err(|_failure| error(ERROR_ACTIVATION))
}

fn separation_decision(input: SeparationInput<'_>) -> Result<AgentRoleSeparationDecision, String> {
    let policy = input
        .catalog
        .separation_policy(input.profile.separation_policy_ref())
        .ok_or_else(|| error(ERROR_SEPARATION))?;
    let actor = if input.profile.role_id() == REVIEWER
        && input.fixture == SmokeFixture::CollapseReviewerActor
    {
        input.origin.actor.as_str()
    } else {
        input.bindings.actor.as_str()
    };
    let expected_verifier =
        (input.profile.role_id() == VERIFIER).then_some(input.bindings.verifier.as_str());
    let expected_rubric =
        (input.profile.role_id() == JUDGE).then_some(input.bindings.rubric.as_str());
    evaluate_agent_role_separation(
        policy,
        &AgentRoleSeparationObservation {
            candidate_assignment_hash: input.bindings.assignment_nonce.as_str(),
            subject_assignment_hash: input.origin.assignment.as_str(),
            candidate_actor_hash: actor,
            subject_actor_hash: input.origin.actor.as_str(),
            candidate_run_thread_hash: input.bindings.run_thread.as_str(),
            subject_run_thread_hash: input.origin.run_thread.as_str(),
            candidate_session_hash: input.bindings.session.as_str(),
            subject_session_hash: input.origin.session.as_str(),
            candidate_model_profile_hash: input.model_plan.plan().profile_hash(),
            subject_model_profile_hash: input.origin.model_profile.as_str(),
            candidate_workspace_hash: input.bindings.workspace.as_str(),
            subject_workspace_hash: input.origin.workspace.as_str(),
            candidate_mutation_lineage_hash: input.bindings.mutation_lineage.as_str(),
            subject_mutation_lineage_hash: input.origin.mutation_lineage.as_str(),
            subject_hash: input.subject,
            observed_subject_hash: input.subject,
            expected_verifier_hash: expected_verifier,
            observed_verifier_hash: expected_verifier,
            expected_rubric_hash: expected_rubric,
            observed_rubric_hash: expected_rubric,
            policy_hash: input.profile.policy_hash(),
            observed_policy_hash: input.profile.policy_hash(),
            satisfied_state_dimensions: STATE_DIMENSIONS,
        },
    )
    .map_err(|_failure| error(ERROR_SEPARATION))
}

fn typed_role_ceiling(profile: &AgentRoleProfile) -> Result<Vec<TypedSubagentCapability>, String> {
    profile
        .capability_ceiling()
        .iter()
        .map(|label| match label.as_str() {
            "context_read" => Ok(context_capability()),
            "mutation_intent" => Ok(mutation_capability()),
            "tool_use" => Ok(tool_capability()),
            _unsupported => Err(error(ERROR_CAPABILITY)),
        })
        .collect()
}

fn model_capability_ceiling(
    role_ceiling: &[TypedSubagentCapability],
    model_plan: &AgentRoleBoundModelPlan,
) -> Vec<TypedSubagentCapability> {
    role_ceiling
        .iter()
        .filter(|capability| {
            !matches!(capability, TypedSubagentCapability::ToolUse { .. })
                || !model_plan.plan().tools().is_empty()
        })
        .cloned()
        .collect()
}

fn resolve_capabilities(
    assignment: &AgentRoleAssignment,
    profile: &AgentRoleProfile,
    role_ceiling: &[TypedSubagentCapability],
    model_ceiling: &[TypedSubagentCapability],
) -> Result<AgentRoleCapabilityResolution, String> {
    resolve_agent_role_capabilities(AgentRoleCapabilityResolutionInput {
        assignment,
        profile,
        requested: role_ceiling,
        role_ceiling,
        ceilings: AgentRoleCapabilityCeilings {
            verified_grants: role_ceiling,
            host_policy: role_ceiling,
            parent_delegation: role_ceiling,
            task_scope: role_ceiling,
            agent_definition: role_ceiling,
            model_profile: model_ceiling,
            workspace: role_ceiling,
            tool_executor: role_ceiling,
            approval: role_ceiling,
            sandbox: role_ceiling,
            cancellation: role_ceiling,
            timeout: role_ceiling,
            budget: role_ceiling,
        },
    })
    .map_err(|_failure| error(ERROR_CAPABILITY))
}

fn enforce_reviewer_mutation_boundary(
    spec: RoleSpec,
    activation: &RoleActivation<'_>,
    fixture: SmokeFixture,
) -> Result<(), String> {
    if spec.role_id != REVIEWER {
        return Ok(());
    }
    let rejected = admit_agent_role_capability_use(
        &activation.assignment,
        &activation.capabilities,
        &mutation_capability(),
    )
    .is_err();
    if rejected && fixture != SmokeFixture::RequireReviewerMutation {
        return Ok(());
    }
    Err(error(ERROR_REVIEWER_MUTATION))
}

fn admit_output(context: OutputContext<'_>) -> Result<AgentRoleOutput, String> {
    let activation = context.activation;
    let verifier =
        (context.spec.role_id == VERIFIER).then_some(activation.bindings.verifier.as_str());
    let rubric = (context.spec.role_id == JUDGE).then_some(activation.bindings.rubric.as_str());
    let admission = output_admission(context.spec.role_id);
    let resource = agent_role_output_resource(AgentRoleOutputResourceInput {
        assignment: &activation.assignment,
        profile: activation.profile,
        role_plan: &activation.role_plan,
        capability_resolution: &activation.capabilities,
        separation: &activation.separation,
        status: AgentRoleOutputStatus::Admitted,
        effect: context.spec.effect,
        evidence_hash: activation.bindings.evidence.as_str(),
        verifier_hash: verifier,
        rubric_hash: rubric,
        admission,
    })
    .map_err(|_failure| error(ERROR_OUTPUT))?;
    let allow_output =
        !(context.spec.role_id == JUDGE && context.fixture == SmokeFixture::DenyJudgeVeto);
    let (request, receipt) =
        authorized_role_request(resource.as_str(), context.spec.output_ability, allow_output)?;
    let completion_claims = [AgentRoleOutputClaim::AuthoritativeCompletion];
    let claims =
        if context.spec.role_id == JUDGE && context.fixture == SmokeFixture::ClaimCompletion {
            completion_claims.as_slice()
        } else {
            &[]
        };
    admit_agent_role_output(AgentRoleOutputDraft {
        assignment: &activation.assignment,
        profile: activation.profile,
        role_plan: &activation.role_plan,
        capability_resolution: &activation.capabilities,
        separation: &activation.separation,
        status: AgentRoleOutputStatus::Admitted,
        effect: context.spec.effect,
        evidence_hash: activation.bindings.evidence.as_str(),
        verifier_hash: verifier,
        rubric_hash: rubric,
        admission,
        claims,
        output_request: &request,
        output_receipt: &receipt,
        sensitive: &[],
    })
    .map_err(|_failure| error(ERROR_OUTPUT))
}

fn completion_boundaries_reject(
    activation: &RoleActivation<'_>,
    output: &AgentRoleOutput,
) -> Result<(bool, bool), String> {
    // r[impl agent_harness_runtime.multi_role_host_smoke.completion_boundary]
    let admission = output_admission(JUDGE);
    let resource = agent_role_output_resource(AgentRoleOutputResourceInput {
        assignment: &activation.assignment,
        profile: activation.profile,
        role_plan: &activation.role_plan,
        capability_resolution: &activation.capabilities,
        separation: &activation.separation,
        status: AgentRoleOutputStatus::Admitted,
        effect: AgentRoleOutputEffect::Veto,
        evidence_hash: activation.bindings.evidence.as_str(),
        verifier_hash: None,
        rubric_hash: Some(activation.bindings.rubric.as_str()),
        admission,
    })
    .map_err(|_failure| error(ERROR_COMPLETION_BOUNDARY))?;
    let (request, receipt) =
        authorized_role_request(resource.as_str(), AGENT_ROLE_VETO_ABILITY, true)?;
    let claims = [AgentRoleOutputClaim::AuthoritativeCompletion];
    let claim_rejected = admit_agent_role_output(AgentRoleOutputDraft {
        assignment: &activation.assignment,
        profile: activation.profile,
        role_plan: &activation.role_plan,
        capability_resolution: &activation.capabilities,
        separation: &activation.separation,
        status: AgentRoleOutputStatus::Admitted,
        effect: AgentRoleOutputEffect::Veto,
        evidence_hash: activation.bindings.evidence.as_str(),
        verifier_hash: None,
        rubric_hash: Some(activation.bindings.rubric.as_str()),
        admission,
        claims: &claims,
        output_request: &request,
        output_receipt: &receipt,
        sensitive: &[],
    })
    .is_err();
    let candidate_rejected = admit_completion_candidate(&AuthoritativeCompletionCandidateDraft {
        source: CompletionCandidateSource::Typed,
        request: output.output_hash(),
        work_id: WORK_ID,
        subject_hash: activation.assignment.subject_hash(),
    })
    .is_err();
    if !claim_rejected || !candidate_rejected {
        return Err(error(ERROR_COMPLETION_BOUNDARY));
    }
    Ok((claim_rejected, candidate_rejected))
}

fn role_origin(activation: &RoleActivation<'_>) -> RoleOrigin {
    RoleOrigin {
        assignment: activation.assignment.assignment_hash().to_string(),
        actor: activation.assignment.actor_hash().to_string(),
        run_thread: activation.bindings.run_thread.clone(),
        session: activation.bindings.session.clone(),
        model_profile: activation.model_plan.plan().profile_hash().to_string(),
        workspace: activation.bindings.workspace.clone(),
        mutation_lineage: activation.bindings.mutation_lineage.clone(),
    }
}

fn role_record(
    activation: &RoleActivation<'_>,
    output: &AgentRoleOutput,
    receipt_hash: &str,
) -> RoleRecord {
    RoleRecord {
        role_id: activation.profile.role_id().to_string(),
        role_class: activation.profile.class().as_str(),
        model_profile_id: activation.model_plan.plan().profile_id().to_string(),
        model_plan_hash: activation.model_plan.binding_hash().to_string(),
        assignment_hash: activation.assignment.assignment_hash().to_string(),
        separation_hash: activation.separation.decision_hash().to_string(),
        effective_capability_count: activation.capabilities.effective().len(),
        output_kind: output.output_kind().as_str(),
        output_effect: output.effect().as_str(),
        output_hash: output.output_hash().to_string(),
        receipt_hash: receipt_hash.to_string(),
    }
}

fn output_admission(role_id: &str) -> AgentRoleOutputAdmission {
    if role_id == VERIFIER {
        AgentRoleOutputAdmission {
            deterministic_policy: AgentRoleDeterministicPolicyState::Clear,
            host_evidence: AgentRoleAdmissionState::Admitted,
            verifier: AgentRoleAdmissionState::Admitted,
            rubric: AgentRoleAdmissionState::NotAdmitted,
        }
    } else if role_id == JUDGE {
        AgentRoleOutputAdmission {
            deterministic_policy: AgentRoleDeterministicPolicyState::Clear,
            host_evidence: AgentRoleAdmissionState::NotAdmitted,
            verifier: AgentRoleAdmissionState::NotAdmitted,
            rubric: AgentRoleAdmissionState::Admitted,
        }
    } else {
        AgentRoleOutputAdmission {
            deterministic_policy: AgentRoleDeterministicPolicyState::Clear,
            host_evidence: AgentRoleAdmissionState::NotAdmitted,
            verifier: AgentRoleAdmissionState::NotAdmitted,
            rubric: AgentRoleAdmissionState::NotAdmitted,
        }
    }
}

fn authorized_role_request(
    resource: &str,
    ability: &str,
    allowed: bool,
) -> Result<
    (
        animus_adapters::CapabilityRequest,
        animus_adapters::AuthorizationReceipt,
    ),
    String,
> {
    let request = animus_adapters::CapabilityRequest::new(
        animus_adapters::GuardedAction::AgentRole,
        AGENT_ROLE_CAPABILITY_CONTRACT,
        resource,
        ability,
    )
    .map_err(|_failure| error(ERROR_ACTIVATION))?;
    let contract = animus_adapters::CapabilityPolicyContract::new(
        animus_adapters::GuardedAction::AgentRole,
        AGENT_ROLE_CAPABILITY_CONTRACT,
        AUTH_RESOURCE_PREFIX,
        ability,
    )
    .map_err(|_failure| error(ERROR_ACTIVATION))?;
    let grants = if allowed {
        vec![
            animus_adapters::UcanGrant::new(
                AGENT_ROLE_CAPABILITY_CONTRACT,
                AUTH_RESOURCE_PREFIX,
                ability,
            )
            .map_err(|_failure| error(ERROR_ACTIVATION))?,
        ]
    } else {
        Vec::new()
    };
    let receipt = animus_adapters::authorize_capability(&animus_adapters::AuthorizationContext {
        request: &request,
        contract: &contract,
        grants: grants.as_slice(),
        caveats: &[],
        sandbox: &animus_adapters::SandboxConstraint::NotRequired,
    });
    Ok((request, receipt))
}

fn role_bindings(
    role_id: &str,
    output_contract: &str,
    origin: &RoleOrigin,
    evidence: Option<&str>,
) -> RoleBindings {
    let evidence = if let Some(bound_evidence) = evidence {
        bound_evidence.to_string()
    } else {
        identity_hash(role_id, EVIDENCE_DOMAIN)
    };
    RoleBindings {
        assignment_nonce: identity_hash(role_id, "assignment-nonce"),
        task: identity_hash(role_id, "task"),
        actor: identity_hash(role_id, "actor"),
        parent_lineage: origin.assignment.clone(),
        run_thread: identity_hash(role_id, "run-thread"),
        session: identity_hash(role_id, "session"),
        workspace: identity_hash(role_id, "workspace"),
        mutation_lineage: identity_hash(role_id, "mutation-lineage"),
        agent_definition: identity_hash(role_id, "agent-definition"),
        grant_set: identity_hash(role_id, "grant-set"),
        freshness: identity_hash(role_id, "freshness"),
        output_contract: domain_hash(OUTPUT_CONTRACT_DOMAIN, output_contract),
        evidence,
        verifier: identity_hash(role_id, VERIFIER_DOMAIN),
        rubric: identity_hash(role_id, RUBRIC_DOMAIN),
    }
}

fn identity_hash(role_id: &str, label: &str) -> String {
    domain_hash(IDENTITY_DOMAIN, format!("{role_id}:{label}").as_str())
}

fn genesis_origin() -> RoleOrigin {
    RoleOrigin {
        assignment: identity_hash(SUBJECT_ORIGIN, "assignment"),
        actor: identity_hash(SUBJECT_ORIGIN, "actor"),
        run_thread: identity_hash(SUBJECT_ORIGIN, "run-thread"),
        session: identity_hash(SUBJECT_ORIGIN, "session"),
        model_profile: identity_hash(SUBJECT_ORIGIN, "model-profile"),
        workspace: identity_hash(SUBJECT_ORIGIN, "workspace"),
        mutation_lineage: identity_hash(SUBJECT_ORIGIN, "mutation-lineage"),
    }
}

fn context_capability() -> TypedSubagentCapability {
    TypedSubagentCapability::ContextRead {
        stable_label: "context_read".to_string(),
        context_label: BOUNDED_CONTEXT.to_string(),
    }
}

fn mutation_capability() -> TypedSubagentCapability {
    TypedSubagentCapability::MutationIntent {
        stable_label: "mutation_intent".to_string(),
        mutation_kind: CANDIDATE_EDIT.to_string(),
    }
}

fn tool_capability() -> TypedSubagentCapability {
    TypedSubagentCapability::ToolUse {
        stable_label: "tool_use".to_string(),
        tool_label: INSPECT_CONTEXT.to_string(),
    }
}

const _: [(); EXPECTED_ROLE_COUNT] = [(); ROLE_SPECS.len()];
const EXPECTED_ROLE_COUNT: usize = 4;
