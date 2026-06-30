const THINKING_RANK_NONE: u8 = 0;
const THINKING_RANK_LOW: u8 = 1;
const THINKING_RANK_MEDIUM: u8 = 2;
const THINKING_RANK_HIGH: u8 = 3;

/// Reasoning support advertised by a model entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ReasoningSupport {
    /// Model has no explicit reasoning controls.
    None,
    /// Model supports optional reasoning controls.
    Optional,
    /// Model requires reasoning controls.
    Required,
}

/// Requested or effective thinking level for a selected model.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThinkingLevel {
    /// No thinking budget.
    None,
    /// Low thinking budget.
    Low,
    /// Medium thinking budget.
    Medium,
    /// High thinking budget.
    High,
}

impl ThinkingLevel {
    /// Returns the stable thinking-level label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        }
    }

    pub(super) const fn rank(self) -> u8 {
        match self {
            Self::None => THINKING_RANK_NONE,
            Self::Low => THINKING_RANK_LOW,
            Self::Medium => THINKING_RANK_MEDIUM,
            Self::High => THINKING_RANK_HIGH,
        }
    }
}

/// Thinking-level handling policy for unsupported requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ThinkingPolicy {
    /// Clamp to the highest supported level at or below the requested level.
    Clamp,
    /// Reject unsupported requested thinking levels.
    Reject,
}
