//! Repository-owned bounded-execution feasibility checker.
//!
//! The functional core parses and evaluates supplied text. The shell only reads
//! bounded files and prints a metadata-only decision.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::fs;
use std::process::ExitCode;

const SCHEMA: &str = "animus.bounded-execution-evaluation.v1";
const MAX_PROFILE_BYTES: u64 = 64 * 1024;
const MAX_FIXTURE_BYTES: u64 = 16 * 1024;
const HASH_HEX_CHARS: usize = 64;
const EXPECTED_CONSUMERS: [&str; 4] = ["animus", "octet", "nickel_export", "mantle"];
const SEMANTIC_CLASSES: [&str; 8] = [
    "environment",
    "filesystem_root",
    "network",
    "timeout",
    "output",
    "cancellation",
    "signal",
    "child_cleanup",
];
const CAPABILITY_STATES: [&str; 7] = [
    "requested",
    "configured",
    "enforced",
    "observed",
    "unavailable",
    "unsupported",
    "not_applicable",
];
const DISPOSITIONS: [&str; 6] = [
    "lossless",
    "consumer_extension",
    "mechanism_fact",
    "retained_authority",
    "interoperability",
    "blocker",
];
const OUTCOME_CLASSES: [&str; 11] = [
    "success",
    "nonzero_exit",
    "launch_denial",
    "missing_executable",
    "policy_rejection",
    "timeout",
    "cancellation",
    "signal_termination",
    "output_overflow",
    "cleanup_failure",
    "unsupported_mechanism",
];
const REQUIRED_NON_CLAIMS: [&str; 9] = [
    "sandbox_equivalence",
    "hermeticity",
    "network_denial",
    "filesystem_isolation",
    "executor_correctness",
    "proof_soundness",
    "build_correctness",
    "future_determinism",
    "release_eligibility",
];
const REQUIRED_FIXTURES: [&str; 17] = [
    "success",
    "nonzero_exit",
    "timeout",
    "launch_denial",
    "missing_executable",
    "environment_rejection",
    "path_root_escape",
    "cancellation",
    "signal_termination",
    "output_overflow",
    "network_policy_difference",
    "cleanup_failure",
    "unsupported_mechanism",
    "stale_source",
    "unsafe_diagnostic",
    "weakened_non_claim",
    "semantic_loss",
];
const FORBIDDEN_CLAIMS: [&str; 3] = [
    "universally equivalent",
    "proves hermeticity",
    "sandboxed=true",
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct Consumer {
    id: String,
    revision: String,
    source_blake3: String,
    license: String,
    owner: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Mapping {
    consumer: String,
    semantic_class: String,
    state: String,
    disposition: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Outcome {
    consumer: String,
    class: String,
    disposition: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Criteria {
    minimum_consumers: usize,
    independently_useful_release: bool,
    migration_strategy_ready: bool,
    dependency_cycle: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Profile {
    schema: String,
    consumers: Vec<Consumer>,
    mappings: Vec<Mapping>,
    outcomes: Vec<Outcome>,
    criteria: Criteria,
    non_claims: Vec<String>,
    next_action: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DecisionStatus {
    Extract,
    Incubate,
    Blocked,
}

impl DecisionStatus {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Extract => "extract",
            Self::Incubate => "incubate",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Decision {
    status: DecisionStatus,
    blocker_count: usize,
    reasons: Vec<&'static str>,
}

fn parse_bool(value: &str, field: &str) -> Result<bool, String> {
    match value {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("BOUNDED_EXEC_INVALID_BOOL:{field}")),
    }
}

fn parse_profile(text: &str) -> Result<Profile, Vec<String>> {
    let mut schema = None;
    let mut consumers = Vec::new();
    let mut mappings = Vec::new();
    let mut outcomes = Vec::new();
    let mut criteria = None;
    let mut non_claims = None;
    let mut next_action = None;
    let mut issues = Vec::new();

    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let fields = line.split('|').collect::<Vec<_>>();
        match fields.as_slice() {
            ["schema", value] => schema = Some((*value).to_owned()),
            ["consumer", id, revision, source_blake3, license, owner] => consumers.push(Consumer {
                id: (*id).to_owned(),
                revision: (*revision).to_owned(),
                source_blake3: (*source_blake3).to_owned(),
                license: (*license).to_owned(),
                owner: (*owner).to_owned(),
            }),
            ["mapping", consumer, semantic_class, state, _mechanism, disposition, _observation] => {
                mappings.push(Mapping {
                    consumer: (*consumer).to_owned(),
                    semantic_class: (*semantic_class).to_owned(),
                    state: (*state).to_owned(),
                    disposition: (*disposition).to_owned(),
                });
            }
            ["outcome", consumer, class, disposition, _native_detail] => outcomes.push(Outcome {
                consumer: (*consumer).to_owned(),
                class: (*class).to_owned(),
                disposition: (*disposition).to_owned(),
            }),
            ["criteria", minimum, release, migration, cycle] => {
                let parsed = minimum
                    .parse::<usize>()
                    .map_err(|_| String::from("BOUNDED_EXEC_INVALID_MINIMUM"))
                    .and_then(|minimum_consumers| {
                        Ok(Criteria {
                            minimum_consumers,
                            independently_useful_release: parse_bool(release, "release")?,
                            migration_strategy_ready: parse_bool(migration, "migration")?,
                            dependency_cycle: parse_bool(cycle, "cycle")?,
                        })
                    });
                match parsed {
                    Ok(value) => criteria = Some(value),
                    Err(issue) => issues.push(issue),
                }
            }
            ["nonclaims", values] => {
                non_claims = Some(values.split(',').map(str::to_owned).collect());
            }
            ["next_action", value] => next_action = Some((*value).to_owned()),
            _ => issues.push(String::from("BOUNDED_EXEC_MALFORMED_RECORD")),
        }
    }

    if !issues.is_empty() {
        return Err(issues);
    }
    Ok(Profile {
        schema: schema.ok_or_else(|| vec![String::from("BOUNDED_EXEC_MISSING_SCHEMA")])?,
        consumers,
        mappings,
        outcomes,
        criteria: criteria.ok_or_else(|| vec![String::from("BOUNDED_EXEC_MISSING_CRITERIA")])?,
        non_claims: non_claims
            .ok_or_else(|| vec![String::from("BOUNDED_EXEC_MISSING_NON_CLAIMS")])?,
        next_action: next_action
            .ok_or_else(|| vec![String::from("BOUNDED_EXEC_MISSING_NEXT_ACTION")])?,
    })
}

fn validate_profile(profile: &Profile, raw_text: &str) -> Vec<String> {
    let mut issues = Vec::new();
    if profile.schema != SCHEMA {
        issues.push(String::from("BOUNDED_EXEC_UNKNOWN_SCHEMA"));
    }
    let mut consumer_ids = BTreeSet::new();
    for consumer in &profile.consumers {
        if !EXPECTED_CONSUMERS.contains(&consumer.id.as_str()) {
            issues.push(format!("BOUNDED_EXEC_UNKNOWN_CONSUMER:{}", consumer.id));
        }
        if !consumer_ids.insert(consumer.id.as_str()) {
            issues.push(format!("BOUNDED_EXEC_DUPLICATE_CONSUMER:{}", consumer.id));
        }
        if !valid_blake3(&consumer.source_blake3) {
            issues.push(format!(
                "BOUNDED_EXEC_INVALID_SOURCE_IDENTITY:{}",
                consumer.id
            ));
        }
        if consumer.revision.is_empty() || consumer.license.is_empty() || consumer.owner.is_empty()
        {
            issues.push(format!("BOUNDED_EXEC_INCOMPLETE_CONSUMER:{}", consumer.id));
        }
    }
    for expected in EXPECTED_CONSUMERS {
        if !consumer_ids.contains(expected) {
            issues.push(format!("BOUNDED_EXEC_MISSING_CONSUMER:{expected}"));
        }
    }

    let mut mapping_keys = BTreeSet::new();
    let mut per_consumer = BTreeMap::<&str, BTreeSet<&str>>::new();
    for mapping in &profile.mappings {
        if !EXPECTED_CONSUMERS.contains(&mapping.consumer.as_str()) {
            issues.push(format!(
                "BOUNDED_EXEC_UNKNOWN_MAPPING_CONSUMER:{}",
                mapping.consumer
            ));
        }
        if !SEMANTIC_CLASSES.contains(&mapping.semantic_class.as_str()) {
            issues.push(format!(
                "BOUNDED_EXEC_UNKNOWN_SEMANTIC:{}",
                mapping.semantic_class
            ));
        }
        if !CAPABILITY_STATES.contains(&mapping.state.as_str()) {
            issues.push(format!("BOUNDED_EXEC_UNKNOWN_STATE:{}", mapping.state));
        }
        if !DISPOSITIONS.contains(&mapping.disposition.as_str()) {
            issues.push(format!(
                "BOUNDED_EXEC_UNKNOWN_DISPOSITION:{}",
                mapping.disposition
            ));
        }
        let key = format!("{}:{}", mapping.consumer, mapping.semantic_class);
        if !mapping_keys.insert(key.clone()) {
            issues.push(format!("BOUNDED_EXEC_DUPLICATE_MAPPING:{key}"));
        }
        per_consumer
            .entry(mapping.consumer.as_str())
            .or_default()
            .insert(mapping.semantic_class.as_str());
        let cannot_be_lossless = matches!(
            mapping.state.as_str(),
            "requested" | "configured" | "unavailable" | "unsupported"
        );
        if cannot_be_lossless && mapping.disposition == "lossless" {
            issues.push(format!("BOUNDED_EXEC_CLAIM_LAUNDERING:{key}"));
        }
    }
    for consumer in EXPECTED_CONSUMERS {
        let classes = per_consumer.get(consumer);
        for semantic_class in SEMANTIC_CLASSES {
            if classes.is_none_or(|values| !values.contains(semantic_class)) {
                issues.push(format!(
                    "BOUNDED_EXEC_MISSING_SEMANTIC:{consumer}:{semantic_class}"
                ));
            }
        }
    }

    for outcome in &profile.outcomes {
        if !EXPECTED_CONSUMERS.contains(&outcome.consumer.as_str()) {
            issues.push(format!(
                "BOUNDED_EXEC_UNKNOWN_OUTCOME_CONSUMER:{}",
                outcome.consumer
            ));
        }
        if !OUTCOME_CLASSES.contains(&outcome.class.as_str()) {
            issues.push(format!("BOUNDED_EXEC_UNKNOWN_OUTCOME:{}", outcome.class));
        }
        if !DISPOSITIONS.contains(&outcome.disposition.as_str()) {
            issues.push(format!(
                "BOUNDED_EXEC_UNKNOWN_OUTCOME_DISPOSITION:{}",
                outcome.disposition
            ));
        }
    }
    let non_claims = profile
        .non_claims
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    for required in REQUIRED_NON_CLAIMS {
        if !non_claims.contains(required) {
            issues.push(format!("BOUNDED_EXEC_MISSING_NON_CLAIM:{required}"));
        }
    }
    for forbidden in FORBIDDEN_CLAIMS {
        if raw_text.contains(forbidden) {
            issues.push(format!("BOUNDED_EXEC_OVERCLAIM:{forbidden}"));
        }
    }
    if profile.next_action.trim().is_empty() {
        issues.push(String::from("BOUNDED_EXEC_MISSING_NEXT_ACTION"));
    }
    issues
}

fn validate_fixtures(text: &str) -> Vec<String> {
    let mut issues = Vec::new();
    let mut fixture_ids = BTreeSet::new();
    for line in text.lines().filter(|line| !line.trim().is_empty()) {
        let fields = line.split('|').collect::<Vec<_>>();
        match fields.as_slice() {
            ["case", kind @ ("positive" | "negative"), id, _consumers, _expected] => {
                if !fixture_ids.insert(*id) {
                    issues.push(format!("BOUNDED_EXEC_DUPLICATE_FIXTURE:{id}"));
                }
                if *kind == "positive" && id.contains("unsafe") {
                    issues.push(format!("BOUNDED_EXEC_INVALID_POSITIVE_FIXTURE:{id}"));
                }
            }
            _ => issues.push(String::from("BOUNDED_EXEC_MALFORMED_FIXTURE")),
        }
    }
    for required in REQUIRED_FIXTURES {
        if !fixture_ids.contains(required) {
            issues.push(format!("BOUNDED_EXEC_MISSING_FIXTURE:{required}"));
        }
    }
    for unsafe_marker in ["credential=", "/home/", "raw_diagnostic="] {
        if text.contains(unsafe_marker) {
            issues.push(format!("BOUNDED_EXEC_UNSAFE_FIXTURE:{unsafe_marker}"));
        }
    }
    issues
}

fn evaluate(profile: &Profile) -> Decision {
    let blocker_count = profile
        .mappings
        .iter()
        .filter(|mapping| mapping.disposition == "blocker")
        .count()
        + profile
            .outcomes
            .iter()
            .filter(|outcome| outcome.disposition == "blocker")
            .count();
    let mut reasons = Vec::new();
    if blocker_count > 0 {
        reasons.push("semantic_loss_or_missing_common_observation");
    }
    if profile.criteria.dependency_cycle {
        reasons.push("dependency_cycle");
    }
    if profile.consumers.len() < profile.criteria.minimum_consumers {
        reasons.push("insufficient_independent_consumers");
    }
    if !profile.criteria.independently_useful_release {
        reasons.push("independent_release_value_unproven");
    }
    if !profile.criteria.migration_strategy_ready {
        reasons.push("migration_strategy_not_ready");
    }
    let status = if blocker_count > 0 || profile.criteria.dependency_cycle {
        DecisionStatus::Blocked
    } else if reasons.is_empty() {
        DecisionStatus::Extract
    } else {
        DecisionStatus::Incubate
    };
    Decision {
        status,
        blocker_count,
        reasons,
    }
}

fn valid_blake3(value: &str) -> bool {
    value.len() == HASH_HEX_CHARS
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn read_bounded(path: &str, max_bytes: u64) -> Result<String, String> {
    let metadata =
        fs::metadata(path).map_err(|error| format!("read metadata for {path}: {error}"))?;
    if metadata.len() > max_bytes {
        return Err(format!("bounded input exceeds limit: {path}"));
    }
    fs::read_to_string(path).map_err(|error| format!("read {path}: {error}"))
}

fn self_test() -> Result<(), String> {
    let blocked = fixture_profile(true, false, false, false);
    assert_eq!(evaluate(&blocked).status, DecisionStatus::Blocked);
    let incubate = fixture_profile(false, false, false, false);
    assert_eq!(evaluate(&incubate).status, DecisionStatus::Incubate);
    let extract = fixture_profile(false, true, true, false);
    assert_eq!(evaluate(&extract).status, DecisionStatus::Extract);
    let cycle = fixture_profile(false, true, true, true);
    assert_eq!(evaluate(&cycle).status, DecisionStatus::Blocked);
    let mut single_consumer = fixture_profile(false, true, true, false);
    single_consumer.consumers.truncate(1);
    assert_eq!(evaluate(&single_consumer).status, DecisionStatus::Incubate);

    let duplicate = format!(
        "schema|{SCHEMA}\nconsumer|animus|rev|{}|license|owner\nconsumer|animus|rev|{}|license|owner\ncriteria|2|false|false|false\nnonclaims|{}\nnext_action|next",
        "a".repeat(HASH_HEX_CHARS),
        "a".repeat(HASH_HEX_CHARS),
        REQUIRED_NON_CLAIMS.join(",")
    );
    let parsed = parse_profile(&duplicate).map_err(|issues| issues.join(","))?;
    let duplicate_issues = validate_profile(&parsed, &duplicate);
    assert!(duplicate_issues
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_DUPLICATE_CONSUMER")));
    assert!(duplicate_issues
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_MISSING_CONSUMER")));
    assert!(duplicate_issues
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_MISSING_SEMANTIC")));
    assert!(!valid_blake3("ABC"));
    assert!(validate_fixtures("case|positive|success|all|mapped")
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_MISSING_FIXTURE")));
    assert!(validate_fixtures("credential=unsafe")
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_UNSAFE_FIXTURE")));

    let complete = complete_profile_text();
    let parsed = parse_profile(&complete).map_err(|issues| issues.join(","))?;
    let overclaim = format!("{complete} universally equivalent");
    assert!(validate_profile(&parsed, &overclaim)
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_OVERCLAIM")));

    let unknown_state = complete.replace(
        "mapping|animus|environment|requested|",
        "mapping|animus|environment|mystery|",
    );
    let parsed_unknown = parse_profile(&unknown_state).map_err(|issues| issues.join(","))?;
    assert!(validate_profile(&parsed_unknown, &unknown_state)
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_UNKNOWN_STATE")));

    let unknown_outcome = format!("{complete}\noutcome|animus|mystery|blocker|detail");
    let parsed_outcome = parse_profile(&unknown_outcome).map_err(|issues| issues.join(","))?;
    assert!(validate_profile(&parsed_outcome, &unknown_outcome)
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_UNKNOWN_OUTCOME")));

    let laundering = complete.replace(
        "mapping|animus|environment|requested|mechanism|consumer_extension|",
        "mapping|animus|environment|requested|mechanism|lossless|",
    );
    let parsed_laundering = parse_profile(&laundering).map_err(|issues| issues.join(","))?;
    assert!(validate_profile(&parsed_laundering, &laundering)
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_CLAIM_LAUNDERING")));

    let weakened = complete.replace(",release_eligibility", "");
    let parsed_weakened = parse_profile(&weakened).map_err(|issues| issues.join(","))?;
    assert!(validate_profile(&parsed_weakened, &weakened)
        .iter()
        .any(|issue| issue.starts_with("BOUNDED_EXEC_MISSING_NON_CLAIM")));
    Ok(())
}

fn fixture_profile(
    blocker: bool,
    independently_useful_release: bool,
    migration_strategy_ready: bool,
    dependency_cycle: bool,
) -> Profile {
    let disposition = if blocker {
        "blocker"
    } else {
        "consumer_extension"
    };
    Profile {
        schema: SCHEMA.to_owned(),
        consumers: EXPECTED_CONSUMERS
            .iter()
            .map(|id| Consumer {
                id: (*id).to_owned(),
                revision: String::from("revision"),
                source_blake3: "a".repeat(HASH_HEX_CHARS),
                license: String::from("license"),
                owner: String::from("owner"),
            })
            .collect(),
        mappings: vec![Mapping {
            consumer: String::from("animus"),
            semantic_class: String::from("signal"),
            state: String::from("unsupported"),
            disposition: String::from(disposition),
        }],
        outcomes: Vec::new(),
        criteria: Criteria {
            minimum_consumers: 2,
            independently_useful_release,
            migration_strategy_ready,
            dependency_cycle,
        },
        non_claims: REQUIRED_NON_CLAIMS
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
        next_action: String::from("run one discriminating check"),
    }
}

fn complete_profile_text() -> String {
    let mut lines = vec![format!("schema|{SCHEMA}")];
    for consumer in EXPECTED_CONSUMERS {
        lines.push(format!(
            "consumer|{consumer}|revision|{}|license|owner",
            "a".repeat(HASH_HEX_CHARS)
        ));
        for semantic_class in SEMANTIC_CLASSES {
            lines.push(format!(
                "mapping|{consumer}|{semantic_class}|requested|mechanism|consumer_extension|observation"
            ));
        }
    }
    lines.push(String::from("criteria|2|false|false|false"));
    lines.push(format!("nonclaims|{}", REQUIRED_NON_CLAIMS.join(",")));
    lines.push(String::from("next_action|next"));
    lines.join("\n")
}

fn run() -> Result<(), String> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    if args.as_slice() == ["--self-test"] {
        self_test()?;
        println!("bounded-execution evaluation checker self-test: pass");
        return Ok(());
    }
    let [profile_flag, profile_path, fixtures_flag, fixtures_path, hash_flag, profile_blake3] =
        args.as_slice()
    else {
        return Err(String::from(
            "usage: check-bounded-execution-evaluation --profile PATH --fixtures PATH --profile-blake3 HASH",
        ));
    };
    if profile_flag != "--profile"
        || fixtures_flag != "--fixtures"
        || hash_flag != "--profile-blake3"
    {
        return Err(String::from("invalid checker arguments"));
    }
    if !valid_blake3(profile_blake3) {
        return Err(String::from("BOUNDED_EXEC_INVALID_PROFILE_IDENTITY"));
    }
    let profile_text = read_bounded(profile_path, MAX_PROFILE_BYTES)?;
    let fixture_text = read_bounded(fixtures_path, MAX_FIXTURE_BYTES)?;
    let profile = parse_profile(&profile_text).map_err(|issues| issues.join("\n"))?;
    let mut issues = validate_profile(&profile, &profile_text);
    issues.extend(validate_fixtures(&fixture_text));
    if !issues.is_empty() {
        return Err(issues.join("\n"));
    }
    let decision = evaluate(&profile);
    println!("schema=animus.bounded-execution-decision.v1");
    println!("status={}", decision.status.as_str());
    println!("profile_blake3={profile_blake3}");
    println!("consumer_count={}", profile.consumers.len());
    println!("mapping_count={}", profile.mappings.len());
    println!("outcome_count={}", profile.outcomes.len());
    println!("blocker_count={}", decision.blocker_count);
    println!("reasons={}", decision.reasons.join(","));
    println!("next_action={}", profile.next_action);
    println!("non_claims={}", profile.non_claims.join(","));
    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error}");
            ExitCode::FAILURE
        }
    }
}
