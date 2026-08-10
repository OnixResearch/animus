## Context

The accepted hashed-edit contract validates in-memory snapshots and tagged spans. It correctly leaves file reads, locks, writes, path authority, and atomic replacement to a host.

The missing host must not weaken that contract. It also needs a visibility rule so a caller cannot mutate lines outside its observed revision.

Autolith supplies the revision and visibility reference. OnixResearch Cap Root and Durable File Publication supply reviewed mechanism boundaries.

The host remains a consumer. It retains resource authority, replacement policy, diagnostics, evidence, and all semantic claims.

## Decisions

### Decision: use a separate non-default authority-scoped host

**Choice:** Create a focused workspace-edit host crate or std block. Its functional core validates loaded facts and returns a publication plan.

A thin shell owns capability-relative reads, synchronization, staging, rename, and durability operations.

Default, no-std, Wasm, provider, TUI, and unrelated std surfaces do not acquire filesystem authority.

**Rationale:** Concrete file mutation must not enter reusable runtime crates.

### Decision: bind snapshot visibility and text shape

**Choice:** The host snapshot extends `HashedEditSnapshot` with capability-root identity, logical resource identity, revision identity, visible tagged spans, line-ending mode, final-newline state, and complete content hash.

Visibility is expressed through existing tags and contiguous one-based spans. Raw absolute paths do not enter the pure plan.

**Rationale:** A complete content hash proves freshness, but not that the caller observed the target lines.

### Decision: constrain insertions to visible boundaries

**Choice:** Replacement and deletion require every affected line to be visible in the exact snapshot revision.

Insertion before or after a line requires the adjacent visible boundary named by policy. End-of-file insertion requires an explicit visible end boundary.

Unknown, partial, non-contiguous, or stale visibility evidence fails before mutation.

**Rationale:** Insertions can affect hidden structure even when they replace no line.

### Decision: preserve line endings explicitly

**Choice:** The pure apply core uses the admitted newline policy, original line-ending mode, and final-newline state.

Mixed line endings require an explicit supported policy. Silent normalization is denied.

The resulting bytes and after-snapshot BLAKE3 identity are computed before the shell starts publication.

**Rationale:** Text mutation must not create unrelated byte changes.

### Decision: re-read immediately before publication

**Choice:** The shell first reads through the explicit capability root and constructs the snapshot. After pure plan validation, it acquires the host's resource lock and reads the file again.

The second read must match the expected snapshot hash, revision, metadata policy, and capability-root identity. Drift returns stale without staging publication.

**Rationale:** Validation over an earlier read does not protect the publication window.

### Decision: serialize only Animus-owned writers

**Choice:** The host uses a resource-scoped writer generation for Animus operations. Matching lock and generation evidence is required before publication.

The contract states that external processes can still race after the final read. It never presents the host lock as a filesystem-wide compare-and-swap.

**Rationale:** Honest serialization is useful even when it is not universal exclusion.

### Decision: model publication outcomes precisely

**Choice:** Publication states include `Prepared`, `Staged`, `Published`, `DurabilityPending`, `DurabilityComplete`, `FailedBeforePublication`, and `UncertainAfterPublication`.

The shell uses Durable File Publication only where its exact replace and durability contract matches the operation. Otherwise, the Animus host owns the narrower replace shell and its non-claims.

A failure after rename cannot claim rollback unless the host supplies separate exact rollback evidence.

**Rationale:** Atomic visibility and durable storage are separate observations.

### Decision: use capability-relative authority

**Choice:** The shell receives an already-authorized capability root and logical relative resource. Cap Root validates lexical admission before capability-relative I/O.

Symlink, no-follow, metadata, ownership, permission, file-kind, and replacement rules remain explicit host policy and observations.

**Rationale:** Lexical admission does not open or authorize a filesystem root.

### Decision: emit metadata-only receipts

**Choice:** Receipts bind root, resource, revision, before, after, plan, visibility, writer-generation, publication-state, durability-state, redaction, and BLAKE3 identities.

They reject file bodies, replacement text, absolute paths, credentials, VCS data, process data, and unrestricted errors.

**Rationale:** Edit evidence needs identity and outcome, not source disclosure.

## Functional Core

The pure core receives:

- admitted snapshot metadata and bounded source bytes;
- a validated hashed-edit request;
- visibility policy and selected tags;
- expected writer and revision generations; and
- supported publication and newline policy facts.

It returns either a typed denial or:

- exact after bytes;
- before, after, and plan identities;
- one resource-scoped publication plan;
- required pre-publication observations; and
- a receipt draft without private data.

## Imperative Shell

The shell performs only:

1. capability-relative open and first read;
2. pure snapshot construction;
3. pure edit admission and byte construction;
4. resource lock acquisition;
5. exact second read and freshness comparison;
6. bounded staging and publication;
7. durability observations; and
8. safe outcome projection.

## Alternatives Considered

### Write directly through the existing host trait

Rejected because path authority, visibility, second-read freshness, and publication outcomes would remain unspecified.

### Use fuzzy old-text matching

Rejected because repeated text and stale context create ambiguous targets.

### Treat a rename as complete durability

Rejected because directory and file synchronization can fail after visibility changes.

### Claim compare-and-swap from a process lock

Rejected because another process can mutate the file outside the lock.

## Risks and Trade-offs

- Exact visibility can reject useful edits after small unrelated changes. The caller can read a fresh snapshot and retry.
- Capability and publication dependencies add license and version review work.
- Mixed line endings need explicit policy and more fixtures.
- Post-publication failures can return uncertain outcomes that require operator review.

## Non-Claims

This design does not prove universal writer exclusion, semantic edit correctness, authorization, sandboxing, symlink safety without host policy, durable storage correctness, VCS consistency, multi-file atomicity, production readiness, or release eligibility.
