//! Shared, authority-free schema identities for Animus Preserves boundaries.
//!
//! This crate owns only deterministic schema metadata. Family bridge crates add
//! codecs and DTO validation without making runtime dependencies ambient.

/// Current supported version for every cataloged schema family.
pub const CURRENT_SCHEMA_VERSION: u16 = 1;
const SCHEMA_CATALOG_LEN: usize = 9;

const ADAPTER_OWNER: &str = "animus-preserves-adapter";
const RUNTIME_OWNER: &str = "animus-preserves::runtime";
const PROVENANCE_OWNER: &str = "animus-preserves::provenance";
const POLICY_OWNER: &str = "animus-preserves::policy";
const REVIEW_OWNER: &str = "animus-preserves::review";
const DATASPACE_OWNER: &str = "animus-preserves::dataspace";
const SYNDICATE_TURN_OWNER: &str = "animus-preserves::syndicate";

const ADAPTER_DEPENDENCIES: &[&str] = &[
    "animus-adapters",
    "animus-preserves-schema",
    "preserves",
    "serde",
];
const RUNTIME_DEPENDENCIES: &[&str] = &[
    "animus-adapters",
    "animus-preserves-adapter",
    "animus-preserves-schema",
    "animus-runtime",
    "preserves",
    "serde",
];
const PROVENANCE_DEPENDENCIES: &[&str] = &[
    "animus-preserves-schema",
    "animus-runtime",
    "preserves",
    "serde",
];
const POLICY_DEPENDENCIES: &[&str] = &[
    "animus-adapters",
    "animus-preserves-schema",
    "preserves",
    "serde",
];
const REVIEW_DEPENDENCIES: &[&str] = &["animus-preserves-schema", "preserves", "serde"];
const DATASPACE_DEPENDENCIES: &[&str] = &[
    "animus-adapters",
    "animus-preserves-schema",
    "preserves",
    "serde",
];
const SYNDICATE_TURN_DEPENDENCIES: &[&str] = &[
    "animus-adapters",
    "animus-preserves-schema",
    "preserves",
    "serde",
];

const BOUNDED_SAFE_METADATA_POLICY: &str = "bounded-safe-identifiers-and-values; no raw prompts, provider payloads, credentials, private paths, file bodies, or unbounded output";
const REVIEW_METADATA_POLICY: &str =
    "review fixtures only; no compatibility import or external wire claim";

/// Compatibility coverage level for a Preserves payload family.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CoverageTier {
    /// Internal typed DTO round trips without external compatibility promises.
    InternalDtoRoundTrip,
    /// Stable external wire contract with versioning and positive/negative fixtures.
    StableExternalWireContract,
    /// Import/export adapter where compatibility handling is explicit and bounded.
    CompatibilityImportExportAdapter,
    /// Review-only fixture shape that documents intent without publishing a wire contract.
    ReviewOnlyFixture,
}

impl CoverageTier {
    /// Returns the stable tier label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InternalDtoRoundTrip => "internal_dto_round_trip",
            Self::StableExternalWireContract => "stable_external_wire_contract",
            Self::CompatibilityImportExportAdapter => "compatibility_import_export_adapter",
            Self::ReviewOnlyFixture => "review_only_fixture",
        }
    }

    /// Returns whether this tier requires stable schema-version fixture evidence.
    #[must_use]
    pub const fn requires_stable_fixtures(self) -> bool {
        match self {
            Self::StableExternalWireContract | Self::CompatibilityImportExportAdapter => true,
            Self::InternalDtoRoundTrip | Self::ReviewOnlyFixture => false,
        }
    }
}

/// Stable schema family identifier for Preserves boundary payloads.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[repr(usize)]
#[serde(rename_all = "snake_case")]
pub enum SchemaFamily {
    /// Adapter request/response boundary messages.
    AdapterMessage,
    /// Runtime event boundary messages.
    RuntimeEvent,
    /// Final runtime report boundary messages.
    RuntimeReport,
    /// Error/status payloads embedded in stable reports or compatibility adapters.
    Error,
    /// Session/provenance compatibility records.
    SessionProvenanceRecord,
    /// Policy-visible compatibility DTOs.
    PolicyVisibleDto,
    /// Dataset/export rows retained as review fixtures until exporters stabilize.
    DatasetExportRow,
    /// Dataspace scope, assertion, subscription, and orchestration DTO records.
    DataspaceDto,
    /// Capability-free Syndicate Protocol turn packets over dataspace records.
    SyndicateTurn,
}

