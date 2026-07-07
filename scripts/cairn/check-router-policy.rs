use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::path::PathBuf;

const EXIT_FAILURE: i32 = 1;
const OK_ISSUE: &str = "OK";
const POSITIVE_KIND: &str = "positive";
const NEGATIVE_KIND: &str = "negative";
const FAMILY_TAG: &str = "family";
const FIXTURE_TAG: &str = "fixture";
const POLICY_JSON_INCLUDE: &str = "../../../policy/model-router/generated/router-recipe-policy.json";
const POLICY_SCHEMA_SENTINEL: &str = "animus.router-recipe-policy.v1";

const ISSUE_UNKNOWN_FAMILY: &str = "ROUTER_POLICY_UNKNOWN_FAMILY";
const ISSUE_INVALID_ROLE_COMBO: &str = "ROUTER_POLICY_INVALID_ROLE_COMBO";
const ISSUE_OUTPUT_CONTRACT: &str = "ROUTER_POLICY_OUTPUT_CONTRACT";
const ISSUE_BOUND_OUT_OF_RANGE: &str = "ROUTER_POLICY_BOUND_OUT_OF_RANGE";
const ISSUE_UNKNOWN_DEADLINE: &str = "ROUTER_POLICY_UNKNOWN_DEADLINE";
const ISSUE_DISALLOWED_FALLBACK: &str = "ROUTER_POLICY_DISALLOWED_FALLBACK";
const ISSUE_UNKNOWN_BACKEND: &str = "ROUTER_POLICY_UNKNOWN_BACKEND";
const ISSUE_RUST_SYNC: &str = "ROUTER_POLICY_RUST_SYNC";
const ISSUE_POLICY_DRIFT: &str = "ROUTER_POLICY_DRIFT";
const ISSUE_PARSE: &str = "ROUTER_POLICY_PARSE";

const FAMILY_FIELD_COUNT: usize = 17;
const FAMILY_TAG_INDEX: usize = 0;
const FAMILY_ID_INDEX: usize = 1;
const FAMILY_ROLES_INDEX: usize = 2;
const FAMILY_OUTPUTS_INDEX: usize = 3;
const FAMILY_MIN_CANDIDATES_INDEX: usize = 4;
const FAMILY_MAX_CANDIDATES_INDEX: usize = 5;
const FAMILY_MIN_PARALLELISM_INDEX: usize = 6;
const FAMILY_MAX_PARALLELISM_INDEX: usize = 7;
const FAMILY_MIN_QUORUM_INDEX: usize = 8;
const FAMILY_MAX_QUORUM_INDEX: usize = 9;
const FAMILY_MIN_SYNTHESIS_INDEX: usize = 10;
const FAMILY_MAX_SYNTHESIS_INDEX: usize = 11;
const FAMILY_MIN_REPAIR_INDEX: usize = 12;
const FAMILY_MAX_REPAIR_INDEX: usize = 13;
const FAMILY_FALLBACKS_INDEX: usize = 14;
const FAMILY_BACKENDS_INDEX: usize = 15;
const FAMILY_DEADLINES_INDEX: usize = 16;

const FIXTURE_FIELD_COUNT: usize = 15;
const FIXTURE_TAG_INDEX: usize = 0;
const FIXTURE_KIND_INDEX: usize = 1;
const FIXTURE_ID_INDEX: usize = 2;
const FIXTURE_EXPECTED_INDEX: usize = 3;
const FIXTURE_FAMILY_INDEX: usize = 4;
const FIXTURE_ROLES_INDEX: usize = 5;
const FIXTURE_OUTPUT_INDEX: usize = 6;
const FIXTURE_MAX_CANDIDATES_INDEX: usize = 7;
const FIXTURE_MAX_PARALLELISM_INDEX: usize = 8;
const FIXTURE_QUORUM_INDEX: usize = 9;
const FIXTURE_DEADLINE_INDEX: usize = 10;
const FIXTURE_SYNTHESIS_INDEX: usize = 11;
const FIXTURE_REPAIR_INDEX: usize = 12;
const FIXTURE_FALLBACK_INDEX: usize = 13;
const FIXTURE_BACKEND_INDEX: usize = 14;

#[derive(Debug)]
struct Args {
    policy_lines: PathBuf,
    fixtures: PathBuf,
    generated_policy: PathBuf,
    source_policy: PathBuf,
    rust_policy: PathBuf,
}

#[derive(Clone, Debug)]
struct BoundsPolicy {
    min_candidates: u32,
    max_candidates: u32,
    min_parallelism: u32,
    max_parallelism: u32,
    min_quorum: u32,
    max_quorum: u32,
    min_synthesis: u32,
    max_synthesis: u32,
    min_repair: u32,
    max_repair: u32,
}

