#[derive(Clone, Debug)]
enum LeaseBehavior {
    Exact,
    WrongResource,
    WrongRevision,
    WrongGeneration,
    Denied,
}

#[derive(Clone, Debug)]
enum ReleaseBehavior {
    Exact,
    Override(u64),
    Fail,
}

#[derive(Clone, Debug)]
enum SecondReadMutation {
    Write(std::path::PathBuf, Vec<u8>),
    Replace(std::path::PathBuf, Vec<u8>),
    Remove(std::path::PathBuf),
}

#[derive(Clone, Debug)]
struct FakeServices {
    observation: animus_std_workspace_edit::HostObservation,
    second_observation: Option<
        Result<
            animus_std_workspace_edit::HostObservation,
            animus_std_workspace_edit::HostServiceIssue,
        >,
    >,
    checkpoint_failure: Option<(
        animus_std_workspace_edit::HostCheckpoint,
        animus_std_workspace_edit::HostServiceIssue,
    )>,
    mutation_at_second_read: Option<SecondReadMutation>,
    lease_behavior: LeaseBehavior,
    release_behavior: ReleaseBehavior,
    observe_count: usize,
    calls: Vec<&'static str>,
}

impl FakeServices {
    fn current(resource_identity: &str) -> Self {
        let _ = resource_identity;
        Self {
            observation: animus_std_workspace_edit::HostObservation {
                revision_identity: String::from(REVISION_ID),
                metadata_policy_identity: String::from(POLICY_ID),
                writer_generation: WRITER_GENERATION,
            },
            second_observation: None,
            checkpoint_failure: None,
            mutation_at_second_read: None,
            lease_behavior: LeaseBehavior::Exact,
            release_behavior: ReleaseBehavior::Exact,
            observe_count: 0,
            calls: Vec::new(),
        }
    }
}

impl animus_std_workspace_edit::HostServices for FakeServices {
    fn observe(
        &mut self,
        _resource_identity: &str,
    ) -> Result<
        animus_std_workspace_edit::HostObservation,
        animus_std_workspace_edit::HostServiceIssue,
    > {
        self.calls.push("observe");
        self.observe_count = self.observe_count.saturating_add(1);
        if self.observe_count > 1
            && let Some(observation) = &self.second_observation
        {
            return observation.clone();
        }
        Ok(self.observation.clone())
    }

    fn acquire_writer(
        &mut self,
        resource_identity: &str,
        revision_identity: &str,
        expected_generation: u64,
    ) -> Result<animus_std_workspace_edit::WriterLease, animus_std_workspace_edit::HostServiceIssue>
    {
        self.calls.push("acquire");
        match self.lease_behavior {
            LeaseBehavior::Exact => Ok(animus_std_workspace_edit::WriterLease::new(
                resource_identity,
                revision_identity,
                expected_generation,
            )),
            LeaseBehavior::WrongResource => Ok(animus_std_workspace_edit::WriterLease::new(
                OTHER_ID,
                revision_identity,
                expected_generation,
            )),
            LeaseBehavior::WrongRevision => Ok(animus_std_workspace_edit::WriterLease::new(
                resource_identity,
                OTHER_ID,
                expected_generation,
            )),
            LeaseBehavior::WrongGeneration => Ok(animus_std_workspace_edit::WriterLease::new(
                resource_identity,
                revision_identity,
                expected_generation.saturating_add(1),
            )),
            LeaseBehavior::Denied => Err(animus_std_workspace_edit::HostServiceIssue::Denied),
        }
    }

    fn checkpoint(
        &mut self,
        checkpoint: animus_std_workspace_edit::HostCheckpoint,
    ) -> Result<(), animus_std_workspace_edit::HostServiceIssue> {
        self.calls.push(match checkpoint {
            animus_std_workspace_edit::HostCheckpoint::BeforeSecondRead => "checkpoint-second-read",
            animus_std_workspace_edit::HostCheckpoint::BeforePublication => {
                "checkpoint-publication"
            }
        });
        if checkpoint == animus_std_workspace_edit::HostCheckpoint::BeforeSecondRead
            && let Some(mutation) = self.mutation_at_second_read.take()
        {
            match mutation {
                SecondReadMutation::Write(path, bytes) => {
                    std::fs::write(path, bytes)
                        .map_err(|_| animus_std_workspace_edit::HostServiceIssue::Unavailable)?;
                }
                SecondReadMutation::Replace(path, bytes) => {
                    let stage = path.with_extension("metadata-drift-stage");
                    std::fs::write(&stage, bytes)
                        .map_err(|_| animus_std_workspace_edit::HostServiceIssue::Unavailable)?;
                    std::fs::rename(stage, path)
                        .map_err(|_| animus_std_workspace_edit::HostServiceIssue::Unavailable)?;
                }
                SecondReadMutation::Remove(path) => {
                    std::fs::remove_file(path)
                        .map_err(|_| animus_std_workspace_edit::HostServiceIssue::Unavailable)?;
                }
            }
        }
        if let Some((failed_checkpoint, issue)) = self.checkpoint_failure
            && checkpoint == failed_checkpoint
        {
            return Err(issue);
        }
        Ok(())
    }

    fn release_writer(
        &mut self,
        lease: animus_std_workspace_edit::WriterLease,
        release: animus_std_workspace_edit::WriterRelease,
    ) -> Result<u64, animus_std_workspace_edit::HostServiceIssue> {
        self.calls.push("release");
        match self.release_behavior {
            ReleaseBehavior::Override(generation) => Ok(generation),
            ReleaseBehavior::Fail => {
                Err(animus_std_workspace_edit::HostServiceIssue::ReleaseFailed)
            }
            ReleaseBehavior::Exact => match release {
                animus_std_workspace_edit::WriterRelease::NotPublished => {
                    Ok(lease.writer_generation())
                }
                animus_std_workspace_edit::WriterRelease::Published
                | animus_std_workspace_edit::WriterRelease::PublicationUncertain => lease
                    .writer_generation()
                    .checked_add(1)
                    .ok_or(animus_std_workspace_edit::HostServiceIssue::ReleaseFailed),
            },
        }
    }
}

#[derive(Clone, Debug)]
struct Names {
    next: usize,
}

impl Default for Names {
    fn default() -> Self {
        Self { next: 1 }
    }
}

impl animus_std_workspace_edit::StageNameService for Names {
    fn next_stage_leaf(&mut self) -> String {
        let name = format!(".animus-stage-{}", self.next);
        self.next = self.next.saturating_add(1);
        name
    }
}
