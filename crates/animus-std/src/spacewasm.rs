//! Thin injected-service shell for `SpaceWasm` diagnostic observations.

use animus_adapters::spacewasm::{
    ArtifactFacts, Error, MAX_ARTIFACT_BYTES, MAX_FUEL, MAX_INPUT_BYTES, MAX_OUTPUT_BYTES,
    Observation, PRODUCER_TARGET, Profile, REQUIRED_EXPORT, Receipt, SPACEWASM_COHORT,
    SPACEWASM_LOCK, SPACEWASM_REVISION, admit, classify,
};

/// Generated-profile parsing or semantic mismatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileJsonError {
    /// Generated JSON was malformed.
    Malformed,
    /// Generated fields do not match the compiled exact profile.
    Drift,
}

/// Parse and semantically revalidate the checked generated profile.
///
/// # Errors
///
/// Returns a parse or semantic-drift error for malformed or non-exact projections.
pub fn validate_profile_json(bytes: &[u8]) -> Result<(), ProfileJsonError> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| ProfileJsonError::Malformed)?;
    let string = |pointer: &str| value.pointer(pointer).and_then(serde_json::Value::as_str);
    let number = |pointer: &str| value.pointer(pointer).and_then(serde_json::Value::as_u64);
    let exact = string("/schema") == Some("animus.spacewasm-embedded-compat.v1")
        && value
            .pointer("/enabled")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && string("/producer/target") == Some(PRODUCER_TARGET)
        && string("/runtime/revision") == Some(SPACEWASM_REVISION)
        && string("/runtime/cohort_identity") == Some(SPACEWASM_COHORT)
        && string("/runtime/source_lock_identity") == Some(SPACEWASM_LOCK)
        && value
            .pointer("/guest/exports/0")
            .and_then(serde_json::Value::as_str)
            == Some(REQUIRED_EXPORT)
        && value
            .pointer("/guest/exports")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|values| values.len() == 1)
        && value
            .pointer("/guest/imports")
            .and_then(serde_json::Value::as_array)
            .is_some_and(std::vec::Vec::is_empty)
        && number("/limits/artifact_bytes") == Some(MAX_ARTIFACT_BYTES)
        && number("/limits/input_bytes") == Some(MAX_INPUT_BYTES)
        && number("/limits/output_bytes") == Some(MAX_OUTPUT_BYTES)
        && number("/limits/fuel") == Some(MAX_FUEL);
    if exact {
        Ok(())
    } else {
        Err(ProfileJsonError::Drift)
    }
}

/// Explicit request sent to a caller-owned diagnostic runtime service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionRequest<'a> {
    /// Exact guest artifact identity.
    pub artifact_identity: &'a str,
    /// Exact runtime cohort identity.
    pub runtime_cohort: &'a str,
    /// Domain-separated input identity.
    pub input_identity: &'a str,
    /// Bounded input bytes supplied by the caller.
    pub input: &'a [u8],
}

/// Injected diagnostic service. Implementations own all bundle reads and execution effects.
pub trait DiagnosticService {
    /// Execute one already-admitted request and return normalized facts only.
    ///
    /// # Errors
    ///
    /// Returns a bounded service failure without ambient fallback.
    fn execute(
        &mut self,
        request: &ExecutionRequest<'_>,
    ) -> Result<Observation<'static>, ServiceError>;
}

/// Bounded service failure classes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServiceError {
    /// No service was configured.
    Unavailable,
    /// Runtime setup failed.
    Setup,
    /// Runtime execution failed.
    Execution,
    /// Cancellation was observed.
    Cancelled,
    /// Deadline was observed.
    TimedOut,
}

/// Shell-level execution failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostError {
    /// Pure admission failed before service contact.
    Admission(Error),
    /// Injected service failed.
    Service(ServiceError),
}

