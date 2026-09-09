#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "`octet::sealed_enum`: Animus owns this closed metric vocabulary. New variants require explicit wire labels."]
pub enum MetricKind {
    /// Monotonic counter sample.
    Counter,
    /// Gauge sample.
    Gauge,
    /// Duration bucket sample.
    DurationBucket,
}

impl MetricKind {
    /// Returns the stable metric-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Counter => "counter",
            Self::Gauge => "gauge",
            Self::DurationBucket => "duration_bucket",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "`octet::sealed_enum`: Animus owns this closed metric-unit vocabulary. New variants require explicit wire labels."]
pub enum MetricUnit {
    /// Unitless count.
    Count,
    /// Millisecond duration.
    Milliseconds,
    /// Byte count.
    Bytes,
}

impl MetricUnit {
    /// Returns the stable unit label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Count => "count",
            Self::Milliseconds => "milliseconds",
            Self::Bytes => "bytes",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "`octet::sealed_enum`: Animus owns this closed log-level vocabulary. New variants require explicit wire labels."]
pub enum LogLevel {
    /// Informational record.
    Info,
    /// Warning record.
    Warning,
    /// Error record.
    Error,
}

impl LogLevel {
    /// Returns the stable log-level label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "`octet::sealed_enum`: Animus owns this closed delivery vocabulary. New variants require explicit wire labels."]
pub enum DeliveryPolicy {
    /// Sink failures are diagnostics and do not block runtime execution.
    BestEffort,
    /// Required sink delivery failure blocks guarded adapter dispatch.
    AuditRequired,
}

impl DeliveryPolicy {
    /// Returns the stable policy label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BestEffort => "best_effort",
            Self::AuditRequired => "audit_required",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[doc = "`octet::sealed_enum`: Animus owns this closed sink-failure vocabulary. New variants require explicit wire labels."]
pub enum SinkFailureClass {
    /// Sink was unavailable.
    Unavailable,
    /// Sink collector was closed.
    Closed,
    /// Sink record was malformed.
    MalformedRecord,
    /// Sink record was unsafe or leak-bearing.
    UnsafeRecord,
    /// Audit policy denied guarded dispatch.
    AuditPolicyDenied,
}

impl SinkFailureClass {
    /// Returns the stable sink-failure label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unavailable => "unavailable",
            Self::Closed => "closed",
            Self::MalformedRecord => "malformed_record",
            Self::UnsafeRecord => "unsafe_record",
            Self::AuditPolicyDenied => "audit_policy_denied",
        }
    }
}