#[derive(Clone, Debug)]
struct FamilyPolicy {
    id: String,
    roles: BTreeSet<String>,
    outputs: BTreeSet<String>,
    bounds: BoundsPolicy,
    fallbacks: BTreeSet<String>,
    backends: BTreeSet<String>,
    deadlines: BTreeSet<String>,
}

#[derive(Clone, Debug)]
struct Fixture {
    kind: String,
    id: String,
    expected_issue: String,
    family: String,
    roles: BTreeSet<String>,
    output_contract: String,
    max_candidates: u32,
    max_parallelism: u32,
    min_success_quorum: u32,
    timeout_class: String,
    max_synthesis_attempts: u32,
    max_repair_attempts: u32,
    fallback_policy: String,
    backend: String,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(EXIT_FAILURE);
    }
}

fn run() -> Result<(), String> {
    let args = parse_args(env::args().skip(1).collect())?;
    let policy_lines = fs::read_to_string(&args.policy_lines)
        .map_err(|error| format!("failed to read policy lines: {error}"))?;
    let fixture_lines = fs::read_to_string(&args.fixtures)
        .map_err(|error| format!("failed to read fixture lines: {error}"))?;
    let generated_policy = fs::read_to_string(&args.generated_policy)
        .map_err(|error| format!("failed to read generated policy JSON: {error}"))?;
    let source_policy = fs::read_to_string(&args.source_policy)
        .map_err(|error| format!("failed to read source policy JSON: {error}"))?;
    let rust_policy = fs::read_to_string(&args.rust_policy)
        .map_err(|error| format!("failed to read Rust policy bridge: {error}"))?;

    let policies = parse_policies(&policy_lines)?;
    let fixtures = parse_fixtures(&fixture_lines)?;
    let issues = collect_issues(&policies, &fixtures, &generated_policy, &source_policy, &rust_policy);
    if !issues.is_empty() {
        for issue in &issues {
            eprintln!("ERROR {issue}");
        }
        return Err(format!("router policy failed with {} issue(s)", issues.len()));
    }

    print_summary(&policies, &fixtures);
    Ok(())
}

fn parse_args(values: Vec<String>) -> Result<Args, String> {
    let mut policy_lines = None;
    let mut fixtures = None;
    let mut generated_policy = None;
    let mut source_policy = None;
    let mut rust_policy = None;
    let mut index = 0usize;
    while index < values.len() {
        let key = values[index].as_str();
        let value_index = index.saturating_add(1);
        let Some(value) = values.get(value_index) else {
            return Err(format!("{key} requires a value"));
        };
        match key {
            "--policy-lines" => policy_lines = Some(PathBuf::from(value)),
            "--fixtures" => fixtures = Some(PathBuf::from(value)),
            "--generated-policy" => generated_policy = Some(PathBuf::from(value)),
            "--source-policy" => source_policy = Some(PathBuf::from(value)),
            "--rust-policy" => rust_policy = Some(PathBuf::from(value)),
            _ => return Err(format!("unknown argument: {key}")),
        }
        index = index.saturating_add(2);
    }
    Ok(Args {
        policy_lines: policy_lines.ok_or("missing --policy-lines")?,
        fixtures: fixtures.ok_or("missing --fixtures")?,
        generated_policy: generated_policy.ok_or("missing --generated-policy")?,
        source_policy: source_policy.ok_or("missing --source-policy")?,
        rust_policy: rust_policy.ok_or("missing --rust-policy")?,
    })
}

fn parse_policies(input: &str) -> Result<BTreeMap<String, FamilyPolicy>, String> {
    let mut policies = BTreeMap::new();
    for (line_number, line) in non_empty_lines(input).enumerate() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != FAMILY_FIELD_COUNT || parts[FAMILY_TAG_INDEX] != FAMILY_TAG {
            return Err(parse_error(line_number, line));
        }
        let id = parts[FAMILY_ID_INDEX].to_string();
        let policy = FamilyPolicy {
            id: id.clone(),
            roles: csv_set(parts[FAMILY_ROLES_INDEX]),
            outputs: csv_set(parts[FAMILY_OUTPUTS_INDEX]),
            bounds: BoundsPolicy {
                min_candidates: parse_count(parts[FAMILY_MIN_CANDIDATES_INDEX], line_number)?,
                max_candidates: parse_count(parts[FAMILY_MAX_CANDIDATES_INDEX], line_number)?,
                min_parallelism: parse_count(parts[FAMILY_MIN_PARALLELISM_INDEX], line_number)?,
                max_parallelism: parse_count(parts[FAMILY_MAX_PARALLELISM_INDEX], line_number)?,
                min_quorum: parse_count(parts[FAMILY_MIN_QUORUM_INDEX], line_number)?,
                max_quorum: parse_count(parts[FAMILY_MAX_QUORUM_INDEX], line_number)?,
                min_synthesis: parse_count(parts[FAMILY_MIN_SYNTHESIS_INDEX], line_number)?,
                max_synthesis: parse_count(parts[FAMILY_MAX_SYNTHESIS_INDEX], line_number)?,
                min_repair: parse_count(parts[FAMILY_MIN_REPAIR_INDEX], line_number)?,
                max_repair: parse_count(parts[FAMILY_MAX_REPAIR_INDEX], line_number)?,
            },
            fallbacks: csv_set(parts[FAMILY_FALLBACKS_INDEX]),
            backends: csv_set(parts[FAMILY_BACKENDS_INDEX]),
            deadlines: csv_set(parts[FAMILY_DEADLINES_INDEX]),
        };
        if policies.insert(id, policy).is_some() {
            return Err(format!("ERROR {ISSUE_PARSE} duplicate family at line {line_number}"));
        }
    }
    if policies.is_empty() {
        return Err(format!("ERROR {ISSUE_PARSE} policy must define at least one family"));
    }
    Ok(policies)
}

