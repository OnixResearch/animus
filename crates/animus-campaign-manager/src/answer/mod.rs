//! The model's reply: one JSON object in the advice schema the brief declared.

/// A score of one presented candidate.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScoreReply {
    /// Candidate handle, such as `c1`.
    pub candidate: alloc::string::String,
    /// Integer ranks, highest first in priority.
    pub ranks: alloc::vec::Vec<i64>,
}

/// An offer the model adds, with an optional first score.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AddReply {
    /// Offer handle, such as `o1`.
    pub offer: alloc::string::String,
    /// First score of the added candidate; without it the candidate awaits guidance.
    pub ranks: Option<alloc::vec::Vec<i64>>,
}

/// A retirement of one presented candidate.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RetireReply {
    /// Candidate handle.
    pub candidate: alloc::string::String,
    /// Short reason label.
    pub reason: alloc::string::String,
}

/// A prune of presented candidates.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PruneReply {
    /// Candidate handles.
    pub candidates: alloc::vec::Vec<alloc::string::String>,
    /// Short reason label.
    pub reason: alloc::string::String,
}

/// A request to stop the campaign.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StopReply {
    /// Short reason label.
    pub reason: alloc::string::String,
}

/// The model's advice for one brief, as the reply schema spells it. Every field is optional: a
/// missing list is empty and a missing prune or stop is absent.
#[derive(Clone, Debug, Eq, PartialEq, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ManagerReply {
    /// Scores of presented candidates.
    #[serde(default = "alloc::vec::Vec::new")]
    pub scores: alloc::vec::Vec<ScoreReply>,
    /// Offers to add.
    #[serde(default = "alloc::vec::Vec::new")]
    pub add: alloc::vec::Vec<AddReply>,
    /// Retirements.
    #[serde(default = "alloc::vec::Vec::new")]
    pub retire: alloc::vec::Vec<RetireReply>,
    /// At most one prune.
    pub prune: Option<PruneReply>,
    /// At most one stop request.
    pub stop: Option<StopReply>,
}

/// Parses the one JSON object in `message`, ignoring any text around it such as a code fence.
///
/// # Errors
///
/// Returns `MalformedReply` when `message` holds no `{`…`}` span or the span is not exactly the
/// reply schema: an unknown field, a wrong type, or a non-integer rank all refuse.
// r[impl agent_harness_runtime.campaign_manager.reply]
pub fn parse_reply(message: &str) -> Result<ManagerReply, crate::ManagerError> {
    let start = message
        .find('{')
        .ok_or(crate::ManagerError::MalformedReply)?;
    let end = message
        .rfind('}')
        .ok_or(crate::ManagerError::MalformedReply)?;
    let object = message
        .get(start..=end)
        .ok_or(crate::ManagerError::MalformedReply)?;
    debug_assert!(object.starts_with('{'));
    debug_assert!(object.ends_with('}'));
    serde_json::from_str(object).map_err(|_| crate::ManagerError::MalformedReply)
}
