use crate::config::{PueueHostConfigError, PueueHostConfigIssue};

pub(super) fn map_json_error(error: &serde_json::Error) -> PueueHostConfigError {
    let issue = if error.is_syntax() || error.is_eof() {
        PueueHostConfigIssue::Json
    } else {
        PueueHostConfigIssue::Shape
    };
    PueueHostConfigError::new(issue)
}

pub(super) const fn invalid_value() -> PueueHostConfigError {
    PueueHostConfigError::new(PueueHostConfigIssue::Value)
}

pub(super) const fn invalid_profile() -> PueueHostConfigError {
    PueueHostConfigError::new(PueueHostConfigIssue::Profile)
}
