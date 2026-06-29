use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::PathBuf;

const ARG_INDEX: &str = "--index";
const FIELD_SEPARATOR: char = '|';
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const KIND_INDEX: usize = 0;
const HEAD_FIELD_COUNT: usize = 5;
const HEAD_VCS_INDEX: usize = 1;
const HEAD_COMMIT_INDEX: usize = 2;
const HEAD_BRANCH_INDEX: usize = 3;
const HEAD_DIRTY_INDEX: usize = 4;
const CHECK_FIELD_COUNT: usize = 5;
const CHECK_MODE_INDEX: usize = 1;
const CHECK_CLASS_INDEX: usize = 2;
const CHECK_STATUS_INDEX: usize = 3;
const CHECK_HEAD_MATCH_INDEX: usize = 4;
const SURFACE_FIELD_COUNT: usize = 2;
const SURFACE_VALUE_INDEX: usize = 1;
const MISSING_MODE_FIELD_COUNT: usize = 3;
const MISSING_MODE_VALUE_INDEX: usize = 1;
const MISSING_MODE_REASON_INDEX: usize = 2;

const KIND_HEAD: &str = "head";
const KIND_CHECK: &str = "check";
const KIND_CLAIM: &str = "claim";
const KIND_NON_CLAIM: &str = "non_claim";
const KIND_MISSING_MODE: &str = "missing_mode";

const CLASS_PURE: &str = "pure";
const CLASS_OPTIONAL: &str = "optional";
const STATUS_PASS: &str = "pass";
const STATUS_FAIL: &str = "fail";
const STATUS_SKIPPED: &str = "skipped";
const STATUS_MISSING: &str = "missing";

const CLAIM_CURRENT_HEAD_PURE_EVIDENCE: &str = "current-head-pure-evidence";
const SURFACE_LIVE_PROVIDERS: &str = "live-providers";
const SURFACE_TUI_DAEMON_DOGFOOD: &str = "tui-daemon-dogfood";
const SURFACE_VM_CHECKS: &str = "vm-checks";
const SURFACE_REMOTE_SERVICES: &str = "remote-services";
const SURFACE_PLUGIN_RUNTIME: &str = "plugin-runtime";
const SURFACE_PRODUCTION_READINESS: &str = "production-readiness";

const MODE_FOCUSED_RAILS: &str = "focused-rails";
const MODE_WORKSPACE_TESTS: &str = "workspace-tests";
const MODE_CLIPPY: &str = "clippy";
const MODE_FLAKE_CHECK: &str = "flake-check";
const MODE_NICKEL_DRIFT: &str = "nickel-drift";
const MODE_CAIRN_VALIDATE: &str = "cairn-validate";
const MODE_CAIRN_GATES: &str = "cairn-gates";

const REQUIRED_PURE_MODES: &[&str] = &[
    MODE_FOCUSED_RAILS,
    MODE_WORKSPACE_TESTS,
    MODE_CLIPPY,
    MODE_FLAKE_CHECK,
    MODE_NICKEL_DRIFT,
    MODE_CAIRN_VALIDATE,
    MODE_CAIRN_GATES,
];

const OPTIONAL_SURFACES: &[&str] = &[
    SURFACE_LIVE_PROVIDERS,
    SURFACE_TUI_DAEMON_DOGFOOD,
    SURFACE_VM_CHECKS,
    SURFACE_REMOTE_SERVICES,
    SURFACE_PLUGIN_RUNTIME,
    SURFACE_PRODUCTION_READINESS,
];

const KNOWN_CLASSES: &[&str] = &[CLASS_PURE, CLASS_OPTIONAL];
const KNOWN_STATUSES: &[&str] = &[STATUS_PASS, STATUS_FAIL, STATUS_SKIPPED, STATUS_MISSING];
const KNOWN_CLAIMS: &[&str] = &[
    CLAIM_CURRENT_HEAD_PURE_EVIDENCE,
    SURFACE_LIVE_PROVIDERS,
    SURFACE_TUI_DAEMON_DOGFOOD,
    SURFACE_VM_CHECKS,
    SURFACE_REMOTE_SERVICES,
    SURFACE_PLUGIN_RUNTIME,
    SURFACE_PRODUCTION_READINESS,
];

