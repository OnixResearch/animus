//! Concrete child-route execution and denial checks.

#[cfg(test)]
mod cases {
    use animus_std_rlm_host::ChildRoute;

    #[derive(Clone, Copy)]
    struct Scope {
        ambient_discovery: bool,
        cross_session: bool,
    }
    const BOUND_SCOPE: Scope = Scope {
        ambient_discovery: false,
        cross_session: false,
    };
    const HEX_CHARACTERS_PER_BYTE: usize = 2;
    const IDENTITY_CHARS: usize = blake3::OUT_LEN
        .checked_mul(HEX_CHARACTERS_PER_BYTE)
        .expect("BLAKE3 hex width fits");

    const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
    const CHILD_PROFILE: &str = "focused-child";
    const DENIED_CHILD_PROFILE: &str = "unreviewed-child";
    const CHILD_CONTEXT: &str = "Selected bounded child context.";
    const CHILD_RESULT: &str = "Bounded child result.";

    #[test]
    fn concrete_child_route_uses_normal_coding_session_controller() {
        let config =
            animus_live_steel_agent::parse_live_steel_agent_config(CONFIG).expect("config");
        let request = child_request(&config, CHILD_PROFILE, BOUND_SCOPE);
        let mut service = animus_live_steel_agent::build_child_service(&config, CHILD_RESULT)
            .expect("child service");
        let child_config = config.clone();
        let mut admission = move |candidate: &animus_adapter_rlm_kernel::ChildSpawnRequest| {
            animus_live_steel_agent::admit_child_request(&child_config, candidate)
        };
        let mut route = animus_std_rlm_host::ConcreteChildRoute {
            service: &mut service,
            admission: &mut admission,
        };
        let (result, identity) = route
            .execute_child(&request, config.executor_limits().max_child_status_checks)
            .expect("normal child route");

        assert_eq!(result, CHILD_RESULT);
        assert_eq!(identity.as_str().len(), IDENTITY_CHARS);
        assert_eq!(service.registry_len(), 1);
    }

    #[test]
    fn child_profile_and_ambient_discovery_are_denied_before_enqueue() {
        let config =
            animus_live_steel_agent::parse_live_steel_agent_config(CONFIG).expect("config");
        let denied_profile = child_request(&config, DENIED_CHILD_PROFILE, BOUND_SCOPE);
        assert!(animus_live_steel_agent::admit_child_request(&config, &denied_profile).is_err());

        let ambient = child_request(
            &config,
            CHILD_PROFILE,
            Scope {
                ambient_discovery: true,
                ..BOUND_SCOPE
            },
        );
        assert!(
            animus_adapter_rlm_kernel::admit_child_spawn(config.rlm_policy(), &ambient).is_err()
        );
        let crossed = child_request(
            &config,
            CHILD_PROFILE,
            Scope {
                cross_session: true,
                ..BOUND_SCOPE
            },
        );
        assert!(
            animus_adapter_rlm_kernel::admit_child_spawn(config.rlm_policy(), &crossed).is_err()
        );
    }

    #[test]
    fn denied_factory_fails_without_registering_a_child() {
        let config =
            animus_live_steel_agent::parse_live_steel_agent_config(CONFIG).expect("config");
        let request = child_request(&config, CHILD_PROFILE, BOUND_SCOPE);
        let admission =
            animus_live_steel_agent::admit_child_request(&config, &request).expect("admission");
        let mut service = animus_std::child_session_host::ConcreteChildSessionService::new(
            config.child_service_label(),
            animus_std::child_session_host::ConcreteChildSessionServices {
                factory: animus_live_steel_agent::ExplicitChildFactory::denied(
                    config.authority_component_identity(),
                ),
                store: animus_live_steel_agent::MetadataChildStore::default(),
                authority: animus_live_steel_agent::ExplicitChildAuthority::admitted(
                    config.authority_authorization_identity(),
                ),
                timer: animus_live_steel_agent::FixedChildTimer::default(),
                events: animus_live_steel_agent::MetadataChildEvents::default(),
            },
        )
        .expect("denied service");
        let child_config = config.clone();
        let mut admission_builder =
            move |candidate: &animus_adapter_rlm_kernel::ChildSpawnRequest| {
                animus_live_steel_agent::admit_child_request(&child_config, candidate)
            };
        let mut route = animus_std_rlm_host::ConcreteChildRoute {
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
        scope: Scope,
    ) -> animus_adapter_rlm_kernel::ChildSpawnRequest {
        let admitted = config.child_session_limits();
        animus_adapter_rlm_kernel::ChildSpawnRequest::new(
            profile,
            CHILD_CONTEXT,
            admitted.depth,
            admitted.fan_out,
            admitted.tokens,
            admitted.wall_time_ms,
            admitted.result_chars,
            scope.ambient_discovery,
            scope.cross_session,
        )
        .expect("child request")
    }
}
