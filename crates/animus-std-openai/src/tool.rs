use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use serde::Serialize;
use serde_json::Value;

use crate::HostError;

/// Stable reviewed schema label for the model-facing Steel REPL function.
pub const STEEL_REPL_TOOL_SCHEMA: &str = animus_adapters::rlm_kernel::STEEL_REPL_TOOL_SCHEMA;
/// Stable reviewed model-facing Steel REPL function name.
pub const STEEL_REPL_TOOL_NAME: &str = animus_adapters::rlm_kernel::STEEL_REPL_TOOL_NAME;

const FUNCTION_TYPE: &str = "function";
const FUNCTION_CALL_OUTPUT_TYPE: &str = "function_call_output";
const INPUT_TEXT_TYPE: &str = "input_text";
pub(crate) const RESPONSE_FORMAT_TEXT: &str = "text";
const REASONING_NONE: &str = "none";
const ROLE_TOOL: &str = "tool";
const TOOL_CHOICE_AUTO: &str = "auto";
const TOOL_CHOICE_REQUIRED: &str = "required";
const MAX_SAFE_LABEL_CHARS: usize = 256;
const MAX_TOOL_CAPABILITIES: usize = 64;
const MAX_SCHEMA_BYTES: usize = 65_536;
const MAX_TOOL_RESULT_BYTES: usize = 262_144;

pub(crate) const FEATURE_ENDPOINT: &str = "endpoint";
pub(crate) const FEATURE_PROVIDER_FAMILY: &str = "provider_family";
pub(crate) const FEATURE_STREAMING: &str = "streaming";
pub(crate) const FEATURE_STRUCTURED_OUTPUT: &str = "structured_output";
pub(crate) const FEATURE_STOP_SEQUENCES: &str = "stop_sequences";
pub(crate) const FEATURE_MESSAGE_ROLE: &str = "message_role";
pub(crate) const FEATURE_TOOL_CAPABILITY: &str = "tool_capability";
pub(crate) const FEATURE_TOOL_SCHEMA: &str = "tool_schema";
pub(crate) const FEATURE_STREAM_CHANNEL: &str = "stream_channel";
pub(crate) const FEATURE_TOOL_RESULT_SHAPE: &str = "tool_result_shape";
pub(crate) const FEATURE_PARALLEL_TOOL_CALLS: &str = "parallel_tool_calls";
pub(crate) const FEATURE_TOOL_CHOICE: &str = "tool_choice";
pub(crate) const FEATURE_TOOL_COHORT: &str = "steel_repl_tool_cohort";

const INVALID_LABEL_REASON: &str = "OpenAI tool metadata contains an invalid label";
const DUPLICATE_CAPABILITY_REASON: &str = "OpenAI tool capability metadata contains a duplicate";
const CAPABILITY_BOUND_REASON: &str = "OpenAI tool capability metadata exceeds its bound";
const SCHEMA_JSON_REASON: &str = "OpenAI tool schema JSON is malformed";
const SCHEMA_SHAPE_REASON: &str = "OpenAI strict tool schema shape is unsupported";
const SCHEMA_BOUND_REASON: &str = "OpenAI tool schema exceeds its byte bound";
const RESULT_BOUND_REASON: &str = "OpenAI tool result exceeds its byte bound";
const CONTINUATION_MISMATCH_REASON: &str = "OpenAI continuation identities do not match";
const CONTINUATION_MESSAGE_REASON: &str =
    "OpenAI continuation requires one matching canonical tool result";
const INITIAL_TOOL_RESULT_REASON: &str = "OpenAI initial request cannot contain a tool result";
const TOOL_RESULT_TEXT_REASON: &str = "OpenAI tool result must contain exactly one text part";
const TOOL_RESULT_VALUE_REASON: &str =
    "OpenAI canonical tool result does not match the supplied result";

/// Exact provider stream channel admitted for a tool-capable Responses model.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum OpenAiToolStreamChannel {
    /// Assistant-visible text deltas.
    Content,
    /// Provider-exposed reasoning-summary deltas.
    ReasoningSummary,
    /// Function-call argument deltas.
    ToolCall,
    /// Token usage at terminal completion.
    Usage,
    /// One explicit terminal event.
    Finish,
}

/// Exact function name and schema label admitted for one selected model.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct OpenAiToolCapability {
    name: String,
    schema_label: String,
}

