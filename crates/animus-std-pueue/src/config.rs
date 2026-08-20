use std::path::{Component, Path, PathBuf};

use animus_adapters::{QueueCapabilities, QueueLogBounds};
use animus_runtime::{
    ProcessJobProfileError, ProcessJobProfilePolicy, ProcessJobProfileResolveRequest,
    ProcessJobProfileSource, ResolvedProcessJobProfile, resolve_process_job_profile,
};

#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        reason = "the closed generated projection keeps adversarial decoding and conversion reviewable together"
    )
)]
mod dto;

/// Exact generated pueue-host configuration schema.
pub const PUEUE_HOST_CONFIG_SCHEMA: &str = "animus.pueue-host.v1";

const MAX_SAFE_LABEL_CHARS: usize = 128;
const CONFIG_PATH_COMPONENT_COUNT: usize = 1;
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "command_correctness",
    "scheduler_reliability",
    "sandboxing",
    "production_readiness",
    "release_eligibility",
];

/// Stable pueue configuration failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PueueHostConfigIssue {
    /// Input was not valid JSON.
    Json,
    /// JSON used an unknown or incompatible shape.
    Shape,
    /// A path, label, capability, or bound was invalid.
    Value,
    /// A process-job profile failed Animus policy validation.
    Profile,
}

/// Bounded pueue configuration error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PueueHostConfigError {
    issue: PueueHostConfigIssue,
}

impl PueueHostConfigError {
    pub(crate) const fn new(issue: PueueHostConfigIssue) -> Self {
        Self { issue }
    }

    /// Returns the stable issue class.
    #[must_use]
    pub const fn issue(self) -> PueueHostConfigIssue {
        self.issue
    }
}

impl core::fmt::Display for PueueHostConfigError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            formatter,
            "pueue host configuration failed: {:?}",
            self.issue
        )
    }
}

impl std::error::Error for PueueHostConfigError {}

/// Validated process and projection bounds for one pueue namespace.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PueueHostLimits {
    pub(crate) command_timeout_ms: u64,
    pub(crate) daemon_timeout_ms: u64,
    pub(crate) poll_interval_ms: u64,
    pub(crate) wait_poll_attempts: u32,
    pub(crate) task_kill_after_seconds: u32,
    pub(crate) teardown_timeout_ms: u64,
    pub(crate) max_backend_stdout_bytes: usize,
    pub(crate) max_backend_stderr_bytes: usize,
    pub(crate) max_log_lines: u32,
    pub(crate) max_log_bytes: u32,
}

impl PueueHostLimits {
    /// Returns the default model-visible log bounds.
    ///
    /// # Errors
    ///
    /// Returns a queue contract error if validated configuration invariants drift.
    pub fn log_bounds(self) -> Result<QueueLogBounds, animus_adapters::QueueContractError> {
        QueueLogBounds::new(self.max_log_lines, self.max_log_bytes)
    }
}

/// Fully validated pueue host configuration.
#[derive(Clone, Debug)]
pub struct PueueHostConfig {
    pub(crate) namespace_id: String,
    pub(crate) namespace_root: PathBuf,
    pub(crate) config_path: PathBuf,
    pub(crate) pueue_program: PathBuf,
    pub(crate) pueued_program: PathBuf,
    pub(crate) timeout_program: PathBuf,
    pub(crate) owner_label: String,
    pub(crate) environment_policy: String,
    pub(crate) timeout_policy: String,
    pub(crate) limits: PueueHostLimits,
    pub(crate) wait_enabled: bool,
    pub(crate) poll_enabled: bool,
    pub(crate) profile_sources: Vec<ProcessJobProfileSource>,
    pub(crate) profile_policy: ProcessJobProfilePolicy,
}

impl PueueHostConfig {
    /// Returns the safe namespace identity.
    #[must_use]
    pub fn namespace_id(&self) -> &str {
        self.namespace_id.as_str()
    }

    /// Returns the explicit namespace root.
    #[must_use]
    pub fn namespace_root(&self) -> &Path {
        self.namespace_root.as_path()
    }

    /// Returns the explicit pueue configuration path.
    #[must_use]
    pub fn config_path(&self) -> &Path {
        self.config_path.as_path()
    }

