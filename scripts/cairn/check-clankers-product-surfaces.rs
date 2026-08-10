use std::collections::BTreeSet;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARG_FIXTURE: &str = "--fixture";
const ARCHITECTURE_NOTE: &str = "docs/clankers-product-surface-rejection.md";
const SCOPE_DOC: &str = "doc";
const SCOPE_REUSABLE: &str = "reusable";
const EXPECTED_OK: &str = "OK";
const EXPECTED_PREFIX: &str = "expected_issue=";
const SCOPE_PREFIX: &str = "fixture_scope=";
const EXPLICIT_NON_CLAIM_MARKER: &str = "non-claim";
const RUST_EXTENSION: &str = "rs";
const TOML_EXTENSION: &str = "toml";
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const ISSUE_ARCH_NOTE_MISSING: &str = "CLANKERS_ARCH_NOTE_MISSING";
const ISSUE_ARCH_NOTE_INCOMPLETE: &str = "CLANKERS_ARCH_NOTE_INCOMPLETE";
const ISSUE_SCAN_PATH_MISSING: &str = "CLANKERS_SCAN_PATH_MISSING";
const ISSUE_PRODUCT_SURFACE_MARKER: &str = "CLANKERS_PRODUCT_SURFACE_MARKER";

const REUSABLE_SCAN_PATHS: &[&str] = &[
    "crates/animus-core/Cargo.toml",
    "crates/animus-core/src",
    "crates/animus-adapters/Cargo.toml",
    "crates/animus-adapters/src",
    "crates/animus-preserves/Cargo.toml",
    "crates/animus-preserves/src",
    "crates/animus-steel/Cargo.toml",
    "crates/animus-steel/src",
    "crates/animus-std/Cargo.toml",
    "crates/animus-std/src",
];

const REQUIRED_DOC_PHRASES: &[&str] = &[
    "bounded architecture reference",
    "Clankers TUI",
    "daemon/attach protocol",
    "Matrix/P2P",
    "global `.clankers` state",
    "provider router/OAuth",
    "plugin subprocess runtime",
    "Wasm/Extism loading",
    "Automerge desktop storage",
    "self-evolution apply/rollback",
    "live dogfood harnesses",
    "product tools",
    "release tagging workflow",
    "host ownership",
    "injected service boundaries",
    "safety policy",
    "redacted receipts",
    "non-default feature gating",
    "focused validation rails",
];

