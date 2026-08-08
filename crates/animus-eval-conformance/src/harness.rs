use animus_adapter_eval::{
    CoalescingEvidence, EvaluationBinding, GatewayError, ModelCallProjectionDraft,
    ServiceAvailability, ServiceCapability, ServiceCatalog, ServiceDescriptor, ServiceKind,
    ServiceRequirement, ServiceResourceBounds, TaskSourceRequest, TurnCommitEvidence,
    admit_identical_request_coalescing, admit_service, admit_terminal_after_commit,
    build_service_request, project_model_call,
};
use animus_eval_core::{
    EVALUATION_POLICY_SCHEMA, EnvironmentPlanRequest, EpisodeError, EpisodeRunObservation,
    EvaluationNetworkPosture, EvaluationPolicy, EvaluationPolicyDraft, EvaluationReceiptClaim,
    EvaluationReceiptDraft, EvaluationReceiptStatus, LedgerTraceReference, MissingRewardPolicy,
    PassAtNObservation, ReceiptError, RecipeBindings, RecipeError, RecipeKind, RecipeRequest,
    ResumeAction, ResumeEvidence, ResumeEvidenceState, ResumeMaterial, ResumeMismatchPolicy,
    RewardClaim, RewardContribution, RewardDimension, RewardError, TaskDescriptor, TaskOrdering,
    TasksetDescriptor, TasksetError, TasksetFiniteness, TasksetSource, TerminalObservationClass,
    build_resume_key, evaluate_resume, hash_identity, plan_environment, plan_recipe, plan_taskset,
    project_evaluation_receipt, reduce_episode, reduce_pass_at_n, reduce_reward_dimensions,
    validate_evaluation_policy,
};
use animus_runtime::harbor_eval::{
    AtifError, AtifSource, AtifStep, AtifTrajectory, NetworkPosture, PlanError, Reward, Route,
    Task, project_atif, reduce_rewards,
};
use animus_runtime::{
    DatasetExportPolicy, DatasetMetadataProjection, DatasetReplayEntry, DatasetReplayRole,
    DatasetSessionSource, DatasetTarget, EvalFeedbackCandidate, EvalFeedbackCandidateKind,
    EvalFeedbackFingerprint, EvalFeedbackOutputFormat, EvalFeedbackPlan, EvalFeedbackReviewOutcome,
    EvalFeedbackTaskSet, EvalFeedbackTaskSetKind, EvalFeedbackVerifierIdentity,
    blake3_eval_feedback_fingerprint, evaluate_feedback_review, export_chat_messages_row,
    validate_eval_feedback_plan,
};

use crate::{ValidatedConformanceProfile, hash_parts, host_fixture};

const EXPECTED_FAMILIES: u32 = 11;
const EXPECTED_POSITIVE: u32 = 11;
const EXPECTED_NEGATIVE: u32 = 11;
const PAIR: u32 = 2;
const ONE: u32 = 1;
const ONE_SCORE: f64 = 1.0;
const OUTSIDE_SCORE: f64 = 2.0;
const TASKSET_SEED: u64 = 7;
const ONE_INDEX: usize = 1;
const MAX_TASKS: u32 = 32;
const MAX_RUNS: u32 = 16;
const MAX_REWARDS: u32 = 8;
const MAX_ARTIFACTS: u32 = 4;
const SERVICE_ITEMS: u32 = 8;
const SERVICE_BYTES: u64 = 4_096;
const SOURCE_IDENTITY: &str = "b3:0000000000000000000000000000000000000000000000000000000000000000";
const POLICY_IDENTITY: &str = "policy-conformance-v1";

type FamilyRunner = fn() -> Result<(), &'static str>;

/// One bounded conformance diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceDiagnostic {
    /// Mechanism family.
    pub family: String,
    /// Expected stable diagnostic label.
    pub expected: String,
    /// Observed stable diagnostic label.
    pub observed: String,
}

/// Aggregate conformance status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceStatus {
    /// Every positive and expected-negative fixture passed.
    Passed,
    /// At least one fixture did not reach its expected bounded outcome.
    Failed,
    /// Profile or dependency admission blocked fixture execution.
    Blocked,
}

/// Metadata-only aggregate conformance summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConformanceSummary {
    /// Profile identity.
    pub profile_identity: String,
    /// Ordered family identities.
    pub family_identities: Vec<String>,
    /// Ordered fixture identities.
    pub fixture_identities: Vec<String>,
    /// Mechanism family count.
    pub family_count: u32,
    /// Positive fixture count.
    pub positive_count: u32,
    /// Expected-negative fixture count.
    pub negative_count: u32,
    /// Covered public entrypoint count.
    pub public_entrypoint_count: u32,
    /// Explicit deterministic service count.
    pub service_count: u32,
    /// Expected diagnostic count.
    pub expected_diagnostic_count: u32,
    /// Aggregate status.
    pub status: ConformanceStatus,
    /// Evidence redaction state.
    pub redaction: &'static str,
    /// Explicit non-claims.
    pub non_claims: Vec<String>,
    /// Domain-separated summary identity.
    pub identity: String,
}

