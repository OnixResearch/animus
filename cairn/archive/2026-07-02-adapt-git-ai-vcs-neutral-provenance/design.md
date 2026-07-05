# Design: VCS-neutral provenance from Git AI patterns

## Context

Animus currently has a small functional core, adapter contracts, a Preserves bridge, a Steel loop fabric, a standard-library shell, and active Cairn work for a runtime loop, policy, receipts, session ledger, and injected dispatch. Git AI offers a concrete implementation of explicit agent-authorship tracking, but it is intentionally Git-native and product-oriented. Animus needs the reusable provenance ideas without making Git notes, Git hooks, Git command semantics, or Git history rewrites part of the core harness contract.

## Decisions

### 1. Git AI is a compatibility reference, not the source of Animus provenance authority

**Choice:** Use Git AI for concepts and optional interoperability: checkpoint-before/checkpoint-after, line-level attribution, session/model/tool identity, accepted-line stats, and the v3 authorship-log format. Do not import its daemon, installer, hosted telemetry, update flow, local SQLite store, or Git operation watcher.

**Rationale:** Animus is an embeddable harness framework. A Git-specific product architecture would couple runtime logic to one VCS and would duplicate planned receipt, ledger, policy, and adapter boundaries.

### 2. Provenance domain is VCS-neutral

**Choice:** Define the primary provenance model in terms of content observations, repository-relative file identities, content hashes, line ranges, session identities, trace IDs, mutating tool kinds, and safe receipt metadata. Core provenance types must not mention `refs/notes/ai`, commit SHAs, Git commands, Git hooks, branch names, stash, rebase, or any other Git-only concept.

**Rationale:** The same provenance event can be stored in Git notes, a Jujutsu-compatible side store, a database, a Preserves log, or an embedder-provided service. Core logic should be testable without standing up any VCS.

### 3. Version-control storage is an injected backend

**Choice:** Add a version-control provenance backend boundary with operations such as recording checkpoints, loading attribution for a revision/content state, and persisting backend-specific notes or sidecar data. Runtime and provenance cores consume that trait, not concrete Git code.

**Rationale:** This preserves Git AI interoperability while leaving room for non-Git VCS adapters. A fake in-memory backend should be the first verification target; Git support is an adapter, not the definition of provenance.

### 4. Mutating-tool checkpointing happens at the tool-dispatch boundary

**Choice:** Observe mutating tool calls through a typed classifier and dispatcher wrapper. The classifier maps raw tool names to canonical mutation kinds such as edit, write, replace, and rename using Nickel-authored policy. The observer records before/after content observations and touched file identities around the actual tool call.

**Rationale:** Git AI's agent hooks prove the lifecycle, but Animus already owns typed tool dispatch. Recording at that boundary avoids per-agent hook sprawl and keeps provenance consistent for every embedder-provided tool executor.

### 5. Pure range logic is adapted as functional core

**Choice:** Implement line-range parsing/formatting, compression, overlap, removal, hunk-shift, and accepted-line counting as pure deterministic functions with named constants and explicit errors. Use Git AI's behavior as a reference, but rewrite to match Animus safety rails.

**Rationale:** Range math is the reusable part. It should be unit-testable with positive and negative fixtures and should not depend on filesystem, Git, or process execution.

### 6. Git AI v3 compatibility is feature-gated and standards-bound

**Choice:** An optional Git AI compatibility adapter may read and write `authorship/3.0.0` logs under `refs/notes/ai` when a Git backend is explicitly selected. That adapter may use Git AI's SHA-256 session, trace, human, and legacy key formats only because the external standard requires them. Animus-owned hashes and receipt IDs continue to default to BLAKE3.

**Rationale:** Interop should be exact at the boundary and absent elsewhere. This avoids pretending SHA-256 is the Animus default while preserving compatibility with existing Git AI tooling.

### 7. Receipts store safe evidence, not raw prompts

**Choice:** Provenance receipts record safe metadata such as status, backend kind, mutation kind, labels, bounded counts, BLAKE3 content hashes, external compatibility IDs, redaction markers, and session/trace references. Raw prompts, provider payloads, credentials, full paths, and unbounded tool inputs/outputs remain out of receipts.

**Rationale:** Git AI links code to prompts, but Animus' existing evidence posture favors redacted receipts and external/session-ledger references. Provenance should improve auditability without becoming a data leak.

### 8. History rewrites are backend-specific repair jobs

**Choice:** The VCS-neutral core exposes content-based attribution transforms and merge/replay primitives. Backend adapters decide how to apply those primitives for their own operations. Git rebase, stash, cherry-pick, and notes syncing are Git adapter concerns, not generic runtime semantics.

**Rationale:** Version-control systems differ. Encoding Git operation names in core APIs would block a clean non-Git backend and would violate the user's requirement not to depend on Git as the only VCS.

## Risks / Trade-offs

- A provenance abstraction can become too broad. Start with mutating tool observations, line ranges, and injected backend persistence only.
- Exact Git AI compatibility can pull Git assumptions into shared types. Keep adapter DTOs distinct from core provenance DTOs and test the boundary.
- Rename and move tracking are tempting to solve early. Treat them as explicit mutation kinds and add minimal tests first; richer VCS-specific detection can follow.
- Prompt/session linking can leak sensitive data. Store references and hashes by default; require explicit opt-in compatibility behavior for any Git AI prompt fields that the standard allows.
