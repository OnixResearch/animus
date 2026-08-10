//! Non-default pending-entry recovery host over one injected service.

#![allow(missing_docs)]

type AdmissionRequest = animus_runtime::pending_input_vault::AdmissionRequest;
type Boundary = animus_runtime::pending_input_vault::Boundary;
type CommitOutcome = animus_runtime::pending_input_vault::CommitOutcome;
type CompletedDisposition = animus_runtime::pending_input_vault::CompletedDisposition;
type ConversationIdentity = animus_runtime::pending_input_vault::ConversationIdentity;
type DispositionDecision = animus_runtime::pending_input_vault::DispositionDecision;
type DispositionRequest = animus_runtime::pending_input_vault::DispositionRequest;
type EntryIdentity = animus_runtime::pending_input_vault::EntryIdentity;
type Error = animus_runtime::pending_input_vault::Error;
type Generation = animus_runtime::pending_input_vault::Generation;
type LoadedBody = animus_runtime::pending_input_vault::LoadedBody;
type Policy = animus_runtime::pending_input_vault::Policy;
type Projection = animus_runtime::pending_input_vault::Projection;
type RecoveryControlPlan = animus_runtime::pending_input_vault::RecoveryControlPlan;
type RecoveryState = animus_runtime::pending_input_vault::RecoveryState;

/// Current admission evidence supplied by the existing coding-session owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmissionObservation {
    pub entry: EntryIdentity,
    pub conversation: ConversationIdentity,
    pub generation: Generation,
    pub is_admitted: bool,
}

/// Host disposition result. Admission references remain non-transcript data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DispositionResult {
    pub state: RecoveryState,
    pub completed: Option<CompletedDisposition>,
    pub is_repeated: bool,
}

/// Explicit host over one injected store and transaction service.
pub struct Host<S> {
    policy: Policy,
    boundary: Boundary,
    service: S,
}

