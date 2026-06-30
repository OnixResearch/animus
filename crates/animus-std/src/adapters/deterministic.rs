const FIXTURE_MODEL_LABEL: &str = "fixture_model";
const FIXTURE_FILESYSTEM_LABEL: &str = "fixture_filesystem";
const FIXTURE_PROCESS_LABEL: &str = "fixture_process";
const FIXTURE_NETWORK_LABEL: &str = "fixture_network";
const FIXTURE_CLOCK_LABEL: &str = "fixture_clock";
const FIXTURE_CREDENTIAL_LABEL: &str = "fixture_credential";
const FIXTURE_MODEL_MESSAGE: &str = "fixture model response";
const FIXTURE_TOOL_OUTPUT: &str = "fixture tool output";
const FIXTURE_MODEL_OUTPUT_LABEL: &str = "fixture_model_ok";
const FIXTURE_FILESYSTEM_OUTPUT_LABEL: &str = "fixture_filesystem_ok";
const FIXTURE_PROCESS_OUTPUT_LABEL: &str = "fixture_process_ok";
const FIXTURE_NETWORK_OUTPUT_LABEL: &str = "fixture_network_ok";
const FIXTURE_CLOCK_OUTPUT_LABEL: &str = "fixture_clock_ok";
const FIXTURE_CREDENTIAL_OUTPUT_LABEL: &str = "fixture_credential_ok";

/// Deterministic fixture adapter kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureKind {
    /// Fixture model behavior.
    Model,
    /// Fixture filesystem behavior.
    Filesystem,
    /// Fixture process behavior.
    Process,
    /// Fixture network behavior.
    Network,
    /// Fixture clock behavior.
    Clock,
    /// Fixture credential behavior.
    Credential,
}

impl FixtureKind {
    /// Returns the stable fixture label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Filesystem => "filesystem",
            Self::Process => "process",
            Self::Network => "network",
            Self::Clock => "clock",
            Self::Credential => "credential",
        }
    }

    const fn descriptor(self) -> super::taxonomy::AdapterDescriptor<'static> {
        match self {
            Self::Model => super::taxonomy::AdapterDescriptor::fixture(FIXTURE_MODEL_LABEL),
            Self::Filesystem => {
                super::taxonomy::AdapterDescriptor::fixture(FIXTURE_FILESYSTEM_LABEL)
            }
            Self::Process => super::taxonomy::AdapterDescriptor::fixture(FIXTURE_PROCESS_LABEL),
            Self::Network => super::taxonomy::AdapterDescriptor::fixture(FIXTURE_NETWORK_LABEL),
            Self::Clock => super::taxonomy::AdapterDescriptor::fixture(FIXTURE_CLOCK_LABEL),
            Self::Credential => {
                super::taxonomy::AdapterDescriptor::fixture(FIXTURE_CREDENTIAL_LABEL)
            }
        }
    }

    const fn output_label(self) -> &'static str {
        match self {
            Self::Model => FIXTURE_MODEL_OUTPUT_LABEL,
            Self::Filesystem => FIXTURE_FILESYSTEM_OUTPUT_LABEL,
            Self::Process => FIXTURE_PROCESS_OUTPUT_LABEL,
            Self::Network => FIXTURE_NETWORK_OUTPUT_LABEL,
            Self::Clock => FIXTURE_CLOCK_OUTPUT_LABEL,
            Self::Credential => FIXTURE_CREDENTIAL_OUTPUT_LABEL,
        }
    }
}

/// Deterministic fixture output with a safe plan receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureOutput {
    kind: FixtureKind,
    output_label: &'static str,
    plan: super::outcome::AdapterPlan,
}

impl FixtureOutput {
    /// Returns the fixture kind that produced this output.
    #[must_use]
    pub const fn kind(&self) -> FixtureKind {
        self.kind
    }

    /// Returns a safe output label rather than a raw payload.
    #[must_use]
    pub const fn output_label(&self) -> &'static str {
        self.output_label
    }

    /// Returns the adapter plan and safe receipt.
    #[must_use]
    pub const fn plan(&self) -> &super::outcome::AdapterPlan {
        &self.plan
    }
}

/// Deterministic in-memory fixture bundle for default std-shell tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Fixtures {
    prerequisites: super::gates::EffectPrerequisites,
}

impl Fixtures {
    /// Builds fixture adapters with all gates satisfied.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            prerequisites: super::gates::EffectPrerequisites::satisfied(),
        }
    }

    /// Builds fixture adapters with explicit prerequisite state.
    #[must_use]
    pub const fn with_prerequisites(prerequisites: super::gates::EffectPrerequisites) -> Self {
        Self { prerequisites }
    }

    /// Dispatches a deterministic fixture request.
    ///
    /// # Errors
    ///
    /// Returns [`animus_adapters::SafeReceiptError`] when safe receipt metadata
    /// cannot be constructed.
    pub fn dispatch(
        &self,
        kind: FixtureKind,
        payload: &str,
    ) -> Result<FixtureOutput, animus_adapters::SafeReceiptError> {
        let plan =
            super::planning::plan_adapter_effect(kind.descriptor(), self.prerequisites, payload)?;
        Ok(FixtureOutput {
            kind,
            output_label: kind.output_label(),
            plan,
        })
    }
}

impl Default for Fixtures {
    fn default() -> Self {
        Self::new()
    }
}

/// Deterministic model fixture implementing the model adapter contract.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FixtureModelAdapter;

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for FixtureModelAdapter
{
    type Error = animus_adapters::AdapterContractError;

    fn call(
        &mut self,
        _request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        animus_adapters::ModelResponse::new(FIXTURE_MODEL_MESSAGE)
    }
}

/// Deterministic tool fixture implementing the tool adapter contract.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FixtureToolAdapter;

impl animus_adapters::ComponentAdapter<animus_adapters::ToolRequest, animus_adapters::ToolResponse>
    for FixtureToolAdapter
{
    type Error = animus_adapters::AdapterContractError;

    fn call(
        &mut self,
        _request: &animus_adapters::ToolRequest,
    ) -> Result<animus_adapters::ToolResponse, Self::Error> {
        animus_adapters::ToolResponse::new(FIXTURE_TOOL_OUTPUT)
    }
}

#[cfg(test)]
pub(super) mod test_support {
    pub(in crate::adapters) const FIXTURE_MODEL_MESSAGE: &str = super::FIXTURE_MODEL_MESSAGE;
    pub(in crate::adapters) const FIXTURE_TOOL_OUTPUT: &str = super::FIXTURE_TOOL_OUTPUT;
}
