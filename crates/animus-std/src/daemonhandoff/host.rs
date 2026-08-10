//! Thin imperative shell for one transactional daemon handoff.

const NEXT_LEASE_INCREMENT: u64 = 1;

/// Exact host inputs for one handoff attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HandoffRunRequest {
    /// Strict-idle worker snapshot.
    pub idle: super::core::StrictIdleSnapshot,
    /// Exact session head to commit.
    pub session_head: super::core::SessionHeadIdentity,
    /// Host-retained composer-draft identity.
    pub composer_draft: super::core::ComposerDraftIdentity,
    /// Pending-input vault generation.
    pub vault_generation: super::core::VaultGeneration,
    /// Handoff policy identity.
    pub policy: super::core::PolicyIdentity,
    /// Bounded host deadline class.
    pub deadline_class: String,
}

/// Final host outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum HandoffHostOutcome {
    /// Replacement became ready and the old worker exited.
    Complete(super::core::HandoffTransaction),
    /// Replacement was torn down and a later old-worker lease was restored.
    LeaseRestored(super::core::HandoffTransaction),
}

/// Bounded host orchestration errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HandoffHostError {
    /// Pure core validation or reduction failed.
    Core(super::core::Issue),
    /// Injected host service failed.
    Service(super::adapter::HandoffServiceError),
    /// An adapter observation was malformed.
    MalformedObservation,
    /// A service observation did not match the exact request.
    StaleObservation,
    /// Lease generation overflowed.
    GenerationOverflow,
}

impl From<super::core::Error> for HandoffHostError {
    fn from(error: super::core::Error) -> Self {
        Self::Core(error.issue())
    }
}

/// Explicit host over one injected service set.
pub struct HandoffHost<S> {
    services: S,
}

