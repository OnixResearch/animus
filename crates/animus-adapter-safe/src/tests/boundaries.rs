use super::*;

fn receipt(label: &str) -> Result<Record, Error> {
    Record::new(
        Draft::new(Status::Completed, IssueCode::None, Operation::Mutation)
            .with_labels(&[label])
            .with_redactions(&[Redaction::OutputBounded]),
    )
}

#[test]
fn unicode_admission_retains_character_limits_and_byte_offsets() -> Result<(), Error> {
    let valid = "é".repeat(MAX_LABEL_CHARS);
    assert_eq!(receipt(&valid)?.labels(), core::slice::from_ref(&valid));
    let invalid = valid + "é";
    assert_eq!(
        receipt(&invalid),
        Err(Error::UnsafeMetadata {
            field: "receipt_label",
            class: LeakClass::UnboundedOutput,
        })
    );
    assert_eq!(
        receipt("é\0"),
        Err(Error::NulCharacter {
            field: "receipt_label",
            byte_index: "é".len(),
        })
    );
    Ok(())
}

#[test]
fn review_buffer_grows_without_accepting_unsafe_labels() -> Result<(), Error> {
    let valid = receipt("  event  ")?;
    assert_eq!(valid.labels(), &["event"]);
    assert_eq!(
        valid.review_summary(),
        "safe_receipt animus.safe-receipt.v1 completed none mutation event output_bounded"
    );
    for (text, class) in [
        ("prompt: synthetic", LeakClass::RawPrompt),
        ("{\"model\":\"synthetic\"}", LeakClass::ProviderJson),
        ("token=synthetic", LeakClass::Credential),
        ("/tmp/synthetic", LeakClass::AbsolutePath),
        ("tool_input: synthetic", LeakClass::RawToolInput),
        ("tool_output: synthetic", LeakClass::RawToolOutput),
        ("model_output: synthetic", LeakClass::RawModelOutput),
    ] {
        assert_eq!(
            receipt(text),
            Err(Error::UnsafeMetadata {
                field: "receipt_label",
                class
            })
        );
    }
    assert_eq!(
        receipt(" "),
        Err(Error::EmptyField {
            field: "receipt_label"
        })
    );
    Ok(())
}

#[test]
fn hash_conversion_retains_exact_lowercase_contract() -> Result<(), Error> {
    let bytes = [u8::MAX; BLAKE3_HASH_BYTES];
    let valid = "f".repeat(BLAKE3_HASH_HEX_CHARS);
    assert_eq!(ArtifactHash::from_bytes(&bytes).as_str(), valid);
    assert_eq!(ArtifactHash::new(&valid)?.as_str(), valid);
    for invalid in [
        valid.to_uppercase(),
        alloc::string::String::new(),
        "é".repeat(BLAKE3_HASH_HEX_CHARS),
    ] {
        assert!(matches!(
            ArtifactHash::new(&invalid),
            Err(Error::InvalidHash { .. })
        ));
    }
    Ok(())
}

#[test]
fn collections_and_counts_still_reject_excess() -> Result<(), Error> {
    let labels = [SAFE_LABEL; MAX_COLLECTION_ITEMS];
    let accepted_count = [BoundedCountDraft::new(COUNT_LABEL, MAX_COUNT)];
    let draft = Draft::new(Status::Completed, IssueCode::None, Operation::Mutation)
        .with_labels(&labels)
        .with_counts(&accepted_count)
        .with_redactions(&[Redaction::OutputBounded]);
    assert_eq!(Record::new(draft)?.counts()[0].value(), MAX_COUNT);
    let rejected_count = [BoundedCountDraft::new(
        COUNT_LABEL,
        MAX_COUNT.checked_add(1).expect("test boundary fits u32"),
    )];
    assert!(matches!(
        Record::new(draft.with_counts(&rejected_count)),
        Err(Error::CountTooLarge { .. })
    ));
    let excess_items = MAX_COLLECTION_ITEMS
        .checked_add(1)
        .expect("test boundary fits usize");
    let excess = alloc::vec![SAFE_LABEL; excess_items];
    assert!(matches!(
        Record::new(draft.with_labels(&excess)),
        Err(Error::CollectionTooLarge { .. })
    ));
    assert!(matches!(
        Record::new(draft.with_labels(&[])),
        Err(Error::EmptyCollection { .. })
    ));
    Ok(())
}
