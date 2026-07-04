fn deduplicate_choices(choices: &[ToolCommandChoice]) -> Vec<ToolCommandChoice> {
    let mut seen = std::collections::BTreeSet::new();
    let mut deduplicated = Vec::with_capacity(choices.len());
    for choice in choices {
        if seen.insert(*choice) {
            deduplicated.push(*choice);
        }
    }
    deduplicated
}

struct RequiredText<'value> {
    field: &'static str,
    value: &'value str,
}

fn validate_required_text(input: RequiredText<'_>) -> Result<String, ToolHostError> {
    let RequiredText { field, value } = input;
    assert!(!field.is_empty());
    assert!(MAX_HOST_TEXT_CHARS > 0);
    if value.trim().is_empty() {
        return Err(ToolHostError::InvalidText {
            field,
            reason: REASON_EMPTY,
        });
    }
    if value.chars().count() > MAX_HOST_TEXT_CHARS {
        return Err(ToolHostError::InvalidText {
            field,
            reason: REASON_TOO_LONG,
        });
    }
    if value.chars().any(|character| character == NUL_CHARACTER) {
        return Err(ToolHostError::InvalidText {
            field,
            reason: REASON_NUL,
        });
    }
    Ok(String::from(value))
}
