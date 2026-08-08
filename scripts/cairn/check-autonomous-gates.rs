// r[verify agent_harness_runtime.autonomous_gates.verification]
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const ARG_ROOT: &str = "--root";
const RAIL_NAME: &str = "autonomous-gates";
const RECEIPT_PATH: &str = "target/cairn/focused-rails/autonomous-gates.md";
const PURE_SOURCE: &str = "crates/animus-runtime/src/autonomous_gates/mod.rs";
const POLICY_SOURCE: &str = "config/autonomous-gates-policy.ncl";
const POLICY_JSON: &str = "config/generated/autonomous-gates-policy.json";
const FORBIDDEN_PURE_MARKERS: [&str; 8] = [
    "std::fs",
    "std::process",
    "std::env",
    "Command::new",
    "SystemTime::now",
    "Instant::now",
    "std::net",
    "std::time",
];
const REQUIRED_SOURCE_MARKERS: [&str; 6] = [
    "fingerprint_admitted_file_set",
    "bound_gate_output",
    "gate_rerun_decision",
    "decide_continuation",
    "project_receipt",
    "generated_nickel_policy_matches_rust_defaults",
];
const REQUIRED_POLICY_MARKERS: [&str; 5] = [
    "animus.autonomous-gates.v1",
    "max_continuations",
    "max_gate_output_bytes",
    "not-task-success",
    "not-change-detection-outside-admitted-file-set",
];

fn main() {
    match run() {
        Ok(()) => std::process::exit(EXIT_SUCCESS),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(EXIT_FAILURE);
        }
    }
}

fn run() -> Result<(), String> {
    let root = parse_root(env::args())?;
    validate_static_contracts(&root)?;
    run_commands(&root)?;
    write_receipt(&root)?;
    println!("focused rail {RAIL_NAME}: pass");
    println!("receipt: {RECEIPT_PATH}");
    Ok(())
}

fn parse_root(mut args: impl Iterator<Item = String>) -> Result<PathBuf, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");
    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                root = PathBuf::from(
                    args.next()
                        .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?,
                );
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    root.canonicalize()
        .map_err(|error| format!("failed to resolve root: {error}"))
}

fn validate_static_contracts(root: &Path) -> Result<(), String> {
    let source = read_text(&root.join(PURE_SOURCE))?;
    for marker in FORBIDDEN_PURE_MARKERS {
        if source.contains(marker) {
            return Err(format!("pure autonomous source contains forbidden marker {marker}"));
        }
    }
    for marker in REQUIRED_SOURCE_MARKERS {
        if !source.contains(marker) {
            return Err(format!("pure autonomous source misses required marker {marker}"));
        }
    }
    for path in [POLICY_SOURCE, POLICY_JSON] {
        let text = read_text(&root.join(path))?;
        for marker in REQUIRED_POLICY_MARKERS {
            if !text.contains(marker) {
                return Err(format!("{path} misses required policy marker {marker}"));
            }
        }
    }
    Ok(())
}

fn run_commands(root: &Path) -> Result<(), String> {
    run_command(root, "cargo", &["fmt", "--all", "--", "--check"])?;
    run_command(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "animus-runtime",
            "--features",
            "autonomous-gates",
            "autonomous_gates",
            "--locked",
        ],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "check",
            "-p",
            "animus-runtime",
            "--no-default-features",
            "--features",
            "autonomous-gates",
            "--target",
            "wasm32-unknown-unknown",
            "--locked",
        ],
    )?;
    run_command(root, "scripts/config/check-nickel-configs", &[])?;
    run_command(root, "nix", &["run", ".#cairn", "--", "validate", "--root", "."])
}

fn run_command(root: &Path, program: &str, arguments: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(arguments)
        .current_dir(root)
        .stdin(Stdio::null())
        .status()
        .map_err(|error| format!("failed to run {program}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} failed with status {status}"))
    }
}

fn write_receipt(root: &Path) -> Result<(), String> {
    let path = root.join(RECEIPT_PATH);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create receipt directory: {error}"))?;
    }
    let receipt = format!(
        "# Focused rail: {RAIL_NAME}\n\n- schema: animus.focused-rail.v1\n- status: pass\n- pure core: {PURE_SOURCE}\n- policy: {POLICY_SOURCE}\n- positive fixtures: bounded output, changed-workspace rerun, gate scope, generated policy parity\n- negative fixtures: unchanged failure, limit-as-success, unsafe output, unsafe path, duplicate path\n- non-claims: task success, objective completion, gate sandboxing, production readiness, release eligibility\n"
    );
    fs::write(path, receipt).map_err(|error| format!("failed to write receipt: {error}"))
}

fn read_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}
