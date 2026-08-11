//! Concrete process, transport, credential, cancellation, and teardown tests.

use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use animus_runtime::request_context::{
    ContributionLifetime, ContributionPosture, ContributionSource, ContributionTrust,
};
use animus_std_mcp::{
    CallAuthorityObservation, CallBinding, CallPlan, CallRequestView, CredentialService,
    CredentialServiceError, GateDecision, HostErrorKind, LaunchAuthority, McpClientHost,
    ResolvedCredentials, ServerPosture, TeardownObservation, load_generated_config, parse_config,
    plan_call,
};
use serde_json::{Value, json};

const CREDENTIAL_A: &str = "d3e2fa0ef86858e50ee0a179ba69314f69c02fc139239a9ca01a088143042284";
const CREDENTIAL_B: &str = "e3e2fa0ef86858e50ee0a179ba69314f69c02fc139239a9ca01a088143042284";
const MODEL_PLAN: &str = "34ba2b5157ede27f7fe12c6752de32afd74b1b9ab14cb8de33a9bd1789d2c1af";
const AUTHORITY_SEAL: &str = "7683c183a012c8adfbf5660a53bfda65f7e4dd8928081029348db395e7816bad";
const APPROVAL: &str = "66f0f3c4b7d9522e11c2151506e827f070eb5b75caaf2dce317cd49d1600c6db";
const CAPABILITY_GRANT: &str = "c83cd6142079f5f64ee6ad3f46c725603985e5d618f73e0a28a72068b26a007f";
const MUTATION_PLAN: &str = "27647fce9a45696efa518002af89242b00371f1e2bf88c91eee830efaa62288d";
const OUTPUT_BOUND: &str = "b5ac4c2bfef78f2f8f0b3f572371d4304b9ee26c3537466060aaaf12460a3084";
const TEST_GENERATION: u64 = 7;
const TEST_TIMEOUT_MS: u64 = 40;
const CANCELLATION_DELAY_MS: u64 = 10;

#[derive(Clone, Copy)]
enum AuthorityDenial {
    MissingApproval,
    DeniedCapability,
    StaleModelPlan,
    StaleGeneration,
    SchemaMismatch,
    MutationConflict,
    ChildGrantExpansion,
    Cancellation,
    OutputBoundary,
}

struct EmptyCredentials;

impl CredentialService for EmptyCredentials {
    fn resolve(
        &self,
        _credential_refs: &[String],
        _environment_keys: &[String],
    ) -> Result<ResolvedCredentials, CredentialServiceError> {
        Ok(ResolvedCredentials {
            snapshot_identity: String::from(CREDENTIAL_A),
            environment: Vec::new(),
        })
    }
}

struct UnavailableCredentials;

impl CredentialService for UnavailableCredentials {
    fn resolve(
        &self,
        _credential_refs: &[String],
        _environment_keys: &[String],
    ) -> Result<ResolvedCredentials, CredentialServiceError> {
        Err(CredentialServiceError::Unavailable)
    }
}

struct RotatingCredentials {
    calls: AtomicUsize,
}

impl RotatingCredentials {
    const fn new() -> Self {
        Self {
            calls: AtomicUsize::new(0),
        }
    }
}

