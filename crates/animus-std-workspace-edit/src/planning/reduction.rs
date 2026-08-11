/// Reduce reviewed Durable File Publication observations to Animus-owned states.
///
/// # Errors
///
/// Returns [`crate::model::HostIssue::InvalidPublication`] when the mechanism reports
/// no finite terminal observation stream.
// r[impl agent_harness_runtime.hashed_edit_host.publication]
pub fn reduce_publication(
    classification: &durable_file_publication::core::Classification,
) -> Result<crate::model::PublicationOutcome, crate::model::HostIssue> {
    if classification.observations.is_empty() {
        return Err(crate::model::HostIssue::InvalidPublication);
    }
    let observation_count = u32::try_from(classification.observations.len())
        .map_err(|_| crate::model::HostIssue::InvalidPublication)?;
    let (state, durability) = match classification.disposition {
        durable_file_publication::core::PublicationDisposition::NotCommitted { .. }
        | durable_file_publication::core::PublicationDisposition::DestinationExists { .. } => (
            crate::model::PublicationState::FailedBeforePublication,
            crate::model::DurabilityState::NotPublished,
        ),
        durable_file_publication::core::PublicationDisposition::CommittedVisibilityOnly => (
            crate::model::PublicationState::Published,
            crate::model::DurabilityState::VisibilityOnly,
        ),
        durable_file_publication::core::PublicationDisposition::CommittedAndParentSynchronized => (
            crate::model::PublicationState::DurabilityComplete,
            crate::model::DurabilityState::Complete,
        ),
        durable_file_publication::core::PublicationDisposition::CommittedDurabilityUnknown => (
            crate::model::PublicationState::UncertainAfterPublication,
            crate::model::DurabilityState::Unknown,
        ),
    };
    debug_assert!(observation_count > 0);
    Ok(crate::model::PublicationOutcome {
        state,
        durability,
        observation_count,
    })
}
