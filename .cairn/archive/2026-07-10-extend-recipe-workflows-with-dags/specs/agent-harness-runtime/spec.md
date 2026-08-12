# agent-harness-runtime Change Spec: Recipe workflow DAGs

## Requirements

### Requirement: Recipes may declare optional DAG step structure

r[agent_harness_runtime.goose_recipe_workflows.dag] Animus recipe workflow manifests MAY declare optional DAG step descriptors containing safe step IDs, step kinds, dependency edges, declared input references, declared output references, validation gate references, retry policy references, subrecipe references, and capability ceilings while preserving existing linear recipe behavior.

#### Scenario: Parallel investigation steps are represented as data

r[agent_harness_runtime.goose_recipe_workflows.dag.parallel]
- GIVEN a trusted recipe declares two independent investigation steps and one downstream synthesis step
- AND the synthesis step depends on declared safe outputs from both investigations
- WHEN recipe manifest validation runs
- THEN Animus accepts the DAG structure as workflow data
- AND it does not execute model calls, tool calls, cleanup actions, or provider dispatch.

### Requirement: Recipe DAG structure fails closed before effects

r[agent_harness_runtime.goose_recipe_workflows.dag.validation] Recipe DAG validation MUST reject duplicate step IDs, duplicate dependency edges, unknown dependency endpoints, self edges, cycles, unsafe step labels, over-limit graphs, ambiguous roots, and malformed step metadata before any workflow effect is planned.

#### Scenario: Cyclic recipe is rejected

r[agent_harness_runtime.goose_recipe_workflows.dag.validation.cycle]
- GIVEN a recipe step depends on another step that eventually depends back on the first step
- WHEN recipe DAG validation runs
- THEN validation returns a cycle diagnostic with safe step identity evidence
- AND no retry action, cleanup action, model request, tool request, or subrecipe invocation is planned.

### Requirement: Recipe-specific gates apply after structural DAG validation

r[agent_harness_runtime.goose_recipe_workflows.dag.recipe_gates] A structurally valid recipe DAG MUST still pass recipe-specific gates for parameter binding, declared output references, trusted subrecipes, capability ceilings, model profile eligibility, toolset admission, sandbox requirements, retry policy safety, and structured-output schema compatibility.

#### Scenario: Untrusted subrecipe blocks a ready step

r[agent_harness_runtime.goose_recipe_workflows.dag.recipe_gates.untrusted_subrecipe]
- GIVEN a DAG step references a subrecipe whose trust receipt is missing, stale, or denied
- WHEN recipe-specific DAG gates run
- THEN the step is reported as blocked with a trust diagnostic
- AND the subrecipe cannot influence prompt assembly, tool availability, model selection, memory use, or workflow permissions.

### Requirement: Recipe DAG execution plans are deterministic and non-executing

r[agent_harness_runtime.goose_recipe_workflows.dag.execution_plan] Animus MUST project accepted recipe DAGs into deterministic topological execution batches, blocked-step explanations, and host-effect request descriptors without executing steps, selecting live parallelism, calling providers, invoking tools, persisting progress, or dispatching cleanup.

#### Scenario: Ready batch is stable

r[agent_harness_runtime.goose_recipe_workflows.dag.execution_plan.batch]
- GIVEN a recipe DAG has several independent ready steps with admitted capability ceilings
- WHEN execution planning runs
- THEN the ready steps are returned in a deterministic sorted batch
- AND actual scheduling, concurrency limits, cancellation, persistence, and dispatch remain host-owned.

### Requirement: Recipe DAG fan-in uses safe references and receipts

r[agent_harness_runtime.goose_recipe_workflows.dag.receipts] Recipe DAG fan-in and receipt projection MUST use declared safe output references, schema-validated fields, bounded summaries, BLAKE3 hashes, step status classes, issue codes, and redaction markers, and MUST NOT inject raw upstream outputs into downstream prompts by default.

#### Scenario: Raw upstream output is rejected

r[agent_harness_runtime.goose_recipe_workflows.dag.receipts.rejects_raw_output]
- GIVEN a downstream step attempts to bind an upstream raw tool body, raw model response, provider payload, credential-looking value, absolute path dump, raw diff, or unbounded log as prompt input
- WHEN recipe DAG fan-in validation runs
- THEN validation rejects the binding with a leak diagnostic
- AND no workflow success receipt is emitted for that fan-in.

### Requirement: Recipe DAG verification covers positive and negative cases

r[agent_harness_runtime.goose_recipe_workflows.dag.verification] Recipe DAG adoption MUST include focused positive and negative tests for optional DAG manifests, linear compatibility, parallel batches, fan-in, subrecipes, validation gates, retry plans, topological ordering, duplicate steps, unknown steps, cycles, undeclared outputs, untrusted subrecipes, capability widening, retry bypass, unsafe labels, receipt leaks, Nickel export drift, documentation, Cairn validation, and Cairn gates.

#### Scenario: Closeout evidence records workflow non-claims

r[agent_harness_runtime.goose_recipe_workflows.dag.verification.closeout]
- GIVEN maintainers intend to archive recipe workflow DAG support
- WHEN closeout validation runs
- THEN evidence names recipe DAG DTOs, Nickel contracts, generated JSON drift checks, structural validation, recipe gates, execution planning, receipt projection, positive tests, negative tests, Cairn validation, and Cairn gates
- AND it records workflow scheduling, provider dispatch, tool execution, shell execution, persistence, dynamic branching, and automatic recipe trust as non-claims.
