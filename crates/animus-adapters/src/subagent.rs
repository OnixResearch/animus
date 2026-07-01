//! Recipe-backed subagent delegation DTOs and pure validation helpers.

const ID_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("subagent_id");
const ROLE_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("subagent_role");
const TASK_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("subagent_task");
const RECIPE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_recipe");
const PROFILE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_profile");
const PARAMETER_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_parameter_binding");
const TOOL_FIELD: crate::validation::FieldName = crate::validation::FieldName::new("subagent_tool");
const WORKSPACE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_workspace_scope");
const MODEL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_model_profile");
const TIMEOUT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_timeout");
const RETURN_MODE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_return_mode");
const SUMMARY_SCHEMA_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_summary_schema");
const SOURCE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_delegation_source");
const CAPABILITY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_capability");
const EXTERNAL_PROTOCOL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_external_protocol");
const EXTERNAL_SERVICE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_external_service");
const HANDOFF_STATUS_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_handoff_status");
const HANDOFF_SUMMARY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_handoff_summary");
const HANDOFF_DETAIL_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_handoff_detail_ref");
const OUTCOME_KIND_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("subagent_outcome_kind");

const SOURCE_USER: &str = "user";
const SOURCE_TRUSTED_SKILL: &str = "trusted_skill";
const SOURCE_ADMITTED_AUTOMATION: &str = "admitted_automation";
const SOURCE_AUTOMATIC: &str = "automatic";
const RETURN_SUMMARY_ONLY: &str = "summary_only";
const RETURN_STATUS_PLUS_SUMMARY: &str = "status_plus_summary";
const RETURN_FULL_DETAIL_BOUNDED: &str = "full_detail_with_bounds";
const PROTOCOL_ACP: &str = "acp";
const PROTOCOL_MCP: &str = "mcp";
const OUTCOME_SUCCESS: &str = "success";
const OUTCOME_FAILURE: &str = "failure";
const OUTCOME_TIMEOUT: &str = "timeout";
const OUTCOME_CANCELLATION: &str = "cancellation";
const OUTCOME_MALFORMED_SUMMARY: &str = "malformed_summary";
const OUTCOME_DENIED_TOOL: &str = "denied_tool";
const OUTCOME_PARTIAL_PARALLEL_SUCCESS: &str = "partial_parallel_success";
const OUTCOME_WRITE_CONFLICT: &str = "write_conflict";
const MIN_TURN_COUNT: u32 = 1;
const MIN_DETAIL_BOUND_CHARS: u32 = 1;

type Text = alloc::string::String;
type Collection<T> = alloc::vec::Vec<T>;

/// Error raised while validating recipe-backed subagent data.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SubagentError {
    /// Generic adapter contract validation failed.
    Contract(crate::AdapterContractError),
    /// An enum-like subagent field contained an unsupported value.
    UnsupportedValue {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported normalized value.
        value: Text,
    },
    /// A required collection was empty.
    EmptyCollection {
        /// Boundary collection field name.
        field: &'static str,
    },
    /// A collection contained a duplicate value.
    DuplicateValue {
        /// Boundary collection field name.
        field: &'static str,
        /// Duplicated normalized value.
        value: Text,
    },
    /// Delegation attempted automatic spawning.
    ImplicitDelegationDenied,
    /// Delegation attempted recursive subagent spawning.
    RecursiveDelegationDenied,
    /// Automation delegation was not admitted by policy.
    AutomationDelegationDenied,
    /// A recipe reference was missing an admitted trust receipt.
    UntrustedRecipeDenied,
    /// Turn ceiling was zero or unbounded.
    InvalidTurnCeiling,
    /// A requested capability was denied by an intersection input.
    CapabilityDenied {
        /// Requested capability label.
        capability: Text,
    },
    /// Full detail handoff omitted its bounded detail reference.
    MissingDetailReference,
    /// Full detail handoff exceeded its configured bound.
    DetailBoundsExceeded,
    /// External subagent requested ambient CLI discovery.
    AmbientExternalDiscoveryDenied,
    /// External subagent requested inherited credentials.
    InheritedCredentialDenied,
    /// External subagent descriptor omitted an injected service label.
    MissingExternalService,
}

impl From<crate::AdapterContractError> for SubagentError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

/// Source that explicitly delegated a subagent.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubagentDelegationSource {
    /// User requested a child run.
    User,
    /// Trusted skill requested a child run.
    TrustedSkill,
    /// Automation policy admitted the child run.
    AdmittedAutomation,
    /// Automatic spawning request, always denied by validation.
    Automatic,
}

