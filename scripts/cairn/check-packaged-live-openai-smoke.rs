use std::env;
use std::fs;
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode, Output, Stdio};

const EXIT_SUCCESS: u8 = 0;
const EXIT_FAILURE: u8 = 1;
const ARG_ROOT: &str = "--root";
const HOST_MANIFEST: &str = "crates/animus-std-openai/Cargo.toml";
const HOST_LIB: &str = "crates/animus-std-openai/src/lib.rs";
const SMOKE_CORE: &str = "crates/animus-std-openai/src/smoke.rs";
const OPENAI_SMOKE_BINARY: &str =
    "crates/animus-std-openai/src/bin/animus-live-openai-smoke.rs";
const OPENROUTER_SMOKE_BINARY: &str =
    "crates/animus-std-openai/src/bin/animus-live-openrouter-smoke.rs";
const LIVE_TEST: &str = "crates/animus-std-openai/tests/live-openai.rs";
const HOST_DOC: &str = "docs/live-openai-provider-host.md";
const ROADMAP_DOC: &str = "docs/dogfoodable-host-readiness-roadmap.md";
const RELEASE_CHECKER: &str = "scripts/cairn/check-release-evidence-index.rs";
const CURRENT_HEAD_FIXTURE: &str =
    "cairn/release-evidence-index-fixtures/valid-current-head-live-provider.ncl";
const STALE_FIXTURE: &str =
    "cairn/release-evidence-index-fixtures/stale-live-provider.ncl";
const DIRTY_FIXTURE: &str =
    "cairn/release-evidence-index-fixtures/dirty-live-provider.ncl";
const OPENAI_MODEL_ID: &str = "gpt-5-mini";
const OPENROUTER_MODEL_ID: &str = "openai/gpt-5.5";
const INVALID_MODEL: &str = "invalid model with spaces";
const CREDENTIAL_SENTINEL: &str = "packaged-live-secret-sentinel";
const OPENAI_FAILURE_MARKER: &str = "LIVE_OPENAI_SMOKE status=failed";
const OPENROUTER_FAILURE_MARKER: &str = "LIVE_OPENROUTER_SMOKE status=failed";

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
    run_checks(&root)?;
    validate_packaged_denials(&root)?;
    println!("focused rail packaged-live-provider-smokes: pass");
    println!("live requests: not run; explicit credentials and models remain required");
    Ok(())
}

fn parse_root() -> Result<PathBuf, String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.as_slice() {
        [] => env::current_dir().map_err(|error| error.to_string()),
        [flag, root] if flag == ARG_ROOT => PathBuf::from(root)
            .canonicalize()
            .map_err(|error| format!("cannot resolve root: {error}")),
        _other => Err("usage: check-packaged-live-openai-smoke [--root PATH]".to_owned()),
    }
}

fn validate_contract(root: &Path) -> Result<(), String> {
    // r[verify agent_harness_runtime.packaged_live_openai_smoke.verification]
    // r[verify agent_harness_runtime.packaged_live_openai_smoke.current_head_evidence.failure]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.verification]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.docs]
    let manifest = read(root, HOST_MANIFEST)?;
    let library = read(root, HOST_LIB)?;
    let core = read(root, SMOKE_CORE)?;
    let openai_binary = read(root, OPENAI_SMOKE_BINARY)?;
    let openrouter_binary = read(root, OPENROUTER_SMOKE_BINARY)?;
    let live_test = read(root, LIVE_TEST)?;
    let docs = format!("{}\n{}", read(root, HOST_DOC)?, read(root, ROADMAP_DOC)?);
    let release_checker = read(root, RELEASE_CHECKER)?;

    require(
        manifest.contains("name = \"animus-live-openai-smoke\"")
            && manifest.contains("name = \"animus-live-openrouter-smoke\"")
            && manifest.contains("required-features = [\"reqwest-client\"]"),
        "packaged live provider binaries are not gated by reqwest-client",
    )?;
    for marker in [
        "plan_live_openai_smoke",
        "render_live_openai_smoke_evidence",
        "run_packaged_live_openai_smoke",
        "plan_live_openrouter_smoke",
        "render_live_openrouter_smoke_evidence",
        "run_packaged_live_openrouter_smoke",
    ] {
        require(
            library.contains(marker) || core.contains(marker),
            format!("packaged live smoke marker is missing: {marker}").as_str(),
        )?;
    }
    for binary in [&openai_binary, &openrouter_binary] {
        for marker in [
            "--model",
            "--credential-stdin",
            "MAX_CREDENTIAL_STDIN_BYTES",
            "credential_stdin_oversized",
            "credential_stdin_multiline",
        ] {
            require(
                binary.contains(marker),
                format!("packaged input marker is missing: {marker}").as_str(),
            )?;
        }
        for forbidden in [
            "ANIMUS_OPENAI_LIVE_API_KEY",
            "OPENROUTER_API_KEY",
            "std::env::var",
            "raw_response=present",
            "model_output=present",
        ] {
            require(
                !binary.contains(forbidden),
                format!("packaged binary gained forbidden marker: {forbidden}").as_str(),
            )?;
        }
    }
    require(
        core.contains("raw_response=omitted")
            && core.contains("model_output=omitted")
            && core.contains("credential=omitted"),
        "metadata-only live smoke evidence markers are incomplete",
    )?;
    require(
        live_test.contains("run_packaged_live_openai_smoke")
            && live_test.contains("#[ignore ="),
        "ignored live test does not reuse the packaged smoke",
    )?;
    require(
        release_checker.contains("live-provider receipt does not match the recorded head")
            && root.join(CURRENT_HEAD_FIXTURE).is_file()
            && root.join(STALE_FIXTURE).is_file()
            && root.join(DIRTY_FIXTURE).is_file(),
        "current-head live-provider evidence fixtures or validation are missing",
    )?;
    require(
        docs.contains("animus-live-openai-smoke")
            && docs.contains("animus-live-openrouter-smoke")
            && docs.contains("https://openrouter.ai/api/v1/responses")
            && docs.contains("current head")
            && docs.contains("remains a non-claim"),
        "packaged live smoke docs or non-claims are incomplete",
    )
}

