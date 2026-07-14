const IROH_PROTOCOL_FIELD: &str = "iroh_protocol_id";
const IROH_NODE_FIELD: &str = "iroh_node_id";
const IROH_TOPIC_FIELD: &str = "iroh_topic_id";
const IROH_ADMISSION_FIELD: &str = "iroh_remote_admission";
const IROH_ROUTE_LABEL_PREFIX: &str = "iroh-route:";
const IROH_POLICY_HASH_SCHEMA: &str = "animus.iroh-agent-comms.policy.v1";
const IROH_SUMMARY_SEPARATOR: &str = "|";
const NO_TICKET_HASH: &str = "no-ticket-hash";
const IROH_MAX_FRAME_REASON: &str = "iroh route max frame bytes must be greater than zero";
const IROH_UNSAFE_TEXT_REASON: &str = "iroh route or frame contains unsafe raw transport material";
const IROH_UNSUPPORTED_PROTOCOL_REASON: &str = "iroh protocol id is not supported";
const IROH_UNSUPPORTED_ADMISSION_REASON: &str = "iroh remote admission status is not supported";
const IROH_TRANSPORT_UNAVAILABLE_REASON: &str = "iroh transport unavailable";
const IROH_ADMISSION_ADMITTED: &str = "admitted";
const IROH_ADMISSION_DENIED: &str = "denied";
const IROH_ADMISSION_PROMPT_REQUIRED: &str = "prompt_required";
const IROH_ADMISSION_MISSING: &str = "missing";
const MIN_IROH_LABEL_CHARS: usize = animus_adapters::MIN_ADAPTER_FIELD_CHARS;
const MAX_IROH_LABEL_CHARS: usize = animus_adapters::MAX_ADAPTER_FIELD_CHARS;

/// Stable ALPN-style protocol label for Animus Iroh agent-comms frames.
pub const IROH_AGENT_COMMS_PROTOCOL: &str = "animus.agent-comms.v1";

const IROH_UNSAFE_MARKERS: &[&str] = &[
    "iroh://",
    "ticket:",
    "nodeaddr:",
    "relay=",
    "raw_ticket",
    "private_key",
    "secret",
    "authorization:",
    "http://",
    "https://",
    "/home/",
    "/users/",
    "raw_prompt:",
    "provider_payload:",
    "credential:",
    "file_body:",
    "tool_input:",
    "tool_output:",
    "model_output:",
    "unbounded_log:",
    "api_key=",
    "sk-",
];

const _: () = assert!(MAX_IROH_LABEL_CHARS >= MIN_IROH_LABEL_CHARS);

/// Admission status supplied by the host before Iroh transport effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IrohRemoteAdmission {
    /// Remote admission has already allowed this route.
    Admitted,
    /// Remote admission denied this route.
    Denied,
    /// Remote admission requires an out-of-band prompt before use.
    PromptRequired,
    /// Remote admission evidence is missing.
    Missing,
}

impl IrohRemoteAdmission {
    /// Parses a stable Iroh remote-admission label.
    ///
    /// # Errors
    ///
    /// Returns [`IrohAdapterError`] when the label is empty, unsafe, or unsupported.
    pub fn parse(value: &str) -> Result<Self, IrohAdapterError> {
        match required_safe_text(IROH_ADMISSION_FIELD, value)?.as_str() {
            IROH_ADMISSION_ADMITTED => Ok(Self::Admitted),
            IROH_ADMISSION_DENIED => Ok(Self::Denied),
            IROH_ADMISSION_PROMPT_REQUIRED => Ok(Self::PromptRequired),
            IROH_ADMISSION_MISSING => Ok(Self::Missing),
            _unsupported => Err(IrohAdapterError::UnsafeRoute {
                reason: IROH_UNSUPPORTED_ADMISSION_REASON,
            }),
        }
    }

    /// Returns the stable admission label shared with the neutral agent-comms route.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => IROH_ADMISSION_ADMITTED,
            Self::Denied => IROH_ADMISSION_DENIED,
            Self::PromptRequired => IROH_ADMISSION_PROMPT_REQUIRED,
            Self::Missing => IROH_ADMISSION_MISSING,
        }
    }
}

/// Safe route descriptor for host-owned Iroh agent communication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IrohRouteDescriptor {
    node_id: String,
    topic_id: String,
    protocol_id: String,
    ticket_hash: Option<animus_adapters::SafeHash>,
    remote_admission: IrohRemoteAdmission,
    neutral_route: animus_adapters::AgentCommsRoute,
}

