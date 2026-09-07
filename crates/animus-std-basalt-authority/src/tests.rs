use std::cell::{Cell, RefCell};
use std::collections::BTreeSet;

use animus_adapters::{
    AgentAuthorityBindingDraft, BindingFreshness, GuardedAction, HostAuthorityBounds,
    HostAuthorityIssue, HostAuthorityRequest, HostAuthorityRequestDraft, HostGateStatus,
    RequiredHostGates, admit_agent_authority_binding, admit_host_authority_request,
};
use basalt::authority_core::default_authority_policy_ref;
use basalt::ucan::core::holder_invocation::HOLDER_INVOCATION_NONCE_BYTES;
use basalt::ucan::token::DenyUnknownCaveats;
use basalt::ucan::{
    AudienceDid, CapabilityDocument, CapabilitySet, CaveatDocument, CompactToken,
    ED25519_SECRET_KEY_BYTES, Ed25519InMemorySigner, HolderInvocationLimits, HolderInvocationNonce,
    HolderInvocationReplayAdmission, HolderInvocationReplayCandidate, HolderInvocationReplayError,
    HolderInvocationSigningRequest, IssuerDid, KeyResolutionContext, KeyResolver, KeyResolverError,
    NoRevocations, ProofCollection, ProofReferences, RevocationList, SignerSignature, TokenSigner,
    TokenTimeBounds, VerificationLimits, VerificationTime, issue_token_with_signer,
    proof_reference, sign_holder_invocation,
};
use basalt::ucan_enforcement::{
    EnforcementAction, UcanEnforcementBoundaries, UcanEnforcementReason, bind_enforcement_action,
};
use basalt::{EnforcementRequest, default_policy};

use super::*;

const HASH_A: &str = "b3:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "b3:bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C: &str = "b3:cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const PARENT_RESOURCE: &str = "did:key:z6Mkresource";
const DOCUMENT_RESOURCE: &str = "did:key:z6Mkresource/docs/report";
const OTHER_RESOURCE: &str = "did:key:z6Mkresource/videos/report";
const DOCUMENT_ABILITY: &str = "document/write";
const DOCUMENT_CONTRACT: &str = "document-write";
const READ_ABILITY: &str = "document/read";
const READ_CONTRACT: &str = "document-read";
const CAVEAT_DOMAIN: &str = "example";
const CAVEAT_TYPE: &str = "approval";
const CAVEAT_KEY: &str = "required";
const CAVEAT_PAYLOAD: &[u8] = b"yes";
const MALFORMED_TOKEN: &str = "not.enough";
const ISSUER_SEED_BYTE: u8 = 11;
const DELEGATE_SEED_BYTE: u8 = 13;
const HOLDER_SEED_BYTE: u8 = 17;
const OTHER_HOLDER_SEED_BYTE: u8 = 23;
const NONCE_BYTE: u8 = 19;
const OTHER_NONCE_BYTE: u8 = 29;
const TOKEN_NOT_BEFORE_SECONDS: u64 = 1_000;
const TOKEN_EXPIRES_SECONDS: u64 = 2_000;
const VERIFY_AT_SECONDS: u64 = 1_500;
const EXPIRED_VERIFY_AT_SECONDS: u64 = 2_500;
const UNAVAILABLE_KEY_MESSAGE: &str = "injected unavailable key service";
const IDENTITY_CAPACITY_BYTES: usize = 160;
const RESOURCE_CEILING_BYTES: usize = 512;
const CHECKED_FACT_ALLOWANCE: u32 = 32;
const FIRST_CALL: u32 = 1;

struct Fixture {
    token: CompactToken,
    holder: Ed25519InMemorySigner,
    audience: AudienceDid,
    keys: KeyResolutionContext,
    proofs: ProofCollection,
}

impl Fixture {
    fn direct() -> Self {
        Self::direct_with_capability(document_capability(DOCUMENT_RESOURCE))
    }

    fn caveated() -> Self {
        let caveat = CaveatDocument::new(
            CAVEAT_DOMAIN.to_owned(),
            CAVEAT_TYPE.to_owned(),
            CAVEAT_KEY.to_owned(),
            CAVEAT_PAYLOAD.to_vec(),
        )
        .expect("caveat");
        let capability = CapabilityDocument::with_caveats(
            PARENT_RESOURCE.to_owned(),
            DOCUMENT_ABILITY.to_owned(),
            vec![caveat],
        )
        .expect("caveated capability");
        Self::direct_with_capability(capability)
    }

    fn direct_with_capability(capability: CapabilityDocument) -> Self {
        let issuer = signer(ISSUER_SEED_BYTE);
        let holder = signer(HOLDER_SEED_BYTE);
        let audience = AudienceDid::from(holder.issuer().expect("holder DID"));
        let token = issue_token_with_signer(
            &issuer,
            &audience,
            &CapabilitySet::new(vec![capability]).expect("capability set"),
            &ProofReferences::empty(),
            TokenTimeBounds::new(TOKEN_NOT_BEFORE_SECONDS, TOKEN_EXPIRES_SECONDS)
                .expect("time bounds"),
        )
        .expect("token");
        let keys = KeyResolutionContext::new(vec![
            issuer.verification_key().expect("issuer key"),
            holder.verification_key().expect("holder key"),
        ]);
        Self {
            token,
            holder,
            audience,
            keys,
            proofs: ProofCollection::empty(),
        }
    }

