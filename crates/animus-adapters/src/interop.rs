//! Data-only ACP/MCP interop descriptors, pass-through planning, and safe receipts.

const ID_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("interop_id");
const TRANSPORT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_transport");
const COMMAND_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_command_label");
const ENDPOINT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_endpoint_label");
const TOOL_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("interop_tool");
const RESOURCE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_resource");
const PROMPT_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("mcp_prompt");
const SCHEMA_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("mcp_schema");
const CONTRACT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_result_contract");
const TIMEOUT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("interop_timeout");
const ENV_KEY_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("env_key");
const PROVENANCE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("interop_provenance_hash");
const PROFILE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("acp_model_profile");
const PERMISSION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("acp_permission_mode");
const SESSION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("acp_session_correlation");
const POLICY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("acp_passthrough_policy");
const LIMITATION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("acp_limitation");
const OPERATION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("interop_operation");
const STATUS_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("receipt_status");
const REDACTION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("receipt_redaction");

const TRANSPORT_STDIO: &str = "stdio";
const TRANSPORT_STREAMABLE_HTTP: &str = "streamable_http";
const PERMISSION_READ_ONLY: &str = "read_only";
const PERMISSION_ASK: &str = "ask";
const PERMISSION_UNRESTRICTED: &str = "unrestricted";
const SESSION_NEW: &str = "new_session";
const SESSION_JOIN_BY_LABEL: &str = "join_by_label";
const POLICY_DENY: &str = "deny";
const POLICY_READ_ONLY: &str = "read_only";
const POLICY_INTERSECTION: &str = "intersection";
const OP_DISCOVERY: &str = "discovery";
const OP_SESSION_START: &str = "session_start";
const OP_PASS_THROUGH: &str = "pass_through";
const OP_DENIAL: &str = "denial";
const OP_TIMEOUT: &str = "timeout";
const OP_UNAVAILABLE_BINARY: &str = "unavailable_binary";
const OP_MALFORMED_SCHEMA: &str = "malformed_schema";
const OP_NETWORK_POLICY: &str = "network_policy";
const HASH_SEPARATOR: &str = "|";
const BLAKE3_HEX_CHARS: usize = 64;
const ASCII_HEX_RADIX: u32 = 16;
const ENV_VALUE_SEPARATOR: char = '=';
const RAW_URL_MARKER: &str = "://";
const LABEL_SPACE: char = ' ';
const MIN_COUNT: u32 = 0;

/// Descriptor-side effects that interop validation explicitly does not perform.
pub const PROHIBITED_INTEROP_EFFECTS: &[&str] = &[
    "stdio process launch",
    "HTTP probing",
    "ambient binary discovery",
    "credential lookup",
    "external agent contact",
    "tool execution",
];

type Text = alloc::string::String;
type Collection<T> = alloc::vec::Vec<T>;

/// Error raised while validating ACP/MCP interop contracts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InteropError {
    /// Generic adapter contract validation failed.
    Contract(crate::AdapterContractError),
    /// An enum-like interop field contained an unsupported value.
    UnsupportedValue {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported normalized value.
        value: Text,
    },
    /// A required collection was empty.
    EmptyCollection {
        /// Boundary collection field name.
        field: &'static str,
    },
    /// A named collection contained a duplicate value.
    DuplicateValue {
        /// Boundary collection field name.
        field: &'static str,
        /// Duplicated normalized value.
        value: Text,
    },
    /// A descriptor provided an environment value instead of a key name.
    EnvironmentValueLeak,
    /// A command or endpoint included raw launch/network material instead of a label.
    UnsafeRuntimeLabel,
    /// A descriptor omitted required timeout metadata.
    MissingTimeout,
    /// An ACP descriptor attempted an unsafe permission-mode downgrade.
    UnsafePermissionMapping,
    /// An ACP descriptor omitted session correlation policy.
    MissingSessionCorrelation,
    /// A BLAKE3 hash field was malformed.
    InvalidHash {
        /// Boundary field name.
        field: &'static str,
    },
    /// Receipt text contained raw interop material.
    ReceiptLeak,
}

impl From<crate::AdapterContractError> for InteropError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

/// MCP transport kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum McpTransportKind {
    /// Stdio transport, represented by a host-owned command label.
    Stdio,
    /// Streamable HTTP transport, represented by a host-owned endpoint label.
    StreamableHttp,
}