impl OpenAiToolCapability {
    /// Builds one exact tool capability.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when a label is empty, too long, or unsafe.
    pub fn new(name: &str, schema_label: &str) -> Result<Self, HostError> {
        validate_label(name)?;
        validate_label(schema_label)?;
        if name != STEEL_REPL_TOOL_NAME || schema_label != STEEL_REPL_TOOL_SCHEMA {
            return unsupported(FEATURE_TOOL_COHORT);
        }
        Ok(Self {
            name: String::from(name),
            schema_label: String::from(schema_label),
        })
    }

    /// Returns the exact function name.
    #[must_use]
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    /// Returns the exact canonical schema label.
    #[must_use]
    pub fn schema_label(&self) -> &str {
        self.schema_label.as_str()
    }
}

/// Exact selected-model capability metadata for the reviewed tool cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiToolCapabilityMatrix {
    model_id: String,
    roles: Vec<animus_adapters::CanonicalModelRole>,
    tools: Vec<OpenAiToolCapability>,
    stream_channels: Vec<OpenAiToolStreamChannel>,
}

impl OpenAiToolCapabilityMatrix {
    /// Builds exact selected-model metadata after bounded validation.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] for empty, duplicate, over-limit, or unsafe metadata.
    pub fn new(
        model_id: &str,
        roles: Vec<animus_adapters::CanonicalModelRole>,
        tools: Vec<OpenAiToolCapability>,
        stream_channels: Vec<OpenAiToolStreamChannel>,
    ) -> Result<Self, HostError> {
        validate_label(model_id)?;
        if roles.is_empty()
            || stream_channels.is_empty()
            || tools.len() != 1
            || tools.len() > MAX_TOOL_CAPABILITIES
            || roles.len() > MAX_TOOL_CAPABILITIES
            || stream_channels.len() > MAX_TOOL_CAPABILITIES
        {
            return Err(HostError::InvalidToolCohort {
                reason: CAPABILITY_BOUND_REASON,
            });
        }
        validate_unique_by(roles.as_slice(), |role| role.as_str())?;
        validate_unique(tools.as_slice())?;
        validate_unique(stream_channels.as_slice())?;
        Ok(Self {
            model_id: String::from(model_id),
            roles,
            tools,
            stream_channels,
        })
    }

    /// Returns the exact selected model identifier.
    #[must_use]
    pub fn model_id(&self) -> &str {
        self.model_id.as_str()
    }

    /// Returns the admitted canonical roles.
    #[must_use]
    pub fn roles(&self) -> &[animus_adapters::CanonicalModelRole] {
        self.roles.as_slice()
    }

    /// Returns exact tool and schema capabilities.
    #[must_use]
    pub fn tools(&self) -> &[OpenAiToolCapability] {
        self.tools.as_slice()
    }

    /// Returns exact stream channels.
    #[must_use]
    pub fn stream_channels(&self) -> &[OpenAiToolStreamChannel] {
        self.stream_channels.as_slice()
    }
}

/// Reviewed JSON schema bound to one canonical tool-schema label.
#[derive(Clone, Debug, PartialEq)]
pub struct OpenAiToolSchemaBinding {
    label: String,
    schema: Value,
}

impl OpenAiToolSchemaBinding {
    /// Parses and admits one strict function schema.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when the label, byte bound, JSON, or strict-object shape is invalid.
    pub fn from_json(label: &str, schema_json: &[u8]) -> Result<Self, HostError> {
        validate_label(label)?;
        if schema_json.len() > MAX_SCHEMA_BYTES {
            return Err(HostError::InvalidToolSchema {
                reason: SCHEMA_BOUND_REASON,
            });
        }
        let schema: Value =
            serde_json::from_slice(schema_json).map_err(|_error| HostError::InvalidToolSchema {
                reason: SCHEMA_JSON_REASON,
            })?;
        validate_strict_schema(&schema)?;
        Ok(Self {
            label: String::from(label),
            schema,
        })
    }

    /// Returns the canonical schema label.
    #[must_use]
    pub fn label(&self) -> &str {
        self.label.as_str()
    }

    pub(crate) const fn schema(&self) -> &Value {
        &self.schema
    }
}

/// Exact tool-selection policy for one provider request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiToolChoice {
    kind: OpenAiToolChoiceKind,
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum OpenAiToolChoiceKind {
    Auto,
    Required,
    Named(String),
}