impl CredentialService for RotatingCredentials {
    fn resolve(
        &self,
        _credential_refs: &[String],
        _environment_keys: &[String],
    ) -> Result<ResolvedCredentials, CredentialServiceError> {
        let call = self.calls.fetch_add(1, Ordering::AcqRel);
        let identity = if call == 0 {
            CREDENTIAL_A
        } else {
            CREDENTIAL_B
        };
        Ok(ResolvedCredentials {
            snapshot_identity: String::from(identity),
            environment: Vec::new(),
        })
    }
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.boundary.selected]
// r[verify agent_harness_runtime.mcp_client_host.discovery.valid]
// r[verify agent_harness_runtime.mcp_client_host.authority.allowed]
// r[verify agent_harness_runtime.mcp_client_host.context.request_local]
// r[verify agent_harness_runtime.mcp_client_host.verification.complete]
fn concrete_host_initializes_discovers_calls_and_stops() {
    let config = config_for("normal", None, None);
    assert!(config.is_ok());
    let config = take_config(config);
    let host = McpClientHost::connect(
        config,
        TEST_GENERATION,
        launch_authority(),
        Box::new(EmptyCredentials),
    );
    assert!(host.is_ok());
    let mut host = match host {
        Ok(host) => host,
        Err(error) => panic!("unexpected host error: {error:?}"),
    };
    assert_eq!(host.catalog().tools().len(), 1);
    assert!(host.catalog().tool("read_text").is_some());
    assert!(host.instructions().is_some());
    let Some(instructions) = host.instructions() else {
        panic!("fixture instructions must be admitted");
    };
    assert_eq!(
        instructions.contribution().source(),
        ContributionSource::Mcp
    );
    assert_eq!(
        instructions.contribution().trust(),
        ContributionTrust::Untrusted
    );
    assert_eq!(
        instructions.contribution().posture(),
        ContributionPosture::Advisory
    );
    assert_eq!(
        instructions.contribution().lifetime(),
        ContributionLifetime::CurrentRequest
    );
    let arguments = json!({"value": "safe"});
    let plan = call_plan(&host, &arguments);
    assert!(plan.is_ok());
    let plan = match plan {
        Ok(plan) => plan,
        Err(error) => panic!("unexpected plan error: {error}"),
    };
    let cancellation = AtomicBool::new(false);
    let output = host.call(&plan, &arguments, &cancellation);
    assert!(output.is_ok());
    let output = match output {
        Ok(output) => output,
        Err(error) => panic!("unexpected call error: {error:?}"),
    };
    assert!(!output.is_error());
    assert_eq!(output.value()["structuredContent"]["accepted"], true);
    assert_eq!(output.identity().len(), 64);
    let teardown = host.stop();
    assert!(matches!(
        teardown,
        TeardownObservation::Clean | TeardownObservation::Forced
    ));
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.verification.incomplete]
fn discovery_failures_are_bounded_and_never_ready() {
    for mode in ["partial-catalog", "duplicate-tools", "too-many-tools"] {
        let config = take_config(config_for(mode, None, None));
        let result = McpClientHost::connect(
            config,
            TEST_GENERATION,
            launch_authority(),
            Box::new(EmptyCredentials),
        );
        assert!(result.is_err(), "mode {mode} must fail");
        let error = match result {
            Ok(_host) => panic!("mode {mode} unexpectedly reached readiness"),
            Err(error) => error,
        };
        assert_eq!(error.kind, HostErrorKind::Discovery);
        assert!(matches!(
            error.teardown,
            TeardownObservation::Clean | TeardownObservation::Forced
        ));
    }
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.lifecycle.optional]
fn malformed_initialization_preserves_optional_unavailability() {
    let config = take_config(config_for(
        "malformed-initialize",
        Some(ServerPosture::Optional),
        None,
    ));
    let result = McpClientHost::connect_visible(
        config,
        TEST_GENERATION,
        launch_authority(),
        Box::new(EmptyCredentials),
    );
    assert!(result.is_err());
    let unavailable = match result {
        Ok(_host) => panic!("malformed server unexpectedly reached readiness"),
        Err(unavailable) => unavailable,
    };
    assert_eq!(unavailable.posture, ServerPosture::Optional);
    assert_eq!(unavailable.error.kind, HostErrorKind::Protocol);
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.lifecycle.required]
fn required_protocol_and_frame_failures_block_readiness() {
    for mode in ["wrong-version", "oversized-discovery", "early-exit"] {
        let config = take_config(config_for(mode, None, None));
        let result = McpClientHost::connect_visible(
            config,
            TEST_GENERATION,
            launch_authority(),
            Box::new(EmptyCredentials),
        );
        assert!(result.is_err(), "mode {mode} must fail");
        let unavailable = match result {
            Ok(_host) => panic!("mode {mode} unexpectedly reached readiness"),
            Err(unavailable) => unavailable,
        };
        assert_eq!(unavailable.posture, ServerPosture::Required);
        let expected = if mode == "early-exit" {
            HostErrorKind::Process
        } else {
            HostErrorKind::Protocol
        };
        assert_eq!(unavailable.error.kind, expected);
    }
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.boundary.ambient]
fn launch_and_credential_failures_have_no_process_teardown_claim() {
    let config = take_config(config_for("normal", None, None));
    let mut launch = launch_authority();
    launch.executable_identity = String::from(CREDENTIAL_B);
    let launch_result =
        McpClientHost::connect(config, TEST_GENERATION, launch, Box::new(EmptyCredentials));
    assert!(launch_result.is_err());
    let launch_error = match launch_result {
        Ok(_host) => panic!("stale launch authority unexpectedly started"),
        Err(error) => error,
    };
    assert_eq!(launch_error.kind, HostErrorKind::LaunchAuthority);
    assert_eq!(launch_error.teardown, TeardownObservation::NotStarted);

    let config = take_config(config_for("normal", None, None));
    let credential_result = McpClientHost::connect(
        config,
        TEST_GENERATION,
        launch_authority(),
        Box::new(UnavailableCredentials),
    );
    assert!(credential_result.is_err());
    let credential_error = match credential_result {
        Ok(_host) => panic!("missing credentials unexpectedly started"),
        Err(error) => error,
    };
    assert_eq!(credential_error.kind, HostErrorKind::Credential);
    assert_eq!(credential_error.teardown, TeardownObservation::NotStarted);
}

