//! The text of one rendered brief, line by line.

/// Schema of the one JSON object the model must reply with.
const REPLY_SCHEMA: &str = r#"{"scores":[{"candidate":"c1","ranks":[0]}],"add":[{"offer":"o1","ranks":[0]}],"retire":[{"candidate":"c2","reason":"label"}],"prune":{"candidates":["c3"],"reason":"label"},"stop":{"reason":"label"}}"#;

/// What each granted action lets the model do, in the order the mandate lists grants.
const GRANT_WORDING: [(campaign_core::AdvisoryActionKind, &str); 6] = [
    (
        campaign_core::AdvisoryActionKind::CandidateBatch,
        "add offers",
    ),
    (
        campaign_core::AdvisoryActionKind::Guidance,
        "first or raised scores",
    ),
    (
        campaign_core::AdvisoryActionKind::Revision,
        "lowered scores",
    ),
    (
        campaign_core::AdvisoryActionKind::Retirement,
        "retire candidates",
    ),
    (campaign_core::AdvisoryActionKind::Prune, "prune candidates"),
    (campaign_core::AdvisoryActionKind::Stop, "stop the campaign"),
];

/// The full brief text: instructions, grants, counters, candidates, and offers.
pub(super) fn render(
    brief: &campaign_core::AdvisoryBrief,
    setup: &crate::ManagerSetup,
    menu: crate::CandidateMenu<'_>,
    handles: &super::BriefHandles,
) -> alloc::string::String {
    let instructions = alloc::format!(
        "You are the campaign manager of one exploration campaign.\n\
         Objective: {objective}\n\
         Reply with exactly one JSON object and no other text. Every field is optional:\n\
         {REPLY_SCHEMA}\n\
         Ranks: {dimensions} integers per score, each from -{magnitude} to {magnitude}. Higher ranks run first, compared from the first integer.\n\
         Reasons: short labels on one line, at most {reason_chars} characters.\n",
        objective = setup.objective,
        dimensions = handles.rank_dimensions,
        magnitude = handles.rank_magnitude,
        reason_chars = crate::MAX_REASON_CHARS,
    );
    let text = [
        instructions,
        grants(&setup.mandate.terms),
        counters(brief),
        candidates(brief, menu),
        offers(menu),
    ]
    .concat();
    debug_assert!(text.ends_with('\n'));
    debug_assert!(text.starts_with("You are"));
    text
}

/// One line listing what the mandate allows per reply, and one listing what it refuses.
fn grants(terms: &campaign_core::AdvisoryTerms) -> alloc::string::String {
    let mut allowed = alloc::vec::Vec::with_capacity(GRANT_WORDING.len());
    let mut refused = alloc::vec::Vec::with_capacity(GRANT_WORDING.len());
    for (action, wording) in GRANT_WORDING {
        match terms.max_per_proposal(action) {
            Some(maximum) => allowed.push(alloc::format!("{wording} (at most {maximum})")),
            None => refused.push(wording),
        }
    }
    let refused_line = if refused.is_empty() {
        alloc::string::String::new()
    } else {
        alloc::format!("Not allowed: {}.\n", refused.join(", "))
    };
    alloc::format!("Allowed per reply: {}.\n{refused_line}", allowed.join(", "))
}

/// The campaign's counters and remaining budgets.
fn counters(brief: &campaign_core::AdvisoryBrief) -> alloc::string::String {
    let budgets: alloc::vec::Vec<alloc::string::String> = brief
        .remaining_budgets
        .iter()
        .enumerate()
        .map(|(index, budget)| alloc::format!("{}={}", super::handle('d', index), budget.amount))
        .collect();
    alloc::format!(
        "Campaign: {} picks so far, {} running, {} results, {} findings; {} candidates not shown, {} retired, {} pruned. Budget left: {}.\n",
        brief.selection_count,
        brief.outstanding_selections,
        brief.accepted_results,
        brief.findings,
        brief.omitted_candidates,
        brief.retired_candidates,
        brief.pruned_candidates,
        budgets.join(", "),
    )
}

/// One line per listed candidate, unscored first, then best-ranked first.
fn candidates(
    brief: &campaign_core::AdvisoryBrief,
    menu: crate::CandidateMenu<'_>,
) -> alloc::string::String {
    let lines = brief.candidates.iter().enumerate().map(|(index, entry)| {
        let score = entry.latest_guidance.as_ref().map_or_else(
            || alloc::string::String::from("unscored"),
            |guidance| alloc::format!("ranks {:?}", guidance.ranks),
        );
        let running = if entry.has_outstanding_selection {
            ", running"
        } else {
            ""
        };
        let label = menu
            .labels
            .iter()
            .find(|label| label.candidate_id == entry.candidate.candidate_id)
            .map_or("-", |label| label.label.as_str());
        alloc::format!(
            "{} {score}, picked {} times{running}: {label}\n",
            super::handle('c', index),
            entry.selection_count,
        )
    });
    let text: alloc::string::String = core::iter::once(alloc::string::String::from(
        "Candidates (unscored first, then best first):\n",
    ))
    .chain(lines)
    .collect();
    debug_assert_eq!(
        text.lines().count(),
        brief.candidates.len().saturating_add(1),
        "handle c<n> must name the n-th listed line"
    );
    debug_assert!(text.ends_with('\n'));
    text
}

/// One line per offer, or nothing without offers.
fn offers(menu: crate::CandidateMenu<'_>) -> alloc::string::String {
    if menu.offers.is_empty() {
        return alloc::string::String::new();
    }
    let lines =
        menu.offers.iter().enumerate().map(|(index, offer)| {
            alloc::format!("{}: {}\n", super::handle('o', index), offer.label)
        });
    core::iter::once(alloc::string::String::from("Offers you may add:\n"))
        .chain(lines)
        .collect()
}
