const SELECTED_TRUSTED_SUMMARY: &str = "selected trusted lifecycle status";
const SELECTED_HEURISTIC_SUMMARY: &str = "selected heuristic fallback status";
const UNKNOWN_STATUS_SUMMARY: &str = "no current trusted status authority";
const MAX_STATUS_EVIDENCE_SUMMARY_CHARS: usize = 256;
const FIRST_INDEX: usize = 0;

const RAW_PROMPT_MARKER: &str = "RAW_PROMPT:";
const TERMINAL_SNAPSHOT_MARKER: &str = "TERMINAL_SNAPSHOT:";
const PROVIDER_PAYLOAD_MARKER: &str = "PROVIDER_PAYLOAD:";
const API_KEY_MARKER: &str = "API_KEY=";
const SECRET_KEY_PREFIX_MARKER: &str = "sk-";
const UNBOUNDED_DETECTOR_MARKER: &str = "UNBOUNDED_DETECTOR_EVIDENCE:";

const UNSAFE_EVIDENCE_MARKERS: &[&str] = &[
    RAW_PROMPT_MARKER,
    TERMINAL_SNAPSHOT_MARKER,
    PROVIDER_PAYLOAD_MARKER,
    API_KEY_MARKER,
    SECRET_KEY_PREFIX_MARKER,
    UNBOUNDED_DETECTOR_MARKER,
];

/// Freshness class assigned to a status decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusFreshness {
    /// A current lifecycle or heuristic authority was selected.
    Current,
    /// Matching observations existed but all authoritative candidates were stale.
    Stale,
    /// No matching usable observation existed.
    Missing,
    /// Current lifecycle authorities conflicted and the selection failed closed.
    Conflicting,
}

/// Reason a decision selected fallback or unknown state.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatusFallbackReason {
    /// No trusted lifecycle report matched the target.
    NoTrustedLifecycle,
    /// Trusted lifecycle reports matched but were expired by TTL.
    StaleLifecycle,
    /// Only untrusted lifecycle reports matched the target.
    UntrustedLifecycle,
    /// Current trusted lifecycle authorities disagreed at the newest sequence.
    ConflictingLifecycleAuthority,
    /// No observation matched the target.
    NoObservation,
}

/// Reason an observation was not selected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RejectedStatusReason {
    /// Observation targeted another agent or pane.
    TargetMismatch,
    /// Observation was expired by its TTL.
    Stale,
    /// Observation came from an untrusted lifecycle source.
    UntrustedLifecycle,
    /// Heuristic fallback was suppressed by a trusted lifecycle authority.
    HeuristicSuppressed,
    /// Observation was older than the selected observation.
    OlderSequence,
    /// Observation duplicated the selected lifecycle authority class.
    DuplicateLifecycleAuthority,
    /// Observation conflicted with another current lifecycle authority.
    ConflictingLifecycleAuthority,
}

/// Safe summary of a selected status authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusAuthoritySummary {
    source_id: String,
    kind: animus_adapters::AgentStatusObservationKind,
    state: animus_adapters::SemanticAgentState,
    sequence: animus_adapters::StatusSequence,
}

impl StatusAuthoritySummary {
    fn from_observation(observation: &animus_adapters::AgentStatusObservation) -> Self {
        Self {
            source_id: String::from(observation.source_id()),
            kind: observation.kind(),
            state: observation.state(),
            sequence: observation.sequence(),
        }
    }

    /// Returns the selected source identity.
    #[must_use]
    pub fn source_id(&self) -> &str {
        self.source_id.as_str()
    }

    /// Returns the selected observation kind.
    #[must_use]
    pub const fn kind(&self) -> animus_adapters::AgentStatusObservationKind {
        self.kind
    }

    /// Returns the selected semantic state.
    #[must_use]
    pub const fn state(&self) -> animus_adapters::SemanticAgentState {
        self.state
    }

    /// Returns the selected sequence.
    #[must_use]
    pub const fn sequence(&self) -> animus_adapters::StatusSequence {
        self.sequence
    }
}

/// Safe summary of a rejected status candidate.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RejectedStatusCandidate {
    authority: StatusAuthoritySummary,
    reason: RejectedStatusReason,
}

