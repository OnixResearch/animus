use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARG_INVENTORY: &str = "--inventory";
const ARG_PUBLIC_EXPORTS: &str = "--public-exports";
const ENTRY_KIND: &str = "entry";
const FIELD_SEPARATOR: char = '|';
const ENTRY_FIELD_COUNT: usize = 8;
const EXPORT_FIELD_COUNT: usize = 2;
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const CLASS_REUSABLE: &str = "reusable-contract";
const CLASS_HOST_INJECTION: &str = "host-injection-surface";
const CLASS_SHELL: &str = "shell-adapter";
const SIDE_EFFECT_NONE: &str = "none";
const SIDE_EFFECT_INJECTED: &str = "injected-only";
const SIDE_EFFECT_SHELL: &str = "shell-owned";
const STABILITY_STABLE: &str = "stable";
const STABILITY_EXPERIMENTAL: &str = "experimental";
const STABILITY_OPTIONAL: &str = "optional";

const ISSUE_MALFORMED_RECORD: &str = "RUNTIME_FACADE_MALFORMED_RECORD";
const ISSUE_UNKNOWN_CLASS: &str = "RUNTIME_FACADE_UNKNOWN_CLASS";
const ISSUE_UNKNOWN_SIDE_EFFECT: &str = "RUNTIME_FACADE_UNKNOWN_SIDE_EFFECT";
const ISSUE_UNKNOWN_STABILITY: &str = "RUNTIME_FACADE_UNKNOWN_STABILITY";
const ISSUE_DUPLICATE_EXPORT: &str = "RUNTIME_FACADE_DUPLICATE_EXPORT";
const ISSUE_OWNER_MISMATCH: &str = "RUNTIME_FACADE_OWNER_MISMATCH";
const ISSUE_CLASS_SIDE_EFFECT_CONTRADICTION: &str =
    "RUNTIME_FACADE_CLASS_SIDE_EFFECT_CONTRADICTION";
const ISSUE_REUSABLE_AMBIENT_AUTHORITY: &str = "RUNTIME_FACADE_REUSABLE_AMBIENT_AUTHORITY";
const ISSUE_UNCLASSIFIED_EXPORT: &str = "RUNTIME_FACADE_UNCLASSIFIED_EXPORT";
const ISSUE_UNKNOWN_EXPORT: &str = "RUNTIME_FACADE_UNKNOWN_EXPORT";
const ISSUE_UNSAFE_EVIDENCE_FIELD: &str = "RUNTIME_FACADE_UNSAFE_EVIDENCE_FIELD";

const CRATE_LIBS: &[(&str, &str)] = &[
    ("animus", "src/lib.rs"),
    ("animus-core", "crates/animus-core/src/lib.rs"),
    ("animus-adapters", "crates/animus-adapters/src/lib.rs"),
    ("animus-preserves", "crates/animus-preserves/src/lib.rs"),
    ("animus-std", "crates/animus-std/src/lib.rs"),
    ("animus-steel", "crates/animus-steel/src/lib.rs"),
    ("animus-tui", "crates/animus-tui/src/lib.rs"),
];

const DECLARATION_PREFIXES: &[&str] = &[
    "pub const ",
    "pub enum ",
    "pub fn ",
    "pub struct ",
    "pub trait ",
    "pub type ",
];

const AMBIENT_AUTHORITY_MARKERS: &[&str] = &[
    "filesystem",
    "credential",
    "provider discovery",
    "provider config",
    "network",
    "process",
    "daemon",
    "plugin",
    "tui state",
    "clock",
    "environment",
    "persistence",
];

