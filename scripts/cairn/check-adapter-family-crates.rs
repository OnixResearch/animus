use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const FAMILY_CRATES: &[&str] = &[
    "animus-adapter-safe",
    "animus-adapter-observability",
    "animus-adapter-rlm-kernel",
];
const FORBIDDEN_AUTHORITY_MARKERS: &[&str] = &[
    "std::fs",
    "std::process",
    "std::net",
    "std::env",
    "std::time",
    "Command::new",
    "TcpStream",
    "UdpSocket",
    "reqwest",
    "tokio::fs",
    "crossterm",
];
const UNSAFE_RECEIPT_MARKERS: &[&str] = &[
    "token=secret",
    "RAW_PROMPT:",
    "PROVIDER_PAYLOAD:",
    "UNBOUNDED_OUTPUT:",
];

#[derive(Debug)]
struct Config {
    root: PathBuf,
}

#[derive(Debug)]
struct ValidationError {
    code: &'static str,
    detail: String,
}

type ValidationResult = Result<(), Vec<ValidationError>>;

fn main() {
    let config = match parse_args(env::args().skip(1)) {
        Ok(config) => config,
        Err(error) => {
            eprintln!("{}: {}", error.code, error.detail);
            std::process::exit(1);
        }
    };

    if let Err(errors) = validate_all(&config) {
        for error in errors {
            eprintln!("{}: {}", error.code, error.detail);
        }
        std::process::exit(1);
    }

    println!("adapter family crates: ok families={}", FAMILY_CRATES.len());
}

fn parse_args<I>(args: I) -> Result<Config, ValidationError>
where
    I: IntoIterator<Item = String>,
{
    let mut root = PathBuf::from(".");
    let mut iter = args.into_iter();
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--root" => {
                root = iter.next().ok_or_else(|| ValidationError {
                    code: "ADAPTER_FAMILY_MISSING_ARG_VALUE",
                    detail: "--root requires a value".to_string(),
                })?.into();
            }
            other => {
                return Err(ValidationError {
                    code: "ADAPTER_FAMILY_UNKNOWN_ARG",
                    detail: other.to_string(),
                });
            }
        }
    }
    Ok(Config { root })
}

