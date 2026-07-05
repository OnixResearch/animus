# Git AI provenance adaptation boundary

Animus uses Git AI as a bounded reference for explicit checkpointing, line-level attribution, session/model/tool identity, accepted-line statistics, and optional Git AI v3 authorship-log interoperability.

Animus does not adopt Git AI's daemon, installer, updater, hosted telemetry, SQLite prompt store, Git operation watcher, Git-only workflow assumptions, or Git as the required VCS. Reusable runtime and provenance contracts stay VCS-neutral; Git-specific refs, Git AI external IDs, and `authorship/3.0.0` records live only at the optional compatibility adapter boundary.

## Reusable provenance core

The `animus-runtime` provenance module defines pure DTOs and helpers for:

- repository-relative file identity, BLAKE3 content hashes, content observations, one-based line ranges, sessions, trace IDs, and canonical mutating-tool kinds;
- VCS-neutral field review that rejects Git note refs, commit SHAs, branch/stash/rebase fields, Git commands, and Git hook semantics from reusable DTOs;
- injected version-control provenance backend contracts with disabled, optional, and required backend modes;
- before/after mutating-tool checkpoints over already-loaded observations, with missing required backends failing before complete line-level evidence is claimed;
- Nickel-authored mutating-tool classification policy under `policy/provenance/` for canonical edit/write/replace/rename kinds, file identity fields, backend requirements, and receipt redaction;
- pure range compression, removal, hunk-shift, and accepted-line counting logic;
- safe checkpoint receipts that carry status, backend kind, mutation kind, bounded counts, session/trace IDs, BLAKE3 content hashes, and redaction state without raw prompts, provider payloads, credentials, full paths, raw tool bodies, raw model output, raw diffs, or unbounded outputs.

The core is deterministic over in-memory inputs. It does not read files, invoke Git or Jujutsu, inspect branches, spawn commands, install hooks, persist notes, contact providers, or own shell policy.

## Optional Git AI compatibility

The optional compatibility DTO validates Git AI v3 `authorship/3.0.0` records stored under `refs/notes/ai`. SHA-256-shaped external keys are accepted only because the Git AI format requires them. Animus-owned hashes and receipt identities continue to use BLAKE3.

A non-Git backend can record provenance without Git AI IDs, Git commit SHAs, Git CLI availability, or Git notes.

## Focused validation

Run:

```sh
scripts/cairn/check-provenance-git-ai --root .
```

The rail checks Nickel export drift, rejects an invalid typed policy fixture, runs provenance tests, verifies non-Git backend coverage, and writes a safe focused-rail receipt under `target/cairn/focused-rails/`.
