use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::os::fd::AsRawFd;
use std::os::unix::net::UnixStream as StdUnixStream;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::{Duration, UNIX_EPOCH};

use animus_adapters::{SafeHash, SandboxCowDisposition};
use sandlock_core::policy::{BranchAction, ByteSize};
use sandlock_core::{ExitStatus, Policy, PolicyBuilder, Sandbox};
use tokio::io::AsyncReadExt;
use tokio::net::UnixStream;

use super::core::{
    evaluate_sandlock_preflight, execution_facts_hash, project_sandlock_receipt,
    sandlock_command_hash,
};
use super::model::{
    SandlockCapability, SandlockCleanupStatus, SandlockCowOutcome, SandlockExecutionFacts,
    SandlockExecutionStatus, SandlockHostError, SandlockHostReceipt, SandlockLaunchPlan,
    SandlockPreflightDecision, SandlockProbeReport, SandlockRootAccess,
};

const OUTPUT_READ_BUFFER_BYTES: usize = 8_192;
const CANCELLATION_POLL_MILLIS: u64 = 10;
const EMPTY_OUTPUT_BYTES: u64 = 0;
const PROBE_HASH_DOMAIN: &str = "animus.sandlock-functional-probe.v1";
const COW_SNAPSHOT_HASH_DOMAIN: &str = "animus.sandlock-cow-snapshot.v1";
const COW_EVIDENCE_HASH_DOMAIN: &str = "animus.sandlock-cow-evidence.v1";
const COW_SNAPSHOT_MAX_ENTRIES: u32 = 4_096;
const COW_SNAPSHOT_MAX_BYTES: u64 = 67_108_864;
const COW_READ_BUFFER_BYTES: usize = 8_192;

/// Explicit capability-root grant owned by the Linux shell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockCapabilityRootGrant {
    label: String,
    grant_hash: SafeHash,
    root_path: PathBuf,
}

impl SandlockCapabilityRootGrant {
    /// Builds an explicit host capability root.
    ///
    /// # Errors
    ///
    /// Returns [`SandlockHostError`] when the safe label is invalid or the root
    /// is not an absolute lexical path.
    pub fn new(
        label: &str,
        grant_hash: SafeHash,
        root_path: PathBuf,
    ) -> Result<Self, SandlockHostError> {
        super::core::validate_safe_label("capability_root_label", label)?;
        if !is_safe_absolute_root(root_path.as_path()) {
            return Err(SandlockHostError::RootResolutionFailed);
        }
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

/// Resolved concrete root retained only inside the authority-scoped shell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockResolvedRoot {
    label: String,
    path: PathBuf,
    access: SandlockRootAccess,
}

impl SandlockResolvedRoot {
    /// Returns the safe label without exposing it to durable evidence automatically.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub(crate) fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub(crate) const fn access(&self) -> SandlockRootAccess {
        self.access
    }
}

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

    /// Requests cancellation of probe or target execution.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::Release);
    }

    /// Returns whether cancellation was observed.
    #[must_use]
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

/// One capability-specific functional probe command owned by the host profile.
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

    /// Returns the mechanism this command proves on successful exit.
    #[must_use]
    pub const fn capability(self) -> SandlockCapability {
        self.capability
    }

    /// Returns the exact bounded probe command.
    #[must_use]
    pub const fn command(self) -> &'a [&'a str] {
        self.command
    }
}

/// Borrowed Linux host execution request.
#[derive(Clone, Copy, Debug)]
pub struct SandlockHostExecutionRequest<'a> {
    /// Pure admitted launch plan.
    pub plan: &'a SandlockLaunchPlan,
    /// Explicit host capability roots.
    pub root_grants: &'a [SandlockCapabilityRootGrant],
    /// One reviewed functional probe command for every requested mechanism.
    pub mechanism_probes: &'a [SandlockMechanismProbe<'a>],
    /// Exact target command whose hash is bound by the launch plan.
    pub command: &'a [&'a str],
    /// Explicit cancellation handle.
    pub cancellation: &'a SandlockCancellation,
}

