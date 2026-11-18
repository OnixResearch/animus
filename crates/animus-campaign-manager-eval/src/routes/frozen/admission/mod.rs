//! The versioned frozen-policy artifact and its admission before any scoring.
//!
//! The canonical encoding is the compact JSON of [`ArtifactDocument`], whose keys are declared in
//! sorted order at every level, so re-encoding never depends on the input's key order or spacing.
//! The identity is Animus's domain-separated `b3:` identity of that encoding under the schema.

/// Artifact schema the route admits.
pub const ARTIFACT_SCHEMA: &str = "animus.campaign-manager-local-scorer.v1";
/// The only policy kind: fixed-point linear scoring over per-mille features.
pub const POLICY_KIND: &str = "linear-per-mille-v1";
/// Feature schema the projection implements.
pub const FEATURE_SCHEMA: &str = "animus.campaign-manager-brief-features.v1";
/// Action schema the reply implements.
pub const ACTION_SCHEMA: &str = "animus.campaign-manager-rank-action.v1";
/// Artifact API version the route supports.
pub const ARTIFACT_API: u32 = 1;
/// Runtime crate an artifact must name.
const RUNTIME_CRATE: &str = "animus-campaign-manager-eval";
/// Largest weight, bias, or rank-bound magnitude.
pub const MAX_PARAMETER_MAGNITUDE: i64 = 1_000_000;
/// Largest declared operation budget per decision.
const MAX_OPERATION_BUDGET: u64 = 1_000_000;
/// Largest artifact encoding admitted, in bytes.
const MAX_ARTIFACT_BYTES: usize = 16_384;

/// Inclusive rank bounds with `min < max`, each within [`MAX_PARAMETER_MAGNITUDE`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RankBounds {
    min: i64,
    max: i64,
}

impl RankBounds {
    /// Bounds from `min` to `max`, or `None` unless `min < max` within the parameter magnitude.
    #[must_use]
    pub const fn new(min: i64, max: i64) -> Option<Self> {
        let is_bounded = min >= -MAX_PARAMETER_MAGNITUDE && max <= MAX_PARAMETER_MAGNITUDE;
        if is_bounded && min < max {
            Some(Self { min, max })
        } else {
            None
        }
    }

    /// The smallest admitted rank.
    #[must_use]
    pub const fn min(self) -> i64 {
        self.min
    }

    /// The largest admitted rank.
    #[must_use]
    pub const fn max(self) -> i64 {
        self.max
    }

    /// Whether `rank` lies within the bounds.
    #[must_use]
    pub const fn contains(self, rank: i64) -> bool {
        rank >= self.min && rank <= self.max
    }

    /// Whether every rank within the bounds fits the presented `magnitude`.
    #[must_use]
    pub fn fits(self, magnitude: u32) -> bool {
        let magnitude = u64::from(magnitude);
        self.min.unsigned_abs() <= magnitude && self.max.unsigned_abs() <= magnitude
    }
}

/// An artifact admitted for scoring.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedArtifact {
    /// The artifact's canonical `b3:` identity.
    pub identity: String,
    /// One weight per declared feature, in projection order.
    pub weights: [i64; super::features::FEATURE_COUNT],
    /// The score offset.
    pub bias: i64,
    /// Inclusive rank bounds of every score.
    pub rank_bounds: RankBounds,
    /// Largest number of inference operations one decision may take.
    pub operation_budget: u64,
}

/// Why an artifact was refused before any scoring.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactError {
    /// No artifact bytes were supplied.
    Unavailable,
    /// The encoding exceeds the admitted size.
    TooLarge,
    /// The bytes are not one artifact document, or hold unknown fields.
    Malformed,
    /// Another artifact schema.
    Schema,
    /// Another policy kind.
    Policy,
    /// Another feature schema or feature list.
    Features,
    /// Another action schema.
    Action,
    /// Weights or bias are missing, extra, or out of bounds.
    Parameters,
    /// Rank bounds are inverted or out of bounds.
    RankBounds,
    /// Another runtime crate or API, or an invalid operation budget.
    Runtime,
    /// The canonical identity differs from the selected identity.
    Identity,
}

