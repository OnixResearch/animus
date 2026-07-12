use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::PathBuf;

const ARG_ROOT: &str = "--root";
const ARG_CATALOG: &str = "--catalog";
const ARG_DOCS: &str = "--docs";
const COMPONENT_KIND: &str = "component";
const FIELD_SEPARATOR: char = '|';
const LIST_SEPARATOR: char = ',';
const COMPONENT_FIELD_COUNT: usize = 12;
const INDEX_KIND: usize = 0;
const INDEX_COMPONENT_ID: usize = 1;
const INDEX_OWNER: usize = 2;
const INDEX_COMPONENT_CLASS: usize = 3;
const INDEX_FEATURE_GATE: usize = 4;
const INDEX_REQUIRED_AUTHORITY: usize = 5;
const INDEX_DEPENDS_ON: usize = 6;
const INDEX_COMPOSES_WITH: usize = 7;
const INDEX_CAPABILITY_FAMILY: usize = 8;
const INDEX_FOCUSED_CHECKS: usize = 9;
const INDEX_EVIDENCE_LABELS: usize = 10;
const INDEX_PUBLIC_IMPORT: usize = 11;
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const CLASS_REUSABLE: &str = "reusable-contract";
const CLASS_HOST_INJECTION: &str = "host-injection-surface";
const CLASS_SHELL: &str = "shell-adapter";
const CLASS_OPTIONAL_BRIDGE: &str = "optional-bridge";
const CLASS_RUNTIME_PRELUDE: &str = "runtime-prelude";
const CLASS_CRATE_FEATURE: &str = "crate-feature";
const AUTHORITY_NONE: &str = "none";
const AUTHORITY_INJECTED: &str = "injected-only";
const AUTHORITY_SHELL: &str = "shell-owned";
const DOC_ROW_CHECK_SEPARATOR: &str = "<br>";

const ISSUE_MALFORMED_RECORD: &str = "COMPONENT_CATALOG_MALFORMED_RECORD";
const ISSUE_DUPLICATE_ID: &str = "COMPONENT_CATALOG_DUPLICATE_ID";
const ISSUE_INVALID_ID: &str = "COMPONENT_CATALOG_INVALID_ID";
const ISSUE_UNKNOWN_CLASS: &str = "COMPONENT_CATALOG_UNKNOWN_CLASS";
const ISSUE_UNKNOWN_FEATURE: &str = "COMPONENT_CATALOG_UNKNOWN_FEATURE";
const ISSUE_UNKNOWN_AUTHORITY: &str = "COMPONENT_CATALOG_UNKNOWN_AUTHORITY";
const ISSUE_UNKNOWN_FAMILY: &str = "COMPONENT_CATALOG_UNKNOWN_FAMILY";
const ISSUE_UNKNOWN_DEPENDENCY: &str = "COMPONENT_CATALOG_UNKNOWN_DEPENDENCY";
const ISSUE_UNKNOWN_COMPOSITION_LINK: &str = "COMPONENT_CATALOG_UNKNOWN_COMPOSITION_LINK";
const ISSUE_DEPENDENCY_CYCLE: &str = "COMPONENT_CATALOG_DEPENDENCY_CYCLE";
const ISSUE_MISSING_CHECKS: &str = "COMPONENT_CATALOG_MISSING_CHECKS";
const ISSUE_REUSABLE_AMBIENT_AUTHORITY: &str =
    "COMPONENT_CATALOG_REUSABLE_AMBIENT_AUTHORITY";
const ISSUE_OPTIONAL_BRIDGE_AUTHORITY: &str = "COMPONENT_CATALOG_OPTIONAL_BRIDGE_AUTHORITY";
const ISSUE_UNSAFE_EVIDENCE_FIELD: &str = "COMPONENT_CATALOG_UNSAFE_EVIDENCE_FIELD";
const ISSUE_DOC_DRIFT: &str = "COMPONENT_CATALOG_DOC_DRIFT";