impl SchemaFamily {
    /// Returns the stable schema-family label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AdapterMessage => "adapter_message",
            Self::RuntimeEvent => "runtime_event",
            Self::RuntimeReport => "runtime_report",
            Self::Error => "error",
            Self::SessionProvenanceRecord => "session_provenance_record",
            Self::PolicyVisibleDto => "policy_visible_dto",
            Self::DatasetExportRow => "dataset_export_row",
            Self::DataspaceDto => "dataspace_dto",
            Self::SyndicateTurn => "syndicate_turn",
        }
    }

    /// Returns the declared compatibility tier.
    #[must_use]
    pub const fn coverage_tier(self) -> CoverageTier {
        match self {
            Self::AdapterMessage
            | Self::RuntimeEvent
            | Self::RuntimeReport
            | Self::DataspaceDto
            | Self::SyndicateTurn => CoverageTier::StableExternalWireContract,
            Self::Error | Self::SessionProvenanceRecord | Self::PolicyVisibleDto => {
                CoverageTier::CompatibilityImportExportAdapter
            }
            Self::DatasetExportRow => CoverageTier::ReviewOnlyFixture,
        }
    }
}

/// Versioned schema identity bound to one boundary payload family.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct SchemaIdentity {
    family: SchemaFamily,
    version: u16,
}

impl SchemaIdentity {
    /// Builds an explicit schema identity.
    #[must_use]
    pub const fn new(family: SchemaFamily, version: u16) -> Self {
        Self { family, version }
    }

    /// Builds the current schema identity for a family.
    #[must_use]
    pub const fn current(family: SchemaFamily) -> Self {
        Self::new(family, CURRENT_SCHEMA_VERSION)
    }

    /// Returns the schema family.
    #[must_use]
    pub const fn family(self) -> SchemaFamily {
        self.family
    }

    /// Returns the schema version.
    #[must_use]
    pub const fn version(self) -> u16 {
        self.version
    }

    /// Returns the coverage tier for this identity family.
    #[must_use]
    pub const fn coverage_tier(self) -> CoverageTier {
        self.family.coverage_tier()
    }

    /// Validates that this schema identity uses the supported version.
    ///
    /// # Errors
    ///
    /// Returns [`SchemaVersionError`] when the version is unsupported.
    pub const fn validate_supported(self) -> Result<(), SchemaVersionError> {
        if self.version == CURRENT_SCHEMA_VERSION {
            return Ok(());
        }
        Err(SchemaVersionError {
            family: self.family,
            version: self.version,
            supported: CURRENT_SCHEMA_VERSION,
        })
    }
}

/// Unsupported schema-version details without raw payload material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaVersionError {
    family: SchemaFamily,
    version: u16,
    supported: u16,
}

impl SchemaVersionError {
    /// Returns the affected schema family.
    #[must_use]
    pub const fn family(self) -> SchemaFamily {
        self.family
    }

    /// Returns the unsupported version.
    #[must_use]
    pub const fn version(self) -> u16 {
        self.version
    }

    /// Returns the supported version.
    #[must_use]
    pub const fn supported(self) -> u16 {
        self.supported
    }
}

/// Published wire-stability posture for one schema family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WireStatus {
    /// A versioned external text and binary contract is published.
    Stable,
    /// A bounded compatibility import/export adapter is published.
    Compatibility,
    /// Only review fixtures are published.
    ReviewOnly,
}

impl WireStatus {
    /// Returns the stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stable => "stable",
            Self::Compatibility => "compatibility",
            Self::ReviewOnly => "review_only",
        }
    }
}

/// Catalog entry declaring one Preserves boundary schema family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaDescriptor {
    identity: SchemaIdentity,
    coverage_tier: CoverageTier,
    owner: &'static str,
    dependencies: &'static [&'static str],
    safe_metadata_policy: &'static str,
    wire_status: WireStatus,
    fixture_set: &'static str,
}

