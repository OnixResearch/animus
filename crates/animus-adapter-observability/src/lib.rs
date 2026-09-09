//! Implementation-free observability adapter contracts.
//!
//! Metrics and logging adapters receive only validated safe projections. Concrete
//! sinks, global subscribers, files, clocks, network exporters, and dashboards
//! remain host-owned behavior outside reusable crates.

#![no_std]
#![cfg_attr(dylint_lib = "octet", feature(register_tool))]
#![cfg_attr(dylint_lib = "octet", register_tool(tigerstyle))]
#![deny(missing_docs)]
#![deny(clippy::all)]
#![deny(clippy::pedantic)]
#![deny(clippy::undocumented_unsafe_blocks)]
#![allow(clippy::module_name_repetitions)]
#![forbid(unsafe_code)]

extern crate alloc;

use alloc::string::ToString;

const METRIC_SCHEMA: &str = "animus.metric-sample.v1";
const LOG_SCHEMA: &str = "animus.log-record.v1";
const SINK_SCHEMA: &str = "animus.observability-sink.v1";
const OBSERVABILITY_OPERATION_LABEL: &str = "observability";
const MIN_LABEL_COUNT: usize = 1;
const MAX_LABEL_COUNT: usize = 16;
const MAX_MESSAGE_CHARS: usize = animus_adapter_safe::MAX_LABEL_CHARS;
const LABEL_JOINER: char = ':';
const LABEL_JOINER_WIDTH: usize = 1;
const SAFE_LABEL_PREFIX_COUNT: usize = 3;

include!("inner/error.rs");
include!("inner/defs.rs");
include!("inner/metric.rs");
include!("inner/log.rs");
include!("inner/sink.rs");
include!("inner/validation.rs");

#[cfg(test)]
mod tests {
    use super::*;

    const METRIC_NAME: &str = "runtime.events";
    const LOG_TARGET: &str = "runtime.event";
    const EVENT_KIND_KEY: &str = "event_kind";
    const EVENT_KIND_VALUE: &str = "runtime_started";
    const STATUS_KEY: &str = "status";
    const STATUS_VALUE: &str = "completed";
    const METRIC_VALUE: u64 = 1;
    const MESSAGE: &str = "runtime_started";
    const EMPTY_NAME: &str = "";
    const UNSAFE_VALUE: &str = "token=secret";
    const PRIVATE_PATH: &str = "/home/example/private.txt";

    #[test]
    fn dtos_accept_safe_metrics_logs_and_sink_outcomes() -> Result<(), Error> {
        let labels = [
            MetricLabelDraft::new(EVENT_KIND_KEY, EVENT_KIND_VALUE),
            MetricLabelDraft::new(STATUS_KEY, STATUS_VALUE),
        ];
        let metric = MetricSample::new(MetricSampleDraft {
            name: METRIC_NAME,
            kind: MetricKind::Counter,
            unit: MetricUnit::Count,
            value: METRIC_VALUE,
            labels: &labels,
        })?;
        assert_eq!(metric.name(), METRIC_NAME);
        assert_eq!(metric.kind(), MetricKind::Counter);
        assert_eq!(metric.labels().len(), labels.len());

        let log = LogRecord::new(LogRecordDraft::new(
            LOG_TARGET,
            LogLevel::Info,
            MESSAGE,
            &labels,
        ))?;
        assert_eq!(log.target(), LOG_TARGET);
        assert_eq!(log.level(), LogLevel::Info);
        assert_eq!(log.message(), MESSAGE);

        assert!(SinkOutcome::Delivered.is_delivered());
        let failed = SinkOutcome::failed(SinkFailureClass::Unavailable, "sink_unavailable")?;
        assert!(!failed.is_delivered());
        assert!(evaluate_delivery(DeliveryPolicy::BestEffort, &failed).allows_dispatch());
        assert!(!evaluate_delivery(DeliveryPolicy::AuditRequired, &failed).allows_dispatch());
        Ok(())
    }

    #[test]
    fn dtos_reject_empty_names_unsafe_labels_and_unbounded_messages() {
        let labels = [MetricLabelDraft::new(EVENT_KIND_KEY, EVENT_KIND_VALUE)];
        assert_eq!(
            MetricSample::new(MetricSampleDraft {
                name: EMPTY_NAME,
                kind: MetricKind::Counter,
                unit: MetricUnit::Count,
                value: METRIC_VALUE,
                labels: &labels,
            }),
            Err(Error::EmptyName)
        );
        assert!(matches!(
            MetricSample::new(MetricSampleDraft {
                name: METRIC_NAME,
                kind: MetricKind::Gauge,
                unit: MetricUnit::Count,
                value: METRIC_VALUE,
                labels: &[MetricLabelDraft::new(EVENT_KIND_KEY, UNSAFE_VALUE)],
            }),
            Err(Error::UnsafeMetadata(_))
        ));
        assert!(matches!(
            LogRecord::new(LogRecordDraft::new(
                LOG_TARGET,
                LogLevel::Error,
                PRIVATE_PATH,
                &labels,
            )),
            Err(Error::UnsafeMetadata(_))
        ));
        assert_eq!(
            LogRecord::new(LogRecordDraft::new(
                LOG_TARGET,
                LogLevel::Warning,
                MESSAGE,
                &[]
            )),
            Err(Error::InvalidLabelCount)
        );
    }
}
