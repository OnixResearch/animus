use animus_adapters::{
    SafeHash, SandboxCowDisposition, SandboxDescriptor, SandboxEnvironmentPolicy,
    SandboxFilesystemMode, SandboxHttpPolicy, SandboxNetworkPolicy, SandboxProcessProfile,
    SandboxWorkdirMode,
};

/// Reviewed upstream Sandlock release.
pub const SANDLOCK_COHORT_VERSION: &str = "0.7.0";
/// Exact reviewed upstream Sandlock revision.
pub const SANDLOCK_COHORT_REVISION: &str = "e8c4cc0ef58e3fc145c4d147a61b889b4f291a10";
/// SPDX license expression for the reviewed upstream cohort.
pub const SANDLOCK_COHORT_LICENSE: &str = "Apache-2.0";
/// Stable schema identity for Sandlock launch plans.
pub const SANDLOCK_LAUNCH_PLAN_SCHEMA: &str = "animus.sandlock-launch-plan.v1";
/// Stable schema identity for Sandlock host receipts.
pub const SANDLOCK_HOST_RECEIPT_SCHEMA: &str = "animus.sandlock-host-receipt.v1";
/// Sandlock receipts never claim authorization or kernel correctness.
pub const SANDLOCK_RECEIPT_NON_CLAIMS: &[&str] = &[
    "not_authorization",
    "not_kernel_correctness",
    "not_complete_containment",
    "not_upstream_private_storage_cleanup",
    "not_build_hermeticity",
    "not_cross_platform_parity",
    "not_release_eligibility",
];

/// Prior authorization posture supplied to the adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandlockAuthorizationDecision {
    /// The stronger Animus/Basalt authority layer allowed the exact request.
    Allowed,
    /// The stronger authority layer denied the request.
    Denied,
}

/// Exact prior authority binding required before Sandlock host effects.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockAuthorizationBinding {
    authorization_hash: SafeHash,
    action_hash: SafeHash,
    subject_hash: SafeHash,
    target_hash: SafeHash,
    descriptor_hash: SafeHash,
    policy_hash: SafeHash,
    command_hash: SafeHash,
    decision: SandlockAuthorizationDecision,
    current: bool,
}

impl SandlockAuthorizationBinding {
    /// Builds an exact authorization binding from already-admitted identities.
    #[must_use]
    pub fn new(draft: SandlockAuthorizationBindingDraft) -> Self {
        Self {
            authorization_hash: draft.authorization_hash,
            action_hash: draft.action_hash,
            subject_hash: draft.subject_hash,
            target_hash: draft.target_hash,
            descriptor_hash: draft.descriptor_hash,
            policy_hash: draft.policy_hash,
            command_hash: draft.command_hash,
            decision: draft.decision,
            current: draft.current,
        }
    }

    /// Returns the safe authorization identity.
    #[must_use]
    pub const fn authorization_hash(&self) -> &SafeHash {
        &self.authorization_hash
    }

    pub(crate) const fn action_hash(&self) -> &SafeHash {
        &self.action_hash
    }

    pub(crate) const fn subject_hash(&self) -> &SafeHash {
        &self.subject_hash
    }

    pub(crate) const fn target_hash(&self) -> &SafeHash {
        &self.target_hash
    }

    pub(crate) const fn descriptor_hash(&self) -> &SafeHash {
        &self.descriptor_hash
    }

    pub(crate) const fn policy_hash(&self) -> &SafeHash {
        &self.policy_hash
    }

    pub(crate) const fn command_hash(&self) -> &SafeHash {
        &self.command_hash
    }

    pub(crate) const fn decision(&self) -> SandlockAuthorizationDecision {
        self.decision
    }

    pub(crate) const fn is_current(&self) -> bool {
        self.current
    }
}

