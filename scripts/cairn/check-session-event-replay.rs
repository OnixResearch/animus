use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;
const REQUIRED_FILES: [&str; 8] = [
    "crates/animus-session-core/src/sessioneventreplay.rs",
    "config/session-event-replay-policy.ncl",
    "config/generated/session-event-replay-policy.json",
    "docs/session-event-replay.md",
    ".cairn/component-catalog.ncl",
    ".cairn/runtime-facade-inventory.ncl",
    "fixtures/session-event-replay/positive.ncl",
    "fixtures/session-event-replay/negative.ncl",
];
const SOURCE_MARKERS: [&str; 13] = [
    "struct ReplayCursor",
    "struct EventSequence",
    "enum SessionEventClass",
    "DurableSemantic",
    "LiveOnlyDelta",
    "MetadataObservation",
    "enum ReplayOutcome",
    "ReplayOutcome::Gap",
    "ReplayOutcome::Duplicate",
    "ReplayOutcome::Rebuild",
    "fn plan_session_event_replay",
    "fn project_replay_plan",
    "fn project_replay_receipt",
];
const POLICY_MARKERS: [&str; 8] = [
    "maximum_event_count",
    "maximum_projected_bytes",
    "live_only_delta",
    "metadata-only-v1",
    "not-transcript-truth",
    "not-transport-delivery",
    "not-durable-storage",
    "not-conversation-continuity",
];
const REFERENCE_MARKERS: [&str; 4] = [
    "withastro/flue",
    "bf86b8726f5ba189844185fdbeca0e194344ded1",
    "rivet-dev/agentos",
    "5b76f89ffd70817de303cf4654b1badd79c0442d",
];

struct Inputs {
    source: String,
    policy: String,
    readme: String,
    docs: String,
    catalog: String,
    inventory: String,
}

fn parse_root(args: &[String]) -> Result<PathBuf, String> {
    match args {
        [] => Ok(PathBuf::from(".")),
        [flag, root] if flag == "--root" => Ok(PathBuf::from(root)),
        _ => Err("usage: check-session-event-replay [--root PATH]".to_string()),
    }
}

fn read_inputs(root: &Path) -> Result<Inputs, String> {
    for relative in REQUIRED_FILES {
        if !root.join(relative).is_file() {
            return Err(format!("missing required file: {relative}"));
        }
    }
    let read = |relative: &str| {
        fs::read_to_string(root.join(relative))
            .map_err(|error| format!("cannot read {relative}: {error}"))
    };
    Ok(Inputs {
        source: read("crates/animus-session-core/src/sessioneventreplay.rs")?,
        policy: read("config/session-event-replay-policy.ncl")?,
        readme: read("README.md")?,
        docs: read("docs/session-event-replay.md")?,
        catalog: read(".cairn/component-catalog.ncl")?,
        inventory: read(".cairn/runtime-facade-inventory.ncl")?,
    })
}

fn require_markers(label: &str, text: &str, markers: &[&str]) -> Result<(), String> {
    let missing: Vec<&str> = markers
        .iter()
        .copied()
        .filter(|marker| !text.contains(marker))
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!("{label} misses markers: {}", missing.join(", ")))
    }
}

fn validate(inputs: &Inputs) -> Result<(), String> {
    require_markers("source", &inputs.source, &SOURCE_MARKERS)?;
    require_markers("policy", &inputs.policy, &POLICY_MARKERS)?;
    require_markers("README references", &inputs.readme, &REFERENCE_MARKERS)?;
    require_markers(
        "documentation",
        &inputs.docs,
        &["ACP", "headless", "TUI", "second transcript", "live_only_delta"],
    )?;
    require_markers(
        "component catalog",
        &inputs.catalog,
        &["runtime.session-event-replay", "check-session-event-replay"],
    )?;
    require_markers(
        "runtime facade inventory",
        &inputs.inventory,
        &["session-event-replay", "session_event_replay"],
    )?;
    if inputs.source.contains("std::fs")
        || inputs.source.contains("std::process")
        || inputs.source.contains("std::net")
    {
        return Err("pure replay source contains an ambient effect marker".to_string());
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    let root = parse_root(args)?;
    let inputs = read_inputs(&root)?;
    validate(&inputs)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => {
            println!("session event replay static check: pass");
            ExitCode::from(SUCCESS)
        }
        Err(error) => {
            eprintln!("session event replay static check: fail: {error}");
            ExitCode::from(FAILURE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Inputs, validate};

    fn valid_inputs() -> Inputs {
        Inputs {
            source: "struct ReplayCursor struct EventSequence enum SessionEventClass DurableSemantic LiveOnlyDelta MetadataObservation enum ReplayOutcome ReplayOutcome::Gap ReplayOutcome::Duplicate ReplayOutcome::Rebuild fn plan_session_event_replay fn project_replay_plan fn project_replay_receipt".to_string(),
            policy: "maximum_event_count maximum_projected_bytes live_only_delta metadata-only-v1 not-transcript-truth not-transport-delivery not-durable-storage not-conversation-continuity".to_string(),
            readme: "withastro/flue bf86b8726f5ba189844185fdbeca0e194344ded1 rivet-dev/agentos 5b76f89ffd70817de303cf4654b1badd79c0442d".to_string(),
            docs: "ACP headless TUI second transcript live_only_delta".to_string(),
            catalog: "runtime.session-event-replay check-session-event-replay".to_string(),
            inventory: "session-event-replay session_event_replay".to_string(),
        }
    }

    #[test]
    fn accepts_complete_contract_surface() {
        assert_eq!(validate(&valid_inputs()), Ok(()));
    }

    #[test]
    fn rejects_missing_and_effectful_contract_surface() {
        let mut missing = valid_inputs();
        missing.source = "std::fs".to_string();
        assert!(validate(&missing).is_err());
    }
}