impl IrohRouteDescriptor {
    /// Validates a safe Iroh route descriptor without constructing an Iroh endpoint.
    ///
    /// # Errors
    ///
    /// Returns [`IrohAdapterError`] when the route contains raw transport material,
    /// uses an unsupported protocol, carries an invalid ticket hash, or omits admission.
    pub fn new(draft: IrohRouteDescriptorDraft<'_>) -> Result<Self, IrohAdapterError> {
        if draft.max_frame_bytes == 0 {
            return Err(IrohAdapterError::UnsafeRoute {
                reason: IROH_MAX_FRAME_REASON,
            });
        }

        let protocol_id = required_safe_text(IROH_PROTOCOL_FIELD, draft.protocol_id)?;
        if protocol_id != IROH_AGENT_COMMS_PROTOCOL {
            return Err(IrohAdapterError::UnsafeRoute {
                reason: IROH_UNSUPPORTED_PROTOCOL_REASON,
            });
        }

        let node_id = required_safe_text(IROH_NODE_FIELD, draft.node_id)?;
        let topic_id = required_safe_text(IROH_TOPIC_FIELD, draft.topic_id)?;
        let ticket_hash = draft
            .ticket_hash
            .map(animus_adapters::SafeHash::new)
            .transpose()?;
        let remote_admission = IrohRemoteAdmission::parse(draft.remote_admission)?;
        let route_label = build_route_label(node_id.as_str(), topic_id.as_str());
        let policy_hash = build_policy_hash(&PolicyHashInput {
            node_id: node_id.as_str(),
            topic_id: topic_id.as_str(),
            protocol_id: protocol_id.as_str(),
            ticket_hash: ticket_hash.as_ref(),
            admission: remote_admission,
            max_frame_bytes: draft.max_frame_bytes,
        });
        let neutral_route =
            animus_adapters::AgentCommsRoute::new(animus_adapters::AgentCommsRouteDraft {
                route_label: route_label.as_str(),
                transport_family: animus_adapters::AgentCommsTransportFamily::Iroh.as_str(),
                scope: draft.scope,
                endpoint_hash: ticket_hash.as_ref().map(animus_adapters::SafeHash::as_str),
                policy_hash: Some(policy_hash.as_str()),
                admission: remote_admission.as_str(),
                max_frame_bytes: draft.max_frame_bytes,
            })?;

        Ok(Self {
            node_id,
            topic_id,
            protocol_id,
            ticket_hash,
            remote_admission,
            neutral_route,
        })
    }

    /// Returns the safe remote node label.
    #[must_use]
    pub fn node_id(&self) -> &str {
        self.node_id.as_str()
    }

    /// Returns the safe Iroh topic label.
    #[must_use]
    pub fn topic_id(&self) -> &str {
        self.topic_id.as_str()
    }

    /// Returns the protocol label.
    #[must_use]
    pub fn protocol_id(&self) -> &str {
        self.protocol_id.as_str()
    }

    /// Returns the route scope.
    #[must_use]
    pub const fn scope(&self) -> &animus_adapters::Scope {
        self.neutral_route.scope()
    }

    /// Returns the optional hash of the host-owned ticket or node address.
    #[must_use]
    pub const fn ticket_hash(&self) -> Option<&animus_adapters::SafeHash> {
        self.ticket_hash.as_ref()
    }

    /// Returns the host-supplied admission status.
    #[must_use]
    pub const fn remote_admission(&self) -> IrohRemoteAdmission {
        self.remote_admission
    }

    /// Returns the configured maximum safe frame size.
    #[must_use]
    pub const fn max_frame_bytes(&self) -> u32 {
        self.neutral_route.max_frame_bytes()
    }

    /// Returns the neutral route used by reusable agent-comms helpers.
    #[must_use]
    pub const fn agent_comms_route(&self) -> &animus_adapters::AgentCommsRoute {
        &self.neutral_route
    }

    /// Returns the stable neutral route hash.
    #[must_use]
    pub const fn route_hash(&self) -> &animus_adapters::SafeHash {
        self.neutral_route.route_hash()
    }
}

/// Borrowed input used to validate an Iroh route descriptor.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IrohRouteDescriptorDraft<'a> {
    /// Safe remote node identity label; raw tickets and private keys are rejected.
    pub node_id: &'a str,
    /// Safe topic label bound to one tenant/workspace scope.
    pub topic_id: &'a str,
    /// Protocol label; must equal [`IROH_AGENT_COMMS_PROTOCOL`].
    pub protocol_id: &'a str,
    /// Local scope this route is allowed to serve.
    pub scope: animus_adapters::Scope,
    /// Optional BLAKE3 hash of a host-owned ticket or node address.
    pub ticket_hash: Option<&'a str>,
    /// Remote admission status label.
    pub remote_admission: &'a str,
    /// Maximum redacted frame size accepted for this route.
    pub max_frame_bytes: u32,
}

/// Iroh agent-comms frame family.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IrohFrameKind {
    /// Redacted dataspace assertion projection.
    DataspaceAssertion,
    /// Host orchestration event projection.
    OrchestrationEvent,
    /// Headless command projection supplied by a host integration.
    HeadlessCommand,
    /// Headless event projection supplied by a host integration.
    HeadlessEvent,
    /// Subagent lifecycle projection supplied by a host integration.
    SubagentLifecycle,
}

impl IrohFrameKind {
    /// Returns the stable frame-kind label shared with the neutral agent-comms frame.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.as_agent_comms_kind().as_str()
    }

    const fn as_agent_comms_kind(self) -> animus_adapters::AgentCommsFrameKind {
        match self {
            Self::DataspaceAssertion => animus_adapters::AgentCommsFrameKind::DataspaceAssertion,
            Self::OrchestrationEvent => animus_adapters::AgentCommsFrameKind::OrchestrationEvent,
            Self::HeadlessCommand => animus_adapters::AgentCommsFrameKind::HeadlessCommand,
            Self::HeadlessEvent => animus_adapters::AgentCommsFrameKind::HeadlessEvent,
            Self::SubagentLifecycle => animus_adapters::AgentCommsFrameKind::SubagentLifecycle,
        }
    }
}

