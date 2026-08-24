use std::collections::BTreeSet;

use serde_json::Value;

use super::{DiscoveredCatalog, DiscoveredTool, DiscoveryError};
use crate::config::framed_hash;
use crate::{LifecycleBinding, McpHostConfig, ToolPolicy};

const JSON_RPC_VERSION: &str = "2.0";
const SCHEMA_DOMAIN: &str = "animus.mcp-client-host.schema.v1";
const DESCRIPTION_DOMAIN: &str = "animus.mcp-client-host.description.v1";
const CATALOG_DOMAIN: &str = "animus.mcp-client-host.catalog.v1";
const SOURCE_PREFIX: &str = "mcp";

/// Borrowed bounded discovery-validation input.
#[derive(Clone, Copy)]
pub struct DiscoveryRequest<'a> {
    /// Exact raw response frame under the configured byte bound.
    pub bytes: &'a [u8],
    /// Exact JSON-RPC response identity.
    pub expected_response_id: u64,
    /// Current validated configuration.
    pub config: &'a McpHostConfig,
    /// Current lifecycle generation binding.
    pub binding: &'a LifecycleBinding,
    /// Current catalog when validating same-generation rediscovery.
    pub previous: Option<&'a DiscoveredCatalog>,
}

struct DiscoveryAccumulator {
    names: BTreeSet<String>,
    schema_bytes_total: usize,
}

/// Validates a complete discovery response and maps every descriptor through existing contracts.
///
/// # Errors
///
/// Returns a bounded discovery error without retaining raw server diagnostics.
// r[impl agent_harness_runtime.mcp_client_host.discovery]
pub fn validate_discovery(
    request: DiscoveryRequest<'_>,
) -> Result<DiscoveredCatalog, DiscoveryError> {
    if request.bytes.len() > request.config.server().limits.max_frame_bytes {
        return Err(DiscoveryError::BudgetExceeded);
    }
    let value = serde_json::from_slice::<Value>(request.bytes)
        .map_err(|_| DiscoveryError::MalformedResponse)?;
    validate_envelope(&value, request.expected_response_id)?;
    let result = value
        .get("result")
        .and_then(Value::as_object)
        .ok_or(DiscoveryError::MalformedResponse)?;
    if result
        .get("nextCursor")
        .is_some_and(|cursor| !cursor.is_null())
    {
        return Err(DiscoveryError::BudgetExceeded);
    }
    let tools = result
        .get("tools")
        .and_then(Value::as_array)
        .ok_or(DiscoveryError::MalformedResponse)?;
    if tools.is_empty()
        || tools.len() > request.config.server().limits.max_tools
        || tools.len() != request.config.server().tool_policies.len()
    {
        return Err(DiscoveryError::BudgetExceeded);
    }
    let mut accumulator = DiscoveryAccumulator {
        names: BTreeSet::new(),
        schema_bytes_total: 0,
    };
    let mut admitted = Vec::with_capacity(tools.len());
    for tool in tools {
        let mapped = map_tool(tool, request.config, request.binding, &mut accumulator)?;
        admitted.push(mapped);
    }
    admitted.sort_by(|left, right| left.mapping.tool_name().cmp(right.mapping.tool_name()));
    let identity = catalog_identity(admitted.as_slice());
    let catalog = DiscoveredCatalog {
        identity,
        generation: request.binding.generation,
        tools: admitted,
    };
    validate_previous(request.previous, &catalog)?;
    Ok(catalog)
}

fn validate_envelope(value: &Value, expected_response_id: u64) -> Result<(), DiscoveryError> {
    let object = value.as_object().ok_or(DiscoveryError::MalformedResponse)?;
    if object.get("jsonrpc").and_then(Value::as_str) != Some(JSON_RPC_VERSION) {
        return Err(DiscoveryError::MalformedResponse);
    }
    if object.get("id").and_then(Value::as_u64) != Some(expected_response_id) {
        return Err(DiscoveryError::CorrelationDrift);
    }
    if object.contains_key("error") {
        return Err(DiscoveryError::MalformedResponse);
    }
    Ok(())
}

