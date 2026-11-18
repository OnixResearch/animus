//! The frozen local scorer route and its random-ranking baseline.
//!
//! The scorer reads only the rendered brief, projects the declared features of every listed
//! candidate, scores them with an admitted frozen policy, and replies with candidate scores only:
//! it adds no offers and retires, prunes, or stops nothing. Every call records its decision; a call
//! the scorer cannot answer falls back to the declared `explore-only` reply and says so.

mod admission;
mod baseline;
mod features;

pub use admission::{
    ACTION_SCHEMA, ARTIFACT_API, ARTIFACT_SCHEMA, AdmittedArtifact, ArtifactError, FEATURE_SCHEMA,
    MAX_PARAMETER_MAGNITUDE, POLICY_KIND, RankBounds, admit_artifact, artifact_identity,
};
pub use baseline::RandomRankRoute;
pub use features::{FEATURE_COUNT, FEATURE_NAMES};

/// Decisions one route retains; later calls are refused.
pub const MAX_DECISIONS: usize = 4_096;
/// The declared fallback policy, with the scorer's grant.
pub const FALLBACK_POLICY: super::ScriptedPolicy = super::ScriptedPolicy::ExploreOnly;
/// Hexadecimal characters of the artifact identity in the route identity.
const ROUTE_IDENTITY_DIGEST_CHARS: usize = 16;

/// What one call of the scorer did.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Disposition {
    /// The admitted policy scored every listed candidate.
    Scored,
    /// The brief lists no candidate, too many, an out-of-schema signal, or no rank magnitude.
    MalformedBrief,
    /// A score, or the artifact's rank bounds, fall outside the presented rank domain.
    OutOfRange,
    /// Scoring would exceed the artifact's declared operation budget.
    OperationBudget,
    /// The artifact was refused at admission.
    IncompatibleArtifact,
    /// No artifact was supplied.
    Unavailable,
}

/// Deterministic cost units of one call.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CostUnits {
    /// Brief bytes read.
    pub brief_bytes: u64,
    /// Feature values projected.
    pub feature_operations: u64,
    /// Weight-feature products summed.
    pub inference_operations: u64,
    /// Candidates the fallback policy answered for.
    pub fallback_candidates: u64,
}

/// The retained record of one call.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DecisionRecord {
    /// The call's index on this route.
    pub call: u32,
    /// The admitted artifact identity, if any.
    pub artifact: Option<String>,
    /// The projected feature rows, in brief order; empty when nothing was projected.
    pub features: Vec<[i64; FEATURE_COUNT]>,
    /// The policy's scores, in brief order; empty unless scored.
    pub scores: Vec<i64>,
    /// What the call did.
    pub disposition: Disposition,
    /// The policy that answered instead of the scorer, if any.
    pub fallback: Option<super::ScriptedPolicy>,
    /// The call's cost units.
    pub cost: CostUnits,
}

/// A manager route answered by an admitted frozen local policy.
pub struct LocalScorerRoute {
    identity: String,
    artifact: Result<AdmittedArtifact, ArtifactError>,
    decisions: Vec<DecisionRecord>,
}

/// The pure outcome of one decision, before the reply is rendered.
struct Decided {
    handles: Vec<String>,
    features: Vec<[i64; FEATURE_COUNT]>,
    scores: Vec<i64>,
    disposition: Disposition,
}