    fn delegated() -> Self {
        let issuer = signer(ISSUER_SEED_BYTE);
        let delegate = signer(DELEGATE_SEED_BYTE);
        let holder = signer(HOLDER_SEED_BYTE);
        let delegate_audience = AudienceDid::from(delegate.issuer().expect("delegate DID"));
        let audience = AudienceDid::from(holder.issuer().expect("holder DID"));
        let parent = issue_token_with_signer(
            &issuer,
            &delegate_audience,
            &CapabilitySet::new(vec![document_capability(PARENT_RESOURCE)])
                .expect("parent capability set"),
            &ProofReferences::empty(),
            token_time_bounds(),
        )
        .expect("parent token");
        let child = issue_token_with_signer(
            &delegate,
            &audience,
            &CapabilitySet::new(vec![document_capability(DOCUMENT_RESOURCE)])
                .expect("child capability set"),
            &ProofReferences::from_references(vec![proof_reference(&parent)]),
            token_time_bounds(),
        )
        .expect("child token");
        let keys = KeyResolutionContext::new(vec![
            issuer.verification_key().expect("issuer key"),
            delegate.verification_key().expect("delegate key"),
            holder.verification_key().expect("holder key"),
        ]);
        Self {
            token: child,
            holder,
            audience,
            keys,
            proofs: ProofCollection::from_tokens(vec![parent]),
        }
    }
}

struct RecordingReplay {
    calls: Cell<u32>,
    seen: RefCell<BTreeSet<(String, [u8; HOLDER_INVOCATION_NONCE_BYTES])>>,
}

impl RecordingReplay {
    const fn new() -> Self {
        Self {
            calls: Cell::new(0),
            seen: RefCell::new(BTreeSet::new()),
        }
    }
}

impl HolderInvocationReplayAdmission for RecordingReplay {
    fn admit_holder_invocation(
        &self,
        candidate: &HolderInvocationReplayCandidate<'_>,
    ) -> Result<(), HolderInvocationReplayError> {
        self.calls.set(
            self.calls
                .get()
                .checked_add(FIRST_CALL)
                .expect("bounded replay call count"),
        );
        let key = (
            candidate.audience().as_str().to_owned(),
            *candidate.nonce().as_bytes(),
        );
        if self.seen.borrow_mut().insert(key) {
            Ok(())
        } else {
            Err(HolderInvocationReplayError::Duplicate)
        }
    }
}

struct UnavailableKeys;

impl KeyResolver for UnavailableKeys {
    fn resolve_key(
        &self,
        issuer: &IssuerDid,
    ) -> Result<basalt::ucan::Ed25519PublicKey, KeyResolverError> {
        Err(KeyResolverError::Backend {
            issuer: issuer.as_str().to_owned(),
            message: UNAVAILABLE_KEY_MESSAGE.to_owned(),
        })
    }
}

struct RecordingCommit {
    calls: u32,
    allow: bool,
}

impl HostAuthorityDecisionCommit for RecordingCommit {
    fn commit_decision(
        &mut self,
        _receipt: &animus_adapters::HostAuthorityReceipt,
    ) -> Result<String, HostAuthorityIssue> {
        self.calls = self
            .calls
            .checked_add(FIRST_CALL)
            .ok_or(HostAuthorityIssue::CommitFailed)?;
        if self.allow {
            Ok(String::from(HASH_C))
        } else {
            Err(HostAuthorityIssue::CommitFailed)
        }
    }
}

#[test]
fn live_holder_authority_dispatches_only_after_local_gates_and_commit() {
    let fixture = Fixture::direct();
    let replay = RecordingReplay::new();
    let generic = document_request(fixture.audience.as_str(), GuardedAction::Filesystem);
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
    let mut commit = RecordingCommit {
        calls: 0,
        allow: true,
    };
    let effect_calls = Cell::new(0_u32);

    let dispatched = dispatch_with_basalt(
        BasaltDispatchContext {
            service: &service,
            request: &generic,
            gates: all_gates(),
            commit: &mut commit,
        },
        || {
            effect_calls.set(
                effect_calls
                    .get()
                    .checked_add(FIRST_CALL)
                    .expect("effect calls"),
            );
            Ok("effect-result")
        },
    )
    .expect("admitted dispatch");

    assert_eq!(dispatched.output, "effect-result");
    assert_eq!(commit.calls, FIRST_CALL);
    assert_eq!(effect_calls.get(), FIRST_CALL);
    assert_eq!(replay.calls.get(), FIRST_CALL);
    assert_eq!(dispatched.authority_receipt.dispatch_class, "committed");
    assert!(!format!("{:?}", dispatched.authority_receipt).contains(fixture.token.as_str()));
}

#[test]
fn delegated_holder_authority_reaches_the_same_exact_adapter_boundary() {
    let fixture = Fixture::delegated();
    let replay = RecordingReplay::new();
    let generic = document_request(fixture.audience.as_str(), GuardedAction::Filesystem);
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

    let decision = service
        .authorize_with_evidence(&generic)
        .expect("delegated holder authority");

    assert_eq!(
        decision.status(),
        animus_adapters::HostAuthorityStatus::Allowed
    );
    assert_eq!(replay.calls.get(), FIRST_CALL);
}

