//! Authority-scoped standard-library observability sinks.
//!
//! This crate implements deterministic no-op and in-memory sinks over the
//! implementation-free `animus-adapter-observability` contracts. It owns no
//! filesystem, process, network, credential, terminal, clock, global subscriber,
//! or persistence authority.

#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

/// No-op observability sink that accepts validated records and drops them.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NoopSink;

impl animus_adapter_observability::MetricsAdapter for NoopSink {
    fn emit_metric(
        &mut self,
        _sample: &animus_adapter_observability::MetricSample,
    ) -> Result<animus_adapter_observability::SinkOutcome, animus_adapter_observability::Error>
    {
        Ok(animus_adapter_observability::SinkOutcome::Delivered)
    }
}

impl animus_adapter_observability::LoggingAdapter for NoopSink {
    fn emit_log(
        &mut self,
        _record: &animus_adapter_observability::LogRecord,
    ) -> Result<animus_adapter_observability::SinkOutcome, animus_adapter_observability::Error>
    {
        Ok(animus_adapter_observability::SinkOutcome::Delivered)
    }
}

/// Deterministic in-memory observability sink.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InMemorySink {
    metrics: Vec<animus_adapter_observability::MetricSample>,
    logs: Vec<animus_adapter_observability::LogRecord>,
    state: SinkState,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum SinkState {
    #[default]
    Open,
    Unavailable,
    Closed,
}

impl InMemorySink {
    /// Builds an open in-memory sink.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            metrics: Vec::new(),
            logs: Vec::new(),
            state: SinkState::Open,
        }
    }

    /// Builds a sink that reports unavailable without ambient fallback.
    #[must_use]
    pub const fn unavailable() -> Self {
        Self {
            metrics: Vec::new(),
            logs: Vec::new(),
            state: SinkState::Unavailable,
        }
    }

    /// Closes the sink to model a collector shutdown.
    pub fn close(&mut self) {
        self.state = SinkState::Closed;
    }

    /// Returns collected metric samples.
    #[must_use]
    pub fn metrics(&self) -> &[animus_adapter_observability::MetricSample] {
        self.metrics.as_slice()
    }

    /// Returns collected log records.
    #[must_use]
    pub fn logs(&self) -> &[animus_adapter_observability::LogRecord] {
        self.logs.as_slice()
    }

    fn validate_state(&self) -> Result<(), animus_adapter_observability::Error> {
        match self.state {
            SinkState::Open => Ok(()),
            SinkState::Unavailable => Err(animus_adapter_observability::Error::SinkUnavailable),
            SinkState::Closed => Err(animus_adapter_observability::Error::CollectorClosed),
        }
    }
}

impl animus_adapter_observability::MetricsAdapter for InMemorySink {
    fn emit_metric(
        &mut self,
        sample: &animus_adapter_observability::MetricSample,
    ) -> Result<animus_adapter_observability::SinkOutcome, animus_adapter_observability::Error>
    {
        self.validate_state()?;
        self.metrics.push(sample.clone());
        Ok(animus_adapter_observability::SinkOutcome::Delivered)
    }
}

impl animus_adapter_observability::LoggingAdapter for InMemorySink {
    fn emit_log(
        &mut self,
        record: &animus_adapter_observability::LogRecord,
    ) -> Result<animus_adapter_observability::SinkOutcome, animus_adapter_observability::Error>
    {
        self.validate_state()?;
        self.logs.push(record.clone());
        Ok(animus_adapter_observability::SinkOutcome::Delivered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapter_observability::{LoggingAdapter, MetricsAdapter};

    const METRIC_NAME: &str = "runtime.events";
    const LOG_TARGET: &str = "runtime.event";
    const EVENT_KIND_KEY: &str = "event_kind";
    const EVENT_KIND_VALUE: &str = "runtime_started";
    const MESSAGE: &str = "runtime_started";
    const PRIVATE_PATH: &str = "/home/example/private.txt";
    const METRIC_VALUE: u64 = 1;
    const FIRST_RECORD_COUNT: usize = 1;

    #[test]
    fn direct_sinks_accept_validated_records_and_preserve_facade_behavior()
    -> Result<(), animus_adapter_observability::Error> {
        // r[verify agent_harness_runtime.std_host_adapter_crates.migration.pure_plans]
        // r[verify agent_harness_runtime.std_host_adapter_crates.verification]
        let metric = metric()?;
        let log = log()?;
        let mut noop = NoopSink;
        assert!(noop.emit_metric(&metric)?.is_delivered());
        assert!(noop.emit_log(&log)?.is_delivered());

        let mut memory = InMemorySink::new();
        assert!(memory.emit_metric(&metric)?.is_delivered());
        assert!(memory.emit_log(&log)?.is_delivered());
        assert_eq!(memory.metrics().len(), FIRST_RECORD_COUNT);
        assert_eq!(memory.logs().len(), FIRST_RECORD_COUNT);
        Ok(())
    }

    #[test]
    fn missing_closed_and_unsafe_observability_paths_fail_without_fallback()
    -> Result<(), animus_adapter_observability::Error> {
        // r[verify agent_harness_runtime.std_host_adapter_crates.authority.no_fallback]
        // r[verify agent_harness_runtime.std_host_adapter_crates.verification]
        let metric = metric()?;
        let log = log()?;
        let mut unavailable = InMemorySink::unavailable();
        assert_eq!(
            unavailable.emit_metric(&metric),
            Err(animus_adapter_observability::Error::SinkUnavailable)
        );

        let mut closed = InMemorySink::new();
        closed.close();
        assert_eq!(
            closed.emit_log(&log),
            Err(animus_adapter_observability::Error::CollectorClosed)
        );
        assert!(matches!(
            animus_adapter_observability::LogRecord::new(
                animus_adapter_observability::LogRecordDraft::new(
                    LOG_TARGET,
                    animus_adapter_observability::LogLevel::Error,
                    PRIVATE_PATH,
                    &[label()],
                ),
            ),
            Err(animus_adapter_observability::Error::UnsafeMetadata(_))
        ));
        Ok(())
    }

    fn metric()
    -> Result<animus_adapter_observability::MetricSample, animus_adapter_observability::Error> {
        animus_adapter_observability::MetricSample::new(
            animus_adapter_observability::MetricSampleDraft {
                name: METRIC_NAME,
                kind: animus_adapter_observability::MetricKind::Counter,
                unit: animus_adapter_observability::MetricUnit::Count,
                value: METRIC_VALUE,
                labels: &[label()],
            },
        )
    }

    fn log() -> Result<animus_adapter_observability::LogRecord, animus_adapter_observability::Error>
    {
        animus_adapter_observability::LogRecord::new(
            animus_adapter_observability::LogRecordDraft::new(
                LOG_TARGET,
                animus_adapter_observability::LogLevel::Info,
                MESSAGE,
                &[label()],
            ),
        )
    }

    const fn label() -> animus_adapter_observability::MetricLabelDraft<'static> {
        animus_adapter_observability::MetricLabelDraft::new(EVENT_KIND_KEY, EVENT_KIND_VALUE)
    }
}
