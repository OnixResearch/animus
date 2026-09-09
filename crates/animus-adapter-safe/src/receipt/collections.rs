pub(super) fn labels(
    values: &[&str],
) -> Result<alloc::vec::Vec<alloc::string::String>, crate::Error> {
    crate::ensure_required_collection(crate::LABELS_FIELD, values.len())?;
    let mut labels = values
        .iter()
        .map(|value| crate::required_label(crate::LABEL_FIELD, value))
        .collect::<Result<alloc::vec::Vec<_>, _>>()?;
    labels.sort();
    labels.dedup();
    Ok(labels)
}

pub(super) fn counts(
    values: &[super::BoundedCountDraft<'_>],
) -> Result<alloc::vec::Vec<super::BoundedCount>, crate::Error> {
    crate::ensure_optional_collection(crate::COUNTS_FIELD, values.len())?;
    let mut counts = values
        .iter()
        .map(count)
        .collect::<Result<alloc::vec::Vec<_>, _>>()?;
    counts.sort();
    counts.dedup();
    Ok(counts)
}

pub(super) fn hashes(
    values: &[crate::ArtifactHash],
) -> Result<alloc::vec::Vec<crate::ArtifactHash>, crate::Error> {
    crate::ensure_optional_collection(crate::HASHES_FIELD, values.len())?;
    let mut hashes = values.to_vec();
    hashes.sort();
    hashes.dedup();
    Ok(hashes)
}

pub(super) fn redactions(
    values: &[crate::Redaction],
) -> Result<alloc::vec::Vec<crate::Redaction>, crate::Error> {
    crate::ensure_required_collection(crate::REDACTIONS_FIELD, values.len())?;
    let mut redactions = values.to_vec();
    redactions.sort();
    redactions.dedup();
    Ok(redactions)
}

fn count(value: &super::BoundedCountDraft<'_>) -> Result<super::BoundedCount, crate::Error> {
    let label = crate::required_label(crate::COUNT_LABEL_FIELD, value.label)?;
    if value.value > crate::MAX_COUNT {
        return Err(crate::Error::CountTooLarge {
            field: crate::COUNTS_FIELD,
            max: crate::MAX_COUNT,
            actual: value.value,
        });
    }
    Ok(super::BoundedCount::new(label, value.value))
}
