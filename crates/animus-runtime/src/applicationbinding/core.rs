//! Pure validation, projection, commit, dispatch, and explanation logic.

use super::data::{
    FIRST_SEQUENCE, HASH_HEX_CHARS, HASH_SEPARATOR, KAMACITE_BINDING_RECEIPT_SCHEMA,
    KAMACITE_BINDING_SCHEMA, KAMACITE_CORE_BLAKE3, KAMACITE_PRODUCER_REVISION,
    KAMACITE_RECEIPT_SCHEMA_BLAKE3, KAMACITE_SCHEMA_BLAKE3, KAMACITE_VALIDATION_RECEIPT_BLAKE3,
    MAX_EFFECT_OPERATIONS, NEXT_FOCUSED_CHECK, PILOT_BINDING_HASH_DOMAIN, PILOT_COMMIT_HASH_DOMAIN,
    PILOT_EXPLANATION_SCHEMA, REDACTION_MARKER, REQUIRED_NON_CLAIMS,
};
use super::data::{
    KamaciteContractCohort, PilotApplicationHostBinding, PilotBindingCommit, PilotBindingDraft,
    PilotBindingParts, PilotCommitDraft, PilotDispatchAdmission, PilotDispatchDraft,
    PilotEnforcementClass, PilotEnforcementObservation, PilotError, PilotIdentity,
    PilotIdentityDomain, PilotIssue, PilotIssueCode,
};
use crate::runtime_authority_seal::{RuntimeAuthorityUnitState, start_runtime_authority_effect};
use core::fmt::Write as _;

const FIELD_COHORT: &str = "kamacite_contract";
const FIELD_APPLICATION: &str = "application_identity";
const FIELD_ENTRYPOINT: &str = "entrypoint_identity";
const FIELD_INPUT_SCHEMA: &str = "input_schema_identity";
const FIELD_OUTPUT_SCHEMA: &str = "output_schema_identity";
const FIELD_EFFECT_SUMMARY: &str = "effect_summary_identity";
const FIELD_EFFECT_OPERATION: &str = "effect_operation_identities";
const FIELD_HOST_PROFILE: &str = "host_profile_identity";
const FIELD_ADAPTER_COHORT: &str = "adapter_cohort_identity";
const FIELD_RESOURCE_PROFILE: &str = "resource_profile_identity";
const FIELD_POLICY: &str = "policy_identity";
const FIELD_COMPATIBILITY: &str = "compatibility_identity";
const FIELD_NON_CLAIMS: &str = "non_claims";
const FIELD_SEMANTIC_CLAIMS: &str = "semantic_claims";
const FIELD_SENSITIVE: &str = "sensitive";
const FIELD_SEQUENCE: &str = "sequence";
const FIELD_BINDING: &str = "binding_identity";
const FIELD_SESSION_BINDING: &str = "session_binding_identity";
const FIELD_SEAL: &str = "runtime_authority_seal";
const FIELD_CURRENT_ENFORCEMENT: &str = "current_enforcement";
const JSON_NULL: &str = "null";

/// Admit one domain-tagged BLAKE3 identity.
///
/// # Errors
///
/// Returns an invalid-identity issue when `hex` is not one lowercase BLAKE3 value.
pub fn admit_pilot_identity(
    domain: PilotIdentityDomain,
    hex: &str,
) -> Result<PilotIdentity, PilotError> {
    if !is_valid_hash(hex) {
        return Err(error(PilotIssueCode::InvalidIdentity, "identity"));
    }
    Ok(PilotIdentity::new(domain, String::from(hex)))
}

