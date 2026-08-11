#[test]
fn host_gates_fail_before_io_for_each_denied_class() {
    let allowed = animus_std_workspace_edit::GateObservations {
        executor: animus_std_workspace_edit::GateDecision::Allowed,
        capability: animus_std_workspace_edit::GateDecision::Allowed,
        authority: animus_std_workspace_edit::GateDecision::Allowed,
        approval: animus_std_workspace_edit::GateDecision::Allowed,
        mutation: animus_std_workspace_edit::GateDecision::Allowed,
        lifecycle: animus_std_workspace_edit::LifecycleGate::Current,
        output: animus_std_workspace_edit::GateDecision::Allowed,
    };
    assert_eq!(
        animus_std_workspace_edit::validate_host_gates(allowed),
        Ok(())
    );
    let denied = [
        animus_std_workspace_edit::GateObservations {
            executor: animus_std_workspace_edit::GateDecision::Denied,
            ..allowed
        },
        animus_std_workspace_edit::GateObservations {
            capability: animus_std_workspace_edit::GateDecision::Denied,
            ..allowed
        },
        animus_std_workspace_edit::GateObservations {
            authority: animus_std_workspace_edit::GateDecision::Denied,
            ..allowed
        },
        animus_std_workspace_edit::GateObservations {
            approval: animus_std_workspace_edit::GateDecision::Denied,
            ..allowed
        },
        animus_std_workspace_edit::GateObservations {
            mutation: animus_std_workspace_edit::GateDecision::Denied,
            ..allowed
        },
        animus_std_workspace_edit::GateObservations {
            lifecycle: animus_std_workspace_edit::LifecycleGate::Cancelled,
            ..allowed
        },
        animus_std_workspace_edit::GateObservations {
            lifecycle: animus_std_workspace_edit::LifecycleGate::TimedOut,
            ..allowed
        },
        animus_std_workspace_edit::GateObservations {
            output: animus_std_workspace_edit::GateDecision::Denied,
            ..allowed
        },
    ];
    for gates in denied {
        assert!(matches!(
            animus_std_workspace_edit::validate_host_gates(gates),
            Err(animus_std_workspace_edit::HostIssue::GateDenied(_))
        ));
    }
}