const ALLOWED_CLASSES: &[&str] = &[
    CLASS_REUSABLE,
    CLASS_HOST_INJECTION,
    CLASS_SHELL,
    CLASS_OPTIONAL_BRIDGE,
    CLASS_RUNTIME_PRELUDE,
    CLASS_CRATE_FEATURE,
];

const ALLOWED_FEATURES: &[&str] = &[
    "always",
    "default",
    "compat",
    "router",
    "runtime",
    "status",
    "orchestration",
    "steel",
    "preserves",
    "std",
    "tui",
    "all-families",
    "batch",
    "compaction",
    "dataset-export",
    "headless",
    "loop",
    "model-runtime-profiles",
    "agent-role-profiles",
    "provider-retry",
    "policy",
    "provenance",
    "quality",
    "research",
    "scaffold",
    "worker-team",
];

const ALLOWED_AUTHORITIES: &[&str] = &[
    AUTHORITY_NONE,
    AUTHORITY_INJECTED,
    AUTHORITY_SHELL,
    "terminal-owned",
    "process-owned",
    "filesystem-owned",
    "network-owned",
    "persistence-owned",
    "credential-owned",
];

const ALLOWED_FAMILIES: &[&str] = &[
    "adapter-contracts",
    "agent-role",
    "compatibility",
    "batch",
    "compaction",
    "component-catalog",
    "context",
    "core",
    "dataset-export",
    "document-ingest",
    "headless",
    "model",
    "model-evidence",
    "observability",
    "orchestration",
    "policy",
    "preserves",
    "provenance",
    "quality",
    "research",
    "router",
    "runtime",
    "safe-receipt",
    "security-policy",
    "scaffold",
    "status",
    "std-shell",
    "steel",
    "tool",
    "tui",
    "worker-team",
    "workspace",
];

const UNSAFE_FIELD_MARKERS: &[&str] = &[
    "/home/",
    "api_key=",
    "aws_secret_access_key",
    "begin private key",
    "credential",
    "private-path",
    "provider-payload",
    "provider_payload",
    "raw-file",
    "raw-prompt",
    "raw-transcript",
    "raw_prompt",
    "secret",
    "sk-",
    "unbounded-output",
    "unbounded_output",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
    catalog: PathBuf,
    docs: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ComponentEntry {
    component_id: String,
    owner: String,
    component_class: String,
    feature_gate: String,
    required_authority: String,
    depends_on: Vec<String>,
    composes_with: Vec<String>,
    capability_family: String,
    focused_checks: Vec<String>,
    evidence_labels: Vec<String>,
    public_import: String,
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
    let catalog_text = fs::read_to_string(&config.catalog)
        .map_err(|error| format!("failed to read component catalog: {error}"))?;
    let (entries, mut issues) = parse_catalog(&catalog_text);
    issues.extend(validate_catalog(&entries));

    if let Some(docs_path) = config.docs {
        let docs_text = fs::read_to_string(&docs_path)
            .map_err(|error| format!("failed to read component catalog docs: {error}"))?;
        let expected_docs = render_docs(&entries);
        if docs_text != expected_docs {
            issues.push(Issue {
                code: ISSUE_DOC_DRIFT,
                subject: docs_path.display().to_string(),
                detail: "docs/component-catalog.md does not match Nickel catalog projection".to_string(),
            });
        }
    }

    issues.sort();
    print_report(&entries, &issues);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "component catalog failed with {} issue(s)",
            issues.len()
        ))
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");
    let mut catalog = None;
    let mut docs = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = PathBuf::from(value);
            }
            ARG_CATALOG => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_CATALOG} requires a path"))?;
                catalog = Some(PathBuf::from(value));
            }
            ARG_DOCS => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_DOCS} requires a path"))?;
                docs = Some(PathBuf::from(value));
            }
            _ => return Err(format!("unknown argument: {argument}")),
        }
    }

    let catalog = catalog.unwrap_or_else(|| root.join("cairn/component-catalog.ncl.lines"));
    Ok(Config {
        root,
        catalog,
        docs,
    })
}

