use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const RECEIPT_PATH: &str = "target/cairn/focused-rails/lean4-loop-fabric.md";
const STATUS_PASSED: &str = "passed";
const LEAN_DIR: &str = "lean/animus-lean";
const LEAN_EXECUTABLE: &str = "lean/animus-lean/.lake/build/bin/animus_lean";
const PROFILE_SOURCE: &str = "config/lean-loop-profile.ncl";
const PROFILE_GENERATED: &str = "config/generated/lean-loop-profile.json";
const NEGATIVE_CASES: &str = "config/lean-loop-profile/negative-cases.ncl";
const PROOF_TRUST: &str = "lean/animus-lean/ProofTrust.lean";
const CORE_LEAN: &str = "lean/animus-lean/AnimusLean/Core.lean";
const PROTOCOL_LEAN: &str = "lean/animus-lean/AnimusLean/Protocol.lean";
const SHELL_LEAN: &str = "lean/animus-lean/Main.lean";
const BRIDGE_CRATE: &str = "crates/animus-std-lean-loop";
const PROFILE_IDENTITY_DOMAIN: &str = "animus.lean-loop.profile.v1";
const SOURCE_IDENTITY_DOMAIN: &str = "animus.lean-loop.focused-source.v1";
const PROOF_SOURCE_IDENTITY_DOMAIN: &str = "animus.lean-loop.proof-source.v1";
const PROOF_SOURCE_PATHS: &[&str] = &[CORE_LEAN, PROTOCOL_LEAN, PROOF_TRUST];
const SOURCE_IDENTITY_PATHS: &[&str] = &[
    "Cargo.lock",
    "Cargo.toml",
    PROFILE_SOURCE,
    PROFILE_GENERATED,
    NEGATIVE_CASES,
    "lean/animus-lean/lakefile.lean",
    "lean/animus-lean/lake-manifest.json",
    "lean/animus-lean/lean-toolchain",
    "lean/animus-lean/AnimusLean.lean",
    CORE_LEAN,
    PROTOCOL_LEAN,
    SHELL_LEAN,
    "lean/animus-lean/Test.lean",
    PROOF_TRUST,
    "crates/animus-std-lean-loop/Cargo.toml",
    "crates/animus-std-lean-loop/src/lib.rs",
    "crates/animus-std-lean-loop/src/model.rs",
    "crates/animus-std-lean-loop/src/core.rs",
    "crates/animus-std-lean-loop/src/protocol.rs",
    "crates/animus-std-lean-loop/src/transport.rs",
    "docs/lean4-loop-fabric.md",
    "scripts/cairn/check-lean4-loop-fabric",
    "scripts/cairn/check-lean4-loop-fabric.rs",
    "flake.nix",
];
const REQUIRED_NEGATIVE_CASES: &[&str] = &[
    "unknown-protocol",
    "unknown-toolchain",
    "non-positive-bound",
    "inconsistent-bounds",
    "unknown-decision",
    "ambient-executable-discovery",
    "unrestricted-environment",
    "implicit-fallback",
    "missing-sandbox-posture",
    "stale-source-hash",
    "unsafe-label",
    "weakened-redaction",
    "weakened-non-claims",
];
const REQUIRED_THEOREMS: &[&str] = &[
    "deterministic_transition",
    "closed_decision_set",
    "consume_step_nonincreasing",
    "terminal_absorption",
    "invalid_profile_fails_closed",
    "reference_initial_phase",
    "reference_model_then_respond",
];
const ALLOWED_AXIOMS: &[&str] = &["propext", "Classical.choice", "Quot.sound"];
const FORBIDDEN_PROOF_TOKENS: &[&str] = &["sorry", "admit", "axiom "];
const FORBIDDEN_PURE_TOKENS: &[&str] = &[
    "IO.",
    "getEnv",
    "readFile",
    "writeFile",
    "Process",
    "Socket",
    "System.FilePath",
];
const FORBIDDEN_SHELL_TOKENS: &[&str] = &[
    "getEnv",
    "readFile",
    "writeFile",
    "Socket",
    "Process.spawn",
    "System.FilePath",
];
const FORBIDDEN_BRIDGE_TOKENS: &[&str] = &[
    "unsafe fn",
    "unsafe impl",
    "unsafe {",
    "Command::new(\"",
    "TcpStream",
    "UdpSocket",
    "reqwest",
    "hyper::",
    "std::env::var",
    "fallback_to_steel",
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
    validate_sources(&root)?;
    validate_profile_freshness(&root)?;
    run_lean_checks(&root)?;
    run_rust_checks(&root)?;
    validate_dependency_isolation(&root)?;
    let source_identity = aggregate_identity(&root, SOURCE_IDENTITY_DOMAIN, SOURCE_IDENTITY_PATHS)?;
    let proof_source_identity = aggregate_identity(
        &root,
        PROOF_SOURCE_IDENTITY_DOMAIN,
        PROOF_SOURCE_PATHS,
    )?;
    validate_proof_source_identity(&root, proof_source_identity.as_str())?;
    let profile_identity = domain_identity(
        &root,
        PROFILE_IDENTITY_DOMAIN,
        root.join(PROFILE_GENERATED).as_path(),
    )?;
    let artifact_identity = domain_identity(
        &root,
        "animus.lean-loop.artifact.v1",
        root.join(LEAN_EXECUTABLE).as_path(),
    )?;
    write_receipt(
        &root,
        source_identity.as_str(),
        proof_source_identity.as_str(),
        profile_identity.as_str(),
        artifact_identity.as_str(),
    )?;
    println!("focused rail lean4-loop-fabric: pass");
    println!("receipt: {}", root.join(RECEIPT_PATH).display());
    Ok(())
}