fn validate_all(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();
    collect(validate_workspace(config), &mut errors);
    collect(validate_family_crates(config), &mut errors);
    collect(validate_facade(config), &mut errors);
    collect(validate_metadata(config), &mut errors);
    collect(validate_fixtures(config), &mut errors);
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn collect(result: ValidationResult, errors: &mut Vec<ValidationError>) {
    if let Err(mut nested) = result {
        errors.append(&mut nested);
    }
}

fn validate_workspace(config: &Config) -> ValidationResult {
    let cargo = read_required(config, "Cargo.toml")?;
    let lock = read_required(config, "Cargo.lock")?;
    let mut errors = Vec::new();
    for family in FAMILY_CRATES {
        require_contains(&cargo, family, "ADAPTER_FAMILY_WORKSPACE_MISSING", &mut errors);
        require_contains(&lock, family, "ADAPTER_FAMILY_LOCK_MISSING", &mut errors);
    }
    finish(errors)
}

fn validate_family_crates(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();
    for family in FAMILY_CRATES {
        let cargo_path = format!("crates/{family}/Cargo.toml");
        let lib_path = format!("crates/{family}/src/lib.rs");
        match read_required(config, &cargo_path) {
            Ok(cargo) => {
                require_contains(&cargo, "edition.workspace", "ADAPTER_FAMILY_CRATE_EDITION", &mut errors);
                require_contains(&cargo, "[lints]", "ADAPTER_FAMILY_CRATE_LINTS", &mut errors);
            }
            Err(mut nested) => errors.append(&mut nested),
        }
        match read_required(config, &lib_path) {
            Ok(lib) => {
                require_contains(&lib, "#![no_std]", "ADAPTER_FAMILY_NO_STD", &mut errors);
                require_contains(&lib, "#![forbid(unsafe_code)]", "ADAPTER_FAMILY_UNSAFE_FORBID", &mut errors);
                require_no_forbidden_markers(&lib, &lib_path, &mut errors);
            }
            Err(mut nested) => errors.append(&mut nested),
        }
    }
    finish(errors)
}

fn validate_facade(config: &Config) -> ValidationResult {
    let adapters_cargo = read_required(config, "crates/animus-adapters/Cargo.toml")?;
    let adapters_lib = read_required(config, "crates/animus-adapters/src/lib.rs")?;
    let tests = read_required(config, "crates/animus-adapters/src/familycrates.rs")?;
    let mut errors = Vec::new();
    for family in FAMILY_CRATES {
        require_contains(
            &adapters_cargo,
            family,
            "ADAPTER_FAMILY_FACADE_DEP_MISSING",
            &mut errors,
        );
    }
    require_contains(
        &adapters_lib,
        "pub mod observability {",
        "ADAPTER_FAMILY_OBSERVABILITY_FACADE",
        &mut errors,
    );
    require_contains(
        &tests,
        "observability_types_match_facade",
        "ADAPTER_FAMILY_PARITY_TEST_MISSING",
        &mut errors,
    );
    require_contains(
        &adapters_lib,
        "pub mod rlm_kernel {",
        "ADAPTER_FAMILY_RLM_KERNEL_FACADE",
        &mut errors,
    );
    require_contains(
        &tests,
        "rlm_kernel_types_match_facade",
        "ADAPTER_FAMILY_RLM_KERNEL_PARITY_TEST_MISSING",
        &mut errors,
    );
    require_contains(
        &tests,
        "unsafe_receipt_labels_are_rejected",
        "ADAPTER_FAMILY_NEGATIVE_TEST_MISSING",
        &mut errors,
    );
    finish(errors)
}

fn validate_metadata(config: &Config) -> ValidationResult {
    let component = read_required(config, "cairn/component-catalog.ncl")?;
    let contracts = read_required(config, "docs/contracts-index.md")?;
    let conformance = read_required(config, "cairn/adapter-conformance-index.ncl")?;
    let docs = read_required(config, "docs/adapter-family-crates.md")?;
    let readme = read_required(config, "README.md")?;
    let mut errors = Vec::new();
    for family in FAMILY_CRATES {
        require_contains(&component, family, "ADAPTER_FAMILY_COMPONENT_DRIFT", &mut errors);
        require_contains(&contracts, family, "ADAPTER_FAMILY_CONTRACTS_DRIFT", &mut errors);
        require_contains(&conformance, family, "ADAPTER_FAMILY_CONFORMANCE_DRIFT", &mut errors);
        require_contains(&docs, family, "ADAPTER_FAMILY_DOCS_DRIFT", &mut errors);
    }
    require_contains(
        &component,
        "scripts/cairn/check-adapter-family-crates --root .",
        "ADAPTER_FAMILY_COMPONENT_CHECK_MISSING",
        &mut errors,
    );
    require_contains(
        &readme,
        "docs/adapter-family-crates.md",
        "ADAPTER_FAMILY_README_LINK_MISSING",
        &mut errors,
    );
    finish(errors)
}

fn validate_fixtures(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();
    check_fixture(
        config,
        "cairn/adapter-family-crates-fixtures/valid-family-split.txt",
        None,
        &mut errors,
    );
    check_fixture(
        config,
        "cairn/adapter-family-crates-fixtures/negative-missing-facade-export.txt",
        Some("ADAPTER_FAMILY_FIXTURE_MISSING_FACADE"),
        &mut errors,
    );
    check_fixture(
        config,
        "cairn/adapter-family-crates-fixtures/negative-dependency-cycle.txt",
        Some("ADAPTER_FAMILY_FIXTURE_DEPENDENCY_CYCLE"),
        &mut errors,
    );
    check_fixture(
        config,
        "cairn/adapter-family-crates-fixtures/negative-authority-leak.txt",
        Some("ADAPTER_FAMILY_FIXTURE_AUTHORITY_LEAK"),
        &mut errors,
    );
    check_fixture(
        config,
        "cairn/adapter-family-crates-fixtures/negative-unsafe-receipt.txt",
        Some("ADAPTER_FAMILY_FIXTURE_UNSAFE_RECEIPT"),
        &mut errors,
    );
    finish(errors)
}

fn check_fixture(
    config: &Config,
    relative: &str,
    expected_error: Option<&'static str>,
    errors: &mut Vec<ValidationError>,
) {
    let text = match read_required(config, relative) {
        Ok(text) => text,
        Err(mut nested) => {
            errors.append(&mut nested);
            return;
        }
    };
    match validate_fixture_text(&text) {
        Ok(()) => {
            if let Some(code) = expected_error {
                errors.push(ValidationError {
                    code: "ADAPTER_FAMILY_NEGATIVE_FIXTURE_PASSED",
                    detail: format!("{relative} expected {code}"),
                });
            } else {
                println!("positive fixture ok: {relative}");
            }
        }
        Err(error) => {
            if expected_error == Some(error.code) {
                println!("negative fixture ok: {relative} -> {}", error.code);
            } else {
                errors.push(error);
            }
        }
    }
}

fn validate_fixture_text(text: &str) -> Result<(), ValidationError> {
    let fields = parse_fixture(text);
    if !fields.contains_key("facade") {
        return Err(fixture_error("ADAPTER_FAMILY_FIXTURE_MISSING_FACADE", "facade"));
    }
    if has_cycle(&fields) {
        return Err(fixture_error("ADAPTER_FAMILY_FIXTURE_DEPENDENCY_CYCLE", "edge"));
    }
    if fields
        .get("authority")
        .into_iter()
        .flatten()
        .any(|value| value != "none")
        || fields.contains_key("forbidden")
    {
        return Err(fixture_error("ADAPTER_FAMILY_FIXTURE_AUTHORITY_LEAK", "authority"));
    }
    if fields
        .get("receipt")
        .into_iter()
        .flatten()
        .any(|value| contains_any(value, UNSAFE_RECEIPT_MARKERS))
    {
        return Err(fixture_error("ADAPTER_FAMILY_FIXTURE_UNSAFE_RECEIPT", "receipt"));
    }
    if !fields.contains_key("crate") || !fields.contains_key("check") {
        return Err(fixture_error("ADAPTER_FAMILY_FIXTURE_INCOMPLETE", "crate/check"));
    }
    Ok(())
}

fn parse_fixture(text: &str) -> BTreeMap<String, Vec<String>> {
    let mut fields: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for line in text.lines() {
        let Some((key, value)) = line.split_once('=') else {
            continue;
        };
        fields
            .entry(key.trim().to_string())
            .or_default()
            .push(value.trim().to_string());
    }
    fields
}

fn has_cycle(fields: &BTreeMap<String, Vec<String>>) -> bool {
    let Some(edges) = fields.get("edge") else {
        return false;
    };
    let mut graph: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for edge in edges {
        let Some((from, to)) = edge.split_once("->") else {
            continue;
        };
        graph.entry(from).or_default().insert(to);
    }
    graph.iter().any(|(from, targets)| {
        targets
            .iter()
            .any(|target| graph.get(target).is_some_and(|reverse| reverse.contains(from)))
    })
}

fn require_no_forbidden_markers(text: &str, path: &str, errors: &mut Vec<ValidationError>) {
    for marker in FORBIDDEN_AUTHORITY_MARKERS {
        if text.contains(marker) {
            errors.push(ValidationError {
                code: "ADAPTER_FAMILY_FORBIDDEN_AUTHORITY",
                detail: format!("{path}: {marker}"),
            });
        }
    }
}

fn read_required(config: &Config, relative: &str) -> Result<String, Vec<ValidationError>> {
    let path = config.root.join(relative);
    fs::read_to_string(&path).map_err(|error| {
        vec![ValidationError {
            code: "ADAPTER_FAMILY_READ",
            detail: format!("{}: {error}", display_path(&path)),
        }]
    })
}

fn require_contains(text: &str, needle: &str, code: &'static str, errors: &mut Vec<ValidationError>) {
    if !text.contains(needle) {
        errors.push(ValidationError {
            code,
            detail: needle.to_string(),
        });
    }
}

fn contains_any(value: &str, markers: &[&str]) -> bool {
    markers.iter().any(|marker| value.contains(marker))
}

fn fixture_error(code: &'static str, detail: &str) -> ValidationError {
    ValidationError {
        code,
        detail: detail.to_string(),
    }
}

fn finish(errors: Vec<ValidationError>) -> ValidationResult {
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn display_path(path: &Path) -> String {
    path.to_string_lossy().into_owned()
}
