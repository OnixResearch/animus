//! Compatibility shim for authority-scoped std observability sinks.

pub use animus_std_observability::{InMemorySink, NoopSink};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn facade_aliases_resolve_to_direct_sink_types() {
        // r[verify agent_harness_runtime.std_host_adapter_crates.facade.compat]
        assert_eq!(
            core::any::type_name::<InMemorySink>(),
            core::any::type_name::<animus_std_observability::InMemorySink>()
        );
        assert_eq!(
            core::any::type_name::<NoopSink>(),
            core::any::type_name::<animus_std_observability::NoopSink>()
        );
    }
}