/// Validate and project one authority-free pilot binding from already-loaded identities.
///
/// # Errors
///
/// Returns deterministic issues for stale cohorts, wrong identity roles, ambiguous hosts, open
/// effects, cohort drift, unsafe fields, or weakened non-claims.
pub fn project_application_host_binding(
    draft: &PilotBindingDraft<'_>,
) -> Result<PilotApplicationHostBinding, PilotError> {
    let issues = validate_binding_draft(draft);
    if !issues.is_empty() {
        return Err(PilotError::from_issues(issues));
    }

    let [host_profile_identity] = draft.host_profile_identities else {
        return Err(error(
            PilotIssueCode::AmbiguousHostProfile,
            FIELD_HOST_PROFILE,
        ));
    };
    let mut effect_operation_identities = draft.effect_operation_identities.to_vec();
    effect_operation_identities.sort();
    let mut non_claims = draft
        .non_claims
        .iter()
        .map(|value| String::from(*value))
        .collect::<Vec<_>>();
    non_claims.sort();
    let projection_hex = binding_projection_hash(draft, &effect_operation_identities, &non_claims);
    let binding_identity = PilotIdentity::new(PilotIdentityDomain::Projection, projection_hex);

    Ok(PilotApplicationHostBinding::from_parts(PilotBindingParts {
        application_identity: draft.application_identity.clone(),
        entrypoint_identity: draft.entrypoint_identity.clone(),
        input_schema_identity: draft.input_schema_identity.clone(),
        output_schema_identity: draft.output_schema_identity.clone(),
        effect_summary_identity: draft.effect_summary_identity.clone(),
        effect_operation_identities,
        host_profile_identity: host_profile_identity.clone(),
        adapter_cohort_identity: draft.adapter_cohort_identity.clone(),
        resource_profile_identity: draft.resource_profile_identity.clone(),
        policy_identity: draft.policy_identity.clone(),
        compatibility_identity: draft.compatibility_identity.clone(),
        binding_identity,
        non_claims,
    }))
}

/// Bind the pilot projection and the existing durable session binding before effects.
///
/// # Errors
///
/// Returns a sequence or session-profile mismatch before creating commit evidence.
pub fn commit_application_host_binding(
    draft: PilotCommitDraft<'_>,
) -> Result<PilotBindingCommit, PilotError> {
    let mut issues = Vec::new();
    if draft.application_binding_sequence < FIRST_SEQUENCE
        || draft.session_binding_sequence < FIRST_SEQUENCE
    {
        issues.push(issue(PilotIssueCode::InvalidSequence, FIELD_SEQUENCE));
    }
    if draft.session_binding.profile().profile_identity()
        != draft.binding.host_profile_identity().hex()
    {
        issues.push(issue(
            PilotIssueCode::SessionBindingMismatch,
            FIELD_SESSION_BINDING,
        ));
    }
    if !issues.is_empty() {
        return Err(PilotError::from_issues(issues));
    }

    let highest_binding_sequence = draft
        .application_binding_sequence
        .max(draft.session_binding_sequence);
    let mut hasher = blake3::Hasher::new();
    hash_part(&mut hasher, PILOT_COMMIT_HASH_DOMAIN);
    hash_part(&mut hasher, draft.binding.binding_identity().hex());
    hash_part(&mut hasher, draft.session_binding.binding_identity());
    hash_part(&mut hasher, &draft.application_binding_sequence.to_string());
    hash_part(&mut hasher, &draft.session_binding_sequence.to_string());
    let commit_identity = PilotIdentity::new(
        PilotIdentityDomain::Receipt,
        hasher.finalize().to_hex().to_string(),
    );
    Ok(PilotBindingCommit::new(
        draft.binding.binding_identity().clone(),
        String::from(draft.session_binding.binding_identity()),
        highest_binding_sequence,
        commit_identity,
    ))
}

/// Admit one dependent pilot operation after exact binding, seal, and current checks.
///
/// # Errors
///
/// Returns deterministic issues when binding order, operation declaration, the accepted seal, or
/// any current enforcement observation fails.
pub fn prepare_pilot_dispatch(
    draft: PilotDispatchDraft<'_>,
) -> Result<PilotDispatchAdmission, PilotError> {
    let issues = validate_dispatch_binding(&draft);
    if !issues.is_empty() {
        return Err(PilotError::from_issues(issues));
    }
    let running_unit =
        start_runtime_authority_effect(draft.sealed_unit, draft.recheck).map_err(|_| {
            error(
                PilotIssueCode::CurrentEnforcementDenied,
                FIELD_CURRENT_ENFORCEMENT,
            )
        })?;
    Ok(PilotDispatchAdmission {
        binding_identity: draft.binding.binding_identity().clone(),
        commit_identity: draft.commit.commit_identity().clone(),
        operation_identity: draft.operation.clone(),
        running_unit,
    })
}