impl McpTransportKind {
    /// Parses a stable MCP transport label.
    ///
    /// # Errors
    ///
    /// Returns [`InteropError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, InteropError> {
        parse_value(TRANSPORT_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            TRANSPORT_STDIO => Some(Self::Stdio),
            TRANSPORT_STREAMABLE_HTTP => Some(Self::StreamableHttp),
            _ => None,
        }
    }
}

/// ACP permission mode projection.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpPermissionMode {
    /// Read-only external mode.
    ReadOnly,
    /// Ask-before-action external mode.
    Ask,
    /// Unrestricted external mode, admitted only by explicit host policy.
    Unrestricted,
}

impl AcpPermissionMode {
    /// Parses a stable ACP permission mode label.
    ///
    /// # Errors
    ///
    /// Returns [`InteropError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, InteropError> {
        parse_value(PERMISSION_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            PERMISSION_READ_ONLY => Some(Self::ReadOnly),
            PERMISSION_ASK => Some(Self::Ask),
            PERMISSION_UNRESTRICTED => Some(Self::Unrestricted),
            _ => None,
        }
    }
}

/// ACP session correlation policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpSessionCorrelation {
    /// Always create a new external session.
    NewSession,
    /// Join or correlate by an explicit redacted session label.
    JoinByLabel,
}

impl AcpSessionCorrelation {
    /// Parses a stable ACP session correlation label.
    ///
    /// # Errors
    ///
    /// Returns [`InteropError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, InteropError> {
        parse_value(SESSION_FIELD, value, Self::from_normalized).map_err(|error| match error {
            InteropError::Contract(crate::AdapterContractError::EmptyField { .. }) => {
                InteropError::MissingSessionCorrelation
            }
            other => other,
        })
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            SESSION_NEW => Some(Self::NewSession),
            SESSION_JOIN_BY_LABEL => Some(Self::JoinByLabel),
            _ => None,
        }
    }
}

/// ACP tool pass-through policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AcpToolPassThroughPolicy {
    /// Do not pass tools through.
    Deny,
    /// Allow read-only pass-through after intersection.
    ReadOnly,
    /// Resolve pass-through by full capability intersection.
    Intersection,
}

impl AcpToolPassThroughPolicy {
    /// Parses a stable ACP tool pass-through policy label.
    ///
    /// # Errors
    ///
    /// Returns [`InteropError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, InteropError> {
        parse_value(POLICY_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            POLICY_DENY => Some(Self::Deny),
            POLICY_READ_ONLY => Some(Self::ReadOnly),
            POLICY_INTERSECTION => Some(Self::Intersection),
            _ => None,
        }
    }
}

/// Untrusted MCP descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct McpDescriptorDraft<'a> {
    /// Descriptor identity.
    pub identity: &'a str,
    /// MCP transport label.
    pub transport: &'a str,
    /// Host-owned command label for stdio transports.
    pub command_label: Option<&'a str>,
    /// Host-owned endpoint label for HTTP transports.
    pub endpoint_label: Option<&'a str>,
    /// Tool identities declared by the MCP server.
    pub tools: &'a [&'a str],
    /// Resource identities declared by the MCP server.
    pub resources: &'a [&'a str],
    /// Prompt identities declared by the MCP server.
    pub prompts: &'a [&'a str],
    /// Schema identity labels.
    pub schema_ids: &'a [&'a str],
    /// Result contract labels.
    pub result_contracts: &'a [&'a str],
    /// Timeout class label.
    pub timeout_class: &'a str,
    /// Environment key names required by the extension.
    pub environment_keys: &'a [&'a str],
    /// Available tool ceilings.
    pub available_tool_ceilings: &'a [&'a str],
    /// BLAKE3 provenance hash.
    pub provenance_hash: &'a str,
}

/// Validated MCP capability descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct McpDescriptor {
    identity: Text,
    transport: McpTransportKind,
    command_label: Option<Text>,
    endpoint_label: Option<Text>,
    tools: Collection<Text>,
    timeout_class: Text,
    environment_keys: Collection<Text>,
    provenance_hash: Text,
}

