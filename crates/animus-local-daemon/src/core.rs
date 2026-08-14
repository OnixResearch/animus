//! Deterministic supervisor lifecycle and routing core.

use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

const REGISTRY_SCHEMA: &str = "animus.local-daemon-registry.v1";
const RECEIPT_SCHEMA: &str = "animus.local-daemon-receipt.v1";
const MAX_LABEL_BYTES: usize = 256;
const MIN_GENERATION: u64 = 1;

/// Worker lifecycle known to the supervisor.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WorkerLifecycle {
    /// A process start was admitted but readiness is incomplete.
    Starting,
    /// The exact generation completed readiness.
    Ready,
    /// A live worker lost its endpoint.
    Disconnected,
    /// Authorized stop finalization is in progress.
    Stopping,
    /// Complete-tree teardown was observed.
    Stopped,
    /// The registry has no matching live process.
    Stale,
    /// A PID names another process identity.
    IdentityMismatch,
    /// The process exited but was not reaped.
    Zombie,
    /// The live process missed its bounded health deadline.
    Unresponsive,
    /// Process start failed before readiness.
    FailedStart,
    /// Restore validation blocked continuation.
    RestoreBlocked,
}

/// Worker restore status projected without session content.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RestoreStatus {
    /// No prior durable state exists.
    Fresh,
    /// Durable session and kernel heads matched.
    Restored,
    /// Durable restore is incompatible or unavailable.
    Blocked,
}

/// One client attachment lease.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct Attachment {
    /// Client identity.
    pub client_id: String,
    /// Bounded event sequence visible at attach time.
    pub last_sequence: u64,
}

/// Durable metadata-only record for one root worker generation.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorkerRegistryRecord {
    /// Root session identity.
    pub root_id: String,
    /// Worker identity.
    pub worker_id: String,
    /// Monotonic worker generation.
    pub generation: u64,
    /// Host process identifier.
    pub process_id: u32,
    /// BLAKE3 identity over host-observed process facts.
    pub process_identity: String,
    /// Explicit endpoint identity.
    pub endpoint_identity: String,
    /// Latest safe session head identity.
    pub session_head_identity: String,
    /// Current lifecycle.
    pub lifecycle: WorkerLifecycle,
    /// Current restore status.
    pub restore_status: RestoreStatus,
    /// Current client attachments.
    pub attachments: Vec<Attachment>,
    /// Last bounded event sequence.
    pub event_sequence: u64,
    /// Metadata-only record identity.
    pub receipt_identity: String,
}

/// Canonical registry ordered by root identity.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct WorkerRegistry {
    /// Registry schema.
    pub schema: String,
    /// Worker records.
    pub workers: Vec<WorkerRegistryRecord>,
}

impl WorkerRegistry {
    /// Creates and validates canonical registry state.
    ///
    /// # Errors
    ///
    /// Returns [`LocalDaemonError`] for invalid labels, duplicates, or generations.
    pub fn new(
        mut workers: Vec<WorkerRegistryRecord>,
        max_workers: u16,
    ) -> Result<Self, LocalDaemonError> {
        if workers.len() > usize::from(max_workers) {
            return Err(LocalDaemonError::BoundExceeded);
        }
        for worker in &workers {
            validate_record(worker)?;
        }
        workers.sort_by(|left, right| left.root_id.cmp(&right.root_id));
        if workers.windows(2).any(|pair| {
            pair[0].root_id == pair[1].root_id || pair[0].worker_id == pair[1].worker_id
        }) {
            return Err(LocalDaemonError::DuplicateWorker);
        }
        Ok(Self {
            schema: String::from(REGISTRY_SCHEMA),
            workers,
        })
    }

    /// Returns one root record.
    #[must_use]
    pub fn find(&self, root_id: &str) -> Option<&WorkerRegistryRecord> {
        self.workers.iter().find(|worker| worker.root_id == root_id)
    }
}

/// Supplied process and readiness facts for recovery classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "facts are independent host observations"
)]
pub struct HealthFacts {
    /// Whether the recorded process identifier exists.
    pub process_exists: bool,
    /// Whether current process identity matches the record.
    pub identity_matches: bool,
    /// Whether the process is a zombie.
    pub zombie: bool,
    /// Whether the endpoint is connected.
    pub endpoint_connected: bool,
    /// Whether a health response arrived before deadline.
    pub health_response: bool,
}

