//! Thin host seam for current execution backend enforcement observations.

use animus_runtime::execution_backend_planner::{
    BackendEnforcementResult, CurrentBackendEnforcementService, ExecutionBackendPlan,
    recheck_execution_backend,
};

/// Host-owned backend enforcement recheck seam.
pub struct ExecutionBackendHost<'a> {
    service: &'a dyn CurrentBackendEnforcementService,
}

impl<'a> ExecutionBackendHost<'a> {
    /// Creates a host seam over an explicitly supplied service.
    #[must_use]
    pub const fn new(service: &'a dyn CurrentBackendEnforcementService) -> Self {
        Self { service }
    }

    /// Rechecks the selected backend before the caller executes effects.
    #[must_use]
    pub fn recheck(&self, plan: &ExecutionBackendPlan) -> BackendEnforcementResult {
        recheck_execution_backend(self.service, plan)
    }
}

#[cfg(test)]
mod tests {
    use super::ExecutionBackendHost;
    use animus_runtime::execution_backend_planner::{
        AdapterId, ApplicationHostBindingId, BackendEnforcementDenial,
        BackendEnforcementDenialRecord, BackendEnforcementResult, BackendId, BackendPlanId,
        BackendResourceLimits, CurrentBackendEnforcementService, CurrentBackendObservation,
        CurrentBackendServiceError, EffectPermissionId, EnforcementProfileId,
        ExecutionBackendClass, ExecutionBackendPlan, ObservationId, ResourceProfileId,
        RuntimeAuthorityUnitId, WorkloadId, WorkloadRequirement,
    };

    const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
    const HASH_D: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
    const HASH_E: &str = "eeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee";
    const HASH_F: &str = "ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
    const HASH_1: &str = "1111111111111111111111111111111111111111111111111111111111111111";
    const RESOURCE_LIMIT: u64 = 1;

    fn plan() -> ExecutionBackendPlan {
        ExecutionBackendPlan {
            plan_id: BackendPlanId::parse(HASH_A).expect("plan"),
            workload_id: WorkloadId::parse(HASH_B).expect("workload"),
            backend_id: BackendId::parse(HASH_C).expect("backend"),
            class: ExecutionBackendClass::EmbeddedComponent,
            adapter_id: AdapterId::parse(HASH_D).expect("adapter"),
            application_host_binding_id: ApplicationHostBindingId::parse(HASH_E).expect("binding"),
            runtime_authority_unit_id: RuntimeAuthorityUnitId::parse(HASH_F).expect("authority"),
            effect_permission_id: EffectPermissionId::parse(HASH_1).expect("permission"),
            resource_profile_id: ResourceProfileId::parse(HASH_A).expect("resource"),
            enforcement_profile_id: EnforcementProfileId::parse(HASH_B).expect("enforcement"),
            policy_identity: HASH_C.to_string(),
            requirements: vec![WorkloadRequirement::WasmComponent],
            resources: BackendResourceLimits {
                memory_bytes: RESOURCE_LIMIT,
                storage_bytes: RESOURCE_LIMIT,
                cpu_milliseconds: RESOURCE_LIMIT,
                wall_milliseconds: RESOURCE_LIMIT,
            },
            unnecessary_broader_classes: Vec::new(),
        }
    }

    struct FakeService {
        result: Result<CurrentBackendObservation, CurrentBackendServiceError>,
    }

    impl CurrentBackendEnforcementService for FakeService {
        fn observe(
            &self,
            _plan: &ExecutionBackendPlan,
        ) -> Result<CurrentBackendObservation, CurrentBackendServiceError> {
            self.result.clone()
        }
    }

    fn observation(plan: &ExecutionBackendPlan) -> CurrentBackendObservation {
        CurrentBackendObservation {
            observation_id: ObservationId::parse(HASH_D).expect("observation"),
            backend_id: plan.backend_id.clone(),
            adapter_id: plan.adapter_id.clone(),
            application_host_binding_id: plan.application_host_binding_id.clone(),
            runtime_authority_unit_id: plan.runtime_authority_unit_id.clone(),
            effect_permission_id: plan.effect_permission_id.clone(),
            resource_profile_id: plan.resource_profile_id.clone(),
            enforcement_profile_id: plan.enforcement_profile_id.clone(),
            available: true,
            resources_admitted: true,
            enforcement_current: true,
        }
    }

    #[test]
    fn supplied_service_can_admit_exact_current_plan() {
        let plan = plan();
        let service = FakeService {
            result: Ok(observation(&plan)),
        };
        assert!(matches!(
            ExecutionBackendHost::new(&service).recheck(&plan),
            BackendEnforcementResult::Admitted(_)
        ));
    }

    #[test]
    fn unavailable_service_denies_before_effects() {
        let service = FakeService {
            result: Err(CurrentBackendServiceError::Unavailable),
        };
        assert_eq!(
            ExecutionBackendHost::new(&service).recheck(&plan()),
            BackendEnforcementResult::Denied(BackendEnforcementDenialRecord {
                reason: BackendEnforcementDenial::ObservationUnavailable,
                observation_id: None,
            })
        );
    }
}
