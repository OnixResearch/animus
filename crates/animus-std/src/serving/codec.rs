//! JSON boundary for snapshots, the generated profile, and receipts.
//!
//! Every DTO denies unknown fields. Enum values use the kebab-case labels of
//! `animus_adapters::serving`. The pure core revalidates every decoded value.

use animus_adapters::serving::{
    AdmissionOffer, AdmissionRecord, Attestation, AvailabilitySnapshot, CapabilitySet,
    CapacityDraft, CapacityKind, CapacityOrigin, CapacityRule, CapacityScope, CapacityStanding,
    CapacityUnit, ContractIssue, Counter, EligibleRoute, Envelope, IdentityRule, InvocationReceipt,
    Locality, ModelDraft, PolicyGenerationProvenance, ProbabilitySemantics, Profile, ProfileDraft,
    Readiness, ReplicaRule, RolloutDraft, RolloutOperation, ServiceDraft, SnapshotDraft, Usage,
};
use serde::Deserialize;
use serde_json::{Value, json};

const BYTES_PER_KIBIBYTE: usize = 1024;
const MAX_SNAPSHOT_KIBIBYTES: usize = 256;
const MAX_PROFILE_KIBIBYTES: usize = 16;
/// Maximum accepted snapshot JSON bytes.
pub const MAX_SNAPSHOT_BYTES: usize = MAX_SNAPSHOT_KIBIBYTES * BYTES_PER_KIBIBYTE;
const MAX_PROFILE_BYTES: usize = MAX_PROFILE_KIBIBYTES * BYTES_PER_KIBIBYTE;
const EVIDENCE_LABELS: [&str; 2] = ["fixture", "live-endpoint"];

/// JSON decode or validation failure. Carries no input text.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodecError {
    /// The input exceeded the byte bound.
    TooLarge,
    /// The JSON shape, a field, or an enum label was malformed or unknown.
    Malformed,
    /// The schema label was not the expected version.
    WrongSchema,
    /// The generated profile drifted from the reviewed bounds or non-claims.
    ParityMismatch,
    /// The pure core rejected a decoded value.
    Contract(ContractIssue),
}

