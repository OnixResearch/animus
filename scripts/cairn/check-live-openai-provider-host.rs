use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const ARG_ROOT: &str = "--root";
const ROOT_MANIFEST: &str = "Cargo.toml";
const HOST_MANIFEST: &str = "crates/animus-std-openai/Cargo.toml";
const HOST_SOURCE: &str = "crates/animus-std-openai/src";
const LIVE_TEST: &str = "crates/animus-std-openai/tests/live-openai.rs";
const TOOL_TEST: &str = "crates/animus-std-openai/src/tests/tool_streaming.rs";
const TOOL_FIXTURE: &str = "crates/animus-std-openai/tests/fixtures/openai-responses-tool-call.sse";
const FINAL_FIXTURE: &str = "crates/animus-std-openai/tests/fixtures/openai-responses-final.sse";
const HOST_DOC: &str = "docs/live-openai-provider-host.md";
const COMPONENT_CATALOG: &str = ".cairn/component-catalog.ncl";
const FACADE_INVENTORY: &str = ".cairn/runtime-facade-inventory.ncl";
const CONFORMANCE_INDEX: &str = ".cairn/adapter-conformance-index.ncl";
const DEFAULT_MEMBERS_START: &str = "default-members = [";
const SECTION_END: &str = "]";
const REQWEST_DEPENDENCY: &str = "reqwest v0.12.28";
const REQUIRED_SOURCE_MARKERS: &[&str] = &[
    "build_openai_request_body",
    "normalize_openai_response",
    "ReqwestOpenAiHost",
    "CancellationState::Cancelled",
    "redirect(reqwest::redirect::Policy::none())",
    ".no_proxy()",
    ".https_only(config.https_only())",
    ".checked_add(1)",
    "ResponseTooLarge",
    "CredentialBindingMismatch",
    "OpenAiResponsesSseParser",
    "build_tool_capable_openai_request_body",
    "STEEL_REPL_TOOL_NAME",
    "STEEL_REPL_TOOL_SCHEMA",
    "previous_response_id",
    "FUNCTION_CALL_OUTPUT_TYPE",
    "parallel_tool_calls",
    "OpenAiToolStreamRequest",
    "observe_provider_retry_stream",
];
const FORBIDDEN_REUSABLE_MARKERS: &[&str] = &["reqwest::", "ReqwestOpenAiHost"];
const REUSABLE_SOURCE_ROOTS: &[&str] = &[
    "crates/animus-core/src",
    "crates/animus-adapters/src",
    "crates/animus-runtime/src",
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
    println!("focused rail live-openai-provider-host: pass");
    println!("live smoke: not run; remains an explicit non-claim");
    Ok(())
}

fn parse_root() -> Result<PathBuf, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [] => env::current_dir().map_err(|error| error.to_string()),
        [flag, root] if flag == ARG_ROOT => PathBuf::from(root)
            .canonicalize()
            .map_err(|error| format!("cannot resolve root: {error}")),
        _other => Err("usage: check-live-openai-provider-host [--root PATH]".to_owned()),
    }
}

