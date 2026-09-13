/// Builds one exact child admission from the current request and config.
///
/// # Errors
///
/// Returns [`animus_std_rlm_host::DispatchFailure`] for profile, limit, or authority drift.
pub fn admit_child_request(
    config: &crate::ValidatedLiveSteelAgentConfig,
    request: &animus_adapter_rlm_kernel::ChildSpawnRequest,
) -> Result<
    animus_runtime::child_session_service::ConcreteChildSessionAdmission,
    animus_std_rlm_host::DispatchFailure,
> {
    if !config
        .admitted_child_profiles()
        .iter()
        .any(|profile| profile == request.profile())
    {
        return Err(animus_std_rlm_host::DispatchFailure::new(
            animus_std_rlm_host::RouteClass::Subagent,
            animus_std_rlm_host::DispatchFailureClass::Denied,
            "child_profile",
        ));
    }
    let (child, capability) = project_child(request)?;
    let configured_budget = config.child_session_limits();
    let requested_budget = animus_runtime::child_session_service::ChildSessionLimits {
        depth: request.depth(),
        fan_out: request.fan_out(),
        wall_time_ms: request.wall_time_ms(),
        tokens: request.token_budget(),
        turns: configured_budget.turns,
        tool_calls: configured_budget.tool_calls,
        result_chars: request.result_char_budget(),
    };
    let ceiling = animus_runtime::child_session_service::ChildSessionAuthorityCeiling {
        capabilities: vec![capability.clone()],
        workspace_scopes: vec![String::from(super::CHILD_WORKSPACE_SCOPE)],
        models: vec![String::from(config.model_id())],
        tools: vec![String::from(super::CHILD_TOOL_LABEL)],
        limits: configured_budget,
    };
    debug_assert_eq!(child.parent_id, super::CHILD_PARENT_ID);
    debug_assert_eq!(requested_budget.turns, configured_budget.turns);
    animus_runtime::child_session_service::admit_concrete_child_session(
        &animus_runtime::child_session_service::ConcreteChildSessionAdmissionDraft {
            subagent: animus_runtime::subagents::admit_subagent_child_run(&child),
            context_packet: child.context_packet,
            objective: request.context_slice().to_owned(),
            profile_id: request.profile().to_owned(),
            parent_authority_identity: config.authority_authorization_identity().to_owned(),
            workspace_scope: String::from(super::CHILD_WORKSPACE_SCOPE),
            model: String::from(config.model_id()),
            tools: vec![String::from(super::CHILD_TOOL_LABEL)],
            requested_capabilities: vec![capability.clone()],
            parent: ceiling.clone(),
            profile: ceiling,
            semantic_role_capabilities: vec![capability.clone()],
            workspace_capabilities: vec![capability.clone()],
            host_capabilities: vec![capability],
            requested_limits: requested_budget,
            handoff_mode: animus_runtime::subagents::SubagentHandoffMode::StatusPlusSummary,
            kernel_profile: None,
            skill_ids: Vec::new(),
        },
    )
    .map_err(|_error| {
        animus_std_rlm_host::DispatchFailure::new(
            animus_std_rlm_host::RouteClass::Subagent,
            animus_std_rlm_host::DispatchFailureClass::Denied,
            "child_admission",
        )
    })
}

const MAX_CONTEXT_ITEMS: u16 = 32;
const MAX_CONTEXT_TOKENS: u32 = 8_192;
fn project_child(
    request: &animus_adapter_rlm_kernel::ChildSpawnRequest,
) -> Result<
    (
        animus_runtime::subagents::SubagentChildRun,
        animus_runtime::subagents::TypedSubagentCapability,
    ),
    animus_std_rlm_host::DispatchFailure,
> {
    let request_identity = blake3::hash(request.context_slice().as_bytes())
        .to_hex()
        .to_string();
    let child_id = format!("{}{request_identity}", super::CHILD_ID_PREFIX);
    let packet_id = format!("{}{request_identity}", super::CHILD_PACKET_PREFIX);
    let capability = animus_runtime::subagents::TypedSubagentCapability::WorkspaceAccess {
        stable_label: String::from("workspace-read"),
        scope_label: String::from(super::CHILD_WORKSPACE_SCOPE),
        mode: animus_runtime::subagents::SubagentWorkspaceAccessMode::Read,
    };
    let child = animus_runtime::subagents::validate_subagent_child_run(
        &animus_runtime::subagents::SubagentChildRunDraft {
            child_id,
            parent_id: String::from(super::CHILD_PARENT_ID),
            kind: animus_runtime::subagents::SubagentChildRunKind::FreshContextInvestigation,
            context_packet: Some(animus_runtime::subagents::SubagentContextPacketDraft {
                packet_id,
                child_kind:
                    animus_runtime::subagents::SubagentChildRunKind::FreshContextInvestigation,
                admission: animus_runtime::subagents::SubagentContextAdmissionPolicy {
                    max_items: MAX_CONTEXT_ITEMS,
                    max_tokens: MAX_CONTEXT_TOKENS,
                    admit_unsafe_classes: false,
                },
                items: vec![animus_runtime::subagents::SubagentContextItemDraft {
                    kind: animus_runtime::subagents::SubagentContextItemKind::Summary,
                    label: String::from(super::CHILD_CONTEXT_LABEL),
                    safe_body: request.context_slice().to_owned(),
                    token_estimate: super::CHILD_CONTEXT_TOKEN_ESTIMATE,
                    redaction: animus_runtime::subagents::SubagentContextRedaction::SafeSummary,
                }],
            }),
            capabilities: vec![capability.clone()],
            recipe: None,
            fresh_context: Some(animus_runtime::subagents::FreshContextSubagentDraft {
                objective: request.context_slice().to_owned(),
                scheduler_label: String::from(super::CHILD_SCHEDULER_LABEL),
                read_only: true,
            }),
            worker_team: None,
            execution_requested: false,
        },
    )
    .map_err(|_error| {
        animus_std_rlm_host::DispatchFailure::new(
            animus_std_rlm_host::RouteClass::Subagent,
            animus_std_rlm_host::DispatchFailureClass::Malformed,
            "child_request",
        )
    })?;
    debug_assert_eq!(child.parent_id, super::CHILD_PARENT_ID);
    debug_assert!(!child.child_id.is_empty());
    Ok((child, capability))
}
