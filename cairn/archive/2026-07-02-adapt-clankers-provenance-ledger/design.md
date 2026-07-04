# Design: Animus-owned provenance ledger from bounded Clankers and Toolpath references

## Context

Animus has small no-std core and adapter crates, a Preserves bridge, a Steel loop fabric, and a standard-library shell. Active Cairn work already calls for safe receipts, neutral session ledger behavior, injected dispatch, and VCS-neutral provenance. The remaining design question is whether Animus should import Toolpath or Clankers structures directly, or define its own internal model.

Clankers is a closer internal reference because its reusable pieces already emphasize safe metadata, BLAKE3 artifact identity, neutral replay DTOs, injected tool/provider boundaries, and redaction. Toolpath is valuable because it names a public provenance document format with Graph/Path/Step DAGs, actor taxonomy, raw/structural change perspectives, typed refs, JSONL streaming, and review renderers. The right split is: Animus owns the internal ledger; Toolpath is an adapter target and review vocabulary.

## Decisions

### 1. Animus owns the internal provenance ledger

**Choice:** Define Animus-owned DTOs for provenance and session replay rather than depending on `toolpath`, `toolpath-convo`, `clankers-session`, or `clanker-message` as core runtime contracts.

**Rationale:** Animus crates have stricter dependency and embeddability goals than either reference. Owning the DTOs lets `animus-core` and `animus-adapters` stay small while preserving the option to serialize through Preserves or other boundary formats.

**Boundary:** Reference crates may be used in optional adapters or tests only after dependency isolation is verified. Core DTOs must be expressible without `std`, clocks, filesystem, provider clients, daemon state, or concrete storage.

### 2. Clankers is the primary bounded internal reference

**Choice:** Adapt the patterns, not the product: neutral session ledger replay, safe typed ledger facts, artifact hashes/redaction classes, tool-host outcomes, effect requests/results, and receipt sanitization.

**Rationale:** These Clankers patterns match Animus' direction: FCIS split, injected services, no raw payload receipts, and deterministic replay. They are more directly useful than Toolpath's interchange-focused crates for internal runtime behavior.

**Out of scope:** Automerge desktop sessions, Clankers transcript compatibility records, daemon/attach control plane, TUI, Matrix/P2P, plugin runtime, self-evolution, product CLI, and global session directories.

### 3. Toolpath is optional interchange

**Choice:** Add Toolpath compatibility only as a std-only adapter that projects Animus provenance into Toolpath Graph/Path/Step documents and can later import redacted Toolpath documents into Animus review fixtures.

**Adapt:** Use Toolpath's Graph/Path/Step shape as the public interchange vocabulary; map Animus branch parents and active heads onto Toolpath `step.parents` and `path.head`; render Animus actor types into Toolpath-compatible `human:`, `agent:`, `tool:`, and `ci:` labels; project safe structural changes, artifact hashes, intents, refs, and correlation links into Toolpath `change` and `meta` fields; and keep JSONL streaming plus Markdown/DOT rendering as optional shell/review surfaces.

**Rationale:** Toolpath's Graph/Path/Step, actor taxonomy, raw/structural perspectives, typed refs, and dead-end queries are useful for sharing provenance. They should not dictate Animus core storage, replay semantics, identity hashing, or receipt leakage policy.

**Boundary:** Default exports are redacted and safe: raw prompts, provider payloads, tool inputs/outputs, full paths, credentials, unbounded before/after contents, and raw unified diffs are omitted or represented by BLAKE3 hashes/redaction markers unless a future explicit policy enables full-fidelity export. Toolpath Pathbase, cache/share/resume CLI behavior, Git/GitHub derivation, provider session importers, and `toolpath-convo` are not core Animus runtime dependencies.

### 4. Ledger replay is a pure functional core

**Choice:** Model session replay with entries such as message, summary, usage, receipt, and unsupported. Replaying entries returns model-visible context plus safe metadata, or a typed unsupported-entry error.

**Rationale:** Replay must be testable without a session store, database, daemon, or provider. Clankers' neutral ledger split is a good pattern, but Animus should keep the type surface smaller.

### 5. Typed facts are append-only safe metadata

**Choice:** Add typed facts for model calls, tool calls, runtime blocks, errors, artifact references, authorization decisions, provenance observations, todos/work items when needed, and opaque unknown records. Every fact carries safe query fields only.

**Rationale:** Append-only facts support audit, search, and receipt evidence without turning the ledger into a raw transcript store. Unknown future records should round-trip as opaque safe metadata or fail closed without leaking raw payloads.

### 6. Artifact identity uses BLAKE3 canonical envelopes

**Choice:** Use BLAKE3 for Animus-owned artifact hashes and receipt IDs. Canonical envelopes include kind, version, redaction class, normalized payload, and sorted dependencies.

**Rationale:** This matches Animus hashing defaults and Clankers' safe artifact identity pattern. Toolpath or Git AI adapters may use externally mandated SHA-256 only at compatibility boundaries.

### 7. Branch/dead-end queries are internal and Toolpath-compatible by projection

**Choice:** Provide pure parent/branch query helpers for ancestry, branch divergence, unique branch entries, and abandoned/dead-end attempts. Toolpath export can project these into Step parents and Path head semantics.

**Rationale:** Animus needs branch-aware session/provenance behavior internally. Toolpath's DAG shape is useful, but Animus queries should operate over Animus DTOs so they can be tested before any exporter exists.

### 8. Receipts are safe evidence, not transcript archives

**Choice:** Runtime and provenance receipts include status, issue code, actor/session/trace labels, bounded counts, artifact hashes, mutation/effect kinds, backend/interchange labels, and redaction markers. They exclude raw prompt text, provider payloads, credentials, full paths, raw tool bodies, raw model outputs, and unbounded outputs.

**Rationale:** This preserves reviewability while protecting operator data and secrets.

## Risks / Trade-offs

- Owning DTOs means more code than importing Toolpath directly. The benefit is boundary control and no accidental std/storage leakage.
- Adapting from Clankers can pull product assumptions into Animus. Every adapted field must pass a bounded-reference review.
- Toolpath interoperability can tempt full transcript export or raw diff leakage. Keep the first adapter redacted and policy-gated.
- Canonical envelopes need stable fixtures. Start with a small versioned shape and positive/negative tests before treating hashes as public contracts.
- Branch queries and VCS provenance can overlap. Keep session/provenance DAG queries VCS-neutral and let VCS-specific storage remain behind injected backends.
