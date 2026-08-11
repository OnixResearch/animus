use std::io::Read;

const RESOURCE_IDENTITY_DOMAIN: &[u8] = b"animus.workspace-edit.resource.v1";
const FILE_METADATA_DOMAIN: &[u8] = b"animus.workspace-edit.file-metadata.v1";
const BOUNDED_READ_SENTINEL_BYTES: u64 = 1;
const MAX_BOUNDED_READ_BYTES: u64 =
    crate::model::text::MAX_SOURCE_BYTES_U64.saturating_add(BOUNDED_READ_SENTINEL_BYTES);
const UNIX_PERMISSION_MASK: u32 = 0o777;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct FileFacts {
    pub(crate) mode: Option<u32>,
    pub(crate) owner: Option<u32>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct FileRead {
    pub(crate) bytes: Vec<u8>,
    pub(crate) metadata_identity: String,
    pub(crate) facts: FileFacts,
}

fn frame_part(hasher: &mut blake3::Hasher, value: &[u8]) {
    hasher.update(&(value.len() as u64).to_le_bytes());
    hasher.update(value);
}

fn hash_parts(domain: &[u8], parts: &[&[u8]]) -> String {
    let mut hasher = blake3::Hasher::new();
    frame_part(&mut hasher, domain);
    for part in parts {
        frame_part(&mut hasher, part);
    }
    hasher.finalize().to_hex().to_string()
}

/// Compute the logical resource identity from one root identity and admitted path.
///
/// # Errors
///
/// Rejects an unsafe, non-UTF-8, absolute, empty, or parent-escaping path.
// r[impl agent_harness_runtime.hashed_edit_host.boundary]
pub fn logical_resource_identity(
    root_identity: &str,
    logical_resource: &std::path::Path,
) -> Result<String, crate::concrete::HostError> {
    let admitted = cap_root_core::RootRelativePath::new(
        logical_resource,
        cap_root_core::AdmissionMode::NonEmptyRelative,
    )
    .map_err(|_| crate::concrete::HostError::PathDenied)?;
    let normalized = admitted
        .as_path()
        .to_str()
        .ok_or(crate::concrete::HostError::PathDenied)?;
    Ok(hash_parts(
        RESOURCE_IDENTITY_DOMAIN,
        &[root_identity.as_bytes(), normalized.as_bytes()],
    ))
}

#[cfg(unix)]
fn unix_file_facts(metadata: &cap_std::fs::Metadata) -> FileFacts {
    use cap_std::fs::MetadataExt;

    FileFacts {
        mode: Some(metadata.mode() & UNIX_PERMISSION_MASK),
        owner: Some(metadata.uid()),
    }
}

#[cfg(not(unix))]
fn unix_file_facts(_metadata: &cap_std::fs::Metadata) -> FileFacts {
    FileFacts {
        mode: None,
        owner: None,
    }
}

#[cfg(unix)]
fn frame_unix_metadata(hasher: &mut blake3::Hasher, metadata: &cap_std::fs::Metadata) {
    use cap_std::fs::MetadataExt;

    frame_part(hasher, &metadata.dev().to_le_bytes());
    frame_part(hasher, &metadata.ino().to_le_bytes());
    frame_part(hasher, &metadata.nlink().to_le_bytes());
    frame_part(hasher, &metadata.gid().to_le_bytes());
    frame_part(hasher, &metadata.mtime().to_le_bytes());
    frame_part(hasher, &metadata.mtime_nsec().to_le_bytes());
    frame_part(hasher, &metadata.ctime().to_le_bytes());
    frame_part(hasher, &metadata.ctime_nsec().to_le_bytes());
}

fn metadata_identity(metadata: &cap_std::fs::Metadata, facts: FileFacts) -> String {
    let mut hasher = blake3::Hasher::new();
    frame_part(&mut hasher, FILE_METADATA_DOMAIN);
    frame_part(&mut hasher, b"regular-file");
    frame_part(&mut hasher, &metadata.len().to_le_bytes());
    frame_part(&mut hasher, &facts.mode.unwrap_or_default().to_le_bytes());
    frame_part(&mut hasher, &facts.owner.unwrap_or_default().to_le_bytes());
    #[cfg(unix)]
    frame_unix_metadata(&mut hasher, metadata);
    let identity = hasher.finalize().to_hex().to_string();
    debug_assert!(!identity.is_empty());
    identity
}

pub(crate) fn enforce_file_policy(
    request: &crate::concrete::HostRequest<'_>,
    facts: FileFacts,
) -> Result<(), crate::concrete::HostError> {
    if request.expected_mode.is_some() && request.expected_mode != facts.mode {
        return Err(crate::concrete::HostError::PermissionDenied);
    }
    if request.expected_owner.is_some() && request.expected_owner != facts.owner {
        return Err(crate::concrete::HostError::OwnershipDenied);
    }
    if request.final_mode & !UNIX_PERMISSION_MASK != 0 {
        return Err(crate::concrete::HostError::PermissionDenied);
    }
    Ok(())
}

fn observed_file(
    root: &cap_std::fs::Dir,
    admitted: &cap_root_core::RootRelativePath,
) -> Result<cap_std::fs::Metadata, crate::concrete::HostError> {
    let metadata = root
        .symlink_metadata(admitted.as_path())
        .map_err(|_| crate::concrete::HostError::ReadFailed)?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(crate::concrete::HostError::UnsupportedFileKind);
    }
    Ok(metadata)
}

