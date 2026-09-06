//! Verifies the repository-owned outcome-machine lifecycle runner.
#![cfg(test)]

const PINNED_CAIRN_REVISION: &str = "5e1fe08d62149b286ff583eb056e655fe92e3cf1";
const CHANGE_POLICY_ROUTE: &str = "agent-receipt|change|doctor";
const TRACEABILITY_POLICY_ROUTE: &str = "|traceability|tracey|";
const POLICY_ARGUMENT: &str = "--policy";
const LOCAL_POLICY_PATH: &str = "${self}/cairn-policy/generated/cairn-policy.json";

fn validate_runner(source: &str) -> Result<(), &'static str> {
    if !source.contains(PINNED_CAIRN_REVISION) {
        return Err("Cairn revision is not pinned");
    }
    if !source.contains(CHANGE_POLICY_ROUTE) {
        return Err("change commands do not receive the pinned policy");
    }
    if !source.contains(TRACEABILITY_POLICY_ROUTE) {
        return Err("traceability commands do not receive the pinned policy");
    }
    if !source.contains(POLICY_ARGUMENT) {
        return Err("the runner does not inject a policy argument");
    }
    if !source.contains(LOCAL_POLICY_PATH) {
        return Err("the runner does not use the repository policy");
    }
    Ok(())
}

fn source() -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../flake.nix");
    std::fs::read_to_string(path).expect("repository-owned flake source")
}

#[test]
fn pinned_runner_covers_profile_lifecycle_commands() {
    // r[verify animus.lifecycle.outcome_machine_profile]
    let source = source();
    assert_eq!(validate_runner(&source), Ok(()));
}

#[test]
fn moving_or_incomplete_runner_fails_admission() {
    let moving_source = "url = github:OnixResearch/cairn/main";
    assert_eq!(
        validate_runner(moving_source),
        Err("Cairn revision is not pinned")
    );
    assert_eq!(
        validate_runner(PINNED_CAIRN_REVISION),
        Err("change commands do not receive the pinned policy")
    );
    let source = source();
    for (missing, expected) in [
        (
            TRACEABILITY_POLICY_ROUTE,
            "traceability commands do not receive the pinned policy",
        ),
        (
            POLICY_ARGUMENT,
            "the runner does not inject a policy argument",
        ),
        (
            LOCAL_POLICY_PATH,
            "the runner does not use the repository policy",
        ),
    ] {
        assert_eq!(
            validate_runner(&source.replace(missing, "omitted")),
            Err(expected)
        );
    }
}
