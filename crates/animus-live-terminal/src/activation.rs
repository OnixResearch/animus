//! Product-owned activation profile for the live terminal composition root.

use animus_runtime::composition_activation_graph as activation;

const ACTIVATION_GENERATION: u64 = 1;
#[cfg(test)]
const TERMINAL_PORT_COUNT: usize = 1;
#[cfg(test)]
const TERMINAL_EVENT_COUNT: usize = 1;
const REQUIRED_NON_CLAIMS: [&str; 6] = [
    "not-implementation-correctness",
    "not-runtime-liveness",
    "not-effect-success",
    "not-authority-grant",
    "not-production-readiness",
    "not-release-eligibility",
];
const COMPONENT_DAEMON_CONTRACT: &str = "adapters.daemon-workers";
const COMPONENT_DAEMON_HOST: &str = "std.local-daemon-supervisor-host";
const COMPONENT_TUI: &str = "tui.host-shell";
const COMPONENT_TERMINAL: &str = "std.live-terminal-agent-host";

/// Runs the pure live-terminal activation preflight.
///
/// # Errors
///
/// Returns a typed graph error when the maintained profile is incomplete.
pub fn preflight_live_terminal_activation()
-> Result<activation::ActivationDecision, activation::ActivationGraphError> {
    let graph = activation::ValidatedActivationGraph::validate(graph_draft())?;
    activation::preflight_composition_activation(&graph, &selection(), observations().as_slice())
}

fn graph_draft() -> activation::ActivationGraphDraft {
    activation::ActivationGraphDraft {
        components: vec![
            component(
                COMPONENT_DAEMON_CONTRACT,
                "always",
                activation::ActivationAuthorityClass::None,
            ),
            component(
                COMPONENT_DAEMON_HOST,
                "local-daemon-supervisor-host",
                activation::ActivationAuthorityClass::ShellOwned,
            ),
            component(
                COMPONENT_TUI,
                "tui",
                activation::ActivationAuthorityClass::ShellOwned,
            ),
            component(
                COMPONENT_TERMINAL,
                "live-terminal",
                activation::ActivationAuthorityClass::ShellOwned,
            ),
        ],
        ports: vec![activation::ActivationPort {
            port_id: String::from("port.live-terminal-connection"),
            owner_component_id: String::from(COMPONENT_DAEMON_CONTRACT),
            provider_component_ids: vec![String::from(COMPONENT_DAEMON_HOST)],
            consumer_component_ids: vec![String::from(COMPONENT_TERMINAL)],
            scope_class: String::from("client"),
            multiplicity: activation::PortMultiplicity::ExactlyOne,
            lifecycle_owner_component_id: String::from(COMPONENT_TERMINAL),
            posture: activation::ActivationPosture::Required,
            feature_gate: String::from("live-terminal"),
            authority: activation::ActivationAuthorityClass::ShellOwned,
            focused_check: String::from("check-composition-activation-graph"),
        }],
        events: vec![activation::ActivationEventRoute {
            event_id: String::from("event.live-terminal-snapshot"),
            producer_component_id: String::from(COMPONENT_DAEMON_HOST),
            consumer_component_ids: vec![String::from(COMPONENT_TERMINAL)],
            delivery: activation::ActivationDeliveryClass::SnapshotFirst,
            durability: activation::ActivationDurabilityClass::LiveOnly,
            lifecycle_owner_component_id: String::from(COMPONENT_TERMINAL),
            posture: activation::ActivationPosture::Required,
            bounded_cycle: false,
        }],
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

fn selection() -> activation::SelectedComposition {
    activation::SelectedComposition {
        generation: ACTIVATION_GENERATION,
        component_ids: vec![
            String::from(COMPONENT_DAEMON_CONTRACT),
            String::from(COMPONENT_DAEMON_HOST),
            String::from(COMPONENT_TUI),
            String::from(COMPONENT_TERMINAL),
        ],
        enabled_features: vec![
            String::from("local-daemon-supervisor-host"),
            String::from("tui"),
            String::from("live-terminal"),
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
    fn live_terminal_profile_is_ready() {
        // r[verify agent_harness_runtime.composition_activation_graph.fail_loud]
        let graph = activation::ValidatedActivationGraph::validate(graph_draft()).expect("graph");
        let decision = preflight_live_terminal_activation().expect("terminal activation");

        assert_eq!(graph.ports().len(), TERMINAL_PORT_COUNT);
        assert_eq!(graph.events().len(), TERMINAL_EVENT_COUNT);
        assert_eq!(decision.status, activation::ActivationDecisionStatus::Ready);
        assert!(decision.effects_admitted());
        assert!(decision.issues.is_empty());
    }

    #[test]
    fn stale_daemon_readiness_blocks_terminal_activation() {
        // r[verify agent_harness_runtime.composition_activation_graph.verification]
        let graph = activation::ValidatedActivationGraph::validate(graph_draft()).expect("graph");
        let mut observations = observations();
        let daemon = observations
            .iter_mut()
            .find(|observation| observation.component_id == COMPONENT_DAEMON_HOST)
            .expect("daemon observation");
        daemon.generation = ACTIVATION_GENERATION.saturating_add(1);
        let decision = activation::preflight_composition_activation(
            &graph,
            &selection(),
            observations.as_slice(),
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
                .any(|issue| { issue.kind == activation::ActivationIssueKind::StaleReadiness })
        );
    }
}