/// Render the Kamacite-compatible JSON input shape without metadata or authority claims.
#[must_use]
pub fn render_kamacite_binding_json(binding: &PilotApplicationHostBinding) -> String {
    let operations = binding
        .effect_operation_identities()
        .iter()
        .map(identity_json)
        .collect::<Vec<_>>()
        .join(",");
    let non_claims = binding
        .non_claims()
        .iter()
        .map(|value| format!("\"{value}\""))
        .collect::<Vec<_>>()
        .join(",");
    format!(
        concat!(
            "{{\"kind\":\"application-host-binding\",",
            "\"schema_version\":\"{}\",",
            "\"application_identity\":{},",
            "\"entrypoint_identity\":{},",
            "\"input_schema_identity\":{},",
            "\"output_schema_identity\":{},",
            "\"effect_summary_identity\":{},",
            "\"effect_summary_closed\":true,",
            "\"effect_operation_identities\":[{}],",
            "\"host_profile_identity\":{},",
            "\"alternative_host_profile_identities\":[],",
            "\"fallback_host_profile_identity\":{},",
            "\"adapter_cohort_identity\":{},",
            "\"resource_profile_identity\":{},",
            "\"policy_identity\":{},",
            "\"compatibility_identity\":{},",
            "\"expected_adapter_cohort_identity\":{},",
            "\"expected_resource_profile_identity\":{},",
            "\"expected_host_profile_identity\":{},",
            "\"metadata\":{{\"aliases\":[],\"display_name\":null,\"documentation\":null,\"path\":null,\"source_span\":null}},",
            "\"semantic_claims\":[],",
            "\"non_claims\":[{}]}}"
        ),
        binding.kamacite_schema(),
        identity_json(binding.application_identity()),
        identity_json(binding.entrypoint_identity()),
        identity_json(binding.input_schema_identity()),
        identity_json(binding.output_schema_identity()),
        identity_json(binding.effect_summary_identity()),
        operations,
        identity_json(binding.host_profile_identity()),
        JSON_NULL,
        identity_json(binding.adapter_cohort_identity()),
        identity_json(binding.resource_profile_identity()),
        identity_json(binding.policy_identity()),
        identity_json(binding.compatibility_identity()),
        identity_json(binding.adapter_cohort_identity()),
        identity_json(binding.resource_profile_identity()),
        identity_json(binding.host_profile_identity()),
        non_claims,
    )
}

/// Render a bounded read-only explanation for an admitted binding.
#[must_use]
pub fn explain_application_host_binding(binding: &PilotApplicationHostBinding) -> String {
    let mut output = String::new();
    let _ = writeln!(output, "schema={PILOT_EXPLANATION_SCHEMA}");
    let _ = writeln!(
        output,
        "projection_identity={}",
        binding.binding_identity().hex()
    );
    let _ = writeln!(
        output,
        "application_identity={}",
        binding.application_identity().hex()
    );
    let _ = writeln!(
        output,
        "entrypoint_identity={}",
        binding.entrypoint_identity().hex()
    );
    let _ = writeln!(
        output,
        "effect_summary_identity={}",
        binding.effect_summary_identity().hex()
    );
    let _ = writeln!(
        output,
        "effect_classes={}",
        binding.effect_operation_identities().len()
    );
    let _ = writeln!(
        output,
        "host_profile_identity={}",
        binding.host_profile_identity().hex()
    );
    let _ = writeln!(
        output,
        "adapter_cohort_identity={}",
        binding.adapter_cohort_identity().hex()
    );
    let _ = writeln!(
        output,
        "resource_profile_identity={}",
        binding.resource_profile_identity().hex()
    );
    let _ = writeln!(output, "compatibility=exact-frozen-kamacite-cohort");
    let _ = writeln!(output, "redaction={REDACTION_MARKER}");
    for non_claim in binding.non_claims() {
        let _ = writeln!(output, "non_claim={non_claim}");
    }
    let _ = writeln!(output, "next_check={NEXT_FOCUSED_CHECK}");
    output
}