fn map_tool(
    value: &Value,
    config: &McpHostConfig,
    binding: &LifecycleBinding,
    accumulator: &mut DiscoveryAccumulator,
) -> Result<DiscoveredTool, DiscoveryError> {
    let object = value.as_object().ok_or(DiscoveryError::MalformedResponse)?;
    let name = object
        .get("name")
        .and_then(Value::as_str)
        .ok_or(DiscoveryError::MalformedResponse)?;
    validate_tool_name(name, config)?;
    if !accumulator.names.insert(String::from(name)) {
        return Err(DiscoveryError::DuplicateTool);
    }
    let policy = configured_policy(config, name)?;
    let description = object
        .get("description")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if description.len() > config.server().limits.max_description_bytes {
        return Err(DiscoveryError::BudgetExceeded);
    }
    validate_resource_metadata(object, config)?;
    let canonical_schemas = admit_schemas(object, config, accumulator)?;
    let schema_identity = framed_hash(SCHEMA_DOMAIN, canonical_schemas.as_slice());
    let description_identity = framed_hash(DESCRIPTION_DOMAIN, description.as_bytes());
    let source_metadata = format!(
        "{SOURCE_PREFIX}-{}-g{}",
        config.server().id,
        binding.generation
    );
    let capabilities = borrowed(policy.capability_labels.as_slice());
    let resource_policies = borrowed(policy.resource_policy_refs.as_slice());
    let ability_policies = borrowed(policy.ability_policy_refs.as_slice());
    let mapping =
        animus_adapters::McpToolMapping::validate(&animus_adapters::McpToolMappingDraft {
            server_label: Some(config.server().id.as_str()),
            tool_name: Some(name),
            schema_identity: Some(schema_identity.as_str()),
            capability_labels: capabilities.as_slice(),
            source_metadata: Some(source_metadata.as_str()),
            result_bounds: Some(policy.result_bounds.as_str()),
            resource_policy_refs: resource_policies.as_slice(),
            ability_policy_refs: ability_policies.as_slice(),
        })
        .map_err(|_| DiscoveryError::MappingRejected)?;
    let definition = mapping
        .to_tool_definition()
        .map_err(|_| DiscoveryError::MappingRejected)?;
    Ok(DiscoveredTool {
        mapping,
        definition,
        description_identity,
        read_only_hint: classify_read_only_hint(object),
    })
}

fn classify_read_only_hint(object: &serde_json::Map<String, Value>) -> crate::ReadOnlyHint {
    let Some(annotations) = object.get("annotations") else {
        return crate::ReadOnlyHint::Missing;
    };
    let Some(annotations) = annotations.as_object() else {
        return crate::ReadOnlyHint::Unknown;
    };
    match annotations.get("readOnlyHint") {
        Some(Value::Bool(true)) => crate::ReadOnlyHint::DeclaredTrue,
        Some(Value::Bool(false)) => crate::ReadOnlyHint::DeclaredFalse,
        Some(_) => crate::ReadOnlyHint::Malformed,
        None => crate::ReadOnlyHint::Missing,
    }
}

fn admit_schemas(
    object: &serde_json::Map<String, Value>,
    config: &McpHostConfig,
    accumulator: &mut DiscoveryAccumulator,
) -> Result<Vec<u8>, DiscoveryError> {
    let input_schema = object
        .get("inputSchema")
        .ok_or(DiscoveryError::MalformedSchema)?;
    let input = canonical_schema(input_schema, config)?;
    let mut canonical = Vec::new();
    canonical.extend_from_slice(&(input.len() as u64).to_le_bytes());
    canonical.extend_from_slice(input.as_slice());
    match object.get("outputSchema") {
        Some(output_schema) => {
            let output = canonical_schema(output_schema, config)?;
            canonical.push(1);
            canonical.extend_from_slice(&(output.len() as u64).to_le_bytes());
            canonical.extend_from_slice(output.as_slice());
        }
        None => canonical.push(0),
    }
    accumulator.schema_bytes_total = accumulator
        .schema_bytes_total
        .checked_add(canonical.len())
        .ok_or(DiscoveryError::BudgetExceeded)?;
    if accumulator.schema_bytes_total > config.server().limits.max_schema_bytes_total {
        return Err(DiscoveryError::BudgetExceeded);
    }
    Ok(canonical)
}