impl RejectedStatusCandidate {
    fn new(
        observation: &animus_adapters::AgentStatusObservation,
        reason: RejectedStatusReason,
    ) -> Self {
        Self {
            authority: StatusAuthoritySummary::from_observation(observation),
            reason,
        }
    }

    /// Returns the rejected authority summary.
    #[must_use]
    pub const fn authority(&self) -> &StatusAuthoritySummary {
        &self.authority
    }

    /// Returns the rejection reason.
    #[must_use]
    pub const fn reason(&self) -> RejectedStatusReason {
        self.reason
    }
}

/// Bounded redacted evidence summary for durable explanations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusEvidenceSummary {
    text: String,
}

impl StatusEvidenceSummary {
    /// Builds a bounded, redacted evidence summary.
    ///
    /// # Errors
    ///
    /// Returns [`StatusEvidenceError`] when the summary is empty, overlong, or
    /// contains an unsafe raw-evidence marker.
    pub fn new(text: &str) -> Result<Self, StatusEvidenceError> {
        validate_status_evidence_summary(text)?;
        Ok(Self {
            text: String::from(text),
        })
    }

    fn trusted_lifecycle() -> Self {
        Self {
            text: String::from(SELECTED_TRUSTED_SUMMARY),
        }
    }

    fn heuristic_fallback() -> Self {
        Self {
            text: String::from(SELECTED_HEURISTIC_SUMMARY),
        }
    }

    fn unknown_status() -> Self {
        Self {
            text: String::from(UNKNOWN_STATUS_SUMMARY),
        }
    }

    /// Returns the safe summary text.
    #[must_use]
    pub fn text(&self) -> &str {
        self.text.as_str()
    }
}

/// Evidence-summary validation error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum StatusEvidenceError {
    /// Summary contained no user-visible characters.
    EmptySummary,
    /// Summary exceeded the bounded evidence limit.
    SummaryTooLong {
        /// Maximum accepted character count.
        max_chars: usize,
        /// Actual character count.
        actual_chars: usize,
    },
    /// Summary contained a raw or unsafe evidence marker.
    UnsafeEvidenceMarker {
        /// Marker that was detected.
        marker: &'static str,
    },
}

impl core::fmt::Display for StatusEvidenceError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EmptySummary => write!(formatter, "status evidence summary must not be empty"),
            Self::SummaryTooLong {
                max_chars,
                actual_chars,
            } => write!(
                formatter,
                "status evidence summary must be at most {max_chars} characters; got {actual_chars}"
            ),
            Self::UnsafeEvidenceMarker { marker } => write!(
                formatter,
                "status evidence summary contains unsafe marker {marker}"
            ),
        }
    }
}

impl std::error::Error for StatusEvidenceError {}

/// Validates a durable status evidence summary.
///
/// # Errors
///
/// Returns [`StatusEvidenceError`] when the summary is empty, overlong, or
/// contains a raw prompt, terminal snapshot, provider payload, credential-looking
/// value, or unbounded detector evidence marker.
pub fn validate_status_evidence_summary(text: &str) -> Result<(), StatusEvidenceError> {
    let visible_chars = text.trim().chars().count();
    if visible_chars == FIRST_INDEX {
        return Err(StatusEvidenceError::EmptySummary);
    }

    let actual_chars = text.chars().count();
    if actual_chars > MAX_STATUS_EVIDENCE_SUMMARY_CHARS {
        return Err(StatusEvidenceError::SummaryTooLong {
            max_chars: MAX_STATUS_EVIDENCE_SUMMARY_CHARS,
            actual_chars,
        });
    }

    for marker in UNSAFE_EVIDENCE_MARKERS {
        if text.contains(marker) {
            return Err(StatusEvidenceError::UnsafeEvidenceMarker { marker });
        }
    }

    Ok(())
}

/// Bounded explanation for a status selection decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StatusDecisionExplanation {
    target_id: String,
    selected_authority: Option<StatusAuthoritySummary>,
    rejected_candidates: Vec<RejectedStatusCandidate>,
    freshness: StatusFreshness,
    fallback_reason: Option<StatusFallbackReason>,
    evidence_hashes: Vec<String>,
    evidence_summaries: Vec<StatusEvidenceSummary>,
}

