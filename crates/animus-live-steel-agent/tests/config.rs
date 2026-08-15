//! Configuration parity and rejection tests.

use animus_live_steel_agent::{LiveSteelAgentConfigError, parse_live_steel_agent_config};
use serde_json::Value;

const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");

#[test]
fn generated_nickel_configuration_revalidates_in_rust() {
    // r[verify agent_harness_runtime.live_steel_agent_shell.configuration]
    let config = parse_live_steel_agent_config(CONFIG).expect("generated configuration");
    assert_eq!(config.application_id(), "animus-live-steel-agent");
    assert_eq!(
        config.credential_reference(),
        "brokered:injected-openai-api-key"
    );
    assert_eq!(config.skill_modules().len(), 1);
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

fn parse_value(value: &Value) -> Result<(), LiveSteelAgentConfigError> {
    let bytes = serde_json::to_vec(value).expect("encode fixture");
    parse_live_steel_agent_config(bytes.as_slice()).map(|_config| ())
}