impl OpenAiToolChoice {
    /// Lets the provider select no tool or one declared tool.
    #[must_use]
    pub const fn auto() -> Self {
        Self {
            kind: OpenAiToolChoiceKind::Auto,
        }
    }

    /// Requires the provider to select one declared tool.
    #[must_use]
    pub const fn required() -> Self {
        Self {
            kind: OpenAiToolChoiceKind::Required,
        }
    }

    /// Requires one exact declared tool.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when the tool name is invalid.
    pub fn named(name: &str) -> Result<Self, HostError> {
        validate_label(name)?;
        Ok(Self {
            kind: OpenAiToolChoiceKind::Named(String::from(name)),
        })
    }
}

/// Explicit serial tool policy for the reviewed provider cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OpenAiToolPolicy {
    choice: OpenAiToolChoice,
    parallel_tool_calls: bool,
}

impl OpenAiToolPolicy {
    /// Builds the reviewed serial tool policy.
    #[must_use]
    pub const fn serial(choice: OpenAiToolChoice) -> Self {
        Self {
            choice,
            parallel_tool_calls: false,
        }
    }

    /// Returns the exact tool choice.
    #[must_use]
    pub const fn choice(&self) -> &OpenAiToolChoice {
        &self.choice
    }

    /// Returns whether parallel function calls are allowed.
    #[must_use]
    pub const fn parallel_tool_calls(&self) -> bool {
        self.parallel_tool_calls
    }
}

/// Host-local identity binding for one committed provider function call.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenAiContinuationBinding {
    model_call_id: String,
    provider_response_id: String,
    output_item_id: String,
    tool_call_id: String,
    tool_name: String,
    arguments_identity: String,
    arguments_json: String,
}

impl OpenAiContinuationBinding {
    pub(crate) fn from_completed_call(call: &OpenAiCompletedToolCall) -> Self {
        Self {
            model_call_id: call.model_call_id.clone(),
            provider_response_id: call.provider_response_id.clone(),
            output_item_id: call.output_item_id.clone(),
            tool_call_id: call.tool_call_id.clone(),
            tool_name: call.tool_name.clone(),
            arguments_identity: blake3_identity(call.arguments_json.as_bytes()),
            arguments_json: call.arguments_json.clone(),
        }
    }

    /// Returns the Animus model-call identity.
    #[must_use]
    pub fn model_call_id(&self) -> &str {
        self.model_call_id.as_str()
    }

    /// Returns the provider response identity needed for `previous_response_id`.
    #[must_use]
    pub fn provider_response_id(&self) -> &str {
        self.provider_response_id.as_str()
    }

    /// Returns the provider output-item identity.
    #[must_use]
    pub fn output_item_id(&self) -> &str {
        self.output_item_id.as_str()
    }

    /// Returns the provider function-call identity.
    #[must_use]
    pub fn tool_call_id(&self) -> &str {
        self.tool_call_id.as_str()
    }

    /// Returns the admitted tool name.
    #[must_use]
    pub fn tool_name(&self) -> &str {
        self.tool_name.as_str()
    }

    /// Returns the BLAKE3 identity of the completed JSON arguments.
    #[must_use]
    pub fn arguments_identity(&self) -> &str {
        self.arguments_identity.as_str()
    }

    /// Returns the private completed arguments for chat-protocol continuations.
    #[must_use]
    pub(crate) fn arguments_json(&self) -> &str {
        self.arguments_json.as_str()
    }
}

impl fmt::Debug for OpenAiContinuationBinding {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenAiContinuationBinding")
            .field("model_call_id", &self.model_call_id)
            .field("provider_response_id", &"[REDACTED]")
            .field("output_item_id", &"[REDACTED]")
            .field("tool_call_id", &"[REDACTED]")
            .field("tool_name", &self.tool_name)
            .field("arguments_identity", &self.arguments_identity)
            .field("arguments_json", &"[REDACTED]")
            .finish()
    }
}

/// One bounded provider function call completed by the production parser.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenAiCompletedToolCall {
    pub(crate) model_call_id: String,
    pub(crate) provider_response_id: String,
    pub(crate) output_item_id: String,
    pub(crate) tool_call_id: String,
    pub(crate) tool_name: String,
    pub(crate) arguments_json: String,
}

impl OpenAiCompletedToolCall {
    /// Returns the exact function name.
    #[must_use]
    pub fn tool_name(&self) -> &str {
        self.tool_name.as_str()
    }

