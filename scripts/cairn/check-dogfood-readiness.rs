use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ARG_ROOT: &str = "--root";
const ARG_FIXTURE: &str = "--fixture";
const DOC_PATH: &str = "docs/dogfoodable-host-readiness-roadmap.md";
const EXPECTED_OK: &str = "OK";
const EXPECTED_PREFIX: &str = "expected_issue=";
const FIELD_SEPARATOR: char = '|';
const RECORD_KIND_INDEX: usize = 0;
const RECORD_VALUE_INDEX: usize = 1;
const RECORD_FIELD_COUNT: usize = 2;
const EXIT_FAILURE: i32 = 1;
const EXIT_SUCCESS: i32 = 0;

const KIND_EVIDENCE: &str = "evidence";
const KIND_CLAIM: &str = "claim";
const KIND_NON_CLAIM: &str = "non_claim";

const ISSUE_DOC_MISSING: &str = "DOGFOOD_DOC_MISSING";
const ISSUE_DOC_INCOMPLETE: &str = "DOGFOOD_DOC_INCOMPLETE";
const ISSUE_MALFORMED_RECORD: &str = "DOGFOOD_CLAIM_MALFORMED_RECORD";
const ISSUE_UNKNOWN_CLAIM: &str = "DOGFOOD_CLAIM_UNKNOWN_SURFACE";
const ISSUE_UNKNOWN_EVIDENCE: &str = "DOGFOOD_CLAIM_UNKNOWN_EVIDENCE";
const ISSUE_MISSING_EVIDENCE: &str = "DOGFOOD_CLAIM_MISSING_EVIDENCE";
const ISSUE_MISSING_NON_CLAIM: &str = "DOGFOOD_CLAIM_MISSING_NON_CLAIM";
const ISSUE_CONFLICTING_CLAIM: &str = "DOGFOOD_CLAIM_CONFLICTING_STATUS";

const CLAIM_DETERMINISTIC_FIXTURES: &str = "deterministic-fixture-checks";
const CLAIM_FAKE_BACKED_DOGFOOD: &str = "fake-backed-dogfood";
const CLAIM_PACKAGED_HEADLESS_DOGFOOD: &str = "packaged-headless-dogfood";
const CLAIM_LIVE_PROVIDERS: &str = "live-providers";
const CLAIM_TUI_DOGFOOD: &str = "tui-dogfood";
const CLAIM_DAEMON_ATTACH: &str = "daemon-attach";
const CLAIM_PLUGIN_RUNTIME: &str = "plugin-runtime";
const CLAIM_PERSISTENT_SESSION: &str = "persistent-session";
const CLAIM_PRODUCTION_READINESS: &str = "production-readiness";
const CLAIM_CLANKERS_PARITY: &str = "clankers-parity";

const EVIDENCE_DETERMINISTIC_FIXTURES: &str = "deterministic-fixtures";
const EVIDENCE_FAKE_BACKED_INTEGRATION: &str = "fake-backed-integration";
const EVIDENCE_PACKAGED_HEADLESS_SMOKE: &str = "packaged-headless-smoke";
const EVIDENCE_LIVE_PROVIDER_SMOKE: &str = "live-provider-smoke";
const EVIDENCE_TUI_DOGFOOD_SMOKE: &str = "tui-dogfood-smoke";
const EVIDENCE_DAEMON_ATTACH_SMOKE: &str = "daemon-attach-smoke";
const EVIDENCE_PLUGIN_RUNTIME_SMOKE: &str = "plugin-runtime-smoke";
const EVIDENCE_PERSISTENT_SESSION_SMOKE: &str = "persistent-session-smoke";
const EVIDENCE_PRODUCTION_READINESS: &str = "production-readiness-evidence";
const EVIDENCE_CLANKERS_PARITY: &str = "clankers-parity-evidence";

const REQUIRED_DOC_PHRASES: &[&str] = &[
    "actionable blocker",
    "intentional non-claim",
    "rejected default product surface",
    "optional future host adapter",
    "Runnable CLI/headless entrypoint",
    "Provider execution",
    "Tool execution",
    "Session persistence",
    "Readiness harness",
    "non-default headless/CLI host shell",
    "deterministic fake adapters",
    "Minimum dogfood prompt path",
    "Operational readiness matrix",
    "Successor Cairn ordering",
    "deterministic fixture checks",
    "fake-backed integration",
    "packaged headless smoke",
    "optional live-provider smoke",
    "optional TUI dogfood",
    "current-head release evidence index",
    "Claim-hygiene rule",
];