/// Recovery classification.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RecoveryKind {
    /// Existing worker is healthy and ready.
    Healthy,
    /// Existing worker is still starting.
    Starting,
    /// Existing worker is stopping.
    Stopping,
    /// Existing worker is stopped.
    Stopped,
    /// Existing worker is disconnected.
    Disconnected,
    /// Registry record is stale.
    Stale,
    /// PID identity belongs to another process.
    IdentityMismatch,
    /// Process is a zombie.
    Zombie,
    /// Worker is unresponsive.
    Unresponsive,
    /// Worker start failed.
    FailedStart,
    /// Restore blocks continuation.
    RestoreBlocked,
}

/// Pure recovery result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct RecoveryDecision {
    /// Current classification.
    pub kind: RecoveryKind,
    /// Whether attachment is admitted.
    pub attachment_allowed: bool,
    /// Whether complete-tree teardown is required.
    pub teardown_required: bool,
    /// Whether explicit retry can create a new generation.
    pub retry_allowed: bool,
}

/// Classifies one worker from supplied facts.
#[must_use]
pub fn decide_recovery(
    lifecycle: WorkerLifecycle,
    restore_status: RestoreStatus,
    facts: HealthFacts,
) -> RecoveryDecision {
    if restore_status == RestoreStatus::Blocked || lifecycle == WorkerLifecycle::RestoreBlocked {
        return recovery(RecoveryKind::RestoreBlocked, false, false, false);
    }
    if lifecycle == WorkerLifecycle::Starting {
        return recovery(RecoveryKind::Starting, false, false, false);
    }
    if lifecycle == WorkerLifecycle::Stopping {
        return recovery(RecoveryKind::Stopping, false, true, false);
    }
    if lifecycle == WorkerLifecycle::Stopped {
        return recovery(RecoveryKind::Stopped, false, false, true);
    }
    if lifecycle == WorkerLifecycle::FailedStart {
        return recovery(RecoveryKind::FailedStart, false, true, true);
    }
    if !facts.process_exists {
        return recovery(RecoveryKind::Stale, false, false, true);
    }
    if !facts.identity_matches {
        return recovery(RecoveryKind::IdentityMismatch, false, false, true);
    }
    if facts.zombie {
        return recovery(RecoveryKind::Zombie, false, true, true);
    }
    if !facts.endpoint_connected {
        return recovery(RecoveryKind::Disconnected, false, true, true);
    }
    if !facts.health_response {
        return recovery(RecoveryKind::Unresponsive, false, true, true);
    }
    recovery(RecoveryKind::Healthy, true, false, false)
}

fn recovery(
    kind: RecoveryKind,
    attachment_allowed: bool,
    teardown_required: bool,
    retry_allowed: bool,
) -> RecoveryDecision {
    RecoveryDecision {
        kind,
        attachment_allowed,
        teardown_required,
        retry_allowed,
    }
}

/// Returns the next admitted worker generation.
///
/// # Errors
///
/// Returns [`LocalDaemonError::GenerationOverflow`] on overflow.
pub fn next_generation(current: u64) -> Result<u64, LocalDaemonError> {
    current
        .checked_add(MIN_GENERATION)
        .ok_or(LocalDaemonError::GenerationOverflow)
}

/// Admits an exact readiness handshake.
///
/// # Errors
///
/// Returns [`LocalDaemonError`] when root, worker, generation, process, endpoint,
/// or restore facts do not match the start record.
pub fn admit_ready(
    expected: &WorkerRegistryRecord,
    observed: &WorkerRegistryRecord,
) -> Result<WorkerRegistryRecord, LocalDaemonError> {
    if expected.root_id != observed.root_id
        || expected.worker_id != observed.worker_id
        || expected.generation != observed.generation
        || expected.process_id != observed.process_id
        || expected.process_identity != observed.process_identity
        || expected.endpoint_identity != observed.endpoint_identity
        || observed.lifecycle != WorkerLifecycle::Ready
        || observed.restore_status == RestoreStatus::Blocked
    {
        return Err(LocalDaemonError::IdentityMismatch);
    }
    validate_record(observed)?;
    Ok(observed.clone())
}

/// A supervisor routing request over supplied registry state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RoutingRequest {
    /// Return a snapshot before event flow.
    Snapshot {
        /// Root session identity.
        root_id: String,
    },
    /// Attach one bounded subscription.
    Attach {
        /// Root session identity.
        root_id: String,
        /// Client identity.
        client_id: String,
        /// Maximum retained events.
        max_events: u32,
    },
    /// Detach one client lease only.
    Detach {
        /// Root session identity.
        root_id: String,
        /// Client identity.
        client_id: String,
    },
    /// Route a typed command without executing it.
    Command {
        /// Root session identity.
        root_id: String,
        /// Command correlation identity.
        correlation_id: String,
    },
}

