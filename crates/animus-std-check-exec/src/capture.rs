//! Private retained output from one selected-check execution.

/// Opt-in, untrusted output associated with its metadata observation.
///
/// This object has no public constructor or deserializer. Its bytes can contain
/// secrets. The host does not publish, interpret, or authenticate their meaning.
/// A retained prefix is not a complete record when its stream is truncated.
///
/// ```compile_fail
/// let forged = animus_std_check_exec::CapturedCheck {
///     observation: todo!(),
///     output: todo!(),
/// };
/// ```
pub struct CapturedCheck {
    observation: animus_std::HostCheckExecutionObservation,
    output: bounded_exec::ExecutionOutput,
}

impl CapturedCheck {
    pub(crate) fn new(
        observation: animus_std::HostCheckExecutionObservation,
        output: bounded_exec::ExecutionOutput,
    ) -> Self {
        assert!(output.stdout.bytes.len() <= output.stdout.observed_bytes);
        assert!(output.stderr.bytes.len() <= output.stderr.observed_bytes);
        Self {
            observation,
            output,
        }
    }

    /// Borrow the unchanged metadata-only observation from this execution.
    #[must_use]
    pub const fn observation(&self) -> &animus_std::HostCheckExecutionObservation {
        &self.observation
    }

    /// Borrow the pinned runner's complete bounded output contract.
    ///
    /// Includes retained prefixes, observed byte counts, truncation, completion,
    /// exit/signal, and disposition. These facts do not prove output semantics,
    /// executable authenticity, authorization, sandboxing, or side-effect safety.
    #[must_use]
    pub const fn output(&self) -> &bounded_exec::ExecutionOutput {
        &self.output
    }

    /// Discard private output and preserve the original service result.
    #[must_use]
    pub fn into_observation(self) -> animus_std::HostCheckExecutionObservation {
        self.observation
    }
}

impl std::fmt::Debug for CapturedCheck {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("CapturedCheck")
            .field("status", &self.observation.status)
            .field("stdout_bytes", &self.observation.stdout_bytes)
            .field("stderr_bytes", &self.observation.stderr_bytes)
            .field("stdout_truncated", &self.output.stdout.truncated)
            .field("stderr_truncated", &self.output.stderr.truncated)
            .finish_non_exhaustive()
    }
}