impl StatusDecisionExplanation {
    fn new(parts: StatusDecisionExplanationParts) -> Self {
        Self {
            target_id: String::from(parts.target_id),
            selected_authority: parts.selected_authority,
            rejected_candidates: parts.rejected_candidates,
            freshness: parts.freshness,
            fallback_reason: parts.fallback_reason,
            evidence_hashes: parts.evidence_hashes,
            evidence_summaries: vec![parts.evidence_summary],
        }
    }

    /// Returns the target identity for this explanation.
    #[must_use]
    pub fn target_id(&self) -> &str {
        self.target_id.as_str()
    }

    /// Returns the selected authority summary, if any.
    #[must_use]
    pub const fn selected_authority(&self) -> Option<&StatusAuthoritySummary> {
        self.selected_authority.as_ref()
    }

    /// Returns rejected candidates and reasons.
    #[must_use]
    pub fn rejected_candidates(&self) -> &[RejectedStatusCandidate] {
        self.rejected_candidates.as_slice()
    }

    /// Returns the freshness class.
    #[must_use]
    pub const fn freshness(&self) -> StatusFreshness {
        self.freshness
    }

    /// Returns the fallback reason, if the selected state used fallback/unknown.
    #[must_use]
    pub const fn fallback_reason(&self) -> Option<StatusFallbackReason> {
        self.fallback_reason
    }

    /// Returns safe evidence hashes associated with the selected authority.
    #[must_use]
    pub fn evidence_hashes(&self) -> &[String] {
        self.evidence_hashes.as_slice()
    }

    /// Returns bounded redacted evidence summaries.
    #[must_use]
    pub fn evidence_summaries(&self) -> &[StatusEvidenceSummary] {
        self.evidence_summaries.as_slice()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct StatusDecisionExplanationParts<'a> {
    target_id: &'a str,
    selected_authority: Option<StatusAuthoritySummary>,
    rejected_candidates: Vec<RejectedStatusCandidate>,
    freshness: StatusFreshness,
    fallback_reason: Option<StatusFallbackReason>,
    evidence_hashes: Vec<String>,
    evidence_summary: StatusEvidenceSummary,
}

/// Pure status-selection result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AgentStatusSelection {
    target_id: String,
    semantic_state: animus_adapters::SemanticAgentState,
    message: Option<String>,
    presentation: animus_adapters::AgentStatusPresentation,
    explanation: StatusDecisionExplanation,
}

impl AgentStatusSelection {
    fn from_observation(parts: AgentStatusSelectionParts<'_>) -> Self {
        let selected_authority = StatusAuthoritySummary::from_observation(parts.observation);
        let evidence_hashes = parts.observation.evidence_hashes().to_vec();
        Self {
            target_id: String::from(parts.request.target_id()),
            semantic_state: parts.observation.state(),
            message: parts.observation.message().map(String::from),
            presentation: parts.observation.presentation().clone(),
            explanation: StatusDecisionExplanation::new(StatusDecisionExplanationParts {
                target_id: parts.request.target_id(),
                selected_authority: Some(selected_authority),
                rejected_candidates: parts.rejected_candidates,
                freshness: StatusFreshness::Current,
                fallback_reason: parts.fallback_reason,
                evidence_hashes,
                evidence_summary: parts.evidence_summary,
            }),
        }
    }

    fn unknown(
        request: &animus_adapters::HostStatusRequest,
        rejected_candidates: Vec<RejectedStatusCandidate>,
        freshness: StatusFreshness,
        fallback_reason: StatusFallbackReason,
    ) -> Self {
        Self {
            target_id: String::from(request.target_id()),
            semantic_state: animus_adapters::SemanticAgentState::Unknown,
            message: None,
            presentation: animus_adapters::AgentStatusPresentation::default(),
            explanation: StatusDecisionExplanation::new(StatusDecisionExplanationParts {
                target_id: request.target_id(),
                selected_authority: None,
                rejected_candidates,
                freshness,
                fallback_reason: Some(fallback_reason),
                evidence_hashes: Vec::new(),
                evidence_summary: StatusEvidenceSummary::unknown_status(),
            }),
        }
    }

    /// Returns the selected target identity.
    #[must_use]
    pub fn target_id(&self) -> &str {
        self.target_id.as_str()
    }

