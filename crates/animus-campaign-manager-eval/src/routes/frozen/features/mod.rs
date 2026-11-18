//! The declared brief feature projection, `animus.campaign-manager-brief-features.v1`.
//!
//! Only what the brief shows at the decision enters: listed candidates in brief order, their
//! families, and their observed signals. Offers, hidden targets, and earlier briefs never do.

/// Features per listed candidate.
pub const FEATURE_COUNT: usize = 6;
/// Feature names in projection order.
pub const FEATURE_NAMES: [&str; FEATURE_COUNT] = [
    "executed",
    "signal",
    "family_best_signal",
    "family_unexplored",
    "brief_unexplored",
    "family_leads",
];
/// Fixed-point scale of every feature and of the score's weighted sum.
pub(super) const PER_MILLE: i64 = 1_000;
/// Most candidates one brief may list.
const MAX_LISTED: usize = crate::MAX_CANDIDATES;

/// One feature row per listed candidate, with the candidate handles, in brief order.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct Projection {
    pub(super) handles: Vec<String>,
    pub(super) rows: Vec<[i64; FEATURE_COUNT]>,
}

/// Why a brief cannot be projected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum BriefFault {
    /// The brief lists no candidate.
    NoCandidates,
    /// The brief lists more candidates than the task schema admits.
    TooManyCandidates,
    /// A listed signal exceeds the task schema's bound.
    SignalOutOfRange,
}

/// What every row of one brief shares.
struct BriefTotals<'brief> {
    listed: &'brief [super::super::reading::Listed],
    best_signal: Option<u32>,
    unexplored_count: usize,
}

/// `part_count` of `whole_count` candidates.
#[derive(Clone, Copy)]
struct Share {
    part_count: usize,
    whole_count: usize,
}

/// Projects every listed candidate of `view`.
///
/// # Errors
///
/// Returns the [`BriefFault`] of a brief the declared projection does not cover.
// r[impl agent_harness_runtime.campaign_manager_eval.local_scorer.schema]
pub(super) fn project(view: &super::super::reading::PromptView) -> Result<Projection, BriefFault> {
    let listed = view.candidates.as_slice();
    if listed.is_empty() {
        return Err(BriefFault::NoCandidates);
    }
    if listed.len() > MAX_LISTED {
        return Err(BriefFault::TooManyCandidates);
    }
    if listed.iter().any(|entry| {
        entry
            .signal
            .is_some_and(|signal| signal > crate::MAX_SIGNAL)
    }) {
        return Err(BriefFault::SignalOutOfRange);
    }
    let totals = BriefTotals {
        listed,
        best_signal: listed.iter().filter_map(|entry| entry.signal).max(),
        unexplored_count: listed.iter().filter(|entry| entry.signal.is_none()).count(),
    };
    let rows = listed
        .iter()
        .map(|entry| row(&totals, entry))
        .collect::<Result<Vec<_>, _>>()?;
    let handles: Vec<String> = listed.iter().map(|entry| entry.handle.clone()).collect();
    debug_assert_eq!(rows.len(), handles.len());
    debug_assert!(
        rows.iter()
            .flatten()
            .all(|feature| (0..=PER_MILLE).contains(feature))
    );
    Ok(Projection { handles, rows })
}

/// The feature row of `entry` within `totals`.
fn row(
    totals: &BriefTotals<'_>,
    entry: &super::super::reading::Listed,
) -> Result<[i64; FEATURE_COUNT], BriefFault> {
    let family = || {
        totals
            .listed
            .iter()
            .filter(|other| other.family == entry.family)
    };
    let family_count = family().count();
    let family_unexplored_count = family().filter(|other| other.signal.is_none()).count();
    let family_best = family().filter_map(|other| other.signal).max();
    let has_lead = totals.best_signal.is_some() && family_best == totals.best_signal;
    debug_assert!(family_count >= 1, "the entry belongs to its own family");
    debug_assert!(family_unexplored_count <= family_count);
    Ok([
        if entry.signal.is_some() { PER_MILLE } else { 0 },
        entry.signal.map_or(Ok(0), signal_per_mille)?,
        family_best.map_or(Ok(0), signal_per_mille)?,
        per_mille(Share {
            part_count: family_unexplored_count,
            whole_count: family_count,
        })?,
        per_mille(Share {
            part_count: totals.unexplored_count,
            whole_count: totals.listed.len(),
        })?,
        if has_lead { PER_MILLE } else { 0 },
    ])
}

/// `signal` as per mille of the task schema's bound, rounding down.
fn signal_per_mille(signal: u32) -> Result<i64, BriefFault> {
    i64::from(signal)
        .checked_mul(PER_MILLE)
        .and_then(|scaled| scaled.checked_div(i64::from(crate::MAX_SIGNAL)))
        .ok_or(BriefFault::SignalOutOfRange)
}

/// `share` as per mille, rounding down.
fn per_mille(share: Share) -> Result<i64, BriefFault> {
    let part = i64::try_from(share.part_count).map_err(|_| BriefFault::TooManyCandidates)?;
    let whole = i64::try_from(share.whole_count).map_err(|_| BriefFault::TooManyCandidates)?;
    part.checked_mul(PER_MILLE)
        .and_then(|scaled| scaled.checked_div(whole))
        .ok_or(BriefFault::NoCandidates)
}
