use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const ARG_OUT: &str = "--out";
const ARG_VCS: &str = "--vcs";
const ARG_COMMIT: &str = "--commit";
const ARG_BRANCH: &str = "--branch";
const ARG_DIRTY: &str = "--dirty";
const ARG_CHECK: &str = "--check";
const ARG_STALE_CHECK: &str = "--stale-check";
const ARG_MISSING_MODE: &str = "--missing-mode";
const ARG_CLAIM_CURRENT_HEAD: &str = "--claim-current-head";
const ARG_CLAIM: &str = "--claim";
const ARG_NON_CLAIM: &str = "--non-claim";
const ARG_NO_DEFAULT_NON_CLAIMS: &str = "--no-default-non-claims";
const ARG_REAL_CHECK_RECEIPT: &str = "--real-check-receipt";
const VCS_GIT: &str = "git";
const DETACHED_BRANCH: &str = "DETACHED";
const STATUS_PASS: &str = "pass";
const STATUS_FAIL: &str = "fail";
const STATUS_SKIPPED: &str = "skipped";
const STATUS_MISSING: &str = "missing";
const CLASS_PURE: &str = "pure";
const CLASS_OPTIONAL: &str = "optional";
const BOOL_TRUE: &str = "true";
const BOOL_FALSE: &str = "false";
const FIELD_SEPARATOR: &str = "|";
const ASSIGN_SEPARATOR: char = '=';
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const MODE_FOCUSED_RAILS: &str = "focused-rails";
const MODE_WORKSPACE_TESTS: &str = "workspace-tests";
const MODE_CLIPPY: &str = "clippy";
const MODE_FLAKE_CHECK: &str = "flake-check";
const MODE_NICKEL_DRIFT: &str = "nickel-drift";
const MODE_CAIRN_VALIDATE: &str = "cairn-validate";
const MODE_CAIRN_GATES: &str = "cairn-gates";
const MODE_PACKAGED_HEADLESS_DOGFOOD_SMOKE: &str = "packaged-headless-dogfood-smoke";

const CLAIM_CURRENT_HEAD_PURE_EVIDENCE: &str = "current-head-pure-evidence";
const CLAIM_PACKAGED_HEADLESS_DOGFOOD: &str = "packaged-headless-dogfood";
const CLAIM_BOUNDED_REAL_CHECK_COMPLETION: &str = "bounded-real-check-completion";
const REAL_CHECK_KIND: &str = "real_check_receipt";
const SURFACE_LIVE_PROVIDERS: &str = "live-providers";
const SURFACE_TUI_DAEMON_DOGFOOD: &str = "tui-daemon-dogfood";
const SURFACE_VM_CHECKS: &str = "vm-checks";
const SURFACE_REMOTE_SERVICES: &str = "remote-services";
const SURFACE_PLUGIN_RUNTIME: &str = "plugin-runtime";
const SURFACE_PRODUCTION_READINESS: &str = "production-readiness";

