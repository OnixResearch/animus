//! Data-only evaluation contracts for Goose MCP Apps-style UI metadata.

use alloc::vec::Vec;

type Text = alloc::string::String;
type Collection<T> = alloc::vec::Vec<T>;

const RESOURCE_URI_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_app_resource_uri");
const MIME_PROFILE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_app_mime_profile");
const TITLE_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_app_title");
const CSP_DOMAIN_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_app_csp_domain");
const PERMISSION_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_app_permission");
const HOST_CONTEXT_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_app_host_context");
const CALLBACK_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_app_callback");
const SIZE_POLICY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_app_size_policy");
const LIFECYCLE_POLICY_FIELD: crate::validation::FieldName =
    crate::validation::FieldName::new("mcp_app_lifecycle_policy");

const SIZE_POLICY_BOUNDED: &str = "bounded";
const SIZE_POLICY_HOST_FIT: &str = "host-fit";
const LIFECYCLE_POLICY_PASSIVE: &str = "passive";
const LIFECYCLE_POLICY_BOUNDED_CALLBACKS: &str = "bounded-callbacks";
const LIFECYCLE_POLICY_UNBOUNDED_CALLBACKS: &str = "unbounded-callbacks";
const WILDCARD_DOMAIN_MARKER: char = '*';
const INSECURE_HTTP_PREFIX: &str = "http://";
const CAMERA_PERMISSION: &str = "camera";
const MICROPHONE_PERMISSION: &str = "microphone";
const GEOLOCATION_PERMISSION: &str = "geolocation";
const CLIPBOARD_WRITE_PERMISSION: &str = "clipboard-write";
const SCREEN_CAPTURE_PERMISSION: &str = "screen-capture";
const DIRECT_CHAT_CALLBACK: &str = "direct-chat-message";
const APPEND_MODEL_MESSAGE_CALLBACK: &str = "append-model-message";
const AMBIENT_HOST_CONTROL_CONTEXT: &str = "ambient-host-control";
const HOST_COMMAND_CONTEXT: &str = "host-command";
const SHELL_CONTEXT: &str = "shell";
const MAX_CSP_DOMAIN_ITEMS: usize = 16;
const MAX_PERMISSION_ITEMS: usize = 16;
const MAX_HOST_CONTEXT_ITEMS: usize = 16;
const MAX_CALLBACK_ITEMS: usize = 16;

const _: () = assert!(MAX_CSP_DOMAIN_ITEMS > 0);
const _: () = assert!(MAX_PERMISSION_ITEMS > 0);
const _: () = assert!(MAX_HOST_CONTEXT_ITEMS > 0);
const _: () = assert!(MAX_CALLBACK_ITEMS > 0);

/// Descriptor-side effects that MCP App metadata validation explicitly does not perform.
pub const PROHIBITED_MCP_APP_VALIDATION_EFFECTS: &[&str] = &[
    "HTML rendering",
    "JavaScript execution",
    "remote resource loading",
    "browser permission requests",
    "chat callback dispatch",
    "host control execution",
];

/// Matrix axes used to evaluate MCP Apps-style UI metadata.
pub const MCP_APP_EVALUATION_AXES: &[EvaluationAxis] = &[
    EvaluationAxis::ResourceIdentity,
    EvaluationAxis::MimeProfile,
    EvaluationAxis::CspDomains,
    EvaluationAxis::PermissionRequests,
    EvaluationAxis::HostContextSync,
    EvaluationAxis::CallbackChannels,
    EvaluationAxis::Sizing,
    EvaluationAxis::LifecycleBounds,
    EvaluationAxis::AdoptionRisk,
];

/// Evaluation axes required for MCP Apps-style surface review.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvaluationAxis {
    /// UI resource identity is named without loading the resource.
    ResourceIdentity,
    /// MIME/profile tags are recorded as data.
    MimeProfile,
    /// Content security policy domains are classified.
    CspDomains,
    /// Browser or device permission requests are classified.
    PermissionRequests,
    /// Host theme/context synchronization labels are classified.
    HostContextSync,
    /// Callback channel labels are classified.
    CallbackChannels,
    /// Preferred sizing policy is classified.
    Sizing,
    /// Lifecycle and callback bounds are classified.
    LifecycleBounds,
    /// Adoption decision and risk reason are recorded.
    AdoptionRisk,
}

/// Admission state for a future host-policy gate.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum PolicyAdmission {
    /// Host policy has not admitted the behavior.
    #[default]
    Denied,
    /// Host policy explicitly admits the behavior.
    Admitted,
}

