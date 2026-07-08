const MEMORY_ADAPTER_LABEL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("memory_adapter_label");
const MEMORY_ADAPTER_OWNER_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("memory_adapter_owner");
const MEMORY_CONFLICT_GROUP_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("memory_conflict_group");
const MEMORY_EXPIRATION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("memory_expiration_label");
const MEMORY_SUPERSESSION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("memory_supersession_id");
const MEMORY_QUERY_LABEL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("memory_query_label");
const ZERO_MEMORY_UNITS: u32 = 0;
const RANK_OFFSET: usize = 1;

/// Swappable generic memory adapter role.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GenericMemoryAdapterRole {
    /// Imports already-admitted instruction or memory source descriptors.
    SourceImport,
    /// Persists admitted records in a host-owned durable store.
    DurableStore,
    /// Retrieves or searches normalized memory records.
    RetrievalSearch,
    /// Extracts candidate memories from already-admitted context.
    Extraction,
    /// Supplies embedding, ranking, or hybrid-search support.
    EmbeddingSearchSupport,
    /// Reviews candidate memories for admission.
    Admission,
    /// Applies update, delete, tombstone, or supersession requests.
    UpdateDelete,
    /// Supplies instruction-source memory behavior.
    InstructionSource,
}

impl GenericMemoryAdapterRole {
    /// Returns the stable role label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceImport => "source-import",
            Self::DurableStore => "durable-store",
            Self::RetrievalSearch => "retrieval-search",
            Self::Extraction => "extraction",
            Self::EmbeddingSearchSupport => "embedding-search-support",
            Self::Admission => "admission",
            Self::UpdateDelete => "update-delete",
            Self::InstructionSource => "instruction-source",
        }
    }
}

/// Capability declared by a generic memory adapter.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GenericMemoryAdapterCapability {
    /// Adapter can import sources into normalized records.
    SourceImport,
    /// Adapter can durably store admitted records.
    DurableStore,
    /// Adapter can retrieve or search normalized records.
    RetrievalSearch,
    /// Adapter can propose extracted memory candidates.
    Extraction,
    /// Adapter can provide embedding, ranking, or search assistance.
    EmbeddingSearch,
    /// Adapter can review admission decisions.
    AdmissionReview,
    /// Adapter can update or delete existing records.
    UpdateDelete,
    /// Adapter can supply instruction-source memories.
    InstructionSource,
    /// Adapter can emit safe operation receipts.
    SafeReceipts,
}

impl GenericMemoryAdapterCapability {
    /// Returns the stable capability label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SourceImport => "source-import",
            Self::DurableStore => "durable-store",
            Self::RetrievalSearch => "retrieval-search",
            Self::Extraction => "extraction",
            Self::EmbeddingSearch => "embedding-search",
            Self::AdmissionReview => "admission-review",
            Self::UpdateDelete => "update-delete",
            Self::InstructionSource => "instruction-source",
            Self::SafeReceipts => "safe-receipts",
        }
    }
}

/// Borrowed descriptor for a host-injected memory adapter family member.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryAdapterFamilyDescriptorDraft<'a> {
    /// Safe adapter label.
    pub adapter_label: &'a str,
    /// Adapter role.
    pub role: GenericMemoryAdapterRole,
    /// Declared adapter capabilities.
    pub capabilities: &'a [GenericMemoryAdapterCapability],
    /// Adapter availability.
    pub availability: MemoryAvailabilityStatus,
    /// Safe owner label.
    pub owner_label: &'a str,
}

/// Validated descriptor for a host-injected memory adapter family member.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryAdapterFamilyDescriptor {
    adapter_label: String,
    role: GenericMemoryAdapterRole,
    capabilities: Vec<GenericMemoryAdapterCapability>,
    availability: MemoryAvailabilityStatus,
    owner_label: String,
}

