fn ensure_name(value: &str) -> Result<(), Error> {
    if value.is_empty() {
        return Err(Error::EmptyName);
    }
    Ok(())
}

fn ensure_label_count(count: usize) -> Result<(), Error> {
    if !(MIN_LABEL_COUNT..=MAX_LABEL_COUNT).contains(&count) {
        return Err(Error::InvalidLabelCount);
    }
    Ok(())
}

fn validate_safe_labels(labels: &[&str]) -> Result<(), Error> {
    animus_adapter_safe::Record::new(
        animus_adapter_safe::Draft::new(
            animus_adapter_safe::Status::Completed,
            animus_adapter_safe::IssueCode::ProvenanceObservation,
            animus_adapter_safe::Operation::ProvenanceObservation,
        )
        .with_labels(labels)
        .with_redactions(&[animus_adapter_safe::Redaction::PayloadHashOnly]),
    )
    .map(|_record| ())
    .map_err(Error::UnsafeMetadata)
}

struct SafeLabelEnvelope<'a> {
    schema: &'static str,
    name: &'a str,
    labels: &'a [&'a str],
}

fn validate_safe_label_envelope(envelope: &SafeLabelEnvelope<'_>) -> Result<(), Error> {
    let mut combined = alloc::vec::Vec::with_capacity(
        envelope
            .labels
            .len()
            .saturating_add(SAFE_LABEL_PREFIX_COUNT),
    );
    combined.push(envelope.schema);
    combined.push(OBSERVABILITY_OPERATION_LABEL);
    combined.push(envelope.name);
    combined.extend_from_slice(envelope.labels);
    validate_safe_labels(combined.as_slice())
}

fn label_strings(labels: &[MetricLabel]) -> alloc::vec::Vec<alloc::string::String> {
    labels.iter().map(MetricLabel::joined).collect()
}

fn label_refs(labels: &[alloc::string::String]) -> alloc::vec::Vec<&str> {
    labels.iter().map(alloc::string::String::as_str).collect()
}