fn parse_root() -> Result<PathBuf, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [flag, value] if flag == "--root" => Ok(PathBuf::from(value)),
        [] => env::current_dir().map_err(|error| error.to_string()),
        _other => Err("usage: check-lean4-loop-fabric [--root PATH]".to_owned()),
    }
}

fn validate_sources(root: &Path) -> Result<(), String> {
    for path in SOURCE_IDENTITY_PATHS {
        require(root.join(path).is_file(), format!("missing required source {path}"))?;
    }
    let core = read(root, CORE_LEAN)?;
    let protocol = read(root, PROTOCOL_LEAN)?;
    let shell = read(root, SHELL_LEAN)?;
    let bridge = read_tree(root.join(BRIDGE_CRATE).join("src").as_path())?;
    let negatives = read(root, NEGATIVE_CASES)?;
    for theorem in REQUIRED_THEOREMS {
        require(
            core.contains(format!("theorem {theorem}").as_str()),
            format!("missing reviewed theorem {theorem}"),
        )?;
        require(
            read(root, PROOF_TRUST)?.contains(format!("AnimusLean.{theorem}").as_str()),
            format!("missing theorem trust query {theorem}"),
        )?;
    }
    for token in FORBIDDEN_PROOF_TOKENS {
        require(
            !core.contains(token),
            format!("proof source contains forbidden token {token}"),
        )?;
    }
    for token in FORBIDDEN_PURE_TOKENS {
        require(
            !core.contains(token) && !protocol.contains(token),
            format!("pure Lean module contains effect token {token}"),
        )?;
    }
    for token in FORBIDDEN_SHELL_TOKENS {
        require(
            !shell.contains(token),
            format!("Lean shell contains ambient token {token}"),
        )?;
    }
    for token in FORBIDDEN_BRIDGE_TOKENS {
        require(
            !bridge.contains(token),
            format!("Rust bridge contains forbidden authority token {token}"),
        )?;
    }
    for case in REQUIRED_NEGATIVE_CASES {
        require(
            negatives.contains(format!("id = \"{case}\"").as_str()),
            format!("missing negative profile case {case}"),
        )?;
    }
    Ok(())
}

fn validate_proof_source_identity(root: &Path, expected: &str) -> Result<(), String> {
    let generated = read(root, PROFILE_GENERATED)?;
    let model = read(root, "crates/animus-std-lean-loop/src/model.rs")?;
    let expected_field = format!("\"proof_source_identity\": \"{expected}\"");
    require(
        generated.contains(expected_field.as_str()),
        "generated profile proof-source identity is stale".to_owned(),
    )?;
    require(
        model.contains(expected),
        "compiled Rust proof-source identity is stale".to_owned(),
    )?;
    require(
        generated.contains(PROOF_SOURCE_IDENTITY_DOMAIN),
        "generated profile proof-source domain is stale".to_owned(),
    )
}

fn validate_profile_freshness(root: &Path) -> Result<(), String> {
    let output = Command::new("nickel")
        .current_dir(root)
        .args([
            "export",
            "--format",
            "json",
            "--field",
            "lean_loop_profile",
            PROFILE_SOURCE,
        ])
        .output()
        .map_err(|error| format!("cannot run Nickel profile export: {error}"))?;
    require(output.status.success(), "Nickel profile export failed".to_owned())?;
    let checked = fs::read(root.join(PROFILE_GENERATED))
        .map_err(|error| format!("cannot read generated Lean profile: {error}"))?;
    require(
        output.stdout == checked,
        "generated Lean profile is stale".to_owned(),
    )
}

