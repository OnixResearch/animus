const PAYLOAD_BYTES_COUNT: &str = "payload_bytes";
const FIXTURE_MODEL_LABEL: &str = "fixture_model";
const FIXTURE_FILESYSTEM_LABEL: &str = "fixture_filesystem";
const FIXTURE_PROCESS_LABEL: &str = "fixture_process";
const FIXTURE_NETWORK_LABEL: &str = "fixture_network";
const FIXTURE_CLOCK_LABEL: &str = "fixture_clock";
const FIXTURE_CREDENTIAL_LABEL: &str = "fixture_credential";
const FIXTURE_MODEL_MESSAGE: &str = "fixture model response";
const FIXTURE_TOOL_OUTPUT: &str = "fixture tool output";
const FIXTURE_MODEL_OUTPUT_LABEL: &str = "fixture_model_ok";
const FIXTURE_FILESYSTEM_OUTPUT_LABEL: &str = "fixture_filesystem_ok";
const FIXTURE_PROCESS_OUTPUT_LABEL: &str = "fixture_process_ok";
const FIXTURE_NETWORK_OUTPUT_LABEL: &str = "fixture_network_ok";
const FIXTURE_CLOCK_OUTPUT_LABEL: &str = "fixture_clock_ok";
const FIXTURE_CREDENTIAL_OUTPUT_LABEL: &str = "fixture_credential_ok";

const MODEL_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::PromptOmitted,
    animus_adapters::ReceiptRedaction::ProviderPayloadOmitted,
    animus_adapters::ReceiptRedaction::ModelOutputOmitted,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];
const TOOL_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::ToolInputOmitted,
    animus_adapters::ReceiptRedaction::ToolOutputOmitted,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];
const FILESYSTEM_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::AbsolutePathOmitted,
    animus_adapters::ReceiptRedaction::ToolInputOmitted,
    animus_adapters::ReceiptRedaction::ToolOutputOmitted,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];
const NETWORK_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::ProviderPayloadOmitted,
    animus_adapters::ReceiptRedaction::OutputBounded,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];
const FIXTURE_REDACTIONS: &[animus_adapters::ReceiptRedaction] = &[
    animus_adapters::ReceiptRedaction::OutputBounded,
    animus_adapters::ReceiptRedaction::PayloadHashOnly,
];

/// Concrete adapter family owned by the standard-library shell.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum AdapterFamily {
    /// Host-owned model provider clients.
    ModelProvider,
    /// Host-owned filesystem tool adapters.
    FilesystemTool,
    /// Host-owned process execution adapters.
    ProcessExecution,
    /// Host-owned HTTP or network helper adapters.
    HttpNetworkHelper,
    /// Deterministic fixture backends used by default tests.
    FixtureBackend,
}

impl AdapterFamily {
    /// Returns the stable adapter family label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ModelProvider => "model_provider",
            Self::FilesystemTool => "filesystem_tool",
            Self::ProcessExecution => "process_execution",
            Self::HttpNetworkHelper => "http_network_helper",
            Self::FixtureBackend => "fixture_backend",
        }
    }

    /// Returns true because concrete families are shell-owned by contract.
    #[must_use]
    pub const fn is_shell_owned(self) -> bool {
        match self {
            Self::ModelProvider
            | Self::FilesystemTool
            | Self::ProcessExecution
            | Self::HttpNetworkHelper
            | Self::FixtureBackend => true,
        }
    }

    const fn operation(self) -> animus_adapters::ReceiptOperation {
        match self {
            Self::ModelProvider => animus_adapters::ReceiptOperation::ModelDispatch,
            Self::FilesystemTool | Self::ProcessExecution | Self::HttpNetworkHelper => {
                animus_adapters::ReceiptOperation::ToolExecution
            }
            Self::FixtureBackend => animus_adapters::ReceiptOperation::AdapterBoundary,
        }
    }

    const fn redactions(self) -> &'static [animus_adapters::ReceiptRedaction] {
        match self {
            Self::ModelProvider => MODEL_REDACTIONS,
            Self::FilesystemTool => FILESYSTEM_REDACTIONS,
            Self::ProcessExecution => TOOL_REDACTIONS,
            Self::HttpNetworkHelper => NETWORK_REDACTIONS,
            Self::FixtureBackend => FIXTURE_REDACTIONS,
        }
    }
}