/// Conformance evidence validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceEvidenceError {
    /// A private data class was declared present.
    Leak,
    /// A live, correctness, completion, production, or release claim was requested.
    Overclaim,
    /// A summary count, identity, status, or redaction field was malformed.
    Malformed,
}

/// Negative-only private-material flags.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct ConformanceLeakFlags {
    /// Task or fixture body appeared.
    pub fixture_body: bool,
    /// Transcript appeared.
    pub transcript: bool,
    /// Hidden test appeared.
    pub hidden_test: bool,
    /// Reasoning appeared.
    pub reasoning: bool,
    /// Provider or tool body appeared.
    pub effect_body: bool,
    /// Secret value appeared.
    pub secret_value: bool,
    /// Environment value appeared.
    pub environment_value: bool,
    /// Physical path appeared.
    pub physical_path: bool,
    /// Unrestricted log appeared.
    pub unrestricted_log: bool,
}

/// Claims accepted by conformance evidence validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceClaim {
    /// The selected deterministic fixture matrix was observed under named bounds.
    BoundedFixtureObservation,
    /// A live host was correct.
    LiveHostCorrectness,
    /// Verifier soundness was proved.
    VerifierSoundness,
    /// Benchmark validity was proved.
    BenchmarkValidity,
    /// Authoritative completion was established.
    AuthoritativeCompletion,
    /// Production readiness was established.
    ProductionReadiness,
    /// Release eligibility was established.
    ReleaseEligibility,
}

/// Execute the complete positive and expected-negative matrix through public production APIs.
///
/// # Errors
/// Returns bounded family diagnostics when any fixture diverges from its expected outcome.
pub fn run_evaluation_conformance(
    validated: &ValidatedConformanceProfile,
) -> Result<ConformanceSummary, Vec<ConformanceDiagnostic>> {
    let runners: [FamilyRunner; EXPECTED_FAMILIES as usize] = [
        run_taskset_family,
        run_recipe_family,
        run_episode_family,
        run_port_family,
        run_gateway_family,
        run_verifier_family,
        run_reward_family,
        run_persistence_family,
        run_compatibility_family,
        run_downstream_family,
        run_evidence_family,
    ];
    let mut diagnostics = Vec::new();
    for (family, runner) in validated.profile.families.iter().zip(runners) {
        if let Err(observed) = runner() {
            diagnostics.push(ConformanceDiagnostic {
                family: family.id.clone(),
                expected: family.expected_diagnostic.clone(),
                observed: observed.into(),
            });
        }
    }
    if !diagnostics.is_empty() {
        return Err(diagnostics);
    }
    let family_count = u32::try_from(validated.family_identities.len())
        .map_err(|_| malformed_diagnostic("family-count"))?;
    let fixture_count = u32::try_from(validated.fixture_identities.len())
        .map_err(|_| malformed_diagnostic("fixture-count"))?;
    let public_entrypoint_count = u32::try_from(validated.profile.public_entrypoints.len())
        .map_err(|_| malformed_diagnostic("entrypoint-count"))?;
    let service_count = u32::try_from(validated.profile.services.len())
        .map_err(|_| malformed_diagnostic("service-count"))?;
    let expected_diagnostic_count = u32::try_from(validated.profile.families.len())
        .map_err(|_| malformed_diagnostic("diagnostic-count"))?;
    let identity = hash_parts(
        "animus.native-evaluation-conformance.summary.v1",
        &[
            validated.identity.as_str(),
            family_count.to_string().as_str(),
            fixture_count.to_string().as_str(),
            public_entrypoint_count.to_string().as_str(),
            service_count.to_string().as_str(),
            "passed",
            "metadata_only",
        ],
    );
    let summary = ConformanceSummary {
        profile_identity: validated.identity.clone(),
        family_identities: validated.family_identities.clone(),
        fixture_identities: validated.fixture_identities.clone(),
        family_count,
        positive_count: EXPECTED_POSITIVE,
        negative_count: EXPECTED_NEGATIVE,
        public_entrypoint_count,
        service_count,
        expected_diagnostic_count,
        status: ConformanceStatus::Passed,
        redaction: "metadata_only",
        non_claims: validated.profile.evidence.non_claims.clone(),
        identity,
    };
    validate_conformance_summary(
        &summary,
        ConformanceLeakFlags::default(),
        &[ConformanceClaim::BoundedFixtureObservation],
    )?;
    Ok(summary)
}

