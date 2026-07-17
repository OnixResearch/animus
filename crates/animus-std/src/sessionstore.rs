//! Capability-scoped local durable session-store host.

use cap_std::fs::{OpenOptionsExt, PermissionsExt};
use std::io::{Read, Seek, Write};

type CapabilityDir = cap_std::fs::Dir;
type CapabilityFile = cap_std::fs::File;
type DurableSessionRecoveryReport =
    animus_runtime::durable_session_store::DurableSessionRecoveryReport;
type DurableSessionStoreDescriptor =
    animus_runtime::durable_session_store::DurableSessionStoreDescriptor;
type DurableSessionStorePolicy = animus_runtime::durable_session_store::DurableSessionStorePolicy;
type DurableSessionStorePolicyInput =
    animus_runtime::durable_session_store::DurableSessionStorePolicyInput;
type DurableSessionTransaction = animus_runtime::durable_session_store::DurableSessionTransaction;

const GENERATED_POLICY_FILE: &str = "config/generated/durable-session-store-policy.json";
const PRIVATE_FILE_MODE: u32 = 0o600;
const MAX_LOCAL_NAME_BYTES: usize = 128;
const READ_BUFFER_BYTES: usize = 8_192;

#[derive(serde::Deserialize)]
#[serde(deny_unknown_fields)]
struct DurableSessionStorePolicyJson {
    schema: String,
    policy_id: String,
    max_frame_payload_bytes: u32,
    max_transaction_frames: u32,
    max_file_bytes: u64,
    file_mode: u32,
    durability: String,
    locking: String,
    private_data: String,
    encryption_postures: Vec<String>,
    tail_recovery: String,
    committed_corruption: String,
    receipt_fields: Vec<String>,
    policy_hash: String,
}

/// Generated durable-store policy JSON failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DurableSessionStoreJsonErrorKind {
    /// Input was not valid JSON.
    InvalidJson,
    /// Input had unknown fields or invalid types.
    InvalidShape,
}

/// Generated durable-store policy loading error.
#[derive(Debug)]
pub enum DurableSessionStoreConfigError {
    /// Explicit policy file read failed.
    Read(std::io::Error),
    /// JSON decoding failed with a bounded class.
    Json {
        /// Bounded JSON issue.
        kind: DurableSessionStoreJsonErrorKind,
    },
    /// Runtime policy validation failed.
    Policy(animus_runtime::durable_session_store::DurableSessionStoreError),
}

impl core::fmt::Display for DurableSessionStoreConfigError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Read(_error) => write!(formatter, "failed to read {GENERATED_POLICY_FILE}"),
            Self::Json { kind } => write!(formatter, "durable store JSON rejected: {kind:?}"),
            Self::Policy(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for DurableSessionStoreConfigError {}

/// Parses and revalidates generated durable-store policy JSON.
///
/// # Errors
///
/// Returns [`DurableSessionStoreConfigError`] for malformed JSON, unknown
/// fields, or failed runtime policy validation.
pub fn parse_durable_session_store_policy_json(
    text: &str,
) -> Result<DurableSessionStorePolicy, DurableSessionStoreConfigError> {
    let decoded: DurableSessionStorePolicyJson = serde_json::from_str(text).map_err(|error| {
        let kind = if error.is_syntax() || error.is_eof() {
            DurableSessionStoreJsonErrorKind::InvalidJson
        } else {
            DurableSessionStoreJsonErrorKind::InvalidShape
        };
        DurableSessionStoreConfigError::Json { kind }
    })?;
    let input = DurableSessionStorePolicyInput {
        schema: decoded.schema,
        policy_id: decoded.policy_id,
        max_frame_payload_bytes: decoded.max_frame_payload_bytes,
        max_transaction_frames: decoded.max_transaction_frames,
        max_file_bytes: decoded.max_file_bytes,
        file_mode: decoded.file_mode,
        durability: decoded.durability,
        locking: decoded.locking,
        private_data: decoded.private_data,
        encryption_postures: decoded.encryption_postures,
        tail_recovery: decoded.tail_recovery,
        committed_corruption: decoded.committed_corruption,
        receipt_fields: decoded.receipt_fields,
        policy_hash: decoded.policy_hash,
    };
    animus_runtime::durable_session_store::validate_durable_session_store_policy(&input)
        .map_err(DurableSessionStoreConfigError::Policy)
}

/// Loads generated durable-store policy from one explicit caller path.
///
/// # Errors
///
/// Returns [`DurableSessionStoreConfigError`] for read, decode, or validation
/// failures.
pub fn load_durable_session_store_policy(
    path: &std::path::Path,
) -> Result<DurableSessionStorePolicy, DurableSessionStoreConfigError> {
    let text = std::fs::read_to_string(path).map_err(DurableSessionStoreConfigError::Read)?;
    parse_durable_session_store_policy_json(text.as_str())
}

/// Capability-local journal and lock names.
pub struct DurableSessionStoreLocation<'a> {
    /// Caller-supplied capability directory.
    pub root: &'a CapabilityDir,
    /// Safe relative journal file name.
    pub journal_name: &'a str,
    /// Safe relative writer lease file name.
    pub lock_name: &'a str,
}