/// Runtime exposure mode for a std-shell adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectExposure {
    /// A host-owned adapter that may perform live effects only after gates pass.
    HostOwned,
    /// A deterministic fixture adapter that never performs live host effects.
    FixtureOnly,
}

impl EffectExposure {
    /// Returns the stable exposure label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::HostOwned => "host_owned",
            Self::FixtureOnly => "fixture_only",
        }
    }

    /// Returns true when the adapter can represent a live host effect.
    #[must_use]
    pub const fn allows_live_effects(self) -> bool {
        match self {
            Self::HostOwned => true,
            Self::FixtureOnly => false,
        }
    }
}

/// Borrowed descriptor for one concrete std-shell adapter.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AdapterDescriptor<'a> {
    family: AdapterFamily,
    label: &'a str,
    exposure: EffectExposure,
}

impl<'a> AdapterDescriptor<'a> {
    /// Builds a host-owned adapter descriptor.
    #[must_use]
    pub const fn host_owned(family: AdapterFamily, label: &'a str) -> Self {
        Self {
            family,
            label,
            exposure: EffectExposure::HostOwned,
        }
    }

    /// Builds a deterministic fixture adapter descriptor.
    #[must_use]
    pub const fn fixture(label: &'a str) -> Self {
        Self {
            family: AdapterFamily::FixtureBackend,
            label,
            exposure: EffectExposure::FixtureOnly,
        }
    }

    /// Returns the adapter family.
    #[must_use]
    pub const fn family(self) -> AdapterFamily {
        self.family
    }

    /// Returns the safe adapter label supplied by the shell.
    #[must_use]
    pub const fn label(self) -> &'a str {
        self.label
    }

    /// Returns the adapter exposure mode.
    #[must_use]
    pub const fn exposure(self) -> EffectExposure {
        self.exposure
    }

    /// Returns true when this descriptor can represent a live host effect.
    #[must_use]
    pub const fn allows_live_effects(self) -> bool {
        self.exposure.allows_live_effects()
    }
}

/// State of a required prerequisite gate.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GateState {
    /// The prerequisite is present and satisfied.
    Satisfied,
    /// The prerequisite contract or service is absent.
    Missing,
    /// Policy or user approval explicitly denied the action.
    Denied,
    /// The prerequisite was present but incompatible with the request.
    Mismatch,
}

/// Credential prerequisite state for a concrete adapter request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CredentialState {
    /// The adapter does not require credentials.
    NotRequired,
    /// Required credentials are available to the host adapter.
    Present,
    /// Required credentials are missing.
    Missing,
}

/// Output-validation state for a concrete adapter request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutputState {
    /// Adapter output is valid at its boundary.
    Valid,
    /// Adapter output is malformed and must fail closed.
    Malformed,
}

/// Termination state observed before or during adapter dispatch.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TerminationState {
    /// Dispatch can continue.
    Continue,
    /// Dispatch timed out.
    TimedOut,
    /// Dispatch was cancelled.
    Cancelled,
}

/// Pure prerequisite snapshot evaluated before a std-shell adapter performs effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EffectPrerequisites {
    provider_registry: GateState,
    tool_contract: GateState,
    policy: GateState,
    sandbox: GateState,
    receipt_contract: GateState,
    event_contract: GateState,
    host_approval: GateState,
    credentials: CredentialState,
    output: OutputState,
    termination: TerminationState,
}

impl EffectPrerequisites {
    /// Builds a prerequisite snapshot with every required gate satisfied.
    #[must_use]
    pub const fn satisfied() -> Self {
        Self {
            provider_registry: GateState::Satisfied,
            tool_contract: GateState::Satisfied,
            policy: GateState::Satisfied,
            sandbox: GateState::Satisfied,
            receipt_contract: GateState::Satisfied,
            event_contract: GateState::Satisfied,
            host_approval: GateState::Satisfied,
            credentials: CredentialState::NotRequired,
            output: OutputState::Valid,
            termination: TerminationState::Continue,
        }
    }

    /// Marks provider-registry metadata absent.
    #[must_use]
    pub const fn without_provider_registry(mut self) -> Self {
        self.provider_registry = GateState::Missing;
        self
    }

    /// Marks accepted tool contracts absent.
    #[must_use]
    pub const fn without_tool_contract(mut self) -> Self {
        self.tool_contract = GateState::Missing;
        self
    }

    /// Marks runtime policy absent.
    #[must_use]
    pub const fn without_policy(mut self) -> Self {
        self.policy = GateState::Missing;
        self
    }

