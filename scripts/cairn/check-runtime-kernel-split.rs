use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ARG_TREE: &str = "--tree";
const ARG_KERNEL: &str = "--kernel";
const ARG_FIXTURE: &str = "--fixture";
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;
const EXPECTED_PREFIX: &str = "expected_issue=";
const KIND_PREFIX: &str = "kind=";
const FIXTURE_SEPARATOR: &str = "---";
const EXPECTED_OK: &str = "OK";
const KIND_TREE: &str = "tree";
const KIND_SOURCE: &str = "source";

const ISSUE_FORBIDDEN_DEP: &str = "RUNTIME_KERNEL_FORBIDDEN_DEPENDENCY";
const ISSUE_FORBIDDEN_SOURCE: &str = "RUNTIME_KERNEL_FORBIDDEN_SOURCE";
const ISSUE_MISSING_EXPORT: &str = "RUNTIME_KERNEL_MISSING_EXPORT";
const ISSUE_MALFORMED_FIXTURE: &str = "RUNTIME_KERNEL_MALFORMED_FIXTURE";

const FORBIDDEN_TREE_MARKERS: &[&str] = &[
    "animus-steel",
    "animus-std",
    "animus-tui",
    "ratatui",
    "tokio",
    "reqwest",
];

const FORBIDDEN_SOURCE_MARKERS: &[&str] = &[
    "pub use crate::headless",
    "pub use crate::provenance",
    "pub use crate::worker_team",
    "pub use crate::batch",
    "pub use crate::dataset_export",
    "animus_steel::",
    "std::env",
    "std::fs",
    "std::net",
    "std::process",
    "std::time",
];

const REQUIRED_KERNEL_EXPORTS: &[&str] = &[
    "RuntimeInput",
    "RuntimeConfig",
    "LoopFabric",
    "TurnState",
    "RuntimeEvent",
    "RunReport",
    "RuntimeCapabilityRegistry",
    "ModelDispatchPlan",
    "ToolDispatchPlan",
    "run_agent_loop",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    tree: Option<PathBuf>,
    kernel: Option<PathBuf>,
    fixture: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Issue {
    code: &'static str,
    subject: String,
    detail: String,
}

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
    let config = parse_args(env::args())?;
    if let Some(fixture) = config.fixture {
        return run_fixture(&fixture);
    }

    let mut issues = Vec::new();
    if let Some(tree) = config.tree {
        let text = read_text(&tree)?;
        issues.extend(validate_tree(&text, &tree.display().to_string()));
    }
    if let Some(kernel) = config.kernel {
        let text = read_text(&kernel)?;
        issues.extend(validate_source(&text, &kernel.display().to_string()));
    }
    finish(issues, "runtime kernel split")
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut tree = None;
    let mut kernel = None;
    let mut fixture = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_TREE => tree = Some(PathBuf::from(next_arg(&mut args, ARG_TREE)?)),
            ARG_KERNEL => kernel = Some(PathBuf::from(next_arg(&mut args, ARG_KERNEL)?)),
            ARG_FIXTURE => fixture = Some(PathBuf::from(next_arg(&mut args, ARG_FIXTURE)?)),
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }

    Ok(Config {
        tree,
        kernel,
        fixture,
    })
}

fn next_arg(args: &mut impl Iterator<Item = String>, name: &str) -> Result<String, String> {
    args.next().ok_or_else(|| format!("{name} requires a path"))
}

fn read_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn run_fixture(path: &Path) -> Result<(), String> {
    let text = read_text(path)?;
    let fixture = parse_fixture(&text, &path.display().to_string())?;
    let issues = match fixture.kind.as_str() {
        KIND_TREE => validate_tree(&fixture.body, &path.display().to_string()),
        KIND_SOURCE => validate_source(&fixture.body, &path.display().to_string()),
        _ => vec![Issue {
            code: ISSUE_MALFORMED_FIXTURE,
            subject: path.display().to_string(),
            detail: format!("unknown fixture kind {}", fixture.kind),
        }],
    };

    if fixture.expected_issue == EXPECTED_OK {
        finish(issues, &path.display().to_string())?;
        println!("positive fixture ok: {}", path.display());
        return Ok(());
    }

    if issues
        .iter()
        .any(|issue| issue.code == fixture.expected_issue.as_str())
    {
        println!(
            "negative fixture ok: {} -> {}",
            path.display(),
            fixture.expected_issue
        );
        Ok(())
    } else {
        print_issues(&issues);
        Err(format!(
            "fixture {} did not report expected issue {}",
            path.display(),
            fixture.expected_issue
        ))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Fixture {
    expected_issue: String,
    kind: String,
    body: String,
}

fn parse_fixture(text: &str, subject: &str) -> Result<Fixture, String> {
    let mut lines = text.lines();
    let expected_line = lines
        .next()
        .ok_or_else(|| format!("{subject}: missing expected issue"))?;
    let kind_line = lines
        .next()
        .ok_or_else(|| format!("{subject}: missing kind"))?;
    let separator = lines
        .next()
        .ok_or_else(|| format!("{subject}: missing separator"))?;
    if !expected_line.starts_with(EXPECTED_PREFIX)
        || !kind_line.starts_with(KIND_PREFIX)
        || separator != FIXTURE_SEPARATOR
    {
        return Err(format!("{subject}: malformed fixture header"));
    }
    Ok(Fixture {
        expected_issue: expected_line[EXPECTED_PREFIX.len()..].to_string(),
        kind: kind_line[KIND_PREFIX.len()..].to_string(),
        body: lines.collect::<Vec<_>>().join("\n"),
    })
}

fn validate_tree(text: &str, subject: &str) -> Vec<Issue> {
    FORBIDDEN_TREE_MARKERS
        .iter()
        .filter(|marker| text.contains(**marker))
        .map(|marker| Issue {
            code: ISSUE_FORBIDDEN_DEP,
            subject: subject.to_string(),
            detail: (*marker).to_string(),
        })
        .collect()
}

fn validate_source(text: &str, subject: &str) -> Vec<Issue> {
    let mut issues = Vec::new();
    for marker in FORBIDDEN_SOURCE_MARKERS {
        if text.contains(marker) {
            issues.push(Issue {
                code: ISSUE_FORBIDDEN_SOURCE,
                subject: subject.to_string(),
                detail: (*marker).to_string(),
            });
        }
    }
    for export in REQUIRED_KERNEL_EXPORTS {
        if !text.contains(export) {
            issues.push(Issue {
                code: ISSUE_MISSING_EXPORT,
                subject: subject.to_string(),
                detail: (*export).to_string(),
            });
        }
    }
    issues
}

fn finish(mut issues: Vec<Issue>, label: &str) -> Result<(), String> {
    issues.sort();
    print_issues(&issues);
    if issues.is_empty() {
        println!("{label}: ok");
        Ok(())
    } else {
        Err(format!("{label} failed with {} issue(s)", issues.len()))
    }
}

fn print_issues(issues: &[Issue]) {
    for issue in issues {
        println!(
            "ERROR {} {} {}",
            issue.code, issue.subject, issue.detail
        );
    }
}
