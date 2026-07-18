//! Non-default Mesh LLM host adapter over an injected bounded transport.

use animus_adapters::mesh::{DispatchAdmission, DispatchDecision, ResponseModelDecision};
use serde::Deserialize;

/// Reviewed API path for the initial profile.
pub const RESPONSES_PATH: &str = "/v1/responses";
/// Reviewed catalog path.
pub const MODELS_PATH: &str = "/v1/models";
const BYTES_PER_MEBIBYTE: usize = 1024 * 1024;
const MAX_RESPONSE_MEBIBYTES: usize = 8;
/// Maximum transport response bytes.
pub const MAX_RESPONSE_BYTES: usize = MAX_RESPONSE_MEBIBYTES * BYTES_PER_MEBIBYTE;

/// Caller-owned bounded HTTP request.
pub struct TransportRequest<'a> {
    /// Safe endpoint identity, not a URL.
    pub endpoint_identity: &'a str,
    /// Explicit reviewed API path.
    pub path: &'static str,
    /// Already projected request body.
    pub body: &'a [u8],
    /// Maximum admitted response bytes.
    pub max_response_bytes: usize,
}

/// Bounded transport response.
pub struct TransportOutcome {
    /// Provider status class.
    pub status: TransportStatus,
    /// Bounded response bytes for the reviewed parser.
    pub body: Vec<u8>,
    /// Reported exact semantic model.
    pub response_model: Option<String>,
}

/// Canonical transport status before retry ownership.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportStatus {
    /// Successful bounded response.
    Success,
    /// Retry-class transient failure; retry remains externally owned.
    TransientFailure,
    /// Permanent provider failure.
    PermanentFailure,
    /// Caller cancellation was observed.
    Cancelled,
    /// Caller deadline expired.
    TimedOut,
}

/// Injected network authority; implementations own HTTP, credentials, headers,
/// deadlines, cancellation, and bounded reads.
pub trait MeshTransport {
    /// Sends one admitted bounded request.
    ///
    /// # Errors
    ///
    /// Returns [`MeshHostError`] without exposing credentials or raw diagnostics.
    fn send(&mut self, request: &TransportRequest<'_>) -> Result<TransportOutcome, MeshHostError>;
}

/// Typed host failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeshHostError {
    /// Existing policy or authority denied dispatch.
    AdmissionDenied,
    /// No injected transport was available.
    MissingTransport,
    /// Provider response exceeded the bound.
    ResponseTooLarge,
    /// Provider reported a different semantic model.
    ModelSubstitution,
    /// Provider or transport failed.
    TransportFailed,
    /// Request was cancelled.
    Cancelled,
    /// Request timed out.
    TimedOut,
}

/// Admitted exact-model inference request.
pub struct InferenceRequest<'a> {
    /// Existing pure dispatch admission facts.
    pub admission: &'a DispatchAdmission,
    /// Safe endpoint identity from explicit host configuration.
    pub endpoint_identity: &'a str,
    /// Exact model selected by Animus.
    pub exact_model: &'a str,
    /// Reviewed canonical projection bytes.
    pub projected_body: &'a [u8],
}

/// Safe normalized observation handed to existing response/stream parsers.
pub struct ProviderObservation {
    /// Canonical transport status.
    pub status: TransportStatus,
    /// Exact response-model fidelity.
    pub model_fidelity: ResponseModelDecision,
    /// Bounded provider bytes.
    pub body: Vec<u8>,
}

/// Executes only after all existing policy and authority decisions admit the request.
///
/// # Errors
///
/// Returns [`MeshHostError`] before transport when admission fails, and maps
/// bounded transport, cancellation, timeout, and model-substitution failures
/// without local retry or fallback.
pub fn execute<T: MeshTransport>(
    transport: &mut T,
    request: &InferenceRequest<'_>,
) -> Result<ProviderObservation, MeshHostError> {
    if animus_adapters::mesh::admit_dispatch(request.admission) != DispatchDecision::Admitted {
        return Err(MeshHostError::AdmissionDenied);
    }
    let transport_request = TransportRequest {
        endpoint_identity: request.endpoint_identity,
        path: RESPONSES_PATH,
        body: request.projected_body,
        max_response_bytes: MAX_RESPONSE_BYTES,
    };
    let outcome = transport.send(&transport_request)?;
    if outcome.body.len() > MAX_RESPONSE_BYTES {
        return Err(MeshHostError::ResponseTooLarge);
    }
    match outcome.status {
        TransportStatus::Cancelled => return Err(MeshHostError::Cancelled),
        TransportStatus::TimedOut => return Err(MeshHostError::TimedOut),
        TransportStatus::TransientFailure | TransportStatus::PermanentFailure => {
            return Err(MeshHostError::TransportFailed);
        }
        TransportStatus::Success => {}
    }
    let fidelity = animus_adapters::mesh::validate_response_model(
        request.exact_model,
        outcome.response_model.as_deref(),
    );
    if fidelity != ResponseModelDecision::Exact {
        return Err(MeshHostError::ModelSubstitution);
    }
    Ok(ProviderObservation {
        status: outcome.status,
        model_fidelity: fidelity,
        body: outcome.body,
    })
}

/// Checked generated Mesh gateway profile.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MeshProfile {
    /// Domain-separated exact JSON identity.
    pub identity: String,
    /// Safe profile label.
    pub profile_id: String,
    /// Safe provider label.
    pub provider_label: String,
}

#[derive(Deserialize)]
struct RawProfile {
    schema: String,
    profile_id: String,
    provider_label: String,
    api_style: String,
    gateway_posture: String,
    exact_model_required: bool,
    denied_virtual_models: Vec<String>,
    provider_fallback: String,
    unknown_capability: String,
    redaction: String,
    physical_route: String,
    non_claims: Vec<String>,
    catalog: RawCatalog,
    response: RawResponse,
}

