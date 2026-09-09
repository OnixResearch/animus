/// Borrowed metric label before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetricLabelDraft<'a> {
    /// Label key.
    pub key: &'a str,
    /// Label value.
    pub value: &'a str,
}

impl<'a> MetricLabelDraft<'a> {
    /// Builds a metric label draft.
    #[must_use]
    pub const fn new(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }
}

/// Validated metric label.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetricLabel {
    key: alloc::string::String,
    value: alloc::string::String,
}

impl MetricLabel {
    /// Builds a label after safe metadata validation.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] when key or value is empty or unsafe.
    pub fn new(draft: MetricLabelDraft<'_>) -> Result<Self, Error> {
        ensure_name(draft.key)?;
        ensure_name(draft.value)?;
        validate_safe_labels(&[draft.key, draft.value])?;
        Ok(Self {
            key: draft.key.to_string(),
            value: draft.value.to_string(),
        })
    }

    /// Returns the label key.
    #[must_use]
    pub fn key(&self) -> &str {
        self.key.as_str()
    }

    /// Returns the label value.
    #[must_use]
    pub fn value(&self) -> &str {
        self.value.as_str()
    }

    fn joined(&self) -> alloc::string::String {
        let capacity_bytes = self
            .key
            .len()
            .saturating_add(self.value.len())
            .saturating_add(LABEL_JOINER_WIDTH);
        let mut joined = alloc::string::String::with_capacity(capacity_bytes);
        joined.push_str(self.key.as_str());
        joined.push(LABEL_JOINER);
        joined.push_str(self.value.as_str());
        joined
    }
}

/// Borrowed safe metric sample before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MetricSampleDraft<'a> {
    /// Metric name.
    pub name: &'a str,
    /// Metric kind.
    pub kind: MetricKind,
    /// Metric unit.
    pub unit: MetricUnit,
    /// Non-negative metric value.
    pub value: u64,
    /// Safe metric labels.
    pub labels: &'a [MetricLabelDraft<'a>],
}

/// Validated safe metric sample.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MetricSample {
    name: alloc::string::String,
    kind: MetricKind,
    unit: MetricUnit,
    value: u64,
    labels: alloc::vec::Vec<MetricLabel>,
}

impl MetricSample {
    /// Builds a metric sample after validating labels and safe metadata.
    ///
    /// # Errors
    ///
    /// Returns [`Error`] for invalid names, labels, or unsafe metadata.
    pub fn new(draft: MetricSampleDraft<'_>) -> Result<Self, Error> {
        ensure_name(draft.name)?;
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
            schema: METRIC_SCHEMA,
            name: draft.name,
            labels: &label_refs,
        })?;
        Ok(Self {
            name: draft.name.to_string(),
            kind: draft.kind,
            unit: draft.unit,
            value: draft.value,
            labels,
        })
    }

    /// Returns the metric name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns metric kind.
    #[must_use]
    pub const fn kind(&self) -> MetricKind {
        self.kind
    }

    /// Returns metric unit.
    #[must_use]
    pub const fn unit(&self) -> MetricUnit {
        self.unit
    }

    /// Returns metric value.
    #[must_use]
    pub const fn value(&self) -> u64 {
        self.value
    }

    /// Returns safe labels.
    #[must_use]
    pub fn labels(&self) -> &[MetricLabel] {
        self.labels.as_slice()
    }
}
