//! Deterministic production-path conformance for native Animus evaluation.
//!
//! The crate parses checked fixture metadata, invokes public evaluation APIs with
//! finite scripted fakes, and emits metadata-only summaries. It owns no live
//! provider, tool, process, socket, filesystem, credential, clock, scheduler,
//! persistence, benchmark, Verifiers, Python, container, or cloud authority.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(
    clippy::module_name_repetitions,
    clippy::struct_excessive_bools,
    clippy::struct_field_names,
    clippy::too_many_arguments
)]
#![forbid(unsafe_code)]

mod harness;
mod host_fixture;
mod profile;

pub use harness::*;
pub use profile::*;

/// Native evaluation conformance schema.
pub const CONFORMANCE_SCHEMA: &str = "animus.native-evaluation-conformance.v1";
/// Stable conformance profile label.
pub const CONFORMANCE_PROFILE_ID: &str = "native-evaluation-composition-v1";
/// Reviewed Verifiers revision used only for scenario inspiration.
pub const VERIFIERS_CONFORMANCE_REVISION: &str = "1a997814e85a2e4f409add960aa852737388dbd8";
/// Reviewed source cohort identity.
pub const VERIFIERS_CONFORMANCE_COHORT_IDENTITY: &str =
    "b3:c6336b188596db9b18cb2f0a721c9f5bc551e619342e89807e20ec45f3bb676f";
/// Maximum registered conformance fixtures.
pub const MAX_CONFORMANCE_FIXTURES: u32 = 64;
/// Maximum deterministic fake service classes.
pub const MAX_CONFORMANCE_SERVICES: u32 = 16;

/// Compute one domain-separated BLAKE3 identity from framed string parts.
#[must_use]
pub fn hash_parts(domain: &str, parts: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    frame(&mut hasher, domain.as_bytes());
    for part in parts {
        frame(&mut hasher, part.as_bytes());
    }
    format!("b3:{}", hasher.finalize().to_hex())
}

fn frame(hasher: &mut blake3::Hasher, value: &[u8]) {
    let length = u64::try_from(value.len()).expect("bounded identity input fits u64");
    hasher.update(&length.to_le_bytes());
    hasher.update(value);
}