/// Validated neutral frame projected for an Iroh host transport.
pub type IrohAgentFrame = animus_adapters::AgentCommsFrame;

/// Safe neutral receipt emitted by the Iroh host seam.
pub type IrohAgentReceipt = animus_adapters::AgentCommsReceipt;

/// Borrowed input used to validate an Iroh-labeled neutral frame.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IrohAgentFrameDraft<'a> {
    /// Safe frame identity.
    pub frame_id: &'a str,
    /// Frame family.
    pub kind: IrohFrameKind,
    /// Admitted route descriptor.
    pub route: &'a IrohRouteDescriptor,
    /// BLAKE3 hash of redacted payload metadata.
    pub payload_hash: &'a str,
    /// Bounded payload byte count.
    pub payload_bytes: u32,
    /// Bounded item count.
    pub item_count: u32,
    /// Redaction evidence labels.
    pub redactions: &'a [&'a str],
}

/// Host-owned Iroh transport port used by the adapter shell.
pub trait IrohAgentTransport {
    /// Publishes one already validated dataspace assertion over an Iroh route.
    ///
    /// # Errors
    ///
    /// Returns [`IrohAdapterError`] when the host transport cannot publish safely.
    fn publish_assertion(
        &mut self,
        route: &IrohRouteDescriptor,
        assertion: &animus_adapters::Assertion,
        frame: &IrohAgentFrame,
    ) -> Result<(), IrohAdapterError>;

    /// Queries assertions available through the configured Iroh route.
    ///
    /// # Errors
    ///
    /// Returns [`IrohAdapterError`] when the host transport cannot query safely.
    fn query_assertions(
        &mut self,
        route: &IrohRouteDescriptor,
        subscription: &animus_adapters::Subscription,
    ) -> Result<Vec<animus_adapters::Assertion>, IrohAdapterError>;

    /// Publishes one host orchestration event over an Iroh route.
    ///
    /// # Errors
    ///
    /// Returns [`IrohAdapterError`] when the host transport cannot publish safely.
    fn publish_orchestration_event(
        &mut self,
        route: &IrohRouteDescriptor,
        event: &animus_adapters::OrchestrationEvent,
        frame: &IrohAgentFrame,
    ) -> Result<(), IrohAdapterError>;
}

/// Feature-gated host-owned Iroh agent communication adapter.
pub struct IrohAgentCommsAdapter<T> {
    route: IrohRouteDescriptor,
    transport: T,
}

impl<T> IrohAgentCommsAdapter<T> {
    /// Builds an adapter shell over caller-owned route and transport configuration.
    #[must_use]
    pub const fn new(route: IrohRouteDescriptor, transport: T) -> Self {
        Self { route, transport }
    }

    /// Returns the configured route descriptor.
    #[must_use]
    pub const fn route(&self) -> &IrohRouteDescriptor {
        &self.route
    }

    /// Returns the wrapped host transport.
    #[must_use]
    pub const fn transport(&self) -> &T {
        &self.transport
    }

    /// Returns mutable access to the wrapped host transport.
    #[must_use]
    pub fn transport_mut(&mut self) -> &mut T {
        &mut self.transport
    }
}

impl<T> animus_adapters::AdapterService for IrohAgentCommsAdapter<T>
where
    T: IrohAgentTransport,
{
    type Error = IrohAdapterError;

    fn publish(&mut self, assertion: &animus_adapters::Assertion) -> Result<(), Self::Error> {
        admit_iroh_scope(&self.route, assertion.scope())?;
        let frame = project_assertion_frame(&self.route, assertion)?;
        self.transport
            .publish_assertion(&self.route, assertion, &frame)
    }

    fn subscribe(
        &mut self,
        subscription: &animus_adapters::Subscription,
    ) -> Result<Vec<animus_adapters::Assertion>, Self::Error> {
        admit_iroh_scope(&self.route, subscription.scope())?;
        self.transport.query_assertions(&self.route, subscription)
    }

    fn handle_event(
        &mut self,
        event: &animus_adapters::OrchestrationEvent,
    ) -> Result<(), Self::Error> {
        admit_iroh_scope(&self.route, event.scope())?;
        let frame = project_orchestration_event_frame(&self.route, event)?;
        self.transport
            .publish_orchestration_event(&self.route, event, &frame)
    }
}

/// Typed error raised by the optional Iroh adapter seam.
#[derive(Debug)]
pub enum IrohAdapterError {
    /// Route validation failed before transport effects.
    UnsafeRoute {
        /// Safe reason label.
        reason: &'static str,
    },
    /// Frame validation failed before transport effects.
    UnsafeFrame {
        /// Safe reason label.
        reason: &'static str,
    },
    /// Adapter DTO validation failed.
    AdapterContract {
        /// Source adapter contract error.
        source: animus_adapters::AdapterContractError,
    },
    /// Remote admission was not granted for this route.
    AdmissionRequired,
    /// Route and frame/query/event scopes do not match.
    ScopeMismatch,
    /// Frame exceeded the configured route ceiling.
    FrameTooLarge {
        /// Configured maximum byte count.
        max_bytes: u32,
        /// Actual byte count.
        actual_bytes: u32,
    },
    /// The explicit Iroh transport was not configured or unavailable.
    LiveBackendUnavailable,
    /// Transport returned a safe bounded failure reason.
    Transport {
        /// Safe bounded reason.
        reason: &'static str,
    },
}

