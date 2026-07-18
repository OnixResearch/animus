//! Pure admission and receipt contracts for the opt-in SpaceWasm diagnostic lane.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Exact reviewed NASA `SpaceWasm` revision.
pub const SPACEWASM_REVISION: &str = "e24cf09355a90497148eb5029fdb8e3400bd63e3";
/// Exact Octet runtime cohort identity.
pub const SPACEWASM_COHORT: &str =
    "b3:7c537eef7bff66d0a90508715de08de79c9aaac23352e1007a6d47e7fb4674a2";
/// Exact upstream lock identity.
pub const SPACEWASM_LOCK: &str =
    "b3:4e6a07910125d4c921a62cec03d8bc9acb3bf341ebd1a11999df08d6c6c5c5f1";
/// Exact supported producer target.
pub const PRODUCER_TARGET: &str = "wasm32v1-none";
/// Exact required export.
pub const REQUIRED_EXPORT: &str = "run";
/// Maximum admitted artifact size.
pub const MAX_ARTIFACT_BYTES: u64 = 1_048_576;
/// Maximum admitted input size.
pub const MAX_INPUT_BYTES: u64 = 4_096;
/// Maximum admitted output size.
pub const MAX_OUTPUT_BYTES: u64 = 4_096;
/// Maximum admitted fuel observation.
pub const MAX_FUEL: u64 = 1_000_000;

/// Static profile supplied by the checked Nickel projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Profile<'a> {
    /// Explicit enablement.
    pub enabled: bool,
    /// Producer target.
    pub target: &'a str,
    /// Runtime source revision.
    pub runtime_revision: &'a str,
    /// Runtime cohort identity.
    pub runtime_cohort: &'a str,
    /// Upstream lock identity.
    pub source_lock: &'a str,
    /// Declared exports.
    pub exports: &'a [&'a str],
    /// Declared imports.
    pub imports: &'a [&'a str],
    /// Artifact byte limit.
    pub max_artifact_bytes: u64,
    /// Input byte limit.
    pub max_input_bytes: u64,
    /// Output byte limit.
    pub max_output_bytes: u64,
    /// Fuel limit.
    pub max_fuel: u64,
}

/// Static exact-byte and Octet observations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactFacts<'a> {
    /// Domain-separated exact-byte identity.
    pub artifact_identity: &'a str,
    /// Observed artifact length.
    pub artifact_bytes: u64,
    /// Octet profile identifier.
    pub octet_profile: &'a str,
    /// Octet-reported artifact identity.
    pub octet_artifact_identity: &'a str,
    /// Octet-reported cohort identity.
    pub octet_cohort: &'a str,
    /// Observed exports.
    pub exports: &'a [&'a str],
    /// Observed imports.
    pub imports: &'a [&'a str],
    /// Observed proposal labels.
    pub proposals: &'a [&'a str],
}

/// Normalized result returned by an injected runtime service.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Observation<'a> {
    /// Exact artifact identity executed.
    pub artifact_identity: &'a str,
    /// Exact runtime cohort used.
    pub runtime_cohort: &'a str,
    /// Input identity.
    pub input_identity: &'a str,
    /// Output identity when output exists.
    pub output_identity: Option<&'a str>,
    /// Input byte count.
    pub input_bytes: u64,
    /// Output byte count.
    pub output_bytes: u64,
    /// Fuel observation.
    pub fuel: u64,
    /// Result class.
    pub result: ResultClass,
}

/// Bounded runtime result classes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResultClass {
    /// Declared operation returned an admitted output.
    Returned,
    /// Declared trap fixture trapped.
    ExpectedTrap,
    /// Runtime exhausted its declared resource bound.
    ResourceExhausted,
    /// Runtime setup or execution failed.
    RuntimeFailure,
}

