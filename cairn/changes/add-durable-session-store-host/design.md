## Context

The accepted session-persistence adapter deliberately persists safe metadata in a deterministic fake store and leaves file layout, locking, clocks, encryption, and recovery to hosts. A dogfoodable coding session additionally needs private model-visible messages across restart. Conflating that private journal with safe evidence would violate existing receipt and export boundaries.

Tau uses a simple append-only JSONL file. Animus will adapt append-only incremental persistence and active-leaf restoration, but not direct synchronous append semantics, ambient UUID/time identity, raw-evidence export, or best-effort recovery.

## Decisions

### 1. Add a separate authority-scoped host block

**Choice:** Implement the local backend in a non-default std authority crate or feature, expected to be a focused block such as `animus-std-session`. Reusable crates own only record DTOs, validators, pure commit/recovery plans, and receipt projection.

**Rationale:** Filesystem handles, permissions, locks, synchronization, and recovery are effects and must not enter `animus-core`, `animus-adapters`, or reusable runtime defaults.

### 2. Require an explicit store descriptor

**Choice:** Opening a store requires a capability-scoped location handle, store/schema/profile identity, session and branch scope, size bounds, durability policy, lock policy, private-data posture, encryption posture, and safe receipt policy. No home-directory, current-directory, or global-session fallback is provided.

**Rationale:** A host-selected store is authority. Ambient discovery would make persistence surprising and hard to review.

### 3. Separate private journal and safe ledger projections

**Choice:** The store contains replay-private payload records and metadata-only safe ledger records or sidecars. Private records may contain admitted model-visible message bodies and tool-result bodies needed for replay. Safe records contain only identities, kinds, ordering, counts, redaction classes, status, and receipt references. The two channels bind through BLAKE3 identities.

**Rationale:** Durable session continuity requires private content, while evidence, diagnostics, and default exports must remain safe. Calling both channels one "safe ledger" would blur the claim boundary.

### 4. Use versioned bounded framing

**Choice:** Each frame carries a schema/profile version, record kind, monotonic logical sequence, parent/branch identity, bounded payload length, payload BLAKE3 hash, prior committed-frame hash, and frame identity. A separate commit frame binds an ordered transaction group and proposed active leaf. Payload encoding may use a reviewed Preserves adapter, but framing and validation do not depend on ambient format inference.

**Rationale:** Newline-delimited text alone cannot reliably distinguish an incomplete tail from a malformed committed record and may not safely frame arbitrary payloads.

### 5. Treat message plus leaf movement as one logical commit

**Choice:** User, assistant, tool-result, summary, repair, and branch-navigation updates are committed as bounded transaction groups. The active leaf advances only through a valid commit frame. Readers ignore an uncommitted trailing group and never infer the head from file order alone.

**Rationale:** A crash between a message append and leaf append must not select an ambiguous branch.

### 6. Declare durability policy explicitly

**Choice:** The first durable evidence profile requires data synchronization after each committed group before dependent provider or tool effects continue. Weaker buffered profiles may exist only with a different non-durable claim label and cannot satisfy the durable-session readiness rung.

**Rationale:** Returning success before the selected durability boundary would overstate restart guarantees.

### 7. Enforce one writer and bounded readers

**Choice:** The local backend acquires an exclusive writer lease and rejects conflicting writers, stale lease identities, and unsupported lock states. Readers operate over a validated committed snapshot and bounded file/record budgets. There is no last-writer-wins merge.

**Rationale:** Parent hashes and active leaves cannot remain coherent under uncontrolled concurrent append.

### 8. Recover tails but fail on committed corruption

**Choice:** Recovery classifies clean end, incomplete trailing frame, incomplete trailing transaction, stale uncommitted head, committed hash mismatch, sequence gap, unsupported schema, and mid-file corruption. It may ignore or quarantine only an uncommitted trailing fragment after recording a safe diagnostic. Committed corruption, stale head ambiguity, or unsupported migration fails closed.

**Rationale:** Tail recovery is useful; silent truncation or skipping committed corruption is data loss disguised as success.

### 9. Keep identity deterministic and clock-independent

**Choice:** Record, transaction, branch, snapshot, and receipt identities use domain-separated BLAKE3 over canonical validated inputs. Wall-clock observations may be host-supplied optional metadata but never determine replay order, uniqueness, or content identity.

**Rationale:** Ambient UUID/time generation impedes deterministic fixtures and can hide ordering bugs.

### 10. Make private-data posture explicit

**Choice:** The first backend requires restrictive host permissions and records whether encryption is required, externally provided, or explicitly absent. If policy requires encryption and no admitted encryption service exists, opening the store fails. No encryption subsystem or key discovery is added by this change.

**Rationale:** Private replay content must not be mistaken for safe evidence, while key management remains a separate authority surface.

### 11. Validate through a backend conformance corpus

**Choice:** The fake and local-file implementations run the same append, commit, replay, branch, compaction, repair, restart, tail-tear, corruption, stale-head, lock-conflict, and leak tests. File-specific tests use explicit temporary capability roots supplied by the test shell.

**Rationale:** A shared corpus proves semantic parity without making the fake store evidence for filesystem behavior.

## Risks / Trade-offs

- Per-group synchronization costs latency. The durable profile prioritizes honest restart claims; batching can be a separately named profile later.
- A private journal increases sensitive-data handling obligations even though receipts remain safe.
- Advisory locking differs across filesystems. Unsupported or unproven lock semantics must block the durable claim.
- BLAKE3 chaining detects corruption and order changes but is not authentication without a separately managed key.

## Non-Goals

- A default global session directory.
- Database, daemon, cloud, remote, or multi-writer storage.
- Built-in encryption, key discovery, backup, replication, or retention automation.
- Filesystem correctness, power-loss proof, or cross-filesystem parity.
- Exposing private journal records through receipts, logs, or default exports.
