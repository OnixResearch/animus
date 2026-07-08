# agent-harness-runtime Change Spec: Unified subagent facade

## Requirements

### Requirement: Runtime exposes a unified child-run facade

r[agent_harness_runtime.subagent_facade.boundary] Animus runtime MUST expose a unified subagent facade that represents recipe-backed subagents, fresh-context investigations, read-only side questions, and worker-team tasks as typed child-run drafts, kinds, admissions, diagnostics, and receipts.

#### Scenario: Embedder validates through one facade

r[agent_harness_runtime.subagent_facade.all_kinds]
- GIVEN an embedder has one valid recipe-backed subagent, one valid fresh-context investigation, one valid side question, and one valid worker-team task
- WHEN each draft is validated through the subagent facade
- THEN each accepted result identifies the child-run kind and safe identity
- AND validation performs no model, tool, scheduler, worker-service, filesystem, process, network, or credential side effect.

### Requirement: Facade projections are lossless

r[agent_harness_runtime.subagent_facade.lossless_projection] The subagent facade MUST preserve source-specific child-run metadata, diagnostics, handoff settings, capability ceilings, context identities, and safe receipt hashes without collapsing them into untyped free-form maps.

#### Scenario: Source-specific diagnostic survives projection

- GIVEN a recipe-backed subagent is denied because recipe trust is missing
- WHEN the denial is converted into the facade diagnostic type
- THEN the diagnostic still identifies the recipe-trust failure class and source child-run kind
- AND no lower-level denial reason is replaced by a generic unknown failure.

### Requirement: Facade validation fails closed

r[agent_harness_runtime.subagent_facade.fail_closed] The subagent facade MUST fail closed for unsupported child-run kinds, missing kind-specific required fields, invalid lower-level DTOs, lossy projection attempts, or any validation path that would require execution authority.

#### Scenario: Unsupported child-run kind is rejected

- GIVEN a child-run draft names a kind that has no registered typed validator
- WHEN facade validation runs
- THEN validation returns an unsupported-kind diagnostic
- AND no fallback child runner, shell command, external CLI, or model request is attempted.

### Requirement: Facade closeout is evidenced

r[agent_harness_runtime.subagent_facade.verification] The unified subagent facade adoption MUST include positive validation tests for every supported child-run kind, negative fail-closed tests, public export checks, documentation, Cairn validation, and Cairn gates before archive.

#### Scenario: Facade evidence is complete

- GIVEN the unified subagent facade change is ready to archive
- WHEN closeout validation runs
- THEN evidence names facade conversion tests, lower-level validator compatibility tests, docs, Cairn validation, and Cairn gates
- AND skipped checks record a blocker and next-best validation.