#[test]
fn malformed_call_response_stops_before_model_visibility() {
    let config = take_config(config_for("malformed-call", None, None));
    let host = McpClientHost::connect(
        config,
        TEST_GENERATION,
        launch_authority(),
        Box::new(EmptyCredentials),
    );
    assert!(host.is_ok());
    let mut host = match host {
        Ok(host) => host,
        Err(error) => panic!("unexpected host error: {error:?}"),
    };
    let arguments = json!({"value": "safe"});
    let plan = call_plan(&host, &arguments);
    assert!(plan.is_ok());
    let plan = match plan {
        Ok(plan) => plan,
        Err(error) => panic!("unexpected plan error: {error}"),
    };
    let result = host.call(&plan, &arguments, &AtomicBool::new(false));
    assert!(result.is_err());
    let error = match result {
        Ok(_output) => panic!("malformed result unexpectedly became visible"),
        Err(error) => error,
    };
    assert_eq!(error.kind, HostErrorKind::Protocol);
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.authority.denied]
fn stale_or_expanded_authority_never_reaches_transport() {
    let config = take_config(config_for("normal", None, None));
    let host = McpClientHost::connect(
        config,
        TEST_GENERATION,
        launch_authority(),
        Box::new(EmptyCredentials),
    );
    assert!(host.is_ok());
    let mut host = match host {
        Ok(host) => host,
        Err(error) => panic!("unexpected host error: {error:?}"),
    };
    let arguments = json!({"value": "safe"});
    let cases = [
        AuthorityDenial::MissingApproval,
        AuthorityDenial::DeniedCapability,
        AuthorityDenial::StaleModelPlan,
        AuthorityDenial::StaleGeneration,
        AuthorityDenial::SchemaMismatch,
        AuthorityDenial::MutationConflict,
        AuthorityDenial::ChildGrantExpansion,
        AuthorityDenial::Cancellation,
        AuthorityDenial::OutputBoundary,
    ];
    for case in cases {
        let plan = call_plan_with(&host, &arguments, |binding, observation| match case {
            AuthorityDenial::MissingApproval => observation.approval = GateDecision::Denied,
            AuthorityDenial::DeniedCapability => observation.capability = GateDecision::Denied,
            AuthorityDenial::StaleModelPlan => {
                observation.model_plan_identity = String::from(CREDENTIAL_B);
            }
            AuthorityDenial::StaleGeneration => {
                observation.server_generation = TEST_GENERATION.saturating_add(1);
            }
            AuthorityDenial::SchemaMismatch => {
                observation.schema_identity = String::from(CREDENTIAL_B);
            }
            AuthorityDenial::MutationConflict => observation.mutation = GateDecision::Denied,
            AuthorityDenial::ChildGrantExpansion => binding.caller_capabilities.clear(),
            AuthorityDenial::Cancellation => observation.cancellation = GateDecision::Denied,
            AuthorityDenial::OutputBoundary => observation.output = GateDecision::Denied,
        });
        assert!(plan.is_err());
    }
    assert_eq!(host.lifecycle().completed_call_count(), 0);
    let teardown = host.stop();
    assert!(matches!(
        teardown,
        TeardownObservation::Clean | TeardownObservation::Forced
    ));
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.lifecycle.credential_drift]
fn credential_drift_fences_and_stops_the_generation() {
    let config = take_config(config_for("normal", None, None));
    let host = McpClientHost::connect(
        config,
        TEST_GENERATION,
        launch_authority(),
        Box::new(RotatingCredentials::new()),
    );
    assert!(host.is_ok());
    let mut host = match host {
        Ok(host) => host,
        Err(error) => panic!("unexpected host error: {error:?}"),
    };
    let arguments = json!({"value": "safe"});
    let plan = call_plan(&host, &arguments);
    assert!(plan.is_ok());
    let plan = match plan {
        Ok(plan) => plan,
        Err(error) => panic!("unexpected plan error: {error}"),
    };
    let result = host.call(&plan, &arguments, &AtomicBool::new(false));
    assert!(result.is_err());
    let error = match result {
        Ok(_output) => panic!("credential drift unexpectedly dispatched"),
        Err(error) => error,
    };
    assert_eq!(error.kind, HostErrorKind::CredentialDrift);
    assert!(!host.lifecycle().restart_blocked());
}

