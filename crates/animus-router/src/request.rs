const PRIVACY_LOCAL_ONLY: &str = "local_only";
const PRIVACY_REMOTE_ALLOWED: &str = "remote_allowed";
const FALLBACK_ELIGIBLE: &str = "eligible";
const FALLBACK_INELIGIBLE: &str = "ineligible";

const TASK_SIMPLE: &str = "simple";
const TASK_REASONING: &str = "reasoning";
const TASK_CODING: &str = "coding";
const TASK_REVIEW: &str = "review";
const TASK_WORKFLOW: &str = "workflow";

const BAND_LOW: &str = "low";
const BAND_MEDIUM: &str = "medium";
const BAND_HIGH: &str = "high";
const BAND_STRICT: &str = "strict";

const EMPTY_COUNT: u32 = 0;
const WORKFLOW_TOOL_CALL_THRESHOLD: u32 = 2;
const HIGH_TOOL_CALL_THRESHOLD: u32 = 1;
const MEDIUM_PROMPT_CHAR_THRESHOLD: u32 = 2_000;
const HIGH_PROMPT_CHAR_THRESHOLD: u32 = 8_000;
const STRICT_PROMPT_CHAR_THRESHOLD: u32 = 24_000;
const REVIEW_PROMPT_CHAR_THRESHOLD: u32 = 4_000;
const MEDIUM_CONTEXT_PRESSURE_PER_MILLE: u16 = 400;
const HIGH_CONTEXT_PRESSURE_PER_MILLE: u16 = 700;
const STRICT_CONTEXT_PRESSURE_PER_MILLE: u16 = 900;
const MAX_CONTEXT_PRESSURE_PER_MILLE: u16 = 1_000;

const UNSAFE_SENTINELS: &[&str] = &[
    "prompt:",
    "provider payload",
    "api_key",
    "credential",
    "secret",
    "\n",
];

/// Bounded request metadata consumed by the default router signal projector.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RequestMetadata {
    prompt_char_estimate: u32,
    tool_call_count: u32,
    contains_code: bool,
    requires_structured_output: bool,
    context_window_fill_per_mille: u16,
    privacy_policy: PrivacyPolicy,
    fallback_hint: FallbackHint,
}

impl RequestMetadata {
    /// Builds bounded request metadata from host-supplied safe fields.
    ///
    /// # Errors
    ///
    /// Returns [`SignalProjectionError`] when metadata is empty, privacy is
    /// missing or unsupported, fallback is unsupported, context pressure is out
    /// of range, or a label resembles an unsafe raw payload.
    pub fn new(draft: RequestMetadataDraft<'_>) -> Result<Self, SignalProjectionError> {
        validate_non_empty(&draft)?;
        validate_context_pressure(draft.context_window_fill_per_mille)?;
        reject_unsafe_label("privacy_policy", draft.privacy_policy)?;
        reject_unsafe_label("fallback_hint", draft.fallback_hint)?;
        Ok(Self {
            prompt_char_estimate: draft.prompt_char_estimate,
            tool_call_count: draft.tool_call_count,
            contains_code: draft.contains_code,
            requires_structured_output: draft.requires_structured_output,
            context_window_fill_per_mille: draft.context_window_fill_per_mille,
            privacy_policy: PrivacyPolicy::parse(draft.privacy_policy)?,
            fallback_hint: FallbackHint::parse(draft.fallback_hint)?,
        })
    }

    /// Returns the bounded prompt character estimate.
    #[must_use]
    pub const fn prompt_char_estimate(&self) -> u32 {
        self.prompt_char_estimate
    }

    /// Returns the bounded tool-call count.
    #[must_use]
    pub const fn tool_call_count(&self) -> u32 {
        self.tool_call_count
    }

    /// Returns whether code-like content was observed by the host.
    #[must_use]
    pub const fn contains_code(&self) -> bool {
        self.contains_code
    }

    /// Returns whether structured output was requested.
    #[must_use]
    pub const fn requires_structured_output(&self) -> bool {
        self.requires_structured_output
    }

    /// Returns context-window fill in per-mille units.
    #[must_use]
    pub const fn context_window_fill_per_mille(&self) -> u16 {
        self.context_window_fill_per_mille
    }
}

