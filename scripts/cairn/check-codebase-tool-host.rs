use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const ARG_ROOT: &str = "--root";
const ROOT_MANIFEST: &str = "Cargo.toml";
const STD_MANIFEST: &str = "crates/animus-std/Cargo.toml";
const TOOL_CORE_SOURCE: &str = "crates/animus-std/src/codebasetool/core.rs";
const TOOL_SHELL_SOURCE: &str = "crates/animus-std/src/codebasetool/shell.rs";
const TOOL_SMOKE_SOURCE: &str = "crates/animus-std/src/codebasetool/smoke.rs";
const KIT_SOURCE: &str = "crates/animus-runtime/src/toolexecutorkit.rs";
const SMOKE_SOURCE: &str = "crates/animus-std/src/bin/animus-codebase-tool-smoke.rs";
const TOOL_DOC: &str = "docs/read-only-codebase-tool-host.md";
const COMPONENT_CATALOG: &str = "cairn/component-catalog.ncl";
const FACADE_INVENTORY: &str = "cairn/runtime-facade-inventory.ncl";
const TREE_SITTER_DEPENDENCY: &str = "tree-sitter v";
const REQUIRED_TOOL_MARKERS: &[&str] = &[
    "CODEBASE_QUERY_TOOL_NAME",
    "decode_codebase_query",
    "CodebaseQueryToolExecutor",
    "execute_codebase_query_tool",
    "run_packaged_codebase_tool_smoke",
    "Dir::open_ambient_dir",
];
const REQUIRED_KIT_MARKERS: &[&str] = &[
    "REQUEST_HASH_DOMAIN",
    "request_identity",
    "RequestBindingMismatch",
    "ToolExecutionOutcome",
    "execute_admitted_tool",
];
const FORBIDDEN_TOOL_MARKERS: &[&str] = &[
    "std::env",
    "std::process",
    "reqwest",
    "TcpStream",
    "Command::new",
    "read_dir",
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
    run_packaged_smoke(&root)?;
    println!("focused rail codebase-tool-host: pass");
    println!("provider-directed tool use: not run; remains an explicit non-claim");
    Ok(())
}

fn parse_root() -> Result<PathBuf, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [] => env::current_dir().map_err(|error| error.to_string()),
        [flag, root] if flag == ARG_ROOT => PathBuf::from(root)
            .canonicalize()
            .map_err(|error| format!("cannot resolve root: {error}")),
        _other => Err("usage: check-codebase-tool-host [--root PATH]".to_owned()),
    }
}

fn validate_contract(root: &Path) -> Result<(), String> {
    // r[verify agent_harness_runtime.read_only_codebase_tool.host]
    // r[verify agent_harness_runtime.read_only_codebase_tool.smoke]
    let root_manifest = read(root, ROOT_MANIFEST)?;
    let std_manifest = read(root, STD_MANIFEST)?;
    let tool_source = [
        read(root, TOOL_CORE_SOURCE)?,
        read(root, TOOL_SHELL_SOURCE)?,
        read(root, TOOL_SMOKE_SOURCE)?,
    ]
    .join("\n");
    let kit_source = read(root, KIT_SOURCE)?;
    let smoke_source = read(root, SMOKE_SOURCE)?;
    let docs = read(root, TOOL_DOC)?;

    require(
        root_manifest.contains("codebase-tool-host = [")
            && root_manifest.contains("animus-std/codebase-tool-host"),
        "root codebase tool feature is not explicit",
    )?;
    require(
        std_manifest.contains(
            "codebase-tool-host = [\"codebase-graph-host\", \"dep:animus-runtime\"]",
        ) && std_manifest.contains("required-features = [\"codebase-tool-host\"]"),
        "std codebase tool feature or packaged binary gate is incomplete",
    )?;
    for marker in REQUIRED_TOOL_MARKERS {
        require(
            tool_source.contains(marker),
            format!("required codebase tool marker is missing: {marker}").as_str(),
        )?;
    }
    for marker in REQUIRED_KIT_MARKERS {
        require(
            kit_source.contains(marker),
            format!("required executor-kit marker is missing: {marker}").as_str(),
        )?;
    }
    for marker in FORBIDDEN_TOOL_MARKERS {
        require(
            !tool_source.contains(marker),
            format!("codebase tool module gained forbidden ambient effect: {marker}").as_str(),
        )?;
    }
    require(
        smoke_source.contains("--root")
            && smoke_source.contains("--path")
            && !smoke_source.contains("current_dir"),
        "packaged smoke does not require explicit root and paths",
    )?;
    require(
        docs.contains("complete current request")
            && docs.contains("does not prove semantic analysis")
            && docs.contains("metadata-only completion evidence"),
        "codebase tool documentation or non-claims are incomplete",
    )?;
    require(
        read(root, COMPONENT_CATALOG)?.contains("std.codebase-tool-host")
            && read(root, FACADE_INVENTORY)?.contains("animus::codebase_tool"),
        "codebase tool component or facade metadata is missing",
    )
}

