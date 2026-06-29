type Text = alloc::string::String;
type Collection<T> = alloc::vec::Vec<T>;

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
pub struct PluginDescriptorDraft<'a> {
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
pub struct PluginDescriptor {
    identity: Text,
    runtime_kind: RuntimeKind,
    declared_tools: Collection<ToolDescriptor>,
    capabilities: Collection<Text>,
    launch_policy: LaunchPolicy,
    sandbox: SandboxRequirements,
    redaction: RedactionPolicy,
}

impl PluginDescriptor {
    /// Validates untrusted manifest-shaped data as a descriptor only.
    ///
    /// # Errors
    ///
    /// Returns [`PluginDescriptorError`] when required fields are missing, field
    /// text is invalid, runtime/sandbox/network labels are unknown, tool names
    /// collide, or path/network declarations are unsafe.
    pub fn validate(
        draft: &PluginDescriptorDraft<'_>,
        built_in_tool_names: &[&str],
        existing_catalog_tool_names: &[&str],
    ) -> Result<Self, PluginDescriptorError> {
        let identity = validation::required_text(IDENTITY_FIELD, draft.identity)?;
        let runtime_text = validation::required_text(RUNTIME_KIND_FIELD, draft.runtime_kind)?;
        let runtime_kind = validation::parse_runtime_kind(&runtime_text)?;
        let declared_tools = validation::validate_tools(
            draft.declared_tools,
            built_in_tool_names,
            existing_catalog_tool_names,
        )?;
        let capabilities =
            validation::validate_required_texts(CAPABILITY_FIELD, draft.capabilities)?;
        let launch_policy = validation::validate_launch_policy(runtime_kind, &draft.launch_policy)?;
        let sandbox = validation::validate_sandbox_requirements(&draft.sandbox)?;
        validation::assert_network_sandbox_combination(sandbox.network, sandbox.mode)?;
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

/// Validated tool declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolDescriptor {
    name: Text,
    description: Option<Text>,
}

/// Working-directory descriptor modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorkingDirectoryMode {
    /// Future hosts may interpret paths relative to the project root.
    ProjectRoot,
    /// Future hosts may interpret paths relative to the descriptor location.
    ManifestDirectory,
}

/// Validated launch metadata.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LaunchPolicy {
    command_label: Option<Text>,
    arguments: Collection<Text>,
    working_directory: WorkingDirectoryMode,
    environment_allowlist: Collection<Text>,
    writable_roots: Collection<Text>,
}

/// Sandbox-mode descriptor labels.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandboxMode {
    /// No sandbox claim is made by the descriptor.
    Unconfined,
    /// A future host must enforce isolation before running anything.
    Isolated,
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

/// Validated sandbox and network claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SandboxRequirements {
    mode: SandboxMode,
    network: NetworkPolicy,
}

/// Validated redaction claims.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RedactionPolicy {
    redact_transcripts: bool,
    redact_secrets: bool,
}

mod accessors;
mod error;
mod validation;

/// Descriptor manifest validation error.
pub type PluginDescriptorError = self::error::ValidationFailure;

#[cfg(test)]
mod tests;
