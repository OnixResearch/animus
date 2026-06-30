//! Preserves bridge for `animus` adapter boundary messages.
//!
//! This crate keeps Preserves support optional and separate from the `no_std`
//! core and adapter-contract crates. Use it when an embedder wants stable,
//! cross-language Preserves text or binary payloads at component boundaries.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

mod codec;
mod error;
mod message;
mod schema;

pub use codec::{
    from_bytes, from_record_bytes, from_record_text, from_text, from_versioned_bytes,
    from_versioned_text, to_bytes, to_record_bytes, to_record_text, to_text, to_versioned_bytes,
    to_versioned_text,
};
pub use message::{RuntimeFailureEnvelope, RuntimeReportStatus};
pub use schema::{
    BoundaryRecord, CompatibilityField, CoverageTier, SchemaDescriptor, SchemaFamily,
    SchemaIdentity, schema_catalog, schema_identity_for_message,
};

/// Error returned by Preserves bridge operations.
pub type CodecError = error::Failure;

/// Typed Preserves boundary message.
pub type BoundaryMessage = message::Envelope;

/// Boundary message kind.
pub type BoundaryKind = message::Kind;
