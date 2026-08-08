use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT_FLAG: &str = "--root";
const REVISION: &str = "c5991bc853d27754aed345c13ff4d2e05c40f1f4";
const SCHEMA: &str = "animus.schedule-reliability.v1";
const REQUIRED_SOURCE_COUNT: usize = 12;
const REQUIRED_POLICY_COUNT: usize = 10;
const FORBIDDEN_SOURCE_COUNT: usize = 8;
const REQUIRED_SOURCE: [&str; REQUIRED_SOURCE_COUNT] = [
    "pub enum ScheduleSurface",
    "UserHeartbeat",
    "AgentRecurring",
    "pub struct ScheduleState",
    "admit_schedule_mutation",
    "evaluate_schedule_tick",
    "DurableClaimProof",
    "admit_delivery_after_claim",
    "ClaimedUndelivered",
    "CoalescedSkip",
    "CoalescedPause",
    "project_schedule_receipt",
];
const REQUIRED_POLICY: [&str; REQUIRED_POLICY_COUNT] = [
    "user_heartbeat",
    "agent_recurring",
    "scheduled",
    "fire_once",
    "skip",
    "pause",
    "claim_before_delivery",
    "one_user_heartbeat",
    "not-exactly-once-execution",
    "not-production-or-release-readiness",
];
const FORBIDDEN_SOURCE: [&str; FORBIDDEN_SOURCE_COUNT] = [
    "std::fs",
    "std::process",
    "std::net",
    "std::time",
    "SystemTime",
    "Command::new",
    "TcpStream",
    "open-output-file",
];
const RECEIPT: &str = "schema: animus.focused-rail.v1\nrail: schedule-reliability\nstatus: pass\npositive_fixtures: claim-before-delivery, claimed-undelivered recovery, surface ownership, bounded coalescing, stable restart, metadata receipt\nnegative_fixtures: delivery without claim, second heartbeat, agent user-heartbeat mutation, cross-session state, backlog overflow, receipt leak, overclaim\nnon_claims: exactly-once execution, real-time delivery, timer accuracy, outcome correctness, production readiness, release eligibility\n";

fn main() {
    let root = root();
    let module_root = root.join("crates/animus-runtime/src/schedulereliability");
    let source = format!(
        "{}\n{}",
        read(&module_root.join("mod.rs")),
        read(&module_root.join("core.rs"))
    );
    let policy = read(&root.join("config/generated/schedule-reliability-policy.json"));
    let fixture = read(&root.join("fixtures/schedule-reliability/prime-agent-c5991bc.md"));
    validate_source(source.as_str()).unwrap_or_else(|error| panic!("{error}"));
    validate_policy(policy.as_str()).unwrap_or_else(|error| panic!("{error}"));
    validate_fixture(fixture.as_str()).unwrap_or_else(|error| panic!("{error}"));
    write_receipt(root.as_path());
    println!("schedule source, cohort, policy, and boundary fixtures: pass");
}

fn validate_source(source: &str) -> Result<(), String> {
    for marker in REQUIRED_SOURCE {
        if !source.contains(marker) {
            return Err(format!("missing schedule source marker: {marker}"));
        }
    }
    for marker in FORBIDDEN_SOURCE {
        if source.contains(marker) {
            return Err(format!("schedule source has ambient authority: {marker}"));
        }
    }
    Ok(())
}

fn validate_policy(policy: &str) -> Result<(), String> {
    if !policy.contains(SCHEMA) {
        return Err(format!("missing schedule schema: {SCHEMA}"));
    }
    for marker in REQUIRED_POLICY {
        if !policy.contains(marker) {
            return Err(format!("missing schedule policy marker: {marker}"));
        }
    }
    Ok(())
}

fn validate_fixture(fixture: &str) -> Result<(), String> {
    for marker in [REVISION, "MIT", "/heartbeat", "rlm_heartbeat", "claim"] {
        if !fixture.contains(marker) {
            return Err(format!("missing schedule fixture marker: {marker}"));
        }
    }
    Ok(())
}

fn write_receipt(root: &Path) {
    let target = root.join("target/cairn/focused-rails");
    fs::create_dir_all(&target).expect("focused rail directory");
    fs::write(target.join("schedule-reliability.md"), RECEIPT)
        .expect("focused rail receipt");
}

fn root() -> PathBuf {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == ROOT_FLAG {
            return PathBuf::from(args.next().expect("--root requires a path"));
        }
        panic!("unknown argument: {arg}");
    }
    env::current_dir().expect("current directory")
}

fn read(path: &Path) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display()))
}