impl<S> HandoffHost<S>
where
    S: super::adapter::HandoffHostServices,
{
    /// Creates a host without discovering processes, sockets, leases, stores, or clocks.
    #[must_use]
    pub const fn new(services: S) -> Self {
        Self { services }
    }

    /// Runs one success path or an admitted cancellation-and-restore path.
    ///
    /// # Errors
    ///
    /// Returns a bounded core, service, malformed-observation, stale-observation,
    /// or generation-overflow issue. A blocked rollback never resumes ingress.
    // r[impl agent_harness_runtime.daemon_handoff.transaction]
    // r[impl agent_harness_runtime.daemon_handoff.claim]
    // r[impl agent_harness_runtime.daemon_handoff.cancellation]
    // r[impl agent_harness_runtime.daemon_handoff.session_state]
    pub fn run(
        &mut self,
        request: &HandoffRunRequest,
    ) -> Result<HandoffHostOutcome, HandoffHostError> {
        super::adapter::validate_handoff_label(&request.deadline_class)
            .map_err(|_| HandoffHostError::MalformedObservation)?;
        let idle = super::core::validate_strict_idle(&request.idle)?;
        let prepared = super::core::prepare_handoff(&idle)?;
        let paused = self.pause_ingress(&prepared)?;
        let released = self.commit_and_release(&paused, request)?;
        let process = self.start_process(&released, request)?;
        let Some(started) = process else {
            return self
                .cancel_and_restore(&released, super::core::CancellationCause::SpawnFailure);
        };
        let claimed = match self.claim(&started) {
            Ok(claimed) => claimed,
            Err(
                HandoffHostError::Core(_)
                | HandoffHostError::MalformedObservation
                | HandoffHostError::StaleObservation,
            ) => {
                return self
                    .cancel_and_restore(&started, super::core::CancellationCause::ClaimFailure);
            }
            Err(error) => return Err(error),
        };
        let ready = match self.readiness(&claimed) {
            Ok(ready) => ready,
            Err(HandoffHostError::Service(super::adapter::HandoffServiceError::TimedOut)) => {
                return self.cancel_and_restore(&claimed, super::core::CancellationCause::Deadline);
            }
            Err(
                HandoffHostError::Core(_)
                | HandoffHostError::MalformedObservation
                | HandoffHostError::StaleObservation,
            ) => {
                return self.cancel_and_restore(
                    &claimed,
                    super::core::CancellationCause::ReadinessFailure,
                );
            }
            Err(error) => return Err(error),
        };
        let complete = self.exit_old_worker(&ready)?;
        Ok(HandoffHostOutcome::Complete(complete))
    }

    /// Returns the injected service set for bounded inspection.
    #[must_use]
    pub const fn services(&self) -> &S {
        &self.services
    }

    /// Returns mutable access to the injected service set.
    #[must_use]
    pub fn services_mut(&mut self) -> &mut S {
        &mut self.services
    }

    fn pause_ingress(
        &mut self,
        prepared: &super::core::HandoffTransaction,
    ) -> Result<super::core::HandoffTransaction, HandoffHostError> {
        let request = super::adapter::HandoffIngressRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: prepared.handoff.as_str().to_string(),
            worker: prepared.binding.old_worker.as_str().to_string(),
            session: prepared.binding.session.as_str().to_string(),
            primary_queue_generation: prepared.binding.primary_queue_generation.value(),
            child_queue_generation: prepared.binding.child_queue_generation.value(),
            steering_generation: prepared.binding.steering_generation.value(),
        };
        let observed = self
            .services
            .pause_ingress(&request)
            .map_err(HandoffHostError::Service)?;
        let is_exact = observed.handoff == request.handoff
            && observed.primary_queue_generation == request.primary_queue_generation
            && observed.child_queue_generation == request.child_queue_generation
            && observed.steering_generation == request.steering_generation;
        if !is_exact {
            return Err(HandoffHostError::StaleObservation);
        }
        super::core::pause_ingress(
            prepared,
            &super::core::IngressPauseObservation {
                handoff: prepared.handoff.clone(),
                primary_queue_generation: prepared.binding.primary_queue_generation,
                child_queue_generation: prepared.binding.child_queue_generation,
                steering_generation: prepared.binding.steering_generation,
                all_ingress_paused: observed.all_paused(),
            },
        )
        .map_err(Into::into)
    }

    fn commit_and_release(
        &mut self,
        paused: &super::core::HandoffTransaction,
        input: &HandoffRunRequest,
    ) -> Result<super::core::HandoffTransaction, HandoffHostError> {
        let commit_request = super::adapter::HandoffCommitRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: paused.handoff.as_str().to_string(),
            session: paused.binding.session.as_str().to_string(),
            session_head: input.session_head.as_str().to_string(),
            composer_draft: input.composer_draft.as_str().to_string(),
            vault_generation: input.vault_generation.value(),
            active_turn_generation: paused.binding.active_turn_generation.value(),
            authority_profile: paused.binding.authority_profile.as_str().to_string(),
            policy: input.policy.as_str().to_string(),
        };
        let committed = self
            .services
            .commit_handoff(&commit_request)
            .map_err(HandoffHostError::Service)?;
        if committed.handoff != commit_request.handoff || !committed.committed {
            return Err(HandoffHostError::StaleObservation);
        }
        super::adapter::validate_handoff_label(&committed.commit)
            .map_err(|_| HandoffHostError::MalformedObservation)?;
        let durable = super::core::DurableHandoffState {
            session_head: input.session_head.clone(),
            composer_draft: input.composer_draft.clone(),
            vault_generation: input.vault_generation,
            active_turn_generation: paused.binding.active_turn_generation,
            authority_profile: paused.binding.authority_profile.clone(),
            policy: input.policy.clone(),
            commit: super::core::CommitIdentity::new(&committed.commit),
        };
        let lease_request = super::adapter::HandoffLeaseRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: paused.handoff.as_str().to_string(),
            session: paused.binding.session.as_str().to_string(),
            worker: paused.binding.old_worker.as_str().to_string(),
            operation: super::adapter::HandoffLeaseOperation::Release,
            generation: paused.binding.lease_generation.value(),
            authorization: committed.commit.clone(),
        };
        let released = self
            .services
            .update_lease(&lease_request)
            .map_err(HandoffHostError::Service)?;
        let is_exact = released.handoff == lease_request.handoff
            && released.operation == super::adapter::HandoffLeaseOperation::Release
            && released.generation == lease_request.generation;
        if !is_exact {
            return Err(HandoffHostError::StaleObservation);
        }
        super::core::release_lease(
            paused,
            &durable,
            &super::core::LeaseReleaseObservation {
                handoff: paused.handoff.clone(),
                released_generation: paused.binding.lease_generation,
                durable_commit: durable.commit.clone(),
                released: released.completed,
            },
        )
        .map_err(Into::into)
    }

    fn start_process(
        &mut self,
        released: &super::core::HandoffTransaction,
        input: &HandoffRunRequest,
    ) -> Result<Option<super::core::HandoffTransaction>, HandoffHostError> {
        let request = super::adapter::HandoffProcessStartRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: released.handoff.as_str().to_string(),
            session: released.binding.session.as_str().to_string(),
            released_generation: released.binding.lease_generation.value(),
            deadline_class: input.deadline_class.clone(),
        };
        let observed = self
            .services
            .start_replacement(&request)
            .map_err(HandoffHostError::Service)?;
        if observed.handoff != request.handoff {
            return Err(HandoffHostError::StaleObservation);
        }
        if !observed.started {
            return Ok(None);
        }
        for value in [
            &observed.process_attempt,
            &observed.process,
            &observed.endpoint,
        ] {
            super::adapter::validate_handoff_label(value)
                .map_err(|_| HandoffHostError::MalformedObservation)?;
        }
        let next = super::core::start_replacement(
            released,
            &super::core::ProcessStartObservation {
                handoff: released.handoff.clone(),
                process_attempt: super::core::ProcessAttemptIdentity::new(observed.process_attempt),
                process: super::core::ProcessIdentity::new(observed.process),
                endpoint: super::core::EndpointIdentity::new(observed.endpoint),
                started: true,
            },
        )?;
        Ok(Some(next))
    }

    fn claim(
        &mut self,
        started: &super::core::HandoffTransaction,
    ) -> Result<super::core::HandoffTransaction, HandoffHostError> {
        let request = super::adapter::HandoffClaimRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: started.handoff.as_str().to_string(),
            session: started.binding.session.as_str().to_string(),
            released_generation: started
                .released_generation
                .ok_or(HandoffHostError::StaleObservation)?
                .value(),
            process_attempt: started
                .process_attempt
                .as_ref()
                .ok_or(HandoffHostError::StaleObservation)?
                .as_str()
                .to_string(),
            process: started
                .process
                .as_ref()
                .ok_or(HandoffHostError::StaleObservation)?
                .as_str()
                .to_string(),
            endpoint: started
                .endpoint
                .as_ref()
                .ok_or(HandoffHostError::StaleObservation)?
                .as_str()
                .to_string(),
            protocol_version: super::core::HANDOFF_PROTOCOL_VERSION,
        };
        let observed = self
            .services
            .authenticate_claim(&request)
            .map_err(HandoffHostError::Service)?;
        super::adapter::validate_handoff_claim_observation(&observed)
            .map_err(|_| HandoffHostError::MalformedObservation)?;
        let claim = super::core::ReplacementClaim {
            handoff: super::core::HandoffIdentity::new(observed.handoff),
            session: super::core::SessionIdentity::new(observed.session),
            released_generation: super::core::LeaseGeneration::new(observed.released_generation),
            process_attempt: super::core::ProcessAttemptIdentity::new(observed.process_attempt),
            process: super::core::ProcessIdentity::new(observed.process),
            endpoint: super::core::EndpointIdentity::new(observed.endpoint.clone()),
            claimant: super::core::ClaimantIdentity::new(observed.claimant),
            protocol_version: observed.protocol_version,
            freshness_nonce: super::core::FreshnessNonce::new(observed.freshness_nonce),
        };
        let authentication = super::core::AuthenticationObservation {
            endpoint: super::core::EndpointIdentity::new(observed.endpoint),
            process: claim.process.clone(),
            authenticated: observed.authenticated,
            current: observed.current,
            observation: super::core::ObservationIdentity::new(observed.authentication_observation),
        };
        super::core::admit_claim(started, &claim, &authentication).map_err(Into::into)
    }

    fn readiness(
        &mut self,
        claimed: &super::core::HandoffTransaction,
    ) -> Result<super::core::HandoffTransaction, HandoffHostError> {
        let durable = claimed
            .durable_state
            .as_ref()
            .ok_or(HandoffHostError::StaleObservation)?;
        let claim = claimed
            .claim
            .as_ref()
            .ok_or(HandoffHostError::StaleObservation)?;
        let request = super::adapter::HandoffReadinessRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: claimed.handoff.as_str().to_string(),
            claim: claim.as_str().to_string(),
            session_head: durable.session_head.as_str().to_string(),
            composer_draft: durable.composer_draft.as_str().to_string(),
            vault_generation: durable.vault_generation.value(),
            active_turn_generation: durable.active_turn_generation.value(),
            authority_profile: durable.authority_profile.as_str().to_string(),
            policy: durable.policy.as_str().to_string(),
        };
        let observed = self
            .services
            .wait_readiness(&request)
            .map_err(HandoffHostError::Service)?;
        super::adapter::validate_handoff_readiness_observation(&observed)
            .map_err(|_| HandoffHostError::MalformedObservation)?;
        super::core::commit_readiness(
            claimed,
            &super::core::ReadinessObservation {
                handoff: super::core::HandoffIdentity::new(observed.handoff),
                claim: super::core::ClaimIdentity::new(observed.claim),
                session_head: super::core::SessionHeadIdentity::new(observed.session_head),
                composer_draft: super::core::ComposerDraftIdentity::new(observed.composer_draft),
                vault_generation: super::core::VaultGeneration::new(observed.vault_generation),
                active_turn_generation: super::core::ActiveTurnGeneration::new(
                    observed.active_turn_generation,
                ),
                authority_profile: super::core::AuthorityProfileIdentity::new(
                    observed.authority_profile,
                ),
                policy: super::core::PolicyIdentity::new(observed.policy),
                services: super::core::ServiceReadinessIdentity::new(observed.services),
                session_load: map_readiness_check(observed.session_load),
                vault_visibility: map_readiness_check(observed.vault_visibility),
                authority_admission: map_readiness_check(observed.authority_admission),
                recovery_admission: map_readiness_check(observed.recovery_admission),
                service_readiness: map_readiness_check(observed.service_readiness),
                readiness_commit: map_readiness_check(observed.readiness_commit),
            },
        )
        .map_err(Into::into)
    }

    fn exit_old_worker(
        &mut self,
        ready: &super::core::HandoffTransaction,
    ) -> Result<super::core::HandoffTransaction, HandoffHostError> {
        let readiness = ready
            .readiness
            .as_ref()
            .ok_or(HandoffHostError::StaleObservation)?;
        let request = super::adapter::HandoffExitRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: ready.handoff.as_str().to_string(),
            old_worker: ready.binding.old_worker.as_str().to_string(),
            readiness: readiness.as_str().to_string(),
        };
        let observed = self
            .services
            .exit_old_worker(&request)
            .map_err(HandoffHostError::Service)?;
        for value in [
            &observed.old_worker,
            &observed.readiness,
            &observed.observation,
        ] {
            super::adapter::validate_handoff_label(value)
                .map_err(|_| HandoffHostError::MalformedObservation)?;
        }
        super::core::complete_handoff(
            ready,
            &super::core::CompletionObservation {
                old_worker: super::core::WorkerIdentity::new(observed.old_worker),
                readiness: super::core::ReadinessIdentity::new(observed.readiness),
                exited: observed.exited,
                observation: super::core::ObservationIdentity::new(observed.observation),
            },
        )
        .map_err(Into::into)
    }

    fn cancel_and_restore(
        &mut self,
        current: &super::core::HandoffTransaction,
        cause: super::core::CancellationCause,
    ) -> Result<HandoffHostOutcome, HandoffHostError> {
        let (cancelled, plan) = self.commit_cancelled(current, cause)?;
        let termination = self.terminate_and_observe(&cancelled)?;
        let restored = self.restore_cancelled_lease(&cancelled, &termination, &plan)?;
        self.resume_ingress(&restored)?;
        Ok(HandoffHostOutcome::LeaseRestored(restored))
    }

    fn commit_cancelled(
        &mut self,
        current: &super::core::HandoffTransaction,
        cause: super::core::CancellationCause,
    ) -> Result<
        (
            super::core::HandoffTransaction,
            super::core::CancellationPlan,
        ),
        HandoffHostError,
    > {
        let plan = super::core::plan_cancellation(current, cause)?;
        let request = super::adapter::HandoffCancellationRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: plan.handoff.as_str().to_string(),
            prior_transition: plan.prior_transition.as_str().to_string(),
            process_attempt: plan
                .process_attempt
                .as_ref()
                .map(|value| value.as_str().to_string()),
            cause: plan.cause.as_str().to_string(),
            cancellation: plan.cancellation.as_str().to_string(),
        };
        let committed = self
            .services
            .commit_cancellation(&request)
            .map_err(HandoffHostError::Service)?;
        let cancelled = super::core::commit_cancellation(
            current,
            &plan,
            &super::core::CancellationCommitObservation {
                cancellation: super::core::CancellationIdentity::new(committed.cancellation),
                committed: committed.committed,
                observation: super::core::ObservationIdentity::new(committed.observation),
            },
        )?;
        Ok((cancelled, plan))
    }

    fn terminate_and_observe(
        &mut self,
        cancelled: &super::core::HandoffTransaction,
    ) -> Result<super::core::ProcessTreeTerminationObservation, HandoffHostError> {
        let request = super::adapter::HandoffTerminationRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: cancelled.handoff.as_str().to_string(),
            process_attempt: cancelled
                .process_attempt
                .as_ref()
                .map(|value| value.as_str().to_string()),
            cancellation: cancelled
                .cancellation
                .as_ref()
                .ok_or(HandoffHostError::StaleObservation)?
                .as_str()
                .to_string(),
        };
        if request.process_attempt.is_some() {
            self.services
                .terminate_replacement(&request)
                .map_err(HandoffHostError::Service)?;
        }
        let terminal = self
            .services
            .observe_termination(&request)
            .map_err(HandoffHostError::Service)?;
        super::adapter::validate_handoff_termination_observation(&terminal)
            .map_err(|_| HandoffHostError::MalformedObservation)?;
        let mut observation = super::core::ProcessTreeTerminationObservation {
            process_attempt: terminal
                .process_attempt
                .map(super::core::ProcessAttemptIdentity::new),
            leader_terminal: terminal.leader_terminal,
            descendants_terminal: terminal.descendants_terminal,
            owned_processes: terminal.owned_processes,
            terminal_processes: terminal.terminal_processes,
            live_descendants: terminal.live_descendants,
            termination: super::core::TerminationIdentity::new("unprojected"),
        };
        observation.termination = super::core::project_termination_identity(&observation);
        Ok(observation)
    }

    fn restore_cancelled_lease(
        &mut self,
        cancelled: &super::core::HandoffTransaction,
        termination: &super::core::ProcessTreeTerminationObservation,
        plan: &super::core::CancellationPlan,
    ) -> Result<super::core::HandoffTransaction, HandoffHostError> {
        let durable = cancelled
            .durable_state
            .as_ref()
            .ok_or(HandoffHostError::StaleObservation)?;
        let next_generation = cancelled
            .binding
            .lease_generation
            .value()
            .checked_add(NEXT_LEASE_INCREMENT)
            .ok_or(HandoffHostError::GenerationOverflow)?;
        let request = super::adapter::HandoffRollbackRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: cancelled.handoff.as_str().to_string(),
            old_worker: cancelled.binding.old_worker.as_str().to_string(),
            authority_profile: cancelled.binding.authority_profile.as_str().to_string(),
            session_head: durable.session_head.as_str().to_string(),
            next_generation,
        };
        let rollback = self
            .services
            .revalidate_rollback(&request)
            .map_err(HandoffHostError::Service)?;
        let preflight = rollback_observation(&rollback, false, &rollback.observation);
        super::core::validate_lease_restoration_preconditions(cancelled, termination, &preflight)?;
        let lease_request = super::adapter::HandoffLeaseRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: rollback.handoff.clone(),
            session: cancelled.binding.session.as_str().to_string(),
            worker: rollback.old_worker.clone(),
            operation: super::adapter::HandoffLeaseOperation::Restore,
            generation: rollback.next_generation,
            authorization: plan.cancellation.as_str().to_string(),
        };
        let lease = self
            .services
            .update_lease(&lease_request)
            .map_err(HandoffHostError::Service)?;
        let is_exact = lease.handoff == lease_request.handoff
            && lease.operation == super::adapter::HandoffLeaseOperation::Restore
            && lease.generation == lease_request.generation;
        if !is_exact {
            return Err(HandoffHostError::StaleObservation);
        }
        let committed = rollback_observation(&rollback, lease.completed, &lease.observation);
        super::core::restore_lease(cancelled, termination, &committed).map_err(Into::into)
    }

    fn resume_ingress(
        &mut self,
        restored: &super::core::HandoffTransaction,
    ) -> Result<(), HandoffHostError> {
        let request = super::adapter::HandoffIngressResumeRequest {
            schema: super::adapter::HANDOFF_SCHEMA.to_string(),
            handoff: restored.handoff.as_str().to_string(),
            worker: restored.binding.old_worker.as_str().to_string(),
            lease_generation: restored
                .restored_generation
                .ok_or(HandoffHostError::StaleObservation)?
                .value(),
        };
        let observed = self
            .services
            .resume_ingress(&request)
            .map_err(HandoffHostError::Service)?;
        let is_exact = observed.handoff == request.handoff
            && observed.lease_generation == request.lease_generation
            && check_passed(observed.resumed);
        if !is_exact {
            return Err(HandoffHostError::StaleObservation);
        }
        Ok(())
    }
}

