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

pub use codec::{from_bytes, from_text, to_bytes, to_text};

/// Error returned by Preserves bridge operations.
pub type CodecError = error::Failure;

/// Typed Preserves boundary message.
pub type BoundaryMessage = message::Envelope;

/// Boundary message kind.
pub type BoundaryKind = message::Kind;