/// Owned exact authorization-binding input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockAuthorizationBindingDraft {
    /// Safe identity of the prior authorization result.
    pub authorization_hash: SafeHash,
    /// Safe action identity.
    pub action_hash: SafeHash,
    /// Safe subject identity.
    pub subject_hash: SafeHash,
    /// Safe target identity.
    pub target_hash: SafeHash,
    /// Exact normalized descriptor identity.
    pub descriptor_hash: SafeHash,
    /// Exact host policy identity.
    pub policy_hash: SafeHash,
    /// Exact command identity.
    pub command_hash: SafeHash,
    /// Prior authority decision.
    pub decision: SandlockAuthorizationDecision,
    /// Host-supplied freshness observation.
    pub current: bool,
}

/// Access granted to one confined relative root.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SandlockRootAccess {
    /// The resolved root may only be read.
    ReadOnly,
    /// The resolved root may be read and written.
    ReadWrite,
}

impl SandlockRootAccess {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::ReadOnly => "read_only",
            Self::ReadWrite => "read_write",
        }
    }
}

/// Safe capability-root binding consumed by the pure launch planner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockRootBinding {
    label: String,
    grant_hash: SafeHash,
    relative_path: String,
    access: SandlockRootAccess,
}

impl SandlockRootBinding {
    /// Creates a safe relative root binding.
    ///
    /// # Errors
    ///
    /// Returns [`SandlockHostError`] when the label or relative path is unsafe.
    pub fn new(
        label: &str,
        grant_hash: SafeHash,
        relative_path: &str,
        access: SandlockRootAccess,
    ) -> Result<Self, SandlockHostError> {
        super::core::validate_safe_label("root_label", label)?;
        super::core::validate_relative_path(relative_path)?;
        Ok(Self {
            label: label.to_owned(),
            grant_hash,
            relative_path: relative_path.to_owned(),
            access,
        })
    }

    /// Returns the safe root label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    /// Returns the capability-grant identity.
    #[must_use]
    pub const fn grant_hash(&self) -> &SafeHash {
        &self.grant_hash
    }

    /// Returns the confined relative path.
    #[must_use]
    pub fn relative_path(&self) -> &str {
        self.relative_path.as_str()
    }

    /// Returns granted access.
    #[must_use]
    pub const fn access(&self) -> SandlockRootAccess {
        self.access
    }
}

/// Exact Sandlock mechanism class required and probed before target launch.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum SandlockCapability {
    /// Landlock filesystem confinement.
    Filesystem,
    /// seccomp-bpf syscall filtering.
    SyscallFilter,
    /// seccomp notification supervisor setup.
    Supervisor,
    /// COW workdir staging and disposition.
    Cow,
    /// Process and memory resource enforcement.
    Resources,
    /// Outbound and bind network enforcement, including deny-all.
    Network,
    /// HTTP method, host, and path policy.
    Http,
    /// Clean environment projection.
    Environment,
    /// Stable time and randomness controls.
    Determinism,
}

impl SandlockCapability {
    /// Returns the stable capability label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Filesystem => "filesystem",
            Self::SyscallFilter => "syscall_filter",
            Self::Supervisor => "supervisor",
            Self::Cow => "cow",
            Self::Resources => "resources",
            Self::Network => "network",
            Self::Http => "http",
            Self::Environment => "environment",
            Self::Determinism => "determinism",
        }
    }
}

/// Typed host policy values used to build one exact upstream Sandlock policy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockHostPolicy {
    policy_id: String,
    policy_hash: SafeHash,
    timeout_millis: u64,
    max_output_bytes: u64,
    max_memory_bytes: Option<u64>,
    max_processes: Option<u32>,
    max_open_files: Option<u32>,
    max_cpu_percent: Option<u8>,
    egress_allow: Vec<String>,
    http_allow: Vec<String>,
    http_deny: Vec<String>,
    deterministic_epoch_seconds: Option<u64>,
    deterministic_seed: Option<u64>,
    cow_workdir_root: Option<String>,
}

