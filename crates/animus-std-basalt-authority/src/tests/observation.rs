use super::*;
use basalt::ucan::{
    HolderRecheckPolicies, HolderTokenVerificationContext, OperationProfileAdmission,
    OperationProfileAdmissionError, OperationProfileId, recheck_holder_authorization,
};

struct DocumentProfile;

impl OperationProfileAdmission for DocumentProfile {
    fn admit_operation_profile(
        &self,
        profile: &OperationProfileId,
    ) -> Result<(), OperationProfileAdmissionError> {
        if profile == &EnforcementAction::DocumentWrite.operation_profile_id() {
            Ok(())
        } else {
            Err(OperationProfileAdmissionError::UnknownProfile)
        }
    }
}

// r[verify agent_harness_runtime.holder_observation.boundary]
pub(super) fn check_observations(
    view: &ScopedAuthority<'_>,
    fixture: &Fixture,
    replay: &RecordingReplay,
    commits: &Cell<u32>,
) {
    let policies = HolderRecheckPolicies {
        caveats: &DenyUnknownCaveats,
        profiles: &DocumentProfile,
    };
    let receipt = view.receipt().clone();
    let initial_time = VerificationTime::from_unix_seconds(VERIFY_AT_SECONDS);
    let context = HolderTokenVerificationContext::new(
        initial_time,
        &fixture.keys,
        &fixture.proofs,
        &NoRevocations,
        VerificationLimits::default(),
    );
    for _ in 0..2 {
        let observation = recheck_holder_authorization(
            view.holder(),
            fixture.token.as_str(),
            &context,
            &policies,
        )
        .expect("same admitted holder remains observable");
        assert!(std::ptr::eq(observation.authorization(), view.holder()));
        assert_eq!(observation.observed_at(), initial_time);
        assert_eq!(view.receipt(), &receipt);
        assert_eq!(replay.calls.get(), FIRST_CALL);
        assert_eq!(commits.get(), FIRST_CALL);
    }
    assert!(matches!(
        recheck_holder_authorization(view.holder(), MALFORMED_TOKEN, &context, &policies),
        Err(basalt::ucan::HolderRecheckError::Binding(_))
    ));
    let revoked = RevocationList::from_references(vec![proof_reference(&fixture.token)]);
    for (seconds, revocations) in [
        (
            EXPIRED_VERIFY_AT_SECONDS,
            &NoRevocations as &dyn basalt::ucan::RevocationChecker,
        ),
        (
            VERIFY_AT_SECONDS,
            &revoked as &dyn basalt::ucan::RevocationChecker,
        ),
    ] {
        let current = HolderTokenVerificationContext::new(
            VerificationTime::from_unix_seconds(seconds),
            &fixture.keys,
            &fixture.proofs,
            revocations,
            VerificationLimits::default(),
        );
        assert!(matches!(
            recheck_holder_authorization(
                view.holder(),
                fixture.token.as_str(),
                &current,
                &policies
            ),
            Err(basalt::ucan::HolderRecheckError::Verification(_))
        ));
        assert_eq!(view.receipt(), &receipt);
        assert_eq!(replay.calls.get(), FIRST_CALL);
        assert_eq!(commits.get(), FIRST_CALL);
    }
}