impl<S> Host<S>
where
    S: animus_runtime::pending_input_vault::Service,
{
    /// Creates a host without discovering a global store.
    ///
    /// # Errors
    ///
    /// Returns a pure policy error before the service is used.
    pub fn new(policy: Policy, boundary: Boundary, service: S) -> Result<Self, Error> {
        animus_runtime::pending_input_vault::validate_policy(&policy)?;
        animus_runtime::pending_input_vault::validate_boundary(&boundary)?;
        Ok(Self {
            policy,
            boundary,
            service,
        })
    }

    /// Loads one explicit boundary and returns review or blocked state only.
    ///
    /// # Errors
    ///
    /// Returns a bounded service or pure validation error.
    // r[impl agent_harness_runtime.pending_input_vault.boundary]
    pub fn startup(&mut self) -> Result<Projection, Error> {
        animus_runtime::pending_input_vault::load_startup(
            &mut self.service,
            &self.policy,
            &self.boundary,
        )
    }

    /// Commits one exact disposition, then attempts cleanup.
    ///
    /// # Errors
    ///
    /// Returns a bounded planning or commit error. Cleanup failure is retained
    /// as `CleanupRequired` instead of erasing completed evidence.
    // r[impl agent_harness_runtime.pending_input_vault.transactions]
    pub fn disposition(
        &mut self,
        projection: &Projection,
        request: &DispositionRequest,
        prior: Option<&CompletedDisposition>,
    ) -> Result<DispositionResult, Error> {
        let decision =
            animus_runtime::pending_input_vault::plan_disposition(projection, request, prior)?;
        match decision {
            DispositionDecision::Repeated(completed) => Ok(DispositionResult {
                state: state_after_completion(&completed),
                completed: Some(completed),
                is_repeated: true,
            }),
            DispositionDecision::Planned(plan) => self.commit_new(projection, &plan),
        }
    }

    /// Retries cleanup without repeating a completed disposition.
    ///
    /// # Errors
    ///
    /// Returns a bounded validation or cleanup-service error. Prior completion
    /// evidence remains authoritative when cleanup fails.
    pub fn retry_cleanup(
        &mut self,
        completed: &CompletedDisposition,
    ) -> Result<CompletedDisposition, Error> {
        animus_runtime::pending_input_vault::cleanup_completed(&mut self.service, completed)
    }

    /// Loads one exact body only after current normal admission succeeds.
    ///
    /// # Errors
    ///
    /// Returns unauthorized or stale before body access.
    // r[impl agent_harness_runtime.pending_input_vault.recovery]
    pub fn load_after_admission(
        &mut self,
        request: &AdmissionRequest,
        observation: &AdmissionObservation,
    ) -> Result<LoadedBody, Error> {
        if !observation.is_admitted {
            return Err(Error::new(
                animus_runtime::pending_input_vault::Issue::Unauthorized,
            ));
        }
        let is_exact = observation.entry == request.entry
            && observation.conversation == request.conversation
            && observation.generation == request.generation;
        if !is_exact {
            return Err(Error::new(
                animus_runtime::pending_input_vault::Issue::Stale,
            ));
        }
        animus_runtime::pending_input_vault::load_admitted_body(&mut self.service, request)
    }

    /// Runs one separately authorized recovery-only plan.
    ///
    /// # Errors
    ///
    /// Returns a bounded service error while ordinary ingress stays blocked.
    pub fn recover(
        &mut self,
        plan: &RecoveryControlPlan,
    ) -> Result<animus_runtime::pending_input_vault::RecoveryControlObservation, Error> {
        animus_runtime::pending_input_vault::execute_recovery_control(
            &mut self.service,
            &self.boundary,
            plan,
        )
    }

    /// Returns the injected service for bounded host inspection.
    #[must_use]
    pub const fn service(&self) -> &S {
        &self.service
    }

    /// Returns mutable access to the injected service.
    #[must_use]
    pub fn service_mut(&mut self) -> &mut S {
        &mut self.service
    }

    fn commit_new(
        &mut self,
        projection: &Projection,
        plan: &animus_runtime::pending_input_vault::DispositionPlan,
    ) -> Result<DispositionResult, Error> {
        let outcome =
            animus_runtime::pending_input_vault::commit_plan(&mut self.service, projection, plan)?;
        match outcome {
            CommitOutcome::ReviewRequired => Ok(DispositionResult {
                state: RecoveryState::ReviewRequired,
                completed: None,
                is_repeated: false,
            }),
            CommitOutcome::BlockedCorrupt(class) => Ok(DispositionResult {
                state: RecoveryState::BlockedCorrupt(class),
                completed: None,
                is_repeated: false,
            }),
            CommitOutcome::Completed(completed) => {
                let mut retained = *completed;
                match animus_runtime::pending_input_vault::cleanup_completed(
                    &mut self.service,
                    &retained,
                ) {
                    Ok(updated) => retained = updated,
                    Err(_) => {
                        retained.cleanup =
                            animus_runtime::pending_input_vault::CleanupState::Failed;
                    }
                }
                Ok(DispositionResult {
                    state: state_after_completion(&retained),
                    completed: Some(retained),
                    is_repeated: false,
                })
            }
        }
    }
}