/// Pure routing result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RoutingDecision {
    /// Snapshot-first attachment projection.
    Snapshot {
        /// Worker identity.
        worker_id: String,
        /// Worker generation.
        generation: u64,
        /// Last bounded event sequence.
        event_sequence: u64,
    },
    /// Attachment accepted.
    Attached {
        /// Worker identity.
        worker_id: String,
        /// Worker execution remains independent from attachment state.
        execution_continues: bool,
    },
    /// Attachment removed only.
    Detached {
        /// Worker identity.
        worker_id: String,
        /// Worker execution remains active.
        execution_continues: bool,
    },
    /// Command target selected.
    Routed {
        /// Worker identity.
        worker_id: String,
        /// Correlation identity.
        correlation_id: String,
    },
}

/// Decides routing without provider, tool, Steel, or session execution.
///
/// # Errors
///
/// Returns [`LocalDaemonError`] for unknown, unhealthy, overbound, or invalid requests.
pub fn decide_routing(
    registry: &WorkerRegistry,
    request: &RoutingRequest,
    max_attachments: u16,
    configured_max_events: u32,
) -> Result<RoutingDecision, LocalDaemonError> {
    let root_id = match request {
        RoutingRequest::Snapshot { root_id }
        | RoutingRequest::Attach { root_id, .. }
        | RoutingRequest::Detach { root_id, .. }
        | RoutingRequest::Command { root_id, .. } => root_id,
    };
    let worker = registry
        .find(root_id)
        .ok_or(LocalDaemonError::UnknownWorker)?;
    if worker.lifecycle != WorkerLifecycle::Ready || worker.restore_status == RestoreStatus::Blocked
    {
        return Err(LocalDaemonError::WorkerUnavailable);
    }
    match request {
        RoutingRequest::Snapshot { .. } => Ok(RoutingDecision::Snapshot {
            worker_id: worker.worker_id.clone(),
            generation: worker.generation,
            event_sequence: worker.event_sequence,
        }),
        RoutingRequest::Attach {
            client_id,
            max_events,
            ..
        } => {
            validate_label(client_id)?;
            if *max_events == 0
                || *max_events > configured_max_events
                || worker.attachments.len() >= usize::from(max_attachments)
            {
                return Err(LocalDaemonError::BoundExceeded);
            }
            Ok(RoutingDecision::Attached {
                worker_id: worker.worker_id.clone(),
                execution_continues: true,
            })
        }
        RoutingRequest::Detach { client_id, .. } => {
            validate_label(client_id)?;
            Ok(RoutingDecision::Detached {
                worker_id: worker.worker_id.clone(),
                execution_continues: true,
            })
        }
        RoutingRequest::Command { correlation_id, .. } => {
            validate_label(correlation_id)?;
            Ok(RoutingDecision::Routed {
                worker_id: worker.worker_id.clone(),
                correlation_id: correlation_id.clone(),
            })
        }
    }
}

/// Supplied stop-finalization facts.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(
    clippy::struct_excessive_bools,
    reason = "facts are independent teardown observations"
)]
pub struct StopFacts {
    /// Whether stop was authorized.
    pub authorized: bool,
    /// Whether the root process exited.
    pub root_exited: bool,
    /// Whether all owned descendants exited.
    pub descendants_exited: bool,
    /// Whether the root process was reaped.
    pub root_reaped: bool,
}

/// Pure stop decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct StopDecision {
    /// Next lifecycle.
    pub lifecycle: WorkerLifecycle,
    /// Whether stop finalization must continue without clients.
    pub continue_finalization: bool,
    /// Whether stopped can be committed to the registry.
    pub commit_stopped: bool,
}

/// Decides stop finalization from process-tree facts only.
///
/// # Errors
///
/// Returns [`LocalDaemonError::AuthorityDenied`] when stop lacks authority.
pub fn decide_stop(facts: StopFacts) -> Result<StopDecision, LocalDaemonError> {
    if !facts.authorized {
        return Err(LocalDaemonError::AuthorityDenied);
    }
    let complete = facts.root_exited && facts.descendants_exited && facts.root_reaped;
    Ok(StopDecision {
        lifecycle: if complete {
            WorkerLifecycle::Stopped
        } else {
            WorkerLifecycle::Stopping
        },
        continue_finalization: !complete,
        commit_stopped: complete,
    })
}

/// Bounded aggregate counts for one daemon receipt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalDaemonReceiptCounts {
    /// Worker count.
    pub workers: usize,
    /// Attachment count.
    pub attachments: usize,
    /// Event count.
    pub events: usize,
}

