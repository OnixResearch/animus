impl<Value> super::Store<Value> {
    /// Builds a store only when capacity and all policy values are explicit.
    ///
    /// # Errors
    ///
    /// Returns [`super::StoreError::MissingCapacity`] when capacity is absent.
    /// Returns a policy error for invalid watermarks or promotion threshold.
    pub fn new(config: super::Config) -> Result<Self, super::StoreError> {
        let policy = animus_core::retention::Policy::new(
            config.capacity.ok_or(super::StoreError::MissingCapacity)?,
            config.low_watermark,
            config.high_watermark,
            config.promotion_threshold_percent,
        )?;
        let slot_count =
            usize::try_from(policy.capacity()).map_err(|_| super::StoreError::StateInvariant)?;
        let slots = std::iter::repeat_with(|| None).take(slot_count).collect();
        Ok(Self {
            policy,
            state: std::sync::Arc::new(std::sync::Mutex::new(super::storage::State {
                slots,
                generation: 0,
            })),
        })
    }

    /// Returns the explicit capacity.
    #[must_use]
    pub const fn capacity(&self) -> u32 {
        self.policy.capacity()
    }

    /// Returns the current retained entry count.
    ///
    /// # Errors
    ///
    /// Returns [`super::StoreError::LockPoisoned`] after a panicking holder.
    pub fn len(&self) -> Result<u32, super::StoreError> {
        let state = self.lock()?;
        super::storage::active_count(&state)
    }

    /// Returns true when no entry is retained.
    ///
    /// # Errors
    ///
    /// Returns [`super::StoreError::LockPoisoned`] after a panicking holder.
    pub fn is_empty(&self) -> Result<bool, super::StoreError> {
        self.len().map(|length_entries| length_entries == 0)
    }

    /// Looks up one digest and applies the core promotion decision.
    ///
    /// # Errors
    ///
    /// Returns an error when synchronization or generation facts fail.
    pub fn get(
        &self,
        key: &animus_core::retention::Digest,
    ) -> Result<Option<std::sync::Arc<Value>>, super::StoreError> {
        let mut state = self.lock()?;
        let Some(slot) = state
            .slots
            .iter()
            .position(|candidate| candidate.as_ref().is_some_and(|entry| entry.key == *key))
        else {
            return Ok(None);
        };
        let generation = super::storage::next_generation(&mut state)?;
        let entry = state.slots[slot]
            .as_mut()
            .ok_or(super::StoreError::StateInvariant)?;
        let age_percent = super::storage::relative_age_percent(
            generation.saturating_sub(entry.promotion_generation),
            self.policy.capacity(),
        )?;
        if animus_core::retention::decide_promotion(
            self.policy,
            animus_core::retention::PromotionFacts { age_percent },
        )? {
            entry.promotion_generation = generation;
        }
        Ok(Some(std::sync::Arc::clone(&entry.value)))
    }

    /// Inserts a value and releases replaced or trimmed values after unlocking.
    ///
    /// # Errors
    ///
    /// Returns an error for invalid synchronized state.
    pub fn insert(
        &self,
        key: animus_core::retention::Digest,
        value: Value,
    ) -> Result<super::InsertOutcome, super::StoreError> {
        self.insert_shared(key, std::sync::Arc::new(value))
    }

    /// Returns a retained value or invokes the supplied adapter on a miss.
    ///
    /// Adapter execution occurs outside the cache guard. A hit is advisory and
    /// does not prove tool correctness, freshness, or authority.
    ///
    /// # Errors
    ///
    /// Returns [`super::LoadError::Store`] for store failures and
    /// [`super::LoadError::Adapter`] for adapter failures.
    pub fn get_or_try_insert_with<AdapterError, Load>(
        &self,
        key: animus_core::retention::Digest,
        load: Load,
    ) -> Result<super::Access<Value>, super::LoadError<AdapterError>>
    where
        Load: FnOnce() -> Result<Value, AdapterError>,
    {
        if let Some(value) = self.get(&key).map_err(super::LoadError::Store)? {
            return Ok(super::Access::Hit(value));
        }
        let value = std::sync::Arc::new(load().map_err(super::LoadError::Adapter)?);
        self.insert_shared(key, std::sync::Arc::clone(&value))
            .map_err(super::LoadError::Store)?;
        Ok(super::Access::Miss(value))
    }

    fn insert_shared(
        &self,
        key: animus_core::retention::Digest,
        value: std::sync::Arc<Value>,
    ) -> Result<super::InsertOutcome, super::StoreError> {
        let (outcome, deferred_release) = {
            let mut state = self.lock()?;
            let generation = super::storage::next_generation(&mut state)?;
            let pending = super::storage::Pending {
                key,
                value,
                generation,
            };
            if let Some(slot) = occupied_slot(&state, key) {
                replace_existing(&mut state, slot, pending)?
            } else {
                self.insert_new(&mut state, pending)?
            }
        };
        drop(deferred_release);
        Ok(outcome)
    }

    fn insert_new(
        &self,
        state: &mut super::storage::State<Value>,
        pending: super::storage::Pending<Value>,
    ) -> Result<(super::InsertOutcome, std::vec::Vec<std::sync::Arc<Value>>), super::StoreError>
    {
        let free_slots = free_slots(state)?;
        let entries = super::storage::policy_entries(state)?;
        let insertion = animus_core::retention::plan_insert(
            self.policy,
            super::storage::active_count(state)?,
            &free_slots,
            &entries,
        )?;
        let mut release = super::storage::ReleaseBatch::new();
        super::mutation::apply(state, pending, insertion.disposition, &mut release)?;
        self.apply_trim(state, &mut release)?;
        let outcome = if self.policy.capacity() == 0 {
            super::InsertOutcome::NotRetained
        } else {
            super::InsertOutcome::Inserted {
                evicted_keys: release.keys,
            }
        };
        Ok((outcome, release.values))
    }

    fn apply_trim(
        &self,
        state: &mut super::storage::State<Value>,
        release: &mut super::storage::ReleaseBatch<Value>,
    ) -> Result<(), super::StoreError> {
        let trim =
            animus_core::retention::plan_trim(self.policy, super::storage::active_count(state)?)?;
        for _release_index in 0..trim.release_count {
            let entries = super::storage::policy_entries(state)?;
            let slot = animus_core::retention::select_oldest(&entries)
                .ok_or(super::StoreError::StateInvariant)?;
            let slot = usize::try_from(slot).map_err(|_| super::StoreError::StateInvariant)?;
            let entry = state
                .slots
                .get_mut(slot)
                .and_then(Option::take)
                .ok_or(super::StoreError::StateInvariant)?;
            release.keys.push(entry.key);
            release.values.push(entry.value);
        }
        Ok(())
    }

    fn lock(
        &self,
    ) -> Result<std::sync::MutexGuard<'_, super::storage::State<Value>>, super::StoreError> {
        self.state
            .lock()
            .map_err(|_poisoned| super::StoreError::LockPoisoned)
    }
}

fn occupied_slot<Value>(
    state: &super::storage::State<Value>,
    key: animus_core::retention::Digest,
) -> Option<usize> {
    state
        .slots
        .iter()
        .position(|candidate| candidate.as_ref().is_some_and(|entry| entry.key == key))
}

fn replace_existing<Value>(
    state: &mut super::storage::State<Value>,
    slot: usize,
    pending: super::storage::Pending<Value>,
) -> Result<(super::InsertOutcome, std::vec::Vec<std::sync::Arc<Value>>), super::StoreError> {
    let prior = state.slots[slot].replace(pending.into_entry());
    let prior = prior.ok_or(super::StoreError::StateInvariant)?;
    Ok((super::InsertOutcome::Replaced, std::vec![prior.value]))
}

fn free_slots<Value>(
    state: &super::storage::State<Value>,
) -> Result<std::vec::Vec<u32>, super::StoreError> {
    state
        .slots
        .iter()
        .enumerate()
        .filter_map(|(slot, entry)| entry.is_none().then_some(slot))
        .map(|slot| u32::try_from(slot).map_err(|_| super::StoreError::StateInvariant))
        .collect()
}