/// Borrowed fields used to build [`RequestMetadata`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RequestMetadataDraft<'a> {
    /// Bounded prompt character estimate supplied by the host.
    pub prompt_char_estimate: u32,
    /// Bounded count of declared tool calls.
    pub tool_call_count: u32,
    /// Whether the host observed code-like content without passing raw text.
    pub contains_code: bool,
    /// Whether the request demands structured output.
    pub requires_structured_output: bool,
    /// Context-window fill in per-mille units, from 0 to 1000 inclusive.
    pub context_window_fill_per_mille: u16,
    /// Privacy/locality policy label.
    pub privacy_policy: &'a str,
    /// Fallback eligibility hint label.
    pub fallback_hint: &'a str,
}

/// Error returned while projecting request metadata into router signals.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SignalProjectionError {
    /// Metadata carried no useful bounded signal.
    EmptyMetadata,
    /// Privacy policy label was missing.
    MissingPrivacyPolicy,
    /// Privacy policy label was not supported.
    UnsupportedPrivacyPolicy,
    /// Fallback hint label was not supported.
    UnsupportedFallbackHint,
    /// Context-window fill exceeded the per-mille range.
    ContextWindowOutOfRange {
        /// Maximum accepted context fill per-mille value.
        max_per_mille: u16,
        /// Actual context fill per-mille value.
        actual_per_mille: u16,
    },
    /// A label looked like raw prompt/provider/credential material.
    UnsafePayload {
        /// Field that carried unsafe-looking text.
        field: &'static str,
    },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum PrivacyPolicy {
    LocalOnly,
    RemoteAllowed,
}

impl PrivacyPolicy {
    fn parse(value: &str) -> Result<Self, SignalProjectionError> {
        match value.trim() {
            "" => Err(SignalProjectionError::MissingPrivacyPolicy),
            PRIVACY_LOCAL_ONLY => Ok(Self::LocalOnly),
            PRIVACY_REMOTE_ALLOWED => Ok(Self::RemoteAllowed),
            _ => Err(SignalProjectionError::UnsupportedPrivacyPolicy),
        }
    }

    const fn as_signal(self) -> &'static str {
        match self {
            Self::LocalOnly => PRIVACY_LOCAL_ONLY,
            Self::RemoteAllowed => PRIVACY_REMOTE_ALLOWED,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FallbackHint {
    Eligible,
    Ineligible,
}

impl FallbackHint {
    fn parse(value: &str) -> Result<Self, SignalProjectionError> {
        match value.trim() {
            FALLBACK_ELIGIBLE => Ok(Self::Eligible),
            FALLBACK_INELIGIBLE => Ok(Self::Ineligible),
            _ => Err(SignalProjectionError::UnsupportedFallbackHint),
        }
    }

    const fn as_signal(self) -> &'static str {
        match self {
            Self::Eligible => FALLBACK_ELIGIBLE,
            Self::Ineligible => FALLBACK_INELIGIBLE,
        }
    }
}

/// Implementation-free signal-source seam for host classifiers.
pub trait SignalSource {
    /// Projects bounded request metadata into a router signal draft.
    ///
    /// # Errors
    ///
    /// Returns [`SignalProjectionError`] when the source cannot produce safe
    /// bounded signal labels.
    fn project(
        &self,
        metadata: &RequestMetadata,
    ) -> Result<crate::SignalDraft<'static>, SignalProjectionError>;
}

/// Deterministic default signal source.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct RuleSignalSource;

impl SignalSource for RuleSignalSource {
    fn project(
        &self,
        metadata: &RequestMetadata,
    ) -> Result<crate::SignalDraft<'static>, SignalProjectionError> {
        project_signals(metadata)
    }
}

/// Pure default projector from bounded request metadata to router signal bands.
///
/// # Errors
///
/// Returns [`SignalProjectionError`] only when supplied metadata is invalid;
/// validated [`RequestMetadata`] projects deterministically.
pub fn project_signals(
    metadata: &RequestMetadata,
) -> Result<crate::SignalDraft<'static>, SignalProjectionError> {
    Ok(crate::SignalDraft {
        task_shape: task_shape(metadata),
        risk_band: risk_band(metadata),
        contract_pressure: contract_pressure(metadata),
        context_pressure: context_pressure(metadata),
        latency_band: latency_band(metadata),
        cost_band: cost_band(metadata),
        privacy_band: metadata.privacy_policy.as_signal(),
        fallback_eligibility: metadata.fallback_hint.as_signal(),
    })
}

