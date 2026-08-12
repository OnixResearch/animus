use std::{
    env, fs,
    path::{Path, PathBuf},
    process,
};

const SUCCESS: i32 = 0;
const FAILURE: i32 = 1;
const EXPECTED_ARGUMENTS: usize = 2;

fn main() {
    let arguments = env::args().collect::<Vec<_>>();
    if arguments.len() != EXPECTED_ARGUMENTS {
        eprintln!("usage: check-evaluation-composition-conformance <root>");
        process::exit(FAILURE);
    }
    let root = PathBuf::from(&arguments[1]);
    match validate_repository(&root) {
        Ok(()) => {
            println!("evaluation composition conformance static check: pass");
            process::exit(SUCCESS);
        }
        Err(error) => {
            eprintln!("evaluation composition conformance static check: {error}");
            process::exit(FAILURE);
        }
    }
}

fn validate_repository(root: &Path) -> Result<(), String> {
    let crate_root = read(root, "crates/animus-eval-conformance/src/lib.rs")?;
    let profile = read(root, "crates/animus-eval-conformance/src/profile.rs")?;
    let harness = read(root, "crates/animus-eval-conformance/src/harness.rs")?;
    let host_fixture = read(root, "crates/animus-eval-conformance/src/host_fixture.rs")?;
    let manifest = read(root, "crates/animus-eval-conformance/Cargo.toml")?;
    let nickel = read(root, "config/evaluation-composition-conformance.ncl")?;
    let generated = read(
        root,
        "config/generated/evaluation-composition-conformance.json",
    )?;
    let positive = read(
        root,
        "fixtures/evaluation-composition-conformance/positive.ncl",
    )?;
    let negative = read(
        root,
        "fixtures/evaluation-composition-conformance/negative.ncl",
    )?;

    validate_source_shapes(&crate_root, &profile, &harness, &host_fixture, &manifest)?;
    require(
        &nickel,
        "animus.native-evaluation-conformance.v1",
        "Nickel profile",
    )?;
    require(
        &generated,
        "native-evaluation-composition-v1",
        "generated profile",
    )?;
    validate_fixture_inventory(&positive, &negative)?;
    for path in [
        "docs/evaluation-composition-conformance.md",
        ".cairn/component-catalog.ncl",
        ".cairn/runtime-facade-inventory.ncl",
        "docs/contracts-index.md",
        "docs/focused-verification-rails.md",
        "docs/request-lifecycle.md",
        "docs/safety-contract.md",
        "README.md",
    ] {
        if read(root, path)?.trim().is_empty() {
            return Err(format!("{path} is empty"));
        }
    }
    Ok(())
}

fn validate_source_shapes(
    crate_root: &str,
    profile: &str,
    harness: &str,
    host_fixture: &str,
    manifest: &str,
) -> Result<(), String> {
    for required in [
        "#![forbid(unsafe_code)]",
        "pub const CONFORMANCE_SCHEMA",
        "pub fn hash_parts",
    ] {
        require(crate_root, required, "crate root")?;
    }
    for required in [
        "pub fn parse_conformance_profile",
        "pub fn validate_conformance_profile",
        "#[serde(deny_unknown_fields)]",
        "animus.native-evaluation-conformance.family.v1",
        "animus.native-evaluation-conformance.fixture.v1",
        "animus.native-evaluation-conformance.profile.v1",
    ] {
        require(profile, required, "profile validation")?;
    }
    for entrypoint in [
        "plan_taskset",
        "plan_environment",
        "plan_recipe",
        "reduce_pass_at_n",
        "reduce_episode",
        "reduce_reward_dimensions",
        "build_resume_key",
        "evaluate_resume",
        "project_evaluation_receipt",
        "admit_service",
        "build_service_request",
        "project_model_call",
        "admit_terminal_after_commit",
        "admit_identical_request_coalescing",
        "project_atif",
        "reduce_rewards",
        "validate_eval_feedback_plan",
        "evaluate_feedback_review",
        "export_chat_messages_row",
    ] {
        require(harness, entrypoint, "public production-path harness")?;
    }
    for entrypoint in [
        "execute_evaluation_host",
        "preflight_host",
        "project_host_receipt",
    ] {
        require(host_fixture, entrypoint, "public host fixture")?;
    }
    for forbidden in [
        "fn plan_taskset(",
        "fn plan_recipe(",
        "fn reduce_episode(",
        "fn reduce_reward_dimensions(",
        "fn evaluate_resume(",
        "fn project_model_call(",
        "fn project_host_receipt(",
        "::tests::",
        "mock_call_count",
    ] {
        if harness.contains(forbidden) || host_fixture.contains(forbidden) {
            return Err(format!(
                "test-only implementation or private path found: {forbidden}"
            ));
        }
    }
    for required_dependency in [
        "animus-eval-core",
        "animus-adapter-eval",
        "animus-std-eval",
        "animus-runtime",
        "default-features = false",
    ] {
        require(manifest, required_dependency, "conformance manifest")?;
    }
    for forbidden_dependency in [
        "tokio",
        "reqwest",
        "hyper",
        "python",
        "docker",
        "kubernetes",
        "verifiers",
    ] {
        if manifest.to_ascii_lowercase().contains(forbidden_dependency) {
            return Err(format!("live dependency found: {forbidden_dependency}"));
        }
    }
    Ok(())
}

