use super::*;

const GENERATED_POLICY: &str =
    include_str!("../../../../config/generated/provider-retry-policy.json");
const RUN_ID: &str = "run.retry.host";
const PROVIDER_ID: &str = "mock";
const MODEL_ID: &str = "mock-text";
const NATIVE_TRANSIENT: &str = "network-reset";
const NATIVE_THROTTLED: &str = "rate-limit";
const NATIVE_UNAVAILABLE: &str = "service-down";
const MODEL_PROMPT: &str = "retry the exact request";
const MODEL_RESPONSE: &str = "retry response";
const MAPPING_VERSION: &str = "v1";
const FIRST_ATTEMPT_COUNT: u32 = 1;
const STALE_ATTEMPT_COUNT: u32 = 4;
const FIRST_CALL_COUNT: u32 = 1;
const ZERO_CALL_COUNT: u32 = 0;
const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const POLICY_HASH: &str = "c4c5f883019c739cc4dd3c4edd790dfa038d095109f6bbac2e5f40f7483c15a6";

type ProviderRetryAttemptState = animus_runtime::provider_retry::ProviderRetryAttemptState;
type ProviderRetryCancellation = animus_runtime::provider_retry::ProviderRetryCancellation;
type ProviderRetryFailureObservation =
    animus_runtime::provider_retry::ProviderRetryFailureObservation;
type ProviderRetryInvocation = animus_runtime::provider_retry::ProviderRetryInvocation;
type ProviderRetryInvocationInput = animus_runtime::provider_retry::ProviderRetryInvocationInput;
type ProviderRetryPlanInput<'a> = animus_runtime::provider_retry::ProviderRetryPlanInput<'a>;
type ProviderRetryStreamState = animus_runtime::provider_retry::ProviderRetryStreamState;

#[test]
fn generated_policy_loads_and_revalidates() {
    let policy = parse_provider_retry_policy_json(GENERATED_POLICY).expect("generated policy");

    assert_eq!(policy.policy_hash(), POLICY_HASH);
    assert_eq!(policy.max_attempts(), STALE_ATTEMPT_COUNT);
}

#[test]
fn malformed_unknown_and_stale_generated_policy_fail_closed() {
    let malformed = "{";
    let unknown = GENERATED_POLICY.replacen(
        "\"schema\":",
        "\"unexpected\": true, \"schema\":",
        FIRST_CALL_COUNT as usize,
    );
    let stale = GENERATED_POLICY.replacen(POLICY_HASH, ZERO_HASH, FIRST_CALL_COUNT as usize);

    assert_eq!(
        parse_provider_retry_policy_json(malformed),
        Err(ProviderRetryConfigError::Json(
            ProviderRetryJsonErrorKind::EndOfInput
        ))
    );
    assert_eq!(
        parse_provider_retry_policy_json(unknown.as_str()),
        Err(ProviderRetryConfigError::Json(
            ProviderRetryJsonErrorKind::Data
        ))
    );
    assert_eq!(
        parse_provider_retry_policy_json(stale.as_str()),
        Err(ProviderRetryConfigError::Validation(
            ProviderRetryIssue::StalePolicy
        ))
    );
}

#[test]
fn injected_timer_is_called_once_and_missing_timer_is_unavailable() {
    let (_policy, invocation, decision) = admitted_decision(NATIVE_TRANSIENT);
    let wait = animus_runtime::provider_retry::plan_provider_retry_wait(&decision)
        .expect("timer wait plan");
    let mut timer = FakeTimer {
        call_count: ZERO_CALL_COUNT,
        outcome: ProviderRetryTimerOutcome::Completed,
    };
    let completed = execute_provider_retry_wait(Some(&mut timer), wait.request());
    let unavailable = execute_provider_retry_wait(None, wait.request());

    assert_eq!(timer.call_count, FIRST_CALL_COUNT);
    assert_eq!(completed.outcome, ProviderRetryTimerOutcome::Completed);
    assert_eq!(completed.exact_plan_hash, invocation.exact_plan_hash());
    assert_eq!(unavailable.outcome, ProviderRetryTimerOutcome::Unavailable);
    assert_eq!(unavailable.run_id, RUN_ID);
}

#[test]
fn injected_timer_cancellation_blocks_existing_model_port() {
    let (_policy, invocation, decision) = admitted_decision(NATIVE_TRANSIENT);
    let wait = animus_runtime::provider_retry::plan_provider_retry_wait(&decision)
        .expect("timer wait plan");
    let mut timer = FakeTimer {
        call_count: ZERO_CALL_COUNT,
        outcome: ProviderRetryTimerOutcome::Cancelled,
    };
    let observation = execute_provider_retry_wait(Some(&mut timer), wait.request());
    let evaluation = animus_runtime::provider_retry::evaluate_provider_retry_timer(
        &decision,
        wait.state(),
        &observation,
    )
    .expect("cancelled timer");
    let model_request = model_request();
    let mut dispatcher = FakeDispatch {
        call_count: ZERO_CALL_COUNT,
    };
    let result = dispatch_provider_retry_exact(
        Some(&mut dispatcher),
        ProviderRetryDispatchInput {
            gate: evaluation.gate(),
            invocation: &invocation,
            request: &model_request,
        },
    );

    assert_eq!(
        result,
        Err(ProviderRetryDispatchError::GateBlocked(
            ProviderRetryDispatchGateStatus::Cancelled
        ))
    );
    assert_eq!(dispatcher.call_count, ZERO_CALL_COUNT);
}

