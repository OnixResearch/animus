const ENV_SOURCE_LABEL: &str = "env";
const FILE_SOURCE_LABEL: &str = "file";
const OAUTH_SOURCE_LABEL: &str = "oauth";

/// Credential source label for a provider entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CredentialSource {
    /// No credential is required.
    None,
    /// Credential is expected in an environment variable.
    Environment {
        /// Environment variable name.
        name: String,
    },
    /// Credential is expected in a shell-owned credential file.
    File {
        /// Redacted credential file label.
        label: String,
    },
    /// Credential is expected through a shell-owned OAuth flow.
    Oauth {
        /// Redacted OAuth provider label.
        label: String,
    },
}

impl CredentialSource {
    /// Builds an environment-variable credential source.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when the environment key is unsafe.
    pub fn environment(name: &str) -> Result<Self, super::RegistryError> {
        super::identity::validate_id(super::identity::IdField::CredentialSource, name)?;
        Ok(Self::Environment {
            name: String::from(name),
        })
    }

    /// Builds a file credential source with a redacted label.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when the label is unsafe.
    pub fn file(label: &str) -> Result<Self, super::RegistryError> {
        super::identity::validate_id(super::identity::IdField::CredentialSource, label)?;
        Ok(Self::File {
            label: String::from(label),
        })
    }

    /// Builds an OAuth credential source with a redacted label.
    ///
    /// # Errors
    ///
    /// Returns [`super::RegistryError`] when the label is unsafe.
    pub fn oauth(label: &str) -> Result<Self, super::RegistryError> {
        super::identity::validate_id(super::identity::IdField::CredentialSource, label)?;
        Ok(Self::Oauth {
            label: String::from(label),
        })
    }

    /// Returns a safe source label that never includes secret material.
    #[must_use]
    pub fn safe_label(&self) -> String {
        match self {
            Self::None => String::from("none"),
            Self::Environment { name } => format!("{ENV_SOURCE_LABEL}:{name}"),
            Self::File { label } => format!("{FILE_SOURCE_LABEL}:{label}"),
            Self::Oauth { label } => format!("{OAUTH_SOURCE_LABEL}:{label}"),
        }
    }
}

/// Credential requirement declared by a provider entry.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialRequirement {
    source: CredentialSource,
    is_required: bool,
}

impl CredentialRequirement {
    /// Builds a credential requirement.
    #[must_use]
    pub const fn new(source: CredentialSource, is_required: bool) -> Self {
        Self {
            source,
            is_required,
        }
    }

    /// Builds a provider entry with no credential requirement.
    #[must_use]
    pub const fn none() -> Self {
        Self::new(CredentialSource::None, false)
    }

    /// Returns the credential source.
    #[must_use]
    pub const fn source(&self) -> &CredentialSource {
        &self.source
    }

    /// Returns whether credential absence is an error.
    #[must_use]
    pub const fn is_required(&self) -> bool {
        self.is_required
    }
}

/// Opaque credential handle returned by shell credential resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CredentialHandle {
    source_label: String,
}

impl CredentialHandle {
    /// Returns the redacted credential source label.
    #[must_use]
    pub fn source_label(&self) -> &str {
        &self.source_label
    }
}

/// Redacted missing-credential diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MissingCredentialDiagnostic {
    source_label: String,
}

impl MissingCredentialDiagnostic {
    /// Builds a redacted missing-credential diagnostic.
    #[must_use]
    pub fn new(source: &CredentialSource) -> Self {
        Self {
            source_label: source.safe_label(),
        }
    }

    /// Returns the redacted credential source label.
    #[must_use]
    pub fn source_label(&self) -> &str {
        &self.source_label
    }
}

impl std::fmt::Display for MissingCredentialDiagnostic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "source={}", self.source_label)
    }
}

/// Resolves a provider credential from environment-like key/value pairs.
///
/// This function is deterministic and testable. A real shell supplies
/// `std::env::vars()` or an equivalent secret store at the imperative edge.
///
/// # Errors
///
/// Returns [`super::RegistryError`] when a required credential is missing.
pub fn resolve_credential_from_env_pairs<I, K, V>(
    requirement: &CredentialRequirement,
    pairs: I,
) -> Result<Option<CredentialHandle>, super::RegistryError>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    match requirement.source() {
        CredentialSource::None => Ok(None),
        CredentialSource::Environment { name } => resolve_environment(requirement, name, pairs),
        CredentialSource::File { .. } | CredentialSource::Oauth { .. } => {
            missing_credential(requirement)
        }
    }
}

fn resolve_environment<I, K, V>(
    requirement: &CredentialRequirement,
    name: &str,
    pairs: I,
) -> Result<Option<CredentialHandle>, super::RegistryError>
where
    I: IntoIterator<Item = (K, V)>,
    K: AsRef<str>,
    V: AsRef<str>,
{
    let is_found = pairs
        .into_iter()
        .any(|(key, value)| key.as_ref() == name && !value.as_ref().is_empty());

    if is_found {
        return Ok(Some(CredentialHandle {
            source_label: requirement.source().safe_label(),
        }));
    }

    missing_credential(requirement)
}

fn missing_credential(
    requirement: &CredentialRequirement,
) -> Result<Option<CredentialHandle>, super::RegistryError> {
    if requirement.is_required() {
        return Err(super::RegistryError::MissingCredential {
            diagnostic: MissingCredentialDiagnostic::new(requirement.source()),
        });
    }

    Ok(None)
}
