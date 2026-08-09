use std::fmt::Write as _;

use animus_adapters::codegraph::{CodeGraphQueryResult, CodeGraphSymbolKind};
use animus_runtime::tool_executor_kit::{
    ToolCatalog, ToolDefinition, ToolDefinitionDraft, ToolExecutorKitError,
    ToolExecutorOutputPolicy, ToolMutationPosture,
};

/// Exact tool name exposed by the first read-only codebase host.
pub const CODEBASE_QUERY_TOOL_NAME: &str = "codebase_graph_query";
/// Exact current argument schema identity.
pub const CODEBASE_QUERY_SCHEMA_IDENTITY: &str = "animus.codebase-graph-query.args.v1";
/// Exact host executor binding label.
pub const CODEBASE_QUERY_EXECUTOR_BINDING: &str = "animus.codebase-graph-query.executor.v1";
/// Exact read-only runtime capability label.
pub const CODEBASE_QUERY_CAPABILITY: &str = "observe_read";
/// Exact symbol argument name.
pub const CODEBASE_QUERY_ARGUMENT_SYMBOL: &str = "symbol";
/// Exact symbol-kind argument name.
pub const CODEBASE_QUERY_ARGUMENT_KIND: &str = "kind";
/// Exact result-limit argument name.
pub const CODEBASE_QUERY_ARGUMENT_LIMIT: &str = "max_results";

const TOOL_LABEL: &str = "Codebase graph query";
const TOOL_DESCRIPTION: &str = "Find bounded syntactic Rust definitions and references";
const TOOL_PROMPT: &str = "Use codebase_graph_query for bounded Rust symbol locations.";
const TOOL_GUIDELINE: &str = "Treat results as syntactic facts, not semantic analysis.";
const TOOL_SOURCE: &str = "animus.codebase-graph-host.v1";
const TOOL_PROVENANCE: &str = "animus";
const TOOL_RESULT_SHAPE: &str = "animus.codebase-graph-query.result.v1";
const KIND_ANY: &str = "any";
const KIND_DEFINITION: &str = "definition";
const KIND_REFERENCE: &str = "reference";
const EVIDENCE_PREFIX: &str = "evidence_class=";
const TRUNCATED_PREFIX: &str = "truncated=";
const MATCH_PREFIX: &str = "match";
const OUTPUT_MAX_VISIBLE_CHARS: u32 = 4_096;
const OUTPUT_MAX_BYTES: u64 = 65_536;

/// Stable read-only codebase tool failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CodebaseToolError {
    /// Tool metadata or current request construction failed.
    Contract,
    /// Executor-kit catalog, planning, execution, or receipt validation failed.
    ExecutorKit(ToolExecutorKitError),
    /// The request selected a different tool, schema, capability, or mutation posture.
    UnsupportedRequest,
    /// The requested syntactic symbol class was not admitted.
    InvalidKind,
    /// The requested finite result limit was malformed or zero.
    InvalidLimit,
    /// The capability-scoped host could not open, index, or query the explicit root.
    HostFailed,
    /// The capability-scoped graph rejected the query.
    QueryFailed,
    /// Stable result rendering failed.
    RenderFailed,
}

impl From<ToolExecutorKitError> for CodebaseToolError {
    fn from(value: ToolExecutorKitError) -> Self {
        Self::ExecutorKit(value)
    }
}

/// Owned pure query decoded from one exact current tool request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodebaseToolQuery {
    symbol: String,
    kind: Option<CodeGraphSymbolKind>,
    max_results: usize,
}

impl CodebaseToolQuery {
    /// Returns the exact requested symbol.
    #[must_use]
    pub fn symbol(&self) -> &str {
        self.symbol.as_str()
    }

    /// Returns the optional syntactic symbol class.
    #[must_use]
    pub const fn kind(&self) -> Option<CodeGraphSymbolKind> {
        self.kind
    }