#[test]
fn exact_dispatch_runs_once_only_after_completed_correlated_timer() {
    let (_policy, invocation, decision) = admitted_decision(NATIVE_TRANSIENT);
    let evaluation = completed_timer_evaluation(&decision);
    let request = model_request();
    let mut dispatcher = FakeDispatch {
        call_count: ZERO_CALL_COUNT,
    };
    let response = dispatch_provider_retry_exact(
        Some(&mut dispatcher),
        ProviderRetryDispatchInput {
            gate: evaluation.gate(),
            invocation: &invocation,
            request: &request,
        },
    )
    .expect("exact dispatch");

    assert_eq!(dispatcher.call_count, FIRST_CALL_COUNT);
    assert_eq!(response.message(), MODEL_RESPONSE);
}

#[test]
fn adapter_errors_remain_host_owned_after_admitted_dispatch() {
    let (_policy, invocation, decision) = admitted_decision(NATIVE_TRANSIENT);
    let evaluation = completed_timer_evaluation(&decision);
    let request = model_request();
    let mut dispatcher = FailingDispatch;
    let result = dispatch_provider_retry_exact(
        Some(&mut dispatcher),
        ProviderRetryDispatchInput {
            gate: evaluation.gate(),
            invocation: &invocation,
            request: &request,
        },
    );

    assert_eq!(
        result,
        Err(ProviderRetryDispatchError::Adapter(FakeDispatchError))
    );
    assert_eq!(
        evaluation.gate().status(),
        ProviderRetryDispatchGateStatus::Ready
    );
}

#[test]
fn throttled_and_unavailable_failures_retry_through_existing_model_port() {
    for native_class in [NATIVE_THROTTLED, NATIVE_UNAVAILABLE] {
        let (_policy, invocation, decision) = admitted_decision(native_class);
        let evaluation = completed_timer_evaluation(&decision);
        let request = model_request();
        let mut dispatcher = FakeDispatch {
            call_count: ZERO_CALL_COUNT,
        };
        let response = dispatch_provider_retry_exact(
            Some(&mut dispatcher),
            ProviderRetryDispatchInput {
                gate: evaluation.gate(),
                invocation: &invocation,
                request: &request,
            },
        )
        .expect("existing model port dispatch");

        assert_eq!(dispatcher.call_count, FIRST_CALL_COUNT);
        assert_eq!(response.message(), MODEL_RESPONSE);
    }
}

#[test]
fn blocked_timer_and_missing_dispatcher_never_invoke_provider() {
    let (_policy, invocation, decision) = admitted_decision(NATIVE_TRANSIENT);
    let wait = animus_runtime::provider_retry::plan_provider_retry_wait(&decision)
        .expect("timer wait plan");
    let mut stale = timer_observation(&decision, ProviderRetryTimerOutcome::Completed);
    stale.next_attempt_count = STALE_ATTEMPT_COUNT;
    let blocked = animus_runtime::provider_retry::evaluate_provider_retry_timer(
        &decision,
        wait.state(),
        &stale,
    )
    .expect("blocked timer");
    let ready_observation = timer_observation(&decision, ProviderRetryTimerOutcome::Completed);
    let ready = animus_runtime::provider_retry::evaluate_provider_retry_timer(
        &decision,
        blocked.state(),
        &ready_observation,
    )
    .expect("ready timer");
    let request = model_request();
    let changed_request =
        animus_adapters::ModelRequest::new("changed retry request").expect("changed model request");
    let mut dispatcher = FakeDispatch {
        call_count: ZERO_CALL_COUNT,
    };
    let blocked_input = ProviderRetryDispatchInput {
        gate: blocked.gate(),
        invocation: &invocation,
        request: &request,
    };
    let stale_input = ProviderRetryDispatchInput {
        gate: ready.gate(),
        invocation: &invocation,
        request: &changed_request,
    };
    let ready_input = ProviderRetryDispatchInput {
        gate: ready.gate(),
        invocation: &invocation,
        request: &request,
    };

    assert_eq!(
        evaluate_provider_retry_dispatch(&blocked_input),
        ProviderRetryDispatchAdmission::GateBlocked(ProviderRetryDispatchGateStatus::BlockedStale)
    );
    assert_eq!(
        evaluate_provider_retry_dispatch(&stale_input),
        ProviderRetryDispatchAdmission::BindingMismatch
    );
    assert_eq!(
        dispatch_provider_retry_exact(Some(&mut dispatcher), blocked_input),
        Err(ProviderRetryDispatchError::GateBlocked(
            ProviderRetryDispatchGateStatus::BlockedStale
        ))
    );
    assert_eq!(
        dispatch_provider_retry_exact(Some(&mut dispatcher), stale_input),
        Err(ProviderRetryDispatchError::BindingMismatch)
    );
    assert_eq!(dispatcher.call_count, ZERO_CALL_COUNT);
    assert_eq!(
        dispatch_provider_retry_exact::<FakeDispatch>(None, ready_input),
        Err(ProviderRetryDispatchError::ServiceUnavailable)
    );
    assert_eq!(dispatcher.call_count, ZERO_CALL_COUNT);
}