fn rollback_observation(
    rollback: &super::adapter::HandoffRollbackObservation,
    acquired: bool,
    observation: &str,
) -> super::core::LeaseRestoreObservation {
    super::core::LeaseRestoreObservation {
        handoff: super::core::HandoffIdentity::new(&rollback.handoff),
        old_worker: super::core::WorkerIdentity::new(&rollback.old_worker),
        authority_profile: super::core::AuthorityProfileIdentity::new(&rollback.authority_profile),
        session_head: super::core::SessionHeadIdentity::new(&rollback.session_head),
        restored_generation: super::core::LeaseGeneration::new(rollback.next_generation),
        authority_current: rollback.authority_current,
        store_head_current: rollback.store_head_current,
        acquired,
        observation: super::core::ObservationIdentity::new(observation),
    }
}

const fn map_readiness_check(
    state: super::adapter::HandoffCheckState,
) -> super::core::ReadinessCheck {
    match state {
        super::adapter::HandoffCheckState::Passed => super::core::ReadinessCheck::Passed,
        super::adapter::HandoffCheckState::Failed => super::core::ReadinessCheck::Failed,
    }
}

const fn check_passed(state: super::adapter::HandoffCheckState) -> bool {
    matches!(state, super::adapter::HandoffCheckState::Passed)
}
