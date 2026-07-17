use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use animus_adapters::SafeHash;

use super::model::{
    SandlockCapability, SandlockHostError, SandlockHostReceipt, SandlockLaunchPlan,
    SandlockPreflightDecision, SandlockProbeReport,
};

/// Explicit capability-root grant placeholder on unsupported hosts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockCapabilityRootGrant {
    label: String,
    grant_hash: SafeHash,
    root_path: PathBuf,
}

impl SandlockCapabilityRootGrant {
    /// Builds an explicit host capability root descriptor.
    ///
    /// # Errors
    ///
    /// Returns [`SandlockHostError`] when the safe label is invalid.
    pub fn new(
        label: &str,
        grant_hash: SafeHash,
        root_path: PathBuf,
    ) -> Result<Self, SandlockHostError> {
        super::core::validate_safe_label("capability_root_label", label)?;
        Ok(Self {
            label: label.to_owned(),
            grant_hash,
            root_path,
        })
    }

    /// Returns the safe root label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }
}

/// Unsupported-host resolved-root placeholder.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockResolvedRoot;

/// Explicit cancellation handle supplied by the host.
#[derive(Clone, Debug, Default)]
pub struct SandlockCancellation {
    cancelled: Arc<AtomicBool>,
}

impl SandlockCancellation {
    /// Creates a non-cancelled handle.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Requests cancellation.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns whether cancellation was observed.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

/// One capability-specific functional probe command on unsupported hosts.
#[derive(Clone, Copy, Debug)]
pub struct SandlockMechanismProbe<'a> {
    capability: SandlockCapability,
    command: &'a [&'a str],
}

impl<'a> SandlockMechanismProbe<'a> {
    /// Binds one exact command to one requested mechanism class.
    #[must_use]
    pub const fn new(capability: SandlockCapability, command: &'a [&'a str]) -> Self {
        Self {
            capability,
            command,
        }
    }

    /// Returns the mechanism class.
    #[must_use]
    pub const fn capability(self) -> SandlockCapability {
        self.capability
    }

    /// Returns the exact probe command.
    #[must_use]
    pub const fn command(self) -> &'a [&'a str] {
        self.command
    }
}

/// Borrowed unsupported-host execution request.
#[derive(Clone, Copy, Debug)]
pub struct SandlockHostExecutionRequest<'a> {
    /// Pure admitted launch plan.
    pub plan: &'a SandlockLaunchPlan,
    /// Explicit host capability roots.
    pub root_grants: &'a [SandlockCapabilityRootGrant],
    /// One functional command per requested mechanism.
    pub mechanism_probes: &'a [SandlockMechanismProbe<'a>],
    /// Exact target command.
    pub command: &'a [&'a str],
    /// Explicit cancellation handle.
    pub cancellation: &'a SandlockCancellation,
}

/// Successful host result shape retained for cross-platform API parity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockHostExecutionResult {
    /// Functional probe report.
    pub probe: SandlockProbeReport,
    /// Pure preflight decision.
    pub preflight: SandlockPreflightDecision,
    /// Metadata-only receipt.
    pub receipt: SandlockHostReceipt,
}

/// Rejects root resolution on unsupported hosts.
///
/// # Errors
///
/// Always returns [`SandlockHostError::UnsupportedPlatform`].
pub fn resolve_sandlock_roots(
    _plan: &SandlockLaunchPlan,
    _grants: &[SandlockCapabilityRootGrant],
) -> Result<Vec<SandlockResolvedRoot>, SandlockHostError> {
    Err(SandlockHostError::UnsupportedPlatform)
}

/// Rejects Sandlock target execution on unsupported hosts before any effect.
///
/// # Errors
///
/// Always returns [`SandlockHostError::UnsupportedPlatform`].
pub async fn execute_sandlock_host(
    _request: SandlockHostExecutionRequest<'_>,
) -> Result<SandlockHostExecutionResult, SandlockHostError> {
    // r[impl agent_harness_runtime.sandlock_host_adapter.feature_gate]
    Err(SandlockHostError::UnsupportedPlatform)
}
