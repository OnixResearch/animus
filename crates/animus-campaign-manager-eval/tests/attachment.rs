//! Replay of the checked qualification fixtures without rerunning any trial: validation reasons,
//! duplicates, conflicts, and the checked negative fixtures.

mod support;

const NEGATIVE_FIXTURES: [(&str, &str); 5] = [
    (
        "noncanonical-whitespace",
        include_str!(
            "../../../fixtures/qualification-attachment/negative/noncanonical-whitespace.json"
        ),
    ),
    (
        "prefixed-profile-revision",
        include_str!(
            "../../../fixtures/qualification-attachment/negative/prefixed-profile-revision.json"
        ),
    ),
    (
        "redaction-host-path",
        include_str!(
            "../../../fixtures/qualification-attachment/negative/redaction-host-path.json"
        ),
    ),
    (
        "authority-field",
        include_str!("../../../fixtures/qualification-attachment/negative/authority-field.json"),
    ),
    (
        "failures-dropped-from-denominator",
        include_str!(
            "../../../fixtures/qualification-attachment/negative/failures-dropped-from-denominator.json"
        ),
    ),
];

/// The signal-greedy route's fixture keys and checked bytes.
fn greedy() -> Result<(support::SubjectKeys, &'static [u8]), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let route = animus_campaign_manager_eval::ScriptedPolicy::SignalGreedy.identity();
    Ok((
        support::qualification_keys(&set, route),
        support::GREEDY_FIXTURE.trim_end().as_bytes(),
    ))
}

#[test]
// r[verify agent_harness_runtime.qualification_attachment.replay]
// r[verify agent_harness_runtime.qualification_attachment.validator]
fn checked_fixtures_qualify_only_their_exact_subject() -> Result<(), support::Failure> {
    let set = animus_campaign_manager_eval::embedded_task_set()?;
    let (keys, greedy) = greedy()?;
    let explore_route = animus_campaign_manager_eval::ScriptedPolicy::ExploreOnly.identity();
    let explore_keys = support::qualification_keys(&set, explore_route);
    let explore = support::EXPLORE_FIXTURE.trim_end().as_bytes();
    let expected = support::expected_subject(&keys);
    let answer = animus_runtime::qualification::qualify(&[greedy, explore, greedy], &expected);
    let animus_runtime::qualification::QualificationAnswer::Qualified(qualified) = answer else {
        return Err(support::Failure::Unexpected("the fixture did not qualify"));
    };
    assert_eq!(
        qualified.qualification_ref,
        animus_runtime::qualification::attachment_identity(greedy)
    );
    assert_eq!(qualified.observed_at_ms, support::OBSERVED_AT_MS);
    let cross_profile = animus_runtime::qualification::ExpectedSubject {
        lattice_profile_revision: &explore_keys.revision,
        ..expected
    };
    assert_eq!(
        support::rejection_reason(greedy, &cross_profile),
        animus_runtime::qualification::RejectionReason::SubjectMismatch
    );
    let other_family = animus_runtime::qualification::ExpectedSubject {
        task_family: "code.repair",
        ..expected
    };
    assert_eq!(
        support::rejection_reason(greedy, &other_family),
        animus_runtime::qualification::RejectionReason::ScopeMismatch
    );
    let admission = animus_runtime::qualification::ExpectedSubject {
        required_class: animus_runtime::qualification::EvidenceClass::DeploymentAdmission,
        ..expected
    };
    assert_eq!(
        support::rejection_reason(greedy, &admission),
        animus_runtime::qualification::RejectionReason::InsufficientClass
    );
    let replay = animus_runtime::qualification::replay_attachments(&[greedy, explore, greedy]);
    assert_eq!((replay.accepted.len(), replay.duplicates), (2, 1));
    assert!(
        replay.conflicting_sources.is_empty(),
        "different routes of one job never conflict"
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.qualification_attachment.applicability]
fn changed_or_unavailable_evaluator_components_stop_qualification() -> Result<(), support::Failure>
{
    let (keys, greedy) = greedy()?;
    let expected = support::expected_subject(&keys);
    let late = animus_runtime::qualification::ExpectedSubject {
        now_ms: support::VALID_UNTIL_MS,
        ..expected
    };
    assert_eq!(
        support::rejection_reason(greedy, &late),
        animus_runtime::qualification::RejectionReason::Stale
    );
    let strengthened = animus_runtime::qualification::component_identity(
        animus_runtime::qualification::ComponentKind::Verifier,
        b"strengthened-verifier",
    );
    let changed_verifier = [animus_runtime::qualification::CurrentComponent {
        kind: animus_runtime::qualification::ComponentKind::Verifier,
        identity: Some(&strengthened),
    }];
    let verifier = animus_runtime::qualification::ExpectedSubject {
        current_components: &changed_verifier,
        ..expected
    };
    assert_eq!(
        support::rejection_reason(greedy, &verifier),
        animus_runtime::qualification::RejectionReason::Stale
    );
    let protocol_gone = [animus_runtime::qualification::CurrentComponent {
        kind: animus_runtime::qualification::ComponentKind::TaskProtocol,
        identity: None,
    }];
    let protocol = animus_runtime::qualification::ExpectedSubject {
        current_components: &protocol_gone,
        ..expected
    };
    assert_eq!(
        support::rejection_reason(greedy, &protocol),
        animus_runtime::qualification::RejectionReason::Unusable
    );
    let requires_prompt = animus_runtime::qualification::ExpectedSubject {
        required_components: &[animus_runtime::qualification::ComponentKind::Prompt],
        ..expected
    };
    assert_eq!(
        support::rejection_reason(greedy, &requires_prompt),
        animus_runtime::qualification::RejectionReason::Unusable
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.qualification_attachment.replay]
fn conflicting_claims_over_one_run_are_not_qualified() -> Result<(), support::Failure> {
    let (keys, greedy) = greedy()?;
    let decoded = animus_runtime::qualification::decode_attachment(greedy)
        .map_err(|_| support::Failure::Unexpected("fixture decodes"))?;
    let mut altered = decoded.document;
    if let Some(outcomes) = altered.outcomes.as_mut() {
        outcomes.accepted = 2;
        outcomes.rejected = 1;
    }
    let altered = animus_runtime::qualification::encode_attachment(&altered)
        .map_err(|_| support::Failure::Unexpected("altered encodes"))?;
    assert_ne!(altered.identity, decoded.identity);
    assert_eq!(
        animus_runtime::qualification::qualify(
            &[greedy, &altered.canonical],
            &support::expected_subject(&keys)
        ),
        animus_runtime::qualification::QualificationAnswer::NotQualified(
            animus_runtime::qualification::RejectionReason::Conflicting
        )
    );
    Ok(())
}

#[test]
// r[verify agent_harness_runtime.qualification_attachment.redaction]
fn checked_negative_fixtures_are_malformed() {
    for (name, text) in NEGATIVE_FIXTURES {
        let rejection =
            animus_runtime::qualification::decode_attachment(text.trim_end().as_bytes());
        assert_eq!(
            rejection.map_err(|rejection| rejection.reason).err(),
            Some(animus_runtime::qualification::RejectionReason::Malformed),
            "{name}"
        );
    }
    let greedy = support::GREEDY_FIXTURE.trim_end();
    for forbidden in ['/', ' ', '\\'] {
        assert!(!greedy.contains(forbidden), "{forbidden:?}");
    }
}