    /// Marks runtime policy denied.
    #[must_use]
    pub const fn with_policy_denied(mut self) -> Self {
        self.policy = GateState::Denied;
        self
    }

    /// Marks sandbox compatibility missing.
    #[must_use]
    pub const fn without_sandbox(mut self) -> Self {
        self.sandbox = GateState::Missing;
        self
    }

    /// Marks sandbox compatibility mismatched.
    #[must_use]
    pub const fn with_sandbox_mismatch(mut self) -> Self {
        self.sandbox = GateState::Mismatch;
        self
    }

    /// Marks safe receipt contracts absent.
    #[must_use]
    pub const fn without_receipt_contract(mut self) -> Self {
        self.receipt_contract = GateState::Missing;
        self
    }

    /// Marks runtime event contracts absent.
    #[must_use]
    pub const fn without_event_contract(mut self) -> Self {
        self.event_contract = GateState::Missing;
        self
    }

    /// Marks host approval absent.
    #[must_use]
    pub const fn without_host_approval(mut self) -> Self {
        self.host_approval = GateState::Missing;
        self
    }

    /// Marks host approval denied.
    #[must_use]
    pub const fn with_host_approval_denied(mut self) -> Self {
        self.host_approval = GateState::Denied;
        self
    }

    /// Marks credentials present for adapters that require them.
    #[must_use]
    pub const fn with_credentials_present(mut self) -> Self {
        self.credentials = CredentialState::Present;
        self
    }

    /// Marks required credentials missing.
    #[must_use]
    pub const fn with_missing_credentials(mut self) -> Self {
        self.credentials = CredentialState::Missing;
        self
    }

    /// Marks adapter output malformed.
    #[must_use]
    pub const fn with_malformed_output(mut self) -> Self {
        self.output = OutputState::Malformed;
        self
    }

    /// Marks dispatch timed out.
    #[must_use]
    pub const fn with_timeout(mut self) -> Self {
        self.termination = TerminationState::TimedOut;
        self
    }

    /// Marks dispatch cancelled.
    #[must_use]
    pub const fn with_cancellation(mut self) -> Self {
        self.termination = TerminationState::Cancelled;
        self
    }

    /// Returns the provider registry gate state.
    #[must_use]
    pub const fn provider_registry(self) -> GateState {
        self.provider_registry
    }

    /// Returns the tool contract gate state.
    #[must_use]
    pub const fn tool_contract(self) -> GateState {
        self.tool_contract
    }

    /// Returns the policy gate state.
    #[must_use]
    pub const fn policy(self) -> GateState {
        self.policy
    }

    /// Returns the sandbox gate state.
    #[must_use]
    pub const fn sandbox(self) -> GateState {
        self.sandbox
    }

    /// Returns the safe receipt contract gate state.
    #[must_use]
    pub const fn receipt_contract(self) -> GateState {
        self.receipt_contract
    }

    /// Returns the runtime event contract gate state.
    #[must_use]
    pub const fn event_contract(self) -> GateState {
        self.event_contract
    }

    /// Returns the host approval gate state.
    #[must_use]
    pub const fn host_approval(self) -> GateState {
        self.host_approval
    }

    /// Returns the credential state.
    #[must_use]
    pub const fn credentials(self) -> CredentialState {
        self.credentials
    }

    /// Returns the output validation state.
    #[must_use]
    pub const fn output(self) -> OutputState {
        self.output
    }

    /// Returns the termination state.
    #[must_use]
    pub const fn termination(self) -> TerminationState {
        self.termination
    }
}

/// Typed issue selected before a concrete adapter may perform host effects.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterIssue {
    /// No issue was observed.
    None,
    /// Provider registry metadata is missing.
    MissingProviderRegistry,
    /// Accepted tool contract metadata is missing.
    MissingToolContract,
    /// Runtime policy metadata is missing.
    MissingPolicy,
    /// Capability policy or host approval denied the request.
    DeniedCapability,
    /// Sandbox metadata is missing.
    MissingSandbox,
    /// Sandbox metadata does not match the request.
    SandboxMismatch,
    /// Safe receipt contract metadata is missing.
    MissingReceiptContract,
    /// Runtime event contract metadata is missing.
    MissingEventContract,
    /// Host approval is required but absent.
    MissingHostApproval,
    /// Credentials required by the adapter are absent.
    MissingCredentials,
    /// Adapter output failed boundary validation.
    MalformedOutput,
    /// Adapter dispatch timed out.
    TimedOut,
    /// Adapter dispatch was cancelled.
    Cancelled,
}