/// The artifact document, keys in sorted order at every level.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct ArtifactDocument {
    action_schema: String,
    bias: i64,
    feature_schema: String,
    features: Vec<String>,
    policy: String,
    provenance: Provenance,
    rank_bounds: Bounds,
    runtime: Runtime,
    schema: String,
    weights: Vec<i64>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct Provenance {
    evaluation: String,
    kind: String,
    non_claims: Vec<String>,
    producer: String,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct Bounds {
    max: i64,
    min: i64,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct Runtime {
    artifact_api: u32,
    #[serde(rename = "crate")]
    crate_name: String,
    operation_budget: u64,
}

/// The canonical `b3:` identity of one artifact encoding.
///
/// # Errors
///
/// Returns [`ArtifactError::TooLarge`] or [`ArtifactError::Malformed`] for bytes that are not one
/// bounded artifact document.
pub fn artifact_identity(bytes: &[u8]) -> Result<String, ArtifactError> {
    let document = parse(bytes)?;
    canonical_identity(&document)
}

/// Admits the artifact `bytes` when their canonical identity equals `expected_identity`.
///
/// # Errors
///
/// Returns the first [`ArtifactError`] that refuses the artifact; no score is computed.
// r[impl agent_harness_runtime.campaign_manager_eval.local_scorer.artifact]
pub fn admit_artifact(
    bytes: &[u8],
    expected_identity: &str,
) -> Result<AdmittedArtifact, ArtifactError> {
    let document = parse(bytes)?;
    check_contracts(&document)?;
    let weights: [i64; super::features::FEATURE_COUNT] = document
        .weights
        .as_slice()
        .try_into()
        .map_err(|_| ArtifactError::Parameters)?;
    if !is_parameter(document.bias) || !weights.iter().copied().all(is_parameter) {
        return Err(ArtifactError::Parameters);
    }
    let rank_bounds = RankBounds::new(document.rank_bounds.min, document.rank_bounds.max)
        .ok_or(ArtifactError::RankBounds)?;
    let identity = canonical_identity(&document)?;
    if identity != expected_identity {
        return Err(ArtifactError::Identity);
    }
    debug_assert!(animus_eval_core::validate_blake3_identity(&identity).is_ok());
    Ok(AdmittedArtifact {
        identity,
        weights,
        bias: document.bias,
        rank_bounds,
        operation_budget: document.runtime.operation_budget,
    })
}

fn parse(bytes: &[u8]) -> Result<ArtifactDocument, ArtifactError> {
    if bytes.is_empty() {
        return Err(ArtifactError::Unavailable);
    }
    if bytes.len() > MAX_ARTIFACT_BYTES {
        return Err(ArtifactError::TooLarge);
    }
    serde_json::from_slice(bytes).map_err(|_| ArtifactError::Malformed)
}

fn check_contracts(document: &ArtifactDocument) -> Result<(), ArtifactError> {
    if document.schema != ARTIFACT_SCHEMA {
        return Err(ArtifactError::Schema);
    }
    if document.policy != POLICY_KIND {
        return Err(ArtifactError::Policy);
    }
    let is_declared_projection = document.feature_schema == FEATURE_SCHEMA
        && document
            .features
            .iter()
            .map(String::as_str)
            .eq(super::features::FEATURE_NAMES);
    if !is_declared_projection {
        return Err(ArtifactError::Features);
    }
    if document.action_schema != ACTION_SCHEMA {
        return Err(ArtifactError::Action);
    }
    let runtime = &document.runtime;
    if runtime.crate_name != RUNTIME_CRATE {
        return Err(ArtifactError::Runtime);
    }
    if runtime.artifact_api != ARTIFACT_API {
        return Err(ArtifactError::Runtime);
    }
    if !(1..=MAX_OPERATION_BUDGET).contains(&runtime.operation_budget) {
        return Err(ArtifactError::Runtime);
    }
    debug_assert_eq!(document.features.len(), super::features::FEATURE_COUNT);
    debug_assert!(document.runtime.operation_budget != 0);
    Ok(())
}

fn canonical_identity(document: &ArtifactDocument) -> Result<String, ArtifactError> {
    let canonical = serde_json::to_string(document).map_err(|_| ArtifactError::Malformed)?;
    Ok(animus_eval_core::hash_identity(
        ARTIFACT_SCHEMA,
        &[&canonical],
    ))
}

const fn is_parameter(value: i64) -> bool {
    value >= -MAX_PARAMETER_MAGNITUDE && value <= MAX_PARAMETER_MAGNITUDE
}
