#[derive(Clone, Copy)]
pub(super) struct QueueGuard<'a> {
    pub(super) generation: u64,
    pub(super) identity: &'a str,
}

pub(super) fn admitted_input(
    body: &str,
) -> animus_runtime::coding_session_host::CodingSessionInput {
    animus_runtime::coding_session_host::CodingSessionInput {
        body: String::from(body),
        posture:
            animus_runtime::coding_session_host::CodingSessionInputPosture::ExpandedAndAdmitted,
    }
}

pub(super) fn require_control_label(value: &str) -> Result<(), crate::LiveSteelAgentError> {
    const MAX_CONTROL_LABEL_BYTES: usize = 256;
    if value.is_empty()
        || value.len() > MAX_CONTROL_LABEL_BYTES
        || value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control() || byte.is_ascii_whitespace())
    {
        return Err(crate::LiveSteelAgentError::Tool);
    }
    Ok(())
}

pub(super) fn require_blake3_identity(identity: &str) -> Result<(), crate::LiveSteelAgentError> {
    const BLAKE3_HEX_LENGTH: usize = 64;
    let is_valid = identity.len() == BLAKE3_HEX_LENGTH
        && identity
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
    if !is_valid {
        return Err(crate::LiveSteelAgentError::Session);
    }
    Ok(())
}
