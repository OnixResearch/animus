use crate::{PueueHost, PueueProcessService};
use animus_adapters::{ComponentAdapter, ServiceOutcome};
use animus_runtime::tool_executor_kit::{
    ToolCatalog, ToolDefinition, ToolDefinitionDraft, ToolExecutionOutcome, ToolExecutionPlan,
    ToolExecutionPlanRequest, ToolExecutorAdmission, ToolExecutorBinding, ToolExecutorKitError,
    ToolExecutorOutput, ToolExecutorOutputPolicy, ToolExecutorReadiness, ToolExecutorService,
    ToolExecutorServiceFailure, ToolMutationPosture, execute_admitted_tool, plan_tool_execution,
};

#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "safe pueue response rendering handles every closed queue variant and redaction marker"
    )
)]
mod render;
#[cfg_attr(
    animus_octet,
    allow(
        tigerstyle::assertion_density,
        tigerstyle::non_trait_imports,
        tigerstyle::path_segment_repetition,
        reason = "closed pueue request decoding returns typed errors for every adversarial field and target"
    )
)]
mod request;

/// Exact pueue tool name.
pub const PUEUE_TOOL_NAME: &str = "pueue";
/// Exact pueue tool argument schema identity.
pub const PUEUE_TOOL_SCHEMA_IDENTITY: &str = "animus.pueue-tool.args.v1";
/// Exact pueue tool executor binding.
pub const PUEUE_TOOL_EXECUTOR_BINDING: &str = "animus.pueue-tool.executor.v1";
/// Exact pueue host capability label.
pub const PUEUE_TOOL_CAPABILITY: &str = "queue-control";

const PUEUE_TOOL_ARGUMENT_REQUEST: &str = "request_json";
const TOOL_LABEL: &str = "Pueue queue";
const TOOL_DESCRIPTION: &str = "Run reviewed background profiles and inspect bounded queue state";
const TOOL_PROMPT: &str = "Use pueue only with reviewed profile names and explicit targets.";
const TOOL_GUIDELINE: &str = "Do not supply raw commands or implicit mutation targets.";
const TOOL_SOURCE: &str = "animus.pueue-host.v1";
const TOOL_PROVENANCE: &str = "animus";
const TOOL_RESULT_SHAPE: &str = "animus.pueue-tool.result.v1";
const TOOL_MAX_VISIBLE_CHARS: u32 = 8_192;
const TOOL_MAX_OUTPUT_BYTES: u64 = 65_536;
const UNSAFE_OUTPUT_MARKERS: &[&str] = &[
    "RAW_PROMPT:",
    "RAW_FILE_BODY:",
    "RAW_STDOUT:",
    "RAW_STDERR:",
    "PROVIDER_PAYLOAD:",
    "CREDENTIAL:",
    "BEGIN PRIVATE KEY",
    "API_KEY=",
    "AWS_SECRET_ACCESS_KEY",
    "sk-",
    "/home/",
    "/Users/",
    ":\\",
];

/// Stable pueue tool failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PueueToolError {
    /// Static tool metadata or request construction failed.
    Contract,
    /// Tool JSON failed exact request decoding.
    InvalidRequest,
    /// Executor-kit admission, execution, output, or receipt validation failed.
    ExecutorKit(ToolExecutorKitError),
}

impl From<ToolExecutorKitError> for PueueToolError {
    fn from(value: ToolExecutorKitError) -> Self {
        Self::ExecutorKit(value)
    }
}

/// Tool executor over one explicit pueue host.
pub struct PueueToolExecutor<Service> {
    host: PueueHost<Service>,
}

impl<Service> PueueToolExecutor<Service> {
    /// Wraps one explicit pueue host.
    #[must_use]
    pub const fn new(host: PueueHost<Service>) -> Self {
        Self { host }
    }

    /// Returns the wrapped host.
    #[must_use]
    pub const fn host(&self) -> &PueueHost<Service> {
        &self.host
    }

    /// Returns the wrapped host mutably.
    #[must_use]
    pub const fn host_mut(&mut self) -> &mut PueueHost<Service> {
        &mut self.host
    }
}

impl<Service> ToolExecutorService for PueueToolExecutor<Service>
where
    Service: PueueProcessService,
{
    fn binding_label(&self) -> &str {
        PUEUE_TOOL_EXECUTOR_BINDING
    }

    fn execute(
        &mut self,
        _plan: &ToolExecutionPlan,
        request: &animus_adapters::ToolRequest,
    ) -> Result<ToolExecutorOutput, ToolExecutorServiceFailure> {
        let queue_request = request::decode(self.host.config(), request)
            .map_err(|_error| ToolExecutorServiceFailure::ExecutionFailed)?;
        let outcome = match self.host.call(&queue_request) {
            Ok(value) => value,
            Err(never) => match never {},
        };
        let response = match outcome {
            ServiceOutcome::Completed { value, .. } => value,
            ServiceOutcome::Cancelled { .. } => {
                return Err(ToolExecutorServiceFailure::Cancelled);
            }
            ServiceOutcome::TimedOut { .. } => {
                return Err(ToolExecutorServiceFailure::TimedOut);
            }
            ServiceOutcome::Absent { .. }
            | ServiceOutcome::Unavailable { .. }
            | ServiceOutcome::Denied { .. }
            | ServiceOutcome::MalformedOutput { .. } => {
                return Err(ToolExecutorServiceFailure::ExecutionFailed);
            }
        };
        let body = render::response(&response)
            .map_err(|_error| ToolExecutorServiceFailure::ExecutionFailed)?;
        Ok(ToolExecutorOutput::new(
            body.as_str(),
            None,
            animus_adapters::tool::OutputRedactionClass::Redacted,
        ))
    }
}