impl PolicyAdmission {
    const fn is_admitted(self) -> bool {
        matches!(self, Self::Admitted)
    }
}

/// Borrowed policy-admission flags for evaluating UI metadata.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PolicyAdmissionDraft {
    /// Whether remote resource domains are admitted by host policy.
    pub remote_domains: PolicyAdmission,
    /// Whether browser or device permissions are admitted by host policy.
    pub browser_permissions: PolicyAdmission,
    /// Whether bounded host callbacks are admitted by host policy.
    pub host_callbacks: PolicyAdmission,
    /// Whether direct model-visible chat callbacks are admitted by host policy.
    pub direct_chat_messages: PolicyAdmission,
}

/// Borrowed MCP Apps-style UI metadata before validation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DescriptorDraft<'a> {
    /// Optional UI resource URI or stable resource identifier.
    pub resource_uri: Option<&'a str>,
    /// Optional MIME/profile tag for the resource.
    pub mime_profile: Option<&'a str>,
    /// Optional human-facing title.
    pub title: Option<&'a str>,
    /// Content security policy domain labels.
    pub csp_domains: &'a [&'a str],
    /// Browser or device permission request labels.
    pub permission_requests: &'a [&'a str],
    /// Host-context synchronization labels.
    pub host_context_sync: &'a [&'a str],
    /// UI-to-host callback channel labels.
    pub callback_channels: &'a [&'a str],
    /// Optional preferred sizing policy.
    pub size_policy: Option<&'a str>,
    /// Optional lifecycle/callback bound policy.
    pub lifecycle_policy: Option<&'a str>,
    /// Explicit policy admissions used for classification.
    pub policy_admission: PolicyAdmissionDraft,
    /// Whether the descriptor claims validation should execute UI behavior.
    pub claims_runtime_execution: bool,
}

/// Validated data-only UI metadata descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Descriptor {
    resource_uri: Text,
    mime_profile: Text,
    title: Text,
    csp_domains: Collection<Text>,
    permission_requests: Collection<Text>,
    host_context_sync: Collection<Text>,
    callback_channels: Collection<Text>,
    size_policy: SizePolicy,
    lifecycle_policy: LifecyclePolicy,
    policy_admission: PolicyAdmissionDraft,
    claims_runtime_execution: bool,
}

impl Descriptor {
    /// Validates a descriptor and returns its data-only evaluation classification.
    ///
    /// # Errors
    ///
    /// Returns [`EvaluationError`] when required fields are missing, field text is
    /// malformed, collection ceilings are exceeded, or size/lifecycle labels are
    /// unsupported.
    pub fn evaluate(draft: &DescriptorDraft<'_>) -> Result<Evaluation, EvaluationError> {
        let descriptor = Self::validate(draft)?;
        let decision = classify_descriptor(&descriptor);
        Ok(Evaluation {
            descriptor,
            decision,
        })
    }

    /// Validates descriptor-shaped metadata without executing UI behavior.
    ///
    /// # Errors
    ///
    /// Returns [`EvaluationError`] when required fields are missing, field text is
    /// malformed, collection ceilings are exceeded, or size/lifecycle labels are
    /// unsupported.
    pub fn validate(draft: &DescriptorDraft<'_>) -> Result<Self, EvaluationError> {
        let resource_uri = required_text(RESOURCE_URI_FIELD, draft.resource_uri)?;
        let mime_profile = required_text(MIME_PROFILE_FIELD, draft.mime_profile)?;
        let title = required_text(TITLE_FIELD, draft.title)?;
        let csp_domains =
            normalize_collection(CSP_DOMAIN_FIELD, draft.csp_domains, MAX_CSP_DOMAIN_ITEMS)?;
        let permission_requests = normalize_collection(
            PERMISSION_FIELD,
            draft.permission_requests,
            MAX_PERMISSION_ITEMS,
        )?;
        let host_context_sync = normalize_collection(
            HOST_CONTEXT_FIELD,
            draft.host_context_sync,
            MAX_HOST_CONTEXT_ITEMS,
        )?;
        let callback_channels =
            normalize_collection(CALLBACK_FIELD, draft.callback_channels, MAX_CALLBACK_ITEMS)?;
        let size_text = required_text(SIZE_POLICY_FIELD, draft.size_policy)?;
        let lifecycle_text = required_text(LIFECYCLE_POLICY_FIELD, draft.lifecycle_policy)?;
        let size_policy = parse_size_policy(size_text.as_str()).ok_or_else(|| {
            EvaluationError::UnsupportedValue {
                field: SIZE_POLICY_FIELD.as_str(),
                value: size_text.clone(),
            }
        })?;
        let lifecycle_policy =
            parse_lifecycle_policy(lifecycle_text.as_str()).ok_or_else(|| {
                EvaluationError::UnsupportedValue {
                    field: LIFECYCLE_POLICY_FIELD.as_str(),
                    value: lifecycle_text,
                }
            })?;

        Ok(Self {
            resource_uri,
            mime_profile,
            title,
            csp_domains,
            permission_requests,
            host_context_sync,
            callback_channels,
            size_policy,
            lifecycle_policy,
            policy_admission: draft.policy_admission,
            claims_runtime_execution: draft.claims_runtime_execution,
        })
    }

