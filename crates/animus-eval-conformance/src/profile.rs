use std::collections::BTreeSet;

use serde::Deserialize;

use crate::{
    CONFORMANCE_PROFILE_ID, CONFORMANCE_SCHEMA, MAX_CONFORMANCE_FIXTURES, MAX_CONFORMANCE_SERVICES,
    VERIFIERS_CONFORMANCE_COHORT_IDENTITY, VERIFIERS_CONFORMANCE_REVISION, hash_parts,
};

const EXPECTED_FAMILY_COUNT: usize = 11;
const EXPECTED_ENTRYPOINT_COUNT: usize = 23;
const EXPECTED_SERVICE_COUNT: usize = 13;
const EXPECTED_SERVICE_COUNT_U32: u32 = 13;
const EXPECTED_FIXTURE_COUNT: u32 = 22;
const EXPECTED_MAX_IDENTITY_BYTES: u32 = 512;
const EXPECTED_MAX_EVIDENCE_LABELS: u32 = 64;

const REQUIRED_FAMILIES: [&str; EXPECTED_FAMILY_COUNT] = [
    "taskset",
    "recipes",
    "episode",
    "ports",
    "gateway",
    "verifier",
    "rewards",
    "persistence",
    "compatibility",
    "downstream",
    "evidence",
];

const REQUIRED_ENTRYPOINTS: [&str; EXPECTED_ENTRYPOINT_COUNT] = [
    "plan_taskset",
    "plan_environment",
    "plan_recipe",
    "reduce_pass_at_n",
    "reduce_episode",
    "reduce_reward_dimensions",
    "build_resume_key",
    "evaluate_resume",
    "project_evaluation_receipt",
    "admit_service",
    "build_service_request",
    "project_model_call",
    "admit_terminal_after_commit",
    "admit_identical_request_coalescing",
    "execute_evaluation_host",
    "preflight_host",
    "project_host_receipt",
    "plan_trials",
    "project_atif",
    "reduce_rewards",
    "validate_eval_feedback_plan",
    "evaluate_feedback_review",
    "export_chat_messages_row",
];

const REQUIRED_SERVICES: [&str; EXPECTED_SERVICE_COUNT] = [
    "task_source",
    "agent_execution",
    "verifier_execution",
    "artifact_collect",
    "artifact_restore",
    "model_call_capture",
    "result_append",
    "result_replace",
    "result_reopen",
    "tool",
    "timer",
    "cancellation",
    "cleanup",
];

/// One reviewed upstream source cohort.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SourceCohort {
    /// Source repository label.
    pub repository: String,
    /// Immutable revision.
    pub revision: String,
    /// Reviewed license.
    pub license: String,
    /// Domain-separated BLAKE3 cohort identity.
    pub identity: String,
    /// Ordered reviewed source paths.
    pub paths: Vec<String>,
}

/// One mechanism family and its paired fixtures.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct FamilyProfile {
    /// Stable family label.
    pub id: String,
    /// Positive fixture label.
    pub positive_fixture: String,
    /// Negative fixture label.
    pub negative_fixture: String,
    /// Expected negative diagnostic label.
    pub expected_diagnostic: String,
}

/// Named finite conformance bounds.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConformanceBounds {
    /// Total registered fixtures.
    pub fixtures: u32,
    /// Maximum identity bytes accepted by fixture metadata.
    pub max_identity_bytes: u32,
    /// Maximum evidence labels in one summary.
    pub max_evidence_labels: u32,
    /// Maximum deterministic fake services.
    pub max_services: u32,
}

/// Explicit dependency-isolation posture.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DependencyProfile {
    /// Crate and feature cohort used by the suite.
    pub crate_features: Vec<String>,
    /// Live dependency classes, which must remain empty.
    pub live_dependencies: Vec<String>,
}

/// Safe evidence and claim posture.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct EvidenceProfile {
    /// Whether evidence contains metadata only.
    pub metadata_only: bool,
    /// Whether raw fixture content can appear.
    pub raw_content_allowed: bool,
    /// Whether physical paths can appear.
    pub physical_paths_allowed: bool,
    /// Whether live behavior can be claimed.
    pub live_claims_allowed: bool,
    /// Explicit bounded non-claims.
    pub non_claims: Vec<String>,
}

/// Strict generated conformance profile.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ConformanceProfile {
    /// Profile schema.
    pub schema: String,
    /// Stable profile label.
    pub profile_id: String,
    /// Reviewed upstream scenario cohort.
    pub source_cohort: SourceCohort,
    /// Registered mechanism families.
    pub families: Vec<FamilyProfile>,
    /// Public production entrypoints that fixtures must call.
    pub public_entrypoints: Vec<String>,
    /// Explicit deterministic fake service classes.
    pub services: Vec<String>,
    /// Named finite bounds.
    pub bounds: ConformanceBounds,
    /// Dependency-isolation posture.
    pub dependencies: DependencyProfile,
    /// Safe evidence posture.
    pub evidence: EvidenceProfile,
}

