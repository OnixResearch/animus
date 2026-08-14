//! Metadata-only daemon receipt tests.

use animus_local_daemon::{
    LocalDaemonError, LocalDaemonReceipt, LocalDaemonReceiptCounts, parse_local_daemon_config,
};

const CONFIG: &[u8] = include_bytes!("../../../config/generated/local-daemon-supervisor.json");
const HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

#[test]
fn receipt_contains_only_bounded_counts_identities_and_non_claims() {
    let config = parse_local_daemon_config(CONFIG).expect("config");
    let receipt = LocalDaemonReceipt::new(
        &config,
        HASH,
        LocalDaemonReceiptCounts {
            workers: 1,
            attachments: 2,
            events: 3,
        },
    )
    .expect("receipt");
    assert_eq!(receipt.worker_count, 1);
    assert_eq!(receipt.attachment_count, 2);
    assert_eq!(receipt.event_count, 3);
    assert_eq!(
        receipt.identity.len(),
        animus_adapter_daemon::BLAKE3_HEX_CHARS
    );
    assert!(
        receipt
            .non_claims
            .iter()
            .any(|value| value == "not-security-sandbox")
    );
}

#[test]
fn malformed_registry_identity_fails_before_receipt_projection() {
    let config = parse_local_daemon_config(CONFIG).expect("config");
    assert_eq!(
        LocalDaemonReceipt::new(
            &config,
            "not-a-hash",
            LocalDaemonReceiptCounts {
                workers: 1,
                attachments: 1,
                events: 1,
            },
        ),
        Err(LocalDaemonError::InvalidIdentity)
    );
}