#[derive(Deserialize)]
struct RawCatalog {
    max_models: usize,
    max_model_id_bytes: usize,
}
#[derive(Deserialize)]
struct RawResponse {
    max_bytes: usize,
}

/// Generated-profile parse or parity failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MeshProfileError {
    /// JSON shape is malformed.
    Malformed,
    /// Parsed values differ from the reviewed profile.
    ParityMismatch,
}

/// Revalidates the checked Nickel projection at the Rust shell boundary.
///
/// # Errors
///
/// Returns [`MeshProfileError`] for malformed JSON or any policy drift.
pub fn parse_profile(input: &[u8]) -> Result<MeshProfile, MeshProfileError> {
    let raw: RawProfile = serde_json::from_slice(input).map_err(|_| MeshProfileError::Malformed)?;
    let required_non_claims = [
        "not-remote-runtime-integrity",
        "not-peer-honesty",
        "not-physical-route-truth",
        "not-model-correctness",
        "not-split-parity",
        "not-production-readiness",
    ];
    let parity = raw.schema == "animus.mesh-provider-profile.v1"
        && raw.api_style == "responses"
        && raw.gateway_posture == "private_only"
        && raw.exact_model_required
        && raw.denied_virtual_models == ["auto", "mesh"]
        && raw.provider_fallback == "denied"
        && raw.unknown_capability == "deny_required"
        && raw.redaction == "metadata_only"
        && raw.physical_route == "unknown"
        && raw.catalog.max_models == animus_adapters::mesh::MAX_CATALOG_MODELS
        && raw.catalog.max_model_id_bytes == animus_adapters::mesh::MAX_MODEL_ID_BYTES
        && raw.response.max_bytes == MAX_RESPONSE_BYTES
        && raw
            .non_claims
            .iter()
            .map(String::as_str)
            .eq(required_non_claims);
    if !parity {
        return Err(MeshProfileError::ParityMismatch);
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"animus.mesh.profile.v1");
    hasher.update(input);
    Ok(MeshProfile {
        identity: hasher.finalize().to_hex().to_string(),
        profile_id: raw.profile_id,
        provider_label: raw.provider_label,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::mesh::ExactModelDecision;

    fn admission() -> DispatchAdmission {
        DispatchAdmission {
            exact_model: ExactModelDecision::Admitted,
            profile_admitted: true,
            privacy_admitted: true,
            budget_admitted: true,
            authority_admitted: true,
            remote_admitted: true,
            deadline_available: true,
            cancellation_available: true,
            transport_available: true,
        }
    }
    struct FakeTransport {
        called: bool,
        model: &'static str,
        status: TransportStatus,
    }
    impl MeshTransport for FakeTransport {
        fn send(
            &mut self,
            request: &TransportRequest<'_>,
        ) -> Result<TransportOutcome, MeshHostError> {
            self.called = true;
            assert_eq!(request.path, RESPONSES_PATH);
            Ok(TransportOutcome {
                status: self.status,
                body: b"fixture".to_vec(),
                response_model: Some(self.model.into()),
            })
        }
    }
    #[test]
    fn admitted_exact_model_reaches_injected_transport() {
        let facts = admission();
        let mut transport = FakeTransport {
            called: false,
            model: "model-a",
            status: TransportStatus::Success,
        };
        let result = execute(
            &mut transport,
            &InferenceRequest {
                admission: &facts,
                endpoint_identity: "endpoint:1",
                exact_model: "model-a",
                projected_body: b"request",
            },
        )
        .unwrap();
        assert!(transport.called);
        assert_eq!(result.model_fidelity, ResponseModelDecision::Exact);
    }
    #[test]
    fn denied_authority_never_contacts_transport() {
        let facts = DispatchAdmission {
            authority_admitted: false,
            ..admission()
        };
        let mut transport = FakeTransport {
            called: false,
            model: "model-a",
            status: TransportStatus::Success,
        };
        assert!(
            execute(
                &mut transport,
                &InferenceRequest {
                    admission: &facts,
                    endpoint_identity: "endpoint:1",
                    exact_model: "model-a",
                    projected_body: b"request"
                }
            )
            .is_err()
        );
        assert!(!transport.called);
    }
    #[test]
    fn checked_profile_parses_and_drift_fails() {
        let bytes = include_bytes!("../../../config/generated/mesh-provider-profile.json");
        let profile = parse_profile(bytes).unwrap();
        assert_eq!(profile.provider_label, "mesh-private");
        let drift = String::from_utf8_lossy(bytes).replace("private_only", "public");
        assert_eq!(
            parse_profile(drift.as_bytes()),
            Err(MeshProfileError::ParityMismatch)
        );
    }

    #[test]
    fn response_substitution_and_timeout_fail_closed() {
        let facts = admission();
        let request = InferenceRequest {
            admission: &facts,
            endpoint_identity: "endpoint:1",
            exact_model: "model-a",
            projected_body: b"request",
        };
        let mut substitution = FakeTransport {
            called: false,
            model: "model-b",
            status: TransportStatus::Success,
        };
        assert_eq!(
            execute(&mut substitution, &request).err(),
            Some(MeshHostError::ModelSubstitution)
        );
        let mut timeout = FakeTransport {
            called: false,
            model: "model-a",
            status: TransportStatus::TimedOut,
        };
        assert_eq!(
            execute(&mut timeout, &request).err(),
            Some(MeshHostError::TimedOut)
        );
    }
}
