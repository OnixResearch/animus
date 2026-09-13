#[cfg(feature = "reqwest-client")]
mod execution;
mod policy;
mod report;
#[cfg(test)]
mod tests;
#[cfg(feature = "reqwest-client")]
pub use execution::{
    run_packaged_live_local_dspark_smoke, run_packaged_live_openai_smoke,
    run_packaged_live_openrouter_smoke,
};
pub use policy::{
    plan_live_local_dspark_smoke, plan_live_openai_smoke, plan_live_openrouter_smoke,
};
pub use report::{
    render_live_local_dspark_smoke_evidence, render_live_openai_smoke_evidence,
    render_live_openrouter_smoke_evidence,
};

const LOCAL_DSPARK_CREDENTIAL_SOURCE: &str = "animus-live-local-dspark-smoke-stdin";
const LOCAL_DSPARK_CREDENTIAL_LABEL: &str = "brokered:animus-live-local-dspark-smoke-stdin";
const LOCAL_DSPARK_PROVIDER_ID: &str = "local-dspark";
const LOCAL_DSPARK_STATUS: &str = "LIVE_LOCAL_DSPARK_SMOKE status=passed";
/// Exact reviewed model for every packaged live `local-dspark` smoke.
pub const LOCAL_DSPARK_SMOKE_MODEL_ID: &str = "deepseek-v4-flash-0731-ablit-100";
const OPENAI_CREDENTIAL_SOURCE: &str = "animus-live-openai-smoke-stdin";
const OPENAI_CREDENTIAL_LABEL: &str = "brokered:animus-live-openai-smoke-stdin";
const OPENAI_PROVIDER_ID: &str = "openai";
const OPENAI_STATUS: &str = "LIVE_OPENAI_SMOKE status=passed";
const OPENROUTER_CREDENTIAL_SOURCE: &str = "animus-live-openrouter-smoke-stdin";
const OPENROUTER_CREDENTIAL_LABEL: &str = "brokered:animus-live-openrouter-smoke-stdin";
const OPENROUTER_PROVIDER_ID: &str = "openrouter";
const OPENROUTER_STATUS: &str = "LIVE_OPENROUTER_SMOKE status=passed";
/// Exact reviewed model for every packaged live `OpenRouter` smoke.
pub const OPENROUTER_SMOKE_MODEL_ID: &str = "deepseek/deepseek-v4-flash-0731";
const PROMPT: &str = "Reply with the single word READY.";
#[cfg(any(feature = "reqwest-client", test))]
const TIMEOUT_SECONDS: u64 = 60;
#[cfg(any(feature = "reqwest-client", test))]
const MAX_RESPONSE_BYTES: u64 = 262_144;
const OUTPUT_LIMIT: u32 = 32;
const CONTEXT_BUDGET: u32 = 8_192;
const MODEL_OUTPUT_BUDGET: u32 = 4_096;
const REVIEWED_PROJECTION: &[&str] = &[
    "family:openai_responses",
    "role:user",
    "text:Reply with the single word READY.",
    "response:text",
];
const NON_CLAIMS: &str =
    "reliability,quality,billing,routing,production,retry,fallback,streaming,tool-use";

#[derive(Clone, Copy)]
struct SmokeCohort {
    provider_id: &'static str,
    endpoint_identity: &'static str,
    #[cfg(feature = "reqwest-client")]
    endpoint_url: &'static str,
    credential_source: &'static str,
    credential_label: &'static str,
    deadline_label: &'static str,
    cancellation_label: &'static str,
    status: &'static str,
}

const OPENAI_COHORT: SmokeCohort = SmokeCohort {
    provider_id: OPENAI_PROVIDER_ID,
    endpoint_identity: crate::OFFICIAL_OPENAI_RESPONSES_ENDPOINT_IDENTITY,
    #[cfg(feature = "reqwest-client")]
    endpoint_url: crate::OFFICIAL_OPENAI_RESPONSES_URL,
    credential_source: OPENAI_CREDENTIAL_SOURCE,
    credential_label: OPENAI_CREDENTIAL_LABEL,
    deadline_label: "deadline-packaged-live-openai-smoke",
    cancellation_label: "cancel-packaged-live-openai-smoke",
    status: OPENAI_STATUS,
};

const LOCAL_DSPARK_COHORT: SmokeCohort = SmokeCohort {
    provider_id: LOCAL_DSPARK_PROVIDER_ID,
    endpoint_identity: crate::LOCAL_DSPARK_RESPONSES_ENDPOINT_IDENTITY,
    #[cfg(feature = "reqwest-client")]
    endpoint_url: crate::LOCAL_DSPARK_RESPONSES_URL,
    credential_source: LOCAL_DSPARK_CREDENTIAL_SOURCE,
    credential_label: LOCAL_DSPARK_CREDENTIAL_LABEL,
    deadline_label: "deadline-packaged-live-local-dspark-smoke",
    cancellation_label: "cancel-packaged-live-local-dspark-smoke",
    status: LOCAL_DSPARK_STATUS,
};

const OPENROUTER_COHORT: SmokeCohort = SmokeCohort {
    provider_id: OPENROUTER_PROVIDER_ID,
    endpoint_identity: crate::OPENROUTER_RESPONSES_ENDPOINT_IDENTITY,
    #[cfg(feature = "reqwest-client")]
    endpoint_url: crate::OPENROUTER_RESPONSES_URL,
    credential_source: OPENROUTER_CREDENTIAL_SOURCE,
    credential_label: OPENROUTER_CREDENTIAL_LABEL,
    deadline_label: "deadline-packaged-live-openrouter-smoke",
    cancellation_label: "cancel-packaged-live-openrouter-smoke",
    status: OPENROUTER_STATUS,
};

/// Safe failure from packaged live Responses smoke planning or execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LiveOpenAiSmokeError {
    /// The explicit model identity was not admitted.
    InvalidModel,
    /// The fixed smoke request or registry metadata could not be constructed.
    InvalidPlan,
    /// The concrete provider dispatch planner rejected the fixed smoke request.
    Dispatch(animus_std::ProviderDispatchError),
    /// The accepted live host rejected configuration, credentials, transport, or response data.
    Host(crate::HostError),
    /// The canonical response contained no admitted output item.
    EmptyOutcome,
}

impl std::fmt::Display for LiveOpenAiSmokeError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidModel => {
                std::fmt::Formatter::write_str(formatter, "live Responses smoke model is invalid")?;
            }
            Self::InvalidPlan => {
                std::fmt::Formatter::write_str(formatter, "live Responses smoke plan is invalid")?;
            }
            Self::Dispatch(source) => {
                write!(formatter, "live Responses smoke dispatch failed: {source}")?;
            }
            Self::Host(source) => {
                write!(formatter, "live Responses smoke host failed: {source}")?;
            }
            Self::EmptyOutcome => std::fmt::Formatter::write_str(
                formatter,
                "live Responses smoke response has no output",
            )?,
        }
        Ok(())
    }
}

impl std::error::Error for LiveOpenAiSmokeError {}

impl From<crate::HostError> for LiveOpenAiSmokeError {
    fn from(source: crate::HostError) -> Self {
        Self::Host(source)
    }
}
