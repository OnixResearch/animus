## Context

Animus already models ledger entries, active branches, runtime events, model stream frames, headless commands, TUI state, and snapshot-first host streams. These contracts leave projection ownership distributed across hosts.

The new contract selects one admitted ledger branch as the only source for model-visible conversation facts. It then produces a bounded display projection for TUI, headless, browser, or remote hosts.

camelAI commit `70dd57a6219d9a19b25840aebbf1105c63c3a5f2` is a bounded design reference. Its model transcript feeds a rebuildable browser message store through exact render IDs and a revision-aware high-water mark.

Animus will retain exact IDs and rebuildable views. It will reject parsed-count cursors, timestamp identity, silent row skipping, and direct coupling to one UI framework.

Completion requires deterministic rebuild and reconnect evidence. A single successful render, mock-only ID equality, or cache persistence without replay validation is false completion.

## Decisions

### Decision: project from one admitted ledger branch

**Choice:** Projection input includes the selected session, branch, leaf, ledger generation, ordered entries, projection policy, optional active-turn state, and optional cache cursor.

The pure core validates the complete input before it emits a snapshot, delta, rebuild plan, heal plan, or blocked outcome. It never reads storage or mutates ledger entries.

Rendered caches cannot supply model context, tool correlation, authority, recovery truth, or completion evidence.

**Rationale:** Two writable transcript stores create drift and unclear repair authority. One ledger plus disposable views gives every repair one direction.

### Decision: use exact typed identities and sequence order

**Choice:** Derive each view-item identity with domain-separated BLAKE3 over the projection profile and canonical source identities.

A user item binds its ledger entry identity. An assistant item binds the canonical turn identity and selected branch. Steering markers bind the steering entry and assistant item identities.

Projection order uses validated ledger and turn sequences. Timestamps remain optional display metadata and never determine identity, joins, or canonical order.

**Rationale:** Equal content and timestamps are common. Exact source identity prevents accidental merging and content-based deduplication errors.

### Decision: make the cursor revision-fenced

**Choice:** The cursor binds projection schema, policy identity, session, branch, leaf, ledger generation, last projected entry identity, last projection sequence, and optional active-turn identity.

A matching cursor can request deterministic top-up. A stale generation, changed leaf, missing anchor, reversed sequence, policy drift, or active-turn mismatch returns a full rebuild plan or blocked outcome.

Counts can appear as bounded diagnostics, but a count is not a cursor or identity.

**Rationale:** Rewrites and branch movement can preserve row counts. A revision-fenced identity prevents silent acceptance of a stale cache.

### Decision: require snapshot-first streams

**Choice:** Every projection stream begins with one accepted redaction-safe snapshot. Incremental events then carry the stream identity, projection sequence, source revision, exact item identity, and mutation class.

Incremental-before-snapshot, duplicate sequence, sequence gap, cross-stream event, stale source revision, and unsupported mutation class fail closed.

Reconnect starts from a validated cursor or a fresh snapshot. The projection core does not infer delivery from transport success.

**Rationale:** Snapshot-first behavior gives clients a complete base and makes dropped or reordered deltas visible.

### Decision: fold one active turn into one visible assistant item

**Choice:** Multiple assistant ledger entries that share one exact turn identity fold into one visible item in canonical order. The item keeps its identity during streaming, recovery, and terminal replay.

Interposed steering messages add typed markers at their exact ledger positions. They do not become assistant text or disappear during a rebuild.

A terminal replacement can update only the item bound to the same turn, branch, generation lineage, and policy. An unrelated writer cannot replace it through equal content.

**Rationale:** One agent run can contain multiple provider turns and steering inputs. Stable folding prevents duplicate bubbles and lost steering context.

### Decision: make cache convergence explicit

**Choice:** A cache service receives exact snapshot or delta actions and returns applied identities, prior and resulting cursor, durability class, and conflict facts.

Applying the same action twice is an exact no-op. Competing writers converge only when source and resulting identities match. Divergent cache content, partial application, stale cursor, or unsupported schema triggers rebuild or blocks.

A full rebuild clears only the declared projection cache after preflight. It does not delete or rewrite the source ledger.

**Rationale:** Idempotence requires exact action identity and post-state evidence, not best-effort upsert behavior.

### Decision: keep legacy healing conservative

**Choice:** Legacy metadata can be added only through an exact source identity join. A uniquely validated migration key can be used when policy admits it.

Ambiguous timestamps, equal content, missing identities, active-turn state, or visible-content differences produce no heal action. A heal can add admitted metadata but cannot alter visible parts, role, item identity, or source ledger.

**Rationale:** Migration convenience cannot justify fabricated attribution or altered conversation content.

### Decision: apply redaction before projection output

**Choice:** Typed Nickel policy selects visible content classes, hidden reasoning posture, tool detail posture, author attribution, source labels, safe summaries, bounds, and receipt fields.

The pure projection validates already-classified source facts and produces bounded visible items. Hidden reasoning, credentials, raw tool bodies, provider payloads, private paths, and rejected custom metadata cannot enter views or receipts.

**Rationale:** A rebuild must not widen data exposure compared with a live stream.

### Decision: emit metadata-only receipts

**Choice:** Receipts bind schema, policy, source ledger revision, stream, cursor, snapshot or delta, item counts, fold counts, steering counts, rebuild or heal class, cache outcome, redaction, and non-claims.

They omit transcript text, reasoning, tool bodies, provider payloads, credentials, physical paths, raw UI state, and framework data.

**Rationale:** Receipts prove bounded projection facts only. They do not need rendered content.

## State Flow

```text
admitted ledger branch + policy + active turn + optional cursor
                         |
                         v
                 pure input validation
                         |
             +-----------+------------+
             |                        |
             v                        v
      matching cursor          stale or absent cursor
             |                        |
             v                        v
     deterministic delta       deterministic snapshot
             |                        |
             +-----------+------------+
                         v
                host cache application
                         |
                         v
             typed application evidence
                         |
                         v
              metadata-only receipt
```

## Adversarial Cases

- Two user entries have equal text and equal timestamps.
- The ledger rewrites one entry while keeping the same count.
- A client reconnects before its first snapshot.
- A cache applies a delta and crashes before cursor persistence.
- Two assistant entries share one turn with a steering entry between them.
- A live writer and rebuild writer target the same exact item.
- A legacy timestamp matches multiple source entries.
- Redaction policy changes while a cursor remains cached.
- A corrupt cache contains an unknown item or sequence gap.

Each case must produce one deterministic exact action, rebuild, no-op, or blocked outcome.

## Alternatives Considered

### Keep separate authoritative model and render histories

Rejected because conflict resolution lacks one owner. A render repair can then alter provider context accidentally.

### Deduplicate by text, tool-call IDs, or timestamps

Rejected because each value can repeat or collide. Exact canonical source identities are required.

### Use only row count as a high-water mark

Rejected because rewrites, branch changes, and filtered entries can preserve or distort counts.

### Let each UI host invent its own reducer

Rejected because reconnect, redaction, steering, and item identity then drift across TUI, headless, browser, and remote hosts.

## Risks and Trade-offs

- One shared projection contract adds schema and migration work for hosts.
- Strict cursor validation can cause more full rebuilds. Rebuilds are safer than stale incremental repair.
- Folding many provider turns into one item can lose provider-specific presentation. Source ledger facts remain available to separately authorized inspectors.
- This design does not prove host cache or transport correctness.

## Non-Claims

This design does not prove transcript truth, complete event capture, UI delivery, cache durability, transport ordering, renderer correctness, accessibility, privacy beyond selected projection rules, task completion, production readiness, or release eligibility.
