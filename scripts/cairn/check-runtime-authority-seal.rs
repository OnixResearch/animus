// r[verify agent_harness_runtime.runtime_authority_seal.verification]
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;
const ARG_ROOT: &str = "--root";
const RECEIPT_SCHEMA: &str = "animus.focused-rail-receipt.v1";
const RAIL_NAME: &str = "runtime-authority-seal";
const RECEIPT_PATH: &str = "target/cairn/focused-rails/runtime-authority-seal.md";
const DOC_PATH: &str = "docs/runtime-authority-seal.md";
const INVENTORY_PATH: &str = "cairn/runtime-facade-inventory.ncl";
const PURE_ROOT: &str = "crates/animus-runtime/src/authorityseal";
const RUNTIME_TEST_COUNT: usize = 10;
const HOST_TEST_COUNT: usize = 4;

const PURE_FILES: &[&str] = &["core.rs", "data.rs", "evidence.rs", "mod.rs"];
const FORBIDDEN_PURE_MARKERS: &[&str] = &[
    "std::fs",
    "std::process",
    "std::env",
    "Command::new",
    "TcpStream",
    "UdpSocket",
    "SystemTime::now",
    "Instant::now",
];
const REQUIRED_RUNTIME_TESTS: &[&str] = &[
    "authorityseal::tests::prepares_commits_runs_and_completes_one_unit",
    "authorityseal::tests::canonical_order_is_deterministic",
    "authorityseal::tests::issue_order_is_deterministic",
    "authorityseal::tests::rejects_missing_duplicate_and_bad_runtime_views",
    "authorityseal::tests::narrowing_is_allowed_but_widening_and_mutation_are_denied",
    "authorityseal::tests::receipts_reject_sensitive_data_overclaims_and_tampering",
];
const REQUIRED_HOST_TESTS: &[&str] = &[
    "runtime_authority_seal::tests::fake_dispatch_commits_seal_before_effect_and_cleans_up",
    "runtime_authority_seal::tests::bad_commit_and_denied_recheck_never_call_effect",
    "runtime_authority_seal::tests::cleanup_failure_is_reported_after_effect_completion",
    "runtime_authority_seal::tests::failed_effect_runs_cleanup_and_returns_safe_error",
];
const REQUIRED_DOC_MARKERS: &[&str] = &[
    "The seal does not grant authority.",
    "Kamacite owns the canonical application-host binding artifact.",
    "operating-system isolation",
    "effect correctness",
];
const REQUIRED_INVENTORY_MARKERS: &[&str] = &[
    "animus-runtime::runtime_authority_seal",
    "animus-std::runtime_authority_seal_host",
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
    validate_test_inventory(&root)?;
    run_checks(&root)?;
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
    let pure_root = root.join(PURE_ROOT);
    for relative in PURE_FILES {
        let path = pure_root.join(relative);
        let source = read_text(&path)?;
        for marker in FORBIDDEN_PURE_MARKERS {
            if source.contains(marker) {
                return Err(format!(
                    "pure authority-seal source contains forbidden host marker {marker}: {}",
                    path.display()
                ));
            }
        }
    }
    validate_markers(&root.join(DOC_PATH), REQUIRED_DOC_MARKERS)?;
    validate_markers(&root.join(INVENTORY_PATH), REQUIRED_INVENTORY_MARKERS)?;
    Ok(())
}

fn validate_test_inventory(root: &Path) -> Result<(), String> {
    let runtime_list = capture(
        root,
        "cargo",
        &["test", "-p", "animus-runtime", "--locked", "--", "--list"],
    )?;
    validate_output_markers(
        "runtime test inventory",
        &runtime_list,
        REQUIRED_RUNTIME_TESTS,
    )?;
    let host_list = capture(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "animus-std",
            "--features",
            "runtime-authority-seal",
            "--locked",
            "--",
            "--list",
        ],
    )?;
    validate_output_markers("host test inventory", &host_list, REQUIRED_HOST_TESTS)
}