const ISSUE_MALFORMED_RECORD: &str = "RELEASE_EVIDENCE_MALFORMED_RECORD";
const ISSUE_DUPLICATE_HEAD: &str = "RELEASE_EVIDENCE_DUPLICATE_HEAD";
const ISSUE_MISSING_HEAD: &str = "RELEASE_EVIDENCE_MISSING_HEAD";
const ISSUE_INVALID_BOOL: &str = "RELEASE_EVIDENCE_INVALID_BOOL";
const ISSUE_UNKNOWN_MODE: &str = "RELEASE_EVIDENCE_UNKNOWN_MODE";
const ISSUE_UNKNOWN_CLASS: &str = "RELEASE_EVIDENCE_UNKNOWN_CLASS";
const ISSUE_UNKNOWN_STATUS: &str = "RELEASE_EVIDENCE_UNKNOWN_STATUS";
const ISSUE_UNKNOWN_SURFACE: &str = "RELEASE_EVIDENCE_UNKNOWN_SURFACE";
const ISSUE_WRONG_PURE_CLASS: &str = "RELEASE_EVIDENCE_WRONG_PURE_CLASS";
const ISSUE_MISSING_NON_CLAIM: &str = "RELEASE_EVIDENCE_MISSING_NON_CLAIM";
const ISSUE_DIRTY_HEAD: &str = "RELEASE_EVIDENCE_DIRTY_HEAD";
const ISSUE_STALE_RECEIPT: &str = "RELEASE_EVIDENCE_STALE_RECEIPT";
const ISSUE_MISSING_MODE: &str = "RELEASE_EVIDENCE_MISSING_MODE";
const ISSUE_OVERCLAIM: &str = "RELEASE_EVIDENCE_OVERCLAIM";

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    index: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct HeadRecord {
    vcs: String,
    commit: String,
    branch: String,
    dirty: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CheckRecord {
    mode: String,
    class: String,
    status: String,
    head_match: bool,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct EvidenceIndex {
    head: Option<HeadRecord>,
    checks: Vec<CheckRecord>,
    claims: BTreeSet<String>,
    non_claims: BTreeSet<String>,
    missing_modes: BTreeMap<String, String>,
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
    let text = fs::read_to_string(&config.index)
        .map_err(|error| format!("failed to read release evidence index lines: {error}"))?;
    let (index, mut issues) = parse_index(&text);
    issues.extend(validate_index(&index));
    issues.sort();
    print_report(&index, &issues);
    if issues.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "release evidence index failed with {} issue(s)",
            issues.len()
        ))
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut index = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_INDEX => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_INDEX} requires a path"))?;
                index = Some(PathBuf::from(value));
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let index = index.ok_or_else(|| format!("{ARG_INDEX} is required"))?;
    Ok(Config { index })
}

fn parse_index(text: &str) -> (EvidenceIndex, Vec<Issue>) {
    let mut index = EvidenceIndex::default();
    let mut issues = Vec::new();

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let fields: Vec<&str> = line.split(FIELD_SEPARATOR).collect();
        match fields[KIND_INDEX] {
            KIND_HEAD => parse_head(&fields, line, &mut index, &mut issues),
            KIND_CHECK => parse_check(&fields, line, &mut index, &mut issues),
            KIND_CLAIM => parse_surface_record(&fields, line, &mut index.claims, &mut issues),
            KIND_NON_CLAIM => {
                parse_surface_record(&fields, line, &mut index.non_claims, &mut issues)
            }
            KIND_MISSING_MODE => parse_missing_mode(&fields, line, &mut index, &mut issues),
            _ => issues.push(issue(
                ISSUE_MALFORMED_RECORD,
                line,
                "record kind must be head, check, claim, non_claim, or missing_mode",
            )),
        }
    }

    (index, issues)
}

