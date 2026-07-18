//! Pure authority catalog and fail-closed host-adapter planning.
//!
//! This module describes std-shell blocks as data. It never reads environment
//! variables, opens files, starts processes, contacts networks, inspects
//! credentials, persists state, or accesses terminal state.

const HOST_ADAPTER_DESCRIPTOR_COUNT: usize = 13;
const NO_HOST_EFFECT: &[HostAuthority] = &[HostAuthority::NoHostEffect];
const ENVIRONMENT_AUTHORITY: &[HostAuthority] = &[HostAuthority::EnvironmentRead];
const FILESYSTEM_AUTHORITY: &[HostAuthority] = &[HostAuthority::FilesystemRead];
const PROVIDER_AUTHORITY: &[HostAuthority] = &[
    HostAuthority::EnvironmentRead,
    HostAuthority::CredentialAccess,
];
const INGEST_AUTHORITY: &[HostAuthority] = &[
    HostAuthority::FilesystemRead,
    HostAuthority::ProcessExecution,
];
const TRANSPORT_AUTHORITY: &[HostAuthority] = &[HostAuthority::NetworkTransport];
const SANDBOX_AUTHORITY: &[HostAuthority] = &[
    HostAuthority::FilesystemRead,
    HostAuthority::ProcessExecution,
    HostAuthority::NetworkTransport,
];
const PERSISTENCE_AUTHORITY: &[HostAuthority] = &[HostAuthority::Persistence];
const CODING_SESSION_AUTHORITY: &[HostAuthority] = &[
    HostAuthority::NetworkTransport,
    HostAuthority::ProcessExecution,
    HostAuthority::Persistence,
];

/// Explicit host authority class for a std adapter block.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HostAuthority {
    /// No host effect; the block is pure or in-memory only.
    NoHostEffect,
    /// Read explicitly supplied environment pairs or process environment state.
    EnvironmentRead,
    /// Read files under an explicitly admitted capability root.
    FilesystemRead,
    /// Execute an explicitly admitted host process.
    ProcessExecution,
    /// Open an explicitly admitted network transport.
    NetworkTransport,
    /// Resolve a credential through an explicit host-owned source.
    CredentialAccess,
    /// Read or write an explicitly supplied durable store.
    Persistence,
    /// Access terminal input, output, or presentation state.
    TerminalAccess,
}

impl HostAuthority {
    /// Returns the stable authority label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoHostEffect => "no_host_effect",
            Self::EnvironmentRead => "environment_read",
            Self::FilesystemRead => "filesystem_read",
            Self::ProcessExecution => "process_execution",
            Self::NetworkTransport => "network_transport",
            Self::CredentialAccess => "credential_access",
            Self::Persistence => "persistence",
            Self::TerminalAccess => "terminal_access",
        }
    }
}

/// Authority-scoped std host adapter family.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum HostAdapterFamily {
    /// Runtime configuration loading and parsing.
    ConfigLoading,
    /// Capability-rooted local context resolution.
    LocalContext,
    /// Provider registry selection and credential-source diagnostics.
    ProviderRegistry,
    /// No-op and deterministic in-memory observability sinks.
    Observability,
    /// Document extraction planning and shell validation.
    DocumentIngest,
    /// Model-evidence ingestion planning and shell validation.
    ModelEvidenceIngest,
    /// Optional Iroh transport seam.
    IrohTransport,
    /// Optional Syndicate transport seam.
    SyndicateTransport,
    /// Optional composition of Iroh transport and Syndicate turn packets.
    IrohSyndicateTransport,
    /// Optional Linux Sandlock process-enforcement shell.
    SandlockEnforcement,
    /// Optional capability-scoped durable local session store.
    DurableSessionStore,
    /// Optional composed coding-session lifecycle host.
    CodingSessionHost,
    /// Packaged smoke and demonstration shell.
    SmokeDemo,
}

impl HostAdapterFamily {
    /// Returns the stable family label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ConfigLoading => "config_loading",
            Self::LocalContext => "local_context",
            Self::ProviderRegistry => "provider_registry",
            Self::Observability => "observability",
            Self::DocumentIngest => "document_ingest",
            Self::ModelEvidenceIngest => "model_evidence_ingest",
            Self::IrohTransport => "iroh_transport",
            Self::SyndicateTransport => "syndicate_transport",
            Self::IrohSyndicateTransport => "iroh_syndicate_transport",
            Self::SandlockEnforcement => "sandlock_enforcement",
            Self::DurableSessionStore => "durable_session_store",
            Self::CodingSessionHost => "coding_session_host",
            Self::SmokeDemo => "smoke_demo",
        }
    }
}

