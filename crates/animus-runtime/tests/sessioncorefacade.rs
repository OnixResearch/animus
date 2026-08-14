//! Compatibility checks for the `animus-runtime` session-core facade.

const MAX_ENTRIES: u32 = 64;
const MAX_PARTS_PER_ITEM: u32 = 8;
const POLICY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

#[test]
fn runtime_facade_preserves_session_core_types() {
    // r[agent_harness_runtime.session_core_crate.compatibility]
    let run_id_result = animus_adapters::AgentControlRunId::new("run.session-core-facade");
    assert!(run_id_result.is_ok(), "run identity must be valid");
    let Ok(run_id) = run_id_result else {
        return;
    };
    let core_state = animus_session_core::AgentControlState::new(run_id);
    let facade_state: animus_runtime::AgentControlState = core_state.clone();

    let _: fn(
        animus_runtime::active_turn_recovery::ActiveTurnRecoveryError,
    ) -> animus_session_core::active_turn_recovery::ActiveTurnRecoveryError = |value| value;
    let _: fn(
        animus_runtime::branch_explorer::BranchExplorerError,
    ) -> animus_session_core::branch_explorer::BranchExplorerError = |value| value;
    let _: fn(
        animus_runtime::component_scope::ComponentScopeError,
    ) -> animus_session_core::component_scope::ComponentScopeError = |value| value;
    let _: fn(
        animus_runtime::conversation_projection::ConversationProjectionError,
    ) -> animus_session_core::conversation_projection::ConversationProjectionError = |value| value;
    let _: fn(
        animus_runtime::interrupted_tool_repair::InterruptedToolRepairError,
    ) -> animus_session_core::interrupted_tool_repair::InterruptedToolRepairError = |value| value;
    let _: fn(
        animus_runtime::session_authority_profile::SessionAuthorityError,
    ) -> animus_session_core::session_authority_profile::SessionAuthorityError = |value| value;
    let _: fn(
        animus_runtime::runtime_authority_seal::RuntimeAuthoritySealError,
    ) -> animus_session_core::runtime_authority_seal::RuntimeAuthoritySealError = |value| value;
    let _: fn(
        animus_runtime::session_tree_export::SessionTreeExportError,
    ) -> animus_session_core::session_tree_export::SessionTreeExportError = |value| value;
    let _: fn(animus_runtime::SessionBranchError) -> animus_session_core::SessionBranchError =
        |value| value;

    assert_eq!(facade_state, core_state);
    assert_eq!(
        std::mem::size_of::<animus_runtime::active_turn_recovery::ActiveTurnRecoveryError>(),
        std::mem::size_of::<animus_session_core::active_turn_recovery::ActiveTurnRecoveryError>()
    );
    assert_eq!(
        std::mem::size_of::<animus_runtime::branch_explorer::BranchExplorerError>(),
        std::mem::size_of::<animus_session_core::branch_explorer::BranchExplorerError>()
    );
    assert_eq!(
        std::mem::size_of::<animus_runtime::component_scope::ComponentScopeError>(),
        std::mem::size_of::<animus_session_core::component_scope::ComponentScopeError>()
    );
    assert_eq!(
        std::mem::size_of::<animus_runtime::conversation_projection::ConversationProjectionError>(),
        std::mem::size_of::<
            animus_session_core::conversation_projection::ConversationProjectionError,
        >()
    );
    assert_eq!(
        std::mem::size_of::<animus_runtime::interrupted_tool_repair::InterruptedToolRepairError>(),
        std::mem::size_of::<animus_session_core::interrupted_tool_repair::InterruptedToolRepairError>(
        )
    );
    assert_eq!(
        std::mem::size_of::<animus_runtime::session_authority_profile::SessionAuthorityError>(),
        std::mem::size_of::<animus_session_core::session_authority_profile::SessionAuthorityError>(
        )
    );
    assert_eq!(
        std::mem::size_of::<animus_runtime::runtime_authority_seal::RuntimeAuthoritySealError>(),
        std::mem::size_of::<animus_session_core::runtime_authority_seal::RuntimeAuthoritySealError>(
        )
    );
    assert_eq!(
        std::mem::size_of::<animus_runtime::session_tree_export::SessionTreeExportError>(),
        std::mem::size_of::<animus_session_core::session_tree_export::SessionTreeExportError>()
    );
    assert_eq!(
        std::mem::size_of::<animus_runtime::SessionBranchError>(),
        std::mem::size_of::<animus_session_core::SessionBranchError>()
    );
}

#[test]
fn runtime_facade_preserves_content_identity() {
    // r[agent_harness_runtime.session_core_crate.compatibility]
    let input = policy_input();
    let core_result =
        animus_session_core::conversation_projection::validate_conversation_projection_policy(
            &input,
        );
    assert!(core_result.is_ok(), "core policy must validate");
    let Ok(core) = core_result else {
        return;
    };
    let facade_result =
        animus_runtime::conversation_projection::validate_conversation_projection_policy(&input);
    assert!(facade_result.is_ok(), "facade policy must validate");
    let Ok(facade) = facade_result else {
        return;
    };

    assert_eq!(facade, core);
    assert_eq!(facade.identity(), core.identity());
}

fn policy_input() -> animus_session_core::conversation_projection::ConversationProjectionPolicyInput
{
    animus_session_core::conversation_projection::ConversationProjectionPolicyInput {
        schema: String::from(
            animus_session_core::conversation_projection::CONVERSATION_PROJECTION_SCHEMA,
        ),
        policy_id: String::from("canonical-conversation"),
        max_entries: MAX_ENTRIES,
        max_parts_per_item: MAX_PARTS_PER_ITEM,
        reasoning_posture: String::from("hidden"),
        tool_posture: String::from("hash_only"),
        cursor_posture: String::from("revision_fenced"),
        cache_posture: String::from("disposable"),
        receipt_fields: ["source", "stream", "cursor", "counts", "non_claims"]
            .map(String::from)
            .into(),
        non_claims: [
            "transcript_truth",
            "ui_delivery",
            "cache_durability",
            "renderer_correctness",
            "task_completion",
            "production_readiness",
        ]
        .map(String::from)
        .into(),
        policy_hash: String::from(POLICY_HASH),
    }
}

#[cfg(feature = "session-event-replay")]
#[test]
fn runtime_facade_preserves_feature_gated_replay_type() {
    let _: fn(
        animus_runtime::session_event_replay::ReplayError,
    ) -> animus_session_core::session_event_replay::ReplayError = |value| value;
}
