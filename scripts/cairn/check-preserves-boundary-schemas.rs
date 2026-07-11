use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARG_SELF_TEST: &str = "--self-test";
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;
const ROOT_ARGUMENT_COUNT: usize = 2;
const DEPENDENCIES_SECTION: &str = "[dependencies]";
const ADAPTER_MANIFEST: &str = "crates/animus-preserves-adapter/Cargo.toml";
const FACADE_MANIFEST: &str = "crates/animus-preserves/Cargo.toml";
const FACADE_SOURCE: &str = "crates/animus-preserves/src/lib.rs";
const SCHEMA_SOURCE: &str = "crates/animus-preserves-schema/src/lib.rs";
const DOCS_SOURCE: &str = "docs/preserves-boundary-schemas.md";
const ISSUE_ADAPTER_AMBIENT_RUNTIME: &str = "PRESERVES_ADAPTER_AMBIENT_RUNTIME";
const ISSUE_ADAPTER_MISSING_DEPENDENCY: &str = "PRESERVES_ADAPTER_MISSING_DEPENDENCY";
const ISSUE_FACADE_MISSING_DEPENDENCY: &str = "PRESERVES_FACADE_MISSING_DEPENDENCY";
const ISSUE_FACADE_MISSING_FAMILY: &str = "PRESERVES_FACADE_MISSING_FAMILY";
const ISSUE_CATALOG_INCOMPLETE: &str = "PRESERVES_CATALOG_INCOMPLETE";
const ISSUE_DOCS_INCOMPLETE: &str = "PRESERVES_DOCS_INCOMPLETE";
const REQUIRED_ADAPTER_DEPENDENCIES: &[&str] = &[
    "animus-adapters",
    "animus-preserves-schema",
    "preserves",
    "serde",
];
const FORBIDDEN_ADAPTER_DEPENDENCIES: &[&str] = &["animus-preserves", "animus-runtime"];
const REQUIRED_FACADE_DEPENDENCIES: &[&str] = &[
    "animus-preserves-adapter",
    "animus-preserves-schema",
    "animus-runtime",
];
const REQUIRED_FACADE_FAMILIES: &[&str] = &[
    "pub mod adapter",
    "pub mod policy",
    "pub mod provenance",
    "pub mod review",
    "pub mod runtime",
];
const REQUIRED_CATALOG_MARKERS: &[&str] = &[
    "animus-preserves-adapter",
    "animus-preserves::runtime",
    "animus-preserves::provenance",
    "animus-preserves::policy",
    "animus-preserves::review",
    "animus-preserves::dataspace",
    "safe_metadata_policy",
    "wire_status",
    "fixture_set",
];
const REQUIRED_DOC_MARKERS: &[&str] = &[
    "animus-preserves-adapter",
    "animus-preserves-schema",
    "Family ownership matrix",
    "Review-only",
    "not stable wire",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Issue {
    code: &'static str,
    detail: String,
}

fn dependency_names(manifest: &str) -> BTreeSet<&str> {
    let mut dependencies = BTreeSet::new();
    let mut in_dependencies = false;
    for line in manifest.lines() {
        let trimmed = line.trim();
        if trimmed == DEPENDENCIES_SECTION {
            in_dependencies = true;
            continue;
        }
        if in_dependencies && trimmed.starts_with('[') {
            break;
        }
        if !in_dependencies || trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some((name, _)) = trimmed.split_once('=') {
            dependencies.insert(name.trim());
        }
    }
    dependencies
}

fn validate_sources(
    adapter_manifest: &str,
    facade_manifest: &str,
    facade_source: &str,
    schema_source: &str,
    docs_source: &str,
) -> Vec<Issue> {
    let adapter_dependencies = dependency_names(adapter_manifest);
    let facade_dependencies = dependency_names(facade_manifest);
    let mut issues = Vec::new();

    for dependency in REQUIRED_ADAPTER_DEPENDENCIES {
        if !adapter_dependencies.contains(dependency) {
            issues.push(Issue {
                code: ISSUE_ADAPTER_MISSING_DEPENDENCY,
                detail: format!("adapter boundary is missing {dependency}"),
            });
        }
    }
    for dependency in FORBIDDEN_ADAPTER_DEPENDENCIES {
        if adapter_dependencies.contains(dependency) {
            issues.push(Issue {
                code: ISSUE_ADAPTER_AMBIENT_RUNTIME,
                detail: format!("adapter boundary must not depend on {dependency}"),
            });
        }
    }
    for dependency in REQUIRED_FACADE_DEPENDENCIES {
        if !facade_dependencies.contains(dependency) {
            issues.push(Issue {
                code: ISSUE_FACADE_MISSING_DEPENDENCY,
                detail: format!("compatibility facade is missing {dependency}"),
            });
        }
    }
    for marker in REQUIRED_FACADE_FAMILIES {
        if !facade_source.contains(marker) {
            issues.push(Issue {
                code: ISSUE_FACADE_MISSING_FAMILY,
                detail: format!("compatibility facade is missing {marker}"),
            });
        }
    }
    for marker in REQUIRED_CATALOG_MARKERS {
        if !schema_source.contains(marker) {
            issues.push(Issue {
                code: ISSUE_CATALOG_INCOMPLETE,
                detail: format!("schema catalog is missing {marker}"),
            });
        }
    }
    for marker in REQUIRED_DOC_MARKERS {
        if !docs_source.contains(marker) {
            issues.push(Issue {
                code: ISSUE_DOCS_INCOMPLETE,
                detail: format!("Preserves boundary docs are missing {marker}"),
            });
        }
    }
    issues
}

fn read_source(root: &Path, relative: &str) -> Result<String, String> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|error| format!("failed to read {}: {error}", path.display()))
}

