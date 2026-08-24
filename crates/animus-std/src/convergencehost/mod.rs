//! Host-owned real-parallel execution and post-join commit for supervised cycles.
//!
//! Injected run services produce commit candidates in scoped worker threads.
//! The scope joins every worker before the shell calls any commit service.

/// Injected guarded-work service.
pub trait RunService: Sync {
    /// Produces one typed observation without committing user side effects.
    fn run(&self, call_id: &str) -> animus_runtime::convergence::RunObservation;
}

/// Injected post-join commit service.
pub trait CommitService {
    /// Commits one completed candidate after every worker has joined.
    ///
    /// # Errors
    ///
    /// Returns a typed host error without partial completion claims.
    fn commit(
        &mut self,
        observation: &animus_runtime::convergence::RunObservation,
    ) -> Result<(), HostError>;
}

/// Joined observations waiting for post-guard commit.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BatchRun {
    cycle: animus_runtime::convergence::Cycle,
    observations: std::vec::Vec<animus_runtime::convergence::RunObservation>,
}

impl BatchRun {
    /// Returns the pure cycle state after joined worker observations.
    #[must_use]
    pub const fn cycle(&self) -> &animus_runtime::convergence::Cycle {
        &self.cycle
    }

    /// Returns observations in prepared call order.
    #[must_use]
    pub fn observations(&self) -> &[animus_runtime::convergence::RunObservation] {
        &self.observations
    }
}

/// Supervised batch shell failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostError {
    /// Cycle was not prepared or observations did not match.
    InvalidCycle,
    /// A scoped worker panicked.
    WorkerPanicked,
    /// Commit service rejected a candidate.
    CommitFailed,
}

/// Runs every prepared call in its own scoped worker and joins all workers.
///
/// # Errors
///
/// Returns [`HostError`] when the cycle is not prepared, a worker panics, or
/// pure observation validation fails.
pub fn run_parallel<Service: RunService>(
    cycle: &animus_runtime::convergence::Cycle,
    service: &Service,
) -> Result<BatchRun, HostError> {
    if cycle.phase() != animus_runtime::convergence::CyclePhase::Prepared {
        return Err(HostError::InvalidCycle);
    }
    let observations = std::thread::scope(|scope| {
        let mut workers = std::vec::Vec::with_capacity(cycle.call_ids().len());
        for call_id in cycle.call_ids() {
            workers.push(scope.spawn(move || service.run(call_id)));
        }
        let mut joined = std::vec::Vec::with_capacity(workers.len());
        for worker in workers {
            joined.push(worker.join().map_err(|_panic| HostError::WorkerPanicked)?);
        }
        Ok::<_, HostError>(joined)
    })?;
    let observed = animus_runtime::convergence::observe_batch(cycle, &observations, true)
        .map_err(|_invalid| HostError::InvalidCycle)?;
    Ok(BatchRun {
        cycle: observed,
        observations,
    })
}

/// Commits every completed candidate after all workers have joined.
///
/// # Errors
///
/// Returns [`HostError`] for a non-ready batch or any commit rejection. A
/// rejected commit never produces a finished cycle.
pub fn commit_batch<Service: CommitService>(
    run: &BatchRun,
    service: &mut Service,
) -> Result<animus_runtime::convergence::Cycle, HostError> {
    if run.cycle.phase() != animus_runtime::convergence::CyclePhase::ReadyToCommit {
        return Err(HostError::InvalidCycle);
    }
    let mut committed_call_count = 0_u32;
    for observation in &run.observations {
        service
            .commit(observation)
            .map_err(|_error| HostError::CommitFailed)?;
        committed_call_count = committed_call_count
            .checked_add(1)
            .ok_or(HostError::CommitFailed)?;
    }
    animus_runtime::convergence::commit_cycle(
        &run.cycle,
        animus_runtime::convergence::GuardObservation {
            guard_released: true,
            side_effects_inside_guard: false,
            committed_call_count,
        },
    )
    .map_err(|_invalid| HostError::InvalidCycle)
}

#[cfg(test)]
mod tests;
