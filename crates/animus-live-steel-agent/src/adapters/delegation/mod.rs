pub(crate) mod admission;
mod controller;
pub(crate) mod services;
// Explicit concrete child-session service composition.

const CHILD_PARENT_ID: &str = "parent.live-steel-agent";
const CHILD_WORKSPACE_SCOPE: &str = "workspace.explicit-none";
const CHILD_TOOL_LABEL: &str = "steel_repl";
const CHILD_CONTEXT_LABEL: &str = "selected-context";
const CHILD_PACKET_PREFIX: &str = "packet.live-steel-agent.";
const CHILD_ID_PREFIX: &str = "child.live-steel-agent.";
const CHILD_SCHEDULER_LABEL: &str = "scheduler.live-steel-agent";
const CHILD_CONTEXT_TOKEN_ESTIMATE: u32 = 1;
const CHILD_BRANCH_ID: &str = "branch.live-steel-agent-child";
const CHILD_QUEUE_POLICY_LABEL: &str = "live-steel-agent-child-queue";
const CHILD_RUN_PREFIX: &str = "run.live-steel-agent-child.";
const CHILD_ENTRY_PREFIX: &str = "entry.live-steel-agent-child.user.";
const MAX_CHILD_QUEUE_ITEMS: u32 = 1;
const CHILD_SUMMARY: &str = "bounded child summary";

/// Explicit child-session factory with no provider or credential discovery.
#[derive(Clone)]
pub struct ExplicitChildFactory {
    service_identity: String,
    response: String,
    deny: bool,
}

impl ExplicitChildFactory {
    /// Builds one admitted deterministic child factory.
    #[must_use]
    pub fn admitted(service_identity: impl Into<String>, response: impl Into<String>) -> Self {
        Self {
            service_identity: service_identity.into(),
            response: response.into(),
            deny: false,
        }
    }

    /// Builds one explicit denied child factory for negative checks.
    #[must_use]
    pub fn denied(service_identity: impl Into<String>) -> Self {
        Self {
            service_identity: service_identity.into(),
            response: String::new(),
            deny: true,
        }
    }
}

impl animus_std::child_session_host::ChildSessionFactory for ExplicitChildFactory {
    fn services(&self) -> animus_runtime::child_session_service::ChildSessionServiceSet {
        animus_runtime::child_session_service::ChildSessionServiceSet::all_available()
    }

    fn service_identity(&self) -> &str {
        self.service_identity.as_str()
    }

    fn create(
        &mut self,
        admission: &animus_runtime::child_session_service::ConcreteChildSessionAdmission,
    ) -> Result<
        Box<dyn animus_std::child_session_host::ChildCodingSessionController>,
        animus_std::child_session_host::ChildSessionHostFailure,
    > {
        if self.deny {
            return Err(
                animus_std::child_session_host::ChildSessionHostFailure::new(
                    animus_runtime::child_session_service::ChildSessionFailureClass::Denied,
                    "explicit child factory denied creation",
                ),
            );
        }
        controller::build(admission, String::clone(&self.response), false)
    }

    fn restore(
        &mut self,
        admission: &animus_runtime::child_session_service::ConcreteChildSessionAdmission,
        _record: &animus_runtime::child_session_service::ChildSessionLifecycleRecord,
    ) -> Result<
        Box<dyn animus_std::child_session_host::ChildCodingSessionController>,
        animus_std::child_session_host::ChildSessionHostFailure,
    > {
        if self.deny {
            return Err(
                animus_std::child_session_host::ChildSessionHostFailure::new(
                    animus_runtime::child_session_service::ChildSessionFailureClass::Denied,
                    "explicit child factory denied restore",
                ),
            );
        }
        controller::build(admission, String::clone(&self.response), true)
    }
}

/// Concrete child service type selected by the live application.
pub type LiveConcreteChildService = animus_std::child_session_host::ConcreteChildSessionService<
    ExplicitChildFactory,
    services::MetadataChildStore,
    services::ExplicitChildAuthority,
    services::FixedChildTimer,
    services::MetadataChildEvents,
>;

/// Builds the concrete child service selected at the visible root.
///
/// # Errors
///
/// Returns a safe host failure when the service identity is malformed.
pub fn build_child_service(
    config: &crate::ValidatedLiveSteelAgentConfig,
    response: impl Into<String>,
) -> Result<LiveConcreteChildService, animus_std::child_session_host::ChildSessionHostFailure> {
    animus_std::child_session_host::ConcreteChildSessionService::new(
        config.child_service_label(),
        animus_std::child_session_host::ConcreteChildSessionServices {
            factory: ExplicitChildFactory::admitted(
                config.authority_component_identity(),
                response,
            ),
            store: services::MetadataChildStore::default(),
            authority: services::ExplicitChildAuthority::admitted(
                config.authority_authorization_identity(),
            ),
            timer: services::FixedChildTimer::default(),
            events: services::MetadataChildEvents::default(),
        },
    )
}
