const FIELD_RUN_ID: &str = "run_id";
const FIELD_CORRELATION_ID: &str = "correlation_id";
const FIELD_PROMPT: &str = "prompt";
const FIELD_REASON: &str = "reason";
const FIELD_CHOICES: &str = "choices";
const FIELD_PROGRESS_TOTAL: &str = "progress_total";
const REASON_EMPTY: &str = "empty";
const REASON_TOO_LONG: &str = "too_long";
const REASON_NUL: &str = "contains_nul";
const REASON_PROGRESS_EXCEEDS_TOTAL: &str = "completed_exceeds_total";
const NUL_CHARACTER: char = '\0';
const MAX_HOST_TEXT_CHARS: usize = animus_adapters::MAX_ADAPTER_FIELD_CHARS;

include!("bridge/core.rs");
include!("bridge/failure.rs");
include!("bridge/event.rs");
include!("bridge/validation.rs");

#[cfg(test)]
pub(crate) mod test_support {
    include!("bridge/support.rs");
}
