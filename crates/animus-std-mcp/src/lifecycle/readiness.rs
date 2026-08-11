use super::{HostReadiness, LifecyclePhase, ServerReadinessEntry};
use crate::ServerPosture;

/// Reduces explicitly configured required and optional server outcomes.
#[must_use]
pub fn reduce_host_readiness(entries: &[ServerReadinessEntry<'_>]) -> HostReadiness {
    let mut required_ready = true;
    let mut required_failure_count = 0_usize;
    let mut optional_unavailable_count = 0_usize;
    for entry in entries {
        match (entry.posture, entry.state.phase()) {
            (ServerPosture::Required | ServerPosture::Optional, LifecyclePhase::Ready) => {}
            (ServerPosture::Required, _) => {
                required_ready = false;
                required_failure_count = required_failure_count.saturating_add(1);
            }
            (ServerPosture::Optional, _) => {
                optional_unavailable_count = optional_unavailable_count.saturating_add(1);
            }
        }
    }
    HostReadiness {
        required_ready,
        required_failure_count,
        optional_unavailable_count,
    }
}