fn parse_head(fields: &[&str], line: &str, index: &mut EvidenceIndex, issues: &mut Vec<Issue>) {
    if fields.len() != HEAD_FIELD_COUNT {
        issues.push(issue(
            ISSUE_MALFORMED_RECORD,
            line,
            "head record must have vcs, commit, branch, and dirty fields",
        ));
        return;
    }
    if index.head.is_some() {
        issues.push(issue(
            ISSUE_DUPLICATE_HEAD,
            line,
            "index must contain exactly one head record",
        ));
        return;
    }
    let Some(dirty) = parse_bool(fields[HEAD_DIRTY_INDEX], line, "dirty", issues) else {
        return;
    };
    index.head = Some(HeadRecord {
        vcs: fields[HEAD_VCS_INDEX].to_string(),
        commit: fields[HEAD_COMMIT_INDEX].to_string(),
        branch: fields[HEAD_BRANCH_INDEX].to_string(),
        dirty,
    });
}

fn parse_check(fields: &[&str], line: &str, index: &mut EvidenceIndex, issues: &mut Vec<Issue>) {
    if fields.len() != CHECK_FIELD_COUNT {
        issues.push(issue(
            ISSUE_MALFORMED_RECORD,
            line,
            "check record must have mode, class, status, and head_match fields",
        ));
        return;
    }

    let mode = fields[CHECK_MODE_INDEX];
    let class = fields[CHECK_CLASS_INDEX];
    let status = fields[CHECK_STATUS_INDEX];

    if !is_known_mode(mode) {
        issues.push(issue(
            ISSUE_UNKNOWN_MODE,
            mode,
            "check mode is not part of the release evidence schema",
        ));
    }
    if !contains(KNOWN_CLASSES, class) {
        issues.push(issue(
            ISSUE_UNKNOWN_CLASS,
            class,
            "check class must be pure or optional",
        ));
    }
    if !contains(KNOWN_STATUSES, status) {
        issues.push(issue(
            ISSUE_UNKNOWN_STATUS,
            status,
            "check status must be pass, fail, skipped, or missing",
        ));
    }

    let Some(head_match) = parse_bool(fields[CHECK_HEAD_MATCH_INDEX], line, "head_match", issues)
    else {
        return;
    };
    index.checks.push(CheckRecord {
        mode: mode.to_string(),
        class: class.to_string(),
        status: status.to_string(),
        head_match,
    });
}

fn parse_surface_record(
    fields: &[&str],
    line: &str,
    surfaces: &mut BTreeSet<String>,
    issues: &mut Vec<Issue>,
) {
    if fields.len() != SURFACE_FIELD_COUNT {
        issues.push(issue(
            ISSUE_MALFORMED_RECORD,
            line,
            "surface record must have exactly one surface field",
        ));
        return;
    }
    let surface = fields[SURFACE_VALUE_INDEX];
    if !contains(KNOWN_CLAIMS, surface) {
        issues.push(issue(
            ISSUE_UNKNOWN_SURFACE,
            surface,
            "claim or non-claim surface is not part of the release evidence schema",
        ));
    }
    surfaces.insert(surface.to_string());
}

fn parse_missing_mode(
    fields: &[&str],
    line: &str,
    index: &mut EvidenceIndex,
    issues: &mut Vec<Issue>,
) {
    if fields.len() != MISSING_MODE_FIELD_COUNT {
        issues.push(issue(
            ISSUE_MALFORMED_RECORD,
            line,
            "missing_mode record must have mode and reason fields",
        ));
        return;
    }
    let mode = fields[MISSING_MODE_VALUE_INDEX];
    if !is_known_mode(mode) {
        issues.push(issue(
            ISSUE_UNKNOWN_MODE,
            mode,
            "missing mode is not part of the release evidence schema",
        ));
    }
    index.missing_modes.insert(
        mode.to_string(),
        fields[MISSING_MODE_REASON_INDEX].to_string(),
    );
}

fn validate_index(index: &EvidenceIndex) -> Vec<Issue> {
    let mut issues = Vec::new();
    let checks_by_mode = checks_by_mode(&index.checks);
    let has_current_head_claim = makes_current_head_claim(&index.claims);

    match &index.head {
        Some(head) => {
            if has_current_head_claim && head.dirty {
                issues.push(issue(
                    ISSUE_DIRTY_HEAD,
                    &head.commit,
                    "dirty worktree evidence cannot be current-head proof",
                ));
            }
        }
        None => issues.push(issue(
            ISSUE_MISSING_HEAD,
            KIND_HEAD,
            "index must include a head record before making evidence claims",
        )),
    }

    if has_current_head_claim {
        validate_required_pure_modes(index, &checks_by_mode, &mut issues);
    }
    validate_optional_claims(index, &checks_by_mode, &mut issues);

    issues
}

