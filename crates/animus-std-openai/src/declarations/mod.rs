//! Exact tool metadata, admission, and request projection.

mod admission;
mod catalog;
mod correlation;
mod projection;
mod selection;
mod shapes;

pub use admission::OpenAiToolDispatch;
pub use catalog::{OpenAiToolCapability, OpenAiToolCapabilityMatrix, OpenAiToolStreamChannel};
pub use correlation::{OpenAiCompletedToolCall, OpenAiContinuationBinding, OpenAiToolResult};
pub use projection::build_tool_capable_openai_request_body;
pub use selection::{OpenAiToolChoice, OpenAiToolPolicy};
pub use shapes::OpenAiToolSchemaBinding;

/// Stable reviewed schema label for the model-facing Steel REPL function.
pub const STEEL_REPL_TOOL_SCHEMA: &str = animus_adapters::rlm_kernel::STEEL_REPL_TOOL_SCHEMA;
/// Stable reviewed model-facing Steel REPL function name.
pub const STEEL_REPL_TOOL_NAME: &str = animus_adapters::rlm_kernel::STEEL_REPL_TOOL_NAME;

const MAX_SAFE_LABEL_CHARS: usize = 256;
const MAX_TOOL_CAPABILITIES: usize = 64;
const MAX_SCHEMA_BYTES: usize = 65_536;
const MAX_TOOL_RESULT_BYTES: usize = 262_144;
const INVALID_LABEL_REASON: &str = "OpenAI tool metadata contains an invalid label";
const DUPLICATE_CAPABILITY_REASON: &str = "OpenAI tool capability metadata contains a duplicate";
const CAPABILITY_BOUND_REASON: &str = "OpenAI tool capability metadata exceeds its bound";
const SCHEMA_JSON_REASON: &str = "OpenAI tool schema JSON is malformed";
const SCHEMA_SHAPE_REASON: &str = "OpenAI strict tool schema shape is unsupported";
const SCHEMA_BOUND_REASON: &str = "OpenAI tool schema exceeds its byte bound";
const RESULT_BOUND_REASON: &str = "OpenAI tool result exceeds its byte bound";
const CONTINUATION_MISMATCH_REASON: &str = "OpenAI continuation identities do not match";
const CONTINUATION_MESSAGE_REASON: &str =
    "OpenAI continuation requires one matching canonical tool result";
const INITIAL_TOOL_RESULT_REASON: &str = "OpenAI initial request cannot contain a tool result";
const TOOL_RESULT_TEXT_REASON: &str = "OpenAI tool result must contain exactly one text part";
const TOOL_RESULT_VALUE_REASON: &str =
    "OpenAI canonical tool result does not match the supplied result";

fn validate_label(value: &str) -> Result<(), crate::HostError> {
    if value.is_empty()
        || std::cmp::PartialOrd::gt(&value.chars().count(), &MAX_SAFE_LABEL_CHARS)
        || value
            .chars()
            .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(crate::HostError::InvalidToolCohort {
            reason: INVALID_LABEL_REASON,
        });
    }
    Ok(())
}

fn validate_unique<T: Ord>(values: &[T]) -> Result<(), crate::HostError> {
    let mut unique = std::collections::BTreeSet::new();
    for value in values {
        if !unique.insert(value) {
            return Err(crate::HostError::InvalidToolCohort {
                reason: DUPLICATE_CAPABILITY_REASON,
            });
        }
    }
    Ok(())
}

fn validate_unique_by<T, K: Ord>(
    values: &[T],
    key: impl Fn(&T) -> K,
) -> Result<(), crate::HostError> {
    let mut unique = std::collections::BTreeSet::new();
    for value in values {
        if !unique.insert(key(value)) {
            return Err(crate::HostError::InvalidToolCohort {
                reason: DUPLICATE_CAPABILITY_REASON,
            });
        }
    }
    Ok(())
}

fn unsupported<T>(feature: &'static str) -> Result<T, crate::HostError> {
    Err(crate::HostError::UnsupportedPlan { feature })
}

pub(crate) fn blake3_identity(bytes: &[u8]) -> String {
    let mut output = String::from("BLAKE3:");
    output.push_str(blake3::hash(bytes).to_hex().as_str());
    output
}
