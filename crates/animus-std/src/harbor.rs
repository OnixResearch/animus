//! Thin optional host shell for one admitted local Harbor-compatible trial.

use animus_adapters::harbor::{
    ATIF_SCHEMA, EvaluationEnvironmentService, HARBOR_RELEASE, HARBOR_REVISION, HARBOR_TASK_SCHEMA,
    OutcomeClass, TrialObservation, TrialRequest, VerifierService,
};

/// Generated-profile parse or semantic-drift error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProfileError {
    /// Malformed JSON.
    Malformed,
    /// Exact cohort or policy drift.
    Drift,
}

/// Revalidate the checked generated Harbor profile before host execution.
///
/// # Errors
/// Returns malformed or drift when the generated projection is not exact and conservative.
pub fn validate_profile_json(bytes: &[u8]) -> Result<(), ProfileError> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| ProfileError::Malformed)?;
    let string = |path: &str| value.pointer(path).and_then(serde_json::Value::as_str);
    let exact = string("/schema") == Some("animus.harbor-evaluation-profile.v1")
        && value
            .pointer("/enabled")
            .and_then(serde_json::Value::as_bool)
            == Some(true)
        && string("/harbor/release") == Some(HARBOR_RELEASE)
        && string("/harbor/revision") == Some(HARBOR_REVISION)
        && string("/harbor/task_schema") == Some(HARBOR_TASK_SCHEMA)
        && string("/harbor/atif_schema") == Some(ATIF_SCHEMA)
        && string("/task/network") == Some("none")
        && string("/environment/network") == Some("none")
        && string("/verifier/mode") == Some("separate")
        && value
            .pointer("/secrets/values_allowed")
            .and_then(serde_json::Value::as_bool)
            == Some(false);
    if exact {
        Ok(())
    } else {
        Err(ProfileError::Drift)
    }
}

/// Host orchestration error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum HostError {
    /// Environment service failure.
    Environment(OutcomeClass),
    /// Verifier service failure.
    Verifier(OutcomeClass),
    /// Identity or artifact mismatch.
    EvidenceMismatch,
    /// Shared verifier result cannot be promoted.
    SharedVerifier,
}

/// Run one admitted environment request followed by its separate verifier.
///
/// # Errors
/// Returns explicit environment, verifier, isolation, or evidence failures without fallback.
pub fn execute_separate_verifier(
    request: &TrialRequest,
    environment: &mut dyn EvaluationEnvironmentService,
    verifier: &mut dyn VerifierService,
) -> Result<(TrialObservation, TrialObservation), HostError> {
    if request.environment_identity == request.verifier_identity {
        return Err(HostError::SharedVerifier);
    }
    let subject = environment
        .execute(request)
        .map_err(HostError::Environment)?;
    if subject.trial_identity != request.trial_identity
        || subject.outcome != OutcomeClass::Completed
    {
        return Err(HostError::EvidenceMismatch);
    }
    let verification = verifier.verify(request).map_err(HostError::Verifier)?;
    if verification.trial_identity != request.trial_identity
        || verification.outcome != OutcomeClass::Completed
    {
        return Err(HostError::EvidenceMismatch);
    }
    for artifact in &verification.artifacts {
        if !request.artifacts.contains(artifact) {
            return Err(HostError::EvidenceMismatch);
        }
    }
    Ok((subject, verification))
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Environment {
        calls: usize,
        result: Result<TrialObservation, OutcomeClass>,
    }
    impl EvaluationEnvironmentService for Environment {
        fn execute(&mut self, _: &TrialRequest) -> Result<TrialObservation, OutcomeClass> {
            self.calls += 1;
            self.result.clone()
        }
    }
    struct Verifier {
        calls: usize,
        result: Result<TrialObservation, OutcomeClass>,
    }
    impl VerifierService for Verifier {
        fn verify(&mut self, _: &TrialRequest) -> Result<TrialObservation, OutcomeClass> {
            self.calls += 1;
            self.result.clone()
        }
    }
    fn request() -> TrialRequest {
        TrialRequest {
            trial_identity: "trial:1".into(),
            task_identity: "task:1".into(),
            agent_identity: "agent:1".into(),
            model_route_identity: "model:1".into(),
            environment_identity: "env:1".into(),
            verifier_identity: "verifier:1".into(),
            artifacts: vec!["artifact:1".into()],
        }
    }
    fn observation() -> TrialObservation {
        TrialObservation {
            trial_identity: "trial:1".into(),
            outcome: OutcomeClass::Completed,
            trajectory_identity: Some("trajectory:1".into()),
            rewards: vec![("correctness".into(), 1.0)],
            artifacts: vec!["artifact:1".into()],
        }
    }
    #[test]
    fn exact_cohort_and_separate_vertical_slice_pass() {
        assert_eq!(
            validate_profile_json(include_bytes!(
                "../../../config/generated/harbor-evaluation-profile.json"
            )),
            Ok(())
        );
        assert_eq!(validate_profile_json(b"{}"), Err(ProfileError::Drift));
        assert_eq!(
            (
                HARBOR_RELEASE,
                HARBOR_REVISION,
                HARBOR_TASK_SCHEMA,
                ATIF_SCHEMA
            ),
            (
                "v0.18.0",
                "527d50deb63a5d279e8c20593c18a2cbc7f61f9e",
                "1.3",
                "ATIF-v1.7"
            )
        );
        let mut env = Environment {
            calls: 0,
            result: Ok(observation()),
        };
        let mut verifier = Verifier {
            calls: 0,
            result: Ok(observation()),
        };
        assert!(execute_separate_verifier(&request(), &mut env, &mut verifier).is_ok());
        assert_eq!((env.calls, verifier.calls), (1, 1));
    }
    #[test]
    fn unavailable_shared_and_tampered_paths_fail_closed() {
        let mut shared = request();
        shared.verifier_identity = shared.environment_identity.clone();
        let mut env = Environment {
            calls: 0,
            result: Ok(observation()),
        };
        let mut verifier = Verifier {
            calls: 0,
            result: Ok(observation()),
        };
        assert_eq!(
            execute_separate_verifier(&shared, &mut env, &mut verifier),
            Err(HostError::SharedVerifier)
        );
        assert_eq!((env.calls, verifier.calls), (0, 0));
        env.result = Err(OutcomeClass::TimedOut);
        assert_eq!(
            execute_separate_verifier(&request(), &mut env, &mut verifier),
            Err(HostError::Environment(OutcomeClass::TimedOut))
        );
        let mut tampered = observation();
        tampered.artifacts = vec!["artifact:other".into()];
        env.result = Ok(observation());
        verifier.result = Ok(tampered);
        assert_eq!(
            execute_separate_verifier(&request(), &mut env, &mut verifier),
            Err(HostError::EvidenceMismatch)
        );
    }
}