const PRODUCT_MARKERS: &[ProductMarker] = &[
    ProductMarker {
        label: "clankers dependency or runtime assumption",
        patterns: &["clankers", "clankers::", "clankers-", "clankers_"],
    },
    ProductMarker {
        label: "global Clankers state path",
        patterns: &["~/.clankers", ".clankers"],
    },
    ProductMarker {
        label: "daemon or attach protocol",
        patterns: &["daemon/attach", "attach protocol", "product daemon"],
    },
    ProductMarker {
        label: "Matrix or P2P product coordination",
        patterns: &[
            "matrix/p2p",
            "matrix room",
            "matrix client",
            "matrix-sdk",
            "p2p",
            "peer-to-peer",
        ],
    },
    ProductMarker {
        label: "provider router or OAuth ownership",
        patterns: &["provider router", "oauth"],
    },
    ProductMarker {
        label: "plugin subprocess runtime",
        patterns: &["plugin subprocess", "plugin runtime", "product plugin"],
    },
    ProductMarker {
        label: "Wasm or Extism default runtime loading",
        patterns: &["wasm/extism", "extism", "wasm plugin"],
    },
    ProductMarker {
        label: "Automerge desktop storage",
        patterns: &["automerge", "desktop storage"],
    },
    ProductMarker {
        label: "self-evolution workflow",
        patterns: &["self-evolution", "apply/rollback"],
    },
    ProductMarker {
        label: "dogfood product harness",
        patterns: &["live dogfood", "dogfood harness"],
    },
    ProductMarker {
        label: "product tool default",
        patterns: &["product tool", "product tools"],
    },
    ProductMarker {
        label: "release tagging ownership",
        patterns: &["release tagging"],
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
    fixture: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ProductMarker {
    label: &'static str,
    patterns: &'static [&'static str],
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Issue {
    code: &'static str,
    path: String,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FixtureMetadata {
    expected_issue: String,
    scope: String,
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
        None => run_repo(&config.root),
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

fn run_repo(root: &Path) -> Result<(), String> {
    let mut issues = Vec::new();
    issues.extend(validate_architecture_note(root));
    issues.extend(validate_reusable_paths(root));
    issues.sort();
    print_issues(&issues);
    if issues.is_empty() {
        println!("clankers product-surface rail: ok");
        Ok(())
    } else {
        Err(format!(
            "clankers product-surface rail found {} issue(s)",
            issues.len()
        ))
    }
}

fn run_fixture(root: &Path, fixture: &Path) -> Result<(), String> {
    let text = fs::read_to_string(fixture)
        .map_err(|error| format!("failed to read fixture {}: {error}", fixture.display()))?;
    let metadata = parse_fixture_metadata(&text)?;
    let body = strip_fixture_metadata(&text);
    let fixture_label = relative_path(root, fixture);
    let mut issues = match metadata.scope.as_str() {
        SCOPE_DOC => Vec::new(),
        SCOPE_REUSABLE => scan_text_for_product_markers(&fixture_label, &body),
        other => return Err(format!("unsupported fixture scope: {other}")),
    };
    issues.sort();

    if metadata.expected_issue == EXPECTED_OK {
        if issues.is_empty() {
            println!("positive fixture ok: {fixture_label}");
            return Ok(());
        }
        print_issues(&issues);
        return Err(format!("fixture {fixture_label} expected to pass"));
    }

    if issues
        .iter()
        .any(|issue| issue.code == metadata.expected_issue.as_str())
    {
        println!(
            "negative fixture ok: {fixture_label} -> {}",
            metadata.expected_issue
        );
        Ok(())
    } else {
        print_issues(&issues);
        Err(format!(
            "fixture {fixture_label} did not report expected issue {}",
            metadata.expected_issue
        ))
    }
}

fn validate_architecture_note(root: &Path) -> Vec<Issue> {
    let note_path = root.join(ARCHITECTURE_NOTE);
    let note_label = ARCHITECTURE_NOTE.to_string();
    let Ok(text) = fs::read_to_string(&note_path) else {
        return vec![issue(
            ISSUE_ARCH_NOTE_MISSING,
            &note_label,
            "missing Clankers product-surface rejection note",
        )];
    };

    let mut issues = Vec::new();
    for phrase in REQUIRED_DOC_PHRASES {
        if !contains_case_insensitive(&text, phrase) {
            issues.push(issue(
                ISSUE_ARCH_NOTE_INCOMPLETE,
                &note_label,
                &format!("missing required documentation phrase: {phrase}"),
            ));
        }
    }
    issues
}

fn validate_reusable_paths(root: &Path) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut files = Vec::new();
    for relative in REUSABLE_SCAN_PATHS {
        let path = root.join(relative);
        if !path.exists() {
            issues.push(issue(
                ISSUE_SCAN_PATH_MISSING,
                relative,
                "configured reusable scan path is missing",
            ));
            continue;
        }
        if let Err(error) = collect_scan_files(&path, &mut files) {
            issues.push(issue(
                ISSUE_SCAN_PATH_MISSING,
                relative,
                &format!("failed to collect reusable files: {error}"),
            ));
        }
    }
    files.sort();
    files.dedup();

    for file in files {
        let label = relative_path(root, &file);
        match fs::read_to_string(&file) {
            Ok(text) => issues.extend(scan_text_for_product_markers(&label, &text)),
            Err(error) => issues.push(issue(
                ISSUE_SCAN_PATH_MISSING,
                &label,
                &format!("failed to read reusable file: {error}"),
            )),
        }
    }

    issues
}

fn collect_scan_files(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.is_file() {
        if should_scan_file(path) {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }

    let mut entries = fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let entry_path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_scan_files(&entry_path, files)?;
        } else if should_scan_file(&entry_path) {
            files.push(entry_path);
        }
    }
    Ok(())
}

fn should_scan_file(path: &Path) -> bool {
    path.file_name() == Some(OsStr::new("Cargo.toml"))
        || path.extension() == Some(OsStr::new(RUST_EXTENSION))
        || path.extension() == Some(OsStr::new(TOML_EXTENSION))
}

fn scan_text_for_product_markers(path: &str, text: &str) -> Vec<Issue> {
    let haystack = marker_scan_projection(text);
    let mut issues = Vec::new();
    let mut reported = BTreeSet::new();
    for marker in PRODUCT_MARKERS {
        for pattern in marker.patterns {
            if haystack.contains(&pattern.to_ascii_lowercase()) && reported.insert(marker.label) {
                issues.push(issue(
                    ISSUE_PRODUCT_SURFACE_MARKER,
                    path,
                    &format!(
                        "reusable contract mentions rejected surface: {}",
                        marker.label
                    ),
                ));
            }
        }
    }
    issues
}

fn marker_scan_projection(text: &str) -> String {
    text.lines()
        .filter(|line| !contains_case_insensitive(line, EXPLICIT_NON_CLAIM_MARKER))
        .collect::<Vec<_>>()
        .join("\n")
        .to_ascii_lowercase()
}

fn parse_fixture_metadata(text: &str) -> Result<FixtureMetadata, String> {
    let mut expected_issue = None;
    let mut scope = None;
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix(EXPECTED_PREFIX) {
            expected_issue = Some(value.trim().to_string());
        }
        if let Some(value) = trimmed.strip_prefix(SCOPE_PREFIX) {
            scope = Some(value.trim().to_string());
        }
    }
    Ok(FixtureMetadata {
        expected_issue: expected_issue
            .ok_or_else(|| format!("fixture metadata requires {EXPECTED_PREFIX}"))?,
        scope: scope.ok_or_else(|| format!("fixture metadata requires {SCOPE_PREFIX}"))?,
    })
}

fn strip_fixture_metadata(text: &str) -> String {
    text.lines()
        .filter(|line| {
            let trimmed = line.trim();
            !trimmed.starts_with(EXPECTED_PREFIX) && !trimmed.starts_with(SCOPE_PREFIX)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn contains_case_insensitive(text: &str, phrase: &str) -> bool {
    text.to_ascii_lowercase()
        .contains(&phrase.to_ascii_lowercase())
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

fn issue(code: &'static str, path: &str, detail: &str) -> Issue {
    Issue {
        code,
        path: path.to_string(),
        detail: detail.to_string(),
    }
}

fn print_issues(issues: &[Issue]) {
    for issue in issues {
        println!("ERROR {} {} -- {}", issue.code, issue.path, issue.detail);
    }
}
