use super::*;

mod boundaries;

const HASH_TEXT: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";
const SAFE_LABEL: &str = "runtime_started";
const SAFE_BACKEND: &str = "local";
const COUNT_LABEL: &str = "events";
const COUNT_VALUE: u32 = 1;
const UNSAFE_CREDENTIAL: &str = "token=secret";
const PRIVATE_PATH: &str = "/home/example/private.txt";

#[test]
fn positive_safe_receipt_accepts_bounded_metadata() -> Result<(), Error> {
    let hash = ArtifactHash::new(HASH_TEXT)?;
    let counts = [BoundedCountDraft::new(COUNT_LABEL, COUNT_VALUE)];
    let receipt = Record::new(
        Draft::new(
            Status::Completed,
            IssueCode::None,
            Operation::ProvenanceObservation,
        )
        .with_labels(&[SAFE_LABEL])
        .with_counts(&counts)
        .with_artifact_hashes(&[hash])
        .with_backend_label(SAFE_BACKEND)
        .with_redactions(&[Redaction::PayloadHashOnly]),
    )?;
    assert_eq!(receipt.schema(), SCHEMA);
    assert_eq!(receipt.counts()[0].value(), COUNT_VALUE);
    assert!(receipt.review_summary().contains(SAFE_LABEL));
    Ok(())
}

#[test]
fn negative_safe_receipt_rejects_leaks_and_bad_hashes() {
    assert!(matches!(
        Record::new(
            Draft::new(
                Status::Blocked,
                IssueCode::LeakRejected,
                Operation::RuntimeBlock
            )
            .with_labels(&[UNSAFE_CREDENTIAL])
            .with_redactions(&[Redaction::CredentialOmitted]),
        ),
        Err(Error::UnsafeMetadata {
            class: LeakClass::Credential,
            ..
        })
    ));
    assert!(matches!(
        Record::new(
            Draft::new(
                Status::Blocked,
                IssueCode::LeakRejected,
                Operation::RuntimeBlock
            )
            .with_labels(&[PRIVATE_PATH])
            .with_redactions(&[Redaction::AbsolutePathOmitted]),
        ),
        Err(Error::UnsafeMetadata {
            class: LeakClass::AbsolutePath,
            ..
        })
    ));
    assert!(matches!(
        ArtifactHash::new("not-a-blake3-hex"),
        Err(Error::InvalidHash { .. })
    ));
}
