# Design: Add runtime DAG contract kit

## Context

Graph behavior is already spread across runtime and adapter contracts: Kanban parent links, workspace parent trees, artifact dependency hashes, Toolpath projections, subagent fan-out/fan-in, ResearchSpec promotion dependencies, and Cairn roadmap sequencing. The common need is not a graph database; it is pure validation and explanation over already-loaded safe identifiers.

## Decisions

### 1. The DAG kit owns structure only

**Choice:** Define generic node IDs, edge IDs, edge endpoints, optional edge labels, graph bounds, validation issues, topological batches, blocked-node explanations, and graph receipt hashes. Domain payloads remain in their owning modules.

**Rationale:** Shared structure prevents duplicated cycle and ordering logic while keeping task, recipe, artifact, workspace, and experiment semantics local.

### 2. Validation is pure and deterministic

**Choice:** DAG validation operates only on in-memory typed inputs and returns safe reports. It rejects duplicate node IDs, duplicate edges, unknown endpoints, self edges, cycles, unsafe labels, and over-limit graphs. Topological batches are sorted by stable safe IDs.

**Rationale:** Deterministic ordering is required for receipts, reproducible tests, and reviewable blocked-dependency diagnostics.

### 3. Receipts are hash-backed summaries

**Choice:** Graph receipts include schema labels, bounded counts, issue codes, safe graph labels, sorted node/edge identities, topological batch hashes, and BLAKE3 receipt identities. They do not include domain payload bodies.

**Rationale:** Graph evidence should explain structure without leaking prompts, output, paths, diffs, credentials, or provider data.

### 4. The kit does not schedule

**Choice:** The kit never dispatches graph nodes, starts services, persists state, polls children, mutates workspaces, or opens transports. Hosts and domain modules translate accepted graph structure into their own effect requests.

**Rationale:** A DAG contract is useful because it is safe to run before effects and easy to test without standing up the world.

## Risks / Trade-offs

- A generic kit can become too broad. Keep the first version limited to structure, ordering, diagnostics, and receipts.
- Existing domain code may already have cycle checks. Initial integration should wrap or compare behavior rather than rewriting every domain at once.
- Deterministic batch ordering may differ from host scheduling preference. Hosts may reorder only after preserving the validated dependency relation and receipt evidence.