    /// Returns the caller-owned pueued executable for explicit supervision.
    #[must_use]
    pub fn pueued_program(&self) -> &Path {
        self.pueued_program.as_path()
    }

    /// Returns the reviewed task-timeout executable.
    #[must_use]
    pub fn timeout_program(&self) -> &Path {
        self.timeout_program.as_path()
    }

    /// Returns the exact enqueue environment-policy label.
    #[must_use]
    pub fn environment_policy(&self) -> &str {
        self.environment_policy.as_str()
    }

    /// Returns the exact enqueue timeout-policy label.
    #[must_use]
    pub fn timeout_policy(&self) -> &str {
        self.timeout_policy.as_str()
    }

    /// Returns the host limits.
    #[must_use]
    pub const fn limits(&self) -> PueueHostLimits {
        self.limits
    }

    /// Returns capabilities implemented by this host.
    #[must_use]
    pub const fn capabilities(&self) -> QueueCapabilities {
        let capabilities = QueueCapabilities::empty();
        let capabilities = if self.wait_enabled {
            capabilities.with(animus_adapters::QueueCapabilityKind::Wait)
        } else {
            capabilities
        };
        if self.poll_enabled {
            capabilities.with(animus_adapters::QueueCapabilityKind::Poll)
        } else {
            capabilities
        }
    }

    /// Resolves one reviewed process-job profile through Animus policy.
    ///
    /// # Errors
    ///
    /// Returns the exact Animus profile error before any pueue process call.
    pub fn resolve_profile(
        &self,
        profile_name: &str,
    ) -> Result<ResolvedProcessJobProfile, ProcessJobProfileError> {
        resolve_process_job_profile(ProcessJobProfileResolveRequest {
            sources: self.profile_sources.as_slice(),
            profile_name,
            owner_label: self.owner_label.as_str(),
            policy: &self.profile_policy,
        })
    }
}

/// Parses and revalidates a generated pueue-host projection.
///
/// # Errors
///
/// Returns [`PueueHostConfigError`] for malformed JSON, unknown fields, unsafe
/// authority, unsupported capabilities, invalid bounds, or rejected profiles.
pub fn parse_pueue_host_config(text: &str) -> Result<PueueHostConfig, PueueHostConfigError> {
    dto::decode_config(text)
}

pub(crate) fn validate_label(value: &str) -> Result<String, PueueHostConfigError> {
    let normalized = value.trim();
    let mut characters = normalized.chars();
    let is_first_character_safe = characters
        .next()
        .is_some_and(|character| character.is_ascii_alphanumeric());
    let is_remaining_text_safe = characters.all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.' | ':')
    });
    if !is_first_character_safe
        || !is_remaining_text_safe
        || normalized.chars().count() > MAX_SAFE_LABEL_CHARS
    {
        return Err(PueueHostConfigError::new(PueueHostConfigIssue::Value));
    }
    Ok(String::from(normalized))
}

pub(crate) fn validate_absolute_path(value: &str) -> Result<PathBuf, PueueHostConfigError> {
    let path = PathBuf::from(value);
    let has_parent = path
        .components()
        .any(|component| component == Component::ParentDir);
    if !path.is_absolute() || has_parent {
        return Err(PueueHostConfigError::new(PueueHostConfigIssue::Value));
    }
    Ok(path)
}

pub(crate) fn validate_config_leaf(value: &str) -> Result<&str, PueueHostConfigError> {
    let path = Path::new(value);
    let is_config_leaf_valid = path.components().count() == CONFIG_PATH_COMPONENT_COUNT
        && matches!(path.components().next(), Some(Component::Normal(_)));
    if !is_config_leaf_valid {
        return Err(PueueHostConfigError::new(PueueHostConfigIssue::Value));
    }
    Ok(value)
}

pub(crate) fn validate_non_claims(values: &[String]) -> Result<(), PueueHostConfigError> {
    let is_non_claim_set_complete = REQUIRED_NON_CLAIMS
        .iter()
        .all(|required| values.iter().any(|value| value == required));
    if is_non_claim_set_complete {
        Ok(())
    } else {
        Err(PueueHostConfigError::new(PueueHostConfigIssue::Value))
    }
}
