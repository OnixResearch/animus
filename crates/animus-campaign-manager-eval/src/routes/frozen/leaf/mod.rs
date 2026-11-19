//! The frozen-policy route: each listed candidate is classified by an admitted
//! `animus.frozen-policy.v1` artifact through the shared `animus-frozen-policy` leaf, and the
//! route maps each class to a rank.
//!
//! The route projects the same brief features as the linear scorer, decides each candidate with
//! every class eligible and no per-candidate fallback, and answers only when every candidate was
//! selected. Any refusal answers the whole brief with `explore-only` and records why. The route
//! owns the class meaning; the leaf owns admission, evaluation, selection, and receipts.

/// Action schema of the candidate classes.
pub const CLASS_ACTION_SCHEMA: &str = "animus.campaign-manager-candidate-class.v1";
/// Candidate classes in declared order: rank by the observed signal, explore, or exploit.
pub const CLASS_ACTIONS: [&str; 3] = ["follow-signal", "explore", "exploit"];
/// Inference operations one candidate's decision may take: the deterministic deadline.
pub const CANDIDATE_OPERATIONS: u64 = 64;
/// Hexadecimal characters of the artifact identity in the route identity.
const ROUTE_IDENTITY_DIGEST_CHARS: usize = 16;

/// A manager route whose candidates are classified by an admitted frozen policy.
pub struct ClassifierRoute {
    identity: String,
    binding: animus_frozen_policy::PolicyBinding,
    policy: Result<animus_frozen_policy::AdmittedPolicy, animus_frozen_policy::AdmissionError>,
    decisions: Vec<super::DecisionRecord>,
}

/// One brief's per-candidate decisions, before the reply is rendered.
struct Classified {
    decided: super::Decided,
    receipts: Vec<String>,
    normalizations: u64,
    inferences: u64,
}

impl ClassifierRoute {
    /// A route over `policy`, pinned to the `selected` artifact identity. A refused artifact, or
    /// one whose actions are not [`CLASS_ACTIONS`], gives a route whose every call falls back and
    /// records the refusal.
    #[must_use]
    pub fn new(
        policy: Result<animus_frozen_policy::AdmittedPolicy, animus_frozen_policy::AdmissionError>,
        selected: &str,
    ) -> Self {
        let policy = policy.and_then(|admitted| {
            if admitted
                .actions()
                .iter()
                .map(String::as_str)
                .eq(CLASS_ACTIONS)
            {
                Ok(admitted)
            } else {
                Err(animus_frozen_policy::AdmissionError::ActionSchema)
            }
        });
        let identity = match &policy {
            Ok(admitted) => {
                let digest = admitted
                    .identity()
                    .trim_start_matches(animus_eval_core::BLAKE3_PREFIX);
                let short = digest.get(..ROUTE_IDENTITY_DIGEST_CHARS).unwrap_or(digest);
                format!("model:frozen-policy:{short}")
            }
            Err(_) => String::from("model:frozen-policy:refused"),
        };
        debug_assert!(animus_eval_core::is_safe_identity(&identity));
        Self {
            identity,
            binding: animus_frozen_policy::PolicyBinding {
                action_schema: String::from(CLASS_ACTION_SCHEMA),
                artifact: String::from(selected),
                feature_schema: String::from(super::FEATURE_SCHEMA),
            },
            policy,
            decisions: Vec::new(),
        }
    }

