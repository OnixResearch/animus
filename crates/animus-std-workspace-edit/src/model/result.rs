/// Publication state retained without durability overclaim.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PublicationState {
    /// Pure plan exists; no stage exists.
    Prepared,
    /// A private stage exists; target visibility has not changed.
    Staged,
    /// Rename made the after bytes visible.
    Published,
    /// Rename committed and required durability remains pending.
    DurabilityPending,
    /// Required payload and parent synchronization completed.
    DurabilityComplete,
    /// No target publication occurred.
    FailedBeforePublication,
    /// Publication occurred but later durability or observation failed.
    UncertainAfterPublication,
}

impl PublicationState {
    /// Return the stable receipt label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Prepared => "prepared",
            Self::Staged => "staged",
            Self::Published => "published",
            Self::DurabilityPending => "durability-pending",
            Self::DurabilityComplete => "durability-complete",
            Self::FailedBeforePublication => "failed-before-publication",
            Self::UncertainAfterPublication => "uncertain-after-publication",
        }
    }
}

/// Durability observation separated from publication visibility.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DurabilityState {
    /// Target did not publish.
    NotPublished,
    /// Caller requested visibility only.
    VisibilityOnly,
    /// Required synchronization remains incomplete.
    Pending,
    /// Required synchronization calls completed.
    Complete,
    /// Publication occurred but durability is uncertain.
    Unknown,
}

impl DurabilityState {
    /// Return the stable receipt label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NotPublished => "not-published",
            Self::VisibilityOnly => "visibility-only",
            Self::Pending => "pending",
            Self::Complete => "complete",
            Self::Unknown => "unknown",
        }
    }
}

/// Pure terminal publication outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicationOutcome {
    pub(crate) state: PublicationState,
    pub(crate) durability: DurabilityState,
    pub(crate) observation_count: u32,
}

impl PublicationOutcome {
    /// Return the exact terminal publication state.
    #[must_use]
    pub const fn state(&self) -> PublicationState {
        self.state
    }

    /// Return the separate durability state.
    #[must_use]
    pub const fn durability(&self) -> DurabilityState {
        self.durability
    }

    /// Return the finite mechanism observation count.
    #[must_use]
    pub const fn observation_count(&self) -> u32 {
        self.observation_count
    }
}
