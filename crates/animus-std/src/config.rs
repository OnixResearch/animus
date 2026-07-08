/// Environment variable read by [`load_runtime`].
pub const INTENT_ENV_KEY: &str = "ANIMUS_INTENT";

/// Typed runtime configuration owned by the shell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Runtime {
    intent: animus_core::Intent,
}

impl Runtime {
    /// Parses runtime configuration from one raw intent string.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidIntent`] when core validation rejects the provided
    /// intent.
    pub fn from_intent(raw_intent: &str) -> Result<Self, Error> {
        let intent = animus_core::Intent::new(raw_intent)
            .map_err(|source| Error::InvalidIntent { source })?;
        Ok(Self { intent })
    }

    /// Parses runtime configuration from environment-like key/value pairs.
    ///
    /// This function is deterministic and testable; [`load_runtime`] is the thin
    /// shell that supplies the real process environment.
    ///
    /// # Errors
    ///
    /// Returns [`Error::MissingIntent`] when [`INTENT_ENV_KEY`] is absent.
    /// Returns [`Error::InvalidIntent`] when the value is present but fails core
    /// validation.
    pub fn from_env_pairs<I, K, V>(pairs: I) -> Result<Self, Error>
    where
        I: IntoIterator<Item = (K, V)>,
        K: AsRef<str>,
        V: AsRef<str>,
    {
        let raw_intent = pairs
            .into_iter()
            .find_map(|(key, value)| {
                (key.as_ref() == INTENT_ENV_KEY).then(|| value.as_ref().to_owned())
            })
            .ok_or(Error::MissingIntent {
                key: INTENT_ENV_KEY,
            })?;

        Self::from_intent(&raw_intent)
    }

    /// Returns the validated runtime intent.
    #[must_use]
    pub const fn intent(&self) -> &animus_core::Intent {
        &self.intent
    }

    /// Converts typed shell configuration into deterministic core state.
    #[must_use]
    pub fn into_state(self) -> animus_core::AnimusState {
        animus_core::AnimusState::new(self.intent)
    }
}

/// Loads typed runtime configuration from the process environment.
///
/// # Errors
///
/// Returns [`Error::MissingIntent`] when [`INTENT_ENV_KEY`] is absent. Returns
/// [`Error::InvalidIntent`] when the environment value fails core validation.
#[cfg_attr(
    animus_octet,
    allow(
        ambient_env,
        reason = "load_runtime is the imperative shell boundary that captures process environment"
    )
)]
pub fn load_runtime() -> Result<Runtime, Error> {
    Runtime::from_env_pairs(std::env::vars())
}

/// Runtime configuration errors raised by the shell.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// The required intent setting was not present.
    MissingIntent {
        /// Missing environment key.
        key: &'static str,
    },
    /// The intent setting was present but rejected by the core.
    InvalidIntent {
        /// Core validation error.
        source: animus_core::CoreError,
    },
}

impl std::fmt::Display for Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingIntent { key } => {
                write!(formatter, "missing required environment variable {key}")
            }
            Self::InvalidIntent { source } => write!(formatter, "invalid intent: {source}"),
        }
    }
}

impl std::error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_INTENT: &str = "build reliable systems";
    const PADDED_VALID_INTENT: &str = "  build reliable systems  ";
    const OTHER_ENV_KEY: &str = "OTHER_ENV_KEY";
    const OTHER_ENV_VALUE: &str = "ignored";

    #[test]
    fn loads_from_env_pairs() {
        let config = Runtime::from_env_pairs([(INTENT_ENV_KEY, PADDED_VALID_INTENT)])
            .expect("valid env pairs should parse");

        assert_eq!(config.intent().as_str(), VALID_INTENT);
    }

    #[test]
    fn rejects_missing_intent() {
        let error = Runtime::from_env_pairs([(OTHER_ENV_KEY, OTHER_ENV_VALUE)])
            .expect_err("missing intent should fail");

        assert_eq!(
            error,
            Error::MissingIntent {
                key: INTENT_ENV_KEY,
            }
        );
    }

    #[test]
    fn rejects_invalid_intent() {
        let error = Runtime::from_env_pairs([(INTENT_ENV_KEY, "   ")])
            .expect_err("blank intent should fail");

        assert_eq!(
            error,
            Error::InvalidIntent {
                source: animus_core::CoreError::EmptyIntent,
            }
        );
    }
}
