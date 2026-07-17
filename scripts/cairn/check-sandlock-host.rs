use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output};

const EXIT_FAILURE: u8 = 1;
const EXIT_SUCCESS: u8 = 0;
const ARG_ROOT: &str = "--root";
const LIVE_ENV: &str = "ANIMUS_SANDLOCK_LINUX_LIVE";
const LIVE_ENABLED: &str = "1";
const COHORT_VERSION: &str = "0.7.0";
const COHORT_REVISION: &str = "e8c4cc0ef58e3fc145c4d147a61b889b4f291a10";
const COHORT_LICENSE: &str = "Apache-2.0";
const FEATURE_NAME: &str = "sandlock-host";
const DEPENDENCY_NAME: &str = "sandlock-core";
const NON_LINUX_TARGET: &str = "x86_64-apple-darwin";
const RECEIPT_PATH: &str = "target/cairn/focused-rails/sandlock-host.md";
const LOCK_PATH: &str = "Cargo.lock";
const STD_MANIFEST: &str = "crates/animus-std/Cargo.toml";
const STD_SOURCE_ROOT: &str = "crates/animus-std/src/sandlockhost";
const CONTRACT_DOC: &str = "docs/sandlock-sandbox-enforcement.md";
const REUSABLE_ROOTS: &[&str] = &[
    "crates/animus-core",
    "crates/animus-adapters",
    "crates/animus-runtime",
];
const REQUIRED_SOURCE_MARKERS: &[&str] = &[
    "r[impl agent_harness_runtime.sandlock_host_adapter.translation]",
    "r[impl agent_harness_runtime.sandlock_host_adapter.preflight]",
    "r[impl agent_harness_runtime.sandlock_host_adapter.authority]",
    "r[impl agent_harness_runtime.sandlock_host_adapter.execution]",
    "r[impl agent_harness_runtime.sandlock_host_adapter.receipts]",
    "validate_mechanism_probe_suite",
    "supplied != required",
    "cow_snapshot",
    "not_upstream_private_storage_cleanup",
];
const REQUIRED_DOC_MARKERS: &[&str] = &[
    "not authorization",
    "kernel correctness",
    "ANIMUS_SANDLOCK_LINUX_LIVE=1",
    COHORT_REVISION,
    COHORT_LICENSE,
];

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::from(EXIT_SUCCESS),
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(EXIT_FAILURE)
        }
    }
}

fn run() -> Result<(), String> {
    let root = parse_root()?;
    validate_source_contract(&root)?;
    validate_dependency_isolation(&root)?;
    run_checks(&root)?;
    let live_status = run_live_check_if_enabled(&root)?;
    write_receipt(&root, live_status.as_str())?;
    println!("focused rail sandlock-host: pass");
    println!("receipt: {}", root.join(RECEIPT_PATH).display());
    Ok(())
}

fn parse_root() -> Result<PathBuf, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [] => env::current_dir().map_err(|error| error.to_string()),
        [flag, root] if flag == ARG_ROOT => PathBuf::from(root)
            .canonicalize()
            .map_err(|error| format!("cannot resolve root: {error}")),
        _ => Err("usage: check-sandlock-host [--root PATH]".to_owned()),
    }
}

fn validate_source_contract(root: &Path) -> Result<(), String> {
    let manifest = read(root, STD_MANIFEST)?;
    require(
        manifest.contains(&format!("{FEATURE_NAME} = [\"dep:{DEPENDENCY_NAME}\"")),
        "optional sandlock-host feature is missing",
    )?;
    require(
        manifest.contains("[target.'cfg(target_os = \"linux\")'.dependencies]"),
        "sandlock dependency is not Linux-target-scoped",
    )?;
    require(
        manifest.contains(COHORT_REVISION),
        "reviewed Sandlock revision is not pinned",
    )?;
    let lock = read(root, LOCK_PATH)?;
    require(
        lock.contains(&format!("name = \"{DEPENDENCY_NAME}\""))
            && lock.contains(&format!("version = \"{COHORT_VERSION}\""))
            && lock.contains(COHORT_REVISION),
        "Cargo.lock does not bind the reviewed Sandlock cohort",
    )?;
    let source = read_tree(root.join(STD_SOURCE_ROOT).as_path())?;
    for marker in REQUIRED_SOURCE_MARKERS {
        require(
            source.contains(marker),
            format!("required source marker is missing: {marker}").as_str(),
        )?;
    }
    let docs = read(root, CONTRACT_DOC)?;
    for marker in REQUIRED_DOC_MARKERS {
        require(
            docs.contains(marker),
            format!("required documentation marker is missing: {marker}").as_str(),
        )?;
    }
    for reusable in REUSABLE_ROOTS {
        let text = read_tree(root.join(reusable).as_path())?;
        require(
            !text.contains("sandlock_core") && !text.contains("sandlock-core"),
            format!("reusable crate gained Sandlock dependency: {reusable}").as_str(),
        )?;
    }
    Ok(())
}