impl AdapterIssue {
    /// Returns the stable issue label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::MissingProviderRegistry => "missing_provider_registry",
            Self::MissingToolContract => "missing_tool_contract",
            Self::MissingPolicy => "missing_policy",
            Self::DeniedCapability => "denied_capability",
            Self::MissingSandbox => "missing_sandbox",
            Self::SandboxMismatch => "sandbox_mismatch",
            Self::MissingReceiptContract => "missing_receipt_contract",
            Self::MissingEventContract => "missing_event_contract",
            Self::MissingHostApproval => "missing_host_approval",
            Self::MissingCredentials => "missing_credentials",
            Self::MalformedOutput => "malformed_output",
            Self::TimedOut => "timed_out",
            Self::Cancelled => "cancelled",
        }
    }

    const fn receipt_status(self) -> animus_adapters::ReceiptStatus {
        match self {
            Self::None => animus_adapters::ReceiptStatus::Completed,
            Self::DeniedCapability => animus_adapters::ReceiptStatus::Denied,
            Self::MalformedOutput | Self::TimedOut | Self::Cancelled => {
                animus_adapters::ReceiptStatus::Failed
            }
            Self::MissingProviderRegistry
            | Self::MissingToolContract
            | Self::MissingPolicy
            | Self::MissingSandbox
            | Self::SandboxMismatch
            | Self::MissingReceiptContract
            | Self::MissingEventContract
            | Self::MissingHostApproval
            | Self::MissingCredentials => animus_adapters::ReceiptStatus::Blocked,
        }
    }

    const fn receipt_issue_code(self) -> animus_adapters::ReceiptIssueCode {
        match self {
            Self::None => animus_adapters::ReceiptIssueCode::None,
            Self::DeniedCapability => animus_adapters::ReceiptIssueCode::AuthorizationDenied,
            Self::MalformedOutput | Self::TimedOut | Self::Cancelled => {
                animus_adapters::ReceiptIssueCode::AdapterError
            }
            Self::MissingProviderRegistry
            | Self::MissingToolContract
            | Self::MissingPolicy
            | Self::MissingSandbox
            | Self::SandboxMismatch
            | Self::MissingReceiptContract
            | Self::MissingEventContract
            | Self::MissingHostApproval
            | Self::MissingCredentials => animus_adapters::ReceiptIssueCode::RuntimeBlocked,
        }
    }
}

/// Result verdict for a std-shell adapter plan.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterVerdict {
    /// All prerequisites are satisfied and dispatch may proceed.
    Allowed,
    /// Dispatch must not proceed because a typed issue was selected.
    Blocked {
        /// Typed issue that blocked dispatch.
        issue: AdapterIssue,
    },
}

impl AdapterVerdict {
    /// Returns true when the adapter may dispatch.
    #[must_use]
    pub const fn is_allowed(self) -> bool {
        match self {
            Self::Allowed => true,
            Self::Blocked { .. } => false,
        }
    }

    /// Returns the typed issue for this verdict.
    #[must_use]
    pub const fn issue(self) -> AdapterIssue {
        match self {
            Self::Allowed => AdapterIssue::None,
            Self::Blocked { issue } => issue,
        }
    }
}

/// Pure effect plan produced before a std-shell adapter performs work.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterPlan {
    family: AdapterFamily,
    verdict: AdapterVerdict,
    receipt: animus_adapters::SafeReceipt,
}

impl AdapterPlan {
    /// Returns the planned adapter family.
    #[must_use]
    pub const fn family(&self) -> AdapterFamily {
        self.family
    }

    /// Returns the plan verdict.
    #[must_use]
    pub const fn verdict(&self) -> AdapterVerdict {
        self.verdict
    }

    /// Returns the safe receipt for the plan.
    #[must_use]
    pub const fn receipt(&self) -> &animus_adapters::SafeReceipt {
        &self.receipt
    }
}