#[test]
fn timeout_cancels_and_tears_down_before_restart() {
    let config = take_config(config_for("delay-call", None, Some(TEST_TIMEOUT_MS)));
    let host = McpClientHost::connect(
        config,
        TEST_GENERATION,
        launch_authority(),
        Box::new(EmptyCredentials),
    );
    assert!(host.is_ok());
    let mut host = match host {
        Ok(host) => host,
        Err(error) => panic!("unexpected host error: {error:?}"),
    };
    let arguments = json!({"value": "safe"});
    let plan = call_plan(&host, &arguments);
    assert!(plan.is_ok());
    let plan = match plan {
        Ok(plan) => plan,
        Err(error) => panic!("unexpected plan error: {error}"),
    };
    let result = host.call(&plan, &arguments, &AtomicBool::new(false));
    assert!(result.is_err());
    let error = match result {
        Ok(_output) => panic!("delayed call unexpectedly completed"),
        Err(error) => error,
    };
    assert_eq!(error.kind, HostErrorKind::Cancelled);
    assert!(matches!(
        error.teardown,
        TeardownObservation::Clean | TeardownObservation::Forced
    ));
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.lifecycle.cancelled]
fn explicit_cancellation_stops_the_active_generation() {
    let config = take_config(config_for("delay-call", None, None));
    let host = McpClientHost::connect(
        config,
        TEST_GENERATION,
        launch_authority(),
        Box::new(EmptyCredentials),
    );
    assert!(host.is_ok());
    let mut host = match host {
        Ok(host) => host,
        Err(error) => panic!("unexpected host error: {error:?}"),
    };
    let arguments = json!({"value": "safe"});
    let plan = call_plan(&host, &arguments);
    assert!(plan.is_ok());
    let plan = match plan {
        Ok(plan) => plan,
        Err(error) => panic!("unexpected plan error: {error}"),
    };
    let pre_cancelled = host.call(&plan, &arguments, &AtomicBool::new(true));
    assert!(pre_cancelled.is_err());
    let pre_cancelled = match pre_cancelled {
        Ok(_output) => panic!("pre-cancelled call unexpectedly dispatched"),
        Err(error) => error,
    };
    assert_eq!(pre_cancelled.kind, HostErrorKind::Cancelled);
    assert_eq!(pre_cancelled.teardown, TeardownObservation::Retained);
    assert_eq!(
        host.lifecycle().phase(),
        animus_std_mcp::LifecyclePhase::Ready
    );

    let cancellation = Arc::new(AtomicBool::new(false));
    let cancellation_writer = Arc::clone(&cancellation);
    let writer = thread::spawn(move || {
        thread::sleep(Duration::from_millis(CANCELLATION_DELAY_MS));
        cancellation_writer.store(true, Ordering::Release);
    });
    let result = host.call(&plan, &arguments, cancellation.as_ref());
    assert!(writer.join().is_ok());
    assert!(result.is_err());
    let error = match result {
        Ok(_output) => panic!("cancelled call unexpectedly completed"),
        Err(error) => error,
    };
    assert_eq!(error.kind, HostErrorKind::Cancelled);
    assert!(matches!(
        error.teardown,
        TeardownObservation::Clean | TeardownObservation::Forced
    ));
}

#[cfg(unix)]
#[test]
fn unix_process_group_teardown_covers_descendants() {
    let config = take_config(config_for("descendant", None, None));
    let host = McpClientHost::connect(
        config,
        TEST_GENERATION,
        launch_authority(),
        Box::new(EmptyCredentials),
    );
    assert!(host.is_ok());
    let mut host = match host {
        Ok(host) => host,
        Err(error) => panic!("unexpected host error: {error:?}"),
    };
    assert_eq!(
        host.process_scope(),
        animus_std_mcp::ProcessScope::ProcessGroup
    );
    let observation = host.stop();
    assert!(matches!(
        observation,
        TeardownObservation::Clean | TeardownObservation::Forced
    ));
}

