use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT_FLAG: &str = "--root";
const EXPECTED_REVISION: &str = "c5991bc853d27754aed345c13ff4d2e05c40f1f4";
const EXPECTED_COHORT: &str = "prime-agent-continual-harness";
const EXPECTED_SCHEMA: &str = "animus.continual-harness.v1";
const EXPECTED_LICENSE: &str = "MIT";
const REQUIRED_FILE_COUNT: usize = 7;
const ARCHIVED_TASKS_PATH: &str =
    "cairn/archive/2026-08-07-adapt-prime-agent-continual-harness/tasks.md";
const REQUIRED_MARKERS: [&str; 8] = [
    "#![no_std]",
    "plan_promotion",
    "requests_promotion",
    "ReceiptContainsContent",
    "SNAPSHOT_DOMAIN",
    "BLAKE3_HEX_CHARS",
    "UnsupportedTarget",
    "ForgedSnapshot",
];
const RECEIPT_MARKERS: [&str; 6] = [
    "policy_identity",
    "target_class",
    "operation",
    "snapshot_identity",
    "state_identity",
    "non_claims",
];
const NEGATIVE_FIXTURE_MARKERS: [&str; 5] = [
    "base_prompt",
    "not-lesson-correctness",
    "not-model-improvement",
    "not-automatic-promotion",
    "not-release-eligibility",
];

fn main() {
    let root = parse_root();
    let required_paths = required_paths(&root);
    assert_eq!(required_paths.len(), REQUIRED_FILE_COUNT);
    for path in &required_paths {
        assert!(path.is_file(), "missing required continual-harness file: {}", path.display());
    }

    let source = read(&root.join("crates/animus-adapter-continual/src/lib.rs"));
    for marker in REQUIRED_MARKERS {
        assert!(source.contains(marker), "missing continual-harness source marker: {marker}");
    }
    for marker in RECEIPT_MARKERS {
        assert!(source.contains(marker), "missing metadata receipt marker: {marker}");
    }

    let fixture = read(&root.join("fixtures/continual-harness/prime-agent-c5991bc.md"));
    assert_contains_all(&fixture, [EXPECTED_REVISION, EXPECTED_COHORT, EXPECTED_LICENSE, "/refine"]);
    assert_contains_all(&fixture, NEGATIVE_FIXTURE_MARKERS);

    let policy = read(&root.join("config/generated/continual-harness-policy.json"));
    assert_contains_all(&policy, [EXPECTED_SCHEMA, EXPECTED_COHORT, EXPECTED_REVISION, EXPECTED_LICENSE]);
    assert_contains_all(&policy, [
        "supplemental_prompt",
        "memory",
        "skill_description",
        "subagent_specification",
        "evidence_required",
        "conflict_detection",
        "promotion_requires_host_action",
    ]);
    println!("continual harness source, cohort, policy, and negative-boundary fixtures: pass");
}

fn parse_root() -> PathBuf {
    let arguments: Vec<String> = env::args().collect();
    let mut index = 1usize;
    while index < arguments.len() {
        if arguments[index] == ROOT_FLAG {
            let next = index + 1usize;
            assert!(next < arguments.len(), "--root requires a path");
            return PathBuf::from(&arguments[next]);
        }
        index += 1usize;
    }
    env::current_dir().expect("current directory must be available")
}

fn required_paths(root: &Path) -> Vec<PathBuf> {
    vec![
        root.join("Cargo.toml"),
        root.join("crates/animus-adapter-continual/Cargo.toml"),
        root.join("crates/animus-adapter-continual/src/lib.rs"),
        root.join("config/continual-harness-policy.ncl"),
        root.join("config/generated/continual-harness-policy.json"),
        root.join("fixtures/continual-harness/prime-agent-c5991bc.md"),
        root.join(ARCHIVED_TASKS_PATH),
    ]
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

fn assert_contains_all<const COUNT: usize>(text: &str, markers: [&str; COUNT]) {
    for marker in markers {
        assert!(text.contains(marker), "missing required marker: {marker}");
    }
}