impl SubagentDelegationSource {
    /// Parses a stable delegation source label.
    ///
    /// # Errors
    ///
    /// Returns [`SubagentError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, SubagentError> {
        parse_value(SOURCE_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            SOURCE_USER => Some(Self::User),
            SOURCE_TRUSTED_SKILL => Some(Self::TrustedSkill),
            SOURCE_ADMITTED_AUTOMATION => Some(Self::AdmittedAutomation),
            SOURCE_AUTOMATIC => Some(Self::Automatic),
            _ => None,
        }
    }

    /// Returns the stable source label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::User => SOURCE_USER,
            Self::TrustedSkill => SOURCE_TRUSTED_SKILL,
            Self::AdmittedAutomation => SOURCE_ADMITTED_AUTOMATION,
            Self::Automatic => SOURCE_AUTOMATIC,
        }
    }
}

/// Parent handoff return mode for subagent results.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubagentReturnMode {
    /// Parent receives only bounded summary and evidence references.
    SummaryOnly,
    /// Parent receives status and bounded summary.
    StatusPlusSummary,
    /// Parent may receive a bounded detail reference.
    FullDetailWithBounds,
}

impl SubagentReturnMode {
    /// Parses a stable return mode label.
    ///
    /// # Errors
    ///
    /// Returns [`SubagentError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, SubagentError> {
        parse_value(RETURN_MODE_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            RETURN_SUMMARY_ONLY => Some(Self::SummaryOnly),
            RETURN_STATUS_PLUS_SUMMARY => Some(Self::StatusPlusSummary),
            RETURN_FULL_DETAIL_BOUNDED => Some(Self::FullDetailWithBounds),
            _ => None,
        }
    }

    /// Returns the stable return mode label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SummaryOnly => RETURN_SUMMARY_ONLY,
            Self::StatusPlusSummary => RETURN_STATUS_PLUS_SUMMARY,
            Self::FullDetailWithBounds => RETURN_FULL_DETAIL_BOUNDED,
        }
    }
}

/// External subagent protocol kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ExternalSubagentProtocol {
    /// Agent Client Protocol adapter.
    Acp,
    /// Model Context Protocol adapter.
    Mcp,
}

impl ExternalSubagentProtocol {
    /// Parses a stable external subagent protocol label.
    ///
    /// # Errors
    ///
    /// Returns [`SubagentError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, SubagentError> {
        parse_value(EXTERNAL_PROTOCOL_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            PROTOCOL_ACP => Some(Self::Acp),
            PROTOCOL_MCP => Some(Self::Mcp),
            _ => None,
        }
    }
}

/// Untrusted external subagent descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ExternalSubagentDescriptorDraft<'a> {
    /// Protocol label.
    pub protocol: &'a str,
    /// External agent identity label.
    pub identity: &'a str,
    /// Injected host service label.
    pub service_label: &'a str,
    /// Declared tool labels available to the external agent.
    pub declared_tools: &'a [&'a str],
    /// Optional credential reference label; never inherited ambient credentials.
    pub credential_label: Option<&'a str>,
    /// Whether the descriptor asks runtime to discover an ambient CLI.
    pub ambient_cli: bool,
    /// Whether the descriptor asks to inherit parent credentials.
    pub inherited_credentials: bool,
}

/// Validated external subagent descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalSubagentDescriptor {
    protocol: ExternalSubagentProtocol,
    identity: Text,
    service_label: Text,
    declared_tools: Collection<Text>,
    credential_label: Option<Text>,
}

impl ExternalSubagentDescriptor {
    /// Validates an external subagent descriptor as injected-adapter data only.
    ///
    /// # Errors
    ///
    /// Returns [`SubagentError`] when protocol, service, tools, or credential
    /// boundaries are malformed or ambient discovery is requested.
    pub fn validate(draft: &ExternalSubagentDescriptorDraft<'_>) -> Result<Self, SubagentError> {
        if draft.ambient_cli {
            return Err(SubagentError::AmbientExternalDiscoveryDenied);
        }
        if draft.inherited_credentials {
            return Err(SubagentError::InheritedCredentialDenied);
        }
        let protocol = ExternalSubagentProtocol::parse(draft.protocol)?;
        let identity = required_text(ID_FIELD, draft.identity)?;
        let service_label = required_text(EXTERNAL_SERVICE_FIELD, draft.service_label).map_err(
            |error| match error {
                SubagentError::Contract(crate::AdapterContractError::EmptyField { .. }) => {
                    SubagentError::MissingExternalService
                }
                other => other,
            },
        )?;
        let declared_tools = text_collection(TOOL_FIELD, draft.declared_tools, true)?;
        let credential_label = optional_text(PROFILE_FIELD, draft.credential_label)?;
        Ok(Self {
            protocol,
            identity,
            service_label,
            declared_tools,
            credential_label,
        })
    }

    /// Returns the external protocol.
    #[must_use]
    pub const fn protocol(&self) -> ExternalSubagentProtocol {
        self.protocol
    }

    /// Returns the external identity label.
    #[must_use]
    pub fn identity(&self) -> &str {
        self.identity.as_str()
    }

    /// Returns the injected service label.
    #[must_use]
    pub fn service_label(&self) -> &str {
        self.service_label.as_str()
    }