/// Validate aggregate evidence without accepting private material or overclaims.
///
/// # Errors
/// Returns a leak, overclaim, or malformed class.
pub fn validate_conformance_summary(
    summary: &ConformanceSummary,
    leaks: ConformanceLeakFlags,
    claims: &[ConformanceClaim],
) -> Result<(), Vec<ConformanceDiagnostic>> {
    if leaks.fixture_body
        || leaks.transcript
        || leaks.hidden_test
        || leaks.reasoning
        || leaks.effect_body
        || leaks.secret_value
        || leaks.environment_value
        || leaks.physical_path
        || leaks.unrestricted_log
    {
        return Err(evidence_diagnostic("evidence-leak"));
    }
    if claims
        .iter()
        .any(|claim| *claim != ConformanceClaim::BoundedFixtureObservation)
    {
        return Err(evidence_diagnostic("evidence-overclaim"));
    }
    if summary.status != ConformanceStatus::Passed
        || summary.family_count != EXPECTED_FAMILIES
        || summary.positive_count != EXPECTED_POSITIVE
        || summary.negative_count != EXPECTED_NEGATIVE
        || summary.fixture_identities.len() != (EXPECTED_POSITIVE + EXPECTED_NEGATIVE) as usize
        || summary.redaction != "metadata_only"
        || !summary.profile_identity.starts_with("b3:")
        || !summary.identity.starts_with("b3:")
        || summary.non_claims.is_empty()
    {
        return Err(evidence_diagnostic("evidence-malformed"));
    }
    Ok(())
}

/// Render a bounded evidence summary without fixture content or physical paths.
#[must_use]
pub fn render_conformance_summary(summary: &ConformanceSummary) -> String {
    format!(
        "schema: {schema}\nprofile_identity: {profile}\nsummary_identity: {summary_id}\nstatus: PASS\nfamily_count: {families}\npositive_count: {positive}\nnegative_count: {negative}\npublic_entrypoint_count: {entrypoints}\nservice_count: {services}\nexpected_diagnostic_count: {diagnostics}\nredaction: {redaction}\nnon_claim_count: {non_claims}\n",
        schema = crate::CONFORMANCE_SCHEMA,
        profile = summary.profile_identity,
        summary_id = summary.identity,
        families = summary.family_count,
        positive = summary.positive_count,
        negative = summary.negative_count,
        entrypoints = summary.public_entrypoint_count,
        services = summary.service_count,
        diagnostics = summary.expected_diagnostic_count,
        redaction = summary.redaction,
        non_claims = summary.non_claims.len(),
    )
}

fn malformed_diagnostic(observed: &'static str) -> Vec<ConformanceDiagnostic> {
    vec![ConformanceDiagnostic {
        family: "matrix".into(),
        expected: "bounded-count".into(),
        observed: observed.into(),
    }]
}

fn evidence_diagnostic(observed: &'static str) -> Vec<ConformanceDiagnostic> {
    vec![ConformanceDiagnostic {
        family: "evidence".into(),
        expected: "metadata-only".into(),
        observed: observed.into(),
    }]
}

fn run_taskset_family() -> Result<(), &'static str> {
    let policy = core_policy(ResumeMismatchPolicy::Rerun);
    let tasks = [task("task-one"), task("task-two")];
    let members = ["task-one", "task-two"];
    let exact = plan_taskset(
        &TasksetDescriptor {
            taskset_identity: "taskset-exact",
            version: "v1",
            ordering: TaskOrdering::Stable,
            source: TasksetSource::Exact { members: &members },
        },
        &tasks,
        &policy,
    )
    .map_err(|_| "taskset-positive")?;
    let lazy = plan_taskset(
        &TasksetDescriptor {
            taskset_identity: "taskset-lazy",
            version: "v1",
            ordering: TaskOrdering::Stable,
            source: TasksetSource::Lazy {
                generator_identity: "generator-one",
                generator_version: "v1",
                seed: TASKSET_SEED,
                finiteness: TasksetFiniteness::Infinite,
                selection_bound: ONE,
            },
        },
        &tasks,
        &policy,
    )
    .map_err(|_| "taskset-positive")?;
    if exact.tasks.len() != PAIR as usize || lazy.tasks.len() != ONE as usize {
        return Err("taskset-positive");
    }
    let unsafe_shuffle = TasksetDescriptor {
        taskset_identity: "taskset-unsafe-shuffle",
        version: "v1",
        ordering: TaskOrdering::GlobalShuffle,
        source: TasksetSource::Lazy {
            generator_identity: "generator-one",
            generator_version: "v1",
            seed: TASKSET_SEED,
            finiteness: TasksetFiniteness::Infinite,
            selection_bound: ONE,
        },
    };
    if plan_taskset(&unsafe_shuffle, &tasks[..1], &policy) != Err(TasksetError::Ordering) {
        return Err("taskset-unsafe-shuffle");
    }
    let missing_bound = TasksetDescriptor {
        taskset_identity: "taskset-missing-bound",
        version: "v1",
        ordering: TaskOrdering::Stable,
        source: TasksetSource::Lazy {
            generator_identity: "generator-one",
            generator_version: "v1",
            seed: TASKSET_SEED,
            finiteness: TasksetFiniteness::Infinite,
            selection_bound: 0,
        },
    };
    if plan_taskset(&missing_bound, &tasks[..1], &policy) != Err(TasksetError::Bound) {
        return Err("taskset-missing-bound");
    }
    let unstable = TasksetDescriptor {
        taskset_identity: "taskset-unstable",
        version: "v1",
        ordering: TaskOrdering::Stable,
        source: TasksetSource::Lazy {
            generator_identity: "",
            generator_version: "v1",
            seed: TASKSET_SEED,
            finiteness: TasksetFiniteness::Finite,
            selection_bound: ONE,
        },
    };
    if plan_taskset(&unstable, &tasks[..1], &policy) != Err(TasksetError::Generator) {
        return Err("taskset-unstable-generator");
    }
    let duplicates = [task("task-one"), task("task-one")];
    let duplicate_members = ["task-one", "task-one"];
    let duplicate_descriptor = TasksetDescriptor {
        taskset_identity: "taskset-duplicates",
        version: "v1",
        ordering: TaskOrdering::Stable,
        source: TasksetSource::Exact {
            members: &duplicate_members,
        },
    };
    if plan_taskset(&duplicate_descriptor, &duplicates, &policy) != Err(TasksetError::Membership) {
        return Err("taskset-duplicate-task");
    }
    Ok(())
}