#[test]
fn replay_local_gate_commit_and_mapping_failures_never_dispatch_effects() {
    let fixture = Fixture::direct();
    let replay = RecordingReplay::new();
    let generic = document_request(fixture.audience.as_str(), GuardedAction::Filesystem);
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
    let mut commit = RecordingCommit {
        calls: 0,
        allow: true,
    };
    let effect_calls = Cell::new(0_u32);
    let mut denied_gates = all_gates();
    denied_gates.sandbox = HostGateStatus::Denied;

    let local = dispatch_with_basalt(
        BasaltDispatchContext {
            service: &service,
            request: &generic,
            gates: denied_gates,
            commit: &mut commit,
        },
        || {
            effect_calls.set(FIRST_CALL);
            Ok(())
        },
    )
    .expect_err("local gate denial");
    assert_eq!(local.issue(), HostAuthorityIssue::LocalGateDenied);
    assert_eq!(effect_calls.get(), 0);
    assert_eq!(commit.calls, 0);

    let replay_denied = service
        .authorize_with_evidence(&generic)
        .expect_err("same nonce must replay deny");
    assert_eq!(replay_denied.issue(), HostAuthorityIssue::ReplayDenied);
    assert!(replay_denied.basalt_receipt().is_some());

    let unsupported = document_request(fixture.audience.as_str(), GuardedAction::Exec);
    assert_eq!(
        plan_basalt_request(&unsupported).expect_err("unsupported action"),
        HostAuthorityIssue::UnsupportedAction
    );
}

#[test]
fn unavailable_and_stale_verification_services_fail_before_replay() {
    let fixture = Fixture::direct();
    let replay = RecordingReplay::new();
    let generic = document_request(fixture.audience.as_str(), GuardedAction::Filesystem);
    let enforcement =
        EnforcementRequest::new(DOCUMENT_CONTRACT, DOCUMENT_RESOURCE, DOCUMENT_ABILITY);
    let nonce = nonce();
    let signature = sign(&fixture, &enforcement, nonce);
    let policy = default_policy().expect("policy");
    let policy_ref = default_authority_policy_ref(DOCUMENT_CONTRACT);
    let unavailable = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: fixture.token.as_str(),
        nonce,
        holder_signature: &signature,
        audience: &fixture.audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: UcanEnforcementBoundaries {
            verification_time: VerificationTime::from_unix_seconds(VERIFY_AT_SECONDS),
            key_resolver: &UnavailableKeys,
            proof_store: &fixture.proofs,
            revocations: &NoRevocations,
            caveat_policies: &DenyUnknownCaveats,
            replay: &replay,
            verification_limits: VerificationLimits::default(),
            holder_limits: HolderInvocationLimits::default(),
        },
    });
    assert_eq!(
        unavailable
            .authorize_with_evidence(&generic)
            .expect_err("unavailable key service")
            .issue(),
        HostAuthorityIssue::ProviderUnavailable
    );
    assert_eq!(replay.calls.get(), 0);

    let stale = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: fixture.token.as_str(),
        nonce,
        holder_signature: &signature,
        audience: &fixture.audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: UcanEnforcementBoundaries {
            verification_time: VerificationTime::from_unix_seconds(EXPIRED_VERIFY_AT_SECONDS),
            key_resolver: &fixture.keys,
            proof_store: &fixture.proofs,
            revocations: &NoRevocations,
            caveat_policies: &DenyUnknownCaveats,
            replay: &replay,
            verification_limits: VerificationLimits::default(),
            holder_limits: HolderInvocationLimits::default(),
        },
    });
    assert_eq!(
        stale
            .authorize_with_evidence(&generic)
            .expect_err("expired verification time")
            .issue(),
        HostAuthorityIssue::StaleVerification
    );
    assert_eq!(replay.calls.get(), 0);
}

#[test]
fn raw_token_holder_and_proof_fail_before_replay() {
    let direct = Fixture::direct();
    let delegated = Fixture::delegated();
    let replay = RecordingReplay::new();
    let generic = document_request(direct.audience.as_str(), GuardedAction::Filesystem);
    let enforcement =
        EnforcementRequest::new(DOCUMENT_CONTRACT, DOCUMENT_RESOURCE, DOCUMENT_ABILITY);
    let nonce = nonce();
    let signature = sign(&direct, &enforcement, nonce);
    let policy = default_policy().expect("policy");
    let policy_ref = default_authority_policy_ref(DOCUMENT_CONTRACT);

    let malformed = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: MALFORMED_TOKEN,
        nonce,
        holder_signature: &signature,
        audience: &direct.audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: boundaries(&direct, &replay),
    });
    assert_failure_reason(
        &malformed
            .authorize_with_evidence(&generic)
            .expect_err("malformed token"),
        HostAuthorityIssue::VerificationDenied,
        UcanEnforcementReason::MalformedCredential,
    );

    let wrong_holder_signature = sign(&direct, &enforcement, other_nonce());
    let wrong_holder = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: direct.token.as_str(),
        nonce,
        holder_signature: &wrong_holder_signature,
        audience: &direct.audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: boundaries(&direct, &replay),
    });
    assert_failure_reason(
        &wrong_holder
            .authorize_with_evidence(&generic)
            .expect_err("wrong holder"),
        HostAuthorityIssue::VerificationDenied,
        UcanEnforcementReason::HolderDenied,
    );

    let delegated_generic =
        document_request(delegated.audience.as_str(), GuardedAction::Filesystem);
    let delegated_signature = sign(&delegated, &enforcement, nonce);
    let empty_proofs = ProofCollection::empty();
    let proof_denied = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: delegated.token.as_str(),
        nonce,
        holder_signature: &delegated_signature,
        audience: &delegated.audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: UcanEnforcementBoundaries {
            verification_time: VerificationTime::from_unix_seconds(VERIFY_AT_SECONDS),
            key_resolver: &delegated.keys,
            proof_store: &empty_proofs,
            revocations: &NoRevocations,
            caveat_policies: &DenyUnknownCaveats,
            replay: &replay,
            verification_limits: VerificationLimits::default(),
            holder_limits: HolderInvocationLimits::default(),
        },
    });
    assert_failure_reason(
        &proof_denied
            .authorize_with_evidence(&delegated_generic)
            .expect_err("missing proof"),
        HostAuthorityIssue::VerificationDenied,
        UcanEnforcementReason::ProofDenied,
    );
    assert_eq!(replay.calls.get(), 0);
}

