use super::*;

#[test]
fn malformed_manifest_fields_fail_validation() {
    let missing_description = SkillManifest::validate(&SkillManifestDraft {
        description: None,
        ..valid_draft()
    })
    .expect_err("missing description should fail");
    assert_eq!(
        missing_description,
        SkillManifestError::MissingField {
            field: DESCRIPTION_FIELD.as_str(),
        }
    );

    let overlong_chars = MAX_DESCRIPTION_CHARS.saturating_add(1);
    let long_description = alloc::string::String::from("x").repeat(overlong_chars);
    let overlong_description = SkillManifest::validate(&SkillManifestDraft {
        description: Some(long_description.as_str()),
        ..valid_draft()
    })
    .expect_err("overlong description should fail");
    assert_eq!(
        overlong_description,
        SkillManifestError::DescriptionTooLong {
            max_chars: MAX_DESCRIPTION_CHARS,
            actual_chars: overlong_chars,
        }
    );

    let unsupported_version = SkillManifest::validate(&SkillManifestDraft {
        version: Some(BAD_VERSION),
        ..valid_draft()
    })
    .expect_err("unsupported version shape should fail");
    assert_eq!(
        unsupported_version,
        SkillManifestError::UnsupportedVersion {
            value: Text::from(BAD_VERSION),
        }
    );

    let invalid_name = SkillManifest::validate(&SkillManifestDraft {
        name: Some(INVALID_NAME),
        ..valid_draft()
    })
    .expect_err("invalid skill name should fail");
    assert_eq!(
        invalid_name,
        SkillManifestError::MalformedName {
            field: NAME_FIELD.as_str(),
            value: Text::from(INVALID_NAME),
        }
    );

    let missing_identity = SkillManifest::validate(&SkillManifestDraft {
        content_identity: None,
        ..valid_draft()
    })
    .expect_err("missing content identity should fail");
    assert_eq!(
        missing_identity,
        SkillManifestError::MissingField {
            field: CONTENT_IDENTITY_FIELD.as_str(),
        }
    );

    let unsafe_identity = SkillManifest::validate(&SkillManifestDraft {
        content_identity: Some(RAW_PATH_HANDLE),
        ..valid_draft()
    })
    .expect_err("path-like content handles should fail");
    assert!(matches!(
        unsafe_identity,
        SkillManifestError::InvalidContentIdentity { .. }
    ));

    let uppercase_identity = SkillManifest::validate(&SkillManifestDraft {
        content_identity: Some(UPPERCASE_HASH),
        ..valid_draft()
    })
    .expect_err("uppercase hash should fail");
    assert!(matches!(
        uppercase_identity,
        SkillManifestError::InvalidContentIdentity { .. }
    ));

    let duplicate_tag = SkillManifest::validate(&SkillManifestDraft {
        tags: &[TAG, TAG],
        ..valid_draft()
    })
    .expect_err("duplicate tags should fail");
    assert_eq!(
        duplicate_tag,
        SkillManifestError::DuplicateTag {
            tag: Text::from(TAG),
        }
    );

    let malformed_related = SkillManifest::validate(&SkillManifestDraft {
        related_skills: &[INVALID_RELATED_NAME],
        ..valid_draft()
    })
    .expect_err("malformed related skill names should fail");
    assert_eq!(
        malformed_related,
        SkillManifestError::MalformedName {
            field: RELATED_SKILL_FIELD.as_str(),
            value: Text::from(INVALID_RELATED_NAME),
        }
    );
}

#[test]
fn catalog_diagnostics_fail_closed_for_duplicates_and_unresolved_related() {
    let first = valid_manifest();
    let duplicate = SkillManifest::validate(&SkillManifestDraft {
        content_identity: Some(SECOND_HASH),
        ..valid_draft()
    })
    .expect("duplicate descriptor should validate before cataloging");
    let unresolved = SkillManifest::validate(&SkillManifestDraft {
        name: Some(SECOND_NAME),
        content_identity: Some(SECOND_HASH),
        related_skills: &[MISSING_RELATED_NAME],
        ..valid_draft()
    })
    .expect("descriptor with unresolved relation should validate before cataloging");

    let catalog = advertise_skill_resources(&[first, duplicate, unresolved]);

    assert_eq!(catalog.summaries().len(), EXPECTED_SINGLE_ITEM);
    assert_eq!(catalog.summaries()[0].name(), SECOND_NAME);
    assert_eq!(catalog.diagnostics().len(), EXPECTED_TWO_ITEMS);
    assert!(catalog.diagnostics().iter().any(|diagnostic| {
        diagnostic.issue() == SkillResourceIssue::DuplicateSkillName
            && diagnostic.name() == NAME
            && diagnostic.related_skill().is_none()
    }));
    assert!(catalog.diagnostics().iter().any(|diagnostic| {
        diagnostic.issue() == SkillResourceIssue::UnresolvedRelatedSkill
            && diagnostic.name() == SECOND_NAME
            && diagnostic.related_skill() == Some(MISSING_RELATED_NAME)
    }));
}

#[test]
fn untrusted_or_invalid_body_loads_are_omitted() {
    let trusted = valid_manifest();
    let untrusted = untrusted_manifest();
    let manifests = [trusted, untrusted];
    let untrusted_body = [SkillBodyDraft {
        name: UNTRUSTED_NAME,
        content_identity: VALID_HASH,
        body: SELECTED_BODY,
    }];
    let untrusted_context = build_skill_disclosure_context(SkillDisclosureRequest {
        manifests: &manifests,
        loaded_bodies: &untrusted_body,
        selected_name: Some(UNTRUSTED_NAME),
    });
    assert!(untrusted_context.selected_body().is_none());

    let wrong_identity_body = [SkillBodyDraft {
        name: NAME,
        content_identity: SECOND_HASH,
        body: SELECTED_BODY,
    }];
    let wrong_identity_context = build_skill_disclosure_context(SkillDisclosureRequest {
        manifests: &manifests,
        loaded_bodies: &wrong_identity_body,
        selected_name: Some(NAME),
    });
    assert!(wrong_identity_context.selected_body().is_none());

    let empty_body = [SkillBodyDraft {
        name: NAME,
        content_identity: VALID_HASH,
        body: EMPTY_BODY,
    }];
    let empty_body_context = build_skill_disclosure_context(SkillDisclosureRequest {
        manifests: &manifests,
        loaded_bodies: &empty_body,
        selected_name: Some(NAME),
    });
    assert!(empty_body_context.selected_body().is_none());
}