/// Render stable issue codes for a denied binding or dispatch attempt.
#[must_use]
pub fn explain_application_host_binding_error(error: &PilotError) -> String {
    let mut output = String::new();
    let _ = writeln!(output, "schema={PILOT_EXPLANATION_SCHEMA}");
    let _ = writeln!(output, "status=denied");
    for issue in error.issues() {
        let _ = writeln!(output, "issue={}", issue.code.as_str());
    }
    let _ = writeln!(output, "redaction={REDACTION_MARKER}");
    let _ = writeln!(output, "next_check={NEXT_FOCUSED_CHECK}");
    output
}

fn validate_binding_draft(draft: &PilotBindingDraft<'_>) -> Vec<PilotIssue> {
    let mut issues = Vec::new();
    validate_cohort(draft.cohort, &mut issues);
    validate_identity_domain(
        draft.application_identity,
        PilotIdentityDomain::TargetArtifact,
        FIELD_APPLICATION,
        &mut issues,
    );
    validate_identity_domain(
        draft.entrypoint_identity,
        PilotIdentityDomain::TargetArtifact,
        FIELD_ENTRYPOINT,
        &mut issues,
    );
    validate_identity_domain(
        draft.input_schema_identity,
        PilotIdentityDomain::Adapter,
        FIELD_INPUT_SCHEMA,
        &mut issues,
    );
    validate_identity_domain(
        draft.output_schema_identity,
        PilotIdentityDomain::Adapter,
        FIELD_OUTPUT_SCHEMA,
        &mut issues,
    );
    validate_identity_domain(
        draft.effect_summary_identity,
        PilotIdentityDomain::AlgebraicEffect,
        FIELD_EFFECT_SUMMARY,
        &mut issues,
    );
    validate_effects(draft, &mut issues);
    validate_host(draft, &mut issues);
    validate_identity_domain(
        draft.adapter_cohort_identity,
        PilotIdentityDomain::Adapter,
        FIELD_ADAPTER_COHORT,
        &mut issues,
    );
    validate_identity_domain(
        draft.resource_profile_identity,
        PilotIdentityDomain::Adapter,
        FIELD_RESOURCE_PROFILE,
        &mut issues,
    );
    validate_identity_domain(
        draft.policy_identity,
        PilotIdentityDomain::Adapter,
        FIELD_POLICY,
        &mut issues,
    );
    validate_identity_domain(
        draft.compatibility_identity,
        PilotIdentityDomain::EvidenceChain,
        FIELD_COMPATIBILITY,
        &mut issues,
    );
    validate_expected_cohorts(draft, &mut issues);
    validate_claim_boundary(draft, &mut issues);
    issues
}

fn validate_cohort(cohort: KamaciteContractCohort<'_>, issues: &mut Vec<PilotIssue>) {
    let exact = cohort.binding_schema == KAMACITE_BINDING_SCHEMA
        && cohort.receipt_schema == KAMACITE_BINDING_RECEIPT_SCHEMA
        && cohort.producer_revision == KAMACITE_PRODUCER_REVISION
        && cohort.schema_blake3 == KAMACITE_SCHEMA_BLAKE3
        && cohort.receipt_schema_blake3 == KAMACITE_RECEIPT_SCHEMA_BLAKE3
        && cohort.validation_receipt_blake3 == KAMACITE_VALIDATION_RECEIPT_BLAKE3
        && cohort.core_blake3 == KAMACITE_CORE_BLAKE3;
    if !exact {
        issues.push(issue(PilotIssueCode::StaleKamaciteContract, FIELD_COHORT));
    }
}