/// Metadata-only descriptor for one std host adapter block.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostAdapterDescriptor {
    family: HostAdapterFamily,
    owner: &'static str,
    feature_gate: &'static str,
    required_authority: &'static [HostAuthority],
    facade_path: &'static str,
    direct_dependency: &'static str,
    focused_check: &'static str,
}

impl HostAdapterDescriptor {
    /// Returns the adapter family.
    #[must_use]
    pub const fn family(self) -> HostAdapterFamily {
        self.family
    }

    /// Returns the owning crate or module.
    #[must_use]
    pub const fn owner(self) -> &'static str {
        self.owner
    }

    /// Returns the feature-gate label.
    #[must_use]
    pub const fn feature_gate(self) -> &'static str {
        self.feature_gate
    }

    /// Returns every required host authority class.
    #[must_use]
    pub const fn required_authority(self) -> &'static [HostAuthority] {
        self.required_authority
    }

    /// Returns the compatibility facade import path.
    #[must_use]
    pub const fn facade_path(self) -> &'static str {
        self.facade_path
    }

    /// Returns the smallest direct dependency/import guidance.
    #[must_use]
    pub const fn direct_dependency(self) -> &'static str {
        self.direct_dependency
    }

    /// Returns the focused validation command.
    #[must_use]
    pub const fn focused_check(self) -> &'static str {
        self.focused_check
    }
}

const HOST_ADAPTER_DESCRIPTORS: [HostAdapterDescriptor; HOST_ADAPTER_DESCRIPTOR_COUNT] = [
    HostAdapterDescriptor {
        family: HostAdapterFamily::ConfigLoading,
        owner: "animus-std::config",
        feature_gate: "std",
        required_authority: ENVIRONMENT_AUTHORITY,
        facade_path: "animus_std::{RuntimeConfig, load_runtime}",
        direct_dependency: "animus-std",
        focused_check: "cargo test -p animus-std config --locked",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::LocalContext,
        owner: "animus-std::context",
        feature_gate: "std",
        required_authority: FILESYSTEM_AUTHORITY,
        facade_path: "animus_std::{LocalFileContextResolver, LocalDirectoryContextResolver}",
        direct_dependency: "animus-std",
        focused_check: "cargo test -p animus-std context --locked",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::ProviderRegistry,
        owner: "animus-std::providers",
        feature_gate: "std",
        required_authority: PROVIDER_AUTHORITY,
        facade_path: "animus_std::ProviderRegistry",
        direct_dependency: "animus-std",
        focused_check: "cargo test -p animus-std providers --locked",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::Observability,
        owner: "animus-std-observability",
        feature_gate: "std-observability",
        required_authority: NO_HOST_EFFECT,
        facade_path: "animus_std::{NoopObservabilitySink, InMemoryObservabilitySink}",
        direct_dependency: "animus-std-observability",
        focused_check: "cargo test -p animus-std-observability --locked",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::DocumentIngest,
        owner: "animus-std::document",
        feature_gate: "std",
        required_authority: INGEST_AUTHORITY,
        facade_path: "animus_std::{DocumentExtractionHost, plan_document_extraction}",
        direct_dependency: "animus-std",
        focused_check: "cargo test -p animus-std document --locked",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::ModelEvidenceIngest,
        owner: "animus-std::modelevidence",
        feature_gate: "std",
        required_authority: INGEST_AUTHORITY,
        facade_path: "animus_std::{ModelEvidenceIngestHost, plan_model_evidence_ingest}",
        direct_dependency: "animus-std",
        focused_check: "cargo test -p animus-std modelevidence --locked",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::IrohTransport,
        owner: "animus-std::iroh",
        feature_gate: "iroh-host",
        required_authority: TRANSPORT_AUTHORITY,
        facade_path: "animus_std::IrohAgentCommsAdapter",
        direct_dependency: "animus-std/iroh-host",
        focused_check: "cargo test -p animus-std --features iroh-host --locked",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::SyndicateTransport,
        owner: "animus-std::{dataspace,syndicate}",
        feature_gate: "syndicate-host",
        required_authority: TRANSPORT_AUTHORITY,
        facade_path: "animus_std::{SyndicateHostDataspaceAdapter,SyndicateProtocolAdapter}",
        direct_dependency: "animus-std/syndicate-host",
        focused_check: "cargo test -p animus-std --features syndicate-host --locked",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::IrohSyndicateTransport,
        owner: "animus-std::irohsyndicate",
        feature_gate: "iroh-syndicate-host",
        required_authority: TRANSPORT_AUTHORITY,
        facade_path: "animus_std::IrohSyndicatePacketTransport",
        direct_dependency: "animus-std/iroh-syndicate-host",
        focused_check: "cargo test -p animus-std --features iroh-syndicate-host irohsyndicate --locked",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::SandlockEnforcement,
        owner: "animus-std::sandlock_host",
        feature_gate: "sandlock-host",
        required_authority: SANDBOX_AUTHORITY,
        facade_path: "animus_std::sandlock_host",
        direct_dependency: "animus-std/sandlock-host",
        focused_check: "scripts/cairn/check-sandlock-host --root .",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::DurableSessionStore,
        owner: "animus-std::durablefile",
        feature_gate: "durable-session-store",
        required_authority: PERSISTENCE_AUTHORITY,
        facade_path: "animus_std::{DurableSessionLocalStore,open_durable_session_store}",
        direct_dependency: "animus-std/durable-session-store",
        focused_check: "scripts/cairn/check-durable-session-store --root .",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::CodingSessionHost,
        owner: "animus-std::coding_session_host",
        feature_gate: "coding-session-host",
        required_authority: CODING_SESSION_AUTHORITY,
        facade_path: "animus_std::coding_session_host::CodingSessionHost",
        direct_dependency: "animus-std/coding-session-host",
        focused_check: "scripts/cairn/check-coding-session-host --root .",
    },
    HostAdapterDescriptor {
        family: HostAdapterFamily::SmokeDemo,
        owner: "animus-std::dogfood_smoke",
        feature_gate: "dogfood-smoke",
        required_authority: NO_HOST_EFFECT,
        facade_path: "animus_std::run_packaged_headless_dogfood_smoke",
        direct_dependency: "animus-std/dogfood-smoke",
        focused_check: "scripts/cairn/check-packaged-headless-dogfood-smoke --root .",
    },
];