/// Explicit local-store open request.
pub struct DurableSessionStoreOpenRequest<'a> {
    /// Generated policy.
    pub policy: &'a DurableSessionStorePolicy,
    /// Validated store descriptor.
    pub descriptor: &'a DurableSessionStoreDescriptor,
    /// Capability-local location.
    pub location: DurableSessionStoreLocation<'a>,
    /// Safe caller-owned lease identity.
    pub lease_id: &'a str,
}

/// Host synchronization boundary for deterministic failure injection.
pub trait DurableSessionSynchronizer {
    /// Synchronizes the supplied file to the host's declared durable boundary.
    ///
    /// # Errors
    ///
    /// Returns a host-private I/O error when synchronization fails.
    fn synchronize(&mut self, file: &CapabilityFile) -> Result<(), std::io::Error>;
}

/// Concrete file synchronization service.
#[derive(Clone, Copy, Debug, Default)]
pub struct FileSynchronizer;

impl DurableSessionSynchronizer for FileSynchronizer {
    fn synchronize(&mut self, file: &CapabilityFile) -> Result<(), std::io::Error> {
        file.sync_all()
    }
}

/// Bounded local host failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DurableSessionHostIssue {
    /// Request or capability-local name was invalid.
    InvalidAdmission,
    /// Required external encryption is unsupported by this local backend.
    EncryptionUnavailable,
    /// Another writer lease is active.
    LockConflict,
    /// The requested lease identity is already stranded.
    StaleLease,
    /// Capability-local open, read, append, permission, or close failed.
    IoFailure,
    /// Data synchronization failed.
    SynchronizationFailed,
    /// Pure framing or recovery validation failed.
    Validation,
    /// Transaction did not extend the opened committed head.
    StaleHead,
    /// Journal byte budget would be exceeded.
    BoundExceeded,
}

/// Bounded local host error.
#[derive(Debug)]
pub struct DurableSessionHostError {
    issue: DurableSessionHostIssue,
}

impl DurableSessionHostError {
    const fn new(issue: DurableSessionHostIssue) -> Self {
        Self { issue }
    }

    /// Returns the bounded issue class.
    #[must_use]
    pub const fn issue(&self) -> DurableSessionHostIssue {
        self.issue
    }
}

impl core::fmt::Display for DurableSessionHostError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "durable session host failed: {:?}", self.issue)
    }
}

impl std::error::Error for DurableSessionHostError {}

/// Successful durable append outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DurableSessionCommitOutcome {
    /// Prior committed head.
    pub prior_head_hash: String,
    /// New committed head.
    pub new_head_hash: String,
    /// Committed private frame count.
    pub committed_frame_count: u32,
    /// Exact bytes appended, including commit framing.
    pub appended_bytes: u64,
    /// Synchronization completed.
    pub synchronized: bool,
}

