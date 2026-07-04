/// Evaluates activation after discovery, project admission, and runtime policy.
#[must_use]
pub fn activate(request: super::ActivationRequest<'_>) -> super::Activation {
    assert!(
        request.policy.model_roles.len() <= super::super::MAX_ITEMS,
        "model-role policy stays bounded"
    );
    assert!(
        request.policy.capabilities.len() <= super::super::MAX_ITEMS,
        "capability policy stays bounded"
    );
    let admission = project_admission_status(request.manifest, request.admission);
    if admission == super::AdmissionStatus::Denied {
        return denied_activation(admission);
    }
    active_activation(request, admission)
}

fn denied_activation(admission_status: super::AdmissionStatus) -> super::Activation {
    let mut issues = super::super::Collection::with_capacity(1);
    issues.push(super::ActivationIssue::ProjectAdmissionRequired);
    super::Activation {
        status: super::ActivationStatus::Denied,
        admission_status,
        issues,
        prompt_fragments: super::super::Collection::new(),
        selected_model_role: None,
        effective_capabilities: super::super::Collection::new(),
        effective_tools: super::super::Collection::new(),
        effective_memory_scopes: super::super::Collection::new(),
        effective_context_reference_scopes: super::super::Collection::new(),
        effective_workflow_abilities: super::super::Collection::new(),
        denied_escalations: super::super::Collection::new(),
    }
}

fn active_activation(
    request: super::ActivationRequest<'_>,
    admission_status: super::AdmissionStatus,
) -> super::Activation {
    assert!(
        request.manifest.prompt_fragments().len() <= super::super::MAX_ITEMS,
        "activation prompt fragments are bounded"
    );
    assert_eq!(
        admission_status,
        project_admission_status(request.manifest, request.admission),
        "activation admission status matches the request"
    );
    let mut denied = super::super::Collection::with_capacity(super::super::MAX_ITEMS);
    let selected_model_role =
        select_model_role(request.manifest, request.policy.model_roles, &mut denied);
    let sets = effective_sets(request, &mut denied);
    build_active(
        request.manifest,
        admission_status,
        selected_model_role,
        sets,
        denied,
    )
}

struct EffectiveSets {
    effective_capabilities: super::super::Collection<super::super::Text>,
    effective_tools: super::super::Collection<super::super::Text>,
    effective_memory_scopes: super::super::Collection<super::super::Text>,
    effective_context_reference_scopes: super::super::Collection<super::super::Text>,
    effective_workflow_abilities: super::super::Collection<super::super::Text>,
}

fn effective_sets(
    request: super::ActivationRequest<'_>,
    denied: &mut super::super::Collection<super::DeniedEscalation>,
) -> EffectiveSets {
    assert!(
        request.manifest.capability_ceilings().len() <= super::super::MAX_ITEMS,
        "capability requests are bounded"
    );
    assert!(
        request.manifest.workflow_abilities().len() <= super::super::MAX_ITEMS,
        "workflow requests are bounded"
    );
    EffectiveSets {
        effective_capabilities: filter_allowed(
            super::AbilityKind::Capability,
            request.manifest.capability_ceilings(),
            request.policy.capabilities,
            denied,
        ),
        effective_tools: filter_allowed(
            super::AbilityKind::Tool,
            request.manifest.toolset_limits(),
            request.policy.tools,
            denied,
        ),
        effective_memory_scopes: filter_allowed(
            super::AbilityKind::Memory,
            request.manifest.memory_scopes(),
            request.policy.memory_scopes,
            denied,
        ),
        effective_context_reference_scopes: filter_allowed(
            super::AbilityKind::ContextReference,
            request.manifest.context_reference_scopes(),
            request.policy.context_reference_scopes,
            denied,
        ),
        effective_workflow_abilities: filter_allowed(
            super::AbilityKind::Workflow,
            request.manifest.workflow_abilities(),
            request.policy.workflow_abilities,
            denied,
        ),
    }
}

fn build_active(
    manifest: &super::super::Manifest,
    admission_status: super::AdmissionStatus,
    selected_model_role: Option<super::super::Text>,
    sets: EffectiveSets,
    denied_escalations: super::super::Collection<super::DeniedEscalation>,
) -> super::Activation {
    let mut issues = super::super::Collection::with_capacity(1);
    if !denied_escalations.is_empty() {
        issues.push(super::ActivationIssue::CapabilityEscalationDenied);
    }
    super::Activation {
        status: super::ActivationStatus::Active,
        admission_status,
        issues,
        prompt_fragments: manifest.prompt_fragments.clone(),
        selected_model_role,
        effective_capabilities: sets.effective_capabilities,
        effective_tools: sets.effective_tools,
        effective_memory_scopes: sets.effective_memory_scopes,
        effective_context_reference_scopes: sets.effective_context_reference_scopes,
        effective_workflow_abilities: sets.effective_workflow_abilities,
        denied_escalations,
    }
}

fn project_admission_status(
    manifest: &super::super::Manifest,
    admission: super::Admission<'_>,
) -> super::AdmissionStatus {
    if manifest.source_scope() != super::super::SourceScope::Project {
        return super::AdmissionStatus::NotRequired;
    }
    if super::super::contains_label(
        admission.admitted_manifest_identities,
        manifest.manifest_identity(),
    ) {
        return super::AdmissionStatus::Admitted;
    }
    super::AdmissionStatus::Denied
}

fn select_model_role(
    manifest: &super::super::Manifest,
    model_roles: &[&str],
    denied_escalations: &mut super::super::Collection<super::DeniedEscalation>,
) -> Option<super::super::Text> {
    if super::super::contains_label(model_roles, manifest.model_preference()) {
        return Some(super::super::Text::from(manifest.model_preference()));
    }
    denied_escalations.push(denied_escalation(
        super::AbilityKind::ModelRole,
        manifest.model_preference(),
    ));
    None
}

fn filter_allowed(
    kind: super::AbilityKind,
    requested: &[super::super::Text],
    allowed: &[&str],
    denied_escalations: &mut super::super::Collection<super::DeniedEscalation>,
) -> super::super::Collection<super::super::Text> {
    assert!(
        requested.len() <= super::super::MAX_ITEMS,
        "requested labels are bounded"
    );
    assert!(
        allowed.len() <= super::super::MAX_ITEMS,
        "allowed labels are bounded"
    );
    let mut effective = super::super::Collection::with_capacity(requested.len());
    for value in requested {
        if super::super::contains_label(allowed, value.as_str()) {
            effective.push(value.clone());
        } else {
            denied_escalations.push(denied_escalation(kind, value));
        }
    }
    effective
}

fn denied_escalation(kind: super::AbilityKind, value: &str) -> super::DeniedEscalation {
    super::DeniedEscalation {
        kind,
        value: super::super::Text::from(value),
    }
}