/// Plans one std-shell adapter effect from deterministic inputs.
///
/// The returned receipt contains only safe labels, counts, redaction markers, and
/// a BLAKE3 payload hash. Raw prompts, provider payloads, credentials, paths,
/// tool bodies, and model outputs are not copied into the receipt.
///
/// # Errors
///
/// Returns [`animus_adapters::SafeReceiptError`] when a caller supplied unsafe
/// metadata labels or count values that cannot be represented safely.
pub fn plan_adapter_effect(
    descriptor: AdapterDescriptor<'_>,
    prerequisites: EffectPrerequisites,
    payload: &str,
) -> Result<AdapterPlan, animus_adapters::SafeReceiptError> {
    let issue = select_issue(prerequisites);
    let verdict = verdict_from_issue(issue);
    let receipt = build_receipt(descriptor, issue, payload)?;

    Ok(AdapterPlan {
        family: descriptor.family(),
        verdict,
        receipt,
    })
}

/// Deterministic fixture adapter kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum FixtureKind {
    /// Fixture model behavior.
    Model,
    /// Fixture filesystem behavior.
    Filesystem,
    /// Fixture process behavior.
    Process,
    /// Fixture network behavior.
    Network,
    /// Fixture clock behavior.
    Clock,
    /// Fixture credential behavior.
    Credential,
}

impl FixtureKind {
    /// Returns the stable fixture label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Model => "model",
            Self::Filesystem => "filesystem",
            Self::Process => "process",
            Self::Network => "network",
            Self::Clock => "clock",
            Self::Credential => "credential",
        }
    }

    const fn descriptor(self) -> AdapterDescriptor<'static> {
        match self {
            Self::Model => AdapterDescriptor::fixture(FIXTURE_MODEL_LABEL),
            Self::Filesystem => AdapterDescriptor::fixture(FIXTURE_FILESYSTEM_LABEL),
            Self::Process => AdapterDescriptor::fixture(FIXTURE_PROCESS_LABEL),
            Self::Network => AdapterDescriptor::fixture(FIXTURE_NETWORK_LABEL),
            Self::Clock => AdapterDescriptor::fixture(FIXTURE_CLOCK_LABEL),
            Self::Credential => AdapterDescriptor::fixture(FIXTURE_CREDENTIAL_LABEL),
        }
    }

    const fn output_label(self) -> &'static str {
        match self {
            Self::Model => FIXTURE_MODEL_OUTPUT_LABEL,
            Self::Filesystem => FIXTURE_FILESYSTEM_OUTPUT_LABEL,
            Self::Process => FIXTURE_PROCESS_OUTPUT_LABEL,
            Self::Network => FIXTURE_NETWORK_OUTPUT_LABEL,
            Self::Clock => FIXTURE_CLOCK_OUTPUT_LABEL,
            Self::Credential => FIXTURE_CREDENTIAL_OUTPUT_LABEL,
        }
    }
}

/// Deterministic fixture output with a safe plan receipt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FixtureOutput {
    kind: FixtureKind,
    output_label: &'static str,
    plan: AdapterPlan,
}

impl FixtureOutput {
    /// Returns the fixture kind that produced this output.
    #[must_use]
    pub const fn kind(&self) -> FixtureKind {
        self.kind
    }

    /// Returns a safe output label rather than a raw payload.
    #[must_use]
    pub const fn output_label(&self) -> &'static str {
        self.output_label
    }

    /// Returns the adapter plan and safe receipt.
    #[must_use]
    pub const fn plan(&self) -> &AdapterPlan {
        &self.plan
    }
}

/// Deterministic in-memory fixture adapter bundle for default std-shell tests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FixtureAdapters {
    prerequisites: EffectPrerequisites,
}

impl FixtureAdapters {
    /// Builds fixture adapters with all gates satisfied.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            prerequisites: EffectPrerequisites::satisfied(),
        }
    }

    /// Builds fixture adapters with explicit prerequisite state.
    #[must_use]
    pub const fn with_prerequisites(prerequisites: EffectPrerequisites) -> Self {
        Self { prerequisites }
    }

    /// Dispatches a deterministic fixture request.
    ///
    /// # Errors
    ///
    /// Returns [`animus_adapters::SafeReceiptError`] when safe receipt metadata
    /// cannot be constructed.
    pub fn dispatch(
        &self,
        kind: FixtureKind,
        payload: &str,
    ) -> Result<FixtureOutput, animus_adapters::SafeReceiptError> {
        let plan = plan_adapter_effect(kind.descriptor(), self.prerequisites, payload)?;
        Ok(FixtureOutput {
            kind,
            output_label: kind.output_label(),
            plan,
        })
    }
}

impl Default for FixtureAdapters {
    fn default() -> Self {
        Self::new()
    }
}

/// Deterministic model fixture implementing the model adapter contract.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FixtureModelAdapter;