impl MemoryAdapterFamilyDescriptor {
    /// Validates safe adapter metadata before reusable logic depends on it.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError`] when safe labels fail validation or capabilities are absent.
    // r[impl agent_harness_runtime.memory_adapters.family]
    pub fn validate(draft: &MemoryAdapterFamilyDescriptorDraft<'_>) -> Result<Self, MemoryError> {
        let adapter_label = normalize(MEMORY_ADAPTER_LABEL_FIELD, draft.adapter_label)?;
        if draft.capabilities.is_empty() {
            return Err(MemoryError::MissingAdapterCapabilities { adapter_label });
        }
        Ok(Self {
            adapter_label,
            role: draft.role,
            capabilities: draft.capabilities.to_vec(),
            availability: draft.availability,
            owner_label: normalize(MEMORY_ADAPTER_OWNER_FIELD, draft.owner_label)?,
        })
    }

    /// Returns the safe adapter label.
    #[must_use]
    pub fn adapter_label(&self) -> &str {
        self.adapter_label.as_str()
    }

    /// Returns the adapter role.
    #[must_use]
    pub const fn role(&self) -> GenericMemoryAdapterRole {
        self.role
    }

    /// Returns declared capabilities.
    #[must_use]
    pub fn capabilities(&self) -> &[GenericMemoryAdapterCapability] {
        self.capabilities.as_slice()
    }

    /// Returns adapter availability.
    #[must_use]
    pub const fn availability(&self) -> MemoryAvailabilityStatus {
        self.availability
    }

    /// Returns the safe owner label.
    #[must_use]
    pub fn owner_label(&self) -> &str {
        self.owner_label.as_str()
    }
}

/// Input for fail-closed memory adapter availability evaluation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryAdapterOperationInput<'a> {
    /// Whether memory use or generation is enabled for this operation.
    pub enabled: bool,
    /// Roles required by the operation.
    pub required_roles: &'a [GenericMemoryAdapterRole],
    /// Validated adapters injected by the host.
    pub adapters: &'a [MemoryAdapterFamilyDescriptor],
}

/// Fail-closed availability outcome for a memory operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryAdapterOperationAvailability {
    /// All required adapters are present, available, and capability-compatible.
    Ready,
    /// Memory is disabled before any adapter or ambient fallback can run.
    Disabled {
        /// Role that was not evaluated because memory is disabled.
        role: Option<GenericMemoryAdapterRole>,
    },
    /// A required adapter was absent or unavailable.
    Unavailable {
        /// Missing or unavailable role.
        role: GenericMemoryAdapterRole,
    },
    /// A required adapter did not declare the capability needed by its role.
    Unsupported {
        /// Unsupported role.
        role: GenericMemoryAdapterRole,
    },
}

/// Evaluates whether a memory operation may dispatch to injected adapters only.
///
/// The function performs no filesystem, database, MCP, network, provider, or vector-index fallback.
#[must_use]
// r[impl agent_harness_runtime.memory_adapters.family.missing]
pub fn evaluate_memory_adapter_operation(
    input: MemoryAdapterOperationInput<'_>,
) -> MemoryAdapterOperationAvailability {
    if !input.enabled {
        return MemoryAdapterOperationAvailability::Disabled {
            role: input.required_roles.first().copied(),
        };
    }
    for required_role in input.required_roles {
        let Some(adapter) = input
            .adapters
            .iter()
            .find(|adapter| adapter.role() == *required_role)
        else {
            return MemoryAdapterOperationAvailability::Unavailable {
                role: *required_role,
            };
        };
        if adapter.availability() != MemoryAvailabilityStatus::Available {
            return MemoryAdapterOperationAvailability::Unavailable {
                role: *required_role,
            };
        }
        if !adapter.capabilities().contains(&role_capability(*required_role)) {
            return MemoryAdapterOperationAvailability::Unsupported {
                role: *required_role,
            };
        }
    }
    MemoryAdapterOperationAvailability::Ready
}

/// Generic normalized memory record kind.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GenericMemoryRecordKind {
    /// Instruction-like record.
    Instruction,
    /// Durable fact.
    Fact,
    /// User, project, or host preference.
    Preference,
    /// Summary record.
    Summary,
    /// Pitfall or recurring warning.
    Pitfall,
    /// Extracted candidate awaiting review.
    Candidate,
}

