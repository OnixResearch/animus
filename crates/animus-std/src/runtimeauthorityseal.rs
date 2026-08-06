//! Thin host shell for one sealed runtime-authority dispatch.

#![allow(missing_docs)]

use animus_runtime::runtime_authority_seal as seal;

const HOST_REPORT_SCHEMA: &str = "animus.runtime-authority-seal-host-report.v1";
const HOST_REPORT_NON_CLAIM: &str = "the host report records bounded injected-service observations; it does not prove authorization, isolation, effect correctness, or hidden-authority absence";
#[cfg(test)]
const EXPECTED_EVENT_COUNT: usize = 6;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RuntimeAuthoritySealHostDraft<'a> {
    pub session_id: &'a str,
    pub dispatch_id: &'a str,
    pub session_lineage_identity: &'a str,
    pub durable_binding_identity: &'a str,
    pub authority_profile_identity: &'a str,
    pub enforcement_observation_identity: &'a str,
    pub operation_set_identity: &'a str,
    pub adapter_cohort_identity: &'a str,
    pub resource_profile_identity: &'a str,
    pub resource_budget_identity: &'a str,
    pub runtime_services_identity: &'a str,
    pub operation_identities: &'a [&'a str],
    pub resource_identities: &'a [&'a str],
    pub expected_views: &'a [seal::RuntimeAuthorityExpectedViewDraft<'a>],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeAuthorityHostObservedView {
    pub class: seal::RuntimeAuthorityViewClass,
    pub identity: String,
    pub status: seal::RuntimeAuthorityObservationStatus,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeAuthorityHostRecheck {
    pub session_lineage_identity: String,
    pub authority_profile_identity: String,
    pub durable_binding_identity: String,
    pub enforcement_observation_identity: String,
    pub operation_set_identity: String,
    pub adapter_cohort_identity: String,
    pub resource_profile_identity: String,
    pub resource_budget_identity: String,
    pub runtime_services_identity: String,
    pub operation_identities: Vec<String>,
    pub resource_identities: Vec<String>,
    pub admitted: bool,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeAuthorityEffectObservation {
    pub effect_receipt_identity: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeAuthorityServiceError {
    code: &'static str,
}

impl RuntimeAuthorityServiceError {
    #[must_use]
    pub const fn new(code: &'static str) -> Self {
        Self { code }
    }

    #[must_use]
    pub const fn code(&self) -> &'static str {
        self.code
    }
}

pub trait RuntimeAuthorityObservationService {
    /// Return the current runtime view for each expected class.
    ///
    /// # Errors
    ///
    /// Returns a safe service code when observation fails.
    fn observe(
        &mut self,
        expected: &[seal::RuntimeAuthorityExpectedViewDraft<'_>],
    ) -> Result<Vec<RuntimeAuthorityHostObservedView>, RuntimeAuthorityServiceError>;
}

pub trait RuntimeAuthorityReceiptCommitService {
    /// Commit the metadata receipt and return its observed identity.
    ///
    /// # Errors
    ///
    /// Returns a safe service code when durable commit fails.
    fn commit(
        &mut self,
        receipt: &seal::RuntimeAuthoritySealReceipt,
    ) -> Result<String, RuntimeAuthorityServiceError>;
}

pub trait RuntimeAuthorityCurrentRecheckService {
    /// Return the current enforcement view immediately before effect start.
    ///
    /// # Errors
    ///
    /// Returns a safe service code when the current view cannot be read.
    fn recheck(
        &mut self,
        unit: &seal::RuntimeAuthorityUnit,
    ) -> Result<RuntimeAuthorityHostRecheck, RuntimeAuthorityServiceError>;
}

pub trait RuntimeAuthorityEffectService {
    /// Dispatch one already-sealed unit.
    ///
    /// # Errors
    ///
    /// Returns a safe service code when the effect fails.
    fn dispatch(
        &mut self,
        unit: &seal::RuntimeAuthorityUnit,
    ) -> Result<RuntimeAuthorityEffectObservation, RuntimeAuthorityServiceError>;
}

pub trait RuntimeAuthorityCleanupService {
    /// Clean up one terminal or failed unit and return metadata evidence identity.
    ///
    /// # Errors
    ///
    /// Returns a safe service code when cleanup fails.
    fn cleanup(
        &mut self,
        unit: &seal::RuntimeAuthorityUnit,
    ) -> Result<String, RuntimeAuthorityServiceError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeAuthoritySealHostStage {
    Observe,
    Prepare,
    CommitReceipt,
    CurrentRecheck,
    Effect,
    Cleanup,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeAuthoritySealHostError {
    stage: RuntimeAuthoritySealHostStage,
    service_code: Option<&'static str>,
    core_issues: Vec<seal::RuntimeAuthorityIssue>,
}

impl RuntimeAuthoritySealHostError {
    #[must_use]
    pub const fn stage(&self) -> RuntimeAuthoritySealHostStage {
        self.stage
    }

    #[must_use]
    pub const fn service_code(&self) -> Option<&'static str> {
        self.service_code
    }

    #[must_use]
    pub fn core_issues(&self) -> &[seal::RuntimeAuthorityIssue] {
        self.core_issues.as_slice()
    }

    fn service(stage: RuntimeAuthoritySealHostStage, error: &RuntimeAuthorityServiceError) -> Self {
        Self {
            stage,
            service_code: Some(error.code()),
            core_issues: Vec::new(),
        }
    }

    fn core(stage: RuntimeAuthoritySealHostStage, error: &seal::RuntimeAuthoritySealError) -> Self {
        Self {
            stage,
            service_code: None,
            core_issues: error.issues().to_vec(),
        }
    }
}

impl core::fmt::Display for RuntimeAuthoritySealHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "runtime authority host failed at {:?}",
            self.stage
        )
    }
}

impl std::error::Error for RuntimeAuthoritySealHostError {}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeAuthoritySealHostEvent {
    UnitPrepared,
    SealReceiptCommitted,
    CurrentEnforcementRechecked,
    EffectStarted,
    EffectCompleted,
    CleanupCompleted,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeAuthoritySealHostReport {
    pub schema: &'static str,
    pub unit: seal::RuntimeAuthorityUnit,
    pub seal_receipt: seal::RuntimeAuthoritySealReceipt,
    pub effect_receipt_identity: String,
    pub cleanup_identity: String,
    pub events: Vec<RuntimeAuthoritySealHostEvent>,
    pub seal_committed_before_effect: bool,
    pub non_claim: &'static str,
}

pub struct RuntimeAuthoritySealHostServices<'a, O, C, R, E, L> {
    pub observation: &'a mut O,
    pub receipt_commit: &'a mut C,
    pub current_recheck: &'a mut R,
    pub effect: &'a mut E,
    pub cleanup: &'a mut L,
}

/// Seal, recheck, dispatch, and clean up one unit through injected services.
///
/// # Errors
///
/// Returns a stage and safe issue data when an injected service or pure admission step fails.
pub fn execute_runtime_authority_sealed_dispatch<O, C, R, E, L>(
    draft: RuntimeAuthoritySealHostDraft<'_>,
    receipt_draft: seal::RuntimeAuthorityReceiptDraft<'_>,
    services: &mut RuntimeAuthoritySealHostServices<'_, O, C, R, E, L>,
) -> Result<RuntimeAuthoritySealHostReport, RuntimeAuthoritySealHostError>
where
    O: RuntimeAuthorityObservationService,
    C: RuntimeAuthorityReceiptCommitService,
    R: RuntimeAuthorityCurrentRecheckService,
    E: RuntimeAuthorityEffectService,
    L: RuntimeAuthorityCleanupService,
{
    let plan = prepare_with_observation(draft, receipt_draft, services.observation)?;
    let sealed = commit_plan(&plan, services.receipt_commit)?;
    let running = recheck_and_start(&sealed, services.current_recheck)?;
    let (effect, completed, cleanup_identity) =
        run_effect_and_cleanup(&running, services.effect, services.cleanup)?;
    debug_assert!(completed.state().is_terminal());
    Ok(RuntimeAuthoritySealHostReport {
        schema: HOST_REPORT_SCHEMA,
        unit: completed,
        seal_receipt: plan.receipt().clone(),
        effect_receipt_identity: effect.effect_receipt_identity,
        cleanup_identity,
        events: host_events(),
        seal_committed_before_effect: true,
        non_claim: HOST_REPORT_NON_CLAIM,
    })
}

fn prepare_with_observation<O: RuntimeAuthorityObservationService>(
    draft: RuntimeAuthoritySealHostDraft<'_>,
    receipt_draft: seal::RuntimeAuthorityReceiptDraft<'_>,
    service: &mut O,
) -> Result<seal::RuntimeAuthoritySealPlan, RuntimeAuthoritySealHostError> {
    let observed = service.observe(draft.expected_views).map_err(|error| {
        RuntimeAuthoritySealHostError::service(RuntimeAuthoritySealHostStage::Observe, &error)
    })?;
    let observed_drafts = observed
        .iter()
        .map(|view| seal::RuntimeAuthorityObservedViewDraft {
            class: view.class,
            identity: view.identity.as_str(),
            status: view.status,
        })
        .collect::<Vec<_>>();
    let core_draft = seal::RuntimeAuthorityUnitDraft {
        session_id: draft.session_id,
        dispatch_id: draft.dispatch_id,
        session_lineage_identity: draft.session_lineage_identity,
        durable_binding_identity: draft.durable_binding_identity,
        authority_profile_identity: draft.authority_profile_identity,
        enforcement_observation_identity: draft.enforcement_observation_identity,
        operation_set_identity: draft.operation_set_identity,
        adapter_cohort_identity: draft.adapter_cohort_identity,
        resource_profile_identity: draft.resource_profile_identity,
        resource_budget_identity: draft.resource_budget_identity,
        runtime_services_identity: draft.runtime_services_identity,
        operation_identities: draft.operation_identities,
        resource_identities: draft.resource_identities,
        expected_views: draft.expected_views,
        observed_views: observed_drafts.as_slice(),
    };
    seal::prepare_runtime_authority_seal(core_draft, receipt_draft).map_err(|error| {
        RuntimeAuthoritySealHostError::core(RuntimeAuthoritySealHostStage::Prepare, &error)
    })
}

fn commit_plan<C: RuntimeAuthorityReceiptCommitService>(
    plan: &seal::RuntimeAuthoritySealPlan,
    service: &mut C,
) -> Result<seal::RuntimeAuthorityUnit, RuntimeAuthoritySealHostError> {
    let committed_identity = service.commit(plan.receipt()).map_err(|error| {
        RuntimeAuthoritySealHostError::service(RuntimeAuthoritySealHostStage::CommitReceipt, &error)
    })?;
    seal::commit_runtime_authority_seal(plan, committed_identity.as_str()).map_err(|error| {
        RuntimeAuthoritySealHostError::core(RuntimeAuthoritySealHostStage::CommitReceipt, &error)
    })
}

fn recheck_and_start<R: RuntimeAuthorityCurrentRecheckService>(
    unit: &seal::RuntimeAuthorityUnit,
    service: &mut R,
) -> Result<seal::RuntimeAuthorityUnit, RuntimeAuthoritySealHostError> {
    let recheck = service.recheck(unit).map_err(|error| {
        RuntimeAuthoritySealHostError::service(
            RuntimeAuthoritySealHostStage::CurrentRecheck,
            &error,
        )
    })?;
    let operations = recheck
        .operation_identities
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let resources = recheck
        .resource_identities
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>();
    let draft = seal::RuntimeAuthorityRecheckDraft {
        session_lineage_identity: recheck.session_lineage_identity.as_str(),
        authority_profile_identity: recheck.authority_profile_identity.as_str(),
        durable_binding_identity: recheck.durable_binding_identity.as_str(),
        enforcement_observation_identity: recheck.enforcement_observation_identity.as_str(),
        operation_set_identity: recheck.operation_set_identity.as_str(),
        adapter_cohort_identity: recheck.adapter_cohort_identity.as_str(),
        resource_profile_identity: recheck.resource_profile_identity.as_str(),
        resource_budget_identity: recheck.resource_budget_identity.as_str(),
        runtime_services_identity: recheck.runtime_services_identity.as_str(),
        operation_identities: operations.as_slice(),
        resource_identities: resources.as_slice(),
        admitted: recheck.admitted,
    };
    seal::start_runtime_authority_effect(unit, draft).map_err(|error| {
        RuntimeAuthoritySealHostError::core(RuntimeAuthoritySealHostStage::CurrentRecheck, &error)
    })
}

fn run_effect_and_cleanup<E, L>(
    running: &seal::RuntimeAuthorityUnit,
    effect_service: &mut E,
    cleanup_service: &mut L,
) -> Result<
    (
        RuntimeAuthorityEffectObservation,
        seal::RuntimeAuthorityUnit,
        String,
    ),
    RuntimeAuthoritySealHostError,
>
where
    E: RuntimeAuthorityEffectService,
    L: RuntimeAuthorityCleanupService,
{
    let effect = match effect_service.dispatch(running) {
        Ok(effect) => effect,
        Err(error) => {
            let _failed = seal::transition_runtime_authority_unit(
                running,
                seal::RuntimeAuthorityTransition::Fail,
            );
            let _cleanup = cleanup_service.cleanup(running);
            return Err(RuntimeAuthoritySealHostError::service(
                RuntimeAuthoritySealHostStage::Effect,
                &error,
            ));
        }
    };
    let completed = seal::transition_runtime_authority_unit(
        running,
        seal::RuntimeAuthorityTransition::Complete,
    )
    .map_err(|error| {
        RuntimeAuthoritySealHostError::core(RuntimeAuthoritySealHostStage::Effect, &error)
    })?;
    let cleanup_identity = cleanup_service.cleanup(&completed).map_err(|error| {
        RuntimeAuthoritySealHostError::service(RuntimeAuthoritySealHostStage::Cleanup, &error)
    })?;
    Ok((effect, completed, cleanup_identity))
}

fn host_events() -> Vec<RuntimeAuthoritySealHostEvent> {
    vec![
        RuntimeAuthoritySealHostEvent::UnitPrepared,
        RuntimeAuthoritySealHostEvent::SealReceiptCommitted,
        RuntimeAuthoritySealHostEvent::CurrentEnforcementRechecked,
        RuntimeAuthoritySealHostEvent::EffectStarted,
        RuntimeAuthoritySealHostEvent::EffectCompleted,
        RuntimeAuthoritySealHostEvent::CleanupCompleted,
    ]
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;

    use super::*;

    const EVENT_OBSERVE: &str = "observe";
    const EVENT_COMMIT: &str = "commit";
    const EVENT_RECHECK: &str = "recheck";
    const EVENT_EFFECT: &str = "effect";
    const EVENT_CLEANUP: &str = "cleanup";
    const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const OPERATIONS: &[&str] = &[HASH_A];
    const RESOURCES: &[&str] = &[HASH_B];
    const EXPECTED: &[seal::RuntimeAuthorityExpectedViewDraft<'static>] = &[
        expected(seal::RuntimeAuthorityViewClass::SessionLineage),
        expected(seal::RuntimeAuthorityViewClass::DurableBinding),
        expected(seal::RuntimeAuthorityViewClass::AuthorityProfile),
        expected(seal::RuntimeAuthorityViewClass::EnforcementObservation),
        expected(seal::RuntimeAuthorityViewClass::OperationSet),
        expected(seal::RuntimeAuthorityViewClass::AdapterCohort),
        expected(seal::RuntimeAuthorityViewClass::ResourceProfile),
        expected(seal::RuntimeAuthorityViewClass::ResourceBudget),
        expected(seal::RuntimeAuthorityViewClass::RuntimeServices),
    ];

    const fn expected(
        class: seal::RuntimeAuthorityViewClass,
    ) -> seal::RuntimeAuthorityExpectedViewDraft<'static> {
        seal::RuntimeAuthorityExpectedViewDraft {
            class,
            identity: HASH_A,
        }
    }

    fn host_draft() -> RuntimeAuthoritySealHostDraft<'static> {
        RuntimeAuthoritySealHostDraft {
            session_id: "session.pilot",
            dispatch_id: "dispatch.pilot",
            session_lineage_identity: HASH_A,
            durable_binding_identity: HASH_A,
            authority_profile_identity: HASH_A,
            enforcement_observation_identity: HASH_A,
            operation_set_identity: HASH_A,
            adapter_cohort_identity: HASH_A,
            resource_profile_identity: HASH_A,
            resource_budget_identity: HASH_A,
            runtime_services_identity: HASH_A,
            operation_identities: OPERATIONS,
            resource_identities: RESOURCES,
            expected_views: EXPECTED,
        }
    }

    #[derive(Clone, Copy)]
    enum FakeFailure {
        CommitMismatch,
        DenyRecheck,
        Effect,
        Cleanup,
    }

    #[derive(Default)]
    struct FakeService {
        failure: Option<FakeFailure>,
        effect_calls: usize,
        cleanup_calls: usize,
        trace: Rc<RefCell<Vec<&'static str>>>,
    }

    impl FakeService {
        fn with_trace(trace: &Rc<RefCell<Vec<&'static str>>>) -> Self {
            Self {
                trace: Rc::clone(trace),
                ..Self::default()
            }
        }
    }

    impl RuntimeAuthorityObservationService for FakeService {
        fn observe(
            &mut self,
            expected: &[seal::RuntimeAuthorityExpectedViewDraft<'_>],
        ) -> Result<Vec<RuntimeAuthorityHostObservedView>, RuntimeAuthorityServiceError> {
            self.trace.borrow_mut().push(EVENT_OBSERVE);
            Ok(expected
                .iter()
                .map(|view| RuntimeAuthorityHostObservedView {
                    class: view.class,
                    identity: String::from(view.identity),
                    status: seal::RuntimeAuthorityObservationStatus::Current,
                })
                .collect())
        }
    }

    impl RuntimeAuthorityReceiptCommitService for FakeService {
        fn commit(
            &mut self,
            receipt: &seal::RuntimeAuthoritySealReceipt,
        ) -> Result<String, RuntimeAuthorityServiceError> {
            self.trace.borrow_mut().push(EVENT_COMMIT);
            Ok(
                if matches!(self.failure, Some(FakeFailure::CommitMismatch)) {
                    String::from(HASH_B)
                } else {
                    receipt.receipt_identity.clone()
                },
            )
        }
    }

    impl RuntimeAuthorityCurrentRecheckService for FakeService {
        fn recheck(
            &mut self,
            unit: &seal::RuntimeAuthorityUnit,
        ) -> Result<RuntimeAuthorityHostRecheck, RuntimeAuthorityServiceError> {
            self.trace.borrow_mut().push(EVENT_RECHECK);
            Ok(RuntimeAuthorityHostRecheck {
                session_lineage_identity: String::from(unit.session_lineage_identity()),
                authority_profile_identity: String::from(unit.authority_profile_identity()),
                durable_binding_identity: String::from(unit.durable_binding_identity()),
                enforcement_observation_identity: String::from(
                    unit.enforcement_observation_identity(),
                ),
                operation_set_identity: String::from(unit.operation_set_identity()),
                adapter_cohort_identity: String::from(unit.adapter_cohort_identity()),
                resource_profile_identity: String::from(unit.resource_profile_identity()),
                resource_budget_identity: String::from(unit.resource_budget_identity()),
                runtime_services_identity: String::from(unit.runtime_services_identity()),
                operation_identities: unit.operation_identities().to_vec(),
                resource_identities: unit.resource_identities().to_vec(),
                admitted: !matches!(self.failure, Some(FakeFailure::DenyRecheck)),
            })
        }
    }

    impl RuntimeAuthorityEffectService for FakeService {
        fn dispatch(
            &mut self,
            _unit: &seal::RuntimeAuthorityUnit,
        ) -> Result<RuntimeAuthorityEffectObservation, RuntimeAuthorityServiceError> {
            self.trace.borrow_mut().push(EVENT_EFFECT);
            self.effect_calls += 1;
            if matches!(self.failure, Some(FakeFailure::Effect)) {
                Err(RuntimeAuthorityServiceError::new("fake-effect-failed"))
            } else {
                Ok(RuntimeAuthorityEffectObservation {
                    effect_receipt_identity: String::from(HASH_A),
                })
            }
        }
    }

    impl RuntimeAuthorityCleanupService for FakeService {
        fn cleanup(
            &mut self,
            _unit: &seal::RuntimeAuthorityUnit,
        ) -> Result<String, RuntimeAuthorityServiceError> {
            self.trace.borrow_mut().push(EVENT_CLEANUP);
            self.cleanup_calls += 1;
            if matches!(self.failure, Some(FakeFailure::Cleanup)) {
                Err(RuntimeAuthorityServiceError::new("fake-cleanup-failed"))
            } else {
                Ok(String::from(HASH_B))
            }
        }
    }

    fn receipt_draft() -> seal::RuntimeAuthorityReceiptDraft<'static> {
        seal::RuntimeAuthorityReceiptDraft {
            sensitive: seal::RuntimeAuthoritySensitiveDraft::default(),
            semantic_claims: &[],
        }
    }

    #[test]
    fn fake_dispatch_commits_seal_before_effect_and_cleans_up() {
        // r[verify agent_harness_runtime.runtime_authority_seal.phase.valid]
        // r[verify agent_harness_runtime.runtime_authority_seal.receipt.safe]
        let trace = Rc::new(RefCell::new(Vec::new()));
        let mut observe = FakeService::with_trace(&trace);
        let mut commit = FakeService::with_trace(&trace);
        let mut recheck = FakeService::with_trace(&trace);
        let mut effect = FakeService::with_trace(&trace);
        let mut cleanup = FakeService::with_trace(&trace);
        let report = execute_runtime_authority_sealed_dispatch(
            host_draft(),
            receipt_draft(),
            &mut RuntimeAuthoritySealHostServices {
                observation: &mut observe,
                receipt_commit: &mut commit,
                current_recheck: &mut recheck,
                effect: &mut effect,
                cleanup: &mut cleanup,
            },
        )
        .expect("fake dispatch");
        assert_eq!(
            report.unit.state(),
            seal::RuntimeAuthorityUnitState::Completed
        );
        assert!(report.seal_committed_before_effect);
        assert_eq!(report.events.len(), EXPECTED_EVENT_COUNT);
        assert_eq!(effect.effect_calls, 1);
        assert_eq!(cleanup.cleanup_calls, 1);
        assert_eq!(
            trace.borrow().as_slice(),
            [
                EVENT_OBSERVE,
                EVENT_COMMIT,
                EVENT_RECHECK,
                EVENT_EFFECT,
                EVENT_CLEANUP,
            ]
        );
    }

    #[test]
    fn bad_commit_and_denied_recheck_never_call_effect() {
        // r[verify agent_harness_runtime.runtime_authority_seal.boundary.authorization]
        let mut observe = FakeService::default();
        let mut commit = FakeService {
            failure: Some(FakeFailure::CommitMismatch),
            ..FakeService::default()
        };
        let mut recheck = FakeService::default();
        let mut effect = FakeService::default();
        let mut cleanup = FakeService::default();
        let error = execute_runtime_authority_sealed_dispatch(
            host_draft(),
            receipt_draft(),
            &mut RuntimeAuthoritySealHostServices {
                observation: &mut observe,
                receipt_commit: &mut commit,
                current_recheck: &mut recheck,
                effect: &mut effect,
                cleanup: &mut cleanup,
            },
        )
        .expect_err("commit mismatch");
        assert_eq!(error.stage(), RuntimeAuthoritySealHostStage::CommitReceipt);
        assert_eq!(effect.effect_calls, 0);

        commit.failure = None;
        recheck.failure = Some(FakeFailure::DenyRecheck);
        let error = execute_runtime_authority_sealed_dispatch(
            host_draft(),
            receipt_draft(),
            &mut RuntimeAuthoritySealHostServices {
                observation: &mut observe,
                receipt_commit: &mut commit,
                current_recheck: &mut recheck,
                effect: &mut effect,
                cleanup: &mut cleanup,
            },
        )
        .expect_err("recheck denied");
        assert_eq!(error.stage(), RuntimeAuthoritySealHostStage::CurrentRecheck);
        assert_eq!(effect.effect_calls, 0);
    }

    #[test]
    fn cleanup_failure_is_reported_after_effect_completion() {
        let mut observe = FakeService::default();
        let mut commit = FakeService::default();
        let mut recheck = FakeService::default();
        let mut effect = FakeService::default();
        let mut cleanup = FakeService {
            failure: Some(FakeFailure::Cleanup),
            ..FakeService::default()
        };
        let error = execute_runtime_authority_sealed_dispatch(
            host_draft(),
            receipt_draft(),
            &mut RuntimeAuthoritySealHostServices {
                observation: &mut observe,
                receipt_commit: &mut commit,
                current_recheck: &mut recheck,
                effect: &mut effect,
                cleanup: &mut cleanup,
            },
        )
        .expect_err("cleanup failure");
        assert_eq!(error.stage(), RuntimeAuthoritySealHostStage::Cleanup);
        assert_eq!(error.service_code(), Some("fake-cleanup-failed"));
        assert_eq!(effect.effect_calls, 1);
        assert_eq!(cleanup.cleanup_calls, 1);
    }

    #[test]
    fn failed_effect_runs_cleanup_and_returns_safe_error() {
        // r[verify agent_harness_runtime.runtime_authority_seal.verification.closeout]
        let mut observe = FakeService::default();
        let mut commit = FakeService::default();
        let mut recheck = FakeService::default();
        let mut effect = FakeService {
            failure: Some(FakeFailure::Effect),
            ..FakeService::default()
        };
        let mut cleanup = FakeService::default();
        let error = execute_runtime_authority_sealed_dispatch(
            host_draft(),
            receipt_draft(),
            &mut RuntimeAuthoritySealHostServices {
                observation: &mut observe,
                receipt_commit: &mut commit,
                current_recheck: &mut recheck,
                effect: &mut effect,
                cleanup: &mut cleanup,
            },
        )
        .expect_err("effect failure");
        assert_eq!(error.stage(), RuntimeAuthoritySealHostStage::Effect);
        assert_eq!(error.service_code(), Some("fake-effect-failed"));
        assert_eq!(effect.effect_calls, 1);
        assert_eq!(cleanup.cleanup_calls, 1);
    }
}
