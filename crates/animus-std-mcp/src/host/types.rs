use std::ffi::OsString;
use std::path::PathBuf;

use serde_json::Value;

use crate::{InstructionContribution, ServerPosture, TeardownObservation};

/// Caller-opened executable and working-root authority facts.
pub struct LaunchAuthority {
    /// Exact executable path supplied by the caller.
    pub executable_path: PathBuf,
    /// Exact executable identity bound by configuration.
    pub executable_identity: String,
    /// Exact working directory supplied by the caller.
    pub working_directory: PathBuf,
    /// Exact working-root identity bound by configuration.
    pub working_root_identity: String,
}

/// Operation-scoped credential material and safe snapshot identity.
pub struct ResolvedCredentials {
    /// Safe BLAKE3 identity for the complete current snapshot.
    pub snapshot_identity: String,
    /// Exact allowed process environment entries.
    pub environment: Vec<(OsString, OsString)>,
}

/// Credential resolver failure without credential text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialServiceError {
    /// Resolver service is unavailable.
    Unavailable,
    /// Resolver output did not match configured references and keys.
    InvalidSnapshot,
}

/// Caller-owned operation-scoped credential resolver.
pub trait CredentialService: Send + Sync {
    /// Resolves only the named references and environment keys.
    ///
    /// # Errors
    ///
    /// Returns a bounded error without credential or environment values.
    fn resolve(
        &self,
        credential_refs: &[String],
        environment_keys: &[String],
    ) -> Result<ResolvedCredentials, CredentialServiceError>;
}

/// Bounded concrete host failure class.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostErrorKind {
    /// Launch authority did not match exact configuration.
    LaunchAuthority,
    /// Credential resolution failed before dispatch.
    Credential,
    /// Child process or framing failed.
    Process,
    /// Initialization or call response violated the protocol cohort.
    Protocol,
    /// Discovery or existing MCP mapping failed.
    Discovery,
    /// Existing Animus authority gates denied the call.
    Authority,
    /// Server instructions failed request-context admission.
    Context,
    /// Credential drift made the live generation stale.
    CredentialDrift,
    /// Request cancellation or timeout stopped the generation.
    Cancelled,
    /// Owned teardown remained uncertain.
    TeardownUncertain,
}

/// Bounded host failure with only teardown metadata.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct HostError {
    /// Failure class.
    pub kind: HostErrorKind,
    /// Owned teardown observation.
    pub teardown: TeardownObservation,
}

/// Visible unavailable server entry for required or optional startup failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ServerUnavailable {
    /// Configured server posture.
    pub posture: ServerPosture,
    /// Bounded startup failure.
    pub error: HostError,
}

/// Bounded call output that is never included in receipt debug output.
pub struct CallOutput {
    pub(crate) value: Value,
    pub(crate) identity: String,
    pub(crate) is_error: bool,
}

impl CallOutput {
    /// Returns the admitted result for immediate caller-owned model visibility.
    #[must_use]
    pub const fn value(&self) -> &Value {
        &self.value
    }

    /// Returns the bounded result identity.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the server-declared tool error posture.
    #[must_use]
    pub const fn is_error(&self) -> bool {
        self.is_error
    }
}

pub(crate) struct StartArtifacts {
    pub(crate) instructions: Option<InstructionContribution>,
    pub(crate) server_info_identity: String,
}