/// Validated profile plus deterministic identities.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidatedConformanceProfile {
    /// Strict parsed profile.
    pub profile: ConformanceProfile,
    /// Domain-separated profile identity.
    pub identity: String,
    /// Ordered domain-separated family identities.
    pub family_identities: Vec<String>,
    /// Ordered positive and negative fixture identities.
    pub fixture_identities: Vec<String>,
}

/// Strict profile validation failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileError {
    /// Generated JSON did not match the strict schema.
    Decode,
    /// Schema or profile identity was stale.
    Schema,
    /// Reviewed source cohort was stale or malformed.
    SourceCohort,
    /// A required family was absent, duplicated, or malformed.
    Family,
    /// A private, unknown, missing, or duplicated entrypoint appeared.
    Entrypoint,
    /// The fake service catalog was incomplete or duplicated.
    Service,
    /// A named bound was zero, excessive, or did not match the matrix.
    Bound,
    /// Live dependency or incomplete crate-feature posture appeared.
    Dependency,
    /// Evidence requested private data or unsupported claims.
    Evidence,
}

/// Parse and revalidate one generated conformance profile.
///
/// # Errors
/// Returns a strict decode or fail-closed profile error.
pub fn parse_conformance_profile(
    bytes: &[u8],
) -> Result<ValidatedConformanceProfile, ProfileError> {
    let profile =
        serde_json::from_slice::<ConformanceProfile>(bytes).map_err(|_| ProfileError::Decode)?;
    validate_conformance_profile(profile)
}

/// Revalidate one parsed conformance profile and compute its identities.
///
/// # Errors
/// Returns the first fail-closed profile class that does not match the required matrix.
pub fn validate_conformance_profile(
    profile: ConformanceProfile,
) -> Result<ValidatedConformanceProfile, ProfileError> {
    if profile.schema != CONFORMANCE_SCHEMA || profile.profile_id != CONFORMANCE_PROFILE_ID {
        return Err(ProfileError::Schema);
    }
    validate_source_cohort(&profile.source_cohort)?;
    validate_exact_strings(&profile.public_entrypoints, &REQUIRED_ENTRYPOINTS)
        .map_err(|()| ProfileError::Entrypoint)?;
    validate_exact_strings(&profile.services, &REQUIRED_SERVICES)
        .map_err(|()| ProfileError::Service)?;
    validate_families(&profile.families)?;
    validate_bounds(profile.bounds)?;
    validate_dependencies(&profile.dependencies)?;
    validate_evidence(&profile.evidence)?;

    let family_identities = profile
        .families
        .iter()
        .map(|family| {
            hash_parts(
                "animus.native-evaluation-conformance.family.v1",
                &[
                    family.id.as_str(),
                    family.positive_fixture.as_str(),
                    family.negative_fixture.as_str(),
                    family.expected_diagnostic.as_str(),
                ],
            )
        })
        .collect::<Vec<_>>();
    let fixture_identities = profile
        .families
        .iter()
        .flat_map(|family| {
            [
                hash_parts(
                    "animus.native-evaluation-conformance.fixture.v1",
                    &[
                        family.id.as_str(),
                        "positive",
                        family.positive_fixture.as_str(),
                    ],
                ),
                hash_parts(
                    "animus.native-evaluation-conformance.fixture.v1",
                    &[
                        family.id.as_str(),
                        "negative",
                        family.negative_fixture.as_str(),
                        family.expected_diagnostic.as_str(),
                    ],
                ),
            ]
        })
        .collect::<Vec<_>>();
    let identity = hash_parts(
        "animus.native-evaluation-conformance.profile.v1",
        &[
            profile.schema.as_str(),
            profile.profile_id.as_str(),
            profile.source_cohort.identity.as_str(),
            family_identities.join(":").as_str(),
            fixture_identities.join(":").as_str(),
        ],
    );
    Ok(ValidatedConformanceProfile {
        profile,
        identity,
        family_identities,
        fixture_identities,
    })
}

fn validate_source_cohort(cohort: &SourceCohort) -> Result<(), ProfileError> {
    if cohort.repository != "PrimeIntellect-ai/verifiers"
        || cohort.revision != VERIFIERS_CONFORMANCE_REVISION
        || cohort.license != "MIT"
        || cohort.identity != VERIFIERS_CONFORMANCE_COHORT_IDENTITY
        || cohort.paths.is_empty()
        || !cohort.identity.starts_with("b3:")
        || cohort.identity.len() != 67
        || cohort.paths.iter().any(String::is_empty)
    {
        return Err(ProfileError::SourceCohort);
    }
    let unique = cohort.paths.iter().collect::<BTreeSet<_>>();
    if unique.len() != cohort.paths.len() {
        return Err(ProfileError::SourceCohort);
    }
    Ok(())
}

