const EVENT_CONSUMER_UNAVAILABLE: &str = "event consumer unavailable";
const COMMAND_CHANNEL_CLOSED: &str = "command channel closed";
const ADAPTER_FAILED: &str = "tool adapter failed";
const GUARDED_ACTION_DENIED: &str = "guarded action denied";
const GUARDED_ACTION_CANCELLED: &str = "guarded action cancelled";
const INVALID_COMMAND: &str = "invalid command for approval request";

include!("runner/types.rs");
include!("runner/exec.rs");
include!("runner/failure.rs");