impl SandlockHostPolicy {
    /// Validates and canonicalizes a host policy profile.
    ///
    /// # Errors
    ///
    /// Returns [`SandlockHostError`] for zero bounds, unsafe labels, duplicate
    /// rules, or malformed optional root labels.
    pub fn new(draft: SandlockHostPolicyDraft<'_>) -> Result<Self, SandlockHostError> {
        super::core::validate_policy_draft(&draft)?;
        let mut egress_allow = super::core::canonical_strings(draft.egress_allow)?;
        let mut http_allow = super::core::canonical_strings(draft.http_allow)?;
        let mut http_deny = super::core::canonical_strings(draft.http_deny)?;
        egress_allow.sort();
        http_allow.sort();
        http_deny.sort();
        let cow_workdir_root = draft.cow_workdir_root.map(str::to_owned);
        let hash = super::core::host_policy_hash(super::core::HostPolicyHashInput {
            draft: &draft,
            egress_allow: egress_allow.as_slice(),
            http_allow: http_allow.as_slice(),
            http_deny: http_deny.as_slice(),
        });
        Ok(Self {
            policy_id: draft.policy_id.to_owned(),
            policy_hash: hash,
            timeout_millis: draft.timeout_millis,
            max_output_bytes: draft.max_output_bytes,
            max_memory_bytes: draft.max_memory_bytes,
            max_processes: draft.max_processes,
            max_open_files: draft.max_open_files,
            max_cpu_percent: draft.max_cpu_percent,
            egress_allow,
            http_allow,
            http_deny,
            deterministic_epoch_seconds: draft.deterministic_epoch_seconds,
            deterministic_seed: draft.deterministic_seed,
            cow_workdir_root,
        })
    }

    /// Returns the safe policy label.
    #[must_use]
    pub fn policy_id(&self) -> &str {
        self.policy_id.as_str()
    }

    /// Returns the canonical policy identity.
    #[must_use]
    pub const fn policy_hash(&self) -> &SafeHash {
        &self.policy_hash
    }

    /// Returns the target timeout in milliseconds.
    #[must_use]
    pub const fn timeout_millis(&self) -> u64 {
        self.timeout_millis
    }

    /// Returns the combined stdout/stderr byte ceiling.
    #[must_use]
    pub const fn max_output_bytes(&self) -> u64 {
        self.max_output_bytes
    }

    pub(crate) const fn max_memory_bytes(&self) -> Option<u64> {
        self.max_memory_bytes
    }

    pub(crate) const fn max_processes(&self) -> Option<u32> {
        self.max_processes
    }

    pub(crate) const fn max_open_files(&self) -> Option<u32> {
        self.max_open_files
    }

    pub(crate) const fn max_cpu_percent(&self) -> Option<u8> {
        self.max_cpu_percent
    }

    pub(crate) fn egress_allow(&self) -> &[String] {
        self.egress_allow.as_slice()
    }

    pub(crate) fn http_allow(&self) -> &[String] {
        self.http_allow.as_slice()
    }

    pub(crate) fn http_deny(&self) -> &[String] {
        self.http_deny.as_slice()
    }

    pub(crate) const fn deterministic_epoch_seconds(&self) -> Option<u64> {
        self.deterministic_epoch_seconds
    }

    pub(crate) const fn deterministic_seed(&self) -> Option<u64> {
        self.deterministic_seed
    }

    pub(crate) fn cow_workdir_root(&self) -> Option<&str> {
        self.cow_workdir_root.as_deref()
    }
}