fn run_recipe_family() -> Result<(), &'static str> {
    let policy = core_policy(ResumeMismatchPolicy::Rerun);
    let environment = plan_environment(&EnvironmentPlanRequest {
        solver_environment_identity: "environment-solver",
        verifier_environment_identity: Some("environment-verifier"),
        resource_identity: "resources-one",
        network_identity: "network-none",
        require_verifier_isolation: true,
    })
    .map_err(|_| "recipe-environment")?;
    if !environment.verifier_isolated {
        return Err("recipe-environment");
    }
    for kind in recipe_kinds() {
        let plan = plan_recipe(&recipe_request(kind), &policy).map_err(|_| "recipe-positive")?;
        if plan.seats.is_empty() || plan.actions.is_empty() {
            return Err("recipe-positive");
        }
        if kind == RecipeKind::BestOfN {
            let observations = plan
                .seats
                .iter()
                .map(|seat| PassAtNObservation {
                    run_identity: seat.run_identity.as_str(),
                    passed: true,
                })
                .collect::<Vec<_>>();
            let reduction =
                reduce_pass_at_n(&plan, &observations, ONE).map_err(|_| "recipe-pass-at-n")?;
            if !reduction.passed {
                return Err("recipe-pass-at-n");
            }
        }
    }
    let mut unsafe_user = recipe_request(RecipeKind::UserSimulation);
    unsafe_user.simulated_user_trainable = true;
    if plan_recipe(&unsafe_user, &policy) != Err(RecipeError::UserAuthority) {
        return Err("recipe-role-widening");
    }
    let mut overflow = recipe_request(RecipeKind::BestOfN);
    overflow.candidate_count = MAX_RUNS + ONE;
    if plan_recipe(&overflow, &policy) != Err(RecipeError::Bound) {
        return Err("recipe-fanout-overflow");
    }
    Ok(())
}

fn run_episode_family() -> Result<(), &'static str> {
    let policy = core_policy(ResumeMismatchPolicy::Rerun);
    let plan = plan_recipe(&recipe_request(RecipeKind::SingleAgent), &policy)
        .map_err(|_| "episode-plan")?;
    let seat = &plan.seats[0];
    let observation = EpisodeRunObservation {
        run_identity: &seat.run_identity,
        seat_identity: &seat.seat_identity,
        ledger: LedgerTraceReference {
            session: "session-one",
            branch: "branch-one",
            leaf: "leaf-one",
            subject: "subject-one",
            role: "role-solver",
            run: &seat.run_identity,
            policy: "ledger-policy-one",
        },
        terminal: TerminalObservationClass::Completed,
        safe_summary_identity: Some("summary-one"),
    };
    let episode = reduce_episode(&plan, &[observation]).map_err(|_| "episode-positive")?;
    if episode.completed_count != ONE {
        return Err("episode-positive");
    }
    let bad = EpisodeRunObservation {
        ledger: LedgerTraceReference {
            run: "different-run",
            ..observation.ledger
        },
        ..observation
    };
    if reduce_episode(&plan, &[bad]) != Err(EpisodeError::LedgerBinding) {
        return Err("episode-ledger-binding");
    }
    Ok(())
}

