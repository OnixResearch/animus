use serde::Deserialize;

use crate::model::{
    EXPECTED_DECISIONS, EXPECTED_PROOF_SOURCE_IDENTITY, LeanLoopBounds, LeanLoopError,
    LeanLoopFailureClass, LeanLoopProfile, LeanLoopReceipt, LeanSandboxObservation,
    LeanSandboxPosture, NON_CLAIMS, PROFILE_DOMAIN, PROFILE_SCHEMA, PROOF_SOURCE_DOMAIN,
    RECEIPT_DOMAIN, REDACTION_MARKER, TOOLCHAIN_COHORT,
};
use crate::protocol::{LeanCandidateDecision, ToolCandidate};

const FIELD_PROFILE: &str = "profile";
const FIELD_DECISION: &str = "decision";
const FIELD_RECEIPT: &str = "receipt";
const RECEIPT_SCHEMA: &str = "animus.lean-loop.receipt.v1";

#[derive(Deserialize)]
struct GeneratedProfile {
    schema: String,
    protocol: GeneratedProtocol,
    toolchain: GeneratedToolchain,
    allowed_decisions: Vec<String>,
    bounds: GeneratedBounds,
    launch: GeneratedLaunch,
    identity: GeneratedIdentity,
    redaction: GeneratedRedaction,
    non_claims: Vec<String>,
}

#[derive(Deserialize)]
struct GeneratedProtocol {
    identity: String,
    envelope_schema: String,
}

#[derive(Deserialize)]
struct GeneratedToolchain {
    cohort: String,
    package: String,
    source_module: String,
}

#[derive(Deserialize)]
struct GeneratedBounds {
    frame_bytes: u64,
    text_bytes: u64,
    sequence_limit: u64,
    diagnostic_bytes: u64,
    startup_poll_count: u32,
    response_poll_count: u32,
    shutdown_poll_count: u32,
    poll_interval_millis: u64,
    cleanup_poll_count: u32,
    argument_count: u64,
    target_count: u64,
    capability_count: u64,
}

#[derive(Deserialize)]
struct GeneratedLaunch {
    executable_selection: String,
    path_discovery: bool,
    inherited_environment: bool,
    environment_allowlist: Vec<String>,
    working_directory: String,
    sandbox_posture: String,
    fallback_policy: String,
    restart_policy: String,
}

#[derive(Deserialize)]
struct GeneratedIdentity {
    algorithm: String,
    profile_domain: String,
    artifact_domain: String,
    proof_source_domain: String,
    proof_source_identity: String,
    receipt_domain: String,
}

#[derive(Deserialize)]
struct GeneratedRedaction {
    marker: String,
    raw_payloads: bool,
    authoritative_completion_claim: bool,
}

/// Loads and fully validates the checked Nickel runtime projection.
///
/// # Errors
///
/// Returns [`LeanLoopError`] for malformed JSON, cohort drift, ambient launch posture, weakened
/// redaction/non-claims, or invalid numeric bounds.
pub fn load_generated_profile(json: &str) -> Result<LeanLoopProfile, LeanLoopError> {
    let generated =
        serde_json::from_str::<GeneratedProfile>(json).map_err(|_error| profile_error())?;
    validate_generated_profile(&generated)?;
    let bounds = LeanLoopBounds {
        frame_bytes: usize_value(generated.bounds.frame_bytes)?,
        text_bytes: usize_value(generated.bounds.text_bytes)?,
        sequence_limit: generated.bounds.sequence_limit,
        diagnostic_bytes: usize_value(generated.bounds.diagnostic_bytes)?,
        startup_poll_count: generated.bounds.startup_poll_count,
        response_poll_count: generated.bounds.response_poll_count,
        shutdown_poll_count: generated.bounds.shutdown_poll_count,
        poll_interval: std::time::Duration::from_millis(generated.bounds.poll_interval_millis),
        cleanup_poll_count: generated.bounds.cleanup_poll_count,
        argument_count: usize_value(generated.bounds.argument_count)?,
        target_count: usize_value(generated.bounds.target_count)?,
        capability_count: usize_value(generated.bounds.capability_count)?,
    };
    validate_bounds(&bounds)?;
    let sandbox_posture = match generated.launch.sandbox_posture.as_str() {
        "required" => LeanSandboxPosture::Required,
        "fixture-only" => LeanSandboxPosture::FixtureOnly,
        _unknown => return Err(profile_error()),
    };
    Ok(LeanLoopProfile {
        profile_identity: domain_hash(PROFILE_DOMAIN, json.as_bytes()),
        proof_source_identity: generated.identity.proof_source_identity.clone(),
        bounds,
        sandbox_posture,
    })
}