    /// Returns the requested finite result limit.
    #[must_use]
    pub const fn max_results(&self) -> usize {
        self.max_results
    }
}

/// Builds the exact active read-only tool catalog.
///
/// # Errors
///
/// Returns [`CodebaseToolError`] if static metadata violates adapter or executor-kit contracts.
pub fn build_codebase_query_catalog() -> Result<ToolCatalog, CodebaseToolError> {
    let metadata =
        animus_adapters::tool::Definition::validate(&animus_adapters::tool::DefinitionDraft {
            name: Some(CODEBASE_QUERY_TOOL_NAME),
            label: Some(TOOL_LABEL),
            description: Some(TOOL_DESCRIPTION),
            parameter_schema_identity: Some(CODEBASE_QUERY_SCHEMA_IDENTITY),
            prompt_snippet: Some(TOOL_PROMPT),
            prompt_guidelines: &[TOOL_GUIDELINE],
            source_identity: Some(TOOL_SOURCE),
            provenance: Some(TOOL_PROVENANCE),
            capability_labels: &[CODEBASE_QUERY_CAPABILITY],
            result_metadata_shape: Some(TOOL_RESULT_SHAPE),
            status: animus_adapters::tool::DefinitionStatus::Active,
        })
        .map_err(|_error| CodebaseToolError::Contract)?;
    let output_policy =
        ToolExecutorOutputPolicy::new(OUTPUT_MAX_VISIBLE_CHARS, OUTPUT_MAX_BYTES, false)?;
    let definition = ToolDefinition::from_draft(ToolDefinitionDraft {
        metadata,
        executor_binding: CODEBASE_QUERY_EXECUTOR_BINDING,
        argument_names: &[
            CODEBASE_QUERY_ARGUMENT_SYMBOL,
            CODEBASE_QUERY_ARGUMENT_KIND,
            CODEBASE_QUERY_ARGUMENT_LIMIT,
        ],
        mutation_posture: ToolMutationPosture::ReadOnly,
        output_policy,
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved,
    })?;
    ToolCatalog::new(vec![definition]).map_err(Into::into)
}

/// Builds one exact current-shape codebase query request.
///
/// # Errors
///
/// Returns [`CodebaseToolError`] when any supplied value violates the current tool contract.
pub fn build_codebase_query_request(
    symbol: &str,
    kind: &str,
    max_results: usize,
    correlation_id: &str,
) -> Result<animus_adapters::ToolRequest, CodebaseToolError> {
    let max_results = max_results.to_string();
    let fields = [
        animus_adapters::tool::ArgumentFieldDraft {
            name: Some(CODEBASE_QUERY_ARGUMENT_SYMBOL),
            value: Some(symbol),
        },
        animus_adapters::tool::ArgumentFieldDraft {
            name: Some(CODEBASE_QUERY_ARGUMENT_KIND),
            value: Some(kind),
        },
        animus_adapters::tool::ArgumentFieldDraft {
            name: Some(CODEBASE_QUERY_ARGUMENT_LIMIT),
            value: Some(max_results.as_str()),
        },
    ];
    let arguments = animus_adapters::tool::PreparedArguments::prepare(
        &animus_adapters::tool::ArgumentPreparationRequest {
            current_schema_identity: CODEBASE_QUERY_SCHEMA_IDENTITY,
            allowed_field_names: &[
                CODEBASE_QUERY_ARGUMENT_SYMBOL,
                CODEBASE_QUERY_ARGUMENT_KIND,
                CODEBASE_QUERY_ARGUMENT_LIMIT,
            ],
            compatibility_rules: &[],
            fields: &fields,
        },
    )
    .map_err(|_error| CodebaseToolError::Contract)?;
    animus_adapters::ToolRequest::from_draft(&animus_adapters::tool::RequestDraft {
        name: Some(CODEBASE_QUERY_TOOL_NAME),
        arguments: Some(&arguments),
        correlation_id: Some(correlation_id),
        output_policy: animus_adapters::tool::OutputPolicy {
            visible_char_budget: OUTPUT_MAX_VISIBLE_CHARS,
        },
        mutation_targets: &[],
        required_capability_labels: &[CODEBASE_QUERY_CAPABILITY],
        approval_posture: animus_adapters::tool::ApprovalPosture::PreApproved.as_str(),
    })
    .map_err(|_error| CodebaseToolError::Contract)
}