impl McpDescriptor {
    /// Validates an MCP descriptor without starting a server or probing network.
    ///
    /// # Errors
    ///
    /// Returns [`InteropError`] when fields are malformed, transport labels are
    /// unsupported, timeout is missing, tools collide, env values leak, or raw
    /// command/endpoint material is present.
    pub fn validate(draft: &McpDescriptorDraft<'_>) -> Result<Self, InteropError> {
        let identity = required_text(ID_FIELD, draft.identity)?;
        let transport = McpTransportKind::parse(draft.transport)?;
        let command_label = optional_runtime_label(COMMAND_FIELD, draft.command_label)?;
        let endpoint_label = optional_runtime_label(ENDPOINT_FIELD, draft.endpoint_label)?;
        validate_transport_labels(transport, command_label.as_ref(), endpoint_label.as_ref())?;
        let tools = text_collection(TOOL_FIELD, draft.tools, true)?;
        text_collection(RESOURCE_FIELD, draft.resources, false)?;
        text_collection(PROMPT_FIELD, draft.prompts, false)?;
        text_collection(SCHEMA_FIELD, draft.schema_ids, true)?;
        text_collection(CONTRACT_FIELD, draft.result_contracts, true)?;
        let timeout_class = required_text(TIMEOUT_FIELD, draft.timeout_class)
            .map_err(|error| map_empty_to_timeout(error))?;
        let environment_keys = validate_environment_keys(draft.environment_keys)?;
        text_collection(TOOL_FIELD, draft.available_tool_ceilings, true)?;
        let provenance_hash = required_text(PROVENANCE_FIELD, draft.provenance_hash)?;
        validate_hash(PROVENANCE_FIELD.as_str(), provenance_hash.as_str())?;
        Ok(Self {
            identity,
            transport,
            command_label,
            endpoint_label,
            tools,
            timeout_class,
            environment_keys,
            provenance_hash,
        })
    }

    /// Returns descriptor identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns transport kind.
    #[must_use]
    pub const fn transport(&self) -> McpTransportKind {
        self.transport
    }

    /// Returns command label, when present.
    #[must_use]
    pub fn command_label(&self) -> Option<&str> {
        self.command_label.as_deref()
    }

    /// Returns endpoint label, when present.
    #[must_use]
    pub fn endpoint_label(&self) -> Option<&str> {
        self.endpoint_label.as_deref()
    }

    /// Returns declared tool identities.
    #[must_use]
    pub fn tools(&self) -> &[Text] {
        self.tools.as_slice()
    }

    /// Returns timeout class.
    #[must_use]
    pub fn timeout_class(&self) -> &str {
        self.timeout_class.as_str()
    }

    /// Returns environment key names.
    #[must_use]
    pub fn environment_keys(&self) -> &[Text] {
        self.environment_keys.as_slice()
    }

    /// Returns provenance hash.
    #[must_use]
    pub fn provenance_hash(&self) -> &str {
        self.provenance_hash.as_str()
    }
}

/// Untrusted ACP provider descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcpDescriptorDraft<'a> {
    /// External agent identity.
    pub identity: &'a str,
    /// Model or profile preference label.
    pub model_profile: &'a str,
    /// Animus permission mode label.
    pub animus_permission_mode: &'a str,
    /// External ACP permission mode label.
    pub external_permission_mode: &'a str,
    /// Session correlation policy label.
    pub session_correlation: &'a str,
    /// Known limitation labels.
    pub known_limitations: &'a [&'a str],
    /// Tool pass-through policy label.
    pub tool_pass_through_policy: &'a str,
    /// Whether host policy admits an unrestricted external mapping.
    pub host_admits_unrestricted: bool,
}

/// Validated ACP provider descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcpDescriptor {
    identity: Text,
    model_profile: Text,
    animus_permission_mode: AcpPermissionMode,
    external_permission_mode: AcpPermissionMode,
    session_correlation: AcpSessionCorrelation,
    known_limitations: Collection<Text>,
    tool_pass_through_policy: AcpToolPassThroughPolicy,
}

impl AcpDescriptor {
    /// Validates ACP provider metadata without contacting an external agent.
    ///
    /// # Errors
    ///
    /// Returns [`InteropError`] when labels are malformed, session correlation is
    /// missing, or permission mapping would downgrade guarded Animus behavior.
    pub fn validate(draft: &AcpDescriptorDraft<'_>) -> Result<Self, InteropError> {
        let identity = required_text(ID_FIELD, draft.identity)?;
        let model_profile = required_text(PROFILE_FIELD, draft.model_profile)?;
        let animus_permission_mode = AcpPermissionMode::parse(draft.animus_permission_mode)?;
        let external_permission_mode = AcpPermissionMode::parse(draft.external_permission_mode)?;
        if is_unsafe_permission_mapping(
            animus_permission_mode,
            external_permission_mode,
            draft.host_admits_unrestricted,
        ) {
            return Err(InteropError::UnsafePermissionMapping);
        }
        let session_correlation = AcpSessionCorrelation::parse(draft.session_correlation)?;
        let known_limitations = text_collection(LIMITATION_FIELD, draft.known_limitations, false)?;
        let tool_pass_through_policy =
            AcpToolPassThroughPolicy::parse(draft.tool_pass_through_policy)?;
        Ok(Self {
            identity,
            model_profile,
            animus_permission_mode,
            external_permission_mode,
            session_correlation,
            known_limitations,
            tool_pass_through_policy,
        })
    }

