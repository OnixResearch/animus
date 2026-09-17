//! Configuration parity and rejection tests.

use animus_live_steel_agent::{LiveSteelAgentConfigError, parse_live_steel_agent_config};
use serde_json::Value;

const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
const LOCAL_DSPARK_PROVIDER: &str = "local-dspark";
const OPENAI_PROVIDER: &str = "openai";
const OPENROUTER_PROVIDER: &str = "openrouter";
const LOCAL_DSPARK_CREDENTIAL_REFERENCE: &str = "brokered:injected-local-dspark-api-key";
const OPENAI_CREDENTIAL_REFERENCE: &str = "brokered:injected-openai-api-key";
const OPENROUTER_CREDENTIAL_REFERENCE: &str = "brokered:injected-openrouter-api-key";
const LOCAL_DSPARK_MODEL: &str = "deepseek-v4-flash-0731-ablit-100";
const OPENROUTER_MODEL: &str = "deepseek/deepseek-v4-flash-0731";

#[test]
fn generated_nickel_configuration_revalidates_in_rust() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.configuration]
    let config = parse_live_steel_agent_config(CONFIG).expect("generated configuration");
    assert_eq!(config.application_id(), "animus-live-steel-agent");
    assert_eq!(config.provider_id(), OPENAI_PROVIDER);
    assert_eq!(config.credential_reference(), OPENAI_CREDENTIAL_REFERENCE);
    assert_eq!(config.skill_modules().len(), 1);
    let host = config.provider_host_config().expect("OpenAI host config");
    assert_eq!(
        host.endpoint_identity(),
        animus_std_openai::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY
    );
    assert_eq!(
        host.endpoint_url(),
        animus_std_openai::OFFICIAL_OPENAI_RESPONSES_URL
    );
}

#[test]
fn exact_openrouter_configuration_selects_the_matching_host_and_credential() {
    // r[verify agent_harness_runtime.live_steel_agent_provider_cohort.valid]
    let value = openrouter_value();
    let bytes = serde_json::to_vec(&value).expect("encode OpenRouter fixture");
    let config = parse_live_steel_agent_config(bytes.as_slice()).expect("OpenRouter config");

    assert_eq!(config.provider_id(), OPENROUTER_PROVIDER);
    assert_eq!(config.model_id(), OPENROUTER_MODEL);
    assert_eq!(
        config.credential_reference(),
        OPENROUTER_CREDENTIAL_REFERENCE
    );
    assert_eq!(
        config.selected_model().provider_id().as_str(),
        OPENROUTER_PROVIDER
    );
    let host = config
        .provider_host_config()
        .expect("OpenRouter host config");
    assert_eq!(
        host.endpoint_identity(),
        animus_std_openai::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY
    );
    assert_eq!(
        host.endpoint_url(),
        animus_std_openai::OPENROUTER_RESPONSES_URL
    );
}

#[test]
fn exact_local_dspark_configuration_admits_the_cohort_effort_set() {
    // r[verify agent_harness_runtime.local_dspark_responses.endpoint.pass]
    // r[verify agent_harness_runtime.local_dspark_responses.reasoning.pass]
    let value = local_dspark_value();
    let bytes = serde_json::to_vec(&value).expect("encode local-dspark fixture");
    let config = parse_live_steel_agent_config(bytes.as_slice()).expect("local-dspark config");

    assert_eq!(config.provider_id(), LOCAL_DSPARK_PROVIDER);
    assert_eq!(config.model_id(), LOCAL_DSPARK_MODEL);
    assert_eq!(
        config.credential_reference(),
        LOCAL_DSPARK_CREDENTIAL_REFERENCE
    );
    let host = config.provider_host_config().expect("local host config");
    assert_eq!(
        host.endpoint_identity(),
        animus_std_openai::LOCAL_DSPARK_CHAT_COMPLETIONS_ENDPOINT_IDENTITY
    );
    assert_eq!(
        host.endpoint_url(),
        animus_std_openai::LOCAL_DSPARK_CHAT_COMPLETIONS_URL
    );

    // Every admitted cohort effort parses; the wire effort follows the config.
    for admitted in ["none", "low", "high"] {
        let mut variant = local_dspark_value();
        variant["provider"]["reasoning_effort"] = Value::from(admitted);
        let parsed = parse_value(&local_dspark_with_effort(admitted));
        assert!(parsed.is_ok(), "effort {admitted} must be admitted");
    }

    // A label outside the exact cohort set fails closed.
    for rejected in ["medium", "max"] {
        assert_eq!(
            parse_value(&local_dspark_with_effort(rejected)),
            Err(LiveSteelAgentConfigError::ProviderMismatch)
        );
    }
}