impl GenericMemoryRecordKind {
    /// Returns the stable record kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Instruction => "instruction",
            Self::Fact => "fact",
            Self::Preference => "preference",
            Self::Summary => "summary",
            Self::Pitfall => "pitfall",
            Self::Candidate => "candidate",
        }
    }
}

/// Borrowed normalized record-envelope metadata before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryRecordEnvelopeDraft<'a> {
    /// Stable record identity.
    pub id: &'a str,
    /// Generic record kind.
    pub kind: GenericMemoryRecordKind,
    /// Memory scope; missing scope is rejected.
    pub scope: Option<MemoryTrustScope>,
    /// Safe source label.
    pub source_label: &'a str,
    /// Evidence handle; missing evidence is rejected.
    pub evidence_handle: Option<&'a str>,
    /// Host admission status.
    pub admission_status: MemoryAdmissionStatus,
    /// Freshness state.
    pub freshness: MemoryFreshness,
    /// Bounded confidence percentage.
    pub confidence_percent: u8,
    /// Sensitivity or redaction class.
    pub redaction_class: MemoryRedactionClass,
    /// Lowercase BLAKE3 hash for the hidden body or safe summary source.
    pub body_hash: &'a str,
    /// Prompt budget units requested by this record.
    pub budget_units: u32,
    /// Optional safe expiration label.
    pub expiration_label: Option<&'a str>,
    /// Optional record superseded by this record.
    pub supersedes_id: Option<&'a str>,
    /// Optional newer record that supersedes this record.
    pub superseded_by_id: Option<&'a str>,
    /// Optional conflict group label.
    pub conflict_group: Option<&'a str>,
    /// Raw body marker, forbidden at this boundary.
    pub raw_body_marker: Option<&'a str>,
    /// Hidden body marker, forbidden at this boundary.
    pub hidden_body_marker: Option<&'a str>,
}

/// Validated normalized memory record envelope.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryRecordEnvelope {
    id: String,
    kind: GenericMemoryRecordKind,
    scope: MemoryTrustScope,
    source_label: String,
    evidence_handle: String,
    admission_status: MemoryAdmissionStatus,
    freshness: MemoryFreshness,
    confidence: MemoryConfidence,
    redaction_class: MemoryRedactionClass,
    body_hash: crate::artifact::Hash,
    budget_units: NonZeroU32,
    expiration_label: Option<String>,
    supersedes_id: Option<String>,
    superseded_by_id: Option<String>,
    conflict_group: Option<String>,
}

impl MemoryRecordEnvelope {
    /// Validates a backend-normalized memory record before planning or prompt influence.
    ///
    /// # Errors
    ///
    /// Returns [`MemoryError`] when required metadata is missing, malformed, unsafe, expired, or unbudgeted.
    // r[impl agent_harness_runtime.memory_adapters.record_envelope]
    pub fn validate(draft: &MemoryRecordEnvelopeDraft<'_>) -> Result<Self, MemoryError> {
        let id = normalize(DESCRIPTOR_ID_FIELD, draft.id)?;
        reject_record_leaks(draft)?;
        let scope = draft.scope.ok_or_else(|| MemoryError::MissingRecordScope {
            record_id: id.clone(),
        })?;
        let evidence_handle = draft
            .evidence_handle
            .ok_or_else(|| MemoryError::MissingEvidence {
                record_id: id.clone(),
            })
            .and_then(|value| normalize(MEMORY_EVIDENCE_FIELD, value))?;
        reject_expired_descriptor(&id, draft.freshness)?;
        let confidence = MemoryConfidence::new(draft.confidence_percent)?;
        let body_hash = crate::artifact::Hash::new(draft.body_hash)
            .map_err(MemoryError::InvalidHash)?;
        let budget_units = NonZeroU32::new(draft.budget_units).ok_or_else(|| {
            MemoryError::ZeroBudgetUnits {
                descriptor_id: id.clone(),
            }
        })?;
        Ok(Self {
            id,
            kind: draft.kind,
            scope,
            source_label: normalize(MEMORY_SOURCE_FIELD, draft.source_label)?,
            evidence_handle,
            admission_status: draft.admission_status,
            freshness: draft.freshness,
            confidence,
            redaction_class: draft.redaction_class,
            body_hash,
            budget_units,
            expiration_label: normalize_optional(MEMORY_EXPIRATION_FIELD, draft.expiration_label)?,
            supersedes_id: normalize_optional(MEMORY_SUPERSESSION_FIELD, draft.supersedes_id)?,
            superseded_by_id: normalize_optional(
                MEMORY_SUPERSESSION_FIELD,
                draft.superseded_by_id,
            )?,
            conflict_group: normalize_optional(MEMORY_CONFLICT_GROUP_FIELD, draft.conflict_group)?,
        })
    }

