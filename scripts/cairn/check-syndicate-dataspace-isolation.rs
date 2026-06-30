use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const DEFAULT_MANIFESTS: &[&str] = &[
    "crates/animus-core/Cargo.toml",
    "crates/animus-runtime/Cargo.toml",
    "crates/animus-adapters/Cargo.toml",
];
const DEFAULT_SOURCE_DIRS: &[&str] = &[
    "crates/animus-core/src",
    "crates/animus-runtime/src",
    "crates/animus-adapters/src",
];
const FORBIDDEN_DEPENDENCY_KEYS: &[&str] = &[
    "syndicate",
    "syndicate-rs",
    "syndicate_dataspace",
    "dataspace-client",
    "dataspace_client",
];
const FORBIDDEN_SOURCE_MARKERS: &[&str] = &[
    "syndicate::",
    "use syndicate",
    "extern crate syndicate",
    "syndicate_rs::",
    "use syndicate_rs",
    "dataspace_client::",
    "use dataspace_client",
];
const RUST_EXTENSION: &str = "rs";
const TOML_COMMENT: char = '#';
const KEY_VALUE_SEPARATOR: char = '=';
const DOUBLE_QUOTE: char = '"';
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

#[derive(Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
}

#[derive(Debug, Eq, PartialEq)]
struct Issue {
    path: PathBuf,
    line_number: usize,
    marker: &'static str,
    detail: &'static str,
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
    run_self_tests()?;
    let config = parse_args(env::args())?;
    let issues = collect_repo_issues(&config.root)?;
    if issues.is_empty() {
        println!("syndicate dataspace isolation: ok");
        return Ok(());
    }

    for issue in &issues {
        println!(
            "ERROR {}:{} marker={} {}",
            issue.path.display(),
            issue.line_number,
            issue.marker,
            issue.detail
        );
    }
    Err(format!(
        "syndicate dataspace isolation failed with {} issue(s)",
        issues.len()
    ))
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = None;
    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = Some(PathBuf::from(value));
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }
    Ok(Config {
        root: root.unwrap_or_else(|| PathBuf::from(".")),
    })
}

fn collect_repo_issues(root: &Path) -> Result<Vec<Issue>, String> {
    let mut issues = Vec::new();
    for manifest in DEFAULT_MANIFESTS {
        let path = root.join(manifest);
        let text = read_text(&path)?;
        issues.extend(manifest_issues(&path, &text));
    }
    for source_dir in DEFAULT_SOURCE_DIRS {
        let path = root.join(source_dir);
        let mut files = Vec::new();
        collect_rust_files(&path, &mut files)?;
        for file in files {
            let text = read_text(&file)?;
            issues.extend(source_issues(&file, &text));
        }
    }
    Ok(issues)
}

fn read_text(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn collect_rust_files(root: &Path, files: &mut Vec<PathBuf>) -> Result<(), String> {
    if !root.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(root)
        .map_err(|error| format!("failed to read dir {}: {error}", root.display()))?
    {
        let entry = entry.map_err(|error| format!("failed to read dir entry: {error}"))?;
        let path = entry.path();
        let file_type = entry
            .file_type()
            .map_err(|error| format!("failed to inspect {}: {error}", path.display()))?;
        if file_type.is_dir() {
            collect_rust_files(&path, files)?;
        } else if file_type.is_file()
            && path
                .extension()
                .is_some_and(|extension| extension == RUST_EXTENSION)
        {
            files.push(path);
        }
    }
    files.sort();
    Ok(())
}

fn manifest_issues(path: &Path, text: &str) -> Vec<Issue> {
    text.lines()
        .enumerate()
        .filter_map(|(line_index, raw_line)| {
            let line_number = line_index + 1;
            let line = strip_toml_comment(raw_line).trim();
            FORBIDDEN_DEPENDENCY_KEYS
                .iter()
                .copied()
                .find(|marker| dependency_line_leaks(line, marker))
                .map(|marker| Issue {
                    path: path.to_path_buf(),
                    line_number,
                    marker,
                    detail: "default reusable crate manifest names a Syndicate/live dataspace dependency",
                })
        })
        .collect()
}

fn source_issues(path: &Path, text: &str) -> Vec<Issue> {
    text.lines()
        .enumerate()
        .filter_map(|(line_index, line)| {
            let line_number = line_index + 1;
            FORBIDDEN_SOURCE_MARKERS
                .iter()
                .copied()
                .find(|marker| line.contains(marker))
                .map(|marker| Issue {
                    path: path.to_path_buf(),
                    line_number,
                    marker,
                    detail: "default reusable crate source imports a Syndicate/live dataspace API",
                })
        })
        .collect()
}

fn strip_toml_comment(line: &str) -> &str {
    line.split_once(TOML_COMMENT)
        .map_or(line, |(before_comment, _comment)| before_comment)
}

fn dependency_line_leaks(line: &str, marker: &str) -> bool {
    let Some((key, _value)) = line.split_once(KEY_VALUE_SEPARATOR) else {
        return false;
    };
    normalize_key(key) == marker
}

fn normalize_key(key: &str) -> &str {
    key.trim().trim_matches(DOUBLE_QUOTE)
}

fn run_self_tests() -> Result<(), String> {
    check(
        "manifest positive",
        !dependency_line_leaks("blake3 = \"1\"", "syndicate"),
    )?;
    check(
        "manifest negative",
        dependency_line_leaks("syndicate = \"0.1\"", "syndicate"),
    )?;
    check(
        "source positive",
        source_issues(
            Path::new("lib.rs"),
            "//! Syndicate dataspaces remain a reference only",
        )
        .is_empty(),
    )?;
    check(
        "source negative",
        !source_issues(Path::new("lib.rs"), "use syndicate::dataspace;").is_empty(),
    )?;
    println!("self-test: syndicate dataspace isolation fixtures ok");
    Ok(())
}

fn check(label: &str, condition: bool) -> Result<(), String> {
    if condition {
        return Ok(());
    }
    Err(format!("self-test failed: {label}"))
}