fn config_for(
    mode: &str,
    posture: Option<ServerPosture>,
    call_timeout_ms: Option<u64>,
) -> Result<animus_std_mcp::McpHostConfig, String> {
    let generated = load_generated_config().map_err(|error| format!("{error:?}"))?;
    let source = include_str!("../../../config/generated/mcp-client-host.json");
    let mut value = serde_json::from_str::<Value>(source).map_err(|error| error.to_string())?;
    value["server"]["arguments"] = json!(["serve", mode]);
    value["server"]["executable_identity"] =
        Value::String(generated.server().executable_identity.clone());
    if let Some(posture) = posture {
        value["server"]["posture"] = Value::String(match posture {
            ServerPosture::Required => String::from("required"),
            ServerPosture::Optional => String::from("optional"),
        });
    }
    if let Some(timeout_ms) = call_timeout_ms {
        value["server"]["limits"]["call_timeout_ms"] = json!(timeout_ms);
    }
    let bytes = serde_json::to_vec(&value).map_err(|error| error.to_string())?;
    parse_config(bytes.as_slice()).map_err(|error| format!("{error:?}"))
}

fn take_config(
    result: Result<animus_std_mcp::McpHostConfig, String>,
) -> animus_std_mcp::McpHostConfig {
    match result {
        Ok(config) => config,
        Err(error) => panic!("unexpected configuration error: {error}"),
    }
}

fn launch_authority() -> LaunchAuthority {
    let root = std::env::current_dir();
    assert!(root.is_ok());
    let working_directory = match root {
        Ok(root) => root,
        Err(error) => panic!("unexpected current-directory error: {error:?}"),
    };
    let config = load_generated_config();
    assert!(config.is_ok());
    let config = match config {
        Ok(config) => config,
        Err(error) => panic!("unexpected generated configuration error: {error:?}"),
    };
    LaunchAuthority {
        executable_path: PathBuf::from(env!("CARGO_BIN_EXE_animus-mcp-fixture-server")),
        executable_identity: config.server().executable_identity.clone(),
        working_directory,
        working_root_identity: config.server().working_root_identity.clone(),
    }
}

fn call_plan(host: &McpClientHost, arguments: &Value) -> Result<CallPlan, String> {
    call_plan_with(host, arguments, |_binding, _observation| {})
}

fn call_plan_with(
    host: &McpClientHost,
    arguments: &Value,
    adjust: impl FnOnce(&mut CallBinding, &mut CallAuthorityObservation),
) -> Result<CallPlan, String> {
    let tool = host
        .catalog()
        .tool("read_text")
        .ok_or_else(|| String::from("fixture tool missing"))?;
    let schema_identity = String::from(tool.mapping().schema_identity());
    let capabilities = tool.mapping().capability_labels().to_vec();
    let mut binding = CallBinding {
        server_id: host.config().server().id.clone(),
        server_generation: TEST_GENERATION,
        tool_name: String::from("read_text"),
        schema_identity: schema_identity.clone(),
        model_plan_identity: String::from(MODEL_PLAN),
        authority_seal_identity: String::from(AUTHORITY_SEAL),
        approval_identity: String::from(APPROVAL),
        capability_grant_identity: String::from(CAPABILITY_GRANT),
        mutation_plan_identity: String::from(MUTATION_PLAN),
        output_bound_identity: String::from(OUTPUT_BOUND),
        parent_capabilities: capabilities.clone(),
        caller_capabilities: capabilities,
    };
    let mut observation = CallAuthorityObservation {
        server_generation: TEST_GENERATION,
        schema_identity,
        model_plan_identity: String::from(MODEL_PLAN),
        authority_seal_identity: String::from(AUTHORITY_SEAL),
        approval_identity: String::from(APPROVAL),
        capability_grant_identity: String::from(CAPABILITY_GRANT),
        mutation_plan_identity: String::from(MUTATION_PLAN),
        output_bound_identity: String::from(OUTPUT_BOUND),
        arguments: GateDecision::Allowed,
        executor: GateDecision::Allowed,
        capability: GateDecision::Allowed,
        approval: GateDecision::Allowed,
        mutation: GateDecision::Allowed,
        output: GateDecision::Allowed,
        cancellation: GateDecision::Allowed,
    };
    adjust(&mut binding, &mut observation);
    plan_call(CallRequestView {
        tool,
        arguments,
        binding: &binding,
        observation: &observation,
        completed_call_count: host.lifecycle().completed_call_count(),
        limits: host.config().server().limits,
    })
    .map_err(|error| format!("{error:?}"))
}
