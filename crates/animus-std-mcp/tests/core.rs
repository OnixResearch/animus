//! Pure configuration, lifecycle, authority, and receipt tests.

use animus_std_mcp::{
    CancellationReceiptState, ConfigError, DiscoveryRequest, LifecycleBinding, LifecycleEvent,
    LifecyclePhase, McpHostReceiptDraft, ReceiptLeakClass, TeardownState, admit_instructions,
    argument_identity, initial_lifecycle, load_generated_config, parse_config, project_receipt,
    reduce_lifecycle, validate_discovery,
};
use serde_json::json;

const CONFIGURATION_ID: &str = "1222222222222222222222222222222222222222222222222222222222222222";
const POLICY_ID: &str = "a222222222222222222222222222222222222222222222222222222222222222";
const CREDENTIAL_ID: &str = "b222222222222222222222222222222222222222222222222222222222222222";
const ATTEMPT_ID: &str = "c222222222222222222222222222222222222222222222222222222222222222";
const DISCOVERY_RESPONSE_ID: u64 = 2;
const OVERBOUND_TIMEOUT_MS: u64 = 300_001;
const NON_CLAIMS: [&str; 9] = [
    "server_trust",
    "server_correctness",
    "tool_safety",
    "sandboxing",
    "network_isolation",
    "exactly_once_effects",
    "mcp_parity",
    "production_readiness",
    "release_eligibility",
];

#[test]
// r[verify agent_harness_runtime.mcp_client_host.configuration.valid]
fn generated_config_and_initialize_cohort_are_exact() {
    let config = load_generated_config();
    assert!(config.is_ok(), "generated config must pass Rust validation");
    let config = match config {
        Ok(value) => value,
        Err(error) => panic!("unexpected config error: {error:?}"),
    };
    assert_eq!(config.server().transport, "stdio");
    assert_eq!(config.server().protocol_version, "2025-06-18");
    assert_eq!(config.server().id, "fixture-stdio");
    assert_eq!(
        config.server().protocol_version,
        animus_std_mcp::MCP_PROTOCOL_VERSION
    );
    assert_ne!(config.server().transport, "streamable-http");
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.configuration.invalid]
fn rust_revalidation_rejects_transport_and_non_claim_drift() {
    let source = include_str!("../../../config/generated/mcp-client-host.json");
    let value = serde_json::from_str::<serde_json::Value>(source);
    assert!(value.is_ok());
    let mut transport_value = match value {
        Ok(value) => value,
        Err(error) => panic!("unexpected JSON error: {error:?}"),
    };
    transport_value["server"]["transport"] = json!("streamable-http");
    let transport_bytes = serde_json::to_vec(&transport_value);
    assert!(transport_bytes.is_ok());
    let transport_result = match transport_bytes {
        Ok(bytes) => parse_config(bytes.as_slice()),
        Err(error) => panic!("unexpected serialization error: {error:?}"),
    };
    assert_eq!(transport_result, Err(ConfigError::UnsafeBehavior));

    let mut non_claim_value = transport_value;
    non_claim_value["server"]["transport"] = json!("stdio");
    non_claim_value["non_claims"] = json!([]);
    let non_claim_bytes = serde_json::to_vec(&non_claim_value);
    assert!(non_claim_bytes.is_ok());
    let non_claim_result = match non_claim_bytes {
        Ok(bytes) => parse_config(bytes.as_slice()),
        Err(error) => panic!("unexpected serialization error: {error:?}"),
    };
    assert_eq!(non_claim_result, Err(ConfigError::MissingNonClaim));

    let overbound_value = serde_json::from_str::<serde_json::Value>(source);
    assert!(overbound_value.is_ok());
    let mut overbound_value = match overbound_value {
        Ok(value) => value,
        Err(error) => panic!("unexpected JSON error: {error:?}"),
    };
    overbound_value["server"]["limits"]["call_timeout_ms"] = json!(OVERBOUND_TIMEOUT_MS);
    let overbound_bytes = serde_json::to_vec(&overbound_value);
    assert!(overbound_bytes.is_ok());
    let overbound_result = match overbound_bytes {
        Ok(bytes) => parse_config(bytes.as_slice()),
        Err(error) => panic!("unexpected serialization error: {error:?}"),
    };
    assert_eq!(overbound_result, Err(ConfigError::InvalidLimit));
}