fn validate_effects(draft: &PilotBindingDraft<'_>, issues: &mut Vec<PilotIssue>) {
    if !draft.effect_summary_closed {
        issues.push(issue(
            PilotIssueCode::OpenEffectSummary,
            FIELD_EFFECT_SUMMARY,
        ));
    }
    if draft.effect_operation_identities.is_empty() {
        issues.push(issue(
            PilotIssueCode::MissingEffectOperation,
            FIELD_EFFECT_OPERATION,
        ));
    }
    if draft.effect_operation_identities.len() > MAX_EFFECT_OPERATIONS {
        issues.push(issue(
            PilotIssueCode::TooManyEffectOperations,
            FIELD_EFFECT_OPERATION,
        ));
    }
    let mut operations = draft.effect_operation_identities.to_vec();
    for operation in &operations {
        validate_identity_domain(
            operation,
            PilotIdentityDomain::EffectOperation,
            FIELD_EFFECT_OPERATION,
            issues,
        );
    }
    operations.sort();
    if operations.windows(2).any(|window| window[0] == window[1]) {
        issues.push(issue(
            PilotIssueCode::DuplicateEffectOperation,
            FIELD_EFFECT_OPERATION,
        ));
    }
}

fn validate_host(draft: &PilotBindingDraft<'_>, issues: &mut Vec<PilotIssue>) {
    match draft.host_profile_identities {
        [] => issues.push(issue(
            PilotIssueCode::MissingHostProfile,
            FIELD_HOST_PROFILE,
        )),
        [host] => validate_identity_domain(
            host,
            PilotIdentityDomain::Evaluator,
            FIELD_HOST_PROFILE,
            issues,
        ),
        _ => issues.push(issue(
            PilotIssueCode::AmbiguousHostProfile,
            FIELD_HOST_PROFILE,
        )),
    }
}

fn validate_expected_cohorts(draft: &PilotBindingDraft<'_>, issues: &mut Vec<PilotIssue>) {
    if draft.adapter_cohort_identity != draft.expected_adapter_cohort_identity {
        issues.push(issue(PilotIssueCode::CohortMismatch, FIELD_ADAPTER_COHORT));
    }
    if draft.resource_profile_identity != draft.expected_resource_profile_identity {
        issues.push(issue(
            PilotIssueCode::CohortMismatch,
            FIELD_RESOURCE_PROFILE,
        ));
    }
    if let [host] = draft.host_profile_identities {
        if host != draft.expected_host_profile_identity {
            issues.push(issue(PilotIssueCode::CohortMismatch, FIELD_HOST_PROFILE));
        }
    }
}

fn validate_claim_boundary(draft: &PilotBindingDraft<'_>, issues: &mut Vec<PilotIssue>) {
    if !draft.semantic_claims.is_empty() {
        issues.push(issue(
            PilotIssueCode::AuthorityOverclaim,
            FIELD_SEMANTIC_CLAIMS,
        ));
    }
    for required in REQUIRED_NON_CLAIMS {
        if !draft.non_claims.contains(&required) {
            issues.push(issue(PilotIssueCode::MissingNonClaim, FIELD_NON_CLAIMS));
        }
    }
    let sensitive = draft.sensitive;
    if sensitive.prompt.is_some()
        || sensitive.policy_body.is_some()
        || sensitive.tool_payload.is_some()
        || sensitive.credential.is_some()
        || sensitive.endpoint.is_some()
        || sensitive.environment_value.is_some()
        || sensitive.host_path.is_some()
    {
        issues.push(issue(PilotIssueCode::UnsafeField, FIELD_SENSITIVE));
    }
}

fn validate_dispatch_binding(draft: &PilotDispatchDraft<'_>) -> Vec<PilotIssue> {
    let mut issues = Vec::new();
    let binding = draft.binding;
    let commit = draft.commit;
    let sealed_unit = draft.sealed_unit;
    let operation = draft.operation;
    if commit.binding_identity() != binding.binding_identity() {
        issues.push(issue(PilotIssueCode::BindingMismatch, FIELD_BINDING));
    }
    if commit.session_binding_identity() != sealed_unit.durable_binding_identity() {
        issues.push(issue(
            PilotIssueCode::SessionBindingMismatch,
            FIELD_SESSION_BINDING,
        ));
    }
    if draft.effect_sequence <= commit.highest_binding_sequence() {
        issues.push(issue(PilotIssueCode::EffectBeforeBinding, FIELD_SEQUENCE));
    }
    validate_identity_domain(
        operation,
        PilotIdentityDomain::EffectOperation,
        FIELD_EFFECT_OPERATION,
        &mut issues,
    );
    if !binding.effect_operation_identities().contains(operation) {
        issues.push(issue(
            PilotIssueCode::UndeclaredOperation,
            FIELD_EFFECT_OPERATION,
        ));
    }
    if sealed_unit.state() != RuntimeAuthorityUnitState::Sealed
        || sealed_unit.seal_receipt_identity().is_none()
    {
        issues.push(issue(PilotIssueCode::MissingSeal, FIELD_SEAL));
    }
    let exact_seal = sealed_unit.authority_profile_identity()
        == binding.host_profile_identity().hex()
        && sealed_unit.adapter_cohort_identity() == binding.adapter_cohort_identity().hex()
        && sealed_unit.resource_profile_identity() == binding.resource_profile_identity().hex()
        && sealed_unit
            .operation_identities()
            .iter()
            .any(|identity| identity == operation.hex());
    if !exact_seal {
        issues.push(issue(PilotIssueCode::WidenedSeal, FIELD_SEAL));
    }
    validate_current_enforcement(draft.enforcement, &mut issues);
    issues
}

