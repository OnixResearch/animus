use std::net::TcpListener;

use animus_std::sandlock_host::{
    SandlockCancellation, SandlockCapability, SandlockExecutionStatus, SandlockHostError,
    SandlockHostExecutionRequest, SandlockLaunchRequest, SandlockMechanismProbe,
    execute_sandlock_host, plan_sandlock_launch,
};

use super::fixture::{
    CREATED_FILE, DEFAULT_PROCESS_LIMIT, DENIAL_PROCESS_LIMIT, FIXTURE_BINARY, FORBIDDEN_FILE,
    LiveFixture, LiveRoot,
};

pub(crate) async fn run_all() {
    let live_root = LiveRoot::new();

    run_mode(
        &live_root,
        &[FIXTURE_BINARY, "filesystem-denied", FORBIDDEN_FILE],
        DEFAULT_PROCESS_LIMIT,
    )
    .await;

    let listener = TcpListener::bind(("127.0.0.1", 0)).expect("loopback listener should bind");
    let address = listener
        .local_addr()
        .expect("loopback listener should expose an address")
        .to_string();
    run_mode(
        &live_root,
        &[FIXTURE_BINARY, "network-denied", address.as_str()],
        DEFAULT_PROCESS_LIMIT,
    )
    .await;

    run_mode(
        &live_root,
        &[FIXTURE_BINARY, "environment-clean"],
        DEFAULT_PROCESS_LIMIT,
    )
    .await;

    run_mode(
        &live_root,
        &[FIXTURE_BINARY, "cow-discard"],
        DEFAULT_PROCESS_LIMIT,
    )
    .await;
    assert!(!live_root.workspace.join(CREATED_FILE).exists());

    run_mode(
        &live_root,
        &[FIXTURE_BINARY, "process-denied"],
        DENIAL_PROCESS_LIMIT,
    )
    .await;
}

async fn run_mode(live_root: &LiveRoot, command: &[&str], process_limit: u32) {
    let fixture = LiveFixture::new(live_root, command, process_limit);
    let plan = plan_sandlock_launch(SandlockLaunchRequest {
        descriptor: &fixture.descriptor,
        policy: &fixture.policy,
        authorization: &fixture.authorization,
        action_hash: &fixture.action_hash,
        subject_hash: &fixture.subject_hash,
        target_hash: &fixture.target_hash,
        command_hash: &fixture.command_hash,
        root_bindings: fixture.bindings.as_slice(),
    })
    .expect("live fixture should produce an admitted plan");
    let probe_listener =
        TcpListener::bind(("127.0.0.1", 0)).expect("probe loopback listener should bind");
    let probe_address = probe_listener
        .local_addr()
        .expect("probe listener should expose an address")
        .to_string();
    let pass_probe = [FIXTURE_BINARY, "pass"];
    let filesystem_probe = [FIXTURE_BINARY, "filesystem-denied", FORBIDDEN_FILE];
    let network_probe = [FIXTURE_BINARY, "network-denied", probe_address.as_str()];
    let cow_probe = [FIXTURE_BINARY, "cow-discard"];
    let process_limit_text = process_limit.to_string();
    let resource_probe = [
        FIXTURE_BINARY,
        "process-denied",
        process_limit_text.as_str(),
    ];
    let environment_probe = [FIXTURE_BINARY, "environment-clean"];
    let probes = [
        SandlockMechanismProbe::new(SandlockCapability::Filesystem, &filesystem_probe),
        SandlockMechanismProbe::new(SandlockCapability::SyscallFilter, &pass_probe),
        SandlockMechanismProbe::new(SandlockCapability::Supervisor, &pass_probe),
        SandlockMechanismProbe::new(SandlockCapability::Cow, &cow_probe),
        SandlockMechanismProbe::new(SandlockCapability::Resources, &resource_probe),
        SandlockMechanismProbe::new(SandlockCapability::Network, &network_probe),
        SandlockMechanismProbe::new(SandlockCapability::Environment, &environment_probe),
    ];
    let cancellation = SandlockCancellation::new();
    let missing_probe_error = execute_sandlock_host(SandlockHostExecutionRequest {
        plan: &plan,
        root_grants: fixture.grants.as_slice(),
        mechanism_probes: &probes[..probes.len().saturating_sub(1)],
        command,
        cancellation: &cancellation,
    })
    .await
    .expect_err("missing mechanism probe must block before target launch");
    assert_eq!(missing_probe_error, SandlockHostError::MissingCapability);

    let result = execute_sandlock_host(SandlockHostExecutionRequest {
        plan: &plan,
        root_grants: fixture.grants.as_slice(),
        mechanism_probes: &probes,
        command,
        cancellation: &cancellation,
    })
    .await
    .expect("qualified live Sandlock mechanism should pass");

    assert_eq!(result.facts.status, SandlockExecutionStatus::Passed);
    assert_eq!(result.receipt.status(), SandlockExecutionStatus::Passed);
}