fn run_port_family() -> Result<(), &'static str> {
    const CAPABILITIES: &[ServiceCapability] = &[ServiceCapability::TaskSource];
    let services = [ServiceDescriptor {
        kind: ServiceKind::TaskSource,
        implementation_identity: "task-source-conformance-v1",
        capabilities: CAPABILITIES,
        availability: ServiceAvailability::Available,
        bounds: ServiceResourceBounds {
            max_items: SERVICE_ITEMS,
            max_bytes: SERVICE_BYTES,
        },
        policy_identity: POLICY_IDENTITY,
    }];
    let catalog = ServiceCatalog {
        catalog_identity: "catalog-conformance-v1",
        services: &services,
    };
    let requirement = ServiceRequirement {
        kind: ServiceKind::TaskSource,
        implementation_identity: Some("task-source-conformance-v1"),
        required_items: ONE,
        required_bytes: SERVICE_BYTES,
        policy_identity: POLICY_IDENTITY,
    };
    let admission = admit_service(&catalog, &requirement).map_err(|_| "port-admission")?;
    let binding = adapter_binding();
    let request = build_service_request(
        ServiceKind::TaskSource,
        admission.identity(),
        admission.implementation_identity(),
        &binding,
        ONE,
        SERVICE_BYTES,
        &admission,
    )
    .map_err(|_| "port-request")?;
    TaskSourceRequest::new(request).map_err(|_| "port-request")?;

    let denied = ServiceRequirement {
        policy_identity: "different-policy",
        ..requirement
    };
    if admit_service(&catalog, &denied) != Err(animus_adapter_eval::AdmissionError::Mismatch) {
        return Err("port-policy-mismatch");
    }
    host_fixture::run_missing_service_negative()?;
    Ok(())
}

fn run_gateway_family() -> Result<(), &'static str> {
    let projection: ModelCallProjectionDraft<'_> = host_fixture::model_projection();
    project_model_call(&projection).map_err(|_| "gateway-projection")?;
    let terminal = host_fixture::terminal_evidence();
    let commit: TurnCommitEvidence<'_> = host_fixture::commit_evidence();
    admit_terminal_after_commit(&terminal, &commit).map_err(|_| "gateway-commit")?;
    let coalescing: CoalescingEvidence<'_> = host_fixture::coalescing_evidence();
    admit_identical_request_coalescing(&coalescing).map_err(|_| "gateway-coalescing")?;
    let early = TurnCommitEvidence {
        complete: false,
        ..commit
    };
    if admit_terminal_after_commit(&terminal, &early) != Err(GatewayError::CommitOrder) {
        return Err("gateway-early-terminal");
    }
    host_fixture::run_gateway_positive_and_negative()?;
    host_fixture::run_all_host_recipes()?;
    Ok(())
}

fn run_verifier_family() -> Result<(), &'static str> {
    host_fixture::run_verifier_positive_and_negative()
}

fn run_reward_family() -> Result<(), &'static str> {
    let policy = core_policy(ResumeMismatchPolicy::Rerun);
    let dimensions = [RewardDimension {
        name: "correctness",
        minimum: 0.0,
        maximum: ONE_SCORE,
        weight: ONE_SCORE,
        scorer_identity: "scorer-one",
        verifier_identity: "verifier-one",
        rubric_identity: "rubric-one",
        subject_identity: "subject-one",
        trajectory_identity: "trajectory-one",
    }];
    let contribution = RewardContribution {
        dimension: "correctness",
        value: Some(ONE_SCORE),
        scorer_identity: "scorer-one",
        verifier_identity: "verifier-one",
        rubric_identity: "rubric-one",
        subject_identity: "subject-one",
        trajectory_identity: "trajectory-one",
        evidence_identity: "evidence-one",
        judge_produced: true,
        claim: RewardClaim::Evidence,
    };
    let reduction = reduce_reward_dimensions(&dimensions, &[contribution], &policy)
        .map_err(|_| "reward-positive")?;
    if reduction.aggregate != Some(ONE_SCORE) {
        return Err("reward-positive");
    }
    let non_finite = RewardContribution {
        value: Some(f64::NAN),
        ..contribution
    };
    if reduce_reward_dimensions(&dimensions, &[non_finite], &policy) != Err(RewardError::NonFinite)
    {
        return Err("reward-non-finite");
    }
    let infinite = RewardContribution {
        value: Some(f64::INFINITY),
        ..contribution
    };
    if reduce_reward_dimensions(&dimensions, &[infinite], &policy) != Err(RewardError::NonFinite) {
        return Err("reward-infinite");
    }
    let mismatched = RewardContribution {
        scorer_identity: "scorer-mismatch",
        ..contribution
    };
    if reduce_reward_dimensions(&dimensions, &[mismatched], &policy) != Err(RewardError::Binding) {
        return Err("reward-identity-mismatch");
    }
    let outside = RewardContribution {
        value: Some(OUTSIDE_SCORE),
        ..contribution
    };
    if reduce_reward_dimensions(&dimensions, &[outside], &policy) != Err(RewardError::OutOfDomain) {
        return Err("reward-silent-clamp");
    }
    let duplicate = [contribution, contribution];
    if reduce_reward_dimensions(&dimensions, &duplicate, &policy) != Err(RewardError::Dimension) {
        return Err("reward-duplicate");
    }
    Ok(())
}

