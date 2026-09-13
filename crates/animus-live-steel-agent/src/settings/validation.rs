pub(super) fn validate_fixed_components(
    raw: &super::shape::LiveSteelAgentConfig,
) -> Result<(), super::LiveSteelAgentConfigError> {
    validate_label(raw.application_id.as_str())?;
    validate_label(raw.branch_id.as_str())?;
    if raw.schema != super::CONFIG_SCHEMA {
        return Err(super::LiveSteelAgentConfigError::ComponentMismatch);
    }
    validate_blake3(raw.session_identity.as_str())?;
    let actual = [
        raw.components.provider.as_str(),
        raw.components.durable_store.as_str(),
        raw.components.coding_session.as_str(),
        raw.components.steel_kernel.as_str(),
        raw.components.resumable_bridge.as_str(),
        raw.components.rlm_executor.as_str(),
        raw.components.child_service.as_str(),
        raw.components.skill_loader.as_str(),
        raw.components.control.as_str(),
        raw.components.event_sink.as_str(),
        raw.components.receipt_sink.as_str(),
    ];
    let expected = [
        super::COMPONENT_PROVIDER,
        super::COMPONENT_STORE,
        super::COMPONENT_CODING_SESSION,
        super::COMPONENT_KERNEL,
        super::COMPONENT_BRIDGE,
        super::COMPONENT_EXECUTOR,
        super::COMPONENT_CHILD,
        super::COMPONENT_SKILLS,
        super::COMPONENT_CONTROL,
        super::COMPONENT_EVENTS,
        super::COMPONENT_RECEIPTS,
    ];
    if actual != expected {
        return Err(super::LiveSteelAgentConfigError::ComponentMismatch);
    }
    debug_assert_eq!(actual, expected);
    debug_assert_eq!(raw.session_identity.len(), super::BLAKE3_HEX_CHARS);
    Ok(())
}

pub(super) fn validate_store(
    store: &super::shape::StoreConfig,
) -> Result<(), super::LiveSteelAgentConfigError> {
    for label in [
        store.capability_label.as_str(),
        store.session_journal.as_str(),
        store.session_lock.as_str(),
        store.kernel_journal.as_str(),
        store.kernel_lock.as_str(),
    ] {
        validate_label(label)?;
    }
    let has_distinct_leaf_names = store.session_journal != store.session_lock
        && store.session_journal != store.kernel_journal
        && store.session_journal != store.kernel_lock
        && store.session_lock != store.kernel_journal
        && store.session_lock != store.kernel_lock
        && store.kernel_journal != store.kernel_lock;
    let has_required_posture = store.durability == super::DURABILITY
        && store.locking == super::LOCKING
        && store.privacy == super::PRIVACY;
    if !has_required_posture || !has_distinct_leaf_names {
        return Err(super::LiveSteelAgentConfigError::StoreMismatch);
    }
    debug_assert!(has_required_posture);
    debug_assert!(has_distinct_leaf_names);
    Ok(())
}

pub(super) fn validate_services(
    services: &super::shape::ServiceConfig,
) -> Result<(), super::LiveSteelAgentConfigError> {
    let labels = [
        services.child.as_str(),
        services.context.as_str(),
        services.control.as_str(),
        services.events.as_str(),
        services.receipts.as_str(),
    ];
    for label in labels {
        validate_label(label)?;
    }
    Ok(())
}

pub(super) fn validate_authority(
    authority: &super::shape::AuthorityConfig,
) -> Result<(), super::LiveSteelAgentConfigError> {
    for identity in [
        authority.component_identity.as_str(),
        authority.authorization_identity.as_str(),
        authority.route_binding_identity.as_str(),
        authority.route_scope_identity.as_str(),
        authority.operation_identity.as_str(),
        authority.resource_identity.as_str(),
    ] {
        validate_blake3(identity)?;
    }
    if authority.generation == 0 {
        return Err(super::LiveSteelAgentConfigError::InvalidBound);
    }
    Ok(())
}

pub(super) fn validate_evidence(
    evidence: &super::shape::EvidenceConfig,
) -> Result<(), super::LiveSteelAgentConfigError> {
    if evidence.policy != super::EVIDENCE_POLICY
        || evidence.redaction_marker != super::EVIDENCE_REDACTION
    {
        return Err(super::LiveSteelAgentConfigError::EvidenceMismatch);
    }
    exact_sorted_set(&evidence.non_claims, super::REQUIRED_NON_CLAIMS)
        .map_err(|_error| super::LiveSteelAgentConfigError::EvidenceMismatch)
}

pub(super) fn validate_operations(
    actual: &[String],
) -> Result<(), super::LiveSteelAgentConfigError> {
    // r[impl agent_harness_runtime.pueue_host.live_shell_tool_route]
    validate_labels(actual, super::MAX_ITEMS, false)?;
    let actual = actual
        .iter()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    let required = super::REQUIRED_OPERATIONS
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    let allowed = super::ALLOWED_OPERATIONS
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if !required.is_subset(&actual) || !actual.is_subset(&allowed) {
        return Err(super::LiveSteelAgentConfigError::InvalidCollection);
    }
    debug_assert!(required.is_subset(&actual));
    debug_assert!(actual.is_subset(&allowed));
    Ok(())
}

pub(super) fn exact_sorted_set(
    actual: &[String],
    expected: &[&str],
) -> Result<(), super::LiveSteelAgentConfigError> {
    validate_labels(actual, super::MAX_NON_CLAIMS, false)?;
    let actual = actual
        .iter()
        .map(String::as_str)
        .collect::<std::collections::BTreeSet<_>>();
    let expected = expected
        .iter()
        .copied()
        .collect::<std::collections::BTreeSet<_>>();
    if actual != expected {
        return Err(super::LiveSteelAgentConfigError::InvalidCollection);
    }
    Ok(())
}

pub(super) fn validate_labels(
    labels: &[String],
    max_items: usize,
    allow_empty: bool,
) -> Result<(), super::LiveSteelAgentConfigError> {
    if labels.len() > max_items || (!allow_empty && labels.is_empty()) {
        return Err(super::LiveSteelAgentConfigError::InvalidCollection);
    }
    let mut unique = std::collections::BTreeSet::new();
    for label in labels {
        validate_label(label.as_str())?;
        if !unique.insert(label.as_str()) {
            return Err(super::LiveSteelAgentConfigError::InvalidCollection);
        }
    }
    Ok(())
}

pub(super) fn validate_label(label: &str) -> Result<(), super::LiveSteelAgentConfigError> {
    let is_valid = !label.is_empty()
        && label.chars().count() <= super::MAX_SAFE_LABEL_CHARS
        && !label.chars().any(char::is_control);
    if !is_valid {
        return Err(super::LiveSteelAgentConfigError::ServiceMismatch);
    }
    Ok(())
}

pub(super) fn validate_blake3(identity: &str) -> Result<(), super::LiveSteelAgentConfigError> {
    if identity.len() != super::BLAKE3_HEX_CHARS
        || !identity
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    {
        return Err(super::LiveSteelAgentConfigError::InvalidIdentity);
    }
    Ok(())
}
