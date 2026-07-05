# Design: Adapt Ornith scaffold profiles for Animus

## Context

Ornith frames a scaffold as the orchestration that shapes solution search. In Animus, that maps to reviewed strategy data: retry posture, validation checklist, tool-ordering preference, compaction preference, memory inclusion policy, and task-category hints. The useful adaptation is to make those strategies explicit and auditable while preserving Animus' typed adapter boundaries.

## Decisions

### 1. Scaffold profiles are data, not executable code

**Choice:** Represent scaffold profiles as typed descriptors with IDs, revision IDs, task-class labels, supported strategy knobs, provenance, review status, route constraints, and BLAKE3 content hashes.

**Rationale:** A scaffold should be comparable, cacheable, reviewable, and safe to include in receipts. Executable generated harnesses would undermine the Steel/Rust boundary and make policy review opaque.

### 2. Selection and task execution are separate phases

**Choice:** Add a pure scaffold-selection plan that chooses an accepted profile before task execution. The selected profile may influence prompt assembly or Steel decision inputs, but every resulting action still crosses existing Rust validation and adapter contracts.

**Rationale:** Ornith's two-stage idea is useful only if the first stage cannot bypass the second stage's safety checks.

### 3. Proposed revisions require host admission

**Choice:** Runtime or evaluation flows may emit scaffold revision candidates, but persistence and future influence require an injected host admission decision.

**Rationale:** Self-improvement should be reviewed and replayable. A model should not silently mutate the strategy that governs future turns.

### 4. Receipts prove scaffold influence without storing transcripts

**Choice:** Runtime and evaluation receipts record scaffold IDs, revision IDs, hashes, task class, review status, route labels, and validation outcomes. Sensitive bodies remain redacted or represented only by BLAKE3 hashes.

**Rationale:** Operators need to know which strategy shaped a run without exposing prompt internals, tool bodies, provider payloads, credentials, or full paths.

### 5. Profiles remain route neutral

**Choice:** Profiles describe strategy constraints and route compatibility, not concrete provider behavior. A profile may declare requirements such as context budget class or tool-surface needs, but provider-specific execution stays behind injected adapters.

**Rationale:** Animus should let compact/local models benefit from reviewed scaffolds without coupling reusable runtime logic to any model family.

## Risks / Trade-offs

- Too many knobs can turn a profile into an unreviewable policy language. Keep the first contract narrow and typed.
- Route constraints can become provider coupling. Express only reusable capability labels and budgets.
- Scaffold receipts can leak prompt strategy if they store bodies. Keep receipts to identities, hashes, labels, counts, and validation status.
- Admission UX belongs to host shells. Reusable crates should expose typed admission requests and outcomes only.
