const SCOPE_IDENTITY_DOMAIN: &str = "animus.change-scope-handoff.scope.v2";

// r[impl animus.change_scope_handoff.scope]
/// Validates one proposed change scope into a typed immutable fact.
///
/// # Errors
///
/// Returns a typed issue for missing, inconsistent, or overbound scope facts.
pub fn validate_change_scope(
    draft: super::data::ChangeScopeDraft<'_>,
) -> Result<super::data::ChangeScope, super::data::ChangeHandoffError> {
    validate_driving_task(draft.driving_task)?;
    if draft.files.is_empty()
        || !super::digest::is_within_bound(draft.files.len(), super::data::scope_file_bound())
    {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::MissingScopeFiles,
        ));
    }
    if draft.hunks.is_empty()
        || !super::digest::is_within_bound(draft.hunks.len(), super::data::scope_hunk_bound())
    {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::MissingScopeHunks,
        ));
    }
    if draft.obligations.is_empty()
        || !super::digest::is_within_bound(
            draft.obligations.len(),
            super::data::scope_obligation_bound(),
        )
    {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::MissingScopeObligations,
        ));
    }
    for file in draft.files {
        validate_fact_label(file)?;
    }
    for obligation in draft.obligations {
        validate_fact_label(obligation)?;
    }
    for hunk in draft.hunks {
        validate_hunk(hunk)?;
        if !draft.files.contains(&hunk.file_path) {
            return Err(super::digest::error(
                super::data::ChangeHandoffIssue::UndeclaredScopeFile,
            ));
        }
    }
    let mut scope = own_scope(draft);
    scope.scope_identity = scope_identity(&scope)?;
    debug_assert!(super::digest::is_blake3_identity(
        scope.scope_identity.as_str()
    ));
    debug_assert_eq!(scope.hunks.len(), draft.hunks.len());
    Ok(scope)
}

fn own_scope(draft: super::data::ChangeScopeDraft<'_>) -> super::data::ChangeScope {
    let scope = super::data::ChangeScope {
        scope_identity: String::new(),
        driving_task: String::from(draft.driving_task),
        files: draft.files.iter().map(|file| String::from(*file)).collect(),
        hunks: draft
            .hunks
            .iter()
            .map(|hunk| super::data::ChangeScopeHunk {
                file_path: String::from(hunk.file_path),
                hunk_identity: String::from(hunk.hunk_identity),
                old_start: hunk.old_start,
                old_lines: hunk.old_lines,
                new_start: hunk.new_start,
                new_lines: hunk.new_lines,
            })
            .collect(),
        obligations: draft
            .obligations
            .iter()
            .map(|obligation| String::from(*obligation))
            .collect(),
    };
    debug_assert_eq!(scope.files.len(), draft.files.len());
    debug_assert_eq!(scope.obligations.len(), draft.obligations.len());
    scope
}

fn validate_hunk(
    hunk: &super::data::ChangeScopeHunkDraft<'_>,
) -> Result<(), super::data::ChangeHandoffError> {
    validate_fact_label(hunk.file_path)?;
    super::digest::validate_identity(hunk.hunk_identity)
}

fn validate_label(label: &str) -> Result<(), super::data::ChangeHandoffError> {
    if !super::digest::is_within_bound(label.len(), super::data::scope_label_char_bound()) {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::LabelBoundExceeded,
        ));
    }
    Ok(())
}

fn validate_driving_task(label: &str) -> Result<(), super::data::ChangeHandoffError> {
    if label.is_empty() {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::MissingDrivingTask,
        ));
    }
    validate_label(label)
}

fn validate_fact_label(label: &str) -> Result<(), super::data::ChangeHandoffError> {
    if label.is_empty() {
        return Err(super::digest::error(
            super::data::ChangeHandoffIssue::EmptyScopeLabel,
        ));
    }
    validate_label(label)
}

fn scope_identity(
    scope: &super::data::ChangeScope,
) -> Result<String, super::data::ChangeHandoffError> {
    debug_assert!(!scope.files.is_empty());
    debug_assert!(
        scope
            .hunks
            .iter()
            .all(|hunk| scope.files.contains(&hunk.file_path))
    );
    let mut owned: Vec<Vec<u8>> = vec![
        super::data::CHANGE_SCOPE_HANDOFF_CONTRACT_IDENTITY
            .as_bytes()
            .to_vec(),
        scope.driving_task.as_bytes().to_vec(),
        scope.files.len().to_string().into_bytes(),
    ];
    for file in &scope.files {
        owned.push(file.as_bytes().to_vec());
    }
    owned.push(scope.obligations.len().to_string().into_bytes());
    for obligation in &scope.obligations {
        owned.push(obligation.as_bytes().to_vec());
    }
    owned.push(scope.hunks.len().to_string().into_bytes());
    for hunk in &scope.hunks {
        owned.push(hunk.file_path.as_bytes().to_vec());
        owned.push(hunk.hunk_identity.as_bytes().to_vec());
        owned.push(hunk.old_start.to_le_bytes().to_vec());
        owned.push(hunk.old_lines.to_le_bytes().to_vec());
        owned.push(hunk.new_start.to_le_bytes().to_vec());
        owned.push(hunk.new_lines.to_le_bytes().to_vec());
    }
    let fields: Vec<&[u8]> = owned.iter().map(Vec::as_slice).collect();
    super::digest::hash_fields(SCOPE_IDENTITY_DOMAIN, fields.as_slice())
}
