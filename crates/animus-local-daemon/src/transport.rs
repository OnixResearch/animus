//! Explicit bounded Unix stream transport.

use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::io::{Read, Write};
use std::os::fd::AsFd;
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::{Path, PathBuf};
use std::time::Duration;

const SOCKET_MODE: u32 = 0o600;
const FRAME_PREFIX_BYTES: usize = core::mem::size_of::<u32>();
const BLAKE3_HEX_CHARS: usize = 64;

/// Explicit capability-owned Unix endpoint.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalEndpoint {
    path: PathBuf,
    identity: String,
}

impl LocalEndpoint {
    /// Admits one explicit endpoint below a caller-opened capability root path.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] for an invalid local name.
    pub fn new(root: &Path, local_name: &str) -> Result<Self, TransportError> {
        validate_local_name(local_name)?;
        let path = root.join(local_name);
        let material = path.to_str().ok_or(TransportError::InvalidEndpoint)?;
        let identity = blake3::hash(material.as_bytes()).to_hex().to_string();
        Ok(Self { path, identity })
    }

    /// Returns the exact endpoint path.
    #[must_use]
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    /// Returns the endpoint BLAKE3 identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }
}

/// Authenticated local peer facts from the kernel.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[allow(
    clippy::struct_field_names,
    reason = "kernel credential field names are explicit"
)]
pub struct LocalPeerFacts {
    /// Peer process identifier.
    pub process_id: u32,
    /// Peer user identifier.
    pub user_id: u32,
    /// Peer group identifier.
    pub group_id: u32,
}

/// Versioned bounded transport frame.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct TransportFrame<Payload> {
    /// Protocol version.
    pub version: u16,
    /// Exact endpoint identity.
    pub endpoint_identity: String,
    /// Worker generation.
    pub generation: u64,
    /// Correlation identity.
    pub correlation_id: String,
    /// Typed payload.
    pub payload: Payload,
}

/// Bound Unix listener.
pub struct LocalListener {
    endpoint: LocalEndpoint,
    listener: UnixListener,
}

impl LocalListener {
    /// Binds one explicit endpoint. Existing paths fail closed.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] for bind, metadata, or permission failure.
    pub fn bind(endpoint: LocalEndpoint) -> Result<Self, TransportError> {
        if endpoint.path.exists() {
            return Err(TransportError::EndpointExists);
        }
        let listener = UnixListener::bind(endpoint.path()).map_err(|_error| TransportError::Io)?;
        std::fs::set_permissions(
            endpoint.path(),
            std::fs::Permissions::from_mode(SOCKET_MODE),
        )
        .map_err(|_error| TransportError::Io)?;
        validate_socket_metadata(endpoint.path())?;
        Ok(Self { endpoint, listener })
    }

    /// Accepts one stream and returns kernel peer facts.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] when accept or peer authentication fails.
    pub fn accept(&self) -> Result<(LocalClient, LocalPeerFacts), TransportError> {
        let (stream, _address) = self
            .listener
            .accept()
            .map_err(|_error| TransportError::Io)?;
        let credentials = nix::sys::socket::getsockopt(
            &stream.as_fd(),
            nix::sys::socket::sockopt::PeerCredentials,
        )
        .map_err(|_error| TransportError::Peer)?;
        let process_id = credentials.pid();
        if process_id <= 0 {
            return Err(TransportError::Peer);
        }
        let facts = LocalPeerFacts {
            process_id: u32::try_from(process_id).map_err(|_error| TransportError::Peer)?,
            user_id: credentials.uid(),
            group_id: credentials.gid(),
        };
        Ok((LocalClient { stream }, facts))
    }

    /// Returns the bound endpoint.
    #[must_use]
    pub const fn endpoint(&self) -> &LocalEndpoint {
        &self.endpoint
    }

    /// Removes the owned endpoint after the listener is consumed.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] if the endpoint changed type or removal fails.
    pub fn close(self) -> Result<(), TransportError> {
        drop(self.listener);
        validate_socket_metadata(self.endpoint.path())?;
        std::fs::remove_file(self.endpoint.path()).map_err(|_error| TransportError::Io)
    }
}

/// Connected bounded local client.
pub struct LocalClient {
    stream: UnixStream,
}

impl LocalClient {
    /// Connects to one explicit endpoint with no discovery or fallback.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] for endpoint or connection failure.
    pub fn connect(endpoint: &LocalEndpoint) -> Result<Self, TransportError> {
        validate_socket_metadata(endpoint.path())?;
        let stream = UnixStream::connect(endpoint.path()).map_err(|_error| TransportError::Io)?;
        Ok(Self { stream })
    }