    /// Returns declared tool labels.
    #[must_use]
    pub fn declared_tools(&self) -> &[Text] {
        self.declared_tools.as_slice()
    }

    /// Returns the optional credential reference label.
    #[must_use]
    pub fn credential_label(&self) -> Option<&str> {
        self.credential_label.as_deref()
    }
}

/// Untrusted recipe-backed subagent delegation request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubagentDelegationDraft<'a> {
    /// Child run identity.
    pub id: &'a str,
    /// Subagent role label.
    pub role: &'a str,
    /// Child task summary.
    pub task: &'a str,
    /// Optional admitted recipe identity.
    pub recipe_id: Option<&'a str>,
    /// Optional profile identity.
    pub profile_id: Option<&'a str>,
    /// Validated parameter binding labels.
    pub parameter_bindings: &'a [&'a str],
    /// Requested tool ceilings.
    pub tool_ceilings: &'a [&'a str],
    /// Workspace scope label.
    pub workspace_scope: &'a str,
    /// Model profile label.
    pub model_profile: &'a str,
    /// Timeout class label.
    pub timeout_class: &'a str,
    /// Maximum child turns.
    pub max_turns: u32,
    /// Return mode label.
    pub return_mode: &'a str,
    /// Summary schema label.
    pub summary_schema: &'a str,
    /// Delegation source label.
    pub delegation_source: &'a str,
    /// Whether this request is recursive from another subagent.
    pub recursive: bool,
    /// Recipe trust receipt when a recipe is referenced.
    pub recipe_trust: Option<&'a crate::RecipeTrustReceipt>,
    /// Whether automation source was admitted.
    pub automation_admitted: bool,
    /// Optional external subagent descriptor.
    pub external: Option<ExternalSubagentDescriptorDraft<'a>>,
}

/// Validated recipe-backed subagent delegation request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubagentDelegation {
    id: Text,
    role: Text,
    task: Text,
    recipe_id: Option<Text>,
    profile_id: Option<Text>,
    parameter_bindings: Collection<Text>,
    tool_ceilings: Collection<Text>,
    workspace_scope: Text,
    model_profile: Text,
    timeout_class: Text,
    max_turns: u32,
    return_mode: SubagentReturnMode,
    summary_schema: Text,
    delegation_source: SubagentDelegationSource,
    external: Option<ExternalSubagentDescriptor>,
}

impl SubagentDelegation {
    /// Validates a subagent delegation request without starting a child run.
    ///
    /// # Errors
    ///
    /// Returns [`SubagentError`] when delegation is implicit, recursive,
    /// untrusted, unbounded, or requests ambient external-agent authority.
    pub fn validate(draft: &SubagentDelegationDraft<'_>) -> Result<Self, SubagentError> {
        let delegation_source = SubagentDelegationSource::parse(draft.delegation_source)?;
        validate_delegation_admission(draft, delegation_source)?;
        let id = required_text(ID_FIELD, draft.id)?;
        let role = required_text(ROLE_FIELD, draft.role)?;
        let task = required_text(TASK_FIELD, draft.task)?;
        let recipe_id = optional_text(RECIPE_FIELD, draft.recipe_id)?;
        let profile_id = optional_text(PROFILE_FIELD, draft.profile_id)?;
        let parameter_bindings = text_collection(PARAMETER_FIELD, draft.parameter_bindings, false)?;
        let tool_ceilings = text_collection(TOOL_FIELD, draft.tool_ceilings, true)?;
        let workspace_scope = required_text(WORKSPACE_FIELD, draft.workspace_scope)?;
        let model_profile = required_text(MODEL_FIELD, draft.model_profile)?;
        let timeout_class = required_text(TIMEOUT_FIELD, draft.timeout_class)?;
        if draft.max_turns < MIN_TURN_COUNT {
            return Err(SubagentError::InvalidTurnCeiling);
        }
        let return_mode = SubagentReturnMode::parse(draft.return_mode)?;
        let summary_schema = required_text(SUMMARY_SCHEMA_FIELD, draft.summary_schema)?;
        let external = draft
            .external
            .as_ref()
            .map(ExternalSubagentDescriptor::validate)
            .transpose()?;
        Ok(Self {
            id,
            role,
            task,
            recipe_id,
            profile_id,
            parameter_bindings,
            tool_ceilings,
            workspace_scope,
            model_profile,
            timeout_class,
            max_turns: draft.max_turns,
            return_mode,
            summary_schema,
            delegation_source,
            external,
        })
    }

    /// Returns the child run identity.
    #[must_use]
    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    /// Returns the role label.
    #[must_use]
    pub fn role(&self) -> &str {
        self.role.as_str()
    }

    /// Returns the child task summary.
    #[must_use]
    pub fn task(&self) -> &str {
        self.task.as_str()
    }

    /// Returns the optional recipe identity.
    #[must_use]
    pub fn recipe_id(&self) -> Option<&str> {
        self.recipe_id.as_deref()
    }