    /// Returns the UI resource identity.
    #[must_use]
    pub fn resource_uri(&self) -> &str {
        self.resource_uri.as_str()
    }

    /// Returns the MIME/profile tag.
    #[must_use]
    pub fn mime_profile(&self) -> &str {
        self.mime_profile.as_str()
    }

    /// Returns the human-facing title.
    #[must_use]
    pub fn title(&self) -> &str {
        self.title.as_str()
    }

    /// Returns CSP domain labels.
    #[must_use]
    pub fn csp_domains(&self) -> &[Text] {
        self.csp_domains.as_slice()
    }

    /// Returns browser or device permission labels.
    #[must_use]
    pub fn permission_requests(&self) -> &[Text] {
        self.permission_requests.as_slice()
    }

    /// Returns host-context synchronization labels.
    #[must_use]
    pub fn host_context_sync(&self) -> &[Text] {
        self.host_context_sync.as_slice()
    }

    /// Returns callback channel labels.
    #[must_use]
    pub fn callback_channels(&self) -> &[Text] {
        self.callback_channels.as_slice()
    }

    /// Returns the sizing policy.
    #[must_use]
    pub const fn size_policy(&self) -> SizePolicy {
        self.size_policy
    }

    /// Returns the lifecycle policy.
    #[must_use]
    pub const fn lifecycle_policy(&self) -> LifecyclePolicy {
        self.lifecycle_policy
    }

    /// Returns policy-admission flags used for classification.
    #[must_use]
    pub const fn policy_admission(&self) -> PolicyAdmissionDraft {
        self.policy_admission
    }
}

/// Preferred sizing policy for a future UI host.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SizePolicy {
    /// Descriptor declares bounded size hints only.
    Bounded,
    /// Descriptor asks a future host to fit content inside host bounds.
    HostFit,
}

impl SizePolicy {
    /// Returns the stable descriptor label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bounded => SIZE_POLICY_BOUNDED,
            Self::HostFit => SIZE_POLICY_HOST_FIT,
        }
    }
}

/// Lifecycle policy for callbacks and UI lifetime.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum LifecyclePolicy {
    /// Descriptor is passive and declares no callback channel.
    Passive,
    /// Descriptor declares host-mediated callbacks with bounded lifecycle.
    BoundedCallbacks,
    /// Descriptor attempts to keep callbacks unbounded.
    UnboundedCallbacks,
}

impl LifecyclePolicy {
    /// Returns the stable descriptor label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Passive => LIFECYCLE_POLICY_PASSIVE,
            Self::BoundedCallbacks => LIFECYCLE_POLICY_BOUNDED_CALLBACKS,
            Self::UnboundedCallbacks => LIFECYCLE_POLICY_UNBOUNDED_CALLBACKS,
        }
    }
}

/// Final data-only adoption classification for a descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdoptionDecision {
    /// Descriptor is safe as data-only metadata.
    DataOnlyAdoptable,
    /// Descriptor needs explicit host policy before future adoption.
    NeedsHostPolicy,
    /// Descriptor is rejected for default future behavior.
    RejectDefault,
}

/// Risk reason recorded with an adoption decision.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RiskReason {
    /// Descriptor contains safe metadata only.
    SafeMetadata,
    /// Remote domains require explicit host policy.
    RemoteDomainRequiresAdmission,
    /// Browser permissions require explicit host policy.
    BrowserPermissionRequiresAdmission,
    /// Host callbacks require explicit host policy.
    HostCallbackRequiresAdmission,
    /// Descriptor includes a wildcard or insecure CSP domain.
    BroadDomain,
    /// Descriptor requests sensitive device or clipboard permissions without policy admission.
    BroadPermission,
    /// Descriptor declares callbacks without lifecycle bounds.
    UnboundedCallback,
    /// Descriptor asks for ambient host control.
    AmbientHostControl,
    /// Descriptor asks to inject direct model-visible chat messages.
    DirectMessageInjection,
    /// Descriptor claims validation should execute UI runtime behavior.
    RuntimeExecutionClaim,
}