fn task_shape(metadata: &RequestMetadata) -> &'static str {
    if metadata.contains_code {
        return TASK_CODING;
    }
    if metadata.tool_call_count >= WORKFLOW_TOOL_CALL_THRESHOLD {
        return TASK_WORKFLOW;
    }
    if metadata.requires_structured_output
        && metadata.prompt_char_estimate >= REVIEW_PROMPT_CHAR_THRESHOLD
    {
        return TASK_REVIEW;
    }
    if metadata.prompt_char_estimate >= MEDIUM_PROMPT_CHAR_THRESHOLD {
        return TASK_REASONING;
    }
    TASK_SIMPLE
}

fn risk_band(metadata: &RequestMetadata) -> &'static str {
    if metadata.prompt_char_estimate >= STRICT_PROMPT_CHAR_THRESHOLD
        || metadata.context_window_fill_per_mille >= STRICT_CONTEXT_PRESSURE_PER_MILLE
    {
        return BAND_STRICT;
    }
    if metadata.prompt_char_estimate >= HIGH_PROMPT_CHAR_THRESHOLD
        || metadata.tool_call_count >= HIGH_TOOL_CALL_THRESHOLD
    {
        return BAND_HIGH;
    }
    if metadata.prompt_char_estimate >= MEDIUM_PROMPT_CHAR_THRESHOLD
        || metadata.context_window_fill_per_mille >= MEDIUM_CONTEXT_PRESSURE_PER_MILLE
    {
        return BAND_MEDIUM;
    }
    BAND_LOW
}

fn contract_pressure(metadata: &RequestMetadata) -> &'static str {
    if metadata.requires_structured_output {
        return BAND_STRICT;
    }
    if metadata.tool_call_count >= HIGH_TOOL_CALL_THRESHOLD {
        return BAND_HIGH;
    }
    BAND_LOW
}

fn context_pressure(metadata: &RequestMetadata) -> &'static str {
    if metadata.context_window_fill_per_mille >= STRICT_CONTEXT_PRESSURE_PER_MILLE {
        return BAND_STRICT;
    }
    if metadata.context_window_fill_per_mille >= HIGH_CONTEXT_PRESSURE_PER_MILLE {
        return BAND_HIGH;
    }
    if metadata.context_window_fill_per_mille >= MEDIUM_CONTEXT_PRESSURE_PER_MILLE {
        return BAND_MEDIUM;
    }
    BAND_LOW
}

fn latency_band(metadata: &RequestMetadata) -> &'static str {
    match risk_band(metadata) {
        BAND_STRICT | BAND_HIGH => BAND_HIGH,
        BAND_MEDIUM => BAND_MEDIUM,
        _ => BAND_LOW,
    }
}

fn cost_band(metadata: &RequestMetadata) -> &'static str {
    if metadata.tool_call_count >= WORKFLOW_TOOL_CALL_THRESHOLD {
        return BAND_HIGH;
    }
    latency_band(metadata)
}

fn validate_non_empty(draft: &RequestMetadataDraft<'_>) -> Result<(), SignalProjectionError> {
    if draft.prompt_char_estimate == EMPTY_COUNT
        && draft.tool_call_count == EMPTY_COUNT
        && !draft.contains_code
        && !draft.requires_structured_output
    {
        return Err(SignalProjectionError::EmptyMetadata);
    }
    Ok(())
}

fn validate_context_pressure(value: u16) -> Result<(), SignalProjectionError> {
    if value > MAX_CONTEXT_PRESSURE_PER_MILLE {
        return Err(SignalProjectionError::ContextWindowOutOfRange {
            max_per_mille: MAX_CONTEXT_PRESSURE_PER_MILLE,
            actual_per_mille: value,
        });
    }
    Ok(())
}

fn reject_unsafe_label(field: &'static str, value: &str) -> Result<(), SignalProjectionError> {
    if UNSAFE_SENTINELS
        .iter()
        .any(|sentinel| value.contains(sentinel))
    {
        return Err(SignalProjectionError::UnsafePayload { field });
    }
    Ok(())
}
