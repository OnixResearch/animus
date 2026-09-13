#[test]
fn explicit_empty_retains_the_default_empty_head() {
    let explicit = super::MemoryCodingSessionPersistence::empty();
    let compatible = super::MemoryCodingSessionPersistence::default();
    assert!(explicit.messages.is_empty());
    assert_eq!(explicit.messages, compatible.messages);
    assert_eq!(explicit.head_hash, compatible.head_hash);
    assert_eq!(
        explicit.head_hash,
        animus_runtime::coding_session_host::CODING_SESSION_EMPTY_HEAD_HASH,
    );
}

#[test]
fn an_explicit_memory_store_commits_and_reopens_without_provider_contact() {
    let services = super::services_with_persistence(super::MemoryCodingSessionPersistence::empty());
    let mut host = super::CodingSessionHost::new(super::new_state(false), services)
        .expect("valid empty memory host");
    host.prompt(
        super::TEST_RUN,
        super::TEST_USER_ENTRY,
        super::input(super::TEST_PROMPT),
    )
    .expect("valid memory turn");
    let messages = host.message_snapshot().to_vec();
    assert_eq!(messages.len(), super::EXPECTED_SINGLE_TURN_MESSAGES);
    let committed = host.into_services();
    assert_eq!(
        committed.provider.dispatch_count,
        super::EXPECTED_SINGLE_PROVIDER_DISPATCH
    );
    let services = super::services_with_persistence(committed.persistence);
    let mut resumed = super::CodingSessionHost::new(super::new_state(true), services)
        .expect("valid memory recovery host");
    let mut gate = RecoveryOnly { reads: 0 };
    resumed
        .resume_with_gate(&mut gate)
        .expect("committed memory recovery");
    assert_eq!(resumed.message_snapshot(), messages);
    assert_eq!(gate.reads, 1);
    let restored = resumed.into_services();
    assert_eq!(restored.provider.dispatch_count, 0);
    assert_eq!(restored.authority.observation_count, 1);
}

struct RecoveryOnly {
    reads: u32,
}

impl super::CodingSessionEffectGate for RecoveryOnly {
    fn admit(
        &mut self,
        effect: &animus_runtime::coding_session_host::CodingSessionEffectRequest,
    ) -> Result<(), animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        if matches!(
            effect,
            animus_runtime::coding_session_host::CodingSessionEffectRequest::Recovery(_)
        ) {
            self.reads = self
                .reads
                .checked_add(1)
                .ok_or(animus_runtime::coding_session_host::CodingSessionServiceIssue::Failed)?;
            Ok(())
        } else {
            Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied)
        }
    }
}

struct DeniedRecovery;

impl super::CodingSessionEffectGate for DeniedRecovery {
    fn admit(
        &mut self,
        _effect: &animus_runtime::coding_session_host::CodingSessionEffectRequest,
    ) -> Result<(), animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied)
    }
}

#[test]
fn a_denied_resume_cannot_become_authorized_or_change_the_store() {
    let services = super::services_with_persistence(super::MemoryCodingSessionPersistence::empty());
    let mut host = super::CodingSessionHost::new(super::new_state(true), services)
        .expect("valid recovery host");
    let error = host
        .resume_with_gate(&mut DeniedRecovery)
        .expect_err("denied recovery");
    assert_eq!(error.issue(), super::CodingSessionHostIssue::SessionBlocked);
    let rejected = host.into_services();
    assert_eq!(rejected.provider.dispatch_count, 0);
    assert_eq!(rejected.authority.observation_count, 0);
    assert!(rejected.persistence.messages.is_empty());
    assert_eq!(
        rejected.persistence.head_hash,
        animus_runtime::coding_session_host::CODING_SESSION_EMPTY_HEAD_HASH,
    );
}

#[test]
fn a_rejected_prompt_preserves_the_explicit_empty_store() {
    let services = super::services_with_persistence(super::MemoryCodingSessionPersistence::empty());
    let mut host = super::CodingSessionHost::new(super::new_state(false), services)
        .expect("valid empty memory host");
    let error = host
        .prompt(super::TEST_RUN, super::TEST_USER_ENTRY, super::input(""))
        .expect_err("empty prompt must fail before commit or provider contact");
    assert_eq!(
        error.issue(),
        super::CodingSessionHostIssue::ReducerRejected
    );
    assert!(host.message_snapshot().is_empty());
    let rejected = host.into_services();
    assert_eq!(rejected.provider.dispatch_count, 0);
    assert!(rejected.persistence.messages.is_empty());
    assert_eq!(
        rejected.persistence.head_hash,
        animus_runtime::coding_session_host::CODING_SESSION_EMPTY_HEAD_HASH,
    );
}
