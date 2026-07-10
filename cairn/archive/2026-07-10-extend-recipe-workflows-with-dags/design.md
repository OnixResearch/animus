# Design: Extend recipe workflows with DAG steps

## Context

Goose-style recipes are already represented as trusted Animus-owned workflow data. The missing piece is structural: `activities` and `subrecipes` do not describe which steps can run independently, which outputs feed later validation, or why a step is blocked. A recipe DAG can make those dependencies reviewable before any model, tool, or host effect occurs.

## Decisions

### 1. Recipe DAGs are manifest data

**Choice:** Extend recipe manifests with optional DAG step descriptors. A step has a safe ID, step kind, declared dependencies, declared input references, declared output references, validation gate references, retry policy references, and capability ceilings.

**Rationale:** Keeping DAGs in manifest data lets Nickel validate author intent and lets Rust validate the generated interchange artifact.

### 2. Generic DAG validation is necessary but not sufficient

**Choice:** Use the runtime DAG contract kit for structure, then apply recipe-specific checks for declared parameter references, output binding, subrecipe trust, capability ceilings, model profile eligibility, toolset admission, sandbox requirements, and retry policy safety.

**Rationale:** Acyclic structure does not prove a recipe is trusted or authorized.

### 3. Execution plans are topological batches, not dispatch

**Choice:** The reusable adapter layer returns deterministic ready batches, blocked-step diagnostics, and effect request descriptors. It does not execute steps, choose actual parallelism, call providers, invoke tools, run cleanup, or persist progress.

**Rationale:** Hosts need the plan, but effect ownership remains behind injected services and policy gates.

### 4. Fan-in uses safe references only

**Choice:** A downstream step may bind only declared safe output references, hashes, summaries, schema-validated fields, and bounded receipt labels from upstream steps. Raw upstream output cannot be injected by the reusable recipe DAG layer.

**Rationale:** DAG convenience must not become a raw transcript or tool-output bypass.

## Risks / Trade-offs

- DAG manifests are more complex than simple recipes. Keep DAGs optional and preserve existing linear activity validation.
- Hosts may want dynamic branching. The first contract should support deterministic planned structure; dynamic branching can be a later admitted plan update with receipts.
- Subrecipe trust and output binding can be subtle. Negative fixtures should cover untrusted, stale, and undeclared references.
