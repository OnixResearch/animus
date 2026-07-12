//! Host-owned generated retry-policy loading and injected retry effects.

use serde::Deserialize;

type ProviderRetryDispatchGate = animus_runtime::provider_retry::ProviderRetryDispatchGate;
type ProviderRetryDispatchGateStatus =
    animus_runtime::provider_retry::ProviderRetryDispatchGateStatus;
type ModelRequest = animus_adapters::ModelRequest;
type ModelResponse = animus_adapters::ModelResponse;
type ProviderRetryInvocation = animus_runtime::provider_retry::ProviderRetryInvocation;
type ProviderRetryIssue = animus_runtime::provider_retry::ProviderRetryIssue;
type ProviderRetryMappingInput = animus_runtime::provider_retry::ProviderRetryMappingInput;
type ProviderRetryPolicy = animus_runtime::provider_retry::ProviderRetryPolicy;
type ProviderRetryPolicyInput = animus_runtime::provider_retry::ProviderRetryPolicyInput;
type ProviderRetryTimerObservation = animus_runtime::provider_retry::ProviderRetryTimerObservation;
type ProviderRetryTimerOutcome = animus_runtime::provider_retry::ProviderRetryTimerOutcome;
type ProviderRetryTimerRequest = animus_runtime::provider_retry::ProviderRetryTimerRequest;

/// Stable JSON failure category for generated retry policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderRetryJsonErrorKind {
    /// JSON syntax was malformed.
    Syntax,
    /// JSON values did not match the exact generated DTO shape.
    Data,
    /// JSON ended before a complete value was available.
    EndOfInput,
}

/// Safe host-shell error for loading generated retry policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProviderRetryConfigError {
    /// The configured file could not be read.
    Read(std::io::ErrorKind),
    /// JSON shape or syntax was invalid.
    Json(ProviderRetryJsonErrorKind),
    /// Runtime revalidation rejected policy content.
    Validation(ProviderRetryIssue),
}

/// Parse and revalidate generated retry-policy JSON without filesystem access.
///
/// # Errors
///
/// Returns a bounded JSON or runtime-validation category.
pub fn parse_provider_retry_policy_json(
    input: &str,
) -> Result<ProviderRetryPolicy, ProviderRetryConfigError> {
    let generated: GeneratedPolicy =
        serde_json::from_str(input).map_err(|error| classify_json_error(&error))?;
    let policy_input = generated.into_runtime_input();
    animus_runtime::provider_retry::validate_provider_retry_policy(&policy_input)
        .map_err(|error| ProviderRetryConfigError::Validation(error.issue()))
}

/// Read generated retry-policy JSON from an explicit host path and revalidate it.
///
/// # Errors
///
/// Returns a bounded read, JSON, or runtime-validation category.
pub fn load_provider_retry_policy(
    path: impl AsRef<std::path::Path>,
) -> Result<ProviderRetryPolicy, ProviderRetryConfigError> {
    let input = std::fs::read_to_string(path.as_ref())
        .map_err(|error| ProviderRetryConfigError::Read(error.kind()))?;
    parse_provider_retry_policy_json(input.as_str())
}

/// Injected host timer used only after pure retry admission.
pub trait ProviderRetryTimerService {
    /// Wait for the requested cancellable delay and return correlated evidence.
    fn wait(&mut self, request: &ProviderRetryTimerRequest) -> ProviderRetryTimerObservation;
}

/// Invoke an injected timer, or return fail-closed unavailable evidence.
#[must_use]
pub fn execute_provider_retry_wait(
    timer: Option<&mut dyn ProviderRetryTimerService>,
    request: &ProviderRetryTimerRequest,
) -> ProviderRetryTimerObservation {
    match timer {
        Some(timer) => timer.wait(request),
        None => ProviderRetryTimerObservation {
            run_id: request.run_id.clone(),
            next_attempt_count: request.next_attempt_count,
            exact_plan_hash: request.exact_plan_hash.clone(),
            decision_hash: request.decision_hash.clone(),
            outcome: ProviderRetryTimerOutcome::Unavailable,
        },
    }
}

/// Returns the BLAKE3 request identity used by the std model-adapter retry shell.
#[must_use]
pub fn retry_model_request_hash(request: &ModelRequest) -> String {
    blake3::hash(request.prompt().as_bytes())
        .to_hex()
        .to_string()
}

/// Exact host-owned model request bound to one admitted retry gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ProviderRetryDispatchInput<'a> {
    /// Pure timer-evidence gate.
    pub gate: &'a ProviderRetryDispatchGate,
    /// Validated exact invocation accompanying the retained host request.
    pub invocation: &'a ProviderRetryInvocation,
    /// Existing validated model request retained by the host.
    pub request: &'a ModelRequest,
}

