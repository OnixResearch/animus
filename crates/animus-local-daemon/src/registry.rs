//! Capability-relative durable worker registry.

use cap_std::fs::{OpenOptionsExt, PermissionsExt};
use std::io::{Read, Seek, SeekFrom, Write};

const PRIVATE_FILE_MODE: u32 = 0o600;
const TEMP_SUFFIX: &str = ".staged";
const MAX_REGISTRY_BYTES: usize = 1_048_576;

/// Explicit capability-relative registry store.
pub struct RegistryStore<'root> {
    root: &'root cap_std::fs::Dir,
    name: String,
    staged_name: String,
    max_workers: u16,
}

impl<'root> RegistryStore<'root> {
    /// Creates a store over one explicit capability root and local name.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] for unsafe names or zero bounds.
    pub fn new(
        root: &'root cap_std::fs::Dir,
        name: &str,
        max_workers: u16,
    ) -> Result<Self, RegistryError> {
        validate_local_name(name)?;
        if max_workers == 0 {
            return Err(RegistryError::InvalidInput);
        }
        Ok(Self {
            root,
            name: String::from(name),
            staged_name: format!("{name}{TEMP_SUFFIX}"),
            max_workers,
        })
    }

    /// Reads and validates the current registry. Absence returns an empty registry.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] for links, non-files, bounds, malformed JSON, or invalid state.
    pub fn load(&self) -> Result<crate::WorkerRegistry, RegistryError> {
        let mut options = cap_std::fs::OpenOptions::new();
        options.read(true);
        let file = match self.root.open_with(self.name.as_str(), &options) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return crate::WorkerRegistry::new(Vec::new(), self.max_workers)
                    .map_err(|_error| RegistryError::InvalidState);
            }
            Err(_error) => return Err(RegistryError::Io),
        };
        validate_file(&file)?;
        let mut bytes = Vec::new();
        file.take(u64::try_from(MAX_REGISTRY_BYTES + 1).map_err(|_error| RegistryError::Bound)?)
            .read_to_end(&mut bytes)
            .map_err(|_error| RegistryError::Io)?;
        if bytes.len() > MAX_REGISTRY_BYTES {
            return Err(RegistryError::Bound);
        }
        let decoded: crate::WorkerRegistry =
            serde_json::from_slice(bytes.as_slice()).map_err(|_error| RegistryError::Codec)?;
        if decoded.schema != "animus.local-daemon-registry.v1" {
            return Err(RegistryError::InvalidState);
        }
        crate::WorkerRegistry::new(decoded.workers, self.max_workers)
            .map_err(|_error| RegistryError::InvalidState)
    }

    /// Publishes a full validated registry through a synchronized rename.
    ///
    /// # Errors
    ///
    /// Returns [`RegistryError`] for invalid state, bounds, publication, or I/O failure.
    pub fn publish(&self, registry: &crate::WorkerRegistry) -> Result<String, RegistryError> {
        let canonical = crate::WorkerRegistry::new(registry.workers.clone(), self.max_workers)
            .map_err(|_error| RegistryError::InvalidState)?;
        let bytes = serde_json::to_vec(&canonical).map_err(|_error| RegistryError::Codec)?;
        if bytes.len() > MAX_REGISTRY_BYTES {
            return Err(RegistryError::Bound);
        }
        let _remove = self.root.remove_file(self.staged_name.as_str());
        let mut options = cap_std::fs::OpenOptions::new();
        options.write(true).create_new(true).mode(PRIVATE_FILE_MODE);
        let mut file = self
            .root
            .open_with(self.staged_name.as_str(), &options)
            .map_err(|_error| RegistryError::Io)?;
        validate_file(&file)?;
        file.write_all(bytes.as_slice())
            .and_then(|()| file.sync_all())
            .map_err(|_error| RegistryError::Io)?;
        file.seek(SeekFrom::Start(0))
            .map_err(|_error| RegistryError::Io)?;
        drop(file);
        self.root
            .rename(self.staged_name.as_str(), self.root, self.name.as_str())
            .map_err(|_error| RegistryError::Io)?;
        Ok(blake3::hash(bytes.as_slice()).to_hex().to_string())
    }
}

/// Durable registry failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RegistryError {
    /// Name or bound is invalid.
    InvalidInput,
    /// Registry exceeds its byte bound.
    Bound,
    /// Registry JSON is malformed.
    Codec,
    /// Registry state violates ownership or identity rules.
    InvalidState,
    /// Registry I/O or publication failed.
    Io,
}

impl core::fmt::Display for RegistryError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "local registry rejected: {self:?}")
    }
}

impl std::error::Error for RegistryError {}

fn validate_local_name(value: &str) -> Result<(), RegistryError> {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value.contains('/')
        || value.contains('\\')
        || value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control())
    {
        return Err(RegistryError::InvalidInput);
    }
    Ok(())
}

fn validate_file(file: &cap_std::fs::File) -> Result<(), RegistryError> {
    let metadata = file.metadata().map_err(|_error| RegistryError::Io)?;
    if !metadata.file_type().is_file()
        || metadata.file_type().is_symlink()
        || metadata.permissions().mode() & 0o777 != PRIVATE_FILE_MODE
    {
        return Err(RegistryError::InvalidState);
    }
    Ok(())
}
