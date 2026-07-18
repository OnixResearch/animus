use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const STATUS_PASSED: &str = "passed";
const STATUS_BLOCKED: &str = "blocked";
const RECEIPT: &str = "target/cairn/focused-rails/agent-guardrail.md";
const PROFILE_SOURCE: &str = "config/agent-guardrail-profile.ncl";
const PROFILE_GENERATED: &str = "config/generated/agent-guardrail-profile.json";
const NEGATIVE_SOURCE: &str = "config/agent-guardrail-profile/negative-cases.ncl";
const NEGATIVE_GENERATED: &str = "config/generated/agent-guardrail-negative-cases.json";
const SOURCE_DOMAIN: &str = "animus.agent-guardrail.focused-source.v1";
const LIVE_ENVIRONMENT: &str = "ANIMUS_GUARDRAIL_LINUX_LIVE";
const BWRAP_ENVIRONMENT: &str = "ANIMUS_GUARDRAIL_BWRAP";
const SOURCE_PATHS: &[&str] = &[
    "Cargo.lock",
    "Cargo.toml",
    PROFILE_SOURCE,
    PROFILE_GENERATED,
    NEGATIVE_SOURCE,
    NEGATIVE_GENERATED,
    "crates/animus-std-agent-guardrail/Cargo.toml",
    "crates/animus-std-agent-guardrail/src/lib.rs",
    "crates/animus-std-agent-guardrail/src/model.rs",
    "crates/animus-std-agent-guardrail/src/core.rs",
    "crates/animus-std-agent-guardrail/src/shell.rs",
    "crates/animus-std-agent-guardrail/src/transport.rs",
    "crates/animus-std-agent-guardrail/src/bin/animus-agent-guardrail.rs",
    "crates/animus-std-agent-guardrail/src/bin/animus-fake-bwrap.rs",
    "crates/animus-std-agent-guardrail/src/bin/animus-guardrail-live-fixture.rs",
    "crates/animus-std-agent-guardrail/tests/fake_bwrap.rs",
    "crates/animus-std-agent-guardrail/tests/live_linux.rs",
    "docs/agent-guardrail.md",
    "scripts/cairn/check-agent-guardrail",
    "scripts/cairn/check-agent-guardrail.rs",
    "flake.nix",
];
const REQUIRED_NEGATIVES: &[&str] = &[
    "unknown-cohort",
    "project-local-policy",
    "implicit-writable",
    "broad-devices",
    "host-sockets",
    "filtered-network-claim",
    "raw-arguments",
    "command-prefix",
    "passthrough",
    "command-source",
    "implicit-shell",
    "unguarded-fallback",
    "privileged-sidecar",
    "zero-bound",
    "unsafe-label",
    "weakened-redaction",
    "weakened-non-claims",
];
const FORBIDDEN_CORE: &[&str] = &[
    "std::fs::",
    "std::env::",
    "Command::",
    "TcpStream",
    "UdpSocket",
    "unsafe fn",
    "unsafe impl",
    "unsafe {",
];
const FORBIDDEN_PRODUCTION: &[&str] = &[
    "Command::new(\"bwrap\")",
    "fallback_to_argv",
    "project_policy_discovery",
    "serde_yaml",
    "raw_bwrap_arguments",
    "unsafe fn",
    "unsafe impl",
    "unsafe {",
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
    validate_nickel(&root)?;
    run_rust_checks(&root)?;
    validate_isolation(&root)?;
    let live_status = run_live_if_requested(&root)?;
    let source_identity = aggregate_identity(&root, SOURCE_DOMAIN, SOURCE_PATHS)?;
    write_receipt(&root, source_identity.as_str(), live_status)?;
    println!("focused rail agent-guardrail: pass");
    println!("live Linux behavior: {live_status}");
    println!("receipt: {}", root.join(RECEIPT).display());
    Ok(())
}

fn parse_root() -> Result<PathBuf, String> {
    let arguments = env::args().skip(1).collect::<Vec<_>>();
    match arguments.as_slice() {
        [flag, value] if flag == "--root" => Ok(PathBuf::from(value)),
        [] => env::current_dir().map_err(|error| error.to_string()),
        _other => Err("usage: check-agent-guardrail [--root PATH]".to_owned()),
    }
}

