type FixtureMode = super::test_double::FixtureMode;
type FixtureServices = super::test_double::FixtureServices;
type HandoffHost<S> = super::orchestrator::HandoffHost<S>;
type HandoffHostError = super::orchestrator::HandoffHostError;
type HandoffHostOutcome = super::orchestrator::HandoffHostOutcome;
type HandoffState = super::core::HandoffState;
type Issue = super::core::Issue;

const RECEIPT_HASH_CHARS: usize = 64;

#[test]
// r[verify agent_harness_runtime.daemon_handoff.transaction.success]
// r[verify agent_harness_runtime.daemon_handoff.verification.complete]
fn fake_success_commits_state_before_release_and_exits_after_ready() {
    let smoke = super::validation_driver::run_fake_handoff_smoke().expect("fake handoff smoke");
    assert_eq!(smoke.state, "complete");
    assert_eq!(smoke.receipt_identity.len(), RECEIPT_HASH_CHARS);
    assert!(
        smoke
            .non_claims
            .iter()
            .any(|value| value == "not-pty-migration")
    );

    let mut host = HandoffHost::new(FixtureServices::new(FixtureMode::Success));
    let outcome = host
        .run(&super::validation_driver::fixture_request())
        .expect("complete handoff");
    let HandoffHostOutcome::Complete(transaction) = outcome else {
        panic!("success fixture must complete");
    };
    assert_eq!(transaction.state, HandoffState::Complete);
    assert_eq!(
        host.services().calls,
        [
            "pause_ingress",
            "commit_handoff",
            "release_lease",
            "start_replacement",
            "authenticate_claim",
            "wait_readiness",
            "exit_old_worker",
        ]
    );
}

#[test]
// r[verify agent_harness_runtime.daemon_handoff.cancellation.timeout]
fn readiness_timeout_commits_cancel_terminates_tree_then_restores_and_resumes() {
    let mut host = HandoffHost::new(FixtureServices::new(FixtureMode::ReadinessTimeout));
    let outcome = host
        .run(&super::validation_driver::fixture_request())
        .expect("rollback after timeout");
    let HandoffHostOutcome::LeaseRestored(transaction) = outcome else {
        panic!("timeout fixture must restore");
    };
    assert_eq!(transaction.state, HandoffState::LeaseRestored);
    let calls = &host.services().calls;
    let cancel = position(calls, "commit_cancellation");
    let terminate = position(calls, "terminate_replacement");
    let observe = position(calls, "observe_termination");
    let restore = position(calls, "restore_lease");
    let resume = position(calls, "resume_ingress");
    assert!(cancel < terminate);
    assert!(terminate < observe);
    assert!(observe < restore);
    assert!(restore < resume);
}

#[test]
// r[verify agent_harness_runtime.daemon_handoff.verification.complete]
fn spawn_failure_restores_without_inventing_a_process_tree() {
    let mut host = HandoffHost::new(FixtureServices::new(FixtureMode::SpawnFailure));
    let outcome = host
        .run(&super::validation_driver::fixture_request())
        .expect("spawn rollback");
    assert!(matches!(outcome, HandoffHostOutcome::LeaseRestored(_)));
    assert!(!host.services().calls.contains(&"terminate_replacement"));
    assert!(host.services().calls.contains(&"observe_termination"));
}

#[test]
// r[verify agent_harness_runtime.daemon_handoff.claim.denied]
fn wrong_endpoint_cancels_before_any_readiness_or_exit() {
    let mut host = HandoffHost::new(FixtureServices::new(FixtureMode::WrongEndpoint));
    let outcome = host
        .run(&super::validation_driver::fixture_request())
        .expect("claim rollback");
    assert!(matches!(outcome, HandoffHostOutcome::LeaseRestored(_)));
    assert!(!host.services().calls.contains(&"wait_readiness"));
    assert!(!host.services().calls.contains(&"exit_old_worker"));
    assert!(host.services().calls.contains(&"terminate_replacement"));
}

#[test]
// r[verify agent_harness_runtime.daemon_handoff.cancellation.survivor]
fn surviving_descendant_blocks_lease_restore_and_ingress_resume() {
    let mut host = HandoffHost::new(FixtureServices::new(FixtureMode::SurvivingDescendant));
    assert_eq!(
        host.run(&super::validation_driver::fixture_request()),
        Err(HandoffHostError::Core(Issue::TeardownIncomplete))
    );
    assert!(!host.services().calls.contains(&"restore_lease"));
    assert!(!host.services().calls.contains(&"resume_ingress"));
}

#[test]
// r[verify agent_harness_runtime.daemon_handoff.session_state.drift]
fn rollback_drift_blocks_second_owner_and_ingress_resume() {
    let mut host = HandoffHost::new(FixtureServices::new(FixtureMode::RollbackDrift));
    assert_eq!(
        host.run(&super::validation_driver::fixture_request()),
        Err(HandoffHostError::Core(Issue::RollbackDrift))
    );
    assert!(!host.services().calls.contains(&"restore_lease"));
    assert!(!host.services().calls.contains(&"resume_ingress"));
}

fn position(calls: &[&str], expected: &str) -> usize {
    calls
        .iter()
        .position(|value| *value == expected)
        .expect("expected host call")
}
