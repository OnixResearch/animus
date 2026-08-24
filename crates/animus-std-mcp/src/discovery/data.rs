/// One discovered tool after existing MCP and Animus mapping validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveredTool {
    pub(crate) mapping: animus_adapters::McpToolMapping,
    pub(crate) definition: animus_adapters::tool::Definition,
    pub(crate) description_identity: String,
    pub(crate) read_only_hint: crate::ReadOnlyHint,
}

impl DiscoveredTool {
    /// Returns the validated MCP mapping.
    #[must_use]
    pub const fn mapping(&self) -> &animus_adapters::McpToolMapping {
        &self.mapping
    }

    /// Returns the mapped Animus tool definition.
    #[must_use]
    pub const fn definition(&self) -> &animus_adapters::tool::Definition {
        &self.definition
    }

    /// Returns the description identity without retaining description text.
    #[must_use]
    pub fn description_identity(&self) -> &str {
        self.description_identity.as_str()
    }

    /// Returns the untrusted read-only annotation classification.
    #[must_use]
    pub const fn read_only_hint(&self) -> crate::ReadOnlyHint {
        self.read_only_hint
    }
}

/// Complete bounded catalog for one exact server generation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveredCatalog {
    pub(crate) identity: String,
    pub(crate) generation: u64,
    pub(crate) tools: Vec<DiscoveredTool>,
}

impl DiscoveredCatalog {
    /// Returns the complete catalog identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the exact bound generation.
    #[must_use]
    pub const fn generation(&self) -> u64 {
        self.generation
    }

    /// Returns all admitted tools.
    #[must_use]
    pub fn tools(&self) -> &[DiscoveredTool] {
        self.tools.as_slice()
    }

    /// Finds one exact admitted tool.
    #[must_use]
    pub fn tool(&self, name: &str) -> Option<&DiscoveredTool> {
        self.tools
            .iter()
            .find(|tool| tool.mapping.tool_name() == name)
    }
}

/// Bounded discovery failure without raw server payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiscoveryError {
    /// JSON-RPC response framing or shape was malformed.
    MalformedResponse,
    /// The response correlation did not match the request.
    CorrelationDrift,
    /// Tool count or metadata exceeded configured limits.
    BudgetExceeded,
    /// A tool name was duplicated.
    DuplicateTool,
    /// A tool had no exact configured policy.
    UnknownTool,
    /// A tool schema was malformed.
    MalformedSchema,
    /// A current tool schema changed within one generation.
    SchemaDrift,
    /// Existing MCP mapping or Animus tool validation rejected the descriptor.
    MappingRejected,
}