const CLAIM_RULES: &[ClaimRule] = &[
    ClaimRule {
        claim: CLAIM_DETERMINISTIC_FIXTURES,
        evidence: EVIDENCE_DETERMINISTIC_FIXTURES,
    },
    ClaimRule {
        claim: CLAIM_FAKE_BACKED_DOGFOOD,
        evidence: EVIDENCE_FAKE_BACKED_INTEGRATION,
    },
    ClaimRule {
        claim: CLAIM_PACKAGED_HEADLESS_DOGFOOD,
        evidence: EVIDENCE_PACKAGED_HEADLESS_SMOKE,
    },
    ClaimRule {
        claim: CLAIM_LIVE_PROVIDERS,
        evidence: EVIDENCE_LIVE_PROVIDER_SMOKE,
    },
    ClaimRule {
        claim: CLAIM_TUI_DOGFOOD,
        evidence: EVIDENCE_TUI_DOGFOOD_SMOKE,
    },
    ClaimRule {
        claim: CLAIM_DAEMON_ATTACH,
        evidence: EVIDENCE_DAEMON_ATTACH_SMOKE,
    },
    ClaimRule {
        claim: CLAIM_PLUGIN_RUNTIME,
        evidence: EVIDENCE_PLUGIN_RUNTIME_SMOKE,
    },
    ClaimRule {
        claim: CLAIM_PERSISTENT_SESSION,
        evidence: EVIDENCE_PERSISTENT_SESSION_SMOKE,
    },
    ClaimRule {
        claim: CLAIM_PRODUCTION_READINESS,
        evidence: EVIDENCE_PRODUCTION_READINESS,
    },
    ClaimRule {
        claim: CLAIM_CLANKERS_PARITY,
        evidence: EVIDENCE_CLANKERS_PARITY,
    },
];