fn validate_contract(root: &Path) -> Result<(), String> {
    // r[verify agent_harness_runtime.live_openai_provider_host.boundary.default_graph]
    // r[verify agent_harness_runtime.live_openai_provider_host.boundary.explicit_feature]
    // r[verify agent_harness_runtime.live_openai_provider_host.evidence]
    // r[verify agent_harness_runtime.live_openai_provider_host.evidence.non_claim]
    // r[verify agent_harness_runtime.local_dspark_responses.endpoint.pass]
    // r[verify agent_harness_runtime.local_dspark_responses.endpoint.failure]
    // r[verify agent_harness_runtime.local_dspark_responses.transport.pass]
    // r[verify agent_harness_runtime.local_dspark_responses.transport.failure]
    let root_manifest = read(root, ROOT_MANIFEST)?;
    let host_manifest = read(root, HOST_MANIFEST)?;
    let source = read_tree(root.join(HOST_SOURCE).as_path())?;
    let live_test = read(root, LIVE_TEST)?;
    let tool_test = read(root, TOOL_TEST)?;
    let tool_fixture = read(root, TOOL_FIXTURE)?;
    let final_fixture = read(root, FINAL_FIXTURE)?;
    let docs = read(root, HOST_DOC)?;

    require(
        root_manifest.contains("openai-provider-host = [")
            && root_manifest.contains("openai-tool-provider-host = [")
            && root_manifest.contains("dep:animus-std-openai")
            && root_manifest.contains("animus-std-openai/reqwest-client")
            && root_manifest.contains("animus-std-openai/tool-streaming"),
        "root OpenAI host feature is not explicit and optional",
    )?;
    let default_members = section(&root_manifest, DEFAULT_MEMBERS_START, SECTION_END)?;
    require(
        !default_members.contains("animus-std-openai"),
        "live OpenAI host entered workspace default members",
    )?;
    require(
        host_manifest.contains("default = []")
            && host_manifest.contains("reqwest-client = [\"dep:reqwest\"]")
            && host_manifest.contains("tool-streaming = [")
            && host_manifest.contains("animus-std/provider-retry")
            && host_manifest.contains("default-features = false"),
        "OpenAI host dependency gate is incomplete",
    )?;
    for marker in REQUIRED_SOURCE_MARKERS {
        require(
            source.contains(marker),
            format!("required OpenAI host source marker is missing: {marker}").as_str(),
        )?;
    }
    require(
        live_test.contains("#[ignore =")
            && live_test.contains("ANIMUS_OPENAI_LIVE_API_KEY")
            && live_test.contains("ANIMUS_OPENAI_LIVE_MODEL"),
        "ignored explicit live smoke is missing",
    )?;
    require(
        tool_test.contains("literal_tool_stream_and_continuation_share_the_production_parser")
            && tool_test.contains("unsupported_tools_and_lossy_schemas_fail_before_contact")
            && tool_test.contains("malformed_reasoning_correlation_bounds_and_cancellation_fail_safely"),
        "tool-streaming positive or negative tests are incomplete",
    )?;
    require(
        tool_test.contains("explicit_live_openai_tool_stream_smoke")
            && tool_test.contains("ANIMUS_OPENAI_TOOL_LIVE_API_KEY")
            && tool_test.contains("render_openai_tool_stream_evidence"),
        "separately opt-in tool-capable live smoke is incomplete",
    )?;
    require(
        tool_fixture.contains("response.function_call_arguments.delta")
            && tool_fixture.contains("response.reasoning_summary_text.delta")
            && tool_fixture.contains("response.completed")
            && final_fixture.contains("response.output_text.delta")
            && final_fixture.contains("response.completed"),
        "literal OpenAI Responses SSE fixtures are incomplete",
    )?;
    require(
        source.contains("LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY")
            && source.contains("LOCAL_DSPARK_RESPONSES_URL")
            && source.contains("projected_reasoning_effort")
            && source.contains("run_packaged_live_local_dspark_smoke")
            && host_manifest.contains("name = \"animus-live-local-dspark-smoke\""),
        "exact local-dspark cohort or packaged smoke is incomplete",
    )?;
    require(
        docs.contains("Without a separately run current-head live smoke, live availability remains a non-claim")
            && docs.contains("does not discover providers")
            && docs.contains("does not prove provider reliability")
            && docs.contains("OpenAI OpenAPI document")
            && docs.contains("openai-python")
            && docs.contains("deepseek-v4-flash-0731-ablit-100")
            && docs.contains("Cleartext HTTP is allowed only"),
        "live evidence, provenance, or non-claims are incomplete",
    )?;
    require(
        read(root, COMPONENT_CATALOG)?.contains("std.live-openai-provider-host")
            && read(root, FACADE_INVENTORY)?.contains("animus::openai_provider_host")
            && read(root, FACADE_INVENTORY)?.contains("animus::openai_tool_provider_host")
            && read(root, CONFORMANCE_INDEX)?.contains("openai-tool-provider-host"),
        "OpenAI component or facade metadata is missing",
    )?;
    for reusable_root in REUSABLE_SOURCE_ROOTS {
        let reusable = read_tree(root.join(reusable_root).as_path())?;
        for forbidden in FORBIDDEN_REUSABLE_MARKERS {
            require(
                !reusable.contains(forbidden),
                format!("reusable source gained live OpenAI authority: {reusable_root}: {forbidden}")
                    .as_str(),
            )?;
        }
    }
    Ok(())
}