impl
    animus_adapters::ComponentAdapter<animus_adapters::ModelRequest, animus_adapters::ModelResponse>
    for FixtureModelAdapter
{
    type Error = animus_adapters::AdapterContractError;

    fn call(
        &mut self,
        _request: &animus_adapters::ModelRequest,
    ) -> Result<animus_adapters::ModelResponse, Self::Error> {
        animus_adapters::ModelResponse::new(FIXTURE_MODEL_MESSAGE)
    }
}

/// Deterministic tool fixture implementing the tool adapter contract.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct FixtureToolAdapter;

impl animus_adapters::ComponentAdapter<animus_adapters::ToolRequest, animus_adapters::ToolResponse>
    for FixtureToolAdapter
{
    type Error = animus_adapters::AdapterContractError;

    fn call(
        &mut self,
        _request: &animus_adapters::ToolRequest,
    ) -> Result<animus_adapters::ToolResponse, Self::Error> {
        animus_adapters::ToolResponse::new(FIXTURE_TOOL_OUTPUT)
    }
}

const fn verdict_from_issue(issue: AdapterIssue) -> AdapterVerdict {
    match issue {
        AdapterIssue::None => AdapterVerdict::Allowed,
        other => AdapterVerdict::Blocked { issue: other },
    }
}

const fn select_issue(prerequisites: EffectPrerequisites) -> AdapterIssue {
    match prerequisites.termination() {
        TerminationState::TimedOut => return AdapterIssue::TimedOut,
        TerminationState::Cancelled => return AdapterIssue::Cancelled,
        TerminationState::Continue => {}
    }
    match prerequisites.provider_registry() {
        GateState::Missing | GateState::Mismatch => {
            return AdapterIssue::MissingProviderRegistry;
        }
        GateState::Denied => return AdapterIssue::DeniedCapability,
        GateState::Satisfied => {}
    }
    match prerequisites.tool_contract() {
        GateState::Missing | GateState::Mismatch => return AdapterIssue::MissingToolContract,
        GateState::Denied => return AdapterIssue::DeniedCapability,
        GateState::Satisfied => {}
    }
    match prerequisites.policy() {
        GateState::Missing => return AdapterIssue::MissingPolicy,
        GateState::Denied | GateState::Mismatch => return AdapterIssue::DeniedCapability,
        GateState::Satisfied => {}
    }
    select_issue_after_policy(prerequisites)
}

const fn select_issue_after_policy(prerequisites: EffectPrerequisites) -> AdapterIssue {
    match prerequisites.sandbox() {
        GateState::Missing => return AdapterIssue::MissingSandbox,
        GateState::Denied | GateState::Mismatch => return AdapterIssue::SandboxMismatch,
        GateState::Satisfied => {}
    }
    match prerequisites.receipt_contract() {
        GateState::Missing | GateState::Mismatch => return AdapterIssue::MissingReceiptContract,
        GateState::Denied => return AdapterIssue::DeniedCapability,
        GateState::Satisfied => {}
    }
    match prerequisites.event_contract() {
        GateState::Missing | GateState::Mismatch => return AdapterIssue::MissingEventContract,
        GateState::Denied => return AdapterIssue::DeniedCapability,
        GateState::Satisfied => {}
    }
    select_issue_after_contracts(prerequisites)
}

const fn select_issue_after_contracts(prerequisites: EffectPrerequisites) -> AdapterIssue {
    match prerequisites.host_approval() {
        GateState::Missing => return AdapterIssue::MissingHostApproval,
        GateState::Denied | GateState::Mismatch => return AdapterIssue::DeniedCapability,
        GateState::Satisfied => {}
    }
    match prerequisites.credentials() {
        CredentialState::Missing => return AdapterIssue::MissingCredentials,
        CredentialState::NotRequired | CredentialState::Present => {}
    }
    match prerequisites.output() {
        OutputState::Malformed => AdapterIssue::MalformedOutput,
        OutputState::Valid => AdapterIssue::None,
    }
}

