use std::env;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Stdio};

const EXIT_FAILURE: u8 = 1;
const EXIT_SUCCESS: u8 = 0;
const ARG_ROOT: &str = "--root";
const FEATURE_NAME: &str = "acp-stdio-host";
const ACP_PACKAGE: &str = "agent-client-protocol";
const ACP_VERSION: &str = "0.10.4";
const ACP_SCHEMA_PACKAGE: &str = "agent-client-protocol-schema";
const ACP_SCHEMA_VERSION: &str = "0.11.4";
const ACP_LICENSE: &str = "Apache-2.0";
const RECEIPT_PATH: &str = "target/cairn/focused-rails/acp-stdio-host.md";
const ROOT_MANIFEST: &str = "Cargo.toml";
const ACP_MANIFEST: &str = "crates/animus-acp-stdio-host/Cargo.toml";
const LOCK_PATH: &str = "Cargo.lock";
const ACP_SOURCE_ROOT: &str = "crates/animus-acp-stdio-host/src";
const CONTRACT_DOC: &str = "docs/acp-stdio-host.md";
const DEFAULT_MEMBERS_START: &str = "default-members = [";
const SECTION_END: &str = "]";
const DEPENDENCY_NEEDLE: &str = "agent-client-protocol v0.10.4";
const RECEIPT_SCHEMA: &str = "animus.focused-rail.v1";
const STATUS_PASSED: &str = "passed";
const REUSABLE_ROOTS: &[&str] = &[
    "crates/animus-core",
    "crates/animus-adapters",
    "crates/animus-runtime",
    "crates/animus-std",
    "crates/animus-tui",
];
const FORBIDDEN_ACP_SOURCE: &[&str] = &[
    "TcpListener",
    "TcpStream",
    "UdpSocket",
    "tokio::net",
    "reqwest",
    "hyper::",
    "std::env",
    "http://",
    "https://",
];
const REQUIRED_SOURCE_MARKERS: &[&str] = &[
    "decode_acp_frame",
    "reduce_acp_request",
    "reduce_acp_outcome",
    "plan_acp_permission",
    "project_runtime_events",
    "project_acp_receipt",
    "read_bounded_line",
    "CodingSessionEffectGate",
    "AcpTransportReport",
    "ConfirmationEof",
    "BrokenPipe",
    "CleanupFailed",
];
const REQUIRED_DOC_MARKERS: &[&str] = &[
    ACP_VERSION,
    ACP_SCHEMA_VERSION,
    ACP_LICENSE,
    "local-stdio-only",
    "additional gate only",
    "does **not** prove arbitrary-editor interoperability",
];
const SOURCE_IDENTITY_PATHS: &[&str] = &[
    ROOT_MANIFEST,
    ACP_MANIFEST,
    LOCK_PATH,
    "crates/animus-acp-stdio-host/src/codec.rs",
    "crates/animus-acp-stdio-host/src/core.rs",
    "crates/animus-acp-stdio-host/src/events.rs",
    "crates/animus-acp-stdio-host/src/facade.rs",
    "crates/animus-acp-stdio-host/src/lib.rs",
    "crates/animus-acp-stdio-host/src/model.rs",
    "crates/animus-acp-stdio-host/src/permission.rs",
    "crates/animus-acp-stdio-host/src/receipt.rs",
    "crates/animus-acp-stdio-host/src/transport.rs",
    "crates/animus-std/src/codingsession.rs",
    CONTRACT_DOC,
    "cairn/component-catalog.ncl",
    "cairn/runtime-facade-inventory.ncl",
    "scripts/cairn/check-acp-stdio-host",
    "scripts/cairn/check-acp-stdio-host.rs",
    "flake.nix",
];

struct Check<'a> {
    program: &'a str,
    args: &'a [&'a str],
}

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
    validate_contract(&root)?;
    validate_dependency_isolation(&root)?;
    run_checks(&root)?;
    let source_identity = compute_source_identity(&root)?;
    write_receipt(&root, source_identity.as_str())?;
    println!("focused rail acp-stdio-host: pass");
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
        _other => Err("usage: check-acp-stdio-host [--root PATH]".to_owned()),
    }
}

