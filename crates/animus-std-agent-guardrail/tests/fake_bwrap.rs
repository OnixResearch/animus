#![cfg(target_os = "linux")]
//! Linux fake-Bubblewrap sealed transport and CLI integration tests.

use std::ffi::OsString;

use animus_std_agent_guardrail::{
    ActiveContext, AuthorizationBinding, BUBBLEWRAP_COHORT, EnvironmentBinding, ExecutableFacts,
    ExitClass, NetworkPosture, PreflightRequest, RootBinding, SourceKind, build_plan, launch,
    load_generated_profile, observe_executable,
};

const GENERATED: &str = include_str!("../../../config/generated/agent-guardrail-profile.json");

fn test_lock() -> std::sync::MutexGuard<'static, ()> {
    static LOCK: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
    LOCK.get_or_init(|| std::sync::Mutex::new(()))
        .lock()
        .expect("test lock")
}

fn request(
    profile: &animus_std_agent_guardrail::GuardrailProfile,
    executable: ExecutableFacts,
    root: &std::path::Path,
) -> PreflightRequest {
    let authorization = AuthorizationBinding {
        allowed: true,
        identity: String::from("authorization-id"),
        action_identity: String::from("action-id"),
        subject_identity: String::from("subject-id"),
        target_identity: String::from("target-id"),
        descriptor_identity: String::from("descriptor-id"),
        policy_identity: String::from("policy-id"),
        profile_identity: String::from(profile.identity()),
    };
    PreflightRequest {
        context: ActiveContext {
            action_identity: authorization.action_identity.clone(),
            subject_identity: authorization.subject_identity.clone(),
            target_identity: authorization.target_identity.clone(),
            descriptor_identity: authorization.descriptor_identity.clone(),
            policy_identity: authorization.policy_identity.clone(),
            context_identity: String::from("fixture-context"),
            requested_capabilities: vec![String::from("filesystem-read")],
            authorization,
        },
        roots: vec![
            RootBinding {
                label: String::from("tool-store"),
                source_root: root.to_path_buf(),
                source: root.join("store"),
                target: "/nix/store".into(),
                source_kind: SourceKind::Directory,
                exists: true,
                symlink_within_root: true,
            },
            RootBinding {
                label: String::from("worktree"),
                source_root: root.to_path_buf(),
                source: root.join("worktree"),
                target: "/workspace".into(),
                source_kind: SourceKind::Directory,
                exists: true,
                symlink_within_root: true,
            },
        ],
        environment: vec![EnvironmentBinding {
            key: String::from("TERM"),
            value: String::from("xterm"),
        }],
        executable,
        expected_plan_identity: None,
    }
}

#[test]
fn fake_bubblewrap_consumes_sealed_options_and_classifies_child_outcomes() {
    let _lock = test_lock();
    let fake = std::path::PathBuf::from(env!("CARGO_BIN_EXE_animus-fake-bwrap"));
    let executable = observe_executable(fake.as_path()).expect("fake cohort");
    assert_eq!(executable.cohort, BUBBLEWRAP_COHORT);
    let profile = load_generated_profile(GENERATED).expect("profile");
    assert_eq!(profile.network(), NetworkPosture::Deny);
    let root = tempfile::tempdir().expect("root");
    std::fs::create_dir(root.path().join("store")).expect("store");
    std::fs::create_dir(root.path().join("worktree")).expect("worktree");
    let request = request(&profile, executable, root.path());
    let plan = build_plan(&profile, &request).expect("plan");

    let passed = launch(
        &request.executable,
        &plan,
        &[OsString::from("fixture-success")],
    )
    .expect("success launch");
    assert_eq!(passed.exit, ExitClass::Success);

    let failed = launch(
        &request.executable,
        &plan,
        &[OsString::from("fixture-failure")],
    )
    .expect("failed child is observed");
    assert_eq!(failed.exit, ExitClass::NonZero);

    let signaled = launch(
        &request.executable,
        &plan,
        &[OsString::from("fixture-signal")],
    )
    .expect("signaled child is observed");
    assert_eq!(signaled.exit, ExitClass::Signal);
}

#[test]
fn cli_describe_validate_and_exec_are_bounded_and_redacted() {
    let _lock = test_lock();
    let fake = std::path::PathBuf::from(env!("CARGO_BIN_EXE_animus-fake-bwrap"));
    let cli = std::path::PathBuf::from(env!("CARGO_BIN_EXE_animus-agent-guardrail"));
    let executable = observe_executable(fake.as_path()).expect("fake cohort");
    let profile = load_generated_profile(GENERATED).expect("profile");
    let root = tempfile::tempdir().expect("root");
    std::fs::create_dir(root.path().join("store")).expect("store");
    std::fs::create_dir(root.path().join("worktree")).expect("worktree");
    let request = request(&profile, executable, root.path());
    let profile_path = root.path().join("profile.json");
    let request_path = root.path().join("request.json");
    std::fs::write(profile_path.as_path(), GENERATED).expect("profile file");
    std::fs::write(
        request_path.as_path(),
        serde_json::to_vec(&request).expect("request JSON"),
    )
    .expect("request file");

    let description = std::process::Command::new(cli.as_path())
        .args(["describe", "--profile"])
        .arg(profile_path.as_path())
        .output()
        .expect("describe");
    assert!(description.status.success());
    assert!(
        !String::from_utf8_lossy(description.stdout.as_slice())
            .contains(root.path().to_string_lossy().as_ref())
    );

    let validation = std::process::Command::new(cli.as_path())
        .args(["validate", "--profile"])
        .arg(profile_path.as_path())
        .arg("--request")
        .arg(request_path.as_path())
        .arg("--bwrap")
        .arg(fake.as_path())
        .output()
        .expect("validate");
    assert!(validation.status.success());
    let validation_text = String::from_utf8(validation.stdout).expect("validation UTF-8");
    assert!(validation_text.contains("\"mutated\":false"));
    assert!(validation_text.contains("\"child_launched\":false"));

    let execution = std::process::Command::new(cli.as_path())
        .args(["exec", "--profile"])
        .arg(profile_path.as_path())
        .arg("--request")
        .arg(request_path.as_path())
        .arg("--bwrap")
        .arg(fake.as_path())
        .args(["--", "fixture-success"])
        .output()
        .expect("exec");
    assert!(execution.status.success());
    let receipt = String::from_utf8(execution.stdout).expect("receipt UTF-8");
    assert!(receipt.contains("animus.agent-guardrail.receipt.v1"));
    assert!(!receipt.contains(root.path().to_string_lossy().as_ref()));
    assert!(!receipt.contains("xterm"));
}

#[test]
fn missing_incompatible_and_empty_launch_inputs_fail_without_fallback() {
    let _lock = test_lock();
    assert!(observe_executable(std::path::Path::new("/missing/bwrap")).is_err());
    let current = std::env::current_exe().expect("current executable");
    assert!(observe_executable(current.as_path()).is_err());

    let fake = std::path::PathBuf::from(env!("CARGO_BIN_EXE_animus-fake-bwrap"));
    let executable = observe_executable(fake.as_path()).expect("fake cohort");
    let profile = load_generated_profile(GENERATED).expect("profile");
    let root = tempfile::tempdir().expect("root");
    std::fs::create_dir(root.path().join("store")).expect("store");
    std::fs::create_dir(root.path().join("worktree")).expect("worktree");
    let request = request(&profile, executable, root.path());
    let plan = build_plan(&profile, &request).expect("plan");
    assert!(launch(&request.executable, &plan, &[]).is_err());
}