    /// Returns record identity.
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Returns record kind.
    #[must_use]
    pub const fn kind(&self) -> GenericMemoryRecordKind {
        self.kind
    }

    /// Returns memory scope.
    #[must_use]
    pub const fn scope(&self) -> MemoryTrustScope {
        self.scope
    }

    /// Returns safe source label.
    #[must_use]
    pub fn source_label(&self) -> &str {
        self.source_label.as_str()
    }

    /// Returns safe evidence handle.
    #[must_use]
    pub fn evidence_handle(&self) -> &str {
        self.evidence_handle.as_str()
    }

    /// Returns admission status.
    #[must_use]
    pub const fn admission_status(&self) -> MemoryAdmissionStatus {
        self.admission_status
    }

    /// Returns freshness state.
    #[must_use]
    pub const fn freshness(&self) -> MemoryFreshness {
        self.freshness
    }

    /// Returns confidence score.
    #[must_use]
    pub const fn confidence(&self) -> MemoryConfidence {
        self.confidence
    }

    /// Returns redaction class.
    #[must_use]
    pub const fn redaction_class(&self) -> MemoryRedactionClass {
        self.redaction_class
    }

    /// Returns BLAKE3 body hash.
    #[must_use]
    pub const fn body_hash(&self) -> &crate::artifact::Hash {
        &self.body_hash
    }

    /// Returns prompt budget units.
    #[must_use]
    pub const fn budget_units(&self) -> NonZeroU32 {
        self.budget_units
    }

    /// Returns optional safe expiration label.
    #[must_use]
    pub fn expiration_label(&self) -> Option<&str> {
        self.expiration_label.as_deref()
    }

    /// Returns optional superseded record identity.
    #[must_use]
    pub fn supersedes_id(&self) -> Option<&str> {
        self.supersedes_id.as_deref()
    }

    /// Returns optional newer superseding record identity.
    #[must_use]
    pub fn superseded_by_id(&self) -> Option<&str> {
        self.superseded_by_id.as_deref()
    }

    /// Returns optional conflict group.
    #[must_use]
    pub fn conflict_group(&self) -> Option<&str> {
        self.conflict_group.as_deref()
    }
}

/// Adapter-declared safe match reason for memory retrieval.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum MemoryMatchReason {
    /// Exact scope or identity match.
    ExactScope,
    /// Keyword or lexical match supplied by an adapter.
    Keyword,
    /// Semantic match supplied by an embedding or hybrid adapter.
    Semantic,
    /// Recent interaction match.
    Recent,
    /// Host or user pin.
    ManualPin,
}

impl MemoryMatchReason {
    /// Returns the stable match reason label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ExactScope => "exact-scope",
            Self::Keyword => "keyword",
            Self::Semantic => "semantic",
            Self::Recent => "recent",
            Self::ManualPin => "manual-pin",
        }
    }
}

/// Candidate memory returned by a retrieval adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryRetrievalCandidate<'a> {
    /// Normalized record envelope.
    pub record: &'a MemoryRecordEnvelope,
    /// Safe match reason.
    pub match_reason: MemoryMatchReason,
    /// Adapter-declared relevance percentage.
    pub relevance_percent: u8,
    /// Safe backend label; never used for model-visible ordering.
    pub backend_label: &'a str,
}