/// Returns the metadata-only std host adapter catalog.
#[must_use]
pub const fn host_adapter_catalog() -> &'static [HostAdapterDescriptor] {
    // r[impl agent_harness_runtime.std_host_adapter_crates.facade]
    // r[impl agent_harness_runtime.std_host_adapter_crates.authority]
    &HOST_ADAPTER_DESCRIPTORS
}

/// Explicit availability supplied by a host composition root.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostAdapterAvailability {
    /// The selected block is present and admitted.
    Available,
    /// The selected block was not composed into the host.
    Missing,
    /// The selected feature or block is disabled.
    Disabled,
    /// Host policy denied the selected block.
    Denied,
}

/// Whether an absent adapter requested ambient discovery.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostAdapterFallback {
    /// No ambient discovery was requested.
    NotRequested,
    /// Ambient env, filesystem, provider, network, or persistence discovery was requested.
    Requested,
}

/// Pure host adapter planning request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostAdapterPlanRequest {
    /// Requested adapter family.
    pub family: HostAdapterFamily,
    /// Explicit composition availability.
    pub availability: HostAdapterAvailability,
    /// Ambient fallback posture.
    pub fallback: HostAdapterFallback,
}

/// Pure host adapter plan status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostAdapterPlanStatus {
    /// The explicit block may be called by its owning shell.
    Admitted,
    /// The block must not be called.
    Denied,
}

/// First fail-closed host adapter planning issue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostAdapterPlanIssue {
    /// The requested block was missing.
    Missing,
    /// The requested block or feature was disabled.
    Disabled,
    /// Host policy denied the requested block.
    Denied,
    /// Ambient discovery or fallback was requested.
    AmbientFallbackDenied,
}

/// Pure host adapter plan produced before shell effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostAdapterPlan {
    family: HostAdapterFamily,
    status: HostAdapterPlanStatus,
    issue: Option<HostAdapterPlanIssue>,
}

impl HostAdapterPlan {
    /// Returns the requested family.
    #[must_use]
    pub const fn family(self) -> HostAdapterFamily {
        self.family
    }

    /// Returns admitted or denied status.
    #[must_use]
    pub const fn status(self) -> HostAdapterPlanStatus {
        self.status
    }