const EVIDENCE_CLASSES: &[&str] = &[
    EVIDENCE_DETERMINISTIC_FIXTURES,
    EVIDENCE_FAKE_BACKED_INTEGRATION,
    EVIDENCE_PACKAGED_HEADLESS_SMOKE,
    EVIDENCE_LIVE_PROVIDER_SMOKE,
    EVIDENCE_TUI_DOGFOOD_SMOKE,
    EVIDENCE_DAEMON_ATTACH_SMOKE,
    EVIDENCE_PLUGIN_RUNTIME_SMOKE,
    EVIDENCE_PERSISTENT_SESSION_SMOKE,
    EVIDENCE_PRODUCTION_READINESS,
    EVIDENCE_CLANKERS_PARITY,
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Config {
    root: PathBuf,
    fixture: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ClaimRule {
    claim: &'static str,
    evidence: &'static str,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ClaimIndex {
    evidence: BTreeSet<String>,
    claims: BTreeSet<String>,
    non_claims: BTreeSet<String>,
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
    match &config.fixture {
        Some(fixture) => run_fixture(&config.root, fixture),
        None => run_repo(&config.root),
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");
    let mut fixture = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = PathBuf::from(value);
            }
            ARG_FIXTURE => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_FIXTURE} requires a path"))?;
                fixture = Some(PathBuf::from(value));
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    Ok(Config { root, fixture })
}

fn run_repo(root: &Path) -> Result<(), String> {
    let mut issues = validate_doc(root);
    issues.sort();
    print_issues(&issues);
    if issues.is_empty() {
        println!("dogfood readiness rail: ok");
        Ok(())
    } else {
        Err(format!(
            "dogfood readiness rail found {} issue(s)",
            issues.len()
        ))
    }
}

fn run_fixture(root: &Path, fixture: &Path) -> Result<(), String> {
    let text = fs::read_to_string(fixture)
        .map_err(|error| format!("failed to read fixture {}: {error}", fixture.display()))?;
    let expected_issue = parse_expected_issue(&text)?;
    let (claim_index, mut issues) = parse_claim_index(&text);
    issues.extend(validate_claim_index(&claim_index));
    issues.sort();

    let fixture_label = relative_path(root, fixture);
    if expected_issue == EXPECTED_OK {
        if issues.is_empty() {
            println!("positive fixture ok: {fixture_label}");
            return Ok(());
        }
        print_issues(&issues);
        return Err(format!("fixture {fixture_label} expected to pass"));
    }

    if issues
        .iter()
        .any(|issue| issue.code == expected_issue.as_str())
    {
        println!("negative fixture ok: {fixture_label} -> {expected_issue}");
        Ok(())
    } else {
        print_issues(&issues);
        Err(format!(
            "fixture {fixture_label} did not report expected issue {expected_issue}"
        ))
    }
}

fn validate_doc(root: &Path) -> Vec<Issue> {
    let path = root.join(DOC_PATH);
    let Ok(text) = fs::read_to_string(path) else {
        return vec![issue(
            ISSUE_DOC_MISSING,
            DOC_PATH,
            "dogfood readiness roadmap document is missing",
        )];
    };

    let mut issues = Vec::new();
    for phrase in REQUIRED_DOC_PHRASES {
        if !contains_case_insensitive(&text, phrase) {
            issues.push(issue(
                ISSUE_DOC_INCOMPLETE,
                DOC_PATH,
                &format!("missing required roadmap phrase: {phrase}"),
            ));
        }
    }
    issues
}

fn parse_expected_issue(text: &str) -> Result<String, String> {
    for line in text.lines() {
        let trimmed = line.trim();
        if let Some(value) = trimmed.strip_prefix(EXPECTED_PREFIX) {
            return Ok(value.trim().to_string());
        }
    }
    Err(format!("fixture metadata requires {EXPECTED_PREFIX}"))
}

fn parse_claim_index(text: &str) -> (ClaimIndex, Vec<Issue>) {
    let mut index = ClaimIndex::default();
    let mut issues = Vec::new();

    for raw_line in text.lines() {
        let line = raw_line.trim();
        if should_skip_line(line) {
            continue;
        }
        let fields: Vec<&str> = line.split(FIELD_SEPARATOR).collect();
        if fields.len() != RECORD_FIELD_COUNT {
            issues.push(issue(
                ISSUE_MALFORMED_RECORD,
                line,
                "record must have kind and value fields",
            ));
            continue;
        }
        let value = fields[RECORD_VALUE_INDEX].trim();
        match fields[RECORD_KIND_INDEX].trim() {
            KIND_EVIDENCE => parse_evidence(value, &mut index, &mut issues),
            KIND_CLAIM => parse_surface(value, &mut index.claims, &mut issues),
            KIND_NON_CLAIM => parse_surface(value, &mut index.non_claims, &mut issues),
            _unknown => issues.push(issue(
                ISSUE_MALFORMED_RECORD,
                line,
                "record kind must be evidence, claim, or non_claim",
            )),
        }
    }

    (index, issues)
}

fn should_skip_line(line: &str) -> bool {
    line.is_empty() || line.starts_with('#') || line.starts_with(EXPECTED_PREFIX)
}

fn parse_evidence(value: &str, index: &mut ClaimIndex, issues: &mut Vec<Issue>) {
    if !contains(EVIDENCE_CLASSES, value) {
        issues.push(issue(
            ISSUE_UNKNOWN_EVIDENCE,
            value,
            "evidence class is not part of the dogfood readiness schema",
        ));
    }
    index.evidence.insert(value.to_string());
}

fn parse_surface(value: &str, surfaces: &mut BTreeSet<String>, issues: &mut Vec<Issue>) {
    if claim_rule(value).is_none() {
        issues.push(issue(
            ISSUE_UNKNOWN_CLAIM,
            value,
            "claim surface is not part of the dogfood readiness schema",
        ));
    }
    surfaces.insert(value.to_string());
}

fn validate_claim_index(index: &ClaimIndex) -> Vec<Issue> {
    let mut issues = Vec::new();
    for claim in &index.claims {
        if index.non_claims.contains(claim) {
            issues.push(issue(
                ISSUE_CONFLICTING_CLAIM,
                claim,
                "surface is both claimed and listed as a non-claim",
            ));
        }
        if let Some(rule) = claim_rule(claim) {
            if !index.evidence.contains(rule.evidence) {
                issues.push(issue(
                    ISSUE_MISSING_EVIDENCE,
                    claim,
                    &format!("claim requires indexed evidence: {}", rule.evidence),
                ));
            }
        }
    }

    for rule in CLAIM_RULES {
        let has_claim = index.claims.contains(rule.claim);
        let has_non_claim = index.non_claims.contains(rule.claim);
        let has_evidence = index.evidence.contains(rule.evidence);
        if !has_claim && !has_non_claim && !has_evidence {
            issues.push(issue(
                ISSUE_MISSING_NON_CLAIM,
                rule.claim,
                "surface must be claimed with evidence or listed as a non-claim",
            ));
        }
    }
    issues
}

fn claim_rule(claim: &str) -> Option<&'static ClaimRule> {
    CLAIM_RULES.iter().find(|rule| rule.claim == claim)
}

fn contains(values: &[&str], needle: &str) -> bool {
    values.iter().any(|value| *value == needle)
}

fn contains_case_insensitive(text: &str, phrase: &str) -> bool {
    text.to_ascii_lowercase()
        .contains(&phrase.to_ascii_lowercase())
}

fn relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .into_owned()
}

fn issue(code: &'static str, subject: &str, detail: &str) -> Issue {
    Issue {
        code,
        subject: subject.to_string(),
        detail: detail.to_string(),
    }
}

fn print_issues(issues: &[Issue]) {
    for issue in issues {
        println!("ERROR {} {} -- {}", issue.code, issue.subject, issue.detail);
    }
}