/// Open capability-scoped local store with one exclusive writer lease.
///
/// # Errors
///
/// Returns [`DurableSessionHostError`] before journal mutation for invalid
/// admission, encryption requirements, conflicting/stale leases, permission
/// failures, file bounds, or recovery failure.
// r[impl agent_harness_runtime.durable_session_store.boundary]
// r[impl agent_harness_runtime.durable_session_store.locking]
pub fn open_durable_session_store<'store, Synchronizer>(
    request: &DurableSessionStoreOpenRequest<'store>,
    synchronizer: Synchronizer,
) -> Result<DurableSessionLocalStore<'store, Synchronizer>, DurableSessionHostError>
where
    Synchronizer: DurableSessionSynchronizer,
{
    validate_open_request(request)?;
    let mut synchronizer = synchronizer;
    let lock_file = acquire_writer_lease(request, &mut synchronizer)?;
    let mut journal = match open_journal(request) {
        Ok(file) => file,
        Err(error) => {
            drop(lock_file);
            cleanup_writer_lease(request)?;
            return Err(error);
        }
    };
    let recovery = match read_and_recover(request.policy, request.descriptor, &mut journal) {
        Ok(report) => report,
        Err(error) => {
            drop(lock_file);
            cleanup_writer_lease(request)?;
            return Err(error);
        }
    };
    let journal_bytes = match journal.metadata() {
        Ok(metadata) => metadata.len(),
        Err(_error) => {
            drop(lock_file);
            cleanup_writer_lease(request)?;
            return Err(DurableSessionHostError::new(
                DurableSessionHostIssue::IoFailure,
            ));
        }
    };
    debug_assert!(journal_bytes <= request.policy.max_file_bytes());
    debug_assert!(!request.lease_id.is_empty());
    Ok(DurableSessionLocalStore {
        root: request.location.root,
        journal_name: String::from(request.location.journal_name),
        lock_name: String::from(request.location.lock_name),
        lease_id: String::from(request.lease_id),
        policy: request.policy,
        descriptor: request.descriptor,
        lock_file,
        journal,
        recovery,
        journal_bytes,
        synchronizer,
    })
}

/// Open capability-scoped local session store.
pub struct DurableSessionLocalStore<'a, Synchronizer>
where
    Synchronizer: DurableSessionSynchronizer,
{
    root: &'a CapabilityDir,
    journal_name: String,
    lock_name: String,
    lease_id: String,
    policy: &'a DurableSessionStorePolicy,
    descriptor: &'a DurableSessionStoreDescriptor,
    lock_file: CapabilityFile,
    journal: CapabilityFile,
    recovery: DurableSessionRecoveryReport,
    journal_bytes: u64,
    synchronizer: Synchronizer,
}

impl<Synchronizer> core::fmt::Debug for DurableSessionLocalStore<'_, Synchronizer>
where
    Synchronizer: DurableSessionSynchronizer,
{
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        formatter
            .debug_struct("DurableSessionLocalStore")
            .field("journal_name", &self.journal_name)
            .field("lock_name", &self.lock_name)
            .field("lease_id", &self.lease_id)
            .field("committed_head_hash", &self.recovery.committed_head_hash())
            .field("journal_bytes", &self.journal_bytes)
            .finish_non_exhaustive()
    }
}

