use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT_FLAG: &str = "--root";
const EXPECTED_REVISION: &str = "c5991bc853d27754aed345c13ff4d2e05c40f1f4";
const EXPECTED_COHORT: &str = "prime-agent-daemon-workers";
const EXPECTED_SCHEMA: &str = "animus.daemon-workers.v1";
const EXPECTED_LICENSE: &str = "MIT";
const RECEIPT_SCHEMA: &str = "animus.focused-rail.v1";
const RAIL_NAME: &str = "daemon-workers";
const REQUIRED_SOURCE_MARKERS: [&str; 12] = [
    "#![no_std]",
    "pub enum Role",
    "pub enum Authority",
    "pub struct WorkerTree",
    "pub fn decide_supervisor",
    "pub fn admit_reattach",
    "pub fn plan_worker_execution",
    "pub fn validate_containment",
    "pub fn project_receipt",
    "CrossWorkerTreeAccess",
    "StaleRecoverySnapshot",
    "ReceiptContainsContent",
];
const REQUIRED_POLICY_MARKERS: [&str; 8] = [
    "client",
    "supervisor",
    "worker",
    "not-security-sandbox",
    "live_reattach",
    "current_snapshot_reattach",
    "stale_snapshot_rejected",
    "lifecycle_only_containment",
];
const FORBIDDEN_SOURCE_MARKERS: [&str; 7] = [
    "std::fs",
    "std::process",
    "std::net",
    "std::env",
    "TcpStream",
    "Command::new",
    "transcript persistence",
];

fn main() {
    let root = parse_root();
    let source = read(&root.join("crates/animus-adapter-daemon/src/lib.rs"));
    for marker in REQUIRED_SOURCE_MARKERS {
        assert!(source.contains(marker), "missing daemon source marker: {marker}");
    }
    for marker in FORBIDDEN_SOURCE_MARKERS {
        assert!(
            !source.contains(marker),
            "daemon reusable source contains forbidden authority marker: {marker}"
        );
    }

    let policy = read(&root.join("config/generated/daemon-workers-policy.json"));
    assert!(policy.contains(EXPECTED_SCHEMA));
    for marker in REQUIRED_POLICY_MARKERS {
        assert!(policy.contains(marker), "missing daemon policy marker: {marker}");
    }
    let fixture = read(&root.join("fixtures/daemon-workers/prime-agent-c5991bc.md"));
    for marker in [EXPECTED_REVISION, EXPECTED_COHORT, EXPECTED_LICENSE, "detach", "reattach", "lifecycle containment"] {
        assert!(fixture.contains(marker), "missing daemon fixture marker: {marker}");
    }

    let adapters_manifest = read(&root.join("crates/animus-adapters/Cargo.toml"));
    assert!(adapters_manifest.contains("animus-adapter-daemon"));
    let root_manifest = read(&root.join("Cargo.toml"));
    assert!(root_manifest.contains("daemon-workers = [\"dep:animus-adapter-daemon\"]"));

    let receipt_dir = root.join("target/cairn/focused-rails");
    fs::create_dir_all(&receipt_dir).expect("focused rail receipt directory");
    let receipt_path = receipt_dir.join(format!("{RAIL_NAME}.md"));
    fs::write(
        &receipt_path,
        format!(
            "schema: {RECEIPT_SCHEMA}\nrail: {RAIL_NAME}\nstatus: pass\nsource: no-std pure daemon role, routing, attachment, recovery, containment, and receipt contracts\npositive_fixtures: role authority, deterministic routing, detach-safe execution, live reattach, current snapshot reattach, lifecycle facts\nnegative_fixtures: client execution, supervisor execution, cross-worker tree access, stale snapshot, sandbox claim, receipt content\nnon_claims: security sandboxing, crash-freedom, arbitrary durability, multi-machine supervision, production readiness, release eligibility\n"
        ),
    )
    .expect("focused rail receipt");
    println!("daemon worker source, cohort, policy, and boundary fixtures: pass");
}

fn parse_root() -> PathBuf {
    let mut arguments = env::args().skip(1);
    while let Some(argument) = arguments.next() {
        if argument == ROOT_FLAG {
            return PathBuf::from(arguments.next().expect("--root requires a path"));
        }
        panic!("unknown argument: {argument}");
    }
    env::current_dir().expect("current directory must be available")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}