fn run_lean_checks(root: &Path) -> Result<(), String> {
    let lean_root = root.join(LEAN_DIR);
    run_command(
        lean_root.as_path(),
        "lake",
        &["build", "AnimusLean", "animus_lean", "animus_lean_tests"],
        &[],
    )?;
    run_command(
        lean_root.as_path(),
        ".lake/build/bin/animus_lean_tests",
        &[],
        &[],
    )?;
    let output = Command::new("lake")
        .current_dir(lean_root.as_path())
        .args(["env", "lean", "ProofTrust.lean"])
        .output()
        .map_err(|error| format!("cannot run Lean proof trust check: {error}"))?;
    require(output.status.success(), "Lean proof trust command failed".to_owned())?;
    let rendered = String::from_utf8(output.stdout)
        .map_err(|error| format!("proof trust output is not UTF-8: {error}"))?;
    for theorem in REQUIRED_THEOREMS {
        let line = rendered
            .lines()
            .find(|line| line.contains(format!("AnimusLean.{theorem}").as_str()))
            .ok_or_else(|| format!("missing proof trust output for {theorem}"))?;
        let open = line
            .find('[')
            .ok_or_else(|| format!("missing axiom list for {theorem}"))?;
        let close = line
            .rfind(']')
            .ok_or_else(|| format!("missing axiom list close for {theorem}"))?;
        let axioms = &line[open + 1..close];
        for axiom in axioms.split(',').map(str::trim).filter(|value| !value.is_empty()) {
            require(
                ALLOWED_AXIOMS.contains(&axiom),
                format!("unexpected theorem axiom {axiom} for {theorem}"),
            )?;
        }
    }
    Ok(())
}

fn run_rust_checks(root: &Path) -> Result<(), String> {
    run_command(
        root,
        "cargo",
        &["test", "-p", "animus-std-lean-loop", "--all-targets", "--locked"],
        &[],
    )?;
    let executable = root.join(LEAN_EXECUTABLE);
    run_command(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "animus-std-lean-loop",
            "--test",
            "lean_child",
            "--locked",
            "--",
            "--ignored",
            "--exact",
            "repository_lean_child_runs_model_then_response_protocol",
        ],
        &[("ANIMUS_LEAN_EXECUTABLE", executable.to_string_lossy().as_ref())],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "clippy",
            "-p",
            "animus-std-lean-loop",
            "--all-targets",
            "--no-deps",
            "--",
            "-D",
            "warnings",
        ],
        &[],
    )?;
    run_command(
        root,
        "cargo",
        &["test", "-p", "animus-runtime", "fabric", "--locked"],
        &[],
    )?;
    run_command(
        root,
        "cargo",
        &["check", "-p", "animus", "--features", "lean-loop", "--locked"],
        &[],
    )?;
    run_command(root, "cargo", &["check", "-p", "animus", "--locked"], &[])?;
    run_command(
        root,
        "cargo",
        &["check", "-p", "animus", "--no-default-features", "--locked"],
        &[],
    )
}

fn validate_dependency_isolation(root: &Path) -> Result<(), String> {
    for args in [
        ["tree", "-p", "animus", "--edges", "normal", "--prefix", "none"].as_slice(),
        [
            "tree",
            "-p",
            "animus",
            "--no-default-features",
            "--edges",
            "normal",
            "--prefix",
            "none",
        ]
        .as_slice(),
    ] {
        let output = Command::new("cargo")
            .current_dir(root)
            .args(args)
            .output()
            .map_err(|error| format!("cannot run dependency isolation check: {error}"))?;
        require(output.status.success(), "cargo tree failed".to_owned())?;
        let tree = String::from_utf8(output.stdout)
            .map_err(|error| format!("cargo tree output is not UTF-8: {error}"))?;
        require(
            !tree.contains("animus-std-lean-loop")
                && !tree.contains("lean4")
                && !tree.contains(" lake "),
            "default/no-default graph acquired Lean loop dependencies".to_owned(),
        )?;
    }
    run_command(
        root,
        "cargo",
        &[
            "check",
            "-p",
            "animus",
            "--no-default-features",
            "--target",
            "wasm32-unknown-unknown",
            "--locked",
        ],
        &[],
    )
}

fn run_command(
    current_dir: &Path,
    program: &str,
    args: &[&str],
    environment: &[(&str, &str)],
) -> Result<(), String> {
    println!("running: {program} {}", args.join(" "));
    let mut command = Command::new(program);
    command.current_dir(current_dir).args(args);
    for (key, value) in environment {
        command.env(key, value);
    }
    let status = command
        .status()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    require(status.success(), format!("command failed: {program}"))
}

