//! Opt-in, attempt-bound lineage from a real concrete child-session lifecycle.

use animus_runtime::child_session_service::{ChildSessionLifecycleState, ConcreteChildSessionAdmission};
use animus_std::child_session_host::{
    ChildSessionEvent, ChildSessionEventSink, ChildSessionFactory, ChildSessionHostFailure,
    ConcreteChildSessionService, ConcreteChildSessionServices,
};
use animus_std_rlm_host::{ChildRoute, ConcreteChildRoute};

use crate::runtime::child::{ExplicitChildAuthority, MetadataChildStore, admit_child_request};

/// Sealed native-worker binding for one selected implementation attempt.
#[derive(Clone, Debug)]
pub struct ImplementationLineageBinding {
    /// Work identity.
    pub work_identity: String,
    /// Attempt identity.
    pub attempt_identity: String,
    /// Positive attempt fence.
    pub fence: u64,
    /// Digest of the sealed native request, not the child objective.
    pub request_identity: String,
    /// Worker incarnation.
    pub worker_incarnation: String,
    /// Selected profile revision.
    pub profile_revision: String,
}

/// Native terminal parent-child edge, without child output.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ImplementationChildEdge {
    /// Attempt-bound native parent identity.
    pub parent_id: String,
    /// Attempt-bound native child identity.
    pub child_id: String,
    /// Closed terminal lifecycle state.
    pub state: &'static str,
}

/// Complete bounded metadata-only depth-one child observation.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize)]
pub struct ImplementationChildLineage {
    /// Closed wire schema.
    pub schema: &'static str,
    /// Work identity.
    pub work_identity: String,
    /// Attempt identity.
    pub attempt_identity: String,
    /// Positive attempt fence.
    pub fence: u64,
    /// Sealed request identity.
    pub request_identity: String,
    /// Worker incarnation.
    pub worker_incarnation: String,
    /// Selected profile revision.
    pub profile_revision: String,
    /// Attempt-bound native parent identity.
    pub root_id: String,
    /// Complete native depth-one service observation.
    pub coverage: &'static str,
    /// At most 32 native child edges.
    pub edges: Vec<ImplementationChildEdge>,
}

/// Private result and separate metadata-only lineage.
pub struct ImplementationChildProfileResult {
    /// Bounded private child result.
    pub result: String,
    /// Wire-safe native lifecycle observation.
    pub lineage: ImplementationChildLineage,
}

/// Refusal of an invalid or unobserved child lifecycle.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ImplementationLineageError {
    /// Selected request or identity binding is invalid.
    Binding,
    /// Concrete child service failed; no complete evidence exists.
    Execution,
    /// Native lifecycle events are missing, inconsistent, or unbounded.
    Observation,
}

impl core::fmt::Display for ImplementationLineageError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "implementation child lineage unavailable: {self:?}")
    }
}

impl std::error::Error for ImplementationLineageError {}

struct MonotonicChildTimer(std::time::Instant);

impl animus_std::child_session_host::ChildSessionTimer for MonotonicChildTimer {
    fn now_ms(&mut self) -> Result<u64, ChildSessionHostFailure> {
        Ok(u64::try_from(self.0.elapsed().as_millis()).unwrap_or(u64::MAX))
    }
}

#[derive(Default)]
struct BoundedChildEvents(Vec<ChildSessionEvent>);

impl ChildSessionEventSink for BoundedChildEvents {
    fn observe(&mut self, event: &ChildSessionEvent) -> Result<(), ChildSessionHostFailure> {
        if self.0.len() >= 1024 {
            return Err(ChildSessionHostFailure::new(
                animus_runtime::child_session_service::ChildSessionFailureClass::BoundExceeded,
                "implementation child event bound exceeded",
            ));
        }
        self.0.push(event.clone());
        Ok(())
    }
}