impl From<ContractIssue> for CodecError {
    fn from(issue: ContractIssue) -> Self {
        Self::Contract(issue)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct SnapshotDto {
    schema: String,
    source_id: String,
    source_generation: u64,
    sequence: u64,
    observed_tick: u64,
    valid_until_tick: u64,
    services: Vec<ServiceDto>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ServiceDto {
    service_id: String,
    endpoint_identity: String,
    deployment_id: String,
    deployment_generation: u64,
    locality: LocalityDto,
    model: Option<ModelDto>,
    readiness: ReadinessDto,
    capabilities: CapabilitiesDto,
    envelope: Option<EnvelopeDto>,
    capacity: Vec<CapacityDto>,
    admission: AdmissionDto,
    replica: Option<String>,
    resource_consumer_ref: Option<String>,
    rollout: Option<RolloutDto>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum LocalityDto {
    Local,
    PrivateRemote,
    HostedRemote,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum ReadinessDto {
    Absent,
    Unknown,
    Loading,
    Ready,
    Draining,
    Unavailable,
    Stale,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum AttestationDto {
    Claimed,
    Observed,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CapabilityDto {
    Supported,
    Unsupported,
    Unknown,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum AdmissionDto {
    OwnerAdmission,
    NotOffered,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ModelDto {
    model_id: String,
    configuration_ref: String,
    attestation: AttestationDto,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct CapabilitiesDto {
    text: CapabilityDto,
    tools: CapabilityDto,
    streaming: CapabilityDto,
    structured_output: CapabilityDto,
    developer_role: CapabilityDto,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct EnvelopeDto {
    max_context_tokens: u32,
    max_output_tokens: u32,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(deny_unknown_fields)]
struct CapacityDto {
    kind: CapacityKindDto,
    unit: CapacityUnitDto,
    scope: CapacityScopeDto,
    origin: CapacityOriginDto,
    value: Option<u64>,
    observed_tick: u64,
    valid_until_tick: u64,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CapacityKindDto {
    RequestSlots,
    QueueDepth,
    KvCacheTokens,
    RequestQuota,
    TokenQuota,
    FreeMemory,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CapacityUnitDto {
    Requests,
    Tokens,
    Bytes,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CapacityScopeDto {
    Replica,
    Deployment,
    Service,
    Account,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum CapacityOriginDto {
    ServingOwner,
    ProviderApi,
    HostProbe,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RolloutDto {
    qualification_ref: String,
    qualified_backend_ref: String,
    qualified_configuration_ref: String,
    base_model_ref: String,
    tokenizer_ref: String,
    adapter_ref: Option<String>,
    loaded_policy_generation: u64,
    activation: AttestationDto,
    probability_semantics: SemanticsDto,
    operations: Vec<OperationDto>,
    qualified_envelope: EnvelopeDto,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum SemanticsDto {
    Absent,
    SampledTokenLogprobs,
    FullDistribution,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum OperationDto {
    TokenLogprobs,
    PromptLogprobs,
    SeededSampling,
    AdapterRouting,
}

/// Per-service owned lists the borrowed drafts point into.
struct ServiceLists {
    capacity: Vec<CapacityDraft>,
    operations: Vec<RolloutOperation>,
}

/// Decodes and validates an `animus.serving-availability-snapshot.v1` JSON document.
///
/// # Errors
///
/// Returns [`CodecError`] for oversized, malformed, wrong-schema, or invalid input.
pub fn parse_snapshot(input: &[u8]) -> Result<AvailabilitySnapshot, CodecError> {
    if input.len() > MAX_SNAPSHOT_BYTES {
        return Err(CodecError::TooLarge);
    }
    let dto: SnapshotDto = serde_json::from_slice(input).map_err(|_| CodecError::Malformed)?;
    if dto.schema != animus_adapters::serving::SNAPSHOT_SCHEMA {
        return Err(CodecError::WrongSchema);
    }
    let lists: Vec<ServiceLists> = dto.services.iter().map(service_lists).collect();
    let mut services = Vec::with_capacity(dto.services.len());
    for (service, owned) in dto.services.iter().zip(&lists) {
        services.push(service_draft(service, owned)?);
    }
    Ok(AvailabilitySnapshot::new(&SnapshotDraft {
        source_id: &dto.source_id,
        source_generation: dto.source_generation,
        sequence: dto.sequence,
        observed_tick: dto.observed_tick,
        valid_until_tick: dto.valid_until_tick,
        services: &services,
    })?)
}

fn service_lists(service: &ServiceDto) -> ServiceLists {
    ServiceLists {
        capacity: service
            .capacity
            .iter()
            .map(|capacity| capacity_of(*capacity))
            .collect(),
        operations: service.rollout.as_ref().map_or_else(Vec::new, |rollout| {
            rollout
                .operations
                .iter()
                .map(|operation| operation_of(*operation))
                .collect()
        }),
    }
}

fn service_draft<'a>(
    service: &'a ServiceDto,
    owned: &'a ServiceLists,
) -> Result<ServiceDraft<'a>, CodecError> {
    let envelope = match service.envelope {
        None => None,
        Some(envelope) => Some(envelope_of(envelope)?),
    };
    let rollout = match service.rollout.as_ref() {
        None => None,
        Some(rollout) => Some(rollout_draft(rollout, &owned.operations)?),
    };
    Ok(ServiceDraft {
        service_id: &service.service_id,
        endpoint_identity: &service.endpoint_identity,
        deployment_id: &service.deployment_id,
        deployment_generation: service.deployment_generation,
        locality: locality_of(service.locality),
        model: service.model.as_ref().map(|model| ModelDraft {
            model_id: &model.model_id,
            configuration_ref: &model.configuration_ref,
            attestation: attestation_of(model.attestation),
        }),
        readiness: readiness_of(service.readiness),
        capabilities: capabilities_of(service.capabilities),
        envelope,
        capacity: &owned.capacity,
        admission: match service.admission {
            AdmissionDto::OwnerAdmission => AdmissionOffer::OwnerAdmission,
            AdmissionDto::NotOffered => AdmissionOffer::NotOffered,
        },
        replica: service.replica.as_deref(),
        resource_consumer_ref: service.resource_consumer_ref.as_deref(),
        rollout,
    })
}

fn rollout_draft<'a>(
    rollout: &'a RolloutDto,
    operations: &'a [RolloutOperation],
) -> Result<RolloutDraft<'a>, CodecError> {
    Ok(RolloutDraft {
        qualification_ref: &rollout.qualification_ref,
        qualified_backend_ref: &rollout.qualified_backend_ref,
        qualified_configuration_ref: &rollout.qualified_configuration_ref,
        base_model_ref: &rollout.base_model_ref,
        tokenizer_ref: &rollout.tokenizer_ref,
        adapter_ref: rollout.adapter_ref.as_deref(),
        loaded_policy_generation: rollout.loaded_policy_generation,
        activation: attestation_of(rollout.activation),
        probability_semantics: match rollout.probability_semantics {
            SemanticsDto::Absent => ProbabilitySemantics::Absent,
            SemanticsDto::SampledTokenLogprobs => ProbabilitySemantics::SampledTokenLogprobs,
            SemanticsDto::FullDistribution => ProbabilitySemantics::FullDistribution,
        },
        operations,
        qualified_envelope: envelope_of(rollout.qualified_envelope)?,
    })
}

fn envelope_of(envelope: EnvelopeDto) -> Result<Envelope, CodecError> {
    Ok(Envelope::new(
        envelope.max_context_tokens,
        envelope.max_output_tokens,
    )?)
}

const fn locality_of(locality: LocalityDto) -> Locality {
    match locality {
        LocalityDto::Local => Locality::Local,
        LocalityDto::PrivateRemote => Locality::PrivateRemote,
        LocalityDto::HostedRemote => Locality::HostedRemote,
    }
}

const fn readiness_of(readiness: ReadinessDto) -> Readiness {
    match readiness {
        ReadinessDto::Absent => Readiness::Absent,
        ReadinessDto::Unknown => Readiness::Unknown,
        ReadinessDto::Loading => Readiness::Loading,
        ReadinessDto::Ready => Readiness::Ready,
        ReadinessDto::Draining => Readiness::Draining,
        ReadinessDto::Unavailable => Readiness::Unavailable,
        ReadinessDto::Stale => Readiness::Stale,
    }
}

const fn attestation_of(attestation: AttestationDto) -> Attestation {
    match attestation {
        AttestationDto::Claimed => Attestation::Claimed,
        AttestationDto::Observed => Attestation::Observed,
    }
}

const fn capability_of(capability: CapabilityDto) -> animus_adapters::mesh::Capability {
    match capability {
        CapabilityDto::Supported => animus_adapters::mesh::Capability::Supported,
        CapabilityDto::Unsupported => animus_adapters::mesh::Capability::Unsupported,
        CapabilityDto::Unknown => animus_adapters::mesh::Capability::Unknown,
    }
}

const fn capabilities_of(capabilities: CapabilitiesDto) -> CapabilitySet {
    CapabilitySet {
        text: capability_of(capabilities.text),
        tools: capability_of(capabilities.tools),
        streaming: capability_of(capabilities.streaming),
        structured_output: capability_of(capabilities.structured_output),
        developer_role: capability_of(capabilities.developer_role),
    }
}

const fn capacity_of(capacity: CapacityDto) -> CapacityDraft {
    CapacityDraft {
        kind: match capacity.kind {
            CapacityKindDto::RequestSlots => CapacityKind::RequestSlots,
            CapacityKindDto::QueueDepth => CapacityKind::QueueDepth,
            CapacityKindDto::KvCacheTokens => CapacityKind::KvCacheTokens,
            CapacityKindDto::RequestQuota => CapacityKind::RequestQuota,
            CapacityKindDto::TokenQuota => CapacityKind::TokenQuota,
            CapacityKindDto::FreeMemory => CapacityKind::FreeMemory,
        },
        unit: match capacity.unit {
            CapacityUnitDto::Requests => CapacityUnit::Requests,
            CapacityUnitDto::Tokens => CapacityUnit::Tokens,
            CapacityUnitDto::Bytes => CapacityUnit::Bytes,
        },
        scope: match capacity.scope {
            CapacityScopeDto::Replica => CapacityScope::Replica,
            CapacityScopeDto::Deployment => CapacityScope::Deployment,
            CapacityScopeDto::Service => CapacityScope::Service,
            CapacityScopeDto::Account => CapacityScope::Account,
        },
        origin: match capacity.origin {
            CapacityOriginDto::ServingOwner => CapacityOrigin::ServingOwner,
            CapacityOriginDto::ProviderApi => CapacityOrigin::ProviderApi,
            CapacityOriginDto::HostProbe => CapacityOrigin::HostProbe,
        },
        value: match capacity.value {
            None => Counter::Unknown,
            Some(value) => Counter::Known(value),
        },
        observed_tick: capacity.observed_tick,
        valid_until_tick: capacity.valid_until_tick,
    }
}

const fn operation_of(operation: OperationDto) -> RolloutOperation {
    match operation {
        OperationDto::TokenLogprobs => RolloutOperation::TokenLogprobs,
        OperationDto::PromptLogprobs => RolloutOperation::PromptLogprobs,
        OperationDto::SeededSampling => RolloutOperation::SeededSampling,
        OperationDto::AdapterRouting => RolloutOperation::AdapterRouting,
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ProfileDto {
    schema: String,
    profile_id: String,
    identity_rule: IdentityRuleDto,
    capacity_rule: CapacityRuleDto,
    replica_rule: ReplicaRuleDto,
    max_snapshot_age_ticks: u64,
    owner_admission: String,
    redaction: String,
    evidence_labels: Vec<String>,
    bounds: BoundsDto,
    non_claims: Vec<String>,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum IdentityRuleDto {
    ClaimedAccepted,
    ObservedRequired,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum CapacityRuleDto {
    KnownRequired,
    OwnerAdmissionDecides,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
enum ReplicaRuleDto {
    SameModelAllowed,
    Pinned,
}

#[derive(Clone, Copy, Deserialize, Eq, PartialEq)]
#[serde(deny_unknown_fields)]
struct BoundsDto {
    #[serde(rename = "max_services")]
    services: u32,
    #[serde(rename = "max_capacity_observations")]
    capacity_observations: u32,
    #[serde(rename = "max_route_candidates")]
    route_candidates: u32,
    #[serde(rename = "max_rollout_operations")]
    rollout_operations: u32,
    #[serde(rename = "max_token_bytes")]
    token_bytes: u32,
    #[serde(rename = "max_snapshot_bytes")]
    snapshot_bytes: usize,
}

const REVIEWED_BOUNDS: BoundsDto = BoundsDto {
    services: animus_adapters::serving::MAX_SERVICES,
    capacity_observations: animus_adapters::serving::MAX_CAPACITY_OBSERVATIONS,
    route_candidates: animus_adapters::serving::MAX_ROUTE_CANDIDATES,
    rollout_operations: animus_adapters::serving::MAX_ROLLOUT_OPERATIONS,
    token_bytes: animus_adapters::serving::MAX_TOKEN_BYTES,
    snapshot_bytes: MAX_SNAPSHOT_BYTES,
};

/// Revalidates the generated `animus.serving-capacity-profile.v1` Nickel export.
///
/// # Errors
///
/// Returns [`CodecError`] for oversized, malformed, wrong-schema, or drifted
/// input, including any change to owner admission, redaction, evidence labels,
/// bounds, or non-claims.
pub fn parse_profile(input: &[u8]) -> Result<Profile, CodecError> {
    if input.len() > MAX_PROFILE_BYTES {
        return Err(CodecError::TooLarge);
    }
    let dto: ProfileDto = serde_json::from_slice(input).map_err(|_| CodecError::Malformed)?;
    if dto.schema != animus_adapters::serving::PROFILE_SCHEMA {
        return Err(CodecError::WrongSchema);
    }
    let is_reviewed = dto.owner_admission == "required"
        && dto.redaction == "metadata_only"
        && dto
            .evidence_labels
            .iter()
            .map(String::as_str)
            .eq(EVIDENCE_LABELS)
        && dto.bounds == REVIEWED_BOUNDS
        && dto
            .non_claims
            .iter()
            .map(String::as_str)
            .eq(animus_adapters::serving::RECEIPT_NON_CLAIMS);
    if !is_reviewed {
        return Err(CodecError::ParityMismatch);
    }
    Ok(Profile::new(ProfileDraft {
        profile_id: &dto.profile_id,
        identity: match dto.identity_rule {
            IdentityRuleDto::ClaimedAccepted => IdentityRule::ClaimedAccepted,
            IdentityRuleDto::ObservedRequired => IdentityRule::ObservedRequired,
        },
        capacity: match dto.capacity_rule {
            CapacityRuleDto::KnownRequired => CapacityRule::KnownRequired,
            CapacityRuleDto::OwnerAdmissionDecides => CapacityRule::OwnerAdmissionDecides,
        },
        replica: match dto.replica_rule {
            ReplicaRuleDto::SameModelAllowed => ReplicaRule::SameModelAllowed,
            ReplicaRuleDto::Pinned => ReplicaRule::Pinned,
        },
        max_snapshot_age_ticks: dto.max_snapshot_age_ticks,
    })?)
}

/// Projects an `animus.serving-invocation-receipt.v1` JSON document.
///
/// The projection contains only labels, counts, ticks, opaque tokens, and
/// canonical references already validated by the pure core.
#[must_use]
pub fn receipt_json(receipt: &InvocationReceipt) -> Value {
    let (eligible, excluded) = receipt.screened_counts();
    let (source_generation, sequence) = receipt.snapshot_position();
    json!({
        "schema": animus_adapters::serving::RECEIPT_SCHEMA,
        "identity": receipt.identity(),
        "evidence": receipt.evidence().as_str(),
        "request_id": receipt.request_id(),
        "caller": {
            "agent_id": receipt.caller().agent_id(),
            "session_id": receipt.caller().session_id(),
            "attempt_id": receipt.caller().attempt_id(),
        },
        "profile_identity": receipt.profile_identity(),
        "snapshot": {
            "identity": receipt.snapshot_identity(),
            "source_generation": source_generation,
            "sequence": sequence,
            "status": receipt.snapshot_status().as_str(),
        },
        "screened": { "eligible": eligible, "excluded": excluded },
        "route": receipt.selected().map(route_json),
        "admission": admission_json(receipt.admission()),
        "replica": {
            "placed": receipt.replica().label(),
            "changed": receipt.is_replica_changed(),
        },
        "outcome": {
            "class": receipt.outcome().class(),
            "reason": receipt.outcome().as_str(),
        },
        "response": receipt.response().map(|response| json!({
            "model": animus_adapters::serving::response_model_label(response.model),
            "policy_generation": generation_json(response.policy_generation),
        })),
        "usage": usage_json(receipt.usage()),
        "provider_receipt_identity": receipt.provider_receipt_identity(),
        "non_claims": animus_adapters::serving::RECEIPT_NON_CLAIMS,
    })
}

fn route_json(route: &EligibleRoute) -> Value {
    let capacity = match route.capacity() {
        CapacityStanding::Known { kind, value } => {
            json!({ "standing": "known", "kind": kind.as_str(), "value": value })
        }
        CapacityStanding::UnknownOwnerDecides => json!({ "standing": "owner-decides" }),
    };
    json!({
        "route_name": route.route().route_name(),
        "route_ref": route.route().route_ref(),
        "service_id": route.deployment().service_id(),
        "endpoint_identity": route.deployment().endpoint_identity(),
        "deployment_id": route.deployment().deployment_id(),
        "deployment_generation": route.deployment().deployment_generation(),
        "model_id": route.model().model_id(),
        "configuration_ref": route.model().configuration_ref(),
        "model_attestation": route.attestation().as_str(),
        "capacity": capacity,
        "selected_replica": route.replica().label(),
        "resource_consumer_ref": route.resource_consumer_ref(),
    })
}

fn admission_json(admission: &AdmissionRecord) -> Value {
    match admission {
        AdmissionRecord::Granted {
            grant_ref,
            admission_epoch,
        } => json!({
            "status": admission.as_str(),
            "grant_ref": grant_ref.as_str(),
            "admission_epoch": admission_epoch,
        }),
        AdmissionRecord::NotRequested
        | AdmissionRecord::Rejected(_)
        | AdmissionRecord::Unavailable
        | AdmissionRecord::PortIssue => json!({ "status": admission.as_str() }),
    }
}

fn generation_json(provenance: PolicyGenerationProvenance) -> Value {
    match provenance {
        PolicyGenerationProvenance::NotRequired => json!({ "status": "not-required" }),
        PolicyGenerationProvenance::Matched(generation) => {
            json!({ "status": "matched", "observed": generation })
        }
        PolicyGenerationProvenance::Missing => json!({ "status": "missing" }),
        PolicyGenerationProvenance::Crossed { required, observed } => {
            json!({ "status": "crossed", "required": required, "observed": observed })
        }
    }
}

fn usage_json(usage: Usage) -> Value {
    let counter = |counter: Counter| match counter {
        Counter::Unknown => Value::Null,
        Counter::Known(value) => json!(value),
    };
    json!({
        "origin": usage.origin.as_str(),
        "input_tokens": counter(usage.input_tokens),
        "output_tokens": counter(usage.output_tokens),
    })
}
