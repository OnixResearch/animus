//! Generated configuration parity and rejection tests.

use animus_local_daemon::{LocalDaemonConfigError, parse_local_daemon_config};

const CONFIG: &[u8] = include_bytes!("../../../config/generated/local-daemon-supervisor.json");

#[test]
fn generated_nickel_configuration_revalidates_in_rust() {
    let config = parse_local_daemon_config(CONFIG).expect("generated configuration");
    assert_eq!(config.endpoint_name(), "animus-local-daemon.sock");
    assert_eq!(config.registry_name(), "animus-local-daemon-registry.json");
    assert_eq!(
        config.identity().len(),
        animus_adapter_daemon::BLAKE3_HEX_CHARS
    );
}

#[test]
fn unknown_fields_zero_bounds_ambient_endpoints_and_claim_drift_fail_closed() {
    let text = std::str::from_utf8(CONFIG).expect("utf8 config");
    let cases = [
        (
            text.replacen('{', "{\"unknown\":true,", 1),
            LocalDaemonConfigError::Json,
        ),
        (
            text.replace("\"max_workers\": 128", "\"max_workers\": 0"),
            LocalDaemonConfigError::Bound,
        ),
        (
            text.replace("\"explicit-capability-root\"", "\"ambient-home-directory\""),
            LocalDaemonConfigError::Component,
        ),
        (
            text.replace("\"not-security-sandbox\"", "\"security-sandbox\""),
            LocalDaemonConfigError::NonClaims,
        ),
    ];
    for (candidate, expected) in cases {
        assert_eq!(
            parse_local_daemon_config(candidate.as_bytes()),
            Err(expected)
        );
    }
}
