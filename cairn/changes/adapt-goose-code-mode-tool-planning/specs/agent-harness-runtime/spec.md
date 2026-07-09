# agent-harness-runtime Change Spec: Goose Code Mode typed tool planning

## Requirements

### Requirement: Progressive tool discovery DTOs

r[agent_harness_runtime.goose_code_mode_tool_planning.discovery] Animus MUST define progressive tool discovery DTOs for tool-list queries, tool summaries, tool detail requests, schema identities, capability labels, result contracts, source provenance, discovery ceilings, and discovery receipts without exposing executable plugin code or host runtime handles.

#### Scenario: Planner discovers one tool detail set

r[agent_harness_runtime.goose_code_mode_tool_planning.discovery.detail]
- GIVEN a planner needs a file-inspection tool from an admitted catalog containing many tools
- WHEN it requests details for one safe tool identity
- THEN discovery returns bounded metadata for that tool's description, schema identity, capability labels, result contract, and provenance
- AND it does not execute the tool, import a plugin, inspect installation directories, or read credentials.

### Requirement: Declarative tool plan DTOs

r[agent_harness_runtime.goose_code_mode_tool_planning.plan_dtos] Code Mode-style execution MUST be represented as declarative tool plan DTOs for batch identity, ordered steps, dependencies, intermediate bindings, batch policy, cancellation posture, timeout posture, and step outcomes rather than arbitrary host code.

#### Scenario: Batch plan chains bounded outputs

r[agent_harness_runtime.goose_code_mode_tool_planning.plan_dtos.batch]
- GIVEN a planner wants to list files and then read one selected file summary
- WHEN it emits a tool plan
- THEN the plan records two ordered steps and a bounded intermediate binding from the first step to the second
- AND no JavaScript, shell script, Deno program, or host code body is embedded in the plan.

### Requirement: Tool-plan fabric is policy-admitted

r[agent_harness_runtime.goose_code_mode_tool_planning.fabric] Tool-plan producers MAY be Steel-backed or typed non-Steel fabrics, but the selected fabric MUST be admitted by policy, budget, script or profile identity, allowed host actions, and receipt posture before it can influence discovery or tool-plan output.

#### Scenario: Denied Steel planner cannot produce a plan

r[agent_harness_runtime.goose_code_mode_tool_planning.fabric.denied]
- GIVEN a Steel planning profile has an invalid script hash or unsupported host-action allowlist
- WHEN runtime prepares to evaluate the planner
- THEN tool-plan production is denied with a typed policy diagnostic
- AND no discovery request, tool plan, model dispatch, or tool dispatch occurs.

### Requirement: Every planned step re-enters tool admission

r[agent_harness_runtime.goose_code_mode_tool_planning.step_admission] Every tool-plan step MUST pass catalog trust, exact capability-label admission, authority and permission policy, argument-schema validation, mutation-window serialization, output-bound policy, timeout policy, cancellation policy, and receipt policy before a host adapter may dispatch it.

#### Scenario: Undiscovered or denied tool cannot run

r[agent_harness_runtime.goose_code_mode_tool_planning.step_admission.denied]
- GIVEN a tool plan references a tool omitted from the admitted discovery set or lacking the required exact capability label
- WHEN step admission validates the plan
- THEN validation rejects the step with an undeclared-tool or capability-denied diagnostic
- AND no fallback tool, ambient catalog lookup, or adapter dispatch occurs.

### Requirement: Intermediate data is bounded and typed

r[agent_harness_runtime.goose_code_mode_tool_planning.intermediates] Intermediate tool results used by later plan steps MUST be represented through bounded text, safe handles, hashes, counts, redaction classes, and admitted type labels; binary, image, secret, oversized, or unsupported results MUST NOT be silently converted into model-visible text or later tool arguments.

#### Scenario: Binary result is not chained as text

r[agent_harness_runtime.goose_code_mode_tool_planning.intermediates.binary]
- GIVEN an early tool-plan step returns an image or binary result with a host-owned handle
- WHEN a later step attempts to bind that result as text without explicit admitted conversion
- THEN intermediate validation rejects the binding
- AND the binary payload is not included in prompt context, receipts, or tool arguments.

### Requirement: Safe tool-plan receipts

r[agent_harness_runtime.goose_code_mode_tool_planning.receipts] Tool discovery, plan admission, step admission, step dispatch outcome, intermediate rejection, cancellation, timeout, and batch completion MUST produce safe receipts with plan hash, step identities, capability labels, decision classes, output-bound classes, policy hash, and BLAKE3 receipt hash.

#### Scenario: Tool-plan receipt rejects raw script and output bodies

r[agent_harness_runtime.goose_code_mode_tool_planning.receipts.rejects_leaks]
- GIVEN a tool-plan receipt candidate contains raw prompts, planner script bodies, raw tool arguments, raw tool outputs, provider payloads, credentials, full paths, or unbounded intermediate data
- WHEN receipt validation runs
- THEN validation rejects the receipt with a leak diagnostic
- AND durable Code Mode evidence does not echo the raw material.

### Requirement: Typed tool-planning verification

r[agent_harness_runtime.goose_code_mode_tool_planning.verification] Code Mode-style tool-planning adoption MUST include positive progressive-discovery, valid batch, admitted fabric, step admission, and bounded-intermediate tests plus negative arbitrary-code, denied fabric, undiscovered tool, capability denial, unsafe intermediate, unsupported binary, no ambient discovery, and receipt leak fixtures.

#### Scenario: Closeout evidence covers typed Code Mode adaptation

r[agent_harness_runtime.goose_code_mode_tool_planning.verification.closeout]
- GIVEN maintainers intend to archive the Goose Code Mode typed tool-planning adaptation
- WHEN closeout validation runs
- THEN evidence names discovery, plan DTO, fabric, step admission, intermediate, receipt, docs, Cairn validation, and Cairn gate checks
- AND any skipped rail includes a concrete blocker and next-best validation.
