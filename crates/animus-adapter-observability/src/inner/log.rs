/// Borrowed safe log record before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LogRecordDraft<'a> {
    /// Log target label.
    pub target: &'a str,
    /// Log level.
    pub level: LogLevel,
    /// Bounded safe message label, not raw runtime payload text.
    pub message: &'a str,
    /// Safe labels.
    pub labels: &'a [MetricLabelDraft<'a>],
}

impl<'a> LogRecordDraft<'a> {
    /// Builds a log record draft.
    #[must_use]
    pub const fn new(
        target: &'a str,
        level: LogLevel,
        message: &'a str,
        labels: &'a [MetricLabelDraft<'a>],
    ) -> Self {
        Self {
            target,
            level,
            message,
            labels,
        }
    }
}

/// Validated safe log record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LogRecord {
    target: alloc::string::String,
    level: LogLevel,
    message: alloc::string::String,
    labels: alloc::vec::Vec<MetricLabel>,
}

impl LogRecord {
    /// Builds a safe log record after rejecting unsafe metadata.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] for invalid names, labels, message bounds, or leaks.
    pub fn new(draft: LogRecordDraft<'_>) -> Result<Self, Error> {
        ensure_name(draft.target)?;
        ensure_name(draft.message)?;
        if draft.message.chars().count() > MAX_MESSAGE_CHARS {
            return Err(Error::UnboundedMessage);
        }
        ensure_label_count(draft.labels.len())?;
        let labels = draft
            .labels
            .iter()
            .copied()
            .map(MetricLabel::new)
            .collect::<Result<alloc::vec::Vec<_>, _>>()?;
        let label_storage = label_strings(labels.as_slice());
        let label_refs = label_refs(label_storage.as_slice());
        validate_safe_label_envelope(&SafeLabelEnvelope {
            schema: LOG_SCHEMA,
            name: draft.target,
            labels: &label_refs,
        })?;
        validate_safe_labels(&[draft.message])?;
        Ok(Self {
            target: draft.target.to_string(),
            level: draft.level,
            message: draft.message.to_string(),
            labels,
        })
    }

    /// Returns the log target.
    #[must_use]
    pub fn target(&self) -> &str {
        self.target.as_str()
    }

    /// Returns the log level.
    #[must_use]
    pub const fn level(&self) -> LogLevel {
        self.level
    }

    /// Returns the safe message label.
    #[must_use]
    pub fn message(&self) -> &str {
        self.message.as_str()
    }

    /// Returns safe labels.
    #[must_use]
    pub fn labels(&self) -> &[MetricLabel] {
        self.labels.as_slice()
    }
}