/// Data-only MCP Apps evaluation report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Evaluation {
    descriptor: Descriptor,
    decision: Decision,
}

impl Evaluation {
    /// Returns the validated descriptor.
    #[must_use]
    pub const fn descriptor(&self) -> &Descriptor {
        &self.descriptor
    }

    /// Returns the adoption decision.
    #[must_use]
    pub const fn adoption_decision(&self) -> AdoptionDecision {
        self.decision.adoption_decision
    }

    /// Returns the risk reason for the adoption decision.
    #[must_use]
    pub const fn risk_reason(&self) -> RiskReason {
        self.decision.risk_reason
    }

    /// Returns the explicit matrix axes applied to evaluations.
    #[must_use]
    pub const fn covered_axes() -> &'static [EvaluationAxis] {
        MCP_APP_EVALUATION_AXES
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Decision {
    adoption_decision: AdoptionDecision,
    risk_reason: RiskReason,
}

/// Errors raised while validating or classifying MCP Apps-style descriptors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvaluationError {
    /// Generic adapter field contract error.
    Contract(crate::AdapterContractError),
    /// Descriptor value was not one of the supported labels.
    UnsupportedValue {
        /// Boundary field name.
        field: &'static str,
        /// Unsupported normalized value.
        value: Text,
    },
    /// Descriptor collection exceeded its bounded ceiling.
    TooManyItems {
        /// Boundary collection field name.
        field: &'static str,
        /// Maximum supported item count.
        max_items: usize,
        /// Actual item count.
        actual_items: usize,
    },
}

impl From<crate::AdapterContractError> for EvaluationError {
    fn from(error: crate::AdapterContractError) -> Self {
        Self::Contract(error)
    }
}

fn classify_descriptor(descriptor: &Descriptor) -> Decision {
    if descriptor.claims_runtime_execution {
        return reject(RiskReason::RuntimeExecutionClaim);
    }
    if descriptor
        .csp_domains
        .iter()
        .any(|domain| is_broad_domain(domain))
    {
        return reject(RiskReason::BroadDomain);
    }
    if descriptor
        .host_context_sync
        .iter()
        .any(|context| is_ambient_host_control(context))
    {
        return reject(RiskReason::AmbientHostControl);
    }
    if descriptor
        .callback_channels
        .iter()
        .any(|callback| is_direct_chat_callback(callback))
        && !descriptor
            .policy_admission
            .direct_chat_messages
            .is_admitted()
    {
        return reject(RiskReason::DirectMessageInjection);
    }
    if descriptor
        .permission_requests
        .iter()
        .any(|permission| is_sensitive_permission(permission))
        && !descriptor
            .policy_admission
            .browser_permissions
            .is_admitted()
    {
        return reject(RiskReason::BroadPermission);
    }
    if !descriptor.callback_channels.is_empty()
        && descriptor.lifecycle_policy == LifecyclePolicy::UnboundedCallbacks
    {
        return reject(RiskReason::UnboundedCallback);
    }
    if !descriptor.csp_domains.is_empty()
        && !descriptor.policy_admission.remote_domains.is_admitted()
    {
        return needs_policy(RiskReason::RemoteDomainRequiresAdmission);
    }
    if !descriptor.permission_requests.is_empty()
        && !descriptor
            .policy_admission
            .browser_permissions
            .is_admitted()
    {
        return needs_policy(RiskReason::BrowserPermissionRequiresAdmission);
    }
    if !descriptor.callback_channels.is_empty()
        && !descriptor.policy_admission.host_callbacks.is_admitted()
    {
        return needs_policy(RiskReason::HostCallbackRequiresAdmission);
    }
    Decision {
        adoption_decision: AdoptionDecision::DataOnlyAdoptable,
        risk_reason: RiskReason::SafeMetadata,
    }
}

fn reject(risk_reason: RiskReason) -> Decision {
    Decision {
        adoption_decision: AdoptionDecision::RejectDefault,
        risk_reason,
    }
}

fn needs_policy(risk_reason: RiskReason) -> Decision {
    Decision {
        adoption_decision: AdoptionDecision::NeedsHostPolicy,
        risk_reason,
    }
}