/// Runs one depth-one concrete child with an explicitly injected, real factory.
/// Use [`crate::InjectedChildFactory`] for injected provider and workspace tool
/// services. The legacy `ExplicitChildFactory` is a fake and is not selected here.
///
/// # Errors
///
/// Refuses invalid binding, failed execution, or missing native observation.
pub fn run_implementation_child_profile<Create, Provider, Tool>(
    config: &crate::ValidatedLiveSteelAgentConfig,
    request: &animus_adapter_rlm_kernel::ChildSpawnRequest,
    binding: ImplementationLineageBinding,
    factory: crate::InjectedChildFactory<Create, Provider, Tool>,
) -> Result<ImplementationChildProfileResult, ImplementationLineageError>
where
    Create: FnMut(&ConcreteChildSessionAdmission) -> Result<(Provider, Tool), ChildSessionHostFailure>,
    Provider: animus_std::coding_session_host::CodingSessionProviderService + 'static,
    Tool: animus_std::coding_session_host::CodingSessionToolService + 'static,
{
    validate_binding(&binding)?;
    if request.depth() != 1 || request.fan_out() != 1 {
        return Err(ImplementationLineageError::Binding);
    }
    if factory.service_identity() != config.authority_component_identity() {
        return Err(ImplementationLineageError::Binding);
    }
    let expected = admit_child_request(config, request).map_err(|_error| ImplementationLineageError::Binding)?;
    let mut service = ConcreteChildSessionService::new(
        config.child_service_label(),
        ConcreteChildSessionServices {
            factory,
            store: MetadataChildStore::default(),
            authority: ExplicitChildAuthority::admitted(config.authority_authorization_identity()),
            timer: MonotonicChildTimer(std::time::Instant::now()),
            events: BoundedChildEvents::default(),
        },
    ).map_err(|_error| ImplementationLineageError::Binding)?;
    let mut admission = |candidate: &animus_adapter_rlm_kernel::ChildSpawnRequest| {
        let actual = admit_child_request(config, candidate)?;
        if actual.admission_identity() != expected.admission_identity() {
            return Err(animus_std_rlm_host::DispatchFailure::new(
                animus_std_rlm_host::RouteClass::Subagent,
                animus_std_rlm_host::DispatchFailureClass::Denied,
                "child_admission",
            ));
        }
        Ok(actual)
    };
    let (result, _route_identity) = ConcreteChildRoute { service: &mut service, admission: &mut admission }
        .execute_child(request, config.executor_limits().max_child_status_checks)
        .map_err(|_error| ImplementationLineageError::Execution)?;
    let (_factory, _store, _authority, _timer, events) = service.into_services();
    let lineage = project_lineage(config.config_identity(), binding, &[expected], &events.0)?;
    Ok(ImplementationChildProfileResult { result, lineage })
}

/// Parent result and its own child's observed metadata-only lineage.
pub struct ImplementationParentProfileResult<Output> {
    /// The private result returned by the parent composition.
    pub output: Output,
    /// Bounded lifecycle evidence from its concrete child route.
    pub lineage: ImplementationChildLineage,
}

/// Composes the opt-in concrete child route inside a real parent Steel session.
/// The callback may construct `build_executor(config, context, child_route)` and
/// run `LiveSteelAgentComposition` with its own injected parent provider and
/// bounded workspace services. No child route or fake provider is discovered.
///
/// # Errors
///
/// Refuses an incomplete or crossed child lifecycle even when the parent succeeds.
pub fn run_implementation_parent_profile<Create, Provider, Tool, Output>(
    config: &crate::ValidatedLiveSteelAgentConfig,
    binding: ImplementationLineageBinding,
    factory: crate::InjectedChildFactory<Create, Provider, Tool>,
    run_parent: impl FnOnce(&mut dyn ChildRoute) -> Result<Output, ImplementationLineageError>,
) -> Result<ImplementationParentProfileResult<Output>, ImplementationLineageError>
where
    Create: FnMut(&ConcreteChildSessionAdmission) -> Result<(Provider, Tool), ChildSessionHostFailure>,
    Provider: animus_std::coding_session_host::CodingSessionProviderService + 'static,
    Tool: animus_std::coding_session_host::CodingSessionToolService + 'static,
{
    if factory.service_identity() != config.authority_component_identity() {
        return Err(ImplementationLineageError::Binding);
    }
    validate_binding(&binding)?;
    let mut service = ConcreteChildSessionService::new(
        config.child_service_label(),
        ConcreteChildSessionServices {
            factory,
            store: MetadataChildStore::default(),
            authority: ExplicitChildAuthority::admitted(config.authority_authorization_identity()),
            timer: MonotonicChildTimer(std::time::Instant::now()),
            events: BoundedChildEvents::default(),
        },
    ).map_err(|_error| ImplementationLineageError::Binding)?;
    let mut admissions = Vec::new();
    let output = {
        let mut admit = |request: &animus_adapter_rlm_kernel::ChildSpawnRequest| {
            if request.depth() != 1 || request.fan_out() != 1 || admissions.len() >= 32 {
                return Err(animus_std_rlm_host::DispatchFailure::new(
                    animus_std_rlm_host::RouteClass::Subagent,
                    animus_std_rlm_host::DispatchFailureClass::BoundExceeded,
                    "implementation_child_bound",
                ));
            }
            let admission = admit_child_request(config, request)?;
            admissions.push(admission.clone());
            Ok(admission)
        };
        let mut route = ConcreteChildRoute { service: &mut service, admission: &mut admit };
        run_parent(&mut route)?
    };
    let (_factory, _store, _authority, _timer, events) = service.into_services();
    let lineage = project_lineage(config.config_identity(), binding, &admissions, &events.0)?;
    Ok(ImplementationParentProfileResult { output, lineage })
}

