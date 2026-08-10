//! Non-default transactional daemon-handoff host over injected services.

#[path = "host.rs"]
mod orchestrator;
#[path = "fixture.rs"]
mod test_double;
#[path = "smoke.rs"]
mod validation_driver;

/// Versioned handoff DTOs and implementation-free service ports.
pub mod adapter {
    pub use animus_adapters::daemon_workers::*;
}
/// Pure strict-idle and transactional handoff reducers.
pub mod core {
    pub use animus_runtime::daemon_handoff::*;
}
pub use orchestrator::{HandoffHost, HandoffHostError, HandoffHostOutcome, HandoffRunRequest};
pub use validation_driver::{HandoffSmokeReceipt, run_fake_handoff_smoke};

#[cfg(test)]
mod tests;
