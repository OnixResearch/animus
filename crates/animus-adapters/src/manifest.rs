use alloc::string::String;
use alloc::vec::Vec;

const IDENTITY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("manifest_identity");
const RUNTIME_KIND_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("runtime_kind");
const TOOL_NAME_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("declared_tool_name");
const TOOL_DESCRIPTION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("declared_tool_description");
const CAPABILITY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("capability");
const LAUNCH_COMMAND_LABEL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("launch_command_label");
const LAUNCH_ARGUMENT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("launch_argument");
const WORKING_DIRECTORY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("working_directory_mode");
const ENVIRONMENT_ALLOWLIST_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("environment_allowlist_name");
const WRITABLE_ROOT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("writable_root");
const SANDBOX_MODE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("sandbox_mode");
const NETWORK_POLICY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("network_policy");

const RUNTIME_KIND_HOST: &str = "host";
const RUNTIME_KIND_STDIO: &str = "stdio";
const RUNTIME_KIND_WASM: &str = "wasm";
const WORKING_DIRECTORY_PROJECT_ROOT: &str = "project-root";
const WORKING_DIRECTORY_MANIFEST_DIRECTORY: &str = "manifest-directory";
const SANDBOX_MODE_UNCONFINED: &str = "unconfined";
const SANDBOX_MODE_ISOLATED: &str = "isolated";
const NETWORK_POLICY_DENIED: &str = "denied";
const NETWORK_POLICY_LOOPBACK: &str = "loopback";
const NETWORK_POLICY_OUTBOUND: &str = "outbound";
const ENVIRONMENT_VALUE_SEPARATOR: char = '=';
const UNIX_PATH_SEPARATOR: char = '/';
const WINDOWS_PATH_SEPARATOR: char = '\\';
const HOME_PATH_PREFIX: char = '~';
const PATH_ESCAPE_SEGMENT: &str = "..";
const WINDOWS_DRIVE_SEPARATOR: u8 = b':';
const WINDOWS_DRIVE_SEPARATOR_INDEX: usize = 1;

/// Descriptor-side effects that manifest validation explicitly does not perform.
pub const PROHIBITED_PLUGIN_MANIFEST_EFFECTS: &[&str] = &[
    "stdio process launch",
    "wasm module loading",
    "environment inheritance",
    "secret reads",
    "network connections",
    "tool execution",
];

/// Untrusted manifest-shaped data before descriptor validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PluginManifestDraft<'a> {
    /// Optional descriptor identity, required by validation.
    pub identity: Option<&'a str>,
    /// Optional runtime-kind label, required by validation.
    pub runtime_kind: Option<&'a str>,
    /// Declared tool descriptors owned by this manifest.
    pub declared_tools: &'a [ToolDescriptorDraft<'a>],
    /// Capability labels requested by this manifest.
    pub capabilities: &'a [&'a str],
    /// Launch-policy metadata. Validation treats this as data only.
    pub launch_policy: LaunchPolicyDraft<'a>,
    /// Sandbox and network claims. Validation does not enforce a sandbox.
    pub sandbox: SandboxRequirementsDraft<'a>,
    /// Redaction claims for future host-owned logging.
    pub redaction: RedactionPolicyDraft,
}

/// Untrusted manifest-shaped tool declaration data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolDescriptorDraft<'a> {
    /// Optional tool name, required by validation.
    pub name: Option<&'a str>,
    /// Optional human-facing descriptor text.
    pub description: Option<&'a str>,
}

/// Untrusted launch metadata from a descriptor.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct LaunchPolicyDraft<'a> {
    /// Optional command or artifact label. This is never executed by validation.
    pub command_label: Option<&'a str>,
    /// Optional data-only argument labels.
    pub arguments: &'a [&'a str],
    /// Optional working-directory mode label.
    pub working_directory: Option<&'a str>,
    /// Environment variable names that a future host may choose to expose.
    pub environment_allowlist: &'a [&'a str],
    /// Project-relative writable-root descriptors.
    pub writable_roots: &'a [&'a str],
}

/// Untrusted sandbox and network claims from a descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SandboxRequirementsDraft<'a> {
    /// Optional sandbox mode, required by validation.
    pub mode: Option<&'a str>,
    /// Optional network policy, required by validation.
    pub network: Option<&'a str>,
}