fn validate_dependency_isolation(root: &Path) -> Result<(), String> {
    let default_graph = capture(root, "cargo", &["tree", "-p", "animus", "--locked"])?;
    require(
        !default_graph.contains(REQWEST_DEPENDENCY),
        "default root graph includes reqwest",
    )?;
    let no_default_graph = capture(
        root,
        "cargo",
        &["tree", "-p", "animus", "--no-default-features", "--locked"],
    )?;
    require(
        !no_default_graph.contains(REQWEST_DEPENDENCY),
        "no-default root graph includes reqwest",
    )?;
    let explicit_graph = capture(
        root,
        "cargo",
        &[
            "tree",
            "-p",
            "animus",
            "--features",
            "openai-tool-provider-host",
            "--locked",
        ],
    )?;
    require(
        explicit_graph.contains(REQWEST_DEPENDENCY),
        "explicit OpenAI host graph omits reqwest",
    )
}

fn run_checks(root: &Path) -> Result<(), String> {
    // r[verify agent_harness_runtime.live_openai_provider_host.verification]
    // r[verify agent_harness_runtime.live_openai_provider_host.verification.closeout]
    let checks = [
        Check {
            program: "cargo",
            args: &[
                "test",
                "-p",
                "animus-std-openai",
                "--features",
                "tool-streaming",
                "--locked",
            ],
        },
        Check {
            program: "cargo",
            args: &[
                "clippy",
                "-p",
                "animus-std-openai",
                "--all-targets",
                "--features",
                "tool-streaming",
                "--locked",
                "--",
                "-D",
                "warnings",
            ],
        },
        Check {
            program: "cargo",
            args: &[
                "check",
                "-p",
                "animus",
                "--features",
                "openai-tool-provider-host",
                "--locked",
            ],
        },
        Check {
            program: "cargo",
            args: &["test", "-p", "animus-std", "hostadapters", "--locked"],
        },
        Check {
            program: "cargo",
            args: &[
                "test",
                "-p",
                "animus-std",
                "providers::tests::concrete",
                "--locked",
            ],
        },
    ];
    for check in checks {
        run_command(root, check.program, check.args)?;
    }
    Ok(())
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

fn read_tree(root: &Path) -> Result<String, String> {
    let mut files = Vec::new();
    collect_files(root, &mut files)?;
    files.sort();
    let mut output = String::new();
    for file in files {
        output.push_str(
            fs::read_to_string(&file)
                .map_err(|error| format!("cannot read {}: {error}", file.display()))?
                .as_str(),
        );
    }
    Ok(output)
}

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    for entry in fs::read_dir(root).map_err(|error| format!("cannot read {}: {error}", root.display()))? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.is_dir() {
            collect_files(path.as_path(), files)?;
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            files.push(path);
        }
    }
    Ok(())
}

fn section<'a>(source: &'a str, start: &str, end: &str) -> Result<&'a str, String> {
    let start_index = source.find(start).ok_or_else(|| format!("missing section: {start}"))?;
    let remainder = &source[start_index + start.len()..];
    let end_index = remainder.find(end).ok_or_else(|| format!("unterminated section: {start}"))?;
    Ok(&remainder[..end_index])
}

fn require(condition: bool, message: &str) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message.to_owned())
    }
}
