use super::core::{error, hash_fields, is_blake3_identity, validate_identity};
use super::data::{
    ChangeHandoffError, ChangeHandoffIssue, ChangeScope, ChangeScopeDraft, ChangeScopeHunk,
    ChangeScopeHunkDraft, scope_file_bound, scope_hunk_bound, scope_label_char_bound,
    scope_obligation_bound,
};

const SCOPE_IDENTITY_DOMAIN: &str = "animus.change-scope-handoff.scope.v2";

// r[impl animus.change_scope_handoff.scope]
/// Validates one proposed change scope into a typed immutable fact.
///
/// # Errors
///
/// Returns a typed issue when any bound file, hunk, driving-task, or
/// validation-obligation fact is omitted, over bound, or inconsistent.
pub fn validate_change_scope(
    draft: ChangeScopeDraft<'_>,
) -> Result<ChangeScope, ChangeHandoffError> {
    validate_driving_task(draft.driving_task)?;
    if draft.files.is_empty() || draft.files.len() > scope_file_bound() {
        return Err(error(ChangeHandoffIssue::MissingScopeFiles));
    }
    if draft.hunks.is_empty() || draft.hunks.len() > scope_hunk_bound() {
        return Err(error(ChangeHandoffIssue::MissingScopeHunks));
    }
    if draft.obligations.is_empty() || draft.obligations.len() > scope_obligation_bound() {
        return Err(error(ChangeHandoffIssue::MissingScopeObligations));
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
            return Err(error(ChangeHandoffIssue::UndeclaredScopeFile));
        }
    }
    let scope = ChangeScope {
        scope_identity: String::new(),
        driving_task: String::from(draft.driving_task),
        files: draft.files.iter().map(|file| String::from(*file)).collect(),
        hunks: draft
            .hunks
            .iter()
            .map(|hunk| ChangeScopeHunk {
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
    let mut scoped = scope;
    scoped.scope_identity = scope_identity(&scoped);
    debug_assert!(is_blake3_identity(scoped.scope_identity.as_str()));
    Ok(scoped)
}

fn validate_hunk(hunk: &ChangeScopeHunkDraft<'_>) -> Result<(), ChangeHandoffError> {
    validate_fact_label(hunk.file_path)?;
    validate_identity(hunk.hunk_identity)
}

fn validate_label(label: &str) -> Result<(), ChangeHandoffError> {
    if label.len() > scope_label_char_bound() {
        return Err(error(ChangeHandoffIssue::LabelBoundExceeded));
    }
    Ok(())
}

fn validate_driving_task(label: &str) -> Result<(), ChangeHandoffError> {
    if label.is_empty() {
        return Err(error(ChangeHandoffIssue::MissingDrivingTask));
    }
    validate_label(label)
}

fn validate_fact_label(label: &str) -> Result<(), ChangeHandoffError> {
    if label.is_empty() {
        return Err(error(ChangeHandoffIssue::EmptyScopeLabel));
    }
    validate_label(label)
}

fn scope_identity(scope: &ChangeScope) -> String {
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
    hash_fields(SCOPE_IDENTITY_DOMAIN, fields.as_slice())
}