/// Decodes and revalidates exact request values before graph query effects.
///
/// # Errors
///
/// Returns [`CodebaseToolError`] for wrong metadata, malformed kinds, or invalid limits.
pub fn decode_codebase_query(
    request: &animus_adapters::ToolRequest,
) -> Result<CodebaseToolQuery, CodebaseToolError> {
    // r[impl agent_harness_runtime.read_only_codebase_tool.host]
    let exact_metadata = request.name() == CODEBASE_QUERY_TOOL_NAME
        && request.schema_identity() == CODEBASE_QUERY_SCHEMA_IDENTITY
        && request.mutation_targets().is_empty()
        && request.required_capability_labels() == [CODEBASE_QUERY_CAPABILITY]
        && request.approval_posture() == animus_adapters::tool::ApprovalPosture::PreApproved;
    if !exact_metadata {
        return Err(CodebaseToolError::UnsupportedRequest);
    }
    let symbol = argument_value(request, CODEBASE_QUERY_ARGUMENT_SYMBOL)?;
    let kind = match argument_value(request, CODEBASE_QUERY_ARGUMENT_KIND)? {
        KIND_ANY => None,
        KIND_DEFINITION => Some(CodeGraphSymbolKind::Definition),
        KIND_REFERENCE => Some(CodeGraphSymbolKind::Reference),
        _unsupported => return Err(CodebaseToolError::InvalidKind),
    };
    let max_results = argument_value(request, CODEBASE_QUERY_ARGUMENT_LIMIT)?
        .parse::<usize>()
        .map_err(|_error| CodebaseToolError::InvalidLimit)?;
    if max_results == 0 {
        return Err(CodebaseToolError::InvalidLimit);
    }
    Ok(CodebaseToolQuery {
        symbol: String::from(symbol),
        kind,
        max_results,
    })
}

/// Renders stable bounded syntactic facts without source bodies or absolute paths.
///
/// # Errors
///
/// Returns [`CodebaseToolError::RenderFailed`] if in-memory formatting fails.
pub fn render_codebase_query_result(
    result: &CodeGraphQueryResult,
) -> Result<String, CodebaseToolError> {
    let mut output = String::new();
    writeln!(output, "{EVIDENCE_PREFIX}{}", result.evidence_class)
        .map_err(|_error| CodebaseToolError::RenderFailed)?;
    writeln!(output, "{TRUNCATED_PREFIX}{}", result.truncated)
        .map_err(|_error| CodebaseToolError::RenderFailed)?;
    for symbol in &result.matches {
        writeln!(
            output,
            "{MATCH_PREFIX} kind={} path={} start_byte={} end_byte={}",
            symbol_kind_label(symbol.kind),
            symbol.relative_path,
            symbol.start_byte,
            symbol.end_byte,
        )
        .map_err(|_error| CodebaseToolError::RenderFailed)?;
    }
    Ok(output)
}

fn argument_value<'a>(
    request: &'a animus_adapters::ToolRequest,
    name: &str,
) -> Result<&'a str, CodebaseToolError> {
    request
        .arguments()
        .iter()
        .find(|argument| argument.name() == name)
        .map(animus_adapters::tool::ArgumentField::value)
        .ok_or(CodebaseToolError::UnsupportedRequest)
}

const fn symbol_kind_label(kind: CodeGraphSymbolKind) -> &'static str {
    match kind {
        CodeGraphSymbolKind::Definition => KIND_DEFINITION,
        CodeGraphSymbolKind::Reference => KIND_REFERENCE,
    }
}
