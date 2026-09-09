pub(super) fn classify(value: &str) -> Option<crate::LeakClass> {
    assert!(!value.is_empty(), "safe metadata value must not be empty");
    assert!(
        value.len() >= value.chars().count(),
        "byte length covers chars"
    );
    let lower = value.to_ascii_lowercase();
    let trimmed = lower.trim();
    if contains_any(trimmed, crate::PROMPT_MARKERS) {
        return Some(crate::LeakClass::RawPrompt);
    }
    if is_provider_json(trimmed) {
        return Some(crate::LeakClass::ProviderJson);
    }
    after_provider(trimmed)
}

fn after_provider(value: &str) -> Option<crate::LeakClass> {
    if contains_any(value, crate::CREDENTIAL_MARKERS) {
        return Some(crate::LeakClass::Credential);
    }
    if is_path_like(value) {
        return Some(crate::LeakClass::AbsolutePath);
    }
    after_path(value)
}

fn after_path(value: &str) -> Option<crate::LeakClass> {
    if contains_any(value, crate::TOOL_INPUT_MARKERS) {
        return Some(crate::LeakClass::RawToolInput);
    }
    if contains_any(value, crate::TOOL_OUTPUT_MARKERS) {
        return Some(crate::LeakClass::RawToolOutput);
    }
    if contains_any(value, crate::MODEL_OUTPUT_MARKERS) {
        return Some(crate::LeakClass::RawModelOutput);
    }
    None
}

fn is_provider_json(value: &str) -> bool {
    if !value.starts_with(crate::JSON_OBJECT_PREFIX) {
        return false;
    }
    contains_any(value, crate::PROVIDER_MARKERS)
}

fn is_path_like(value: &str) -> bool {
    if value.starts_with(crate::ROOT_PATH_MARKER) {
        return true;
    }
    contains_any(value, crate::PATH_MARKERS)
}

fn contains_any(value: &str, markers: &[&str]) -> bool {
    assert!(!markers.is_empty(), "marker list must not be empty");
    assert!(
        value.len() >= value.chars().count(),
        "byte length covers chars"
    );
    markers.iter().any(|marker| value.contains(marker))
}
