//! Compatibility facade for family-specific Animus Preserves boundaries.
//!
//! Adapter-only consumers should depend on `animus-preserves-adapter` directly.
//! This broader facade preserves existing imports and owns runtime, dataspace,
//! provenance/session, and policy-visible compatibility boundaries.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod codec;
mod dataspace;
mod error;
mod family_codec;
mod message;
mod schema;

/// Policy-visible compatibility records with owning DTO revalidation.
pub mod policy;
/// Session/provenance compatibility records with bounded safe metadata.
pub mod provenance;
/// Review-only fixture declarations without stable wire claims.
pub mod review;

/// Minimal adapter-only boundary, re-exported for facade compatibility.
pub mod adapter {
    pub use animus_preserves_adapter::*;
}

pub use codec::{
    from_bytes, from_record_bytes, from_record_text, from_text, from_versioned_bytes,
    from_versioned_text, to_bytes, to_record_bytes, to_record_text, to_text, to_versioned_bytes,
    to_versioned_text,
};
pub use dataspace::{
    DataspaceMessage, DataspacePayloadKind, DataspaceRecord, DataspaceScopePayload,
    from_dataspace_record_bytes, from_dataspace_record_text, to_dataspace_record_bytes,
    to_dataspace_record_text,
};
pub use message::{RuntimeFailureEnvelope, RuntimeReportStatus};
pub use schema::{
    BoundaryRecord, CompatibilityField, CoverageTier, SchemaDescriptor, SchemaFamily,
    SchemaIdentity, WireStatus, schema_catalog, schema_identity_for_message,
};

/// Error returned by Preserves bridge operations.
pub type CodecError = error::Failure;

/// Typed Preserves boundary message.
pub type BoundaryMessage = message::Envelope;

/// Boundary message kind.
pub type BoundaryKind = message::Kind;

/// Runtime event/report compatibility surface retained by the broad facade.
pub mod runtime {
    pub use crate::{
        BoundaryKind, BoundaryMessage, BoundaryRecord, CodecError, RuntimeFailureEnvelope,
        RuntimeReportStatus, from_bytes, from_record_bytes, from_record_text, from_text,
        from_versioned_bytes, from_versioned_text, to_bytes, to_record_bytes, to_record_text,
        to_text, to_versioned_bytes, to_versioned_text,
    };
}
