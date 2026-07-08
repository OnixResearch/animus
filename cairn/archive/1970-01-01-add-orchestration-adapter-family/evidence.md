# Evidence: add-orchestration-adapter-family

## Implementation

- Added `animus_adapters::orchestration` as a no-std implementation-free adapter family for host-supplied board/task graph loading, handoff append, scheduling admission, cancellation, bounded lifecycle evidence, typed fail-closed errors, and safe orchestration receipts.
- Added `animus_runtime::OrchestrationAdapterBoardService` as a compatibility bridge from the public adapter-family service to the existing `KanbanBoardService` seam while leaving runtime-owned graph validation, promotion, workspace admission, and tenant-safe context assembly in `animus-runtime`.
- Added focused positive and negative coverage for service injection, graph loading, handoff append, scheduling admission, cancellation, safe receipts, missing service, denied capability, unavailable backend, invalid graph, duplicate graph, oversized evidence, unsafe handoff, cross-tenant leak, out-of-scope mutation, unsupported scheduling, schema mismatch, receipt leak rejection, and bridge failure mapping.
- Added `docs/orchestration-adapter-family.md`, README discovery text, contracts-index coverage, adapter-conformance-index cases, and runtime facade inventory entries.

## Checks

- Baseline before edits: `cargo test -p animus-adapters --lib` — passed, 486 tests.
- Baseline before edits: `cargo test -p animus-runtime kanban --lib` — passed with 0 selected tests under the existing filter.
- `cargo fmt -- crates/animus-adapters/src/orchestration.rs crates/animus-runtime/src/kanban.rs crates/animus-runtime/src/exports.rs crates/animus-adapters/src/lib.rs` — passed.
- `cargo test -p animus-adapters orchestration --lib` — passed, 2 selected tests.
- `cargo test -p animus-runtime adapter_family_bridge --lib` — passed, 2 selected tests.
- `cargo test -p animus-adapters --lib` — passed.
- `cargo test -p animus-runtime --lib` — passed, 292 tests.
- `cargo check --workspace --all-targets` — passed.
- `scripts/cairn/check-adapter-conformance-index --root .` — passed.
- `scripts/cairn/check-runtime-facade-inventory --root .` — passed.
- `nix run .#cairn -- validate --root .` — passed.
- `nix run .#cairn -- gate proposal add-orchestration-adapter-family --root .` — passed.
- `nix run .#cairn -- gate design add-orchestration-adapter-family --root .` — passed.
- `nix run .#cairn -- gate tasks add-orchestration-adapter-family --root .` — passed.

## Notes

- `cargo clippy -p animus-adapters --lib -- -D warnings` was checked after refactoring the new orchestration module; remaining failures are existing gateway/OpenClaw guardrail lint debt outside this change's touched orchestration code.
- `nix run .#cairn -- gate ... --run-probes` is policy-disabled for inline probes; gates were rerun without `--run-probes` and passed.

## Non-claims

No new `animus-orchestration` crate, concrete board database, production scheduler, worker process supervisor, VCS automation, filesystem storage layout, network transport, live multi-agent orchestration, clock source, or durable persistence backend is implemented or claimed.