#[test]
fn revocation_caveat_and_resource_scope_fail_before_replay() {
    let direct = Fixture::direct();
    let caveated = Fixture::caveated();
    let replay = RecordingReplay::new();
    let generic = document_request(direct.audience.as_str(), GuardedAction::Filesystem);
    let enforcement =
        EnforcementRequest::new(DOCUMENT_CONTRACT, DOCUMENT_RESOURCE, DOCUMENT_ABILITY);
    let nonce = nonce();
    let signature = sign(&direct, &enforcement, nonce);
    let policy = default_policy().expect("policy");
    let policy_ref = default_authority_policy_ref(DOCUMENT_CONTRACT);
    let revocations = RevocationList::from_references(vec![proof_reference(&direct.token)]);
    let revoked = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: direct.token.as_str(),
        nonce,
        holder_signature: &signature,
        audience: &direct.audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: UcanEnforcementBoundaries {
            verification_time: VerificationTime::from_unix_seconds(VERIFY_AT_SECONDS),
            key_resolver: &direct.keys,
            proof_store: &direct.proofs,
            revocations: &revocations,
            caveat_policies: &DenyUnknownCaveats,
            replay: &replay,
            verification_limits: VerificationLimits::default(),
            holder_limits: HolderInvocationLimits::default(),
        },
    });
    assert_failure_reason(
        &revoked
            .authorize_with_evidence(&generic)
            .expect_err("revoked token"),
        HostAuthorityIssue::VerificationDenied,
        UcanEnforcementReason::Revoked,
    );

    let caveated_generic = document_request(caveated.audience.as_str(), GuardedAction::Filesystem);
    let caveated_signature = sign(&caveated, &enforcement, nonce);
    let caveat_denied = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: caveated.token.as_str(),
        nonce,
        holder_signature: &caveated_signature,
        audience: &caveated.audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: boundaries(&caveated, &replay),
    });
    assert_failure_reason(
        &caveat_denied
            .authorize_with_evidence(&caveated_generic)
            .expect_err("unknown caveat"),
        HostAuthorityIssue::VerificationDenied,
        UcanEnforcementReason::CaveatDenied,
    );

    let profile_identity = operation_profile_identity(EnforcementAction::DocumentWrite);
    let other_generic = authority_request(
        direct.audience.as_str(),
        AuthorityRequestShape {
            action: GuardedAction::Filesystem,
            contract: DOCUMENT_CONTRACT,
            resource: OTHER_RESOURCE,
            ability: DOCUMENT_ABILITY,
            operation_profile_identity: profile_identity.as_str(),
            policy_identity: BASALT_POLICY_IDENTITY,
        },
    );
    let other_enforcement =
        EnforcementRequest::new(DOCUMENT_CONTRACT, OTHER_RESOURCE, DOCUMENT_ABILITY);
    let other_signature = sign(&direct, &other_enforcement, nonce);
    let scope_denied = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: direct.token.as_str(),
        nonce,
        holder_signature: &other_signature,
        audience: &direct.audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: boundaries(&direct, &replay),
    });
    assert_failure_reason(
        &scope_denied
            .authorize_with_evidence(&other_generic)
            .expect_err("resource scope"),
        HostAuthorityIssue::VerificationDenied,
        UcanEnforcementReason::ScopeDenied,
    );
    assert_eq!(replay.calls.get(), 0);
}

#[test]
fn ability_policy_and_operation_digest_fail_before_replay() {
    let direct = Fixture::direct();
    let replay = RecordingReplay::new();
    let generic = document_request(direct.audience.as_str(), GuardedAction::Filesystem);
    let enforcement =
        EnforcementRequest::new(DOCUMENT_CONTRACT, DOCUMENT_RESOURCE, DOCUMENT_ABILITY);
    let nonce = nonce();
    let signature = sign(&direct, &enforcement, nonce);
    let policy = default_policy().expect("policy");
    let profile_identity = operation_profile_identity(EnforcementAction::DocumentRead);
    let read_generic = authority_request(
        direct.audience.as_str(),
        AuthorityRequestShape {
            action: GuardedAction::Filesystem,
            contract: READ_CONTRACT,
            resource: DOCUMENT_RESOURCE,
            ability: READ_ABILITY,
            operation_profile_identity: profile_identity.as_str(),
            policy_identity: BASALT_POLICY_IDENTITY,
        },
    );
    let read_policy_ref = default_authority_policy_ref(READ_CONTRACT);
    let read_enforcement = EnforcementRequest::new(READ_CONTRACT, DOCUMENT_RESOURCE, READ_ABILITY);
    let read_signature = sign_action(
        &direct,
        &read_enforcement,
        nonce,
        EnforcementAction::DocumentRead,
    );
    let ability_denied = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: direct.token.as_str(),
        nonce,
        holder_signature: &read_signature,
        audience: &direct.audience,
        policy: &policy,
        policy_ref: &read_policy_ref,
        boundaries: boundaries(&direct, &replay),
    });
    assert_failure_reason(
        &ability_denied
            .authorize_with_evidence(&read_generic)
            .expect_err("ability scope"),
        HostAuthorityIssue::VerificationDenied,
        UcanEnforcementReason::ScopeDenied,
    );

    let policy_denied = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: direct.token.as_str(),
        nonce,
        holder_signature: &signature,
        audience: &direct.audience,
        policy: &policy,
        policy_ref: &read_policy_ref,
        boundaries: boundaries(&direct, &replay),
    });
    assert_eq!(
        policy_denied
            .authorize_with_evidence(&generic)
            .expect_err("policy contract mismatch")
            .issue(),
        HostAuthorityIssue::PolicyDenied
    );

    let digest_mismatch = authority_request(
        direct.audience.as_str(),
        AuthorityRequestShape {
            action: GuardedAction::Filesystem,
            contract: DOCUMENT_CONTRACT,
            resource: DOCUMENT_RESOURCE,
            ability: DOCUMENT_ABILITY,
            operation_profile_identity: HASH_A,
            policy_identity: BASALT_POLICY_IDENTITY,
        },
    );
    assert_eq!(
        plan_basalt_request(&digest_mismatch).expect_err("operation digest mismatch"),
        HostAuthorityIssue::UnsupportedAction
    );
    assert_eq!(replay.calls.get(), 0);
}