    /// Applies exact read and write deadlines.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] for invalid or rejected deadlines.
    pub fn set_deadline(&self, deadline: Duration) -> Result<(), TransportError> {
        if deadline.is_zero() {
            return Err(TransportError::InvalidDeadline);
        }
        self.stream
            .set_read_timeout(Some(deadline))
            .map_err(|_error| TransportError::Io)?;
        self.stream
            .set_write_timeout(Some(deadline))
            .map_err(|_error| TransportError::Io)
    }

    /// Writes one length-prefixed JSON frame under a finite bound.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] for serialization, bounds, or I/O failure.
    pub fn write_frame<Payload: Serialize>(
        &mut self,
        frame: &TransportFrame<Payload>,
        max_frame_bytes: u32,
    ) -> Result<(), TransportError> {
        validate_frame_header(frame)?;
        let payload = serde_json::to_vec(frame).map_err(|_error| TransportError::Codec)?;
        let payload_len =
            u32::try_from(payload.len()).map_err(|_error| TransportError::FrameTooLarge)?;
        if payload_len == 0 || payload_len > max_frame_bytes {
            return Err(TransportError::FrameTooLarge);
        }
        self.stream
            .write_all(payload_len.to_be_bytes().as_slice())
            .and_then(|()| self.stream.write_all(payload.as_slice()))
            .and_then(|()| self.stream.flush())
            .map_err(|_error| TransportError::Io)
    }

    /// Reads one length-prefixed JSON frame under a finite bound.
    ///
    /// # Errors
    ///
    /// Returns [`TransportError`] for malformed prefixes, bounds, JSON, or I/O failure.
    pub fn read_frame<Payload: DeserializeOwned>(
        &mut self,
        max_frame_bytes: u32,
    ) -> Result<TransportFrame<Payload>, TransportError> {
        let mut prefix = [0_u8; FRAME_PREFIX_BYTES];
        self.stream
            .read_exact(&mut prefix)
            .map_err(|_error| TransportError::Io)?;
        let payload_len = u32::from_be_bytes(prefix);
        if payload_len == 0 || payload_len > max_frame_bytes {
            return Err(TransportError::FrameTooLarge);
        }
        let payload_len =
            usize::try_from(payload_len).map_err(|_error| TransportError::FrameTooLarge)?;
        let mut payload = vec![0_u8; payload_len];
        self.stream
            .read_exact(payload.as_mut_slice())
            .map_err(|_error| TransportError::Io)?;
        let frame: TransportFrame<Payload> =
            serde_json::from_slice(payload.as_slice()).map_err(|_error| TransportError::Codec)?;
        validate_frame_header(&frame)?;
        Ok(frame)
    }
}

/// Bounded local transport failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportError {
    /// Endpoint name or metadata is invalid.
    InvalidEndpoint,
    /// Endpoint already exists.
    EndpointExists,
    /// Endpoint peer identity was unavailable.
    Peer,
    /// Frame is empty or exceeds its configured bound.
    FrameTooLarge,
    /// Frame JSON or header is malformed.
    Codec,
    /// Deadline is zero.
    InvalidDeadline,
    /// Local I/O failed.
    Io,
}

impl core::fmt::Display for TransportError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(formatter, "local transport rejected: {self:?}")
    }
}

impl std::error::Error for TransportError {}

fn validate_local_name(value: &str) -> Result<(), TransportError> {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value.contains('/')
        || value.contains('\\')
        || value
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control())
    {
        return Err(TransportError::InvalidEndpoint);
    }
    Ok(())
}

fn validate_socket_metadata(path: &Path) -> Result<(), TransportError> {
    let metadata = std::fs::symlink_metadata(path).map_err(|_error| TransportError::Io)?;
    if !metadata.file_type().is_socket()
        || metadata.file_type().is_symlink()
        || metadata.mode() & 0o777 != SOCKET_MODE
    {
        return Err(TransportError::InvalidEndpoint);
    }
    Ok(())
}

fn validate_frame_header<Payload>(frame: &TransportFrame<Payload>) -> Result<(), TransportError> {
    if frame.version != 1
        || frame.generation == 0
        || frame.endpoint_identity.len() != BLAKE3_HEX_CHARS
        || !frame
            .endpoint_identity
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
        || frame.correlation_id.is_empty()
        || frame
            .correlation_id
            .bytes()
            .any(|byte| byte == 0 || byte.is_ascii_control())
    {
        return Err(TransportError::Codec);
    }
    Ok(())
}