/// Redaction claims from a descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RedactionPolicyDraft {
    /// Whether host logs should redact transcript payloads.
    pub redact_transcripts: bool,
    /// Whether host logs should redact secret-looking payloads.
    pub redact_secrets: bool,
}

/// Validated descriptor manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PluginManifest {
    identity: String,
    runtime_kind: RuntimeKind,
    declared_tools: Vec<ToolDescriptor>,
    capabilities: Vec<String>,
    launch_policy: LaunchPolicy,
    sandbox: SandboxRequirements,
    redaction: RedactionPolicy,
}

impl PluginManifest {
    /// Validates untrusted manifest-shaped data as a descriptor only.
    ///
    /// # Errors
    ///
    /// Returns [`PluginManifestError`] when required fields are missing, field
    /// text is invalid, runtime/sandbox/network labels are unknown, tool names
    /// collide, or path/network declarations are unsafe.
    pub fn validate(
        draft: &PluginManifestDraft<'_>,
        built_in_tool_names: &[&str],
        existing_catalog_tool_names: &[&str],
    ) -> Result<Self, PluginManifestError> {
        let identity = required_text(IDENTITY_FIELD, draft.identity)?;
        let runtime_text = required_text(RUNTIME_KIND_FIELD, draft.runtime_kind)?;
        let runtime_kind = parse_runtime_kind(&runtime_text)?;
        let declared_tools = validate_tools(
            draft.declared_tools,
            built_in_tool_names,
            existing_catalog_tool_names,
        )?;
        let capabilities = validate_required_texts(CAPABILITY_FIELD, draft.capabilities)?;
        let launch_policy = validate_launch_policy(runtime_kind, &draft.launch_policy)?;
        let sandbox = validate_sandbox_requirements(&draft.sandbox)?;
        assert_network_sandbox_combination(sandbox.network, sandbox.mode)?;
        let redaction = RedactionPolicy {
            redact_transcripts: draft.redaction.redact_transcripts,
            redact_secrets: draft.redaction.redact_secrets,
        };

        Ok(Self {
            identity,
            runtime_kind,
            declared_tools,
            capabilities,
            launch_policy,
            sandbox,
            redaction,
        })
    }

    /// Returns the normalized descriptor identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the descriptor runtime-kind label.
    #[must_use]
    pub const fn runtime_kind(&self) -> RuntimeKind {
        self.runtime_kind
    }

    /// Returns declared tools after validation and collision checks.
    #[must_use]
    pub fn declared_tools(&self) -> &[ToolDescriptor] {
        self.declared_tools.as_slice()
    }

    /// Returns normalized capability labels.
    #[must_use]
    pub fn capabilities(&self) -> &[String] {
        self.capabilities.as_slice()
    }

    /// Returns validated launch-policy metadata.
    #[must_use]
    pub const fn launch_policy(&self) -> &LaunchPolicy {
        &self.launch_policy
    }

    /// Returns validated sandbox and network claims.
    #[must_use]
    pub const fn sandbox(&self) -> &SandboxRequirements {
        &self.sandbox
    }

    /// Returns validated redaction claims.
    #[must_use]
    pub const fn redaction(&self) -> &RedactionPolicy {
        &self.redaction
    }
}

/// Runtime-kind descriptor labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RuntimeKind {
    /// Host-owned implementation; no launch command is required.
    Host,
    /// Stdio-shaped descriptor; validation records a command label but does not launch it.
    Stdio,
    /// Wasm-shaped descriptor; validation records an artifact label but does not load it.
    Wasm,
}

impl RuntimeKind {
    /// Returns the stable descriptor spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => RUNTIME_KIND_HOST,
            Self::Stdio => RUNTIME_KIND_STDIO,
            Self::Wasm => RUNTIME_KIND_WASM,
        }
    }

    const fn requires_launch_command(self) -> bool {
        match self {
            Self::Host => false,
            Self::Stdio | Self::Wasm => true,
        }
    }
}

/// Validated tool declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolDescriptor {
    name: String,
    description: Option<String>,
}

impl ToolDescriptor {
    /// Returns the normalized tool name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns optional descriptor text.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

/// Working-directory descriptor modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkingDirectoryMode {
    /// Future hosts may interpret paths relative to the project root.
    ProjectRoot,
    /// Future hosts may interpret paths relative to the descriptor location.
    ManifestDirectory,
}