impl SchemaDescriptor {
    /// Builds a complete schema catalog descriptor.
    #[must_use]
    pub const fn new(family: SchemaFamily) -> Self {
        Self {
            identity: SchemaIdentity::current(family),
            coverage_tier: family.coverage_tier(),
            owner: owner(family),
            dependencies: dependencies(family),
            safe_metadata_policy: metadata_policy(family),
            wire_status: wire_status(family),
            fixture_set: fixture_set(family),
        }
    }

    /// Returns the versioned schema identity.
    #[must_use]
    pub const fn identity(self) -> SchemaIdentity {
        self.identity
    }

    /// Returns the declared coverage tier.
    #[must_use]
    pub const fn coverage_tier(self) -> CoverageTier {
        self.coverage_tier
    }

    /// Returns the crate or module that owns this family boundary.
    #[must_use]
    pub const fn owner(self) -> &'static str {
        self.owner
    }

    /// Returns the explicit dependency set for the owning boundary.
    #[must_use]
    pub const fn dependencies(self) -> &'static [&'static str] {
        self.dependencies
    }

    /// Returns the safe-metadata handling policy.
    #[must_use]
    pub const fn safe_metadata_policy(self) -> &'static str {
        self.safe_metadata_policy
    }

    /// Returns the stable-wire publication status.
    #[must_use]
    pub const fn wire_status(self) -> WireStatus {
        self.wire_status
    }

    /// Returns the positive/negative fixture-set identifier.
    #[must_use]
    pub const fn fixture_set(self) -> &'static str {
        self.fixture_set
    }
}

const fn owner(family: SchemaFamily) -> &'static str {
    match family {
        SchemaFamily::AdapterMessage => ADAPTER_OWNER,
        SchemaFamily::RuntimeEvent | SchemaFamily::RuntimeReport | SchemaFamily::Error => {
            RUNTIME_OWNER
        }
        SchemaFamily::SessionProvenanceRecord => PROVENANCE_OWNER,
        SchemaFamily::PolicyVisibleDto => POLICY_OWNER,
        SchemaFamily::DatasetExportRow => REVIEW_OWNER,
        SchemaFamily::DataspaceDto => DATASPACE_OWNER,
        SchemaFamily::SyndicateTurn => SYNDICATE_TURN_OWNER,
    }
}

const fn dependencies(family: SchemaFamily) -> &'static [&'static str] {
    match family {
        SchemaFamily::AdapterMessage => ADAPTER_DEPENDENCIES,
        SchemaFamily::RuntimeEvent | SchemaFamily::RuntimeReport | SchemaFamily::Error => {
            RUNTIME_DEPENDENCIES
        }
        SchemaFamily::SessionProvenanceRecord => PROVENANCE_DEPENDENCIES,
        SchemaFamily::PolicyVisibleDto => POLICY_DEPENDENCIES,
        SchemaFamily::DatasetExportRow => REVIEW_DEPENDENCIES,
        SchemaFamily::DataspaceDto => DATASPACE_DEPENDENCIES,
        SchemaFamily::SyndicateTurn => SYNDICATE_TURN_DEPENDENCIES,
    }
}

const fn metadata_policy(family: SchemaFamily) -> &'static str {
    match family {
        SchemaFamily::DatasetExportRow => REVIEW_METADATA_POLICY,
        SchemaFamily::AdapterMessage
        | SchemaFamily::RuntimeEvent
        | SchemaFamily::RuntimeReport
        | SchemaFamily::Error
        | SchemaFamily::SessionProvenanceRecord
        | SchemaFamily::PolicyVisibleDto
        | SchemaFamily::DataspaceDto
        | SchemaFamily::SyndicateTurn => BOUNDED_SAFE_METADATA_POLICY,
    }
}

const fn wire_status(family: SchemaFamily) -> WireStatus {
    match family.coverage_tier() {
        CoverageTier::StableExternalWireContract => WireStatus::Stable,
        CoverageTier::CompatibilityImportExportAdapter => WireStatus::Compatibility,
        CoverageTier::InternalDtoRoundTrip | CoverageTier::ReviewOnlyFixture => {
            WireStatus::ReviewOnly
        }
    }
}