#[test]
fn binding_substitution_and_commit_failure_fail_before_effect() {
    let fixture = Fixture::direct();
    let replay = RecordingReplay::new();
    let generic = document_request(fixture.audience.as_str(), GuardedAction::Filesystem);
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
    let mut commit = RecordingCommit {
        calls: 0,
        allow: false,
    };
    let effect_calls = Cell::new(0_u32);
    let failure = dispatch_with_basalt(
        BasaltDispatchContext {
            service: &service,
            request: &generic,
            gates: all_gates(),
            commit: &mut commit,
        },
        || {
            effect_calls.set(FIRST_CALL);
            Ok(())
        },
    )
    .expect_err("commit failure");
    assert_eq!(failure.issue(), HostAuthorityIssue::CommitFailed);
    assert_eq!(effect_calls.get(), 0);
    assert_eq!(commit.calls, FIRST_CALL);

    let other_holder = signer(OTHER_HOLDER_SEED_BYTE);
    let other_audience = AudienceDid::from(other_holder.issuer().expect("other holder DID"));
    let substituted = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: fixture.token.as_str(),
        nonce,
        holder_signature: &signature,
        audience: &other_audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: boundaries(&fixture, &replay),
    });
    assert_eq!(
        substituted
            .authorize_with_evidence(&generic)
            .expect_err("audience substitution")
            .issue(),
        HostAuthorityIssue::BindingMismatch
    );
}

fn signer(seed: u8) -> Ed25519InMemorySigner {
    Ed25519InMemorySigner::from_seed_bytes([seed; ED25519_SECRET_KEY_BYTES])
}

fn token_time_bounds() -> TokenTimeBounds {
    TokenTimeBounds::new(TOKEN_NOT_BEFORE_SECONDS, TOKEN_EXPIRES_SECONDS).expect("time bounds")
}

const fn nonce() -> HolderInvocationNonce {
    HolderInvocationNonce::from_bytes([NONCE_BYTE; HOLDER_INVOCATION_NONCE_BYTES])
}

const fn other_nonce() -> HolderInvocationNonce {
    HolderInvocationNonce::from_bytes([OTHER_NONCE_BYTE; HOLDER_INVOCATION_NONCE_BYTES])
}

fn sign(
    fixture: &Fixture,
    enforcement: &EnforcementRequest,
    nonce: HolderInvocationNonce,
) -> SignerSignature {
    sign_action(
        fixture,
        enforcement,
        nonce,
        EnforcementAction::DocumentWrite,
    )
}

fn sign_action(
    fixture: &Fixture,
    enforcement: &EnforcementRequest,
    nonce: HolderInvocationNonce,
    action: EnforcementAction,
) -> SignerSignature {
    let operation = bind_enforcement_action(action, enforcement).expect("operation binding");
    sign_holder_invocation(
        &HolderInvocationSigningRequest::new(
            fixture.token.as_str(),
            &fixture.audience,
            enforcement.resource(),
            enforcement.ability(),
            operation,
            nonce,
        ),
        &fixture.holder,
        VerificationLimits::default(),
        HolderInvocationLimits::default(),
    )
    .expect("holder signature")
}

fn boundaries<'a>(
    fixture: &'a Fixture,
    replay: &'a RecordingReplay,
) -> UcanEnforcementBoundaries<'a> {
    UcanEnforcementBoundaries {
        verification_time: VerificationTime::from_unix_seconds(VERIFY_AT_SECONDS),
        key_resolver: &fixture.keys,
        proof_store: &fixture.proofs,
        revocations: &NoRevocations,
        caveat_policies: &DenyUnknownCaveats,
        replay,
        verification_limits: VerificationLimits::default(),
        holder_limits: HolderInvocationLimits::default(),
    }
}

#[derive(Clone, Copy)]
struct AuthorityRequestShape<'a> {
    action: GuardedAction,
    contract: &'a str,
    resource: &'a str,
    ability: &'a str,
    operation_profile_identity: &'a str,
    policy_identity: &'a str,
}

