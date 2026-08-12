# Design: Record subagent lineage in the ledger

Subagent orchestration needs a durable audit trail. The ledger should record safe lineage facts, not raw transcripts or execution-side effects.

## Decisions

### 1. Lineage record is child-run kind neutral

**Choice:** A lineage record covers parent id, child id, child-run kind, delegation source, admission identity, effective capability identity, context packet identity, output hash, handoff mode, terminal status, and failure class.

**Rationale:** All child-run kinds need the same audit skeleton.

### 2. Receipts store hashes and summaries only

**Choice:** Lineage receipts use BLAKE3 identities, safe labels, redaction markers, bounded summaries, and evidence references.

**Rationale:** Audit should not require raw prompt, child transcript, credentials, or unbounded tool output storage.

### 3. Replay validates influence boundaries

**Choice:** Replay checks that parent-visible child influence has a matching lineage record, accepted handoff mode, context packet identity, and output hash.

**Rationale:** Parent state should not silently depend on untracked child output.

### 4. Coordinator events project into lineage

**Choice:** Coordinator lifecycle events provide the imperative shell feed; pure ledger projection validates and normalizes records.

**Rationale:** Execution backends remain host-owned while ledger logic stays deterministic and testable.