fn validate_sources(root: &Path) -> Result<(), String> {
    for path in SOURCE_PATHS {
        require(
            root.join(path).is_file(),
            format!("required guardrail source missing: {path}"),
        )?;
    }
    let core = read(root, "crates/animus-std-agent-guardrail/src/core.rs")?;
    for token in FORBIDDEN_CORE {
        require(
            !core.contains(token),
            format!("pure guardrail core contains forbidden token: {token}"),
        )?;
    }
    let production = [
        read(root, "crates/animus-std-agent-guardrail/src/lib.rs")?,
        read(root, "crates/animus-std-agent-guardrail/src/model.rs")?,
        core,
        read(root, "crates/animus-std-agent-guardrail/src/shell.rs")?,
        read(root, "crates/animus-std-agent-guardrail/src/transport.rs")?,
        read(
            root,
            "crates/animus-std-agent-guardrail/src/bin/animus-agent-guardrail.rs",
        )?,
    ]
    .join("\n");
    for token in FORBIDDEN_PRODUCTION {
        require(
            !production.contains(token),
            format!("guardrail production source contains forbidden token: {token}"),
        )?;
    }
    require(
        production.contains("--args")
            && production.contains("MemfdFlags::ALLOW_SEALING")
            && production.contains("SealFlags::SHRINK")
            && production.contains("--remount-ro")
            && production.contains("env_clear"),
        "sealed transport or private-root enforcement source shape is incomplete".to_owned(),
    )?;
    let manifest = read(root, "Cargo.toml")?;
    let default_members = manifest
        .split("default-members = [")
        .nth(1)
        .and_then(|suffix| suffix.split(']').next())
        .ok_or_else(|| "default-members block missing".to_owned())?;
    require(
        !default_members.contains("animus-std-agent-guardrail"),
        "guardrail crate entered default members".to_owned(),
    )?;
    Ok(())
}

fn validate_nickel(root: &Path) -> Result<(), String> {
    let profile = command_output(
        root,
        "nickel",
        &[
            "export",
            "--format",
            "json",
            "--field",
            "agent_guardrail_profile",
            PROFILE_SOURCE,
        ],
    )?;
    let expected = fs::read(root.join(PROFILE_GENERATED))
        .map_err(|error| format!("cannot read generated profile: {error}"))?;
    require(profile == expected, "guardrail profile projection is stale".to_owned())?;
    let negative = command_output(
        root,
        "nickel",
        &["export", "--format", "json", NEGATIVE_SOURCE],
    )?;
    let expected_negative = fs::read(root.join(NEGATIVE_GENERATED))
        .map_err(|error| format!("cannot read generated negative cases: {error}"))?;
    require(
        String::from_utf8_lossy(negative.as_slice()).trim()
            == String::from_utf8_lossy(expected_negative.as_slice()).trim(),
        format!(
            "guardrail negative-case projection is stale: exported={} expected={}",
            negative.len(),
            expected_negative.len()
        ),
    )?;
    let negative_text = String::from_utf8(negative)
        .map_err(|error| format!("negative cases are not UTF-8: {error}"))?;
    for case in REQUIRED_NEGATIVES {
        require(
            negative_text.contains(case),
            format!("required guardrail negative case missing: {case}"),
        )?;
    }
    Ok(())
}

fn run_rust_checks(root: &Path) -> Result<(), String> {
    run_command(
        root,
        "cargo",
        &["test", "-p", "animus-std-agent-guardrail", "--all-targets", "--locked"],
        &[],
    )?;
    run_command(
        root,
        "cargo",
        &[
            "clippy",
            "-p",
            "animus-std-agent-guardrail",
            "--all-targets",
            "--no-deps",
            "--locked",
            "--",
            "-D",
            "warnings",
        ],
        &[],
    )?;
    run_command(
        root,
        "cargo",
        &["check", "-p", "animus", "--features", "agent-guardrail", "--locked"],
        &[],
    )?;
    run_command(root, "cargo", &["check", "-p", "animus", "--locked"], &[])?;
    run_command(
        root,
        "cargo",
        &["check", "-p", "animus", "--no-default-features", "--locked"],
        &[],
    )?;
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

fn validate_isolation(root: &Path) -> Result<(), String> {
    for arguments in [
        vec!["tree", "-p", "animus", "-e", "normal", "--locked"],
        vec![
            "tree",
            "-p",
            "animus",
            "-e",
            "normal",
            "--no-default-features",
            "--target",
            "wasm32-unknown-unknown",
            "--locked",
        ],
    ] {
        let output = command_output(root, "cargo", arguments.as_slice())?;
        let tree = String::from_utf8(output)
            .map_err(|error| format!("cargo tree output is not UTF-8: {error}"))?;
        require(
            !tree.contains("animus-std-agent-guardrail"),
            "guardrail dependency leaked into default or Wasm surface".to_owned(),
        )?;
    }
    Ok(())
}

fn run_live_if_requested(root: &Path) -> Result<&'static str, String> {
    if env::var(LIVE_ENVIRONMENT).ok().as_deref() != Some("1") {
        return Ok(STATUS_BLOCKED);
    }
    let bwrap = env::var(BWRAP_ENVIRONMENT)
        .map_err(|_error| format!("{BWRAP_ENVIRONMENT} is required for live evidence"))?;
    require(
        Path::new(bwrap.as_str()).is_absolute(),
        "live Bubblewrap path must be absolute".to_owned(),
    )?;
    run_command(
        root,
        "cargo",
        &[
            "test",
            "-p",
            "animus-std-agent-guardrail",
            "--test",
            "live_linux",
            "--locked",
            "--",
            "--ignored",
        ],
        &[(BWRAP_ENVIRONMENT, bwrap.as_str())],
    )?;
    Ok(STATUS_PASSED)
}

