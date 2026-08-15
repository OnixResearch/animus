// r[verify agent_harness_runtime.zeron_agent_harness.verification]
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT_FLAG: &str = "--root";
const REQUIRED_FILE_COUNT: usize = 7;
const ADAPTER_SOURCE_PATH: &str = "crates/animus-adapter-zeron-harness/src/lib.rs";
const HOST_SOURCE_PATH: &str = "crates/animus-std-zeron-harness/src/lib.rs";
const HOST_TESTS_PATH: &str = "crates/animus-std-zeron-harness/tests/bounded_delegation.rs";
const DOCS_PATH: &str = "docs/zeron-harness-command-plane.md";
const ADAPTER_SOURCE_MARKERS: [&str; 10] = [
    "#![no_std]",
    "#![forbid(unsafe_code)]",
    "COMMAND_IDENTITY_DOMAIN",
    "evaluate_command",
    "begin_execution",
    "mark-processed-before-execute",
    "DispatchNextTurn",
    "FailUnavailable",
    "BoundedAgentProcessService",
    "validate_metadata_only",
];
const HOST_SOURCE_MARKERS: [&str; 6] = [
    "bounded-exec",
    "BoundedExecAgentProcessService",
    "ACCEPTED_EXIT_CODE",
    "TerminationScope",
    "OutcomePolicy",
    "map_err(|_| ServiceUnavailable)",
];
const ADAPTER_TESTS_MARKERS: [&str; 6] = [
    "owner_gated_execution_and_non_owner_queue",
    "steer_with_no_active_run_dispatches_next_turn",
    "offline_harness_keeps_commands_queued",
    "processed_commands_are_skipped",
    "process_request_requires_explicit_limits",
    "receipt_candidate_rejects_raw_material",
];
const HOST_TESTS_MARKERS: [&str; 4] = [
    "missing_executable_is_service_unavailable",
    "deadline_produces_timed_out_class",
    "output_flood_respects_retention_bound",
    "bounded_program_succeeds_with_observed_counts",
];
const DOCS_MARKERS: [&str; 6] = [
    "Zeron",
    "owner-gated",
    "mark-processed-before-execute",
    "metadata-only",
    "## Non-claims",
    "zeron-harness-host",
];

fn main() {
    let root = parse_root();
    let required_paths = required_paths(&root);
    assert_eq!(required_paths.len(), REQUIRED_FILE_COUNT);
    for path in &required_paths {
        assert!(
            path.is_file(),
            "missing required Zeron harness file: {}",
            path.display()
        );
    }

    let adapter_source = read(&root.join(ADAPTER_SOURCE_PATH));
    assert_contains_all(
        &adapter_source,
        ADAPTER_SOURCE_MARKERS,
        "adapter source",
    );
    assert_contains_all(
        &adapter_source,
        ADAPTER_TESTS_MARKERS,
        "adapter tests",
    );

    let host_source = read(&root.join(HOST_SOURCE_PATH));
    assert_contains_all(&host_source, HOST_SOURCE_MARKERS, "host source");

    let host_tests = read(&root.join(HOST_TESTS_PATH));
    assert_contains_all(&host_tests, HOST_TESTS_MARKERS, "host tests");

    let docs = read(&root.join(DOCS_PATH));
    assert_contains_all(&docs, DOCS_MARKERS, "docs");

    let root_manifest = read(&root.join("Cargo.toml"));
    assert_contains_all(
        &root_manifest,
        ["zeron-harness-adapter", "zeron-harness-host"],
        "root manifest features",
    );
    println!("zeron harness source, contract, integration, doc, and feature rails: pass");
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
        root.join("crates/animus-adapter-zeron-harness/Cargo.toml"),
        root.join(ADAPTER_SOURCE_PATH),
        root.join("crates/animus-std-zeron-harness/Cargo.toml"),
        root.join(HOST_SOURCE_PATH),
        root.join(HOST_TESTS_PATH),
        root.join(DOCS_PATH),
    ]
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}

fn assert_contains_all<const COUNT: usize>(text: &str, markers: [&str; COUNT], subject: &str) {
    for marker in markers {
        assert!(
            text.contains(marker),
            "missing required {subject} marker: {marker}"
        );
    }
}