fn run_persistence_family() -> Result<(), &'static str> {
    let policy = core_policy(ResumeMismatchPolicy::RedoWholeTask);
    let material = resume_material();
    let key = build_resume_key(&material).map_err(|_| "resume-key")?;
    let exact = ResumeEvidence {
        resume_key: &key,
        evidence_identity: "resume-evidence-one",
        output_evidence_identity: "output-evidence-one",
        state: ResumeEvidenceState::CompleteTerminalSuccess,
    };
    let decision = evaluate_resume(&material, &[exact], &policy).map_err(|_| "resume-positive")?;
    if decision.action != ResumeAction::Keep {
        return Err("resume-positive");
    }
    let stale_key = hash_identity("animus.conformance.stale.v1", &["stale"]);
    let stale = ResumeEvidence {
        resume_key: &stale_key,
        ..exact
    };
    let stale_decision =
        evaluate_resume(&material, &[stale], &policy).map_err(|_| "resume-stale")?;
    if stale_decision.action != ResumeAction::RedoWholeTask {
        return Err("resume-stale");
    }
    let torn = ResumeEvidence {
        state: ResumeEvidenceState::Torn,
        ..exact
    };
    let torn_decision = evaluate_resume(&material, &[torn], &policy).map_err(|_| "resume-torn")?;
    if torn_decision.action != ResumeAction::Block {
        return Err("resume-torn");
    }
    host_fixture::run_cleanup_negative()?;
    Ok(())
}

fn run_compatibility_family() -> Result<(), &'static str> {
    let tasks = [Task {
        identity: "task-one",
        environment: "environment-one",
        verifier: "verifier-one",
        rubric: "rubric-one",
        network: NetworkPosture::None,
        separate_verifier: true,
    }];
    let routes = [Route {
        agent: "agent-one",
        model: "model-one",
    }];
    let plans = animus_runtime::harbor_eval::plan_trials(&tasks, &routes, ONE, &[])
        .map_err(|_| "harbor-plan")?;
    if plans.len() != ONE_INDEX {
        return Err("harbor-plan");
    }
    let steps = [AtifStep {
        id: ONE_INDEX,
        source: AtifSource::Agent,
        calls: &[],
        observations: &[],
        hidden_reasoning_omitted: true,
        terminal: true,
    }];
    let trajectory = AtifTrajectory {
        schema: animus_adapters::harbor::ATIF_SCHEMA,
        identity: "trajectory-one",
        steps: &steps,
        subagents: &[],
    };
    project_atif(&trajectory).map_err(|_| "atif-positive")?;
    let unsupported = AtifTrajectory {
        schema: "ATIF-unsupported",
        ..trajectory
    };
    if project_atif(&unsupported) != Err(AtifError::Version) {
        return Err("atif-version");
    }
    let rewards = [Reward {
        trial: &plans[0].identity,
        metric: "correctness",
        value: ONE_SCORE,
        verifier: "verifier-one",
        advisory: false,
    }];
    reduce_rewards(&rewards).map_err(|_| "harbor-reward")?;
    let duplicated = [rewards[0].clone(), rewards[0].clone()];
    if reduce_rewards(&duplicated).is_ok() {
        return Err("harbor-lossy-identity");
    }
    let shared_tasks = [Task {
        separate_verifier: false,
        ..tasks[0]
    }];
    let shared = animus_runtime::harbor_eval::plan_trials(&shared_tasks, &routes, ONE, &[]);
    if shared != Err(PlanError::VerifierIsolation) {
        return Err("harbor-verifier-isolation");
    }
    Ok(())
}

