use std::path::Path;

use crate::core::load_generated_profile;
use crate::model::{
    ExecutableFacts, GuardrailError, GuardrailErrorKind, GuardrailProfile, PreflightRequest,
};
use crate::transport::observe_executable;

const FIELD_PROFILE: &str = "profile-file";
const FIELD_REQUEST: &str = "request-file";
const FIELD_EXECUTABLE: &str = "executable";

/// Loads one explicit checked profile projection without discovery.
///
/// # Errors
///
/// Returns [`GuardrailError`] when the explicit file cannot be read or fails profile validation.
pub fn load_profile_file(path: &Path) -> Result<GuardrailProfile, GuardrailError> {
    let json = std::fs::read_to_string(path)
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Profile, FIELD_PROFILE))?;
    load_generated_profile(json.as_str())
}

/// Loads one explicit active-context request without discovery.
///
/// # Errors
///
/// Returns [`GuardrailError`] when the explicit file cannot be read or has unknown/malformed data.
pub fn load_request_file(path: &Path) -> Result<PreflightRequest, GuardrailError> {
    let json = std::fs::read_to_string(path)
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Cli, FIELD_REQUEST))?;
    serde_json::from_str(json.as_str())
        .map_err(|_error| GuardrailError::new(GuardrailErrorKind::Cli, FIELD_REQUEST))
}

/// Re-observes an explicit Bubblewrap path and requires exact expected identity/cohort agreement.
///
/// # Errors
///
/// Returns [`GuardrailError`] for probe failure or expected/observed identity drift.
pub fn observe_and_match_executable(
    expected: &ExecutableFacts,
    path: &Path,
) -> Result<ExecutableFacts, GuardrailError> {
    let observed = observe_executable(path)?;
    if observed.identity == expected.identity
        && observed.cohort == expected.cohort
        && observed.args_supported == expected.args_supported
    {
        Ok(observed)
    } else {
        Err(GuardrailError::new(
            GuardrailErrorKind::Executable,
            FIELD_EXECUTABLE,
        ))
    }
}
