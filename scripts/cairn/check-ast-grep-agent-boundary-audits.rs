use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const BLAKE3_HEX_LENGTH: usize = 64;
const EXPECTED_PROFILE_COUNT: usize = 7;
const EXPECTED_RULE_COUNT: usize = 5;

const PROFILES: &[&str] = &[
    "core",
    "adapter-dto",
    "runtime",
    "std-shell",
    "steel-loop-fabric",
    "tui-host",
    "generated-config",
];

const RULES: &[RuleSpec] = &[
    RuleSpec {
        id: "ambient-filesystem-call",
        category: "ambient-authority",
        positive_token: "std::fs::read_to_string",
        negative_fixture: "negative-host-shell-allowed.rs",
    },
    RuleSpec {
        id: "ambient-process-command",
        category: "ambient-authority",
        positive_token: "std::process::Command::new",
        negative_fixture: "negative-host-shell-allowed.rs",
    },
    RuleSpec {
        id: "unsafe-block",
        category: "unsafe-code",
        positive_token: "unsafe {",
        negative_fixture: "negative-pure-reducer.rs",
    },
    RuleSpec {
        id: "panic-macro",
        category: "panic-path",
        positive_token: "panic!",
        negative_fixture: "negative-pure-reducer.rs",
    },
    RuleSpec {
        id: "direct-provider-client",
        category: "provider-boundary",
        positive_token: "OpenAiClient::new",
        negative_fixture: "negative-injected-provider.rs",
    },
];

const NON_CLAIMS: &[&str] = &[
    "not-a-safety-proof",
    "not-adapter-correctness",
    "not-runtime-safety",
    "not-provider-behavior",
    "not-steel-policy-enforcement",
    "not-release-readiness",
];

const REQUIRED_DOC_SNIPPETS: &[&str] = &[
    "inventory-only structural audit rail",
    "candidate review evidence",
    "BLAKE3 rule-bundle identity",
    "bounded finding counts",
    "Receipts intentionally omit raw source payloads",
    "nix build .#checks.x86_64-linux.ast-grep-agent-boundary-audits --no-link -L",
];

#[derive(Clone, Copy)]
struct RuleSpec {
    id: &'static str,
    category: &'static str,
    positive_token: &'static str,
    negative_fixture: &'static str,
}

#[derive(Debug)]
struct Config {
    root: PathBuf,
    ast_grep_version: String,
    rule_bundle_hash: String,
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

    println!(
        "ast-grep agent boundary audits: ok profiles={} rules={} bundle={} tool={}",
        EXPECTED_PROFILE_COUNT, EXPECTED_RULE_COUNT, config.rule_bundle_hash, config.ast_grep_version
    );
}

fn parse_args<I>(args: I) -> Result<Config, ValidationError>
where
    I: IntoIterator<Item = String>,
{
    let mut root = PathBuf::from(".");
    let mut ast_grep_version = None;
    let mut rule_bundle_hash = None;
    let mut iter = args.into_iter();

    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--root" => {
                root = next_value(&mut iter, "--root")?.into();
            }
            "--ast-grep-version" => {
                ast_grep_version = Some(next_value(&mut iter, "--ast-grep-version")?);
            }
            "--rule-bundle-hash" => {
                rule_bundle_hash = Some(next_value(&mut iter, "--rule-bundle-hash")?);
            }
            other => {
                return Err(ValidationError {
                    code: "AST_GREP_AUDIT_UNKNOWN_ARG",
                    detail: other.to_string(),
                });
            }
        }
    }

    let ast_grep_version = ast_grep_version.ok_or_else(|| ValidationError {
        code: "AST_GREP_AUDIT_MISSING_TOOL_VERSION",
        detail: "--ast-grep-version is required".to_string(),
    })?;
    let rule_bundle_hash = rule_bundle_hash.ok_or_else(|| ValidationError {
        code: "AST_GREP_AUDIT_MISSING_BUNDLE_HASH",
        detail: "--rule-bundle-hash is required".to_string(),
    })?;

    Ok(Config {
        root,
        ast_grep_version,
        rule_bundle_hash,
    })
}