/// Admit exact evidence, invoke one injected service, and revalidate its observation.
///
/// # Errors
///
/// Returns a pure admission denial or the injected service's bounded failure class.
pub fn execute(
    profile: &Profile<'_>,
    artifact: &ArtifactFacts<'_>,
    request: &ExecutionRequest<'_>,
    service: &mut dyn DiagnosticService,
) -> Result<Receipt, HostError> {
    admit(profile, artifact).map_err(HostError::Admission)?;
    if request.artifact_identity != artifact.artifact_identity
        || request.runtime_cohort != profile.runtime_cohort
        || request.input.len() as u64 > profile.max_input_bytes
    {
        return Err(HostError::Admission(Error::EvidenceMismatch));
    }
    let observation = service.execute(request).map_err(HostError::Service)?;
    classify(profile, artifact, &observation).map_err(HostError::Admission)
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::spacewasm::ResultClass;

    const ID: &str = "b3:0000000000000000000000000000000000000000000000000000000000000000";

    struct FixtureService {
        calls: usize,
        outcome: Result<Observation<'static>, ServiceError>,
    }
    impl DiagnosticService for FixtureService {
        fn execute(
            &mut self,
            _: &ExecutionRequest<'_>,
        ) -> Result<Observation<'static>, ServiceError> {
            self.calls += 1;
            self.outcome.clone()
        }
    }

    fn profile(enabled: bool) -> Profile<'static> {
        Profile {
            enabled,
            target: PRODUCER_TARGET,
            runtime_revision: SPACEWASM_REVISION,
            runtime_cohort: SPACEWASM_COHORT,
            source_lock: SPACEWASM_LOCK,
            exports: &[REQUIRED_EXPORT],
            imports: &[],
            max_artifact_bytes: MAX_ARTIFACT_BYTES,
            max_input_bytes: MAX_INPUT_BYTES,
            max_output_bytes: MAX_OUTPUT_BYTES,
            max_fuel: MAX_FUEL,
        }
    }
    fn artifact() -> ArtifactFacts<'static> {
        ArtifactFacts {
            artifact_identity: ID,
            artifact_bytes: 128,
            octet_profile: "spacewasm-mvp",
            octet_artifact_identity: ID,
            octet_cohort: SPACEWASM_COHORT,
            exports: &[REQUIRED_EXPORT],
            imports: &[],
            proposals: &["wasm1"],
        }
    }
    fn observation() -> Observation<'static> {
        Observation {
            artifact_identity: ID,
            runtime_cohort: SPACEWASM_COHORT,
            input_identity: ID,
            output_identity: Some(ID),
            input_bytes: 1,
            output_bytes: 1,
            fuel: 1,
            result: ResultClass::Returned,
        }
    }

    #[test]
    fn generated_profile_is_revalidated_and_drift_rejected() {
        let bytes = include_bytes!("../../../config/generated/spacewasm-embedded-compat.json");
        assert_eq!(validate_profile_json(bytes), Ok(()));
        assert_eq!(validate_profile_json(b"{}"), Err(ProfileJsonError::Drift));
        assert_eq!(
            validate_profile_json(b"{"),
            Err(ProfileJsonError::Malformed)
        );
    }

    #[test]
    fn admitted_request_contacts_only_injected_service() {
        let mut service = FixtureService {
            calls: 0,
            outcome: Ok(observation()),
        };
        let receipt = execute(
            &profile(true),
            &artifact(),
            &ExecutionRequest {
                artifact_identity: ID,
                runtime_cohort: SPACEWASM_COHORT,
                input_identity: ID,
                input: &[1],
            },
            &mut service,
        )
        .expect("execute");
        assert_eq!(service.calls, 1);
        assert_eq!(receipt.result, ResultClass::Returned);
    }

    #[test]
    fn denied_evidence_never_contacts_service_and_failures_stay_explicit() {
        let mut service = FixtureService {
            calls: 0,
            outcome: Ok(observation()),
        };
        assert_eq!(
            execute(
                &profile(false),
                &artifact(),
                &ExecutionRequest {
                    artifact_identity: ID,
                    runtime_cohort: SPACEWASM_COHORT,
                    input_identity: ID,
                    input: &[],
                },
                &mut service
            ),
            Err(HostError::Admission(Error::Disabled))
        );
        assert_eq!(service.calls, 0);
        let mut unavailable = FixtureService {
            calls: 0,
            outcome: Err(ServiceError::Unavailable),
        };
        assert_eq!(
            execute(
                &profile(true),
                &artifact(),
                &ExecutionRequest {
                    artifact_identity: ID,
                    runtime_cohort: SPACEWASM_COHORT,
                    input_identity: ID,
                    input: &[],
                },
                &mut unavailable
            ),
            Err(HostError::Service(ServiceError::Unavailable))
        );
    }
}