fn aggregate_identity(root: &Path, domain: &str, paths: &[&str]) -> Result<String, String> {
    let mut first = Command::new("b3sum");
    first.current_dir(root).args(paths);
    let first_output = first
        .output()
        .map_err(|error| format!("cannot run source b3sum: {error}"))?;
    require(
        first_output.status.success(),
        format!(
            "source b3sum failed: {}",
            String::from_utf8_lossy(first_output.stderr.as_slice()).trim()
        ),
    )?;
    let mut second = Command::new("b3sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot start aggregate b3sum: {error}"))?;
    let input = second
        .stdin
        .as_mut()
        .ok_or_else(|| "aggregate b3sum stdin unavailable".to_owned())?;
    input
        .write_all(domain.as_bytes())
        .and_then(|()| input.write_all(&[0]))
        .and_then(|()| input.write_all(first_output.stdout.as_slice()))
        .map_err(|error| format!("cannot write aggregate b3sum input: {error}"))?;
    drop(second.stdin.take());
    let output = second
        .wait_with_output()
        .map_err(|error| format!("cannot finish aggregate b3sum: {error}"))?;
    require(output.status.success(), "aggregate b3sum failed".to_owned())?;
    first_word(output.stdout.as_slice(), "aggregate source identity")
}

fn domain_identity(root: &Path, domain: &str, path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    let mut child = Command::new("b3sum")
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot start domain b3sum: {error}"))?;
    let input = child
        .stdin
        .as_mut()
        .ok_or_else(|| "domain b3sum stdin unavailable".to_owned())?;
    input
        .write_all(domain.as_bytes())
        .and_then(|()| input.write_all(&[0]))
        .and_then(|()| input.write_all(bytes.as_slice()))
        .map_err(|error| format!("cannot write domain b3sum input: {error}"))?;
    drop(child.stdin.take());
    let output = child
        .wait_with_output()
        .map_err(|error| format!("cannot finish domain b3sum: {error}"))?;
    require(output.status.success(), "domain b3sum failed".to_owned())?;
    first_word(output.stdout.as_slice(), "domain identity")
}

fn first_word(bytes: &[u8], label: &str) -> Result<String, String> {
    String::from_utf8(bytes.to_vec())
        .map_err(|error| format!("{label} is not UTF-8: {error}"))?
        .split_whitespace()
        .next()
        .map(str::to_owned)
        .ok_or_else(|| format!("{label} output is empty"))
}

fn write_receipt(
    root: &Path,
    source_identity: &str,
    proof_source_identity: &str,
    profile_identity: &str,
    artifact_identity: &str,
) -> Result<(), String> {
    let path = root.join(RECEIPT_PATH);
    let parent = path
        .parent()
        .ok_or_else(|| "focused receipt parent unavailable".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("cannot create receipt directory: {error}"))?;
    let receipt = format!(
        "# Lean 4 loop fabric focused rail\n\n- schema: animus.focused-rail.v1\n- status: {STATUS_PASSED}\n- source identity (domain-separated BLAKE3): {source_identity}\n- proof source identity (domain-separated BLAKE3): {proof_source_identity}\n- profile identity (domain-separated BLAKE3): {profile_identity}\n- executable identity (domain-separated BLAKE3): {artifact_identity}\n- toolchain cohort: lean4-4.30.0\n- pure Lean kernel and reference policy: passed\n- named theorem suite and reviewed axiom allowlist: passed\n- bounded JSON-line protocol and actual Lean child: passed\n- Rust current-shape decision revalidation: passed\n- fixture-child success and failure classes: passed\n- default/no-std/Wasm dependency isolation: passed\n- metadata-only receipt leak rejection: passed\n- implicit restart, replay, Steel fallback, FFI, sockets, and network: absent\n- arbitrary policy correctness: not claimed\n- Lean compiler/runtime soundness: not claimed\n- child confinement and external effects: not claimed\n- atomic executable path pinning: not claimed\n- theorem-to-Rust equivalence: not claimed\n- authoritative completion and production readiness: not claimed\n"
    );
    fs::write(path, receipt).map_err(|error| format!("cannot write focused receipt: {error}"))
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))
}

fn read_tree(path: &Path) -> Result<String, String> {
    let mut entries = fs::read_dir(path)
        .map_err(|error| format!("cannot read source tree {}: {error}", path.display()))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| format!("cannot enumerate source tree: {error}"))?;
    entries.sort_by_key(std::fs::DirEntry::file_name);
    let mut rendered = String::new();
    for entry in entries {
        let entry_path = entry.path();
        if entry_path.is_dir() {
            rendered.push_str(read_tree(entry_path.as_path())?.as_str());
        } else if entry_path.extension().and_then(std::ffi::OsStr::to_str) == Some("rs") {
            rendered.push_str(
                fs::read_to_string(entry_path.as_path())
                    .map_err(|error| format!("cannot read {}: {error}", entry_path.display()))?
                    .as_str(),
            );
        }
    }
    Ok(rendered)
}

fn require(condition: bool, message: String) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message) }
}