fn validate_dependency_isolation(root: &Path) -> Result<(), String> {
    let default_tree = capture(
        root,
        "cargo",
        &[
            "tree",
            "-p",
            "animus-std",
            "--no-default-features",
            "--locked",
        ],
    )?;
    require(
        !default_tree.contains(DEPENDENCY_NAME),
        "no-default animus-std unexpectedly includes sandlock-core",
    )?;
    let non_linux_tree = capture(
        root,
        "cargo",
        &[
            "tree",
            "-p",
            "animus-std",
            "--features",
            FEATURE_NAME,
            "--target",
            NON_LINUX_TARGET,
            "--locked",
        ],
    )?;
    require(
        !non_linux_tree.contains(DEPENDENCY_NAME),
        "non-Linux feature graph unexpectedly includes sandlock-core",
    )?;
    let linux_tree = capture(
        root,
        "cargo",
        &[
            "tree",
            "-p",
            "animus-std",
            "--features",
            FEATURE_NAME,
            "--target",
            "x86_64-unknown-linux-gnu",
            "--locked",
        ],
    )?;
    require(
        linux_tree.contains(DEPENDENCY_NAME),
        "Linux sandlock-host feature graph omitted sandlock-core",
    )
}

fn run_checks(root: &Path) -> Result<(), String> {
    run_command(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "animus-adapters",
            "--locked",
            "sandbox",
            "--no-fail-fast",
        ],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "animus-std",
            "--features",
            FEATURE_NAME,
            "--locked",
            "sandlock_host",
            "--no-fail-fast",
        ],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "animus-std",
            "--features",
            FEATURE_NAME,
            "--test",
            "sandlock-live",
            "--locked",
        ],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "clippy",
            "-p",
            "animus-std",
            "--all-targets",
            "--features",
            FEATURE_NAME,
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "check",
            "-p",
            "animus-std",
            "--no-default-features",
            "--locked",
        ],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "check",
            "-p",
            "animus",
            "--features",
            FEATURE_NAME,
            "--locked",
        ],
    )?;
    run_command(
        root,
        "bash",
        &["scripts/cairn/check-safe-receipts-artifacts", ARG_ROOT, "."],
    )
}

fn run_live_check_if_enabled(root: &Path) -> Result<String, String> {
    if env::var(LIVE_ENV).as_deref() != Ok(LIVE_ENABLED) {
        return Ok(format!(
            "blocked: set {LIVE_ENV}={LIVE_ENABLED} on a reviewed Linux host with Landlock ABI v6, seccomp notification, and unprivileged Sandlock support"
        ));
    }
    run_command(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "animus-std",
            "--features",
            FEATURE_NAME,
            "--test",
            "sandlock-live",
            "--locked",
            "--",
            "--ignored",
            "--nocapture",
        ],
    )?;
    Ok("passed: explicit real-Linux denial and COW behavior rail".to_owned())
}

fn write_receipt(root: &Path, live_status: &str) -> Result<(), String> {
    let lock_hash = capture(root, "b3sum", &[LOCK_PATH])?
        .split_whitespace()
        .next()
        .ok_or_else(|| "b3sum did not return a lock hash".to_owned())?
        .to_owned();
    let receipt = format!(
        "# Sandlock host focused rail receipt\n\n\
         - schema: `animus.focused-rail-receipt.v1`\n\
         - status: `passed`\n\
         - cohort_version: `{COHORT_VERSION}`\n\
         - cohort_revision: `{COHORT_REVISION}`\n\
         - cohort_license: `{COHORT_LICENSE}`\n\
         - cargo_lock_blake3: `{lock_hash}`\n\
         - deterministic_tests: `passed`\n\
         - positive_and_negative_fixtures: `passed`\n\
         - default_dependency_isolation: `passed`\n\
         - non_linux_dependency_isolation: `passed`\n\
         - live_linux_behavior: `{live_status}`\n\n\
         This receipt proves only the bounded source, dependency, pure/fake, and explicitly reported live checks above. It does not prove authorization, kernel correctness, complete containment, build hermeticity, cross-platform parity, production readiness, or release eligibility.\n"
    );
    let path = root.join(RECEIPT_PATH);
    let parent = path
        .parent()
        .ok_or_else(|| "receipt path has no parent".to_owned())?;
    fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    fs::write(path, receipt).map_err(|error| error.to_string())
}

fn run_command(root: &Path, program: &str, arguments: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(arguments)
        .current_dir(root)
        .status()
        .map_err(|error| format!("failed to run {program}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("command failed: {program} {}", arguments.join(" ")))
    }
}

fn capture(root: &Path, program: &str, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(root)
        .output()
        .map_err(|error| format!("failed to run {program}: {error}"))?;
    output_text(program, arguments, output)
}

fn output_text(program: &str, arguments: &[&str], output: Output) -> Result<String, String> {
    if !output.status.success() {
        return Err(format!(
            "command failed: {program} {}\n{}",
            arguments.join(" "),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))
}

fn read_tree(root: &Path) -> Result<String, String> {
    let mut paths = Vec::new();
    collect_files(root, &mut paths)?;
    paths.sort();
    let mut combined = String::new();
    for path in paths {
        combined.push_str(
            fs::read_to_string(path.as_path())
                .map_err(|error| error.to_string())?
                .as_str(),
        );
    }
    Ok(combined)
}

fn collect_files(root: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(root).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            collect_files(path.as_path(), paths)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            paths.push(path);
        }
    }
    Ok(())
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.to_owned())
    }
}