fn parse_catalog(text: &str) -> (Vec<ComponentEntry>, Vec<Issue>) {
    let mut entries = Vec::new();
    let mut issues = Vec::new();

    for (line_index, raw_line) in text.lines().enumerate() {
        let line = raw_line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(FIELD_SEPARATOR).collect();
        if parts.len() != COMPONENT_FIELD_COUNT || parts[INDEX_KIND] != COMPONENT_KIND {
            issues.push(Issue {
                code: ISSUE_MALFORMED_RECORD,
                subject: format!("line {}", display_line_number(line_index)),
                detail: format!("expected {COMPONENT_FIELD_COUNT} pipe-separated component fields"),
            });
            continue;
        }
        entries.push(ComponentEntry {
            component_id: parts[INDEX_COMPONENT_ID].to_string(),
            owner: parts[INDEX_OWNER].to_string(),
            component_class: parts[INDEX_COMPONENT_CLASS].to_string(),
            feature_gate: parts[INDEX_FEATURE_GATE].to_string(),
            required_authority: parts[INDEX_REQUIRED_AUTHORITY].to_string(),
            depends_on: parse_list(parts[INDEX_DEPENDS_ON]),
            composes_with: parse_list(parts[INDEX_COMPOSES_WITH]),
            capability_family: parts[INDEX_CAPABILITY_FAMILY].to_string(),
            focused_checks: parse_list(parts[INDEX_FOCUSED_CHECKS]),
            evidence_labels: parse_list(parts[INDEX_EVIDENCE_LABELS]),
            public_import: parts[INDEX_PUBLIC_IMPORT].to_string(),
        });
    }

    (entries, issues)
}

fn display_line_number(line_index: usize) -> usize {
    line_index + 1
}

fn parse_list(field: &str) -> Vec<String> {
    if field.trim().is_empty() {
        Vec::new()
    } else {
        field
            .split(LIST_SEPARATOR)
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(str::to_string)
            .collect()
    }
}

fn validate_catalog(entries: &[ComponentEntry]) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut seen_ids = BTreeSet::new();
    let mut all_ids = BTreeSet::new();

    for entry in entries {
        if !all_ids.insert(entry.component_id.clone()) {
            issues.push(Issue {
                code: ISSUE_DUPLICATE_ID,
                subject: entry.component_id.clone(),
                detail: "component id appears more than once".to_string(),
            });
        }
    }

    for entry in entries {
        validate_entry(entry, &all_ids, &mut seen_ids, &mut issues);
    }

    issues.extend(validate_dependency_cycles(entries));
    issues
}

