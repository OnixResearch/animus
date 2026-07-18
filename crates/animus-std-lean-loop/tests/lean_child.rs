//! Nix-selected execution fixture for the repository-owned compiled Lean child.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use animus_runtime::LoopFabric;
use animus_std_lean_loop::{
    LeanLaunchDescriptor, LeanLoopFabric, LeanSandboxObservation, artifact_identity,
    load_generated_profile,
};

const GENERATED_PROFILE: &str = include_str!("../../../config/generated/lean-loop-profile.json");
const LEAN_EXECUTABLE_ENV: &str = "ANIMUS_LEAN_EXECUTABLE";

#[test]
#[ignore = "requires the Nix-built repository Lean executable"]
fn repository_lean_child_runs_model_then_response_protocol() {
    let executable = std::path::PathBuf::from(
        std::env::var(LEAN_EXECUTABLE_ENV).expect("focused rail must select Lean executable"),
    );
    let profile = load_generated_profile(GENERATED_PROFILE).expect("profile");
    let artifact = artifact_identity(executable.as_path()).expect("artifact identity");
    let descriptor = LeanLaunchDescriptor {
        executable,
        expected_artifact_identity: artifact,
        working_directory: std::env::current_dir().expect("current directory"),
        sandbox_observation: LeanSandboxObservation::FixtureOnly,
        cancellation: Arc::new(AtomicBool::new(false)),
    };
    let mut fabric = LeanLoopFabric::launch(profile, descriptor).expect("launch Lean child");
    let first = animus_runtime::RuntimeLoopTurn::new("build reliable systems", None, None)
        .expect("first turn");
    assert!(matches!(
        fabric.decide(&first).expect("model decision"),
        animus_runtime::RuntimeLoopDecision::CallModel { .. }
    ));
    let second =
        animus_runtime::RuntimeLoopTurn::new("build reliable systems", Some("model answer"), None)
            .expect("second turn");
    assert!(matches!(
        fabric.decide(&second).expect("response decision"),
        animus_runtime::RuntimeLoopDecision::Respond { .. }
    ));
    assert!(fabric.shutdown().is_ok());
}