fn document_request(audience_did: &str, action: GuardedAction) -> HostAuthorityRequest {
    let profile_identity = operation_profile_identity(EnforcementAction::DocumentWrite);
    authority_request(
        audience_did,
        AuthorityRequestShape {
            action,
            contract: DOCUMENT_CONTRACT,
            resource: DOCUMENT_RESOURCE,
            ability: DOCUMENT_ABILITY,
            operation_profile_identity: profile_identity.as_str(),
            policy_identity: BASALT_POLICY_IDENTITY,
        },
    )
}

fn authority_request(audience_did: &str, shape: AuthorityRequestShape<'_>) -> HostAuthorityRequest {
    let AuthorityRequestShape {
        action,
        contract,
        resource,
        ability,
        operation_profile_identity,
        policy_identity,
    } = shape;
    let bounds = HostAuthorityBounds {
        identity_capacity_bytes: IDENTITY_CAPACITY_BYTES,
        resource_ceiling_bytes: RESOURCE_CEILING_BYTES,
        checked_fact_allowance: CHECKED_FACT_ALLOWANCE,
    };
    let binding = admit_agent_authority_binding(AgentAuthorityBindingDraft {
        display_label: Some("reviewer"),
        audience_did,
        agent_definition_identity: HASH_A,
        binding_policy_identity: HASH_B,
        freshness_identity: HASH_C,
        subject_identity: HASH_A,
        task_identity: HASH_B,
        actor_identity: HASH_C,
        role_plan_identity: HASH_A,
        session_identity: HASH_B,
        freshness: BindingFreshness::Current,
        bounds,
    })
    .expect("binding");
    admit_host_authority_request(HostAuthorityRequestDraft {
        action,
        contract,
        resource,
        ability,
        binding: &binding,
        agent_definition_identity: HASH_A,
        subject_identity: HASH_A,
        task_identity: HASH_B,
        actor_identity: HASH_C,
        role_plan_identity: HASH_A,
        session_identity: HASH_B,
        policy_identity,
        grant_identity: HASH_A,
        operation_profile_identity,
        correlation_identity: "test-dispatch",
        bounds,
    })
    .expect("request")
}

fn document_capability(resource: &str) -> CapabilityDocument {
    CapabilityDocument::new(resource.to_owned(), DOCUMENT_ABILITY.to_owned()).expect("capability")
}

fn assert_failure_reason(
    failure: &BasaltAuthorityFailure,
    expected_issue: HostAuthorityIssue,
    expected_reason: UcanEnforcementReason,
) {
    assert_eq!(failure.issue(), expected_issue);
    assert_eq!(
        failure
            .basalt_receipt()
            .expect("Basalt denial receipt")
            .reason,
        expected_reason
    );
}

fn all_gates() -> RequiredHostGates {
    RequiredHostGates {
        agent_definition: HostGateStatus::Allowed,
        semantic_role: HostGateStatus::Allowed,
        parent: HostGateStatus::Allowed,
        task: HostGateStatus::Allowed,
        model: HostGateStatus::Allowed,
        tool: HostGateStatus::Allowed,
        workspace: HostGateStatus::Allowed,
        approval: HostGateStatus::Allowed,
        sandbox: HostGateStatus::Allowed,
        cancellation: HostGateStatus::Allowed,
        timeout: HostGateStatus::Allowed,
        budget: HostGateStatus::Allowed,
        runtime_authority_seal: HostGateStatus::Allowed,
    }
}

const PROCESS_EXECUTE_ABILITY: &str = "process/execute";
const PROCESS_EXECUTE_CONTRACT: &str = "process-execute";
const EXECUTION_CONTEXT: &str = concat!(
    "onix://process-execution/",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
);
const OTHER_EXECUTION_CONTEXT: &str = concat!(
    "onix://process-execution/",
    "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
);
const EXECUTION_POLICY_JSON: &str =
    include_str!("../../../fixtures/basalt-authority/process-execution-policy.json");

fn execution_request(audience_did: &str) -> HostAuthorityRequest {
    let profile_identity = operation_profile_identity(EnforcementAction::ProcessExecute);
    authority_request(
        audience_did,
        AuthorityRequestShape {
            action: GuardedAction::Exec,
            contract: PROCESS_EXECUTE_CONTRACT,
            resource: EXECUTION_CONTEXT,
            ability: PROCESS_EXECUTE_ABILITY,
            operation_profile_identity: profile_identity.as_str(),
            policy_identity: BASALT_PROCESS_EXECUTION_POLICY_IDENTITY,
        },
    )
}

fn execution_request_with_policy(
    audience_did: &str,
    policy_identity: &str,
) -> HostAuthorityRequest {
    let profile_identity = operation_profile_identity(EnforcementAction::ProcessExecute);
    authority_request(
        audience_did,
        AuthorityRequestShape {
            action: GuardedAction::Exec,
            contract: PROCESS_EXECUTE_CONTRACT,
            resource: EXECUTION_CONTEXT,
            ability: PROCESS_EXECUTE_ABILITY,
            operation_profile_identity: profile_identity.as_str(),
            policy_identity,
        },
    )
}

fn execution_enforcement() -> EnforcementRequest {
    EnforcementRequest::new(
        PROCESS_EXECUTE_CONTRACT,
        EXECUTION_CONTEXT,
        PROCESS_EXECUTE_ABILITY,
    )
}