fn validate_required_pure_modes(
    index: &EvidenceIndex,
    checks_by_mode: &BTreeMap<String, &CheckRecord>,
    issues: &mut Vec<Issue>,
) {
    for mode in REQUIRED_PURE_MODES {
        let declared_missing = index.missing_modes.contains_key(*mode);
        match checks_by_mode.get(*mode) {
            Some(check) => {
                if check.class != CLASS_PURE {
                    issues.push(issue(
                        ISSUE_WRONG_PURE_CLASS,
                        *mode,
                        "required current-head evidence mode must be classified as pure",
                    ));
                }
                if declared_missing || check.status != STATUS_PASS {
                    issues.push(issue(
                        ISSUE_MISSING_MODE,
                        *mode,
                        "required pure mode is absent, explicitly missing, or not passing",
                    ));
                }
                if !check.head_match {
                    issues.push(issue(
                        ISSUE_STALE_RECEIPT,
                        *mode,
                        "receipt does not match the recorded head",
                    ));
                }
            }
            None => issues.push(issue(
                ISSUE_MISSING_MODE,
                *mode,
                "required pure mode is absent from a current-head evidence claim",
            )),
        }
    }
}

fn validate_optional_claims(
    index: &EvidenceIndex,
    checks_by_mode: &BTreeMap<String, &CheckRecord>,
    issues: &mut Vec<Issue>,
) {
    for surface in OPTIONAL_SURFACES {
        let has_claim = index.claims.contains(*surface);
        let has_non_claim = index.non_claims.contains(*surface);
        let has_passing_check = checks_by_mode
            .get(*surface)
            .is_some_and(|check| check.class == CLASS_OPTIONAL && check.status == STATUS_PASS);

        if has_claim && !has_passing_check {
            issues.push(issue(
                ISSUE_OVERCLAIM,
                *surface,
                "optional host-dependent surface is claimed without passing optional evidence",
            ));
            continue;
        }
        if !has_claim && !has_non_claim && !has_passing_check {
            issues.push(issue(
                ISSUE_MISSING_NON_CLAIM,
                *surface,
                "omitted optional surface must be recorded as a non-claim",
            ));
        }
    }
}

fn checks_by_mode(checks: &[CheckRecord]) -> BTreeMap<String, &CheckRecord> {
    let mut by_mode = BTreeMap::new();
    for check in checks {
        by_mode.insert(check.mode.clone(), check);
    }
    by_mode
}

fn makes_current_head_claim(claims: &BTreeSet<String>) -> bool {
    claims.contains(CLAIM_CURRENT_HEAD_PURE_EVIDENCE)
        || claims.contains(SURFACE_PRODUCTION_READINESS)
}

fn parse_bool(
    value: &str,
    subject: &str,
    field_name: &str,
    issues: &mut Vec<Issue>,
) -> Option<bool> {
    match value {
        "true" => Some(true),
        "false" => Some(false),
        _ => {
            issues.push(issue(
                ISSUE_INVALID_BOOL,
                subject,
                &format!("{field_name} must be true or false"),
            ));
            None
        }
    }
}

fn is_known_mode(mode: &str) -> bool {
    contains(REQUIRED_PURE_MODES, mode) || contains(OPTIONAL_SURFACES, mode)
}

fn contains(values: &[&str], needle: &str) -> bool {
    values.iter().any(|value| *value == needle)
}

fn issue(code: &'static str, subject: &str, detail: &str) -> Issue {
    Issue {
        code,
        subject: subject.to_string(),
        detail: detail.to_string(),
    }
}

fn print_report(index: &EvidenceIndex, issues: &[Issue]) {
    match &index.head {
        Some(head) => println!(
            "head vcs={} commit={} branch={} dirty={}",
            head.vcs, head.commit, head.branch, head.dirty
        ),
        None => println!("head missing"),
    }
    println!(
        "release_evidence_index checks={} claims={} non_claims={} missing_modes={}",
        index.checks.len(),
        index.claims.len(),
        index.non_claims.len(),
        index.missing_modes.len()
    );
    for issue in issues {
        println!("ERROR {} {} -- {}", issue.code, issue.subject, issue.detail);
    }
}