fn validate_generated_profile(profile: &GeneratedProfile) -> Result<(), LeanLoopError> {
    let exact = profile.schema == PROFILE_SCHEMA
        && profile.protocol.identity == crate::PROTOCOL_IDENTITY
        && profile.protocol.envelope_schema == crate::ENVELOPE_SCHEMA_IDENTITY
        && profile.toolchain.cohort == TOOLCHAIN_COHORT
        && profile.toolchain.package == "animus-lean-0.1.0"
        && profile.toolchain.source_module == "AnimusLean.Core"
        && profile.allowed_decisions == EXPECTED_DECISIONS
        && profile.launch.executable_selection == "explicit-absolute-path"
        && !profile.launch.path_discovery
        && !profile.launch.inherited_environment
        && profile.launch.environment_allowlist.is_empty()
        && profile.launch.working_directory == "explicit-absolute-path"
        && profile.launch.fallback_policy == "deny"
        && profile.launch.restart_policy == "deny-after-activation"
        && profile.identity.algorithm == "blake3"
        && profile.identity.profile_domain == PROFILE_DOMAIN
        && profile.identity.artifact_domain == crate::ARTIFACT_DOMAIN
        && profile.identity.proof_source_domain == PROOF_SOURCE_DOMAIN
        && profile.identity.proof_source_identity == EXPECTED_PROOF_SOURCE_IDENTITY
        && profile.identity.receipt_domain == RECEIPT_DOMAIN
        && profile.redaction.marker == REDACTION_MARKER
        && !profile.redaction.raw_payloads
        && !profile.redaction.authoritative_completion_claim
        && profile.non_claims == NON_CLAIMS;
    if exact { Ok(()) } else { Err(profile_error()) }
}

fn validate_bounds(bounds: &LeanLoopBounds) -> Result<(), LeanLoopError> {
    let valid = bounds.frame_bytes > 0
        && bounds.text_bytes > 0
        && bounds.text_bytes <= bounds.frame_bytes
        && bounds.sequence_limit > 0
        && bounds.diagnostic_bytes > 0
        && bounds.startup_poll_count > 0
        && bounds.response_poll_count > 0
        && bounds.shutdown_poll_count > 0
        && !bounds.poll_interval.is_zero()
        && bounds.cleanup_poll_count > 0
        && bounds.argument_count > 0
        && bounds.target_count > 0
        && bounds.capability_count > 0;
    if valid { Ok(()) } else { Err(profile_error()) }
}

fn usize_value(value: u64) -> Result<usize, LeanLoopError> {
    usize::try_from(value).map_err(|_error| profile_error())
}

pub(crate) fn reconstruct_decision(
    profile: &LeanLoopProfile,
    expected_correlation: &str,
    candidate: LeanCandidateDecision,
) -> Result<animus_runtime::RuntimeLoopDecision, LeanLoopError> {
    match candidate {
        LeanCandidateDecision::Model(prompt) => {
            validate_text(profile, prompt.as_str())?;
            let request = animus_adapters::ModelRequest::new(prompt.as_str())
                .map_err(|_error| decision_error())?;
            Ok(animus_runtime::RuntimeLoopDecision::CallModel { request })
        }
        LeanCandidateDecision::Tool(tool) => reconstruct_tool(profile, expected_correlation, &tool),
        LeanCandidateDecision::Respond(message) => {
            validate_text(profile, message.as_str())?;
            let response = animus_adapters::ModelResponse::new(message.as_str())
                .map_err(|_error| decision_error())?;
            Ok(animus_runtime::RuntimeLoopDecision::Respond { response })
        }
        LeanCandidateDecision::Stop => Ok(animus_runtime::RuntimeLoopDecision::Stop),
    }
}

