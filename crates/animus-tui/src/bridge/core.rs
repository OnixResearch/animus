/// Error raised while validating UI-neutral tool-host DTOs.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ToolHostError {
    /// A text field was empty, oversized, or contained a NUL byte.
    InvalidText {
        /// Field that failed validation.
        field: &'static str,
        /// Stable validation reason.
        reason: &'static str,
    },
    /// An approval request did not expose any finite command choices.
    EmptyChoices {
        /// Field that failed validation.
        field: &'static str,
    },
    /// A progress update exceeded its declared total.
    InvalidProgress {
        /// Field that failed validation.
        field: &'static str,
        /// Stable validation reason.
        reason: &'static str,
    },
}

impl core::fmt::Display for ToolHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidText { field, reason } | Self::InvalidProgress { field, reason } => {
                write!(formatter, "tool-host field {field} is invalid: {reason}")
            }
            Self::EmptyChoices { field } => {
                write!(
                    formatter,
                    "tool-host field {field} must list finite choices"
                )
            }
        }
    }
}

impl std::error::Error for ToolHostError {}

/// Stable identity for one hosted tool execution.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ToolRunId(String);

impl ToolRunId {
    /// Builds a run identity from safe host text.
    ///
    /// # Errors
    ///
    /// Returns [`ToolHostError`] when the identity is empty, too long, or contains a NUL character.
    pub fn new(value: &str) -> Result<Self, ToolHostError> {
        Ok(Self(validate_required_text(RequiredText { field: FIELD_RUN_ID, value })?))
    }

    /// Returns the stable run identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Stable identity correlating one host approval command with one approval request.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ToolCorrelationId(String);

impl ToolCorrelationId {
    /// Builds a correlation identity from safe host text.
    ///
    /// # Errors
    ///
    /// Returns [`ToolHostError`] when the identity is empty, too long, or contains a NUL character.
    pub fn new(value: &str) -> Result<Self, ToolHostError> {
        Ok(Self(validate_required_text(RequiredText { field: FIELD_CORRELATION_ID, value })?))
    }

    /// Returns the stable correlation identity.
    #[must_use]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// Finite command choice exposed by an interactive tool request.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ToolCommandChoice {
    /// Approve the guarded action.
    Approve,
    /// Deny the guarded action.
    Deny,
    /// Continue a non-mutating interactive step.
    Continue,
    /// Cancel the pending tool action.
    Cancel,
}

impl ToolCommandChoice {
    /// Returns the stable command label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Approve => "approve",
            Self::Deny => "deny",
            Self::Continue => "continue",
            Self::Cancel => "cancel",
        }
    }
}

/// Host command returned for one approval request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ToolHostCommand {
    /// Approve the guarded action for the matching correlation identity.
    Approve {
        /// Correlation identity supplied by the approval request.
        correlation_id: ToolCorrelationId,
    },
    /// Deny the guarded action for the matching correlation identity.
    Deny {
        /// Correlation identity supplied by the approval request.
        correlation_id: ToolCorrelationId,
    },
    /// Continue a non-mutating step for the matching correlation identity.
    Continue {
        /// Correlation identity supplied by the approval request.
        correlation_id: ToolCorrelationId,
    },
    /// Cancel the pending tool action for the matching correlation identity.
    Cancel {
        /// Correlation identity supplied by the approval request.
        correlation_id: ToolCorrelationId,
    },
}

impl ToolHostCommand {
    /// Returns the finite choice selected by this command.
    #[must_use]
    pub const fn choice(&self) -> ToolCommandChoice {
        match self {
            Self::Approve { .. } => ToolCommandChoice::Approve,
            Self::Deny { .. } => ToolCommandChoice::Deny,
            Self::Continue { .. } => ToolCommandChoice::Continue,
            Self::Cancel { .. } => ToolCommandChoice::Cancel,
        }
    }

    /// Returns the correlation identity supplied by this command.
    #[must_use]
    pub const fn correlation_id(&self) -> &ToolCorrelationId {
        match self {
            Self::Approve { correlation_id }
            | Self::Deny { correlation_id }
            | Self::Continue { correlation_id }
            | Self::Cancel { correlation_id } => correlation_id,
        }
    }
}

/// Bounded progress for one hosted tool execution.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ToolProgress {
    completed_units: u32,
    total_units: Option<u32>,
}

impl ToolProgress {
    /// Builds a progress record.
    ///
    /// # Errors
    ///
    /// Returns [`ToolHostError`] when `completed_units` exceeds a declared total.
    pub fn new(completed_units: u32, total_units: Option<u32>) -> Result<Self, ToolHostError> {
        if total_units.is_some_and(|total| completed_units > total) {
            return Err(ToolHostError::InvalidProgress {
                field: FIELD_PROGRESS_TOTAL,
                reason: REASON_PROGRESS_EXCEEDS_TOTAL,
            });
        }
        Ok(Self {
            completed_units,
            total_units,
        })
    }

    /// Returns completed progress units.
    #[must_use]
    pub const fn completed_units(self) -> u32 {
        self.completed_units
    }

    /// Returns total progress units when a total is known.
    #[must_use]
    pub const fn total_units(self) -> Option<u32> {
        self.total_units
    }
}

/// Approval request emitted by a hosted tool before a guarded action.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ApprovalRequest {
    run_id: ToolRunId,
    correlation_id: ToolCorrelationId,
    prompt: String,
    choices: Vec<ToolCommandChoice>,
}

impl ApprovalRequest {
    /// Builds an approval request with finite command choices.
    ///
    /// # Errors
    ///
    /// Returns [`ToolHostError`] when text fields are invalid or no choices are provided.
    pub fn new(
        run_id: ToolRunId,
        correlation_id: ToolCorrelationId,
        prompt: &str,
        choices: &[ToolCommandChoice],
    ) -> Result<Self, ToolHostError> {
        if choices.is_empty() {
            return Err(ToolHostError::EmptyChoices {
                field: FIELD_CHOICES,
            });
        }
        Ok(Self {
            run_id,
            correlation_id,
            prompt: validate_required_text(RequiredText { field: FIELD_PROMPT, value: prompt })?,
            choices: deduplicate_choices(choices),
        })
    }

    /// Returns the tool run identity.
    #[must_use]
    pub const fn run_id(&self) -> &ToolRunId {
        &self.run_id
    }

    /// Returns the request-command correlation identity.
    #[must_use]
    pub const fn correlation_id(&self) -> &ToolCorrelationId {
        &self.correlation_id
    }

    /// Returns the safe approval prompt.
    #[must_use]
    pub fn prompt(&self) -> &str {
        self.prompt.as_str()
    }

    /// Returns finite command choices available to the host.
    #[must_use]
    pub fn choices(&self) -> &[ToolCommandChoice] {
        self.choices.as_slice()
    }

    /// Returns whether a host command matches the correlation identity and exposed choices.
    #[must_use]
    pub fn allows_command(&self, command: &ToolHostCommand) -> bool {
        command.correlation_id() == &self.correlation_id
            && self
                .choices
                .iter()
                .any(|choice| *choice == command.choice())
    }
}
