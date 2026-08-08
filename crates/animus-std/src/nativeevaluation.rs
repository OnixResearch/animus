//! Strict host-side parsing for generated native evaluation policy.

use animus_eval_core::{
    EVALUATION_POLICY_SCHEMA, EvaluationNetworkPosture, EvaluationPolicy, EvaluationPolicyDraft,
    MissingRewardPolicy, PolicyError, RecipeKind, ResumeMismatchPolicy, validate_evaluation_policy,
};
use serde::Deserialize;

const VERIFIERS_REPOSITORY: &str = "PrimeIntellect-ai/verifiers";
const VERIFIERS_REVISION: &str = "1a997814e85a2e4f409add960aa852737388dbd8";
const VERIFIERS_LICENSE: &str = "MIT";
const VERIFIERS_SOURCE_IDENTITY: &str =
    "b3:069f20983a0d523d84028b76f85b3dd63a712f952e99cdef628824ef1ffe6bfe";

/// Generated native evaluation profile parsing failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NativeEvaluationProfileError {
    /// JSON is malformed, contains unknown fields, or has a type mismatch.
    Json,
    /// The checked source cohort or deny-by-default projection drifted.
    Projection,
    /// The authority-free core rejected the policy.
    Policy(PolicyError),
}

/// Parse generated profile JSON and revalidate it through the no-std core.
///
/// This function accepts caller-loaded bytes. It does not discover or open a
/// configuration path and does not evaluate Nickel at runtime.
///
/// # Errors
/// Returns a bounded JSON, projection, or core-policy class.
pub fn parse_native_evaluation_policy_json(
    input: &str,
) -> Result<EvaluationPolicy, NativeEvaluationProfileError> {
    let generated: GeneratedPolicy =
        serde_json::from_str(input).map_err(|_| NativeEvaluationProfileError::Json)?;
    validate_projection(&generated)?;
    let recipes = generated
        .recipes
        .iter()
        .map(|recipe| recipe.to_core())
        .collect::<Vec<_>>();
    let draft = EvaluationPolicyDraft {
        schema: &generated.schema,
        source_identity: &generated.source_cohort.identity,
        profile_id: &generated.profile_id,
        max_tasks: generated.bounds.tasks,
        max_runs: generated.bounds.runs,
        max_reward_dimensions: generated.bounds.reward_dimensions,
        max_artifacts: generated.bounds.artifacts,
        network_default: EvaluationNetworkPosture::None,
        secret_values_allowed: generated.defaults.secret_values_allowed,
        transcript_bodies_allowed: generated.defaults.transcript_bodies_allowed,
        missing_reward: MissingRewardPolicy::Error,
        resume_mismatch: ResumeMismatchPolicy::Rerun,
        supported_recipes: &recipes,
        evidence_redaction: &generated.evidence.redaction,
    };
    validate_evaluation_policy(&draft, VERIFIERS_SOURCE_IDENTITY)
        .map_err(NativeEvaluationProfileError::Policy)
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedPolicy {
    schema: String,
    profile_id: String,
    source_cohort: GeneratedSourceCohort,
    bounds: GeneratedBounds,
    defaults: GeneratedDefaults,
    taskset: GeneratedTaskset,
    recipes: Vec<GeneratedRecipe>,
    rewards: GeneratedRewards,
    resume: GeneratedResume,
    evidence: GeneratedEvidence,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedSourceCohort {
    repository: String,
    revision: String,
    license: String,
    identity: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedBounds {
    tasks: u32,
    runs: u32,
    reward_dimensions: u32,
    artifacts: u32,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedDefaults {
    network: String,
    secret_values_allowed: bool,
    transcript_bodies_allowed: bool,
    runtime_nickel_allowed: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "this strict DTO mirrors the checked deny-by-default Nickel projection"
)]
struct GeneratedTaskset {
    stable_order_required: bool,
    lazy_generator_identity_required: bool,
    lazy_generator_version_required: bool,
    explicit_seed_required: bool,
    positive_selection_bound_required: bool,
    global_shuffle_allowed: bool,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum GeneratedRecipe {
    SingleAgent,
    BestOfN,
    UserSimulation,
    IsolatedAgenticJudge,
    SharedAdvisoryJudge,
}

impl GeneratedRecipe {
    const fn to_core(self) -> RecipeKind {
        match self {
            Self::SingleAgent => RecipeKind::SingleAgent,
            Self::BestOfN => RecipeKind::BestOfN,
            Self::UserSimulation => RecipeKind::UserSimulation,
            Self::IsolatedAgenticJudge => RecipeKind::IsolatedAgenticJudge,
            Self::SharedAdvisoryJudge => RecipeKind::SharedAdvisoryJudge,
        }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedRewards {
    missing: String,
    non_finite: String,
    out_of_domain: String,
    judge: String,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedResume {
    mismatch: String,
    require_complete: bool,
    require_terminal: bool,
    require_safe: bool,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedEvidence {
    redaction: String,
    raw_content_allowed: bool,
    completion_claim_allowed: bool,
    non_claims: Vec<String>,
}

fn validate_projection(policy: &GeneratedPolicy) -> Result<(), NativeEvaluationProfileError> {
    let source = &policy.source_cohort;
    if policy.schema != EVALUATION_POLICY_SCHEMA
        || source.repository != VERIFIERS_REPOSITORY
        || source.revision != VERIFIERS_REVISION
        || source.license != VERIFIERS_LICENSE
        || source.identity != VERIFIERS_SOURCE_IDENTITY
    {
        return Err(NativeEvaluationProfileError::Projection);
    }
    let defaults = &policy.defaults;
    let taskset = &policy.taskset;
    let rewards = &policy.rewards;
    let resume = &policy.resume;
    let evidence = &policy.evidence;
    if defaults.network != "none"
        || defaults.secret_values_allowed
        || defaults.transcript_bodies_allowed
        || defaults.runtime_nickel_allowed
        || !taskset.stable_order_required
        || !taskset.lazy_generator_identity_required
        || !taskset.lazy_generator_version_required
        || !taskset.explicit_seed_required
        || !taskset.positive_selection_bound_required
        || taskset.global_shuffle_allowed
        || rewards.missing != "error"
        || rewards.non_finite != "error"
        || rewards.out_of_domain != "error"
        || rewards.judge != "advisory"
        || resume.mismatch != "rerun"
        || !resume.require_complete
        || !resume.require_terminal
        || !resume.require_safe
        || evidence.redaction != "metadata_only"
        || evidence.raw_content_allowed
        || evidence.completion_claim_allowed
        || evidence.non_claims.is_empty()
    {
        return Err(NativeEvaluationProfileError::Projection);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_RECIPE_COUNT: usize = 5;
    const GENERATED: &str = include_str!("../../../config/generated/native-evaluation-policy.json");

    #[test]
    fn generated_policy_is_strictly_revalidated() {
        let policy = parse_native_evaluation_policy_json(GENERATED).expect("generated policy");
        assert_eq!(policy.source_identity, VERIFIERS_SOURCE_IDENTITY);
        assert_eq!(policy.supported_recipes.len(), EXPECTED_RECIPE_COUNT);
    }

    #[test]
    fn unknown_fields_and_source_drift_fail_closed() {
        let unknown = GENERATED.replacen("\"schema\":", "\"unexpected\": true, \"schema\":", 1);
        assert_eq!(
            parse_native_evaluation_policy_json(&unknown),
            Err(NativeEvaluationProfileError::Json)
        );
        let drifted = GENERATED.replace(VERIFIERS_REVISION, "deadbeef");
        assert_eq!(
            parse_native_evaluation_policy_json(&drifted),
            Err(NativeEvaluationProfileError::Projection)
        );
    }
}