/// Successful host orchestration result with no raw output bodies.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SandlockHostExecutionResult {
    /// Functional probe report for the exact plan and cohort.
    pub probe: SandlockProbeReport,
    /// Pure all-capability preflight decision.
    pub preflight: SandlockPreflightDecision,
    /// Metadata-only execution facts.
    pub facts: SandlockExecutionFacts,
    /// Self-verifying metadata-only receipt.
    pub receipt: SandlockHostReceipt,
}

/// Resolves every safe relative binding inside an exact capability root.
///
/// # Errors
///
/// Returns [`SandlockHostError`] for missing, duplicate, mismatched, escaping,
/// symlinked-outside, or non-existent resolved roots.
pub fn resolve_sandlock_roots(
    plan: &SandlockLaunchPlan,
    grants: &[SandlockCapabilityRootGrant],
) -> Result<Vec<SandlockResolvedRoot>, SandlockHostError> {
    // r[impl agent_harness_runtime.sandlock_host_adapter.preflight]
    if grants.len() != plan.root_bindings().len() {
        return Err(SandlockHostError::RootResolutionFailed);
    }
    let mut resolved = Vec::with_capacity(plan.root_bindings().len());
    for binding in plan.root_bindings() {
        let matches = grants
            .iter()
            .filter(|grant| grant.label == binding.label())
            .collect::<Vec<_>>();
        if matches.len() != 1 || matches[0].grant_hash != *binding.grant_hash() {
            return Err(SandlockHostError::RootResolutionFailed);
        }
        let grant_root = matches[0]
            .root_path
            .canonicalize()
            .map_err(|_| SandlockHostError::RootResolutionFailed)?;
        let candidate = grant_root.join(binding.relative_path());
        let concrete = candidate
            .canonicalize()
            .map_err(|_| SandlockHostError::RootResolutionFailed)?;
        if !concrete.starts_with(grant_root.as_path()) {
            return Err(SandlockHostError::RootResolutionFailed);
        }
        resolved.push(SandlockResolvedRoot {
            label: binding.label().to_owned(),
            path: concrete,
            access: binding.access(),
        });
    }
    Ok(resolved)
}

/// Runs exact-policy functional preflight, then the exact admitted target.
///
/// # Errors
///
/// Returns [`SandlockHostError`] when root resolution, policy construction,
/// functional probing, target setup, output limits, evidence binding, or receipt
/// verification fails. No unconstrained fallback command is executed.
pub async fn execute_sandlock_host(
    request: SandlockHostExecutionRequest<'_>,
) -> Result<SandlockHostExecutionResult, SandlockHostError> {
    // r[impl agent_harness_runtime.sandlock_host_adapter.execution]
    // r[impl agent_harness_runtime.sandlock_host_adapter.authority]
    if request.cancellation.is_cancelled() {
        return Err(SandlockHostError::ExecutionEvidenceMismatch);
    }
    let command_hash = sandlock_command_hash(request.command)?;
    if &command_hash != request.plan.command_hash() {
        return Err(SandlockHostError::AuthorizationMismatch {
            field: "command_hash",
        });
    }
    let roots = resolve_sandlock_roots(request.plan, request.root_grants)?;
    let cow_root = resolved_cow_root(request.plan, roots.as_slice())?;
    let target_policy = build_sandlock_policy(request.plan, roots.as_slice())?;
    let probe = run_functional_probe(ProbeRunRequest {
        plan: request.plan,
        target_policy: &target_policy,
        probes: request.mechanism_probes,
        cancellation: request.cancellation,
        cow_root: cow_root.as_deref(),
    })
    .await?;
    let preflight = evaluate_sandlock_preflight(request.plan, &probe)?;
    if !preflight.is_ready() {
        return Err(SandlockHostError::MissingCapability);
    }
    let mut facts = run_bounded(BoundedRunRequest {
        plan: request.plan,
        policy: &target_policy,
        command: request.command,
        cancellation: request.cancellation,
        cow_root: cow_root.as_deref(),
    })
    .await?;
    facts.execution_hash = execution_facts_hash(request.plan, &facts);
    let receipt = project_sandlock_receipt(request.plan, &preflight, &facts)?;
    Ok(SandlockHostExecutionResult {
        probe,
        preflight,
        facts,
        receipt,
    })
}

