const PAYLOAD_BYTES_COUNT: &str = "payload_bytes";
const INTERCHANGE_LABEL: &str = "animus_std";

pub(super) fn build(
    descriptor: super::taxonomy::AdapterDescriptor<'_>,
    issue: super::outcome::AdapterIssue,
    payload: &str,
) -> Result<animus_adapters::SafeReceipt, animus_adapters::SafeReceiptError> {
    debug_assert!(descriptor.family().is_shell_owned());
    debug_assert!(!PAYLOAD_BYTES_COUNT.is_empty());
    debug_assert!(!INTERCHANGE_LABEL.is_empty());
    debug_assert!(payload.len() >= payload.chars().count());

    let labels = [
        descriptor.family().as_str(),
        descriptor.label(),
        descriptor.exposure().as_str(),
        issue.as_str(),
    ];
    let counts = [animus_adapters::BoundedCountDraft::new(
        PAYLOAD_BYTES_COUNT,
        bounded_payload_bytes(payload),
    )];
    let payload_hash = payload_hash(payload);
    let hashes = [payload_hash];
    let draft = animus_adapters::SafeReceiptDraft::new(
        issue.receipt_status(),
        issue.receipt_issue_code(),
        descriptor.family().operation(),
    )
    .with_labels(&labels)
    .with_counts(&counts)
    .with_artifact_hashes(&hashes)
    .with_backend_label(descriptor.family().as_str())
    .with_interchange_label(INTERCHANGE_LABEL)
    .with_redactions(descriptor.family().redactions());

    animus_adapters::SafeReceipt::new(draft)
}

fn payload_hash(payload: &str) -> animus_adapters::ArtifactHash {
    let hash = blake3::hash(payload.as_bytes());
    animus_adapters::ArtifactHash::from_bytes(hash.as_bytes())
}

fn bounded_payload_bytes(payload: &str) -> u32 {
    let payload_bytes = match u32::try_from(payload.len()) {
        Ok(value) => value,
        Err(_error) => animus_adapters::MAX_SAFE_RECEIPT_COUNT,
    };
    payload_bytes.min(animus_adapters::MAX_SAFE_RECEIPT_COUNT)
}
