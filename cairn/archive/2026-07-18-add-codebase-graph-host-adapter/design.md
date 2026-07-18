## Context

`animus-adapters` already owns host-injected service contracts, `animus-std` owns capability-scoped filesystem shells, context references own typed retrieval and provenance, retrieval budgets own stop/repeat policy, and the tool executor owns final capability and output admission. A codebase graph should compose those owners rather than become a new ambient repository subsystem.

The Grok Build implementation is prior art for incremental tree-sitter indexing and direct query handles. The adaptation keeps its useful mechanics while replacing global cache assumptions and product coupling with Animus profiles, capability roots, BLAKE3 identities, and safe receipts.

## Decisions

### 1. Keep contracts separate from parser and filesystem dependencies

**Choice:** `animus-adapters` defines transport-neutral code-graph requests, result DTOs, status observations, budgets, and failure classes. A focused optional std host block owns tree-sitter language dependencies, repository reads, watcher integration, parallel work, mmap or ordinary cache I/O, and service lifecycle.

**Rationale:** Embedders that do not need repository indexing must not inherit parser, thread, filesystem, or cache authority.

### 2. Author one typed index profile

**Choice:** Nickel is the source of truth for safe profile labels, admitted language/query versions, capability-root labels, include/exclude policy, binary and special-file behavior, named file/count/byte/result/depth budgets, event-overflow policy, cache posture, and receipt redaction. Generated runtime data is revalidated in Rust, and canonical profile identity uses domain-separated BLAKE3.

**Rationale:** Parser/query changes and resource ceilings materially affect answers and must be reviewable and identity-bound rather than hidden literals.

### 3. Split deterministic graph logic from the I/O shell

**Choice:** Pure functions consume already-loaded relative file observations, language/query profiles, prior graph state, and explicit file events to produce parsed contributions, graph deltas, stale-state decisions, deterministic query plans, stable result order, and safe projections. The imperative shell opens capability-scoped roots, reads bounded bytes, receives watcher events, schedules work, and writes or loads cache artifacts.

**Rationale:** Initial and incremental logic can be tested without a live watcher, repository, thread pool, or cache file.

### 4. Fail closed on incomplete incremental state

**Choice:** Explicit create, modify, delete, and rename events update the graph only after the new observation passes root, type, language, encoding, and budget checks. Watcher overflow, sequence gaps, root replacement, ambiguous rename, unsupported event class, or failed required read marks the index stale. Trusted queries remain blocked until a bounded rescan establishes a new source snapshot identity.

**Rationale:** Returning an answer from a silently incomplete graph is more dangerous than an explicit unavailable result.

### 5. Make caches disposable and identity bound

**Choice:** Cache keys bind workspace safe identity, source snapshot or bounded manifest identity, index profile, language/parser cohort, query version set, schema version, and canonical graph identity using BLAKE3. Cache corruption, version mismatch, unknown fields, unsafe path material, or identity mismatch causes quarantine/rebuild or a typed unavailable result; the cache never mutates source files or becomes authoritative history.

**Rationale:** Derived indexes should improve startup without becoming a second source of truth.

### 6. Use capability roots and repository-relative locations

**Choice:** The shell resolves a caller-supplied capability root and returns validated repository-relative locations only. It rejects traversal, symlink escape, control namespaces, special files, binary/NUL input, unsupported encoding, ignored roots not admitted by profile, and over-budget files before parsing.

**Rationale:** A graph query must not widen filesystem reach or leak physical checkout paths.

### 7. Integrate through existing retrieval and tool gates

**Choice:** Definition, reference, symbol, and scope queries are exposed through an injected service, context-reference resolver role, or explicit read-only tool capability pack. Every query passes existing retrieval budgets and result bounds. Mutating tools may notify the host service after a committed edit, but the graph service itself cannot edit files or authorize tools.

**Rationale:** Indexing is retrieval support, not a parallel tool runtime.

### 8. Preserve claim boundaries in results and receipts

**Choice:** Results carry language/query/profile/source/index identities, bounded repository-relative spans, freshness status, truncation, and syntactic-evidence labels. Receipts contain metadata only and explicitly deny compiler-resolution, type, semantic, completeness, proof, and release claims.

**Rationale:** Tree-sitter queries are useful but are not a compiler or verifier.

## Risks / Trade-offs

- Tree-sitter queries can produce false positives or miss language semantics; explicit syntactic claims and language fixtures bound that risk.
- Watchers can lose events; stale-state blocking and full-rescan recovery trade availability for honesty.
- Large repositories can consume memory and CPU; named profile budgets and direct in-place queries prevent unbounded cloning and work.
- Nickel and Steel are important Onix languages but are not automatically supported by a Rust-first profile.
- Optional mmap and parallelism improve performance but remain host implementation details, not public semantic guarantees.

## Non-Goals

- Compiler-equivalent name resolution, type checking, semantic proof, call-graph completeness, or repository correctness.
- Ambient repository discovery, global caches, unrestricted filesystem watching, or indexing outside admitted capability roots.
- File mutation, automated refactoring, VCS operations, source formatting, or tool authorization.
- Claiming Nickel, Steel, Go, JavaScript, TypeScript, Python, or other language support without dedicated grammar/query fixtures.
- Moving tree-sitter, mmap, threads, filesystem, or cache dependencies into no-std reusable crates.