/// Builds the exact active pueue tool catalog.
///
/// # Errors
///
/// Returns [`PueueToolError`] if static metadata violates Animus contracts.
pub fn build_pueue_tool_catalog() -> Result<ToolCatalog, PueueToolError> {
    let metadata =
        animus_adapters::tool::Definition::validate(&animus_adapters::tool::DefinitionDraft {
            name: Some(PUEUE_TOOL_NAME),
            label: Some(TOOL_LABEL),
            description: Some(TOOL_DESCRIPTION),
            parameter_schema_identity: Some(PUEUE_TOOL_SCHEMA_IDENTITY),
            prompt_snippet: Some(TOOL_PROMPT),
            prompt_guidelines: &[TOOL_GUIDELINE],
            source_identity: Some(TOOL_SOURCE),
            provenance: Some(TOOL_PROVENANCE),
            capability_labels: &[PUEUE_TOOL_CAPABILITY],
            result_metadata_shape: Some(TOOL_RESULT_SHAPE),
            status: animus_adapters::tool::DefinitionStatus::Active,
        })
        .map_err(|_error| PueueToolError::Contract)?;
    let output_policy =
        ToolExecutorOutputPolicy::new(TOOL_MAX_VISIBLE_CHARS, TOOL_MAX_OUTPUT_BYTES, false)?;
    let definition = ToolDefinition::from_draft(ToolDefinitionDraft {
        metadata,
        executor_binding: PUEUE_TOOL_EXECUTOR_BINDING,
        argument_names: &[PUEUE_TOOL_ARGUMENT_REQUEST],
        mutation_posture: ToolMutationPosture::DeclaredTargets,
        output_policy,
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved,
    })?;
    ToolCatalog::new(vec![definition]).map_err(Into::into)
}

/// Named inputs for one exact pueue tool request.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PueueToolRequestDraft<'a> {
    /// Exact closed request JSON.
    pub request_json: &'a str,
    /// Explicit namespace mutation target.
    pub namespace_target: &'a str,
    /// Tool-call correlation identity.
    pub correlation_id: &'a str,
}

/// Builds one exact pueue tool request.
///
/// # Errors
///
/// Returns [`PueueToolError`] when the supplied JSON or labels violate the tool contract.
pub fn build_pueue_tool_request(
    draft: PueueToolRequestDraft<'_>,
) -> Result<animus_adapters::ToolRequest, PueueToolError> {
    let fields = [animus_adapters::tool::ArgumentFieldDraft {
        name: Some(PUEUE_TOOL_ARGUMENT_REQUEST),
        value: Some(draft.request_json),
    }];
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: PUEUE_TOOL_SCHEMA_IDENTITY,
            allowed_field_names: &[PUEUE_TOOL_ARGUMENT_REQUEST],
            compatibility_rules: &[],
            fields: &fields,
        },
    )
    .map_err(|_error| PueueToolError::Contract)?;
    animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(PUEUE_TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some(draft.correlation_id),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: TOOL_MAX_VISIBLE_CHARS,
        },
        mutation_targets: &[draft.namespace_target],
        required_capability_labels: &[PUEUE_TOOL_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .map_err(|_error| PueueToolError::Contract)
}

/// Plans and executes one exact pueue tool request through the executor kit.
///
/// # Errors
///
/// Returns [`PueueToolError`] for catalog, admission, request, host, output, or receipt failures.
pub fn execute_pueue_tool<Service>(
    executor: &mut PueueToolExecutor<Service>,
    request: &animus_adapters::ToolRequest,
    admission: ToolExecutorAdmission,
) -> Result<ToolExecutionOutcome, PueueToolError>
where
    Service: PueueProcessService,
{
    let catalog = build_pueue_tool_catalog()?;
    let registry = animus_runtime::RuntimeCapabilityRegistry::from_drafts(&[
        animus_runtime::RuntimeCapabilityDescriptorDraft {
            label: PUEUE_TOOL_CAPABILITY,
            service_kind: animus_runtime::RuntimeCapabilityServiceKind::Tool.as_str(),
            scope: None,
        },
    ])
    .map_err(|_error| PueueToolError::Contract)?;
    let binding =
        ToolExecutorBinding::new(PUEUE_TOOL_EXECUTOR_BINDING, ToolExecutorReadiness::Ready)?;
    let plan = plan_tool_execution(ToolExecutionPlanRequest {
        request,
        catalog: &catalog,
        capabilities: &registry,
        executors: &[binding],
        admission,
    });
    execute_admitted_tool(executor, &plan, request).map_err(Into::into)
}