    /// Returns external agent identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns model/profile preference.
    #[must_use]
    pub fn model_profile(&self) -> &str {
        self.model_profile.as_str()
    }

    /// Returns Animus permission mode.
    #[must_use]
    pub const fn animus_permission_mode(&self) -> AcpPermissionMode {
        self.animus_permission_mode
    }

    /// Returns external ACP permission mode.
    #[must_use]
    pub const fn external_permission_mode(&self) -> AcpPermissionMode {
        self.external_permission_mode
    }

    /// Returns session correlation policy.
    #[must_use]
    pub const fn session_correlation(&self) -> AcpSessionCorrelation {
        self.session_correlation
    }

    /// Returns known limitation labels.
    #[must_use]
    pub fn known_limitations(&self) -> &[Text] {
        self.known_limitations.as_slice()
    }

    /// Returns tool pass-through policy.
    #[must_use]
    pub const fn tool_pass_through_policy(&self) -> AcpToolPassThroughPolicy {
        self.tool_pass_through_policy
    }
}

/// Tool pass-through intersection input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolPassThroughInput<'a> {
    /// Tool labels requested by the bridge.
    pub requested_tools: &'a [&'a str],
    /// Animus capability ceiling tool labels.
    pub animus_capability_ceilings: &'a [&'a str],
    /// Tool labels admitted by active permission mode.
    pub permission_mode_tools: &'a [&'a str],
    /// Tool labels admitted by host policy.
    pub host_admission: &'a [&'a str],
    /// Tool labels admitted by external agent mode.
    pub external_agent_mode: &'a [&'a str],
    /// Tool labels declared as available by the descriptor.
    pub available_tools: &'a [&'a str],
}

/// Tool pass-through resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolPassThroughResolution {
    allowed_tools: Collection<Text>,
    denied_tools: Collection<Text>,
}

impl ToolPassThroughResolution {
    /// Returns allowed tool labels.
    #[must_use]
    pub fn allowed_tools(&self) -> &[Text] {
        self.allowed_tools.as_slice()
    }

    /// Returns denied tool labels.
    #[must_use]
    pub fn denied_tools(&self) -> &[Text] {
        self.denied_tools.as_slice()
    }
}

/// Resolves bridge tool pass-through by full capability intersection.
///
/// # Errors
///
/// Returns [`InteropError`] when tool labels are malformed or duplicated.
pub fn resolve_tool_pass_through(
    input: ToolPassThroughInput<'_>,
) -> Result<ToolPassThroughResolution, InteropError> {
    let requested = text_collection(TOOL_FIELD, input.requested_tools, true)?;
    let animus = text_collection(TOOL_FIELD, input.animus_capability_ceilings, true)?;
    let permission = text_collection(TOOL_FIELD, input.permission_mode_tools, true)?;
    let host = text_collection(TOOL_FIELD, input.host_admission, true)?;
    let external = text_collection(TOOL_FIELD, input.external_agent_mode, true)?;
    let available = text_collection(TOOL_FIELD, input.available_tools, true)?;
    let mut allowed_tools = Collection::new();
    let mut denied_tools = Collection::new();
    for tool in requested {
        if contains_everywhere(
            tool.as_str(),
            &animus,
            &permission,
            &host,
            &external,
            &available,
        ) {
            allowed_tools.push(tool);
        } else {
            denied_tools.push(tool);
        }
    }
    Ok(ToolPassThroughResolution {
        allowed_tools,
        denied_tools,
    })
}

/// Interop receipt operation kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InteropOperationKind {
    /// Discovery receipt.
    Discovery,
    /// Session start receipt.
    SessionStart,
    /// Tool pass-through receipt.
    PassThrough,
    /// Denial receipt.
    Denial,
    /// Timeout receipt.
    Timeout,
    /// Unavailable binary receipt.
    UnavailableBinary,
    /// Malformed schema receipt.
    MalformedSchema,
    /// Network policy receipt.
    NetworkPolicy,
}

