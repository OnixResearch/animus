pub(super) const AUDIENCE: &str = "animus-headless";
pub(super) const OTHER_AUDIENCE: &str = "other-runtime";
pub(super) const ACTOR: &str = "agent@example";
pub(super) const SCOPE: &str = "session-1";
pub(super) const RESOURCE: &str = "session:session-1";
pub(super) const OTHER_RESOURCE: &str = "session:other";
pub(super) const ABILITY: &str = "prompt_submit";
pub(super) const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
pub(super) const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
pub(super) const PROMPT_BODY: &str = "prompt_body: raw prompt";

#[derive(Clone, Copy)]
pub(super) struct CredentialFixture<'a> {
    pub(super) audience: &'a str,
    pub(super) resource: &'a str,
    pub(super) ability: &'a str,
}

pub(super) const VALID_CREDENTIAL: CredentialFixture<'static> = CredentialFixture {
    audience: AUDIENCE,
    resource: RESOURCE,
    ability: ABILITY,
};
pub(super) const WRONG_AUDIENCE_CREDENTIAL: CredentialFixture<'static> = CredentialFixture {
    audience: OTHER_AUDIENCE,
    resource: RESOURCE,
    ability: ABILITY,
};
pub(super) const OTHER_RESOURCE_CREDENTIAL: CredentialFixture<'static> = CredentialFixture {
    audience: AUDIENCE,
    resource: OTHER_RESOURCE,
    ability: ABILITY,
};

pub(super) fn denied_reason(
    credential: Option<super::super::CredentialEvidenceDraft<'_>>,
    policy_hash: Option<&str>,
    caveat: super::super::AdmissionCaveat,
) -> Option<super::super::AdmissionReason> {
    let request = super::super::AdmissionRequest::new(&super::support::draft(
        credential,
        policy_hash,
        caveat,
    ))
    .expect("request should validate");
    super::super::admit(&request)
        .expect("decision should evaluate")
        .reason()
}

pub(super) fn request_with(
    credential: Option<super::super::CredentialEvidenceDraft<'_>>,
    policy_hash: Option<&str>,
) -> Result<super::super::AdmissionRequest, super::super::AdmissionError> {
    super::super::AdmissionRequest::new(&draft(
        credential,
        policy_hash,
        super::super::AdmissionCaveat::Allow,
    ))
}

pub(super) fn draft<'a>(
    credential: Option<super::super::CredentialEvidenceDraft<'a>>,
    policy_hash: Option<&'a str>,
    caveat: super::super::AdmissionCaveat,
) -> super::super::AdmissionRequestDraft<'a> {
    super::super::AdmissionRequestDraft {
        audience: AUDIENCE,
        actor: ACTOR,
        entrypoint: super::super::Entrypoint::HeadlessRpc,
        scope: SCOPE,
        action: super::super::Action::PromptSubmit,
        resource: RESOURCE,
        ability: ABILITY,
        credential,
        caveat,
        policy_hash,
    }
}

pub(super) fn credential(
    fixture: CredentialFixture<'_>,
) -> super::super::CredentialEvidenceDraft<'_> {
    super::super::CredentialEvidenceDraft {
        audience: fixture.audience,
        actor: ACTOR,
        resource: fixture.resource,
        ability: fixture.ability,
        credential_hash: HASH_A,
        expiry: super::super::CredentialExpiry::Current,
        verifier_status: super::super::CredentialVerifierStatus::Verified,
    }
}
