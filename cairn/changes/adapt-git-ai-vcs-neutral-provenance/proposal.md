# Proposal: Adapt Git AI provenance patterns without Git lock-in

## Why

Git AI proves a useful explicit-provenance pattern for coding agents: agents checkpoint the lines they changed, commits carry line-level attribution, and later stats can explain which agent, model, session, and tool call produced surviving code. Animus should adapt that evidence model because it aligns with planned safe receipts, neutral session ledgers, injected tool dispatch, and harness feedback loops.

Animus must not make Git the only version-control story. Git notes and the Git AI v3 authorship-log format are valuable compatibility targets, but the reusable Animus domain should describe version-control-neutral provenance over content, line ranges, sessions, and checkpoint observations. Git, Jujutsu, or any other VCS/storage integration belongs behind an injected backend adapter.

## What Changes

- Treat Git AI as a bounded reference for explicit checkpointing, line-level attribution, stats, and Git notes interoperability.
- Define a VCS-neutral provenance domain for sessions, trace IDs, mutating tool observations, line ranges, file identities, content hashes, and safe receipt metadata.
- Add a version-control backend boundary so Git notes support is optional and future VCS adapters do not change runtime/core provenance logic.
- Specify a mutating-tool checkpoint observer that records before/after observations around edit/write/replace/rename-like tool calls without relying on Git hooks or Git CLI behavior.
- Adapt pure line-range, range-compression, hunk-shift, and accepted-line counting ideas as testable functional core logic.
- Provide an optional Git AI v3 compatibility adapter that can read/write `refs/notes/ai` authorship logs when a Git backend is selected.
- Add focused positive and negative verification for non-Git fake backends, Git AI compatibility, receipt redaction, and VCS boundary failures.

## Impact

- **Files**: New Cairn change artifacts under `cairn/changes/adapt-git-ai-vcs-neutral-provenance/`; later implementation may add provenance DTO modules or an `animus-provenance` crate, `policy/provenance/` Nickel contracts, optional Git AI compatibility code, docs, focused tests, and flake checks.
- **Testing**: Future implementation must include positive and negative tests for provenance DTO validation, mutating-tool checkpoint classification, pure line-range/hunk transformations, in-memory non-Git backend behavior, Git AI v3 serialization compatibility, missing backend failures, and receipt leak rejection.
- **Non-claims**: This change does not adopt Git AI's daemon, installer, updater, hosted telemetry, SQLite prompt store, Git operation watcher, CLI UX, or Git as the only supported VCS. Git-specific SHA-256 ID derivation is allowed only inside the optional Git AI compatibility adapter because the external standard requires it.