/// Metadata-only daemon observation receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalDaemonReceipt {
    /// Receipt schema.
    pub schema: &'static str,
    /// BLAKE3 configuration identity.
    pub config_identity: String,
    /// BLAKE3 registry identity.
    pub registry_identity: String,
    /// Worker count.
    pub worker_count: u16,
    /// Attachment count.
    pub attachment_count: u32,
    /// Event count.
    pub event_count: u32,
    /// Required claim exclusions.
    pub non_claims: Vec<String>,
    /// Receipt identity.
    pub identity: String,
}

impl LocalDaemonReceipt {
    /// Builds a metadata-only receipt.
    ///
    /// # Errors
    ///
    /// Returns [`LocalDaemonError`] for duplicate non-claims or count overflow.
    pub fn new(
        config: &crate::ValidatedLocalDaemonConfig,
        registry_identity: &str,
        counts: LocalDaemonReceiptCounts,
    ) -> Result<Self, LocalDaemonError> {
        validate_hash(config.identity())?;
        validate_hash(registry_identity)?;
        let worker_count =
            u16::try_from(counts.workers).map_err(|_error| LocalDaemonError::BoundExceeded)?;
        let attachment_count =
            u32::try_from(counts.attachments).map_err(|_error| LocalDaemonError::BoundExceeded)?;
        let event_count =
            u32::try_from(counts.events).map_err(|_error| LocalDaemonError::BoundExceeded)?;
        let non_claims = config.non_claims().to_vec();
        let unique: BTreeSet<&String> = non_claims.iter().collect();
        if unique.len() != non_claims.len() {
            return Err(LocalDaemonError::UnsafeEvidence);
        }
        let material = format!(
            "{RECEIPT_SCHEMA}\0{}\0{registry_identity}\0{worker_count}\0{attachment_count}\0{event_count}\0{}",
            config.identity(),
            non_claims.join("\0")
        );
        Ok(Self {
            schema: RECEIPT_SCHEMA,
            config_identity: String::from(config.identity()),
            registry_identity: String::from(registry_identity),
            worker_count,
            attachment_count,
            event_count,
            non_claims,
            identity: blake3::hash(material.as_bytes()).to_hex().to_string(),
        })
    }
}

/// Safe local daemon failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LocalDaemonError {
    /// A label or BLAKE3 identity is malformed.
    InvalidIdentity,
    /// Registry has duplicate root or worker ownership.
    DuplicateWorker,
    /// A configured or requested bound was exceeded.
    BoundExceeded,
    /// No worker owns the requested root.
    UnknownWorker,
    /// Current worker cannot receive traffic.
    WorkerUnavailable,
    /// Readiness or process identity facts differ.
    IdentityMismatch,
    /// Monotonic generation overflowed.
    GenerationOverflow,
    /// Required authority is absent.
    AuthorityDenied,
    /// Evidence contains an unsafe or duplicated claim.
    UnsafeEvidence,
}

impl core::fmt::Display for LocalDaemonError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "local daemon decision rejected: {self:?}")
    }
}

impl std::error::Error for LocalDaemonError {}

fn validate_record(record: &WorkerRegistryRecord) -> Result<(), LocalDaemonError> {
    validate_label(record.root_id.as_str())?;
    validate_label(record.worker_id.as_str())?;
    validate_hash(record.process_identity.as_str())?;
    validate_hash(record.endpoint_identity.as_str())?;
    validate_hash(record.session_head_identity.as_str())?;
    validate_hash(record.receipt_identity.as_str())?;
    if record.generation < MIN_GENERATION || record.process_id == 0 {
        return Err(LocalDaemonError::InvalidIdentity);
    }
    let mut clients = BTreeSet::new();
    for attachment in &record.attachments {
        validate_label(attachment.client_id.as_str())?;
        if !clients.insert(attachment.client_id.as_str()) {
            return Err(LocalDaemonError::DuplicateWorker);
        }
    }
    Ok(())
}

fn validate_label(value: &str) -> Result<(), LocalDaemonError> {
    if value.is_empty()
        || value.len() > MAX_LABEL_BYTES
        || value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control())
    {
        return Err(LocalDaemonError::InvalidIdentity);
    }
    Ok(())
}

fn validate_hash(value: &str) -> Result<(), LocalDaemonError> {
    if value.len() != animus_adapter_daemon::BLAKE3_HEX_CHARS
        || !value.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(LocalDaemonError::InvalidIdentity);
    }
    Ok(())
}