fn local_dspark_with_effort(effort: &str) -> Value {
    let mut value = local_dspark_value();
    value["provider"]["reasoning_effort"] = Value::from(effort);
    value
}

#[test]
fn exact_host_tool_operation_is_optional_and_unknown_operations_fail_closed() {
    // r[verify agent_harness_runtime.pueue_host.live_shell_tool_route]
    let mut tool: Value = serde_json::from_slice(CONFIG).expect("JSON");
    tool["admitted_operations"]
        .as_array_mut()
        .expect("operation array")
        .push(Value::from("file_read"));
    let bytes = serde_json::to_vec(&tool).expect("file read operation fixture");
    let config = parse_live_steel_agent_config(bytes.as_slice()).expect("file read config");
    assert!(
        config
            .admitted_operations()
            .iter()
            .any(|value| value == "file_read")
    );

    let mut unknown: Value = serde_json::from_slice(CONFIG).expect("JSON");
    unknown["admitted_operations"]
        .as_array_mut()
        .expect("operation array")
        .push(Value::from("kernel_clear"));
    assert_eq!(
        parse_value(&unknown),
        Err(LiveSteelAgentConfigError::InvalidCollection)
    );
}

#[test]
fn missing_component_unknown_field_and_zero_bound_fail_before_effects() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.configuration.missing]
    let mut missing: Value = serde_json::from_slice(CONFIG).expect("JSON");
    missing
        .as_object_mut()
        .expect("object")
        .remove("components");
    assert_eq!(
        parse_value(&missing),
        Err(LiveSteelAgentConfigError::Malformed)
    );

    let mut unknown: Value = serde_json::from_slice(CONFIG).expect("JSON");
    unknown
        .as_object_mut()
        .expect("object")
        .insert(String::from("ambient_provider"), Value::Bool(true));
    assert_eq!(
        parse_value(&unknown),
        Err(LiveSteelAgentConfigError::Malformed)
    );

    let mut zero: Value = serde_json::from_slice(CONFIG).expect("JSON");
    zero["bounds"]["provider_timeout_ms"] = Value::from(0);
    assert_eq!(
        parse_value(&zero),
        Err(LiveSteelAgentConfigError::InvalidBound)
    );
}