impl WorkingDirectoryMode {
    /// Returns the stable descriptor spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectRoot => WORKING_DIRECTORY_PROJECT_ROOT,
            Self::ManifestDirectory => WORKING_DIRECTORY_MANIFEST_DIRECTORY,
        }
    }
}

/// Validated launch metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaunchPolicy {
    command_label: Option<String>,
    arguments: Vec<String>,
    working_directory: WorkingDirectoryMode,
    environment_allowlist: Vec<String>,
    writable_roots: Vec<String>,
}

impl LaunchPolicy {
    /// Returns the optional command or artifact label.
    #[must_use]
    pub fn command_label(&self) -> Option<&str> {
        self.command_label.as_deref()
    }

    /// Returns data-only argument labels.
    #[must_use]
    pub fn arguments(&self) -> &[String] {
        self.arguments.as_slice()
    }

    /// Returns the working-directory descriptor mode.
    #[must_use]
    pub const fn working_directory(&self) -> WorkingDirectoryMode {
        self.working_directory
    }

    /// Returns environment variable names only.
    #[must_use]
    pub fn environment_allowlist(&self) -> &[String] {
        self.environment_allowlist.as_slice()
    }

    /// Returns project-relative writable-root descriptors.
    #[must_use]
    pub fn writable_roots(&self) -> &[String] {
        self.writable_roots.as_slice()
    }
}

/// Sandbox-mode descriptor labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandboxMode {
    /// No sandbox claim is made by the descriptor.
    Unconfined,
    /// A future host must enforce isolation before running anything.
    Isolated,
}

impl SandboxMode {
    /// Returns the stable descriptor spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unconfined => SANDBOX_MODE_UNCONFINED,
            Self::Isolated => SANDBOX_MODE_ISOLATED,
        }
    }
}

/// Network-policy descriptor labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NetworkPolicy {
    /// Network access is denied.
    Denied,
    /// Only loopback network access is claimed.
    Loopback,
    /// Outbound network access is claimed.
    Outbound,
}

impl NetworkPolicy {
    /// Returns the stable descriptor spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Denied => NETWORK_POLICY_DENIED,
            Self::Loopback => NETWORK_POLICY_LOOPBACK,
            Self::Outbound => NETWORK_POLICY_OUTBOUND,
        }
    }
}

/// Validated sandbox and network claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SandboxRequirements {
    mode: SandboxMode,
    network: NetworkPolicy,
}

impl SandboxRequirements {
    /// Returns the sandbox-mode claim.
    #[must_use]
    pub const fn mode(self) -> SandboxMode {
        self.mode
    }

    /// Returns the network-policy claim.
    #[must_use]
    pub const fn network(self) -> NetworkPolicy {
        self.network
    }
}

/// Validated redaction claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RedactionPolicy {
    redact_transcripts: bool,
    redact_secrets: bool,
}

impl RedactionPolicy {
    /// Returns whether future host logs should redact transcript payloads.
    #[must_use]
    pub const fn redact_transcripts(self) -> bool {
        self.redact_transcripts
    }

    /// Returns whether future host logs should redact secret-looking payloads.
    #[must_use]
    pub const fn redact_secrets(self) -> bool {
        self.redact_secrets
    }
}

/// Validation errors for descriptor manifests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PluginManifestError {
    /// A required descriptor field was absent.
    MissingField {
        /// Missing field name.
        field: &'static str,
    },
    /// A descriptor field failed adapter-boundary text validation.
    FieldRejected {
        /// Rejected field name.
        field: &'static str,
        /// Text-validation reason.
        reason: crate::AdapterContractError,
    },
    /// A descriptor field used an unknown enumerated value.
    UnknownDescriptorValue {
        /// Field name with the unknown value.
        field: &'static str,
        /// Unknown value after normalization.
        value: String,
    },
    /// A declared tool name appeared more than once in the same descriptor.
    DuplicateTool {
        /// Duplicate tool name.
        name: String,
    },
    /// A declared tool name collided with a reserved or already registered name.
    ToolNameCollision {
        /// Colliding tool name.
        name: String,
    },
    /// A runtime kind that needs a data-only command label omitted it.
    MissingLaunchCommand {
        /// Runtime kind that requires the label.
        runtime_kind: RuntimeKind,
    },
    /// A writable-root descriptor was absolute.
    AbsoluteWritableRoot {
        /// Rejected writable-root value.
        value: String,
    },
    /// A writable-root descriptor escaped the project boundary.
    PathEscape {
        /// Rejected writable-root value.
        value: String,
    },
    /// An environment allowlist entry included a value instead of a name.
    EnvironmentValueLeak {
        /// Rejected environment entry.
        entry: String,
    },
    /// A network claim was unsafe for the sandbox claim.
    UnsafeNetworkSandboxCombination {
        /// Network policy claim.
        network: NetworkPolicy,
        /// Sandbox mode claim.
        sandbox: SandboxMode,
    },
}