fn validate_binding(binding: &ImplementationLineageBinding) -> Result<(), ImplementationLineageError> {
    let identities = [
        binding.work_identity.as_str(), binding.attempt_identity.as_str(),
        binding.request_identity.as_str(), binding.worker_incarnation.as_str(),
        binding.profile_revision.as_str(),
    ];
    if binding.fence == 0 || identities.iter().any(|identity| !lower_hex_digest(identity)) {
        Err(ImplementationLineageError::Binding)
    } else {
        Ok(())
    }
}

fn lower_hex_digest(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn bound_native_id(config_identity: &str, binding: &ImplementationLineageBinding, native: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    for part in [
        "animus.implementation-child-lineage.id.v1", config_identity,
        binding.work_identity.as_str(), binding.attempt_identity.as_str(),
        binding.request_identity.as_str(), binding.worker_incarnation.as_str(),
        binding.profile_revision.as_str(), native,
    ] {
        hasher.update(&(part.len() as u64).to_le_bytes());
        hasher.update(part.as_bytes());
    }
    hasher.update(&binding.fence.to_le_bytes());
    hasher.finalize().to_hex().to_string()
}

fn project_lineage(
    config_identity: &str,
    binding: ImplementationLineageBinding,
    admissions: &[ConcreteChildSessionAdmission],
    events: &[ChildSessionEvent],
) -> Result<ImplementationChildLineage, ImplementationLineageError> {
    if admissions.is_empty() || admissions.len() > 32 || events.len() > 1024 {
        return Err(ImplementationLineageError::Observation);
    }
    let mut edges = Vec::with_capacity(admissions.len());
    let mut native_root = None;
    for admission in admissions {
        let mut matching = events.iter().filter(|event| {
            event.parent_id == admission.subagent().parent_id
                && event.child_id == admission.subagent().child_id
                && event.admission_identity == admission.admission_identity()
        });
        let first = matching.next().ok_or(ImplementationLineageError::Observation)?;
        let mut last = first;
        for event in matching {
            if last.state.is_terminal() || event.generation != first.generation {
                return Err(ImplementationLineageError::Observation);
            }
            last = event;
        }
        if first.generation == 0
            || events.iter().any(|event| event.generation == first.generation && event.child_id != first.child_id)
            || !matches!(first.state, ChildSessionLifecycleState::Running | ChildSessionLifecycleState::AwaitingParent)
        {
            return Err(ImplementationLineageError::Observation);
        }
        if native_root.is_some_and(|root| root != first.parent_id) {
            return Err(ImplementationLineageError::Observation);
        }
        native_root = Some(first.parent_id.as_str());
        let state = match last.state {
            ChildSessionLifecycleState::Completed => "completed",
            ChildSessionLifecycleState::Partial => "partial",
            ChildSessionLifecycleState::Failed => "failed",
            ChildSessionLifecycleState::TimedOut => "timed_out",
            ChildSessionLifecycleState::Cancelled => "cancelled",
            ChildSessionLifecycleState::BoundExceeded => "bound_exceeded",
            _non_terminal => return Err(ImplementationLineageError::Observation),
        };
        if matches!(last.state, ChildSessionLifecycleState::Completed | ChildSessionLifecycleState::Partial)
            && last.result_identity.as_ref().is_none_or(|identity| !lower_hex_digest(identity))
        {
            return Err(ImplementationLineageError::Observation);
        }
        let child_id = bound_native_id(config_identity, &binding, first.child_id.as_str());
        if edges.iter().any(|edge: &ImplementationChildEdge| edge.child_id == child_id) {
            return Err(ImplementationLineageError::Observation);
        }
        edges.push(ImplementationChildEdge {
            parent_id: bound_native_id(config_identity, &binding, first.parent_id.as_str()),
            child_id,
            state,
        });
    }
    if events.iter().any(|event| !admissions.iter().any(|admission| {
        event.parent_id == admission.subagent().parent_id
            && event.child_id == admission.subagent().child_id
            && event.admission_identity == admission.admission_identity()
    })) {
        return Err(ImplementationLineageError::Observation);
    }
    let root_id = bound_native_id(config_identity, &binding, native_root.ok_or(ImplementationLineageError::Observation)?);
    if edges.iter().any(|edge| edge.parent_id != root_id || edge.child_id == root_id) {
        return Err(ImplementationLineageError::Observation);
    }
    Ok(ImplementationChildLineage {
        schema: "animus.implementation-child-lineage.v1",
        work_identity: binding.work_identity,
        attempt_identity: binding.attempt_identity,
        fence: binding.fence,
        request_identity: binding.request_identity,
        worker_incarnation: binding.worker_incarnation,
        profile_revision: binding.profile_revision,
        root_id,
        coverage: "complete",
        edges,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn binding() -> ImplementationLineageBinding {
        ImplementationLineageBinding {
            work_identity: "a".repeat(64), attempt_identity: "b".repeat(64), fence: 1,
            request_identity: "c".repeat(64), worker_incarnation: "d".repeat(64),
            profile_revision: "e".repeat(64),
        }
    }

    #[test]
    fn digest_binding_is_length_delimited_and_attempt_fenced() {
        let original = binding();
        let root = bound_native_id("config-a", &original, "parent.live-steel-agent");
        assert_ne!(root, bound_native_id("config-a", &original, "child.live-steel-agent"));
        assert_ne!(root, bound_native_id("config-b", &original, "parent.live-steel-agent"));
        let mut crossed = original.clone();
        crossed.fence += 1;
        assert_ne!(root, bound_native_id("config-a", &crossed, "parent.live-steel-agent"));
        crossed = original;
        crossed.request_identity = "f".repeat(64);
        assert_ne!(root, bound_native_id("config-a", &crossed, "parent.live-steel-agent"));
    }

    #[test]
    fn projection_refuses_crossed_and_incomplete_native_events() {
        let config = crate::parse_live_steel_agent_config(include_bytes!("../../../config/generated/live-steel-agent-shell.json"))
            .expect("fixture config");
        let limits = config.child_session_limits();
        let request = animus_adapter_rlm_kernel::ChildSpawnRequest::new(
            "focused-child", "bounded objective", 1, 1, limits.tokens,
            limits.wall_time_ms, limits.result_chars, false, false,
        ).expect("child request");
        let admission = admit_child_request(&config, &request).expect("admission");
        let running = ChildSessionEvent {
            parent_id: admission.subagent().parent_id.clone(),
            child_id: admission.subagent().child_id.clone(),
            generation: 1,
            state: ChildSessionLifecycleState::Running,
            admission_identity: admission.admission_identity().to_owned(),
            result_identity: None,
            failure_class: None,
        };
        assert!(matches!(project_lineage(config.config_identity(), binding(), &[admission.clone()], &[]), Err(ImplementationLineageError::Observation)));
        assert!(matches!(project_lineage(config.config_identity(), binding(), &[admission.clone()], &[running.clone()]), Err(ImplementationLineageError::Observation)));
        let mut terminal = running.clone();
        terminal.state = ChildSessionLifecycleState::Completed;
        terminal.result_identity = Some("f".repeat(64));
        let mut crossed = terminal.clone();
        crossed.parent_id = String::from("other-parent");
        assert!(matches!(project_lineage(config.config_identity(), binding(), &[admission.clone()], &[running.clone(), crossed]), Err(ImplementationLineageError::Observation)));
        let mut crossed = terminal.clone();
        crossed.generation = 2;
        assert!(matches!(project_lineage(config.config_identity(), binding(), &[admission.clone()], &[running.clone(), crossed]), Err(ImplementationLineageError::Observation)));
        let mut resultless = terminal.clone();
        resultless.result_identity = None;
        assert!(matches!(project_lineage(config.config_identity(), binding(), &[admission.clone()], &[running.clone(), resultless]), Err(ImplementationLineageError::Observation)));
        assert!(matches!(project_lineage(config.config_identity(), binding(), &[admission], &[running, terminal.clone(), terminal]), Err(ImplementationLineageError::Observation)));
    }
}