impl core::fmt::Display for IrohAdapterError {
    fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::UnsafeRoute { reason } => write!(formatter, "unsafe iroh route: {reason}"),
            Self::UnsafeFrame { reason } => write!(formatter, "unsafe iroh frame: {reason}"),
            Self::AdapterContract { source } => {
                write!(formatter, "iroh adapter contract failed: {source}")
            }
            Self::AdmissionRequired => formatter.write_str("iroh route requires remote admission"),
            Self::ScopeMismatch => formatter.write_str("iroh route scope mismatch"),
            Self::FrameTooLarge {
                max_bytes,
                actual_bytes,
            } => write!(
                formatter,
                "iroh frame is too large: max {max_bytes} bytes, got {actual_bytes}"
            ),
            Self::LiveBackendUnavailable => formatter.write_str(IROH_TRANSPORT_UNAVAILABLE_REASON),
            Self::Transport { reason } => write!(formatter, "iroh transport failed: {reason}"),
        }
    }
}

impl std::error::Error for IrohAdapterError {}

impl From<animus_adapters::AdapterContractError> for IrohAdapterError {
    fn from(source: animus_adapters::AdapterContractError) -> Self {
        Self::AdapterContract { source }
    }
}

impl From<animus_adapters::AgentCommsError> for IrohAdapterError {
    fn from(error: animus_adapters::AgentCommsError) -> Self {
        match error {
            animus_adapters::AgentCommsError::Contract(source) => Self::AdapterContract { source },
            animus_adapters::AgentCommsError::UnsupportedValue { .. }
            | animus_adapters::AgentCommsError::UnsafeFrame { .. }
            | animus_adapters::AgentCommsError::MissingRedactionEvidence
            | animus_adapters::AgentCommsError::ReceiptLeak { .. } => Self::UnsafeFrame {
                reason: IROH_UNSAFE_TEXT_REASON,
            },
            animus_adapters::AgentCommsError::UnsafeRoute { .. }
            | animus_adapters::AgentCommsError::MissingPolicyEvidence => Self::UnsafeRoute {
                reason: IROH_UNSAFE_TEXT_REASON,
            },
            animus_adapters::AgentCommsError::AdmissionRequired => Self::AdmissionRequired,
            animus_adapters::AgentCommsError::ScopeMismatch => Self::ScopeMismatch,
            animus_adapters::AgentCommsError::FrameTooLarge {
                max_bytes,
                actual_bytes,
            } => Self::FrameTooLarge {
                max_bytes,
                actual_bytes,
            },
            animus_adapters::AgentCommsError::LiveBackendUnavailable => {
                Self::LiveBackendUnavailable
            }
            animus_adapters::AgentCommsError::Transport { reason } => Self::Transport { reason },
        }
    }
}

/// Validates that an Iroh route may serve the supplied scope.
///
/// # Errors
///
/// Returns [`IrohAdapterError`] when admission is missing or the scope crosses a tenant/workspace.
pub fn admit_iroh_scope(
    route: &IrohRouteDescriptor,
    scope: &animus_adapters::Scope,
) -> Result<(), IrohAdapterError> {
    animus_adapters::admit_agent_comms_scope(route.agent_comms_route(), scope)
        .map_err(IrohAdapterError::from)
}

/// Builds a safe redacted Iroh frame using the neutral agent-comms frame validator.
///
/// # Errors
///
/// Returns [`IrohAdapterError`] when admission, frame size, hashes, labels, or redactions fail.
pub fn project_iroh_frame(
    draft: &IrohAgentFrameDraft<'_>,
) -> Result<IrohAgentFrame, IrohAdapterError> {
    animus_adapters::AgentCommsFrame::new(animus_adapters::AgentCommsFrameDraft {
        frame_id: draft.frame_id,
        kind: draft.kind.as_agent_comms_kind(),
        route: draft.route.agent_comms_route(),
        payload_hash: draft.payload_hash,
        payload_bytes: draft.payload_bytes,
        item_count: draft.item_count,
        redactions: draft.redactions,
    })
    .map_err(IrohAdapterError::from)
}

/// Projects one dataspace assertion into safe neutral frame metadata for Iroh.
///
/// # Errors
///
/// Returns [`IrohAdapterError`] when scope, admission, labels, redactions, or frame limits fail.
pub fn project_assertion_frame(
    route: &IrohRouteDescriptor,
    assertion: &animus_adapters::Assertion,
) -> Result<IrohAgentFrame, IrohAdapterError> {
    animus_adapters::project_assertion_frame(route.agent_comms_route(), assertion)
        .map_err(IrohAdapterError::from)
}