// r[verify agent_harness_runtime.acp_stdio_host.boundary]
fn validate_contract(root: &Path) -> Result<(), String> {
    let root_manifest = read(root, ROOT_MANIFEST)?;
    let acp_manifest = read(root, ACP_MANIFEST)?;
    let lock = read(root, LOCK_PATH)?;
    let source = read_tree(root.join(ACP_SOURCE_ROOT).as_path())?;
    let docs = read(root, CONTRACT_DOC)?;

    require(
        root_manifest.contains(&format!("{FEATURE_NAME} = ["))
            && root_manifest.contains("dep:animus-acp-stdio-host"),
        "root ACP feature is not explicit and optional",
    )?;
    let default_members = section(&root_manifest, DEFAULT_MEMBERS_START, SECTION_END)?;
    require(
        !default_members.contains("animus-acp-stdio-host"),
        "ACP host crate entered workspace default members",
    )?;
    require(
        acp_manifest.contains(&format!(
            "{ACP_PACKAGE} = {{ version = \"={ACP_VERSION}\", default-features = false }}"
        )) && acp_manifest.contains(&format!(
            "{ACP_SCHEMA_PACKAGE} = {{ version = \"={ACP_SCHEMA_VERSION}\", default-features = false }}"
        )) && !acp_manifest.contains("features = [\"unstable\"]"),
        "reviewed stable ACP SDK pin is missing",
    )?;
    require(
        lock_package_matches(&lock, ACP_PACKAGE, ACP_VERSION)
            && lock_package_matches(&lock, ACP_SCHEMA_PACKAGE, ACP_SCHEMA_VERSION),
        "Cargo.lock does not bind the reviewed ACP SDK/schema cohort",
    )?;
    for marker in REQUIRED_SOURCE_MARKERS {
        require(
            source.contains(marker),
            format!("required ACP source marker is missing: {marker}").as_str(),
        )?;
    }
    for forbidden in FORBIDDEN_ACP_SOURCE {
        require(
            !source.contains(forbidden),
            format!("ACP source gained forbidden remote/ambient authority: {forbidden}").as_str(),
        )?;
    }
    for marker in REQUIRED_DOC_MARKERS {
        require(
            docs.contains(marker),
            format!("required ACP documentation marker is missing: {marker}").as_str(),
        )?;
    }
    for reusable_root in REUSABLE_ROOTS {
        let reusable = read_tree(root.join(reusable_root).as_path())?;
        require(
            !reusable.contains(ACP_PACKAGE) && !reusable.contains("run_acp_stdio"),
            format!("reusable/default crate gained ACP transport ownership: {reusable_root}")
                .as_str(),
        )?;
    }
    require(
        read(root, "cairn/component-catalog.ncl")?.contains("std.acp-stdio-host")
            && read(root, "cairn/runtime-facade-inventory.ncl")?
                .contains("animus::acp_stdio_host"),
        "ACP component or facade inventory entry is missing",
    )
}

fn validate_dependency_isolation(root: &Path) -> Result<(), String> {
    let default_root = capture(
        root,
        "cargo",
        &["tree", "-p", "animus", "--locked"],
    )?;
    require(
        !default_root.contains(DEPENDENCY_NEEDLE),
        "default root graph unexpectedly includes ACP SDK",
    )?;
    let no_default_root = capture(
        root,
        "cargo",
        &["tree", "-p", "animus", "--no-default-features", "--locked"],
    )?;
    require(
        !no_default_root.contains(DEPENDENCY_NEEDLE),
        "no-default root graph unexpectedly includes ACP SDK",
    )?;
    let wasm_root = capture(
        root,
        "cargo",
        &[
            "tree",
            "-p",
            "animus",
            "--no-default-features",
            "--target",
            "wasm32-unknown-unknown",
            "--locked",
        ],
    )?;
    require(
        !wasm_root.contains(DEPENDENCY_NEEDLE),
        "Wasm root graph unexpectedly includes ACP SDK",
    )?;
    let acp_graph = capture(
        root,
        "cargo",
        &["tree", "-p", "animus-acp-stdio-host", "--locked"],
    )?;
    require(
        acp_graph.contains(DEPENDENCY_NEEDLE),
        "explicit ACP host graph omitted the reviewed SDK",
    )
}

fn run_checks(root: &Path) -> Result<(), String> {
    let checks = [
        Check {
            program: "cargo",
            args: &["test", "-p", "animus-acp-stdio-host", "--locked"],
        },
        Check {
            program: "cargo",
            args: &[
                "test",
                "-p",
                "animus-std",
                "--features",
                "coding-session-host",
                "coding_session::tests",
                "--locked",
            ],
        },
        Check {
            program: "cargo",
            args: &["test", "-p", "animus-tui", "frontend", "--locked"],
        },
        Check {
            program: "cargo",
            args: &[
                "test",
                "-p",
                "animus",
                "--features",
                FEATURE_NAME,
                "--locked",
            ],
        },
        Check {
            program: "cargo",
            args: &[
                "clippy",
                "-p",
                "animus-acp-stdio-host",
                "--all-targets",
                "--no-deps",
                "--",
                "-D",
                "warnings",
            ],
        },
        Check {
            program: "cargo",
            args: &["check", "-p", "animus", "--locked"],
        },
        Check {
            program: "cargo",
            args: &[
                "check",
                "-p",
                "animus",
                "--no-default-features",
                "--locked",
            ],
        },
    ];
    for check in checks {
        run_command(root, check.program, check.args)?;
    }
    Ok(())
}

