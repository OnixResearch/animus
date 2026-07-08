use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARCHITECTURE_NOTE: &str = "docs/tool-catalog-capability-packs.md";
const SCHEMA_SOURCE: &str = "policy/tool-catalog/catalog.ncl";
const FIXTURE_DIR: &str = "cairn/tool-catalog-fixtures";
const FIXTURE_EXTENSION: &str = "catalog";
const EXPECTED_PREFIX: &str = "expected_issue=";
const DESCRIPTOR_SEPARATOR: &str = "---";
const KEY_VALUE_SEPARATOR: char = '=';
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;
const REQUIRED_DESCRIPTOR_FIELDS: &[&str] = &[
    "name",
    "description",
    "runtime_kind",
    "side_effect",
    "approval",
    "input_schema",
    "output_schema",
    "redaction",
];
const CAPABILITY_FIELD: &str = "capability";
const CAPABILITIES_SCHEMA_FIELD: &str = "capabilities";
const OK_EXPECTATION: &str = "OK";
const SCHEMA_PREFIX: &str = "schema:";
const APPROVAL_ALLOW: &str = "allow";
const REDACTION_NONE: &str = "none";
const ISSUE_DOC_MISSING: &str = "TOOL_CATALOG_DOC_MISSING";
const ISSUE_DOC_INCOMPLETE: &str = "TOOL_CATALOG_DOC_INCOMPLETE";
const ISSUE_SCHEMA_MISSING: &str = "TOOL_CATALOG_SCHEMA_MISSING";
const ISSUE_FIXTURE_MISSING: &str = "TOOL_CATALOG_FIXTURE_MISSING";
const ISSUE_MISSING_FIELD: &str = "TOOL_CATALOG_MISSING_FIELD";
const ISSUE_DUPLICATE_NAME: &str = "TOOL_CATALOG_DUPLICATE_NAME";
const ISSUE_BUILTIN_COLLISION: &str = "TOOL_CATALOG_BUILTIN_COLLISION";
const ISSUE_UNKNOWN_RUNTIME: &str = "TOOL_CATALOG_UNKNOWN_RUNTIME";
const ISSUE_INVALID_SCHEMA: &str = "TOOL_CATALOG_INVALID_SCHEMA";
const ISSUE_DANGEROUS_WITHOUT_APPROVAL: &str = "TOOL_CATALOG_DANGEROUS_WITHOUT_APPROVAL";
const ISSUE_SECRET_WITHOUT_REDACTION: &str = "TOOL_CATALOG_SECRET_WITHOUT_REDACTION";
const ISSUE_RUNTIME_NEUTRALITY: &str = "TOOL_CATALOG_RUNTIME_NEUTRALITY";
const ISSUE_FIXTURE_EXPECTATION: &str = "TOOL_CATALOG_FIXTURE_EXPECTATION";

const REQUIRED_DOC_PHRASES: &[&str] = &[
    "bounded reference",
    "descriptor shape",
    "capability packs",
    "Animus-owned manifest",
    "active trusted descriptors",
    "host-injected definition set",
    "does not import Clankers built-in tools",
    "plugin runtimes",
    "provider clients",
    "schema references",
    "duplicate names",
    "dangerous capabilities without prompt-or-deny approval",
    "secret-adjacent capabilities without redaction",
    "exact declared capability identity",
    "broad `default-tool` capability does not authorize",
    "must not execute tools",
    "start plugins",
    "start providers",
    "read secrets",
    "inherit environment values",
    "open network connections",
    "fallback discovery",
    "focused rail",
];

