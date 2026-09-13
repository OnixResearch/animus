//! Observe a completed kernel write without changing its ordering.

pub(super) struct Observed<Persistence> {
    pub(super) inner: Persistence,
    pub(super) has_committed: std::rc::Rc<std::cell::Cell<bool>>,
}

impl<Persistence: animus_live_steel_agent::KernelSafePointPersistence>
    animus_live_steel_agent::KernelSafePointPersistence for Observed<Persistence>
{
    fn persist(
        &mut self,
        journal: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<(), animus_live_steel_agent::SteelToolError> {
        self.inner.persist(journal)?;
        self.has_committed.set(true);
        Ok(())
    }

    fn close(self) -> Result<(), animus_live_steel_agent::SteelToolError> {
        self.inner.close()
    }
}