/// Borrowed host policy input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SandlockHostPolicyDraft<'a> {
    /// Stable safe policy label.
    pub policy_id: &'a str,
    /// Target timeout in milliseconds.
    pub timeout_millis: u64,
    /// Combined stdout/stderr byte ceiling.
    pub max_output_bytes: u64,
    /// Memory limit required by resource-limited profiles.
    pub max_memory_bytes: Option<u64>,
    /// Process limit required by resource-limited profiles.
    pub max_processes: Option<u32>,
    /// Open-file limit required by resource-limited profiles.
    pub max_open_files: Option<u32>,
    /// CPU percentage required by resource-limited profiles.
    pub max_cpu_percent: Option<u8>,
    /// Exact upstream network allow rules.
    pub egress_allow: &'a [&'a str],
    /// Exact upstream HTTP allow rules.
    pub http_allow: &'a [&'a str],
    /// Exact upstream HTTP deny rules.
    pub http_deny: &'a [&'a str],
    /// Stable Unix epoch used by deterministic profiles.
    pub deterministic_epoch_seconds: Option<u64>,
    /// Stable random seed used by deterministic profiles.
    pub deterministic_seed: Option<u64>,
    /// Writable safe root label used for COW staging.
    pub cow_workdir_root: Option<&'a str>,
}

/// Pure launch-planning input.
#[derive(Clone, Copy, Debug)]
pub struct SandlockLaunchRequest<'a> {
    /// Normalized portable sandbox descriptor.
    pub descriptor: &'a SandboxDescriptor,
    /// Exact typed host policy profile.
    pub policy: &'a SandlockHostPolicy,
    /// Prior stronger authorization binding.
    pub authorization: &'a SandlockAuthorizationBinding,
    /// Safe action identity expected by the host call.
    pub action_hash: &'a SafeHash,
    /// Safe subject identity expected by the host call.
    pub subject_hash: &'a SafeHash,
    /// Safe target identity expected by the host call.
    pub target_hash: &'a SafeHash,
    /// Exact command identity expected by the host call.
    pub command_hash: &'a SafeHash,
    /// Safe capability-root bindings.
    pub root_bindings: &'a [SandlockRootBinding],
}

/// Deterministic owned launch plan with no concrete host paths.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockLaunchPlan {
    descriptor: SandboxDescriptor,
    policy: SandlockHostPolicy,
    authorization_hash: SafeHash,
    command_hash: SafeHash,
    root_bindings: Vec<SandlockRootBinding>,
    required_capabilities: Vec<SandlockCapability>,
    cohort_hash: SafeHash,
    plan_hash: SafeHash,
}

impl SandlockLaunchPlan {
    pub(crate) fn from_parts(parts: SandlockLaunchPlanParts<'_>) -> Self {
        Self {
            descriptor: parts.descriptor.clone(),
            policy: parts.policy.clone(),
            authorization_hash: parts.authorization.authorization_hash().clone(),
            command_hash: parts.command_hash.clone(),
            root_bindings: parts.root_bindings.to_vec(),
            required_capabilities: parts.required_capabilities,
            cohort_hash: parts.cohort_hash,
            plan_hash: parts.plan_hash,
        }
    }

    /// Returns the normalized portable descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &SandboxDescriptor {
        &self.descriptor
    }

    /// Returns the exact host policy profile.
    #[must_use]
    pub const fn policy(&self) -> &SandlockHostPolicy {
        &self.policy
    }

    /// Returns the prior authorization identity.
    #[must_use]
    pub const fn authorization_hash(&self) -> &SafeHash {
        &self.authorization_hash
    }

    /// Returns the expected command identity.
    #[must_use]
    pub const fn command_hash(&self) -> &SafeHash {
        &self.command_hash
    }

    /// Returns safe relative root bindings.
    #[must_use]
    pub fn root_bindings(&self) -> &[SandlockRootBinding] {
        self.root_bindings.as_slice()
    }

    /// Returns every mechanism required before target launch.
    #[must_use]
    pub fn required_capabilities(&self) -> &[SandlockCapability] {
        self.required_capabilities.as_slice()
    }

    /// Returns the reviewed upstream cohort identity.
    #[must_use]
    pub const fn cohort_hash(&self) -> &SafeHash {
        &self.cohort_hash
    }

    /// Returns the deterministic launch-plan identity.
    #[must_use]
    pub const fn plan_hash(&self) -> &SafeHash {
        &self.plan_hash
    }
}

