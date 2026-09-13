//! Incremental command and event control for the persistent live session.

impl<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
    super::PersistentLiveHost<Provider, Executor, Persistence, SnapshotStore, KernelPersistence>
where
    Provider: crate::ProviderStreamService,
    Executor: animus_runtime::rlm_kernel::RlmKernelHostExecutor,
    Persistence: crate::SessionPersistence,
    SnapshotStore: crate::SnapshotPersistence,
    KernelPersistence: crate::KernelSafePointPersistence,
{
    /// Starts one prompt without driving its first pending effect.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] for malformed correlation, active
    /// work, invalid input, policy denial, or reducer rejection.
    pub fn begin_prompt(
        &mut self,
        request: super::LiveSteelPrompt<'_>,
    ) -> Result<(), crate::LiveSteelAgentError> {
        super::admission::require_blake3_identity(request.correlation_id)?;
        let run_id = format!(
            "{}{}",
            crate::assembly::RUN_ID_PREFIX,
            request.correlation_id
        );
        let entry_id = format!(
            "{}{}",
            crate::assembly::ENTRY_ID_PREFIX,
            request.correlation_id
        );
        self.host
            .begin_without_driving(
                &animus_runtime::coding_session_host::CodingSessionCommand::Prompt {
                    run_id,
                    entry_id,
                    input: super::admission::admitted_input(request.text),
                },
            )
            .map_err(|_error| crate::LiveSteelAgentError::Session)
    }

    /// Queues one follow-up while the active effect remains pending.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] for malformed correlation, missing
    /// active work, queue bounds, policy denial, or reducer rejection.
    pub fn begin_follow_up(
        &mut self,
        request: super::LiveSteelQueuedInput<'_>,
    ) -> Result<(), crate::LiveSteelAgentError> {
        self.begin_queue_append(request, false)
    }

    /// Queues one steering input while the active effect remains pending.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] for malformed correlation, missing
    /// active work, queue bounds, policy denial, or reducer rejection.
    pub fn begin_steer(
        &mut self,
        request: super::LiveSteelQueuedInput<'_>,
    ) -> Result<(), crate::LiveSteelAgentError> {
        self.begin_queue_append(request, true)
    }

    /// Starts an exact queue edit without driving unrelated effects.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when the queue snapshot is stale,
    /// the item is absent, or the reducer rejects the edit.
    pub fn begin_queue_edit(
        &mut self,
        request: super::LiveSteelQueueEdit<'_>,
    ) -> Result<(), crate::LiveSteelAgentError> {
        self.require_queue_guard(super::admission::QueueGuard {
            generation: request.expected_generation,
            identity: request.expected_queue_identity,
        })?;
        let precondition = animus_runtime::coding_session_host::coding_session_queue_precondition(
            self.host.state_snapshot(),
            request.item_id,
        )
        .map_err(|_error| crate::LiveSteelAgentError::Session)?;
        self.host
            .begin_without_driving(
                &animus_runtime::coding_session_host::CodingSessionCommand::EditQueued {
                    item_id: String::from(request.item_id),
                    precondition,
                    input: super::admission::admitted_input(request.replacement),
                },
            )
            .map_err(|_error| crate::LiveSteelAgentError::Session)
    }

    /// Starts an exact queue removal without driving unrelated effects.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when the queue snapshot is stale,
    /// the item is absent, or the reducer rejects the removal.
    pub fn begin_queue_removal(
        &mut self,
        request: super::LiveSteelQueueRemoval<'_>,
    ) -> Result<(), crate::LiveSteelAgentError> {
        self.require_queue_guard(super::admission::QueueGuard {
            generation: request.expected_generation,
            identity: request.expected_queue_identity,
        })?;
        let precondition = animus_runtime::coding_session_host::coding_session_queue_precondition(
            self.host.state_snapshot(),
            request.item_id,
        )
        .map_err(|_error| crate::LiveSteelAgentError::Session)?;
        self.host
            .begin_without_driving(
                &animus_runtime::coding_session_host::CodingSessionCommand::RemoveQueued {
                    item_id: String::from(request.item_id),
                    precondition,
                },
            )
            .map_err(|_error| crate::LiveSteelAgentError::Session)
    }

    /// Starts cancellation of the current active run.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when no active run exists or the
    /// cancellation command is rejected.
    pub fn begin_cancel(&mut self) -> Result<(), crate::LiveSteelAgentError> {
        let run_id = self
            .host
            .state_snapshot()
            .run()
            .map(|run| String::from(run.run_id()))
            .ok_or(crate::LiveSteelAgentError::Session)?;
        self.host
            .begin_without_driving(
                &animus_runtime::coding_session_host::CodingSessionCommand::Cancel { run_id },
            )
            .map_err(|_error| crate::LiveSteelAgentError::Session)
    }

    /// Executes at most one pending effect.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when an effect, service, commit,
    /// authority check, or reducer transition fails.
    pub fn drive_one(&mut self) -> Result<bool, crate::LiveSteelAgentError> {
        self.host
            .drive_one_pending()
            .map_err(|_error| crate::LiveSteelAgentError::Session)
    }

    /// Drives all currently pending effects to one terminal or idle state.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when an effect fails or the hard
    /// coding-session step bound is exceeded.
    pub fn drive_pending(&mut self) -> Result<(), crate::LiveSteelAgentError> {
        self.host
            .drive_pending()
            .map_err(|_error| crate::LiveSteelAgentError::Session)
    }

    /// Returns safe queue metadata without private queued text.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] when queue metadata cannot be projected.
    pub fn queue_snapshot(
        &self,
    ) -> Result<
        animus_runtime::coding_session_host::CodingSessionQueueSafeSnapshot,
        crate::LiveSteelAgentError,
    > {
        animus_runtime::coding_session_host::project_coding_session_queue_safe_snapshot(
            self.host.state_snapshot(),
        )
        .map_err(|_error| crate::LiveSteelAgentError::Session)
    }

    /// Returns one bounded ordered private event batch after the supplied cursor.
    ///
    /// # Errors
    ///
    /// Returns [`crate::LiveSteelAgentError`] for a stale cursor, zero bound, or
    /// a requested batch larger than [`super::MAX_LIVE_SESSION_EVENT_BATCH`].
    pub fn events_after(
        &self,
        request: super::LiveSteelEventRead,
    ) -> Result<super::PrivateEventBatch, crate::LiveSteelAgentError> {
        if request.maximum == 0 || request.maximum > super::MAX_LIVE_SESSION_EVENT_BATCH {
            return Err(crate::LiveSteelAgentError::Session);
        }
        let cursor = usize::try_from(request.cursor)
            .map_err(|_error| crate::LiveSteelAgentError::Session)?;
        let maximum = usize::try_from(request.maximum)
            .map_err(|_error| crate::LiveSteelAgentError::Session)?;
        let snapshot = self.host.event_snapshot();
        if cursor > snapshot.len() {
            return Err(crate::LiveSteelAgentError::Session);
        }
        let events = snapshot
            .iter()
            .skip(cursor)
            .take(maximum)
            .cloned()
            .collect::<Vec<_>>();
        let next_cursor = cursor
            .checked_add(events.len())
            .and_then(|value| u32::try_from(value).ok())
            .ok_or(crate::LiveSteelAgentError::Session)?;
        Ok(super::PrivateEventBatch {
            next_cursor,
            events,
        })
    }

    fn begin_queue_append(
        &mut self,
        request: super::LiveSteelQueuedInput<'_>,
        is_steering: bool,
    ) -> Result<(), crate::LiveSteelAgentError> {
        super::admission::require_blake3_identity(request.correlation_id)?;
        let item_id = format!("{}{}", super::QUEUED_ITEM_ID_PREFIX, request.correlation_id);
        let entry_id = format!(
            "{}{}",
            crate::assembly::ENTRY_ID_PREFIX,
            request.correlation_id
        );
        let command = if is_steering {
            animus_runtime::coding_session_host::CodingSessionCommand::Steer {
                item_id,
                entry_id,
                input: super::admission::admitted_input(request.text),
            }
        } else {
            animus_runtime::coding_session_host::CodingSessionCommand::FollowUp {
                item_id,
                entry_id,
                input: super::admission::admitted_input(request.text),
            }
        };
        self.host
            .begin_without_driving(&command)
            .map_err(|_error| crate::LiveSteelAgentError::Session)
    }

    fn require_queue_guard(
        &self,
        guard: super::admission::QueueGuard<'_>,
    ) -> Result<(), crate::LiveSteelAgentError> {
        super::admission::require_blake3_identity(guard.identity)?;
        let snapshot = self.queue_snapshot()?;
        let is_matching = snapshot.generation.value() == guard.generation
            && snapshot.queue_identity == guard.identity;
        if !is_matching {
            return Err(crate::LiveSteelAgentError::Session);
        }
        Ok(())
    }
}