fn validate_families(families: &[FamilyProfile]) -> Result<(), ProfileError> {
    if families.len() != EXPECTED_FAMILY_COUNT {
        return Err(ProfileError::Family);
    }
    let actual = families
        .iter()
        .map(|family| family.id.as_str())
        .collect::<Vec<_>>();
    if actual != REQUIRED_FAMILIES {
        return Err(ProfileError::Family);
    }
    let mut fixtures = BTreeSet::new();
    let mut diagnostics = BTreeSet::new();
    for family in families {
        if family.positive_fixture.is_empty()
            || family.negative_fixture.is_empty()
            || family.expected_diagnostic.is_empty()
            || !fixtures.insert(family.positive_fixture.as_str())
            || !fixtures.insert(family.negative_fixture.as_str())
            || !diagnostics.insert(family.expected_diagnostic.as_str())
        {
            return Err(ProfileError::Family);
        }
    }
    Ok(())
}

fn validate_exact_strings<const COUNT: usize>(
    actual: &[String],
    expected: &[&str; COUNT],
) -> Result<(), ()> {
    if actual.len() != COUNT {
        return Err(());
    }
    let actual_set = actual.iter().map(String::as_str).collect::<BTreeSet<_>>();
    let expected_set = expected.iter().copied().collect::<BTreeSet<_>>();
    if actual_set != expected_set || actual.iter().any(|entry| entry.contains("::tests::")) {
        return Err(());
    }
    Ok(())
}

fn validate_bounds(bounds: ConformanceBounds) -> Result<(), ProfileError> {
    if bounds.fixtures != EXPECTED_FIXTURE_COUNT
        || bounds.fixtures > MAX_CONFORMANCE_FIXTURES
        || bounds.max_identity_bytes != EXPECTED_MAX_IDENTITY_BYTES
        || bounds.max_evidence_labels != EXPECTED_MAX_EVIDENCE_LABELS
        || bounds.max_services != EXPECTED_SERVICE_COUNT_U32
        || bounds.max_services > MAX_CONFORMANCE_SERVICES
    {
        return Err(ProfileError::Bound);
    }
    Ok(())
}

fn validate_dependencies(dependencies: &DependencyProfile) -> Result<(), ProfileError> {
    const REQUIRED_CRATES: [&str; 6] = [
        "animus-eval-core",
        "animus-adapter-eval",
        "animus-std-eval",
        "animus-adapters/evaluation",
        "animus-runtime/harbor-eval",
        "animus-runtime/dataset-export",
    ];
    validate_exact_strings(&dependencies.crate_features, &REQUIRED_CRATES)
        .map_err(|()| ProfileError::Dependency)?;
    if !dependencies.live_dependencies.is_empty() {
        return Err(ProfileError::Dependency);
    }
    Ok(())
}

fn validate_evidence(evidence: &EvidenceProfile) -> Result<(), ProfileError> {
    if !evidence.metadata_only
        || evidence.raw_content_allowed
        || evidence.physical_paths_allowed
        || evidence.live_claims_allowed
        || evidence.non_claims.is_empty()
        || evidence.non_claims.len() > EXPECTED_MAX_EVIDENCE_LABELS as usize
        || evidence.non_claims.iter().any(String::is_empty)
    {
        return Err(ProfileError::Evidence);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROFILE: &[u8] =
        include_bytes!("../../../config/generated/evaluation-composition-conformance.json");

    #[test]
    fn generated_profile_is_strict_and_complete() {
        let profile = parse_conformance_profile(PROFILE).expect("generated profile must pass");
        assert_eq!(profile.profile.families.len(), EXPECTED_FAMILY_COUNT);
        assert_eq!(
            profile.profile.public_entrypoints.len(),
            EXPECTED_ENTRYPOINT_COUNT
        );
        assert_eq!(profile.profile.services.len(), EXPECTED_SERVICE_COUNT);
        assert_eq!(
            profile.fixture_identities.len(),
            EXPECTED_FIXTURE_COUNT as usize
        );
    }

    #[test]
    fn stale_unknown_and_live_profiles_fail_closed() {
        let mut value = serde_json::from_slice::<serde_json::Value>(PROFILE).expect("valid JSON");
        value["schema"] = serde_json::Value::String("stale".into());
        assert_eq!(
            parse_conformance_profile(&serde_json::to_vec(&value).expect("encode")),
            Err(ProfileError::Schema)
        );

        let mut value = serde_json::from_slice::<serde_json::Value>(PROFILE).expect("valid JSON");
        value["unknown"] = serde_json::Value::Bool(true);
        assert_eq!(
            parse_conformance_profile(&serde_json::to_vec(&value).expect("encode")),
            Err(ProfileError::Decode)
        );

        let mut value = serde_json::from_slice::<serde_json::Value>(PROFILE).expect("valid JSON");
        value["dependencies"]["live_dependencies"] = serde_json::json!(["provider-network"]);
        assert_eq!(
            parse_conformance_profile(&serde_json::to_vec(&value).expect("encode")),
            Err(ProfileError::Dependency)
        );
    }
}
