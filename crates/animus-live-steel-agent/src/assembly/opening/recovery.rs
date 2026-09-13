//! Recovery can read a session but cannot synthesize missing tool results.

pub(super) struct ReadOnly;

impl animus_std::coding_session_host::CodingSessionEffectGate for ReadOnly {
    fn admit(
        &mut self,
        effect: &animus_runtime::coding_session_host::CodingSessionEffectRequest,
    ) -> Result<(), animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        if matches!(
            effect,
            animus_runtime::coding_session_host::CodingSessionEffectRequest::Recovery(_)
        ) {
            Ok(())
        } else {
            Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied)
        }
    }
}
