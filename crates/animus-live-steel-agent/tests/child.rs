//! Concrete child-route execution and denial checks.

use animus_live_steel_agent::{
    ExplicitChildAuthority, ExplicitChildFactory, FixedChildTimer, MetadataChildEvents,
    MetadataChildStore, admit_child_request, build_child_service, parse_live_steel_agent_config,
};
use animus_std_rlm_host::{ChildRoute, ConcreteChildRoute};

const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
const CHILD_PROFILE: &str = "focused-child";
const DENIED_CHILD_PROFILE: &str = "unreviewed-child";
const CHILD_CONTEXT: &str = "Selected bounded child context.";
const CHILD_RESULT: &str = "Bounded child result.";

#[test]
fn concrete_child_route_uses_normal_coding_session_controller() {
    let config = parse_live_steel_agent_config(CONFIG).expect("config");
    let request = child_request(&config, CHILD_PROFILE, false, false);
    let mut service = build_child_service(&config, CHILD_RESULT).expect("child service");
    let child_config = config.clone();
    let mut admission = move |candidate: &animus_adapter_rlm_kernel::ChildSpawnRequest| {
        admit_child_request(&child_config, candidate)
    };
    let mut route = ConcreteChildRoute {
        service: &mut service,
        admission: &mut admission,
    };
    let (result, identity) = route
        .execute_child(&request, config.executor_limits().max_child_status_checks)
        .expect("normal child route");

    assert_eq!(result, CHILD_RESULT);
    assert_eq!(identity.as_str().len(), blake3::OUT_LEN * 2);
    assert_eq!(service.registry_len(), 1);
}

#[test]
fn child_profile_and_ambient_discovery_are_denied_before_enqueue() {
    let config = parse_live_steel_agent_config(CONFIG).expect("config");
    let denied_profile = child_request(&config, DENIED_CHILD_PROFILE, false, false);
    assert!(admit_child_request(&config, &denied_profile).is_err());

    let ambient = child_request(&config, CHILD_PROFILE, true, false);
    assert!(animus_adapter_rlm_kernel::admit_child_spawn(config.rlm_policy(), &ambient).is_err());
}

#[test]
fn denied_factory_fails_without_registering_a_child() {
    let config = parse_live_steel_agent_config(CONFIG).expect("config");
    let request = child_request(&config, CHILD_PROFILE, false, false);
    let admission = admit_child_request(&config, &request).expect("admission");
    let mut service = animus_std::child_session_host::ConcreteChildSessionService::new(
        config.child_service_label(),
        animus_std::child_session_host::ConcreteChildSessionServices {
            factory: ExplicitChildFactory::denied(config.authority_component_identity()),
            store: MetadataChildStore::default(),
            authority: ExplicitChildAuthority::admitted(
                config.authority_authorization_identity(),
            ),
            timer: FixedChildTimer::default(),
            events: MetadataChildEvents::default(),
        },
    )
    .expect("denied service");
    let child_config = config.clone();
    let mut admission_builder =
        move |candidate: &animus_adapter_rlm_kernel::ChildSpawnRequest| {
            admit_child_request(&child_config, candidate)
        };
    let mut route = ConcreteChildRoute {
        service: &mut service,
        admission: &mut admission_builder,
    };

    assert!(
        route
            .execute_child(&request, config.executor_limits().max_child_status_checks)
            .is_err()
    );
    assert_eq!(service.registry_len(), 0);
    drop(admission);
}

fn child_request(
    config: &animus_live_steel_agent::ValidatedLiveSteelAgentConfig,
    profile: &str,
    ambient_discovery: bool,
    cross_session: bool,
) -> animus_adapter_rlm_kernel::ChildSpawnRequest {
    let limits = config.child_session_limits();
    animus_adapter_rlm_kernel::ChildSpawnRequest::new(
        profile,
        CHILD_CONTEXT,
        limits.depth,
        limits.fan_out,
        limits.tokens,
        limits.wall_time_ms,
        limits.result_chars,
        ambient_discovery,
        cross_session,
    )
    .expect("child request")
}