pub(crate) struct SandlockLaunchPlanParts<'a> {
    pub(crate) descriptor: &'a SandboxDescriptor,
    pub(crate) policy: &'a SandlockHostPolicy,
    pub(crate) authorization: &'a SandlockAuthorizationBinding,
    pub(crate) command_hash: &'a SafeHash,
    pub(crate) root_bindings: &'a [SandlockRootBinding],
    pub(crate) required_capabilities: Vec<SandlockCapability>,
    pub(crate) cohort_hash: SafeHash,
    pub(crate) plan_hash: SafeHash,
}

/// Functional capability-probe report supplied by the Linux shell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockProbeReport {
    plan_hash: SafeHash,
    cohort_hash: SafeHash,
    proven_capabilities: Vec<SandlockCapability>,
    probe_hash: SafeHash,
}

impl SandlockProbeReport {
    /// Builds a metadata-only probe report.
    #[must_use]
    pub fn new(
        plan_hash: SafeHash,
        cohort_hash: SafeHash,
        mut proven_capabilities: Vec<SandlockCapability>,
        probe_hash: SafeHash,
    ) -> Self {
        proven_capabilities.sort();
        proven_capabilities.dedup();
        Self {
            plan_hash,
            cohort_hash,
            proven_capabilities,
            probe_hash,
        }
    }

    /// Returns the plan identity probed by the host.
    #[must_use]
    pub const fn plan_hash(&self) -> &SafeHash {
        &self.plan_hash
    }

    /// Returns the probed upstream cohort identity.
    #[must_use]
    pub const fn cohort_hash(&self) -> &SafeHash {
        &self.cohort_hash
    }

    /// Returns mechanisms proven usable by the host probe.
    #[must_use]
    pub fn proven_capabilities(&self) -> &[SandlockCapability] {
        self.proven_capabilities.as_slice()
    }

    /// Returns the metadata-only probe identity.
    #[must_use]
    pub const fn probe_hash(&self) -> &SafeHash {
        &self.probe_hash
    }
}

/// Pure preflight status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandlockPreflightStatus {
    /// Every requested mechanism was proven for the exact plan and cohort.
    Ready,
    /// Preflight failed closed.
    Blocked,
}

/// Pure fail-closed preflight decision.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockPreflightDecision {
    status: SandlockPreflightStatus,
    missing_capabilities: Vec<SandlockCapability>,
    preflight_hash: SafeHash,
}

impl SandlockPreflightDecision {
    pub(crate) fn new(
        status: SandlockPreflightStatus,
        missing_capabilities: Vec<SandlockCapability>,
        preflight_hash: SafeHash,
    ) -> Self {
        Self {
            status,
            missing_capabilities,
            preflight_hash,
        }
    }

    /// Returns whether target process launch is admitted.
    #[must_use]
    pub const fn is_ready(&self) -> bool {
        matches!(self.status, SandlockPreflightStatus::Ready)
    }

    /// Returns the preflight status.
    #[must_use]
    pub const fn status(&self) -> SandlockPreflightStatus {
        self.status
    }

    /// Returns missing requested mechanism classes.
    #[must_use]
    pub fn missing_capabilities(&self) -> &[SandlockCapability] {
        self.missing_capabilities.as_slice()
    }

    /// Returns the deterministic preflight identity.
    #[must_use]
    pub const fn preflight_hash(&self) -> &SafeHash {
        &self.preflight_hash
    }
}

/// How the target process terminated.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandlockExecutionStatus {
    /// The process exited with code zero.
    Passed,
    /// The process exited nonzero or from a signal.
    Failed,
    /// Policy or Sandlock setup failed before successful target execution.
    SetupFailed,
    /// The configured deadline elapsed.
    TimedOut,
    /// Explicit cancellation was observed.
    Cancelled,
    /// Captured output exceeded the configured ceiling.
    OutputLimitExceeded,
    /// This platform cannot host the adapter.
    UnsupportedPlatform,
}

