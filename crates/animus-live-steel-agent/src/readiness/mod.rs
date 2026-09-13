//! Product-owned activation profile for the live Steel composition root.

const ACTIVATION_GENERATION: u64 = 1;
#[cfg(test)]
const REQUIRED_COMPONENT_COUNT: usize = 13;
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
const COMPONENT_TOOL_STAGE_CORE: &str = "runtime.tool-execution-stage-core";
const COMPONENT_PROVIDER: &str = "std.live-openai-provider-host";
const COMPONENT_STORE: &str = "std.durable-session-store-host";
const COMPONENT_EXECUTOR: &str = "std.rlm-host-effect-executor";
const COMPONENT_SESSION_HOST: &str = "std.coding-session-host";
const COMPONENT_SNAPSHOT_HOST: &str = "std.model-request-snapshot-host";
const COMPONENT_TOOL_STAGE_HOST: &str = "std.tool-execution-stage-host";
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
pub fn preflight_live_application_activation() -> Result<
    animus_runtime::composition_activation_graph::ActivationSnapshot,
    animus_runtime::composition_activation_graph::ActivationGraphError,
> {
    let graph = animus_runtime::composition_activation_graph::ValidatedActivationGraph::validate(
        graph_draft(),
    )?;
    let decision = animus_runtime::composition_activation_graph::preflight_composition_activation(
        &graph,
        &selection(),
        observations().as_slice(),
    )?;
    if !decision.effects_admitted() {
        return Err(
            animus_runtime::composition_activation_graph::ActivationGraphError::InvalidObservation,
        );
    }
    animus_runtime::composition_activation_graph::project_activation_snapshot(&graph, &decision)
}

fn graph_draft() -> animus_runtime::composition_activation_graph::ActivationGraphDraft {
    animus_runtime::composition_activation_graph::ActivationGraphDraft {
        components: COMPONENT_PROFILES.iter().map(|(id, feature, authority)| component((id, feature), *authority)).collect(),
        ports: PORT_PROFILES.iter().map(port).collect(),
        events: vec![
            event(
                "event.live-session-result",
                animus_runtime::composition_activation_graph::ActivationDurabilityClass::MetadataOnly,
                COMPONENT_SHELL,
            ),
            event(
                "event.live-tool-result-committed",
                animus_runtime::composition_activation_graph::ActivationDurabilityClass::Durable,
                COMPONENT_SESSION_HOST,
            ),
        ],
        non_claims: REQUIRED_NON_CLAIMS.map(String::from).to_vec(),
    }
}

fn component(
    profile: (&str, &str),
    authority: animus_runtime::composition_activation_graph::ActivationAuthorityClass,
) -> animus_runtime::composition_activation_graph::ActivationComponent {
    animus_runtime::composition_activation_graph::ActivationComponent {
        component_id: String::from(profile.0),
        feature_gate: String::from(profile.1),
        authority,
        posture: animus_runtime::composition_activation_graph::ActivationPosture::Required,
    }
}

fn port(profile: &PortProfile) -> animus_runtime::composition_activation_graph::ActivationPort {
    animus_runtime::composition_activation_graph::ActivationPort {
        port_id: String::from(profile.port_id),
        owner_component_id: String::from(profile.owner_component_id),
        provider_component_ids: vec![String::from(profile.provider_component_id)],
        consumer_component_ids: vec![String::from(COMPONENT_SHELL)],
        scope_class: String::from(profile.scope_class),
        multiplicity: animus_runtime::composition_activation_graph::PortMultiplicity::ExactlyOne,
        lifecycle_owner_component_id: String::from(COMPONENT_SHELL),
        posture: animus_runtime::composition_activation_graph::ActivationPosture::Required,
        feature_gate: String::from(profile.feature_gate),
        authority:
            animus_runtime::composition_activation_graph::ActivationAuthorityClass::ShellOwned,
        focused_check: String::from("check-composition-activation-graph"),
    }
}