impl LocalScorerRoute {
    /// A route over `artifact`. A refused artifact gives a route whose every call falls back and
    /// records the refusal.
    #[must_use]
    pub fn new(artifact: Result<AdmittedArtifact, ArtifactError>) -> Self {
        let identity = match &artifact {
            Ok(admitted) => {
                let digest = admitted
                    .identity
                    .trim_start_matches(animus_eval_core::BLAKE3_PREFIX);
                let short = digest.get(..ROUTE_IDENTITY_DIGEST_CHARS).unwrap_or(digest);
                format!("model:local-scorer:{short}")
            }
            Err(_) => String::from("model:local-scorer:refused"),
        };
        debug_assert!(animus_eval_core::is_safe_identity(&identity));
        Self {
            identity,
            artifact,
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
    pub fn decisions(&self) -> &[DecisionRecord] {
        &self.decisions
    }

    /// Answers one rendered brief and records the decision.
    ///
    /// # Errors
    ///
    /// Returns [`super::RouteFailure::Rejected`] once [`MAX_DECISIONS`] records are retained.
    // r[impl agent_harness_runtime.campaign_manager_eval.local_scorer.route]
    // r[impl agent_harness_runtime.campaign_manager_eval.local_scorer.record]
    pub fn answer(&mut self, prompt: &str) -> Result<String, super::RouteFailure> {
        if self.decisions.len() >= MAX_DECISIONS {
            return Err(super::RouteFailure::Rejected);
        }
        let call =
            u32::try_from(self.decisions.len()).map_err(|_| super::RouteFailure::Rejected)?;
        let view = super::reading::read_prompt(prompt);
        let decided = decide(self.artifact.as_ref(), &view);
        let mut cost = CostUnits {
            brief_bytes: count(prompt.len())?,
            feature_operations: count(decided.features.len().saturating_mul(FEATURE_COUNT))?,
            inference_operations: count(decided.scores.len().saturating_mul(FEATURE_COUNT))?,
            fallback_candidates: 0,
        };
        let (reply, fallback) = if decided.disposition == Disposition::Scored {
            (render(&decided, view.dimensions), None)
        } else {
            cost.fallback_candidates = count(view.candidates.len())?;
            (FALLBACK_POLICY.reply(prompt), Some(FALLBACK_POLICY))
        };
        debug_assert_eq!(
            fallback.is_none(),
            decided.disposition == Disposition::Scored
        );
        self.decisions.push(DecisionRecord {
            call,
            artifact: self
                .artifact
                .as_ref()
                .ok()
                .map(|admitted| admitted.identity.clone()),
            features: decided.features,
            scores: decided.scores,
            disposition: decided.disposition,
            fallback,
            cost,
        });
        Ok(reply)
    }
}

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for LocalScorerRoute
{
    type Error = super::RouteFailure;

    fn call(
        &mut self,
        request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        let reply = self.answer(request.prompt())?;
        animus_adapters::ModelResponse::new(&reply).map_err(|_| super::RouteFailure::Malformed)
    }
}

/// Decides one brief under `artifact`, without rendering or recording.
// r[impl agent_harness_runtime.campaign_manager_eval.local_scorer.fallback]
fn decide(
    artifact: Result<&AdmittedArtifact, &ArtifactError>,
    view: &super::reading::PromptView,
) -> Decided {
    let admitted = match artifact {
        Ok(admitted) => admitted,
        Err(ArtifactError::Unavailable) => return Decided::refused(Disposition::Unavailable),
        Err(_) => return Decided::refused(Disposition::IncompatibleArtifact),
    };
    let Some(magnitude) = view.magnitude else {
        return Decided::refused(Disposition::MalformedBrief);
    };
    let Ok(projection) = features::project(view) else {
        return Decided::refused(Disposition::MalformedBrief);
    };
    let mut decided = Decided {
        handles: projection.handles,
        features: projection.rows,
        scores: Vec::new(),
        disposition: Disposition::OutOfRange,
    };
    if !admitted.rank_bounds.fits(magnitude) {
        return decided;
    }
    let operations = decided.features.len().saturating_mul(FEATURE_COUNT);
    let is_within_budget =
        u64::try_from(operations).is_ok_and(|operations| operations <= admitted.operation_budget);
    if !is_within_budget {
        decided.disposition = Disposition::OperationBudget;
        return decided;
    }
    let scores: Option<Vec<i64>> = decided
        .features
        .iter()
        .map(|row| score(admitted, row))
        .collect();
    let Some(scores) = scores else {
        return decided;
    };
    if scores
        .iter()
        .all(|score| admitted.rank_bounds.contains(*score))
    {
        decided.disposition = Disposition::Scored;
        decided.scores = scores;
    }
    debug_assert!(
        decided.disposition != Disposition::Scored || decided.scores.len() == decided.handles.len()
    );
    decided
}

/// `bias + (sum of weight * feature) / 1000` in checked arithmetic; division truncates toward zero.
fn score(artifact: &AdmittedArtifact, row: &[i64; FEATURE_COUNT]) -> Option<i64> {
    let mut total: i64 = 0;
    for (weight, feature) in artifact.weights.iter().zip(row) {
        total = total.checked_add(weight.checked_mul(*feature)?)?;
    }
    artifact
        .bias
        .checked_add(total.checked_div(features::PER_MILLE)?)
}

/// The scored reply: one score per listed candidate in brief order, and no offers.
fn render(decided: &Decided, dimensions: u32) -> String {
    let scores: Vec<String> = decided
        .handles
        .iter()
        .zip(&decided.scores)
        .map(|(handle, score)| {
            let ranks = super::reading::ranks(*score, dimensions);
            format!(r#"{{"candidate":"{handle}","ranks":{ranks}}}"#)
        })
        .collect();
    debug_assert_eq!(scores.len(), decided.handles.len());
    format!(r#"{{"scores":[{}],"add":[]}}"#, scores.join(","))
}

impl Decided {
    /// A decision that projected nothing.
    const fn refused(disposition: Disposition) -> Self {
        Self {
            handles: Vec::new(),
            features: Vec::new(),
            scores: Vec::new(),
            disposition,
        }
    }
}

/// `value` as a cost unit; a count past `u64` refuses the call rather than saturating.
fn count(value: usize) -> Result<u64, super::RouteFailure> {
    u64::try_from(value).map_err(|_| super::RouteFailure::Rejected)
}