fn run_downstream_family() -> Result<(), &'static str> {
    let fingerprint = |label: &str| blake3_eval_feedback_fingerprint(label, label.as_bytes());
    let feedback = EvalFeedbackPlan {
        task_set: EvalFeedbackTaskSet {
            label: "taskset-one".into(),
            kind: EvalFeedbackTaskSetKind::Local,
        },
        scaffold_revision_id: "scaffold-one".into(),
        scaffold_hash: fingerprint("scaffold"),
        model_route: "model-route-one".into(),
        verifier: EvalFeedbackVerifierIdentity {
            label: "verifier-one".into(),
            hash: fingerprint("verifier"),
        },
        trust_boundary_hash: fingerprint("trust"),
        tool_surface_hash: fingerprint("tools"),
        sampling_profile: "sampling-one".into(),
        run_count: ONE,
        output_format: EvalFeedbackOutputFormat::Jsonl,
        resource_labels: vec!["resource-bounded".into()],
        unsupported_scope_requests: Vec::new(),
    };
    validate_eval_feedback_plan(&feedback).map_err(|_| "feedback-plan")?;
    let decision = evaluate_feedback_review(&EvalFeedbackCandidate {
        candidate_id: "candidate-one".into(),
        kind: EvalFeedbackCandidateKind::Promote,
        scaffold_revision_id: "scaffold-one".into(),
        receipt_hash: EvalFeedbackFingerprint::new(
            fingerprint("receipt").algorithm,
            fingerprint("receipt").value_hex,
        ),
        review_outcome: EvalFeedbackReviewOutcome::Approved,
    });
    if decision.activates_scaffold
        || decision.changes_runtime_selection_policy
        || !decision.can_request_scaffold_admission
    {
        return Err("feedback-auto-promotion");
    }
    let policy = DatasetExportPolicy {
        system_text: DatasetMetadataProjection::Exclude,
        tool_calls: DatasetMetadataProjection::Exclude,
        tool_results: DatasetMetadataProjection::Exclude,
        hidden_reasoning: DatasetMetadataProjection::Exclude,
        metadata: DatasetMetadataProjection::SafeLabels,
        min_assistant_chars: ONE as usize,
    };
    let source = DatasetSessionSource {
        session_id: "session-one".into(),
        branch_id: "branch-one".into(),
        target: DatasetTarget::ChatMessages,
        replay: vec![
            DatasetReplayEntry {
                entry_id: "entry-user".into(),
                role: DatasetReplayRole::User,
                content: "bounded user text".into(),
                source_span: "span-user".into(),
            },
            DatasetReplayEntry {
                entry_id: "entry-assistant".into(),
                role: DatasetReplayRole::Assistant,
                content: "bounded assistant text".into(),
                source_span: "span-assistant".into(),
            },
        ],
    };
    export_chat_messages_row(&source, &policy).map_err(|_| "dataset-positive")?;
    let unsafe_source = DatasetSessionSource {
        replay: vec![DatasetReplayEntry {
            entry_id: "entry-unsafe".into(),
            role: DatasetReplayRole::ToolResult,
            content: "tool_output: private raw payload".into(),
            source_span: "span-unsafe".into(),
        }],
        ..source
    };
    if export_chat_messages_row(&unsafe_source, &policy).is_ok() {
        return Err("dataset-private-export");
    }
    Ok(())
}

fn run_evidence_family() -> Result<(), &'static str> {
    let labels = ["terminal-observed"];
    let claims = [EvaluationReceiptClaim::BoundedObservation];
    let draft = EvaluationReceiptDraft {
        evaluation_identity: "evaluation-one",
        taskset_identity: "taskset-one",
        recipe_identity: "recipe-one",
        episode_identity: Some("episode-one"),
        reward_identity: Some("reward-one"),
        resume_identity: Some("resume-one"),
        policy_identity: POLICY_IDENTITY,
        status: EvaluationReceiptStatus::Observed,
        task_count: ONE,
        run_count: ONE,
        completed_count: ONE,
        redaction_count: ONE,
        diagnostic_labels: &labels,
        claims: &claims,
    };
    project_evaluation_receipt(&draft).map_err(|_| "receipt-positive")?;
    let leaks = ["raw_prompt"];
    if project_evaluation_receipt(&EvaluationReceiptDraft {
        diagnostic_labels: &leaks,
        ..draft
    }) != Err(ReceiptError::Leak)
    {
        return Err("receipt-leak");
    }
    let overclaims = [EvaluationReceiptClaim::VerifierSoundness];
    if project_evaluation_receipt(&EvaluationReceiptDraft {
        claims: &overclaims,
        ..draft
    }) != Err(ReceiptError::Overclaim)
    {
        return Err("receipt-overclaim");
    }
    host_fixture::run_host_receipt_positive_and_negative()?;
    Ok(())
}

fn core_policy(resume_mismatch: ResumeMismatchPolicy) -> EvaluationPolicy {
    let recipes = recipe_kinds();
    validate_evaluation_policy(
        &EvaluationPolicyDraft {
            schema: EVALUATION_POLICY_SCHEMA,
            source_identity: SOURCE_IDENTITY,
            profile_id: "profile-conformance-v1",
            max_tasks: MAX_TASKS,
            max_runs: MAX_RUNS,
            max_reward_dimensions: MAX_REWARDS,
            max_artifacts: MAX_ARTIFACTS,
            network_default: EvaluationNetworkPosture::None,
            secret_values_allowed: false,
            transcript_bodies_allowed: false,
            missing_reward: MissingRewardPolicy::Error,
            resume_mismatch,
            supported_recipes: &recipes,
            evidence_redaction: "metadata_only",
        },
        SOURCE_IDENTITY,
    )
    .expect("static conformance policy is valid")
}