impl InteropOperationKind {
    /// Parses a stable receipt operation label.
    ///
    /// # Errors
    ///
    /// Returns [`InteropError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, InteropError> {
        parse_value(OPERATION_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            OP_DISCOVERY => Some(Self::Discovery),
            OP_SESSION_START => Some(Self::SessionStart),
            OP_PASS_THROUGH => Some(Self::PassThrough),
            OP_DENIAL => Some(Self::Denial),
            OP_TIMEOUT => Some(Self::Timeout),
            OP_UNAVAILABLE_BINARY => Some(Self::UnavailableBinary),
            OP_MALFORMED_SCHEMA => Some(Self::MalformedSchema),
            OP_NETWORK_POLICY => Some(Self::NetworkPolicy),
            _ => None,
        }
    }

    fn as_str(self) -> &'static str {
        match self {
            Self::Discovery => OP_DISCOVERY,
            Self::SessionStart => OP_SESSION_START,
            Self::PassThrough => OP_PASS_THROUGH,
            Self::Denial => OP_DENIAL,
            Self::Timeout => OP_TIMEOUT,
            Self::UnavailableBinary => OP_UNAVAILABLE_BINARY,
            Self::MalformedSchema => OP_MALFORMED_SCHEMA,
            Self::NetworkPolicy => OP_NETWORK_POLICY,
        }
    }
}

/// Untrusted safe interop receipt metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InteropReceiptDraft<'a> {
    /// Operation kind label.
    pub operation: &'a str,
    /// Status label.
    pub status: &'a str,
    /// Bridge identity hash.
    pub bridge_hash: &'a str,
    /// Command label hash.
    pub command_hash: Option<&'a str>,
    /// Endpoint label hash.
    pub endpoint_hash: Option<&'a str>,
    /// External session hash.
    pub external_session_hash: Option<&'a str>,
    /// Environment key count.
    pub env_key_count: u32,
    /// Argument count.
    pub argument_count: u32,
    /// Redaction markers.
    pub redactions: &'a [&'a str],
}

/// Safe interop receipt with hashes and bounded counts only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InteropReceipt {
    operation: InteropOperationKind,
    status: Text,
    identity_hash: Text,
    env_key_count: u32,
    argument_count: u32,
    redactions: Collection<Text>,
}

impl InteropReceipt {
    /// Builds a safe interop receipt.
    ///
    /// # Errors
    ///
    /// Returns [`InteropError`] when labels are malformed, hashes are invalid, or
    /// redactions are empty.
    pub fn new(draft: &InteropReceiptDraft<'_>) -> Result<Self, InteropError> {
        let operation = InteropOperationKind::parse(draft.operation)?;
        let status = required_text(STATUS_FIELD, draft.status)?;
        validate_hash(ID_FIELD.as_str(), draft.bridge_hash)?;
        if let Some(command_hash) = draft.command_hash {
            validate_hash(COMMAND_FIELD.as_str(), command_hash)?;
        }
        if let Some(endpoint_hash) = draft.endpoint_hash {
            validate_hash(ENDPOINT_FIELD.as_str(), endpoint_hash)?;
        }
        if let Some(external_session_hash) = draft.external_session_hash {
            validate_hash(SESSION_FIELD.as_str(), external_session_hash)?;
        }
        let redactions = text_collection(REDACTION_FIELD, draft.redactions, true)?;
        let env_count = draft.env_key_count.saturating_add(MIN_COUNT);
        let arg_count = draft.argument_count.saturating_add(MIN_COUNT);
        let identity_hash = hash_parts(&[
            operation.as_str(),
            status.as_str(),
            draft.bridge_hash,
            &alloc::format!("{env_count}"),
            &alloc::format!("{arg_count}"),
        ]);
        Ok(Self {
            operation,
            status,
            identity_hash,
            env_key_count: env_count,
            argument_count: arg_count,
            redactions,
        })
    }

    /// Returns operation kind.
    #[must_use]
    pub const fn operation(&self) -> InteropOperationKind {
        self.operation
    }

    /// Returns status label.
    #[must_use]
    pub fn status(&self) -> &str {
        self.status.as_str()
    }

    /// Returns receipt identity hash.
    #[must_use]
    pub fn identity_hash(&self) -> &str {
        self.identity_hash.as_str()
    }