fn event(
    event_id: &str,
    durability: animus_runtime::composition_activation_graph::ActivationDurabilityClass,
    lifecycle_owner_component_id: &str,
) -> animus_runtime::composition_activation_graph::ActivationEventRoute {
    animus_runtime::composition_activation_graph::ActivationEventRoute {
        event_id: String::from(event_id),
        producer_component_id: String::from(COMPONENT_SESSION),
        consumer_component_ids: vec![String::from(COMPONENT_SHELL)],
        delivery: animus_runtime::composition_activation_graph::ActivationDeliveryClass::Ordered,
        durability,
        lifecycle_owner_component_id: String::from(lifecycle_owner_component_id),
        posture: animus_runtime::composition_activation_graph::ActivationPosture::Required,
        bounded_cycle: false,
    }
}

fn selection() -> animus_runtime::composition_activation_graph::SelectedComposition {
    animus_runtime::composition_activation_graph::SelectedComposition {
        generation: ACTIVATION_GENERATION,
        component_ids: vec![
            String::from(COMPONENT_MODEL_PORT),
            String::from(COMPONENT_SESSION_PORT),
            String::from(COMPONENT_RLM_PORT),
            String::from(COMPONENT_SESSION),
            String::from(COMPONENT_SNAPSHOT_CORE),
            String::from(COMPONENT_TOOL_STAGE_CORE),
            String::from(COMPONENT_PROVIDER),
            String::from(COMPONENT_STORE),
            String::from(COMPONENT_EXECUTOR),
            String::from(COMPONENT_SESSION_HOST),
            String::from(COMPONENT_SNAPSHOT_HOST),
            String::from(COMPONENT_TOOL_STAGE_HOST),
            String::from(COMPONENT_SHELL),
        ],
        enabled_features: vec![
            String::from("runtime"),
            String::from("rlm-kernel"),
            String::from("openai-provider-host"),
            String::from("durable-session-store"),
            String::from("rlm-host-executor"),
            String::from("coding-session-host"),
            String::from("tool-execution-stages"),
            String::from("live-provider"),
        ],
        authority_ceiling:
            animus_runtime::composition_activation_graph::ActivationAuthorityClass::ShellOwned,
    }
}

fn observations()
-> Vec<animus_runtime::composition_activation_graph::ActivationReadinessObservation> {
    selection()
        .component_ids
        .into_iter()
        .map(|component_id| {
            animus_runtime::composition_activation_graph::ActivationReadinessObservation {
                observation_id: format!("activation-ready-{component_id}"),
                component_id,
                generation: ACTIVATION_GENERATION,
                status:
                    animus_runtime::composition_activation_graph::ActivationReadinessStatus::Ready,
            }
        })
        .collect()
}

const COMPONENT_PROFILES: &[(
    &str,
    &str,
    animus_runtime::composition_activation_graph::ActivationAuthorityClass,
)] = &[
    (
        COMPONENT_MODEL_PORT,
        "always",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::InjectedOnly,
    ),
    (
        COMPONENT_SESSION_PORT,
        "runtime",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::InjectedOnly,
    ),
    (
        COMPONENT_RLM_PORT,
        "rlm-kernel",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::InjectedOnly,
    ),
    (
        COMPONENT_SESSION,
        "runtime",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::None,
    ),
    (
        COMPONENT_SNAPSHOT_CORE,
        "runtime",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::None,
    ),
    (
        COMPONENT_TOOL_STAGE_CORE,
        "runtime",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::None,
    ),
    (
        COMPONENT_PROVIDER,
        "openai-provider-host",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::ShellOwned,
    ),
    (
        COMPONENT_STORE,
        "durable-session-store",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::ShellOwned,
    ),
    (
        COMPONENT_EXECUTOR,
        "rlm-host-executor",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::ShellOwned,
    ),
    (
        COMPONENT_SESSION_HOST,
        "coding-session-host",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::ShellOwned,
    ),
    (
        COMPONENT_SNAPSHOT_HOST,
        "coding-session-host",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::ShellOwned,
    ),
    (
        COMPONENT_TOOL_STAGE_HOST,
        "tool-execution-stages",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::ShellOwned,
    ),
    (
        COMPONENT_SHELL,
        "live-provider",
        animus_runtime::composition_activation_graph::ActivationAuthorityClass::ShellOwned,
    ),
];
#[cfg(test)]
mod tests;