fn run_command(
    root: &Path,
    program: &str,
    arguments: &[&str],
    environment: &[(&str, &str)],
) -> Result<(), String> {
    println!("running: {program} {}", arguments.join(" "));
    let mut command = Command::new(program);
    command.current_dir(root).args(arguments);
    for (key, value) in environment {
        command.env(key, value);
    }
    let status = command
        .status()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    require(status.success(), format!("command failed: {program}"))
}

fn command_output(root: &Path, program: &str, arguments: &[&str]) -> Result<Vec<u8>, String> {
    let output = Command::new(program)
        .current_dir(root)
        .args(arguments)
        .output()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    require(
        output.status.success(),
        format!(
            "command failed: {program}: {}",
            String::from_utf8_lossy(output.stderr.as_slice()).trim()
        ),
    )?;
    Ok(output.stdout)
}

fn aggregate_identity(root: &Path, domain: &str, paths: &[&str]) -> Result<String, String> {
    let first = Command::new("b3sum")
        .current_dir(root)
        .args(paths)
        .output()
        .map_err(|error| format!("cannot run source b3sum: {error}"))?;
    require(first.status.success(), "source b3sum failed".to_owned())?;
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
        .and_then(|()| input.write_all(first.stdout.as_slice()))
        .map_err(|error| format!("cannot write aggregate b3sum input: {error}"))?;
    drop(second.stdin.take());
    let output = second
        .wait_with_output()
        .map_err(|error| format!("cannot finish aggregate b3sum: {error}"))?;
    require(output.status.success(), "aggregate b3sum failed".to_owned())?;
    String::from_utf8(output.stdout)
        .map_err(|error| format!("aggregate identity is not UTF-8: {error}"))?
        .split_whitespace()
        .next()
        .map(str::to_owned)
        .ok_or_else(|| "aggregate identity output is empty".to_owned())
}

fn write_receipt(root: &Path, source_identity: &str, live_status: &str) -> Result<(), String> {
    let path = root.join(RECEIPT);
    fs::create_dir_all(
        path.parent()
            .ok_or_else(|| "focused receipt parent unavailable".to_owned())?,
    )
    .map_err(|error| format!("cannot create focused receipt directory: {error}"))?;
    let receipt = format!(
        "# Bubblewrap agent guardrail focused rail\n\n- schema: animus.focused-rail.v1\n- status: {STATUS_PASSED}\n- source identity (domain-separated BLAKE3): {source_identity}\n- checked Nickel profile and negative cases: passed\n- pure deterministic plan, authorization, preflight, and receipt tests: passed\n- sealed memfd and fake Bubblewrap transport tests: passed\n- default/no-default/Wasm dependency isolation: passed\n- real Linux Bubblewrap behavior: {live_status}\n- raw argv, project policy, passthrough, command prefix, implicit shell, privileged sidecar, and unguarded fallback: absent\n- hostile-code containment, kernel correctness, same-UID secrecy, filtered egress, authorization, build hermeticity, TOCTOU-safe path resolution, production and release readiness: not claimed\n"
    );
    fs::write(path, receipt).map_err(|error| format!("cannot write focused receipt: {error}"))
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))
}

fn require(condition: bool, message: String) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message) }
}