fn validate_root(root: &Path) -> Result<Vec<Issue>, String> {
    let adapter_manifest = read_source(root, ADAPTER_MANIFEST)?;
    let facade_manifest = read_source(root, FACADE_MANIFEST)?;
    let facade_source = read_source(root, FACADE_SOURCE)?;
    let schema_source = read_source(root, SCHEMA_SOURCE)?;
    let docs_source = read_source(root, DOCS_SOURCE)?;
    Ok(validate_sources(
        &adapter_manifest,
        &facade_manifest,
        &facade_source,
        &schema_source,
        &docs_source,
    ))
}

fn self_test() -> Result<(), String> {
    let adapter_manifest = "[dependencies]\nanimus-adapters = {}\nanimus-preserves-schema = {}\npreserves = {}\nserde = {}\n";
    let facade_manifest = "[dependencies]\nanimus-preserves-adapter = {}\nanimus-preserves-schema = {}\nanimus-runtime = {}\n";
    let facade_source = REQUIRED_FACADE_FAMILIES.join("\n");
    let schema_source = REQUIRED_CATALOG_MARKERS.join("\n");
    let docs_source = REQUIRED_DOC_MARKERS.join("\n");
    let positive = validate_sources(
        adapter_manifest,
        facade_manifest,
        &facade_source,
        &schema_source,
        &docs_source,
    );
    if !positive.is_empty() {
        return Err(format!("positive fixture failed: {positive:?}"));
    }

    let negative_manifest = format!("{adapter_manifest}animus-runtime = {{}}\n");
    let negative = validate_sources(
        &negative_manifest,
        facade_manifest,
        &facade_source,
        &schema_source,
        &docs_source,
    );
    if !negative
        .iter()
        .any(|issue| issue.code == ISSUE_ADAPTER_AMBIENT_RUNTIME)
    {
        return Err(String::from(
            "negative fixture did not detect ambient runtime dependency",
        ));
    }
    Ok(())
}

fn parse_root(arguments: &[String]) -> Result<PathBuf, String> {
    if arguments.is_empty() {
        return env::current_dir().map_err(|error| format!("failed to read current dir: {error}"));
    }
    if arguments.len() == ROOT_ARGUMENT_COUNT && arguments[0] == ARG_ROOT {
        return Ok(PathBuf::from(&arguments[1]));
    }
    Err(String::from("usage: check-preserves-boundary-schemas [--root PATH]"))
}

fn run() -> Result<(), String> {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.as_slice() == [ARG_SELF_TEST] {
        self_test()?;
        println!("preserves boundary checker self-test: ok");
        return Ok(());
    }
    let root = parse_root(&arguments)?;
    let issues = validate_root(&root)?;
    if issues.is_empty() {
        println!("preserves boundary schemas: ok");
        return Ok(());
    }
    for issue in issues {
        eprintln!("ERROR {}: {}", issue.code, issue.detail);
    }
    Err(String::from("Preserves boundary schema validation failed"))
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
