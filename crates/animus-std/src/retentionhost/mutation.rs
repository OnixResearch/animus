pub(super) fn apply<Value>(
    state: &mut super::storage::State<Value>,
    pending: super::storage::Pending<Value>,
    disposition: animus_core::retention::InsertDisposition,
    release: &mut super::storage::ReleaseBatch<Value>,
) -> Result<(), super::StoreError> {
    debug_assert!(release.keys.len() <= state.slots.len());
    debug_assert!(release.values.len() <= state.slots.len());
    match disposition {
        animus_core::retention::InsertDisposition::DoNotRetain => {
            release.values.push(pending.value);
        }
        animus_core::retention::InsertDisposition::UseFreeSlot(slot) => {
            let slot = usize::try_from(slot).map_err(|_| super::StoreError::StateInvariant)?;
            let target = state
                .slots
                .get_mut(slot)
                .ok_or(super::StoreError::StateInvariant)?;
            if target.is_some() {
                return Err(super::StoreError::StateInvariant);
            }
            *target = Some(pending.into_entry());
        }
        animus_core::retention::InsertDisposition::EvictSlot(slot) => {
            let prior = super::storage::replace(state, slot, pending)?;
            release.keys.push(prior.key);
            release.values.push(prior.value);
        }
    }
    Ok(())
}