fn validate_fixture_inventory(positive: &str, negative: &str) -> Result<(), String> {
    for family in [
        "taskset",
        "recipes",
        "episode",
        "ports",
        "gateway",
        "verifier",
        "rewards",
        "persistence",
        "compatibility",
        "downstream",
        "evidence",
    ] {
        require(
            positive,
            &format!("family = \"{family}\""),
            "positive fixtures",
        )?;
        require(
            negative,
            &format!("family = \"{family}\""),
            "negative fixtures",
        )?;
    }
    for hazard in [
        "unstable-generator",
        "missing-bound",
        "unsafe-shuffle",
        "duplicate-task",
        "role-widening",
        "fanout-overflow",
        "shared-judge-promotion",
        "copied-transcript",
        "stale-ledger",
        "wrong-subject",
        "missing-service",
        "wrong-plan",
        "wrong-task",
        "wrong-attempt",
        "denied-capability",
        "malformed-observation",
        "early-terminal",
        "changed-retry",
        "dispatch-before-acceptance",
        "late-write",
        "public-bind",
        "wildcard-network",
        "response-rewrite",
        "over-budget",
        "shared-verifier-mutation",
        "symlink",
        "artifact-collision",
        "hidden-test-exposure",
        "nan",
        "infinity",
        "duplicate-reward",
        "identity-mismatch",
        "silent-clamp",
        "torn-output",
        "stale-resume",
        "failed-cleanup",
        "lossy-harbor",
        "unsupported-atif",
        "automatic-feedback-promotion",
        "unsafe-dataset-export",
        "release-overclaim",
    ] {
        require(negative, hazard, "negative fixtures")?;
    }
    for required in [
        "requires_no_prohibited_call = true",
        "requires_unchanged_state = true",
        "permits_success_receipt = false",
    ] {
        require(negative, required, "negative invariants")?;
    }
    Ok(())
}

fn read(root: &Path, relative: &str) -> Result<String, String> {
    fs::read_to_string(root.join(relative)).map_err(|error| format!("{relative}: {error}"))
}

fn require(content: &str, needle: &str, label: &str) -> Result<(), String> {
    if content.contains(needle) {
        Ok(())
    } else {
        Err(format!("{label} is missing {needle}"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn complete_source_shape_uses_public_production_paths() {
        let manifest = "animus-eval-core animus-adapter-eval animus-std-eval animus-runtime default-features = false";
        validate_source_shapes(
            "#![forbid(unsafe_code)] pub const CONFORMANCE_SCHEMA: &str = \"x\"; pub fn hash_parts() {}",
            "pub fn parse_conformance_profile() {} pub fn validate_conformance_profile() {} #[serde(deny_unknown_fields)] animus.native-evaluation-conformance.family.v1 animus.native-evaluation-conformance.fixture.v1 animus.native-evaluation-conformance.profile.v1",
            &required_harness(),
            "execute_evaluation_host preflight_host project_host_receipt",
            manifest,
        )
        .expect("complete source shape must pass");
    }

    #[test]
    fn duplicate_test_implementation_and_missing_hazard_fail() {
        let manifest = "animus-eval-core animus-adapter-eval animus-std-eval animus-runtime default-features = false";
        let mut harness = required_harness();
        harness.push_str(" fn plan_taskset(");
        assert!(validate_source_shapes(
            "#![forbid(unsafe_code)] pub const CONFORMANCE_SCHEMA: &str = \"x\"; pub fn hash_parts() {}",
            "pub fn parse_conformance_profile() {} pub fn validate_conformance_profile() {} #[serde(deny_unknown_fields)] animus.native-evaluation-conformance.family.v1 animus.native-evaluation-conformance.fixture.v1 animus.native-evaluation-conformance.profile.v1",
            &harness,
            "execute_evaluation_host preflight_host project_host_receipt",
            manifest,
        )
        .is_err());
        assert!(
            validate_fixture_inventory("family = \"taskset\"", "family = \"taskset\"",).is_err()
        );
    }

    fn required_harness() -> String {
        [
            "plan_taskset",
            "plan_environment",
            "plan_recipe",
            "reduce_pass_at_n",
            "reduce_episode",
            "reduce_reward_dimensions",
            "build_resume_key",
            "evaluate_resume",
            "project_evaluation_receipt",
            "admit_service",
            "build_service_request",
            "project_model_call",
            "admit_terminal_after_commit",
            "admit_identical_request_coalescing",
            "project_atif",
            "reduce_rewards",
            "validate_eval_feedback_plan",
            "evaluate_feedback_review",
            "export_chat_messages_row",
        ]
        .join(" ")
    }
}