fn is_safe_absolute_root(path: &Path) -> bool {
    path.is_absolute()
        && path
            .components()
            .all(|component| !matches!(component, std::path::Component::ParentDir))
}

fn build_sandlock_policy(
    plan: &SandlockLaunchPlan,
    roots: &[SandlockResolvedRoot],
) -> Result<Policy, SandlockHostError> {
    let mut builder = Policy::builder().clean_env(true);
    builder = add_filesystem_roots(builder, roots);
    builder = add_network_policy(builder, plan);
    builder = add_http_policy(builder, plan);
    builder = add_resource_policy(builder, plan);
    builder = add_determinism_policy(builder, plan)?;
    builder = add_cow_policy(builder, plan, roots)?;
    builder
        .build()
        .map_err(|_| SandlockHostError::SandlockSetupFailed)
}

fn add_filesystem_roots(
    mut builder: PolicyBuilder,
    roots: &[SandlockResolvedRoot],
) -> PolicyBuilder {
    for root in roots {
        builder = match root.access() {
            SandlockRootAccess::ReadOnly => builder.fs_read(root.path()),
            SandlockRootAccess::ReadWrite => builder.fs_write(root.path()),
        };
    }
    builder
}

fn add_network_policy(mut builder: PolicyBuilder, plan: &SandlockLaunchPlan) -> PolicyBuilder {
    for rule in plan.policy().egress_allow() {
        builder = builder.net_allow(rule.clone());
    }
    builder
}

fn add_http_policy(mut builder: PolicyBuilder, plan: &SandlockLaunchPlan) -> PolicyBuilder {
    for rule in plan.policy().http_allow() {
        builder = builder.http_allow(rule);
    }
    for rule in plan.policy().http_deny() {
        builder = builder.http_deny(rule);
    }
    builder
}

fn add_resource_policy(mut builder: PolicyBuilder, plan: &SandlockLaunchPlan) -> PolicyBuilder {
    if let Some(value) = plan.policy().max_memory_bytes() {
        builder = builder.max_memory(ByteSize::bytes(value));
    }
    if let Some(value) = plan.policy().max_processes() {
        builder = builder.max_processes(value);
    }
    if let Some(value) = plan.policy().max_open_files() {
        builder = builder.max_open_files(value);
    }
    if let Some(value) = plan.policy().max_cpu_percent() {
        builder = builder.max_cpu(value);
    }
    builder
}

fn add_determinism_policy(
    mut builder: PolicyBuilder,
    plan: &SandlockLaunchPlan,
) -> Result<PolicyBuilder, SandlockHostError> {
    if let Some(epoch_seconds) = plan.policy().deterministic_epoch_seconds() {
        let time = UNIX_EPOCH
            .checked_add(Duration::from_secs(epoch_seconds))
            .ok_or(SandlockHostError::SandlockSetupFailed)?;
        builder = builder.time_start(time);
    }
    if let Some(seed) = plan.policy().deterministic_seed() {
        builder = builder.random_seed(seed);
    }
    Ok(builder)
}

fn add_cow_policy(
    mut builder: PolicyBuilder,
    plan: &SandlockLaunchPlan,
    roots: &[SandlockResolvedRoot],
) -> Result<PolicyBuilder, SandlockHostError> {
    let disposition = plan.descriptor().cow_disposition();
    if disposition == SandboxCowDisposition::Disabled {
        return Ok(builder);
    }
    let label = plan
        .policy()
        .cow_workdir_root()
        .ok_or(SandlockHostError::InvalidCowRoot)?;
    let root = roots
        .iter()
        .find(|root| root.label() == label && root.access() == SandlockRootAccess::ReadWrite)
        .ok_or(SandlockHostError::InvalidCowRoot)?;
    let on_exit = match disposition {
        SandboxCowDisposition::Disabled | SandboxCowDisposition::DryRun => BranchAction::Abort,
        SandboxCowDisposition::Commit => BranchAction::Commit,
    };
    builder = builder
        .workdir(root.path())
        .cwd(root.path())
        .on_exit(on_exit)
        .on_error(BranchAction::Abort);
    Ok(builder)
}