    /// Returns environment key count.
    #[must_use]
    pub const fn env_key_count(&self) -> u32 {
        self.env_key_count
    }

    /// Returns argument count.
    #[must_use]
    pub const fn argument_count(&self) -> u32 {
        self.argument_count
    }

    /// Returns redaction markers.
    #[must_use]
    pub fn redactions(&self) -> &[Text] {
        self.redactions.as_slice()
    }
}

/// Validates rendered receipt text for raw interop material.
///
/// # Errors
///
/// Returns [`InteropError::ReceiptLeak`] when raw command, endpoint, argument, or
/// credential-looking material appears.
pub fn validate_interop_receipt_text(text: &str) -> Result<(), InteropError> {
    let lowered = text.to_ascii_lowercase();
    if lowered.contains("http://")
        || lowered.contains("https://")
        || lowered.contains("api_key")
        || lowered.contains("args=")
        || lowered.contains("session=")
    {
        return Err(InteropError::ReceiptLeak);
    }
    Ok(())
}

fn validate_transport_labels(
    transport: McpTransportKind,
    command_label: Option<&Text>,
    endpoint_label: Option<&Text>,
) -> Result<(), InteropError> {
    match transport {
        McpTransportKind::Stdio if command_label.is_none() => Err(InteropError::UnsafeRuntimeLabel),
        McpTransportKind::StreamableHttp if endpoint_label.is_none() => {
            Err(InteropError::UnsafeRuntimeLabel)
        }
        _ => Ok(()),
    }
}

fn validate_environment_keys(values: &[&str]) -> Result<Collection<Text>, InteropError> {
    let keys = text_collection(ENV_KEY_FIELD, values, false)?;
    if keys
        .iter()
        .any(|key| key.contains(ENV_VALUE_SEPARATOR) || key.contains(RAW_URL_MARKER))
    {
        return Err(InteropError::EnvironmentValueLeak);
    }
    Ok(keys)
}

