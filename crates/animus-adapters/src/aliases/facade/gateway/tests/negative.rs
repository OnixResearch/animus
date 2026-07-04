use super::support::*;

#[test]
fn missing_credential_blocks_before_side_effects() {
    let request = request_with(None, Some(HASH_B)).expect("request should validate");

    let decision = super::super::admit(&request).expect("decision should evaluate");

    assert_eq!(decision.verdict(), super::super::AdmissionVerdict::Deny);
    assert_eq!(
        decision.reason(),
        Some(super::super::AdmissionReason::MissingCredential)
    );
    assert!(!decision.is_allowed());
}

#[test]
fn wrong_audience_fails_closed() {
    let reason = denied_reason(
        Some(credential(WRONG_AUDIENCE_CREDENTIAL)),
        Some(HASH_B),
        super::super::AdmissionCaveat::Allow,
    );

    assert_eq!(reason, Some(super::super::AdmissionReason::WrongAudience));
}

#[test]
fn expired_credential_fails_closed() {
    let reason = denied_reason(
        Some(super::super::CredentialEvidenceDraft {
            expiry: super::super::CredentialExpiry::Expired,
            ..credential(VALID_CREDENTIAL)
        }),
        Some(HASH_B),
        super::super::AdmissionCaveat::Allow,
    );

    assert_eq!(
        reason,
        Some(super::super::AdmissionReason::ExpiredCredential)
    );
}

#[test]
fn revoked_credential_fails_closed() {
    let reason = denied_reason(
        Some(super::super::CredentialEvidenceDraft {
            verifier_status: super::super::CredentialVerifierStatus::Revoked,
            ..credential(VALID_CREDENTIAL)
        }),
        Some(HASH_B),
        super::super::AdmissionCaveat::Allow,
    );

    assert_eq!(
        reason,
        Some(super::super::AdmissionReason::RevokedCredential)
    );
}

#[test]
fn missing_grant_fails_closed() {
    let reason = denied_reason(
        Some(credential(OTHER_RESOURCE_CREDENTIAL)),
        Some(HASH_B),
        super::super::AdmissionCaveat::Allow,
    );

    assert_eq!(reason, Some(super::super::AdmissionReason::MissingGrant));
}

#[test]
fn caveat_denial_fails_closed() {
    let reason = denied_reason(
        Some(credential(VALID_CREDENTIAL)),
        Some(HASH_B),
        super::super::AdmissionCaveat::Deny,
    );

    assert_eq!(reason, Some(super::super::AdmissionReason::CaveatDenied));
}

#[test]
fn verifier_unavailable_fails_closed() {
    let reason = denied_reason(
        Some(super::super::CredentialEvidenceDraft {
            verifier_status: super::super::CredentialVerifierStatus::Unavailable,
            ..credential(VALID_CREDENTIAL)
        }),
        Some(HASH_B),
        super::super::AdmissionCaveat::Allow,
    );

    assert_eq!(
        reason,
        Some(super::super::AdmissionReason::VerifierUnavailable)
    );
}

#[test]
fn missing_policy_fails_closed() {
    let reason = denied_reason(
        Some(credential(VALID_CREDENTIAL)),
        None,
        super::super::AdmissionCaveat::Allow,
    );

    assert_eq!(reason, Some(super::super::AdmissionReason::MissingPolicy));
}

#[test]
fn unsupported_entrypoint_fails_closed() {
    let request = super::super::AdmissionRequest::new(&super::super::AdmissionRequestDraft {
        entrypoint: super::super::Entrypoint::Unsupported,
        ..draft(
            Some(credential(VALID_CREDENTIAL)),
            Some(HASH_B),
            super::super::AdmissionCaveat::Allow,
        )
    })
    .expect("request should validate");

    let reason = super::super::admit(&request)
        .expect("decision should evaluate")
        .reason();

    assert_eq!(
        reason,
        Some(super::super::AdmissionReason::UnsupportedEntrypoint)
    );
}

#[test]
fn receipt_leak_fixtures_are_rejected() {
    for marker in super::super::LEAK_MARKERS {
        assert_eq!(
            super::super::validate_safe_text(marker),
            Err(super::super::AdmissionError::ReceiptLeak { marker })
        );
    }
}
