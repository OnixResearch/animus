//! Rendering one Campaign brief into a bounded model request.
//!
//! The model sees handles (`c1`, `c2`, … for listed candidates and `o1`, `o2`, … for offers), the
//! host's labels, the campaign's counters, and the mandate's grants; it never sees Campaign
//! identities, so a reply can only name what the brief presented.

mod lines;

/// First handle number; handles count from one so the model reads `c1` as the first candidate.
const FIRST_HANDLE: usize = 1;

/// The handles one rendered brief presented and the score domain it declared.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BriefHandles {
    /// Candidate of handle `c<n>` at position `n - 1`.
    candidates: alloc::vec::Vec<campaign_core::BriefedCandidate>,
    /// Offer of handle `o<n>` at position `n - 1`.
    offers: alloc::vec::Vec<crate::CandidateOffer>,
    /// Ranks per score.
    rank_dimensions: u32,
    /// Largest absolute rank.
    rank_magnitude: u64,
}

impl BriefHandles {
    /// The listed candidate a `c<n>` handle names.
    pub(crate) fn candidate(&self, handle: &str) -> Option<&campaign_core::BriefedCandidate> {
        handle_index(handle, 'c').and_then(|index| self.candidates.get(index))
    }

    /// The offer an `o<n>` handle names.
    pub(crate) fn offer(&self, handle: &str) -> Option<&crate::CandidateOffer> {
        handle_index(handle, 'o').and_then(|index| self.offers.get(index))
    }

    /// Ranks every score must hold.
    #[must_use]
    pub const fn rank_dimensions(&self) -> u32 {
        self.rank_dimensions
    }

    /// Largest absolute rank a score may hold.
    #[must_use]
    pub const fn rank_magnitude(&self) -> u64 {
        self.rank_magnitude
    }
}

/// One brief rendered for the model, with the brief and the handles it presented.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RenderedBrief {
    /// The Campaign brief the request renders.
    pub brief: campaign_core::AdvisoryBrief,
    /// The model request holding the rendered brief.
    pub request: animus_adapters::ModelRequest,
    /// The handles and score domain the request presented.
    pub handles: BriefHandles,
}

/// Renders `brief` of `state` for the model under `setup`, showing `menu`'s labels and offers.
///
/// # Errors
///
/// Returns `InvalidText` for an objective or label that is empty, too long, or holds a control
/// character, `TooManyOffers` for more than [`crate::MAX_OFFERS`] offers, `RankDimensions` for a
/// configured rank count of zero or above the campaign's rank-dimension bound, and `Prompt` when
/// the rendered brief exceeds the model request contract.
// r[impl agent_harness_runtime.campaign_manager.brief]
pub fn render_brief(
    state: &campaign_core::FrontierState,
    brief: &campaign_core::AdvisoryBrief,
    setup: &crate::ManagerSetup,
    menu: crate::CandidateMenu<'_>,
) -> Result<RenderedBrief, crate::ManagerError> {
    crate::host::check_text(&setup.objective, crate::MAX_OBJECTIVE_CHARS, "objective")?;
    if menu.offers.len() > crate::MAX_OFFERS {
        return Err(crate::ManagerError::TooManyOffers);
    }
    for text in menu.labels.iter().map(|label| label.label.as_str()) {
        crate::host::check_text(text, crate::MAX_LABEL_CHARS, "label")?;
    }
    for text in menu.offers.iter().map(|offer| offer.label.as_str()) {
        crate::host::check_text(text, crate::MAX_LABEL_CHARS, "label")?;
    }
    let handles = BriefHandles {
        candidates: brief.candidates.clone(),
        offers: menu.offers.to_vec(),
        rank_dimensions: rank_dimensions(state, brief, setup)?,
        rank_magnitude: brief.bounds.max_rank_magnitude,
    };
    let text = lines::render(brief, setup, menu, &handles);
    let request = animus_adapters::ModelRequest::new(&text).map_err(crate::ManagerError::Prompt)?;
    debug_assert_eq!(handles.candidates.len(), brief.candidates.len());
    debug_assert!(handles.offers.len() <= crate::MAX_OFFERS);
    Ok(RenderedBrief {
        brief: brief.clone(),
        request,
        handles,
    })
}

/// The campaign's fixed rank shape once any candidate holds guidance, else the configured count;
/// either must lie within the campaign's rank-dimension bound.
fn rank_dimensions(
    state: &campaign_core::FrontierState,
    brief: &campaign_core::AdvisoryBrief,
    setup: &crate::ManagerSetup,
) -> Result<u32, crate::ManagerError> {
    let established = state
        .candidates()
        .iter()
        .find_map(|entry| entry.latest_guidance.as_ref())
        .map(|guidance| u32::try_from(guidance.ranks.len()));
    let dimensions = match established {
        Some(Ok(dimensions)) => dimensions,
        Some(Err(_)) => return Err(crate::ManagerError::RankDimensions),
        None => setup.rank_dimensions,
    };
    let is_within_bound = u64::from(dimensions) <= brief.bounds.max_rank_dimensions;
    if dimensions == 0 || !is_within_bound {
        return Err(crate::ManagerError::RankDimensions);
    }
    Ok(dimensions)
}

/// The zero-based position a canonical handle such as `c3` names, for the given prefix; a
/// non-canonical spelling such as `c03` or `c+3` names nothing.
fn handle_index(handle: &str, prefix: char) -> Option<usize> {
    let number: usize = handle.strip_prefix(prefix)?.parse().ok()?;
    let index = number.checked_sub(FIRST_HANDLE)?;
    (self::handle(prefix, index) == handle).then_some(index)
}

/// The handle of the zero-based position `index` under `prefix`.
pub(crate) fn handle(prefix: char, index: usize) -> alloc::string::String {
    alloc::format!("{prefix}{}", index.saturating_add(FIRST_HANDLE))
}
