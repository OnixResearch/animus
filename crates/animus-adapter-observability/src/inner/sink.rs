/// Sink delivery outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SinkOutcome {
    /// Sink accepted the record.
    Delivered,
    /// Sink failed under best-effort or audit policy.
    Failed {
        /// Stable failure class.
        class: SinkFailureClass,
        /// Safe reason label.
        reason: alloc::string::String,
    },
}

impl SinkOutcome {
    /// Builds a failed outcome after validating the reason label.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the reason leaks unsafe metadata.
    pub fn failed(class: SinkFailureClass, reason: &str) -> Result<Self, Error> {
        ensure_name(reason)?;
        validate_safe_labels(&[SINK_SCHEMA, class.as_str(), reason])?;
        Ok(Self::Failed {
            class,
            reason: reason.to_string(),
        })
    }

    /// Returns true when the sink delivered the record.
    #[must_use]
    pub const fn is_delivered(&self) -> bool {
        matches!(self, Self::Delivered)
    }
}

/// Decision for guarded dispatch after observability delivery is evaluated.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GuardDecision {
    /// Guarded dispatch may continue.
    Continue,
    /// Guarded dispatch must fail closed before adapter execution.
    Blocked {
        /// Stable failure class.
        class: SinkFailureClass,
        /// Safe reason label.
        reason: alloc::string::String,
    },
}

impl GuardDecision {
    /// Returns true when guarded dispatch may continue.
    #[must_use]
    pub const fn allows_dispatch(&self) -> bool {
        matches!(self, Self::Continue)
    }
}

/// Evaluates sink delivery according to best-effort or audit-required policy.
#[must_use]
pub fn evaluate_delivery(policy: DeliveryPolicy, outcome: &SinkOutcome) -> GuardDecision {
    match (policy, outcome) {
        (DeliveryPolicy::BestEffort, _)
        | (DeliveryPolicy::AuditRequired, SinkOutcome::Delivered) => GuardDecision::Continue,
        (DeliveryPolicy::AuditRequired, SinkOutcome::Failed { class, reason }) => {
            GuardDecision::Blocked {
                class: *class,
                reason: reason.clone(),
            }
        }
    }
}

/// Implementation-free metrics sink contract.
pub trait MetricsAdapter {
    /// Emits a validated metric sample.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the sink cannot accept the sample.
    fn emit_metric(&mut self, sample: &MetricSample) -> Result<SinkOutcome, Error>;
}

/// Implementation-free logging sink contract.
pub trait LoggingAdapter {
    /// Emits a validated log record.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when the sink cannot accept the record.
    fn emit_log(&mut self, record: &LogRecord) -> Result<SinkOutcome, Error>;
}