fn next_value<I>(iter: &mut I, flag: &'static str) -> Result<String, ValidationError>
where
    I: Iterator<Item = String>,
{
    iter.next().ok_or_else(|| ValidationError {
        code: "AST_GREP_AUDIT_MISSING_ARG_VALUE",
        detail: format!("{flag} requires a value"),
    })
}

fn validate_all(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();

    collect(validate_tool_identity(config), &mut errors);
    collect(validate_profile_source(config), &mut errors);
    collect(validate_rule_files(config), &mut errors);
    collect(validate_fixtures(config), &mut errors);
    collect(validate_docs(config), &mut errors);
    collect(validate_readme_link(config), &mut errors);

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

fn validate_tool_identity(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();
    if !config.ast_grep_version.starts_with("ast-grep ") {
        errors.push(ValidationError {
            code: "AST_GREP_AUDIT_TOOL_VERSION",
            detail: config.ast_grep_version.clone(),
        });
    }
    if config.rule_bundle_hash.len() != BLAKE3_HEX_LENGTH
        || !config.rule_bundle_hash.chars().all(|ch| ch.is_ascii_hexdigit())
    {
        errors.push(ValidationError {
            code: "AST_GREP_AUDIT_BUNDLE_HASH",
            detail: config.rule_bundle_hash.clone(),
        });
    }
    finish(errors)
}

fn validate_profile_source(config: &Config) -> ValidationResult {
    let source = read_required(config, ".cairn/ast-grep-agent-boundary-audits.ncl")?;
    let mut errors = Vec::new();

    for profile in PROFILES {
        require_contains(&source, profile, "AST_GREP_AUDIT_MISSING_PROFILE", &mut errors);
    }
    for rule in RULES {
        require_contains(&source, rule.id, "AST_GREP_AUDIT_MISSING_RULE", &mut errors);
        require_contains(
            &source,
            rule.category,
            "AST_GREP_AUDIT_MISSING_RULE_CATEGORY",
            &mut errors,
        );
    }
    for non_claim in NON_CLAIMS {
        require_contains(
            &source,
            non_claim,
            "AST_GREP_AUDIT_MISSING_NON_CLAIM",
            &mut errors,
        );
    }
    require_contains(
        &source,
        "BLAKE3 over .cairn/ast-grep-agent-boundary-rules/*.yml",
        "AST_GREP_AUDIT_MISSING_BUNDLE_SCHEMA",
        &mut errors,
    );
    require_contains(
        &source,
        "findings_summary = \"counts only; no raw source payloads\"",
        "AST_GREP_AUDIT_UNSAFE_RECEIPT_SCHEMA",
        &mut errors,
    );

    finish(errors)
}

fn validate_rule_files(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();
    for rule in RULES {
        let relative = format!(".cairn/ast-grep-agent-boundary-rules/{}.yml", rule.id);
        match read_required(config, &relative) {
            Ok(text) => {
                require_contains(
                    &text,
                    &format!("id: {}", rule.id),
                    "AST_GREP_AUDIT_RULE_ID_DRIFT",
                    &mut errors,
                );
                require_contains(
                    &text,
                    "language: Rust",
                    "AST_GREP_AUDIT_RULE_LANGUAGE",
                    &mut errors,
                );
                require_contains(
                    &text,
                    "severity: hint",
                    "AST_GREP_AUDIT_RULE_SEVERITY",
                    &mut errors,
                );
                require_contains(
                    &text,
                    "inventory-only",
                    "AST_GREP_AUDIT_RULE_POSTURE",
                    &mut errors,
                );
                require_contains(&text, "rule:", "AST_GREP_AUDIT_RULE_BODY", &mut errors);
            }
            Err(mut nested) => errors.append(&mut nested),
        }
    }
    finish(errors)
}

fn validate_fixtures(config: &Config) -> ValidationResult {
    let mut errors = Vec::new();
    let positive = match read_required(
        config,
        ".cairn/ast-grep-agent-boundary-fixtures/positive-boundary-violations.rs",
    ) {
        Ok(text) => text,
        Err(error) => return Err(error),
    };

    for rule in RULES {
        require_contains(
            &positive,
            rule.positive_token,
            "AST_GREP_AUDIT_POSITIVE_FIXTURE_MISS",
            &mut errors,
        );
        let negative_path = format!(
            ".cairn/ast-grep-agent-boundary-fixtures/{}",
            rule.negative_fixture
        );
        match read_required(config, &negative_path) {
            Ok(text) => {
                if text.contains(rule.positive_token) {
                    errors.push(ValidationError {
                        code: "AST_GREP_AUDIT_NEGATIVE_FIXTURE_LEAK",
                        detail: format!("{} contains {}", negative_path, rule.positive_token),
                    });
                }
                require_contains(
                    &text,
                    "fn ",
                    "AST_GREP_AUDIT_NEGATIVE_FIXTURE_TOO_EMPTY",
                    &mut errors,
                );
            }
            Err(mut nested) => errors.append(&mut nested),
        }
    }

    finish(errors)
}

fn validate_docs(config: &Config) -> ValidationResult {
    let docs = read_required(config, "docs/ast-grep-agent-boundary-audits.md")?;
    let mut errors = Vec::new();
    for snippet in REQUIRED_DOC_SNIPPETS {
        require_contains(&docs, snippet, "AST_GREP_AUDIT_DOCS_DRIFT", &mut errors);
    }
    for profile in PROFILES {
        require_contains(&docs, profile, "AST_GREP_AUDIT_DOCS_PROFILE", &mut errors);
    }
    for rule in RULES {
        require_contains(&docs, rule.id, "AST_GREP_AUDIT_DOCS_RULE", &mut errors);
    }
    finish(errors)
}

fn validate_readme_link(config: &Config) -> ValidationResult {
    let readme = read_required(config, "README.md")?;
    let mut errors = Vec::new();
    require_contains(
        &readme,
        "docs/ast-grep-agent-boundary-audits.md",
        "AST_GREP_AUDIT_README_LINK",
        &mut errors,
    );
    require_contains(
        &readme,
        "check-ast-grep-agent-boundary-audits",
        "AST_GREP_AUDIT_README_CHECK",
        &mut errors,
    );
    finish(errors)
}

fn read_required(config: &Config, relative: &str) -> Result<String, Vec<ValidationError>> {
    let path = config.root.join(relative);
    fs::read_to_string(&path).map_err(|error| {
        vec![ValidationError {
            code: "AST_GREP_AUDIT_READ",
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