#[derive(Clone, Copy)]
struct ProbeRunRequest<'a> {
    plan: &'a SandlockLaunchPlan,
    target_policy: &'a Policy,
    probes: &'a [SandlockMechanismProbe<'a>],
    cancellation: &'a SandlockCancellation,
    cow_root: Option<&'a Path>,
}

fn resolved_cow_root(
    plan: &SandlockLaunchPlan,
    roots: &[SandlockResolvedRoot],
) -> Result<Option<PathBuf>, SandlockHostError> {
    if plan.descriptor().cow_disposition() == SandboxCowDisposition::Disabled {
        return Ok(None);
    }
    let label = plan
        .policy()
        .cow_workdir_root()
        .ok_or(SandlockHostError::InvalidCowRoot)?;
    let root = roots
        .iter()
        .find(|root| root.label() == label && root.access() == SandlockRootAccess::ReadWrite)
        .ok_or(SandlockHostError::InvalidCowRoot)?;
    Ok(Some(root.path().to_path_buf()))
}

async fn run_functional_probe(
    request: ProbeRunRequest<'_>,
) -> Result<SandlockProbeReport, SandlockHostError> {
    let abi = sandlock_core::landlock_abi_version()
        .map_err(|_| SandlockHostError::SandlockSetupFailed)?;
    if abi < sandlock_core::MIN_LANDLOCK_ABI {
        return Err(SandlockHostError::MissingCapability);
    }
    validate_mechanism_probe_suite(request.plan, request.probes)?;
    let mut probe_policy = request.target_policy.clone();
    probe_policy.on_exit = BranchAction::Abort;
    probe_policy.on_error = BranchAction::Abort;
    let mut proven = Vec::with_capacity(request.probes.len());
    let mut hasher = blake3::Hasher::new();
    hasher.update(PROBE_HASH_DOMAIN.as_bytes());
    hasher.update(request.plan.plan_hash().as_str().as_bytes());
    for probe in request.probes {
        let mut facts = run_bounded(BoundedRunRequest {
            plan: request.plan,
            policy: &probe_policy,
            command: probe.command(),
            cancellation: request.cancellation,
            cow_root: request.cow_root,
        })
        .await?;
        facts.execution_hash = execution_facts_hash(request.plan, &facts);
        let command_hash = sandlock_command_hash(probe.command())?;
        hasher.update(probe.capability().as_str().as_bytes());
        hasher.update(command_hash.as_str().as_bytes());
        hasher.update(facts.execution_hash.as_str().as_bytes());
        if facts.status != SandlockExecutionStatus::Passed {
            return Err(SandlockHostError::MechanismProbeFailed {
                capability: probe.capability(),
            });
        }
        proven.push(probe.capability());
    }
    let probe_hash = SafeHash::from_blake3_bytes(hasher.finalize().as_bytes());
    Ok(SandlockProbeReport::new(
        request.plan.plan_hash().clone(),
        request.plan.cohort_hash().clone(),
        proven,
        probe_hash,
    ))
}

fn validate_mechanism_probe_suite(
    plan: &SandlockLaunchPlan,
    probes: &[SandlockMechanismProbe<'_>],
) -> Result<(), SandlockHostError> {
    let required = plan
        .required_capabilities()
        .iter()
        .copied()
        .collect::<BTreeSet<_>>();
    let supplied = probes
        .iter()
        .map(|probe| probe.capability())
        .collect::<BTreeSet<_>>();
    if supplied != required || supplied.len() != probes.len() {
        return Err(SandlockHostError::MissingCapability);
    }
    for probe in probes {
        sandlock_command_hash(probe.command())?;
    }
    Ok(())
}

#[derive(Clone, Copy)]
struct BoundedRunRequest<'a> {
    plan: &'a SandlockLaunchPlan,
    policy: &'a Policy,
    command: &'a [&'a str],
    cancellation: &'a SandlockCancellation,
    cow_root: Option<&'a Path>,
}

