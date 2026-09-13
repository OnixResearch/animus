//! Packaged application admission checks.

const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
const PROMPT: &str = "This prompt must not cause effects without a credential.";

#[test]
fn missing_credential_fails_before_store_creation() {
    let temp = tempfile::tempdir().expect("temporary parent");
    let absent_store = temp.path().join("must-not-be-created");
    let result = animus_live_steel_agent::run_live_application(
        animus_live_steel_agent::LiveApplicationInput {
            config_json: CONFIG,
            store_root: absent_store.as_path(),
            prompt: PROMPT,
            credential: String::new(),
            context: animus_live_steel_agent::ExplicitContextMap::default(),
        },
    );

    assert_eq!(
        result.map(|_output| ()),
        Err(animus_live_steel_agent::LiveApplicationError::Provider)
    );
    assert!(!absent_store.exists());
}

#[test]
fn malformed_configuration_fails_before_credential_or_store_use() {
    let temp = tempfile::tempdir().expect("temporary parent");
    let absent_store = temp.path().join("must-not-be-created");
    let result = animus_live_steel_agent::run_live_application(
        animus_live_steel_agent::LiveApplicationInput {
            config_json: b"{}",
            store_root: absent_store.as_path(),
            prompt: PROMPT,
            credential: String::from("not-used"),
            context: animus_live_steel_agent::ExplicitContextMap::default(),
        },
    );

    assert_eq!(
        result.map(|_output| ()),
        Err(animus_live_steel_agent::LiveApplicationError::Configuration)
    );
    assert!(!absent_store.exists());
}
