pub(super) fn build(
    admission: &animus_runtime::child_session_service::ConcreteChildSessionAdmission,
    response: String,
    restore: bool,
) -> Result<
    Box<dyn animus_std::child_session_host::ChildCodingSessionController>,
    animus_std::child_session_host::ChildSessionHostFailure,
> {
    let state = child_state(admission, restore)?;
    let host = animus_std::coding_session_host::CodingSessionHost::new(
        state,
        animus_std::coding_session_host::CodingSessionHostServices {
            provider: animus_std::coding_session_host::FakeCodingSessionProvider {
                response,
                dispatch_count: 0,
            },
            tool: animus_std::coding_session_host::DenyUnusedCodingSessionTool,
            persistence: animus_std::coding_session_host::MemoryCodingSessionPersistence::default(),
            policy: animus_std::coding_session_host::AllowCodingSessionPolicy,
            authority: animus_std::coding_session_host::AdmittedCodingSessionAuthority {
                observation_count: 0,
            },
            cancellation: animus_std::coding_session_host::ObservedCodingSessionCancellation,
            timer: animus_std::coding_session_host::AvailableCodingSessionTimer,
            summarizer: animus_std::coding_session_host::FakeCodingSessionSummarizer {
                summary: String::from(super::CHILD_SUMMARY),
            },
            events: animus_std::coding_session_host::NoopCodingSessionEventSink,
        },
    )
    .map_err(|_error| child_session_failure("child coding-session host is unavailable"))?;
    let run_id = format!(
        "{}{}",
        super::CHILD_RUN_PREFIX,
        admission.admission_identity()
    );
    let entry_id = format!(
        "{}{}",
        super::CHILD_ENTRY_PREFIX,
        admission.admission_identity()
    );
    debug_assert!(run_id.starts_with(super::CHILD_RUN_PREFIX));
    debug_assert!(entry_id.starts_with(super::CHILD_ENTRY_PREFIX));
    let controller = if restore {
        animus_std::child_session_host::NormalCodingSessionController::restore(
            host, run_id, entry_id,
        )
        .map_err(|_error| child_session_failure("child coding-session restore failed"))?
    } else {
        animus_std::child_session_host::NormalCodingSessionController::new(host, run_id, entry_id)
    };
    Ok(Box::new(controller))
}

pub(super) fn child_session_failure(
    message: &'static str,
) -> animus_std::child_session_host::ChildSessionHostFailure {
    animus_std::child_session_host::ChildSessionHostFailure::new(
        animus_runtime::child_session_service::ChildSessionFailureClass::Session,
        message,
    )
}

fn child_state(
    admission: &animus_runtime::child_session_service::ConcreteChildSessionAdmission,
    is_restore: bool,
) -> Result<
    animus_runtime::coding_session_host::CodingSessionState,
    animus_std::child_session_host::ChildSessionHostFailure,
> {
    let components = animus_runtime::session_authority_profile::SessionAuthorityComponentClass::ALL
        .iter()
        .map(
            |class| animus_runtime::session_authority_profile::SessionAuthorityComponentDraft {
                class: *class,
                identity: admission.parent_authority_identity(),
            },
        )
        .collect::<Vec<_>>();
    let authority_profile =
        animus_runtime::session_authority_profile::build_session_authority_profile(
            animus_runtime::session_authority_profile::SessionAuthorityProfileDraft {
                components: components.as_slice(),
            },
        )
        .map_err(|_error| child_session_failure("child authority profile is malformed"))?;
    let queue_policy =
        animus_runtime::MessageQueuePolicy::new(animus_runtime::MessageQueuePolicyDraft {
            max_items: super::MAX_CHILD_QUEUE_ITEMS,
            allowed_sources: &["coding-session-host"],
            policy_label: super::CHILD_QUEUE_POLICY_LABEL,
            policy_hash: admission.admission_identity(),
            allow_queue_mutations: true,
            allow_interrupts: true,
        })
        .map_err(|_error| child_session_failure("child queue policy is malformed"))?;
    let state = animus_runtime::coding_session_host::new_coding_session_state(
        animus_runtime::coding_session_host::CodingSessionConfig {
            session_id: admission.subagent().child_id.clone(),
            branch_id: String::from(super::CHILD_BRANCH_ID),
            queue_profile:
                animus_runtime::coding_session_host::CodingSessionQueueProfile::Sequential,
            services: animus_runtime::coding_session_host::CodingSessionServiceSet::all_available(),
            authority_profile,
            authority_authorization_identity: admission.parent_authority_identity().to_owned(),
            resume_required: is_restore,
        },
        queue_policy,
    )
    .map_err(|_error| child_session_failure("child coding-session state is malformed"))?;
    debug_assert!(!admission.subagent().child_id.is_empty());
    debug_assert!(!admission.admission_identity().is_empty());
    Ok(state)
}
