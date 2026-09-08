use super::*;

const NARROW_NOT_BEFORE_SECONDS: u64 = 1_200;
const NARROW_EXPIRES_SECONDS: u64 = 1_800;

fn narrow_fixture() -> (Fixture, TokenTimeBounds) {
    let mut fixture = Fixture::direct();
    let bounds = TokenTimeBounds::new(NARROW_NOT_BEFORE_SECONDS, NARROW_EXPIRES_SECONDS)
        .expect("narrow bounds");
    fixture.token = issue_token_with_signer(
        &signer(ISSUER_SEED_BYTE),
        &fixture.audience,
        &CapabilitySet::new(vec![document_capability(DOCUMENT_RESOURCE)]).expect("capability"),
        &ProofReferences::empty(),
        bounds,
    )
    .expect("narrow token");
    (fixture, bounds)
}

struct Commit<'a> {
    calls: &'a Cell<u32>,
    outcome: Result<&'static str, HostAuthorityIssue>,
}
impl HostAuthorityDecisionCommit for Commit<'_> {
    fn commit_decision(
        &mut self,
        receipt: &animus_adapters::HostAuthorityReceipt,
    ) -> Result<String, HostAuthorityIssue> {
        assert_eq!(receipt.dispatch_class, "committed");
        self.calls.set(
            self.calls
                .get()
                .checked_add(FIRST_CALL)
                .expect("bounded commits"),
        );
        self.outcome.map(String::from)
    }
}

// r[verify agent_harness_runtime.scoped_authority.view]
// r[verify agent_harness_runtime.scoped_authority.order]
#[test]
fn direct_and_delegated_views_follow_commit_and_preserve_exact_authority() {
    for (fixture, expected_time) in [
        (Fixture::direct(), token_time_bounds()),
        (Fixture::delegated(), token_time_bounds()),
        narrow_fixture(),
    ] {
        let replay = RecordingReplay::new();
        let request = document_request(fixture.audience.as_str(), GuardedAction::Filesystem);
        let enforcement =
            EnforcementRequest::new(DOCUMENT_CONTRACT, DOCUMENT_RESOURCE, DOCUMENT_ABILITY);
        let nonce = nonce();
        let signature = sign(&fixture, &enforcement, nonce);
        let policy = default_policy().expect("policy");
        let policy_ref = default_authority_policy_ref(DOCUMENT_CONTRACT);
        let service = BasaltAuthorityService::new(BasaltAuthorityInvocation {
            compact_token: fixture.token.as_str(),
            nonce,
            holder_signature: &signature,
            audience: &fixture.audience,
            policy: &policy,
            policy_ref: &policy_ref,
            boundaries: boundaries(&fixture, &replay),
        });
        let commits = Cell::new(0);
        let mut commit = Commit {
            calls: &commits,
            outcome: Ok(HASH_C),
        };
        let result = dispatch_with_basalt_scoped(
            BasaltDispatchContext {
                service: &service,
                request: &request,
                gates: all_gates(),
                commit: &mut commit,
            },
            |view| {
                assert_eq!(commits.get(), FIRST_CALL);
                assert_eq!(replay.calls.get(), FIRST_CALL);
                assert_eq!(view.commit_identity(), HASH_C);
                assert_eq!(view.holder().audience(), &fixture.audience);
                assert_eq!(view.holder().resource(), DOCUMENT_RESOURCE);
                assert_eq!(view.holder().ability(), DOCUMENT_ABILITY);
                assert_eq!(view.holder().nonce(), nonce);
                assert_eq!(view.holder().effective_time_bounds(), expected_time);
                assert_eq!(view.receipt().request_identity, request.request_identity());
                assert_eq!(
                    view.decision().decision_identity(),
                    view.receipt().decision_identity
                );
                Ok(view.receipt().receipt_identity.clone())
            },
        )
        .expect("scoped dispatch");
        assert_eq!(result.output, result.authority_receipt.receipt_identity);
        assert_eq!(commits.get(), FIRST_CALL);
        assert_eq!(replay.calls.get(), FIRST_CALL);
        assert!(!format!("{:?}", result.authority_receipt).contains(fixture.token.as_str()));
        let denied = dispatch_with_basalt_scoped(
            BasaltDispatchContext {
                service: &service,
                request: &request,
                gates: all_gates(),
                commit: &mut commit,
            },
            |_view| -> Result<(), HostAuthorityIssue> { panic!("replay reached callback") },
        )
        .expect_err("replay denial");
        assert_eq!(denied.issue(), HostAuthorityIssue::ReplayDenied);
        assert_eq!(commits.get(), FIRST_CALL);
    }
}

