# Design: Qualifier-inspired safe quality records for Animus

## Context

Qualifier is a Rust CLI/library for continuous annotations stored as `.qual` JSONL files beside source code. Its strongest transferable ideas are not the product CLI itself, but the data discipline: a small Metabox envelope, deterministic BLAKE3 record IDs, append-only files, unknown-record preservation, threaded references, supersession instead of mutation, and span content hashes for drift detection. It also defines AGENTS-CLI, a compact convention for CLI tools that teach AI coding agents how to use them.

Animus already has active Cairn work for safe receipts, neutral ledgers, VCS-neutral provenance, BLAKE3 canonical identities, injected dispatch, Nickel policy, and redaction. Qualifier should therefore be adapted as a compatibility/reference layer rather than a source of runtime authority.

## Decisions

### 1. Qualifier is a bounded quality-record reference

**Choice:** Use Qualifier as a bounded reference for ambient review annotations, Metabox-style envelopes, append-only JSONL sidecars, span freshness, and AGENTS-CLI guidance. Do not make Qualifier the Animus session ledger, transcript store, scoring engine, or core dependency.

**Rationale:** The annotation semantics are directly useful for reviewable agent work. The CLI product and Git-oriented diff behavior are less reusable than Animus-owned provenance, receipt, and backend abstractions.

**Boundary:** Reusable `animus-core` and `animus-adapters` types must be expressible without `std`, filesystem walking, clocks, Git, Qualifier crates, JSONL storage, or concrete CLI behavior.

### 2. Animus owns the safe record envelope

**Choice:** Define Animus-owned record envelopes using the same proven shape: version, type, subject, issuer, optional issuer classification, creation metadata supplied by the shell, content-addressed BLAKE3 id, and a typed body.

**Rationale:** A uniform envelope lets generic tooling query who said what about which subject without knowing every body schema. Owning the type avoids accidentally inheriting Qualifier release cadence, dependency choices, or raw-body assumptions.

**Boundary:** Canonicalization and validation are pure functions over in-memory DTOs. The shell provides time, issuer identity, file reads, and persistence.

### 3. Append-only sidecars are optional interchange, not internal storage

**Choice:** Add Qualifier-compatible `.qual` read/write behavior only as an optional std/CLI adapter. Internal provenance and receipts remain Animus-owned and may be serialized through Preserves or embedder storage.

**Rationale:** `.qual` files are excellent for repository-local review observations, but they should not define session replay, runtime receipt persistence, or model-visible context.

**Boundary:** The adapter preserves unknown records during read/compact/write operations and rejects unsafe records before projecting them into Animus evidence. Core logic sees safe DTOs, not raw JSON.

### 4. Annotation lifecycle is adapted as pure state transitions

**Choice:** Adapt Qualifier annotation fields and lifecycle ideas: built-in kinds, custom kinds with typo checks, threaded `references`, `supersedes` chains, `resolve` records, `waiver` rationale, suggested fixes, same-subject supersession, cycle detection, and active-record filtering.

**Rationale:** These semantics match review conversations and safe issue closure. Supersession keeps history immutable while letting active views hide resolved concerns.

**Boundary:** Animus must not auto-resolve human-authored annotations just because an agent changed nearby code. Resolution is an explicit caller/user decision.

### 5. Span freshness is VCS-neutral

**Choice:** Reuse the idea of hashing full lines covered by a span with BLAKE3 and classifying freshness as fresh, drifted, missing, or no-hash. Implement this as pure line/span logic plus a thin shell that reads files.

**Rationale:** This complements planned VCS-neutral range provenance and gives agents a safe way to surface stale observations after edits.

**Boundary:** Git branch comparison, merge-base logic, and `qualifier diff` internals are not adopted into core. Any VCS comparison uses Animus' injected backend model.

### 6. AGENTS-CLI informs future Animus CLI guidance

**Choice:** When Animus exposes a public CLI, provide `animus agents` with a self-contained orientation page, topic pages for concepts/workflows/pitfalls and non-trivial commands, markdown output, discoverability in `--help`, and structured unknown-topic failures.

**Rationale:** Animus is itself an agent-harness framework. A first-contact guide for coding agents reduces misuse better than generated flag tables alone.

**Boundary:** Prompt/guidance text stays compact, version-locked to the CLI, and explicit about safety constraints. It must not replace repo-owned docs or policy specs.

## Risks / Trade-offs

- Qualifier's `.qual` ergonomics can tempt using sidecar files as the runtime ledger. Keep sidecar support optional and adapter-owned.
- Metabox-like envelopes can become generic catch-alls. Require typed bodies, positive/negative validation, and explicit redaction rules before storing evidence.
- Unknown-record preservation can conflict with fail-closed redaction. Preserve raw unknown lines only in the adapter's sidecar round-trip path; expose only sanitized metadata to Animus core.
- AGENTS-CLI pages can become stale prompt bloat. Keep them short, generated into the binary from maintained source pages, and covered by topic-discovery tests.
- Qualifier's Git diff behavior is useful but Git-specific. Reimplement only the VCS-neutral semantics that fit Animus' backend boundary.
