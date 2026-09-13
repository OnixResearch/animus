/// Metadata-only in-memory lifecycle records owned by one live run.
#[derive(Clone, Default)]
pub struct MetadataChildStore {
    records: std::collections::BTreeMap<
        String,
        animus_runtime::child_session_service::ChildSessionLifecycleRecord,
    >,
}

impl animus_std::child_session_host::ChildSessionLifecycleStore for MetadataChildStore {
    fn persist(
        &mut self,
        _service_id: &str,
        record: &animus_runtime::child_session_service::ChildSessionLifecycleRecord,
    ) -> Result<(), animus_std::child_session_host::ChildSessionHostFailure> {
        self.records.insert(record.child_id.clone(), record.clone());
        Ok(())
    }

    fn load(
        &mut self,
        _service_id: &str,
    ) -> Result<
        Vec<animus_runtime::child_session_service::ChildSessionLifecycleRecord>,
        animus_std::child_session_host::ChildSessionHostFailure,
    > {
        let mut records = Vec::with_capacity(self.records.len());
        records.extend(self.records.values().cloned());
        Ok(records)
    }
}

/// Fresh parent-authority observation for every child operation.
#[derive(Clone)]
pub struct ExplicitChildAuthority {
    parent_id: String,
    authority_identity: String,
    admitted: bool,
}

impl ExplicitChildAuthority {
    /// Builds current admitted authority.
    #[must_use]
    pub fn admitted(authority_identity: impl Into<String>) -> Self {
        Self {
            parent_id: String::from(super::CHILD_PARENT_ID),
            authority_identity: authority_identity.into(),
            admitted: true,
        }
    }

    /// Builds current denied authority.
    #[must_use]
    pub fn denied(authority_identity: impl Into<String>) -> Self {
        Self {
            parent_id: String::from(super::CHILD_PARENT_ID),
            authority_identity: authority_identity.into(),
            admitted: false,
        }
    }
}

impl animus_std::child_session_host::ChildSessionAuthorityService for ExplicitChildAuthority {
    fn observe(
        &mut self,
        parent_id: &str,
    ) -> Result<
        animus_std::child_session_host::ChildSessionAuthorityObservation,
        animus_std::child_session_host::ChildSessionHostFailure,
    > {
        if parent_id != self.parent_id {
            return Err(
                animus_std::child_session_host::ChildSessionHostFailure::new(
                    animus_runtime::child_session_service::ChildSessionFailureClass::Denied,
                    "child parent identity drifted",
                ),
            );
        }
        Ok(
            animus_std::child_session_host::ChildSessionAuthorityObservation {
                parent_id: String::clone(&self.parent_id),
                authority_identity: String::clone(&self.authority_identity),
                admitted: self.admitted,
            },
        )
    }
}

/// Explicit monotonic child timer.
#[derive(Clone, Copy, Default)]
pub struct FixedChildTimer {
    now_ms: u64,
}

impl animus_std::child_session_host::ChildSessionTimer for FixedChildTimer {
    fn now_ms(&mut self) -> Result<u64, animus_std::child_session_host::ChildSessionHostFailure> {
        Ok(self.now_ms)
    }
}

/// Metadata-only child event sink.
#[derive(Clone, Default)]
pub struct MetadataChildEvents {
    events: Vec<animus_std::child_session_host::ChildSessionEvent>,
    event_count: u64,
}

impl MetadataChildEvents {
    /// Returns the safe event count.
    #[must_use]
    pub const fn len(&self) -> u64 {
        self.event_count
    }

    /// Returns whether no event was observed.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.events.is_empty()
    }
}

impl animus_std::child_session_host::ChildSessionEventSink for MetadataChildEvents {
    fn observe(
        &mut self,
        event: &animus_std::child_session_host::ChildSessionEvent,
    ) -> Result<(), animus_std::child_session_host::ChildSessionHostFailure> {
        let event_count = self.event_count.checked_add(1).ok_or_else(|| {
            animus_std::child_session_host::ChildSessionHostFailure::new(
                animus_runtime::child_session_service::ChildSessionFailureClass::Session,
                "child event count exceeded its portable bound",
            )
        })?;
        self.events.push(event.clone());
        self.event_count = event_count;
        Ok(())
    }
}