fn run_checks(root: &Path) -> Result<(), String> {
    run_command(root, "cargo", &["fmt", "--all", "--", "--check"])?;
    run_command(
        root,
        "cargo",
        &["test", "-p", "animus-runtime", "authorityseal", "--locked"],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "animus-std",
            "--features",
            "runtime-authority-seal",
            "runtime_authority",
            "--locked",
        ],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "clippy",
            "-p",
            "animus-runtime",
            "--all-targets",
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
            "clippy",
            "-p",
            "animus-std",
            "--features",
            "runtime-authority-seal",
            "--all-targets",
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
    )
}

fn write_receipt(root: &Path) -> Result<(), String> {
    let source_hash = hash_files(
        root,
        PURE_FILES
            .iter()
            .map(|file| Path::new(PURE_ROOT).join(file)),
    )?;
    let evidence_hash = hash_files(
        root,
        [PathBuf::from(DOC_PATH), PathBuf::from(INVENTORY_PATH)],
    )?;
    let receipt = format!(
        "schema: {RECEIPT_SCHEMA}\nrail: {RAIL_NAME}\nstatus: pass\nruntime_source_set_blake3: {source_hash}\nevidence_source_set_blake3: {evidence_hash}\nruntime_tests: {RUNTIME_TEST_COUNT}\nhost_tests: {HOST_TEST_COUNT}\nclaims: deterministic-unit-identity,exact-runtime-view,receipt-before-effect,current-recheck,monotonic-authority,terminal-cleanup,metadata-only-evidence\nnon_claims: authorization,operating-system-isolation,hidden-authority-absence,sandbox-correctness,host-integrity,effect-correctness,release-eligibility\n"
    );
    let path = root.join(RECEIPT_PATH);
    let parent = path
        .parent()
        .ok_or_else(|| String::from("receipt path has no parent"))?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("failed to create receipt directory: {error}"))?;
    fs::write(&path, receipt)
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}

fn hash_files(root: &Path, paths: impl IntoIterator<Item = PathBuf>) -> Result<String, String> {
    let mut paths = paths.into_iter().collect::<Vec<_>>();
    paths.sort();
    let mut bytes = Vec::new();
    for relative in paths {
        bytes.extend_from_slice(relative.to_string_lossy().as_bytes());
        bytes.push(0);
        bytes.extend_from_slice(
            &fs::read(root.join(&relative))
                .map_err(|error| format!("failed to read {}: {error}", relative.display()))?,
        );
        bytes.push(0);
    }
    let mut child = Command::new("b3sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| format!("failed to start b3sum: {error}"))?;
    child
        .stdin
        .take()
        .ok_or_else(|| String::from("b3sum stdin is unavailable"))?
        .write_all(&bytes)
        .map_err(|error| format!("failed to write b3sum input: {error}"))?;
    let output = child
        .wait_with_output()
        .map_err(|error| format!("failed to wait for b3sum: {error}"))?;
    if !output.status.success() {
        return Err(String::from("b3sum failed"));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| format!("b3sum returned invalid UTF-8: {error}"))?
        .split_whitespace()
        .next()
        .map(String::from)
        .ok_or_else(|| String::from("b3sum returned no identity"))
}

fn validate_markers(path: &Path, markers: &[&str]) -> Result<(), String> {
    let text = read_text(path)?;
    validate_output_markers(path.to_string_lossy().as_ref(), &text, markers)
}

fn validate_output_markers(subject: &str, text: &str, markers: &[&str]) -> Result<(), String> {
    for marker in markers {
        if !text.contains(marker) {
            return Err(format!("{subject} is missing required marker: {marker}"));
        }
    }
    Ok(())
}

fn read_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn capture(root: &Path, program: &str, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(arguments)
        .current_dir(root)
        .output()
        .map_err(|error| format!("failed to start {program}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "{program} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| format!("{program} returned invalid UTF-8: {error}"))
}

fn run_command(root: &Path, program: &str, arguments: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(arguments)
        .current_dir(root)
        .status()
        .map_err(|error| format!("failed to start {program}: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("{program} failed with status {status}"))
    }
}