    /// Returns the completed JSON arguments for tool admission outside this adapter.
    #[must_use]
    pub fn arguments_json(&self) -> &str {
        self.arguments_json.as_str()
    }

    /// Returns a host-local continuation binding for this committed call.
    #[must_use]
    pub fn continuation_binding(&self) -> OpenAiContinuationBinding {
        OpenAiContinuationBinding::from_completed_call(self)
    }
}

impl fmt::Debug for OpenAiCompletedToolCall {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenAiCompletedToolCall")
            .field("model_call_id", &self.model_call_id)
            .field("provider_response_id", &"[REDACTED]")
            .field("output_item_id", &"[REDACTED]")
            .field("tool_call_id", &"[REDACTED]")
            .field("tool_name", &self.tool_name)
            .field("arguments_json", &"[REDACTED]")
            .finish()
    }
}

/// Explicit tool result and correlation facts for one continuation request.
#[derive(Clone, Eq, PartialEq)]
pub struct OpenAiToolResult {
    model_call_id: String,
    provider_response_id: String,
    output_item_id: String,
    tool_call_id: String,
    output: String,
}

impl OpenAiToolResult {
    /// Builds a result that matches one parser-issued binding.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when the output exceeds the admitted bound.
    pub fn for_binding(
        binding: &OpenAiContinuationBinding,
        output: &str,
    ) -> Result<Self, HostError> {
        for label in [
            binding.model_call_id(),
            binding.provider_response_id(),
            binding.output_item_id(),
            binding.tool_call_id(),
        ] {
            validate_label(label)?;
        }
        if output.len() > MAX_TOOL_RESULT_BYTES {
            return Err(HostError::InvalidToolCohort {
                reason: RESULT_BOUND_REASON,
            });
        }
        Ok(Self {
            model_call_id: String::from(binding.model_call_id()),
            provider_response_id: String::from(binding.provider_response_id()),
            output_item_id: String::from(binding.output_item_id()),
            tool_call_id: String::from(binding.tool_call_id()),
            output: String::from(output),
        })
    }

    /// Returns the private tool output for request projection.
    #[must_use]
    pub fn output(&self) -> &str {
        self.output.as_str()
    }
}

impl fmt::Debug for OpenAiToolResult {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenAiToolResult")
            .field("model_call_id", &self.model_call_id)
            .field("provider_response_id", &"[REDACTED]")
            .field("output_item_id", &"[REDACTED]")
            .field("tool_call_id", &"[REDACTED]")
            .field("output", &"[REDACTED]")
            .finish()
    }
}

/// Host-owned request admission for an initial or continuation tool call.
#[derive(Clone)]
pub struct OpenAiToolDispatch {
    model_call_id: String,
    capabilities: OpenAiToolCapabilityMatrix,
    schemas: Vec<OpenAiToolSchemaBinding>,
    policy: OpenAiToolPolicy,
    continuation: Option<(OpenAiContinuationBinding, OpenAiToolResult)>,
}

impl OpenAiToolDispatch {
    /// Builds an initial tool-capable dispatch admission.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when the model-call identity is invalid.
    pub fn initial(
        model_call_id: &str,
        capabilities: OpenAiToolCapabilityMatrix,
        schemas: Vec<OpenAiToolSchemaBinding>,
        policy: OpenAiToolPolicy,
    ) -> Result<Self, HostError> {
        validate_label(model_call_id)?;
        Ok(Self {
            model_call_id: String::from(model_call_id),
            capabilities,
            schemas,
            policy,
            continuation: None,
        })
    }

    /// Builds one exact continuation admission from an initial cohort.
    ///
    /// # Errors
    ///
    /// Returns [`HostError`] when the correlation does not match this model call.
    pub fn continue_with(
        mut self,
        binding: OpenAiContinuationBinding,
        result: OpenAiToolResult,
    ) -> Result<Self, HostError> {
        if self.model_call_id != binding.model_call_id() {
            return Err(HostError::Correlation {
                reason: CONTINUATION_MISMATCH_REASON,
            });
        }
        self.continuation = Some((binding, result));
        Ok(self)
    }

    /// Returns the Animus model-call identity.
    #[must_use]
    pub fn model_call_id(&self) -> &str {
        self.model_call_id.as_str()
    }

    /// Returns exact selected-model capability metadata.
    #[must_use]
    pub const fn capabilities(&self) -> &OpenAiToolCapabilityMatrix {
        &self.capabilities
    }

