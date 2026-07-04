use super::check::*;
use super::context::*;
use super::types::*;
use super::*;

impl super::super::PluginDescriptor {
    /// Plans a host-owned registry operation without mutating state or dispatching executors.
    #[must_use]
    pub fn plan_registry_operation(
        &self,
        draft: &PluginRegistryOperationDraft<'_>,
    ) -> PluginRegistryPlan {
        let host = match validate_host_compatibility(&draft.host) {
            Ok(host) => host,
            Err(()) => {
                return self.blocked_plan(
                    draft.operation,
                    draft.entrypoint_id,
                    PluginDiagnosticKind::MalformedContext,
                );
            }
        };

        match draft.operation {
            PluginRegistryOperationKind::Link => self.plan_link_operation(draft.operation),
            PluginRegistryOperationKind::Unlink
            | PluginRegistryOperationKind::Enable
            | PluginRegistryOperationKind::Disable
            | PluginRegistryOperationKind::List
            | PluginRegistryOperationKind::ActionList => self.planned_operation(draft.operation),
            PluginRegistryOperationKind::PlanInvocation
            | PluginRegistryOperationKind::PlanEvent
            | PluginRegistryOperationKind::PlanPaneEntrypoint => {
                self.plan_dispatch_like_operation(draft, &host)
            }
        }
    }

    fn plan_link_operation(&self, operation: PluginRegistryOperationKind) -> PluginRegistryPlan {
        let mut diagnostics = Collection::with_capacity(self.declared_roles.len());
        for role in &self.declared_roles {
            if matches!(role.kind(), PluginRoleKind::EventHook) {
                if let Some(event_name) = role.event_name() {
                    if !is_known_event(event_name) {
                        diagnostics.push(diagnostic(
                            PluginDiagnosticKind::UnknownEvent,
                            PluginDiagnosticSeverity::Warning,
                        ));
                    }
                }
            }
            if matches!(role.kind(), PluginRoleKind::LinkHandler) && role.link_scheme().is_none() {
                diagnostics.push(diagnostic(
                    PluginDiagnosticKind::MissingOptionalMetadata,
                    PluginDiagnosticSeverity::Warning,
                ));
            }
        }
        if diagnostics.is_empty() {
            self.planned_operation(operation)
        } else {
            self.plan_with_diagnostics(
                operation,
                None,
                PluginRegistryPlanStatus::Warning,
                diagnostics,
            )
        }
    }

    fn plan_dispatch_like_operation(
        &self,
        draft: &PluginRegistryOperationDraft<'_>,
        host: &PluginHostCompatibility,
    ) -> PluginRegistryPlan {
        if !draft.plugin_enabled {
            return self.blocked_plan(
                draft.operation,
                draft.entrypoint_id,
                PluginDiagnosticKind::DisabledPlugin,
            );
        }
        if !draft.policy_allows {
            return self.blocked_plan(
                draft.operation,
                draft.entrypoint_id,
                PluginDiagnosticKind::PolicyDenied,
            );
        }
        if draft.context.is_none() {
            return self.blocked_plan(
                draft.operation,
                draft.entrypoint_id,
                PluginDiagnosticKind::MalformedContext,
            );
        }
        let Some(role) = self.target_role(draft) else {
            return self.blocked_plan(
                draft.operation,
                draft.entrypoint_id,
                target_failure_kind(draft.operation),
            );
        };
        let compatibility_diagnostics = compatibility_diagnostics(role.compatibility(), host);
        if !compatibility_diagnostics.is_empty() {
            return self.plan_with_diagnostics(
                draft.operation,
                Some(role.id()),
                PluginRegistryPlanStatus::Blocked,
                compatibility_diagnostics,
            );
        }
        self.plan_for_role(draft.operation, role)
    }

    fn target_role(&self, draft: &PluginRegistryOperationDraft<'_>) -> Option<&PluginRole> {
        match draft.operation {
            PluginRegistryOperationKind::PlanInvocation => {
                let id = draft.entrypoint_id?;
                self.declared_roles.iter().find(|role| {
                    role.id() == id
                        && matches!(
                            role.kind(),
                            PluginRoleKind::Action | PluginRoleKind::LinkHandler
                        )
                })
            }
            PluginRegistryOperationKind::PlanPaneEntrypoint => {
                let id = draft.entrypoint_id?;
                self.declared_roles.iter().find(|role| {
                    role.id() == id && matches!(role.kind(), PluginRoleKind::HostPaneEntrypoint)
                })
            }
            PluginRegistryOperationKind::PlanEvent => {
                let event_name = draft.event_name?;
                if !is_known_event(event_name) {
                    return None;
                }
                self.declared_roles.iter().find(|role| {
                    matches!(role.kind(), PluginRoleKind::EventHook)
                        && role.event_name() == Some(event_name)
                })
            }
            PluginRegistryOperationKind::Link
            | PluginRegistryOperationKind::Unlink
            | PluginRegistryOperationKind::Enable
            | PluginRegistryOperationKind::Disable
            | PluginRegistryOperationKind::List
            | PluginRegistryOperationKind::ActionList => None,
        }
    }

    fn planned_operation(&self, operation: PluginRegistryOperationKind) -> PluginRegistryPlan {
        self.plan_with_diagnostics(
            operation,
            None,
            PluginRegistryPlanStatus::Planned,
            Collection::new(),
        )
    }

    fn plan_for_role(
        &self,
        operation: PluginRegistryOperationKind,
        role: &PluginRole,
    ) -> PluginRegistryPlan {
        PluginRegistryPlan {
            operation,
            manifest_identity: Text::from(self.identity()),
            entrypoint_id: Some(Text::from(role.id())),
            status: PluginRegistryPlanStatus::Planned,
            diagnostics: Collection::new(),
            command_label: role.command_label().map(Text::from),
            artifact_label: role.artifact_label().map(Text::from),
        }
    }

    fn blocked_plan(
        &self,
        operation: PluginRegistryOperationKind,
        entrypoint_id: Option<&str>,
        kind: PluginDiagnosticKind,
    ) -> PluginRegistryPlan {
        let diagnostics = Collection::from([diagnostic(kind, PluginDiagnosticSeverity::Failure)]);
        self.plan_with_diagnostics(
            operation,
            entrypoint_id,
            PluginRegistryPlanStatus::Blocked,
            diagnostics,
        )
    }

    fn plan_with_diagnostics(
        &self,
        operation: PluginRegistryOperationKind,
        entrypoint_id: Option<&str>,
        status: PluginRegistryPlanStatus,
        diagnostics: Collection<PluginDiagnostic>,
    ) -> PluginRegistryPlan {
        PluginRegistryPlan {
            operation,
            manifest_identity: Text::from(self.identity()),
            entrypoint_id: entrypoint_id.map(Text::from),
            status,
            diagnostics,
            command_label: None,
            artifact_label: None,
        }
    }
}