fn build_receipt(
    descriptor: AdapterDescriptor<'_>,
    issue: AdapterIssue,
    payload: &str,
) -> Result<animus_adapters::SafeReceipt, animus_adapters::SafeReceiptError> {
    let labels = [
        descriptor.family().as_str(),
        descriptor.label(),
        descriptor.exposure().as_str(),
        issue.as_str(),
    ];
    let counts = [animus_adapters::BoundedCountDraft::new(
        PAYLOAD_BYTES_COUNT,
        bounded_payload_bytes(payload),
    )];
    let payload_hash = payload_hash(payload);
    let hashes = [payload_hash];
    let draft = animus_adapters::SafeReceiptDraft::new(
        issue.receipt_status(),
        issue.receipt_issue_code(),
        descriptor.family().operation(),
    )
    .with_labels(&labels)
    .with_counts(&counts)
    .with_artifact_hashes(&hashes)
    .with_backend_label(descriptor.family().as_str())
    .with_interchange_label("animus_std")
    .with_redactions(descriptor.family().redactions());

    animus_adapters::SafeReceipt::new(draft)
}

fn payload_hash(payload: &str) -> animus_adapters::ArtifactHash {
    let hash = blake3::hash(payload.as_bytes());
    animus_adapters::ArtifactHash::from_bytes(hash.as_bytes())
}

fn bounded_payload_bytes(payload: &str) -> u32 {
    let payload_bytes = match u32::try_from(payload.len()) {
        Ok(value) => value,
        Err(_error) => animus_adapters::MAX_SAFE_RECEIPT_COUNT,
    };
    payload_bytes.min(animus_adapters::MAX_SAFE_RECEIPT_COUNT)
}

#[cfg(test)]
mod tests {
    use super::*;
    use animus_adapters::ComponentAdapter;

    const SAFE_PAYLOAD: &str = "safe fixture payload";
    const RAW_PROMPT_PAYLOAD: &str = "prompt: summarize private material";
    const SECRET_PAYLOAD: &str = "token=sk-redacted";
    const PRIVATE_PATH_PAYLOAD: &str = "/home/example/private.txt";
    const MODEL_REQUEST_TEXT: &str = "intent: build reliable systems";
    const TOOL_NAME: &str = "fixture-tool";
    const TOOL_INPUT: &str = "fixture input";
    const CONCRETE_DEPENDENCY_MARKERS: &[&str] = &[
        "reqwest",
        "ureq",
        "tokio",
        "redb",
        "rusqlite",
        "keyring",
        "octocrab",
        "crossterm",
    ];
    const REUSABLE_CARGO_TOMLS: &[(&str, &str)] = &[
        ("animus-core", include_str!("../../animus-core/Cargo.toml")),
        (
            "animus-adapters",
            include_str!("../../animus-adapters/Cargo.toml"),
        ),
        (
            "animus-runtime",
            include_str!("../../animus-runtime/Cargo.toml"),
        ),
    ];

    #[test]
    fn adapter_families_cover_required_shell_edges() {
        let families = [
            AdapterFamily::ModelProvider,
            AdapterFamily::FilesystemTool,
            AdapterFamily::ProcessExecution,
            AdapterFamily::HttpNetworkHelper,
            AdapterFamily::FixtureBackend,
        ];

        for family in families {
            assert!(family.is_shell_owned());
            assert!(!family.as_str().is_empty());
        }
    }

    #[test]
    fn effect_plan_allows_gated_model_dispatch() {
        let descriptor =
            AdapterDescriptor::host_owned(AdapterFamily::ModelProvider, "openai_class");
        let plan = plan_adapter_effect(
            descriptor,
            EffectPrerequisites::satisfied().with_credentials_present(),
            SAFE_PAYLOAD,
        )
        .expect("safe plan receipt should build");

        assert_eq!(plan.family(), AdapterFamily::ModelProvider);
        assert!(plan.verdict().is_allowed());
        assert_eq!(
            plan.receipt().status(),
            animus_adapters::ReceiptStatus::Completed
        );
        assert_eq!(plan.receipt().artifact_hashes().len(), usize::from(1_u8));
    }

    #[test]
    fn fixtures_cover_model_filesystem_process_network_clock_and_credentials() {
        let fixtures = FixtureAdapters::new();
        let kinds = [
            FixtureKind::Model,
            FixtureKind::Filesystem,
            FixtureKind::Process,
            FixtureKind::Network,
            FixtureKind::Clock,
            FixtureKind::Credential,
        ];

        for kind in kinds {
            let output = fixtures
                .dispatch(kind, SAFE_PAYLOAD)
                .expect("fixture output should be safe");
            assert_eq!(output.kind(), kind);
            assert!(output.plan().verdict().is_allowed());
            assert!(output.output_label().starts_with("fixture_"));
        }
    }

