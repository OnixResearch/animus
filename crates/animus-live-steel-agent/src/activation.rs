//! Product-owned activation profile for the live Steel composition root.

use animus_runtime::composition_activation_graph as activation;

const ACTIVATION_GENERATION: u64 = 1;
#[cfg(test)]
const REQUIRED_COMPONENT_COUNT: usize = 11;
const LIVE_PORT_COUNT: usize = 3;
#[cfg(test)]
const LIVE_EVENT_COUNT: usize = 2;
const REQUIRED_NON_CLAIMS: [&str; 6] = [
    "not-implementation-correctness",
    "not-runtime-liveness",
    "not-effect-success",
    "not-authority-grant",
    "not-production-readiness",
    "not-release-eligibility",
];

const COMPONENT_MODEL_PORT: &str = "adapters.model-port";
const COMPONENT_SESSION_PORT: &str = "runtime.ledger-session-seam";
const COMPONENT_RLM_PORT: &str = "runtime.rlm-host-executor";
const COMPONENT_SESSION: &str = "runtime.coding-session-core";
const COMPONENT_SNAPSHOT_CORE: &str = "runtime.model-request-snapshot-core";
const COMPONENT_PROVIDER: &str = "std.live-openai-provider-host";
const COMPONENT_STORE: &str = "std.durable-session-store-host";
const COMPONENT_EXECUTOR: &str = "std.rlm-host-effect-executor";
const COMPONENT_SESSION_HOST: &str = "std.coding-session-host";
const COMPONENT_SNAPSHOT_HOST: &str = "std.model-request-snapshot-host";
const COMPONENT_SHELL: &str = "std.live-steel-agent-shell";

struct PortProfile {
    port_id: &'static str,
    owner_component_id: &'static str,
    provider_component_id: &'static str,
    scope_class: &'static str,
    feature_gate: &'static str,
}

const PORT_PROFILES: [PortProfile; LIVE_PORT_COUNT] = [
    PortProfile {
        port_id: "port.live-model-provider",
        owner_component_id: COMPONENT_MODEL_PORT,
        provider_component_id: COMPONENT_PROVIDER,
        scope_class: "application",
        feature_gate: "live-provider",
    },
    PortProfile {
        port_id: "port.live-durable-session",
        owner_component_id: COMPONENT_SESSION_PORT,
        provider_component_id: COMPONENT_STORE,
        scope_class: "session",
        feature_gate: "durable-session-store",
    },
    PortProfile {
        port_id: "port.live-rlm-executor",
        owner_component_id: COMPONENT_RLM_PORT,
        provider_component_id: COMPONENT_EXECUTOR,
        scope_class: "request",
        feature_gate: "rlm-host-executor",
    },
];

/// Validates the maintained live Steel composition before shell effects.
///
/// # Errors
///
/// Returns a typed graph error when the maintained profile is incomplete.
pub fn preflight_live_application_activation()
-> Result<activation::ActivationSnapshot, activation::ActivationGraphError> {
    let graph = activation::ValidatedActivationGraph::validate(graph_draft())?;
    let decision = activation::preflight_composition_activation(
        &graph,
        &selection(),
        observations().as_slice(),
    )?;
    if !decision.effects_admitted() {
        return Err(activation::ActivationGraphError::InvalidObservation);
    }
    activation::project_activation_snapshot(&graph, &decision)
}

fn graph_draft() -> activation::ActivationGraphDraft {
    activation::ActivationGraphDraft {
        components: vec![
            component(
                COMPONENT_MODEL_PORT,
                "always",
                activation::ActivationAuthorityClass::InjectedOnly,
            ),
            component(
                COMPONENT_SESSION_PORT,
                "runtime",
                activation::ActivationAuthorityClass::InjectedOnly,
            ),
            component(
                COMPONENT_RLM_PORT,
                "rlm-kernel",
                activation::ActivationAuthorityClass::InjectedOnly,
            ),
            component(
                COMPONENT_SESSION,
                "runtime",
                activation::ActivationAuthorityClass::None,
            ),
            component(
                COMPONENT_SNAPSHOT_CORE,
                "runtime",
                activation::ActivationAuthorityClass::None,
            ),
            component(
                COMPONENT_PROVIDER,
                "openai-provider-host",
                activation::ActivationAuthorityClass::ShellOwned,
            ),
            component(
                COMPONENT_STORE,
                "durable-session-store",
                activation::ActivationAuthorityClass::ShellOwned,
            ),
            component(
                COMPONENT_EXECUTOR,
                "rlm-host-executor",
                activation::ActivationAuthorityClass::ShellOwned,
            ),
            component(
                COMPONENT_SESSION_HOST,
                "coding-session-host",
                activation::ActivationAuthorityClass::ShellOwned,
            ),
            component(
                COMPONENT_SNAPSHOT_HOST,
                "coding-session-host",
                activation::ActivationAuthorityClass::ShellOwned,
            ),
            component(
                COMPONENT_SHELL,
                "live-provider",
                activation::ActivationAuthorityClass::ShellOwned,
            ),
        ],
        ports: PORT_PROFILES.iter().map(port).collect(),
        events: vec![
            event(
                "event.live-session-result",
                activation::ActivationDurabilityClass::MetadataOnly,
                COMPONENT_SHELL,
            ),
            event(
                "event.live-tool-result-committed",
                activation::ActivationDurabilityClass::Durable,
                COMPONENT_SESSION_HOST,
            ),
        ],
        non_claims: REQUIRED_NON_CLAIMS.map(String::from).to_vec(),
    }
}

