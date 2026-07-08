# Design: Add orchestration adapter family

Animus should expose host-backed orchestration effects the same way it exposes other swappable families: pure DTOs and reducers stay reusable, while effectful storage, scheduling, supervision, clocks, VCS commands, filesystem authority, network transport, and persistence cross explicit adapter contracts.

## Decisions

### 1. Do not add an orchestration crate for this step

**Choice:** Keep `animus-runtime` as the reusable orchestration owner and add a public adapter family for host-owned orchestration effects instead of creating `animus-orchestration`.

**Rationale:** The current orchestration surface is tightly coupled to runtime-owned task graph decisions. A new crate would add dependency and compatibility cost before there is a separate no-std decision core large enough to justify its own barrier.

### 2. Promote the host-effect seam to `animus-adapters`

**Choice:** Move or bridge the `KanbanBoardService`-style service contract into an implementation-free `family:orchestration` module in `animus-adapters`. The contract should consume validated labels, graph/handoff DTOs, schedule/admission requests, cancellation metadata, and bounded evidence handles.

**Rationale:** Embedders should implement host services against the smallest stable boundary. `animus-adapters` is the existing no-std crate for component contracts, so it is the right place for host-owned orchestration ports.

### 3. Keep runtime logic pure and fail-closed

**Choice:** `animus-runtime` continues to own pure graph validation, dependency promotion, workspace mutation admission, and tenant-safe context assembly. Runtime shell helpers may call injected orchestration adapters, but missing or denied services return typed failures without discovering storage or creating schedulers.

**Rationale:** This preserves the functional-core / imperative-shell split and prevents reusable runtime defaults from gaining ambient process, filesystem, network, or persistence authority.

### 4. Preserve compatibility while moving the seam

**Choice:** Existing public runtime exports should keep working through aliases, bridge traits, or a documented compatibility module until a later accepted change removes them. New code should prefer the `animus-adapters` family path.

**Rationale:** `KanbanBoardService` is already exported through `animus-runtime`. A hard break would punish embedders for adopting the current seam before the adapter family existed.

### 5. Treat evidence as safe metadata, not durable board content

**Choice:** Orchestration adapter outcomes and receipts record safe family labels, operation kinds, task or board labels, status classes, counts, redaction markers, and BLAKE3 hashes of bounded DTOs. They must not store raw prompts, provider payloads, credentials, full paths, hidden reasoning, unbounded logs, or host database records.

**Rationale:** Orchestration is likely to touch sensitive task content and workspace labels. Durable evidence should prove the seam without becoming an unbounded data export.

### 6. Add conformance before implementation closeout

**Choice:** Closeout requires contracts-index and facade-inventory entries plus focused tests for positive service injection and negative fail-closed cases.

**Rationale:** Adapter families are only useful if embedders can find the owner module, authority boundary, evidence behavior, and minimum conformance expectations.

## Risks / Trade-offs

- Keeping orchestration in runtime may delay a future crate split. This is acceptable until the pure orchestration decision core grows independently of runtime loop composition.
- Bridging runtime-local names adds temporary compatibility surface. The benefit is a safer migration path for embedders.
- A no-op or fixture std adapter does not prove production scheduling. Evidence must label concrete schedulers, board databases, and persistence as host-owned non-claims unless separately implemented.