    /// Returns the first fail-closed issue.
    #[must_use]
    pub const fn issue(self) -> Option<HostAdapterPlanIssue> {
        self.issue
    }

    /// Returns whether the owning shell may call the explicit block.
    #[must_use]
    pub const fn is_admitted(self) -> bool {
        matches!(self.status, HostAdapterPlanStatus::Admitted)
    }
}

/// Plans one explicit host adapter call without performing host effects.
#[must_use]
pub const fn plan_host_adapter(request: HostAdapterPlanRequest) -> HostAdapterPlan {
    // r[impl agent_harness_runtime.std_host_adapter_crates.authority.no_fallback]
    let issue = plan_issue(request.availability, request.fallback);
    HostAdapterPlan {
        family: request.family,
        status: plan_status(issue),
        issue,
    }
}

const fn plan_issue(
    availability: HostAdapterAvailability,
    fallback: HostAdapterFallback,
) -> Option<HostAdapterPlanIssue> {
    if matches!(fallback, HostAdapterFallback::Requested) {
        return Some(HostAdapterPlanIssue::AmbientFallbackDenied);
    }
    match availability {
        HostAdapterAvailability::Available => None,
        HostAdapterAvailability::Missing => Some(HostAdapterPlanIssue::Missing),
        HostAdapterAvailability::Disabled => Some(HostAdapterPlanIssue::Disabled),
        HostAdapterAvailability::Denied => Some(HostAdapterPlanIssue::Denied),
    }
}

const fn plan_status(issue: Option<HostAdapterPlanIssue>) -> HostAdapterPlanStatus {
    if issue.is_none() {
        HostAdapterPlanStatus::Admitted
    } else {
        HostAdapterPlanStatus::Denied
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXPECTED_DESCRIPTOR_COUNT: usize = HOST_ADAPTER_DESCRIPTOR_COUNT;
    const EXPECTED_NEGATIVE_CASE_COUNT: usize = 4;

    #[test]
    fn catalog_entries_and_available_blocks_are_admitted() {
        // r[verify agent_harness_runtime.std_host_adapter_crates.authority]
        // r[verify agent_harness_runtime.std_host_adapter_crates.migration.pure_plans]
        let catalog = host_adapter_catalog();
        assert_eq!(catalog.len(), EXPECTED_DESCRIPTOR_COUNT);
        for descriptor in catalog {
            assert!(!descriptor.owner().is_empty());
            assert!(!descriptor.feature_gate().is_empty());
            assert!(!descriptor.required_authority().is_empty());
            assert!(!descriptor.facade_path().is_empty());
            assert!(!descriptor.direct_dependency().is_empty());
            assert!(!descriptor.focused_check().is_empty());
            let plan = plan_host_adapter(HostAdapterPlanRequest {
                family: descriptor.family(),
                availability: HostAdapterAvailability::Available,
                fallback: HostAdapterFallback::NotRequested,
            });
            assert!(plan.is_admitted());
            assert_eq!(plan.issue(), None);
        }
    }

    #[test]
    fn missing_disabled_denied_and_fallback_paths_fail_closed() {
        // r[verify agent_harness_runtime.std_host_adapter_crates.authority.no_fallback]
        // r[verify agent_harness_runtime.std_host_adapter_crates.verification]
        let cases = [
            (
                HostAdapterAvailability::Missing,
                HostAdapterFallback::NotRequested,
                HostAdapterPlanIssue::Missing,
            ),
            (
                HostAdapterAvailability::Disabled,
                HostAdapterFallback::NotRequested,
                HostAdapterPlanIssue::Disabled,
            ),
            (
                HostAdapterAvailability::Denied,
                HostAdapterFallback::NotRequested,
                HostAdapterPlanIssue::Denied,
            ),
            (
                HostAdapterAvailability::Available,
                HostAdapterFallback::Requested,
                HostAdapterPlanIssue::AmbientFallbackDenied,
            ),
        ];
        assert_eq!(cases.len(), EXPECTED_NEGATIVE_CASE_COUNT);
        for (availability, fallback, expected) in cases {
            let plan = plan_host_adapter(HostAdapterPlanRequest {
                family: HostAdapterFamily::ProviderRegistry,
                availability,
                fallback,
            });
            assert!(!plan.is_admitted());
            assert_eq!(plan.status(), HostAdapterPlanStatus::Denied);
            assert_eq!(plan.issue(), Some(expected));
        }
    }
}