/// Deterministic admission failures.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// Profile is not explicitly enabled.
    Disabled,
    /// Producer target or runtime cohort drifted.
    CohortDrift,
    /// Surface contains an undeclared export, import, or proposal.
    SurfaceWidening,
    /// A finite byte or fuel limit was violated.
    ResourceLimit,
    /// Exact-byte evidence disagrees.
    EvidenceMismatch,
    /// A safe identity or result is malformed.
    MalformedObservation,
    /// Receipt text would leak unsafe material or overclaim.
    UnsafeReceipt,
}

/// Metadata-only compatibility receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Receipt {
    /// Exact guest identity.
    pub artifact_identity: String,
    /// Exact runtime cohort.
    pub runtime_cohort: String,
    /// Octet profile.
    pub octet_profile: String,
    /// Input identity.
    pub input_identity: String,
    /// Optional output identity.
    pub output_identity: Option<String>,
    /// Result class.
    pub result: ResultClass,
    /// Stable non-claims.
    pub non_claims: Vec<&'static str>,
}

/// Admit a checked profile and exact static artifact report.
///
/// # Errors
///
/// Returns a deterministic denial when profile, surface, resource, or evidence facts drift.
pub fn admit(profile: &Profile<'_>, artifact: &ArtifactFacts<'_>) -> Result<(), Error> {
    if !profile.enabled {
        return Err(Error::Disabled);
    }
    if profile.target != PRODUCER_TARGET
        || profile.runtime_revision != SPACEWASM_REVISION
        || profile.runtime_cohort != SPACEWASM_COHORT
        || profile.source_lock != SPACEWASM_LOCK
    {
        return Err(Error::CohortDrift);
    }
    if profile.exports != [REQUIRED_EXPORT]
        || !profile.imports.is_empty()
        || artifact.exports != [REQUIRED_EXPORT]
        || !artifact.imports.is_empty()
        || artifact.proposals != ["wasm1"]
    {
        return Err(Error::SurfaceWidening);
    }
    if profile.max_artifact_bytes != MAX_ARTIFACT_BYTES
        || profile.max_input_bytes != MAX_INPUT_BYTES
        || profile.max_output_bytes != MAX_OUTPUT_BYTES
        || profile.max_fuel != MAX_FUEL
        || artifact.artifact_bytes > profile.max_artifact_bytes
    {
        return Err(Error::ResourceLimit);
    }
    if !safe_identity(artifact.artifact_identity)
        || artifact.octet_profile != "spacewasm-mvp"
        || artifact.octet_artifact_identity != artifact.artifact_identity
        || artifact.octet_cohort != profile.runtime_cohort
    {
        return Err(Error::EvidenceMismatch);
    }
    Ok(())
}

/// Classify one normalized injected-service observation and project a safe receipt.
///
/// # Errors
///
/// Returns a deterministic denial when admission or normalized observation checks fail.
pub fn classify(
    profile: &Profile<'_>,
    artifact: &ArtifactFacts<'_>,
    observation: &Observation<'_>,
) -> Result<Receipt, Error> {
    admit(profile, artifact)?;
    if observation.artifact_identity != artifact.artifact_identity
        || observation.runtime_cohort != profile.runtime_cohort
        || !safe_identity(observation.input_identity)
        || observation
            .output_identity
            .is_some_and(|value| !safe_identity(value))
    {
        return Err(Error::EvidenceMismatch);
    }
    if observation.input_bytes > profile.max_input_bytes
        || observation.output_bytes > profile.max_output_bytes
        || observation.fuel > profile.max_fuel
    {
        return Err(Error::ResourceLimit);
    }
    if observation.result == ResultClass::RuntimeFailure {
        return Err(Error::MalformedObservation);
    }
    Ok(Receipt {
        artifact_identity: observation.artifact_identity.to_string(),
        runtime_cohort: observation.runtime_cohort.to_string(),
        octet_profile: artifact.octet_profile.to_string(),
        input_identity: observation.input_identity.to_string(),
        output_identity: observation.output_identity.map(ToString::to_string),
        result: observation.result,
        non_claims: alloc::vec![
            "not-general-wasm-runtime-support",
            "not-plugin-activation",
            "not-runtime-correctness",
            "not-sandbox-completeness",
            "not-production-or-release-readiness",
        ],
    })
}