const UNSAFE_FIELD_MARKERS: &[&str] = &[
    "/home/",
    "RAW_PROMPT:",
    "PROVIDER_PAYLOAD:",
    "UNBOUNDED_OUTPUT:",
    "BEGIN PRIVATE KEY",
    "API_KEY=",
    "AWS_SECRET_ACCESS_KEY",
    "sk-",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
    inventory: PathBuf,
    public_exports: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct InventoryEntry {
    export_path: String,
    owning_crate: String,
    source: String,
    class: String,
    side_effect: String,
    stability: String,
    review_note: String,
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
    let inventory_text = fs::read_to_string(&config.inventory)
        .map_err(|error| format!("failed to read inventory lines: {error}"))?;
    let public_exports = match &config.public_exports {
        Some(path) => read_public_exports(path)?,
        None => discover_public_exports(&config.root)?,
    };
    let (entries, mut issues) = parse_inventory(&inventory_text);
    issues.extend(validate_inventory(&entries, &public_exports));
    issues.sort();
    print_report(&entries, &public_exports, &issues);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "runtime facade inventory failed with {} issue(s)",
            issues.len()
        ))
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");
    let mut inventory = None;
    let mut public_exports = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = PathBuf::from(value);
            }
            ARG_INVENTORY => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_INVENTORY} requires a path"))?;
                inventory = Some(PathBuf::from(value));
            }
            ARG_PUBLIC_EXPORTS => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_PUBLIC_EXPORTS} requires a path"))?;
                public_exports = Some(PathBuf::from(value));
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let inventory = inventory.ok_or_else(|| format!("{ARG_INVENTORY} is required"))?;
    Ok(Config {
        root,
        inventory,
        public_exports,
    })
}

fn parse_inventory(text: &str) -> (Vec<InventoryEntry>, Vec<Issue>) {
    let mut entries = Vec::new();
    let mut issues = Vec::new();
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split(FIELD_SEPARATOR).collect();
        if fields.len() != ENTRY_FIELD_COUNT || fields.first() != Some(&ENTRY_KIND) {
            issues.push(issue(
                ISSUE_MALFORMED_RECORD,
                line,
                "inventory record must have the expected entry fields",
            ));
            continue;
        }
        entries.push(InventoryEntry {
            export_path: fields[1].to_string(),
            owning_crate: fields[2].to_string(),
            source: fields[3].to_string(),
            class: fields[4].to_string(),
            side_effect: fields[5].to_string(),
            stability: fields[6].to_string(),
            review_note: fields[7].to_string(),
        });
    }
    (entries, issues)
}

fn validate_inventory(entries: &[InventoryEntry], public_exports: &BTreeSet<String>) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut inventory_paths = BTreeSet::new();
    let mut entry_by_path = BTreeMap::new();

    for entry in entries {
        if !inventory_paths.insert(entry.export_path.clone()) {
            issues.push(issue(
                ISSUE_DUPLICATE_EXPORT,
                &entry.export_path,
                "export appears more than once in the inventory",
            ));
        }
        entry_by_path.insert(entry.export_path.clone(), entry);
        validate_entry(entry, public_exports, &mut issues);
    }

    for export in public_exports {
        if !entry_by_path.contains_key(export) {
            issues.push(issue(
                ISSUE_UNCLASSIFIED_EXPORT,
                export,
                "runtime-facing public export is missing an inventory record",
            ));
        }
    }

    issues
}

