use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARCHITECTURE_NOTE: &str = "docs/plugin-manifest-boundary.md";
const MANIFEST_CORE_PATHS: &[&str] = &[
    "crates/animus-adapters/src/manifest.rs",
    "crates/animus-adapters/src/manifest/accessors.rs",
    "crates/animus-adapters/src/manifest/error.rs",
    "crates/animus-adapters/src/manifest/validation.rs",
    "crates/animus-adapters/src/manifest/validation/path.rs",
];
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const REQUIRED_DOC_PHRASES: &[&str] = &[
    "bounded reference for descriptor shape",
    "not as runtime behavior",
    "must not launch stdio processes",
    "load Wasm modules",
    "inherit environment values",
    "read secrets",
    "execute tools",
    "does not grant authority",
    "separate injected service boundary",
];

const FORBIDDEN_CORE_PATTERNS: &[ForbiddenPattern] = &[
    ForbiddenPattern {
        label: "process launch",
        tokens: &["std::process", "process::Command", "Command::new", "tokio::process"],
    },
    ForbiddenPattern {
        label: "filesystem read",
        tokens: &["std::fs", "fs::read", "read_to_string", "File::open"],
    },
    ForbiddenPattern {
        label: "environment read",
        tokens: &["std::env", "env::var", "vars_os"],
    },
    ForbiddenPattern {
        label: "network connection",
        tokens: &["std::net", "TcpStream", "UdpSocket", "reqwest", "ureq"],
    },
    ForbiddenPattern {
        label: "wasm loader",
        tokens: &["wasmtime::", "wasmer::", "extism::"],
    },
    ForbiddenPattern {
        label: "tool dispatch",
        tokens: &["ToolAdapter", "ComponentAdapter", ".call("],
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ForbiddenPattern {
    label: &'static str,
    tokens: &'static [&'static str],
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Issue {
    path: String,
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
    let mut issues = Vec::new();
    issues.extend(validate_architecture_note(&config.root));
    issues.extend(validate_manifest_core(&config.root));
    issues.sort();

    for issue in &issues {
        println!("ERROR {} -- {}", issue.path, issue.detail);
    }

    if issues.is_empty() {
        println!("plugin manifest runtime-neutrality rail: ok");
        Ok(())
    } else {
        Err(format!(
            "plugin manifest runtime-neutrality rail found {} issue(s)",
            issues.len()
        ))
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
            ARCHITECTURE_NOTE,
            "missing plugin manifest architecture note",
        )];
    };

    let mut issues = Vec::new();
    for phrase in REQUIRED_DOC_PHRASES {
        if !text.contains(phrase) {
            issues.push(issue(
                ARCHITECTURE_NOTE,
                &format!("missing required documentation phrase: {phrase}"),
            ));
        }
    }
    issues
}

fn validate_manifest_core(root: &Path) -> Vec<Issue> {
    let mut issues = Vec::new();
    for manifest_core in MANIFEST_CORE_PATHS {
        let core_path = root.join(manifest_core);
        let Ok(text) = fs::read_to_string(&core_path) else {
            issues.push(issue(manifest_core, "missing plugin manifest core"));
            continue;
        };

        for pattern in FORBIDDEN_CORE_PATTERNS {
            for token in pattern.tokens {
                if text.contains(token) {
                    issues.push(issue(
                        manifest_core,
                        &format!(
                            "descriptor core contains forbidden {} token: {token}",
                            pattern.label
                        ),
                    ));
                }
            }
        }
    }
    issues
}

fn issue(path: &str, detail: &str) -> Issue {
    Issue {
        path: path.to_string(),
        detail: detail.to_string(),
    }
}