fn validate_dependency_isolation(root: &Path) -> Result<(), String> {
    let default_graph = capture(root, "cargo", &["tree", "-p", "animus", "--locked"])?;
    require(
        !default_graph.contains(TREE_SITTER_DEPENDENCY),
        "default root graph includes tree-sitter",
    )?;
    let no_default_graph = capture(
        root,
        "cargo",
        &["tree", "-p", "animus", "--no-default-features", "--locked"],
    )?;
    require(
        !no_default_graph.contains(TREE_SITTER_DEPENDENCY),
        "no-default root graph includes tree-sitter",
    )?;
    let explicit_graph = capture(
        root,
        "cargo",
        &[
            "tree",
            "-p",
            "animus",
            "--features",
            "codebase-tool-host",
            "--locked",
        ],
    )?;
    require(
        explicit_graph.contains(TREE_SITTER_DEPENDENCY),
        "explicit codebase tool graph omits tree-sitter",
    )
}

fn run_checks(root: &Path) -> Result<(), String> {
    // r[verify agent_harness_runtime.read_only_codebase_tool.verification]
    // r[verify agent_harness_runtime.read_only_codebase_tool.verification.closeout]
    let checks = [
        Check {
            program: "cargo",
            args: &["test", "-p", "animus-runtime", "executionkit", "--locked"],
        },
        Check {
            program: "cargo",
            args: &[
                "test",
                "-p",
                "animus-std",
                "--features",
                "codebase-tool-host",
                "codebase_tool",
                "--locked",
            ],
        },
        Check {
            program: "cargo",
            args: &[
                "test",
                "-p",
                "animus-std",
                "--features",
                "codebase-tool-host",
                "--bin",
                "animus-codebase-tool-smoke",
                "--locked",
            ],
        },
        Check {
            program: "cargo",
            args: &[
                "check",
                "-p",
                "animus",
                "--features",
                "codebase-tool-host",
                "--locked",
            ],
        },
        Check {
            program: "cargo",
            args: &[
                "clippy",
                "-p",
                "animus-runtime",
                "-p",
                "animus-std",
                "--all-targets",
                "--features",
                "animus-std/codebase-tool-host",
                "--",
                "-D",
                "warnings",
            ],
        },
        Check {
            program: "cargo",
            args: &["test", "-p", "animus-std", "hostadapters", "--locked"],
        },
    ];
    for check in checks {
        run_command(root, check.program, check.args)?;
    }
    Ok(())
}

fn run_packaged_smoke(root: &Path) -> Result<(), String> {
    let common = [
        "run",
        "-q",
        "-p",
        "animus-std",
        "--features",
        "codebase-tool-host",
        "--bin",
        "animus-codebase-tool-smoke",
        "--",
        "--root",
        ".",
        "--path",
        "crates/animus-std/src/codebasetool/core.rs",
        "--symbol",
        "decode_codebase_query",
        "--kind",
        "definition",
        "--max-results",
        "2",
    ];
    let output = capture(root, "cargo", &common)?;
    require(
        output.contains("CODEBASE_TOOL_SMOKE status=passed")
            && output.contains("raw_output=omitted")
            && !output.contains("codebasetool/core.rs"),
        "packaged codebase tool smoke output is unsafe or incomplete",
    )?;

    let negative = [
        "run",
        "-q",
        "-p",
        "animus-std",
        "--features",
        "codebase-tool-host",
        "--bin",
        "animus-codebase-tool-smoke",
        "--",
        "--root",
        ".",
        "--path",
        "crates/animus-std/src/codebasetool/core.rs",
        "--symbol",
        "decode_codebase_query",
        "--kind",
        "semantic",
        "--max-results",
        "2",
    ];
    run_command_expect_failure(root, "cargo", &negative)
}

fn run_command(root: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .current_dir(root)
        .status()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    require(
        status.success(),
        format!("command failed: {program} {}", args.join(" ")).as_str(),
    )
}

fn run_command_expect_failure(root: &Path, program: &str, args: &[&str]) -> Result<(), String> {
    let status = Command::new(program)
        .args(args)
        .current_dir(root)
        .status()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    require(
        !status.success(),
        format!("negative command unexpectedly passed: {program} {}", args.join(" ")).as_str(),
    )
}

fn capture(root: &Path, program: &str, args: &[&str]) -> Result<String, String> {
    let output = Command::new(program)
        .args(args)
        .current_dir(root)
        .output()
        .map_err(|error| format!("cannot run {program}: {error}"))?;
    require(
        output.status.success(),
        format!("command failed: {program} {}", args.join(" ")).as_str(),
    )?;
    String::from_utf8(output.stdout).map_err(|error| error.to_string())
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative))
        .map_err(|error| format!("cannot read {relative}: {error}"))
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.to_owned())
    }
}