fn required_text(
    field_name: crate::validation::FieldName,
    value: Option<&str>,
) -> Result<Text, EvaluationError> {
    match value {
        Some(text) => Ok(crate::validation::normalized_adapter_field(
            field_name, text,
        )?),
        None => Err(crate::AdapterContractError::EmptyField {
            field: field_name.as_str(),
        }
        .into()),
    }
}

fn normalize_collection(
    field_name: crate::validation::FieldName,
    values: &[&str],
    max_items: usize,
) -> Result<Collection<Text>, EvaluationError> {
    let actual_items = values.len();
    assert!(max_items > 0, "collection ceiling must be positive");
    if actual_items > max_items {
        return Err(EvaluationError::TooManyItems {
            field: field_name.as_str(),
            max_items,
            actual_items,
        });
    }
    let mut normalized = Vec::with_capacity(actual_items);
    for value in values {
        normalized.push(crate::validation::normalized_adapter_field(
            field_name, value,
        )?);
    }
    Ok(normalized)
}

fn parse_size_policy(value: &str) -> Option<SizePolicy> {
    match value {
        SIZE_POLICY_BOUNDED => Some(SizePolicy::Bounded),
        SIZE_POLICY_HOST_FIT => Some(SizePolicy::HostFit),
        _unsupported => None,
    }
}

fn parse_lifecycle_policy(value: &str) -> Option<LifecyclePolicy> {
    match value {
        LIFECYCLE_POLICY_PASSIVE => Some(LifecyclePolicy::Passive),
        LIFECYCLE_POLICY_BOUNDED_CALLBACKS => Some(LifecyclePolicy::BoundedCallbacks),
        LIFECYCLE_POLICY_UNBOUNDED_CALLBACKS => Some(LifecyclePolicy::UnboundedCallbacks),
        _unsupported => None,
    }
}

fn is_broad_domain(domain: &str) -> bool {
    let lower = domain.to_ascii_lowercase();
    domain.contains(WILDCARD_DOMAIN_MARKER) || lower.starts_with(INSECURE_HTTP_PREFIX)
}

fn is_sensitive_permission(permission: &str) -> bool {
    matches!(
        permission,
        CAMERA_PERMISSION
            | MICROPHONE_PERMISSION
            | GEOLOCATION_PERMISSION
            | CLIPBOARD_WRITE_PERMISSION
            | SCREEN_CAPTURE_PERMISSION
    )
}

fn is_direct_chat_callback(callback: &str) -> bool {
    matches!(
        callback,
        DIRECT_CHAT_CALLBACK | APPEND_MODEL_MESSAGE_CALLBACK
    )
}