impl<Synchronizer> DurableSessionLocalStore<'_, Synchronizer>
where
    Synchronizer: DurableSessionSynchronizer,
{
    /// Returns the validated committed snapshot observed at open or last append.
    #[must_use]
    pub const fn recovery(&self) -> &DurableSessionRecoveryReport {
        &self.recovery
    }

    /// Appends one complete logical group and synchronizes before success.
    ///
    /// # Errors
    ///
    /// Returns [`DurableSessionHostError`] for stale head, size, append,
    /// synchronization, or post-append validation failure.
    // r[impl agent_harness_runtime.durable_session_store.commit]
    pub fn commit(
        &mut self,
        transaction: &DurableSessionTransaction,
    ) -> Result<DurableSessionCommitOutcome, DurableSessionHostError> {
        if !matches!(
            self.recovery.status(),
            animus_runtime::durable_session_store::DurableSessionRecoveryStatus::Empty
                | animus_runtime::durable_session_store::DurableSessionRecoveryStatus::Clean
        ) {
            return Err(DurableSessionHostError::new(
                DurableSessionHostIssue::Validation,
            ));
        }
        let prior_head_hash = self.recovery.committed_head_hash();
        let prior_active_leaf_id = self.recovery.active_leaf_id();
        let transaction_parent_id = transaction
            .frames()
            .first()
            .and_then(|frame| frame.parent_id());
        if transaction.commit().prior_head_hash() != prior_head_hash
            || transaction_parent_id != prior_active_leaf_id
        {
            return Err(DurableSessionHostError::new(
                DurableSessionHostIssue::StaleHead,
            ));
        }
        let bytes = animus_runtime::durable_session_store::encode_durable_session_transaction(
            self.policy,
            transaction,
        )
        .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::Validation))?;
        let appended_bytes = u64::try_from(bytes.len()).map_err(|_error| {
            DurableSessionHostError::new(DurableSessionHostIssue::BoundExceeded)
        })?;
        let next_journal_bytes = self
            .journal_bytes
            .checked_add(appended_bytes)
            .ok_or_else(|| DurableSessionHostError::new(DurableSessionHostIssue::BoundExceeded))?;
        if next_journal_bytes > self.policy.max_file_bytes() {
            return Err(DurableSessionHostError::new(
                DurableSessionHostIssue::BoundExceeded,
            ));
        }
        self.journal
            .seek(std::io::SeekFrom::End(0))
            .and_then(|_offset| self.journal.write_all(bytes.as_slice()))
            .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::IoFailure))?;
        self.synchronizer
            .synchronize(&self.journal)
            .map_err(|_error| {
                DurableSessionHostError::new(DurableSessionHostIssue::SynchronizationFailed)
            })?;
        let recovery = read_and_recover(self.policy, self.descriptor, &mut self.journal)?;
        if recovery.committed_head_hash() != transaction.commit().commit_hash() {
            return Err(DurableSessionHostError::new(
                DurableSessionHostIssue::Validation,
            ));
        }
        let committed_frame_count =
            u32::try_from(transaction.frames().len()).map_err(|_error| {
                DurableSessionHostError::new(DurableSessionHostIssue::BoundExceeded)
            })?;
        let outcome = DurableSessionCommitOutcome {
            prior_head_hash: String::from(prior_head_hash),
            new_head_hash: String::from(recovery.committed_head_hash()),
            committed_frame_count,
            appended_bytes,
            synchronized: true,
        };
        self.recovery = recovery;
        self.journal_bytes = next_journal_bytes;
        Ok(outcome)
    }

    /// Explicitly validates and releases the writer lease.
    ///
    /// # Errors
    ///
    /// Returns [`DurableSessionHostError`] when lease content differs or lease
    /// removal fails. Dropping without close intentionally strands the lease.
    pub fn close(mut self) -> Result<(), DurableSessionHostError> {
        self.synchronizer
            .synchronize(&self.lock_file)
            .map_err(|_error| {
                DurableSessionHostError::new(DurableSessionHostIssue::SynchronizationFailed)
            })?;
        let observed = read_file_bounded(&mut self.lock_file, MAX_LOCAL_NAME_BYTES)?;
        if observed.as_slice() != self.lease_id.as_bytes() {
            return Err(DurableSessionHostError::new(
                DurableSessionHostIssue::StaleLease,
            ));
        }
        drop(self.lock_file);
        self.root
            .remove_file(self.lock_name.as_str())
            .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::IoFailure))
    }
}

/// Reads one committed snapshot without acquiring mutation authority.
///
/// # Errors
///
/// Returns [`DurableSessionHostError`] for unsafe local names, read bounds, I/O,
/// or framing/recovery failure.
pub fn read_durable_session_snapshot(
    root: &CapabilityDir,
    journal_name: &str,
    policy: &DurableSessionStorePolicy,
    descriptor: &DurableSessionStoreDescriptor,
) -> Result<DurableSessionRecoveryReport, DurableSessionHostError> {
    validate_local_name(journal_name)?;
    let mut file = root
        .open(journal_name)
        .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::IoFailure))?;
    read_and_recover(policy, descriptor, &mut file)
}

fn validate_open_request(
    request: &DurableSessionStoreOpenRequest<'_>,
) -> Result<(), DurableSessionHostError> {
    animus_runtime::durable_session_store::validate_durable_session_store_descriptor(
        request.policy,
        request.descriptor,
    )
    .map_err(|error| {
        let issue = if error.issue()
            == animus_runtime::durable_session_store::DurableSessionStoreIssue::EncryptionUnavailable
        {
            DurableSessionHostIssue::EncryptionUnavailable
        } else {
            DurableSessionHostIssue::InvalidAdmission
        };
        DurableSessionHostError::new(issue)
    })?;
    validate_local_name(request.location.journal_name)?;
    validate_local_name(request.location.lock_name)?;
    validate_local_name(request.lease_id)?;
    if request.location.journal_name == request.location.lock_name {
        return Err(DurableSessionHostError::new(
            DurableSessionHostIssue::InvalidAdmission,
        ));
    }
    if request.descriptor.encryption
        == animus_runtime::durable_session_store::DurableSessionEncryptionPosture::ExternalRequired
    {
        return Err(DurableSessionHostError::new(
            DurableSessionHostIssue::EncryptionUnavailable,
        ));
    }
    Ok(())
}

