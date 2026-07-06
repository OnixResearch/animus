const RUNTIME_TOOL_RUN_ID: &str = "runtime-tool";
const UNKNOWN_TOOL_NAME: &str = "unknown-tool";
const NO_ACTIVE_TOOL: &str = "tool: none";
const NO_APPROVAL_PROMPT: &str = "approval: none";
const NO_OUTPUT: &str = "output: none";
const LINE_SEPARATOR: char = '\n';
const SESSION_PREFIX: &str = "session: ";
const TOOL_PREFIX: &str = "tool: ";
const RUN_PREFIX: &str = "run: ";
const PROGRESS_PREFIX: &str = "progress: ";
const APPROVAL_PREFIX: &str = "approval: ";
const CHOICES_PREFIX: &str = "choices: ";
const OUTPUT_PREFIX: &str = "output: ";
const FAILURE_PREFIX: &str = "failure: ";
const FINAL_PREFIX: &str = "final: ";
const UNKNOWN_PROGRESS_TOTAL: &str = "?";
const TOOL_DENIED_REASON: &str = "tool denied";
const TOOL_ADAPTER_FAILED_REASON: &str = "tool adapter failed";
const CANCELLED_REASON: &str = "cancelled";

include!("view/model.rs");
include!("view/text.rs");
include!("view/conversationtypes.rs");
include!("view/conversationmod.rs");
include!("view/conversationrendr.rs");

#[cfg(test)]
mod conversation_tests {
    use super::*;
    include!("view/conversationtest.rs");
}