    /// Returns exact schema bindings.
    #[must_use]
    pub fn schemas(&self) -> &[OpenAiToolSchemaBinding] {
        self.schemas.as_slice()
    }

    /// Returns the exact serial tool policy.
    #[must_use]
    pub const fn policy(&self) -> &OpenAiToolPolicy {
        &self.policy
    }

    pub(crate) fn continuation_parts(
        &self,
    ) -> Option<(&OpenAiContinuationBinding, &OpenAiToolResult)> {
        self.continuation
            .as_ref()
            .map(|(binding, result)| (binding, result))
    }
}

impl fmt::Debug for OpenAiToolDispatch {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("OpenAiToolDispatch")
            .field("model_call_id", &self.model_call_id)
            .field("capabilities", &self.capabilities)
            .field("schemas", &self.schemas.len())
            .field("policy", &self.policy)
            .field("continuation", &self.continuation.is_some())
            .finish()
    }
}

#[derive(Serialize)]
struct ToolRequestBody {
    model: String,
    input: Vec<Value>,
    tools: Vec<WireTool>,
    tool_choice: Value,
    parallel_tool_calls: bool,
    stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reasoning: Option<WireReasoning>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty")]
    metadata: BTreeMap<String, String>,
}

#[derive(Serialize)]
struct WireTool {
    #[serde(rename = "type")]
    kind: &'static str,
    name: String,
    description: String,
    parameters: Value,
    strict: bool,
}

#[derive(Serialize)]
struct WireReasoning {
    effort: String,
    summary: &'static str,
}

/// Builds deterministic JSON for the reviewed streaming tool cohort.
///
/// # Errors
///
/// Returns [`HostError`] before transport for unsupported, lossy, stale, or mismatched input.
pub fn build_tool_capable_openai_request_body(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: &OpenAiToolDispatch,
) -> Result<Vec<u8>, HostError> {
    // r[impl agent_harness_runtime.tool_capable_openai_host.capabilities]
    // r[impl agent_harness_runtime.tool_capable_openai_host.projection]
    validate_tool_plan(config, plan, dispatch)?;
    let request = plan.request();
    let schema_map = schema_map(dispatch.schemas())?;
    let tools = request
        .tools()
        .iter()
        .map(|tool| project_tool(tool, &schema_map))
        .collect::<Result<Vec<_>, _>>()?;
    let (input, previous_response_id) = project_input(request, dispatch)?;
    let body = ToolRequestBody {
        model: String::from(request.model()),
        input,
        tools,
        tool_choice: project_tool_choice(dispatch.policy().choice(), request)?,
        parallel_tool_calls: dispatch.policy().parallel_tool_calls(),
        stream: true,
        previous_response_id,
        reasoning: project_reasoning(config, request.reasoning_effort()),
        max_output_tokens: request.max_output_tokens(),
        temperature: request
            .temperature_millis()
            .map(|value| f64::from(value) / 1_000.0),
        metadata: project_metadata(request.metadata())?,
    };
    serde_json::to_vec(&body).map_err(|_error| HostError::RequestEncoding)
}

fn validate_tool_plan(
    config: &crate::OpenAiHostConfig,
    plan: &animus_std::ProviderDispatchPlan,
    dispatch: &OpenAiToolDispatch,
) -> Result<(), HostError> {
    if plan.endpoint() != config.endpoint_identity() {
        return unsupported(FEATURE_ENDPOINT);
    }
    if plan.api_kind() != animus_std::ApiKind::OpenAiResponses {
        return unsupported(FEATURE_PROVIDER_FAMILY);
    }
    let request = plan.request();
    if request.model() != dispatch.capabilities().model_id() {
        return unsupported(FEATURE_TOOL_CAPABILITY);
    }
    if !request.stream() {
        return unsupported(FEATURE_STREAMING);
    }
    if request.tools().is_empty() {
        return unsupported(FEATURE_TOOL_CAPABILITY);
    }
    if request.response_format() != RESPONSE_FORMAT_TEXT {
        return unsupported(FEATURE_STRUCTURED_OUTPUT);
    }
    if !request.stop_sequences().is_empty() {
        return unsupported(FEATURE_STOP_SEQUENCES);
    }
    if dispatch.policy().parallel_tool_calls() {
        return unsupported(FEATURE_PARALLEL_TOOL_CALLS);
    }
    validate_roles(request, dispatch.capabilities())?;
    validate_tools(request, dispatch.capabilities(), dispatch.schemas())?;
    validate_stream_channels(request, dispatch.capabilities())?;
    validate_continuation(dispatch, request)
}