impl SandlockExecutionStatus {
    /// Returns the stable status label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::SetupFailed => "setup_failed",
            Self::TimedOut => "timed_out",
            Self::Cancelled => "cancelled",
            Self::OutputLimitExceeded => "output_limit_exceeded",
            Self::UnsupportedPlatform => "unsupported_platform",
        }
    }
}

/// COW result class kept separate from process success.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandlockCowOutcome {
    /// COW was not requested.
    NotRequested,
    /// Staged writes were discarded.
    Discarded,
    /// Staged writes were committed by the selected policy.
    Committed,
    /// COW setup or disposition failed.
    Failed,
}

impl SandlockCowOutcome {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::NotRequested => "not_requested",
            Self::Discarded => "discarded",
            Self::Committed => "committed",
            Self::Failed => "failed",
        }
    }
}

/// Cleanup outcome after target termination.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SandlockCleanupStatus {
    /// Cleanup completed.
    Complete,
    /// Cleanup failed or could not be proven complete.
    Failed,
}

impl SandlockCleanupStatus {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Failed => "failed",
        }
    }
}

/// Metadata-only execution facts returned by a host executor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockExecutionFacts {
    /// Process termination class.
    pub status: SandlockExecutionStatus,
    /// Exit code when supplied by the executor.
    pub exit_code: Option<i32>,
    /// Signal number when supplied by the executor.
    pub signal: Option<i32>,
    /// Bounded stdout byte count.
    pub stdout_bytes: u64,
    /// Bounded stderr byte count.
    pub stderr_bytes: u64,
    /// Incremental stdout digest; no body is retained.
    pub stdout_hash: Option<SafeHash>,
    /// Incremental stderr digest; no body is retained.
    pub stderr_hash: Option<SafeHash>,
    /// COW disposition outcome.
    pub cow: SandlockCowOutcome,
    /// Optional bounded COW change count.
    pub cow_change_count: Option<u32>,
    /// Optional COW summary digest.
    pub cow_digest: Option<SafeHash>,
    /// Cleanup outcome.
    pub cleanup: SandlockCleanupStatus,
    /// Capabilities actually enforced according to the exact host executor.
    pub enforced_capabilities: Vec<SandlockCapability>,
    /// Safe metadata-only execution identity.
    pub execution_hash: SafeHash,
}

/// Accepted metadata-only Sandlock host receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockHostReceipt {
    status: SandlockExecutionStatus,
    descriptor_hash: SafeHash,
    policy_hash: SafeHash,
    authorization_hash: SafeHash,
    plan_hash: SafeHash,
    cohort_hash: SafeHash,
    preflight_hash: SafeHash,
    execution_hash: SafeHash,
    requested_capabilities: Vec<String>,
    enforced_capabilities: Vec<String>,
    stdout_bytes: u64,
    stderr_bytes: u64,
    stdout_hash: Option<SafeHash>,
    stderr_hash: Option<SafeHash>,
    cow: SandlockCowOutcome,
    cow_change_count: Option<u32>,
    cow_digest: Option<SafeHash>,
    exit_code: Option<i32>,
    signal: Option<i32>,
    cleanup: SandlockCleanupStatus,
    receipt_hash: SafeHash,
}

