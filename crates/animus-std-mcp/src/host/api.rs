use std::sync::atomic::AtomicBool;

use serde_json::Value;

use super::flow;
use super::{
    CallOutput, CredentialService, HostError, LaunchAuthority, ServerUnavailable, StartArtifacts,
};
use crate::process::StdioSession;
use crate::{
    CallPlan, DiscoveredCatalog, InstructionContribution, LifecycleState, McpHostConfig,
    ProcessScope, TeardownObservation,
};

/// Concrete non-default MCP standard-input client host.
pub struct McpClientHost {
    pub(crate) config: McpHostConfig,
    pub(crate) lifecycle: LifecycleState,
    pub(crate) catalog: DiscoveredCatalog,
    pub(crate) session: Option<StdioSession>,
    pub(crate) credentials: Box<dyn CredentialService>,
    pub(crate) instructions: Option<InstructionContribution>,
    pub(crate) server_info_identity: String,
    pub(crate) next_request_id: u64,
    pub(crate) process_scope: ProcessScope,
}

impl McpClientHost {
    /// Starts, initializes, and discovers one exact configured stdio server.
    ///
    /// # Errors
    ///
    /// Returns a bounded startup error after owned teardown when process effects began.
    // r[impl agent_harness_runtime.mcp_client_host.boundary]
    pub fn connect(
        config: McpHostConfig,
        generation: u64,
        launch: LaunchAuthority,
        credentials: Box<dyn CredentialService>,
    ) -> Result<Self, HostError> {
        let connected = flow::connect(config, generation, launch, credentials)?;
        Ok(Self {
            config: connected.config,
            lifecycle: connected.lifecycle,
            catalog: connected.catalog,
            session: Some(connected.session),
            credentials: connected.credentials,
            instructions: connected.artifacts.instructions,
            server_info_identity: connected.artifacts.server_info_identity,
            next_request_id: crate::protocol::FIRST_CALL_RESPONSE_ID,
            process_scope: connected.scope,
        })
    }

    /// Starts one server and preserves required or optional unavailability.
    ///
    /// # Errors
    ///
    /// Returns a visible unavailable entry without server substitution.
    pub fn connect_visible(
        config: McpHostConfig,
        generation: u64,
        launch: LaunchAuthority,
        credentials: Box<dyn CredentialService>,
    ) -> Result<Self, ServerUnavailable> {
        let posture = config.server().posture;
        Self::connect(config, generation, launch, credentials)
            .map_err(|error| ServerUnavailable { posture, error })
    }

    /// Dispatches one already-planned call to the exact current generation.
    ///
    /// # Errors
    ///
    /// Returns a bounded authority, credential, protocol, cancellation, or teardown error.
    pub fn call(
        &mut self,
        plan: &CallPlan,
        arguments: &Value,
        cancellation: &AtomicBool,
    ) -> Result<CallOutput, HostError> {
        flow::call(self, plan, arguments, cancellation)
    }

    /// Stops the owned process tree and records terminal or uncertain teardown.
    #[must_use]
    pub fn stop(&mut self) -> TeardownObservation {
        flow::stop(self)
    }

    /// Returns the exact validated configuration.
    #[must_use]
    pub const fn config(&self) -> &McpHostConfig {
        &self.config
    }

    /// Returns the current lifecycle.
    #[must_use]
    pub const fn lifecycle(&self) -> &LifecycleState {
        &self.lifecycle
    }

    /// Returns the complete current catalog.
    #[must_use]
    pub const fn catalog(&self) -> &DiscoveredCatalog {
        &self.catalog
    }

    /// Returns request-local instructions, when supplied by the server.
    #[must_use]
    pub const fn instructions(&self) -> Option<&InstructionContribution> {
        self.instructions.as_ref()
    }

    /// Returns the server-info identity without server-info text.
    #[must_use]
    pub fn server_info_identity(&self) -> &str {
        self.server_info_identity.as_str()
    }

    /// Returns the platform process termination scope.
    #[must_use]
    pub const fn process_scope(&self) -> ProcessScope {
        self.process_scope
    }
}

pub(crate) struct ConnectedHost {
    pub(crate) config: McpHostConfig,
    pub(crate) lifecycle: LifecycleState,
    pub(crate) catalog: DiscoveredCatalog,
    pub(crate) session: StdioSession,
    pub(crate) credentials: Box<dyn CredentialService>,
    pub(crate) artifacts: StartArtifacts,
    pub(crate) scope: ProcessScope,
}