pub(crate) fn validate_roles(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    capabilities: &OpenAiToolCapabilityMatrix,
) -> Result<(), HostError> {
    for message in request.messages() {
        let role =
            animus_adapters::CanonicalModelRole::parse(message.role()).map_err(|_error| {
                HostError::UnsupportedPlan {
                    feature: FEATURE_MESSAGE_ROLE,
                }
            })?;
        if !capabilities.roles().contains(&role) {
            return unsupported(FEATURE_MESSAGE_ROLE);
        }
    }
    Ok(())
}

pub(crate) fn validate_tools(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    capabilities: &OpenAiToolCapabilityMatrix,
    schemas: &[OpenAiToolSchemaBinding],
) -> Result<(), HostError> {
    if request.tools().len() != schemas.len() {
        return unsupported(FEATURE_TOOL_SCHEMA);
    }
    for tool in request.tools() {
        let capability = OpenAiToolCapability::new(tool.name(), tool.input_schema_label())?;
        if !capabilities.tools().contains(&capability)
            || !schemas
                .iter()
                .any(|binding| binding.label() == tool.input_schema_label())
        {
            return unsupported(FEATURE_TOOL_CAPABILITY);
        }
    }
    Ok(())
}

pub(crate) fn validate_stream_channels(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    capabilities: &OpenAiToolCapabilityMatrix,
) -> Result<(), HostError> {
    let mut required = vec![
        OpenAiToolStreamChannel::Content,
        OpenAiToolStreamChannel::ToolCall,
        OpenAiToolStreamChannel::Usage,
        OpenAiToolStreamChannel::Finish,
    ];
    if request.reasoning_effort() != REASONING_NONE {
        required.push(OpenAiToolStreamChannel::ReasoningSummary);
    }
    if required
        .iter()
        .any(|channel| !capabilities.stream_channels().contains(channel))
    {
        return unsupported(FEATURE_STREAM_CHANNEL);
    }
    Ok(())
}

pub(crate) fn validate_continuation(
    dispatch: &OpenAiToolDispatch,
    request: &animus_std::OpenAiResponsesDispatchRequest,
) -> Result<(), HostError> {
    match dispatch.continuation_parts() {
        None => {
            if request
                .messages()
                .iter()
                .any(|message| message.role() == ROLE_TOOL)
            {
                return Err(HostError::Correlation {
                    reason: INITIAL_TOOL_RESULT_REASON,
                });
            }
            Ok(())
        }
        Some((binding, result)) => {
            if dispatch.model_call_id() != binding.model_call_id()
                || result.model_call_id != binding.model_call_id
                || result.provider_response_id != binding.provider_response_id
                || result.output_item_id != binding.output_item_id
                || result.tool_call_id != binding.tool_call_id
            {
                return Err(HostError::Correlation {
                    reason: CONTINUATION_MISMATCH_REASON,
                });
            }
            let [message] = request.messages() else {
                return Err(HostError::Correlation {
                    reason: CONTINUATION_MESSAGE_REASON,
                });
            };
            if message.role() != ROLE_TOOL || message.tool_call_id() != Some(binding.tool_call_id())
            {
                return Err(HostError::Correlation {
                    reason: CONTINUATION_MESSAGE_REASON,
                });
            }
            let output = tool_result_text(message)?;
            if output != result.output() {
                return Err(HostError::Correlation {
                    reason: TOOL_RESULT_VALUE_REASON,
                });
            }
            Ok(())
        }
    }
}

fn project_input(
    request: &animus_std::OpenAiResponsesDispatchRequest,
    dispatch: &OpenAiToolDispatch,
) -> Result<(Vec<Value>, Option<String>), HostError> {
    match dispatch.continuation_parts() {
        None => Ok((
            request
                .messages()
                .iter()
                .map(project_regular_message)
                .collect::<Result<Vec<_>, _>>()?,
            None,
        )),
        Some((binding, result)) => Ok((
            vec![serde_json::json!({
                "type": FUNCTION_CALL_OUTPUT_TYPE,
                "call_id": binding.tool_call_id(),
                "output": result.output(),
            })],
            Some(String::from(binding.provider_response_id())),
        )),
    }
}

