# Evidence: adapt-openai-codex-opt-in-memories

r[verify agent_harness_runtime.openai_codex_opt_in_memories.policy_controls]

## Implementation evidence

- Extended `crates/animus-adapters/src/front/memory.rs` with opt-in memory use/generation policy DTOs, evidence-backed record contracts, conservative generation eligibility, bounded injection ledgers, required-rule separation, memory availability status, and retrieval/mutation/availability adapter traits.
- Reused the existing memory facade and prompt-safe metadata boundaries so storage, retrieval/search, update/delete, extraction, and availability remain host-owned adapter contracts.
- Added positive and negative focused tests for opted-in controls, missing consent, record validation, stale records, missing evidence, invalid scope/confidence, backend-swap-equivalent injection plans, disallowed external context, secret-bearing content, redaction failures, exhausted generation budget, bounded/source-attributed injection, and disabled-memory non-authority over guidance/policy.
- Added `docs/openai-codex-opt-in-memories.md` documenting opt-in controls, record contracts, generation eligibility, adapter boundaries, injection ledgers, non-authoritative memory behavior, and non-claims.

## Checks

- Baseline before edits: `nix develop -c cargo test -p animus-adapters memory --locked` — passed, 5 tests.
- After implementation: `nix develop -c cargo fmt --check` — passed.
- After implementation: `nix develop -c cargo test -p animus-adapters memory --locked` — passed, 10 tests.
- After implementation: `nix develop -c cargo check -p animus-adapters --all-targets --locked` — passed.
- Focused clippy: `nix develop -c cargo clippy -p animus-adapters --lib --no-deps -- -D warnings` remains globally blocked by pre-existing `animus-adapters` lint debt; recaptured log at `target/animus-adapters-clippy-memory.log` contains no diagnostics for `crates/animus-adapters/src/front/memory.rs` or `crates/animus-adapters/src/front/mod.rs`.

## Next-best validation for blocked rails

- Fix the existing crate-wide clippy backlog outside the touched memory/front module before using crate-wide clippy as closeout evidence for this change.