fn validate_current_enforcement(
    observations: &[PilotEnforcementObservation],
    issues: &mut Vec<PilotIssue>,
) {
    for class in PilotEnforcementClass::ALL {
        let matching = observations
            .iter()
            .filter(|observation| observation.class == class)
            .collect::<Vec<_>>();
        match matching.as_slice() {
            [observation] if observation.admitted => {}
            _ => issues.push(issue(
                PilotIssueCode::CurrentEnforcementDenied,
                FIELD_CURRENT_ENFORCEMENT,
            )),
        }
    }
}

fn binding_projection_hash(
    draft: &PilotBindingDraft<'_>,
    operations: &[PilotIdentity],
    non_claims: &[String],
) -> String {
    let mut hasher = blake3::Hasher::new();
    hash_part(&mut hasher, PILOT_BINDING_HASH_DOMAIN);
    hash_identity(&mut hasher, draft.application_identity);
    hash_identity(&mut hasher, draft.entrypoint_identity);
    hash_identity(&mut hasher, draft.input_schema_identity);
    hash_identity(&mut hasher, draft.output_schema_identity);
    hash_identity(&mut hasher, draft.effect_summary_identity);
    for operation in operations {
        hash_identity(&mut hasher, operation);
    }
    hash_identity(&mut hasher, &draft.host_profile_identities[0]);
    hash_identity(&mut hasher, draft.adapter_cohort_identity);
    hash_identity(&mut hasher, draft.resource_profile_identity);
    hash_identity(&mut hasher, draft.policy_identity);
    hash_identity(&mut hasher, draft.compatibility_identity);
    for non_claim in non_claims {
        hash_part(&mut hasher, non_claim);
    }
    hasher.finalize().to_hex().to_string()
}

fn validate_identity_domain(
    identity: &PilotIdentity,
    expected: PilotIdentityDomain,
    field: &'static str,
    issues: &mut Vec<PilotIssue>,
) {
    if !is_valid_hash(identity.hex()) {
        issues.push(issue(PilotIssueCode::InvalidIdentity, field));
    }
    if identity.domain() != expected {
        issues.push(issue(PilotIssueCode::WrongIdentityDomain, field));
    }
}

fn hash_identity(hasher: &mut blake3::Hasher, identity: &PilotIdentity) {
    hash_part(hasher, identity.domain().as_str());
    hash_part(hasher, identity.hex());
}

fn hash_part(hasher: &mut blake3::Hasher, value: &str) {
    hasher.update(value.as_bytes());
    hasher.update(HASH_SEPARATOR);
}

fn identity_json(identity: &PilotIdentity) -> String {
    format!(
        "{{\"algorithm\":\"blake3\",\"domain\":\"{}\",\"hex\":\"{}\"}}",
        identity.domain().as_str(),
        identity.hex()
    )
}

fn is_valid_hash(value: &str) -> bool {
    value.len() == HASH_HEX_CHARS
        && value
            .as_bytes()
            .iter()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

const fn issue(code: PilotIssueCode, field: &'static str) -> PilotIssue {
    PilotIssue { code, field }
}

fn error(code: PilotIssueCode, field: &'static str) -> PilotError {
    PilotError::from_issues(vec![issue(code, field)])
}
