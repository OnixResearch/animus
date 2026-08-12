## Context

Provider APIs generally require each assistant tool call in submitted history to have one matching terminal tool result. Animus already validates orphaned tool results and preserves tool call/result pairs across compaction, but cancellation or process loss can leave the inverse defect: a committed assistant call with no committed result. The defect may appear at the active tail or earlier in an active branch followed by later messages.

Tau repairs this condition in memory and on session load. Animus will adapt the invariant while strengthening cause classification, append-only semantics, idempotence, receipt safety, and functional-core / imperative-shell ownership.

## Decisions

### 1. Repair only validated active-branch context

**Choice:** The pure core consumes an already-loaded active-branch transcript with stable entry, call, result, run, and leaf identities. It does not scan storage, sibling branches, provider files, or ambient sessions.

**Rationale:** Only the context selected for the next provider request must be provider-valid. Sibling history must not change merely because another branch is resumed.

### 2. Match calls and results by explicit identity

**Choice:** A result satisfies exactly one preceding assistant tool call with the same call identity and compatible tool identity. Duplicate call IDs, duplicate results, result-before-call ordering, mismatched tools, unsupported entry kinds, or stale leaf identity fail closed.

**Rationale:** Treating a result found anywhere in history as sufficient can bind unrelated calls and hide corruption.

### 3. Separate diagnosis from repair planning

**Choice:** A pure diagnostic pass returns matched calls, unmatched calls, malformed bindings, and the earliest affected entry. A second pure function builds ordered repair actions only when diagnostics contain repairable unmatched calls and no structural corruption.

**Rationale:** Hosts and tests can inspect why repair was or was not admitted without invoking effects or conflating malformed data with recoverable interruption.

### 4. Interruption reason is evidence-bound

**Choice:** Synthetic results use one of `user_cancelled`, `host_cancelled`, `unclean_shutdown`, or `unknown_interruption`. Explicit correlated cancellation evidence is required for either cancellation class. A missing result alone maps to `unknown_interruption`.

**Rationale:** Tau's generic "interrupted by user" text can make an unsupported causal claim after crashes or older malformed sessions.

### 5. Repair never re-executes a tool

**Choice:** Repair actions create terminal synthetic failure results only. They cannot produce tool requests, executor bindings, mutation permits, approvals, retries, or fallback dispatch.

**Rationale:** The external effect may already have happened even when its result was not committed. Re-execution could duplicate writes, commands, or network operations.

### 6. Preserve append-only history

**Choice:** A tail repair appends synthetic results after the affected assistant entry. A historical repair creates an append-only repaired active branch or equivalent typed replay mutation that preserves the old branch and reconstructs ordering without rewriting committed entries. The plan binds the old leaf, repair anchor, affected suffix identities, and proposed new leaf.

**Rationale:** In-place insertion would invalidate content identities and provenance. Historical suffix handling must be explicit rather than silently mutating replay order.

### 7. Make application idempotent

**Choice:** Repair action and receipt identities are domain-separated BLAKE3 hashes over session, branch, old leaf, call, reason, and ordered plan identities. Reapplying a committed plan returns an already-applied outcome; it does not append another synthetic result.

**Rationale:** Recovery may run after uncertain shutdown or repeated resume attempts.

### 8. Gate provider continuation on durable application

**Choice:** Before a host sends reconstructed context to a provider, it must either prove the active branch satisfies the invariant or durably apply the admitted repair and revalidate replay. Store failure, stale head, partial application, or malformed replay blocks provider dispatch.

**Rationale:** An in-memory-only repair can disappear on the next restart and recreate provider rejection.

### 9. Keep receipts metadata-only

**Choice:** Repair receipts include safe session/branch labels or hashes, old/new leaf hashes, call count, reason classes, action count, status, redaction marker, and plan/receipt BLAKE3 identities. They exclude prompts, tool names when unsafe, arguments, outputs, private paths, provider payloads, and exception bodies.

**Rationale:** Operators need to prove repair ordering without turning recovery evidence into a transcript copy.

## Risks / Trade-offs

- Historical branch reconstruction is more complex than tail repair. Explicit repair actions and active-leaf binding prevent hidden mutation but require careful conformance fixtures.
- A synthetic failure makes provider context structurally valid but cannot determine whether an external side effect occurred.
- Strict duplicate-ID rejection can block old sessions that another product might heuristically accept. Fail-closed import diagnostics are preferable to a wrong binding.

## Non-Goals

- Retrying or resuming interrupted tools.
- Inferring cancellation cause from missing records.
- Repairing provider-native transcripts outside validated Animus replay data.
- Rewriting, deleting, or compacting old branch history as part of repair.
- Claiming exactly-once effects, provider compatibility, or production recovery.