#[test]
fn crossed_unknown_and_arbitrary_provider_tuples_fail_closed() {
    // r[verify agent_harness_runtime.live_steel_agent_provider_cohort.crossed]
    // r[verify agent_harness_runtime.live_steel_agent_provider_cohort.closed.unknown]
    let mut crossed_endpoint = openrouter_value();
    crossed_endpoint["provider"]["endpoint_identity"] =
        Value::from(animus_std_openai::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY);
    crossed_endpoint["provider"]["endpoint_url"] =
        Value::from(animus_std_openai::OFFICIAL_OPENAI_RESPONSES_URL);
    assert_eq!(
        parse_value(&crossed_endpoint),
        Err(LiveSteelAgentConfigError::ProviderMismatch)
    );

    let mut crossed_credential = openrouter_value();
    crossed_credential["provider"]["credential_reference"] =
        Value::from(OPENAI_CREDENTIAL_REFERENCE);
    assert_eq!(
        parse_value(&crossed_credential),
        Err(LiveSteelAgentConfigError::ProviderMismatch)
    );

    let mut unknown = openrouter_value();
    unknown["provider"]["provider_id"] = Value::from("unknown-provider");
    assert_eq!(
        parse_value(&unknown),
        Err(LiveSteelAgentConfigError::ProviderMismatch)
    );

    let mut arbitrary = openrouter_value();
    arbitrary["provider"]["endpoint_url"] = Value::from("https://example.invalid/v1/responses");
    assert_eq!(
        parse_value(&arbitrary),
        Err(LiveSteelAgentConfigError::ProviderMismatch)
    );

    // r[verify agent_harness_runtime.live_steel_agent_provider_cohort.closed.no_fallback]
    let mut fallback = openrouter_value();
    fallback["provider"]["fallback_provider"] = Value::from(OPENAI_PROVIDER);
    assert_eq!(
        parse_value(&fallback),
        Err(LiveSteelAgentConfigError::Malformed)
    );
}

#[test]
fn endpoint_credential_and_evidence_drift_fail_closed() {
    let mut endpoint: Value = serde_json::from_slice(CONFIG).expect("JSON");
    endpoint["provider"]["endpoint_url"] = Value::from("https://example.invalid");
    assert_eq!(
        parse_value(&endpoint),
        Err(LiveSteelAgentConfigError::ProviderMismatch)
    );

    let mut credential: Value = serde_json::from_slice(CONFIG).expect("JSON");
    credential["provider"]["credential_reference"] = Value::from("env:OPENAI_API_KEY");
    assert_eq!(
        parse_value(&credential),
        Err(LiveSteelAgentConfigError::ProviderMismatch)
    );

    let mut evidence: Value = serde_json::from_slice(CONFIG).expect("JSON");
    evidence["evidence"]["non_claims"] = Value::Array(Vec::new());
    assert_eq!(
        parse_value(&evidence),
        Err(LiveSteelAgentConfigError::EvidenceMismatch)
    );
}

fn local_dspark_value() -> Value {
    let mut value: Value = serde_json::from_slice(CONFIG).expect("JSON");
    value["provider"]["provider_id"] = Value::from(LOCAL_DSPARK_PROVIDER);
    value["provider"]["model_id"] = Value::from(LOCAL_DSPARK_MODEL);
    value["provider"]["endpoint_identity"] =
        Value::from(animus_std_openai::LOCAL_DSPARK_CHAT_COMPLETIONS_ENDPOINT_IDENTITY);
    value["provider"]["endpoint_url"] =
        Value::from(animus_std_openai::LOCAL_DSPARK_CHAT_COMPLETIONS_URL);
    value["provider"]["credential_reference"] = Value::from(LOCAL_DSPARK_CREDENTIAL_REFERENCE);
    value["provider"]["reasoning_effort"] = Value::from("none");
    value
}

fn openrouter_value() -> Value {
    let mut value: Value = serde_json::from_slice(CONFIG).expect("JSON");
    value["provider"]["provider_id"] = Value::from(OPENROUTER_PROVIDER);
    value["provider"]["model_id"] = Value::from(OPENROUTER_MODEL);
    value["provider"]["endpoint_identity"] =
        Value::from(animus_std_openai::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY);
    value["provider"]["endpoint_url"] = Value::from(animus_std_openai::OPENROUTER_RESPONSES_URL);
    value["provider"]["credential_reference"] = Value::from(OPENROUTER_CREDENTIAL_REFERENCE);
    value
}

fn parse_value(value: &Value) -> Result<(), LiveSteelAgentConfigError> {
    let bytes = serde_json::to_vec(value).expect("encode fixture");
    parse_live_steel_agent_config(bytes.as_slice()).map(|_config| ())
}
