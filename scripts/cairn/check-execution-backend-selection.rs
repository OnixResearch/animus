use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const SUCCESS: u8 = 0;
const FAILURE: u8 = 1;
const REQUIRED_FILES: [&str; 9] = [
    "crates/animus-runtime/src/executionbackend.rs",
    "crates/animus-std/src/executionbackend.rs",
    "config/execution-backend-selection-policy.ncl",
    "config/generated/execution-backend-selection-policy.json",
    "fixtures/execution-backend-selection/positive.ncl",
    "fixtures/execution-backend-selection/negative.ncl",
    "fixtures/execution-backend-selection/lattice-consumer.ncl",
    "docs/execution-backend-selection.md",
    "README.md",
];
const SOURCE_MARKERS: [&str; 14] = [
    "enum ExecutionBackendClass",
    "EmbeddedComponent",
    "GuardedLocalProcess",
    "IsolatedHost",
    "ExternalMachine",
    "enum WorkloadRequirement",
    "struct ExecutionBackendCandidate",
    "struct BackendSelectionPolicy",
    "fn select_execution_backend",
    "SelectedBackendUnavailable",
    "trait CurrentBackendEnforcementService",
    "fn recheck_execution_backend",
    "fn project_backend_selection_receipt",
    "fn validate_backend_evidence_text",
];
const POLICY_MARKERS: [&str; 9] = [
    "preference_order",
    "class_policies",
    "deny_silent_fallback",
    "receipt_redaction",
    "not-backend-availability",
    "not-sandbox-enforcement",
    "not-work-completion",
    "not-output-correctness",
    "metadata-only-v1",
];

struct Inputs {
    source: String,
    host: String,
    policy: String,
    consumer: String,
    docs: String,
    readme: String,
}

fn parse_root(args: &[String]) -> Result<PathBuf, String> {
    match args {
        [] => Ok(PathBuf::from(".")),
        [flag, root] if flag == "--root" => Ok(PathBuf::from(root)),
        _ => Err("usage: check-execution-backend-selection [--root PATH]".to_string()),
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
        source: read("crates/animus-runtime/src/executionbackend.rs")?,
        host: read("crates/animus-std/src/executionbackend.rs")?,
        policy: read("config/execution-backend-selection-policy.ncl")?,
        consumer: read("fixtures/execution-backend-selection/lattice-consumer.ncl")?,
        docs: read("docs/execution-backend-selection.md")?,
        readme: read("README.md")?,
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
    require_markers(
        "host",
        &inputs.host,
        &["ExecutionBackendHost", "CurrentBackendEnforcementService", "recheck"],
    )?;
    require_markers(
        "Lattice consumer",
        &inputs.consumer,
        &["090162f758fdb43c86c9d93d58e4348a478bd86e", "rejected_fields"],
    )?;
    require_markers(
        "documentation",
        &inputs.docs,
        &["weakest", "explicit new plan", "current enforcement", "not prove"],
    )?;
    require_markers(
        "README reference",
        &inputs.readme,
        &["rivet-dev/agentos", "5b76f89ffd70817de303cf4654b1badd79c0442d"],
    )?;
    if inputs.source.contains("std::fs")
        || inputs.source.contains("std::process")
        || inputs.source.contains("std::net")
        || inputs.source.contains("std::env")
    {
        return Err("pure planner contains an ambient effect marker".to_string());
    }
    Ok(())
}

fn run(args: &[String]) -> Result<(), String> {
    validate(&read_inputs(&parse_root(args)?)?)
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => {
            println!("execution backend selection static check: pass");
            ExitCode::from(SUCCESS)
        }
        Err(error) => {
            eprintln!("execution backend selection static check: fail: {error}");
            ExitCode::from(FAILURE)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Inputs, validate};

    fn complete() -> Inputs {
        Inputs {
            source: "enum ExecutionBackendClass EmbeddedComponent GuardedLocalProcess IsolatedHost ExternalMachine enum WorkloadRequirement struct ExecutionBackendCandidate struct BackendSelectionPolicy fn select_execution_backend SelectedBackendUnavailable trait CurrentBackendEnforcementService fn recheck_execution_backend fn project_backend_selection_receipt fn validate_backend_evidence_text".to_string(),
            host: "ExecutionBackendHost CurrentBackendEnforcementService recheck".to_string(),
            policy: "preference_order class_policies deny_silent_fallback receipt_redaction not-backend-availability not-sandbox-enforcement not-work-completion not-output-correctness metadata-only-v1".to_string(),
            consumer: "090162f758fdb43c86c9d93d58e4348a478bd86e rejected_fields".to_string(),
            docs: "weakest explicit new plan current enforcement not prove".to_string(),
            readme: "rivet-dev/agentos 5b76f89ffd70817de303cf4654b1badd79c0442d".to_string(),
        }
    }

    #[test]
    fn accepts_complete_surface() {
        assert_eq!(validate(&complete()), Ok(()));
    }

    #[test]
    fn rejects_missing_or_effectful_surface() {
        let mut inputs = complete();
        inputs.source = "std::process".to_string();
        assert!(validate(&inputs).is_err());
    }
}