fn state_after_completion(completed: &CompletedDisposition) -> RecoveryState {
    match completed.cleanup {
        animus_runtime::pending_input_vault::CleanupState::Complete
        | animus_runtime::pending_input_vault::CleanupState::NotRequired => {
            RecoveryState::DispositionComplete
        }
        animus_runtime::pending_input_vault::CleanupState::Required
        | animus_runtime::pending_input_vault::CleanupState::Failed => {
            RecoveryState::CleanupRequired
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const GENERATION_VALUE: u64 = 11;
    const MAX_ENTRIES_VALUE: u32 = 8;
    const MAX_ENTRY_BYTES_VALUE: u32 = 512;
    const MAX_TOTAL_BYTES_VALUE: u64 = 4_096;
    const MAX_ATTACHMENTS_VALUE: u32 = 2;
    const ENTRY_BYTES_VALUE: u32 = 32;
    const FIRST_SEQUENCE_VALUE: u64 = 1;
    const ZERO_CALLS: u32 = 0;
    const ONE_CALL: u32 = 1;
    const TWO_CALLS: u32 = 2;
    const ONE_ENTRY: usize = 1;
    const BODY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const BODY_TEXT: &str = "body loaded after current admission";

    fn policy() -> Policy {
        Policy {
            entries: MAX_ENTRIES_VALUE,
            entry_bytes: animus_runtime::pending_input_vault::ByteCount::new(MAX_ENTRY_BYTES_VALUE),
            total_bytes: MAX_TOTAL_BYTES_VALUE,
            attachments_per_entry: MAX_ATTACHMENTS_VALUE,
        }
    }

    fn boundary() -> Boundary {
        Boundary {
            id: animus_runtime::pending_input_vault::Identity::new("recovery.std"),
            conversation: ConversationIdentity::new("conversation.std"),
            branch: animus_runtime::pending_input_vault::BranchIdentity::new("branch.main"),
            store: animus_runtime::pending_input_vault::StoreIdentity::new("store.explicit"),
            authority: animus_runtime::pending_input_vault::AuthorityIdentity::new(
                "authority.current",
            ),
            policy: animus_runtime::pending_input_vault::PolicyIdentity::new("policy.current"),
            generation: Generation::new(GENERATION_VALUE),
        }
    }

    fn entry() -> animus_runtime::pending_input_vault::Entry {
        animus_runtime::pending_input_vault::Entry {
            id: EntryIdentity::new("entry.one"),
            conversation: ConversationIdentity::new("conversation.std"),
            generation: Generation::new(GENERATION_VALUE),
            sequence: animus_runtime::pending_input_vault::Sequence::new(FIRST_SEQUENCE_VALUE),
            class: animus_runtime::pending_input_vault::EntryClass::ActiveAccepted,
            body_identity: animus_runtime::pending_input_vault::BodyIdentity::new(BODY_HASH),
            body_bytes: animus_runtime::pending_input_vault::ByteCount::new(ENTRY_BYTES_VALUE),
            attachments: Vec::new(),
            provenance: animus_runtime::pending_input_vault::ProvenanceIdentity::new(
                "route.active",
            ),
        }
    }

    #[derive(Clone)]
    struct FixtureService {
        facts: animus_runtime::pending_input_vault::StartupFacts,
        load_calls: u32,
        commit_calls: u32,
        cleanup_calls: u32,
        body_calls: u32,
        cleanup_fails: bool,
    }

    impl FixtureService {
        fn new() -> Self {
            Self {
                facts: animus_runtime::pending_input_vault::StartupFacts {
                    boundary: boundary(),
                    entries: vec![entry()],
                    completed: None,
                    cleanup: animus_runtime::pending_input_vault::CleanupState::NotRequired,
                    uncertain_commit: false,
                },
                load_calls: ZERO_CALLS,
                commit_calls: ZERO_CALLS,
                cleanup_calls: ZERO_CALLS,
                body_calls: ZERO_CALLS,
                cleanup_fails: false,
            }
        }
    }

    impl animus_runtime::pending_input_vault::Service for FixtureService {
        fn load(
            &mut self,
            _request: &animus_runtime::pending_input_vault::LoadRequest,
        ) -> Result<
            animus_runtime::pending_input_vault::StartupFacts,
            animus_runtime::pending_input_vault::ServiceIssue,
        > {
            self.load_calls += ONE_CALL;
            Ok(self.facts.clone())
        }

        fn commit(
            &mut self,
            plan: &animus_runtime::pending_input_vault::DispositionPlan,
        ) -> Result<
            animus_runtime::pending_input_vault::CommitObservation,
            animus_runtime::pending_input_vault::ServiceIssue,
        > {
            self.commit_calls += ONE_CALL;
            Ok(animus_runtime::pending_input_vault::CommitObservation {
                plan_identity: plan.plan_identity.clone(),
                generation: plan.expected_generation,
                status: animus_runtime::pending_input_vault::CommitStatus::Committed,
                restore_material_retained: plan.kind
                    == animus_runtime::pending_input_vault::DispositionKind::Restore,
            })
        }

        fn cleanup(
            &mut self,
            _completed: &CompletedDisposition,
        ) -> Result<
            animus_runtime::pending_input_vault::CleanupObservation,
            animus_runtime::pending_input_vault::ServiceIssue,
        > {
            self.cleanup_calls += ONE_CALL;
            if self.cleanup_fails {
                return Err(animus_runtime::pending_input_vault::ServiceIssue::CleanupFailed);
            }
            Ok(animus_runtime::pending_input_vault::CleanupObservation::Complete)
        }

        fn load_body(
            &mut self,
            request: &AdmissionRequest,
        ) -> Result<LoadedBody, animus_runtime::pending_input_vault::ServiceIssue> {
            self.body_calls += ONE_CALL;
            Ok(LoadedBody {
                entry: request.entry.clone(),
                body_identity: request.body_identity.clone(),
                body: BODY_TEXT.to_string(),
            })
        }

        fn recover(
            &mut self,
            _plan: &RecoveryControlPlan,
        ) -> Result<
            animus_runtime::pending_input_vault::RecoveryControlObservation,
            animus_runtime::pending_input_vault::ServiceIssue,
        > {
            Ok(animus_runtime::pending_input_vault::RecoveryControlObservation::Complete)
        }
    }

    fn request(projection: &Projection) -> DispositionRequest {
        DispositionRequest {
            id: animus_runtime::pending_input_vault::DispositionIdentity::new("disposition.std"),
            projection: projection.identity.clone(),
            boundary: projection.boundary.clone(),
            expected_generation: projection.boundary.generation,
            selected: vec![EntryIdentity::new("entry.one")],
            kind: animus_runtime::pending_input_vault::DispositionKind::Restore,
            authorized: true,
        }
    }

    #[test]
    // r[verify agent_harness_runtime.pending_input_vault.recovery.restore]
    fn startup_and_commit_do_not_load_or_submit_body() {
        let mut host = Host::new(policy(), boundary(), FixtureService::new()).expect("host");
        let projection = host.startup().expect("startup");
        assert_eq!(projection.state, RecoveryState::ReviewRequired);
        assert_eq!(host.service().load_calls, ONE_CALL);
        assert_eq!(host.service().body_calls, ZERO_CALLS);

        let result = host
            .disposition(&projection, &request(&projection), None)
            .expect("disposition");
        assert_eq!(result.state, RecoveryState::DispositionComplete);
        assert!(!result.is_repeated);
        assert_eq!(host.service().commit_calls, ONE_CALL);
        assert_eq!(host.service().cleanup_calls, ONE_CALL);
        assert_eq!(host.service().body_calls, ZERO_CALLS);
        let completed = result.completed.expect("completed evidence");
        assert_eq!(completed.admission_requests.len(), ONE_ENTRY);
    }

    #[test]
    // r[verify agent_harness_runtime.pending_input_vault.transactions.cleanup]
    fn cleanup_failure_retains_completed_evidence_and_review_state() {
        let mut service = FixtureService::new();
        service.cleanup_fails = true;
        let mut host = Host::new(policy(), boundary(), service).expect("host");
        let projection = host.startup().expect("startup");
        let result = host
            .disposition(&projection, &request(&projection), None)
            .expect("disposition");
        assert_eq!(result.state, RecoveryState::CleanupRequired);
        let completed = result.completed.expect("completed evidence");
        assert_eq!(host.service().commit_calls, ONE_CALL);
        assert_eq!(host.service().cleanup_calls, ONE_CALL);
        assert_eq!(host.service().body_calls, ZERO_CALLS);

        host.service_mut().cleanup_fails = false;
        let cleaned = host.retry_cleanup(&completed).expect("cleanup retry");
        assert_eq!(
            cleaned.cleanup,
            animus_runtime::pending_input_vault::CleanupState::Complete
        );
        assert_eq!(host.service().commit_calls, ONE_CALL);
        assert_eq!(host.service().cleanup_calls, TWO_CALLS);
        assert_eq!(host.service().body_calls, ZERO_CALLS);
    }

    #[test]
    // r[verify agent_harness_runtime.pending_input_vault.recovery.restore]
    fn body_access_requires_exact_current_admission() {
        let mut host = Host::new(policy(), boundary(), FixtureService::new()).expect("host");
        let projection = host.startup().expect("startup");
        let result = host
            .disposition(&projection, &request(&projection), None)
            .expect("disposition");
        let completed = result.completed.expect("completed evidence");
        let admission = &completed.admission_requests[0];
        let denied = AdmissionObservation {
            entry: admission.entry.clone(),
            conversation: admission.conversation.clone(),
            generation: admission.generation,
            is_admitted: false,
        };
        assert!(host.load_after_admission(admission, &denied).is_err());
        assert_eq!(host.service().body_calls, ZERO_CALLS);

        let admitted = AdmissionObservation {
            is_admitted: true,
            ..denied
        };
        let loaded = host
            .load_after_admission(admission, &admitted)
            .expect("admitted body");
        assert_eq!(loaded.body, BODY_TEXT);
        assert_eq!(host.service().body_calls, ONE_CALL);
    }
}