impl core::fmt::Display for PluginManifestError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingField { field } => write!(formatter, "{field} is required"),
            Self::FieldRejected { field, reason } => {
                write!(formatter, "{field} rejected: {reason}")
            }
            Self::UnknownDescriptorValue { field, value } => {
                write!(formatter, "{field} has unknown descriptor value {value}")
            }
            Self::DuplicateTool { name } => write!(formatter, "declared tool {name} is duplicated"),
            Self::ToolNameCollision { name } => {
                write!(
                    formatter,
                    "declared tool {name} collides with an existing tool"
                )
            }
            Self::MissingLaunchCommand { runtime_kind } => write!(
                formatter,
                "{} descriptors require a data-only launch command label",
                runtime_kind.as_str()
            ),
            Self::AbsoluteWritableRoot { value } => {
                write!(formatter, "writable root {value} must be project-relative")
            }
            Self::PathEscape { value } => {
                write!(
                    formatter,
                    "writable root {value} must not escape the project"
                )
            }
            Self::EnvironmentValueLeak { entry } => write!(
                formatter,
                "environment allowlist entry {entry} must name a variable without a value"
            ),
            Self::UnsafeNetworkSandboxCombination { network, sandbox } => write!(
                formatter,
                "network policy {} is unsafe with sandbox mode {}",
                network.as_str(),
                sandbox.as_str()
            ),
        }
    }
}

