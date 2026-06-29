use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARG_TREE: &str = "--tree";
const ARG_FIXTURE: &str = "--fixture";
const EXPECTED_PREFIX: &str = "expected_issue=";
const EXPECTED_OK: &str = "OK";
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const ISSUE_EXCLUDED_PACKAGE: &str = "WASM_SURFACE_EXCLUDED_PACKAGE";
const ISSUE_TREE_EMPTY: &str = "WASM_SURFACE_TREE_EMPTY";

const EXCLUDED_PACKAGES: &[&str] = &[
    "animus-std",
    "animus-steel",
    "animus-tui",
    "animus-preserves",
    "redb",
    "steel-core",
    "rat-widgets",
    "preserves",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
    tree: Option<PathBuf>,
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
    match (&config.tree, &config.fixture) {
        (Some(tree), None) => run_tree(&config.root, tree),
        (None, Some(fixture)) => run_fixture(&config.root, fixture),
        (Some(_tree), Some(_fixture)) => Err(format!(
            "{ARG_TREE} and {ARG_FIXTURE} cannot be used together"
        )),
        (None, None) => Err(format!("{ARG_TREE} or {ARG_FIXTURE} is required")),
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");
    let mut tree = None;
    let mut fixture = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = PathBuf::from(value);
            }
            ARG_TREE => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_TREE} requires a path"))?;
                tree = Some(PathBuf::from(value));
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

    Ok(Config {
        root,
        tree,
        fixture,
    })
}

fn run_tree(root: &Path, tree: &Path) -> Result<(), String> {
    let tree_text = fs::read_to_string(tree)
        .map_err(|error| format!("failed to read cargo tree {}: {error}", tree.display()))?;
    let mut issues = validate_tree(&tree_text, &relative_path(root, tree));
    issues.sort();
    print_issues(&issues);
    if issues.is_empty() {
        println!("wasm surface dependency guard: ok");
        Ok(())
    } else {
        Err(format!(
            "wasm surface dependency guard found {} issue(s)",
            issues.len()
        ))
    }
}

fn run_fixture(root: &Path, fixture: &Path) -> Result<(), String> {
    let fixture_text = fs::read_to_string(fixture)
        .map_err(|error| format!("failed to read fixture {}: {error}", fixture.display()))?;
    let expected_issue = parse_expected_issue(&fixture_text)?;
    let tree_text = strip_fixture_metadata(&fixture_text);
    let fixture_label = relative_path(root, fixture);
    let mut issues = validate_tree(&tree_text, &fixture_label);
    issues.sort();

    if expected_issue == EXPECTED_OK {
        if issues.is_empty() {
            println!("positive fixture ok: {fixture_label}");
            return Ok(());
        }
        print_issues(&issues);
        return Err(format!("fixture {fixture_label} expected to pass"));
    }

    if issues.iter().any(|issue| issue.code == expected_issue) {
        println!("negative fixture ok: {fixture_label} -> {expected_issue}");
        Ok(())
    } else {
        print_issues(&issues);
        Err(format!(
            "fixture {fixture_label} did not report expected issue {expected_issue}"
        ))
    }
}

fn validate_tree(tree_text: &str, subject: &str) -> Vec<Issue> {
    let packages = package_names(tree_text);
    if packages.is_empty() {
        return vec![issue(
            ISSUE_TREE_EMPTY,
            subject,
            "cargo tree output did not contain package names",
        )];
    }

    let mut issues = Vec::new();
    for package in packages {
        if EXCLUDED_PACKAGES.contains(&package.as_str()) {
            issues.push(issue(
                ISSUE_EXCLUDED_PACKAGE,
                subject,
                &format!("Wasm no-std surface includes excluded package: {package}"),
            ));
        }
    }
    issues
}

fn package_names(tree_text: &str) -> Vec<String> {
    tree_text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.split_whitespace().next())
        .map(trim_tree_prefix)
        .filter(|name| !name.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn trim_tree_prefix(raw_name: &str) -> &str {
    raw_name.trim_start_matches(|character: char| {
        matches!(
            character,
            '├' | '└' | '─' | '│' | '+' | '-' | '`' | ' ' | '\t'
        )
    })
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
