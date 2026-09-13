/// Private safe-point persistence port owned by the application shell.
pub trait KernelSafePointPersistence {
    /// Persists one complete committed kernel journal atomically.
    ///
    /// # Errors
    ///
    /// Returns [`crate::SteelToolError::Persistence`] when publication is not confirmed.
    fn persist(
        &mut self,
        journal: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<(), super::SteelToolError>;

    /// Releases the persistence lease.
    ///
    /// # Errors
    ///
    /// Returns [`crate::SteelToolError::Persistence`] when release is uncertain.
    fn close(self) -> Result<(), super::SteelToolError>
    where
        Self: Sized;
}

impl KernelSafePointPersistence for () {
    fn persist(
        &mut self,
        _journal: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<(), super::SteelToolError> {
        Ok(())
    }

    fn close(self) -> Result<(), super::SteelToolError> {
        Ok(())
    }
}

impl<Synchronizer> KernelSafePointPersistence
    for animus_std::DurableSteelKernelPersistence<'_, Synchronizer>
where
    Synchronizer: animus_std::DurableSessionSynchronizer,
{
    fn persist(
        &mut self,
        journal: &animus_steel::SteelKernelPrivateJournal,
    ) -> Result<(), super::SteelToolError> {
        self.commit_snapshot(journal)
            .map(|_outcome| ())
            .map_err(|_error| super::SteelToolError::Persistence)
    }

    fn close(self) -> Result<(), super::SteelToolError> {
        self.close()
            .map_err(|_error| super::SteelToolError::Persistence)
    }
}
