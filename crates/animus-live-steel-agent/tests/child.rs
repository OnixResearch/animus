//! Concrete child-route execution and denial checks.

use animus_live_steel_agent::{
    ExplicitChildAuthority, ExplicitChildFactory, FixedChildTimer, ImplementationLineageBinding,
    ImplementationLineageError, InjectedChildFactory, MetadataChildEvents, MetadataChildStore,
    admit_child_request, build_child_service, parse_live_steel_agent_config,
    run_implementation_child_profile, run_implementation_parent_profile,
};
use animus_std_rlm_host::{ChildRoute, ConcreteChildRoute};
const CONFIG: &[u8] = include_bytes!("../../../config/generated/live-steel-agent-shell.json");
const CHILD_PROFILE: &str = "focused-child";
const DENIED_CHILD_PROFILE: &str = "unreviewed-child";
const CHILD_CONTEXT: &str = "Selected bounded child context.";
const CHILD_RESULT: &str = "Bounded child result.";

struct InjectedFinalProvider(std::rc::Rc<std::cell::Cell<u32>>);

impl animus_std::coding_session_host::CodingSessionProviderService for InjectedFinalProvider {
    fn dispatch(
        &mut self,
        _request: &animus_runtime::coding_session_host::CodingSessionProviderRequest,
    ) -> Result<animus_runtime::coding_session_host::CodingSessionProviderReply, animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        self.0.set(self.0.get() + 1);
        let response = animus_adapters::ModelResponse::new(CHILD_RESULT).expect("bounded fixture response");
        Ok(animus_runtime::coding_session_host::CodingSessionProviderReply::Final(response))
    }
}

struct WorkspaceToolHost;

impl animus_std::coding_session_host::CodingSessionToolService for WorkspaceToolHost {
    fn dispatch(
        &mut self,
        _request: &animus_runtime::coding_session_host::CodingSessionToolRequest,
    ) -> Result<animus_adapters::ToolResponse, animus_runtime::coding_session_host::CodingSessionServiceIssue> {
        Err(animus_runtime::coding_session_host::CodingSessionServiceIssue::Denied)
    }
}

fn lineage_binding() -> ImplementationLineageBinding {
    ImplementationLineageBinding {
        work_identity: "a".repeat(64), attempt_identity: "b".repeat(64), fence: 3,
        request_identity: "c".repeat(64), worker_incarnation: "d".repeat(64),
        profile_revision: "e".repeat(64),
    }
}

#[test]
fn injected_child_runs_genuine_service_lifecycle_and_projects_sealed_lineage() {
    let config = parse_live_steel_agent_config(CONFIG).expect("config");
    let limits = config.child_session_limits();
    let request = animus_adapter_rlm_kernel::ChildSpawnRequest::new(
        CHILD_PROFILE, CHILD_CONTEXT, 1, 1, limits.tokens, limits.wall_time_ms,
        limits.result_chars, false, false,
    ).expect("bounded request");
    let dispatches = std::rc::Rc::new(std::cell::Cell::new(0));
    let provider_dispatches = dispatches.clone();
    let factory = InjectedChildFactory::new(config.authority_component_identity(), move |_admission: &animus_runtime::child_session_service::ConcreteChildSessionAdmission| {
        Ok((InjectedFinalProvider(provider_dispatches.clone()), WorkspaceToolHost))
    });
    let output = run_implementation_child_profile(&config, &request, lineage_binding(), factory)
        .expect("real child lifecycle");
    assert_eq!(output.result, CHILD_RESULT);
    assert_eq!(dispatches.get(), 1);
    assert_eq!(output.lineage.schema, "animus.implementation-child-lineage.v1");
    assert_eq!(output.lineage.coverage, "complete");
    assert_eq!(output.lineage.edges.len(), 1);
    assert_eq!(output.lineage.edges[0].state, "completed");
    assert_eq!(output.lineage.edges[0].parent_id, output.lineage.root_id);
    assert_eq!(output.lineage.root_id.len(), 64);
    assert_eq!(output.lineage.edges[0].child_id.len(), 64);
    let wire = serde_json::to_string(&output.lineage).expect("wire metadata");
    assert!(!wire.contains(CHILD_CONTEXT));
    assert!(!wire.contains(CHILD_RESULT));
}

#[test]
fn parent_route_composition_refuses_missing_and_crossed_binding() {
    let config = parse_live_steel_agent_config(CONFIG).expect("config");
    let make_factory = || InjectedChildFactory::new(config.authority_component_identity(), |_admission: &animus_runtime::child_session_service::ConcreteChildSessionAdmission| {
        Ok((InjectedFinalProvider(std::rc::Rc::new(std::cell::Cell::new(0))), WorkspaceToolHost))
    });
    let empty = run_implementation_parent_profile(&config, lineage_binding(), make_factory(), |_route| Ok(()));
    assert!(matches!(empty, Err(ImplementationLineageError::Observation)));
    let mut crossed = lineage_binding();
    crossed.request_identity = "C".repeat(64);
    let rejected = run_implementation_parent_profile(&config, crossed, make_factory(), |_route| Ok(()));
    assert!(matches!(rejected, Err(ImplementationLineageError::Binding)));
    let limits = config.child_session_limits();
    let request = animus_adapter_rlm_kernel::ChildSpawnRequest::new(
        CHILD_PROFILE, CHILD_CONTEXT, 1, 1, limits.tokens, limits.wall_time_ms,
        limits.result_chars, false, false,
    ).expect("bounded request");
    let output = run_implementation_parent_profile(&config, lineage_binding(), make_factory(), |route| {
        route.execute_child(&request, config.executor_limits().max_child_status_checks)
            .map(|(result, _identity)| result)
            .map_err(|_error| ImplementationLineageError::Execution)
    }).expect("parent-owned route");
    assert_eq!(output.output, CHILD_RESULT);
    assert_eq!(output.lineage.edges[0].state, "completed");
}

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
            authority: ExplicitChildAuthority::admitted(config.authority_authorization_identity()),
            timer: FixedChildTimer::default(),
            events: MetadataChildEvents::default(),
        },
    )
    .expect("denied service");
    let child_config = config.clone();
    let mut admission_builder = move |candidate: &animus_adapter_rlm_kernel::ChildSpawnRequest| {
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