#[test]
fn lifecycle_rejects_stale_generation_and_uncertain_restart() {
    let current_binding = binding(1);
    let state = initial_lifecycle(current_binding.clone());
    assert!(state.is_ok());
    let state = match state {
        Ok(state) => state,
        Err(error) => panic!("unexpected lifecycle error: {error:?}"),
    };
    let starting = reduce_lifecycle(
        &state,
        LifecycleEvent::BeginStart(current_binding.clone()),
        4,
    );
    assert!(starting.is_ok());
    let drifted_start = reduce_lifecycle(&state, LifecycleEvent::BeginStart(binding(2)), 4);
    assert!(drifted_start.is_err());

    let stopping = match starting {
        Ok(starting) => reduce_lifecycle(
            &starting,
            LifecycleEvent::StopRequested(current_binding.clone()),
            4,
        ),
        Err(error) => panic!("unexpected lifecycle error: {error:?}"),
    };
    assert!(stopping.is_ok());
    let uncertain = match stopping {
        Ok(stopping) => reduce_lifecycle(
            &stopping,
            LifecycleEvent::StopObserved {
                binding: current_binding,
                teardown: TeardownState::Uncertain,
            },
            4,
        ),
        Err(error) => panic!("unexpected lifecycle error: {error:?}"),
    };
    assert!(uncertain.is_ok());
    let uncertain = match uncertain {
        Ok(state) => state,
        Err(error) => panic!("unexpected lifecycle error: {error:?}"),
    };
    assert_eq!(uncertain.phase(), LifecyclePhase::Failed);
    assert!(uncertain.restart_blocked());
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.discovery.invalid]
fn discovery_rejects_schema_drift_within_one_generation() {
    let config = load_generated_config();
    assert!(config.is_ok());
    let config = match config {
        Ok(config) => config,
        Err(error) => panic!("unexpected configuration error: {error:?}"),
    };
    let binding = LifecycleBinding {
        server_id: config.server().id.clone(),
        configuration_id: String::from(config.identity()),
        protocol_id: config.server().protocol_version.clone(),
        policy_id: config.server().policy_identity.clone(),
        credential_snapshot_id: String::from(CREDENTIAL_ID),
        generation: 1,
        process_attempt_id: String::from(ATTEMPT_ID),
    };
    let first_schema = json!({"type": "object"});
    let first = discovery_response(&first_schema);
    let first = validate_discovery(DiscoveryRequest {
        bytes: first.as_slice(),
        expected_response_id: DISCOVERY_RESPONSE_ID,
        config: &config,
        binding: &binding,
        previous: None,
    });
    assert!(first.is_ok());
    let first = match first {
        Ok(catalog) => catalog,
        Err(error) => panic!("unexpected discovery error: {error:?}"),
    };
    let changed_schema = json!({
        "type": "object",
        "properties": {"value": {"type": "string"}}
    });
    let changed = discovery_response(&changed_schema);
    let changed = validate_discovery(DiscoveryRequest {
        bytes: changed.as_slice(),
        expected_response_id: DISCOVERY_RESPONSE_ID,
        config: &config,
        binding: &binding,
        previous: Some(&first),
    });
    assert!(changed.is_err());
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.context.denied]
// r[verify agent_harness_runtime.mcp_client_host.evidence.rejects]
fn context_and_receipt_leaks_fail_closed() {
    let empty = admit_instructions("fixture", 1, String::new(), 64);
    assert!(empty.is_err());
    let oversized = admit_instructions("fixture", 1, "x".repeat(65), 64);
    assert!(oversized.is_err());
    let leak_classes = [
        ReceiptLeakClass::RawFrame,
        ReceiptLeakClass::Arguments,
        ReceiptLeakClass::Result,
        ReceiptLeakClass::Instructions,
        ReceiptLeakClass::Credential,
        ReceiptLeakClass::EnvironmentValue,
        ReceiptLeakClass::PrivatePath,
        ReceiptLeakClass::ProviderData,
        ReceiptLeakClass::UnrestrictedError,
        ReceiptLeakClass::Overclaim,
    ];
    for leak in leak_classes {
        assert!(project_receipt(receipt_draft(leak)).is_err());
    }
}