    /// Returns the selected semantic lifecycle state.
    #[must_use]
    pub const fn semantic_state(&self) -> animus_adapters::SemanticAgentState {
        self.semantic_state
    }

    /// Returns the selected bounded lifecycle message, if any.
    #[must_use]
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns presentation-only metadata from the selected observation.
    #[must_use]
    pub const fn presentation(&self) -> &animus_adapters::AgentStatusPresentation {
        &self.presentation
    }

    /// Returns the bounded explanation for this decision.
    #[must_use]
    pub const fn explanation(&self) -> &StatusDecisionExplanation {
        &self.explanation
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct AgentStatusSelectionParts<'a> {
    request: &'a animus_adapters::HostStatusRequest,
    observation: &'a animus_adapters::AgentStatusObservation,
    rejected_candidates: Vec<RejectedStatusCandidate>,
    fallback_reason: Option<StatusFallbackReason>,
    evidence_summary: StatusEvidenceSummary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct SelectedObservation<'a> {
    observation: &'a animus_adapters::AgentStatusObservation,
    index: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct ClassifiedObservations<'a> {
    trusted: Vec<&'a animus_adapters::AgentStatusObservation>,
    heuristics: Vec<&'a animus_adapters::AgentStatusObservation>,
    saw_target: bool,
    saw_stale_lifecycle: bool,
    saw_untrusted_lifecycle: bool,
}

/// Selects one semantic status authority for a target without ambient I/O.
#[must_use]
pub fn select_agent_status(
    request: &animus_adapters::HostStatusRequest,
    current_sequence: animus_adapters::StatusSequence,
    observations: &[animus_adapters::AgentStatusObservation],
) -> AgentStatusSelection {
    let mut rejected_candidates = Vec::new();
    let classified = classify_observations(
        request,
        current_sequence,
        observations,
        &mut rejected_candidates,
    );

    if !classified.trusted.is_empty() {
        return select_trusted_lifecycle(
            request,
            classified.trusted.as_slice(),
            classified.heuristics.as_slice(),
            rejected_candidates,
        );
    }

    let ClassifiedObservations {
        heuristics,
        saw_target,
        saw_stale_lifecycle,
        saw_untrusted_lifecycle,
        ..
    } = classified;

    select_fallback_or_unknown(
        request,
        heuristics.as_slice(),
        rejected_candidates,
        fallback_reason(saw_target, saw_stale_lifecycle, saw_untrusted_lifecycle),
    )
}

fn classify_observations<'a>(
    request: &animus_adapters::HostStatusRequest,
    current_sequence: animus_adapters::StatusSequence,
    observations: &'a [animus_adapters::AgentStatusObservation],
    rejected_candidates: &mut Vec<RejectedStatusCandidate>,
) -> ClassifiedObservations<'a> {
    let mut trusted = Vec::new();
    let mut heuristics = Vec::new();
    let mut saw_target = false;
    let mut saw_stale_lifecycle = false;
    let mut saw_untrusted_lifecycle = false;

    for observation in observations {
        if observation.target_id() != request.target_id() {
            rejected_candidates.push(RejectedStatusCandidate::new(
                observation,
                RejectedStatusReason::TargetMismatch,
            ));
            continue;
        }
        saw_target = true;

        if !observation.is_current_at(current_sequence) {
            if matches!(
                observation.kind(),
                animus_adapters::AgentStatusObservationKind::TrustedLifecycle
                    | animus_adapters::AgentStatusObservationKind::UntrustedLifecycle
            ) {
                saw_stale_lifecycle = true;
            }
            rejected_candidates.push(RejectedStatusCandidate::new(
                observation,
                RejectedStatusReason::Stale,
            ));
            continue;
        }

        match observation.kind() {
            animus_adapters::AgentStatusObservationKind::TrustedLifecycle => {
                trusted.push(observation);
            }
            animus_adapters::AgentStatusObservationKind::UntrustedLifecycle => {
                saw_untrusted_lifecycle = true;
                rejected_candidates.push(RejectedStatusCandidate::new(
                    observation,
                    RejectedStatusReason::UntrustedLifecycle,
                ));
            }
            animus_adapters::AgentStatusObservationKind::HeuristicFallback => {
                heuristics.push(observation);
            }
        }
    }

    ClassifiedObservations {
        trusted,
        heuristics,
        saw_target,
        saw_stale_lifecycle,
        saw_untrusted_lifecycle,
    }
}