fn project_regular_message(
    message: &animus_std::OpenAiResponsesMessage,
) -> Result<Value, HostError> {
    if message.role() == ROLE_TOOL || message.tool_call_id().is_some() {
        return unsupported(FEATURE_TOOL_RESULT_SHAPE);
    }
    let mut content = Vec::with_capacity(message.content().len());
    for part in message.content() {
        match part {
            animus_std::OpenAiResponsesContentPart::Text { text } => {
                content.push(serde_json::json!({"type": INPUT_TEXT_TYPE, "text": text}));
            }
            animus_std::OpenAiResponsesContentPart::ImageDescriptor { .. } => {
                return unsupported("image");
            }
        }
    }
    Ok(serde_json::json!({"role": message.role(), "content": content}))
}

fn project_tool(
    tool: &animus_std::OpenAiResponsesTool,
    schemas: &BTreeMap<&str, &Value>,
) -> Result<WireTool, HostError> {
    let parameters =
        schemas
            .get(tool.input_schema_label())
            .ok_or(HostError::InvalidToolSchema {
                reason: SCHEMA_SHAPE_REASON,
            })?;
    Ok(WireTool {
        kind: FUNCTION_TYPE,
        name: String::from(tool.name()),
        description: String::from(tool.description()),
        parameters: (*parameters).clone(),
        strict: true,
    })
}

fn project_tool_choice(
    choice: &OpenAiToolChoice,
    request: &animus_std::OpenAiResponsesDispatchRequest,
) -> Result<Value, HostError> {
    match &choice.kind {
        OpenAiToolChoiceKind::Auto => Ok(Value::String(String::from(TOOL_CHOICE_AUTO))),
        OpenAiToolChoiceKind::Required => Ok(Value::String(String::from(TOOL_CHOICE_REQUIRED))),
        OpenAiToolChoiceKind::Named(name) => {
            if !request.tools().iter().any(|tool| tool.name() == name) {
                return unsupported(FEATURE_TOOL_CHOICE);
            }
            Ok(serde_json::json!({"type": FUNCTION_TYPE, "name": name}))
        }
    }
}

pub(crate) fn project_chat_tool_choice(
    choice: &OpenAiToolChoice,
    request: &animus_std::OpenAiResponsesDispatchRequest,
) -> Result<Value, HostError> {
    match &choice.kind {
        OpenAiToolChoiceKind::Auto => Ok(Value::String(String::from(TOOL_CHOICE_AUTO))),
        OpenAiToolChoiceKind::Required => Ok(Value::String(String::from(TOOL_CHOICE_REQUIRED))),
        OpenAiToolChoiceKind::Named(name) => {
            if !request.tools().iter().any(|tool| tool.name() == name) {
                return unsupported(FEATURE_TOOL_CHOICE);
            }
            Ok(serde_json::json!({
                "type": FUNCTION_TYPE,
                "function": {"name": name}
            }))
        }
    }
}

fn project_reasoning(config: &crate::OpenAiHostConfig, effort: &str) -> Option<WireReasoning> {
    debug_assert!(effort == REASONING_NONE || !effort.is_empty());
    config
        .projected_reasoning_effort(effort)
        .map(|projected| WireReasoning {
            effort: String::from(projected),
            summary: "auto",
        })
}

fn project_metadata(
    metadata: &[animus_std::OpenAiResponsesMetadata],
) -> Result<BTreeMap<String, String>, HostError> {
    let mut output = BTreeMap::new();
    for entry in metadata {
        if output
            .insert(String::from(entry.key()), String::from(entry.value()))
            .is_some()
        {
            return Err(HostError::InvalidToolCohort {
                reason: DUPLICATE_CAPABILITY_REASON,
            });
        }
    }
    Ok(output)
}

fn tool_result_text(message: &animus_std::OpenAiResponsesMessage) -> Result<&str, HostError> {
    let [part] = message.content() else {
        return Err(HostError::Correlation {
            reason: TOOL_RESULT_TEXT_REASON,
        });
    };
    match part {
        animus_std::OpenAiResponsesContentPart::Text { text } => Ok(text.as_str()),
        animus_std::OpenAiResponsesContentPart::ImageDescriptor { .. } => {
            Err(HostError::Correlation {
                reason: TOOL_RESULT_TEXT_REASON,
            })
        }
    }
}

