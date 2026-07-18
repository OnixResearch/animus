#![cfg(target_os = "linux")]
//! Opt-in real Bubblewrap behavior evidence for the exact checked profile and host prerequisites.

use std::ffi::OsString;

use animus_std_agent_guardrail::{
    ActiveContext, AuthorizationBinding, EnvironmentBinding, ExitClass, PreflightRequest,
    RootBinding, SourceKind, build_plan, launch, load_generated_profile, observe_executable,
};

const GENERATED: &str = include_str!("../../../config/generated/agent-guardrail-profile.json");
const BWRAP_ENVIRONMENT: &str = "ANIMUS_GUARDRAIL_BWRAP";

fn request(
    profile: &animus_std_agent_guardrail::GuardrailProfile,
    executable: animus_std_agent_guardrail::ExecutableFacts,
    root: &std::path::Path,
    fixture: &std::path::Path,
) -> PreflightRequest {
    let authorization = AuthorizationBinding {
        allowed: true,
        identity: String::from("live-authorization"),
        action_identity: String::from("live-action"),
        subject_identity: String::from("live-subject"),
        target_identity: String::from("live-target"),
        descriptor_identity: String::from("live-descriptor"),
        policy_identity: String::from("live-policy"),
        profile_identity: String::from(profile.identity()),
    };
    PreflightRequest {
        context: ActiveContext {
            action_identity: authorization.action_identity.clone(),
            subject_identity: authorization.subject_identity.clone(),
            target_identity: authorization.target_identity.clone(),
            descriptor_identity: authorization.descriptor_identity.clone(),
            policy_identity: authorization.policy_identity.clone(),
            context_identity: String::from("live-context"),
            requested_capabilities: vec![String::from("filesystem-read")],
            authorization,
        },
        roots: vec![
            RootBinding {
                label: String::from("tool-store"),
                source_root: "/nix".into(),
                source: "/nix/store".into(),
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
            RootBinding {
                label: String::from("guardrail-fixture"),
                source_root: fixture.parent().expect("fixture parent").to_path_buf(),
                source: fixture.to_path_buf(),
                target: "/guardrail-fixture".into(),
                source_kind: SourceKind::File,
                exists: true,
                symlink_within_root: true,
            },
        ],
        environment: vec![EnvironmentBinding {
            key: String::from("TERM"),
            value: String::from("dumb"),
        }],
        executable,
        expected_plan_identity: None,
    }
}

fn run(
    executable: &animus_std_agent_guardrail::ExecutableFacts,
    plan: &animus_std_agent_guardrail::GuardrailPlan,
    operation: &str,
    argument: &str,
) -> ExitClass {
    launch(
        executable,
        plan,
        &[
            OsString::from("/guardrail-fixture"),
            OsString::from(operation),
            OsString::from(argument),
        ],
    )
    .expect("guarded launch")
    .exit
}

#[test]
#[ignore = "requires explicit ANIMUS_GUARDRAIL_BWRAP and working user namespaces"]
fn real_profile_allows_worktree_and_denies_unmounted_host_and_network_access() {
    let bwrap = std::env::var_os(BWRAP_ENVIRONMENT).expect("explicit Bubblewrap path");
    let executable = observe_executable(std::path::Path::new(&bwrap)).expect("exact cohort");
    let fixture = std::path::PathBuf::from(env!("CARGO_BIN_EXE_animus-guardrail-live-fixture"));
    let root = tempfile::tempdir().expect("live root");
    let worktree = root.path().join("worktree");
    let secret = root.path().join("credential.txt");
    let sibling = root.path().join("sibling-repository.txt");
    std::fs::create_dir(worktree.as_path()).expect("worktree");
    std::fs::write(secret.as_path(), b"private").expect("secret fixture");
    std::fs::write(sibling.as_path(), b"sibling").expect("sibling fixture");
    std::os::unix::fs::symlink(secret.as_path(), worktree.join("escape")).expect("escape symlink");

    let profile = load_generated_profile(GENERATED).expect("profile");
    let request = request(&profile, executable, root.path(), fixture.as_path());
    let plan = build_plan(&profile, &request).expect("live plan");

    assert_eq!(
        run(
            &request.executable,
            &plan,
            "write",
            "/workspace/allowed.txt",
        ),
        ExitClass::Success
    );
    assert_eq!(
        std::fs::read(worktree.join("allowed.txt")).expect("allowed write"),
        b"guarded"
    );
    for denied in [
        secret.to_string_lossy().into_owned(),
        sibling.to_string_lossy().into_owned(),
        String::from("/workspace/escape"),
    ] {
        assert_eq!(
            run(&request.executable, &plan, "read", denied.as_str(),),
            ExitClass::NonZero
        );
    }
    assert_eq!(
        run(&request.executable, &plan, "write", "/outside-write",),
        ExitClass::NonZero
    );

    let inherited = rustix::fs::memfd_create(
        "animus-guardrail-inherited-fixture",
        rustix::fs::MemfdFlags::empty(),
    )
    .expect("inherited fixture descriptor");
    assert!(
        launch(
            &request.executable,
            &plan,
            &[
                OsString::from("/guardrail-fixture"),
                OsString::from("read"),
                OsString::from("/workspace")
            ],
        )
        .is_err()
    );
    drop(inherited);

    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("listener");
    let address = listener.local_addr().expect("listener address");
    std::thread::spawn(move || {
        let _accepted = listener.accept();
    });
    assert_eq!(
        run(
            &request.executable,
            &plan,
            "connect",
            address.to_string().as_str(),
        ),
        ExitClass::NonZero
    );
}
