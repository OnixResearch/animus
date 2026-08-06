use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const CRATE_NAME: &str = "animus-adapter-weave";
const PINNED_DEPENDENCY: &str = "universal-weave = { version = \"=0.4.1\", default-features = false }";
const LOCK_PACKAGE: &str = "name = \"universal-weave\"";
const LOCK_VERSION: &str = "version = \"0.4.1\"";
const EXCLUDED_LOCK_PACKAGES: &[&str] = &["name = \"rkyv\"", "name = \"loro\""];
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
    "tokio",
    "crossterm",
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

    println!("universal weave adapter checks: ok");
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
                    code: "WEAVE_ADAPTER_MISSING_ARG_VALUE",
                    detail: "--root requires a value".to_string(),
                })?.into();
            }
            other => {
                return Err(ValidationError {
                    code: "WEAVE_ADAPTER_UNKNOWN_ARG",
                    detail: other.to_string(),
                });
            }
        }
    }
    Ok(Config { root })
}

fn validate_all(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();
    collect(validate_crate_manifest(config), &mut errors);
    collect(validate_lockfile(config), &mut errors);
    collect(validate_crate_sources(config), &mut errors);
    collect(validate_workspace_wiring(config), &mut errors);
    collect(validate_registration(config), &mut errors);
    collect(validate_wasm_surface_isolation(config), &mut errors);
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

fn validate_crate_manifest(config: &Config) -> ValidationResult {
    let cargo = read_required(config, "crates/animus-adapter-weave/Cargo.toml")?;
    let mut errors = Vec::new();
    require_contains(&cargo, PINNED_DEPENDENCY, "WEAVE_ADAPTER_PIN_DRIFT", &mut errors);
    require_contains(&cargo, "edition.workspace", "WEAVE_ADAPTER_CRATE_EDITION", &mut errors);
    require_contains(&cargo, "[lints]", "WEAVE_ADAPTER_CRATE_LINTS", &mut errors);
    finish(errors)
}

fn validate_lockfile(config: &Config) -> ValidationResult {
    let lock = read_required(config, "Cargo.lock")?;
    let mut errors = Vec::new();
    require_contains(&lock, LOCK_PACKAGE, "WEAVE_ADAPTER_LOCK_MISSING", &mut errors);
    require_contains(&lock, LOCK_VERSION, "WEAVE_ADAPTER_LOCK_VERSION", &mut errors);
    for excluded in EXCLUDED_LOCK_PACKAGES {
        if lock.contains(excluded) {
            errors.push(ValidationError {
                code: "WEAVE_ADAPTER_EXCLUDED_DEPENDENCY",
                detail: format!("Cargo.lock must not contain {excluded}; keep default features disabled"),
            });
        }
    }
    finish(errors)
}

fn validate_crate_sources(config: &Config) -> ValidationResult {
    let lib = read_required(config, "crates/animus-adapter-weave/src/lib.rs")?;
    let mut errors = Vec::new();
    require_contains(&lib, "#![no_std]", "WEAVE_ADAPTER_NO_STD", &mut errors);
    require_contains(&lib, "#![forbid(unsafe_code)]", "WEAVE_ADAPTER_UNSAFE_FORBID", &mut errors);
    let src_dir = config.root.join("crates/animus-adapter-weave/src");
    let mut sources = Vec::new();
    collect_rs_files(&src_dir, &mut sources);
    for source in sources {
        match fs::read_to_string(&source) {
            Ok(text) => require_no_forbidden_markers(&text, &source, &mut errors),
            Err(error) => errors.push(ValidationError {
                code: "WEAVE_ADAPTER_READ",
                detail: format!("{}: {error}", source.display()),
            }),
        }
    }
    finish(errors)
}

fn validate_workspace_wiring(config: &Config) -> ValidationResult {
    let cargo = read_required(config, "Cargo.toml")?;
    let mut errors = Vec::new();
    require_contains(
        &cargo,
        "\"crates/animus-adapter-weave\"",
        "WEAVE_ADAPTER_WORKSPACE_MISSING",
        &mut errors,
    );
    require_contains(
        &cargo,
        "weave-adapter = [\"dep:animus-adapter-weave\"]",
        "WEAVE_ADAPTER_FEATURE_MISSING",
        &mut errors,
    );
    if let Some(default_members) = section(&cargo, "default-members = [") {
        if default_members.contains(CRATE_NAME) {
            errors.push(ValidationError {
                code: "WEAVE_ADAPTER_DEFAULT_MEMBER",
                detail: "adapter crate must stay out of default-members".to_string(),
            });
        }
    } else {
        errors.push(ValidationError {
            code: "WEAVE_ADAPTER_WORKSPACE_SHAPE",
            detail: "default-members section not found".to_string(),
        });
    }
    finish(errors)
}

fn validate_registration(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();
    match read_required(config, "cairn/component-catalog.ncl") {
        Ok(catalog) => {
            require_contains(&catalog, CRATE_NAME, "WEAVE_ADAPTER_CATALOG_MISSING", &mut errors)
        }
        Err(mut nested) => errors.append(&mut nested),
    }
    match read_required(config, "docs/universal-weave-adapter.md") {
        Ok(docs) => {
            require_contains(&docs, "Non-claims", "WEAVE_ADAPTER_DOCS_NONCLAIMS", &mut errors)
        }
        Err(mut nested) => errors.append(&mut nested),
    }
    match read_required(config, "README.md") {
        Ok(readme) => {
            require_contains(&readme, "universal-weave", "WEAVE_ADAPTER_README_MISSING", &mut errors)
        }
        Err(mut nested) => errors.append(&mut nested),
    }
    finish(errors)
}

fn validate_wasm_surface_isolation(config: &Config) -> ValidationResult {
    let rail = read_required(config, "scripts/cairn/check-wasm-surface.rs")?;
    let mut errors = Vec::new();
    if rail.contains(CRATE_NAME) {
        errors.push(ValidationError {
            code: "WEAVE_ADAPTER_WASM_SURFACE_DRIFT",
            detail: "supported Wasm surface must not name the adapter crate without promotion evidence"
                .to_string(),
        });
    }
    finish(errors)
}

fn collect_rs_files(dir: &Path, output: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_rs_files(&path, output);
        } else if path.extension().is_some_and(|extension| extension == "rs") {
            output.push(path);
        }
    }
}

fn section<'a>(text: &'a str, start: &str) -> Option<&'a str> {
    let begin = text.find(start)?;
    let rest = &text[begin..];
    let end = rest.find(']')?;
    Some(&rest[..end])
}

fn require_no_forbidden_markers(text: &str, path: &Path, errors: &mut Vec<ValidationError>) {
    for marker in FORBIDDEN_AUTHORITY_MARKERS {
        if text.contains(marker) {
            errors.push(ValidationError {
                code: "WEAVE_ADAPTER_FORBIDDEN_AUTHORITY",
                detail: format!("{}: {marker}", path.display()),
            });
        }
    }
}

fn read_required(config: &Config, relative: &str) -> Result<String, Vec<ValidationError>> {
    let path = config.root.join(relative);
    fs::read_to_string(&path).map_err(|error| {
        vec![ValidationError {
            code: "WEAVE_ADAPTER_READ",
            detail: format!("{}: {error}", path.display()),
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

fn finish(errors: Vec<ValidationError>) -> ValidationResult {
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