fn acquire_writer_lease(
    request: &DurableSessionStoreOpenRequest<'_>,
    synchronizer: &mut impl DurableSessionSynchronizer,
) -> Result<CapabilityFile, DurableSessionHostError> {
    let mut options = cap_std::fs::OpenOptions::new();
    options
        .read(true)
        .write(true)
        .create_new(true)
        .mode(PRIVATE_FILE_MODE);
    match request
        .location
        .root
        .open_with(request.location.lock_name, &options)
    {
        Ok(mut file) => {
            if file.write_all(request.lease_id.as_bytes()).is_err() {
                drop(file);
                cleanup_writer_lease(request)?;
                return Err(DurableSessionHostError::new(
                    DurableSessionHostIssue::IoFailure,
                ));
            }
            if synchronizer.synchronize(&file).is_err() {
                drop(file);
                cleanup_writer_lease(request)?;
                return Err(DurableSessionHostError::new(
                    DurableSessionHostIssue::SynchronizationFailed,
                ));
            }
            Ok(file)
        }
        Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
            let issue = classify_existing_lease(request)?;
            Err(DurableSessionHostError::new(issue))
        }
        Err(_error) => Err(DurableSessionHostError::new(
            DurableSessionHostIssue::IoFailure,
        )),
    }
}

fn cleanup_writer_lease(
    request: &DurableSessionStoreOpenRequest<'_>,
) -> Result<(), DurableSessionHostError> {
    request
        .location
        .root
        .remove_file(request.location.lock_name)
        .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::IoFailure))
}

fn classify_existing_lease(
    request: &DurableSessionStoreOpenRequest<'_>,
) -> Result<DurableSessionHostIssue, DurableSessionHostError> {
    let mut existing = request
        .location
        .root
        .open(request.location.lock_name)
        .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::IoFailure))?;
    let observed = read_file_bounded(&mut existing, MAX_LOCAL_NAME_BYTES)?;
    if observed.as_slice() == request.lease_id.as_bytes() {
        Ok(DurableSessionHostIssue::StaleLease)
    } else {
        Ok(DurableSessionHostIssue::LockConflict)
    }
}

fn open_journal(
    request: &DurableSessionStoreOpenRequest<'_>,
) -> Result<CapabilityFile, DurableSessionHostError> {
    let mut options = cap_std::fs::OpenOptions::new();
    options
        .read(true)
        .append(true)
        .create(true)
        .mode(PRIVATE_FILE_MODE);
    let file = request
        .location
        .root
        .open_with(request.location.journal_name, &options)
        .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::IoFailure))?;
    file.set_permissions(cap_std::fs::Permissions::from_mode(PRIVATE_FILE_MODE))
        .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::IoFailure))?;
    Ok(file)
}

fn read_and_recover(
    policy: &DurableSessionStorePolicy,
    descriptor: &DurableSessionStoreDescriptor,
    file: &mut CapabilityFile,
) -> Result<DurableSessionRecoveryReport, DurableSessionHostError> {
    let max_bytes = usize::try_from(policy.max_file_bytes())
        .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::BoundExceeded))?;
    let bytes = read_file_bounded(file, max_bytes)?;
    animus_runtime::durable_session_store::recover_durable_session_log(
        policy,
        descriptor,
        bytes.as_slice(),
    )
    .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::Validation))
}

fn read_file_bounded(
    file: &mut CapabilityFile,
    max_bytes: usize,
) -> Result<Vec<u8>, DurableSessionHostError> {
    file.seek(std::io::SeekFrom::Start(0))
        .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::IoFailure))?;
    let read_bound = u64::try_from(max_bytes)
        .ok()
        .and_then(|value| value.checked_add(1))
        .ok_or_else(|| DurableSessionHostError::new(DurableSessionHostIssue::BoundExceeded))?;
    let mut bytes = Vec::with_capacity(max_bytes.min(READ_BUFFER_BYTES));
    file.take(read_bound)
        .read_to_end(&mut bytes)
        .map_err(|_error| DurableSessionHostError::new(DurableSessionHostIssue::IoFailure))?;
    if bytes.len() > max_bytes {
        return Err(DurableSessionHostError::new(
            DurableSessionHostIssue::BoundExceeded,
        ));
    }
    Ok(bytes)
}

fn validate_local_name(value: &str) -> Result<(), DurableSessionHostError> {
    let has_safe_length = !value.is_empty() && value.len() <= MAX_LOCAL_NAME_BYTES;
    let has_safe_chars = value
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || matches!(character, '.' | '_' | '-'));
    if !has_safe_length || !has_safe_chars {
        return Err(DurableSessionHostError::new(
            DurableSessionHostIssue::InvalidAdmission,
        ));
    }
    Ok(())
}

#[cfg(test)]
#[path = "sessionstore/tests.rs"]
mod tests;