/// Store-agnostic retrieval policy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryRetrievalPolicy<'a> {
    /// Safe query label.
    pub query_label: &'a str,
    /// Allowed memory scopes.
    pub allowed_scopes: &'a [MemoryTrustScope],
    /// Whether stale records may be selected.
    pub include_stale: bool,
    /// Prompt budget for memory injection.
    pub budget: MemoryInjectionBudget,
}

/// Input for pure store-agnostic retrieval planning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MemoryRetrievalInput<'a> {
    /// Retrieval policy.
    pub policy: MemoryRetrievalPolicy<'a>,
    /// Candidate records from one or more adapters.
    pub candidates: &'a [MemoryRetrievalCandidate<'a>],
}

/// Outcome status for one memory retrieval candidate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum MemoryRetrievalOutcomeStatus {
    /// Record is selected for prompt influence.
    Selected,
    /// Record scope is not allowed by policy.
    SkippedScope,
    /// Record is stale and stale inclusion is disabled.
    Stale,
    /// Record is redacted before model-visible influence.
    Redacted,
    /// Record conflicts with an already selected record.
    Conflicting,
    /// Record exceeds the remaining prompt budget.
    OverBudget,
    /// Record has been superseded by another record.
    Superseded,
    /// Record was denied or unavailable by admission.
    Denied,
}

/// Safe store-agnostic retrieval outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryRetrievalOutcome {
    record_id: String,
    source_label: String,
    evidence_handle: String,
    scope: MemoryTrustScope,
    confidence: MemoryConfidence,
    body_hash: crate::artifact::Hash,
    status: MemoryRetrievalOutcomeStatus,
    match_reason: MemoryMatchReason,
    relevance: MemoryConfidence,
    rank: usize,
    requested_units: NonZeroU32,
    consumed_units: u32,
    conflict_group: Option<String>,
}

impl MemoryRetrievalOutcome {
    /// Returns record identity.
    #[must_use]
    pub fn record_id(&self) -> &str {
        self.record_id.as_str()
    }

    /// Returns safe source label.
    #[must_use]
    pub fn source_label(&self) -> &str {
        self.source_label.as_str()
    }

    /// Returns safe evidence handle.
    #[must_use]
    pub fn evidence_handle(&self) -> &str {
        self.evidence_handle.as_str()
    }

    /// Returns memory scope.
    #[must_use]
    pub const fn scope(&self) -> MemoryTrustScope {
        self.scope
    }

    /// Returns record confidence.
    #[must_use]
    pub const fn confidence(&self) -> MemoryConfidence {
        self.confidence
    }

    /// Returns BLAKE3 body hash.
    #[must_use]
    pub const fn body_hash(&self) -> &crate::artifact::Hash {
        &self.body_hash
    }

    /// Returns outcome status.
    #[must_use]
    pub const fn status(&self) -> MemoryRetrievalOutcomeStatus {
        self.status
    }

    /// Returns match reason.
    #[must_use]
    pub const fn match_reason(&self) -> MemoryMatchReason {
        self.match_reason
    }

    /// Returns relevance score.
    #[must_use]
    pub const fn relevance(&self) -> MemoryConfidence {
        self.relevance
    }

    /// Returns model-visible rank.
    #[must_use]
    pub const fn rank(&self) -> usize {
        self.rank
    }

    /// Returns requested units.
    #[must_use]
    pub const fn requested_units(&self) -> NonZeroU32 {
        self.requested_units
    }

    /// Returns consumed units.
    #[must_use]
    pub const fn consumed_units(&self) -> u32 {
        self.consumed_units
    }

    /// Returns optional conflict group.
    #[must_use]
    pub fn conflict_group(&self) -> Option<&str> {
        self.conflict_group.as_deref()
    }
}

/// Pure retrieval plan with selected and skipped memory evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MemoryRetrievalPlan {
    outcomes: Vec<MemoryRetrievalOutcome>,
    selected_count: usize,
    consumed_units: u32,
}

impl MemoryRetrievalPlan {
    /// Returns safe outcomes in deterministic model-visible order.
    #[must_use]
    pub fn outcomes(&self) -> &[MemoryRetrievalOutcome] {
        self.outcomes.as_slice()
    }