fn validate_entry(
    entry: &ComponentEntry,
    all_ids: &BTreeSet<String>,
    seen_ids: &mut BTreeSet<String>,
    issues: &mut Vec<Issue>,
) {
    seen_ids.insert(entry.component_id.clone());

    if !is_valid_component_id(&entry.component_id) {
        issues.push(Issue {
            code: ISSUE_INVALID_ID,
            subject: entry.component_id.clone(),
            detail: "component ids must use lowercase letters, digits, '.', and '-'".to_string(),
        });
    }
    if !ALLOWED_CLASSES.contains(&entry.component_class.as_str()) {
        issues.push(Issue {
            code: ISSUE_UNKNOWN_CLASS,
            subject: entry.component_id.clone(),
            detail: entry.component_class.clone(),
        });
    }
    if !ALLOWED_FEATURES.contains(&entry.feature_gate.as_str()) {
        issues.push(Issue {
            code: ISSUE_UNKNOWN_FEATURE,
            subject: entry.component_id.clone(),
            detail: entry.feature_gate.clone(),
        });
    }
    if !ALLOWED_AUTHORITIES.contains(&entry.required_authority.as_str()) {
        issues.push(Issue {
            code: ISSUE_UNKNOWN_AUTHORITY,
            subject: entry.component_id.clone(),
            detail: entry.required_authority.clone(),
        });
    }
    if !ALLOWED_FAMILIES.contains(&entry.capability_family.as_str()) {
        issues.push(Issue {
            code: ISSUE_UNKNOWN_FAMILY,
            subject: entry.component_id.clone(),
            detail: entry.capability_family.clone(),
        });
    }
    if entry.focused_checks.is_empty() {
        issues.push(Issue {
            code: ISSUE_MISSING_CHECKS,
            subject: entry.component_id.clone(),
            detail: "component entries must name at least one focused check".to_string(),
        });
    }
    if entry.component_class == CLASS_REUSABLE && entry.required_authority != AUTHORITY_NONE {
        issues.push(Issue {
            code: ISSUE_REUSABLE_AMBIENT_AUTHORITY,
            subject: entry.component_id.clone(),
            detail: format!(
                "reusable contract requires {} authority",
                entry.required_authority
            ),
        });
    }
    if entry.component_class == CLASS_OPTIONAL_BRIDGE && entry.required_authority != AUTHORITY_NONE {
        issues.push(Issue {
            code: ISSUE_OPTIONAL_BRIDGE_AUTHORITY,
            subject: entry.component_id.clone(),
            detail: format!(
                "optional bridge requires {} authority",
                entry.required_authority
            ),
        });
    }

    for dependency in &entry.depends_on {
        if !all_ids.contains(dependency) {
            issues.push(Issue {
                code: ISSUE_UNKNOWN_DEPENDENCY,
                subject: entry.component_id.clone(),
                detail: dependency.clone(),
            });
        }
    }
    for link in &entry.composes_with {
        if !all_ids.contains(link) {
            issues.push(Issue {
                code: ISSUE_UNKNOWN_COMPOSITION_LINK,
                subject: entry.component_id.clone(),
                detail: link.clone(),
            });
        }
    }
    for (field_name, field_value) in entry_safe_fields(entry) {
        if contains_unsafe_marker(&field_value) {
            issues.push(Issue {
                code: ISSUE_UNSAFE_EVIDENCE_FIELD,
                subject: entry.component_id.clone(),
                detail: field_name.to_string(),
            });
        }
    }
}

fn is_valid_component_id(value: &str) -> bool {
    !value.is_empty()
        && value
            .chars()
            .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit() || character == '.' || character == '-')
}

fn entry_safe_fields(entry: &ComponentEntry) -> Vec<(&'static str, String)> {
    let mut fields = vec![
        ("component_id", entry.component_id.clone()),
        ("owner", entry.owner.clone()),
        ("feature_gate", entry.feature_gate.clone()),
        ("capability_family", entry.capability_family.clone()),
        ("public_import", entry.public_import.clone()),
    ];
    fields.extend(
        entry
            .focused_checks
            .iter()
            .cloned()
            .map(|value| ("focused_checks", value)),
    );
    fields.extend(
        entry
            .evidence_labels
            .iter()
            .cloned()
            .map(|value| ("evidence_labels", value)),
    );
    fields
}

fn contains_unsafe_marker(value: &str) -> bool {
    let normalized = value.to_ascii_lowercase();
    UNSAFE_FIELD_MARKERS
        .iter()
        .any(|marker| normalized.contains(marker))
}

fn validate_dependency_cycles(entries: &[ComponentEntry]) -> Vec<Issue> {
    let mut graph = BTreeMap::new();
    for entry in entries {
        graph.insert(entry.component_id.as_str(), entry.depends_on.as_slice());
    }

    let mut issues = Vec::new();
    let mut visited = BTreeSet::new();
    let mut visiting = BTreeSet::new();
    for entry in entries {
        detect_cycle(
            entry.component_id.as_str(),
            &graph,
            &mut visiting,
            &mut visited,
            &mut issues,
        );
    }
    issues
}