fn component(
    component_id: &str,
    feature_gate: &str,
    authority: activation::ActivationAuthorityClass,
) -> activation::ActivationComponent {
    activation::ActivationComponent {
        component_id: String::from(component_id),
        feature_gate: String::from(feature_gate),
        authority,
        posture: activation::ActivationPosture::Required,
    }
}

fn port(profile: &PortProfile) -> activation::ActivationPort {
    activation::ActivationPort {
        port_id: String::from(profile.port_id),
        owner_component_id: String::from(profile.owner_component_id),
        provider_component_ids: vec![String::from(profile.provider_component_id)],
        consumer_component_ids: vec![String::from(COMPONENT_SHELL)],
        scope_class: String::from(profile.scope_class),
        multiplicity: activation::PortMultiplicity::ExactlyOne,
        lifecycle_owner_component_id: String::from(COMPONENT_SHELL),
        posture: activation::ActivationPosture::Required,
        feature_gate: String::from(profile.feature_gate),
        authority: activation::ActivationAuthorityClass::ShellOwned,
        focused_check: String::from("check-composition-activation-graph"),
    }
}

fn event(
    event_id: &str,
    durability: activation::ActivationDurabilityClass,
    lifecycle_owner_component_id: &str,
) -> activation::ActivationEventRoute {
    activation::ActivationEventRoute {
        event_id: String::from(event_id),
        producer_component_id: String::from(COMPONENT_SESSION),
        consumer_component_ids: vec![String::from(COMPONENT_SHELL)],
        delivery: activation::ActivationDeliveryClass::Ordered,
        durability,
        lifecycle_owner_component_id: String::from(lifecycle_owner_component_id),
        posture: activation::ActivationPosture::Required,
        bounded_cycle: false,
    }
}

fn selection() -> activation::SelectedComposition {
    activation::SelectedComposition {
        generation: ACTIVATION_GENERATION,
        component_ids: vec![
            String::from(COMPONENT_MODEL_PORT),
            String::from(COMPONENT_SESSION_PORT),
            String::from(COMPONENT_RLM_PORT),
            String::from(COMPONENT_SESSION),
            String::from(COMPONENT_SNAPSHOT_CORE),
            String::from(COMPONENT_PROVIDER),
            String::from(COMPONENT_STORE),
            String::from(COMPONENT_EXECUTOR),
            String::from(COMPONENT_SESSION_HOST),
            String::from(COMPONENT_SNAPSHOT_HOST),
            String::from(COMPONENT_SHELL),
        ],
        enabled_features: vec![
            String::from("runtime"),
            String::from("rlm-kernel"),
            String::from("openai-provider-host"),
            String::from("durable-session-store"),
            String::from("rlm-host-executor"),
            String::from("coding-session-host"),
            String::from("live-provider"),
        ],
        authority_ceiling: activation::ActivationAuthorityClass::ShellOwned,
    }
}

fn observations() -> Vec<activation::ActivationReadinessObservation> {
    selection()
        .component_ids
        .into_iter()
        .map(|component_id| activation::ActivationReadinessObservation {
            observation_id: format!("activation-ready-{component_id}"),
            component_id,
            generation: ACTIVATION_GENERATION,
            status: activation::ActivationReadinessStatus::Ready,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn live_profile_is_ready_before_product_effects() {
        // r[verify agent_harness_runtime.composition_activation_graph.fail_loud]
        let graph = activation::ValidatedActivationGraph::validate(graph_draft()).expect("graph");
        let snapshot = preflight_live_application_activation().expect("live activation");

        assert_eq!(graph.ports().len(), LIVE_PORT_COUNT);
        assert_eq!(graph.events().len(), LIVE_EVENT_COUNT);
        assert_eq!(snapshot.status, activation::ActivationDecisionStatus::Ready);
        assert_eq!(
            usize::try_from(snapshot.selected_component_count).expect("component count"),
            REQUIRED_COMPONENT_COUNT
        );
        assert!(snapshot.issues.is_empty());
    }

    #[test]
    fn missing_provider_blocks_without_fallback() {
        // r[verify agent_harness_runtime.composition_activation_graph.verification]
        let graph = activation::ValidatedActivationGraph::validate(graph_draft()).expect("graph");
        let mut selection = selection();
        selection
            .component_ids
            .retain(|component_id| component_id != COMPONENT_PROVIDER);
        let decision = activation::preflight_composition_activation(
            &graph,
            &selection,
            observations().as_slice(),
        )
        .expect("decision");

        assert_eq!(
            decision.status,
            activation::ActivationDecisionStatus::Blocked
        );
        assert!(!decision.effects_admitted());
        assert!(
            decision
                .issues
                .iter()
                .any(|issue| { issue.kind == activation::ActivationIssueKind::MissingProvider })
        );
    }
}