pub(crate) fn schema_map(
    schemas: &[OpenAiToolSchemaBinding],
) -> Result<BTreeMap<&str, &Value>, HostError> {
    let mut output = BTreeMap::new();
    for binding in schemas {
        if output.insert(binding.label(), binding.schema()).is_some() {
            return Err(HostError::InvalidToolSchema {
                reason: DUPLICATE_CAPABILITY_REASON,
            });
        }
    }
    Ok(output)
}

fn validate_strict_schema(schema: &Value) -> Result<(), HostError> {
    validate_schema_node(schema, true)
}

fn validate_schema_node(schema: &Value, root: bool) -> Result<(), HostError> {
    let Some(object) = schema.as_object() else {
        return invalid_schema_shape();
    };
    let object_schema = object.get("type").and_then(Value::as_str) == Some("object");
    if root && !object_schema {
        return invalid_schema_shape();
    }
    if object_schema {
        let properties = object.get("properties").and_then(Value::as_object).ok_or(
            HostError::InvalidToolSchema {
                reason: SCHEMA_SHAPE_REASON,
            },
        )?;
        let required = object.get("required").and_then(Value::as_array).ok_or(
            HostError::InvalidToolSchema {
                reason: SCHEMA_SHAPE_REASON,
            },
        )?;
        if object.get("additionalProperties").and_then(Value::as_bool) != Some(false) {
            return invalid_schema_shape();
        }
        let required_names = required
            .iter()
            .map(Value::as_str)
            .collect::<Option<BTreeSet<_>>>()
            .ok_or(HostError::InvalidToolSchema {
                reason: SCHEMA_SHAPE_REASON,
            })?;
        let property_names = properties
            .keys()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if required_names != property_names {
            return invalid_schema_shape();
        }
        for property in properties.values() {
            validate_schema_node(property, false)?;
        }
    }
    if object.get("type").and_then(Value::as_str) == Some("array") {
        validate_schema_node(
            object.get("items").ok_or(HostError::InvalidToolSchema {
                reason: SCHEMA_SHAPE_REASON,
            })?,
            false,
        )?;
    }
    for union_key in ["anyOf", "oneOf", "allOf"] {
        if let Some(union) = object.get(union_key) {
            let choices = union.as_array().ok_or(HostError::InvalidToolSchema {
                reason: SCHEMA_SHAPE_REASON,
            })?;
            if choices.is_empty() {
                return invalid_schema_shape();
            }
            for choice in choices {
                validate_schema_node(choice, false)?;
            }
        }
    }
    if let Some(definitions) = object.get("$defs") {
        let definitions = definitions
            .as_object()
            .ok_or(HostError::InvalidToolSchema {
                reason: SCHEMA_SHAPE_REASON,
            })?;
        for definition in definitions.values() {
            validate_schema_node(definition, false)?;
        }
    }
    Ok(())
}

fn invalid_schema_shape<T>() -> Result<T, HostError> {
    Err(HostError::InvalidToolSchema {
        reason: SCHEMA_SHAPE_REASON,
    })
}

fn validate_label(value: &str) -> Result<(), HostError> {
    if value.is_empty()
        || value.chars().count() > MAX_SAFE_LABEL_CHARS
        || value
            .chars()
            .any(|character| character.is_control() || character.is_whitespace())
    {
        return Err(HostError::InvalidToolCohort {
            reason: INVALID_LABEL_REASON,
        });
    }
    Ok(())
}

fn validate_unique<T: Ord>(values: &[T]) -> Result<(), HostError> {
    let mut unique = BTreeSet::new();
    for value in values {
        if !unique.insert(value) {
            return Err(HostError::InvalidToolCohort {
                reason: DUPLICATE_CAPABILITY_REASON,
            });
        }
    }
    Ok(())
}

fn validate_unique_by<T, K: Ord>(values: &[T], key: impl Fn(&T) -> K) -> Result<(), HostError> {
    let mut unique = BTreeSet::new();
    for value in values {
        if !unique.insert(key(value)) {
            return Err(HostError::InvalidToolCohort {
                reason: DUPLICATE_CAPABILITY_REASON,
            });
        }
    }
    Ok(())
}

fn unsupported<T>(feature: &'static str) -> Result<T, HostError> {
    Err(HostError::UnsupportedPlan { feature })
}

pub(crate) fn blake3_identity(bytes: &[u8]) -> String {
    let mut output = String::from("BLAKE3:");
    output.push_str(blake3::hash(bytes).to_hex().as_str());
    output
}
