const MODEL_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::PromptOmitted,
    animus_adapters::ReceiptRedaction::ProviderPayloadOmitted,
    animus_adapters::ReceiptRedaction::ModelOutputOmitted,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];
const TOOL_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::ToolInputOmitted,
    animus_adapters::ReceiptRedaction::ToolOutputOmitted,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];
const FILESYSTEM_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::AbsolutePathOmitted,
    animus_adapters::ReceiptRedaction::ToolInputOmitted,
    animus_adapters::ReceiptRedaction::ToolOutputOmitted,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];
const NETWORK_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::ProviderPayloadOmitted,
    animus_adapters::ReceiptRedaction::OutputBounded,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];
const FIXTURE_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::OutputBounded,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];

/// Concrete adapter family owned by the standard-library shell.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AdapterFamily {
    /// Host-owned model provider clients.
    ModelProvider,
    /// Host-owned filesystem tool adapters.
    FilesystemTool,
    /// Host-owned process execution adapters.
    ProcessExecution,
    /// Host-owned HTTP or network helper adapters.
    HttpNetworkHelper,
    /// Deterministic fixture backends used by default tests.
    FixtureBackend,
}

impl AdapterFamily {
    /// Returns the stable adapter family label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModelProvider => "model_provider",
            Self::FilesystemTool => "filesystem_tool",
            Self::ProcessExecution => "process_execution",
            Self::HttpNetworkHelper => "http_network_helper",
            Self::FixtureBackend => "fixture_backend",
        }
    }

    /// Returns true because concrete families are shell-owned by contract.
    #[must_use]
    pub const fn is_shell_owned(self) -> bool {
        match self {
            Self::ModelProvider
            | Self::FilesystemTool
            | Self::ProcessExecution
            | Self::HttpNetworkHelper
            | Self::FixtureBackend => true,
        }
    }

    pub(super) const fn operation(self) -> animus_adapters::ReceiptOperation {
        match self {
            Self::ModelProvider => animus_adapters::ReceiptOperation::ModelDispatch,
            Self::FilesystemTool | Self::ProcessExecution | Self::HttpNetworkHelper => {
                animus_adapters::ReceiptOperation::ToolExecution
            }
            Self::FixtureBackend => animus_adapters::ReceiptOperation::AdapterBoundary,
        }
    }

    pub(super) const fn redactions(self) -> &'static [animus_adapters::ReceiptRedaction] {
        match self {
            Self::ModelProvider => MODEL_REDACTIONS,
            Self::FilesystemTool => FILESYSTEM_REDACTIONS,
            Self::ProcessExecution => TOOL_REDACTIONS,
            Self::HttpNetworkHelper => NETWORK_REDACTIONS,
            Self::FixtureBackend => FIXTURE_REDACTIONS,
        }
    }
}

/// Runtime exposure mode for a std-shell adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExposure {
    /// A host-owned adapter that may perform live effects only after gates pass.
    HostOwned,
    /// A deterministic fixture adapter that never performs live host effects.
    FixtureOnly,
}

impl EffectExposure {
    /// Returns the stable exposure label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostOwned => "host_owned",
            Self::FixtureOnly => "fixture_only",
        }
    }

    /// Returns true when the adapter can represent a live host effect.
    #[must_use]
    pub const fn allows_live_effects(self) -> bool {
        match self {
            Self::HostOwned => true,
            Self::FixtureOnly => false,
        }
    }
}

/// Borrowed descriptor for one concrete std-shell adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdapterDescriptor<'a> {
    family: AdapterFamily,
    label: &'a str,
    exposure: EffectExposure,
}

impl<'a> AdapterDescriptor<'a> {
    /// Builds a host-owned adapter descriptor.
    #[must_use]
    pub const fn host_owned(family: AdapterFamily, label: &'a str) -> Self {
        Self {
            family,
            label,
            exposure: EffectExposure::HostOwned,
        }
    }

    /// Builds a deterministic fixture adapter descriptor.
    #[must_use]
    pub const fn fixture(label: &'a str) -> Self {
        Self {
            family: AdapterFamily::FixtureBackend,
            label,
            exposure: EffectExposure::FixtureOnly,
        }
    }

    /// Returns the adapter family.
    #[must_use]
    pub const fn family(self) -> AdapterFamily {
        self.family
    }

    /// Returns the safe adapter label supplied by the shell.
    #[must_use]
    pub const fn label(self) -> &'a str {
        self.label
    }

    /// Returns the adapter exposure mode.
    #[must_use]
    pub const fn exposure(self) -> EffectExposure {
        self.exposure
    }

    /// Returns true when this descriptor can represent a live host effect.
    #[must_use]
    pub const fn allows_live_effects(self) -> bool {
        self.exposure.allows_live_effects()
    }
}
