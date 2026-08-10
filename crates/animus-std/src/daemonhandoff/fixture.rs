//! Deterministic local fake services for tests and the packaged smoke.

const OWNED_PROCESS_COUNT: u32 = 2;
const SURVIVING_PROCESS_COUNT: u32 = 1;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum FixtureMode {
    Success,
    SpawnFailure,
    WrongEndpoint,
    ReadinessTimeout,
    SurvivingDescendant,
    RollbackDrift,
}

pub(super) struct FixtureServices {
    pub(super) mode: FixtureMode,
    pub(super) calls: Vec<&'static str>,
}

impl FixtureServices {
    pub(super) const fn new(mode: FixtureMode) -> Self {
        Self {
            mode,
            calls: Vec::new(),
        }
    }
}

impl super::adapter::HandoffIngressPort for FixtureServices {
    fn pause_ingress(
        &mut self,
        request: &super::adapter::HandoffIngressRequest,
    ) -> Result<super::adapter::HandoffIngressObservation, super::adapter::HandoffServiceError>
    {
        self.calls.push("pause_ingress");
        Ok(super::adapter::HandoffIngressObservation {
            handoff: request.handoff.clone(),
            primary_queue_generation: request.primary_queue_generation,
            child_queue_generation: request.child_queue_generation,
            steering_generation: request.steering_generation,
            prompt_pause: super::adapter::HandoffCheckState::Passed,
            steering_pause: super::adapter::HandoffCheckState::Passed,
            follow_up_pause: super::adapter::HandoffCheckState::Passed,
            external_pause: super::adapter::HandoffCheckState::Passed,
        })
    }

    fn resume_ingress(
        &mut self,
        request: &super::adapter::HandoffIngressResumeRequest,
    ) -> Result<super::adapter::HandoffIngressResumeObservation, super::adapter::HandoffServiceError>
    {
        self.calls.push("resume_ingress");
        Ok(super::adapter::HandoffIngressResumeObservation {
            handoff: request.handoff.clone(),
            lease_generation: request.lease_generation,
            resumed: super::adapter::HandoffCheckState::Passed,
            observation: "ingress.resume.fixture".into(),
        })
    }
}

impl super::adapter::HandoffStatePort for FixtureServices {
    fn commit_handoff(
        &mut self,
        request: &super::adapter::HandoffCommitRequest,
    ) -> Result<super::adapter::HandoffCommitObservation, super::adapter::HandoffServiceError> {
        self.calls.push("commit_handoff");
        Ok(super::adapter::HandoffCommitObservation {
            handoff: request.handoff.clone(),
            commit: "commit.handoff.fixture".into(),
            committed: true,
        })
    }

    fn commit_cancellation(
        &mut self,
        request: &super::adapter::HandoffCancellationRequest,
    ) -> Result<super::adapter::HandoffCancellationObservation, super::adapter::HandoffServiceError>
    {
        self.calls.push("commit_cancellation");
        Ok(super::adapter::HandoffCancellationObservation {
            cancellation: request.cancellation.clone(),
            committed: true,
            observation: "cancel.commit.fixture".into(),
        })
    }

    fn revalidate_rollback(
        &mut self,
        request: &super::adapter::HandoffRollbackRequest,
    ) -> Result<super::adapter::HandoffRollbackObservation, super::adapter::HandoffServiceError>
    {
        self.calls.push("revalidate_rollback");
        let session_head = if self.mode == FixtureMode::RollbackDrift {
            "session.head.drift".into()
        } else {
            request.session_head.clone()
        };
        Ok(super::adapter::HandoffRollbackObservation {
            handoff: request.handoff.clone(),
            old_worker: request.old_worker.clone(),
            authority_profile: request.authority_profile.clone(),
            session_head,
            next_generation: request.next_generation,
            authority_current: true,
            store_head_current: self.mode != FixtureMode::RollbackDrift,
            observation: "rollback.recheck.fixture".into(),
        })
    }
}

impl super::adapter::HandoffLeasePort for FixtureServices {
    fn update_lease(
        &mut self,
        request: &super::adapter::HandoffLeaseRequest,
    ) -> Result<super::adapter::HandoffLeaseObservation, super::adapter::HandoffServiceError> {
        self.calls.push(match request.operation {
            super::adapter::HandoffLeaseOperation::Release => "release_lease",
            super::adapter::HandoffLeaseOperation::Restore => "restore_lease",
        });
        Ok(super::adapter::HandoffLeaseObservation {
            handoff: request.handoff.clone(),
            operation: request.operation,
            generation: request.generation,
            completed: true,
            observation: "lease.update.fixture".into(),
        })
    }
}

