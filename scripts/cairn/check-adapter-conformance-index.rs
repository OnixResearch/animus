use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::PathBuf;

const ARG_ROOT: &str = "--root";
const ARG_FAMILIES: &str = "--families";
const ARG_CASES: &str = "--cases";
const ARG_DOCS: &str = "--docs";
const FAMILY_KIND: &str = "family";
const CASE_KIND: &str = "case";
const FIELD_SEPARATOR: char = '|';
const FAMILY_FIELD_COUNT: usize = 12;
const CASE_FIELD_COUNT: usize = 6;
const EXIT_SUCCESS: i32 = 0;
const EXIT_FAILURE: i32 = 1;

const STABILITY_STABLE: &str = "stable";
const STABILITY_EXPERIMENTAL: &str = "experimental";
const STABILITY_OPTIONAL: &str = "optional";
const DISPOSITION_COVERED: &str = "covered";
const DISPOSITION_NOT_APPLICABLE: &str = "not-applicable";
const CASE_HAPPY_PATH: &str = "happy_path";

const ISSUE_MALFORMED_RECORD: &str = "ADAPTER_CONFORMANCE_MALFORMED_RECORD";
const ISSUE_DUPLICATE_FAMILY: &str = "ADAPTER_CONFORMANCE_DUPLICATE_FAMILY";
const ISSUE_DUPLICATE_CASE: &str = "ADAPTER_CONFORMANCE_DUPLICATE_CASE";
const ISSUE_UNKNOWN_FAMILY: &str = "ADAPTER_CONFORMANCE_UNKNOWN_FAMILY";
const ISSUE_UNKNOWN_CASE: &str = "ADAPTER_CONFORMANCE_UNKNOWN_CASE";
const ISSUE_UNKNOWN_STABILITY: &str = "ADAPTER_CONFORMANCE_UNKNOWN_STABILITY";
const ISSUE_UNKNOWN_DISPOSITION: &str = "ADAPTER_CONFORMANCE_UNKNOWN_DISPOSITION";
const ISSUE_MISSING_REQUIRED_FIELD: &str = "ADAPTER_CONFORMANCE_MISSING_REQUIRED_FIELD";
const ISSUE_MISSING_CASE: &str = "ADAPTER_CONFORMANCE_MISSING_CASE";
const ISSUE_MISSING_POSITIVE_COVERAGE: &str = "ADAPTER_CONFORMANCE_MISSING_POSITIVE_COVERAGE";
const ISSUE_MISSING_NEGATIVE_COVERAGE: &str = "ADAPTER_CONFORMANCE_MISSING_NEGATIVE_COVERAGE";
const ISSUE_COVERAGE_WITHOUT_EVIDENCE: &str = "ADAPTER_CONFORMANCE_COVERAGE_WITHOUT_EVIDENCE";
const ISSUE_NA_WITHOUT_REASON: &str = "ADAPTER_CONFORMANCE_NA_WITHOUT_REASON";
const ISSUE_DOCS_MISSING_FAMILY: &str = "ADAPTER_CONFORMANCE_DOCS_MISSING_FAMILY";
const ISSUE_UNSAFE_FIELD: &str = "ADAPTER_CONFORMANCE_UNSAFE_FIELD";

const REQUIRED_CASES: &[&str] = &[
    CASE_HAPPY_PATH,
    "missing_service",
    "denied_capability",
    "unavailable_service",
    "timeout",
    "cancellation",
    "malformed_output",
    "oversized_output",
    "unsafe_reference",
    "schema_mismatch",
    "duplicate_field",
    "unknown_field",
    "leak_rejection",
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
    families_path: PathBuf,
    cases_path: PathBuf,
    docs_path: Option<PathBuf>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct FamilyRecord {
    id: String,
    owner_module: String,
    stability: String,
    authority_boundary: String,
    receipt_behavior: String,
    serialization_bridge: String,
    focused_surface: String,
    focused_validation_check: String,
    positive_test: String,
    negative_test: String,
    docs_anchor: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CaseRecord {
    family_id: String,
    case_id: String,
    disposition: String,
    evidence: String,
    reason: String,
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
    let families_text = fs::read_to_string(&config.families_path)
        .map_err(|error| format!("failed to read family lines: {error}"))?;
    let cases_text = fs::read_to_string(&config.cases_path)
        .map_err(|error| format!("failed to read case lines: {error}"))?;
    let docs_text = match &config.docs_path {
        Some(path) => Some(
            fs::read_to_string(path)
                .map_err(|error| format!("failed to read docs index: {error}"))?,
        ),
        None => None,
    };

    let (families, mut issues) = parse_families(&families_text);
    let (cases, case_issues) = parse_cases(&cases_text);
    issues.extend(case_issues);
    issues.extend(validate(&families, &cases, docs_text.as_deref()));
    issues.sort();
    print_report(&families, &cases, &issues);

    if issues.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "adapter conformance index failed with {} issue(s)",
            issues.len()
        ))
    }
}