fn select_trusted_lifecycle(
    request: &animus_adapters::HostStatusRequest,
    trusted: &[&animus_adapters::AgentStatusObservation],
    heuristics: &[&animus_adapters::AgentStatusObservation],
    mut rejected_candidates: Vec<RejectedStatusCandidate>,
) -> AgentStatusSelection {
    assert!(
        !trusted.is_empty(),
        "trusted lifecycle selection requires candidates"
    );
    let selected = newest_observation(trusted).expect("trusted candidates are non-empty");
    let conflicting = newest_conflicts(
        trusted,
        selected.observation.sequence(),
        selected.observation.state(),
    );
    if conflicting {
        reject_newest_conflicts(
            trusted,
            selected.observation.sequence(),
            &mut rejected_candidates,
        );
        reject_heuristics(
            heuristics,
            RejectedStatusReason::HeuristicSuppressed,
            &mut rejected_candidates,
        );
        return AgentStatusSelection::unknown(
            request,
            rejected_candidates,
            StatusFreshness::Conflicting,
            StatusFallbackReason::ConflictingLifecycleAuthority,
        );
    }

    reject_non_selected(
        trusted,
        selected,
        RejectedStatusReason::DuplicateLifecycleAuthority,
        &mut rejected_candidates,
    );
    reject_heuristics(
        heuristics,
        RejectedStatusReason::HeuristicSuppressed,
        &mut rejected_candidates,
    );
    AgentStatusSelection::from_observation(AgentStatusSelectionParts {
        request,
        observation: selected.observation,
        rejected_candidates,
        fallback_reason: None,
        evidence_summary: StatusEvidenceSummary::trusted_lifecycle(),
    })
}

fn select_fallback_or_unknown(
    request: &animus_adapters::HostStatusRequest,
    heuristics: &[&animus_adapters::AgentStatusObservation],
    mut rejected_candidates: Vec<RejectedStatusCandidate>,
    reason: StatusFallbackReason,
) -> AgentStatusSelection {
    if let Some(selected) = newest_observation(heuristics) {
        reject_non_selected(
            heuristics,
            selected,
            RejectedStatusReason::OlderSequence,
            &mut rejected_candidates,
        );
        return AgentStatusSelection::from_observation(AgentStatusSelectionParts {
            request,
            observation: selected.observation,
            rejected_candidates,
            fallback_reason: Some(reason),
            evidence_summary: StatusEvidenceSummary::heuristic_fallback(),
        });
    }

    let freshness = match reason {
        StatusFallbackReason::StaleLifecycle => StatusFreshness::Stale,
        StatusFallbackReason::ConflictingLifecycleAuthority => StatusFreshness::Conflicting,
        StatusFallbackReason::NoObservation
        | StatusFallbackReason::NoTrustedLifecycle
        | StatusFallbackReason::UntrustedLifecycle => StatusFreshness::Missing,
    };
    AgentStatusSelection::unknown(request, rejected_candidates, freshness, reason)
}

fn newest_observation<'a>(
    observations: &[&'a animus_adapters::AgentStatusObservation],
) -> Option<SelectedObservation<'a>> {
    observations
        .iter()
        .enumerate()
        .max_by(|left, right| {
            compare_observations(left.1, right.1).then_with(|| right.0.cmp(&left.0))
        })
        .map(|(index, observation)| SelectedObservation { observation, index })
}

fn compare_observations(
    left: &animus_adapters::AgentStatusObservation,
    right: &animus_adapters::AgentStatusObservation,
) -> core::cmp::Ordering {
    left.sequence()
        .cmp(&right.sequence())
        .then_with(|| right.source_id().cmp(left.source_id()))
}

fn newest_conflicts(
    trusted: &[&animus_adapters::AgentStatusObservation],
    sequence: animus_adapters::StatusSequence,
    selected_state: animus_adapters::SemanticAgentState,
) -> bool {
    trusted.iter().any(|observation| {
        observation.sequence() == sequence && observation.state() != selected_state
    })
}

