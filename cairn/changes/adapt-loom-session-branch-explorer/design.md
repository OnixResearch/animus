## Context

Animus already validates append-only session nodes, parent links, active-path replay, labels, summaries, and safe receipts. The ledger remains the only model-context authority.

The active `add-canonical-conversation-view-projection` change adds a stable visible conversation for one selected branch. This change adds a bounded topology and operator-control layer around that selected view.

The Loom article is the interaction reference. `universal-weave` commit `4e793545dca359f4f1806c74efe395eefa5affbe` is the data-structure reference.

The reference crate has useful property state-machine tests. Its mutable operations, persistence choices, experimental CRDT surface, and evolving active semantics do not match Animus ownership.

## Decisions

### Decision: keep the Animus ledger authoritative

**Choice:** Build the explorer from one fully validated ledger snapshot and one explicit projection policy.

The projection core accepts in-memory facts only. It returns a snapshot, diagnostic, or blocked outcome without storage, clock, transport, provider, terminal, or process access.

**Rationale:** Another writable tree would create conflicting replay, identity, redaction, and repair authority.

### Decision: separate conversation topology from evidence links

**Choice:** A model-visible session node keeps one replay parent. Evidence, synthesis, comparison, and equivalence use separate typed references.

The branch projection must not turn the conversation tree into an independent DAG. It must reject missing parents, cycles, duplicate identities, and unsupported topology.

**Rationale:** One replay parent gives deterministic context. Multi-source evidence does not define model-history order.

### Decision: project bounded redaction-first branch facts

**Choice:** Each projected node binds schema, policy, ledger revision, entry identity, optional parent, entry class, storage sequence, child count, active-path state, leaf state, safe labels, and admitted artifact references.

The projection includes explicit truncation and omitted-count facts. It excludes transcript text, reasoning, raw tool bodies, provider payloads, credentials, environment values, physical paths, and host handles.

**Rationale:** A topology view needs structure and safe summaries. It does not need private transcript bodies.

### Decision: keep annotations outside replay context

**Choice:** Add typed bookmark and safe-label annotations by entry identity. Add sibling presentation order by parent identity and ordered child identities.

A partial sibling order uses deterministic ledger order for omitted children. Duplicate, unknown, cross-parent, stale, or overbound child identities fail closed.

Annotations can change display state only. They cannot change canonical storage sequence, active-path replay, model context, provenance, or tool correlation.

**Rationale:** Loom-style navigation benefits from bookmarks and stable display order. These are operator preferences, not conversation facts.

### Decision: distinguish inspection, activation, and forking

**Choice:** Inspection can target any admitted node. Activation can target only a true leaf under the expected ledger revision.

Fork-from-node can target any admitted node. It returns an append-only branch plan that uses existing branch-control contracts and creates a new child lineage.

Each plan binds source session, source revision, expected active leaf, target, policy, and operation identity. A host mutation needs exact current facts.

**Rationale:** Inspecting an internal node is safe. Treating it as a leaf would make continuation and child order ambiguous.

### Decision: make external-state posture explicit

**Choice:** Selection and fork projections include supplied external-state posture and effect-receipt references.

They state whether continuation is read-only, shared-world, isolated, or unknown. Unknown and shared-world states never claim rollback or independent world state.

The explorer does not create isolation. Mutating continuation needs separate policy, authority, workspace, and effect evidence.

**Rationale:** Branching a transcript does not undo filesystem, network, provider, or tool effects.

### Decision: compare exact branch structure and safe metadata

**Choice:** Comparison finds the exact common ancestor and ordered suffix identities for two admitted targets.

It can report entry-class counts, usage summaries, model-profile identities, safe artifact references, effect-receipt references, annotations, and truncation facts.

Visible content comparison composes separately authorized conversation projections. Topology comparison does not reveal or infer private content.

**Rationale:** Exact structural comparison is deterministic. Semantic quality and correctness need separate evaluators.

### Decision: mark equivalence without deduplication

**Choice:** An equivalence fact binds two sibling leaves and one domain-separated BLAKE3 generation identity.

The generation identity includes the source revision, fork point, model profile, parameter profile, tool-plan identity, effect-receipt set, and response artifact identity.

Equivalent branches remain present and selectable. The core never deletes, merges, or rewrites them.

**Rationale:** Equal text alone is not equal execution. Preserving both branches protects provenance and effect history.

### Decision: use a pure TUI reducer and thin ports

**Choice:** `animus-tui` owns map/read presentation, collapse state, focus, bookmark commands, comparison selection, and typed operation dispatch.

Pure reducer state produces deterministic text snapshots. Host ports own annotation persistence, active-leaf writes, continuation dispatch, terminal setup, and transport.

The read view composes the accepted canonical conversation projection. The map view consumes only branch projection facts.

**Rationale:** This gives Animus one reference UX without moving terminal authority into reusable crates.

### Decision: adapt concepts without a runtime dependency

**Choice:** Record Loom and `universal-weave` as bounded references. Do not copy source or add a crate, serialization, or CRDT dependency.

Revisit a direct dependency only for a separate generic mutable-document editor with its own authority and lifecycle review.

**Rationale:** The useful part is the interaction model. Animus already owns the required ledger semantics and identity boundaries.

## State Flow

```text
validated ledger + policy + annotations + expected revision
                          |
                          v
                pure branch projection
                          |
              +-----------+-----------+
              |                       |
              v                       v
       map view reducer       selected read projection
              |                       |
              +-----------+-----------+
                          v
               typed inspect/activate/fork request
                          |
                          v
          host freshness and authority revalidation
                          |
                          v
           host mutation or explicit blocked outcome
```

## Adversarial Cases

- The selected target exists but has admitted children.
- A sibling order repeats one child or names another parent's child.
- A bookmark points to an entry absent from the current revision.
- Two branches have equal text but different tool-effect receipts.
- A comparison target belongs to another session or generation.
- The active leaf changes after the operator opens the map.
- A branch follows a mutating tool effect without isolation evidence.
- A projection limit truncates the common ancestor or selected path.
- A receipt draft contains transcript text or a physical path.

Each case must return one deterministic view, no-op, diagnostic, or blocked plan.

## Risks / Trade-offs

- Strict revision fences can require frequent refreshes during active generation.
- Large trees need explicit node, depth, label, and comparison bounds.
- Presentation order can drift from storage order. The UI must label that difference.
- Shared-world branches can mislead operators. External-state posture must remain visible.
- The reference TUI does not prove Tile, browser, transport, or accessibility behavior.

## Non-Claims

This design does not prove transcript truth, tool-effect rollback, world-state isolation, response quality, semantic equivalence, cache durability, UI delivery, renderer correctness, accessibility, production readiness, or release eligibility.