/// Projects one orchestration event into safe neutral frame metadata for Iroh.
///
/// # Errors
///
/// Returns [`IrohAdapterError`] when scope, admission, labels, or frame limits fail.
pub fn project_orchestration_event_frame(
    route: &IrohRouteDescriptor,
    event: &animus_adapters::OrchestrationEvent,
) -> Result<IrohAgentFrame, IrohAdapterError> {
    animus_adapters::project_orchestration_event_frame(route.agent_comms_route(), event)
        .map_err(IrohAdapterError::from)
}

/// Builds a safe Iroh route receipt through the neutral agent-comms receipt validator.
///
/// # Errors
///
/// Returns [`IrohAdapterError`] when route metadata cannot be recorded safely.
pub fn iroh_route_receipt(
    route: &IrohRouteDescriptor,
    status: animus_adapters::AgentCommsReceiptStatus,
) -> Result<IrohAgentReceipt, IrohAdapterError> {
    animus_adapters::AgentCommsReceipt::from_route(route.agent_comms_route(), status)
        .map_err(IrohAdapterError::from)
}

/// Builds a safe Iroh frame receipt through the neutral agent-comms receipt validator.
///
/// # Errors
///
/// Returns [`IrohAdapterError`] when frame metadata cannot be recorded safely.
pub fn iroh_frame_receipt(
    route: &IrohRouteDescriptor,
    frame: &IrohAgentFrame,
    status: animus_adapters::AgentCommsReceiptStatus,
) -> Result<IrohAgentReceipt, IrohAdapterError> {
    animus_adapters::AgentCommsReceipt::from_frame(route.agent_comms_route(), frame, status)
        .map_err(IrohAdapterError::from)
}

fn build_route_label(node_id: &str, topic_id: &str) -> String {
    format!("{IROH_ROUTE_LABEL_PREFIX}{node_id}:{topic_id}")
}

#[derive(Clone, Copy)]
struct PolicyHashInput<'a> {
    node_id: &'a str,
    topic_id: &'a str,
    protocol_id: &'a str,
    ticket_hash: Option<&'a animus_adapters::SafeHash>,
    admission: IrohRemoteAdmission,
    max_frame_bytes: u32,
}

fn build_policy_hash(input: &PolicyHashInput<'_>) -> animus_adapters::SafeHash {
    let ticket_hash = input
        .ticket_hash
        .map_or(NO_TICKET_HASH, animus_adapters::SafeHash::as_str);
    let max_frame_bytes = input.max_frame_bytes.to_string();
    let summary = [
        IROH_POLICY_HASH_SCHEMA,
        input.node_id,
        input.topic_id,
        input.protocol_id,
        ticket_hash,
        input.admission.as_str(),
        max_frame_bytes.as_str(),
    ]
    .join(IROH_SUMMARY_SEPARATOR);
    animus_adapters::SafeHash::from_blake3_bytes(summary.as_bytes())
}

fn required_safe_text(field: &'static str, value: &str) -> Result<String, IrohAdapterError> {
    let trimmed = value.trim();
    let visible_chars = trimmed.chars().count();
    if visible_chars < MIN_IROH_LABEL_CHARS {
        return Err(IrohAdapterError::UnsafeRoute { reason: field });
    }
    if visible_chars > MAX_IROH_LABEL_CHARS || trimmed.contains('\0') {
        return Err(IrohAdapterError::UnsafeRoute { reason: field });
    }
    validate_no_unsafe_markers(trimmed)?;
    Ok(String::from(trimmed))
}