/// Host dispatch failure; only the generic adapter variant may hold host-private diagnostics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ProviderRetryDispatchError<AdapterError> {
    /// Pure timer evaluation did not admit dispatch.
    GateBlocked(ProviderRetryDispatchGateStatus),
    /// The retained request was not bound to the gate's exact invocation.
    BindingMismatch,
    /// No injected model adapter was available.
    ServiceUnavailable,
    /// The existing host model adapter returned its host-private typed error.
    /// This value must be classified before crossing into reusable runtime or receipts.
    Adapter(AdapterError),
}

/// Dispatch one call through the existing injected model-adapter port after admission.
///
/// # Errors
///
/// Returns a bounded gate, binding, service, or typed adapter error. Gate, binding,
/// and service errors occur without invoking the adapter.
pub fn dispatch_provider_retry_exact<Adapter>(
    adapter: Option<&mut Adapter>,
    input: ProviderRetryDispatchInput<'_>,
) -> Result<ModelResponse, ProviderRetryDispatchError<Adapter::Error>>
where
    Adapter: animus_adapters::ComponentAdapter<ModelRequest, ModelResponse>,
{
    match evaluate_provider_retry_dispatch(&input) {
        ProviderRetryDispatchAdmission::Admitted => {}
        ProviderRetryDispatchAdmission::GateBlocked(status) => {
            return Err(ProviderRetryDispatchError::GateBlocked(status));
        }
        ProviderRetryDispatchAdmission::BindingMismatch => {
            return Err(ProviderRetryDispatchError::BindingMismatch);
        }
    }
    let adapter = adapter.ok_or(ProviderRetryDispatchError::ServiceUnavailable)?;
    animus_adapters::ComponentAdapter::call(adapter, input.request)
        .map_err(ProviderRetryDispatchError::Adapter)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ProviderRetryDispatchAdmission {
    Admitted,
    GateBlocked(ProviderRetryDispatchGateStatus),
    BindingMismatch,
}

fn evaluate_provider_retry_dispatch(
    input: &ProviderRetryDispatchInput<'_>,
) -> ProviderRetryDispatchAdmission {
    let Some(dispatch) = input.gate.request() else {
        return ProviderRetryDispatchAdmission::GateBlocked(input.gate.status());
    };
    let observed_request_hash = retry_model_request_hash(input.request);
    let is_exact_plan = dispatch.exact_plan_hash() == input.invocation.exact_plan_hash();
    let is_exact_request = observed_request_hash == input.invocation.request_hash();
    if !is_exact_plan || !is_exact_request {
        return ProviderRetryDispatchAdmission::BindingMismatch;
    }
    debug_assert_eq!(
        dispatch.exact_plan_hash(),
        input.invocation.exact_plan_hash()
    );
    debug_assert_eq!(observed_request_hash, input.invocation.request_hash());
    ProviderRetryDispatchAdmission::Admitted
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedPolicy {
    schema: String,
    policy_id: String,
    max_attempts: u32,
    max_delay_ms: u64,
    delay_schedule_ms: Vec<u64>,
    cancellation_posture: String,
    stream_commit_policy: String,
    retryable_classes: Vec<String>,
    mappings: Vec<GeneratedMapping>,
    policy_hash: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct GeneratedMapping {
    provider_family: String,
    mapping_version: String,
    native_class: String,
    canonical_class: String,
}

impl GeneratedPolicy {
    fn into_runtime_input(self) -> ProviderRetryPolicyInput {
        let input = ProviderRetryPolicyInput {
            schema: self.schema,
            policy_id: self.policy_id,
            max_attempts: self.max_attempts,
            max_delay_ms: self.max_delay_ms,
            delay_schedule_ms: self.delay_schedule_ms,
            cancellation_posture: self.cancellation_posture,
            stream_commit_policy: self.stream_commit_policy,
            retryable_classes: self.retryable_classes,
            mappings: self
                .mappings
                .into_iter()
                .map(GeneratedMapping::into_runtime_input)
                .collect(),
            policy_hash: self.policy_hash,
        };
        debug_assert!(!input.policy_id.is_empty());
        debug_assert!(!input.policy_hash.is_empty());
        input
    }
}

impl GeneratedMapping {
    fn into_runtime_input(self) -> ProviderRetryMappingInput {
        ProviderRetryMappingInput {
            provider_family: self.provider_family,
            mapping_version: self.mapping_version,
            native_class: self.native_class,
            canonical_class: self.canonical_class,
        }
    }
}

fn classify_json_error(error: &serde_json::Error) -> ProviderRetryConfigError {
    let kind = match error.classify() {
        serde_json::error::Category::Syntax => ProviderRetryJsonErrorKind::Syntax,
        serde_json::error::Category::Data | serde_json::error::Category::Io => {
            ProviderRetryJsonErrorKind::Data
        }
        serde_json::error::Category::Eof => ProviderRetryJsonErrorKind::EndOfInput,
    };
    ProviderRetryConfigError::Json(kind)
}

#[cfg(test)]
mod tests;