fn execution_policy_parts() -> (basalt::Policy, basalt::authority_core::BasaltPolicyRef) {
    let policy = basalt::parse_policy_json(EXECUTION_POLICY_JSON).expect("execution policy");
    let artifact = format!(
        "blake3:{}",
        blake3::hash(EXECUTION_POLICY_JSON.as_bytes()).to_hex()
    );
    let policy_ref = basalt::authority_core::BasaltPolicyRef::new(
        artifact,
        &policy.schema_version,
        PROCESS_EXECUTE_CONTRACT,
    );
    (policy, policy_ref)
}

fn execution_fixture() -> Fixture {
    Fixture::direct_with_capability(
        CapabilityDocument::new(
            EXECUTION_CONTEXT.to_owned(),
            PROCESS_EXECUTE_ABILITY.to_owned(),
        )
        .expect("execution capability"),
    )
}

struct ExecutionInvocation<'a> {
    fixture: &'a Fixture,
    nonce: HolderInvocationNonce,
    signature: &'a SignerSignature,
    replay: &'a RecordingReplay,
    policy: &'a basalt::Policy,
    policy_ref: &'a basalt::authority_core::BasaltPolicyRef,
}

fn execution_service<'a>(invocation: &'a ExecutionInvocation<'a>) -> BasaltAuthorityService<'a> {
    let ExecutionInvocation {
        fixture,
        nonce,
        signature,
        replay,
        policy,
        policy_ref,
    } = invocation;
    BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: fixture.token.as_str(),
        nonce: *nonce,
        holder_signature: signature,
        audience: &fixture.audience,
        policy,
        policy_ref,
        boundaries: boundaries(fixture, replay),
    })
}

#[test]
fn execution_policy_fixture_binds_the_reviewed_identity() {
    assert_eq!(
        BASALT_PROCESS_EXECUTION_POLICY_IDENTITY,
        format!(
            "b3:{}",
            blake3::hash(EXECUTION_POLICY_JSON.as_bytes()).to_hex()
        )
    );
    assert_ne!(
        BASALT_PROCESS_EXECUTION_POLICY_IDENTITY,
        BASALT_POLICY_IDENTITY
    );
}

#[test]
fn exec_mapping_requires_exact_purpose_policy_and_context_shape() {
    let audience = execution_fixture().audience;
    let request = execution_request(audience.as_str());
    assert_eq!(
        plan_basalt_request(&request).map(|plan| plan.action),
        Ok(EnforcementAction::ProcessExecute)
    );

    // The packaged default policy identity cannot authorize execution.
    let defaulted = execution_request_with_policy(audience.as_str(), BASALT_POLICY_IDENTITY);
    assert_eq!(
        plan_basalt_request(&defaulted).unwrap_err(),
        HostAuthorityIssue::UnsupportedAction
    );

    // A document-purpose ability or contract stays outside the mapping.
    let wrong_purpose = authority_request(
        audience.as_str(),
        AuthorityRequestShape {
            action: GuardedAction::Exec,
            contract: DOCUMENT_CONTRACT,
            resource: EXECUTION_CONTEXT,
            ability: DOCUMENT_ABILITY,
            operation_profile_identity: operation_profile_identity(
                EnforcementAction::ProcessExecute,
            )
            .as_str(),
            policy_identity: BASALT_PROCESS_EXECUTION_POLICY_IDENTITY,
        },
    );
    assert_eq!(
        plan_basalt_request(&wrong_purpose).unwrap_err(),
        HostAuthorityIssue::UnsupportedAction
    );

    // Wrong operation profile identity is rejected before verification.
    let wrong_profile = authority_request(
        audience.as_str(),
        AuthorityRequestShape {
            action: GuardedAction::Exec,
            contract: PROCESS_EXECUTE_CONTRACT,
            resource: EXECUTION_CONTEXT,
            ability: PROCESS_EXECUTE_ABILITY,
            operation_profile_identity: operation_profile_identity(
                EnforcementAction::DocumentWrite,
            )
            .as_str(),
            policy_identity: BASALT_PROCESS_EXECUTION_POLICY_IDENTITY,
        },
    );
    assert_eq!(
        plan_basalt_request(&wrong_profile).unwrap_err(),
        HostAuthorityIssue::UnsupportedAction
    );
}

#[test]
fn signed_execution_dispatches_once_after_gates_and_commit() {
    let fixture = execution_fixture();
    let replay = RecordingReplay::new();
    let generic = execution_request(fixture.audience.as_str());
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
    let mut commit = RecordingCommit {
        calls: 0,
        allow: true,
    };
    let effect_calls = Cell::new(0_u32);

    let dispatched = dispatch_with_basalt(
        BasaltDispatchContext {
            service: &service,
            request: &generic,
            gates: all_gates(),
            commit: &mut commit,
        },
        || {
            effect_calls.set(
                effect_calls
                    .get()
                    .checked_add(FIRST_CALL)
                    .expect("effect calls"),
            );
            Ok("process-effect")
        },
    )
    .expect("admitted execution dispatch");

    assert_eq!(dispatched.output, "process-effect");
    assert_eq!(commit.calls, FIRST_CALL);
    assert_eq!(effect_calls.get(), FIRST_CALL);
    assert_eq!(replay.calls.get(), FIRST_CALL);
    assert_eq!(dispatched.authority_receipt.dispatch_class, "committed");
    assert!(!format!("{:?}", dispatched.authority_receipt).contains(fixture.token.as_str()));
}

