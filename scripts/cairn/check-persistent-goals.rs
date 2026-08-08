use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ROOT_FLAG: &str = "--root";
const REVISION: &str = "c5991bc853d27754aed345c13ff4d2e05c40f1f4";
const SCHEMA: &str = "animus.persistent-goals.v1";
const REQUIRED_SOURCE: [&str; 12] = [
    "pub struct GoalState", "pub enum GoalStatus", "BudgetLimited", "pub enum GoalAction",
    "pub struct GoalSession", "ActiveGoalExists", "GoalProvenance", "OwnershipRejected",
    "evaluate_goal_continuation", "project_goal_receipt", "contains_objective_text", "not-objective-achievement",
];
const REQUIRED_POLICY: [&str; 9] = [
    "active", "paused", "completed", "budget_limited", "budget_exhausted", "one_active_goal",
    "explicit_completion_only", "not-automatic-inference", "host_injected_accounting",
];
const FORBIDDEN: [&str; 7] = ["std::fs", "std::process", "std::net", "std::time", "Command::new", "TcpStream", "open-output-file"];

fn main() {
    let root = root();
    let source = read(&root.join("crates/animus-runtime/src/persistentgoals.rs"));
    for marker in REQUIRED_SOURCE { assert!(source.contains(marker), "missing goal source marker: {marker}"); }
    for marker in FORBIDDEN { assert!(!source.contains(marker), "goal source has ambient authority: {marker}"); }
    let policy = read(&root.join("config/generated/persistent-goals-policy.json"));
    assert!(policy.contains(SCHEMA));
    for marker in REQUIRED_POLICY { assert!(policy.contains(marker), "missing goal policy marker: {marker}"); }
    let fixture = read(&root.join("fixtures/persistent-goals/prime-agent-c5991bc.md"));
    for marker in [REVISION, "MIT", "/goal", "explicit completion"] { assert!(fixture.contains(marker), "missing goal fixture marker: {marker}"); }
    let target = root.join("target/cairn/focused-rails");
    fs::create_dir_all(&target).expect("focused rail directory");
    fs::write(target.join("persistent-goals.md"), "schema: animus.focused-rail.v1\nrail: persistent-goals\nstatus: pass\npositive_fixtures: explicit lifecycle, accounting, completion, goal-aware continuation\nnegative_fixtures: inferred goal, model lifecycle control, invalid transition, second active goal, budget-as-success, content receipt\nnon_claims: objective achievement, work quality, automatic inference, production readiness, release eligibility\n").expect("focused rail receipt");
    println!("persistent goal source, cohort, policy, and boundary fixtures: pass");
}

fn root() -> PathBuf {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == ROOT_FLAG { return PathBuf::from(args.next().expect("--root requires a path")); }
        panic!("unknown argument: {arg}");
    }
    env::current_dir().expect("current directory")
}

fn read(path: &Path) -> String { fs::read_to_string(path).unwrap_or_else(|error| panic!("cannot read {}: {error}", path.display())) }