fn reconstruct_tool(
    profile: &LeanLoopProfile,
    expected_correlation: &str,
    tool: &ToolCandidate,
) -> Result<animus_runtime::RuntimeLoopDecision, LeanLoopError> {
    if tool.correlation_identity != expected_correlation
        || tool.arguments.is_empty()
        || tool.arguments.len() > profile.bounds.argument_count
        || tool.mutation_targets.len() > profile.bounds.target_count
        || tool.required_capabilities.is_empty()
        || tool.required_capabilities.len() > profile.bounds.capability_count
    {
        return Err(decision_error());
    }
    validate_text(profile, tool.name.as_str())?;
    validate_text(profile, tool.schema_identity.as_str())?;
    let allowed_names = tool
        .arguments
        .iter()
        .map(|argument| argument.name.as_str())
        .collect::<Vec<_>>();
    let fields = tool
        .arguments
        .iter()
        .map(|argument| animus_adapters::tool::ArgumentFieldDraft {
            name: Some(argument.name.as_str()),
            value: Some(argument.value.as_str()),
        })
        .collect::<Vec<_>>();
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: tool.schema_identity.as_str(),
            allowed_field_names: allowed_names.as_slice(),
            compatibility_rules: &[],
            fields: fields.as_slice(),
        },
    )
    .map_err(|_error| decision_error())?;
    let mutation_targets = tool
        .mutation_targets
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let capabilities = tool
        .required_capabilities
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let request = animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(tool.name.as_str()),
        arguments: Some(&arguments),
        correlation_id: Some(tool.correlation_identity.as_str()),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: tool.visible_output_budget,
        },
        mutation_targets: mutation_targets.as_slice(),
        required_capability_labels: capabilities.as_slice(),
        approval_posture: tool.approval_posture.as_str(),
    })
    .map_err(|_error| decision_error())?;
    Ok(animus_runtime::RuntimeLoopDecision::CallTool { request })
}

fn validate_text(profile: &LeanLoopProfile, value: &str) -> Result<(), LeanLoopError> {
    let safe = !value.trim().is_empty()
        && value.len() <= profile.bounds.text_bytes
        && !value.chars().any(char::is_control);
    if safe { Ok(()) } else { Err(decision_error()) }
}

/// Projects a domain-separated metadata-only receipt.
///
/// # Errors
///
/// Returns [`LeanLoopError`] when any supplied identity or safe class contains sensitive or
/// path-shaped material.
pub fn project_receipt(
    profile: &LeanLoopProfile,
    context: crate::LeanReceiptContext<'_>,
    observation: &crate::LeanTransportObservation,
) -> Result<LeanLoopReceipt, LeanLoopError> {
    let artifact_identity = context.artifact_identity;
    let decision_class = context.decision_class;
    for value in [
        profile.profile_identity.as_str(),
        profile.proof_source_identity.as_str(),
        artifact_identity,
        decision_class,
    ] {
        validate_receipt_value(value)?;
    }
    let sandbox_class = match context.sandbox {
        LeanSandboxObservation::Enforced => "enforced",
        LeanSandboxObservation::FixtureOnly => "fixture_only",
        LeanSandboxObservation::Missing => "missing",
    };
    let failure_class = observation
        .failure
        .map(|value| String::from(value.as_str()));
    let frame_count = observation.frame_count;
    let byte_count = observation.byte_count;
    let body = format!(
        "{RECEIPT_SCHEMA}\0{}\0{artifact_identity}\0{}\0{decision_class}\0{}\0{sandbox_class}\0{}\0{frame_count}\0{byte_count}",
        profile.profile_identity,
        profile.proof_source_identity,
        failure_class.as_deref().unwrap_or("none"),
        observation.cleanup.as_str(),
    );
    Ok(LeanLoopReceipt {
        schema: RECEIPT_SCHEMA,
        profile_identity: profile.profile_identity.clone(),
        artifact_identity: String::from(artifact_identity),
        proof_source_identity: profile.proof_source_identity.clone(),
        decision_class: String::from(decision_class),
        failure_class,
        sandbox_class: String::from(sandbox_class),
        cleanup_class: String::from(observation.cleanup.as_str()),
        frame_count: observation.frame_count,
        byte_count: observation.byte_count,
        redaction_marker: REDACTION_MARKER,
        non_claims: NON_CLAIMS.to_vec(),
        receipt_identity: domain_hash(RECEIPT_DOMAIN, body.as_bytes()),
    })
}

