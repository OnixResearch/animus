use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT_FLAG: &str = "--root";
const EXPECTED_REVISION: &str = "c5991bc853d27754aed345c13ff4d2e05c40f1f4";
const EXPECTED_COHORT: &str = "prime-agent-messaging-receipts";
const EXPECTED_SCHEMA: &str = "animus.messaging-receipts.v1";
const EXPECTED_LICENSE: &str = "MIT";
const REQUIRED_SOURCE_MARKERS: [&str; 12] = [
    "pub struct FamilyRoster",
    "pub enum DeliveryMode",
    "pub fn route_message",
    "pub fn check_message_limits",
    "pub enum DeliveryVariant",
    "Delivered",
    "Queued",
    "SpoofedSenderIdentity",
    "CrossFamilyTarget",
    "LimitExceeded",
    "ReceiptContainsContent",
    "MESSAGING_NON_CLAIM",
];
const REQUIRED_POLICY_MARKERS: [&str; 9] = [
    "root",
    "subagent",
    "auto",
    "steer",
    "follow_up",
    "not-read-or-acted",
    "not-exactly-once-or-ordered",
    "host_derived_identity",
    "limits_before_routing",
];
const FORBIDDEN_SOURCE_MARKERS: [&str; 7] = [
    "std::fs",
    "std::process",
    "std::net",
    "std::env",
    "TcpStream",
    "Command::new",
    "persisted queue",
];

fn main() {
    let root = parse_root();
    let source = read(&root.join("crates/animus-adapters/src/messaging.rs"));
    for marker in REQUIRED_SOURCE_MARKERS {
        assert!(source.contains(marker), "missing messaging source marker: {marker}");
    }
    for marker in FORBIDDEN_SOURCE_MARKERS {
        assert!(
            !source.contains(marker),
            "messaging reusable source contains forbidden authority marker: {marker}"
        );
    }
    let policy = read(&root.join("config/generated/messaging-receipts-policy.json"));
    assert!(policy.contains(EXPECTED_SCHEMA));
    for marker in REQUIRED_POLICY_MARKERS {
        assert!(policy.contains(marker), "missing messaging policy marker: {marker}");
    }
    let fixture = read(&root.join("fixtures/messaging-receipts/prime-agent-c5991bc.md"));
    for marker in [EXPECTED_REVISION, EXPECTED_COHORT, EXPECTED_LICENSE, "agent_message", "delivered", "queued"] {
        assert!(fixture.contains(marker), "missing messaging fixture marker: {marker}");
    }
    let adapters_manifest = read(&root.join("crates/animus-adapters/Cargo.toml"));
    assert!(!adapters_manifest.contains("animus-std"));
    let receipt_dir = root.join("target/cairn/focused-rails");
    fs::create_dir_all(&receipt_dir).expect("focused rail receipt directory");
    fs::write(
        receipt_dir.join("messaging-receipts.md"),
        "schema: animus.focused-rail.v1\nrail: messaging-receipts\nstatus: pass\npositive_fixtures: auto idle delivery, auto busy steer, explicit follow_up queue, delivered and queued attestations, family broadcast, admitted limits\nnegative_fixtures: spoofed sender, unknown and cross-family target, denied mode, size/rate/pending limits, missing attestation, receipt body and overclaim\nnon_claims: transport security, exactly-once delivery, ordering, message correctness, comprehension, agent cooperation, production readiness, release eligibility\n",
    )
    .expect("focused rail receipt");
    println!("messaging source, cohort, policy, and boundary fixtures: pass");
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
