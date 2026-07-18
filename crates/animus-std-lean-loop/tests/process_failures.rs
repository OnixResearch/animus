//! Adversarial child-process fixtures for terminal protocol failures.

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use animus_std_lean_loop::{
    LeanLaunchDescriptor, LeanLoopFabric, LeanLoopFailureClass, LeanSandboxObservation,
    artifact_identity, load_generated_profile,
};

const GENERATED_PROFILE: &str = include_str!("../../../config/generated/lean-loop-profile.json");

fn descriptor(path: &str, cancelled: bool) -> LeanLaunchDescriptor {
    let executable = std::path::PathBuf::from(path);
    LeanLaunchDescriptor {
        expected_artifact_identity: artifact_identity(executable.as_path())
            .expect("fixture artifact identity"),
        executable,
        working_directory: std::env::current_dir().expect("current directory"),
        sandbox_observation: LeanSandboxObservation::FixtureOnly,
        cancellation: Arc::new(AtomicBool::new(cancelled)),
    }
}

#[test]
fn malformed_eof_deadline_and_cancellation_fail_without_fallback() {
    let profile = load_generated_profile(GENERATED_PROFILE).expect("profile");

    let malformed = LeanLoopFabric::launch(
        profile.clone(),
        descriptor(env!("CARGO_BIN_EXE_animus-lean-malformed-child"), false),
    )
    .err()
    .expect("malformed handshake denied");
    assert_eq!(malformed.failure(), LeanLoopFailureClass::MalformedResponse);

    let eof = LeanLoopFabric::launch(
        profile.clone(),
        descriptor(env!("CARGO_BIN_EXE_animus-lean-eof-child"), false),
    )
    .err()
    .expect("early EOF denied");
    assert!(matches!(
        eof.failure(),
        LeanLoopFailureClass::InputEof | LeanLoopFailureClass::BrokenPipe
    ));

    let deadline = LeanLoopFabric::launch(
        profile.clone(),
        descriptor(env!("CARGO_BIN_EXE_animus-lean-hang-child"), false),
    )
    .err()
    .expect("handshake deadline denied");
    assert_eq!(deadline.failure(), LeanLoopFailureClass::ResponseDeadline);

    let cancellation = LeanLoopFabric::launch(
        profile,
        descriptor(env!("CARGO_BIN_EXE_animus-lean-fixture-child"), true),
    )
    .err()
    .expect("pre-handshake cancellation denied");
    assert_eq!(
        cancellation.failure(),
        LeanLoopFailureClass::CancellationAmbiguous
    );
}