impl super::adapter::HandoffProcessPort for FixtureServices {
    fn start_replacement(
        &mut self,
        request: &super::adapter::HandoffProcessStartRequest,
    ) -> Result<super::adapter::HandoffProcessStartObservation, super::adapter::HandoffServiceError>
    {
        self.calls.push("start_replacement");
        Ok(super::adapter::HandoffProcessStartObservation {
            handoff: request.handoff.clone(),
            process_attempt: "process.attempt.fixture".into(),
            process: "process.replacement.fixture".into(),
            endpoint: "endpoint.local.fixture".into(),
            started: self.mode != FixtureMode::SpawnFailure,
        })
    }

    fn terminate_replacement(
        &mut self,
        _request: &super::adapter::HandoffTerminationRequest,
    ) -> Result<(), super::adapter::HandoffServiceError> {
        self.calls.push("terminate_replacement");
        Ok(())
    }

    fn observe_termination(
        &mut self,
        request: &super::adapter::HandoffTerminationRequest,
    ) -> Result<super::adapter::HandoffTerminationObservation, super::adapter::HandoffServiceError>
    {
        self.calls.push("observe_termination");
        if request.process_attempt.is_none() {
            return Ok(super::adapter::HandoffTerminationObservation {
                process_attempt: None,
                leader_terminal: true,
                descendants_terminal: true,
                owned_processes: 0,
                terminal_processes: 0,
                live_descendants: 0,
            });
        }
        let is_survivor = self.mode == FixtureMode::SurvivingDescendant;
        Ok(super::adapter::HandoffTerminationObservation {
            process_attempt: request.process_attempt.clone(),
            leader_terminal: true,
            descendants_terminal: !is_survivor,
            owned_processes: OWNED_PROCESS_COUNT,
            terminal_processes: if is_survivor {
                SURVIVING_PROCESS_COUNT
            } else {
                OWNED_PROCESS_COUNT
            },
            live_descendants: if is_survivor {
                SURVIVING_PROCESS_COUNT
            } else {
                0
            },
        })
    }
}

impl super::adapter::HandoffEndpointPort for FixtureServices {
    fn authenticate_claim(
        &mut self,
        request: &super::adapter::HandoffClaimRequest,
    ) -> Result<super::adapter::HandoffClaimObservation, super::adapter::HandoffServiceError> {
        self.calls.push("authenticate_claim");
        let endpoint = if self.mode == FixtureMode::WrongEndpoint {
            "endpoint.local.wrong".into()
        } else {
            request.endpoint.clone()
        };
        Ok(super::adapter::HandoffClaimObservation {
            schema: super::adapter::HANDOFF_SCHEMA.into(),
            handoff: request.handoff.clone(),
            session: request.session.clone(),
            released_generation: request.released_generation,
            process_attempt: request.process_attempt.clone(),
            process: request.process.clone(),
            endpoint,
            claimant: "claimant.fixture".into(),
            protocol_version: request.protocol_version,
            freshness_nonce: "nonce.fixture".into(),
            authenticated: true,
            current: true,
            authentication_observation: "authentication.fixture".into(),
        })
    }
}

impl super::adapter::HandoffReadinessPort for FixtureServices {
    fn wait_readiness(
        &mut self,
        request: &super::adapter::HandoffReadinessRequest,
    ) -> Result<super::adapter::HandoffReadinessObservation, super::adapter::HandoffServiceError>
    {
        self.calls.push("wait_readiness");
        if self.mode == FixtureMode::ReadinessTimeout
            || self.mode == FixtureMode::SurvivingDescendant
            || self.mode == FixtureMode::RollbackDrift
        {
            return Err(super::adapter::HandoffServiceError::TimedOut);
        }
        Ok(super::adapter::HandoffReadinessObservation {
            handoff: request.handoff.clone(),
            claim: request.claim.clone(),
            session_head: request.session_head.clone(),
            composer_draft: request.composer_draft.clone(),
            vault_generation: request.vault_generation,
            active_turn_generation: request.active_turn_generation,
            authority_profile: request.authority_profile.clone(),
            policy: request.policy.clone(),
            services: "services.fixture.ready".into(),
            session_load: super::adapter::HandoffCheckState::Passed,
            vault_visibility: super::adapter::HandoffCheckState::Passed,
            authority_admission: super::adapter::HandoffCheckState::Passed,
            recovery_admission: super::adapter::HandoffCheckState::Passed,
            service_readiness: super::adapter::HandoffCheckState::Passed,
            readiness_commit: super::adapter::HandoffCheckState::Passed,
        })
    }
}

impl super::adapter::HandoffExitPort for FixtureServices {
    fn exit_old_worker(
        &mut self,
        request: &super::adapter::HandoffExitRequest,
    ) -> Result<super::adapter::HandoffExitObservation, super::adapter::HandoffServiceError> {
        self.calls.push("exit_old_worker");
        Ok(super::adapter::HandoffExitObservation {
            old_worker: request.old_worker.clone(),
            readiness: request.readiness.clone(),
            exited: true,
            observation: "old.worker.exit.fixture".into(),
        })
    }
}
