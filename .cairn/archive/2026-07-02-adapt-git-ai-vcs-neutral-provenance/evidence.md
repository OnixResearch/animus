# Evidence: adapt-git-ai-vcs-neutral-provenance

## Implementation evidence

- Added `docs/provenance-git-ai.md` documenting Git AI as a bounded reference for checkpointing, line attribution, accepted-line counts, and optional `authorship/3.0.0` interoperability while rejecting Git AI daemon, installer, updater, hosted telemetry, SQLite prompt store, Git operation watcher, and Git-only workflow assumptions.
- Added VCS-neutral provenance DTOs and pure helpers in `crates/animus-runtime/src/provenance/gitai.rs` for repository-relative file identities, BLAKE3 content hashes, content observations, line ranges, mutation kinds, backend requirements, before/after checkpoints, safe checkpoint receipts, VCS-neutral field checks, range/hunk transforms, and optional Git AI v3 compatibility DTOs.
- Added focused positive and negative runtime tests in `crates/animus-runtime/src/provenancetests/gitai.rs` covering VCS-neutral DTO review, non-Git in-memory backend recording, missing required backend failure, Nickel policy validation classes, pure range transforms, Git AI compatibility boundary validation, non-Git backend isolation, and receipt leak rejection.
- Added Nickel policy source `policy/provenance/policy.ncl` and checked export `policy/provenance/generated/provenance-policy.json` for canonical edit/write/replace/rename classification, file identity fields, backend requirements, and receipt redaction settings.
- Added focused rail `scripts/cairn/check-provenance-git-ai` to check policy export drift, reject an invalid typed policy fixture, run provenance tests, verify docs/source markers, and write safe focused-rail evidence.

## Checks

- Baseline before edits: `nix develop -c cargo test -p animus-runtime provenance` — passed, 15 tests.
- Change gates before edits: `nix run path:$PWD#cairn -- gate proposal/design/tasks adapt-git-ai-vcs-neutral-provenance --root $PWD` — passed.
- After implementation: `nix develop -c cargo test -p animus-runtime provenance` — passed, 28 tests.
- After implementation: `nix develop -c scripts/cairn/check-provenance-git-ai --root .` — passed; safe receipt at `target/cairn/focused-rails/provenance-git-ai.md`.
- After implementation: `nix develop -c cargo check -p animus-runtime --all-targets` — passed.
- Post-archive roadmap cleanup: `scripts/cairn/check-roadmap --root .` — passed after replacing the active-change prerequisite with `agent_harness_runtime.provenance_git_ai.verification.git_optional` and removing the archived overlap owner.
- Focused clippy attempt: `nix develop -c cargo clippy -p animus-runtime --all-targets -- -D warnings` — blocked by pre-existing `animus-adapters` lint failures outside touched files.
- Package-only clippy attempt: `nix develop -c cargo clippy -p animus-runtime --lib --tests --no-deps -- -D warnings` — blocked by pre-existing `animus-runtime` lint failures outside `crates/animus-runtime/src/provenance/gitai.rs` and `crates/animus-runtime/src/provenancetests/gitai.rs`; captured log had no touched provenance-file diagnostics after cleanup.

## Non-claims

This change does not implement live Git notes persistence, Git CLI integration, Git hooks, Git operation watchers, provider behavior, TUI behavior, production readiness, or Git as a required VCS.