fn validate_tools(
    drafts: &[ToolDescriptorDraft<'_>],
    built_in_tool_names: &[&str],
    existing_catalog_tool_names: &[&str],
) -> Result<Vec<ToolDescriptor>, PluginManifestError> {
    let mut tools = Vec::new();
    for draft in drafts {
        let name = required_text(TOOL_NAME_FIELD, draft.name)?;
        if contains_name(built_in_tool_names, &name)
            || contains_name(existing_catalog_tool_names, &name)
        {
            return Err(PluginManifestError::ToolNameCollision { name });
        }
        if tools
            .iter()
            .any(|tool: &ToolDescriptor| tool.name() == name)
        {
            return Err(PluginManifestError::DuplicateTool { name });
        }
        let description = draft
            .description
            .map(|text| optional_text(TOOL_DESCRIPTION_FIELD, text))
            .transpose()?;
        tools.push(ToolDescriptor { name, description });
    }
    Ok(tools)
}

fn validate_launch_policy(
    runtime_kind: RuntimeKind,
    draft: &LaunchPolicyDraft<'_>,
) -> Result<LaunchPolicy, PluginManifestError> {
    let command_label = match draft.command_label {
        Some(value) => Some(required_text_value(LAUNCH_COMMAND_LABEL_FIELD, value)?),
        None if runtime_kind.requires_launch_command() => {
            return Err(PluginManifestError::MissingLaunchCommand { runtime_kind });
        }
        None => None,
    };
    let arguments = validate_optional_texts(LAUNCH_ARGUMENT_FIELD, draft.arguments)?;
    let working_directory = match draft.working_directory {
        Some(value) => {
            parse_working_directory_mode(&required_text_value(WORKING_DIRECTORY_FIELD, value)?)?
        }
        None => WorkingDirectoryMode::ProjectRoot,
    };
    let environment_allowlist = validate_environment_allowlist(draft.environment_allowlist)?;
    let writable_roots = validate_writable_roots(draft.writable_roots)?;

    Ok(LaunchPolicy {
        command_label,
        arguments,
        working_directory,
        environment_allowlist,
        writable_roots,
    })
}

fn validate_sandbox_requirements(
    draft: &SandboxRequirementsDraft<'_>,
) -> Result<SandboxRequirements, PluginManifestError> {
    let mode = parse_sandbox_mode(&required_text(SANDBOX_MODE_FIELD, draft.mode)?)?;
    let network = parse_network_policy(&required_text(NETWORK_POLICY_FIELD, draft.network)?)?;
    Ok(SandboxRequirements { mode, network })
}

fn assert_network_sandbox_combination(
    network: NetworkPolicy,
    sandbox: SandboxMode,
) -> Result<(), PluginManifestError> {
    if network == NetworkPolicy::Outbound && sandbox == SandboxMode::Unconfined {
        return Err(PluginManifestError::UnsafeNetworkSandboxCombination { network, sandbox });
    }
    Ok(())
}

fn validate_environment_allowlist(entries: &[&str]) -> Result<Vec<String>, PluginManifestError> {
    let mut names = Vec::new();
    for entry in entries {
        let name = required_text_value(ENVIRONMENT_ALLOWLIST_FIELD, entry)?;
        if name.contains(ENVIRONMENT_VALUE_SEPARATOR) {
            return Err(PluginManifestError::EnvironmentValueLeak { entry: name });
        }
        names.push(name);
    }
    Ok(names)
}

fn validate_writable_roots(entries: &[&str]) -> Result<Vec<String>, PluginManifestError> {
    let mut roots = Vec::new();
    for entry in entries {
        let root = required_text_value(WRITABLE_ROOT_FIELD, entry)?;
        if is_absolute_path(&root) {
            return Err(PluginManifestError::AbsoluteWritableRoot { value: root });
        }
        if has_path_escape(&root) {
            return Err(PluginManifestError::PathEscape { value: root });
        }
        roots.push(root);
    }
    Ok(roots)
}

fn parse_runtime_kind(value: &str) -> Result<RuntimeKind, PluginManifestError> {
    match value {
        RUNTIME_KIND_HOST => Ok(RuntimeKind::Host),
        RUNTIME_KIND_STDIO => Ok(RuntimeKind::Stdio),
        RUNTIME_KIND_WASM => Ok(RuntimeKind::Wasm),
        unknown => Err(PluginManifestError::UnknownDescriptorValue {
            field: RUNTIME_KIND_FIELD.as_str(),
            value: String::from(unknown),
        }),
    }
}

fn parse_working_directory_mode(value: &str) -> Result<WorkingDirectoryMode, PluginManifestError> {
    match value {
        WORKING_DIRECTORY_PROJECT_ROOT => Ok(WorkingDirectoryMode::ProjectRoot),
        WORKING_DIRECTORY_MANIFEST_DIRECTORY => Ok(WorkingDirectoryMode::ManifestDirectory),
        unknown => Err(PluginManifestError::UnknownDescriptorValue {
            field: WORKING_DIRECTORY_FIELD.as_str(),
            value: String::from(unknown),
        }),
    }
}

fn parse_sandbox_mode(value: &str) -> Result<SandboxMode, PluginManifestError> {
    match value {
        SANDBOX_MODE_UNCONFINED => Ok(SandboxMode::Unconfined),
        SANDBOX_MODE_ISOLATED => Ok(SandboxMode::Isolated),
        unknown => Err(PluginManifestError::UnknownDescriptorValue {
            field: SANDBOX_MODE_FIELD.as_str(),
            value: String::from(unknown),
        }),
    }
}

fn parse_network_policy(value: &str) -> Result<NetworkPolicy, PluginManifestError> {
    match value {
        NETWORK_POLICY_DENIED => Ok(NetworkPolicy::Denied),
        NETWORK_POLICY_LOOPBACK => Ok(NetworkPolicy::Loopback),
        NETWORK_POLICY_OUTBOUND => Ok(NetworkPolicy::Outbound),
        unknown => Err(PluginManifestError::UnknownDescriptorValue {
            field: NETWORK_POLICY_FIELD.as_str(),
            value: String::from(unknown),
        }),
    }
}

fn validate_required_texts(
    field_name: crate::validation::FieldName,
    values: &[&str],
) -> Result<Vec<String>, PluginManifestError> {
    let mut normalized = Vec::new();
    for value in values {
        normalized.push(required_text_value(field_name, value)?);
    }
    Ok(normalized)
}

fn validate_optional_texts(
    field_name: crate::validation::FieldName,
    values: &[&str],
) -> Result<Vec<String>, PluginManifestError> {
    let mut normalized = Vec::new();
    for value in values {
        normalized.push(optional_text(field_name, value)?);
    }
    Ok(normalized)
}

fn required_text(
    field_name: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<String, PluginManifestError> {
    match value {
        Some(raw) => required_text_value(field_name, raw),
        None => Err(PluginManifestError::MissingField {
            field: field_name.as_str(),
        }),
    }
}

fn required_text_value(
    field_name: crate::validation::FieldName,
    value: &str,
) -> Result<String, PluginManifestError> {
    crate::validation::validate_required_adapter_field(field_name, value).map_err(|reason| {
        PluginManifestError::FieldRejected {
            field: field_name.as_str(),
            reason,
        }
    })?;
    Ok(String::from(value.trim()))
}

fn optional_text(
    field_name: crate::validation::FieldName,
    value: &str,
) -> Result<String, PluginManifestError> {
    crate::validation::validate_optional_adapter_field(field_name, value).map_err(|reason| {
        PluginManifestError::FieldRejected {
            field: field_name.as_str(),
            reason,
        }
    })?;
    Ok(String::from(value))
}

fn contains_name(names: &[&str], candidate: &str) -> bool {
    names.contains(&candidate)
}

fn is_absolute_path(path: &str) -> bool {
    path.starts_with(UNIX_PATH_SEPARATOR)
        || path.starts_with(WINDOWS_PATH_SEPARATOR)
        || has_windows_drive_prefix(path)
}

fn has_windows_drive_prefix(path: &str) -> bool {
    let bytes = path.as_bytes();
    bytes.len() > WINDOWS_DRIVE_SEPARATOR_INDEX
        && bytes[WINDOWS_DRIVE_SEPARATOR_INDEX] == WINDOWS_DRIVE_SEPARATOR
        && bytes.first().is_some_and(u8::is_ascii_alphabetic)
}

fn has_path_escape(path: &str) -> bool {
    path.starts_with(HOME_PATH_PREFIX)
        || path
            .split(is_path_separator)
            .any(|segment| segment == PATH_ESCAPE_SEGMENT)
}

fn is_path_separator(character: char) -> bool {
    character == UNIX_PATH_SEPARATOR || character == WINDOWS_PATH_SEPARATOR
}

#[cfg(test)]
mod tests {
    use super::*;

    const IDENTITY: &str = "example.safe";
    const TOOL_NAME: &str = "safe.lookup";
    const SECOND_TOOL_NAME: &str = "safe.summarize";
    const BUILT_IN_TOOL_NAME: &str = "builtin.search";
    const CATALOG_TOOL_NAME: &str = "catalog.lookup";
    const TOOL_DESCRIPTION: &str = "safe lookup descriptor";
    const CAPABILITY: &str = "workspace-read";
    const COMMAND_LABEL: &str = "safe-binary";
    const ARGUMENT_LABEL: &str = "--json";
    const ENVIRONMENT_NAME: &str = "ANIMUS_SAFE_TOKEN";
    const SAFE_WRITABLE_ROOT: &str = "cache/safe";
    const UNKNOWN_RUNTIME_KIND: &str = "daemon";
    const PATH_ESCAPE_ROOT: &str = "../outside";
    const ABSOLUTE_ROOT: &str = "/var/tmp";
    const ENVIRONMENT_VALUE: &str = "SECRET=value";
    const BUILT_IN_TOOLS: &[&str] = &[BUILT_IN_TOOL_NAME];
    const CATALOG_TOOLS: &[&str] = &[CATALOG_TOOL_NAME];
    const CAPABILITIES: &[&str] = &[CAPABILITY];
    const ARGUMENTS: &[&str] = &[ARGUMENT_LABEL];
    const ENVIRONMENT_ALLOWLIST: &[&str] = &[ENVIRONMENT_NAME];
    const WRITABLE_ROOTS: &[&str] = &[SAFE_WRITABLE_ROOT];
    const DECLARED_TOOLS: &[ToolDescriptorDraft<'static>] = &[ToolDescriptorDraft {
        name: Some(TOOL_NAME),
        description: Some(TOOL_DESCRIPTION),
    }];
    const DUPLICATE_DECLARED_TOOLS: &[ToolDescriptorDraft<'static>] = &[
        ToolDescriptorDraft {
            name: Some(TOOL_NAME),
            description: Some(TOOL_DESCRIPTION),
        },
        ToolDescriptorDraft {
            name: Some(TOOL_NAME),
            description: Some(TOOL_DESCRIPTION),
        },
    ];
    const BUILT_IN_COLLISION_TOOLS: &[ToolDescriptorDraft<'static>] = &[ToolDescriptorDraft {
        name: Some(BUILT_IN_TOOL_NAME),
        description: Some(TOOL_DESCRIPTION),
    }];
    const CATALOG_COLLISION_TOOLS: &[ToolDescriptorDraft<'static>] = &[ToolDescriptorDraft {
        name: Some(CATALOG_TOOL_NAME),
        description: Some(TOOL_DESCRIPTION),
    }];
    const TWO_DECLARED_TOOLS: &[ToolDescriptorDraft<'static>] = &[
        ToolDescriptorDraft {
            name: Some(TOOL_NAME),
            description: Some(TOOL_DESCRIPTION),
        },
        ToolDescriptorDraft {
            name: Some(SECOND_TOOL_NAME),
            description: None,
        },
    ];

    fn safe_stdio_draft() -> PluginManifestDraft<'static> {
        PluginManifestDraft {
            identity: Some(IDENTITY),
            runtime_kind: Some(RUNTIME_KIND_STDIO),
            declared_tools: DECLARED_TOOLS,
            capabilities: CAPABILITIES,
            launch_policy: LaunchPolicyDraft {
                command_label: Some(COMMAND_LABEL),
                arguments: ARGUMENTS,
                working_directory: Some(WORKING_DIRECTORY_PROJECT_ROOT),
                environment_allowlist: ENVIRONMENT_ALLOWLIST,
                writable_roots: WRITABLE_ROOTS,
            },
            sandbox: SandboxRequirementsDraft {
                mode: Some(SANDBOX_MODE_ISOLATED),
                network: Some(NETWORK_POLICY_DENIED),
            },
            redaction: RedactionPolicyDraft {
                redact_transcripts: true,
                redact_secrets: true,
            },
        }
    }

    #[test]
    fn accepts_safe_stdio_descriptor() {
        let manifest = PluginManifest::validate(&safe_stdio_draft(), BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect("safe descriptor should validate");

        assert_eq!(manifest.identity(), IDENTITY);
        assert_eq!(manifest.runtime_kind(), RuntimeKind::Stdio);
        assert_eq!(manifest.declared_tools().len(), DECLARED_TOOLS.len());
        assert_eq!(manifest.declared_tools()[0].name(), TOOL_NAME);
        assert_eq!(manifest.capabilities(), &[String::from(CAPABILITY)]);
        assert_eq!(
            manifest.launch_policy().command_label(),
            Some(COMMAND_LABEL)
        );
        assert_eq!(
            manifest.launch_policy().arguments(),
            &[String::from(ARGUMENT_LABEL)]
        );
        assert_eq!(
            manifest.launch_policy().working_directory(),
            WorkingDirectoryMode::ProjectRoot
        );
        assert_eq!(
            manifest.launch_policy().environment_allowlist(),
            &[String::from(ENVIRONMENT_NAME)]
        );
        assert_eq!(
            manifest.launch_policy().writable_roots(),
            &[String::from(SAFE_WRITABLE_ROOT)]
        );
        assert_eq!(manifest.sandbox().mode(), SandboxMode::Isolated);
        assert_eq!(manifest.sandbox().network(), NetworkPolicy::Denied);
        assert!(manifest.redaction().redact_transcripts());
        assert!(manifest.redaction().redact_secrets());
    }

    #[test]
    fn accepts_host_descriptor_without_launch_command() {
        let mut draft = safe_stdio_draft();
        draft.runtime_kind = Some(RUNTIME_KIND_HOST);
        draft.declared_tools = TWO_DECLARED_TOOLS;
        draft.launch_policy.command_label = None;
        draft.launch_policy.working_directory = Some(WORKING_DIRECTORY_MANIFEST_DIRECTORY);
        let manifest = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect("host descriptor should not require a launch label");

        assert_eq!(manifest.runtime_kind(), RuntimeKind::Host);
        assert_eq!(manifest.declared_tools().len(), TWO_DECLARED_TOOLS.len());
        assert_eq!(
            manifest.launch_policy().working_directory(),
            WorkingDirectoryMode::ManifestDirectory
        );
    }

    #[test]
    fn rejects_missing_required_identity() {
        let mut draft = safe_stdio_draft();
        draft.identity = None;
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("missing identity should fail");

        assert_eq!(
            error,
            PluginManifestError::MissingField {
                field: IDENTITY_FIELD.as_str(),
            }
        );
    }

    #[test]
    fn rejects_unknown_runtime_kind() {
        let mut draft = safe_stdio_draft();
        draft.runtime_kind = Some(UNKNOWN_RUNTIME_KIND);
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("unknown runtime kind should fail");

        assert_eq!(
            error,
            PluginManifestError::UnknownDescriptorValue {
                field: RUNTIME_KIND_FIELD.as_str(),
                value: String::from(UNKNOWN_RUNTIME_KIND),
            }
        );
    }

    #[test]
    fn rejects_missing_launch_command_for_stdio() {
        let mut draft = safe_stdio_draft();
        draft.launch_policy.command_label = None;
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("stdio descriptors require a launch label");

        assert_eq!(
            error,
            PluginManifestError::MissingLaunchCommand {
                runtime_kind: RuntimeKind::Stdio,
            }
        );
    }

    #[test]
    fn rejects_duplicate_declared_tools() {
        let mut draft = safe_stdio_draft();
        draft.declared_tools = DUPLICATE_DECLARED_TOOLS;
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("duplicate tool names should fail");

        assert_eq!(
            error,
            PluginManifestError::DuplicateTool {
                name: String::from(TOOL_NAME),
            }
        );
    }

    #[test]
    fn rejects_built_in_tool_collision() {
        let mut draft = safe_stdio_draft();
        draft.declared_tools = BUILT_IN_COLLISION_TOOLS;
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("built-in tool name collision should fail");

        assert_eq!(
            error,
            PluginManifestError::ToolNameCollision {
                name: String::from(BUILT_IN_TOOL_NAME),
            }
        );
    }

    #[test]
    fn rejects_catalog_tool_collision() {
        let mut draft = safe_stdio_draft();
        draft.declared_tools = CATALOG_COLLISION_TOOLS;
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("catalog tool name collision should fail");

        assert_eq!(
            error,
            PluginManifestError::ToolNameCollision {
                name: String::from(CATALOG_TOOL_NAME),
            }
        );
    }

    #[test]
    fn rejects_path_escape_writable_root() {
        let mut draft = safe_stdio_draft();
        draft.launch_policy.writable_roots = &[PATH_ESCAPE_ROOT];
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("path escapes should fail");

        assert_eq!(
            error,
            PluginManifestError::PathEscape {
                value: String::from(PATH_ESCAPE_ROOT),
            }
        );
    }

    #[test]
    fn rejects_absolute_writable_root() {
        let mut draft = safe_stdio_draft();
        draft.launch_policy.writable_roots = &[ABSOLUTE_ROOT];
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("absolute writable roots should fail");

        assert_eq!(
            error,
            PluginManifestError::AbsoluteWritableRoot {
                value: String::from(ABSOLUTE_ROOT),
            }
        );
    }

    #[test]
    fn rejects_environment_value_leakage() {
        let mut draft = safe_stdio_draft();
        draft.launch_policy.environment_allowlist = &[ENVIRONMENT_VALUE];
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("environment allowlist values should fail");

        assert_eq!(
            error,
            PluginManifestError::EnvironmentValueLeak {
                entry: String::from(ENVIRONMENT_VALUE),
            }
        );
    }

    #[test]
    fn rejects_unsafe_network_without_sandbox() {
        let mut draft = safe_stdio_draft();
        draft.sandbox = SandboxRequirementsDraft {
            mode: Some(SANDBOX_MODE_UNCONFINED),
            network: Some(NETWORK_POLICY_OUTBOUND),
        };
        let error = PluginManifest::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
            .expect_err("outbound network without isolation should fail");

        assert_eq!(
            error,
            PluginManifestError::UnsafeNetworkSandboxCombination {
                network: NetworkPolicy::Outbound,
                sandbox: SandboxMode::Unconfined,
            }
        );
    }

    #[test]
    fn runtime_neutrality_policy_lists_prohibited_effects() {
        for prohibited_effect in [
            "stdio process launch",
            "wasm module loading",
            "environment inheritance",
            "secret reads",
            "network connections",
            "tool execution",
        ] {
            assert!(PROHIBITED_PLUGIN_MANIFEST_EFFECTS.contains(&prohibited_effect));
        }
    }
}