fn reject_newest_conflicts(
    observations: &[&animus_adapters::AgentStatusObservation],
    sequence: animus_adapters::StatusSequence,
    rejected_candidates: &mut Vec<RejectedStatusCandidate>,
) {
    for observation in observations {
        if observation.sequence() == sequence {
            rejected_candidates.push(RejectedStatusCandidate::new(
                observation,
                RejectedStatusReason::ConflictingLifecycleAuthority,
            ));
        }
    }
}

fn reject_non_selected(
    observations: &[&animus_adapters::AgentStatusObservation],
    selected: SelectedObservation<'_>,
    same_sequence_reason: RejectedStatusReason,
    rejected_candidates: &mut Vec<RejectedStatusCandidate>,
) {
    for (index, observation) in observations.iter().enumerate() {
        if index == selected.index {
            continue;
        }
        let reason = if observation.sequence() == selected.observation.sequence() {
            same_sequence_reason
        } else {
            RejectedStatusReason::OlderSequence
        };
        rejected_candidates.push(RejectedStatusCandidate::new(observation, reason));
    }
}

fn reject_heuristics(
    observations: &[&animus_adapters::AgentStatusObservation],
    reason: RejectedStatusReason,
    rejected_candidates: &mut Vec<RejectedStatusCandidate>,
) {
    for observation in observations {
        rejected_candidates.push(RejectedStatusCandidate::new(observation, reason));
    }
}

fn fallback_reason(
    saw_target: bool,
    saw_stale_lifecycle: bool,
    saw_untrusted_lifecycle: bool,
) -> StatusFallbackReason {
    if !saw_target {
        return StatusFallbackReason::NoObservation;
    }
    if saw_stale_lifecycle {
        return StatusFallbackReason::StaleLifecycle;
    }
    if saw_untrusted_lifecycle {
        return StatusFallbackReason::UntrustedLifecycle;
    }
    StatusFallbackReason::NoTrustedLifecycle
}

#[cfg(test)]
mod tests {
    use super::*;

    const TARGET_ID: &str = "pane:alpha";
    const OTHER_TARGET_ID: &str = "pane:beta";
    const LIFECYCLE_SOURCE: &str = "lifecycle:trusted";
    const UNTRUSTED_SOURCE: &str = "lifecycle:untrusted";
    const CONFLICT_SOURCE: &str = "lifecycle:conflict";
    const HEURISTIC_SOURCE: &str = "heuristic:terminal";
    const AGENT_LABEL: &str = "agent-alpha";
    const CURRENT_SEQUENCE: animus_adapters::StatusSequence =
        animus_adapters::StatusSequence::new(12);
    const OLDER_SEQUENCE: animus_adapters::StatusSequence =
        animus_adapters::StatusSequence::new(10);
    const CURRENT_TTL: animus_adapters::StatusTtl =
        animus_adapters::StatusTtl::new(animus_adapters::StatusSequence::new(14));
    const EXPIRED_TTL: animus_adapters::StatusTtl =
        animus_adapters::StatusTtl::new(animus_adapters::StatusSequence::new(11));
    const VALID_HASH: &str = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef";