#[test]
fn default_policy_supplied_for_execution_is_denied_before_replay() {
    let fixture = execution_fixture();
    let replay = RecordingReplay::new();
    let generic = execution_request_with_policy(fixture.audience.as_str(), BASALT_POLICY_IDENTITY);
    let enforcement = execution_enforcement();
    let nonce = nonce();
    let signature = sign_action(
        &fixture,
        &enforcement,
        nonce,
        EnforcementAction::ProcessExecute,
    );
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

    let failure = service
        .authorize_with_evidence(&generic)
        .expect_err("default policy cannot authorize execution");
    assert_eq!(failure.issue(), HostAuthorityIssue::PolicyDenied);
    assert_eq!(replay.calls.get(), 0);
}

#[test]
fn changed_execution_context_cannot_reuse_the_holder_signature() {
    let fixture = execution_fixture();
    let replay = RecordingReplay::new();
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

    let substituted = authority_request(
        fixture.audience.as_str(),
        AuthorityRequestShape {
            action: GuardedAction::Exec,
            contract: PROCESS_EXECUTE_CONTRACT,
            resource: OTHER_EXECUTION_CONTEXT,
            ability: PROCESS_EXECUTE_ABILITY,
            operation_profile_identity: operation_profile_identity(
                EnforcementAction::ProcessExecute,
            )
            .as_str(),
            policy_identity: BASALT_PROCESS_EXECUTION_POLICY_IDENTITY,
        },
    );
    let failure = service
        .authorize_with_evidence(&substituted)
        .expect_err("changed context must invalidate the signature");
    assert_eq!(failure.issue(), HostAuthorityIssue::VerificationDenied);
    assert_eq!(replay.calls.get(), 0);
}

#[test]
fn document_capability_cannot_authorize_a_process() {
    let fixture = Fixture::direct_with_capability(
        CapabilityDocument::new(EXECUTION_CONTEXT.to_owned(), DOCUMENT_ABILITY.to_owned())
            .expect("document capability"),
    );
    let replay = RecordingReplay::new();
    let generic = execution_request(fixture.audience.as_str());
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

    let failure = service
        .authorize_with_evidence(&generic)
        .expect_err("document grant must not authorize execution");
    assert_eq!(failure.issue(), HostAuthorityIssue::VerificationDenied);
    assert_eq!(replay.calls.get(), 0);
}

#[test]
fn revoked_execution_and_replayed_nonce_never_dispatch() {
    let fixture = execution_fixture();
    let replay = RecordingReplay::new();
    let generic = execution_request(fixture.audience.as_str());
    let enforcement = execution_enforcement();
    let nonce = nonce();
    let signature = sign_action(
        &fixture,
        &enforcement,
        nonce,
        EnforcementAction::ProcessExecute,
    );
    let (policy, policy_ref) = execution_policy_parts();
    let revocations = RevocationList::from_references(vec![proof_reference(&fixture.token)]);
    let revoked = BasaltAuthorityService::new(BasaltAuthorityInvocation {
        compact_token: fixture.token.as_str(),
        nonce,
        holder_signature: &signature,
        audience: &fixture.audience,
        policy: &policy,
        policy_ref: &policy_ref,
        boundaries: UcanEnforcementBoundaries {
            verification_time: VerificationTime::from_unix_seconds(VERIFY_AT_SECONDS),
            key_resolver: &fixture.keys,
            proof_store: &fixture.proofs,
            revocations: &revocations,
            caveat_policies: &DenyUnknownCaveats,
            replay: &replay,
            verification_limits: VerificationLimits::default(),
            holder_limits: HolderInvocationLimits::default(),
        },
    });
    let revoked_failure = revoked
        .authorize_with_evidence(&generic)
        .expect_err("revoked execution grant");
    assert_eq!(
        revoked_failure.issue(),
        HostAuthorityIssue::VerificationDenied
    );
    assert_eq!(replay.calls.get(), 0);

    let invocation = ExecutionInvocation {
        fixture: &fixture,
        nonce,
        signature: &signature,
        replay: &replay,
        policy: &policy,
        policy_ref: &policy_ref,
    };
    let service = execution_service(&invocation);
    service
        .authorize_with_evidence(&generic)
        .expect("first execution admission");
    let replayed = service
        .authorize_with_evidence(&generic)
        .expect_err("replayed nonce must fail");
    assert_eq!(replayed.issue(), HostAuthorityIssue::ReplayDenied);

    // A fresh holder invocation must still be blocked by a denied local gate
    // before any effect or commit.
    let fresh_nonce = other_nonce();
    let fresh_signature = sign_action(
        &fixture,
        &enforcement,
        fresh_nonce,
        EnforcementAction::ProcessExecute,
    );
    let fresh_invocation = ExecutionInvocation {
        fixture: &fixture,
        nonce: fresh_nonce,
        signature: &fresh_signature,
        replay: &replay,
        policy: &policy,
        policy_ref: &policy_ref,
    };
    let fresh_service = execution_service(&fresh_invocation);
    let mut commit = RecordingCommit {
        calls: 0,
        allow: true,
    };
    let effect_calls = Cell::new(0_u32);
    let mut denied_gates = all_gates();
    denied_gates.sandbox = HostGateStatus::Denied;
    let local = dispatch_with_basalt(
        BasaltDispatchContext {
            service: &fresh_service,
            request: &execution_request(fixture.audience.as_str()),
            gates: denied_gates,
            commit: &mut commit,
        },
        || {
            effect_calls.set(FIRST_CALL);
            Ok(())
        },
    )
    .expect_err("local gate denial blocks execution");
    assert_eq!(local.issue(), HostAuthorityIssue::LocalGateDenied);
    assert_eq!(effect_calls.get(), 0);
}
