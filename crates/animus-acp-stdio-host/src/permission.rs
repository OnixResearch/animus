//! Pure ACP permission and authority-intersection planning.

#![allow(missing_docs)]

use crate::model::{
    AcpClientPermissionDecision, AcpFailureClass, AcpPermissionAdmission, AcpPermissionInput,
    AcpPermissionOutcome, AcpPermissionPlan, AcpPermissionStatus,
};

#[must_use]
// r[impl agent_harness_runtime.acp_stdio_host.permission_bridge]
pub fn plan_acp_permission(input: &AcpPermissionInput) -> AcpPermissionPlan {
    let failure = permission_failure(input);
    let status = if failure.is_some() {
        AcpPermissionStatus::Denied
    } else if input.confirmation_required {
        AcpPermissionStatus::ClientDecisionRequired
    } else {
        AcpPermissionStatus::PreApproved
    };
    AcpPermissionPlan {
        request_id: input.request_id.clone(),
        session_id: input.session_id.clone(),
        tool_name: input.tool_name.clone(),
        tool_correlation_id: input.tool_correlation_id.clone(),
        status,
        failure,
    }
}

#[must_use]
pub fn apply_permission_decision(
    plan: &AcpPermissionPlan,
    decision: Option<AcpClientPermissionDecision>,
) -> AcpPermissionOutcome {
    if let Some(failure) = plan.failure {
        return AcpPermissionOutcome {
            request_id: plan.request_id.clone(),
            client_decision: decision,
            dispatch_allowed: false,
            failure: Some(failure),
        };
    }
    match plan.status {
        AcpPermissionStatus::PreApproved => AcpPermissionOutcome {
            request_id: plan.request_id.clone(),
            client_decision: None,
            dispatch_allowed: true,
            failure: None,
        },
        AcpPermissionStatus::ClientDecisionRequired => client_decision_outcome(plan, decision),
        AcpPermissionStatus::Denied => AcpPermissionOutcome {
            request_id: plan.request_id.clone(),
            client_decision: decision,
            dispatch_allowed: false,
            failure: Some(AcpFailureClass::PermissionDenied),
        },
    }
}

fn client_decision_outcome(
    plan: &AcpPermissionPlan,
    decision: Option<AcpClientPermissionDecision>,
) -> AcpPermissionOutcome {
    let (dispatch_allowed, failure) = match decision {
        Some(AcpClientPermissionDecision::AllowOnce | AcpClientPermissionDecision::AllowAlways) => {
            (true, None)
        }
        Some(AcpClientPermissionDecision::DenyOnce | AcpClientPermissionDecision::DenyAlways) => {
            (false, Some(AcpFailureClass::PermissionDenied))
        }
        Some(AcpClientPermissionDecision::Cancelled) => {
            (false, Some(AcpFailureClass::CancellationAmbiguous))
        }
        None => (false, Some(AcpFailureClass::PermissionUnavailable)),
    };
    AcpPermissionOutcome {
        request_id: plan.request_id.clone(),
        client_decision: decision,
        dispatch_allowed,
        failure,
    }
}

fn permission_failure(input: &AcpPermissionInput) -> Option<AcpFailureClass> {
    if input.sandbox != AcpPermissionAdmission::Admitted {
        return Some(AcpFailureClass::SandboxMismatch);
    }
    if input.executor != AcpPermissionAdmission::Admitted {
        return Some(AcpFailureClass::ExecutorUnavailable);
    }
    let authority = [
        input.policy,
        input.permission_mode,
        input.authorization,
        input.capability,
        input.profile,
    ];
    if authority.contains(&AcpPermissionAdmission::Unavailable) {
        return Some(AcpFailureClass::PermissionUnavailable);
    }
    if authority.contains(&AcpPermissionAdmission::Denied) {
        return Some(AcpFailureClass::PermissionWidening);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn client_allow_permits_only_fully_admitted_request() {
        let plan = plan_acp_permission(&admitted_input());
        let outcome =
            apply_permission_decision(&plan, Some(AcpClientPermissionDecision::AllowOnce));

        assert_eq!(plan.status, AcpPermissionStatus::ClientDecisionRequired);
        assert!(outcome.dispatch_allowed);
        assert_eq!(outcome.failure, None);
    }

    #[test]
    // r[verify agent_harness_runtime.acp_stdio_host.permission_bridge]
    fn client_allow_cannot_widen_denied_capability_or_sandbox() {
        let denied = plan_acp_permission(&AcpPermissionInput {
            capability: AcpPermissionAdmission::Denied,
            ..admitted_input()
        });
        let denied_outcome =
            apply_permission_decision(&denied, Some(AcpClientPermissionDecision::AllowAlways));
        assert!(!denied_outcome.dispatch_allowed);
        assert_eq!(
            denied_outcome.failure,
            Some(AcpFailureClass::PermissionWidening)
        );

        let sandbox = plan_acp_permission(&AcpPermissionInput {
            sandbox: AcpPermissionAdmission::Unavailable,
            ..admitted_input()
        });
        let sandbox_outcome =
            apply_permission_decision(&sandbox, Some(AcpClientPermissionDecision::AllowOnce));
        assert!(!sandbox_outcome.dispatch_allowed);
        assert_eq!(
            sandbox_outcome.failure,
            Some(AcpFailureClass::SandboxMismatch)
        );
    }

    #[test]
    fn absent_or_cancelled_client_decision_fails_closed() {
        let plan = plan_acp_permission(&admitted_input());
        let absent = apply_permission_decision(&plan, None);
        let cancelled =
            apply_permission_decision(&plan, Some(AcpClientPermissionDecision::Cancelled));

        assert_eq!(absent.failure, Some(AcpFailureClass::PermissionUnavailable));
        assert_eq!(
            cancelled.failure,
            Some(AcpFailureClass::CancellationAmbiguous)
        );
    }

    fn admitted_input() -> AcpPermissionInput {
        AcpPermissionInput {
            request_id: String::from("permission.fixture"),
            session_id: String::from("session.fixture"),
            tool_name: String::from("read"),
            tool_correlation_id: String::from("tool.fixture"),
            policy: AcpPermissionAdmission::Admitted,
            permission_mode: AcpPermissionAdmission::Admitted,
            authorization: AcpPermissionAdmission::Admitted,
            capability: AcpPermissionAdmission::Admitted,
            sandbox: AcpPermissionAdmission::Admitted,
            profile: AcpPermissionAdmission::Admitted,
            executor: AcpPermissionAdmission::Admitted,
            confirmation_required: true,
        }
    }
}