    fn request() -> animus_adapters::HostStatusRequest {
        animus_adapters::HostStatusRequest::new(TARGET_ID).expect("valid target should parse")
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    struct ObservationSpec {
        kind: &'static str,
        state: &'static str,
        source_id: &'static str,
        target_id: &'static str,
        sequence: animus_adapters::StatusSequence,
        ttl: Option<animus_adapters::StatusTtl>,
    }

    fn observation(spec: ObservationSpec) -> animus_adapters::AgentStatusObservation {
        animus_adapters::AgentStatusObservation::new(animus_adapters::AgentStatusObservationDraft {
            kind: spec.kind,
            state: spec.state,
            source_id: spec.source_id,
            target_id: spec.target_id,
            agent_label: AGENT_LABEL,
            session: None,
            message: Some("bounded status"),
            presentation: animus_adapters::AgentStatusPresentationDraft {
                title: Some("build pane"),
                display_agent_name: Some("Ralph"),
                activity_text: Some("checking"),
                custom_state_label: Some("needs input"),
            },
            sequence: spec.sequence,
            ttl: spec.ttl,
            evidence_hashes: &[VALID_HASH],
        })
        .expect("status observation should parse")
    }

    #[test]
    fn current_trusted_lifecycle_report_suppresses_heuristic_fallback() {
        let lifecycle = observation(ObservationSpec {
            kind: "trusted_lifecycle",
            state: "working",
            source_id: LIFECYCLE_SOURCE,
            target_id: TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });
        let heuristic = observation(ObservationSpec {
            kind: "heuristic_fallback",
            state: "idle",
            source_id: HEURISTIC_SOURCE,
            target_id: TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });

        let selection = select_agent_status(&request(), CURRENT_SEQUENCE, &[lifecycle, heuristic]);

        assert_eq!(
            selection.semantic_state(),
            animus_adapters::SemanticAgentState::Working
        );
        assert_eq!(selection.message(), Some("bounded status"));
        assert_eq!(
            selection.presentation().custom_state_label(),
            Some("needs input")
        );
        let explanation = selection.explanation();
        assert_eq!(explanation.freshness(), StatusFreshness::Current);
        assert_eq!(explanation.fallback_reason(), None);
        assert_eq!(explanation.evidence_hashes(), &[String::from(VALID_HASH)]);
        assert_eq!(
            explanation
                .selected_authority()
                .expect("trusted authority should be selected")
                .source_id(),
            LIFECYCLE_SOURCE
        );
        assert!(explanation.rejected_candidates().iter().any(|candidate| {
            candidate.authority().source_id() == HEURISTIC_SOURCE
                && candidate.reason() == RejectedStatusReason::HeuristicSuppressed
        }));
    }

    #[test]
    fn stale_lifecycle_degrades_to_current_heuristic_fallback() {
        let stale_lifecycle = observation(ObservationSpec {
            kind: "trusted_lifecycle",
            state: "working",
            source_id: LIFECYCLE_SOURCE,
            target_id: TARGET_ID,
            sequence: OLDER_SEQUENCE,
            ttl: Some(EXPIRED_TTL),
        });
        let heuristic = observation(ObservationSpec {
            kind: "heuristic_fallback",
            state: "idle",
            source_id: HEURISTIC_SOURCE,
            target_id: TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });

        let selection =
            select_agent_status(&request(), CURRENT_SEQUENCE, &[stale_lifecycle, heuristic]);

        assert_eq!(
            selection.semantic_state(),
            animus_adapters::SemanticAgentState::Idle
        );
        assert_eq!(
            selection.explanation().fallback_reason(),
            Some(StatusFallbackReason::StaleLifecycle)
        );
        assert!(
            selection
                .explanation()
                .rejected_candidates()
                .iter()
                .any(|candidate| {
                    candidate.authority().source_id() == LIFECYCLE_SOURCE
                        && candidate.reason() == RejectedStatusReason::Stale
                })
        );
    }

    #[test]
    fn untrusted_lifecycle_without_fallback_fails_closed_to_unknown() {
        let untrusted = observation(ObservationSpec {
            kind: "untrusted_lifecycle",
            state: "done",
            source_id: UNTRUSTED_SOURCE,
            target_id: TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });

        let selection = select_agent_status(&request(), CURRENT_SEQUENCE, &[untrusted]);

        assert_eq!(
            selection.semantic_state(),
            animus_adapters::SemanticAgentState::Unknown
        );
        assert_eq!(
            selection.explanation().freshness(),
            StatusFreshness::Missing
        );
        assert_eq!(
            selection.explanation().fallback_reason(),
            Some(StatusFallbackReason::UntrustedLifecycle)
        );
    }

    #[test]
    fn conflicting_current_lifecycle_authorities_fail_closed_to_unknown() {
        let working = observation(ObservationSpec {
            kind: "trusted_lifecycle",
            state: "working",
            source_id: LIFECYCLE_SOURCE,
            target_id: TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });
        let blocked = observation(ObservationSpec {
            kind: "trusted_lifecycle",
            state: "blocked",
            source_id: CONFLICT_SOURCE,
            target_id: TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });
        let heuristic = observation(ObservationSpec {
            kind: "heuristic_fallback",
            state: "idle",
            source_id: HEURISTIC_SOURCE,
            target_id: TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });

        let selection =
            select_agent_status(&request(), CURRENT_SEQUENCE, &[working, blocked, heuristic]);

        assert_eq!(
            selection.semantic_state(),
            animus_adapters::SemanticAgentState::Unknown
        );
        assert_eq!(
            selection.explanation().freshness(),
            StatusFreshness::Conflicting
        );
        assert_eq!(
            selection.explanation().fallback_reason(),
            Some(StatusFallbackReason::ConflictingLifecycleAuthority)
        );
        assert!(
            selection
                .explanation()
                .rejected_candidates()
                .iter()
                .any(|candidate| {
                    candidate.reason() == RejectedStatusReason::ConflictingLifecycleAuthority
                })
        );
    }

    #[test]
    fn custom_presentation_labels_do_not_escalate_semantic_state() {
        let lifecycle = observation(ObservationSpec {
            kind: "trusted_lifecycle",
            state: "working",
            source_id: LIFECYCLE_SOURCE,
            target_id: TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });

        let selection = select_agent_status(&request(), CURRENT_SEQUENCE, &[lifecycle]);

        assert_eq!(
            selection.semantic_state(),
            animus_adapters::SemanticAgentState::Working
        );
        assert_eq!(
            selection.presentation().custom_state_label(),
            Some("needs input")
        );
        assert_ne!(selection.semantic_state().as_str(), "needs input");
    }

    #[test]
    fn explanation_rejects_raw_evidence_leak_fixtures() {
        let raw_prompt_error = StatusEvidenceSummary::new("RAW_PROMPT: summarize credentials")
            .expect_err("raw prompt evidence should be rejected");
        let terminal_error = StatusEvidenceSummary::new("TERMINAL_SNAPSHOT: full pane text")
            .expect_err("terminal snapshots should be rejected");
        let provider_error = StatusEvidenceSummary::new("PROVIDER_PAYLOAD: {json}")
            .expect_err("provider payloads should be rejected");
        let credential_error = StatusEvidenceSummary::new("API_KEY=secret")
            .expect_err("credential-looking summaries should be rejected");
        let detector_error = StatusEvidenceSummary::new("UNBOUNDED_DETECTOR_EVIDENCE: transcript")
            .expect_err("unbounded detector evidence should be rejected");

        assert_eq!(
            raw_prompt_error,
            StatusEvidenceError::UnsafeEvidenceMarker {
                marker: RAW_PROMPT_MARKER,
            }
        );
        assert_eq!(
            terminal_error,
            StatusEvidenceError::UnsafeEvidenceMarker {
                marker: TERMINAL_SNAPSHOT_MARKER,
            }
        );
        assert_eq!(
            provider_error,
            StatusEvidenceError::UnsafeEvidenceMarker {
                marker: PROVIDER_PAYLOAD_MARKER,
            }
        );
        assert_eq!(
            credential_error,
            StatusEvidenceError::UnsafeEvidenceMarker {
                marker: API_KEY_MARKER,
            }
        );
        assert_eq!(
            detector_error,
            StatusEvidenceError::UnsafeEvidenceMarker {
                marker: UNBOUNDED_DETECTOR_MARKER,
            }
        );
    }

    #[test]
    fn selection_consumes_typed_observations_without_ambient_inputs() {
        let other_target = observation(ObservationSpec {
            kind: "trusted_lifecycle",
            state: "blocked",
            source_id: LIFECYCLE_SOURCE,
            target_id: OTHER_TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });
        let heuristic = observation(ObservationSpec {
            kind: "heuristic_fallback",
            state: "idle",
            source_id: HEURISTIC_SOURCE,
            target_id: TARGET_ID,
            sequence: CURRENT_SEQUENCE,
            ttl: Some(CURRENT_TTL),
        });
        let observations = [other_target, heuristic];

        let first_selection = select_agent_status(&request(), CURRENT_SEQUENCE, &observations);
        let second_selection = select_agent_status(&request(), CURRENT_SEQUENCE, &observations);

        assert_eq!(first_selection, second_selection);
        assert_eq!(
            first_selection.semantic_state(),
            animus_adapters::SemanticAgentState::Idle
        );
        assert!(
            first_selection
                .explanation()
                .rejected_candidates()
                .iter()
                .any(|candidate| { candidate.reason() == RejectedStatusReason::TargetMismatch })
        );
    }
}
