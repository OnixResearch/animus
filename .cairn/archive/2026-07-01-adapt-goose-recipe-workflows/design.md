# Design: Adapt Goose recipe workflows for Animus

Goose recipes are useful because they turn a successful agent setup into a reusable workflow with parameters, tools, validation, retry, and structured output. Animus should adapt the workflow shape while preserving stronger constraints: typed Nickel authoring, pure validation, explicit capability ceilings, and host-owned side effects.

## Decisions

### 1. Recipe manifests are typed workflow contracts

**Choice:** Define an Animus recipe/workflow manifest with identity, description, instructions, initial prompt, activities, parameter definitions, model profile, capability/toolset references, validation gates, retry policy, subrecipes, output schema, and provenance.

**Rationale:** This preserves Goose's repeatability while making every part of the workflow reviewable and testable before execution.

### 2. Nickel owns authoring and validation

**Choice:** Use Nickel contracts as the source of truth. YAML and JSON exports are derived interchange artifacts for tools that require them.

**Rationale:** Nickel catches schema and merge errors before runtime and matches Animus' existing configuration posture.

### 3. Parameter binding is pure and closed over declared inputs

**Choice:** Template variables must correspond to declared parameters or built-in safe values. Optional parameters require defaults. File parameters are admitted only through explicit host-provided content references and cannot carry defaults.

**Rationale:** Goose's parameter substitution is valuable, but unchecked file injection can accidentally import secrets or unbounded context.

### 4. Structured output is a first-class automation contract

**Choice:** A recipe may declare a final response schema. Runtime validates the final output and records typed validation success or failure before automation consumes it.

**Rationale:** Automation needs parseable results, and failures must be visible rather than buried in prose.

### 5. Retry is a plan, not ambient shell authority

**Choice:** Retry checks and cleanup are represented as host-effect requests with capability labels, timeouts, sandbox descriptors, redaction, and receipts. Reusable core never runs shell checks.

**Rationale:** Goose's retry loop is useful, but commands must remain subordinate to Animus policy and injected services.

### 6. Recipes have trust and provenance receipts

**Choice:** Imported, changed, project-provided, or first-run recipes produce BLAKE3-identified trust receipts and may require admission before influencing prompt, tools, model, memory, or workflow permissions.

**Rationale:** Reusable workflows are powerful prompt/code supply-chain inputs and need explicit trust lifecycle evidence.