fn run_checks(root: &Path) -> Result<(), String> {
    // r[verify agent_harness_runtime.packaged_live_openai_smoke.closeout]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.closeout]
    let checks = [
        Check {
            program: "cargo",
            args: &[
                "test",
                "-p",
                "animus-std-openai",
                "--features",
                "reqwest-client",
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
                "reqwest-client",
                "--",
                "-D",
                "warnings",
            ],
        },
        Check {
            program: "sh",
            args: &[
                "scripts/cairn/check-release-evidence-index",
                "--root",
                ".",
            ],
        },
    ];
    for check in checks {
        run_command(root, check.program, check.args)?;
    }
    Ok(())
}

fn validate_packaged_denials(root: &Path) -> Result<(), String> {
    // r[verify agent_harness_runtime.packaged_live_openai_smoke.binary.failure]
    // r[verify agent_harness_runtime.packaged_live_openrouter_smoke.binary.failure]
    for (binary, model, failure_marker) in [
        (
            "animus-live-openai-smoke",
            OPENAI_MODEL_ID,
            OPENAI_FAILURE_MARKER,
        ),
        (
            "animus-live-openrouter-smoke",
            OPENROUTER_MODEL_ID,
            OPENROUTER_FAILURE_MARKER,
        ),
    ] {
        let empty = capture_with_stdin(root, binary, model, "")?;
        require(!empty.status.success(), "empty credential unexpectedly passed")?;
        validate_safe_failure(&empty, failure_marker)?;

        let invalid = capture_with_stdin(
            root,
            binary,
            INVALID_MODEL,
            format!("{CREDENTIAL_SENTINEL}\n").as_str(),
        )?;
        require(!invalid.status.success(), "invalid model unexpectedly passed")?;
        validate_safe_failure(&invalid, failure_marker)?;
    }
    Ok(())
}

fn capture_with_stdin(
    root: &Path,
    binary: &str,
    model: &str,
    input: &str,
) -> Result<Output, String> {
    let mut child = Command::new("cargo")
        .args([
            "run",
            "-q",
            "-p",
            "animus-std-openai",
            "--features",
            "reqwest-client",
            "--bin",
            binary,
            "--",
            "--model",
            model,
            "--credential-stdin",
        ])
        .current_dir(root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("cannot start packaged live smoke: {error}"))?;
    let mut stdin = child
        .stdin
        .take()
        .ok_or_else(|| String::from("packaged live smoke stdin is unavailable"))?;
    stdin
        .write_all(input.as_bytes())
        .map_err(|error| format!("cannot write packaged live smoke stdin: {error}"))?;
    drop(stdin);
    child
        .wait_with_output()
        .map_err(|error| format!("cannot read packaged live smoke output: {error}"))
}

fn validate_safe_failure(output: &Output, failure_marker: &str) -> Result<(), String> {
    let stdout = String::from_utf8(output.stdout.clone()).map_err(|error| error.to_string())?;
    let stderr = String::from_utf8(output.stderr.clone()).map_err(|error| error.to_string())?;
    require(stdout.is_empty(), "failed packaged smoke wrote standard output")?;
    require(
        stderr.contains(failure_marker),
        "failed packaged smoke omitted its stable failure marker",
    )?;
    require(
        !stderr.contains(CREDENTIAL_SENTINEL),
        "failed packaged smoke leaked the credential sentinel",
    )
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