impl SandlockHostReceipt {
    pub(crate) fn from_parts(parts: SandlockHostReceiptParts<'_>, receipt_hash: SafeHash) -> Self {
        Self {
            status: parts.facts.status,
            descriptor_hash: parts.plan.descriptor().descriptor_hash().clone(),
            policy_hash: parts.plan.policy().policy_hash().clone(),
            authorization_hash: parts.plan.authorization_hash().clone(),
            plan_hash: parts.plan.plan_hash().clone(),
            cohort_hash: parts.plan.cohort_hash().clone(),
            preflight_hash: parts.preflight.preflight_hash().clone(),
            execution_hash: parts.facts.execution_hash.clone(),
            requested_capabilities: parts
                .plan
                .required_capabilities()
                .iter()
                .map(|capability| capability.as_str().to_owned())
                .collect(),
            enforced_capabilities: parts
                .facts
                .enforced_capabilities
                .iter()
                .map(|capability| capability.as_str().to_owned())
                .collect(),
            stdout_bytes: parts.facts.stdout_bytes,
            stderr_bytes: parts.facts.stderr_bytes,
            stdout_hash: parts.facts.stdout_hash.clone(),
            stderr_hash: parts.facts.stderr_hash.clone(),
            cow: parts.facts.cow,
            cow_change_count: parts.facts.cow_change_count,
            cow_digest: parts.facts.cow_digest.clone(),
            exit_code: parts.facts.exit_code,
            signal: parts.facts.signal,
            cleanup: parts.facts.cleanup,
            receipt_hash,
        }
    }

    /// Returns the execution status.
    #[must_use]
    pub const fn status(&self) -> SandlockExecutionStatus {
        self.status
    }

    /// Returns the exact launch-plan identity.
    #[must_use]
    pub const fn plan_hash(&self) -> &SafeHash {
        &self.plan_hash
    }

    /// Returns the prior authorization identity without promoting enforcement.
    #[must_use]
    pub const fn authorization_hash(&self) -> &SafeHash {
        &self.authorization_hash
    }

    /// Returns requested capability labels.
    #[must_use]
    pub fn requested_capabilities(&self) -> &[String] {
        self.requested_capabilities.as_slice()
    }

    /// Returns actually enforced capability labels.
    #[must_use]
    pub fn enforced_capabilities(&self) -> &[String] {
        self.enforced_capabilities.as_slice()
    }

    /// Returns the deterministic receipt identity.
    #[must_use]
    pub const fn receipt_hash(&self) -> &SafeHash {
        &self.receipt_hash
    }

    /// Returns the process stdout byte count without exposing output.
    #[must_use]
    pub const fn stdout_bytes(&self) -> u64 {
        self.stdout_bytes
    }

    /// Returns the process stderr byte count without exposing output.
    #[must_use]
    pub const fn stderr_bytes(&self) -> u64 {
        self.stderr_bytes
    }

    /// Returns COW disposition evidence.
    #[must_use]
    pub const fn cow_outcome(&self) -> SandlockCowOutcome {
        self.cow
    }

    /// Returns cleanup evidence.
    #[must_use]
    pub const fn cleanup_status(&self) -> SandlockCleanupStatus {
        self.cleanup
    }

    pub(crate) const fn descriptor_hash(&self) -> &SafeHash {
        &self.descriptor_hash
    }

    pub(crate) const fn policy_hash(&self) -> &SafeHash {
        &self.policy_hash
    }

    pub(crate) const fn cohort_hash(&self) -> &SafeHash {
        &self.cohort_hash
    }

    pub(crate) const fn preflight_hash(&self) -> &SafeHash {
        &self.preflight_hash
    }

    pub(crate) const fn execution_hash(&self) -> &SafeHash {
        &self.execution_hash
    }

    pub(crate) const fn stdout_hash(&self) -> Option<&SafeHash> {
        self.stdout_hash.as_ref()
    }

    pub(crate) const fn stderr_hash(&self) -> Option<&SafeHash> {
        self.stderr_hash.as_ref()
    }

    pub(crate) const fn cow_change_count(&self) -> Option<u32> {
        self.cow_change_count
    }

    pub(crate) const fn cow_digest(&self) -> Option<&SafeHash> {
        self.cow_digest.as_ref()
    }

    pub(crate) const fn exit_code(&self) -> Option<i32> {
        self.exit_code
    }

    pub(crate) const fn signal(&self) -> Option<i32> {
        self.signal
    }
}