const ALLOWED_RUNTIME_KINDS: &[&str] = &["host_descriptor", "injected_service", "external_adapter"];
const DANGEROUS_CAPABILITIES: &[&str] = &[
    "workspace_mutation",
    "shell",
    "network",
    "process",
    "secret_adjacent",
];
const BUILT_IN_TOOL_NAMES: &[&str] = &["shell", "network", "model", "tool", "filesystem", "exec"];
const RUNTIME_NEUTRAL_PATHS: &[&str] = &[SCHEMA_SOURCE, FIXTURE_DIR];
const FORBIDDEN_RUNTIME_TOKENS: &[&str] = &[
    "std::process",
    "process::Command",
    "Command::new",
    "tokio::process",
    "std::env::var",
    "env::var",
    "vars_os",
    "std::net",
    "TcpStream",
    "UdpSocket",
    "reqwest",
    "ureq",
    "wasmtime::",
    "wasmer::",
    "extism::",
    "File::open",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Issue {
    code: &'static str,
    path: String,
    detail: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Fixture {
    expected_issue: String,
    descriptors: Vec<Descriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Descriptor {
    fields: BTreeMap<String, Vec<String>>,
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
    let mut issues = Vec::new();
    issues.extend(validate_architecture_note(&config.root));
    issues.extend(validate_schema_source(&config.root));
    issues.extend(validate_fixture_set(&config.root)?);
    issues.extend(validate_runtime_neutrality(&config.root));
    issues.sort();

    for issue in &issues {
        println!("ERROR {} {} -- {}", issue.code, issue.path, issue.detail);
    }

    if issues.is_empty() {
        println!("tool catalog rail: ok");
        Ok(())
    } else {
        Err(format!("tool catalog rail found {} issue(s)", issues.len()))
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = PathBuf::from(value);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(Config { root })
}

fn validate_architecture_note(root: &Path) -> Vec<Issue> {
    let note_path = root.join(ARCHITECTURE_NOTE);
    let Ok(text) = fs::read_to_string(&note_path) else {
        return vec![issue(
            ISSUE_DOC_MISSING,
            ARCHITECTURE_NOTE,
            "missing tool catalog architecture note",
        )];
    };

    let mut issues = Vec::new();
    for phrase in REQUIRED_DOC_PHRASES {
        if !contains_case_insensitive(&text, phrase) {
            issues.push(issue(
                ISSUE_DOC_INCOMPLETE,
                ARCHITECTURE_NOTE,
                &format!("missing required phrase: {phrase}"),
            ));
        }
    }
    issues
}

fn validate_schema_source(root: &Path) -> Vec<Issue> {
    let schema_path = root.join(SCHEMA_SOURCE);
    let Ok(text) = fs::read_to_string(&schema_path) else {
        return vec![issue(
            ISSUE_SCHEMA_MISSING,
            SCHEMA_SOURCE,
            "missing Nickel tool catalog schema",
        )];
    };

    let mut issues = Vec::new();
    for phrase in REQUIRED_DESCRIPTOR_FIELDS {
        if !text.contains(phrase) {
            issues.push(issue(
                ISSUE_SCHEMA_MISSING,
                SCHEMA_SOURCE,
                &format!("schema missing descriptor field: {phrase}"),
            ));
        }
    }
    if !text.contains(CAPABILITIES_SCHEMA_FIELD) {
        issues.push(issue(
            ISSUE_SCHEMA_MISSING,
            SCHEMA_SOURCE,
            "schema missing capabilities field",
        ));
    }
    issues
}

fn validate_fixture_set(root: &Path) -> Result<Vec<Issue>, String> {
    let fixture_dir = root.join(FIXTURE_DIR);
    if !fixture_dir.exists() {
        return Ok(vec![issue(
            ISSUE_FIXTURE_MISSING,
            FIXTURE_DIR,
            "missing tool catalog fixtures",
        )]);
    }

    let mut fixture_paths = collect_fixture_paths(&fixture_dir)?;
    fixture_paths.sort();
    if fixture_paths.is_empty() {
        return Ok(vec![issue(
            ISSUE_FIXTURE_MISSING,
            FIXTURE_DIR,
            "tool catalog fixture directory is empty",
        )]);
    }

    let mut issues = Vec::new();
    let mut saw_positive = false;
    let mut saw_negative = false;
    for fixture_path in fixture_paths {
        let label = relative_path(root, &fixture_path);
        let text = fs::read_to_string(&fixture_path)
            .map_err(|error| format!("failed to read fixture {label}: {error}"))?;
        let fixture = parse_fixture(&label, &text)?;
        let fixture_issues = validate_catalog(&label, &fixture.descriptors);
        if fixture.expected_issue == OK_EXPECTATION {
            saw_positive = true;
            if !fixture_issues.is_empty() {
                issues.extend(fixture_issues);
                issues.push(issue(
                    ISSUE_FIXTURE_EXPECTATION,
                    &label,
                    "positive fixture produced validation issues",
                ));
            }
        } else {
            saw_negative = true;
            if !fixture_issues
                .iter()
                .any(|candidate| candidate.code == fixture.expected_issue.as_str())
            {
                issues.extend(fixture_issues);
                issues.push(issue(
                    ISSUE_FIXTURE_EXPECTATION,
                    &label,
                    &format!(
                        "negative fixture did not produce expected issue {}",
                        fixture.expected_issue
                    ),
                ));
            }
        }
    }

    if !saw_positive {
        issues.push(issue(
            ISSUE_FIXTURE_MISSING,
            FIXTURE_DIR,
            "missing positive catalog fixture",
        ));
    }
    if !saw_negative {
        issues.push(issue(
            ISSUE_FIXTURE_MISSING,
            FIXTURE_DIR,
            "missing negative catalog fixture",
        ));
    }

    Ok(issues)
}

fn collect_fixture_paths(fixture_dir: &Path) -> Result<Vec<PathBuf>, String> {
    let mut paths = Vec::new();
    let entries = fs::read_dir(fixture_dir)
        .map_err(|error| format!("failed to read fixture directory {}: {error}", fixture_dir.display()))?;
    for entry in entries {
        let entry = entry.map_err(|error| format!("failed to read fixture entry: {error}"))?;
        let path = entry.path();
        if path.extension().and_then(|extension| extension.to_str()) == Some(FIXTURE_EXTENSION) {
            paths.push(path);
        }
    }
    Ok(paths)
}

fn parse_fixture(path: &str, text: &str) -> Result<Fixture, String> {
    let mut expected_issue = None;
    let mut descriptors = Vec::new();
    let mut current_fields: BTreeMap<String, Vec<String>> = BTreeMap::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }
        if let Some(value) = trimmed.strip_prefix(EXPECTED_PREFIX) {
            expected_issue = Some(value.trim().to_string());
            continue;
        }
        if trimmed == DESCRIPTOR_SEPARATOR {
            push_descriptor(&mut descriptors, &mut current_fields);
            continue;
        }
        let Some((key, value)) = trimmed.split_once(KEY_VALUE_SEPARATOR) else {
            return Err(format!("fixture {path} contains invalid line: {trimmed}"));
        };
        current_fields
            .entry(key.trim().to_string())
            .or_default()
            .push(value.trim().to_string());
    }
    push_descriptor(&mut descriptors, &mut current_fields);

    Ok(Fixture {
        expected_issue: expected_issue
            .ok_or_else(|| format!("fixture {path} requires {EXPECTED_PREFIX}"))?,
        descriptors,
    })
}

fn push_descriptor(
    descriptors: &mut Vec<Descriptor>,
    fields: &mut BTreeMap<String, Vec<String>>,
) {
    if fields.is_empty() {
        return;
    }
    descriptors.push(Descriptor {
        fields: std::mem::take(fields),
    });
}

fn validate_catalog(path: &str, descriptors: &[Descriptor]) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut names = BTreeSet::new();
    for descriptor in descriptors {
        issues.extend(validate_descriptor(path, descriptor));
        if let Some(name) = descriptor.first("name") {
            if !names.insert(name.to_string()) {
                issues.push(issue(
                    ISSUE_DUPLICATE_NAME,
                    path,
                    &format!("duplicate descriptor name: {name}"),
                ));
            }
            if contains_label(BUILT_IN_TOOL_NAMES, name) {
                issues.push(issue(
                    ISSUE_BUILTIN_COLLISION,
                    path,
                    &format!("descriptor collides with built-in tool name: {name}"),
                ));
            }
        }
    }
    issues
}

fn validate_descriptor(path: &str, descriptor: &Descriptor) -> Vec<Issue> {
    let mut issues = Vec::new();
    for required in REQUIRED_DESCRIPTOR_FIELDS {
        if descriptor.first(required).is_none() {
            issues.push(issue(
                ISSUE_MISSING_FIELD,
                path,
                &format!("descriptor missing field: {required}"),
            ));
        }
    }
    if descriptor.values(CAPABILITY_FIELD).is_empty() {
        issues.push(issue(
            ISSUE_MISSING_FIELD,
            path,
            "descriptor missing field: capability",
        ));
    }

    if let Some(runtime_kind) = descriptor.first("runtime_kind") {
        if !contains_label(ALLOWED_RUNTIME_KINDS, runtime_kind) {
            issues.push(issue(
                ISSUE_UNKNOWN_RUNTIME,
                path,
                &format!("unknown runtime kind: {runtime_kind}"),
            ));
        }
    }

    for schema_field in ["input_schema", "output_schema"] {
        if let Some(schema_reference) = descriptor.first(schema_field) {
            if !schema_reference.starts_with(SCHEMA_PREFIX) {
                issues.push(issue(
                    ISSUE_INVALID_SCHEMA,
                    path,
                    &format!("{schema_field} must start with {SCHEMA_PREFIX}"),
                ));
            }
        }
    }

    let capabilities = descriptor.values(CAPABILITY_FIELD);
    let approval = descriptor.first("approval");
    if capabilities
        .iter()
        .any(|capability| contains_label(DANGEROUS_CAPABILITIES, capability))
        && approval == Some(APPROVAL_ALLOW)
    {
        issues.push(issue(
            ISSUE_DANGEROUS_WITHOUT_APPROVAL,
            path,
            "dangerous capability requires prompt or deny approval",
        ));
    }

    if capabilities
        .iter()
        .any(|capability| *capability == "secret_adjacent")
        && descriptor.first("redaction") == Some(REDACTION_NONE)
    {
        issues.push(issue(
            ISSUE_SECRET_WITHOUT_REDACTION,
            path,
            "secret-adjacent capability requires metadata or hash redaction",
        ));
    }

    issues
}

fn validate_runtime_neutrality(root: &Path) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut files = Vec::new();
    for relative in RUNTIME_NEUTRAL_PATHS {
        let path = root.join(relative);
        if let Err(error) = collect_text_files(&path, &mut files) {
            issues.push(issue(
                ISSUE_RUNTIME_NEUTRALITY,
                relative,
                &format!("failed to collect runtime-neutrality scan path: {error}"),
            ));
        }
    }
    files.sort();
    files.dedup();
    for file in files {
        let label = relative_path(root, &file);
        match fs::read_to_string(&file) {
            Ok(text) => {
                for token in FORBIDDEN_RUNTIME_TOKENS {
                    if text.contains(token) {
                        issues.push(issue(
                            ISSUE_RUNTIME_NEUTRALITY,
                            &label,
                            &format!("descriptor material contains forbidden runtime token: {token}"),
                        ));
                    }
                }
            }
            Err(error) => issues.push(issue(
                ISSUE_RUNTIME_NEUTRALITY,
                &label,
                &format!("failed to read runtime-neutrality scan file: {error}"),
            )),
        }
    }
    issues
}

fn collect_text_files(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.is_file() {
        files.push(path.to_path_buf());
        return Ok(());
    }
    let mut entries = fs::read_dir(path)?.collect::<Result<Vec<_>, _>>()?;
    entries.sort_by_key(|entry| entry.path());
    for entry in entries {
        let entry_path = entry.path();
        if entry.file_type()?.is_dir() {
            collect_text_files(&entry_path, files)?;
        } else {
            files.push(entry_path);
        }
    }
    Ok(())
}

fn contains_case_insensitive(text: &str, phrase: &str) -> bool {
    text.to_ascii_lowercase()
        .contains(&phrase.to_ascii_lowercase())
}

fn contains_label(labels: &[&str], target: &str) -> bool {
    labels.contains(&target)
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

impl Descriptor {
    fn first(&self, field: &str) -> Option<&str> {
        self.fields
            .get(field)
            .and_then(|values| values.first())
            .map(String::as_str)
    }

    fn values(&self, field: &str) -> Vec<&str> {
        self.fields
            .get(field)
            .map(|values| values.iter().map(String::as_str).collect())
            .unwrap_or_default()
    }
}