    #[test]
    fn fixture_model_and_tool_implement_injected_adapter_contracts() {
        let request = animus_adapters::ModelRequest::new(MODEL_REQUEST_TEXT)
            .expect("fixture model request should parse");
        let mut model = FixtureModelAdapter;
        let response = model.call(&request).expect("fixture model should respond");
        assert_eq!(response.message(), FIXTURE_MODEL_MESSAGE);

        let request = animus_adapters::ToolRequest::new(TOOL_NAME, TOOL_INPUT)
            .expect("fixture tool request should parse");
        let mut tool = FixtureToolAdapter;
        let response = tool.call(&request).expect("fixture tool should respond");
        assert_eq!(response.output(), FIXTURE_TOOL_OUTPUT);
    }

    #[test]
    fn missing_credentials_fail_closed_before_dispatch() {
        let descriptor =
            AdapterDescriptor::host_owned(AdapterFamily::HttpNetworkHelper, "http_api");
        let plan = plan_adapter_effect(
            descriptor,
            EffectPrerequisites::satisfied().with_missing_credentials(),
            SAFE_PAYLOAD,
        )
        .expect("blocked plan receipt should build");

        assert_eq!(
            plan.verdict(),
            AdapterVerdict::Blocked {
                issue: AdapterIssue::MissingCredentials,
            }
        );
        assert_eq!(
            plan.receipt().status(),
            animus_adapters::ReceiptStatus::Blocked
        );
    }

    #[test]
    fn denied_policy_sandbox_mismatch_malformed_output_timeout_and_cancel_fail_closed() {
        let descriptor = AdapterDescriptor::host_owned(AdapterFamily::ProcessExecution, "command");
        let cases = [
            (
                EffectPrerequisites::satisfied().with_policy_denied(),
                AdapterIssue::DeniedCapability,
            ),
            (
                EffectPrerequisites::satisfied().with_sandbox_mismatch(),
                AdapterIssue::SandboxMismatch,
            ),
            (
                EffectPrerequisites::satisfied().with_malformed_output(),
                AdapterIssue::MalformedOutput,
            ),
            (
                EffectPrerequisites::satisfied().with_timeout(),
                AdapterIssue::TimedOut,
            ),
            (
                EffectPrerequisites::satisfied().with_cancellation(),
                AdapterIssue::Cancelled,
            ),
        ];

        for (prerequisites, issue) in cases {
            let plan = plan_adapter_effect(descriptor, prerequisites, SAFE_PAYLOAD)
                .expect("failure receipt should be safe");
            assert_eq!(plan.verdict(), AdapterVerdict::Blocked { issue });
            assert!(!plan.verdict().is_allowed());
        }
    }

    #[test]
    fn safe_receipts_omit_raw_prompt_secret_path_and_payload_text() {
        let descriptor = AdapterDescriptor::host_owned(AdapterFamily::FilesystemTool, "read_file");
        let payload = format!("{RAW_PROMPT_PAYLOAD}\n{SECRET_PAYLOAD}\n{PRIVATE_PATH_PAYLOAD}");
        let plan = plan_adapter_effect(descriptor, EffectPrerequisites::satisfied(), &payload)
            .expect("receipt should hash payload without copying it");
        let summary = plan.receipt().review_summary();

        assert!(!summary.contains(RAW_PROMPT_PAYLOAD));
        assert!(!summary.contains(SECRET_PAYLOAD));
        assert!(!summary.contains(PRIVATE_PATH_PAYLOAD));
        assert!(summary.contains(animus_adapters::ReceiptRedaction::PayloadHashOnly.as_str()));
    }

    #[test]
    fn unsafe_metadata_labels_are_rejected() {
        let descriptor =
            AdapterDescriptor::host_owned(AdapterFamily::ModelProvider, RAW_PROMPT_PAYLOAD);
        let error = plan_adapter_effect(descriptor, EffectPrerequisites::satisfied(), SAFE_PAYLOAD)
            .expect_err("unsafe labels should be rejected");

        assert!(matches!(
            error,
            animus_adapters::SafeReceiptError::UnsafeMetadata { .. }
        ));
    }

    #[test]
    fn reusable_crates_do_not_depend_on_concrete_host_effect_crates() {
        for (crate_name, cargo_toml) in REUSABLE_CARGO_TOMLS {
            for marker in CONCRETE_DEPENDENCY_MARKERS {
                assert!(
                    !cargo_toml.contains(marker),
                    "{crate_name} must not depend on concrete host crate {marker}"
                );
            }
        }
    }
}
