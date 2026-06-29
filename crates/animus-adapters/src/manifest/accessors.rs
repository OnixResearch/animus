impl super::PluginDescriptor {
    /// Returns the normalized descriptor identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the descriptor runtime-kind label.
    #[must_use]
    pub const fn runtime_kind(&self) -> super::RuntimeKind {
        self.runtime_kind
    }

    /// Returns declared tools after validation and collision checks.
    #[must_use]
    pub fn declared_tools(&self) -> &[super::ToolDescriptor] {
        self.declared_tools.as_slice()
    }

    /// Returns normalized capability labels.
    #[must_use]
    pub fn capabilities(&self) -> &[super::Text] {
        self.capabilities.as_slice()
    }

    /// Returns validated launch-policy metadata.
    #[must_use]
    pub const fn launch_policy(&self) -> &super::LaunchPolicy {
        &self.launch_policy
    }

    /// Returns validated sandbox and network claims.
    #[must_use]
    pub const fn sandbox(&self) -> &super::SandboxRequirements {
        &self.sandbox
    }

    /// Returns validated redaction claims.
    #[must_use]
    pub const fn redaction(&self) -> &super::RedactionPolicy {
        &self.redaction
    }
}

impl super::RuntimeKind {
    /// Returns the stable descriptor spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Host => super::RUNTIME_KIND_HOST,
            Self::Stdio => super::RUNTIME_KIND_STDIO,
            Self::Wasm => super::RUNTIME_KIND_WASM,
        }
    }

    pub(super) const fn requires_launch_command(self) -> bool {
        match self {
            Self::Host => false,
            Self::Stdio | Self::Wasm => true,
        }
    }
}

impl super::ToolDescriptor {
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

impl super::WorkingDirectoryMode {
    /// Returns the stable descriptor spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ProjectRoot => super::WORKING_DIRECTORY_PROJECT_ROOT,
            Self::ManifestDirectory => super::WORKING_DIRECTORY_MANIFEST_DIRECTORY,
        }
    }
}

impl super::LaunchPolicy {
    /// Returns the optional command or artifact label.
    #[must_use]
    pub fn command_label(&self) -> Option<&str> {
        self.command_label.as_deref()
    }

    /// Returns data-only argument labels.
    #[must_use]
    pub fn arguments(&self) -> &[super::Text] {
        self.arguments.as_slice()
    }

    /// Returns the working-directory descriptor mode.
    #[must_use]
    pub const fn working_directory(&self) -> super::WorkingDirectoryMode {
        self.working_directory
    }

    /// Returns environment variable names only.
    #[must_use]
    pub fn environment_allowlist(&self) -> &[super::Text] {
        self.environment_allowlist.as_slice()
    }

    /// Returns project-relative writable-root descriptors.
    #[must_use]
    pub fn writable_roots(&self) -> &[super::Text] {
        self.writable_roots.as_slice()
    }
}

impl super::SandboxMode {
    /// Returns the stable descriptor spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unconfined => super::SANDBOX_MODE_UNCONFINED,
            Self::Isolated => super::SANDBOX_MODE_ISOLATED,
        }
    }
}

impl super::NetworkPolicy {
    /// Returns the stable descriptor spelling.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Denied => super::NETWORK_POLICY_DENIED,
            Self::Loopback => super::NETWORK_POLICY_LOOPBACK,
            Self::Outbound => super::NETWORK_POLICY_OUTBOUND,
        }
    }
}

impl super::SandboxRequirements {
    /// Returns the sandbox-mode claim.
    #[must_use]
    pub const fn mode(self) -> super::SandboxMode {
        self.mode
    }

    /// Returns the network-policy claim.
    #[must_use]
    pub const fn network(self) -> super::NetworkPolicy {
        self.network
    }
}

impl super::RedactionPolicy {
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
