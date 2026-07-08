//! Standard-library observability sinks for deterministic embedding and tests.

/// No-op observability sink that accepts safe records and drops them.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct NoopSink;

impl animus_adapters::MetricsAdapter for NoopSink {
    fn emit_metric(
        &mut self,
        _sample: &animus_adapters::MetricSample,
    ) -> Result<animus_adapters::ObservabilitySinkOutcome, animus_adapters::ObservabilityError>
    {
        Ok(animus_adapters::ObservabilitySinkOutcome::Delivered)
    }
}

impl animus_adapters::LoggingAdapter for NoopSink {
    fn emit_log(
        &mut self,
        _record: &animus_adapters::LogRecord,
    ) -> Result<animus_adapters::ObservabilitySinkOutcome, animus_adapters::ObservabilityError>
    {
        Ok(animus_adapters::ObservabilitySinkOutcome::Delivered)
    }
}

/// Deterministic in-memory observability sink.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InMemorySink {
    metrics: Vec<animus_adapters::MetricSample>,
    logs: Vec<animus_adapters::LogRecord>,
    available: bool,
    closed: bool,
}

impl InMemorySink {
    /// Builds an open in-memory sink.
    #[must_use]
    pub fn new() -> Self {
        Self {
            metrics: Vec::new(),
            logs: Vec::new(),
            available: true,
            closed: false,
        }
    }

    /// Builds a sink that reports unavailable.
    #[must_use]
    pub fn unavailable() -> Self {
        Self {
            available: false,
            ..Self::new()
        }
    }

    /// Closes the sink to simulate a collector shutdown.
    pub fn close(&mut self) {
        self.closed = true;
    }

    /// Returns collected metric samples.
    #[must_use]
    pub fn metrics(&self) -> &[animus_adapters::MetricSample] {
        self.metrics.as_slice()
    }

    /// Returns collected log records.
    #[must_use]
    pub fn logs(&self) -> &[animus_adapters::LogRecord] {
        self.logs.as_slice()
    }

    fn ensure_open(&self) -> Result<(), animus_adapters::ObservabilityError> {
        if !self.available {
            return Err(animus_adapters::ObservabilityError::SinkUnavailable);
        }
        if self.closed {
            return Err(animus_adapters::ObservabilityError::CollectorClosed);
        }
        Ok(())
    }
}

impl animus_adapters::MetricsAdapter for InMemorySink {
    fn emit_metric(
        &mut self,
        sample: &animus_adapters::MetricSample,
    ) -> Result<animus_adapters::ObservabilitySinkOutcome, animus_adapters::ObservabilityError>
    {
        self.ensure_open()?;
        self.metrics.push(sample.clone());
        Ok(animus_adapters::ObservabilitySinkOutcome::Delivered)
    }
}

impl animus_adapters::LoggingAdapter for InMemorySink {
    fn emit_log(
        &mut self,
        record: &animus_adapters::LogRecord,
    ) -> Result<animus_adapters::ObservabilitySinkOutcome, animus_adapters::ObservabilityError>
    {
        self.ensure_open()?;
        self.logs.push(record.clone());
        Ok(animus_adapters::ObservabilitySinkOutcome::Delivered)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::{LoggingAdapter, MetricsAdapter};

    const METRIC_NAME: &str = "runtime.events";
    const LOG_TARGET: &str = "runtime.event";
    const EVENT_KIND_KEY: &str = "event_kind";
    const EVENT_KIND_VALUE: &str = "runtime_started";
    const MESSAGE: &str = "runtime_started";
    const METRIC_VALUE: u64 = 1;
    const PRIVATE_PATH: &str = "/home/example/private.txt";
    const FIRST_RECORD_COUNT: usize = 1;

    #[test]
    fn sinks_accept_safe_records() -> Result<(), animus_adapters::ObservabilityError> {
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
    fn in_memory_sink_reports_unavailable_closed_and_unsafe_records()
    -> Result<(), animus_adapters::ObservabilityError> {
        let metric = metric()?;
        let log = log()?;
        let mut unavailable = InMemorySink::unavailable();
        assert_eq!(
            unavailable.emit_metric(&metric),
            Err(animus_adapters::ObservabilityError::SinkUnavailable)
        );

        let mut closed = InMemorySink::new();
        closed.close();
        assert_eq!(
            closed.emit_log(&log),
            Err(animus_adapters::ObservabilityError::CollectorClosed)
        );
        assert!(matches!(
            animus_adapters::LogRecord::new(animus_adapters::LogRecordDraft::new(
                LOG_TARGET,
                animus_adapters::LogLevel::Error,
                PRIVATE_PATH,
                &[label()],
            )),
            Err(animus_adapters::ObservabilityError::UnsafeMetadata(_))
        ));
        Ok(())
    }

    fn metric() -> Result<animus_adapters::MetricSample, animus_adapters::ObservabilityError> {
        animus_adapters::MetricSample::new(animus_adapters::MetricSampleDraft {
            name: METRIC_NAME,
            kind: animus_adapters::MetricKind::Counter,
            unit: animus_adapters::MetricUnit::Count,
            value: METRIC_VALUE,
            labels: &[label()],
        })
    }

    fn log() -> Result<animus_adapters::LogRecord, animus_adapters::ObservabilityError> {
        animus_adapters::LogRecord::new(animus_adapters::LogRecordDraft::new(
            LOG_TARGET,
            animus_adapters::LogLevel::Info,
            MESSAGE,
            &[label()],
        ))
    }

    const fn label() -> animus_adapters::MetricLabelDraft<'static> {
        animus_adapters::MetricLabelDraft::new(EVENT_KIND_KEY, EVENT_KIND_VALUE)
    }
}