fn is_ambient_host_control(context: &str) -> bool {
    matches!(
        context,
        AMBIENT_HOST_CONTROL_CONTEXT | HOST_COMMAND_CONTEXT | SHELL_CONTEXT
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAFE_RESOURCE: &str = "ui://tool/result/summary";
    const SAFE_MIME: &str = "text/html+skybridge";
    const SAFE_TITLE: &str = "Safe summary";
    const SAFE_DOMAIN: &str = "static.example.invalid";
    const SAFE_PERMISSION: &str = "clipboard-read";
    const SAFE_CONTEXT: &str = "theme";
    const SAFE_CALLBACK: &str = "bounded-selection";

    #[test]
    fn evaluation_matrix_accepts_safe_metadata_fixture() {
        let report = Descriptor::evaluate(&safe_descriptor()).expect("safe fixture should parse");

        assert_eq!(
            report.adoption_decision(),
            AdoptionDecision::DataOnlyAdoptable
        );
        assert_eq!(report.risk_reason(), RiskReason::SafeMetadata);
        assert_eq!(Evaluation::covered_axes(), MCP_APP_EVALUATION_AXES);
        assert_eq!(report.descriptor().resource_uri(), SAFE_RESOURCE);
        assert_eq!(report.descriptor().size_policy(), SizePolicy::Bounded);
        assert_eq!(
            report.descriptor().lifecycle_policy(),
            LifecyclePolicy::BoundedCallbacks
        );
    }

    #[test]
    fn evaluation_matrix_classifies_remote_domains_as_policy_gated() {
        let report = Descriptor::evaluate(&DescriptorDraft {
            policy_admission: PolicyAdmissionDraft {
                remote_domains: PolicyAdmission::Denied,
                browser_permissions: PolicyAdmission::Admitted,
                host_callbacks: PolicyAdmission::Admitted,
                direct_chat_messages: PolicyAdmission::Denied,
            },
            ..safe_descriptor()
        })
        .expect("policy-gated fixture should parse");

        assert_eq!(
            report.adoption_decision(),
            AdoptionDecision::NeedsHostPolicy
        );
        assert_eq!(
            report.risk_reason(),
            RiskReason::RemoteDomainRequiresAdmission
        );
    }

    #[test]
    fn evaluation_matrix_rejects_unsafe_fixtures() {
        let cases = [
            (
                DescriptorDraft {
                    csp_domains: &["*"],
                    ..safe_descriptor()
                },
                RiskReason::BroadDomain,
            ),
            (
                DescriptorDraft {
                    permission_requests: &[CAMERA_PERMISSION],
                    policy_admission: PolicyAdmissionDraft::default(),
                    ..safe_descriptor()
                },
                RiskReason::BroadPermission,
            ),
            (
                DescriptorDraft {
                    callback_channels: &[SAFE_CALLBACK],
                    lifecycle_policy: Some(LIFECYCLE_POLICY_UNBOUNDED_CALLBACKS),
                    ..safe_descriptor()
                },
                RiskReason::UnboundedCallback,
            ),
            (
                DescriptorDraft {
                    host_context_sync: &[AMBIENT_HOST_CONTROL_CONTEXT],
                    ..safe_descriptor()
                },
                RiskReason::AmbientHostControl,
            ),
            (
                DescriptorDraft {
                    callback_channels: &[DIRECT_CHAT_CALLBACK],
                    ..safe_descriptor()
                },
                RiskReason::DirectMessageInjection,
            ),
        ];

        for (draft, expected_reason) in cases {
            let report = Descriptor::evaluate(&draft).expect("unsafe fixture should parse");
            assert_eq!(report.adoption_decision(), AdoptionDecision::RejectDefault);
            assert_eq!(report.risk_reason(), expected_reason);
        }
    }

    #[test]
    fn descriptor_validation_is_data_only() {
        let report = Descriptor::evaluate(&DescriptorDraft {
            claims_runtime_execution: true,
            ..safe_descriptor()
        })
        .expect("runtime-claim fixture should parse as data");
        let rendered_effects = alloc::format!("{PROHIBITED_MCP_APP_VALIDATION_EFFECTS:?}");

        assert_eq!(report.adoption_decision(), AdoptionDecision::RejectDefault);
        assert_eq!(report.risk_reason(), RiskReason::RuntimeExecutionClaim);
        assert!(rendered_effects.contains("HTML rendering"));
        assert!(rendered_effects.contains("JavaScript execution"));
        assert!(rendered_effects.contains("chat callback dispatch"));
    }

    #[test]
    fn descriptor_validation_rejects_malformed_data() {
        let missing_uri = Descriptor::evaluate(&DescriptorDraft {
            resource_uri: None,
            ..safe_descriptor()
        })
        .expect_err("resource identity is required");
        let unsupported_size = Descriptor::evaluate(&DescriptorDraft {
            size_policy: Some("unbounded-window"),
            ..safe_descriptor()
        })
        .expect_err("unknown size policy is rejected");

        assert!(matches!(
            missing_uri,
            EvaluationError::Contract(crate::AdapterContractError::EmptyField { .. })
        ));
        assert!(matches!(
            unsupported_size,
            EvaluationError::UnsupportedValue { .. }
        ));
    }

    fn safe_descriptor() -> DescriptorDraft<'static> {
        DescriptorDraft {
            resource_uri: Some(SAFE_RESOURCE),
            mime_profile: Some(SAFE_MIME),
            title: Some(SAFE_TITLE),
            csp_domains: &[SAFE_DOMAIN],
            permission_requests: &[SAFE_PERMISSION],
            host_context_sync: &[SAFE_CONTEXT],
            callback_channels: &[SAFE_CALLBACK],
            size_policy: Some(SIZE_POLICY_BOUNDED),
            lifecycle_policy: Some(LIFECYCLE_POLICY_BOUNDED_CALLBACKS),
            policy_admission: PolicyAdmissionDraft {
                remote_domains: PolicyAdmission::Admitted,
                browser_permissions: PolicyAdmission::Admitted,
                host_callbacks: PolicyAdmission::Admitted,
                direct_chat_messages: PolicyAdmission::Denied,
            },
            claims_runtime_execution: false,
        }
    }
}