fn parse_args(mut args: impl Iterator<Item = String>) -> Result<Config, String> {
    let _program = args.next();
    let mut root = PathBuf::from(".");
    let mut families_path = None;
    let mut cases_path = None;
    let mut docs_path = None;

    while let Some(argument) = args.next() {
        match argument.as_str() {
            ARG_ROOT => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_ROOT} requires a path"))?;
                root = PathBuf::from(value);
            }
            ARG_FAMILIES => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_FAMILIES} requires a path"))?;
                families_path = Some(PathBuf::from(value));
            }
            ARG_CASES => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_CASES} requires a path"))?;
                cases_path = Some(PathBuf::from(value));
            }
            ARG_DOCS => {
                let value = args
                    .next()
                    .ok_or_else(|| format!("{ARG_DOCS} requires a path"))?;
                docs_path = Some(PathBuf::from(value));
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    let families_path = families_path.ok_or_else(|| format!("{ARG_FAMILIES} is required"))?;
    let cases_path = cases_path.ok_or_else(|| format!("{ARG_CASES} is required"))?;
    Ok(Config {
        root,
        families_path,
        cases_path,
        docs_path,
    })
}

fn parse_families(text: &str) -> (Vec<FamilyRecord>, Vec<Issue>) {
    let mut families = Vec::new();
    let mut issues = Vec::new();
    for line in text.lines().map(str::trim) {
        if should_skip_line(line) {
            continue;
        }
        let fields: Vec<&str> = line.split(FIELD_SEPARATOR).collect();
        if fields.len() != FAMILY_FIELD_COUNT || fields.first() != Some(&FAMILY_KIND) {
            issues.push(issue(
                ISSUE_MALFORMED_RECORD,
                line,
                "family record must have the expected fields",
            ));
            continue;
        }
        families.push(FamilyRecord {
            id: fields[1].to_string(),
            owner_module: fields[2].to_string(),
            stability: fields[3].to_string(),
            authority_boundary: fields[4].to_string(),
            receipt_behavior: fields[5].to_string(),
            serialization_bridge: fields[6].to_string(),
            focused_surface: fields[7].to_string(),
            focused_validation_check: fields[8].to_string(),
            positive_test: fields[9].to_string(),
            negative_test: fields[10].to_string(),
            docs_anchor: fields[11].to_string(),
        });
    }
    (families, issues)
}

fn parse_cases(text: &str) -> (Vec<CaseRecord>, Vec<Issue>) {
    let mut cases = Vec::new();
    let mut issues = Vec::new();
    for line in text.lines().map(str::trim) {
        if should_skip_line(line) {
            continue;
        }
        let fields: Vec<&str> = line.split(FIELD_SEPARATOR).collect();
        if fields.len() != CASE_FIELD_COUNT || fields.first() != Some(&CASE_KIND) {
            issues.push(issue(
                ISSUE_MALFORMED_RECORD,
                line,
                "case record must have the expected fields",
            ));
            continue;
        }
        cases.push(CaseRecord {
            family_id: fields[1].to_string(),
            case_id: fields[2].to_string(),
            disposition: fields[3].to_string(),
            evidence: fields[4].to_string(),
            reason: fields[5].to_string(),
        });
    }
    (cases, issues)
}

fn should_skip_line(line: &str) -> bool {
    line.is_empty() || line.starts_with('#')
}

