/// Typed failure class for host-owned TUI/tool-host errors.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToolHostFailureClass {
    /// Terminal setup failed before rendering could start.
    TerminalSetup,
    /// Rendering failed in the host shell.
    Render,
    /// The event consumer was dropped or rejected events.
    DroppedConsumer,
    /// The command channel closed before an approval decision arrived.
    ClosedCommandChannel,
    /// The wrapped tool adapter failed.
    Adapter,
    /// The host denied a guarded action.
    Denied,
    /// The host cancelled a guarded action.
    Cancelled,
    /// The host returned a command that was not offered for the request.
    InvalidCommand,
}

impl ToolHostFailureClass {
    /// Returns the stable failure-class label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::TerminalSetup => "terminal_setup",
            Self::Render => "render",
            Self::DroppedConsumer => "dropped_consumer",
            Self::ClosedCommandChannel => "closed_command_channel",
            Self::Adapter => "adapter",
            Self::Denied => "denied",
            Self::Cancelled => "cancelled",
            Self::InvalidCommand => "invalid_command",
        }
    }
}

/// Typed host failure with a safe reason string.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ToolHostFailure {
    class: ToolHostFailureClass,
    reason: String,
}

impl ToolHostFailure {
    pub(crate) fn from_static_reason(class: ToolHostFailureClass, reason: &'static str) -> Self {
        Self {
            class,
            reason: String::from(reason),
        }
    }

    /// Builds a typed host failure from a safe reason.
    ///
    /// # Errors
    ///
    /// Returns [`ToolHostError`] when the reason is empty, too long, or contains a NUL character.
    pub fn new(class: ToolHostFailureClass, reason: &str) -> Result<Self, ToolHostError> {
        Ok(Self {
            class,
            reason: validate_required_text(RequiredText { field: FIELD_REASON, value: reason })?,
        })
    }

    /// Returns the typed host failure class.
    #[must_use]
    pub const fn class(&self) -> ToolHostFailureClass {
        self.class
    }

    /// Returns the safe failure reason.
    #[must_use]
    pub fn reason(&self) -> &str {
        self.reason.as_str()
    }
}
