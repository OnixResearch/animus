use super::support::*;

#[test]
fn authorized_prompt_admission_records_safe_receipt_without_prompt_body() {
    let request = request_with(Some(credential(VALID_CREDENTIAL)), Some(HASH_B))
        .expect("request should validate");

    let decision = super::super::admit(&request).expect("decision should evaluate");
    let receipt_text = decision.receipt().render_safe_text();

    assert!(decision.is_allowed());
    assert_eq!(decision.verdict(), super::super::AdmissionVerdict::Allow);
    assert_eq!(decision.reason(), None);
    assert_eq!(
        decision
            .receipt()
            .credential_hash()
            .map(crate::ArtifactHash::as_str),
        Some(HASH_A)
    );
    assert!(!receipt_text.contains(ACTOR));
    assert!(!receipt_text.contains(AUDIENCE));
    assert!(!receipt_text.contains(PROMPT_BODY));
    super::super::validate_safe_text(receipt_text.as_str()).expect("safe receipt should validate");
}

#[test]
fn host_owned_boundaries_are_supplied_hashes_only() {
    let request = request_with(Some(credential(VALID_CREDENTIAL)), Some(HASH_B))
        .expect("request should validate");
    let credential = request.credential().expect("credential should exist");

    assert_eq!(credential.credential_hash().as_str(), HASH_A);
    assert_eq!(
        request.policy_hash().map(crate::ArtifactHash::as_str),
        Some(HASH_B)
    );
}