const fn fixture_set(family: SchemaFamily) -> &'static str {
    match family {
        SchemaFamily::AdapterMessage => "preserves.adapter.v1",
        SchemaFamily::RuntimeEvent => "preserves.runtime-event.v1",
        SchemaFamily::RuntimeReport => "preserves.runtime-report.v1",
        SchemaFamily::Error => "preserves.error.compat.v1",
        SchemaFamily::SessionProvenanceRecord => "preserves.session-provenance.compat.v1",
        SchemaFamily::PolicyVisibleDto => "preserves.policy-visible.compat.v1",
        SchemaFamily::DatasetExportRow => "preserves.dataset.review.v1",
        SchemaFamily::DataspaceDto => "preserves.dataspace.v1",
        SchemaFamily::SyndicateTurn => "preserves.syndicate-turn.v1",
    }
}

const SCHEMA_CATALOG: [SchemaDescriptor; SCHEMA_CATALOG_LEN] = [
    SchemaDescriptor::new(SchemaFamily::AdapterMessage),
    SchemaDescriptor::new(SchemaFamily::RuntimeEvent),
    SchemaDescriptor::new(SchemaFamily::RuntimeReport),
    SchemaDescriptor::new(SchemaFamily::Error),
    SchemaDescriptor::new(SchemaFamily::SessionProvenanceRecord),
    SchemaDescriptor::new(SchemaFamily::PolicyVisibleDto),
    SchemaDescriptor::new(SchemaFamily::DatasetExportRow),
    SchemaDescriptor::new(SchemaFamily::DataspaceDto),
    SchemaDescriptor::new(SchemaFamily::SyndicateTurn),
];

/// Returns the declared Preserves schema catalog.
#[must_use]
pub const fn schema_catalog() -> &'static [SchemaDescriptor] {
    &SCHEMA_CATALOG
}

#[cfg(test)]
mod tests {
    use super::{CoverageTier, SchemaFamily, SchemaIdentity, WireStatus, schema_catalog};

    const UNSUPPORTED_SCHEMA_VERSION: u16 = 2;

    #[test]
    fn catalog_declares_owner_dependencies_policy_status_and_fixtures() {
        let catalog = schema_catalog();
        let adapter = catalog[SchemaFamily::AdapterMessage as usize];

        assert_eq!(
            adapter.identity(),
            SchemaIdentity::current(SchemaFamily::AdapterMessage)
        );
        assert_eq!(adapter.owner(), "animus-preserves-adapter");
        assert!(adapter.dependencies().contains(&"animus-adapters"));
        assert!(!adapter.dependencies().contains(&"animus-runtime"));
        assert!(!adapter.safe_metadata_policy().is_empty());
        assert_eq!(adapter.wire_status(), WireStatus::Stable);
        assert_eq!(adapter.fixture_set(), "preserves.adapter.v1");
        let syndicate = catalog[SchemaFamily::SyndicateTurn as usize];
        assert_eq!(syndicate.owner(), "animus-preserves::syndicate");
        assert_eq!(syndicate.wire_status(), WireStatus::Stable);
        assert_eq!(syndicate.fixture_set(), "preserves.syndicate-turn.v1");
        for descriptor in catalog {
            assert!(!descriptor.owner().is_empty());
            assert!(!descriptor.dependencies().is_empty());
            assert!(!descriptor.safe_metadata_policy().is_empty());
            assert!(!descriptor.fixture_set().is_empty());
        }
    }

    #[test]
    fn unsupported_versions_and_review_only_claims_fail_closed() {
        let identity =
            SchemaIdentity::new(SchemaFamily::AdapterMessage, UNSUPPORTED_SCHEMA_VERSION);
        let error = identity
            .validate_supported()
            .expect_err("unsupported schema version must fail");
        let review = schema_catalog()[SchemaFamily::DatasetExportRow as usize];

        assert_eq!(error.version(), UNSUPPORTED_SCHEMA_VERSION);
        let compatibility = schema_catalog()[SchemaFamily::SessionProvenanceRecord as usize];

        assert_eq!(review.coverage_tier(), CoverageTier::ReviewOnlyFixture);
        assert_eq!(review.wire_status(), WireStatus::ReviewOnly);
        assert_eq!(compatibility.wire_status(), WireStatus::Compatibility);
        assert!(!review.coverage_tier().requires_stable_fixtures());
    }
}
