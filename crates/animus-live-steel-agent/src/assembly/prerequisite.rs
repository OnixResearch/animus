// r[impl animus.dialect.ownership.provider]
pub(super) fn prepare_live_model_request_snapshot(
    config: &crate::ValidatedLiveSteelAgentConfig,
    request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    canonical: &animus_adapters::CanonicalModelRequest,
    provider_plan: &animus_std::ProviderDispatchPlan,
) -> Result<
    animus_std::model_request_snapshot_host::PreparedModelRequestSnapshot,
    animus_runtime::coding_session_host::CodingSessionServiceIssue,
> {
    let route_identity = animus_runtime::model_request_snapshot::model_request_route_identity(
        provider_plan.provider_id(),
        super::LIVE_PROVIDER_TRANSPORT_ID,
        provider_plan.model_id(),
        super::LIVE_MODEL_DIALECT_ID,
    );
    let tool_identity =
        animus_runtime::model_request_snapshot::model_request_tool_schema_set_identity(canonical);
    let output_identity =
        animus_runtime::model_request_snapshot::model_request_output_control_identity(canonical);
    let budget_identity =
        animus_runtime::model_request_snapshot::model_request_budget_identity(canonical);
    let authority = crate::build_expected_authority(config)
        .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied)?;
    let model_invocation_identity = hash_projection(provider_plan.request_projection_identity());
    let adapter_cohort_identity = hash_projection(config.endpoint_identity());
    let validated = animus_runtime::model_request_snapshot::validate_model_request_snapshot(
        &animus_runtime::model_request_snapshot::ModelRequestSnapshotDraft {
            schema: animus_runtime::model_request_snapshot::MODEL_REQUEST_SNAPSHOT_SCHEMA,
            session_id: request.session_id(),
            branch_id: request.branch_id(),
            active_leaf_id: request.active_leaf_id(),
            turn_generation: config.authority_generation(),
            request_sequence: request.request_sequence(),
            canonical_request: canonical,
            route: animus_runtime::model_request_snapshot::ModelRequestRouteDraft {
                provider_id: provider_plan.provider_id(),
                provider_transport_id: super::LIVE_PROVIDER_TRANSPORT_ID,
                model_id: provider_plan.model_id(),
                model_dialect_id: super::LIVE_MODEL_DIALECT_ID,
                route_identity: route_identity.as_str(),
            },
            model_invocation_identity: model_invocation_identity.as_str(),
            request_context_plan_identity: config.config_identity(),
            selected_context_contribution_identities: &[],
            tool_schema_set_identity: tool_identity.as_str(),
            output_control_identity: output_identity.as_str(),
            budget_identity: budget_identity.as_str(),
            authority_seal_identity: authority.unit.unit_identity(),
            adapter_cohort_identity: adapter_cohort_identity.as_str(),
            unsupported: empty_unsupported(),
        },
    )
    .map_err(|_error| animus_runtime::coding_session_host::CodingSessionServiceIssue::Malformed)?;
    debug_assert_eq!(validated.session_id(), request.session_id());
    debug_assert_eq!(validated.request_sequence(), request.request_sequence());
    Ok(
        animus_std::model_request_snapshot_host::PreparedModelRequestSnapshot {
            prior_head_hash: String::from(request.durable_head_hash()),
            current:
                animus_std::model_request_snapshot_host::CurrentModelRequestSnapshotBindings::exact(
                    &validated,
                ),
            snapshot: validated,
        },
    )
}

fn hash_projection(value: &str) -> String {
    blake3::hash(value.as_bytes()).to_hex().to_string()
}

fn empty_unsupported()
-> animus_runtime::model_request_snapshot::ModelRequestSnapshotUnsupportedDraft<'static> {
    animus_runtime::model_request_snapshot::ModelRequestSnapshotUnsupportedDraft {
        credential: None,
        transport_header: None,
        provider_sdk_object: None,
        hidden_provider_default: None,
        provider_payload: None,
        private_path: None,
    }
}
