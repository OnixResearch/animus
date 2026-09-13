#[test]
fn live_profile_is_ready_before_product_effects() {
    // r[verify agent_harness_runtime.composition_activation_graph.fail_loud]
    let graph = animus_runtime::composition_activation_graph::ValidatedActivationGraph::validate(
        super::graph_draft(),
    )
    .expect("graph");
    let snapshot = super::preflight_live_application_activation().expect("live activation");

    assert_eq!(graph.ports().len(), super::LIVE_PORT_COUNT);
    assert_eq!(graph.events().len(), super::LIVE_EVENT_COUNT);
    assert_eq!(
        snapshot.status,
        animus_runtime::composition_activation_graph::ActivationDecisionStatus::Ready
    );
    assert_eq!(
        usize::try_from(snapshot.selected_component_count).expect("component count"),
        super::REQUIRED_COMPONENT_COUNT
    );
    assert!(snapshot.issues.is_empty());
}

#[test]
fn missing_provider_blocks_without_fallback() {
    // r[verify agent_harness_runtime.composition_activation_graph.verification]
    let graph = animus_runtime::composition_activation_graph::ValidatedActivationGraph::validate(
        super::graph_draft(),
    )
    .expect("graph");
    let mut selected = super::selection();
    selected
        .component_ids
        .retain(|component_id| component_id != super::COMPONENT_PROVIDER);
    let decision = animus_runtime::composition_activation_graph::preflight_composition_activation(
        &graph,
        &selected,
        super::observations().as_slice(),
    )
    .expect("decision");

    assert_eq!(
        decision.status,
        animus_runtime::composition_activation_graph::ActivationDecisionStatus::Blocked
    );
    assert!(!decision.effects_admitted());
    assert!(
            decision
                .issues
                .iter()
                .any(|issue| { issue.kind == animus_runtime::composition_activation_graph::ActivationIssueKind::MissingProvider })
        );
}
