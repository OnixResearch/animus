/// High-level status rendered by a TUI host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ToolSessionStatus {
    /// No tool is currently active.
    Idle,
    /// A tool or runtime event is in progress.
    Running,
    /// A host approval prompt is waiting for a command.
    WaitingForApproval,
    /// A tool or runtime event completed successfully.
    Finished,
    /// A tool or runtime event was cancelled.
    Cancelled,
    /// A tool or runtime event failed closed.
    Failed,
}

impl ToolSessionStatus {
    /// Returns the stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Idle => "idle",
            Self::Running => "running",
            Self::WaitingForApproval => "waiting_for_approval",
            Self::Finished => "finished",
            Self::Cancelled => "cancelled",
            Self::Failed => "failed",
        }
    }
}
