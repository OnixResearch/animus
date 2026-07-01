let ToolDescriptor = {
  name | String,
  description | String,
  runtime_kind | String,
  capabilities | Array String,
  side_effect | String,
  approval | String,
  input_schema | String,
  output_schema | String,
  redaction | String,
} in
let ToolCatalog = {
  schema | String,
  descriptors | Array ToolDescriptor,
} in
{
  catalog_schema = {
    schema = "animus.tool-catalog.schema.v1",
    runtime_kinds = ["host_descriptor", "injected_service", "external_adapter"],
    capabilities = [
      "observe_read",
      "workspace_mutation",
      "shell",
      "network",
      "process",
      "secret_adjacent",
      "host_custom",
    ],
    side_effects = ["read_only", "workspace_write", "process", "network", "secret_adjacent"],
    approvals = ["allow", "prompt", "deny"],
    redactions = ["metadata_only", "hash_only", "none"],
    schema_reference_prefix = "schema:",
  },

  sample_catalog | ToolCatalog = {
    schema = "animus.tool-catalog.v1",
    descriptors = [
      {
        name = "inspect_context",
        description = "Read bounded context supplied by the host.",
        runtime_kind = "host_descriptor",
        capabilities = ["observe_read"],
        side_effect = "read_only",
        approval = "allow",
        input_schema = "schema:animus.tool.input.context.v1",
        output_schema = "schema:animus.tool.output.context.v1",
        redaction = "metadata_only",
      },
    ],
  },
}
