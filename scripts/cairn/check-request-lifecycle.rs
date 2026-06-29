use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARG_FIXTURE: &str = "--fixture";
const DOC_PATH: &str = "docs/request-lifecycle.md";
const EXPECTED_PREFIX: &str = "expected_issue=";
const EXPECTED_OK: &str = "OK";
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const ISSUE_DOC_MISSING: &str = "REQUEST_LIFECYCLE_DOC_MISSING";
const ISSUE_MARKER_MISSING: &str = "REQUEST_LIFECYCLE_MARKER_MISSING";

const REQUIRED_MARKERS: &[&str] = &[
    "Typed runtime input",
    "animus-std::RuntimeConfig",
    "Runtime::from_env_pairs",
    "Deterministic core state",
    "animus-core::Intent",
    "animus-core::AnimusState",
    "Steel loop decision",
    "animus-steel::LoopTurn",
    "animus-steel::LoopDecision",
    "Adapter dispatch DTOs",
    "animus-adapters::ModelRequest",
    "animus-adapters::ToolRequest",
    "ModelAdapter",
    "ToolAdapter",
    "Std-shell adapter dispatch",
    "animus-std::run_with_model",
    "Runtime event/report projection",
    "animus-std::RunReport",
    "Safe receipt construction",
    "Host ledger persistence seam",
    "LedgerService",
    "Forbidden dependencies and side effects",
    "environment variables",
    "filesystem paths",
    "credentials",
    "provider config",
    "network resources",
    "clocks",
    "concrete host persistence",
    "scripts/cairn/check-request-lifecycle --root .",
    "scripts/cairn/check-evidence-safety --root .",
    "scripts/cairn/check-wasm-surface --root .",
    "nix run .#cairn -- validate --root .",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
    fixture: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Issue {
    code: &'static str,
    subject: String,
    detail: String,
}

fn main() {
    let result = run();
    match result {
        Ok(()) => std::process::exit(EXIT_SUCCESS),
        Err(error) => {
            eprintln!("error: {error}");
            std::process::exit(EXIT_FAILURE);
        }
    }
}

fn run() -> Result<(), String> {
    let config = parse_args(env::args())?;
    match &config.fixture {
        Some(fixture) => run_fixture(&config.root, fixture),
        None => run_doc(&config.root),
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");
    let mut fixture = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = PathBuf::from(value);
            }
            ARG_FIXTURE => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_FIXTURE} requires a path"))?;
                fixture = Some(PathBuf::from(value));
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(Config { root, fixture })
}

fn run_doc(root: &Path) -> Result<(), String> {
    let doc = root.join(DOC_PATH);
    let text = fs::read_to_string(&doc).map_err(|error| {
        format!(
            "{} {} -- failed to read lifecycle document: {error}",
            ISSUE_DOC_MISSING, DOC_PATH
        )
    })?;
    let mut issues = validate_markers(&text, DOC_PATH);
    issues.sort();
    print_issues(&issues);
    if issues.is_empty() {
        println!("request lifecycle marker rail: ok");
        Ok(())
    } else {
        Err(format!(
            "request lifecycle marker rail found {} issue(s)",
            issues.len()
        ))
    }
}

fn run_fixture(root: &Path, fixture: &Path) -> Result<(), String> {
    let text = fs::read_to_string(fixture)
        .map_err(|error| format!("failed to read fixture {}: {error}", fixture.display()))?;
    let expected_issue = parse_expected_issue(&text)?;
    let body = strip_fixture_metadata(&text);
    let label = relative_path(root, fixture);
    let mut issues = validate_markers(&body, &label);
    issues.sort();

    if expected_issue == EXPECTED_OK {
        if issues.is_empty() {
            println!("positive fixture ok: {label}");
            return Ok(());
        }
        print_issues(&issues);
        return Err(format!("fixture {label} expected to pass"));
    }

    if issues.iter().any(|issue| issue.code == expected_issue) {
        println!("negative fixture ok: {label} -> {expected_issue}");
        Ok(())
    } else {
        print_issues(&issues);
        Err(format!(
            "fixture {label} did not report expected issue {expected_issue}"
        ))
    }
}

fn validate_markers(text: &str, subject: &str) -> Vec<Issue> {
    let normalized = text.to_ascii_lowercase();
    let mut issues = Vec::new();
    for marker in REQUIRED_MARKERS {
        if !normalized.contains(&marker.to_ascii_lowercase()) {
            issues.push(issue(
                ISSUE_MARKER_MISSING,
                subject,
                &format!("missing lifecycle marker: {marker}"),
            ));
        }
    }
    issues
}

fn parse_expected_issue(text: &str) -> Result<String, String> {
    for line in text.lines() {
        if let Some(value) = line.trim().strip_prefix(EXPECTED_PREFIX) {
            return Ok(value.trim().to_string());
        }
    }
    Err(format!("fixture metadata requires {EXPECTED_PREFIX}"))
}

fn strip_fixture_metadata(text: &str) -> String {
    text.lines()
        .filter(|line| !line.trim().starts_with(EXPECTED_PREFIX))
        .collect::<Vec<_>>()
        .join("\n")
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

fn issue(code: &'static str, subject: &str, detail: &str) -> Issue {
    Issue {
        code,
        subject: subject.to_string(),
        detail: detail.to_string(),
    }
}

fn print_issues(issues: &[Issue]) {
    for issue in issues {
        println!("ERROR {} {} -- {}", issue.code, issue.subject, issue.detail);
    }
}