    /// Returns the optional profile identity.
    #[must_use]
    pub fn profile_id(&self) -> Option<&str> {
        self.profile_id.as_deref()
    }

    /// Returns parameter binding labels.
    #[must_use]
    pub fn parameter_bindings(&self) -> &[Text] {
        self.parameter_bindings.as_slice()
    }

    /// Returns tool ceiling labels.
    #[must_use]
    pub fn tool_ceilings(&self) -> &[Text] {
        self.tool_ceilings.as_slice()
    }

    /// Returns the workspace scope label.
    #[must_use]
    pub fn workspace_scope(&self) -> &str {
        self.workspace_scope.as_str()
    }

    /// Returns the model profile label.
    #[must_use]
    pub fn model_profile(&self) -> &str {
        self.model_profile.as_str()
    }

    /// Returns the timeout class label.
    #[must_use]
    pub fn timeout_class(&self) -> &str {
        self.timeout_class.as_str()
    }

    /// Returns the maximum turn count.
    #[must_use]
    pub const fn max_turns(&self) -> u32 {
        self.max_turns
    }

    /// Returns the return mode.
    #[must_use]
    pub const fn return_mode(&self) -> SubagentReturnMode {
        self.return_mode
    }

    /// Returns the summary schema label.
    #[must_use]
    pub fn summary_schema(&self) -> &str {
        self.summary_schema.as_str()
    }

    /// Returns the delegation source.
    #[must_use]
    pub const fn delegation_source(&self) -> SubagentDelegationSource {
        self.delegation_source
    }

    /// Returns the external subagent descriptor, when any.
    #[must_use]
    pub const fn external(&self) -> Option<&ExternalSubagentDescriptor> {
        self.external.as_ref()
    }
}

/// Capability intersection input for a subagent request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubagentCapabilityInput<'a> {
    /// Capabilities allowed by parent policy.
    pub parent_policy: &'a [&'a str],
    /// Capabilities allowed by recipe or profile ceilings.
    pub recipe_or_profile: &'a [&'a str],
    /// Capabilities requested by the subagent delegation.
    pub request: &'a [&'a str],
    /// Capabilities allowed by workspace scope.
    pub workspace_scope: &'a [&'a str],
    /// Capabilities admitted by host policy.
    pub host_admission: &'a [&'a str],
}

/// Denied subagent capability.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubagentCapabilityDenial {
    capability: Text,
}

impl SubagentCapabilityDenial {
    /// Returns the denied capability label.
    #[must_use]
    pub fn capability(&self) -> &str {
        self.capability.as_str()
    }
}

/// Effective subagent capability intersection result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubagentCapabilityResolution {
    effective: Collection<Text>,
    denied: Collection<SubagentCapabilityDenial>,
}

impl SubagentCapabilityResolution {
    /// Returns capabilities admitted by every input set.
    #[must_use]
    pub fn effective(&self) -> &[Text] {
        self.effective.as_slice()
    }

    /// Returns requested capabilities denied by at least one input set.
    #[must_use]
    pub fn denied(&self) -> &[SubagentCapabilityDenial] {
        self.denied.as_slice()
    }
}

/// Resolves effective subagent capabilities as a pure intersection.
///
/// # Errors
///
/// Returns [`SubagentError`] when a capability label is malformed or duplicated.
pub fn resolve_subagent_capabilities(
    input: SubagentCapabilityInput<'_>,
) -> Result<SubagentCapabilityResolution, SubagentError> {
    let parent_policy = text_collection(CAPABILITY_FIELD, input.parent_policy, true)?;
    let recipe_or_profile = text_collection(CAPABILITY_FIELD, input.recipe_or_profile, true)?;
    let request = text_collection(CAPABILITY_FIELD, input.request, true)?;
    let workspace_scope = text_collection(CAPABILITY_FIELD, input.workspace_scope, true)?;
    let host_admission = text_collection(CAPABILITY_FIELD, input.host_admission, true)?;
    let mut effective = Collection::new();
    let mut denied = Collection::new();
    for capability in request {
        if contains_all(
            capability.as_str(),
            &parent_policy,
            &recipe_or_profile,
            &workspace_scope,
            &host_admission,
        ) {
            effective.push(capability);
        } else {
            denied.push(SubagentCapabilityDenial { capability });
        }
    }
    Ok(SubagentCapabilityResolution { effective, denied })
}

/// Untrusted subagent handoff data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubagentHandoffDraft<'a> {
    /// Return mode label.
    pub return_mode: &'a str,
    /// Child status label.
    pub status: &'a str,
    /// Bounded child summary.
    pub summary: &'a str,
    /// Optional bounded detail reference.
    pub detail_ref: Option<&'a str>,
    /// Observed detail character count.
    pub detail_chars: u32,
    /// Configured detail character bound.
    pub max_detail_chars: u32,
}

