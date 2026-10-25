//! Host-supplied setup of one campaign manager: its mandate, its objective, and the candidate
//! meaning the host chooses to show the model.

/// Longest objective the manager renders, in characters.
pub const MAX_OBJECTIVE_CHARS: usize = 512;
/// Longest candidate description or offer label, in characters.
pub const MAX_LABEL_CHARS: usize = 96;
/// Longest reason label a model may give for a retirement, prune, or stop, in characters.
pub const MAX_REASON_CHARS: usize = 64;
/// Most new candidates one brief may offer the model.
pub const MAX_OFFERS: usize = 16;

/// The campaign manager's standing configuration for one campaign.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ManagerSetup {
    /// The Campaign mandate that bounds every proposal, bound to the campaign and the manager's
    /// advisor identity.
    pub mandate: campaign_core::AdvisoryMandate,
    /// What the campaign is for, in the host's words; the model reads it verbatim.
    pub objective: alloc::string::String,
    /// Rank dimensions of the manager's scores while the campaign has no guidance record yet;
    /// afterwards the campaign's fixed rank shape applies.
    pub rank_dimensions: u32,
    /// The host's candidate admission rule for batches the manager proposes.
    pub admission_rule: campaign_core::AdmissionRule,
    /// The host's coalescing policy for batches the manager proposes.
    pub coalescing_policy_id: campaign_core::Id,
}

/// A host-written description of one admitted candidate, shown beside its brief handle.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateLabel {
    /// The admitted candidate.
    pub candidate_id: campaign_core::Id,
    /// Short description of what running the candidate does.
    pub label: alloc::string::String,
}

/// A new candidate the host offers; the model may add it to the campaign.
///
/// The host builds the full record, so candidate meaning, parent moment, operation, and costs stay
/// with the host; the model only chooses whether to add it.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CandidateOffer {
    /// The complete candidate record Campaign admits if the model adds the offer.
    pub record: campaign_core::CandidateRecord,
    /// Short description of what running the candidate would do.
    pub label: alloc::string::String,
}

/// The host's view of the candidates for one turn: descriptions of admitted candidates and new
/// candidates on offer.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CandidateMenu<'host> {
    /// Descriptions of admitted candidates; candidates without one are shown without a label.
    pub labels: &'host [CandidateLabel],
    /// New candidates the model may add, at most [`MAX_OFFERS`].
    pub offers: &'host [CandidateOffer],
}

/// Checks that host or model text is non-empty, at most `max_chars` characters, and free of
/// control characters, so it renders on one prompt line.
///
/// # Errors
///
/// Returns `InvalidText` naming `field` otherwise.
pub(crate) fn check_text(
    text: &str,
    max_chars: usize,
    field: &'static str,
) -> Result<(), crate::ManagerError> {
    let is_empty = text.trim().is_empty();
    let is_too_long = text.chars().count() > max_chars;
    let has_control = text.chars().any(char::is_control);
    if is_empty || is_too_long || has_control {
        return Err(crate::ManagerError::InvalidText { field });
    }
    debug_assert!(!text.is_empty());
    debug_assert!(!text.contains('\n'));
    Ok(())
}