#[derive(Clone, Copy)]
pub(crate) struct SandlockHostReceiptParts<'a> {
    pub(crate) plan: &'a SandlockLaunchPlan,
    pub(crate) preflight: &'a SandlockPreflightDecision,
    pub(crate) facts: &'a SandlockExecutionFacts,
}

/// Sandlock launch, preflight, or receipt failure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SandlockHostError {
    /// A safe metadata label was empty or unsafe.
    UnsafeLabel {
        /// Stable name of the unsafe label field.
        field: &'static str,
    },
    /// A root path was absolute, empty, or escaped its capability root.
    UnsafeRelativeRoot,
    /// A root label appeared more than once.
    DuplicateRootBinding,
    /// A descriptor-required writable root was absent.
    MissingWritableRoot,
    /// A root had broader access than the descriptor requested.
    UnexpectedWritableRoot,
    /// COW policy did not name an admitted writable workdir root.
    InvalidCowRoot,
    /// The host policy was malformed or incompatible with descriptor intent.
    InvalidPolicy {
        /// Stable bounded incompatibility reason.
        reason: &'static str,
    },
    /// The stronger authority layer denied the action.
    AuthorizationDenied,
    /// Host-supplied authorization freshness failed.
    AuthorizationStale,
    /// One exact authority identity did not match the launch request.
    AuthorizationMismatch {
        /// Stable name of the mismatched identity field.
        field: &'static str,
    },
    /// Best-effort enforcement is unsupported by this fail-closed adapter.
    BestEffortUnsupported,
    /// The probe was for another plan or cohort.
    ProbeIdentityMismatch,
    /// One or more requested mechanisms were not proven.
    MissingCapability,
    /// One exact capability-specific functional command failed.
    MechanismProbeFailed {
        /// Mechanism whose reviewed functional command did not pass.
        capability: SandlockCapability,
    },
    /// Execution facts disagreed with the admitted plan or preflight.
    ExecutionEvidenceMismatch,
    /// Raw or unsafe receipt evidence was detected.
    UnsafeReceiptEvidence,
    /// The target command was empty or contained an invalid argument.
    InvalidCommand,
    /// This build target cannot host the Linux-only adapter.
    UnsupportedPlatform,
    /// Capability-root resolution failed.
    RootResolutionFailed,
    /// Upstream Sandlock policy or process setup failed.
    SandlockSetupFailed,
}

impl core::fmt::Display for SandlockHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "{self:?}")
    }
}

impl std::error::Error for SandlockHostError {}

pub(crate) fn descriptor_requires_cow(descriptor: &SandboxDescriptor) -> bool {
    descriptor.workdir_mode() == SandboxWorkdirMode::CowStaged
        || descriptor.cow_disposition() != SandboxCowDisposition::Disabled
}

pub(crate) fn descriptor_requires_resources(descriptor: &SandboxDescriptor) -> bool {
    descriptor.process_profile() == SandboxProcessProfile::ResourceLimited
}

pub(crate) fn descriptor_requires_http(descriptor: &SandboxDescriptor) -> bool {
    descriptor.http_policy() != SandboxHttpPolicy::None
}

pub(crate) fn descriptor_requires_determinism(descriptor: &SandboxDescriptor) -> bool {
    descriptor.determinism_policy() != animus_adapters::SandboxDeterminismPolicy::Host
}

pub(crate) fn descriptor_has_minimal_environment(descriptor: &SandboxDescriptor) -> bool {
    descriptor.environment_policy() == SandboxEnvironmentPolicy::Minimal
}

pub(crate) fn descriptor_network_policy(descriptor: &SandboxDescriptor) -> SandboxNetworkPolicy {
    descriptor.network_policy()
}

pub(crate) fn descriptor_filesystem_mode(descriptor: &SandboxDescriptor) -> SandboxFilesystemMode {
    descriptor.filesystem_mode()
}