fn detect_cycle<'a>(
    node: &'a str,
    graph: &BTreeMap<&'a str, &'a [String]>,
    visiting: &mut BTreeSet<&'a str>,
    visited: &mut BTreeSet<&'a str>,
    issues: &mut Vec<Issue>,
) {
    if visited.contains(node) {
        return;
    }
    if !visiting.insert(node) {
        issues.push(Issue {
            code: ISSUE_DEPENDENCY_CYCLE,
            subject: node.to_string(),
            detail: "dependency cycle detected".to_string(),
        });
        return;
    }

    if let Some(dependencies) = graph.get(node) {
        for dependency in *dependencies {
            if graph.contains_key(dependency.as_str()) {
                detect_cycle(dependency.as_str(), graph, visiting, visited, issues);
            }
        }
    }
    visiting.remove(node);
    visited.insert(node);
}

fn render_docs(entries: &[ComponentEntry]) -> String {
    let mut text = String::new();
    text.push_str("# Component catalog\n\n");
    text.push_str("This projection is generated from `cairn/component-catalog.ncl`. Do not edit catalog rows by hand; update the Nickel source and run `scripts/cairn/check-component-catalog --root .`.\n\n");
    text.push_str("The catalog names Lego-style Animus components, the smallest public import surface, authority posture, dependency links, capability family, and focused validation rail. It is metadata evidence only: it does not claim provider, filesystem, process, network, terminal, credential, persistence, production, or live-service readiness unless a row explicitly classifies that authority as shell-owned or injected.\n\n");
    text.push_str("| Component | Class | Owner | Feature gate | Authority | Family | Focused checks | Smallest import |\n");
    text.push_str("| --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for entry in entries {
        text.push_str("| `");
        text.push_str(&entry.component_id);
        text.push_str("` | `");
        text.push_str(&entry.component_class);
        text.push_str("` | `");
        text.push_str(&entry.owner);
        text.push_str("` | `");
        text.push_str(&entry.feature_gate);
        text.push_str("` | `");
        text.push_str(&entry.required_authority);
        text.push_str("` | `");
        text.push_str(&entry.capability_family);
        text.push_str("` | `");
        text.push_str(&entry.focused_checks.join(DOC_ROW_CHECK_SEPARATOR));
        text.push_str("` | `");
        text.push_str(&entry.public_import);
        text.push_str("` |\n");
    }
    text.push_str("\n## Validation\n\n");
    text.push_str("Run `scripts/cairn/check-component-catalog --root .` to validate labels, dependency links, cycles, focused checks, safe evidence fields, fixtures, and this projection. The Nix check is `nix build .#checks.x86_64-linux.component-catalog --no-link -L`.\n");
    text
}

fn print_report(entries: &[ComponentEntry], issues: &[Issue]) {
    for issue in issues {
        println!(
            "ERROR {} {} {}",
            issue.code, issue.subject, issue.detail
        );
    }

    let mut by_class = BTreeMap::<&str, usize>::new();
    let mut by_family = BTreeMap::<&str, usize>::new();
    let mut dependency_links = 0usize;
    let mut composition_links = 0usize;
    for entry in entries {
        *by_class.entry(entry.component_class.as_str()).or_default() += 1;
        *by_family.entry(entry.capability_family.as_str()).or_default() += 1;
        dependency_links += entry.depends_on.len();
        composition_links += entry.composes_with.len();
    }

    for (class, count) in by_class {
        println!("class {class}: {count}");
    }
    for (family, count) in by_family {
        println!("family {family}: {count}");
    }
    println!("dependency links: {dependency_links}");
    println!("composition links: {composition_links}");
    if issues.is_empty() {
        println!("component catalog: ok");
    }
}
