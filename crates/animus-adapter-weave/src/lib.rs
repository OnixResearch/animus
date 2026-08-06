//! Optional `no_std` adapter that wraps the `universal-weave` DAG weave behind
//! Animus-owned DTO contracts, named bounds, and metadata-only BLAKE3 receipts.
//!
//! The adapter models mutable branching workspaces: hosts can add, edit,
//! split, merge, move, bookmark, activate, and remove text nodes. The neutral
//! Animus session ledger remains the sole owner of append-only replay and
//! persistence semantics. This crate never reads files, inspects the
//! environment, spawns processes, accesses clocks, or performs network I/O.

#![no_std]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

mod adapter;
mod content;
mod error;
mod policy;
mod receipt;
mod snapshot;

pub use adapter::WeaveAdapter;
pub use content::WeaveNodeContent;
pub use error::WeaveAdapterError;
pub use policy::WeaveAdapterPolicy;
pub use receipt::{WeaveCounts, WeaveOperationKind, WeaveOperationReceipt};
pub use snapshot::{WeaveSnapshot, WeaveSnapshotEntry};
