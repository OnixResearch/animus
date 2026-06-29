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

fn safe_stdio_draft() -> PluginDescriptorDraft<'static> {
    PluginDescriptorDraft {
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
    let manifest = PluginDescriptor::validate(&safe_stdio_draft(), BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect("safe descriptor should validate");

    assert_eq!(manifest.identity(), IDENTITY);
    assert_eq!(manifest.runtime_kind(), RuntimeKind::Stdio);
    assert_eq!(manifest.declared_tools().len(), DECLARED_TOOLS.len());
    assert_eq!(manifest.declared_tools()[0].name(), TOOL_NAME);
    assert_eq!(manifest.capabilities(), &[Text::from(CAPABILITY)]);
    assert_eq!(
        manifest.launch_policy().command_label(),
        Some(COMMAND_LABEL)
    );
    assert_eq!(
        manifest.launch_policy().arguments(),
        &[Text::from(ARGUMENT_LABEL)]
    );
    assert_eq!(
        manifest.launch_policy().working_directory(),
        WorkingDirectoryMode::ProjectRoot
    );
    assert_eq!(
        manifest.launch_policy().environment_allowlist(),
        &[Text::from(ENVIRONMENT_NAME)]
    );
    assert_eq!(
        manifest.launch_policy().writable_roots(),
        &[Text::from(SAFE_WRITABLE_ROOT)]
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
    let manifest = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
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
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("missing identity should fail");

    assert_eq!(
        error,
        PluginDescriptorError::MissingField {
            field: IDENTITY_FIELD.as_str(),
        }
    );
}

#[test]
fn rejects_unknown_runtime_kind() {
    let mut draft = safe_stdio_draft();
    draft.runtime_kind = Some(UNKNOWN_RUNTIME_KIND);
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("unknown runtime kind should fail");

    assert_eq!(
        error,
        PluginDescriptorError::UnknownDescriptorValue {
            field: RUNTIME_KIND_FIELD.as_str(),
            value: Text::from(UNKNOWN_RUNTIME_KIND),
        }
    );
}

#[test]
fn rejects_missing_launch_command_for_stdio() {
    let mut draft = safe_stdio_draft();
    draft.launch_policy.command_label = None;
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("stdio descriptors require a launch label");

    assert_eq!(
        error,
        PluginDescriptorError::MissingLaunchCommand {
            runtime_kind: RuntimeKind::Stdio,
        }
    );
}

#[test]
fn rejects_duplicate_declared_tools() {
    let mut draft = safe_stdio_draft();
    draft.declared_tools = DUPLICATE_DECLARED_TOOLS;
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("duplicate tool names should fail");

    assert_eq!(
        error,
        PluginDescriptorError::DuplicateTool {
            name: Text::from(TOOL_NAME),
        }
    );
}

#[test]
fn rejects_built_in_tool_collision() {
    let mut draft = safe_stdio_draft();
    draft.declared_tools = BUILT_IN_COLLISION_TOOLS;
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("built-in tool name collision should fail");

    assert_eq!(
        error,
        PluginDescriptorError::ToolNameCollision {
            name: Text::from(BUILT_IN_TOOL_NAME),
        }
    );
}

#[test]
fn rejects_catalog_tool_collision() {
    let mut draft = safe_stdio_draft();
    draft.declared_tools = CATALOG_COLLISION_TOOLS;
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("catalog tool name collision should fail");

    assert_eq!(
        error,
        PluginDescriptorError::ToolNameCollision {
            name: Text::from(CATALOG_TOOL_NAME),
        }
    );
}

#[test]
fn rejects_path_escape_writable_root() {
    let mut draft = safe_stdio_draft();
    draft.launch_policy.writable_roots = &[PATH_ESCAPE_ROOT];
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("path escapes should fail");

    assert_eq!(
        error,
        PluginDescriptorError::PathEscape {
            value: Text::from(PATH_ESCAPE_ROOT),
        }
    );
}

#[test]
fn rejects_absolute_writable_root() {
    let mut draft = safe_stdio_draft();
    draft.launch_policy.writable_roots = &[ABSOLUTE_ROOT];
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("absolute writable roots should fail");

    assert_eq!(
        error,
        PluginDescriptorError::AbsoluteWritableRoot {
            value: Text::from(ABSOLUTE_ROOT),
        }
    );
}

#[test]
fn rejects_environment_value_leakage() {
    let mut draft = safe_stdio_draft();
    draft.launch_policy.environment_allowlist = &[ENVIRONMENT_VALUE];
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("environment allowlist values should fail");

    assert_eq!(
        error,
        PluginDescriptorError::EnvironmentValueLeak {
            entry: Text::from(ENVIRONMENT_VALUE),
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
    let error = PluginDescriptor::validate(&draft, BUILT_IN_TOOLS, CATALOG_TOOLS)
        .expect_err("outbound network without isolation should fail");

    assert_eq!(
        error,
        PluginDescriptorError::UnsafeNetworkSandboxCombination {
            network: NetworkPolicy::Outbound,
            sandbox: SandboxMode::Unconfined,
        }
    );
}