fn canonical_schema(schema: &Value, config: &McpHostConfig) -> Result<Vec<u8>, DiscoveryError> {
    validate_schema_shape(schema)?;
    let canonical = serde_json::to_vec(schema).map_err(|_| DiscoveryError::MalformedSchema)?;
    if canonical.len() > config.server().limits.max_schema_bytes_each {
        return Err(DiscoveryError::BudgetExceeded);
    }
    Ok(canonical)
}

fn validate_tool_name(name: &str, config: &McpHostConfig) -> Result<(), DiscoveryError> {
    if name.is_empty() || name.len() > config.server().limits.max_tool_name_bytes {
        return Err(DiscoveryError::BudgetExceeded);
    }
    if name
        .bytes()
        .any(|byte| !(byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.')))
    {
        return Err(DiscoveryError::MalformedResponse);
    }
    Ok(())
}

fn validate_resource_metadata(
    object: &serde_json::Map<String, Value>,
    config: &McpHostConfig,
) -> Result<(), DiscoveryError> {
    let mut bytes = 0_usize;
    for field in ["annotations", "_meta", "title"] {
        if let Some(value) = object.get(field) {
            let encoded =
                serde_json::to_vec(value).map_err(|_| DiscoveryError::MalformedResponse)?;
            bytes = bytes
                .checked_add(encoded.len())
                .ok_or(DiscoveryError::BudgetExceeded)?;
        }
    }
    if bytes > config.server().limits.max_resource_metadata_bytes {
        return Err(DiscoveryError::BudgetExceeded);
    }
    Ok(())
}

fn validate_schema_shape(schema: &Value) -> Result<(), DiscoveryError> {
    let object = schema.as_object().ok_or(DiscoveryError::MalformedSchema)?;
    if object.get("type").and_then(Value::as_str) != Some("object") {
        return Err(DiscoveryError::MalformedSchema);
    }
    Ok(())
}

fn configured_policy<'a>(
    config: &'a McpHostConfig,
    name: &str,
) -> Result<&'a ToolPolicy, DiscoveryError> {
    config
        .server()
        .tool_policies
        .iter()
        .find(|policy| policy.name == name)
        .ok_or(DiscoveryError::UnknownTool)
}

fn borrowed(values: &[String]) -> Vec<&str> {
    values.iter().map(String::as_str).collect()
}

fn validate_previous(
    previous: Option<&DiscoveredCatalog>,
    current: &DiscoveredCatalog,
) -> Result<(), DiscoveryError> {
    let Some(previous) = previous else {
        return Ok(());
    };
    if previous.generation != current.generation {
        return Err(DiscoveryError::CorrelationDrift);
    }
    for tool in &current.tools {
        let Some(old_tool) = previous.tool(tool.mapping.tool_name()) else {
            return Err(DiscoveryError::SchemaDrift);
        };
        if old_tool.mapping.schema_identity() != tool.mapping.schema_identity() {
            return Err(DiscoveryError::SchemaDrift);
        }
    }
    if previous.tools.len() != current.tools.len() {
        return Err(DiscoveryError::SchemaDrift);
    }
    Ok(())
}

fn catalog_identity(tools: &[DiscoveredTool]) -> String {
    let mut bytes = Vec::new();
    for tool in tools {
        append_framed(&mut bytes, tool.mapping.tool_name().as_bytes());
        append_framed(&mut bytes, tool.mapping.schema_identity().as_bytes());
        append_framed(&mut bytes, tool.description_identity.as_bytes());
    }
    framed_hash(CATALOG_DOMAIN, bytes.as_slice())
}

fn append_framed(output: &mut Vec<u8>, value: &[u8]) {
    output.extend_from_slice(&(value.len() as u64).to_le_bytes());
    output.extend_from_slice(value);
}