fn validate(
    families: &[FamilyRecord],
    cases: &[CaseRecord],
    docs_text: Option<&str>,
) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut family_ids = BTreeSet::new();
    let mut families_by_id = BTreeMap::new();

    for family in families {
        if !family_ids.insert(family.id.clone()) {
            issues.push(issue(
                ISSUE_DUPLICATE_FAMILY,
                &family.id,
                "family appears more than once",
            ));
        }
        families_by_id.insert(family.id.as_str(), family);
        validate_family(family, docs_text, &mut issues);
    }

    let mut cases_by_family: BTreeMap<&str, BTreeMap<&str, &CaseRecord>> = BTreeMap::new();
    let mut seen_cases = BTreeSet::new();
    for case in cases {
        validate_case(case, &families_by_id, &mut issues);
        let key = (case.family_id.as_str(), case.case_id.as_str());
        if !seen_cases.insert(key) {
            issues.push(issue(
                ISSUE_DUPLICATE_CASE,
                &format!("{}::{}", case.family_id, case.case_id),
                "family declares the same conformance case more than once",
            ));
        }
        cases_by_family
            .entry(case.family_id.as_str())
            .or_default()
            .insert(case.case_id.as_str(), case);
    }

    for family in families {
        validate_family_coverage(family, cases_by_family.get(family.id.as_str()), &mut issues);
    }

    issues
}

fn validate_family(family: &FamilyRecord, docs_text: Option<&str>, issues: &mut Vec<Issue>) {
    let required_fields = [
        ("id", family.id.as_str()),
        ("owner_module", family.owner_module.as_str()),
        ("stability", family.stability.as_str()),
        ("authority_boundary", family.authority_boundary.as_str()),
        ("receipt_behavior", family.receipt_behavior.as_str()),
        ("serialization_bridge", family.serialization_bridge.as_str()),
        ("focused_surface", family.focused_surface.as_str()),
        (
            "focused_validation_check",
            family.focused_validation_check.as_str(),
        ),
        ("positive_test", family.positive_test.as_str()),
        ("negative_test", family.negative_test.as_str()),
        ("docs_anchor", family.docs_anchor.as_str()),
    ];
    for (field_name, value) in required_fields {
        validate_required_field(&family.id, field_name, value, issues);
        validate_safe_field(&family.id, field_name, value, issues);
    }

    if !matches!(
        family.stability.as_str(),
        STABILITY_STABLE | STABILITY_EXPERIMENTAL | STABILITY_OPTIONAL
    ) {
        issues.push(issue(
            ISSUE_UNKNOWN_STABILITY,
            &family.id,
            &format!("unsupported stability: {}", family.stability),
        ));
    }

    if let Some(text) = docs_text {
        if !text.contains(family.docs_anchor.as_str()) {
            issues.push(issue(
                ISSUE_DOCS_MISSING_FAMILY,
                &family.id,
                "contracts index does not contain the family docs anchor",
            ));
        }
    }
}

fn validate_case(
    case: &CaseRecord,
    families_by_id: &BTreeMap<&str, &FamilyRecord>,
    issues: &mut Vec<Issue>,
) {
    let required_fields = [
        ("family_id", case.family_id.as_str()),
        ("case_id", case.case_id.as_str()),
        ("disposition", case.disposition.as_str()),
    ];
    for (field_name, value) in required_fields {
        validate_required_field(&case.family_id, field_name, value, issues);
        validate_safe_field(&case.family_id, field_name, value, issues);
    }
    validate_safe_field(&case.family_id, "evidence", case.evidence.as_str(), issues);
    validate_safe_field(&case.family_id, "reason", case.reason.as_str(), issues);

    if !families_by_id.contains_key(case.family_id.as_str()) {
        issues.push(issue(
            ISSUE_UNKNOWN_FAMILY,
            &case.family_id,
            "case references a family not declared in the index",
        ));
    }
    if !REQUIRED_CASES.contains(&case.case_id.as_str()) {
        issues.push(issue(
            ISSUE_UNKNOWN_CASE,
            &format!("{}::{}", case.family_id, case.case_id),
            "case is not part of the required adapter conformance matrix",
        ));
    }
    match case.disposition.as_str() {
        DISPOSITION_COVERED => {
            if case.evidence.trim().is_empty() {
                issues.push(issue(
                    ISSUE_COVERAGE_WITHOUT_EVIDENCE,
                    &format!("{}::{}", case.family_id, case.case_id),
                    "covered cases must name positive or negative coverage evidence",
                ));
            }
        }
        DISPOSITION_NOT_APPLICABLE => {
            if case.reason.trim().is_empty() {
                issues.push(issue(
                    ISSUE_NA_WITHOUT_REASON,
                    &format!("{}::{}", case.family_id, case.case_id),
                    "not-applicable cases require a safe reason",
                ));
            }
        }
        _ => issues.push(issue(
            ISSUE_UNKNOWN_DISPOSITION,
            &format!("{}::{}", case.family_id, case.case_id),
            &format!("unsupported disposition: {}", case.disposition),
        )),
    }
}

