# Proposal: Extend recipe workflows with DAG steps

## Why

Animus recipe workflows already validate manifests, parameters, toolsets, retry policy, structured output, subrecipes, and trust receipts. The current activity list is good for simple runs, but complex recipes need explicit dependency structure: independent investigations can run in parallel, validation gates can join several outputs, and cleanup or retry logic can depend on a specific failed step.

Animus should add recipe-owned DAG step contracts so workflow authors can describe dependency structure without embedding executable host scripts or scheduler behavior.

## What Changes

- Add typed recipe step DAG descriptors for step IDs, kinds, dependency edges, declared inputs, declared outputs, validation gates, retry/cleanup references, and subrecipe calls.
- Validate recipe DAGs with the runtime DAG contract kit plus recipe-specific trust, parameter, output-reference, capability-ceiling, and sandbox checks.
- Produce deterministic topological execution batches and blocked-step explanations as plans only; hosts remain responsible for dispatch, parallelism, cancellation, and persistence.
- Project per-step and whole-graph receipts using safe labels, BLAKE3 hashes, bounded counts, and redaction markers.

## Impact

- **Files**: future adapter recipe DTOs/core, Nickel workflow config contracts, generated JSON drift checks, docs, tests, and facade inventory updates.
- **Testing**: positive linear, parallel, fan-in, subrecipe, validation-gate, and retry-plan cases plus negative cycle, unknown step, undeclared output, untrusted subrecipe, capability widening, unsafe label, retry bypass, and receipt leak fixtures.
- **Non-claims**: this does not add a workflow scheduler, run steps in parallel, execute shell scripts, call providers, discover tools, persist workflow state, or auto-trust imported recipes.