#[test]
// r[verify agent_harness_runtime.mcp_client_host.evidence.safe]
fn receipt_projection_and_argument_identity_do_not_embed_bodies() {
    let arguments = json!({"secret-body": "must-not-enter-receipt"});
    let identity = argument_identity(&arguments);
    assert!(identity.is_ok());
    let receipt = project_receipt(McpHostReceiptDraft {
        server_id: "fixture",
        configuration_id: CONFIGURATION_ID,
        protocol_id: "2025-06-18",
        generation: 1,
        process_attempt_id: ATTEMPT_ID,
        tool_name: Some("read_text"),
        schema_identity: Some(POLICY_ID),
        capability_labels: &["workspace-read"],
        approval_identity: Some(POLICY_ID),
        call_identity: Some(POLICY_ID),
        output_bound_identity: Some(POLICY_ID),
        output_identity: Some(POLICY_ID),
        cancellation: CancellationReceiptState::Requested,
        lifecycle: LifecyclePhase::Ready,
        teardown: TeardownState::NotStarted,
        status: "ready",
        redaction_identity: POLICY_ID,
        non_claims: &NON_CLAIMS,
        leak: ReceiptLeakClass::None,
    });
    assert!(receipt.is_ok());
    let receipt = match receipt {
        Ok(receipt) => receipt,
        Err(error) => panic!("unexpected receipt error: {error:?}"),
    };
    assert_eq!(receipt.approval_identity(), Some(POLICY_ID));
    assert_eq!(receipt.call_identity(), Some(POLICY_ID));
    assert_eq!(receipt.output_bound_identity(), Some(POLICY_ID));
    assert_eq!(receipt.output_identity(), Some(POLICY_ID));
    assert_eq!(receipt.cancellation(), CancellationReceiptState::Requested);
    assert_eq!(receipt.redaction_identity(), POLICY_ID);
    let encoded = format!("{receipt:?}");
    assert!(!encoded.contains("secret-body"));
    assert!(!encoded.contains("must-not-enter-receipt"));
    assert!(!encoded.contains("credential"));
}

fn discovery_response(schema: &serde_json::Value) -> Vec<u8> {
    let value = json!({
        "jsonrpc": "2.0",
        "id": DISCOVERY_RESPONSE_ID,
        "result": {
            "tools": [{
                "name": "read_text",
                "description": "Bounded fixture tool",
                "inputSchema": schema
            }]
        }
    });
    match serde_json::to_vec(&value) {
        Ok(bytes) => bytes,
        Err(error) => panic!("unexpected discovery serialization error: {error:?}"),
    }
}

fn receipt_draft(leak: ReceiptLeakClass) -> McpHostReceiptDraft<'static> {
    McpHostReceiptDraft {
        server_id: "fixture",
        configuration_id: CONFIGURATION_ID,
        protocol_id: "2025-06-18",
        generation: 1,
        process_attempt_id: ATTEMPT_ID,
        tool_name: None,
        schema_identity: None,
        capability_labels: &[],
        approval_identity: None,
        call_identity: None,
        output_bound_identity: None,
        output_identity: None,
        cancellation: CancellationReceiptState::NotRequested,
        lifecycle: LifecyclePhase::Ready,
        teardown: TeardownState::NotStarted,
        status: "ready",
        redaction_identity: POLICY_ID,
        non_claims: &NON_CLAIMS,
        leak,
    }
}

fn binding(generation: u64) -> LifecycleBinding {
    LifecycleBinding {
        server_id: String::from("fixture"),
        configuration_id: String::from(CONFIGURATION_ID),
        protocol_id: String::from("2025-06-18"),
        policy_id: String::from(POLICY_ID),
        credential_snapshot_id: String::from(CREDENTIAL_ID),
        generation,
        process_attempt_id: String::from(ATTEMPT_ID),
    }
}