fn validate_family_coverage(
    family: &FamilyRecord,
    maybe_cases: Option<&BTreeMap<&str, &CaseRecord>>,
    issues: &mut Vec<Issue>,
) {
    let empty_cases = BTreeMap::new();
    let family_cases = maybe_cases.unwrap_or(&empty_cases);
    for required_case in REQUIRED_CASES {
        if !family_cases.contains_key(required_case) {
            issues.push(issue(
                ISSUE_MISSING_CASE,
                &format!("{}::{required_case}", family.id),
                "family omits a required conformance case",
            ));
        }
    }

    let happy_path_is_covered = family_cases
        .get(CASE_HAPPY_PATH)
        .is_some_and(|case| case.disposition == DISPOSITION_COVERED);
    if !happy_path_is_covered {
        issues.push(issue(
            ISSUE_MISSING_POSITIVE_COVERAGE,
            &family.id,
            "family must cover the happy-path conformance case",
        ));
    }

    let covered_negative_count = family_cases
        .iter()
        .filter(|(case_id, case)| **case_id != CASE_HAPPY_PATH && case.disposition == DISPOSITION_COVERED)
        .count();
    if covered_negative_count == 0 {
        issues.push(issue(
            ISSUE_MISSING_NEGATIVE_COVERAGE,
            &family.id,
            "family must cover at least one negative fail-closed case",
        ));
    }
}

fn validate_required_field(
    subject: &str,
    field_name: &str,
    value: &str,
    issues: &mut Vec<Issue>,
) {
    if value.trim().is_empty() {
        issues.push(issue(
            ISSUE_MISSING_REQUIRED_FIELD,
            subject,
            &format!("field is required: {field_name}"),
        ));
    }
}

fn validate_safe_field(subject: &str, field_name: &str, value: &str, issues: &mut Vec<Issue>) {
    let lower = value.to_ascii_lowercase();
    for marker in UNSAFE_FIELD_MARKERS {
        if lower.contains(&marker.to_ascii_lowercase()) {
            issues.push(issue(
                ISSUE_UNSAFE_FIELD,
                subject,
                &format!("field {field_name} contains unsafe evidence marker: {marker}"),
            ));
        }
    }
}

fn print_report(families: &[FamilyRecord], cases: &[CaseRecord], issues: &[Issue]) {
    println!("adapter conformance families: {}", families.len());
    println!("adapter conformance cases: {}", cases.len());
    print_counts("stability", families.iter().map(|family| family.stability.as_str()));
    print_counts(
        "case-disposition",
        cases.iter().map(|case| case.disposition.as_str()),
    );
    if issues.is_empty() {
        println!("adapter conformance index: ok");
        return;
    }
    for issue in issues {
        println!("ERROR {} {} -- {}", issue.code, issue.subject, issue.detail);
    }
}

fn print_counts<'a>(label: &str, values: impl Iterator<Item = &'a str>) {
    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();
    for value in values {
        *counts.entry(value).or_default() += 1;
    }
    for (value, count) in counts {
        println!("{label} {value}: {count}");
    }
}

fn issue(code: &'static str, subject: &str, detail: &str) -> Issue {
    Issue {
        code,
        subject: subject.to_string(),
        detail: detail.to_string(),
    }
}