/// Reject unsafe receipt text and unsupported claim promotion.
///
/// # Errors
///
/// Returns [`Error::UnsafeReceipt`] when denied material or claims appear.
pub fn validate_receipt_text(text: &str) -> Result<(), Error> {
    const DENIED: [&str; 8] = [
        "password",
        "credential",
        "/home/",
        "raw_prompt",
        "provider_payload",
        "production-ready",
        "sandbox-complete",
        "plugin-enabled",
    ];
    if DENIED.iter().any(|needle| text.contains(needle)) {
        return Err(Error::UnsafeReceipt);
    }
    Ok(())
}

fn safe_identity(value: &str) -> bool {
    value.starts_with("b3:")
        && value.len() == 67
        && value.as_bytes()[3..].iter().all(u8::is_ascii_hexdigit)
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID: &str = "b3:0000000000000000000000000000000000000000000000000000000000000000";

    fn profile() -> Profile<'static> {
        Profile {
            enabled: true,
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

    #[test]
    fn exact_profile_artifact_and_results_are_deterministic() {
        let observation = Observation {
            artifact_identity: ID,
            runtime_cohort: SPACEWASM_COHORT,
            input_identity: ID,
            output_identity: Some(ID),
            input_bytes: 4,
            output_bytes: 4,
            fuel: 10,
            result: ResultClass::Returned,
        };
        let first = classify(&profile(), &artifact(), &observation).expect("admitted");
        let second = classify(&profile(), &artifact(), &observation).expect("admitted");
        assert_eq!(first, second);
        assert_eq!(first.non_claims.len(), 5);
    }

    #[test]
    fn expected_trap_is_a_bounded_result() {
        let observation = Observation {
            artifact_identity: ID,
            runtime_cohort: SPACEWASM_COHORT,
            input_identity: ID,
            output_identity: None,
            input_bytes: 1,
            output_bytes: 0,
            fuel: 10,
            result: ResultClass::ExpectedTrap,
        };
        assert_eq!(
            classify(&profile(), &artifact(), &observation)
                .expect("trap")
                .result,
            ResultClass::ExpectedTrap
        );
    }

    #[test]
    fn disabled_drift_widening_tampering_and_limits_fail_closed() {
        let mut disabled = profile();
        disabled.enabled = false;
        assert_eq!(admit(&disabled, &artifact()), Err(Error::Disabled));
        let mut drifted = profile();
        drifted.target = "wasm32-unknown-unknown";
        assert_eq!(admit(&drifted, &artifact()), Err(Error::CohortDrift));
        let mut widened = artifact();
        widened.imports = &["wasi_snapshot_preview1"];
        assert_eq!(admit(&profile(), &widened), Err(Error::SurfaceWidening));
        let mut stale = artifact();
        stale.octet_artifact_identity =
            "b3:ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff";
        assert_eq!(admit(&profile(), &stale), Err(Error::EvidenceMismatch));
        let mut huge = artifact();
        huge.artifact_bytes = MAX_ARTIFACT_BYTES + 1;
        assert_eq!(admit(&profile(), &huge), Err(Error::ResourceLimit));
    }

    #[test]
    fn service_failure_leaks_and_overclaims_are_rejected() {
        let failed = Observation {
            artifact_identity: ID,
            runtime_cohort: SPACEWASM_COHORT,
            input_identity: ID,
            output_identity: None,
            input_bytes: 0,
            output_bytes: 0,
            fuel: 0,
            result: ResultClass::RuntimeFailure,
        };
        assert_eq!(
            classify(&profile(), &artifact(), &failed),
            Err(Error::MalformedObservation)
        );
        assert_eq!(
            validate_receipt_text("credential=/home/user production-ready"),
            Err(Error::UnsafeReceipt)
        );
        assert_eq!(
            validate_receipt_text("artifact=b3:00 not-runtime-correctness"),
            Ok(())
        );
    }
}
