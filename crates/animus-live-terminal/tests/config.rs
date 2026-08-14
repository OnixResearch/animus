//! Configuration parity and fail-closed tests.

use animus_live_terminal::{ConnectionMode, TerminalConfigError, parse_terminal_config};

const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-terminal-agent-host.json");
const HASH_LENGTH: usize = 64;
const EXPECTED_EVENT_BATCH: u32 = 64;
const EXPECTED_PROMPT_BYTES: u32 = 4_096;
const WIDENED_WIDTH: u16 = 5_000;
const MAX_LABEL_BYTES: usize = 256;

#[test]
fn generated_nickel_configuration_revalidates_in_rust() {
    let config = parse_terminal_config(CONFIG).expect("generated terminal config");
    assert_eq!(config.connection_mode(), ConnectionMode::Daemon);
    assert_eq!(config.endpoint_name(), "animus-local-daemon.sock");
    assert_eq!(config.max_event_batch(), EXPECTED_EVENT_BATCH);
    assert_eq!(config.max_prompt_bytes(), EXPECTED_PROMPT_BYTES);
    assert_eq!(config.identity().len(), HASH_LENGTH);
}

#[test]
fn direct_mode_is_explicit_and_unknown_zero_or_widened_values_fail_closed() {
    let mut direct: serde_json::Value = serde_json::from_slice(CONFIG).expect("config JSON");
    direct["connection_mode"] = serde_json::Value::String(String::from("direct"));
    let direct = serde_json::to_vec(&direct).expect("direct fixture");
    assert_eq!(
        parse_terminal_config(direct.as_slice())
            .expect("explicit direct mode")
            .connection_mode(),
        ConnectionMode::Direct
    );

    let mut unknown: serde_json::Value = serde_json::from_slice(CONFIG).expect("config JSON");
    unknown["implicit_daemon_discovery"] = serde_json::Value::Bool(true);
    assert_eq!(
        parse_terminal_config(
            serde_json::to_vec(&unknown)
                .expect("unknown fixture")
                .as_slice()
        ),
        Err(TerminalConfigError::Json)
    );

    for (field, value, error) in [
        (
            "max_event_batch",
            serde_json::json!(0),
            TerminalConfigError::Bound,
        ),
        (
            "max_width",
            serde_json::json!(WIDENED_WIDTH),
            TerminalConfigError::Bound,
        ),
        (
            "raw_mode",
            serde_json::json!(false),
            TerminalConfigError::Contract,
        ),
        (
            "endpoint_name",
            serde_json::json!("../ambient.sock"),
            TerminalConfigError::InvalidValue,
        ),
        (
            "root_id",
            serde_json::json!("a".repeat(MAX_LABEL_BYTES + 1)),
            TerminalConfigError::InvalidValue,
        ),
    ] {
        let mut invalid: serde_json::Value = serde_json::from_slice(CONFIG).expect("config JSON");
        invalid[field] = value;
        assert_eq!(
            parse_terminal_config(
                serde_json::to_vec(&invalid)
                    .expect("invalid fixture")
                    .as_slice()
            ),
            Err(error),
            "field {field} must fail closed"
        );
    }
}
