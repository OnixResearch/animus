const MAX_CALLS: u32 = 2;
const WORKER_COUNT: usize = 2;
const ONE_CALL: u32 = 1;
const EXPECTED_COMMITS: u32 = 2;

struct ParallelService {
    barrier: std::sync::Arc<std::sync::Barrier>,
    active: std::sync::Arc<std::sync::atomic::AtomicU32>,
    max_active: std::sync::Arc<std::sync::atomic::AtomicU32>,
    status: animus_runtime::convergence::RunStatus,
}

impl super::RunService for ParallelService {
    fn run(&self, call_id: &str) -> animus_runtime::convergence::RunObservation {
        let active_count = self
            .active
            .fetch_add(ONE_CALL, std::sync::atomic::Ordering::SeqCst)
            .checked_add(ONE_CALL)
            .expect("active count");
        self.max_active
            .fetch_max(active_count, std::sync::atomic::Ordering::SeqCst);
        self.barrier.wait();
        self.active
            .fetch_sub(ONE_CALL, std::sync::atomic::Ordering::SeqCst);
        animus_runtime::convergence::RunObservation {
            call_id: std::string::String::from(call_id),
            status: self.status,
            telemetry: None,
        }
    }
}

struct CommitTracker {
    active: std::sync::Arc<std::sync::atomic::AtomicU32>,
    commits: u32,
}

impl super::CommitService for CommitTracker {
    fn commit(
        &mut self,
        _observation: &animus_runtime::convergence::RunObservation,
    ) -> Result<(), super::HostError> {
        if self.active.load(std::sync::atomic::Ordering::SeqCst) != 0 {
            return Err(super::HostError::CommitFailed);
        }
        self.commits = self
            .commits
            .checked_add(ONE_CALL)
            .ok_or(super::HostError::CommitFailed)?;
        Ok(())
    }
}

fn prepared_cycle() -> animus_runtime::convergence::Cycle {
    let cycle = animus_runtime::convergence::Cycle::new("cycle-1", MAX_CALLS).expect("cycle");
    animus_runtime::convergence::prepare_cycle(&cycle, &["call-1", "call-2"]).expect("prepared")
}

fn service(
    status: animus_runtime::convergence::RunStatus,
) -> (
    ParallelService,
    std::sync::Arc<std::sync::atomic::AtomicU32>,
    std::sync::Arc<std::sync::atomic::AtomicU32>,
) {
    let active = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
    let max_active = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
    (
        ParallelService {
            barrier: std::sync::Arc::new(std::sync::Barrier::new(WORKER_COUNT)),
            active: std::sync::Arc::clone(&active),
            max_active: std::sync::Arc::clone(&max_active),
            status,
        },
        active,
        max_active,
    )
}

#[test]
fn batch_runs_real_parallel_work_then_commits_after_join() {
    let cycle = prepared_cycle();
    let (service, active, max_active) = service(animus_runtime::convergence::RunStatus::Completed);
    let run = super::run_parallel(&cycle, &service).expect("parallel run");
    let mut commit = CommitTracker { active, commits: 0 };
    let finished = super::commit_batch(&run, &mut commit).expect("commit");

    assert_eq!(
        max_active.load(std::sync::atomic::Ordering::SeqCst),
        MAX_CALLS
    );
    assert_eq!(run.observations().len(), WORKER_COUNT);
    assert_eq!(run.observations()[0].call_id, "call-1");
    assert_eq!(run.observations()[1].call_id, "call-2");
    assert_eq!(commit.commits, EXPECTED_COMMITS);
    assert_eq!(
        finished.phase(),
        animus_runtime::convergence::CyclePhase::Finished
    );
}

#[test]
fn parked_batch_never_calls_commit_service() {
    let cycle = prepared_cycle();
    let (service, active, _max_active) =
        service(animus_runtime::convergence::RunStatus::NeedsHuman);
    let run = super::run_parallel(&cycle, &service).expect("parked run");
    let mut commit = CommitTracker { active, commits: 0 };

    assert_eq!(
        run.cycle().phase(),
        animus_runtime::convergence::CyclePhase::Parked
    );
    assert_eq!(
        super::commit_batch(&run, &mut commit),
        Err(super::HostError::InvalidCycle)
    );
    assert_eq!(commit.commits, 0);
}