#[test]
fn execution_view_binds_the_actual_operation() {
    let fixture = execution_fixture();
    let replay = RecordingReplay::new();
    let request = execution_request(fixture.audience.as_str());
    let enforcement = execution_enforcement();
    let nonce = nonce();
    let signature = sign_action(
        &fixture,
        &enforcement,
        nonce,
        EnforcementAction::ProcessExecute,
    );
    let (policy, policy_ref) = execution_policy_parts();
    let invocation = ExecutionInvocation {
        fixture: &fixture,
        nonce,
        signature: &signature,
        replay: &replay,
        policy: &policy,
        policy_ref: &policy_ref,
    };
    let service = execution_service(&invocation);
    let commits = Cell::new(0);
    let mut commit = Commit {
        calls: &commits,
        outcome: Ok(HASH_C),
    };
    let result = dispatch_with_basalt_scoped(
        BasaltDispatchContext {
            service: &service,
            request: &request,
            gates: all_gates(),
            commit: &mut commit,
        },
        |view| {
            assert_eq!(commits.get(), FIRST_CALL);
            assert_eq!(view.holder().resource(), enforcement.resource());
            assert_eq!(view.holder().ability(), "process/execute");
            assert_eq!(view.holder().effective_time_bounds(), token_time_bounds());
            assert_eq!(
                view.holder().operation(),
                bind_enforcement_action(EnforcementAction::ProcessExecute, &enforcement)
                    .expect("operation")
            );
            Ok(())
        },
    )
    .expect("execution");
    assert_eq!(result.commit_identity, HASH_C);
    assert_eq!(replay.calls.get(), FIRST_CALL);
}

#[derive(Clone, Copy)]
enum Denial {
    Provider,
    Expired,
    Gate,
    Commit,
    InvalidCommit,
    Effect,
}

#[test]
fn denied_paths_never_expose_the_view_and_effect_errors_retain_receipts() {
    for mode in [
        Denial::Provider,
        Denial::Expired,
        Denial::Gate,
        Denial::Commit,
        Denial::InvalidCommit,
        Denial::Effect,
    ] {
        let fixture = Fixture::direct();
        let replay = RecordingReplay::new();
        let request = document_request(fixture.audience.as_str(), GuardedAction::Filesystem);
        let enforcement =
            EnforcementRequest::new(DOCUMENT_CONTRACT, DOCUMENT_RESOURCE, DOCUMENT_ABILITY);
        let nonce = nonce();
        let signature = sign(&fixture, &enforcement, nonce);
        let policy = default_policy().expect("policy");
        let policy_ref = default_authority_policy_ref(DOCUMENT_CONTRACT);
        let mut verification = boundaries(&fixture, &replay);
        if matches!(mode, Denial::Expired) {
            verification.verification_time =
                VerificationTime::from_unix_seconds(EXPIRED_VERIFY_AT_SECONDS);
        }
        let service = BasaltAuthorityService::new(BasaltAuthorityInvocation {
            compact_token: if matches!(mode, Denial::Provider) {
                MALFORMED_TOKEN
            } else {
                fixture.token.as_str()
            },
            nonce,
            holder_signature: &signature,
            audience: &fixture.audience,
            policy: &policy,
            policy_ref: &policy_ref,
            boundaries: verification,
        });
        let mut gates = all_gates();
        if matches!(mode, Denial::Gate) {
            gates.sandbox = HostGateStatus::Denied;
        }
        let commits = Cell::new(0);
        let calls = Cell::new(0);
        let mut commit = Commit {
            calls: &commits,
            outcome: match mode {
                Denial::Commit => Err(HostAuthorityIssue::CommitFailed),
                Denial::InvalidCommit => Ok("invalid-commit"),
                _ => Ok(HASH_C),
            },
        };
        let failure = dispatch_with_basalt_scoped(
            BasaltDispatchContext {
                service: &service,
                request: &request,
                gates,
                commit: &mut commit,
            },
            |_view| -> Result<(), HostAuthorityIssue> {
                calls.set(FIRST_CALL);
                Err(HostAuthorityIssue::ProviderUnavailable)
            },
        )
        .expect_err("denied");
        if matches!(mode, Denial::Effect) {
            assert_eq!(calls.get(), FIRST_CALL);
            assert!(failure.authority_receipt().is_some());
        } else {
            assert_eq!(calls.get(), 0);
        }
        if matches!(mode, Denial::Provider | Denial::Expired) {
            assert_eq!(replay.calls.get(), 0);
            assert_eq!(commits.get(), 0);
        } else {
            assert_eq!(replay.calls.get(), FIRST_CALL);
        }
    }
}
