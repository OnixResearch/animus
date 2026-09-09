mod collections;

/// Borrowed bounded-count input for a safe receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BoundedCountDraft<'a> {
    label: &'a str,
    value: u32,
}

impl<'a> BoundedCountDraft<'a> {
    /// Builds a bounded-count draft.
    #[must_use]
    pub const fn new(label: &'a str, value: u32) -> Self {
        Self { label, value }
    }
}

/// Bounded count retained by a safe receipt.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct BoundedCount {
    label: alloc::string::String,
    value: u32,
}

impl BoundedCount {
    fn new(label: alloc::string::String, value: u32) -> Self {
        Self { label, value }
    }

    /// Returns the safe count label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    /// Returns the bounded count value.
    #[must_use]
    pub const fn value(&self) -> u32 {
        self.value
    }
}

/// Borrowed inputs used to construct a safe receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Draft<'a> {
    status: crate::Status,
    issue_code: crate::IssueCode,
    operation: crate::Operation,
    labels: &'a [&'a str],
    counts: &'a [BoundedCountDraft<'a>],
    artifact_hashes: &'a [crate::ArtifactHash],
    backend_label: Option<&'a str>,
    interchange_label: Option<&'a str>,
    redactions: &'a [crate::Redaction],
}

impl<'a> Draft<'a> {
    /// Builds a draft with required status, issue, and operation classes.
    #[must_use]
    pub const fn new(
        status: crate::Status,
        issue_code: crate::IssueCode,
        operation: crate::Operation,
    ) -> Self {
        Self {
            status,
            issue_code,
            operation,
            labels: &[],
            counts: &[],
            artifact_hashes: &[],
            backend_label: None,
            interchange_label: None,
            redactions: &[],
        }
    }

    /// Adds safe labels.
    #[must_use]
    pub const fn with_labels(mut self, labels: &'a [&'a str]) -> Self {
        self.labels = labels;
        self
    }

    /// Adds bounded counts.
    #[must_use]
    pub const fn with_counts(mut self, counts: &'a [BoundedCountDraft<'a>]) -> Self {
        self.counts = counts;
        self
    }

    /// Adds artifact identity hashes.
    #[must_use]
    pub const fn with_artifact_hashes(mut self, hashes: &'a [crate::ArtifactHash]) -> Self {
        self.artifact_hashes = hashes;
        self
    }

    /// Adds a backend label such as a provider class or local runtime class.
    #[must_use]
    pub const fn with_backend_label(mut self, backend_label: &'a str) -> Self {
        self.backend_label = Some(backend_label);
        self
    }

    /// Adds an interchange label such as `preserves` or `jsonl`.
    #[must_use]
    pub const fn with_interchange_label(mut self, interchange_label: &'a str) -> Self {
        self.interchange_label = Some(interchange_label);
        self
    }

    /// Adds redaction markers.
    #[must_use]
    pub const fn with_redactions(mut self, redactions: &'a [crate::Redaction]) -> Self {
        self.redactions = redactions;
        self
    }
}

/// Safe runtime/provenance receipt containing metadata, counts, hashes, and redactions only.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Record {
    schema: &'static str,
    status: crate::Status,
    issue_code: crate::IssueCode,
    operation: crate::Operation,
    labels: alloc::vec::Vec<alloc::string::String>,
    counts: alloc::vec::Vec<BoundedCount>,
    artifact_hashes: alloc::vec::Vec<crate::ArtifactHash>,
    backend_label: Option<alloc::string::String>,
    interchange_label: Option<alloc::string::String>,
    redactions: alloc::vec::Vec<crate::Redaction>,
}

impl Record {
    /// Builds a safe receipt after validating all metadata fields for leaks.
    ///
    /// # Errors
    ///
    /// Returns [`crate::Error`] when metadata is oversized or unsafe.
    pub fn new(draft: Draft<'_>) -> Result<Self, crate::Error> {
        Ok(Self {
            schema: crate::SCHEMA,
            status: draft.status,
            issue_code: draft.issue_code,
            operation: draft.operation,
            labels: collections::labels(draft.labels)?,
            counts: collections::counts(draft.counts)?,
            artifact_hashes: collections::hashes(draft.artifact_hashes)?,
            backend_label: crate::optional_label(crate::BACKEND_LABEL_FIELD, draft.backend_label)?,
            interchange_label: crate::optional_label(
                crate::INTERCHANGE_LABEL_FIELD,
                draft.interchange_label,
            )?,
            redactions: collections::redactions(draft.redactions)?,
        })
    }

    /// Returns the receipt schema identifier.
    #[must_use]
    pub const fn schema(&self) -> &'static str {
        self.schema
    }

    /// Returns the receipt status.
    #[must_use]
    pub const fn status(&self) -> crate::Status {
        self.status
    }

    /// Returns the receipt issue code.
    #[must_use]
    pub const fn issue_code(&self) -> crate::IssueCode {
        self.issue_code
    }

    /// Returns the effect or mutation operation class.
    #[must_use]
    pub const fn operation(&self) -> crate::Operation {
        self.operation
    }

    /// Returns sorted and deduplicated safe labels.
    #[must_use]
    pub fn labels(&self) -> &[alloc::string::String] {
        self.labels.as_slice()
    }

    /// Returns sorted and deduplicated bounded counts.
    #[must_use]
    pub fn counts(&self) -> &[BoundedCount] {
        self.counts.as_slice()
    }

    /// Returns sorted and deduplicated artifact hashes.
    #[must_use]
    pub fn artifact_hashes(&self) -> &[crate::ArtifactHash] {
        self.artifact_hashes.as_slice()
    }

    /// Returns the optional backend label.
    #[must_use]
    pub fn backend_label(&self) -> Option<&str> {
        self.backend_label.as_deref()
    }

    /// Returns the optional interchange label.
    #[must_use]
    pub fn interchange_label(&self) -> Option<&str> {
        self.interchange_label.as_deref()
    }

    /// Returns sorted and deduplicated redaction markers.
    #[must_use]
    pub fn redactions(&self) -> &[crate::Redaction] {
        self.redactions.as_slice()
    }

    /// Renders bounded review output without raw payload fields.
    #[must_use]
    pub fn review_summary(&self) -> alloc::string::String {
        let mut text = alloc::string::String::new();
        crate::push_review_value(&mut text, crate::REVIEW_PREFIX);
        crate::push_review_value(&mut text, self.schema);
        crate::push_review_value(&mut text, self.status.as_str());
        crate::push_review_value(&mut text, self.issue_code.as_str());
        crate::push_review_value(&mut text, self.operation.as_str());
        for label in &self.labels {
            crate::push_review_value(&mut text, label);
        }
        for hash in &self.artifact_hashes {
            crate::push_review_value(&mut text, hash.as_str());
        }
        for redaction in &self.redactions {
            crate::push_review_value(&mut text, redaction.as_str());
        }
        text
    }
}
