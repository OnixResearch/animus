# Proposal: Add runtime DAG contract kit

## Why

Animus already validates dependency-shaped data in Kanban orchestration, workspace parentage, provenance/artifact lineage, tool plans, subagents, roadmap sequencing, and ResearchSpec promotion groups. Those surfaces currently express graph behavior in local terms, which makes cycle rejection, topological ordering, blocked-dependency diagnostics, and safe receipt projection easy to drift.

Animus should add a small reusable DAG contract kit so domains can share deterministic graph validation and explanation semantics without sharing execution authority or storage assumptions.

## What Changes

- Define typed DAG node, edge, graph, validation report, topological batch, blocked node, and safe receipt identity contracts.
- Provide pure validation for unique nodes, known endpoints, duplicate edges, cycles, unsafe labels, bounded graph size, and deterministic topological ordering.
- Keep domain payloads outside the generic graph core; domain modules own task, recipe, artifact, workspace, subagent, roadmap, and ResearchSpec meaning.
- Emit safe BLAKE3 graph receipt identities and bounded diagnostics without raw prompts, provider payloads, raw file paths, raw diffs, or unbounded outputs.

## Impact

- **Files**: future runtime DAG DTO/core module, facade inventory entries, docs, and focused tests.
- **Testing**: positive DAG, batching, blocked-node, and receipt cases plus negative duplicate, missing endpoint, cycle, over-limit, unsafe-label, unstable-order, and receipt-leak cases.
- **Non-claims**: this does not add a scheduler, execute graph nodes, persist graph state, read files, call providers, spawn processes, render UI graphs, or replace domain-specific validation.