fn validate_entry(
    entry: &InventoryEntry,
    public_exports: &BTreeSet<String>,
    issues: &mut Vec<Issue>,
) {
    if !matches!(
        entry.class.as_str(),
        CLASS_REUSABLE | CLASS_HOST_INJECTION | CLASS_SHELL
    ) {
        issues.push(issue(
            ISSUE_UNKNOWN_CLASS,
            &entry.export_path,
            &format!("unsupported class: {}", entry.class),
        ));
    }

    if !matches!(
        entry.side_effect.as_str(),
        SIDE_EFFECT_NONE | SIDE_EFFECT_INJECTED | SIDE_EFFECT_SHELL
    ) {
        issues.push(issue(
            ISSUE_UNKNOWN_SIDE_EFFECT,
            &entry.export_path,
            &format!("unsupported side-effect allowance: {}", entry.side_effect),
        ));
    }

    if !matches!(
        entry.stability.as_str(),
        STABILITY_STABLE | STABILITY_EXPERIMENTAL | STABILITY_OPTIONAL
    ) {
        issues.push(issue(
            ISSUE_UNKNOWN_STABILITY,
            &entry.export_path,
            &format!("unsupported stability intent: {}", entry.stability),
        ));
    }

    if owner_from_export(&entry.export_path) != Some(entry.owning_crate.as_str()) {
        issues.push(issue(
            ISSUE_OWNER_MISMATCH,
            &entry.export_path,
            "owning crate must match the export path prefix",
        ));
    }

    if !public_exports.contains(&entry.export_path) {
        issues.push(issue(
            ISSUE_UNKNOWN_EXPORT,
            &entry.export_path,
            "inventory classifies an export that is not currently public",
        ));
    }

    validate_class_side_effect(entry, issues);
    validate_safe_fields(entry, issues);
}

fn validate_class_side_effect(entry: &InventoryEntry, issues: &mut Vec<Issue>) {
    match entry.class.as_str() {
        CLASS_REUSABLE if entry.side_effect != SIDE_EFFECT_NONE => issues.push(issue(
            ISSUE_CLASS_SIDE_EFFECT_CONTRADICTION,
            &entry.export_path,
            "reusable contracts must not allow injected or shell-owned effects",
        )),
        CLASS_HOST_INJECTION if entry.side_effect != SIDE_EFFECT_INJECTED => issues.push(issue(
            ISSUE_CLASS_SIDE_EFFECT_CONTRADICTION,
            &entry.export_path,
            "host-injection surfaces must use injected-only side effects",
        )),
        CLASS_SHELL if entry.side_effect != SIDE_EFFECT_SHELL => issues.push(issue(
            ISSUE_CLASS_SIDE_EFFECT_CONTRADICTION,
            &entry.export_path,
            "shell adapters must use shell-owned side effects",
        )),
        _ => {}
    }

    if entry.class == CLASS_REUSABLE {
        let searchable = format!(
            "{} {} {}",
            entry.export_path, entry.source, entry.review_note
        )
        .to_ascii_lowercase();
        for marker in AMBIENT_AUTHORITY_MARKERS {
            if searchable.contains(&marker.to_ascii_lowercase()) {
                issues.push(issue(
                    ISSUE_REUSABLE_AMBIENT_AUTHORITY,
                    &entry.export_path,
                    &format!("reusable contract mentions ambient authority marker: {marker}"),
                ));
            }
        }
    }
}

fn validate_safe_fields(entry: &InventoryEntry, issues: &mut Vec<Issue>) {
    let fields = [
        entry.export_path.as_str(),
        entry.owning_crate.as_str(),
        entry.source.as_str(),
        entry.review_note.as_str(),
    ];
    for field in fields {
        let lower = field.to_ascii_lowercase();
        for marker in UNSAFE_FIELD_MARKERS {
            if lower.contains(&marker.to_ascii_lowercase()) {
                issues.push(issue(
                    ISSUE_UNSAFE_EVIDENCE_FIELD,
                    &entry.export_path,
                    &format!("inventory field contains unsafe evidence marker: {marker}"),
                ));
            }
        }
    }
}

fn owner_from_export(export_path: &str) -> Option<&str> {
    export_path.split_once("::").map(|(owner, _name)| owner)
}

fn discover_public_exports(root: &Path) -> Result<BTreeSet<String>, String> {
    let mut exports = BTreeSet::new();
    for (crate_name, relative_path) in CRATE_LIBS {
        let path = root.join(relative_path);
        let text = fs::read_to_string(&path)
            .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
        collect_exports_from_lib(crate_name, &text, &mut exports);
    }
    Ok(exports)
}