    /// The route's Harbor model-route identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        &self.identity
    }

    /// The retained decision records, in call order.
    #[must_use]
    pub fn decisions(&self) -> &[super::DecisionRecord] {
        &self.decisions
    }

    /// The ranks the route can emit, zero to the exploit rank; a budget-matched random arm draws
    /// from the same bounds. `None` only if the route's own rank constants were inverted.
    #[must_use]
    pub const fn rank_bounds() -> Option<super::RankBounds> {
        super::RankBounds::new(0, super::super::EXPLOIT_RANK)
    }

    /// Answers one rendered brief and records the decision.
    ///
    /// # Errors
    ///
    /// Returns [`super::super::RouteFailure::Rejected`] once [`super::MAX_DECISIONS`] records are
    /// retained.
    // r[impl agent_harness_runtime.frozen_policy.campaign_route]
    pub fn answer(&mut self, prompt: &str) -> Result<String, super::super::RouteFailure> {
        if self.decisions.len() >= super::MAX_DECISIONS {
            return Err(super::super::RouteFailure::Rejected);
        }
        let call = u32::try_from(self.decisions.len())
            .map_err(|_| super::super::RouteFailure::Rejected)?;
        let view = super::super::reading::read_prompt(prompt);
        let classified = self.classify(&view);
        let decided = classified.decided;
        let mut cost = super::CostUnits {
            brief_bytes: super::count(prompt.len())?,
            feature_operations: classified.normalizations,
            inference_operations: classified.inferences,
            fallback_candidates: 0,
        };
        let (reply, fallback) = if decided.disposition == super::Disposition::Scored {
            (super::render(&decided, view.dimensions), None)
        } else {
            cost.fallback_candidates = super::count(view.candidates.len())?;
            (
                super::FALLBACK_POLICY.reply(prompt),
                Some(super::FALLBACK_POLICY),
            )
        };
        self.decisions.push(super::DecisionRecord {
            call,
            artifact: self
                .policy
                .as_ref()
                .ok()
                .map(|admitted| String::from(admitted.identity())),
            features: decided.features,
            scores: decided.scores,
            disposition: decided.disposition,
            fallback,
            cost,
            receipts: classified.receipts,
        });
        Ok(reply)
    }

    /// Classifies every listed candidate of `view`, stopping at the first refusal.
    fn classify(&self, view: &super::super::reading::PromptView) -> Classified {
        let mut classified = Classified {
            decided: super::Decided::refused(super::Disposition::MalformedBrief),
            receipts: Vec::new(),
            normalizations: 0,
            inferences: 0,
        };
        let admitted = match &self.policy {
            Ok(admitted) => admitted,
            Err(animus_frozen_policy::AdmissionError::Unavailable) => {
                classified.decided.disposition = super::Disposition::Unavailable;
                return classified;
            }
            Err(_) => {
                classified.decided.disposition = super::Disposition::IncompatibleArtifact;
                return classified;
            }
        };
        let Some(magnitude) = view.magnitude else {
            return classified;
        };
        let Ok(projection) = super::features::project(view) else {
            return classified;
        };
        let mut ranks = Vec::with_capacity(projection.rows.len());
        let mut disposition = super::Disposition::Scored;
        for (row, listed) in projection.rows.iter().zip(&view.candidates) {
            let decision = animus_frozen_policy::decide(Ok(admitted), &self.request(row));
            let cost = decision.receipt.body.cost;
            classified.normalizations = classified
                .normalizations
                .saturating_add(cost.normalization_operations);
            classified.inferences = classified
                .inferences
                .saturating_add(cost.inference_operations);
            classified.receipts.push(decision.receipt.identity);
            match class_rank(&decision.outcome, listed.signal, magnitude) {
                Ok(rank) => ranks.push(rank),
                Err(refused) => {
                    disposition = refused;
                    break;
                }
            }
        }
        debug_assert!(classified.receipts.len() <= projection.rows.len());
        classified.decided = super::Decided {
            handles: projection.handles,
            features: projection.rows,
            scores: if disposition == super::Disposition::Scored {
                ranks
            } else {
                Vec::new()
            },
            disposition,
        };
        classified
    }

    /// The leaf request of one candidate's feature row: every class eligible, no fallback.
    fn request(&self, row: &[i64; super::FEATURE_COUNT]) -> animus_frozen_policy::DecisionRequest {
        animus_frozen_policy::DecisionRequest {
            binding: self.binding.clone(),
            features: row.to_vec(),
            eligible: CLASS_ACTIONS
                .iter()
                .map(|class| String::from(*class))
                .collect(),
            fallback: None,
            max_operations: CANDIDATE_OPERATIONS,
        }
    }
}

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for ClassifierRoute
{
    type Error = super::super::RouteFailure;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        let reply = self.answer(request.prompt())?;
        animus_adapters::ModelResponse::new(&reply)
            .map_err(|_| super::super::RouteFailure::Malformed)
    }
}

/// The rank of one candidate's selected class, or the disposition of a refusal.
fn class_rank(
    outcome: &animus_frozen_policy::Outcome,
    signal: Option<u32>,
    magnitude: u32,
) -> Result<i64, super::Disposition> {
    let action = match outcome {
        animus_frozen_policy::Outcome::Selected { action, .. } => action.as_str(),
        animus_frozen_policy::Outcome::Fallback { failure, .. }
        | animus_frozen_policy::Outcome::Refused { failure } => {
            return Err(disposition(*failure));
        }
    };
    let rank = match action {
        "follow-signal" => signal.map_or(0, i64::from),
        "explore" => super::super::EXPLORE_RANK,
        "exploit" => super::super::EXPLOIT_RANK,
        _ => return Err(super::Disposition::IncompatibleArtifact),
    };
    if rank.unsigned_abs() > u64::from(magnitude) {
        return Err(super::Disposition::OutOfRange);
    }
    Ok(rank)
}

/// The route disposition of one leaf failure.
const fn disposition(failure: animus_frozen_policy::Failure) -> super::Disposition {
    match failure {
        animus_frozen_policy::Failure::Unavailable => super::Disposition::Unavailable,
        animus_frozen_policy::Failure::IncompatibleArtifact(_) => {
            super::Disposition::IncompatibleArtifact
        }
        animus_frozen_policy::Failure::Stale(_) => super::Disposition::Stale,
        animus_frozen_policy::Failure::MalformedInput(_) => super::Disposition::MalformedBrief,
        animus_frozen_policy::Failure::InputOutOfRange { .. }
        | animus_frozen_policy::Failure::ScoreOutOfRange => super::Disposition::OutOfRange,
        animus_frozen_policy::Failure::OperationBudget => super::Disposition::OperationBudget,
        animus_frozen_policy::Failure::Abstained => super::Disposition::Abstained,
    }
}
