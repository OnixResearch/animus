//! Typed refusals of one campaign-manager turn.

/// Why the campaign manager refused to turn a brief, a model reply, or a review into advice.
///
/// Every refusal leaves the campaign unchanged: the manager builds no control event until Campaign
/// admits the whole proposal.
// r[impl agent_harness_runtime.campaign_manager.refusals]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ManagerError {
    /// Campaign refused the brief, the proposal, or its review.
    Campaign(campaign_core::CampaignError),
    /// The rendered brief does not fit the model request contract.
    Prompt(animus_adapters::AdapterContractError),
    /// Host-supplied text is empty, too long, or holds a control character.
    InvalidText {
        /// Which text was refused: `objective`, `label`, or `reason`.
        field: &'static str,
    },
    /// The host offered more new candidates than one brief may present.
    TooManyOffers,
    /// The configured rank dimension count is zero or above the campaign's rank-dimension bound.
    RankDimensions,
    /// The model reply holds no JSON object, or the object does not match the reply schema.
    MalformedReply,
    /// The model reply names a candidate or offer handle the brief did not present.
    UnknownHandle {
        /// The unknown handle as the model wrote it.
        handle: alloc::string::String,
    },
    /// The model scored a handle with a rank vector of the wrong length.
    RankLength {
        /// The scored handle.
        handle: alloc::string::String,
    },
    /// The model scored a handle with a rank above the campaign's rank-magnitude bound.
    RankMagnitude {
        /// The scored handle.
        handle: alloc::string::String,
    },
}

impl core::fmt::Display for ManagerError {
    /// Writes the refusal and the handle or field it concerns.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Campaign(error) => write!(formatter, "campaign refused the advice: {error}"),
            Self::Prompt(error) => write!(formatter, "brief does not fit a model request: {error}"),
            Self::InvalidText { field } => write!(formatter, "invalid {field} text"),
            Self::TooManyOffers => formatter.write_str("too many candidate offers for one brief"),
            Self::RankDimensions => formatter.write_str("invalid rank dimension count"),
            Self::MalformedReply => formatter.write_str("model reply is not the advice schema"),
            Self::UnknownHandle { handle } => write!(formatter, "unknown handle {handle}"),
            Self::RankLength { handle } => write!(formatter, "wrong rank count for {handle}"),
            Self::RankMagnitude { handle } => write!(formatter, "rank out of bounds for {handle}"),
        }
    }
}

impl core::error::Error for ManagerError {}

impl From<campaign_core::CampaignError> for ManagerError {
    fn from(error: campaign_core::CampaignError) -> Self {
        Self::Campaign(error)
    }
}

/// Why one manager turn produced no admitted advice: the model failed, or the manager or Campaign
/// refused what it returned.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TurnFailure<E> {
    /// The injected model adapter failed.
    Model(E),
    /// The manager or Campaign refused the turn.
    Refused(ManagerError),
}

impl<E> From<ManagerError> for TurnFailure<E> {
    fn from(error: ManagerError) -> Self {
        Self::Refused(error)
    }
}

impl<E> From<campaign_core::CampaignError> for TurnFailure<E> {
    fn from(error: campaign_core::CampaignError) -> Self {
        Self::Refused(ManagerError::Campaign(error))
    }
}

impl<E: core::fmt::Display> core::fmt::Display for TurnFailure<E> {
    /// Writes the model failure or the refusal.
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Model(error) => write!(formatter, "model adapter failed: {error}"),
            Self::Refused(error) => core::fmt::Display::fmt(error, formatter),
        }
    }
}

impl<E: core::fmt::Debug + core::fmt::Display> core::error::Error for TurnFailure<E> {}