fn read_public_exports(path: &Path) -> Result<BTreeSet<String>, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("failed to read public exports {}: {error}", path.display()))?;
    let mut exports = BTreeSet::new();
    for line in text.lines().map(str::trim) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let fields: Vec<&str> = line.split(FIELD_SEPARATOR).collect();
        if fields.len() != EXPORT_FIELD_COUNT {
            return Err(format!("malformed public export line: {line}"));
        }
        exports.insert(format!("{}::{}", fields[0], fields[1]));
    }
    Ok(exports)
}

fn collect_exports_from_lib(crate_name: &str, text: &str, exports: &mut BTreeSet<String>) {
    let mut pending_pub_use = String::new();
    let mut collecting_pub_use = false;

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.starts_with("//") || line.starts_with("#") || line.is_empty() {
            continue;
        }

        if collecting_pub_use {
            pending_pub_use.push(' ');
            pending_pub_use.push_str(line);
            if line.ends_with(';') {
                collect_pub_use_exports(crate_name, &pending_pub_use, exports);
                pending_pub_use.clear();
                collecting_pub_use = false;
            }
            continue;
        }

        if line.starts_with("pub use ") {
            if line.ends_with(';') {
                collect_pub_use_exports(crate_name, line, exports);
            } else {
                pending_pub_use.push_str(line);
                collecting_pub_use = true;
            }
            continue;
        }

        if line.starts_with("pub(crate)") || raw_line.starts_with(char::is_whitespace) {
            continue;
        }

        for prefix in DECLARATION_PREFIXES {
            if let Some(rest) = line.strip_prefix(prefix) {
                if let Some(identifier) = leading_identifier(rest) {
                    exports.insert(format!("{crate_name}::{identifier}"));
                }
            }
        }
    }
}

fn collect_pub_use_exports(crate_name: &str, statement: &str, exports: &mut BTreeSet<String>) {
    let body = statement
        .trim()
        .trim_start_matches("pub use ")
        .trim_end_matches(';')
        .trim();

    if let Some((_prefix, names)) = body.split_once('{') {
        let names = names.trim_end_matches('}');
        for raw_name in names.split(',') {
            let Some(name) = normalized_export_name(raw_name) else {
                continue;
            };
            exports.insert(format!("{crate_name}::{name}"));
        }
        return;
    }

    if let Some(name) = normalized_export_name(body.rsplit("::").next().unwrap_or(body)) {
        exports.insert(format!("{crate_name}::{name}"));
    }
}

fn normalized_export_name(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return None;
    }
    let alias_or_name = trimmed.split(" as ").nth(1).unwrap_or(trimmed).trim();
    leading_identifier(alias_or_name).map(ToOwned::to_owned)
}

fn leading_identifier(text: &str) -> Option<&str> {
    let end = text
        .char_indices()
        .find_map(|(index, character)| {
            (!(character == '_' || character.is_ascii_alphanumeric())).then_some(index)
        })
        .unwrap_or(text.len());
    let candidate = &text[..end];
    (!candidate.is_empty()).then_some(candidate)
}

fn print_report(entries: &[InventoryEntry], public_exports: &BTreeSet<String>, issues: &[Issue]) {
    println!("runtime facade inventory entries: {}", entries.len());
    println!("runtime-facing public exports: {}", public_exports.len());
    let mut class_counts: BTreeMap<&str, usize> = BTreeMap::new();
    for entry in entries {
        *class_counts.entry(entry.class.as_str()).or_default() += 1;
    }
    for (class, count) in class_counts {
        println!("class {class}: {count}");
    }
    if issues.is_empty() {
        println!("runtime facade inventory: ok");
        return;
    }
    for issue in issues {
        println!("ERROR {} {} -- {}", issue.code, issue.subject, issue.detail);
    }
}

fn issue(code: &'static str, subject: &str, detail: &str) -> Issue {
    Issue {
        code,
        subject: subject.to_string(),
        detail: detail.to_string(),
    }
}

#[allow(dead_code)]
fn _assert_io_error_send_sync(error: io::Error) -> io::Error {
    error
}