pub(crate) fn read_resource(
    root: &cap_std::fs::Dir,
    admitted: &cap_root_core::RootRelativePath,
) -> Result<FileRead, crate::concrete::HostError> {
    let before = observed_file(root, admitted)?;
    let before_facts = unix_file_facts(&before);
    if before.len() > crate::model::text::MAX_SOURCE_BYTES_U64 {
        return Err(crate::model::HostIssue::SourceTooLarge.into());
    }
    let file = root
        .open(admitted.as_path())
        .map_err(|_| crate::concrete::HostError::ReadFailed)?;
    let opened_before = file
        .metadata()
        .map_err(|_| crate::concrete::HostError::ReadFailed)?;
    if !opened_before.is_file() {
        return Err(crate::concrete::HostError::UnsupportedFileKind);
    }
    let opened_before_facts = unix_file_facts(&opened_before);
    let mut bytes = Vec::with_capacity(crate::model::MAX_SOURCE_BYTES);
    (&file)
        .take(MAX_BOUNDED_READ_BYTES)
        .read_to_end(&mut bytes)
        .map_err(|_| crate::concrete::HostError::ReadFailed)?;
    if bytes.len() > crate::model::MAX_SOURCE_BYTES {
        return Err(crate::model::HostIssue::SourceTooLarge.into());
    }
    let opened_after = file
        .metadata()
        .map_err(|_| crate::concrete::HostError::ReadFailed)?;
    if !opened_after.is_file() {
        return Err(crate::concrete::HostError::UnsupportedFileKind);
    }
    let after = observed_file(root, admitted)?;
    let opened_after_facts = unix_file_facts(&opened_after);
    let after_facts = unix_file_facts(&after);
    let identities = [
        metadata_identity(&before, before_facts),
        metadata_identity(&opened_before, opened_before_facts),
        metadata_identity(&opened_after, opened_after_facts),
        metadata_identity(&after, after_facts),
    ];
    if identities.windows(2).any(|pair| pair[0] != pair[1]) {
        return Err(crate::concrete::HostError::Core(
            crate::model::HostIssue::Stale(crate::model::StaleField::FileMetadata),
        ));
    }
    debug_assert!(bytes.len() <= crate::model::MAX_SOURCE_BYTES);
    Ok(FileRead {
        bytes,
        metadata_identity: identities[3].clone(),
        facts: after_facts,
    })
}

pub(crate) fn validate_initial(
    request: &crate::concrete::HostRequest<'_>,
    observation: &crate::concrete::HostObservation,
) -> Result<(), crate::concrete::HostError> {
    if observation.revision_identity != request.revision_identity {
        return Err(crate::model::HostIssue::Stale(crate::model::StaleField::Revision).into());
    }
    if observation.metadata_policy_identity != request.metadata_policy_identity {
        return Err(
            crate::model::HostIssue::Stale(crate::model::StaleField::MetadataPolicy).into(),
        );
    }
    if observation.writer_generation != request.writer_generation {
        return Err(
            crate::model::HostIssue::Stale(crate::model::StaleField::WriterGeneration).into(),
        );
    }
    Ok(())
}

pub(crate) fn validate_lease(
    request: &crate::concrete::HostRequest<'_>,
    lease: &crate::concrete::WriterLease,
) -> Result<(), crate::concrete::HostError> {
    if lease.resource_identity() != request.resource_identity {
        return Err(crate::concrete::HostError::Service(
            crate::concrete::HostServiceIssue::Stale,
        ));
    }
    if lease.revision_identity() != request.revision_identity {
        return Err(crate::concrete::HostError::Service(
            crate::concrete::HostServiceIssue::Stale,
        ));
    }
    if lease.writer_generation() != request.writer_generation {
        return Err(crate::concrete::HostError::Service(
            crate::concrete::HostServiceIssue::Stale,
        ));
    }
    Ok(())
}