fn parse_fixtures(input: &str) -> Result<Vec<Fixture>, String> {
    let mut fixtures = Vec::new();
    for (line_number, line) in non_empty_lines(input).enumerate() {
        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() != FIXTURE_FIELD_COUNT || parts[FIXTURE_TAG_INDEX] != FIXTURE_TAG {
            return Err(parse_error(line_number, line));
        }
        fixtures.push(Fixture {
            kind: parts[FIXTURE_KIND_INDEX].to_string(),
            id: parts[FIXTURE_ID_INDEX].to_string(),
            expected_issue: parts[FIXTURE_EXPECTED_INDEX].to_string(),
            family: parts[FIXTURE_FAMILY_INDEX].to_string(),
            roles: csv_set(parts[FIXTURE_ROLES_INDEX]),
            output_contract: parts[FIXTURE_OUTPUT_INDEX].to_string(),
            max_candidates: parse_count(parts[FIXTURE_MAX_CANDIDATES_INDEX], line_number)?,
            max_parallelism: parse_count(parts[FIXTURE_MAX_PARALLELISM_INDEX], line_number)?,
            min_success_quorum: parse_count(parts[FIXTURE_QUORUM_INDEX], line_number)?,
            timeout_class: parts[FIXTURE_DEADLINE_INDEX].to_string(),
            max_synthesis_attempts: parse_count(parts[FIXTURE_SYNTHESIS_INDEX], line_number)?,
            max_repair_attempts: parse_count(parts[FIXTURE_REPAIR_INDEX], line_number)?,
            fallback_policy: parts[FIXTURE_FALLBACK_INDEX].to_string(),
            backend: parts[FIXTURE_BACKEND_INDEX].to_string(),
        });
    }
    if fixtures.is_empty() {
        return Err(format!("ERROR {ISSUE_PARSE} fixtures must not be empty"));
    }
    Ok(fixtures)
}

fn collect_issues(
    policies: &BTreeMap<String, FamilyPolicy>,
    fixtures: &[Fixture],
    generated_policy: &str,
    source_policy: &str,
    rust_policy: &str,
) -> Vec<String> {
    let mut issues = Vec::new();
    if generated_policy != source_policy {
        issues.push(format!("{ISSUE_POLICY_DRIFT} generated policy JSON differs from Nickel export"));
    }
    if !generated_policy.contains(POLICY_SCHEMA_SENTINEL) {
        issues.push(format!("{ISSUE_POLICY_DRIFT} generated policy JSON missing schema"));
    }
    issues.extend(validate_rust_sync(policies, rust_policy));
    for fixture in fixtures {
        issues.extend(validate_fixture(policies, fixture));
    }
    issues
}

fn validate_rust_sync(
    policies: &BTreeMap<String, FamilyPolicy>,
    rust_policy: &str,
) -> Vec<String> {
    let mut issues = Vec::new();
    if !rust_policy.contains(POLICY_JSON_INCLUDE) {
        issues.push(format!("{ISSUE_RUST_SYNC} Rust bridge does not include generated policy JSON"));
    }
    let mut expected_labels = BTreeSet::new();
    for policy in policies.values() {
        expected_labels.insert(policy.id.as_str());
        expected_labels.extend(policy.roles.iter().map(String::as_str));
        expected_labels.extend(policy.outputs.iter().map(String::as_str));
        expected_labels.extend(policy.fallbacks.iter().map(String::as_str));
        expected_labels.extend(policy.deadlines.iter().map(String::as_str));
    }
    for label in expected_labels {
        let quoted = format!("\"{label}\"");
        if !rust_policy.contains(&quoted) {
            issues.push(format!("{ISSUE_RUST_SYNC} Rust bridge missing label {label}"));
        }
    }
    issues
}

