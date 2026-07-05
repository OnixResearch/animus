/// Model stream feature named in negotiation diagnostics.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamFeature {
    /// Assistant-visible content token or delta streaming.
    Content,
    /// Provider-exposed thought or reasoning streaming.
    Thought,
    /// Tool-call delta streaming.
    ToolCallDelta,
    /// Usage update streaming.
    Usage,
}

impl StreamFeature {
    /// Returns the stable stream-feature label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Content => "content_streaming",
            Self::Thought => "thought_streaming",
            Self::ToolCallDelta => "tool_call_delta_streaming",
            Self::Usage => "usage_streaming",
        }
    }
}

/// Whether one stream feature is supported by a model entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamSupport {
    /// The feature is unsupported.
    Unsupported,
    /// The feature is supported.
    Supported,
}

impl StreamSupport {
    /// Returns true when the feature is supported.
    #[must_use]
    pub const fn is_supported(self) -> bool {
        match self {
            Self::Supported => true,
            Self::Unsupported => false,
        }
    }
}

/// Effective streaming capability metadata declared by a model entry.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelStreamingCapabilities {
    content: StreamSupport,
    thought: StreamSupport,
    tool_call_delta: StreamSupport,
    usage: StreamSupport,
}

impl ModelStreamingCapabilities {
    /// Builds explicit streaming capability metadata.
    #[must_use]
    pub const fn new(
        content: StreamSupport,
        thought: StreamSupport,
        tool_call_delta: StreamSupport,
        usage: StreamSupport,
    ) -> Self {
        Self {
            content,
            thought,
            tool_call_delta,
            usage,
        }
    }

    /// Returns metadata for a model with no streaming support.
    #[must_use]
    pub const fn none() -> Self {
        Self::new(
            StreamSupport::Unsupported,
            StreamSupport::Unsupported,
            StreamSupport::Unsupported,
            StreamSupport::Unsupported,
        )
    }

    /// Returns content-stream support.
    #[must_use]
    pub const fn content(self) -> StreamSupport {
        self.content
    }

    /// Returns thought-stream support.
    #[must_use]
    pub const fn thought(self) -> StreamSupport {
        self.thought
    }

    /// Returns tool-call delta stream support.
    #[must_use]
    pub const fn tool_call_delta(self) -> StreamSupport {
        self.tool_call_delta
    }

    /// Returns usage stream support.
    #[must_use]
    pub const fn usage(self) -> StreamSupport {
        self.usage
    }

    pub(super) const fn support_for(self, feature: StreamFeature) -> StreamSupport {
        match feature {
            StreamFeature::Content => self.content,
            StreamFeature::Thought => self.thought,
            StreamFeature::ToolCallDelta => self.tool_call_delta,
            StreamFeature::Usage => self.usage,
        }
    }
}

/// Requested state for one stream feature.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StreamRequestMode {
    /// The host did not request this stream feature.
    Disabled,
    /// The host requested this stream feature.
    Required,
}

impl StreamRequestMode {
    /// Returns true when a host requested the feature.
    #[must_use]
    pub const fn is_required(self) -> bool {
        match self {
            Self::Required => true,
            Self::Disabled => false,
        }
    }
}

/// Host request for model stream modes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ModelStreamingRequest {
    content: StreamRequestMode,
    thought: StreamRequestMode,
    tool_call_delta: StreamRequestMode,
    usage: StreamRequestMode,
}

impl ModelStreamingRequest {
    /// Builds an explicit stream-mode request.
    #[must_use]
    pub const fn new(
        content: StreamRequestMode,
        thought: StreamRequestMode,
        tool_call_delta: StreamRequestMode,
        usage: StreamRequestMode,
    ) -> Self {
        Self {
            content,
            thought,
            tool_call_delta,
            usage,
        }
    }

    /// Returns a content-only streaming request.
    #[must_use]
    pub const fn content_only() -> Self {
        Self::new(
            StreamRequestMode::Required,
            StreamRequestMode::Disabled,
            StreamRequestMode::Disabled,
            StreamRequestMode::Disabled,
        )
    }

    /// Returns a request for content and thought streaming.
    #[must_use]
    pub const fn content_and_thought() -> Self {
        Self::new(
            StreamRequestMode::Required,
            StreamRequestMode::Required,
            StreamRequestMode::Disabled,
            StreamRequestMode::Disabled,
        )
    }

    pub(super) const fn mode_for(self, feature: StreamFeature) -> StreamRequestMode {
        match feature {
            StreamFeature::Content => self.content,
            StreamFeature::Thought => self.thought,
            StreamFeature::ToolCallDelta => self.tool_call_delta,
            StreamFeature::Usage => self.usage,
        }
    }
}

/// Host policy for unsupported stream requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnsupportedStreamPolicy {
    /// Reject unsupported requested stream features.
    Reject,
    /// Disable unsupported non-content stream features while preserving content streaming.
    ContentOnlyDowngrade,
}

/// Bundle of the four per-feature resolved stream modes used to build [`ResolvedStreamingModes`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct StreamModeSet {
    /// Resolved content stream mode.
    pub(super) content: StreamRequestMode,
    /// Resolved thought stream mode.
    pub(super) thought: StreamRequestMode,
    /// Resolved tool-call delta stream mode.
    pub(super) tool_call_delta: StreamRequestMode,
    /// Resolved usage stream mode.
    pub(super) usage: StreamRequestMode,
}

/// Effective stream modes after capability negotiation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ResolvedStreamingModes {
    content: StreamRequestMode,
    thought: StreamRequestMode,
    tool_call_delta: StreamRequestMode,
    usage: StreamRequestMode,
    downgraded: bool,
}

impl ResolvedStreamingModes {
    pub(super) const fn new(modes: StreamModeSet, downgraded: bool) -> Self {
        Self {
            content: modes.content,
            thought: modes.thought,
            tool_call_delta: modes.tool_call_delta,
            usage: modes.usage,
            downgraded,
        }
    }

    /// Returns resolved content stream mode.
    #[must_use]
    pub const fn content(self) -> StreamRequestMode {
        self.content
    }

    /// Returns resolved thought stream mode.
    #[must_use]
    pub const fn thought(self) -> StreamRequestMode {
        self.thought
    }

    /// Returns resolved tool-call delta stream mode.
    #[must_use]
    pub const fn tool_call_delta(self) -> StreamRequestMode {
        self.tool_call_delta
    }

    /// Returns resolved usage stream mode.
    #[must_use]
    pub const fn usage(self) -> StreamRequestMode {
        self.usage
    }

    /// Returns true when at least one unsupported non-content mode was disabled.
    #[must_use]
    pub const fn downgraded(self) -> bool {
        self.downgraded
    }
}