/// Validated parent handoff record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubagentHandoff {
    return_mode: SubagentReturnMode,
    status: Text,
    summary: Text,
    detail_ref: Option<Text>,
}

impl SubagentHandoff {
    /// Returns the return mode.
    #[must_use]
    pub const fn return_mode(&self) -> SubagentReturnMode {
        self.return_mode
    }

    /// Returns the child status label.
    #[must_use]
    pub fn status(&self) -> &str {
        self.status.as_str()
    }

    /// Returns the bounded summary.
    #[must_use]
    pub fn summary(&self) -> &str {
        self.summary.as_str()
    }

    /// Returns the imported detail reference, when allowed by mode and bounds.
    #[must_use]
    pub fn detail_ref(&self) -> Option<&str> {
        self.detail_ref.as_deref()
    }
}

/// Builds a parent handoff record according to return-mode bounds.
///
/// # Errors
///
/// Returns [`SubagentError`] when status or summary is malformed, full detail is
/// unbounded, or full detail exceeds its configured bound.
pub fn build_subagent_handoff(
    draft: &SubagentHandoffDraft<'_>,
) -> Result<SubagentHandoff, SubagentError> {
    let return_mode = SubagentReturnMode::parse(draft.return_mode)?;
    let status = required_text(HANDOFF_STATUS_FIELD, draft.status)?;
    let summary = required_text(HANDOFF_SUMMARY_FIELD, draft.summary)?;
    let detail_ref = match return_mode {
        SubagentReturnMode::SummaryOnly | SubagentReturnMode::StatusPlusSummary => None,
        SubagentReturnMode::FullDetailWithBounds => {
            if draft.max_detail_chars < MIN_DETAIL_BOUND_CHARS
                || draft.detail_chars > draft.max_detail_chars
            {
                return Err(SubagentError::DetailBoundsExceeded);
            }
            Some(required_text(
                HANDOFF_DETAIL_FIELD,
                draft
                    .detail_ref
                    .ok_or(SubagentError::MissingDetailReference)?,
            )?)
        }
    };
    Ok(SubagentHandoff {
        return_mode,
        status,
        summary,
        detail_ref,
    })
}

/// Child outcome kind used for aggregation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SubagentOutcomeKind {
    /// Child completed successfully.
    Success,
    /// Child failed.
    Failure,
    /// Child timed out.
    Timeout,
    /// Child was cancelled.
    Cancellation,
    /// Child returned a malformed summary.
    MalformedSummary,
    /// Child requested a denied tool.
    DeniedTool,
    /// Aggregate-only partial parallel success status.
    PartialParallelSuccess,
    /// Child or aggregate encountered a write conflict.
    WriteConflict,
}

impl SubagentOutcomeKind {
    /// Parses a stable outcome kind label.
    ///
    /// # Errors
    ///
    /// Returns [`SubagentError`] when the label is empty or unsupported.
    pub fn parse(value: &str) -> Result<Self, SubagentError> {
        parse_value(OUTCOME_KIND_FIELD, value, Self::from_normalized)
    }

    fn from_normalized(value: &str) -> Option<Self> {
        match value {
            OUTCOME_SUCCESS => Some(Self::Success),
            OUTCOME_FAILURE => Some(Self::Failure),
            OUTCOME_TIMEOUT => Some(Self::Timeout),
            OUTCOME_CANCELLATION => Some(Self::Cancellation),
            OUTCOME_MALFORMED_SUMMARY => Some(Self::MalformedSummary),
            OUTCOME_DENIED_TOOL => Some(Self::DeniedTool),
            OUTCOME_PARTIAL_PARALLEL_SUCCESS => Some(Self::PartialParallelSuccess),
            OUTCOME_WRITE_CONFLICT => Some(Self::WriteConflict),
            _ => None,
        }
    }
}

/// Untrusted child outcome fact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SubagentOutcomeDraft<'a> {
    /// Child run identity.
    pub child_id: &'a str,
    /// Outcome kind label.
    pub kind: &'a str,
    /// Whether the child summary passed schema validation.
    pub summary_valid: bool,
    /// Whether the child attempted or encountered a parallel write conflict.
    pub write_conflict: bool,
}

/// Aggregated child outcome report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubagentAggregateReport {
    status: SubagentOutcomeKind,
    success_count: u32,
    failure_count: u32,
    write_conflict: bool,
}

impl SubagentAggregateReport {
    /// Returns aggregate status.
    #[must_use]
    pub const fn status(&self) -> SubagentOutcomeKind {
        self.status
    }

    /// Returns successful child count.
    #[must_use]
    pub const fn success_count(&self) -> u32 {
        self.success_count
    }

    /// Returns failed child count.
    #[must_use]
    pub const fn failure_count(&self) -> u32 {
        self.failure_count
    }

    /// Returns whether a write conflict was observed.
    #[must_use]
    pub const fn write_conflict(&self) -> bool {
        self.write_conflict
    }
}

