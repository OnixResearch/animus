//! Positive and negative process-boundary fixtures for the optional Lean-loop host.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use animus_runtime::LoopFabric;
use animus_std_lean_loop::{
    LeanLaunchDescriptor, LeanLoopFabric, LeanLoopFailureClass, LeanSandboxObservation,
    artifact_identity, load_generated_profile,
};

const GENERATED_PROFILE: &str = include_str!("../../../config/generated/lean-loop-profile.json");
const MODEL_MESSAGE: &str = "model answer";
const BAD_ARTIFACT: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";

fn descriptor(
    expected_artifact_identity: String,
    sandbox_observation: LeanSandboxObservation,
) -> LeanLaunchDescriptor {
    LeanLaunchDescriptor {
        executable: std::path::PathBuf::from(env!("CARGO_BIN_EXE_animus-lean-fixture-child")),
        expected_artifact_identity,
        working_directory: std::env::current_dir().expect("current directory"),
        sandbox_observation,
        cancellation: Arc::new(AtomicBool::new(false)),
    }
}

#[test]
fn fixture_child_drives_model_response_tool_stop_and_shutdown() {
    let profile = load_generated_profile(GENERATED_PROFILE).expect("profile");
    let executable = std::path::Path::new(env!("CARGO_BIN_EXE_animus-lean-fixture-child"));
    let artifact = artifact_identity(executable).expect("artifact identity");
    let mut model_fabric = LeanLoopFabric::launch(
        profile.clone(),
        descriptor(artifact.clone(), LeanSandboxObservation::FixtureOnly),
    )
    .expect("launch model fabric");
    let initial = animus_runtime::RuntimeLoopTurn::new("build reliable systems", None, None)
        .expect("initial turn");
    assert!(matches!(
        model_fabric.decide(&initial).expect("model decision"),
        animus_runtime::RuntimeLoopDecision::CallModel { .. }
    ));
    let result =
        animus_runtime::RuntimeLoopTurn::new("build reliable systems", Some(MODEL_MESSAGE), None)
            .expect("result turn");
    assert!(matches!(
        model_fabric.decide(&result).expect("response decision"),
        animus_runtime::RuntimeLoopDecision::Respond { .. }
    ));
    assert!(model_fabric.shutdown().is_ok());

    let mut tool_fabric = LeanLoopFabric::launch(
        profile.clone(),
        descriptor(artifact.clone(), LeanSandboxObservation::FixtureOnly),
    )
    .expect("launch tool fabric");
    let tool_turn =
        animus_runtime::RuntimeLoopTurn::new("fixture.tool", None, None).expect("tool turn");
    assert!(matches!(
        tool_fabric.decide(&tool_turn).expect("tool decision"),
        animus_runtime::RuntimeLoopDecision::CallTool { .. }
    ));
    assert!(tool_fabric.shutdown().is_ok());

    let mut stop_fabric = LeanLoopFabric::launch(
        profile,
        descriptor(artifact, LeanSandboxObservation::FixtureOnly),
    )
    .expect("launch stop fabric");
    let stop_turn =
        animus_runtime::RuntimeLoopTurn::new("fixture.stop", None, None).expect("stop turn");
    assert!(matches!(
        stop_fabric.decide(&stop_turn).expect("stop decision"),
        animus_runtime::RuntimeLoopDecision::Stop
    ));
    assert!(stop_fabric.shutdown().is_ok());
}

#[test]
fn artifact_sandbox_and_spawn_failures_stop_before_activation() {
    let profile = load_generated_profile(GENERATED_PROFILE).expect("profile");
    let artifact_error = LeanLoopFabric::launch(
        profile.clone(),
        descriptor(
            String::from(BAD_ARTIFACT),
            LeanSandboxObservation::FixtureOnly,
        ),
    )
    .err()
    .expect("artifact drift denied");
    assert_eq!(
        artifact_error.failure(),
        LeanLoopFailureClass::ArtifactIdentityMismatch
    );

    let executable = std::path::Path::new(env!("CARGO_BIN_EXE_animus-lean-fixture-child"));
    let artifact = artifact_identity(executable).expect("artifact identity");
    let sandbox_error = LeanLoopFabric::launch(
        profile.clone(),
        descriptor(artifact, LeanSandboxObservation::Missing),
    )
    .err()
    .expect("missing sandbox evidence denied");
    assert_eq!(
        sandbox_error.failure(),
        LeanLoopFailureClass::SandboxEvidenceMissing
    );

    let mut missing = descriptor(
        String::from(BAD_ARTIFACT),
        LeanSandboxObservation::FixtureOnly,
    );
    missing.executable = std::path::PathBuf::from("/definitely/missing/animus-lean");
    assert!(LeanLoopFabric::launch(profile, missing).is_err());
}