fn validate_no_unsafe_markers(value: &str) -> Result<(), IrohAdapterError> {
    let lower = value.to_ascii_lowercase();
    for marker in IROH_UNSAFE_MARKERS {
        if lower.contains(marker) {
            return Err(IrohAdapterError::UnsafeRoute {
                reason: IROH_UNSAFE_TEXT_REASON,
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::AdapterService;

    const TENANT: &str = "tenant:local";
    const WORKSPACE: &str = "workspace:main";
    const OTHER_WORKSPACE: &str = "workspace:other";
    const SESSION: &str = "session:main";
    const BRANCH: &str = "branch:main";
    const FACET: &str = "facet:loop";
    const ACTOR_KIND: &str = "agent";
    const ACTOR_ID: &str = "agent:main";
    const NODE_ID: &str = "node:peer-a";
    const TOPIC_ID: &str = "topic:workspace-main";
    const RAW_TICKET_NODE: &str = "iroh://ticket/raw";
    const ASSERTION_ID: &str = "assertion:runtime";
    const ASSERTION_RAW_ID: &str = "raw_prompt:private";
    const STATUS_ACTIVE: &str = "active";
    const EVENT_ID: &str = "event:cancel";
    const EVENT_CANCELLED: &str = "cancelled";
    const SAFE_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const INVALID_HASH: &str = "not-a-valid-hash";
    const REDACTION_PROMPT: &str = "prompt_omitted";
    const REDACTION_PROVIDER: &str = "provider_payload_omitted";
    const REDACTION_CREDENTIAL: &str = "credential_omitted";
    const REDACTION_LOGS: &str = "logs_bounded";
    const MAX_FRAME_BYTES: u32 = 4_096;
    const TINY_FRAME_BYTES: u32 = 1;
    const EXPECTED_ONE: usize = 1;
    const NO_ITEMS: usize = 0;
    const ITEM_COUNT: u32 = 1;
    const HEADLESS_PAYLOAD_BYTES: u32 = 2;
    const HEADLESS_ITEM_COUNT: u32 = 2;

    #[derive(Default)]
    struct MemoryIrohTransport {
        frames: Vec<IrohAgentFrame>,
        assertions: Vec<animus_adapters::Assertion>,
        unavailable: bool,
    }

    impl IrohAgentTransport for MemoryIrohTransport {
        fn publish_assertion(
            &mut self,
            _route: &IrohRouteDescriptor,
            assertion: &animus_adapters::Assertion,
            frame: &IrohAgentFrame,
        ) -> Result<(), IrohAdapterError> {
            if self.unavailable {
                return Err(IrohAdapterError::LiveBackendUnavailable);
            }
            self.frames.push(frame.clone());
            self.assertions.push(assertion.clone());
            Ok(())
        }

        fn query_assertions(
            &mut self,
            _route: &IrohRouteDescriptor,
            subscription: &animus_adapters::Subscription,
        ) -> Result<Vec<animus_adapters::Assertion>, IrohAdapterError> {
            if self.unavailable {
                return Err(IrohAdapterError::LiveBackendUnavailable);
            }
            Ok(self
                .assertions
                .iter()
                .filter(|assertion| {
                    assertion
                        .scope()
                        .same_tenant_workspace(subscription.scope())
                })
                .filter(|assertion| subscription.allows(assertion.kind()))
                .cloned()
                .collect())
        }

        fn publish_orchestration_event(
            &mut self,
            _route: &IrohRouteDescriptor,
            _event: &animus_adapters::OrchestrationEvent,
            frame: &IrohAgentFrame,
        ) -> Result<(), IrohAdapterError> {
            if self.unavailable {
                return Err(IrohAdapterError::LiveBackendUnavailable);
            }
            self.frames.push(frame.clone());
            Ok(())
        }
    }

    fn scope(workspace: &str) -> animus_adapters::Scope {
        animus_adapters::Scope::new(animus_adapters::ScopeDraft {
            tenant_id: TENANT,
            workspace_id: workspace,
            session_id: SESSION,
            branch_id: BRANCH,
            facet_id: FACET,
            actor_kind: ACTOR_KIND,
            actor_id: ACTOR_ID,
        })
        .expect("scope should validate")
    }

    fn route_for(workspace: &str, admission: &str) -> IrohRouteDescriptor {
        route_with_max(workspace, admission, MAX_FRAME_BYTES)
    }

    fn route_with_max(
        workspace: &str,
        admission: &str,
        max_frame_bytes: u32,
    ) -> IrohRouteDescriptor {
        IrohRouteDescriptor::new(IrohRouteDescriptorDraft {
            node_id: NODE_ID,
            topic_id: TOPIC_ID,
            protocol_id: IROH_AGENT_COMMS_PROTOCOL,
            scope: scope(workspace),
            ticket_hash: Some(SAFE_HASH),
            remote_admission: admission,
            max_frame_bytes,
        })
        .expect("route should validate")
    }

    fn assertion_for(workspace: &str, stable_id: &str) -> animus_adapters::Assertion {
        animus_adapters::Assertion::new(animus_adapters::AssertionDraft {
            stable_id,
            kind: "runtime_event",
            status: STATUS_ACTIVE,
            scope: scope(workspace),
            redactions: &[
                REDACTION_PROMPT,
                REDACTION_PROVIDER,
                REDACTION_CREDENTIAL,
                REDACTION_LOGS,
            ],
            content_hash: Some(SAFE_HASH),
            item_count: ITEM_COUNT,
            lease_expires_at: None,
        })
        .expect("assertion should validate")
    }

    fn query(workspace: &str) -> animus_adapters::Subscription {
        animus_adapters::Subscription::new(animus_adapters::SubscriptionDraft {
            scope: scope(workspace),
            kinds: &["runtime_event"],
            include_raw_payloads: false,
            cross_workspace: false,
        })
        .expect("subscription should validate")
    }

    fn cancellation_event(workspace: &str) -> animus_adapters::OrchestrationEvent {
        animus_adapters::OrchestrationEvent::new(animus_adapters::OrchestrationEventDraft {
            stable_id: EVENT_ID,
            kind: EVENT_CANCELLED,
            scope: scope(workspace),
            target_owner: None,
        })
        .expect("event should validate")
    }

    #[test]
    fn iroh_adapter_publishes_queries_and_frames_safe_assertions() {
        let route = route_for(WORKSPACE, IROH_ADMISSION_ADMITTED);
        let assertion = assertion_for(WORKSPACE, ASSERTION_ID);
        let mut adapter = IrohAgentCommsAdapter::new(route, MemoryIrohTransport::default());

        adapter.publish(&assertion).expect("publish should pass");
        let matches = adapter
            .subscribe(&query(WORKSPACE))
            .expect("query should pass");

        assert_eq!(matches.len(), EXPECTED_ONE);
        assert_eq!(adapter.transport().frames.len(), EXPECTED_ONE);
        assert_eq!(
            adapter.transport().frames[0].kind(),
            animus_adapters::AgentCommsFrameKind::DataspaceAssertion
        );
        assert!(
            adapter.transport().frames[0]
                .redactions()
                .contains(&animus_adapters::AgentCommsRedaction::ProviderPayloadOmitted)
        );
    }

    #[test]
    fn iroh_adapter_projects_orchestration_events_without_raw_payloads() {
        let route = route_for(WORKSPACE, IROH_ADMISSION_ADMITTED);
        let event = cancellation_event(WORKSPACE);
        let mut adapter = IrohAgentCommsAdapter::new(route, MemoryIrohTransport::default());

        adapter
            .handle_event(&event)
            .expect("event publish should pass");

        assert_eq!(adapter.transport().frames.len(), EXPECTED_ONE);
        assert_eq!(
            adapter.transport().frames[0].kind(),
            animus_adapters::AgentCommsFrameKind::OrchestrationEvent
        );
        assert_eq!(adapter.transport().frames[0].item_count(), ITEM_COUNT);
    }

    #[test]
    fn iroh_route_maps_to_neutral_route_and_safe_receipts() {
        let route = route_for(WORKSPACE, IROH_ADMISSION_ADMITTED);
        let assertion = assertion_for(WORKSPACE, ASSERTION_ID);
        let frame = project_assertion_frame(&route, &assertion).expect("frame should project");
        let route_receipt =
            iroh_route_receipt(&route, animus_adapters::AgentCommsReceiptStatus::Admitted)
                .expect("route receipt should validate");
        let frame_receipt = iroh_frame_receipt(
            &route,
            &frame,
            animus_adapters::AgentCommsReceiptStatus::Completed,
        )
        .expect("frame receipt should validate");

        assert_eq!(
            route.agent_comms_route().transport_family(),
            animus_adapters::AgentCommsTransportFamily::Iroh
        );
        assert_eq!(frame.route_hash(), route.route_hash());
        assert_eq!(
            route_receipt.transport_family(),
            animus_adapters::AgentCommsTransportFamily::Iroh
        );
        assert_eq!(frame_receipt.frame_hash(), frame.frame_hash());
    }

    #[test]
    fn iroh_route_and_frame_reject_raw_transport_and_sensitive_material() {
        let raw_route = IrohRouteDescriptor::new(IrohRouteDescriptorDraft {
            node_id: RAW_TICKET_NODE,
            topic_id: TOPIC_ID,
            protocol_id: IROH_AGENT_COMMS_PROTOCOL,
            scope: scope(WORKSPACE),
            ticket_hash: Some(SAFE_HASH),
            remote_admission: IROH_ADMISSION_ADMITTED,
            max_frame_bytes: MAX_FRAME_BYTES,
        })
        .expect_err("raw ticket marker should fail");
        let route = route_for(WORKSPACE, IROH_ADMISSION_ADMITTED);
        let raw_assertion = assertion_for(WORKSPACE, ASSERTION_RAW_ID);
        let raw_frame = project_assertion_frame(&route, &raw_assertion)
            .expect_err("raw assertion marker should fail");

        assert!(matches!(raw_route, IrohAdapterError::UnsafeRoute { .. }));
        assert!(matches!(raw_frame, IrohAdapterError::UnsafeFrame { .. }));
    }

    #[test]
    fn iroh_adapter_fails_closed_without_admission_or_matching_scope() {
        let denied = route_for(WORKSPACE, IROH_ADMISSION_DENIED);
        let assertion = assertion_for(WORKSPACE, ASSERTION_ID);
        let mut adapter = IrohAgentCommsAdapter::new(denied, MemoryIrohTransport::default());
        let admission_error = adapter
            .publish(&assertion)
            .expect_err("denied route should fail");
        let prompt_route = route_for(WORKSPACE, IROH_ADMISSION_PROMPT_REQUIRED);
        let prompt_error = project_assertion_frame(&prompt_route, &assertion)
            .expect_err("prompt-required route should fail");
        let admitted = route_for(WORKSPACE, IROH_ADMISSION_ADMITTED);
        let other_assertion = assertion_for(OTHER_WORKSPACE, ASSERTION_ID);
        let scope_error = project_assertion_frame(&admitted, &other_assertion)
            .expect_err("cross workspace should fail");

        assert!(matches!(
            admission_error,
            IrohAdapterError::AdmissionRequired
        ));
        assert!(matches!(prompt_error, IrohAdapterError::AdmissionRequired));
        assert!(matches!(scope_error, IrohAdapterError::ScopeMismatch));
    }

    #[test]
    fn iroh_adapter_rejects_oversized_frames_and_unavailable_transport() {
        let tiny_route = route_with_max(WORKSPACE, IROH_ADMISSION_ADMITTED, TINY_FRAME_BYTES);
        let assertion = assertion_for(WORKSPACE, ASSERTION_ID);
        let size_error = project_assertion_frame(&tiny_route, &assertion)
            .expect_err("oversized frame should fail");
        let route = route_for(WORKSPACE, IROH_ADMISSION_ADMITTED);
        let mut unavailable = IrohAgentCommsAdapter::new(
            route,
            MemoryIrohTransport {
                frames: Vec::new(),
                assertions: Vec::new(),
                unavailable: true,
            },
        );
        let unavailable_error = unavailable
            .publish(&assertion)
            .expect_err("unavailable transport should fail");

        assert!(matches!(size_error, IrohAdapterError::FrameTooLarge { .. }));
        assert!(matches!(
            unavailable_error,
            IrohAdapterError::LiveBackendUnavailable
        ));
        assert_eq!(unavailable.transport().frames.len(), NO_ITEMS);
    }

    #[test]
    fn iroh_frame_projection_accepts_headless_and_subagent_metadata_kinds() {
        let route = route_for(WORKSPACE, IROH_ADMISSION_ADMITTED);
        let payload_hash = animus_adapters::SafeHash::from_blake3_bytes(b"headless-summary");
        let command = project_iroh_frame(&IrohAgentFrameDraft {
            frame_id: "frame:headless-command",
            kind: IrohFrameKind::HeadlessCommand,
            route: &route,
            payload_hash: payload_hash.as_str(),
            payload_bytes: HEADLESS_PAYLOAD_BYTES,
            item_count: HEADLESS_ITEM_COUNT,
            redactions: &[REDACTION_PROMPT, REDACTION_PROVIDER],
        })
        .expect("headless frame should validate");
        let subagent = project_iroh_frame(&IrohAgentFrameDraft {
            frame_id: "frame:subagent-lifecycle",
            kind: IrohFrameKind::SubagentLifecycle,
            route: &route,
            payload_hash: payload_hash.as_str(),
            payload_bytes: HEADLESS_PAYLOAD_BYTES,
            item_count: ITEM_COUNT,
            redactions: &[REDACTION_LOGS],
        })
        .expect("subagent frame should validate");

        assert_eq!(
            command.kind(),
            animus_adapters::AgentCommsFrameKind::HeadlessCommand
        );
        assert_eq!(
            subagent.kind(),
            animus_adapters::AgentCommsFrameKind::SubagentLifecycle
        );
    }

    #[test]
    fn iroh_route_and_frame_reject_invalid_protocol_hash_redaction_and_receipt_leaks() {
        let invalid_protocol = IrohRouteDescriptor::new(IrohRouteDescriptorDraft {
            node_id: NODE_ID,
            topic_id: TOPIC_ID,
            protocol_id: "animus.agent-comms.v0",
            scope: scope(WORKSPACE),
            ticket_hash: Some(SAFE_HASH),
            remote_admission: IROH_ADMISSION_ADMITTED,
            max_frame_bytes: MAX_FRAME_BYTES,
        })
        .expect_err("invalid protocol should fail");
        let invalid_hash = IrohRouteDescriptor::new(IrohRouteDescriptorDraft {
            node_id: NODE_ID,
            topic_id: TOPIC_ID,
            protocol_id: IROH_AGENT_COMMS_PROTOCOL,
            scope: scope(WORKSPACE),
            ticket_hash: Some(INVALID_HASH),
            remote_admission: IROH_ADMISSION_ADMITTED,
            max_frame_bytes: MAX_FRAME_BYTES,
        })
        .expect_err("invalid ticket hash should fail");
        let route = route_for(WORKSPACE, IROH_ADMISSION_ADMITTED);
        let payload_hash = animus_adapters::SafeHash::from_blake3_bytes(b"missing-redaction");
        let missing_redaction = project_iroh_frame(&IrohAgentFrameDraft {
            frame_id: "frame:missing-redaction",
            kind: IrohFrameKind::HeadlessEvent,
            route: &route,
            payload_hash: payload_hash.as_str(),
            payload_bytes: HEADLESS_PAYLOAD_BYTES,
            item_count: ITEM_COUNT,
            redactions: &[],
        })
        .expect_err("missing redaction should fail");
        let receipt_leak =
            animus_adapters::AgentCommsReceipt::new(&animus_adapters::AgentCommsReceiptDraft {
                route_label: RAW_TICKET_NODE,
                transport_family: animus_adapters::AgentCommsTransportFamily::Iroh.as_str(),
                scope: scope(WORKSPACE),
                endpoint_hash: Some(SAFE_HASH),
                policy_hash: Some(SAFE_HASH),
                admission: IROH_ADMISSION_ADMITTED,
                max_frame_bytes: MAX_FRAME_BYTES,
                frame_id: "frame:receipt-leak",
                frame_kind: animus_adapters::AgentCommsFrameKind::DataspaceAssertion.as_str(),
                payload_hash: SAFE_HASH,
                payload_bytes: ITEM_COUNT,
                item_count: ITEM_COUNT,
                status: animus_adapters::AgentCommsReceiptStatus::Completed.as_str(),
                redactions: &[REDACTION_PROMPT],
            })
            .map_err(IrohAdapterError::from)
            .expect_err("receipt leak should fail");

        assert!(matches!(
            invalid_protocol,
            IrohAdapterError::UnsafeRoute { .. }
        ));
        assert!(matches!(
            invalid_hash,
            IrohAdapterError::AdapterContract { .. }
        ));
        assert!(matches!(
            missing_redaction,
            IrohAdapterError::UnsafeFrame { .. }
        ));
        assert!(matches!(
            receipt_leak,
            IrohAdapterError::UnsafeFrame { .. } | IrohAdapterError::UnsafeRoute { .. }
        ));
    }
}