    /// Returns selected outcome count.
    #[must_use]
    pub const fn selected_count(&self) -> usize {
        self.selected_count
    }

    /// Returns total consumed budget units.
    #[must_use]
    pub const fn consumed_units(&self) -> u32 {
        self.consumed_units
    }
}

/// Plans memory retrieval without contacting stores, embedding services, providers, files, clocks, or networks.
///
/// # Errors
///
/// Returns [`MemoryError`] when candidate metadata, scores, labels, or bounds are invalid.
// r[impl agent_harness_runtime.memory_adapters.retrieval_plan]
pub fn plan_memory_retrieval(
    input: MemoryRetrievalInput<'_>,
) -> Result<MemoryRetrievalPlan, MemoryError> {
    normalize(MEMORY_QUERY_LABEL_FIELD, input.policy.query_label)?;
    if input.candidates.len() > MAX_MEMORY_DESCRIPTORS {
        return Err(MemoryError::TooManyItems {
            field: DESCRIPTOR_ID_FIELD.as_str(),
            max_items: MAX_MEMORY_DESCRIPTORS,
            actual_items: input.candidates.len(),
        });
    }
    let mut ordered: Vec<&MemoryRetrievalCandidate<'_>> = Vec::with_capacity(input.candidates.len());
    for candidate in input.candidates {
        normalize(MEMORY_ADAPTER_LABEL_FIELD, candidate.backend_label)?;
        MemoryConfidence::new(candidate.relevance_percent)?;
        ordered.push(candidate);
    }
    ordered.sort_by(|left, right| compare_memory_retrieval_candidates(left, right));
    let mut conflict_groups: Vec<String> = Vec::new();
    let mut outcomes = Vec::with_capacity(ordered.len());
    let mut consumed_units = ZERO_MEMORY_UNITS;
    let mut selected_count = usize::default();
    for candidate in ordered {
        let decision = memory_retrieval_decision(
            candidate,
            &input.policy,
            consumed_units,
            conflict_groups.as_slice(),
        );
        if decision.status == MemoryRetrievalOutcomeStatus::Selected {
            selected_count = selected_count.saturating_add(RANK_OFFSET);
            consumed_units = consumed_units.saturating_add(decision.consumed_units);
            if let Some(group) = candidate.record.conflict_group() {
                conflict_groups.push(String::from(group));
            }
        }
        outcomes.push(memory_retrieval_outcome(
            candidate,
            decision.status,
            outcomes.len().saturating_add(RANK_OFFSET),
            decision.consumed_units,
        )?);
    }
    Ok(MemoryRetrievalPlan {
        outcomes,
        selected_count,
        consumed_units,
    })
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct MemoryRetrievalDecision {
    status: MemoryRetrievalOutcomeStatus,
    consumed_units: u32,
}

fn memory_retrieval_decision(
    candidate: &MemoryRetrievalCandidate<'_>,
    policy: &MemoryRetrievalPolicy<'_>,
    consumed_units: u32,
    conflict_groups: &[String],
) -> MemoryRetrievalDecision {
    let record = candidate.record;
    if record.admission_status() != MemoryAdmissionStatus::Admitted {
        return skipped_retrieval(MemoryRetrievalOutcomeStatus::Denied);
    }
    if !scope_allowed(record.scope(), policy.allowed_scopes) {
        return skipped_retrieval(MemoryRetrievalOutcomeStatus::SkippedScope);
    }
    if record.freshness() == MemoryFreshness::Stale && !policy.include_stale {
        return skipped_retrieval(MemoryRetrievalOutcomeStatus::Stale);
    }
    if record.redaction_class() == MemoryRedactionClass::Secret {
        return skipped_retrieval(MemoryRetrievalOutcomeStatus::Redacted);
    }
    if record.superseded_by_id().is_some() {
        return skipped_retrieval(MemoryRetrievalOutcomeStatus::Superseded);
    }
    if let Some(group) = record.conflict_group() {
        if conflict_groups.iter().any(|selected| selected == group) {
            return skipped_retrieval(MemoryRetrievalOutcomeStatus::Conflicting);
        }
    }
    let remaining_units = policy.budget.units().get().saturating_sub(consumed_units);
    let requested_units = record.budget_units().get();
    if requested_units > remaining_units {
        return skipped_retrieval(MemoryRetrievalOutcomeStatus::OverBudget);
    }
    MemoryRetrievalDecision {
        status: MemoryRetrievalOutcomeStatus::Selected,
        consumed_units: requested_units,
    }
}

fn memory_retrieval_outcome(
    candidate: &MemoryRetrievalCandidate<'_>,
    status: MemoryRetrievalOutcomeStatus,
    rank: usize,
    consumed_units: u32,
) -> Result<MemoryRetrievalOutcome, MemoryError> {
    let relevance = MemoryConfidence::new(candidate.relevance_percent)?;
    Ok(MemoryRetrievalOutcome {
        record_id: String::from(candidate.record.id()),
        source_label: String::from(candidate.record.source_label()),
        evidence_handle: String::from(candidate.record.evidence_handle()),
        scope: candidate.record.scope(),
        confidence: candidate.record.confidence(),
        body_hash: candidate.record.body_hash().clone(),
        status,
        match_reason: candidate.match_reason,
        relevance,
        rank,
        requested_units: candidate.record.budget_units(),
        consumed_units,
        conflict_group: candidate.record.conflict_group().map(String::from),
    })
}

fn skipped_retrieval(status: MemoryRetrievalOutcomeStatus) -> MemoryRetrievalDecision {
    MemoryRetrievalDecision {
        status,
        consumed_units: ZERO_MEMORY_UNITS,
    }
}

fn compare_memory_retrieval_candidates(
    left: &MemoryRetrievalCandidate<'_>,
    right: &MemoryRetrievalCandidate<'_>,
) -> core::cmp::Ordering {
    right
        .relevance_percent
        .cmp(&left.relevance_percent)
        .then(right.record.confidence().cmp(&left.record.confidence()))
        .then(left.record.scope().cmp(&right.record.scope()))
        .then(left.record.kind().cmp(&right.record.kind()))
        .then(left.record.id().cmp(right.record.id()))
}

fn scope_allowed(scope: MemoryTrustScope, allowed_scopes: &[MemoryTrustScope]) -> bool {
    allowed_scopes.contains(&scope)
}

fn role_capability(role: GenericMemoryAdapterRole) -> GenericMemoryAdapterCapability {
    match role {
        GenericMemoryAdapterRole::SourceImport => GenericMemoryAdapterCapability::SourceImport,
        GenericMemoryAdapterRole::DurableStore => GenericMemoryAdapterCapability::DurableStore,
        GenericMemoryAdapterRole::RetrievalSearch => GenericMemoryAdapterCapability::RetrievalSearch,
        GenericMemoryAdapterRole::Extraction => GenericMemoryAdapterCapability::Extraction,
        GenericMemoryAdapterRole::EmbeddingSearchSupport => {
            GenericMemoryAdapterCapability::EmbeddingSearch
        }
        GenericMemoryAdapterRole::Admission => GenericMemoryAdapterCapability::AdmissionReview,
        GenericMemoryAdapterRole::UpdateDelete => GenericMemoryAdapterCapability::UpdateDelete,
        GenericMemoryAdapterRole::InstructionSource => {
            GenericMemoryAdapterCapability::InstructionSource
        }
    }
}

fn reject_record_leaks(draft: &MemoryRecordEnvelopeDraft<'_>) -> Result<(), MemoryError> {
    if draft.raw_body_marker.is_some() || draft.hidden_body_marker.is_some() {
        return Err(memory_leak(MemoryReceiptLeak::RawMemoryBody));
    }
    assert!(draft.raw_body_marker.is_none());
    assert!(draft.hidden_body_marker.is_none());
    Ok(())
}

fn normalize_optional(
    field: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<Option<String>, MemoryError> {
    value.map(|inner| normalize(field, inner)).transpose()
}