fn validate_fixture(
    policies: &BTreeMap<String, FamilyPolicy>,
    fixture: &Fixture,
) -> Vec<String> {
    let actual_issue = evaluate_fixture(policies, fixture);
    match fixture.kind.as_str() {
        POSITIVE_KIND => validate_positive_fixture(fixture, actual_issue),
        NEGATIVE_KIND => validate_negative_fixture(fixture, actual_issue),
        _ => vec![format!("{ISSUE_PARSE} {} has unknown kind {}", fixture.id, fixture.kind)],
    }
}

fn validate_positive_fixture(fixture: &Fixture, actual_issue: Option<&'static str>) -> Vec<String> {
    if fixture.expected_issue != OK_ISSUE {
        return vec![format!(
            "{ISSUE_PARSE} positive fixture {} must declare expected issue {OK_ISSUE}",
            fixture.id
        )];
    }
    if let Some(issue) = actual_issue {
        return vec![format!("{issue} positive fixture {} failed", fixture.id)];
    }
    Vec::new()
}

fn validate_negative_fixture(fixture: &Fixture, actual_issue: Option<&'static str>) -> Vec<String> {
    let Some(issue) = actual_issue else {
        return vec![format!(
            "{} negative fixture {} unexpectedly passed",
            fixture.expected_issue, fixture.id
        )];
    };
    if issue != fixture.expected_issue {
        return vec![format!(
            "{issue} negative fixture {} expected {}",
            fixture.id, fixture.expected_issue
        )];
    }
    Vec::new()
}

fn evaluate_fixture(
    policies: &BTreeMap<String, FamilyPolicy>,
    fixture: &Fixture,
) -> Option<&'static str> {
    let Some(policy) = policies.get(&fixture.family) else {
        return Some(ISSUE_UNKNOWN_FAMILY);
    };
    if !same_set(&policy.roles, &fixture.roles) {
        return Some(ISSUE_INVALID_ROLE_COMBO);
    }
    if !policy.outputs.contains(&fixture.output_contract) {
        return Some(ISSUE_OUTPUT_CONTRACT);
    }
    if !within_bounds(&policy.bounds, fixture) {
        return Some(ISSUE_BOUND_OUT_OF_RANGE);
    }
    if !policy.deadlines.contains(&fixture.timeout_class) {
        return Some(ISSUE_UNKNOWN_DEADLINE);
    }
    if !policy.fallbacks.contains(&fixture.fallback_policy) {
        return Some(ISSUE_DISALLOWED_FALLBACK);
    }
    if !policy.backends.contains(&fixture.backend) {
        return Some(ISSUE_UNKNOWN_BACKEND);
    }
    None
}

fn within_bounds(bounds: &BoundsPolicy, fixture: &Fixture) -> bool {
    in_range(fixture.max_candidates, bounds.min_candidates, bounds.max_candidates)
        && in_range(fixture.max_parallelism, bounds.min_parallelism, bounds.max_parallelism)
        && in_range(fixture.min_success_quorum, bounds.min_quorum, bounds.max_quorum)
        && in_range(
            fixture.max_synthesis_attempts,
            bounds.min_synthesis,
            bounds.max_synthesis,
        )
        && in_range(fixture.max_repair_attempts, bounds.min_repair, bounds.max_repair)
        && fixture.min_success_quorum <= fixture.max_candidates
        && fixture.max_parallelism <= fixture.max_candidates
}

fn in_range(value: u32, minimum: u32, maximum: u32) -> bool {
    value >= minimum && value <= maximum
}

fn same_set(expected: &BTreeSet<String>, actual: &BTreeSet<String>) -> bool {
    expected == actual
}

fn non_empty_lines(input: &str) -> impl Iterator<Item = &str> {
    input.lines().map(str::trim).filter(|line| !line.is_empty())
}

fn csv_set(input: &str) -> BTreeSet<String> {
    input
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

fn parse_count(input: &str, line_number: usize) -> Result<u32, String> {
    input
        .parse::<u32>()
        .map_err(|error| format!("ERROR {ISSUE_PARSE} invalid number at line {line_number}: {error}"))
}

fn parse_error(line_number: usize, line: &str) -> String {
    format!("ERROR {ISSUE_PARSE} invalid line {line_number}: {line}")
}

fn print_summary(policies: &BTreeMap<String, FamilyPolicy>, fixtures: &[Fixture]) {
    let positive_count = fixtures
        .iter()
        .filter(|fixture| fixture.kind == POSITIVE_KIND)
        .count();
    let negative_count = fixtures
        .iter()
        .filter(|fixture| fixture.kind == NEGATIVE_KIND)
        .count();
    println!("router policy families: {}", policies.len());
    println!("router policy positive fixtures: {positive_count}");
    println!("router policy negative fixtures: {negative_count}");
    println!("router policy: ok");
}