const fn recipe_kinds() -> [RecipeKind; 5] {
    [
        RecipeKind::SingleAgent,
        RecipeKind::BestOfN,
        RecipeKind::UserSimulation,
        RecipeKind::IsolatedAgenticJudge,
        RecipeKind::SharedAdvisoryJudge,
    ]
}

const fn recipe_bindings() -> RecipeBindings<'static> {
    RecipeBindings {
        task_identity: "task-one",
        role_identity: "role-solver",
        model_identity: "model-one",
        harness_identity: "harness-one",
        tool_identity: "tools-one",
        environment_identity: "environment-solver",
        verifier_identity: "verifier-one",
        verifier_environment_identity: "environment-verifier",
        rubric_identity: "rubric-one",
        resource_identity: "resources-one",
        cancellation_identity: "cancel-one",
        deadline_identity: "deadline-one",
        redaction_identity: "redaction-one",
        artifact_manifest_identity: "artifacts-one",
    }
}

const fn recipe_request(kind: RecipeKind) -> RecipeRequest<'static> {
    RecipeRequest {
        kind,
        bindings: recipe_bindings(),
        candidate_count: PAIR,
        artifact_count: ONE,
        simulated_user_trainable: false,
        simulated_user_assistant_tools: false,
    }
}

const fn task(identity: &'static str) -> TaskDescriptor<'static> {
    TaskDescriptor {
        task_identity: identity,
        data_identity: "data-bounded",
        behavior_identity: "behavior-v1",
        scorer_identity: "scorer-v1",
        policy_identity: "task-policy-v1",
        version: "v1",
    }
}

const fn adapter_binding() -> EvaluationBinding<'static> {
    EvaluationBinding {
        evaluation_identity: "evaluation-one",
        plan_identity: "plan-one",
        task_identity: "task-one",
        episode_identity: "episode-one",
        run_identity: "run-one",
        role_identity: "role-solver",
        model_identity: "model-one",
        harness_identity: "harness-one",
        environment_identity: "environment-solver",
        solver_environment_identity: "environment-solver",
        tool_identity: "tools-one",
        verifier_identity: "verifier-one",
        rubric_identity: "rubric-one",
        artifact_identity: "artifacts-one",
        policy_identity: POLICY_IDENTITY,
        cancellation_identity: "cancel-one",
        deadline_identity: "deadline-one",
        attempt: ONE,
        attempt_lineage_identity: "attempt-lineage-one",
        correlation_identity: "correlation-one",
    }
}

const fn resume_material() -> ResumeMaterial<'static> {
    ResumeMaterial {
        task_data_identity: "task-data-one",
        taskset_identity: "taskset-one",
        recipe_identity: "recipe-one",
        model_identity: "model-one",
        harness_identity: "harness-one",
        environment_identity: "environment-one",
        tool_identity: "tools-one",
        verifier_identity: "verifier-one",
        rubric_identity: "rubric-one",
        reward_identity: "reward-one",
        sampling_identity: "sampling-one",
        redaction_identity: "redaction-one",
        output_identity: "output-one",
        policy_identity: POLICY_IDENTITY,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parse_conformance_profile;

    const PROFILE: &[u8] =
        include_bytes!("../../../config/generated/evaluation-composition-conformance.json");

    #[test]
    fn full_matrix_uses_public_paths_and_emits_bounded_evidence() {
        let profile = parse_conformance_profile(PROFILE).expect("profile");
        let summary = run_evaluation_conformance(&profile).expect("complete matrix");
        assert_eq!(summary.family_count, EXPECTED_FAMILIES);
        assert_eq!(summary.positive_count, EXPECTED_POSITIVE);
        assert_eq!(summary.negative_count, EXPECTED_NEGATIVE);
        let rendered = render_conformance_summary(&summary);
        assert!(rendered.contains("status: PASS"));
        assert!(!rendered.contains("bounded user text"));
    }

    #[test]
    fn private_evidence_and_overclaims_fail_closed() {
        let profile = parse_conformance_profile(PROFILE).expect("profile");
        let summary = run_evaluation_conformance(&profile).expect("complete matrix");
        assert!(
            validate_conformance_summary(
                &summary,
                ConformanceLeakFlags {
                    hidden_test: true,
                    ..ConformanceLeakFlags::default()
                },
                &[ConformanceClaim::BoundedFixtureObservation],
            )
            .is_err()
        );
        assert!(
            validate_conformance_summary(
                &summary,
                ConformanceLeakFlags::default(),
                &[ConformanceClaim::ProductionReadiness],
            )
            .is_err()
        );
    }
}
