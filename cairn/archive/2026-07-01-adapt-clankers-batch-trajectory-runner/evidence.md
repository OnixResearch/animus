# Evidence: adapt-clankers-batch-trajectory-runner

## Implemented surfaces

- `crates/animus-runtime/src/batch.rs` defines pure batch planning, resume evaluation, plan-order output normalization, safe receipt validation, and host-injected execution/renderer ports.
- `crates/animus-runtime/src/lib.rs` reexports the batch runtime facade.
- `docs/batch-trajectory-runner.md` documents local-only scope, non-claims, and focused verification.
- `cairn/runtime-facade-inventory.ncl` classifies the new batch runtime exports.

## Passed checks

- `nix run .#cairn -- gate proposal adapt-clankers-batch-trajectory-runner --root .` — PASS.
- `nix run .#cairn -- gate design adapt-clankers-batch-trajectory-runner --root .` — PASS.
- `nix run .#cairn -- gate tasks adapt-clankers-batch-trajectory-runner --root .` — PASS before task closeout edits.
- `nix run .#cairn -- gate proposal adapt-clankers-batch-trajectory-runner --root .` — PASS after the headless protocol prerequisite was archived.
- `nix run .#cairn -- gate design adapt-clankers-batch-trajectory-runner --root .` — PASS after the headless protocol prerequisite was archived.
- `nix run .#cairn -- gate tasks adapt-clankers-batch-trajectory-runner --root .` — PASS after task closeout edits.
- `nix run .#cairn -- sync adapt-clankers-batch-trajectory-runner --root .` — PASS dry-run with an unblocked sync plan.
- `nix run .#cairn -- sync adapt-clankers-batch-trajectory-runner --root . --execute` — reported `mutated: true`, but the manifest before/after hash was unchanged; the batch requirement delta was manually merged into `cairn/specs/agent-harness-runtime/spec.md` per the Cairn sync pitfall guidance.
- `rg 'agent_harness_runtime\\.clankers_batch_runner' cairn/specs/agent-harness-runtime/spec.md` — PASS with planning, execution, resume, output, metadata, and verification IDs present.
- `nix run .#cairn -- validate --root .` — PASS after manual accepted-spec sync with `valid: true`, `changes: 44`, and `specs_validated: 45`.
- `scripts/cairn/check-roadmap --root .` — PASS after the headless prerequisite archive and roadmap hygiene repair.
- `cargo fmt --all -- --check` — PASS.
- `cargo test -p animus-runtime --locked` — PASS: 56 tests, including 10 batch positive/negative fixtures and 5 headless protocol fixtures.

## Blocked broader checks

- `cargo clippy -p animus-runtime --all-targets --offline -- -D warnings` is blocked before reaching the batch crate by pre-existing `animus-adapters` lint debt. First issue class observed: `clippy::vec_init_then_push` in `crates/animus-adapters/src/actor/admit/core.rs`, followed by existing `struct_field_names`, `too_many_arguments`, `assertions_on_constants`, wildcard import, and missing docs/panics diagnostics. Next best check run: locked `animus-runtime` unit/doc tests.
- `scripts/cairn/check-runtime-facade-inventory --root .` is blocked by pre-existing inventory drift across `animus-adapters`, `animus-preserves`, and `animus-runtime`. First issue class observed: `RUNTIME_FACADE_UNCLASSIFIED_EXPORT` for existing adapter exports. Next best check: the new batch exports were added to `cairn/runtime-facade-inventory.ncl` and the runtime facade doc was updated.

## Non-claims

No live provider calls, cloud batch API, daemon/session persistence, TUI dogfood, filesystem traversal, dataset export, performance claim, or production-readiness claim was made.
