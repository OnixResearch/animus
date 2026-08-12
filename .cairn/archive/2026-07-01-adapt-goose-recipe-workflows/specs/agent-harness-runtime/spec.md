# agent-harness-runtime Change Spec: Goose recipe workflow adaptation

## Requirements

### Requirement: Typed recipe workflow manifests

r[agent_harness_runtime.goose_recipe_workflows.manifest] Animus MUST define typed recipe/workflow manifests for identity, instructions, initial prompt, activities, parameters, model profile, capability ceilings, toolset references, validation gates, retry policy, subrecipes, response schema, and provenance.

#### Scenario: Valid recipe manifest loads deterministically

r[agent_harness_runtime.goose_recipe_workflows.manifest.valid]
- GIVEN a recipe manifest declares instructions, prompt, parameters, allowed toolsets, validation gates, response schema, and provenance
- WHEN pure manifest validation runs
- THEN validation returns a deterministic accepted manifest DTO
- AND no provider, filesystem, process, network, credential, or tool adapter is invoked.

### Requirement: Nickel is the authoritative recipe source

r[agent_harness_runtime.goose_recipe_workflows.nickel_source] Animus recipe authoring MUST use typed Nickel contracts as source of truth, and YAML or JSON recipe files MUST be derived interchange artifacts unless an external adapter explicitly imports them under validation.

#### Scenario: Generated recipe drift is rejected

r[agent_harness_runtime.goose_recipe_workflows.nickel_source.drift]
- GIVEN a YAML export differs from the Nickel recipe source for the same recipe identity
- WHEN generated-config drift validation runs
- THEN validation reports recipe drift
- AND runtime does not prefer the edited export over the checked Nickel source.

### Requirement: Parameter binding is pure and declared

r[agent_harness_runtime.goose_recipe_workflows.parameters] Recipe parameter binding MUST validate declared template variables, safe built-in variables, optional defaults, select options, and file-parameter admission before prompt assembly.

#### Scenario: Unsafe parameter template fails closed

r[agent_harness_runtime.goose_recipe_workflows.parameters.invalid]
- GIVEN a recipe template references an undeclared variable, omits an optional default, includes an invalid select option, or assigns a default to a file parameter
- WHEN parameter binding runs
- THEN binding returns a typed validation error
- AND no recipe prompt, activity, or subrecipe input is assembled from the invalid template.

### Requirement: Structured automation output is validated

r[agent_harness_runtime.goose_recipe_workflows.structured_output] Recipes that declare structured output MUST validate final responses against the declared schema before reporting automation success.

#### Scenario: Malformed final output blocks success

r[agent_harness_runtime.goose_recipe_workflows.structured_output.invalid]
- GIVEN a headless recipe declares a required JSON output field
- WHEN the final assistant output omits that field or uses the wrong type
- THEN output validation records a structured-output failure
- AND automation does not consume the malformed output as a successful result.

### Requirement: Retry policy remains host-mediated

r[agent_harness_runtime.goose_recipe_workflows.retry_policy] Recipe retry checks and cleanup actions MUST be represented as host-effect requests with capability labels, sandbox requirements, timeouts, redaction, and receipts rather than direct reusable-core shell execution.

#### Scenario: Retry cannot bypass policy denial

r[agent_harness_runtime.goose_recipe_workflows.retry_policy.bypass]
- GIVEN a recipe retry check requests a shell command denied by active capability or sandbox policy
- WHEN the retry planner evaluates the check
- THEN the retry attempt records a denied host-effect request
- AND reusable core does not execute the command or report validation success.

### Requirement: Recipe trust and provenance are explicit

r[agent_harness_runtime.goose_recipe_workflows.trust] Imported, changed, first-run, or project-provided recipes MUST have explicit trust and provenance receipts before they can influence prompt assembly, tool availability, model selection, memory use, or workflow permissions.

#### Scenario: Untrusted recipe has no influence

r[agent_harness_runtime.goose_recipe_workflows.trust.denied]
- GIVEN a project recipe has not been admitted by host policy or user confirmation
- WHEN runtime prepares a session using that recipe
- THEN the recipe is rejected or activated in a no-influence failure state
- AND prompt, tools, model, memory, and workflow permissions remain unchanged.

### Requirement: Recipe workflow closeout is evidenced

r[agent_harness_runtime.goose_recipe_workflows.verification] Recipe workflow adoption MUST include positive and negative tests for manifest validation, Nickel source authority, parameter binding, structured output, retry policy, trust receipts, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Recipe workflow evidence is complete

- GIVEN the Goose recipe workflow change is ready to archive
- WHEN closeout validation runs
- THEN evidence includes focused recipe workflow tests, Nickel drift checks, structured-output failures, retry-bypass failures, trust-denial failures, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
