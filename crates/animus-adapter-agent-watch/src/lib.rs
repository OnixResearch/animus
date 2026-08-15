#![no_std]
#![forbid(unsafe_code)]
//! Pure, session-local, acyclic agent watch-graph contracts for Animus.
//!
//! This family owns typed watch edges, atomic cycle-closing rejection, and a
//! watcher-only redacted status projection. It contacts no processes, sockets,
//! persistence, or providers. Hosts own messaging, retry execution, and
//! dispatch.

extern crate alloc;

mod graph;
mod receipt;
mod status;
mod validate;

pub use graph::*;
pub use receipt::*;
pub use status::*;
pub use validate::*;

/// Watch-graph schema identity.
pub const WATCH_SCHEMA: &str = "animus.agent-watch.v1";
/// Watch receipt domain label.
pub const WATCH_RECEIPT_DOMAIN: &str = "animus/agent-watch-receipt";
/// Maximum safe agent-label length in bytes.
pub const MAX_WATCH_LABEL_BYTES: usize = 160;

pub(crate) fn hash_parts(domain: &str, parts: &[&str]) -> alloc::string::String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(domain.as_bytes());
    for part in parts {
        hasher.update(&[0]);
        hasher.update(part.as_bytes());
    }
    alloc::format!("b3:{}", hasher.finalize().to_hex())
}

#[cfg(test)]
mod tests;