fn compute_source_identity(root: &Path) -> Result<String, String> {
    let mut first = Command::new("b3sum");
    first.current_dir(root).args(SOURCE_IDENTITY_PATHS);
    let first_output = first
        .output()
        .map_err(|error| format!("cannot run b3sum: {error}"))?;
    if !first_output.status.success() {
        let diagnostic = String::from_utf8_lossy(first_output.stderr.as_slice());
        return Err(format!(
            "source b3sum command failed: {}",
            diagnostic.trim()
        ));
    }

    let mut second = Command::new("b3sum")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot start aggregate b3sum: {error}"))?;
    let stdin = second
        .stdin
        .as_mut()
        .ok_or_else(|| "aggregate b3sum stdin unavailable".to_owned())?;
    stdin
        .write_all(first_output.stdout.as_slice())
        .map_err(|error| format!("cannot write aggregate b3sum input: {error}"))?;
    drop(second.stdin.take());
    let output = second
        .wait_with_output()
        .map_err(|error| format!("cannot finish aggregate b3sum: {error}"))?;
    if !output.status.success() {
        let diagnostic = String::from_utf8_lossy(output.stderr.as_slice());
        return Err(format!(
            "aggregate b3sum command failed: {}",
            diagnostic.trim()
        ));
    }
    let rendered = String::from_utf8(output.stdout)
        .map_err(|error| format!("aggregate b3sum output is not UTF-8: {error}"))?;
    rendered
        .split_whitespace()
        .next()
        .map(str::to_owned)
        .ok_or_else(|| "aggregate b3sum output is empty".to_owned())
}

fn write_receipt(root: &Path, source_identity: &str) -> Result<(), String> {
    let path = root.join(RECEIPT_PATH);
    let parent = path
        .parent()
        .ok_or_else(|| "focused receipt parent is unavailable".to_owned())?;
    fs::create_dir_all(parent)
        .map_err(|error| format!("cannot create focused receipt directory: {error}"))?;
    let receipt = format!(
        "# ACP stdio host focused rail\n\n- schema: {RECEIPT_SCHEMA}\n- status: {STATUS_PASSED}\n- source identity (BLAKE3): {source_identity}\n- reviewed cohort: {ACP_PACKAGE} {ACP_VERSION} / {ACP_SCHEMA_PACKAGE} {ACP_SCHEMA_VERSION}\n- license: {ACP_LICENSE}\n- stable protocol subset: passed\n- pure reducer and correlation fixtures: passed\n- one coding-session facade mapping: passed\n- additive permission and tool admission: passed\n- ordered headless/TUI/ACP event projection: passed\n- bounded stdio framing and failure classification: passed\n- durable resume and authority-profile seams: passed\n- default/no-std/Wasm dependency isolation: passed\n- metadata-only receipts and leak rejection: passed\n- sockets, WebSocket, HTTP, TLS, retry, and network fallback: absent\n- arbitrary editor interoperability: not claimed\n- remote transport security: not claimed\n- live providers and concrete product tools: not claimed\n- production readiness and release eligibility: not claimed\n"
    );
    fs::write(path, receipt).map_err(|error| format!("cannot write focused receipt: {error}"))
}

fn lock_package_matches(lock: &str, package: &str, version: &str) -> bool {
    lock.split("[[package]]").any(|record| {
        record.contains(&format!("name = \"{package}\""))
            && record.contains(&format!("version = \"{version}\""))
    })
}

fn section<'a>(text: &'a str, start: &str, end: &str) -> Result<&'a str, String> {
    let after_start = text
        .split_once(start)
        .map(|(_prefix, suffix)| suffix)
        .ok_or_else(|| format!("missing section start: {start}"))?;
    after_start
        .split_once(end)
        .map(|(body, _suffix)| body)
        .ok_or_else(|| format!("missing section end after: {start}"))
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))
}

fn read_tree(root: &Path) -> Result<String, String> {
    let mut paths = Vec::new();
    collect_files(root, &mut paths)?;
    paths.sort();
    let mut text = String::new();
    for path in paths {
        if path.extension().is_some_and(|extension| {
            matches!(extension.to_str(), Some("rs" | "toml" | "md"))
        }) {
            let content = fs::read_to_string(&path)
                .map_err(|error| format!("cannot read {}: {error}", path.display()))?;
            text.push_str(content.as_str());
            text.push('\n');
        }
    }
    Ok(text)
}

fn collect_files(root: &Path, paths: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(root)
        .map_err(|error| format!("cannot read directory {}: {error}", root.display()))?
    {
        let path = entry
            .map_err(|error| format!("cannot read directory entry: {error}"))?
            .path();
        if path.is_dir() {
            collect_files(path.as_path(), paths)?;
        } else if path.is_file() {
            paths.push(path);
        }
    }
    Ok(())
}

fn capture(root: &Path, program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .current_dir(root)
        .args(args)
        .output()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "command failed: {program} {}\n{}",
            args.join(" "),
            String::from_utf8_lossy(output.stderr.as_slice())
        ));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| format!("command output is not UTF-8: {error}"))
}

fn run_command(root: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    println!("running: {program} {}", args.join(" "));
    let status = Command::new(program)
        .current_dir(root)
        .args(args)
        .status()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    require(
        status.success(),
        format!("command failed: {program} {}", args.join(" ")).as_str(),
    )
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.to_owned())
    }
}