/// Aggregates subagent outcomes without treating failures as success.
///
/// # Errors
///
/// Returns [`SubagentError`] when the outcome list is empty or fields are malformed.
pub fn aggregate_subagent_outcomes(
    drafts: &[SubagentOutcomeDraft<'_>],
) -> Result<SubagentAggregateReport, SubagentError> {
    if drafts.is_empty() {
        return Err(SubagentError::EmptyCollection {
            field: ID_FIELD.as_str(),
        });
    }
    let mut success_count = 0_u32;
    let mut failure_count = 0_u32;
    let mut write_conflict = false;
    for draft in drafts {
        required_text(ID_FIELD, draft.child_id)?;
        let kind = SubagentOutcomeKind::parse(draft.kind)?;
        if kind == SubagentOutcomeKind::Success && draft.summary_valid && !draft.write_conflict {
            success_count = success_count.saturating_add(MIN_TURN_COUNT);
        } else {
            failure_count = failure_count.saturating_add(MIN_TURN_COUNT);
        }
        write_conflict = write_conflict
            || draft.write_conflict
            || matches!(kind, SubagentOutcomeKind::WriteConflict);
    }
    let status = classify_aggregate(success_count, failure_count, write_conflict);
    Ok(SubagentAggregateReport {
        status,
        success_count,
        failure_count,
        write_conflict,
    })
}

fn classify_aggregate(
    success_count: u32,
    failure_count: u32,
    write_conflict: bool,
) -> SubagentOutcomeKind {
    if write_conflict {
        return SubagentOutcomeKind::WriteConflict;
    }
    if success_count > 0 && failure_count > 0 {
        return SubagentOutcomeKind::PartialParallelSuccess;
    }
    if success_count > 0 {
        return SubagentOutcomeKind::Success;
    }
    SubagentOutcomeKind::Failure
}

fn validate_delegation_admission(
    draft: &SubagentDelegationDraft<'_>,
    source: SubagentDelegationSource,
) -> Result<(), SubagentError> {
    if source == SubagentDelegationSource::Automatic {
        return Err(SubagentError::ImplicitDelegationDenied);
    }
    if draft.recursive {
        return Err(SubagentError::RecursiveDelegationDenied);
    }
    if source == SubagentDelegationSource::AdmittedAutomation && !draft.automation_admitted {
        return Err(SubagentError::AutomationDelegationDenied);
    }
    if draft.recipe_id.is_some() {
        let Some(receipt) = draft.recipe_trust else {
            return Err(SubagentError::UntrustedRecipeDenied);
        };
        if receipt.status() != crate::RecipeTrustStatus::Admitted {
            return Err(SubagentError::UntrustedRecipeDenied);
        }
    }
    Ok(())
}

fn parse_value<T>(
    field_name: crate::validation::FieldName,
    value: &str,
    parser: impl FnOnce(&str) -> Option<T>,
) -> Result<T, SubagentError> {
    let normalized = crate::validation::normalized_adapter_field(field_name, value)?;
    parser(normalized.as_str()).ok_or(SubagentError::UnsupportedValue {
        field: field_name.as_str(),
        value: normalized,
    })
}

fn required_text(
    field_name: crate::validation::FieldName,
    value: &str,
) -> Result<Text, SubagentError> {
    crate::validation::normalized_adapter_field(field_name, value).map_err(SubagentError::from)
}

fn optional_text(
    field_name: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<Option<Text>, SubagentError> {
    value
        .map(|text| required_text(field_name, text))
        .transpose()
}

fn text_collection(
    field_name: crate::validation::FieldName,
    values: &[&str],
    required: bool,
) -> Result<Collection<Text>, SubagentError> {
    if required && values.is_empty() {
        return Err(SubagentError::EmptyCollection {
            field: field_name.as_str(),
        });
    }
    let mut normalized = Collection::with_capacity(values.len());
    for value in values {
        let text = required_text(field_name, value)?;
        if normalized.iter().any(|existing| existing == &text) {
            return Err(SubagentError::DuplicateValue {
                field: field_name.as_str(),
                value: text,
            });
        }
        normalized.push(text);
    }
    Ok(normalized)
}

fn contains_all(
    capability: &str,
    parent_policy: &[Text],
    recipe_or_profile: &[Text],
    workspace_scope: &[Text],
    host_admission: &[Text],
) -> bool {
    contains_text(parent_policy, capability)
        && contains_text(recipe_or_profile, capability)
        && contains_text(workspace_scope, capability)
        && contains_text(host_admission, capability)
}

fn contains_text(values: &[Text], needle: &str) -> bool {
    values.iter().any(|value| value.as_str() == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    const CHILD_ID: &str = "child-review";
    const ROLE: &str = "reviewer";
    const TASK: &str = "Review adapter contract changes.";
    const RECIPE_ID: &str = "goose.recipe.workflow";
    const PROFILE_ID: &str = "review-profile";
    const PARAM_TOPIC: &str = "topic=adapter contracts";
    const TOOL_READ: &str = "tool.read";
    const TOOL_WRITE: &str = "tool.write";
    const SECRET_READ: &str = "secret.read";
    const WORKSPACE_READONLY: &str = "readonly-workspace";
    const MODEL_PROFILE: &str = "standard-reasoning";
    const TIMEOUT: &str = "standard";
    const SUMMARY_SCHEMA: &str = "review-summary";
    const CONTENT_HASH: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const MAX_TURNS: u32 = 4;
    const ZERO_TURNS: u32 = 0;
    const DETAIL_CHARS: u32 = 10;
    const MAX_DETAIL_CHARS: u32 = 20;
    const EXCESSIVE_DETAIL_CHARS: u32 = 21;

    fn admitted_recipe() -> crate::RecipeTrustReceipt {
        crate::RecipeTrustReceipt::new(&crate::RecipeProvenanceDraft {
            source_kind: "project",
            source_label: "config/recipe-workflows.ncl",
            content_hash: CONTENT_HASH,
            admitted: true,
        })
        .expect("recipe trust should parse")
    }

    fn pending_recipe() -> crate::RecipeTrustReceipt {
        crate::RecipeTrustReceipt::new(&crate::RecipeProvenanceDraft {
            source_kind: "project",
            source_label: "config/recipe-workflows.ncl",
            content_hash: CONTENT_HASH,
            admitted: false,
        })
        .expect("recipe trust should parse")
    }

    fn valid_delegation<'a>(recipe: &'a crate::RecipeTrustReceipt) -> SubagentDelegationDraft<'a> {
        SubagentDelegationDraft {
            id: CHILD_ID,
            role: ROLE,
            task: TASK,
            recipe_id: Some(RECIPE_ID),
            profile_id: Some(PROFILE_ID),
            parameter_bindings: &[PARAM_TOPIC],
            tool_ceilings: &[TOOL_READ],
            workspace_scope: WORKSPACE_READONLY,
            model_profile: MODEL_PROFILE,
            timeout_class: TIMEOUT,
            max_turns: MAX_TURNS,
            return_mode: RETURN_STATUS_PLUS_SUMMARY,
            summary_schema: SUMMARY_SCHEMA,
            delegation_source: SOURCE_USER,
            recursive: false,
            recipe_trust: Some(recipe),
            automation_admitted: false,
            external: None,
        }
    }

    #[test]
    fn recipe_backed_delegation_accepts_explicit_admitted_request() {
        let recipe = admitted_recipe();
        let delegation = SubagentDelegation::validate(&valid_delegation(&recipe))
            .expect("delegation should validate");

        assert_eq!(delegation.id(), CHILD_ID);
        assert_eq!(delegation.recipe_id(), Some(RECIPE_ID));
        assert_eq!(delegation.parameter_bindings()[0], PARAM_TOPIC);
        assert_eq!(delegation.tool_ceilings()[0], TOOL_READ);
        assert_eq!(
            delegation.return_mode(),
            SubagentReturnMode::StatusPlusSummary
        );
    }

    #[test]
    fn explicit_delegation_denies_automatic_recursive_untrusted_and_unapproved() {
        let admitted = admitted_recipe();
        let pending = pending_recipe();

        assert_eq!(
            SubagentDelegation::validate(&SubagentDelegationDraft {
                delegation_source: SOURCE_AUTOMATIC,
                ..valid_delegation(&admitted)
            }),
            Err(SubagentError::ImplicitDelegationDenied)
        );
        assert_eq!(
            SubagentDelegation::validate(&SubagentDelegationDraft {
                recursive: true,
                ..valid_delegation(&admitted)
            }),
            Err(SubagentError::RecursiveDelegationDenied)
        );
        assert_eq!(
            SubagentDelegation::validate(&SubagentDelegationDraft {
                recipe_trust: Some(&pending),
                ..valid_delegation(&pending)
            }),
            Err(SubagentError::UntrustedRecipeDenied)
        );
        assert_eq!(
            SubagentDelegation::validate(&SubagentDelegationDraft {
                delegation_source: SOURCE_ADMITTED_AUTOMATION,
                automation_admitted: false,
                ..valid_delegation(&admitted)
            }),
            Err(SubagentError::AutomationDelegationDenied)
        );
        assert_eq!(
            SubagentDelegation::validate(&SubagentDelegationDraft {
                max_turns: ZERO_TURNS,
                ..valid_delegation(&admitted)
            }),
            Err(SubagentError::InvalidTurnCeiling)
        );
    }

    #[test]
    fn capability_intersection_allows_only_everywhere_admitted_tools() {
        let resolution = resolve_subagent_capabilities(SubagentCapabilityInput {
            parent_policy: &[TOOL_READ, TOOL_WRITE, SECRET_READ],
            recipe_or_profile: &[TOOL_READ],
            request: &[TOOL_READ, TOOL_WRITE, SECRET_READ],
            workspace_scope: &[TOOL_READ],
            host_admission: &[TOOL_READ],
        })
        .expect("capabilities should parse");

        assert_eq!(resolution.effective(), &[Text::from(TOOL_READ)]);
        assert_eq!(resolution.denied().len(), 2);
        assert!(
            resolution
                .denied()
                .iter()
                .any(|denial| denial.capability() == TOOL_WRITE)
        );
        assert!(
            resolution
                .denied()
                .iter()
                .any(|denial| denial.capability() == SECRET_READ)
        );
    }

    #[test]
    fn return_modes_bound_parent_handoff_detail() {
        let summary_only = build_subagent_handoff(&SubagentHandoffDraft {
            return_mode: RETURN_SUMMARY_ONLY,
            status: OUTCOME_SUCCESS,
            summary: "bounded summary",
            detail_ref: Some("raw-log-ref"),
            detail_chars: EXCESSIVE_DETAIL_CHARS,
            max_detail_chars: MAX_DETAIL_CHARS,
        })
        .expect("summary-only ignores detail import");
        let full_detail = build_subagent_handoff(&SubagentHandoffDraft {
            return_mode: RETURN_FULL_DETAIL_BOUNDED,
            status: OUTCOME_SUCCESS,
            summary: "bounded summary",
            detail_ref: Some("detail-ref"),
            detail_chars: DETAIL_CHARS,
            max_detail_chars: MAX_DETAIL_CHARS,
        })
        .expect("bounded detail should import");

        assert_eq!(summary_only.detail_ref(), None);
        assert_eq!(full_detail.detail_ref(), Some("detail-ref"));
        assert_eq!(
            build_subagent_handoff(&SubagentHandoffDraft {
                return_mode: RETURN_FULL_DETAIL_BOUNDED,
                status: OUTCOME_SUCCESS,
                summary: "bounded summary",
                detail_ref: Some("detail-ref"),
                detail_chars: EXCESSIVE_DETAIL_CHARS,
                max_detail_chars: MAX_DETAIL_CHARS,
            }),
            Err(SubagentError::DetailBoundsExceeded)
        );
    }

    #[test]
    fn external_subagents_require_injected_descriptors() {
        let descriptor = ExternalSubagentDescriptor::validate(&ExternalSubagentDescriptorDraft {
            protocol: PROTOCOL_MCP,
            identity: "mcp-reviewer",
            service_label: "host-mcp-service",
            declared_tools: &[TOOL_READ],
            credential_label: None,
            ambient_cli: false,
            inherited_credentials: false,
        })
        .expect("descriptor should validate");

        assert_eq!(descriptor.protocol(), ExternalSubagentProtocol::Mcp);
        assert_eq!(descriptor.declared_tools()[0], TOOL_READ);
        assert_eq!(
            ExternalSubagentDescriptor::validate(&ExternalSubagentDescriptorDraft {
                ambient_cli: true,
                protocol: PROTOCOL_ACP,
                identity: "ambient",
                service_label: "host-acp-service",
                declared_tools: &[TOOL_READ],
                credential_label: None,
                inherited_credentials: false,
            }),
            Err(SubagentError::AmbientExternalDiscoveryDenied)
        );
        assert_eq!(
            ExternalSubagentDescriptor::validate(&ExternalSubagentDescriptorDraft {
                inherited_credentials: true,
                protocol: PROTOCOL_ACP,
                identity: "ambient",
                service_label: "host-acp-service",
                declared_tools: &[TOOL_READ],
                credential_label: None,
                ambient_cli: false,
            }),
            Err(SubagentError::InheritedCredentialDenied)
        );
    }

    #[test]
    fn aggregation_keeps_failures_and_write_conflicts_visible() {
        let partial = aggregate_subagent_outcomes(&[
            SubagentOutcomeDraft {
                child_id: "child-ok",
                kind: OUTCOME_SUCCESS,
                summary_valid: true,
                write_conflict: false,
            },
            SubagentOutcomeDraft {
                child_id: "child-timeout",
                kind: OUTCOME_TIMEOUT,
                summary_valid: false,
                write_conflict: false,
            },
        ])
        .expect("outcomes should aggregate");
        let conflict = aggregate_subagent_outcomes(&[SubagentOutcomeDraft {
            child_id: "child-conflict",
            kind: OUTCOME_WRITE_CONFLICT,
            summary_valid: false,
            write_conflict: true,
        }])
        .expect("conflict outcome should aggregate");

        assert_eq!(
            partial.status(),
            SubagentOutcomeKind::PartialParallelSuccess
        );
        assert_eq!(partial.success_count(), MIN_TURN_COUNT);
        assert_eq!(partial.failure_count(), MIN_TURN_COUNT);
        assert_eq!(conflict.status(), SubagentOutcomeKind::WriteConflict);
        assert!(conflict.write_conflict());
    }
}