const REQUIRED_PURE_MODES: &[&str] = &[
    MODE_FOCUSED_RAILS,
    MODE_WORKSPACE_TESTS,
    MODE_CLIPPY,
    MODE_FLAKE_CHECK,
    MODE_NICKEL_DRIFT,
    MODE_CAIRN_VALIDATE,
    MODE_CAIRN_GATES,
];
const ADDITIONAL_PURE_MODES: &[&str] = &[MODE_PACKAGED_HEADLESS_DOGFOOD_SMOKE];
const OPTIONAL_SURFACES: &[&str] = &[
    SURFACE_LIVE_PROVIDERS,
    SURFACE_TUI_DAEMON_DOGFOOD,
    SURFACE_VM_CHECKS,
    SURFACE_REMOTE_SERVICES,
    SURFACE_PLUGIN_RUNTIME,
    SURFACE_PRODUCTION_READINESS,
];
const CLAIM_SURFACES: &[&str] = &[
    CLAIM_CURRENT_HEAD_PURE_EVIDENCE,
    CLAIM_PACKAGED_HEADLESS_DOGFOOD,
    CLAIM_BOUNDED_REAL_CHECK_COMPLETION,
    SURFACE_LIVE_PROVIDERS,
    SURFACE_TUI_DAEMON_DOGFOOD,
    SURFACE_VM_CHECKS,
    SURFACE_REMOTE_SERVICES,
    SURFACE_PLUGIN_RUNTIME,
    SURFACE_PRODUCTION_READINESS,
];
const STATUSES: &[&str] = &[STATUS_PASS, STATUS_FAIL, STATUS_SKIPPED, STATUS_MISSING];
const REAL_CHECK_RECEIPT_KEYS: &[&str] = &[
    "schema",
    "rail",
    "mode",
    "status",
    "head",
    "head_dirty",
    "source_blake3",
    "selected_check_ids",
    "cairn_binding",
    "claims",
    "non_claims",
];
const REAL_CHECK_REQUIRED_KEYS: &[&str] = &[
    "schema",
    "rail",
    "mode",
    "status",
    "head",
    "head_dirty",
    "source_blake3",
    "selected_check_ids",
    "cairn_binding",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct HeadInput {
    vcs: String,
    commit: String,
    branch: String,
    dirty: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CheckInput {
    mode: String,
    status: String,
    head_match: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct MissingModeInput {
    mode: String,
    reason: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RealCheckReceiptInput {
    schema: String,
    rail: String,
    mode: String,
    status: String,
    head: String,
    head_dirty: bool,
    source_blake3: String,
    selected_check_ids: String,
    cairn_binding: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct RenderInput {
    head: HeadInput,
    checks: Vec<CheckInput>,
    missing_modes: Vec<MissingModeInput>,
    claims: BTreeSet<String>,
    non_claims: BTreeSet<String>,
    real_check_receipt: Option<RealCheckReceiptInput>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct Config {
    out: Option<PathBuf>,
    vcs: Option<String>,
    commit: Option<String>,
    branch: Option<String>,
    dirty: Option<bool>,
    checks: Vec<CheckInput>,
    missing_modes: Vec<MissingModeInput>,
    claims: BTreeSet<String>,
    non_claims: BTreeSet<String>,
    default_non_claims: bool,
    real_check_receipt: Option<PathBuf>,
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
    let input = build_render_input(&config)?;
    let text = render_index(&input);
    write_output(config.out.as_ref(), text.as_str())
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut config = Config {
        default_non_claims: true,
        ..Config::default()
    };

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_OUT => config.out = Some(PathBuf::from(required_value(&mut args, ARG_OUT)?)),
            ARG_VCS => config.vcs = Some(required_value(&mut args, ARG_VCS)?),
            ARG_COMMIT => config.commit = Some(required_value(&mut args, ARG_COMMIT)?),
            ARG_BRANCH => config.branch = Some(required_value(&mut args, ARG_BRANCH)?),
            ARG_DIRTY => config.dirty = Some(parse_bool(&required_value(&mut args, ARG_DIRTY)?)?),
            ARG_CHECK => config.checks.push(parse_check_arg(
                &required_value(&mut args, ARG_CHECK)?,
                true,
            )?),
            ARG_STALE_CHECK => config.checks.push(parse_check_arg(
                &required_value(&mut args, ARG_STALE_CHECK)?,
                false,
            )?),
            ARG_MISSING_MODE => {
                config
                    .missing_modes
                    .push(parse_missing_mode_arg(&required_value(
                        &mut args,
                        ARG_MISSING_MODE,
                    )?)?)
            }
            ARG_CLAIM_CURRENT_HEAD => {
                config
                    .claims
                    .insert(CLAIM_CURRENT_HEAD_PURE_EVIDENCE.to_owned());
            }
            ARG_CLAIM => {
                config
                    .claims
                    .insert(parse_claim_surface(&required_value(&mut args, ARG_CLAIM)?)?);
            }
            ARG_NON_CLAIM => {
                config
                    .non_claims
                    .insert(parse_claim_surface(&required_value(
                        &mut args,
                        ARG_NON_CLAIM,
                    )?)?);
            }
            ARG_NO_DEFAULT_NON_CLAIMS => config.default_non_claims = false,
            ARG_REAL_CHECK_RECEIPT => {
                config.real_check_receipt = Some(PathBuf::from(required_value(
                    &mut args,
                    ARG_REAL_CHECK_RECEIPT,
                )?));
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(config)
}

fn required_value(
    args: &mut impl Iterator<Item = String>,
    argument: &str,
) -> Result<String, String> {
    args.next()
        .ok_or_else(|| format!("{argument} requires a value"))
}

fn parse_check_arg(value: &str, head_match: bool) -> Result<CheckInput, String> {
    let (mode, status) = value
        .split_once(ASSIGN_SEPARATOR)
        .ok_or_else(|| format!("check must use MODE=STATUS: {value}"))?;
    validate_mode(mode)?;
    if !contains(STATUSES, status) {
        return Err(format!("unknown check status: {status}"));
    }
    Ok(CheckInput {
        mode: mode.to_owned(),
        status: status.to_owned(),
        head_match,
    })
}

fn parse_missing_mode_arg(value: &str) -> Result<MissingModeInput, String> {
    let (mode, reason) = value
        .split_once(ASSIGN_SEPARATOR)
        .ok_or_else(|| format!("missing mode must use MODE=REASON: {value}"))?;
    validate_mode(mode)?;
    validate_label(reason, "missing-mode reason")?;
    Ok(MissingModeInput {
        mode: mode.to_owned(),
        reason: reason.to_owned(),
    })
}

fn parse_claim_surface(value: &str) -> Result<String, String> {
    if contains(CLAIM_SURFACES, value) {
        Ok(value.to_owned())
    } else {
        Err(format!("unknown claim surface: {value}"))
    }
}

fn parse_bool(value: &str) -> Result<bool, String> {
    match value {
        BOOL_TRUE => Ok(true),
        BOOL_FALSE => Ok(false),
        _ => Err(format!("boolean must be true or false: {value}")),
    }
}

fn build_render_input(config: &Config) -> Result<RenderInput, String> {
    let actual_head = if needs_actual_head(config) {
        Some(collect_git_head()?)
    } else {
        None
    };
    let head = HeadInput {
        vcs: config
            .vcs
            .clone()
            .or_else(|| actual_head.as_ref().map(|head| head.vcs.clone()))
            .unwrap_or_else(|| VCS_GIT.to_owned()),
        commit: config
            .commit
            .clone()
            .or_else(|| actual_head.as_ref().map(|head| head.commit.clone()))
            .ok_or_else(|| "commit is required outside a Git worktree".to_owned())?,
        branch: config
            .branch
            .clone()
            .or_else(|| actual_head.as_ref().map(|head| head.branch.clone()))
            .ok_or_else(|| "branch is required outside a Git worktree".to_owned())?,
        dirty: config
            .dirty
            .or_else(|| actual_head.as_ref().map(|head| head.dirty))
            .ok_or_else(|| "dirty status is required outside a Git worktree".to_owned())?,
    };

    let mut non_claims = config.non_claims.clone();
    if config.default_non_claims {
        for surface in OPTIONAL_SURFACES {
            if !config.claims.contains(*surface) {
                non_claims.insert((*surface).to_owned());
            }
        }
    }

    let real_check_receipt = config
        .real_check_receipt
        .as_ref()
        .map(|path| read_real_check_receipt(path))
        .transpose()?;

    Ok(RenderInput {
        head,
        checks: config.checks.clone(),
        missing_modes: config.missing_modes.clone(),
        claims: config.claims.clone(),
        non_claims,
        real_check_receipt,
    })
}

fn needs_actual_head(config: &Config) -> bool {
    config.commit.is_none() || config.branch.is_none() || config.dirty.is_none()
}

fn collect_git_head() -> Result<HeadInput, String> {
    let commit = git_output(&["rev-parse", "--verify", "HEAD"])?;
    let branch = git_output(&["symbolic-ref", "--quiet", "--short", "HEAD"])
        .unwrap_or_else(|_| DETACHED_BRANCH.to_owned());
    let status = git_output(&["status", "--porcelain"])?;
    Ok(HeadInput {
        vcs: VCS_GIT.to_owned(),
        commit,
        branch,
        dirty: !status.trim().is_empty(),
    })
}

// r[impl agent_harness_runtime.real_check_evidence_index.generator]
fn read_real_check_receipt(path: &PathBuf) -> Result<RealCheckReceiptInput, String> {
    let text = fs::read_to_string(path)
        .map_err(|error| format!("failed to read real-check receipt: {error}"))?;
    parse_real_check_receipt(&text)
}

fn parse_real_check_receipt(text: &str) -> Result<RealCheckReceiptInput, String> {
    let mut fields = BTreeMap::new();
    for line in text.lines() {
        let (key, value) = line
            .split_once(": ")
            .ok_or_else(|| format!("malformed real-check receipt line: {line}"))?;
        if !contains(REAL_CHECK_RECEIPT_KEYS, key) {
            return Err(format!("unknown real-check receipt field: {key}"));
        }
        if fields.insert(key.to_owned(), value.to_owned()).is_some() {
            return Err(format!("duplicate real-check receipt field: {key}"));
        }
    }
    for key in REAL_CHECK_REQUIRED_KEYS {
        if !fields.contains_key(*key) {
            return Err(format!("missing real-check receipt field: {key}"));
        }
    }

    let value = |key: &str| -> Result<String, String> {
        fields
            .get(key)
            .cloned()
            .ok_or_else(|| format!("missing real-check receipt field: {key}"))
    };
    let input = RealCheckReceiptInput {
        schema: value("schema")?,
        rail: value("rail")?,
        mode: value("mode")?,
        status: value("status")?,
        head: value("head")?,
        head_dirty: parse_bool(&value("head_dirty")?)?,
        source_blake3: value("source_blake3")?,
        selected_check_ids: value("selected_check_ids")?,
        cairn_binding: value("cairn_binding")?,
    };
    for field in [
        input.schema.as_str(),
        input.rail.as_str(),
        input.mode.as_str(),
        input.status.as_str(),
        input.head.as_str(),
        input.source_blake3.as_str(),
        input.selected_check_ids.as_str(),
        input.cairn_binding.as_str(),
    ] {
        validate_label(field, "real-check receipt field")?;
    }
    Ok(input)
}

fn git_output(args: &[&str]) -> Result<String, String> {
    let output = Command::new(VCS_GIT)
        .args(args)
        .output()
        .map_err(|error| format!("failed to invoke git: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "git {} failed with status {}",
            args.join(" "),
            output.status
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().to_owned())
}

fn render_index(input: &RenderInput) -> String {
    let mut lines = Vec::new();
    lines.push(format!(
        "head{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}",
        input.head.vcs,
        input.head.commit,
        input.head.branch,
        render_bool(input.head.dirty)
    ));
    for check in &input.checks {
        lines.push(format!(
            "check{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}",
            check.mode,
            mode_class(check.mode.as_str()),
            check.status,
            render_bool(check.head_match)
        ));
    }
    for missing_mode in &input.missing_modes {
        lines.push(format!(
            "missing_mode{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}",
            missing_mode.mode, missing_mode.reason
        ));
    }
    if let Some(receipt) = input.real_check_receipt.as_ref() {
        lines.push(format!(
            "{REAL_CHECK_KIND}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}{FIELD_SEPARATOR}{}",
            receipt.schema,
            receipt.rail,
            receipt.mode,
            receipt.status,
            receipt.head,
            render_bool(receipt.head_dirty),
            receipt.source_blake3,
            receipt.selected_check_ids,
            receipt.cairn_binding
        ));
    }
    for claim in &input.claims {
        lines.push(format!("claim{FIELD_SEPARATOR}{claim}"));
    }
    for non_claim in &input.non_claims {
        lines.push(format!("non_claim{FIELD_SEPARATOR}{non_claim}"));
    }
    lines.push(String::new());
    lines.join("\n")
}

fn render_bool(value: bool) -> &'static str {
    if value {
        BOOL_TRUE
    } else {
        BOOL_FALSE
    }
}

fn mode_class(mode: &str) -> &'static str {
    if is_pure_mode(mode) {
        CLASS_PURE
    } else {
        CLASS_OPTIONAL
    }
}

fn validate_mode(mode: &str) -> Result<(), String> {
    if is_pure_mode(mode) || contains(OPTIONAL_SURFACES, mode) {
        Ok(())
    } else {
        Err(format!("unknown evidence mode: {mode}"))
    }
}

fn validate_label(value: &str, field: &str) -> Result<(), String> {
    if value.trim().is_empty() || value.contains(FIELD_SEPARATOR) {
        Err(format!("invalid {field}: {value}"))
    } else {
        Ok(())
    }
}

fn is_pure_mode(mode: &str) -> bool {
    contains(REQUIRED_PURE_MODES, mode) || contains(ADDITIONAL_PURE_MODES, mode)
}

fn contains(values: &[&str], needle: &str) -> bool {
    values.iter().any(|value| *value == needle)
}

fn write_output(out: Option<&PathBuf>, text: &str) -> Result<(), String> {
    match out {
        Some(path) => {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)
                    .map_err(|error| format!("failed to create output directory: {error}"))?;
            }
            fs::write(path, text)
                .map_err(|error| format!("failed to write release evidence index: {error}"))
        }
        None => {
            print!("{text}");
            Ok(())
        }
    }
}
