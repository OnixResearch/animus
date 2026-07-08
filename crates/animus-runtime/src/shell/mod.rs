mod runner;

const NO_PLANNING_STEPS_REMAINING: usize = 0;
const PLANNING_GUARD_EXHAUSTED: &str = "runtime planning step guard exhausted";

/// Borrowed injected model/tool service bundle used by runtime runners.
pub struct RuntimeAdapters<'a, Model, Tool> {
    model: Option<&'a mut Model>,
    tool: Option<&'a mut Tool>,
    middleware: crate::RuntimeMiddlewareStack<'a>,
    capabilities: crate::RuntimeCapabilityRegistry,
    tool_definitions: Vec<animus_adapters::tool::Definition>,
}

impl<'a, Model, Tool> RuntimeAdapters<'a, Model, Tool> {
    /// Builds a borrowed adapter bundle with required model and tool services.
    #[must_use]
    pub fn new(model: &'a mut Model, tool: &'a mut Tool) -> Self {
        Self {
            model: Some(model),
            tool: Some(tool),
            middleware: crate::RuntimeMiddlewareStack::empty(),
            capabilities: crate::RuntimeCapabilityRegistry::model_tool_presence(true, true),
            tool_definitions: Vec::new(),
        }
    }

    /// Builds a borrowed adapter bundle that can represent absent services.
    #[must_use]
    pub fn new_optional(model: Option<&'a mut Model>, tool: Option<&'a mut Tool>) -> Self {
        let is_model_present = model.is_some();
        let is_tool_present = tool.is_some();
        Self {
            model,
            tool,
            middleware: crate::RuntimeMiddlewareStack::empty(),
            capabilities: crate::RuntimeCapabilityRegistry::model_tool_presence(
                is_model_present,
                is_tool_present,
            ),
            tool_definitions: Vec::new(),
        }
    }

    /// Adds host-injected middleware hooks in deterministic execution order.
    #[must_use]
    pub fn with_middleware(mut self, middleware: crate::RuntimeMiddlewareStack<'a>) -> Self {
        self.middleware = middleware;
        self
    }

    /// Replaces the explicit capability registry used for dispatch admission.
    #[must_use]
    pub fn with_capability_registry(
        mut self,
        capabilities: crate::RuntimeCapabilityRegistry,
    ) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Replaces the active trusted tool definitions used for dispatch admission.
    #[must_use]
    pub fn with_tool_definitions(
        mut self,
        tool_definitions: Vec<animus_adapters::tool::Definition>,
    ) -> Self {
        self.tool_definitions = tool_definitions;
        self
    }

    /// Returns the explicit capability registry used for dispatch admission.
    #[must_use]
    pub const fn capability_registry(&self) -> &crate::RuntimeCapabilityRegistry {
        &self.capabilities
    }

    /// Returns the active trusted tool definitions used for dispatch admission.
    #[must_use]
    pub fn tool_definitions(&self) -> &[animus_adapters::tool::Definition] {
        self.tool_definitions.as_slice()
    }
}

/// Checked Steel profile and script selected for one runtime execution.
#[cfg(feature = "steel")]
pub struct SteelPolicyRuntime<'profile, 'script> {
    profile: &'profile animus_steel::SteelPolicyProfile,
    script: animus_steel::SteelPolicyScript<'script>,
}

#[cfg(feature = "steel")]
impl<'profile, 'script> SteelPolicyRuntime<'profile, 'script> {
    /// Builds a checked Steel policy runtime selection.
    #[must_use]
    pub const fn new(
        profile: &'profile animus_steel::SteelPolicyProfile,
        script: animus_steel::SteelPolicyScript<'script>,
    ) -> Self {
        Self { profile, script }
    }

    /// Returns the selected policy profile.
    #[must_use]
    pub const fn profile(&self) -> &'profile animus_steel::SteelPolicyProfile {
        self.profile
    }

    /// Returns the selected Steel script.
    #[must_use]
    pub const fn script(&self) -> animus_steel::SteelPolicyScript<'script> {
        self.script
    }
}

/// Runs the agent loop with caller-supplied fabric and adapters.
#[must_use]
pub fn run_agent_loop<Fabric, Model, Tool>(
    input: &crate::RuntimeInput,
    config: crate::RuntimeConfig,
    fabric: &mut Fabric,
    mut adapters: RuntimeAdapters<'_, Model, Tool>,
) -> crate::RunReport
where
    Fabric: crate::LoopFabric,
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    let mut state = crate::TurnState::new(input);
    let mut planning_steps_remaining = config
        .max_turns()
        .count()
        .saturating_add(crate::MIN_MAX_TURNS.count());

    while planning_steps_remaining != NO_PLANNING_STEPS_REMAINING {
        planning_steps_remaining =
            planning_steps_remaining.saturating_sub(crate::MIN_MAX_TURNS.count());
        match runner::run_planned_step(config, state, fabric, &mut adapters) {
            runner::Step::Continue { state: next_state } => state = next_state,
            runner::Step::Complete { report } => return report,
        }
    }

    crate::complete_with_fabric_error(state, String::from(PLANNING_GUARD_EXHAUSTED))
}

/// Runs the agent loop with a Steel program as the default loop fabric.
#[cfg(feature = "steel")]
#[must_use]
pub fn run_with_steel_program<Model, Tool>(
    program: &str,
    input: &crate::RuntimeInput,
    config: crate::RuntimeConfig,
    adapters: RuntimeAdapters<'_, Model, Tool>,
) -> crate::RunReport
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    match animus_steel::SteelLoopFabric::new(program) {
        Ok(mut fabric) => run_agent_loop(input, config, &mut fabric, adapters),
        Err(error) => {
            crate::complete_with_fabric_error(crate::TurnState::new(input), error.to_string())
        }
    }
}

/// Runs the agent loop with a Steel program after checked policy activation.
#[cfg(feature = "steel")]
#[must_use]
pub fn run_with_steel_policy<Model, Tool>(
    policy: &SteelPolicyRuntime<'_, '_>,
    input: &crate::RuntimeInput,
    config: crate::RuntimeConfig,
    adapters: RuntimeAdapters<'_, Model, Tool>,
) -> crate::RunReport
where
    Model: animus_adapters::ModelAdapter,
    Model::Error: core::fmt::Display,
    Tool: animus_adapters::ToolAdapter,
    Tool::Error: core::fmt::Display,
{
    let activation = animus_steel::SteelPolicyActivation::new(
        policy.script(),
        config.max_turns().policy_count(),
        animus_steel::DEFAULT_RUNTIME_HOST_ACTIONS,
    );
    let receipt = policy.profile().activate(&activation);
    if !receipt.is_allowed() {
        return crate::complete_with_steel_policy_denial(crate::TurnState::new(input), &receipt);
    }

    match animus_steel::SteelLoopFabric::new(policy.script().source()) {
        Ok(mut fabric) => run_agent_loop(input, config, &mut fabric, adapters),
        Err(error) => {
            crate::complete_with_fabric_error(crate::TurnState::new(input), error.to_string())
        }
    }
}

#[cfg(all(test, feature = "steel"))]
mod tests;