fn optional_runtime_label(
    field_name: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<Option<Text>, InteropError> {
    let Some(text) = value else {
        return Ok(None);
    };
    let normalized = required_text(field_name, text)?;
    if normalized.contains(RAW_URL_MARKER) || normalized.contains(LABEL_SPACE) {
        return Err(InteropError::UnsafeRuntimeLabel);
    }
    Ok(Some(normalized))
}

fn is_unsafe_permission_mapping(
    animus: AcpPermissionMode,
    external: AcpPermissionMode,
    host_admits_unrestricted: bool,
) -> bool {
    external == AcpPermissionMode::Unrestricted
        && animus != AcpPermissionMode::Unrestricted
        && !host_admits_unrestricted
}

fn contains_everywhere(
    tool: &str,
    animus: &[Text],
    permission: &[Text],
    host: &[Text],
    external: &[Text],
    available: &[Text],
) -> bool {
    contains_text(animus, tool)
        && contains_text(permission, tool)
        && contains_text(host, tool)
        && contains_text(external, tool)
        && contains_text(available, tool)
}

fn map_empty_to_timeout(error: InteropError) -> InteropError {
    match error {
        InteropError::Contract(crate::AdapterContractError::EmptyField { .. }) => {
            InteropError::MissingTimeout
        }
        other => other,
    }
}

fn parse_value<T>(
    field_name: crate::validation::FieldName,
    value: &str,
    parser: impl FnOnce(&str) -> Option<T>,
) -> Result<T, InteropError> {
    let normalized = crate::validation::normalized_adapter_field(field_name, value)?;
    parser(normalized.as_str()).ok_or(InteropError::UnsupportedValue {
        field: field_name.as_str(),
        value: normalized,
    })
}

fn required_text(
    field_name: crate::validation::FieldName,
    value: &str,
) -> Result<Text, InteropError> {
    crate::validation::normalized_adapter_field(field_name, value).map_err(InteropError::from)
}

fn text_collection(
    field_name: crate::validation::FieldName,
    values: &[&str],
    required: bool,
) -> Result<Collection<Text>, InteropError> {
    if required && values.is_empty() {
        return Err(InteropError::EmptyCollection {
            field: field_name.as_str(),
        });
    }
    let mut normalized: Collection<Text> = Collection::with_capacity(values.len());
    for value in values {
        let text = required_text(field_name, value)?;
        if normalized.iter().any(|existing| existing == &text) {
            return Err(InteropError::DuplicateValue {
                field: field_name.as_str(),
                value: text,
            });
        }
        normalized.push(text);
    }
    Ok(normalized)
}

fn contains_text(values: &[Text], needle: &str) -> bool {
    values.iter().any(|value| value.as_str() == needle)
}

fn validate_hash(field: &'static str, value: &str) -> Result<(), InteropError> {
    let hash_len = value.chars().count();
    if hash_len != BLAKE3_HEX_CHARS {
        return Err(InteropError::InvalidHash { field });
    }
    if value
        .chars()
        .any(|character| character.to_digit(ASCII_HEX_RADIX).is_none() || character.is_uppercase())
    {
        return Err(InteropError::InvalidHash { field });
    }
    Ok(())
}

fn hash_parts(parts: &[&str]) -> Text {
    let mut text = Text::new();
    for (index, part) in parts.iter().enumerate() {
        if index > 0 {
            text.push_str(HASH_SEPARATOR);
        }
        text.push_str(part);
    }
    alloc::format!("{}", blake3::hash(text.as_bytes()).to_hex())
}

#[cfg(test)]
mod tests {
    use super::*;

    const MCP_ID: &str = "mcp-review-tools";
    const ACP_ID: &str = "acp-coder";
    const COMMAND_LABEL: &str = "review-server";
    const ENDPOINT_LABEL: &str = "review-http";
    const TOOL_READ: &str = "tool.read";
    const TOOL_WRITE: &str = "tool.write";
    const RESOURCE: &str = "repo-summary";
    const PROMPT: &str = "review-prompt";
    const SCHEMA: &str = "schema.review";
    const CONTRACT: &str = "json-result";
    const TIMEOUT: &str = "standard";
    const ENV_KEY: &str = "MCP_TOKEN";
    const MODEL_PROFILE: &str = "standard-reasoning";
    const LIMITATION: &str = "no-background-writes";
    const HASH: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
    const HASH_CHARS: usize = 64;
    const FIRST_INDEX: usize = 0;
    const ENV_COUNT: u32 = 1;
    const ARG_COUNT: u32 = 2;

    fn mcp_stdio() -> McpDescriptorDraft<'static> {
        McpDescriptorDraft {
            identity: MCP_ID,
            transport: TRANSPORT_STDIO,
            command_label: Some(COMMAND_LABEL),
            endpoint_label: None,
            tools: &[TOOL_READ],
            resources: &[RESOURCE],
            prompts: &[PROMPT],
            schema_ids: &[SCHEMA],
            result_contracts: &[CONTRACT],
            timeout_class: TIMEOUT,
            environment_keys: &[ENV_KEY],
            available_tool_ceilings: &[TOOL_READ],
            provenance_hash: HASH,
        }
    }

    fn acp_descriptor() -> AcpDescriptorDraft<'static> {
        AcpDescriptorDraft {
            identity: ACP_ID,
            model_profile: MODEL_PROFILE,
            animus_permission_mode: PERMISSION_ASK,
            external_permission_mode: PERMISSION_ASK,
            session_correlation: SESSION_JOIN_BY_LABEL,
            known_limitations: &[LIMITATION],
            tool_pass_through_policy: POLICY_INTERSECTION,
            host_admits_unrestricted: false,
        }
    }

    #[test]
    fn mcp_descriptors_accept_stdio_and_streamable_http_labels() {
        let stdio = McpDescriptor::validate(&mcp_stdio()).expect("stdio descriptor should parse");
        let http = McpDescriptor::validate(&McpDescriptorDraft {
            transport: TRANSPORT_STREAMABLE_HTTP,
            command_label: None,
            endpoint_label: Some(ENDPOINT_LABEL),
            ..mcp_stdio()
        })
        .expect("http descriptor should parse");

        assert_eq!(stdio.transport(), McpTransportKind::Stdio);
        assert_eq!(stdio.command_label(), Some(COMMAND_LABEL));
        assert_eq!(http.transport(), McpTransportKind::StreamableHttp);
        assert_eq!(http.endpoint_label(), Some(ENDPOINT_LABEL));
        assert_eq!(stdio.environment_keys()[FIRST_INDEX], ENV_KEY);
    }

    #[test]
    fn mcp_descriptors_reject_unsafe_runtime_claims() {
        assert!(matches!(
            McpDescriptor::validate(&McpDescriptorDraft {
                transport: "websocket",
                ..mcp_stdio()
            }),
            Err(InteropError::UnsupportedValue { .. })
        ));
        assert_eq!(
            McpDescriptor::validate(&McpDescriptorDraft {
                tools: &[TOOL_READ, TOOL_READ],
                ..mcp_stdio()
            }),
            Err(InteropError::DuplicateValue {
                field: TOOL_FIELD.as_str(),
                value: Text::from(TOOL_READ),
            })
        );
        assert_eq!(
            McpDescriptor::validate(&McpDescriptorDraft {
                timeout_class: "",
                ..mcp_stdio()
            }),
            Err(InteropError::MissingTimeout)
        );
        assert_eq!(
            McpDescriptor::validate(&McpDescriptorDraft {
                environment_keys: &["MCP_TOKEN=secret"],
                ..mcp_stdio()
            }),
            Err(InteropError::EnvironmentValueLeak)
        );
        assert_eq!(
            McpDescriptor::validate(&McpDescriptorDraft {
                transport: TRANSPORT_STREAMABLE_HTTP,
                command_label: None,
                endpoint_label: Some("https://example.test"),
                ..mcp_stdio()
            }),
            Err(InteropError::UnsafeRuntimeLabel)
        );
    }

    #[test]
    fn acp_descriptors_reject_unsafe_permission_downgrades() {
        let descriptor = AcpDescriptor::validate(&acp_descriptor()).expect("ACP descriptor parses");

        assert_eq!(descriptor.identity(), ACP_ID);
        assert_eq!(
            descriptor.session_correlation(),
            AcpSessionCorrelation::JoinByLabel
        );
        assert_eq!(
            AcpDescriptor::validate(&AcpDescriptorDraft {
                external_permission_mode: PERMISSION_UNRESTRICTED,
                ..acp_descriptor()
            }),
            Err(InteropError::UnsafePermissionMapping)
        );
        assert_eq!(
            AcpDescriptor::validate(&AcpDescriptorDraft {
                session_correlation: "",
                ..acp_descriptor()
            }),
            Err(InteropError::MissingSessionCorrelation)
        );
    }

    #[test]
    fn descriptor_validation_is_data_only() {
        let rendered = alloc::format!("{:?}", PROHIBITED_INTEROP_EFFECTS);
        let descriptor = McpDescriptor::validate(&mcp_stdio()).expect("descriptor parses");

        assert_eq!(descriptor.identity(), MCP_ID);
        assert!(rendered.contains("stdio process launch"));
        assert!(rendered.contains("credential lookup"));
    }

    #[test]
    fn tool_pass_through_is_intersection_only() {
        let resolution = resolve_tool_pass_through(ToolPassThroughInput {
            requested_tools: &[TOOL_READ, TOOL_WRITE],
            animus_capability_ceilings: &[TOOL_READ, TOOL_WRITE],
            permission_mode_tools: &[TOOL_READ],
            host_admission: &[TOOL_READ],
            external_agent_mode: &[TOOL_READ, TOOL_WRITE],
            available_tools: &[TOOL_READ],
        })
        .expect("tools should resolve");

        assert_eq!(resolution.allowed_tools(), &[Text::from(TOOL_READ)]);
        assert_eq!(resolution.denied_tools(), &[Text::from(TOOL_WRITE)]);
    }

    #[test]
    fn interop_receipts_are_hash_and_count_only() {
        let receipt = InteropReceipt::new(&InteropReceiptDraft {
            operation: OP_PASS_THROUGH,
            status: "denied",
            bridge_hash: HASH,
            command_hash: Some(HASH),
            endpoint_hash: Some(HASH),
            external_session_hash: Some(HASH),
            env_key_count: ENV_COUNT,
            argument_count: ARG_COUNT,
            redactions: &["command_hashed", "endpoint_hashed", "session_hashed"],
        })
        .expect("receipt should parse");

        assert_eq!(receipt.operation(), InteropOperationKind::PassThrough);
        assert_eq!(receipt.env_key_count(), ENV_COUNT);
        assert_eq!(receipt.argument_count(), ARG_COUNT);
        assert_eq!(receipt.identity_hash().len(), HASH_CHARS);
        assert_eq!(
            validate_interop_receipt_text("bridge hash redactions bounded"),
            Ok(())
        );
        assert_eq!(
            validate_interop_receipt_text("endpoint=https://example.test args=raw api_key=x"),
            Err(InteropError::ReceiptLeak)
        );
    }
}
