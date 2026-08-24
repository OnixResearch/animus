pub(super) const AGE_PERCENT_MAX: u64 = 100;

pub(super) struct Entry<Value> {
    pub(super) key: animus_core::retention::Digest,
    pub(super) value: std::sync::Arc<Value>,
    pub(super) promotion_generation: u64,
}

pub(super) struct Pending<Value> {
    pub(super) key: animus_core::retention::Digest,
    pub(super) value: std::sync::Arc<Value>,
    pub(super) generation: u64,
}

impl<Value> Pending<Value> {
    pub(super) fn into_entry(self) -> Entry<Value> {
        Entry {
            key: self.key,
            value: self.value,
            promotion_generation: self.generation,
        }
    }
}

pub(super) struct State<Value> {
    pub(super) slots: std::vec::Vec<Option<Entry<Value>>>,
    pub(super) generation: u64,
}

pub(super) struct ReleaseBatch<Value> {
    pub(super) keys: std::vec::Vec<animus_core::retention::Digest>,
    pub(super) values: std::vec::Vec<std::sync::Arc<Value>>,
}

impl<Value> ReleaseBatch<Value> {
    pub(super) const fn new() -> Self {
        Self {
            keys: std::vec::Vec::new(),
            values: std::vec::Vec::new(),
        }
    }
}

pub(super) fn next_generation<Value>(state: &mut State<Value>) -> Result<u64, super::StoreError> {
    state.generation = state
        .generation
        .checked_add(1)
        .ok_or(super::StoreError::GenerationOverflow)?;
    Ok(state.generation)
}

pub(super) fn active_count<Value>(state: &State<Value>) -> Result<u32, super::StoreError> {
    let count_entries = state.slots.iter().flatten().count();
    u32::try_from(count_entries).map_err(|_| super::StoreError::StateInvariant)
}

pub(super) fn policy_entries<Value>(
    state: &State<Value>,
) -> Result<std::vec::Vec<animus_core::retention::Entry>, super::StoreError> {
    state
        .slots
        .iter()
        .enumerate()
        .filter_map(|(slot, entry)| entry.as_ref().map(|entry| (slot, entry)))
        .map(|(slot, entry)| {
            let slot = u32::try_from(slot).map_err(|_| super::StoreError::StateInvariant)?;
            Ok(animus_core::retention::Entry {
                slot,
                promotion_generation: entry.promotion_generation,
            })
        })
        .collect()
}

pub(super) fn replace<Value>(
    state: &mut State<Value>,
    slot: u32,
    pending: Pending<Value>,
) -> Result<Entry<Value>, super::StoreError> {
    let slot = usize::try_from(slot).map_err(|_| super::StoreError::StateInvariant)?;
    let target = state
        .slots
        .get_mut(slot)
        .ok_or(super::StoreError::StateInvariant)?;
    target
        .replace(pending.into_entry())
        .ok_or(super::StoreError::StateInvariant)
}

pub(super) fn relative_age_percent(
    distance_generations: u64,
    capacity_entries: u32,
) -> Result<u8, super::StoreError> {
    if capacity_entries == 0 {
        return Err(super::StoreError::StateInvariant);
    }
    let age_percent = distance_generations
        .saturating_mul(AGE_PERCENT_MAX)
        .checked_div(u64::from(capacity_entries))
        .ok_or(super::StoreError::StateInvariant)?
        .min(AGE_PERCENT_MAX);
    u8::try_from(age_percent).map_err(|_| super::StoreError::StateInvariant)
}