async fn run_bounded(
    request: BoundedRunRequest<'_>,
) -> Result<SandlockExecutionFacts, SandlockHostError> {
    sandlock_command_hash(request.command)?;
    let before_cow = request.cow_root.map(cow_snapshot).transpose()?;
    let pipes = IoPipes::new()?;
    let mut sandbox =
        Sandbox::new(request.policy).map_err(|_| SandlockHostError::SandlockSetupFailed)?;
    sandbox
        .spawn_with_io(
            request.command,
            Some(pipes.child_stdin.as_raw_fd()),
            Some(pipes.child_stdout.as_raw_fd()),
            Some(pipes.child_stderr.as_raw_fd()),
        )
        .await
        .map_err(|_| SandlockHostError::SandlockSetupFailed)?;
    let readers = pipes.start_readers(request.plan.policy().max_output_bytes())?;
    let wait_outcome = wait_for_process(
        &mut sandbox,
        Duration::from_millis(request.plan.policy().timeout_millis()),
        request.cancellation,
        readers.output_exceeded.clone(),
    )
    .await;
    let process_cleanup = cleanup_after_wait(&mut sandbox, wait_outcome).await;
    let output_exceeded = readers.output_exceeded.clone();
    let (stdout, stderr) = readers.finish().await;
    drop(sandbox);
    let cow_evidence = evaluate_cow_cleanup(request.cow_root, before_cow.as_ref());
    let cleanup = combined_cleanup(process_cleanup, cow_evidence.complete);
    let status = classify_status(
        wait_outcome,
        cleanup,
        output_exceeded.load(Ordering::Acquire),
    );
    let (exit_code, signal) = exit_parts(wait_outcome);
    Ok(SandlockExecutionFacts {
        status,
        exit_code,
        signal,
        stdout_bytes: stdout.bytes,
        stderr_bytes: stderr.bytes,
        stdout_hash: stdout.hash,
        stderr_hash: stderr.hash,
        cow: cow_evidence.outcome,
        cow_change_count: cow_evidence.change_count,
        cow_digest: cow_evidence.digest,
        cleanup,
        enforced_capabilities: if status == SandlockExecutionStatus::SetupFailed {
            Vec::new()
        } else {
            request.plan.required_capabilities().to_vec()
        },
        execution_hash: SafeHash::from_blake3_bytes(b"pending-execution-hash"),
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct CowSnapshot {
    hash: SafeHash,
    entry_count: u32,
    byte_count: u64,
}

struct CowEvidence {
    outcome: SandlockCowOutcome,
    change_count: Option<u32>,
    digest: Option<SafeHash>,
    complete: bool,
}

fn cow_snapshot(root: &Path) -> Result<CowSnapshot, SandlockHostError> {
    let mut paths = Vec::new();
    collect_cow_paths(root, &mut paths)?;
    paths.sort();
    let entry_count =
        u32::try_from(paths.len()).map_err(|_| SandlockHostError::SandlockSetupFailed)?;
    if entry_count > COW_SNAPSHOT_MAX_ENTRIES {
        return Err(SandlockHostError::SandlockSetupFailed);
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(COW_SNAPSHOT_HASH_DOMAIN.as_bytes());
    let mut byte_count = EMPTY_OUTPUT_BYTES;
    for path in paths {
        hash_cow_entry(root, path.as_path(), &mut hasher, &mut byte_count)?;
    }
    Ok(CowSnapshot {
        hash: SafeHash::from_blake3_bytes(hasher.finalize().as_bytes()),
        entry_count,
        byte_count,
    })
}

fn collect_cow_paths(current: &Path, paths: &mut Vec<PathBuf>) -> Result<(), SandlockHostError> {
    for entry in fs::read_dir(current).map_err(|_| SandlockHostError::SandlockSetupFailed)? {
        let path = entry
            .map_err(|_| SandlockHostError::SandlockSetupFailed)?
            .path();
        let metadata = fs::symlink_metadata(path.as_path())
            .map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        paths.push(path.clone());
        let count =
            u32::try_from(paths.len()).map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        if count > COW_SNAPSHOT_MAX_ENTRIES {
            return Err(SandlockHostError::SandlockSetupFailed);
        }
        if metadata.is_dir() {
            collect_cow_paths(path.as_path(), paths)?;
        }
    }
    Ok(())
}

fn hash_cow_entry(
    root: &Path,
    path: &Path,
    hasher: &mut blake3::Hasher,
    byte_count: &mut u64,
) -> Result<(), SandlockHostError> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| SandlockHostError::SandlockSetupFailed)?;
    hasher.update(relative.as_os_str().as_encoded_bytes());
    let metadata =
        fs::symlink_metadata(path).map_err(|_| SandlockHostError::SandlockSetupFailed)?;
    if metadata.is_dir() {
        hasher.update(b"directory");
        return Ok(());
    }
    if metadata.file_type().is_symlink() {
        hasher.update(b"symlink");
        let target = fs::read_link(path).map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        hasher.update(target.as_os_str().as_encoded_bytes());
        return Ok(());
    }
    if !metadata.is_file() {
        return Err(SandlockHostError::SandlockSetupFailed);
    }
    hasher.update(b"file");
    let mut file = fs::File::open(path).map_err(|_| SandlockHostError::SandlockSetupFailed)?;
    let mut buffer = [0_u8; COW_READ_BUFFER_BYTES];
    loop {
        let read = file
            .read(&mut buffer)
            .map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        if read == 0 {
            break;
        }
        let read_u64 = u64::try_from(read).map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        *byte_count = (*byte_count).saturating_add(read_u64);
        if *byte_count > COW_SNAPSHOT_MAX_BYTES {
            return Err(SandlockHostError::SandlockSetupFailed);
        }
        hasher.update(&buffer[..read]);
    }
    Ok(())
}

fn evaluate_cow_cleanup(root: Option<&Path>, before: Option<&CowSnapshot>) -> CowEvidence {
    let (Some(root), Some(before)) = (root, before) else {
        return CowEvidence {
            outcome: SandlockCowOutcome::NotRequested,
            change_count: None,
            digest: None,
            complete: true,
        };
    };
    let Ok(after) = cow_snapshot(root) else {
        return CowEvidence {
            outcome: SandlockCowOutcome::Failed,
            change_count: None,
            digest: None,
            complete: false,
        };
    };
    let complete = before == &after;
    CowEvidence {
        outcome: if complete {
            SandlockCowOutcome::Discarded
        } else {
            SandlockCowOutcome::Failed
        },
        change_count: complete.then_some(0),
        digest: Some(cow_evidence_hash(before, &after)),
        complete,
    }
}

fn cow_evidence_hash(before: &CowSnapshot, after: &CowSnapshot) -> SafeHash {
    let mut hasher = blake3::Hasher::new();
    hasher.update(COW_EVIDENCE_HASH_DOMAIN.as_bytes());
    hasher.update(before.hash.as_str().as_bytes());
    hasher.update(before.entry_count.to_string().as_bytes());
    hasher.update(before.byte_count.to_string().as_bytes());
    hasher.update(after.hash.as_str().as_bytes());
    hasher.update(after.entry_count.to_string().as_bytes());
    hasher.update(after.byte_count.to_string().as_bytes());
    SafeHash::from_blake3_bytes(hasher.finalize().as_bytes())
}

fn combined_cleanup(
    process_cleanup: SandlockCleanupStatus,
    cow_complete: bool,
) -> SandlockCleanupStatus {
    if process_cleanup == SandlockCleanupStatus::Complete && cow_complete {
        SandlockCleanupStatus::Complete
    } else {
        SandlockCleanupStatus::Failed
    }
}

#[derive(Clone, Copy, Debug)]
enum WaitOutcome {
    Exited(ExitClass),
    TimedOut,
    Cancelled,
    OutputLimitExceeded,
    WaitFailed,
}

#[derive(Clone, Copy, Debug)]
enum ExitClass {
    Code(i32),
    Signal(i32),
    Killed,
    Timeout,
}

async fn wait_for_process(
    sandbox: &mut Sandbox,
    timeout: Duration,
    cancellation: &SandlockCancellation,
    output_exceeded: Arc<AtomicBool>,
) -> WaitOutcome {
    let wait = sandbox.wait();
    tokio::pin!(wait);
    tokio::select! {
        result = &mut wait => match result {
            Ok(run) => WaitOutcome::Exited(map_exit(&run.exit_status)),
            Err(_) => WaitOutcome::WaitFailed,
        },
        () = tokio::time::sleep(timeout) => WaitOutcome::TimedOut,
        () = wait_for_stop(cancellation, output_exceeded.clone()) => {
            if output_exceeded.load(Ordering::Acquire) {
                WaitOutcome::OutputLimitExceeded
            } else {
                WaitOutcome::Cancelled
            }
        }
    }
}

async fn wait_for_stop(cancellation: &SandlockCancellation, output_exceeded: Arc<AtomicBool>) {
    let poll_interval = Duration::from_millis(CANCELLATION_POLL_MILLIS);
    loop {
        if cancellation.is_cancelled() || output_exceeded.load(Ordering::Acquire) {
            return;
        }
        tokio::time::sleep(poll_interval).await;
    }
}

async fn cleanup_after_wait(sandbox: &mut Sandbox, outcome: WaitOutcome) -> SandlockCleanupStatus {
    if matches!(outcome, WaitOutcome::Exited(_)) {
        return SandlockCleanupStatus::Complete;
    }
    let kill_ok = sandbox.kill().is_ok();
    let wait_ok = sandbox.wait().await.is_ok();
    if kill_ok && wait_ok {
        SandlockCleanupStatus::Complete
    } else {
        SandlockCleanupStatus::Failed
    }
}

fn classify_status(
    outcome: WaitOutcome,
    cleanup: SandlockCleanupStatus,
    output_exceeded: bool,
) -> SandlockExecutionStatus {
    if cleanup == SandlockCleanupStatus::Failed {
        return SandlockExecutionStatus::Failed;
    }
    if output_exceeded || matches!(outcome, WaitOutcome::OutputLimitExceeded) {
        return SandlockExecutionStatus::OutputLimitExceeded;
    }
    match outcome {
        WaitOutcome::Exited(ExitClass::Code(0)) => SandlockExecutionStatus::Passed,
        WaitOutcome::Exited(_) => SandlockExecutionStatus::Failed,
        WaitOutcome::TimedOut => SandlockExecutionStatus::TimedOut,
        WaitOutcome::Cancelled => SandlockExecutionStatus::Cancelled,
        WaitOutcome::OutputLimitExceeded => SandlockExecutionStatus::OutputLimitExceeded,
        WaitOutcome::WaitFailed => SandlockExecutionStatus::SetupFailed,
    }
}

fn exit_parts(outcome: WaitOutcome) -> (Option<i32>, Option<i32>) {
    match outcome {
        WaitOutcome::Exited(ExitClass::Code(code)) => (Some(code), None),
        WaitOutcome::Exited(ExitClass::Signal(signal)) => (None, Some(signal)),
        WaitOutcome::Exited(ExitClass::Killed | ExitClass::Timeout)
        | WaitOutcome::TimedOut
        | WaitOutcome::Cancelled
        | WaitOutcome::OutputLimitExceeded
        | WaitOutcome::WaitFailed => (None, None),
    }
}

fn map_exit(status: &ExitStatus) -> ExitClass {
    match status {
        ExitStatus::Code(code) => ExitClass::Code(*code),
        ExitStatus::Signal(signal) => ExitClass::Signal(*signal),
        ExitStatus::Killed => ExitClass::Killed,
        ExitStatus::Timeout => ExitClass::Timeout,
    }
}

struct IoPipes {
    child_stdin: StdUnixStream,
    child_stdout: StdUnixStream,
    child_stderr: StdUnixStream,
    parent_stdout: StdUnixStream,
    parent_stderr: StdUnixStream,
}

impl IoPipes {
    fn new() -> Result<Self, SandlockHostError> {
        let (parent_stdin, child_stdin) =
            StdUnixStream::pair().map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        let (parent_stdout, child_stdout) =
            StdUnixStream::pair().map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        let (parent_stderr, child_stderr) =
            StdUnixStream::pair().map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        drop(parent_stdin);
        Ok(Self {
            child_stdin,
            child_stdout,
            child_stderr,
            parent_stdout,
            parent_stderr,
        })
    }

    fn start_readers(self, max_bytes: u64) -> Result<OutputReaders, SandlockHostError> {
        self.parent_stdout
            .set_nonblocking(true)
            .map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        self.parent_stderr
            .set_nonblocking(true)
            .map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        let stdout = UnixStream::from_std(self.parent_stdout)
            .map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        let stderr = UnixStream::from_std(self.parent_stderr)
            .map_err(|_| SandlockHostError::SandlockSetupFailed)?;
        drop(self.child_stdin);
        drop(self.child_stdout);
        drop(self.child_stderr);
        let combined = Arc::new(AtomicU64::new(EMPTY_OUTPUT_BYTES));
        let output_exceeded = Arc::new(AtomicBool::new(false));
        let stdout_task = tokio::spawn(read_stream_digest(
            stdout,
            combined.clone(),
            output_exceeded.clone(),
            max_bytes,
        ));
        let stderr_task = tokio::spawn(read_stream_digest(
            stderr,
            combined,
            output_exceeded.clone(),
            max_bytes,
        ));
        Ok(OutputReaders {
            stdout_task,
            stderr_task,
            output_exceeded,
        })
    }
}

struct OutputReaders {
    stdout_task: tokio::task::JoinHandle<StreamDigest>,
    stderr_task: tokio::task::JoinHandle<StreamDigest>,
    output_exceeded: Arc<AtomicBool>,
}

impl OutputReaders {
    async fn finish(self) -> (StreamDigest, StreamDigest) {
        let stdout = self.stdout_task.await.unwrap_or_default();
        let stderr = self.stderr_task.await.unwrap_or_default();
        (stdout, stderr)
    }
}

#[derive(Clone, Debug, Default)]
struct StreamDigest {
    bytes: u64,
    hash: Option<SafeHash>,
}

async fn read_stream_digest(
    mut stream: UnixStream,
    combined: Arc<AtomicU64>,
    output_exceeded: Arc<AtomicBool>,
    max_bytes: u64,
) -> StreamDigest {
    let mut buffer = [0_u8; OUTPUT_READ_BUFFER_BYTES];
    let mut bytes = EMPTY_OUTPUT_BYTES;
    let mut hasher = blake3::Hasher::new();
    loop {
        let read = match stream.read(&mut buffer).await {
            Ok(0) | Err(_) => break,
            Ok(count) => count,
        };
        let count = u64::try_from(read).unwrap_or(u64::MAX);
        bytes = bytes.saturating_add(count);
        let prior = combined.fetch_add(count, Ordering::AcqRel);
        if prior.saturating_add(count) > max_bytes {
            output_exceeded.store(true, Ordering::Release);
            break;
        }
        hasher.update(&buffer[..read]);
    }
    StreamDigest {
        bytes,
        hash: Some(SafeHash::from_blake3_bytes(hasher.finalize().as_bytes())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cancellation_is_explicit_and_monotonic() {
        let cancellation = SandlockCancellation::new();
        assert!(!cancellation.is_cancelled());
        cancellation.cancel();
        assert!(cancellation.is_cancelled());
    }

    #[test]
    fn capability_root_rejects_relative_and_parent_paths() {
        let hash = SafeHash::from_blake3_bytes(b"grant");
        assert!(
            SandlockCapabilityRootGrant::new("root", hash.clone(), PathBuf::from("relative"))
                .is_err()
        );
        assert!(
            SandlockCapabilityRootGrant::new("root", hash, PathBuf::from("/tmp/../escape"))
                .is_err()
        );
    }
}
