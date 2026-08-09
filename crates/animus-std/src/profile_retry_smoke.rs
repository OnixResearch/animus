//! Packaged fake-backed host smoke for exact profile-bound provider retry.

use animus_runtime::provider_retry::{
    ProviderRetryAttemptState, ProviderRetryCancellation, ProviderRetryDecisionStatus,
    ProviderRetryFailureObservation, ProviderRetryInvocation, ProviderRetryInvocationInput,
    ProviderRetryPlanInput, ProviderRetryPolicy, ProviderRetryReceiptInput,
    ProviderRetryReceiptStatus, ProviderRetrySensitiveDraft, ProviderRetryStreamState,
    ProviderRetryTimerObservation, ProviderRetryTimerOutcome, ProviderRetryTimerRequest,
};
use animus_runtime::{
    ModelRuntimeBinding, ModelRuntimeProfileResolution, ModelRuntimeProfileSet,
    ModelRuntimeTargetDraft, ResolvedModelRuntimeProfile,
};

const ARG_PROFILE_POLICY: &str = "--profile-policy";
const ARG_RETRY_POLICY: &str = "--retry-policy";
const ARG_PROVIDER: &str = "--provider";
const ARG_MODEL: &str = "--model";
const ARG_ROUTER_ROLE: &str = "--router-role";
const ARG_PROMPT: &str = "--prompt";
const ARG_FAILURE_CLASS: &str = "--failure-class";
const ARG_CANCEL_TIMER: &str = "--cancel-timer";
const ARG_CHANGE_REQUEST: &str = "--change-request";
const ARG_MISSING_MODEL_SERVICE: &str = "--missing-model-service";
const MAPPING_VERSION: &str = "v1";
const RUN_ID: &str = "profile-retry-smoke";
const RESPONSE_TEXT: &str = "deterministic profile retry response";
const CHANGED_REQUEST_TEXT: &str = "changed profile retry request";
const RECORD_HEADER: &str = "ANIMUS_PROFILE_RETRY_SMOKE";
const RECORD_NON_CLAIM: &str = "NON_CLAIM";
const SMOKE_SCHEMA: &str = "animus.profile-retry-smoke.v1";
const STATUS_COMPLETED: &str = "completed";
const STATUS_FAILED: &str = "failed";
const ERROR_MISSING_ARGUMENT: &str = "missing-argument";
const ERROR_MISSING_ARGUMENT_VALUE: &str = "missing-argument-value";
const ERROR_UNKNOWN_ARGUMENT: &str = "unknown-argument";
const ERROR_PROFILE_POLICY: &str = "profile-policy";
const ERROR_RETRY_POLICY: &str = "retry-policy";
const ERROR_PROFILE_TARGET: &str = "profile-target";
const ERROR_PROFILE_RESOLUTION: &str = "profile-resolution";
const ERROR_REQUEST: &str = "model-request";
const ERROR_INVOCATION: &str = "retry-invocation";
const ERROR_DECISION: &str = "retry-decision";
const ERROR_TIMER_PLAN: &str = "retry-timer-plan";
const ERROR_TIMER_EVALUATION: &str = "retry-timer-evaluation";
const ERROR_DISPATCH: &str = "retry-dispatch";
const ERROR_RECEIPT: &str = "retry-receipt";
const PROMPT_HASH_DOMAIN: &str = "animus.profile-retry-smoke.prompt.v1";
const TOOL_CATALOG_HASH_DOMAIN: &str = "animus.profile-retry-smoke.tool-catalog.v1";
const RESPONSE_FORMAT_HASH_DOMAIN: &str = "animus.profile-retry-smoke.response-format.v1";
const REASONING_HASH_DOMAIN: &str = "animus.profile-retry-smoke.reasoning.v1";
const STREAM_HASH_DOMAIN: &str = "animus.profile-retry-smoke.stream.v1";
const CREDENTIAL_HASH_DOMAIN: &str = "animus.profile-retry-smoke.credential-class.v1";
const RESPONSE_HASH_DOMAIN: &str = "animus.profile-retry-smoke.response.v1";
const RESPONSE_FORMAT_LABEL: &str = "text";
const REASONING_LABEL: &str = "none";
const STREAM_LABEL: &str = "non-streaming";
const CREDENTIAL_CLASS_LABEL: &str = "fake-no-credential";
const HASH_SEPARATOR: &str = "|";
const FIRST_ATTEMPT_COUNT: u32 = 1;
const SECOND_ATTEMPT_COUNT: u32 = 2;
const FIRST_CALL_COUNT: u32 = 1;
const ZERO_CALL_COUNT: u32 = 0;
const NON_CLAIMS: &[&str] = &[
    "live-provider",
    "provider-reliability",
    "fallback",
    "streaming",
    "persistence",
    "tui",
    "daemon",
    "production-readiness",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct SmokeArgs {
    profile_policy_path: String,
    retry_policy_path: String,
    provider_id: String,
    model_id: String,
    router_role: Option<String>,
    prompt: String,
    failure_class: String,
    timer_outcome: ProviderRetryTimerOutcome,
    change_request: bool,
    model_service_available: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SmokeControls {
    timer_outcome: ProviderRetryTimerOutcome,
    change_request: bool,
    model_service_available: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SmokeTarget<'a> {
    provider_id: &'a str,
    model_id: &'a str,
    router_role: Option<&'a str>,
}

#[derive(Clone, Copy)]
struct SmokeInput<'a> {
    profiles: &'a ModelRuntimeProfileSet,
    retry_policy: &'a ProviderRetryPolicy,
    target: SmokeTarget<'a>,
    prompt: &'a str,
    failure_class: &'a str,
    controls: SmokeControls,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct SmokeReport {
    profile_id: String,
    profile_hash: String,
    resolution_hash: String,
    retry_policy_hash: String,
    invocation_hash: String,
    decision_hash: String,
    request_hash: String,
    response_hash: String,
    receipt_hash: String,
    failure_reason: &'static str,
    timer_calls: u32,
    model_calls: u32,
    completed_attempt_count: u32,
}

/// Run the packaged profile-bound provider-retry smoke from explicit CLI arguments.
///
/// # Errors
///
/// Returns a stable bounded error when argument parsing, explicit policy loading,
/// exact profile resolution, retry admission, injected effects, or safe receipt
/// projection fails.
pub fn run_packaged_profile_retry_smoke(args: &[String]) -> Result<String, String> {
    let parsed = parse_args(args)?;
    let profiles = super::load_model_runtime_profiles(parsed.profile_policy_path.as_str())
        .map_err(|_error| error(ERROR_PROFILE_POLICY))?;
    let retry_policy = super::load_provider_retry_policy(parsed.retry_policy_path.as_str())
        .map_err(|_error| error(ERROR_RETRY_POLICY))?;
    let controls = SmokeControls {
        timer_outcome: parsed.timer_outcome,
        change_request: parsed.change_request,
        model_service_available: parsed.model_service_available,
    };
    let report = run_smoke(SmokeInput {
        profiles: &profiles,
        retry_policy: &retry_policy,
        target: SmokeTarget {
            provider_id: parsed.provider_id.as_str(),
            model_id: parsed.model_id.as_str(),
            router_role: parsed.router_role.as_deref(),
        },
        prompt: parsed.prompt.as_str(),
        failure_class: parsed.failure_class.as_str(),
        controls,
    })?;
    Ok(render_report(&report))
}

fn parse_args(args: &[String]) -> Result<SmokeArgs, String> {
    let mut cursor = ArgCursor::new(args);
    let mut profile_policy_path = None;
    let mut retry_policy_path = None;
    let mut provider_id = None;
    let mut model_id = None;
    let mut router_role = None;
    let mut prompt = None;
    let mut failure_class = None;
    let mut timer_outcome = ProviderRetryTimerOutcome::Completed;
    let mut change_request = false;
    let mut model_service_available = true;

    while let Some(argument) = cursor.next() {
        match argument.as_str() {
            ARG_PROFILE_POLICY => {
                profile_policy_path = Some(cursor.next_value(ARG_PROFILE_POLICY)?);
            }
            ARG_RETRY_POLICY => retry_policy_path = Some(cursor.next_value(ARG_RETRY_POLICY)?),
            ARG_PROVIDER => provider_id = Some(cursor.next_value(ARG_PROVIDER)?),
            ARG_MODEL => model_id = Some(cursor.next_value(ARG_MODEL)?),
            ARG_ROUTER_ROLE => router_role = Some(cursor.next_value(ARG_ROUTER_ROLE)?),
            ARG_PROMPT => prompt = Some(cursor.next_value(ARG_PROMPT)?),
            ARG_FAILURE_CLASS => failure_class = Some(cursor.next_value(ARG_FAILURE_CLASS)?),
            ARG_CANCEL_TIMER => timer_outcome = ProviderRetryTimerOutcome::Cancelled,
            ARG_CHANGE_REQUEST => change_request = true,
            ARG_MISSING_MODEL_SERVICE => model_service_available = false,
            _unknown => return Err(error(ERROR_UNKNOWN_ARGUMENT)),
        }
    }

    Ok(SmokeArgs {
        profile_policy_path: required(profile_policy_path)?,
        retry_policy_path: required(retry_policy_path)?,
        provider_id: required(provider_id)?,
        model_id: required(model_id)?,
        router_role,
        prompt: required(prompt)?,
        failure_class: required(failure_class)?,
        timer_outcome,
        change_request,
        model_service_available,
    })
}

fn run_smoke(input: SmokeInput<'_>) -> Result<SmokeReport, String> {
    // r[impl agent_harness_runtime.profile_retry_host_smoke.contract]
    // r[impl agent_harness_runtime.profile_retry_host_smoke.binding]
    let capability_identity = exact_capability_identity(input.profiles, input.target)?;
    let resolved =
        resolve_exact_profile(input.profiles, input.target, capability_identity.as_str())?;
    let request =
        animus_adapters::ModelRequest::new(input.prompt).map_err(|_error| error(ERROR_REQUEST))?;
    let invocation = build_invocation(&resolved, input.retry_policy, &request, input.prompt)?;
    let state = ProviderRetryAttemptState {
        run_id: RUN_ID.to_string(),
        completed_attempt_count: FIRST_ATTEMPT_COUNT,
        exact_plan_hash: invocation.exact_plan_hash().to_string(),
        stream_state: ProviderRetryStreamState::TransportStarted,
        cancellation: ProviderRetryCancellation::None,
    };
    let failure = ProviderRetryFailureObservation {
        provider_family: input.target.provider_id.to_string(),
        mapping_version: MAPPING_VERSION.to_string(),
        native_class: input.failure_class.to_string(),
    };
    let decision = animus_runtime::provider_retry::plan_provider_retry(ProviderRetryPlanInput {
        policy: input.retry_policy,
        current: &invocation,
        next: &invocation,
        state: &state,
        observation: &failure,
    })
    .map_err(|_error| error(ERROR_DECISION))?;
    if decision.status() != ProviderRetryDecisionStatus::Retry {
        return Err(error(ERROR_DECISION));
    }

    let wait = animus_runtime::provider_retry::plan_provider_retry_wait(&decision)
        .map_err(|_error| error(ERROR_TIMER_PLAN))?;
    let mut timer = FakeTimer {
        call_count: ZERO_CALL_COUNT,
        outcome: input.controls.timer_outcome,
    };
    let timer_observation = super::execute_provider_retry_wait(Some(&mut timer), wait.request());
    let timer_evaluation = animus_runtime::provider_retry::evaluate_provider_retry_timer(
        &decision,
        wait.state(),
        &timer_observation,
    )
    .map_err(|_error| error(ERROR_TIMER_EVALUATION))?;
    let dispatch_request = if input.controls.change_request {
        animus_adapters::ModelRequest::new(CHANGED_REQUEST_TEXT)
            .map_err(|_error| error(ERROR_REQUEST))?
    } else {
        request.clone()
    };
    let mut model = FakeModel {
        call_count: ZERO_CALL_COUNT,
    };
    let adapter = input.controls.model_service_available.then_some(&mut model);
    let response = super::dispatch_provider_retry_exact(
        adapter,
        super::ProviderRetryDispatchInput {
            gate: timer_evaluation.gate(),
            invocation: &invocation,
            request: &dispatch_request,
        },
    )
    .map_err(|_error| error(ERROR_DISPATCH))?;
    let reason_classes = [decision.reason()];
    let receipt =
        animus_runtime::provider_retry::project_provider_retry_receipt(ProviderRetryReceiptInput {
            policy_hash: input.retry_policy.policy_hash(),
            exact_plan_hash: invocation.exact_plan_hash(),
            completed_attempt_count: SECOND_ATTEMPT_COUNT,
            reason_classes: &reason_classes,
            timer_outcome: Some(timer_observation.outcome),
            status: ProviderRetryReceiptStatus::Succeeded,
            sensitive: ProviderRetrySensitiveDraft::default(),
        })
        .map_err(|_error| error(ERROR_RECEIPT))?;
    assert_eq!(timer.call_count, FIRST_CALL_COUNT);
    assert_eq!(model.call_count, FIRST_CALL_COUNT);
    assert_eq!(receipt.status(), ProviderRetryReceiptStatus::Succeeded);

    Ok(SmokeReport {
        profile_id: resolved.profile().profile_id().to_string(),
        profile_hash: resolved.profile().profile_hash().to_string(),
        resolution_hash: resolved.resolution_hash().to_string(),
        retry_policy_hash: input.retry_policy.policy_hash().to_string(),
        invocation_hash: invocation.exact_plan_hash().to_string(),
        decision_hash: decision.decision_hash().to_string(),
        request_hash: invocation.request_hash().to_string(),
        response_hash: domain_hash(RESPONSE_HASH_DOMAIN, response.message()),
        receipt_hash: receipt.receipt_hash().to_string(),
        failure_reason: decision.reason().as_str(),
        timer_calls: timer.call_count,
        model_calls: model.call_count,
        completed_attempt_count: receipt.completed_attempt_count(),
    })
}

fn exact_capability_identity(
    profiles: &ModelRuntimeProfileSet,
    target: SmokeTarget<'_>,
) -> Result<String, String> {
    let mut identity = None;
    for profile in profiles.profiles() {
        let ModelRuntimeBinding::Exact {
            provider_id: bound_provider,
            model_id: bound_model,
            router_role: bound_role,
            capability_identity,
        } = profile.binding()
        else {
            continue;
        };
        let role_matches = bound_role.as_deref() == target.router_role;
        if bound_provider == target.provider_id && bound_model == target.model_id && role_matches {
            if identity.is_some() {
                return Err(error(ERROR_PROFILE_TARGET));
            }
            identity = Some(capability_identity.clone());
        }
    }
    identity.ok_or_else(|| error(ERROR_PROFILE_TARGET))
}

fn resolve_exact_profile(
    profiles: &ModelRuntimeProfileSet,
    target: SmokeTarget<'_>,
    capability_identity: &str,
) -> Result<Box<ResolvedModelRuntimeProfile>, String> {
    let resolution = animus_runtime::resolve_model_runtime_profile(
        profiles,
        ModelRuntimeTargetDraft {
            provider_id: target.provider_id,
            model_id: target.model_id,
            router_role: target.router_role,
            capability_identity,
        },
    )
    .map_err(|_error| error(ERROR_PROFILE_RESOLUTION))?;
    match resolution {
        ModelRuntimeProfileResolution::Resolved(resolved) => Ok(resolved),
        ModelRuntimeProfileResolution::CompatibilityDisabled => {
            Err(error(ERROR_PROFILE_RESOLUTION))
        }
    }
}

fn build_invocation(
    resolved: &ResolvedModelRuntimeProfile,
    retry_policy: &ProviderRetryPolicy,
    request: &animus_adapters::ModelRequest,
    prompt: &str,
) -> Result<ProviderRetryInvocation, String> {
    let tool_catalog_hash = expanded_tool_hash(resolved);
    let mut input = ProviderRetryInvocationInput {
        provider_id: resolved.target().provider_id().to_string(),
        model_id: resolved.target().model_id().to_string(),
        request_hash: super::retry_model_request_hash(request),
        profile_plan_hash: resolved.resolution_hash().to_string(),
        prompt_hash: domain_hash(PROMPT_HASH_DOMAIN, prompt),
        tool_catalog_hash,
        response_format_hash: domain_hash(RESPONSE_FORMAT_HASH_DOMAIN, RESPONSE_FORMAT_LABEL),
        reasoning_intent_hash: domain_hash(REASONING_HASH_DOMAIN, REASONING_LABEL),
        stream_intent_hash: domain_hash(STREAM_HASH_DOMAIN, STREAM_LABEL),
        credential_handle_hash: domain_hash(CREDENTIAL_HASH_DOMAIN, CREDENTIAL_CLASS_LABEL),
        policy_hash: retry_policy.policy_hash().to_string(),
        exact_plan_hash: String::new(),
    };
    input.exact_plan_hash =
        animus_runtime::provider_retry::provider_retry_invocation_input_content_hash(&input);
    animus_runtime::provider_retry::validate_provider_retry_invocation(&input, retry_policy)
        .map_err(|_error| error(ERROR_INVOCATION))
}

fn expanded_tool_hash(resolved: &ResolvedModelRuntimeProfile) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(TOOL_CATALOG_HASH_DOMAIN.as_bytes());
    for tool in resolved.expanded_tools() {
        hasher.update(HASH_SEPARATOR.as_bytes());
        hasher.update(tool.name().as_bytes());
    }
    hasher.finalize().to_hex().to_string()
}

fn domain_hash(domain: &str, value: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    hasher.update(HASH_SEPARATOR.as_bytes());
    hasher.update(value.as_bytes());
    hasher.finalize().to_hex().to_string()
}

fn render_report(report: &SmokeReport) -> String {
    // r[impl agent_harness_runtime.profile_retry_host_smoke.receipt]
    let mut lines = vec![format!(
        "{RECORD_HEADER} schema={SMOKE_SCHEMA} status={STATUS_COMPLETED} profile_id={} profile_hash={} resolution_hash={} retry_policy_hash={} invocation_hash={} decision_hash={} request_hash={} response_hash={} receipt_hash={} reason={} timer_calls={} model_calls={} completed_attempts={}",
        report.profile_id,
        report.profile_hash,
        report.resolution_hash,
        report.retry_policy_hash,
        report.invocation_hash,
        report.decision_hash,
        report.request_hash,
        report.response_hash,
        report.receipt_hash,
        report.failure_reason,
        report.timer_calls,
        report.model_calls,
        report.completed_attempt_count,
    )];
    for surface in NON_CLAIMS {
        lines.push(format!("{RECORD_NON_CLAIM} surface={surface}"));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn required(value: Option<String>) -> Result<String, String> {
    value.ok_or_else(|| error(ERROR_MISSING_ARGUMENT))
}

fn error(issue: &str) -> String {
    format!("{RECORD_HEADER} status={STATUS_FAILED} issue={issue}")
}

struct ArgCursor<'a> {
    args: &'a [String],
    index: usize,
}

impl<'a> ArgCursor<'a> {
    const fn new(args: &'a [String]) -> Self {
        Self { args, index: 0 }
    }

    fn next(&mut self) -> Option<&'a String> {
        let argument = self.args.get(self.index)?;
        self.index = self.index.saturating_add(1);
        Some(argument)
    }

    fn next_value(&mut self, _argument: &str) -> Result<String, String> {
        let value = self
            .next()
            .ok_or_else(|| error(ERROR_MISSING_ARGUMENT_VALUE))?;
        Ok(value.clone())
    }
}

struct FakeTimer {
    call_count: u32,
    outcome: ProviderRetryTimerOutcome,
}

impl super::ProviderRetryTimerService for FakeTimer {
    fn wait(&mut self, request: &ProviderRetryTimerRequest) -> ProviderRetryTimerObservation {
        self.call_count = self.call_count.saturating_add(FIRST_CALL_COUNT);
        ProviderRetryTimerObservation {
            run_id: request.run_id.clone(),
            next_attempt_count: request.next_attempt_count,
            exact_plan_hash: request.exact_plan_hash.clone(),
            decision_hash: request.decision_hash.clone(),
            outcome: self.outcome,
        }
    }
}

struct FakeModel {
    call_count: u32,
}

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for FakeModel
{
    type Error = FakeModelError;

    fn call(
        &mut self,
        _request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        self.call_count = self.call_count.saturating_add(FIRST_CALL_COUNT);
        animus_adapters::ModelResponse::new(RESPONSE_TEXT).map_err(|_error| FakeModelError)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FakeModelError;

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATED_PROFILES: &str =
        include_str!("../../../config/generated/model-runtime-profiles.json");
    const GENERATED_RETRY_POLICY: &str =
        include_str!("../../../config/generated/provider-retry-policy.json");
    const PROVIDER_ID: &str = "mock";
    const MODEL_ID: &str = "mock-text";
    const PROMPT: &str = "private profile retry prompt";
    const RETRYABLE_FAILURE: &str = "network-reset";
    const TERMINAL_FAILURE: &str = "auth";

    #[derive(Clone, Copy)]
    struct TestCase<'a> {
        model_id: &'a str,
        failure_class: &'a str,
        controls: SmokeControls,
    }

    #[test]
    fn exact_loaded_profile_dispatches_one_retry_and_emits_safe_evidence() {
        // r[verify agent_harness_runtime.profile_retry_host_smoke.binding.pass]
        // r[verify agent_harness_runtime.profile_retry_host_smoke.receipt.safe]
        let profiles = super::super::parse_model_runtime_profiles_json(GENERATED_PROFILES)
            .expect("generated profiles");
        let retry_policy = super::super::parse_provider_retry_policy_json(GENERATED_RETRY_POLICY)
            .expect("generated retry policy");
        let report = run_smoke(test_input(
            &profiles,
            &retry_policy,
            TestCase {
                model_id: MODEL_ID,
                failure_class: RETRYABLE_FAILURE,
                controls: successful_controls(),
            },
        ))
        .expect("profile retry smoke");
        let output = render_report(&report);

        assert!(output.contains(RECORD_HEADER));
        assert!(output.contains("status=completed"));
        assert!(output.contains("profile_id=mock-standard"));
        assert!(output.contains("reason=transient_transport"));
        assert!(output.contains("timer_calls=1"));
        assert!(output.contains("model_calls=1"));
        assert!(output.contains("completed_attempts=2"));
        assert!(output.contains("surface=live-provider"));
        assert!(!output.contains(PROMPT));
        assert!(!output.contains(RESPONSE_TEXT));
    }

    #[test]
    fn terminal_failure_missing_profile_and_bad_arguments_fail_before_dispatch() {
        // r[verify agent_harness_runtime.profile_retry_host_smoke.contract.failure]
        // r[verify agent_harness_runtime.profile_retry_host_smoke.binding.failure]
        let profiles = super::super::parse_model_runtime_profiles_json(GENERATED_PROFILES)
            .expect("generated profiles");
        let retry_policy = super::super::parse_provider_retry_policy_json(GENERATED_RETRY_POLICY)
            .expect("generated retry policy");
        let terminal = run_smoke(test_input(
            &profiles,
            &retry_policy,
            TestCase {
                model_id: MODEL_ID,
                failure_class: TERMINAL_FAILURE,
                controls: successful_controls(),
            },
        ))
        .expect_err("terminal failure");
        let missing_profile = run_smoke(test_input(
            &profiles,
            &retry_policy,
            TestCase {
                model_id: "missing-model",
                failure_class: RETRYABLE_FAILURE,
                controls: successful_controls(),
            },
        ))
        .expect_err("missing profile");
        let missing_arguments = parse_args(&[]).expect_err("missing arguments");

        assert!(terminal.contains(ERROR_DECISION));
        assert!(missing_profile.contains(ERROR_PROFILE_TARGET));
        assert!(missing_arguments.contains(ERROR_MISSING_ARGUMENT));
    }

    #[test]
    fn cancelled_timer_changed_request_and_missing_model_service_fail_closed() {
        // r[verify agent_harness_runtime.profile_retry_host_smoke.binding.failure]
        let profiles = super::super::parse_model_runtime_profiles_json(GENERATED_PROFILES)
            .expect("generated profiles");
        let retry_policy = super::super::parse_provider_retry_policy_json(GENERATED_RETRY_POLICY)
            .expect("generated retry policy");
        let cancelled = run_smoke(test_input(
            &profiles,
            &retry_policy,
            TestCase {
                model_id: MODEL_ID,
                failure_class: RETRYABLE_FAILURE,
                controls: SmokeControls {
                    timer_outcome: ProviderRetryTimerOutcome::Cancelled,
                    ..successful_controls()
                },
            },
        ))
        .expect_err("cancelled timer");
        let changed = run_smoke(test_input(
            &profiles,
            &retry_policy,
            TestCase {
                model_id: MODEL_ID,
                failure_class: RETRYABLE_FAILURE,
                controls: SmokeControls {
                    change_request: true,
                    ..successful_controls()
                },
            },
        ))
        .expect_err("changed request");
        let missing_model = run_smoke(test_input(
            &profiles,
            &retry_policy,
            TestCase {
                model_id: MODEL_ID,
                failure_class: RETRYABLE_FAILURE,
                controls: SmokeControls {
                    model_service_available: false,
                    ..successful_controls()
                },
            },
        ))
        .expect_err("missing model service");

        assert!(cancelled.contains(ERROR_DISPATCH));
        assert!(changed.contains(ERROR_DISPATCH));
        assert!(missing_model.contains(ERROR_DISPATCH));
    }

    const fn test_input<'a>(
        profiles: &'a ModelRuntimeProfileSet,
        retry_policy: &'a ProviderRetryPolicy,
        case: TestCase<'a>,
    ) -> SmokeInput<'a> {
        SmokeInput {
            profiles,
            retry_policy,
            target: SmokeTarget {
                provider_id: PROVIDER_ID,
                model_id: case.model_id,
                router_role: None,
            },
            prompt: PROMPT,
            failure_class: case.failure_class,
            controls: case.controls,
        }
    }

    const fn successful_controls() -> SmokeControls {
        SmokeControls {
            timer_outcome: ProviderRetryTimerOutcome::Completed,
            change_request: false,
            model_service_available: true,
        }
    }
}