struct FakeTimer {
    call_count: u32,
    outcome: ProviderRetryTimerOutcome,
}

impl ProviderRetryTimerService for FakeTimer {
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

struct FakeDispatch {
    call_count: u32,
}

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for FakeDispatch
{
    type Error = FakeDispatchError;

    fn call(
        &mut self,
        _request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        self.call_count = self.call_count.saturating_add(FIRST_CALL_COUNT);
        animus_adapters::ModelResponse::new(MODEL_RESPONSE).map_err(|_error| FakeDispatchError)
    }
}

struct FailingDispatch;

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for FailingDispatch
{
    type Error = FakeDispatchError;

    fn call(
        &mut self,
        _request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        Err(FakeDispatchError)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct FakeDispatchError;

fn model_request() -> animus_adapters::ModelRequest {
    animus_adapters::ModelRequest::new(MODEL_PROMPT).expect("model request")
}

fn admitted_decision(
    native_class: &str,
) -> (
    ProviderRetryPolicy,
    ProviderRetryInvocation,
    animus_runtime::provider_retry::ProviderRetryDecision,
) {
    let policy = parse_provider_retry_policy_json(GENERATED_POLICY).expect("generated policy");
    let invocation = invocation(&policy);
    let state = ProviderRetryAttemptState {
        run_id: RUN_ID.to_string(),
        completed_attempt_count: FIRST_ATTEMPT_COUNT,
        exact_plan_hash: invocation.exact_plan_hash().to_string(),
        stream_state: ProviderRetryStreamState::TransportStarted,
        cancellation: ProviderRetryCancellation::None,
    };
    let failure = ProviderRetryFailureObservation {
        provider_family: PROVIDER_ID.to_string(),
        mapping_version: MAPPING_VERSION.to_string(),
        native_class: native_class.to_string(),
    };
    let decision = animus_runtime::provider_retry::plan_provider_retry(ProviderRetryPlanInput {
        policy: &policy,
        current: &invocation,
        next: &invocation,
        state: &state,
        observation: &failure,
    })
    .expect("retry decision");
    debug_assert_eq!(
        decision.status(),
        animus_runtime::provider_retry::ProviderRetryDecisionStatus::Retry
    );
    debug_assert_eq!(decision.exact_plan_hash(), invocation.exact_plan_hash());
    (policy, invocation, decision)
}

fn invocation(policy: &ProviderRetryPolicy) -> ProviderRetryInvocation {
    let hash = |value: &str| blake3::hash(value.as_bytes()).to_hex().to_string();
    let mut input = ProviderRetryInvocationInput {
        provider_id: PROVIDER_ID.to_string(),
        model_id: MODEL_ID.to_string(),
        request_hash: retry_model_request_hash(&model_request()),
        profile_plan_hash: hash("profile"),
        prompt_hash: hash("prompt"),
        tool_catalog_hash: hash("tools"),
        response_format_hash: hash("format"),
        reasoning_intent_hash: hash("reasoning"),
        stream_intent_hash: hash("stream"),
        credential_handle_hash: hash("credential"),
        policy_hash: policy.policy_hash().to_string(),
        exact_plan_hash: String::new(),
    };
    input.exact_plan_hash =
        animus_runtime::provider_retry::provider_retry_invocation_input_content_hash(&input);
    let invocation =
        animus_runtime::provider_retry::validate_provider_retry_invocation(&input, policy)
            .expect("invocation");
    debug_assert_eq!(invocation.request_hash(), input.request_hash);
    debug_assert_eq!(invocation.policy_hash(), policy.policy_hash());
    invocation
}

fn completed_timer_evaluation(
    decision: &animus_runtime::provider_retry::ProviderRetryDecision,
) -> animus_runtime::provider_retry::ProviderRetryTimerEvaluation {
    let wait = animus_runtime::provider_retry::plan_provider_retry_wait(decision)
        .expect("timer wait plan");
    let observation = timer_observation(decision, ProviderRetryTimerOutcome::Completed);
    animus_runtime::provider_retry::evaluate_provider_retry_timer(
        decision,
        wait.state(),
        &observation,
    )
    .expect("completed timer")
}

fn timer_observation(
    decision: &animus_runtime::provider_retry::ProviderRetryDecision,
    outcome: ProviderRetryTimerOutcome,
) -> ProviderRetryTimerObservation {
    ProviderRetryTimerObservation {
        run_id: RUN_ID.to_string(),
        next_attempt_count: decision.next_attempt_count().expect("next attempt"),
        exact_plan_hash: decision.exact_plan_hash().to_string(),
        decision_hash: decision.decision_hash().to_string(),
        outcome,
    }
}