fn validate_receipt_value(value: &str) -> Result<(), LeanLoopError> {
    let lowered = value.to_ascii_lowercase();
    let unsafe_value = value.trim().is_empty()
        || value.len() > crate::model::REDACTION_MARKER.len().saturating_mul(2)
        || value.contains('/')
        || value.contains('\\')
        || value.contains('\0')
        || lowered.contains("secret")
        || lowered.contains("credential")
        || lowered.contains("prompt");
    if unsafe_value {
        Err(LeanLoopError::new(
            LeanLoopFailureClass::ReceiptRejected,
            FIELD_RECEIPT,
        ))
    } else {
        Ok(())
    }
}

pub(crate) fn domain_hash(domain: &str, bytes: &[u8]) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(&[0]);
    hasher.update(bytes);
    hasher.finalize().to_hex().to_string()
}

const fn profile_error() -> LeanLoopError {
    LeanLoopError::new(LeanLoopFailureClass::ProfileRejected, FIELD_PROFILE)
}

const fn decision_error() -> LeanLoopError {
    LeanLoopError::new(LeanLoopFailureClass::DecisionRejected, FIELD_DECISION)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::LeanCleanupStatus;
    use crate::protocol::{LeanCandidateDecision, ToolArgumentCandidate, ToolCandidate};
    use crate::test_support::{ARTIFACT_IDENTITY, profile};

    const CORRELATION: &str = "tool-call-1";
    const RECEIPT_FRAME_COUNT: u32 = 2;
    const RECEIPT_BYTE_COUNT: u64 = 512;
    const INVALID_TEXT_BOUND: u64 = 70_000;
    const GENERATED_PROFILE: &str =
        include_str!("../../../config/generated/lean-loop-profile.json");

    #[test]
    fn generated_profile_accepts_exact_source_and_rejects_policy_widening() {
        assert!(load_generated_profile(GENERATED_PROFILE).is_ok());
        let mutations = [
            ("/protocol/identity", serde_json::json!("unknown")),
            ("/toolchain/cohort", serde_json::json!("lean4-unreviewed")),
            ("/bounds/frame_bytes", serde_json::json!(0)),
            ("/bounds/text_bytes", serde_json::json!(INVALID_TEXT_BOUND)),
            ("/allowed_decisions/0", serde_json::json!("shell")),
            ("/launch/path_discovery", serde_json::json!(true)),
            ("/launch/inherited_environment", serde_json::json!(true)),
            ("/launch/fallback_policy", serde_json::json!("steel")),
            ("/launch/sandbox_posture", serde_json::Value::Null),
            (
                "/identity/proof_source_identity",
                serde_json::json!("stale"),
            ),
            ("/toolchain/source_module", serde_json::json!("unknown")),
            ("/toolchain/package", serde_json::json!("/private/path")),
            ("/redaction/raw_payloads", serde_json::json!(true)),
            ("/non_claims", serde_json::json!([])),
        ];
        for (pointer, replacement) in mutations {
            let mut value = serde_json::from_str::<serde_json::Value>(GENERATED_PROFILE)
                .expect("generated profile JSON");
            let target = value.pointer_mut(pointer).expect("fixture pointer");
            *target = replacement;
            assert!(
                load_generated_profile(value.to_string().as_str()).is_err(),
                "mutation {pointer} must fail"
            );
        }
    }

    #[test]
    fn model_response_stop_and_current_tool_revalidate() {
        let profile = profile();
        assert!(matches!(
            reconstruct_decision(
                &profile,
                CORRELATION,
                LeanCandidateDecision::Model(String::from("prompt"))
            ),
            Ok(animus_runtime::RuntimeLoopDecision::CallModel { .. })
        ));
        assert!(matches!(
            reconstruct_decision(
                &profile,
                CORRELATION,
                LeanCandidateDecision::Respond(String::from("done"))
            ),
            Ok(animus_runtime::RuntimeLoopDecision::Respond { .. })
        ));
        assert!(matches!(
            reconstruct_decision(&profile, CORRELATION, LeanCandidateDecision::Stop),
            Ok(animus_runtime::RuntimeLoopDecision::Stop)
        ));
        assert!(matches!(
            reconstruct_decision(&profile, CORRELATION, LeanCandidateDecision::Tool(tool())),
            Ok(animus_runtime::RuntimeLoopDecision::CallTool { .. })
        ));
    }

    #[test]
    fn unsafe_candidate_fields_and_stale_tool_correlation_fail_closed() {
        let profile = profile();
        assert!(
            reconstruct_decision(
                &profile,
                CORRELATION,
                LeanCandidateDecision::Model(String::from("\0hidden"))
            )
            .is_err()
        );
        let mut stale = tool();
        stale.correlation_identity = String::from("stale");
        assert!(
            reconstruct_decision(&profile, CORRELATION, LeanCandidateDecision::Tool(stale))
                .is_err()
        );
        let mut legacy = tool();
        legacy.approval_posture = String::from("legacy");
        assert!(
            reconstruct_decision(&profile, CORRELATION, LeanCandidateDecision::Tool(legacy))
                .is_err()
        );
    }

    #[test]
    fn receipt_is_deterministic_and_rejects_sensitive_identity_input() {
        let profile = profile();
        let observation = crate::LeanTransportObservation {
            frame_count: RECEIPT_FRAME_COUNT,
            byte_count: RECEIPT_BYTE_COUNT,
            failure: None,
            cleanup: LeanCleanupStatus::Completed,
        };
        let context = crate::LeanReceiptContext {
            artifact_identity: ARTIFACT_IDENTITY,
            decision_class: "model",
            sandbox: LeanSandboxObservation::FixtureOnly,
        };
        let first = project_receipt(&profile, context, &observation).expect("receipt");
        let second = project_receipt(&profile, context, &observation).expect("repeat receipt");
        assert_eq!(first, second);
        assert!(!format!("{first:?}").contains("provider payload"));
        assert!(
            project_receipt(
                &profile,
                crate::LeanReceiptContext {
                    artifact_identity: "/private/credential",
                    decision_class: "model",
                    sandbox: LeanSandboxObservation::FixtureOnly,
                },
                &observation,
            )
            .is_err()
        );
    }

    fn tool() -> ToolCandidate {
        ToolCandidate {
            name: String::from("search"),
            schema_identity: String::from("schema.search.v1"),
            arguments: vec![ToolArgumentCandidate {
                name: String::from("query"),
                value: String::from("adapter contracts"),
            }],
            correlation_identity: String::from(CORRELATION),
            visible_output_budget: 64,
            mutation_targets: vec![String::from("resource:file:README")],
            required_capabilities: vec![String::from("tool.read")],
            approval_posture: String::from("pre_approved"),
        }
    }
}
